//! Yerler: slot (JS slotBase, slotBase0, durusDef, durusW), Bekçi'nin yeri (bekci), kale tarafı (gside),
//! taşıyıcının yolu ve kesişme (tasHamle, yolBag, yolKesis, presKesis, koyHedef).
#![allow(non_snake_case)]

use crate::core_sabit::{Q, TQ};
use crate::mat::{hypot, jmax, jmin, jsign};
use crate::model::{KafaD, Mac, Oyuncu, Pid};
use crate::oyuncu::Tepki;
use crate::saha::{KR, MOUTH};
use crate::temas::{bek_dl, bek_dv, bek_re, Kural};
use crate::yetenek::mek;

use super::gecici_dis as dis;
use super::*;

/// JS gside(a, t, off): `a`dan `t` takımının kalesine doğru `off` kadar.
pub fn gside(ax: f64, ay: f64, t: u8, off: f64) -> (f64, f64) {
    let gx = own_x(t);
    let dx = gx - ax;
    let dy = GH / 2.0 - ay;
    let l = hypot(dx, dy);
    let l = if l != 0.0 && l == l { l } else { 1.0 };
    (ax + dx / l * off, ay + dy / l * off)
}

/// Slot (JS `att && p.rslot ? p.rslot : p.slot`): (d, l).
pub fn slot_of(p: &Oyuncu, att: bool) -> (f64, f64) {
    match (&p.govde.rslot, att) {
        (Some(r), true) => (r.d, r.l),
        _ => (p.govde.slot.d, p.govde.slot.l),
    }
}
/// JS `(q.rslot || q.slot)`.
pub fn rslot_of(p: &Oyuncu) -> (f64, f64) {
    match &p.govde.rslot {
        Some(r) => (r.d, r.l),
        None => (p.govde.slot.d, p.govde.slot.l),
    }
}

/// Deney (ALAN_HUCUM_BOY_IZ; ölçümde HUCUM_BOY_ZORLA): hücum dizilişinin ön ucu toptan en çok bir pas menzili ileride.
pub static HUCUM_BOY_ZORLA: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub fn hucum_boy() -> bool {
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    HUCUM_BOY_ZORLA.load(std::sync::atomic::Ordering::Relaxed) || *A.get_or_init(|| crate::iz_acik() && std::env::var("ALAN_HUCUM_BOY_IZ").is_ok())
}

/// JS slotBase0(m, p, E, att): takımın bloğunda slotun yeri. `sl`: slot verilirse (JS `{...p, slot, rslot}`).
pub fn slot_base0(m: &Mac, ps: &[Oyuncu], p: &Oyuncu, sl: Option<(f64, f64)>, ex: f64, ey: f64, att: bool) -> (f64, f64) {
    let t = p.takim();
    let tac = &m.tac[t as usize];
    let s = sl.unwrap_or_else(|| slot_of(p, att));
    let x0 = own_x(t);
    let dir = dir_of(t);
    let bd = (ex - x0) * dir;
    let (back, len, wid, sh);
    if att {
        back = cl(bd - 18.0 * GKX - 8.0 * (1.0 - tac.risk.unwrap_or(f64::NAN)) * GKX, 10.0 * GKX, 62.0 * GKX);
        let mut last = 0.0;
        for q in ps {
            if q.team != p.team && !q.bekci() {
                last = jmax(last, (q.x - x0) * dir);
            }
        }
        let mut front = jmin(94.0 * GKX, jmax(back + 24.0 * GKX, last + 1.0));
        // ALAN_HUCUM_BOY_IZ: ön uç toptan bir pas menzilinden ileri gitmez (öndekiler topla bağlantıda kalır)
        if hucum_boy() {
            front = jmax(back + 24.0 * GKX, jmin(front, bd + crate::kafa::kafa2_baglam::ayar().pass_max_f));
        }
        len = front - back;
        wid = tac.genislik.as_deref().and_then(GEN).unwrap_or(38.0 * GKY);
        sh = 0.15;
    } else {
        back = cl(jmin(tac.blok.as_deref().and_then(BLOK).unwrap_or(34.0 * GKX), bd - 6.0), 8.0 * GKX, 62.0 * GKX);
        len = 22.0 * GKX;
        wid = 26.0 * GKY;
        sh = 0.4;
    }
    (cl(x0 + dir * (back + (s.0 - 0.2) / 0.6 * len), 2.0, GW - 2.0), cl(GCY + s.1 * wid / 2.0 + (ey - GCY) * sh, 2.0, GH - 2.0))
}

/// JS durusDef(m, p, k, att): duran top dayanağı k'de slotun yeri.
pub fn durus_def(m: &Mac, ps: &[Oyuncu], p: &Oyuncu, k: usize, att: bool) -> (f64, f64) {
    let t = p.takim();
    slot_base0(m, ps, p, None, own_x(t) + dir_of(t) * DUR_A[k], GCY, att)
}

