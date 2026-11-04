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
    let gx = ownX(t);
    let dx = gx - ax;
    let dy = GH / 2.0 - ay;
    let l = hypot(dx, dy);
    let l = if l != 0.0 && l == l { l } else { 1.0 };
    (ax + dx / l * off, ay + dy / l * off)
}

/// Slot (JS `att && p.rslot ? p.rslot : p.slot`): (d, l).
pub fn slotOf(p: &Oyuncu, att: bool) -> (f64, f64) {
    match (&p.rslot, att) {
        (Some(r), true) => (r.d, r.l),
        _ => (p.slot.d, p.slot.l),
    }
}
/// JS `(q.rslot || q.slot)`.
pub fn rslotOf(p: &Oyuncu) -> (f64, f64) {
    match &p.rslot {
        Some(r) => (r.d, r.l),
        None => (p.slot.d, p.slot.l),
    }
}

/// JS slotBase0(m, p, E, att): takımın bloğunda slotun yeri. `sl`: slot verilirse (JS `{...p, slot, rslot}`).
pub fn slotBase0(m: &Mac, ps: &[Oyuncu], p: &Oyuncu, sl: Option<(f64, f64)>, ex: f64, ey: f64, att: bool) -> (f64, f64) {
    let t = p.takim();
    let tac = &m.tac[t as usize];
    let s = sl.unwrap_or_else(|| slotOf(p, att));
    let x0 = ownX(t);
    let dir = dirOf(t);
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
        let front = jmin(94.0 * GKX, jmax(back + 24.0 * GKX, last + 1.0));
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
pub fn durusDef(m: &Mac, ps: &[Oyuncu], p: &Oyuncu, k: usize, att: bool) -> (f64, f64) {
    let t = p.takim();
    slotBase0(m, ps, p, None, ownX(t) + dirOf(t) * DUR_A[k], GCY, att)
}

/// JS durusW(f): topun derinliğine göre üç dayanağın payı.
pub fn durusW(f: f64) -> [f64; 3] {
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
pub fn slotBase(m: &Mac, ps: &[Oyuncu], p: &Oyuncu, ex: f64, ey: f64, att: bool) -> Yer {
    let b = slotBase0(m, ps, p, None, ex, ey, att);
    let t = p.takim();
    let D = match m.tac.get(t as usize).and_then(|q| q.durus.as_ref()) {
        Some(d) => d,
        None => return Yer { x: b.0, y: b.1, el: None },
    };
    let pl = if att { D.huc.iter().map(|v| v.iter().map(|q| (q.f, q.y)).collect::<Vec<_>>()).collect::<Vec<_>>() } else { D.sav.iter().map(|v| v.iter().map(|q| (q.f, q.y)).collect::<Vec<_>>()).collect::<Vec<_>>() };
    let x0 = ownX(t);
    let dir = dirOf(t);
    let w = durusW((ex - x0) * dir);
    let (mut ox, mut oy, mut el) = (0.0, 0.0, 0.0);
    for k in 0..3 {
        if w[k] == 0.0 || w[k] != w[k] {
            continue;
        }
        let q = match pl.get(k).and_then(|v| v.get((p.i - 1.0) as usize)) {
            Some(q) if p.i >= 1.0 => *q,
            _ => continue,
        };
        if q.0.is_some() || q.1.is_some() {
            el += w[k];
        }
        let d0 = durusDef(m, ps, p, k, att);
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
    pub holder: Option<&'a Oyuncu>,
    /// JS `q === p` için Bekçi'nin sırası
    pub ben: Option<Pid>,
}

/// JS bekci(p, E, m): Bekçi'nin yeri (açıyı daraltır, uzaktan topa destek, karşı karşıyada çıkış).
/// Taşıyıcı rakipse Bekçi topun şimdiki yerine değil, kendisi yerine varana kadar (tepki + yol / hız) taşıyıcının
/// gideceği yere göre yerleşir: yoksa vardığında açı çoktan kaymış olur, kapattığı açı yanlış açıdır.
pub fn bekci(p: &Oyuncu, ex: f64, ey: f64, m: Option<&BekM>, k: &dyn Kural) -> (f64, f64) {
    let g0 = bekci_an(p, ex, ey, m, k);
    let hh = match m.and_then(|m| m.holder).filter(|hh| hh.team != p.team) {
        Some(h) => h,
        None => return g0,
    };
    let hv = hypot(hh.vx, hh.vy);
    if !(hv > 1e-9) {
        return g0;
    }
    let sp = jmax(1e-6, crate::mac::enerji::spd(p));
    let mut t = bek_re(p) + hypot(g0.0 - p.x, g0.1 - p.y) / sp;
    // taşıyıcı kale çizgisini geçemez: öngörü çizgide kesilir
    let gx = ownX(p.takim());
    let (dx0, dx1) = (ex - gx, ex + hh.vx * t - gx);
    if dx0 * dx1 <= 0.0 && hh.vx.abs() > 1e-9 {
        t = jmax(0.0, (gx - ex) / hh.vx);
    }
    bekci_an(p, ex + hh.vx * t, ey + hh.vy * t, m, k)
}

fn bekci_an(p: &Oyuncu, ex: f64, ey: f64, m: Option<&BekM>, k: &dyn Kural) -> (f64, f64) {
    let gx = ownX(p.takim());
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
    let mut r = if let Some(vb) = m.and_then(|m| m.holder).filter(|hh| hh.team != p.team).and_then(dis::sutHiz) {
        bek_derinlik(p, ex, ey, gx, L, bxu, byu, vb)
    } else {
        let cov = Q.bek_body + 0.8 * bek_dl(p);
        let MoE = jmax(0.4, MOUTH * dx.abs() / L);
        cl(L * cl(1.0 - cov / MoE, 0.1, 0.55) + 0.08 * (bekY(p, mek::ACI_DARALT) - 10.0), 2.5, KR)
    };
    if let Some(m) = m {
        if L > S.bekSupL {
            let mut dD = 1e9;
            for qi in 0..m.ps.len() {
                let q = m.ps.get(qi);
                if q.team == p.team && !q.bekci() {
                    dD = jmin(dD, hypot(q.x - gx, q.y - GCY));
                }
            }
            let rs = jmin(jmin(S.bekSupMax + 0.3 * (bekY(p, mek::ACI_DARALT) - 10.0), dD - S.bekSupArka), KR + (L - S.bekSupL) * S.bekSupK);
            if rs > r {
                r = rs;
            }
        }
    }
    if let Some(m) = m {
        if let Some(hh) = m.holder {
            if hh.team != p.team && L < 30.0 {
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
                let kk = bekY(p, mek::KARSI_KARSIYA);
                let yakin = L < KR + 6.0;
                let rOut = cl(
                    if yakin { L - 1.2 - 0.05 * (20.0 - kk) } else { jmax(L * (0.35 + 0.01 * kk), L - (6.0 - 0.15 * kk)) },
                    2.5,
                    jmax(2.5, jmin(KR + 4.0, L - if yakin { 1.2 } else { 2.5 })),
                );
                /* (rY: taşıyıcının şut hazırlığına göre çıkış yarıçapı — artık kullanılmıyor, hesaplanmaz) */
                if yakin && rOut > r {
                    r += (rOut - r) * w;
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
    (ex + bx * st, ey + by * st)
}

/// Bekçi'nin derinliği değerle (dalış modeliyle aynı hesap): açıortay üstündeki aday derinliklerde, taşıyıcının kale
/// ağzına (direkten direğe, gövde aralığıyla) çekebileceği en sert şutlar tartılır: Bekçi tepki süresince yerinde,
/// sonra dalış hızıyla en çok tam uzanma kadar; top çizgiye varmadan gövdesi topun yoluna değebiliyorsa o şut kapanır.
/// Kale ağzının en büyük kapanan payı seçilir; eşitlikte Bekçi'nin şimdiki derinliğine en yakın aday (titreme yok).
/// Adaylar Bekçi'nin bir tikte aldığı yol aralığıyla, 2,5 br'den Kuyu sınırına (eski sınırlar).
fn bek_derinlik(p: &Oyuncu, ex: f64, ey: f64, gx: f64, L: f64, bxu: f64, byu: f64, vb: f64) -> f64 {
    let (re, dv, dl) = (bek_re(p), bek_dv(p), bek_dl(p));
    let vb = jmax(vb, 1e-6);
    let adim = jmax(0.05, crate::mac::enerji::spd(p));
    let r_simdi = hypot(p.x - gx, p.y - GCY);
    // saf hesap: girdileri bit bit aynıysa sonuç aynı (L, bxu, byu ex, ey, gx'ten çıkar)
    let anahtar = [re, dv, dl, adim, r_simdi, ex, ey, gx, vb].map(f64::to_bits);
    if let Some(v) = BEK_DER.with(|c| c.borrow().get(&anahtar).copied()) {
        BEK_DER_SAY.with(|c| c.set((c.get().0 + 1, c.get().1)));
        return v;
    }
    BEK_DER_SAY.with(|c| c.set((c.get().0, c.get().1 + 1)));
    let v = bek_derinlik0(ex, ey, gx, L, bxu, byu, vb, re, dv, dl, adim, r_simdi);
    BEK_DER.with(|c| {
        let mut c = c.borrow_mut();
        if c.len() > 200_000 {
            c.clear();
        }
        c.insert(anahtar, v);
    });
    v
}
thread_local! {
    static BEK_DER: std::cell::RefCell<crate::HizliMap<[u64; 9], f64>> = std::cell::RefCell::new(Default::default());
    pub static BEK_DER_SAY: std::cell::Cell<(u64, u64)> = const { std::cell::Cell::new((0, 0)) };
}
#[allow(clippy::too_many_arguments)]
fn bek_derinlik0(ex: f64, ey: f64, gx: f64, L: f64, bxu: f64, byu: f64, vb: f64, re: f64, dv: f64, dl: f64, adim: f64, r_simdi: f64) -> f64 {
    let body = Q.bek_body;
    let n = (2.0 * MOUTH / body).ceil().max(1.0) as usize;
    // şut yollarının noktaları ve o anki erişim aday derinlikten bağımsız: bir kez (sonuç aynı)
    let mut yol: Vec<Vec<(f64, f64, f64)>> = Vec::with_capacity(n + 1);
    for j in 0..=n {
        let ty = GCY - MOUTH + 2.0 * MOUTH * j as f64 / n as f64;
        let (ux, uy) = (gx - ex, ty - ey);
        let ld = hypot(ux, uy);
        if !(ld > 0.0) {
            continue;
        }
        let (ux, uy) = (ux / ld, uy / ld);
        let mut v = Vec::new();
        let mut s = 0.0;
        loop {
            let t = s / vb;
            let er = body + jmin(dl, dv * jmax(0.0, t - re));
            v.push((ex + ux * s, ey + uy * s, er));
            if s >= ld {
                break;
            }
            s = jmin(ld, s + vb);
        }
        yol.push(v);
    }
    let kap = |r: f64| -> usize {
        let st = jmax(0.0, L - r);
        let (kx, ky) = (ex + bxu * st, ey + byu * st);
        let mut c = 0;
        for v in &yol {
            for &(px, py, er) in v {
                let (ax, ay) = (px - kx, py - ky);
                // hypot(ax, ay) >= |ax|, |ay| (IEEE): bunlardan biri er'i aşıyorsa uzak, hypot gereksiz
                if ax.abs() > er || ay.abs() > er {
                    continue;
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
    while r <= r_maks + 1e-9 {
        let k = kap(r);
        let d = (r - r_simdi).abs();
        if k > ek || (k == ek && d < ed) {
            eniyi = r;
            ek = k;
            ed = d;
        }
        r += adim;
    }
    eniyi
}

/// JS koyHedef(p, h, ax, ay): taşıyıcının önüne gövde koyma noktası.
pub fn koyHedef(p: &Oyuncu, h: &Oyuncu, ax: f64, ay: f64) -> (f64, f64) {
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
    pub sM: f64,
}

/// JS tasHamle(h, dt): taşıyıcının gidebileceği yönler (kale, hızı, duvar boyu).
pub fn tasHamle(hx: f64, hy: f64, hvx: f64, hvy: f64, dt: u8) -> Vec<Yon> {
    let ox = ownX(dt);
    let mut yon: Vec<Yon> = Vec::new();
    let mut ek = |ad: &'static str, ux: f64, uy: f64| {
        let l = hypot(ux, uy);
        if l > 0.0 {
            yon.push(Yon { ad, ux: ux / l, uy: uy / l, sM: 0.0 });
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
        let mut sM = f64::INFINITY;
        if Y.ux > 0.0 {
            sM = jmin(sM, (GW - hx) / Y.ux);
        }
        if Y.ux < 0.0 {
            sM = jmin(sM, -hx / Y.ux);
        }
        if Y.uy > 0.0 {
            sM = jmin(sM, (GH - hy) / Y.uy);
        }
        if Y.uy < 0.0 {
            sM = jmin(sM, -hy / Y.uy);
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
                    sM = jmin(sM, r);
                }
            }
        }
        Y.sM = jmax(0.0, sM);
    }
    yon
}

/// match.js ALAN_KOSU_TP(pr, d, v1): sprint profiliyle `d` yolun süresi (saf).
pub fn ALAN_KOSU_TP(pr: Option<dis::KosuPr>, d: f64, v1: f64) -> f64 {
    match pr {
        Some(pr) if d > 0.0 => {
            let dS = v1 * pr.Ts;
            if d <= dS {
                d / v1
            } else {
                pr.Ts + (d - dS) / (v1 * pr.k)
            }
        }
        _ => d / v1,
    }
}

/// JS yolBag(p, h) dönüşü: kesişmenin değişmeyenleri (hızlar, tepki, sprint profilleri, varış tabloları).
#[derive(Clone, Debug)]
pub struct YolBag {
    pub vH: f64,
    pub vP: f64,
    pub tR0: f64,
    pub prH: Option<dis::KosuPr>,
    pub prP: Option<dis::KosuPr>,
    pub vSh: f64,
    pub gH: GecTab,
    pub gP: GecTab,
}
impl YolBag {
    /// JS `KT(q, d, v1)`: `h` mı `p` mi.
    #[inline]
    pub fn KT(&self, hmi: bool, d: f64, v1: f64) -> f64 {
        ALAN_KOSU_TP(if hmi { self.prH } else { self.prP }, d, v1)
    }
}

/// yolBag/yolKesis'in taşıyıcı hız yanılgısı (görüşe bağlı, 15 tikte bir değişir).
pub fn hizYanilgi(p: &Oyuncu, h: &Oyuncu, d: &KafaD) -> f64 {
    let g = p.yet.ozk("gorus");
    let hs = sinh_k(12.9898 * (p.id + 1.0) + 4.1414 * (h.id + 1.0) + 78.233 * (super::D_TICK.with(|t| t.get()).or(d._tick).filter(|t| *t != 0.0 && *t == *t).unwrap_or(0.0) / 15.0).floor());
    1.0 + (kesir(hs) - 0.5) * (20.0 - g) * 0.02
}

/// JS yolBag(p, h): `p` (presçi) ile `h` (taşıyıcı) arasındaki yarışın değişmeyenleri.
pub fn yolBag(p: &Oyuncu, h: &Oyuncu, k: &dyn Kural, d: &KafaD) -> YolBag {
    let prH = dis::ALAN_KOSU_PR(h, k);
    let prP = dis::ALAN_KOSU_PR(p, k);
    let vSh = vSp(h, k);
    let vSq = vSp(p, k);
    YolBag {
        vH: vSh * dis::topK(h) * hizYanilgi(p, h, d),
        vP: vSq,
        tR0: jmax(0.0, TEPKI(p, Tepki::Kovala, k)),
        prH,
        prP,
        vSh,
        gH: gecTabV(h, Some(vSh), k),
        gP: gecTabV(p, Some(vSq), k),
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
pub fn yolKesis(p: &Oyuncu, h: &Oyuncu, Y: &Yon, cx: &YolBag) -> Kesis {
    let (vH, vP, tR0) = (cx.vH, cx.vP, cx.tR0);
    let wx = h.x - p.x;
    let wy = h.y - p.y;
    let a = wx * Y.ux + wy * Y.uy;
    let w2 = wx * wx + wy * wy;
    let nok = |s: f64| (h.x + Y.ux * jmin(Y.sM, jmax(0.0, s)), h.y + Y.uy * jmin(Y.sM, jmax(0.0, s)));
    let (mut vh, mut vp, mut tR, mut s, mut once) = (vH, vP, tR0, 0.0, true);
    for _ in 0..3 {
        let k = vp / vh;
        let A = 1.0 - k * k;
        let B = 2.0 * a + 2.0 * vp * vp * tR / vh;
        let Cq = w2 - vp * vp * tR * tR;
        let sMin = tR * vh;
        let mut r: Option<f64> = None;
        if A.abs() < 1e-9 {
            if B != 0.0 && -Cq / B >= sMin {
                r = Some(-Cq / B);
            }
        } else {
            let D = B * B - 4.0 * A * Cq;
            if D >= 0.0 {
                let mut x = [(-B - D.sqrt()) / (2.0 * A), (-B + D.sqrt()) / (2.0 * A)];
                if x[0] > x[1] {
                    x.swap(0, 1);
                }
                for xx in x {
                    if xx >= sMin && r.is_none() {
                        r = Some(xx);
                    }
                }
            }
        }
        once = matches!(r, Some(rr) if rr <= Y.sM);
        s = if once { r.unwrap() } else if k < 1.0 { (jmax(0.0, w2 - a * a)).sqrt() * k / (1.0 - k * k).sqrt() - a } else { Y.sM };
        s = jmin(Y.sM, jmax(0.0, s));
        let P = nok(s);
        let d = hypot(P.0 - p.x, P.1 - p.y);
        vh = if s > 0.0 { s / cx.KT(true, s, vH) } else { vH };
        vp = if d > 0.0 { d / cx.KT(false, d, vP) } else { vP };
        tR = tR0 + jmax(0.0, gecOf(&cx.gP, P.0, P.1, Some(d))) - jmax(0.0, gecOf(&cx.gH, P.0, P.1, Some(s)));
    }
    let P = nok(s);
    Kesis { x: P.0, y: P.1, s, once }
}

/// JS presKesis'in hesabı (yönlerin tehlikesi verilmiş): en tehlikeli kaçışı kapatan kesişme.
pub fn presKesisT(p: &Oyuncu, h: &Oyuncu, yol: &[Yon], cx: &YolBag, tehl: &[f64]) -> Option<Kesis> {
    let (vH, vP, tR) = (cx.vSh, cx.vP, cx.tR0);
    let K: Vec<Kesis> = yol.iter().map(|Y| yolKesis(p, h, Y, cx)).collect();
    let mut best: Option<Kesis> = None;
    let (mut bv, mut bt) = (f64::INFINITY, f64::INFINITY);
    for (xi, X) in K.iter().enumerate() {
        let dX = hypot(X.x - p.x, X.y - p.y);
        let tX = tR + dX / vP;
        let mut kal = f64::NEG_INFINITY;
        for (j, Z) in yol.iter().enumerate() {
            let qa = jmin(Z.sM, jmax(0.0, (X.x - h.x) * Z.ux + (X.y - h.y) * Z.uy));
            let Qx = h.x + Z.ux * qa;
            let Qy = h.y + Z.uy * qa;
            let kap = if j == xi { X.once } else { tX + hypot(Qx - X.x, Qy - X.y) / vP <= qa / vH };
            if !kap {
                kal = jmax(kal, tehl[j]);
            }
        }
        if kal < bv || (kal == bv && tX < bt) {
            bv = kal;
            bt = tX;
            best = Some(*X);
        }
    }
    best
}

/// JS presKesis(p, h, dt, m).
pub fn presKesis(p: &Oyuncu, h: &Oyuncu, dt: u8, k: &dyn Kural, d: &KafaD) -> Option<Kesis> {
    let yol = tasHamle(h.x, h.y, h.vx, h.vy, dt);
    if yol.is_empty() {
        return Some(Kesis { x: h.x, y: h.y, s: 0.0, once: false });
    }
    let att = h.takim();
    let cx = yolBag(p, h, k, d);
    let tehl: Vec<f64> = yol.iter().map(|Y| dis::threat(att, h.x + Y.ux * Y.sM, h.y + Y.uy * Y.sM)).collect();
    presKesisT(p, h, &yol, &cx, &tehl)
}