/// JS durusW(f): topun derinliğine göre üç dayanağın payı.
pub fn durus_w(f: f64) -> [f64; 3] {
    if f <= DUR_A[0] {
        return [1.0, 0.0, 0.0];
    }
    if f >= DUR_A[2] {
        return [0.0, 0.0, 1.0];
    }
    if f <= DUR_A[1] {
        let u = (f - DUR_A[0]) / (DUR_A[1] - DUR_A[0]);
        return [1.0 - u, u, 0.0];
    }
    let u = (f - DUR_A[1]) / (DUR_A[2] - DUR_A[1]);
    [0.0, 1.0 - u, u]
}

/// JS slotBase dönüşü: yer ve (duran top dizilişi uygulandıysa) dizilişin payı `el`.
#[derive(Clone, Copy, Debug)]
pub struct Yer {
    pub x: f64,
    pub y: f64,
    pub el: Option<f64>,
}

/// JS slotBase(m, p, E, att): slotun yeri, takımın duran top dizilişiyle kaydırılmış.
pub fn slot_base(m: &Mac, ps: &[Oyuncu], p: &Oyuncu, ex: f64, ey: f64, att: bool) -> Yer {
    let b = slot_base0(m, ps, p, None, ex, ey, att);
    let t = p.takim();
    let D = match m.tac.get(t as usize).and_then(|q| q.durus.as_ref()) {
        Some(d) => d,
        None => return Yer { x: b.0, y: b.1, el: None },
    };
    let pl = if att { D.huc.iter().map(|v| v.iter().map(|q| (q.f, q.y)).collect::<Vec<_>>()).collect::<Vec<_>>() } else { D.sav.iter().map(|v| v.iter().map(|q| (q.f, q.y)).collect::<Vec<_>>()).collect::<Vec<_>>() };
    let x0 = own_x(t);
    let dir = dir_of(t);
    let w = durus_w((ex - x0) * dir);
    let (mut ox, mut oy, mut el) = (0.0, 0.0, 0.0);
    for k in 0..3 {
        if w[k] == 0.0 || w[k] != w[k] {
            continue;
        }
        let q = match pl.get(k).and_then(|v| v.get((p.govde.i - 1.0) as usize)) {
            Some(q) if p.govde.i >= 1.0 => *q,
            _ => continue,
        };
        if q.0.is_some() || q.1.is_some() {
            el += w[k];
        }
        let d0 = durus_def(m, ps, p, k, att);
        if let Some(f) = q.0 {
            ox += w[k] * (x0 + dir * f * GKX - d0.0);
        }
        if let Some(y) = q.1 {
            oy += w[k] * (y * GKY - d0.1);
        }
    }
    if (ox != 0.0 && ox == ox) || (oy != 0.0 && oy == oy) {
        Yer { x: cl(b.0 + ox, 2.0, GW - 2.0), y: cl(b.1 + oy, 2.0, GH - 2.0), el: Some(el) }
    } else {
        Yer { x: b.0, y: b.1, el: None }
    }
}

/// Bekçi'nin yerine bakarken maçın gerekli parçası (JS `m`): oyuncular, taşıyıcı, Bekçi'nin kendi sırası.
/// Bekçi'nin okuduğu dünya: maçın düz oyuncuları ya da kafanın paylaşımlı dünyası (kopyasız).
#[derive(Clone, Copy)]
pub enum BekPs<'a> {
    Duz(&'a [Oyuncu]),
    Pay(&'a [std::sync::Arc<Oyuncu>]),
}
impl<'a> BekPs<'a> {
    #[inline]
    pub fn len(self) -> usize {
        match self {
            BekPs::Duz(v) => v.len(),
            BekPs::Pay(v) => v.len(),
        }
    }
    #[inline]
    pub fn get(self, i: usize) -> &'a Oyuncu {
        match self {
            BekPs::Duz(v) => &v[i],
            BekPs::Pay(v) => &v[i],
        }
    }
}
pub struct BekM<'a> {
    pub ps: BekPs<'a>,
    /// JS `m.holder`: topu tutan (sahte de olabilir: kafa "top burada olsa" diye sorar)
    pub holder: Option<crate::dunya::Yerde<'a>>,
    /// JS `q === p` için Bekçi'nin sırası
    pub ben: Option<Pid>,
    /// Taşıyıcının şut tehdidi tartılır mı: maç başladı, kafa kurulu (maç kurulurken hayır). Eskiden
    /// sutHiz bunu iş parçacığına özel kafadan soruyordu; artık bağlamı kuran maçtan bilir.
    pub sut: bool,
}

/// JS bekci(p, E, m): Bekçi'nin yeri (açıyı daraltır, uzaktan topa destek, karşı karşıyada çıkış).
/// Taşıyıcı rakipse Bekçi topun şimdiki yerine değil, kendisi yerine varana kadar (tepki + yol / hız) taşıyıcının
/// gideceği yere göre yerleşir: yoksa vardığında açı çoktan kaymış olur, kapattığı açı yanlış açıdır.
pub fn bekci(p: &Oyuncu, ex: f64, ey: f64, m: Option<&BekM>, k: &dyn Kural) -> (f64, f64) {
    let g0 = bekci_an(p, ex, ey, m, k);
    let hh = match m.and_then(|m| m.holder).filter(|hh| hh.govde.team != p.team) {
        Some(h) => h,
        None => return g0,
    };
    let (hvx, hvy) = (hh.govde.vx, hh.govde.vy);
    let hv = hypot(hvx, hvy);
    if !(hv > 1e-9) {
        return g0;
    }
    let sp = jmax(1e-6, crate::kural::enerji::spd(p));
    let mut t = bek_re(p) + hypot(g0.0 - p.x, g0.1 - p.y) / sp;
    // taşıyıcı kale çizgisini geçemez: öngörü çizgide kesilir
    let gx = own_x(p.takim());
    let (dx0, dx1) = (ex - gx, ex + hvx * t - gx);
    if dx0 * dx1 <= 0.0 && hvx.abs() > 1e-9 {
        t = jmax(0.0, (gx - ex) / hvx);
    }
    bekci_an(p, ex + hvx * t, ey + hvy * t, m, k)
}

fn bekci_an(p: &Oyuncu, ex: f64, ey: f64, m: Option<&BekM>, _k: &dyn Kural) -> (f64, f64) {
    let gx = own_x(p.takim());
    let dx = ex - gx;
    let dy = ey - GCY;
    let L = {
        let l = hypot(dx, dy);
        if l != 0.0 && l == l {
            l
        } else {
            1.0
        }
    };
    // açıortay (Bekçi'nin duracağı çizgi): topun yerinden iki direğin ortası
    let (bxu, byu) = {
        let nz = |l: f64| if l != 0.0 && l == l { l } else { 1.0 };
        let (a1x, a1y, a2x, a2y) = (gx - ex, GCY - MOUTH - ey, gx - ex, GCY + MOUTH - ey);
        let (l1, l2) = (nz(hypot(a1x, a1y)), nz(hypot(a2x, a2y)));
        let (x, y) = (a1x / l1 + a2x / l2, a1y / l1 + a2y / l2);
        let l = nz(hypot(x, y));
        (x / l, y / l)
    };
    let mut r = if let Some(vb) = m.filter(|m| m.sut).and_then(|m| m.holder).filter(|hh| hh.govde.team != p.team).map(|hh| dis::sut_hiz(hh.govde)) {
        bek_derinlik(p, ex, ey, gx, L, bxu, byu, vb)
    } else {
        let cov = Q.bek_body + 0.8 * bek_dl(p);
        let mo_e = jmax(0.4, MOUTH * dx.abs() / L);
        cl(L * cl(1.0 - cov / mo_e, 0.1, 0.55) + 0.08 * (bek_y(p, mek::ACI_DARALT) - 10.0), 2.5, KR)
    };
    if let Some(m) = m {
        if L > S.bek_sup_l {
            let mut d_d = 1e9;
            for qi in 0..m.ps.len() {
                let q = m.ps.get(qi);
                if q.team == p.team && !q.bekci() {
                    d_d = jmin(d_d, hypot(q.x - gx, q.y - GCY));
                }
            }
            let rs = jmin(jmin(S.bek_sup_max + 0.3 * (bek_y(p, mek::ACI_DARALT) - 10.0), d_d - S.bek_sup_arka), KR + (L - S.bek_sup_l) * S.bek_sup_k);
            if rs > r {
                r = rs;
            }
        }
    }
    if let Some(m) = m {
        if let Some(hh) = m.holder {
            if hh.govde.team != p.team && L < 30.0 {
                let mut dmin = 1e9;
                for qi in 0..m.ps.len() {
                    let q = m.ps.get(qi);
                    if q.team != p.team || Some(qi) == m.ben {
                        continue;
                    }
                    let ux = gx - hh.x;
                    let uy = GCY - hh.y;
                    let ul = {
                        let l = hypot(ux, uy);
                        if l != 0.0 && l == l {
                            l
                        } else {
                            1.0
                        }
                    };
                    let al = ((q.x - hh.x) * ux + (q.y - hh.y) * uy) / ul;
                    if al > 2.0 && al < ul - 1.0 {
                        dmin = jmin(dmin, (-(q.x - hh.x) * uy + (q.y - hh.y) * ux).abs() / ul);
                    }
                }
                let w = jmax(cl((dmin - 1.5) / 3.0, 0.0, 1.0), cl((24.0 - L) / 10.0, 0.0, 1.0)) * cl((30.0 - L) / 6.0, 0.0, 1.0);
                let kk = bek_y(p, mek::KARSI_KARSIYA);
                let yakin = L < KR + 6.0;
                let r_out = cl(
                    if yakin { L - 1.2 - 0.05 * (20.0 - kk) } else { jmax(L * (0.35 + 0.01 * kk), L - (6.0 - 0.15 * kk)) },
                    2.5,
                    jmax(2.5, jmin(KR + 4.0, L - if yakin { 1.2 } else { 2.5 })),
                );
                /* (rY: taşıyıcının şut hazırlığına göre çıkış yarıçapı — artık kullanılmıyor, hesaplanmaz) */
                if yakin && r_out > r {
                    r += (r_out - r) * w;
                } /* şut hazırlığında her tik bir adım öne çıkma yok (dalış modelinde öne çıkan Bekçi tepkisi içinde yetişemiyordu); yalnız taşıyıcı dibe girince (karşı karşıya) üstüne çıkar */
            }
        }
    }
    let _r = r;
    let a1x = gx - ex;
    let a1y = GCY - MOUTH - ey;
    let a2x = gx - ex;
    let a2y = GCY + MOUTH - ey;
    let nz = |l: f64| if l != 0.0 && l == l { l } else { 1.0 };
    let l1 = nz(hypot(a1x, a1y));
    let l2 = nz(hypot(a2x, a2y));
    let mut bx = a1x / l1 + a2x / l2;
    let mut by = a1y / l1 + a2y / l2;
    let bl = nz(hypot(bx, by));
    bx /= bl;
    by /= bl;
    let st = jmax(0.0, L - _r);
    let (qx, qy) = (ex + bx * st, ey + by * st);
    // deney (bek_direk): top direğin dışındayken (dar açı) Bekçi yakın direğin dışına taşmaz, çizgiden direğin gerektirdiğinden
    // çok çıkmaz: açıortayda topun üstüne çıkınca yakın direkle arası açılıyor, içeri çekilen şut uzak köşeye gidiyordu
    if bek_direk() && (ey - GCY).abs() > MOUTH {
        let yan = if ey > GCY { 1.0 } else { -1.0 };
        let ust = MOUTH - 0.6 * Q.bek_body;
        let qy = if (qy - GCY) * yan > ust { GCY + yan * ust } else { qy };
        let dr = if qx > gx { 1.0 } else { -1.0 };
        let qx = if (qx - gx).abs() > BEK_DIREK_DER { gx + dr * BEK_DIREK_DER } else { qx };
        return (qx, qy);
    }
    (qx, qy)
}

/// bek_direk: dar açıda Bekçi'nin çizgiden en çok çıkışı (br).
pub const BEK_DIREK_DER: f64 = 1.5;
/// Deney (ALAN_BEK_DIREK_IZ; ölçümde/wasm'da BEK_DIREK_ZORLA): dar açıda Bekçi yakın direği tutar. Varsayılan kapalı.
pub static BEK_DIREK_ZORLA: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub fn bek_direk() -> bool {
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    BEK_DIREK_ZORLA.load(std::sync::atomic::Ordering::Relaxed) || *A.get_or_init(|| crate::iz_acik() && std::env::var("ALAN_BEK_DIREK_IZ").is_ok())
}

/// Bekçi'nin derinliği değerle (dalış modeliyle aynı hesap): açıortay üstündeki aday derinliklerde, taşıyıcının kale
/// ağzına (direkten direğe, gövde aralığıyla) çekebileceği en sert şutlar tartılır: Bekçi tepki süresince yerinde,
/// sonra dalış hızıyla en çok tam uzanma kadar; top çizgiye varmadan gövdesi topun yoluna değebiliyorsa o şut kapanır.
/// Kale ağzının en büyük kapanan payı seçilir; eşitlikte Bekçi'nin şimdiki derinliğine en yakın aday (titreme yok).
/// Adaylar Bekçi'nin bir tikte aldığı yol aralığıyla, 2,5 br'den Kuyu sınırına (eski sınırlar).
#[inline(never)]
fn bek_derinlik(p: &Oyuncu, ex: f64, ey: f64, gx: f64, L: f64, bxu: f64, byu: f64, vb: f64) -> f64 {
    let (re, dv, dl) = (bek_re(p), bek_dv(p), bek_dl(p));
    let vb = jmax(vb, 1e-6);
    let adim = jmax(0.05, crate::kural::enerji::spd(p));
    let r_simdi = hypot(p.x - gx, p.y - GCY);
    // saf hesap: girdileri bit bit aynıysa sonuç aynı (L, bxu, byu ex, ey, gx'ten çıkar). Adayların kapattığı yol sayıları
    // Bekçi'nin şimdiki derinliğine (r_simdi) bağlı değil: r_simdi'siz anahtarla saklanır, seçim (en çok kapatan; eşitlikte
    // şimdikine en yakın, önce gelen) burada; budamalı eski seçimle birebir aynı (budama yalnız seçilemeyecek adayı keser).
    let anahtar = [re, dv, dl, adim, ex, ey, gx, vb].map(f64::to_bits);
    let say = match BEK_DER.with(|c| c.borrow().get(&anahtar).cloned()) {
        Some(v) => {
            BEK_DER_SAY.with(|c| c.set((c.get().0 + 1, c.get().1)));
            v
        }
        None => {
            BEK_DER_SAY.with(|c| c.set((c.get().0, c.get().1 + 1)));
            let mut v = Vec::new();
            bek_derinlik0(ex, ey, gx, L, bxu, byu, vb, re, dv, dl, adim, r_simdi, Some(&mut v));
            let v = std::rc::Rc::new(v);
            BEK_DER.with(|c| {
                let mut c = c.borrow_mut();
                if c.len() > 200_000 {
                    c.clear();
                }
                c.insert(anahtar, v.clone());
            });
            v
        }
    };
    let (mut eniyi, mut ek, mut ed) = (2.5, 0usize, f64::INFINITY);
    for &(r, k) in say.iter() {
        let d = (r - r_simdi).abs();
        if k > ek || (k == ek && d < ed) {
            eniyi = r;
            ek = k;
            ed = d;
        }
    }
    eniyi
}
thread_local! {
    static BEK_DER: std::cell::RefCell<crate::HizliMap<[u64; 8], std::rc::Rc<Vec<(f64, usize)>>>> = std::cell::RefCell::new(Default::default());
    pub static BEK_DER_SAY: std::cell::Cell<(u64, u64)> = const { std::cell::Cell::new((0, 0)) };
    /// bek_derinlik0'ın şut yolu tamponu: (yollar (ux, uy, baş, son), noktalar (px, py, er), s'ler)
    /// noktalar (px, py, er, er²·(1−δ), er²·(1+δ)): karekökü atlamak için erişimin karesinin bandı
    static BEK_YOL: std::cell::RefCell<(Vec<(f64, f64, usize, usize)>, Vec<(f64, f64, f64, f64, f64)>, Vec<f64>)> = const { std::cell::RefCell::new((Vec::new(), Vec::new(), Vec::new())) };
}
/// Karekökü atlama bandı (göreli): yuvarlama hatalarının (~1e-16) çok üstünde.
const KOK_PAY: f64 = 1e-12;
#[allow(clippy::too_many_arguments)]
#[inline(never)]
fn bek_derinlik0(ex: f64, ey: f64, gx: f64, L: f64, bxu: f64, byu: f64, vb: f64, re: f64, dv: f64, dl: f64, adim: f64, r_simdi: f64, mut sayim: Option<&mut Vec<(f64, usize)>>) -> f64 {
    let body = Q.bek_body;
    let n = (2.0 * MOUTH / body).ceil().max(1.0) as usize;
    // şut yollarının noktaları ve o anki erişim aday derinlikten bağımsız: bir kez (sonuç aynı)
    // Şut yolları düz tamponda: yol başına (ux, uy, başı, sonu); noktalar (px, py, er) ve yol üstündeki s'leri.
    // Eskisi Vec<Vec<..>> idi; değerler ve gezme sırası aynı.
    BEK_YOL.with(|t| {
        let mut t = t.borrow_mut();
        let (yollar, pts, ss) = &mut *t;
        yollar.clear();
        pts.clear();
        ss.clear();
        let mut em: f64 = 0.0;
        for j in 0..=n {
            let ty = GCY - MOUTH + 2.0 * MOUTH * j as f64 / n as f64;
            let (ux, uy) = (gx - ex, ty - ey);
            let ld = hypot(ux, uy);
            if !(ld > 0.0) {
                continue;
            }
            let (ux, uy) = (ux / ld, uy / ld);
            let bas = pts.len();
            let mut s = 0.0;
            loop {
                let t = s / vb;
                let er = body + jmin(dl, dv * jmax(0.0, t - re));
                let e2 = er * er;
                pts.push((ex + ux * s, ey + uy * s, er, e2 * (1.0 - KOK_PAY), e2 * (1.0 + KOK_PAY)));
                ss.push(s);
                em = if er > em || er != er { er } else { em };
                if s >= ld {
                    break;
                }
                s = jmin(ld, s + vb);
            }
            yollar.push((ux, uy, bas, pts.len()));
        }
        // Adaya, yol üstündeki izdüşümünden (|s − proj|) en büyük erişimden uzak nokta yetişemez (|P − k| ≥ |s − proj|;
        // 1e-6 pay kayan nokta hatasının çok üstünde): o noktalar sınanmaz, sayım aynı. NaN'da pencere açılmaz (hepsi sınanır).
        let EM = em + 1e-6;
        // yol başına pencerenin başı: aday adım adım kaydığı için önceki yerden yürünür (sıralı s'lerde
        // partition_point(|s| s < lo) ile aynı yer)
        let mut gost: Vec<usize> = yollar.iter().map(|y| y.2).collect();
        // `gerek`: adayın seçilebilmesi için kapatması gereken en az yol. Kalan yollar hepsini kapatsa bile
        // yetişemeyecekse sayım bırakılır (dönen değer gerek'ten küçük: aday seçilmez, sonuç aynı).
        let n_y = yollar.len();
        let mut kap = |r: f64, gerek: usize| -> usize {
            let st = jmax(0.0, L - r);
            let (kx, ky) = (ex + bxu * st, ey + byu * st);
            let mut c = 0;
            for (yi, &(ux, uy, bas, son)) in yollar.iter().enumerate() {
                if c + (n_y - yi) < gerek {
                    return c;
                }
                let proj = (kx - ex) * ux + (ky - ey) * uy;
                let (lo, hi) = (proj - EM, proj + EM);
                let mut i0 = gost[yi];
                while i0 > bas && !(ss[i0 - 1] < lo) {
                    i0 -= 1;
                }
                while i0 < son && ss[i0] < lo {
                    i0 += 1;
                }
                gost[yi] = i0;
                for (&s, &(px, py, er, e2a, e2u)) in ss[i0..son].iter().zip(&pts[i0..son]) {
                    if s > hi {
                        break;
                    }
                    let (ax, ay) = (px - kx, py - ky);
                    // hypot(ax, ay) >= |ax|, |ay| (IEEE): bunlardan biri er'i aşıyorsa uzak, hypot gereksiz
                    if ax.abs() > er || ay.abs() > er {
                        continue;
                    }
                    // hypot = sqrt(q), q = ax·ax + ay·ay (aynı işlemler). sqrt doğru yuvarlanır: q, er²'nin δ = 1e-12
                    // bandının dışındaysa karşılaştırmanın sonucu karekök alınmadan bellidir (yuvarlama hatası
                    // ~1e-16). Bandın içinde, NaN'da, sonsuzda ve er ≤ 0'da eski karşılaştırma.
                    if er > 0.0 {
                        let q = ax * ax + ay * ay;
                        if q <= e2a {
                            c += 1;
                            break;
                        }
                        if q >= e2u {
                            continue;
                        }
                    }
                    if hypot(ax, ay) <= er {
                        c += 1;
                        break;
                    }
                }
            }
            c
        };
    let r_maks = jmin(KR, jmax(2.5, L - body));
    let (mut eniyi, mut ek, mut ed) = (2.5, 0usize, f64::INFINITY);
    let mut r = 2.5;
    let n_yol = yollar.len();
    while r <= r_maks + 1e-9 {
        // sayım kipi: her adayın kapattığı yol sayısı (budamasız); seçim çağıranda (r_simdi'ye bağlı tek yer eşitlik)
        if let Some(v) = sayim.as_deref_mut() {
            v.push((r, kap(r, 0)));
            r += adim;
            continue;
        }
        let d = (r - r_simdi).abs();
        // bütün yolları kapatan bir aday varken daha uzak (ya da eşit uzak) aday seçilemez: sayım gereksiz
        if ek == n_yol && !(d < ed) {
            r += adim;
            continue;
        }
        let k = kap(r, if d < ed { ek } else { ek + 1 });
        if k > ek || (k == ek && d < ed) {
            eniyi = r;
            ek = k;
            ed = d;
        }
        r += adim;
    }
    eniyi
    })
}

/// JS koyHedef(p, h, ax, ay): taşıyıcının önüne gövde koyma noktası.
pub fn koy_hedef(p: &Oyuncu, h: &Oyuncu, ax: f64, ay: f64) -> (f64, f64) {
    let BD = 0.95 * TQ.min_d;
    let sp = hypot(h.vx, h.vy);
    let (mut ux, mut uy) = (ax, ay);
    if sp > 0.05 && (h.vx * ax + h.vy * ay) / sp > 0.0 {
        ux = h.vx / sp;
        uy = h.vy / sp;
    }
    let dx = h.x + ux * BD - p.x;
    let dy = h.y + uy * BD - p.y;
    let al = dx * ux + dy * uy;
    let dq = hypot(p.x - h.x, p.y - h.y);
    if dq < BD + 0.6 && al > 0.0 {
        (p.x + dx - ux * al - 0.08 * ux, p.y + dy - uy * al - 0.08 * uy)
    } else {
        (h.x + ux * BD, h.y + uy * BD)
    }
}

/// Taşıyıcının olası bir yönü (JS tasHamle öğesi): birim yön ve sahada/Kuyu dışında gidebileceği yol `sM`.
#[derive(Clone, Copy, Debug)]
pub struct Yon {
    pub ad: &'static str,
    pub ux: f64,
    pub uy: f64,
    pub s_m: f64,
}

/// JS tasHamle(h, dt): taşıyıcının gidebileceği yönler (kale, hızı, duvar boyu).
pub fn tas_hamle(hx: f64, hy: f64, hvx: f64, hvy: f64, dt: u8) -> Vec<Yon> {
    let ox = own_x(dt);
    let mut yon: Vec<Yon> = Vec::new();
    let mut ek = |ad: &'static str, ux: f64, uy: f64| {
        let l = hypot(ux, uy);
        if l > 0.0 {
            yon.push(Yon { ad, ux: ux / l, uy: uy / l, s_m: 0.0 });
        }
    };
    ek("kale", ox - hx, GCY - hy);
    ek("hız", hvx, hvy);
    for wy in [0.0, GH] {
        let dw = (hy - wy).abs();
        if dw > 9.0 {
            continue;
        }
        let g = {
            let s = jsign(ox - hx);
            if s != 0.0 && s == s {
                s
            } else {
                1.0
            }
        };
        ek("duvar", 10.0 * g, (if wy == 0.0 { -1.0 } else { 1.0 }) * jmin(if dw - 1.5 > 0.0 { dw - 1.5 } else { 0.0 }, 2.5));
    }
    for Y in yon.iter_mut() {
        let mut s_m = f64::INFINITY;
        if Y.ux > 0.0 {
            s_m = jmin(s_m, (GW - hx) / Y.ux);
        }
        if Y.ux < 0.0 {
            s_m = jmin(s_m, -hx / Y.ux);
        }
        if Y.uy > 0.0 {
            s_m = jmin(s_m, (GH - hy) / Y.uy);
        }
        if Y.uy < 0.0 {
            s_m = jmin(s_m, -hy / Y.uy);
        }
        {
            let R = Q.kuyu_a;
            let cx = hx - ox;
            let cy = hy - GCY;
            let b = cx * Y.ux + cy * Y.uy;
            let c = cx * cx + cy * cy - R * R;
            let D = b * b - c;
            if D >= 0.0 {
                let r = -b - D.sqrt();
                if r > 0.0 {
                    s_m = jmin(s_m, r);
                }
            }
        }
        Y.s_m = jmax(0.0, s_m);
    }
    yon
}

/// match.js ALAN_KOSU_TP(pr, d, v1): sprint profiliyle `d` yolun süresi (saf).
pub fn alan_kosu_tp(pr: Option<dis::KosuPr>, d: f64, v1: f64) -> f64 {
    match pr {
        Some(pr) if d > 0.0 => {
            let d_s = v1 * pr.Ts;
            if d <= d_s {
                d / v1
            } else {
                pr.Ts + (d - d_s) / (v1 * pr.k)
            }
        }
        _ => d / v1,
    }
}

/// JS yolBag(p, h) dönüşü: kesişmenin değişmeyenleri (hızlar, tepki, sprint profilleri, varış tabloları).
#[derive(Clone, Debug)]
pub struct YolBag {
    pub v_h: f64,
    pub v_p: f64,
    pub t_r0: f64,
    pub pr_h: Option<dis::KosuPr>,
    pub pr_p: Option<dis::KosuPr>,
    pub v_sh: f64,
    pub g_h: GecTab,
    pub g_p: GecTab,
}
impl YolBag {
    /// JS `KT(q, d, v1)`: `h` mı `p` mi.
    #[inline]
    pub fn KT(&self, hmi: bool, d: f64, v1: f64) -> f64 {
        alan_kosu_tp(if hmi { self.pr_h } else { self.pr_p }, d, v1)
    }
}

/// yolBag/yolKesis'in taşıyıcı hız yanılgısı (görüşe bağlı, 15 tikte bir değişir).
pub fn hiz_yanilgi(p: &Oyuncu, h: &Oyuncu, _d: &KafaD, dtik: Option<f64>) -> f64 {
    let g = p.govde.yet.ozk("gorus");
    let hs = sinh_k(12.9898 * (p.id + 1.0) + 4.1414 * (h.id + 1.0) + 78.233 * (dtik.filter(|t| *t != 0.0 && *t == *t).unwrap_or(0.0) / 15.0).floor());
    1.0 + (kesir(hs) - 0.5) * (20.0 - g) * 0.02
}

/// JS yolBag(p, h): `p` (presçi) ile `h` (taşıyıcı) arasındaki yarışın değişmeyenleri.
pub fn yol_bag(p: &Oyuncu, h: &Oyuncu, k: &dyn Kural, d: &KafaD, dtik: Option<f64>) -> YolBag {
    let pr_h = dis::alan_kosu_pr(h, k);
    let pr_p = dis::alan_kosu_pr(p, k);
    let v_sh = v_sp(h, k);
    let v_sq = v_sp(p, k);
    YolBag {
        v_h: v_sh * dis::top_k(h) * hiz_yanilgi(p, h, d, dtik),
        v_p: v_sq,
        t_r0: jmax(0.0, TEPKI(p, Tepki::Kovala, k)),
        pr_h,
        pr_p,
        v_sh,
        g_h: gec_tab_v(h, Some(v_sh), k),
        g_p: gec_tab_v(p, Some(v_sq), k),
    }
}

/// JS yolKesis dönüşü.
#[derive(Clone, Copy, Debug)]
pub struct Kesis {
    pub x: f64,
    pub y: f64,
    pub s: f64,
    pub once: bool,
}

/// JS yolKesis(p, h, Y, cx): taşıyıcı `Y` yönünde koşarsa `p`nin onu en erken kestiği nokta.
pub fn yol_kesis(p: &Oyuncu, h: &Oyuncu, Y: &Yon, cx: &YolBag) -> Kesis {
    let (v_h, v_p, t_r0) = (cx.v_h, cx.v_p, cx.t_r0);
    let wx = h.x - p.x;
    let wy = h.y - p.y;
    let a = wx * Y.ux + wy * Y.uy;
    let w2 = wx * wx + wy * wy;
    let nok = |s: f64| (h.x + Y.ux * jmin(Y.s_m, jmax(0.0, s)), h.y + Y.uy * jmin(Y.s_m, jmax(0.0, s)));
    let (mut vh, mut vp, mut t_r, mut s, mut once) = (v_h, v_p, t_r0, 0.0, true);
    for _ in 0..3 {
        let k = vp / vh;
        let A = 1.0 - k * k;
        let B = 2.0 * a + 2.0 * vp * vp * t_r / vh;
        let cq = w2 - vp * vp * t_r * t_r;
        let s_min = t_r * vh;
        let mut r: Option<f64> = None;
        if A.abs() < 1e-9 {
            if B != 0.0 && -cq / B >= s_min {
                r = Some(-cq / B);
            }
        } else {
            let D = B * B - 4.0 * A * cq;
            if D >= 0.0 {
                let mut x = [(-B - D.sqrt()) / (2.0 * A), (-B + D.sqrt()) / (2.0 * A)];
                if x[0] > x[1] {
                    x.swap(0, 1);
                }
                for xx in x {
                    if xx >= s_min && r.is_none() {
                        r = Some(xx);
                    }
                }
            }
        }
        once = matches!(r, Some(rr) if rr <= Y.s_m);
        s = if once { r.unwrap() } else if k < 1.0 { (jmax(0.0, w2 - a * a)).sqrt() * k / (1.0 - k * k).sqrt() - a } else { Y.s_m };
        s = jmin(Y.s_m, jmax(0.0, s));
        let P = nok(s);
        let d = hypot(P.0 - p.x, P.1 - p.y);
        vh = if s > 0.0 { s / cx.KT(true, s, v_h) } else { v_h };
        vp = if d > 0.0 { d / cx.KT(false, d, v_p) } else { v_p };
        t_r = t_r0 + jmax(0.0, gec_of(&cx.g_p, P.0, P.1, Some(d))) - jmax(0.0, gec_of(&cx.g_h, P.0, P.1, Some(s)));
    }
    let P = nok(s);
    Kesis { x: P.0, y: P.1, s, once }
}

/// JS presKesis'in hesabı (yönlerin tehlikesi verilmiş): en tehlikeli kaçışı kapatan kesişme.
pub fn pres_kesis_t(p: &Oyuncu, h: &Oyuncu, yol: &[Yon], cx: &YolBag, tehl: &[f64]) -> Option<Kesis> {
    let (v_h, v_p, t_r) = (cx.v_sh, cx.v_p, cx.t_r0);
    let K: Vec<Kesis> = yol.iter().map(|Y| yol_kesis(p, h, Y, cx)).collect();
    let mut best: Option<Kesis> = None;
    let (mut bv, mut bt) = (f64::INFINITY, f64::INFINITY);
    for (xi, X) in K.iter().enumerate() {
        let dX = hypot(X.x - p.x, X.y - p.y);
        let t_x = t_r + dX / v_p;
        let mut kal = f64::NEG_INFINITY;
        for (j, Z) in yol.iter().enumerate() {
            let qa = jmin(Z.s_m, jmax(0.0, (X.x - h.x) * Z.ux + (X.y - h.y) * Z.uy));
            let Qx = h.x + Z.ux * qa;
            let Qy = h.y + Z.uy * qa;
            let kap = if j == xi { X.once } else { t_x + hypot(Qx - X.x, Qy - X.y) / v_p <= qa / v_h };
            if !kap {
                kal = jmax(kal, tehl[j]);
            }
        }
        if kal < bv || (kal == bv && t_x < bt) {
            bv = kal;
            bt = t_x;
            best = Some(*X);
        }
    }
    best
}

/// JS presKesis(p, h, dt, m).
pub fn pres_kesis(p: &Oyuncu, h: &Oyuncu, dt: u8, k: &dyn Kural, d: &KafaD, dtik: Option<f64>) -> Option<Kesis> {
    let yol = tas_hamle(h.x, h.y, h.vx, h.vy, dt);
    if yol.is_empty() {
        return Some(Kesis { x: h.x, y: h.y, s: 0.0, once: false });
    }
    let att = h.takim();
    let cx = yol_bag(p, h, k, d, dtik);
    let tehl: Vec<f64> = yol.iter().map(|Y| dis::threat(att, h.x + Y.ux * Y.s_m, h.y + Y.uy * Y.s_m)).collect();
    pres_kesis_t(p, h, &yol, &cx, &tehl)
}
