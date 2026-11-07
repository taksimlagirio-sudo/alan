//! Topla temas (core.js: touchCap, vurusOku, okuT, ucK, oppReach, menzil, ctrlP, contact, thruP, stepAll,
//! Bekçi: bekDV, bekDL, bekKay, bekOku, bekOkuE, bekRe, bekHar, bekR; Kuyu: kuyuMer, alanIc, kuyuGirer;
//! gövde: govde, govdeIt).

use crate::alan::{k as alan_k, Kaynak};
use crate::core_sabit::{IN, Q};
use crate::mat::{atan2, cos, exp, hypot, jmax, jmin, jround, kis, log, sin, acos, PI};
use crate::oyuncu::{tepki, Oyuncu, Tepki, TepkiS};
use crate::saha::*;
use crate::top::{adim, hyp, saha_ici, Top, TopOlay};
use crate::yetenek::{denge_k, mek, yum, zar, DK_ALIS};

/// Maçın çekirdeğe verdikleri: sprint katı (maç sprint evresiyle sarar), ikilinin `yavas` eşiği.
pub trait Kural {
    fn spr(&self, p: &Oyuncu) -> f64;
    fn ik_yavas(&self) -> f64 {
        0.35
    }
    fn tepki_s(&self) -> TepkiS {
        TepkiS::default()
    }
    /// match.js kosuPr(p): koşu planının (Ts, k) değeri (maç kuralı verir).
    fn kosu_pr(&self, _p: &Oyuncu) -> Option<(f64, f64)> {
        unimplemented!("Kural::kosu_pr (yalnız maç kuralında)")
    }
    /// core.js ALAN_EN3_KUL(p, e, bizde, B): enerjinin kullanım değeri (maç kuralı verir).
    fn en3_kul(&self, _p: &Oyuncu, _e: f64, _bizde: bool, _b: Option<(f64, f64)>) -> f64 {
        unimplemented!("Kural::en3_kul (yalnız maç kuralında)")
    }
}

/// Çekirdeğin kendi kuralı (sprint evresi yok).
pub struct CekirdekKural;
impl Kural for CekirdekKural {
    fn spr(&self, p: &Oyuncu) -> f64 {
        p.spr0()
    }
}

/// Topun temasla ilgili hâli (JS'te topun üstündeki alanlar).
#[derive(Clone, Debug, Default)]
pub struct TopTemas {
    /// topun kimliği (JS'te nesnenin kendisi; `_rcvB === b`)
    pub kimlik: u64,
    /// vuruş anı (kt) ve vuruşun zarı (kz)
    pub kt: Option<f64>,
    pub kz: f64,
    /// gönderme (şut) mu, Bekçi'ye mi (kBek)
    pub gon: bool,
    pub k_bek: bool,
    pub from: Option<usize>,
    pub recv: Option<usize>,
    /// vuruşun okunabilirliği
    pub cue: f64,
    /// okuma önbelleği (JS b._oku)
    pub oku: crate::HizliMap<i64, (f64, f64)>,
    /// yeniden dokunma beklemesi: oyuncu → tik (JS b.cds)
    pub cds: crate::HizliMap<usize, f64>,
    /// bek_cd: Bekçi'nin yeniden dokunma beklemesinin başladığı tik (bekleme topun o anki hızıyla yeniden hesaplanır)
    pub cds0: crate::HizliMap<usize, f64>,
    /// Bekçi'nin harekete geçirdiği top (bekId: oyuncu id'si, bekM)
    pub bek_id: Option<u32>,
    pub bek_m: f64,
    pub sap: bool,
    /// temas aranırken segmentin payı (JS _tt)
    pub tt: f64,
    /// kafadaki hayali top (Bekçi dalışı oyuncuyu yerinden oynatmaz)
    pub hayal: bool,
    pub dal: Option<Dal>,
    /// dalışın başladığı an (maçta bir kez kurulur): Bekçi'nin yeri ve dalış hızı
    pub dal_bas: Option<DalBas>,
    pub holder: Option<usize>,
    pub gvx: f64,
    pub gvy: f64,
}

/// Dalışın başlangıcı: Bekçi'nin tepki bittiği andaki yeri, dalış hızı (şimdiki hız + itki) ve süresi.
#[derive(Clone, Copy, Debug)]
pub struct DalBas {
    pub p: usize,
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub t0: f64,
    pub td: f64,
    /// itki (dalış bitince söner: Bekçi uçmaya devam etmez)
    pub ix: f64,
    pub iy: f64,
    /// gereken uzanmanın tam uzanmaya oranı (barın bedeli)
    pub f: f64,
}

#[derive(Clone, Copy, Debug)]
pub struct Dal {
    pub p: usize,
    pub x: f64,
    pub y: f64,
    pub f: f64,
}

/// Topun tamamı: fizik + temas.
#[derive(Clone, Debug)]
pub struct TopT {
    pub b: Top,
    pub t: TopTemas,
}

fn yas_t(b: &TopT) -> f64 {
    b.b.tp.unwrap_or(b.b.t) + b.t.tt
}

pub fn bek_dv(p: &Oyuncu) -> f64 {
    Q.bek_dive_v + 0.01 * (p.govde.yet.y(mek::DALIS) - 10.0)
}
pub fn bek_dl(p: &Oyuncu) -> f64 {
    Q.bek_dive * jmax(0.5, 1.0 + 0.007 * (p.govde.yet.y(mek::DALIS) - 10.0))
}
pub fn bek_kay(p: &Oyuncu) -> f64 {
    kis(0.5 + 0.015 * (p.govde.yet.y(mek::YANA_KAYMA) - 10.0), 0.4, 0.8)
}

fn sez_f(p: &Oyuncu, v: &Oyuncu) -> f64 {
    kis(0.5 + IN.sez * (p.govde.yet.y(mek::VURUS_SEZ) - 10.0) - IN.giz * (v.govde.yet.y(mek::VURUS_GIZLE) - 10.0), 0.05, 0.95)
}

/// Bekçi vuruşu okur (JS bekOku): tepkinin zar katı ve vuruşun okunduğu öne alma.
pub fn bek_oku(b: &mut TopT, pi: usize, ps: &[Oyuncu]) -> Option<(f64, f64)> {
    let p = &ps[pi];
    let from = b.t.from?;
    if b.t.kt.is_none() || !b.t.gon || b.b.defl != 0.0 || b.b.walls != 0.0 || !p.bekci() || p.takim() == b.b.team {
        return None;
    }
    let bek = b.t.k_bek;
    let a = 100.0 * p.team + p.id;
    let ck = ((if bek { -3e3 } else { -2e3 }) - a) as i64;
    if let Some(r) = b.t.oku.get(&ck) {
        return Some(*r);
    }
    let k = b.t.kt.unwrap() + 1.618 * b.t.kz;
    let z = if bek { 0.0 } else { zar(a, k, 21.0) + zar(a, k, 23.0) - 1.0 };
    let f = sez_f(p, &ps[from]);
    let u = (IN.u_a + IN.u_b) / 2.0;
    let r = (1.0 + IN.tepki_z * z * 2.0 * (1.0 - f), b.t.cue * (f * u - 0.5)); /* v5 · okuma zarının genişliği okumayla: Bekçi'nin vuruşu sezmesi şutçunun gizlemesinden iyiyse (f > 0,5) tepkisi daha tutarlı, kötüyse daha dağınık; eşitken eskisi gibi. Ortalama değişmez (zar ortalaması 0) */
    b.t.oku.insert(ck, r);
    Some(r)
}

pub fn bek_oku_e(p: &Oyuncu, v: &Oyuncu, cue: f64) -> f64 {
    let f = sez_f(p, v);
    cue * (f * (IN.u_a + IN.u_b) / 2.0 - 0.5)
}

pub fn bek_re(p: &Oyuncu) -> f64 {
    let ok = p.govde.yet.y(mek::BEK_REFLEKS);
    Q.bek_tepki * (1.0 - (ok - 10.0) * Q.bek_tepki_ok)
}

/// Bekçi'nin şimdiki koşusunun dalışa katkısı (JS bekHar).
pub fn bek_har(p: &Oyuncu, x: f64, y: f64, vx: f64, vy: f64, k: &dyn Kural) -> f64 {
    let sp = hyp(p.vx, p.vy);
    let s = jmin(1.0, sp / (p.taban_hiz() * k.spr(p)));
    if s <= 1e-6 {
        return 0.0;
    }
    let l2 = vx * vx + vy * vy;
    let t = if l2 > 1e-9 { jmax(0.0, ((p.x - x) * vx + (p.y - y) * vy) / l2) } else { 0.0 };
    let nx = x + vx * t - p.x;
    let ny = y + vy * t - p.y;
    let nl = hyp(nx, ny);
    let c = if nl > 0.3 { (p.vx * nx + p.vy * ny) / (sp * nl) } else { 0.0 };
    s * jmax(0.0, -c)
} /* v5 · hareket cezası yalnız dalış yönüne ters harekete: topun yoluna koşan (açı daraltan) ve yana kayan Bekçi dalışa hazırdır, ceza almaz; yoldan uzaklaşan tam ceza. Yer değiştirmenin kendisi maçta zaten Bekçi'nin konumunda. Önce yana kayma yarım cezaydı: açıyı izleyen Bekçi şutların neredeyse hepsinde yana tam hızla kayıyor, tepkisine 1–2 tik ekleniyor, 13–19 br'den sert şutların hepsi gol oluyordu (283884: 10 şutta 9 gol). */

/// Bekçi'nin menzili (JS bekR): gövde + dalış (topun yaşı ve tepkisiyle).
pub fn bek_r(b: &mut TopT, pi: usize, ps: &[Oyuncu]) -> f64 {
    let p = &ps[pi];
    let hm = if b.b.defl == 0.0 && b.t.bek_id == Some(p.id as u32) && b.t.bek_m > 0.0 { b.t.bek_m } else { 0.0 };
    let o = bek_oku(b, pi, ps);
    let re = (match o {
        Some((k, on)) => bek_re(p) * k - on,
        None => bek_re(p),
    }) + Q.bek_har_t * hm;
    Q.bek_body + jmin(bek_dl(p) * (1.0 - Q.bek_har_d * hm), jmax(0.0, yas_t(b) - re) * bek_dv(p))
}

/// Bekçi'nin dalışı yeni modelde mi tartılır: rakip şutu (gönder), sekmemiş, Bekçi şutun Bekçisi.
fn dalis_modeli(b: &TopT, pi: usize, ps: &[Oyuncu]) -> bool {
    let p = &ps[pi];
    p.bekci() && b.t.gon && b.b.defl == 0.0 && b.b.walls == 0.0 && p.takim() != b.b.team && b.t.bek_id == Some(p.id as u32)
}

/// Bekçi'nin dalışı (hareket): tepki süresince eski hareketini sürdürür; tepki bitince topun yolundaki en yakın
/// noktaya doğru itki alır: dalış hızı = şimdiki hız + itki (büyüklüğü dalış hızı, bek_dv), süresi gereken uzanma / dalış hızı, en çok tam uzanma
/// (duran Bekçi eski modeldeki kadar uzanır). Ters yöne koşan Bekçi'nin hızı itkinin bir kısmını yer, açı daraltan ve
/// topun tarafına kayanınki ekler: hareket cezası kural değil, hareketin kendisi. Döner: (x, y, vx, vy) bu andaki yer ve hız
/// (tepki sürüyorsa None: Bekçi kendi hareketinde). Maç ve kafa aynı işlevi kullanır.
pub fn bek_dalis(b: &mut TopT, pi: usize, ps: &[Oyuncu]) -> Option<(f64, f64, f64, f64)> {
    if !dalis_modeli(b, pi, ps) {
        return None;
    }
    let p = &ps[pi];
    let oku = bek_oku(b, pi, ps);
    let re = match oku { Some((k, on)) => bek_re(p) * k - on, None => bek_re(p) };
    let tau = yas_t(b);
    if tau <= re {
        return None;
    }
    let bas = match b.t.dal_bas.filter(|d| d.p == pi) {
        Some(d) => d,
        None => {
            // tepki bitti: dalışın başı. Bekçi topun yolunu bu anki uçuşundan okur ve topla aynı anda varabileceği
            // ilk noktaya dalar: |B + û·s − K| = gövde + (dalış hızı / top hızı)·s (ikinci derece); yoksa en az geç kaldığı nokta.
            let (bx, by, bvx, bvy) = (b.b.x, b.b.y, b.b.vx, b.b.vy);
            let vb = hyp(bvx, bvy);
            let e = p.en.map_or(1.0, |e| kis(e, 0.0, 1.0));
            let dv = bek_dv(p) * (crate::core_sabit::EN3.tepe_alt + (1.0 - crate::core_sabit::EN3.tepe_alt) * e); /* itki barla (tepe hızının bar kuralıyla aynı ölçü): barı düşük Bekçi daha yavaş uzanır */
            let dl = bek_dl(p);
            let (ux, uy) = if vb > 1e-9 { (bvx / vb, bvy / vb) } else { (0.0, 0.0) };
            let (wx, wy) = (bx - p.x, by - p.y);
            let a = wx * ux + wy * uy;
            let w2 = wx * wx + wy * wy;
            let kk = if vb > 1e-9 { dv / vb } else { 0.0 };
            let body = Q.bek_body;
            let s_kes = {
                let qa = 1.0 - kk * kk;
                let qb = 2.0 * a - 2.0 * body * kk;
                let qc = w2 - body * body;
                let mut r: Option<f64> = None;
                if qc <= 0.0 {
                    r = Some(0.0);
                } else if qa.abs() < 1e-12 {
                    if qb < 0.0 { r = Some(-qc / qb); }
                } else {
                    let dd = qb * qb - 4.0 * qa * qc;
                    if dd >= 0.0 {
                        let sq = dd.sqrt();
                        let (r1, r2) = ((-qb - sq) / (2.0 * qa), (-qb + sq) / (2.0 * qa));
                        let (lo, hi) = if r1 < r2 { (r1, r2) } else { (r2, r1) };
                        if lo >= 0.0 { r = Some(lo); } else if hi >= 0.0 { r = Some(hi); }
                    }
                }
                // en az yolla uzanma: topun yolunun Bekçi'ye en yakın noktası (dik ayak) zamanında yetişilebiliyorsa oraya
                // (gerçek Bekçi topa doğru atılmaz, yolun önüne en kısa yoldan uzanır); yetişilemiyorsa yetişilen en yakın
                // geç nokta (ikinci kök), o da yoksa en az geç kalınan nokta
                let ayak = (-a).max(0.0);
                let ayak_d = (w2 + 2.0 * a * ayak + ayak * ayak).max(0.0).sqrt();
                let yeter = ayak_d <= body + kk * ayak;
                let r = if yeter { Some(ayak) } else if qc > 0.0 && qa.abs() >= 1e-12 {
                    let dd = qb * qb - 4.0 * qa * qc;
                    if dd >= 0.0 { let sq = dd.sqrt(); let h = ((-qb - sq) / (2.0 * qa)).max((-qb + sq) / (2.0 * qa)); if h >= ayak { Some(h) } else { r } } else { r }
                } else { r };
                match r {
                    Some(v) => v,
                    // yetişemiyor: en az geç kalınan nokta (|w + ûs| − k·s en küçük)
                    None => {
                        let bb = (w2 - a * a).max(0.0).sqrt();
                        if kk < 1.0 { (kk * bb / (1.0 - kk * kk).sqrt() - a).max(0.0) } else { (-a).max(0.0) }
                    }
                }
            };
            let (nx, ny) = (bx + ux * s_kes - p.x, by + uy * s_kes - p.y);
            let nl = hyp(nx, ny);
            let (mut nx, mut ny) = (nx, ny);
            // yanlış yöne dalış: Bekçi vuruşu önceden okuyup öne aldıysa (on > 0), şutçunun gizlemesi sezmesinden iyi
            // olduğu ölçüde (1 − f) yanlış köşeyi okur: dalış yönü şutçuyla Bekçi'yi birleştiren eksene göre aynalanır.
            if let (Some((_, on)), Some(fi)) = (oku, b.t.from) {
                if on > 1e-9 && nl > body { /* öne alma gerçekten var (sezme ile gizleme eşitken kayan nokta artığı sayılmaz) */
                    let f = sez_f(p, &ps[fi]);
                    let a0 = 100.0 * p.team + p.id;
                    let k0 = b.t.kt.unwrap_or(0.0) + 1.618 * b.t.kz;
                    if zar(a0, k0, 31.0) < 1.0 - f {
                        let (ax, ay) = (ps[fi].x - p.x, ps[fi].y - p.y);
                        let al = hyp(ax, ay);
                        if al > 1e-9 {
                            let (ax, ay) = (ax / al, ay / al);
                            let dt = nx * ax + ny * ay;
                            nx = 2.0 * dt * ax - nx;
                            ny = 2.0 * dt * ay - ny;
                        }
                    }
                }
            }
            let (ix, iy) = if nl > body { (nx / nl * dv, ny / nl * dv) } else { (0.0, 0.0) };
            let d = DalBas { p: pi, x: p.x, y: p.y, vx: p.vx + ix, vy: p.vy + iy, t0: re, td: if nl > Q.bek_body { jmin(dl, nl - Q.bek_body) / dv } else { 0.0 }, ix, iy, /* dalış topun yoluna varınca biter (gövde yola değer); en çok tam uzanma */ f: jmin(1.0, jmax(0.0, nl - Q.bek_body) / jmax(0.5, dl)) };
            if !b.t.hayal {
                b.t.dal_bas = Some(d);
            }
            d
        }
    };
    let u = jmin(tau - bas.t0, bas.td);
    let bitti = tau - bas.t0 >= bas.td;
    let (vx, vy) = if bitti { (bas.vx - bas.ix, bas.vy - bas.iy) } else { (bas.vx, bas.vy) };
    Some((bas.x + bas.vx * u, bas.y + bas.vy * u, vx, vy))
}

/// Alma kapasitesi (JS touchCap).
pub struct Kap {
    pub cap: f64,
    pub vr: f64,
    pub vrx: f64,
    pub vry: f64,
    pub s: f64,
    pub sk: f64,
}

/// ALAN_DURUM.har: oyuncunun hızının tepe hızına oranı.
fn har(p: &Oyuncu) -> f64 {
    jmin(1.0, hypot(p.vx, p.vy) / (p.taban_hiz() * p.spr0()))
}

pub fn touch_cap(b: &mut TopT, pi: usize, ps: &[Oyuncu], k: &dyn Kural) -> Kap {
    let p = &ps[pi];
    let mate = p.takim() == b.b.team;
    let bek = p.bekci();
    let mut ark_k = 0.0;
    let mut sk = if bek { p.govde.yet.y(mek::TUT_CEL) } else if mate { p.govde.yet.y(mek::KONTROL_KAP) } else { p.govde.yet.y(mek::KESME_KAP) };
    let mut pr = 0.0;
    for o in ps {
        if o.team != p.team {
            pr += o.etki().infl(p.x, p.y);
        }
    }
    {
        let ps0 = if mate && !bek { hyp(p.vx, p.vy) } else { 0.0 };
        if ps0 > 0.03 {
            let rx = b.b.vx - p.vx;
            let ry = b.b.vy - p.vy;
            let rl = hyp(rx, ry);
            let ak = if rl > 1e-6 { jmax(0.0, (rx * p.vx + ry * p.vy) / (rl * ps0)) * jmin(1.0, ps0 / (p.taban_hiz() * k.spr(p))) } else { 0.0 };
            if ak > 0.0 {
                sk += IN.ark_w * ak * (p.govde.yet.ozk("ceviklik") - sk);
                ark_k = IN.ark_k * ak;
            }
        }
    }
    let o = vurus_oku(b, pi, ps, false);
    let ok = if !bek { p.govde.yet.y(mek::HAZIRLIK) } else { p.govde.yet.ozk("refleks") };
    let tau = jmax(1.0, Q.tepki0 - ok * Q.tepki_ok) * o.map(|x| x.0).unwrap_or(1.0);
    let haz = jmin(1.0, (b.b.tp.unwrap_or(b.b.t) + o.map(|x| x.1).unwrap_or(0.0)) / tau);
    let pr_k = 1.0 - exp(-jmax(0.0, pr - 0.4) * Q.press_k / Q.press_max);
    let ch_t = if b.b.alive { b.b.ch * Q.cap_ch } else { 0.0 };
    let cap = if !bek {
        let alis = DK_ALIS.b * yum(&p.govde.yet, "B") * pr_k + DK_ALIS.d * (1.0 - p.den.unwrap_or(1.0)) + if mate { DK_ALIS.h * har(p) * yum(&p.govde.yet, "H") } else { 0.0 };
        Q.cap0 + (sk - 10.0) * Q.cap_sk - ch_t - Q.cap_pr * alis - ark_k
    } else {
        Q.cap0 + (sk - 10.0) * Q.cap_sk - ch_t - Q.cap_pr * pr_k - ark_k
    } * (Q.cap_haz + (1.0 - Q.cap_haz) * haz);
    let vrx = b.b.vx - p.vx;
    let vry = b.b.vy - p.vy;
    Kap {
        cap: jmax(0.08, cap * if !bek || mate { 1.0 } else { Q.bek_el * jmax(0.2, 1.0 + 0.05 * (sk - 10.0)) }),
        vr: hyp(vrx, vry),
        vrx,
        vry,
        s: Q.cap_s0 + (20.0 - sk) * Q.cap_ssk,
        sk,
    }
}

/// Rakibin vuruşu okuması (JS vurusOku): tepki katı ve öne alma; Bekçi okumaz (bekOku).
pub fn vurus_oku(b: &mut TopT, pi: usize, ps: &[Oyuncu], bek0: bool) -> Option<(f64, f64)> {
    let p = &ps[pi];
    if b.t.kt.is_none() || b.b.defl != 0.0 || b.b.walls != 0.0 || p.takim() == b.b.team || p.bekci() {
        return None;
    }
    let bek = bek0 || b.t.k_bek;
    let a = 100.0 * p.team + p.id;
    let ck = if bek { (-1.0 - a) as i64 } else { a as i64 };
    if let Some(r) = b.t.oku.get(&ck) {
        return Some(*r);
    }
    let k = b.t.kt.unwrap() + 1.618 * b.t.kz;
    let z = if bek { 0.0 } else { zar(a, k, 21.0) + zar(a, k, 23.0) - 1.0 };
    let mut on = 0.0;
    if let Some(fi) = b.t.from {
        if b.t.cue > 0.0 {
            let f = sez_f(p, &ps[fi]);
            on = b.t.cue * f * if bek { (IN.u_a + IN.u_b) / 2.0 } else { IN.u_a + (IN.u_b - IN.u_a) * zar(a, k, 29.0) };
        }
    }
    let r = (1.0 + IN.tepki_z * z, on);
    b.t.oku.insert(ck, r);
    Some(r)
}

pub fn oku_t(b: &mut TopT, pi: usize, ps: &[Oyuncu], d: f64, bek: bool) -> f64 {
    match vurus_oku(b, pi, ps, bek) {
        Some((k, on)) => d * k - on,
        None => d,
    }
}

/// Uzanmanın ucunda alışın zorluğu (JS ucK).
pub fn uc_k(x: f64, p: Option<&Oyuncu>) -> f64 {
    if x > IN.uc_x0 {
        1.0 - IN.uc_k * (match p {
            Some(p) => kis(1.0 - IN.uc_y * (p.govde.yet.y(mek::UZANMA_UCU) - 10.0), 0.4, 1.6),
            None => 1.0,
        }) * jmin(1.0, (x - IN.uc_x0) / (1.0 - IN.uc_x0))
    } else {
        1.0
    }
}

/// Rakibin uzanması topun yaşıyla büyür (JS oppReach).
pub fn opp_reach(b: &mut TopT, pi: usize, ps: &[Oyuncu], r: f64, s: &TepkiS) -> f64 {
    let p = &ps[pi];
    if p.bekci() || p.takim() == b.b.team {
        return r;
    }
    let rr = oku_t(b, pi, ps, tepki(p, Tepki::Uzanma, s), false);
    jmin(r, Q.reach_body + jmax(0.0, yas_t(b) - rr) * Q.reach_grow)
}

/// Uzanma menzili (JS menzil): dengesi bozuk oyuncu az uzanır.
pub fn menzil(p: &Oyuncu, yavas: f64) -> f64 {
    let r = Q.reach + 0.04 * (p.govde.yet.y(mek::UZANMA_MENZIL) - 10.0);
    match p.den {
        Some(d) if d < 1.0 && !p.bekci() => 1.0 + (r - 1.0) * denge_k(Some(d), yavas),
        _ => r,
    }
}

pub fn ctrl_p(b: &mut TopT, pi: usize, ps: &[Oyuncu], x: Option<f64>, k: &dyn Kural) -> f64 {
    let mut t = touch_cap(b, pi, ps, k);
    if let Some(x) = x {
        if !ps[pi].bekci() {
            t.cap *= uc_k(x, Some(&ps[pi]));
        }
    }
    kis(1.0 / (1.0 + exp((t.vr - t.cap) / t.s)), 0.02, 0.99)
}

pub fn bek_uc(d: f64, r: f64) -> f64 {
    kis((d - 1.0) / jmax(0.1, r - 1.0), 0.0, 1.0)
}

/// Sert topun bacaktan sızma ihtimali (JS thruP).
pub fn thru_p(b: &Top, p: &Oyuncu) -> f64 {
    if p.takim() == b.team {
        return 0.0;
    }
    let sp = hyp(b.vx, b.vy);
    jmax(0.0, jmin(Q.thru_max * if p.bekci() { Q.bek_thru } else { 1.0 }, (sp - Q.thru_v) * Q.thru_k - 0.02 * (p.govde.yet.y(mek::SIZMA) - 10.0)))
}

/// Temasın sonucu (JS contact'ın dönüşü).
#[derive(Clone, Copy, Debug)]
pub struct TemasSonuc {
    pub p: usize,
    pub took: bool,
    pub pp: f64,
    pub thru: bool,
}

/// Topun bu tikte geçtiği yolda ilk dokunan oyuncu: tutar ya da sektirir (JS contact).
pub fn contact(b: &mut TopT, ps: &mut [Oyuncu], rnd: &mut dyn FnMut() -> f64, ox: f64, oy: f64, k: &dyn Kural) -> Option<TemasSonuc> {
    let ts = k.tepki_s();
    let yavas = k.ik_yavas();
    let sx = b.b.x - ox;
    let sy = b.b.y - oy;
    let sl = sx * sx + sy * sy;
    let sq = sl.sqrt();
    let mut best: Option<usize> = None;
    let (mut bd, mut be, mut b_r) = (1e9f64, 1e9f64, 1.0f64);
    for pi in 0..ps.len() {
        let p = &ps[pi];
        // bek_cd: Bekçi'nin beklemesi topun şimdiki hızına göre: çelip dibine düşen yavaş top beklemeden alınabilir (bekleme temas
        // anındaki hızla 12 tike dek sabitti; 302 / 8317'de çelinen top Bekçi'nin 0,9 br'sinde 9 tik sürünüp girdi)
        let bekliyor = b.t.cds.get(&pi).copied().unwrap_or(0.0) > b.b.t && !(bek_cd() && p.bekci() && b.t.cds0.get(&pi).map_or(false, |&t0| {
            let v = hyp(b.b.vx, b.b.vy);
            b.b.t >= t0 + jmax(Q.cd_min, jround(Q.cd * jmin(1.0, v / Q.cd_v)))
        }));
        if p.noTouch || p.hal.sonuk || bekliyor {
            continue;
        }
        if Some(pi) == b.t.from && b.b.t < 8.0 {
            continue;
        }
        if b.t.recv.is_some() && p.takim() == b.b.team && Some(pi) != b.t.recv && b.b.defl == 0.0 && !b.t.sap {
            continue;
        }
        if b.t.gon && p.takim() == b.b.team && !p.bekci() && b.b.defl == 0.0 {
            continue;
        }
        let yeni = dalis_modeli(b, pi, ps);
        let vardi = b.t.dal_bas.is_some();
        let (px, py) = if yeni { match bek_dalis(b, pi, ps) { Some((x, y, vx, vy)) => { if !b.t.hayal { let yeni_bas = !vardi && b.t.dal_bas.is_some(); let f = b.t.dal_bas.map_or(0.0, |d| d.f); let q = &mut ps[pi]; if yeni_bas && f > 0.0 { q.hal_mut()._dalis = Some(q.hal._dalis.unwrap_or(0.0) + f); } q.x = x; q.y = y; q.vx = vx; q.vy = vy; } (x, y) } None => (ps[pi].x, ps[pi].y) } } else { (ps[pi].x, ps[pi].y) }; /* maçta Bekçi'nin gerçek yeri dalışla güncellenir (ekranda görünür); dalışın bedeli bardan, gereken uzanmanın payıyla (dalış başlarken bir kez) */
        let t0 = if sl > 0.0 { ((px - ox) * sx + (py - oy) * sy) / sl } else { 1.0 };
        let tt = kis(t0, 0.0, 1.0);
        let dd = hyp(ox + sx * tt - px, oy + sy * tt - py);
        b.t.tt = tt;
        let r0 = if yeni { Q.bek_body } else if ps[pi].bekci() { bek_r(b, pi, ps) } else { menzil(&ps[pi], yavas) } * if t0 < 0.0 { Q.away_k } else { 1.0 };
        let r = opp_reach(b, pi, ps, r0, &ts);
        let r2 = jmax(r, 1.15);
        if dd < r2 {
            let en = if sl > 0.0 { jmax(0.0, t0 * sq - (r2 * r2 - dd * dd).sqrt()) / sq } else { 0.0 };
            if en < be - 1e-9 || (en < be + 1e-9 && dd < bd) {
                be = en;
                bd = dd;
                best = Some(pi);
                b_r = r2;
            }
        }
    }
    b.t.tt = 0.0;
    let bi = best?;
    if ps[bi].bekci() && !dalis_modeli(b, bi, ps) {
        let mn = menzil(&ps[bi], yavas);
        if bd > mn + 1e-6 {
            let bp = &ps[bi];
            let tb = if sl > 0.0 { kis(((bp.x - ox) * sx + (bp.y - oy) * sy) / sl, 0.0, 1.0) } else { 1.0 };
            let cx = ox + sx * tb;
            let cy = oy + sy * tb;
            let dx = cx - bp.x;
            let dy = cy - bp.y;
            let dl = {
                let h = hyp(dx, dy);
                if h == 0.0 || h != h {
                    1.0
                } else {
                    h
                }
            };
            let d = bd - mn;
            let f = d / jmax(0.5, bek_dl(bp));
            if b.t.hayal {
                b.t.dal = Some(Dal { p: bi, x: bp.x + dx / dl * d, y: bp.y + dy / dl * d, f });
            } else {
                let bp = &mut ps[bi];
                bp.x += dx / dl * d;
                bp.y += dy / dl * d;
                bp.hal_mut()._dalis = Some(bp.hal._dalis.unwrap_or(0.0) + f);
            }
        }
    }
    let mut tk = touch_cap(b, bi, ps, k);
    let bp = &ps[bi];
    if !bp.bekci() {
        let tb = if sl > 0.0 { ((bp.x - ox) * sx + (bp.y - oy) * sy) / sl } else { -1.0 };
        let d_g = if tb < 0.0 || !(sq > 1e-9) { bd } else { ((bp.x - ox) * sy - (bp.y - oy) * sx).abs() / sq };
        tk.cap *= uc_k(d_g / b_r, Some(bp));
    }
    let pp = kis(1.0 / (1.0 + exp((tk.vr - tk.cap) / tk.s)), 0.02, 0.99);
    let u = rnd();
    let cap_e = tk.cap + tk.s * log(u / (1.0 - u));
    if tk.vr <= cap_e {
        b.b.done = Some(crate::top::Bitis::Tutuldu);
        b.t.holder = Some(bi);
        {
            let gx = if bp.team == 0.0 { W } else { 0.0 };
            let yn = hyp(b.b.x - bp.x, b.b.y - bp.y) > 0.001;
            let ix = if yn { b.b.x - bp.x } else if true { bp.tx - bp.x } else { gx - bp.x };
            let iy = if yn { b.b.y - bp.y } else if true { bp.ty - bp.y } else { CY - bp.y };
            let il = {
                let h = hyp(ix, iy);
                if h == 0.0 || h != h {
                    1.0
                } else {
                    h
                }
            };
            b.b.x = bp.x + ix / il * Q.body;
            b.b.y = bp.y + iy / il * Q.body;
            saha_ici(&mut b.b);
        }
        b.t.gvx = b.b.vx;
        b.t.gvy = b.b.vy;
        b.b.vx = 0.0;
        b.b.vy = 0.0;
        b.b.log.push((b.b.t, TopOlay::Tut));
        return Some(TemasSonuc { p: bi, took: true, pp, thru: false });
    }
    let mate_t = bp.takim() == b.b.team;
    let sp0 = {
        let h = hyp(b.b.vx, b.b.vy);
        if h == 0.0 || h != h {
            1e-6
        } else {
            h
        }
    };
    let cdv = if !bp.bekci() { Q.cd - 0.5 * (bp.govde.yet.y(mek::YENIDEN_DOKUN) - 10.0) } else { Q.cd };
    b.t.cds.insert(bi, b.b.t + jmax(Q.cd_min, jround(cdv * jmin(1.0, sp0 / Q.cd_v))));
    if bek_cd() && bp.bekci() {
        b.t.cds0.insert(bi, b.b.t);
    }
    let thru = !mate_t && bd >= 1.0 && {
        let r = rnd();
        r < thru_p(&b.b, bp) * if bp.bekci() { bek_uc(bd, b_r) } else { 1.0 }
    };
    if thru {
        let an = atan2(b.b.vy, b.b.vx) + 0.5 * (rnd() - 0.5);
        b.b.vx = cos(an) * sp0 * Q.thru_keep;
        b.b.vy = sin(an) * sp0 * Q.thru_keep;
    } else {
        let tt = if sl > 0.0 { kis(((bp.x - ox) * sx + (bp.y - oy) * sy) / sl, 0.0, 1.0) } else { 1.0 };
        let cx = ox + sx * tt;
        let cy = oy + sy * tt;
        let mut nx = cx - bp.x;
        let mut ny = cy - bp.y;
        let mut nl = hyp(nx, ny);
        if nl < 0.001 {
            nx = -tk.vrx;
            ny = -tk.vry;
            nl = {
                let h = hyp(nx, ny);
                if h == 0.0 || h != h {
                    1.0
                } else {
                    h
                }
            };
        }
        nx /= nl;
        ny /= nl;
        let left = jmax(0.0, tk.vr - jmax(0.0, cap_e));
        let vrd = if tk.vr == 0.0 || tk.vr != tk.vr { 1.0 } else { tk.vr };
        let ux = tk.vrx / vrd;
        let uy = tk.vry / vrd;
        let dn = ux * nx + uy * ny;
        let mut ox2 = ux;
        let mut oy2 = uy;
        if dn < 0.0 && !mate_t {
            ox2 = ux - (1.0 + Q.rest2) * dn * nx;
            oy2 = uy - (1.0 + Q.rest2) * dn * ny;
        }
        let ol = {
            let h = hyp(ox2, oy2);
            if h == 0.0 || h != h {
                1.0
            } else {
                h
            }
        };
        ox2 /= ol;
        oy2 /= ol;
        let bk_p = bp.bekci() && !mate_t;
        let gx_a = if bp.team == 0.0 { W } else { 0.0 };
        let (mut bkx, mut bky): (Option<f64>, Option<f64>) = (None, None);
        if bk_p {
            let own = if bp.team == 0.0 { 0.0 } else { W };
            let out = if own == 0.0 { 1.0 } else { -1.0 };
            let lp = kis(15.0 * jmax(left, 0.8 + 0.04 * (tk.sk - 10.0)), 3.0, 14.0);
            let mut bs = -1e9;
            let mut a = -80.0f64;
            while a <= 80.0 {
                let r = a * PI / 180.0;
                let dx = cos(r) * out;
                let dy = sin(r);
                let px = kis(bp.x + dx * lp, 1.0, W - 1.0);
                let py = kis(bp.y + dy * lp, 1.0, H - 1.0);
                let mut od = 12.0;
                for o in ps.iter() {
                    if o.team == bp.team || o.noTouch {
                        continue;
                    }
                    od = jmin(od, hyp(o.x - px, o.y - py));
                }
                let sc = od - if (px - own).abs() < 22.0 && (py - CY).abs() < 12.0 { 6.0 } else { 0.0 } - 0.01 * acos(kis(dx * ox2 + dy * oy2, -1.0, 1.0));
                if sc > bs {
                    bs = sc;
                    bkx = Some(dx);
                    bky = Some(dy);
                }
                a += 10.0;
            }
        }
        let rcv = mate_t && (b.t.recv == Some(bi) || bp.hal._rcvB == Some(b.t.kimlik));
        let ix = if mate_t && true && !rcv {
            bp.tx - bp.x
        } else if bk_p {
            bkx.unwrap_or(0.35 * (gx_a - bp.x))
        } else {
            gx_a - bp.x
        };
        let iy = if mate_t && true && !rcv {
            bp.ty - bp.y
        } else if bk_p {
            bky.unwrap_or(25.0 * if bp.y >= CY { 1.0 } else { -1.0 })
        } else {
            CY - bp.y
        };
        let il = {
            let h = hyp(ix, iy);
            if h == 0.0 || h != h {
                1.0
            } else {
                h
            }
        };
        let st = jmin(1.0, Q.steer * if bk_p { Q.bek_steer } else { 1.0 } * if bk_p { 1.0 } else { kis(cap_e / tk.vr, 0.0, 1.0) } * (0.5 + 0.04 * (tk.sk - 10.0)));
        let jx = ox2 * (1.0 - st) + ix / il * st;
        let jy = oy2 * (1.0 - st) + iy / il * st;
        let jl = {
            let h = hyp(jx, jy);
            if h == 0.0 || h != h {
                1.0
            } else {
                h
            }
        };
        let ns = (rnd() - 0.5) * (20.0 - tk.sk) * 0.04;
        let cs = cos(ns);
        let sn = sin(ns);
        let fx = jx / jl;
        let fy = jy / jl;
        let left_b = if bk_p { jmax(left, 0.8 + 0.04 * (tk.sk - 10.0)) } else { left };
        b.b.vx = bp.vx + (fx * cs - fy * sn) * left_b;
        b.b.vy = bp.vy + (fx * sn + fy * cs) * left_b;
        b.b.x = bp.x + nx * (Q.reach + 0.05);
        b.b.y = bp.y + ny * (Q.reach + 0.05);
        saha_ici(&mut b.b);
    }
    if b.b.alive {
        b.b.e = jmax(0.0, b.b.e - Q.defl_e * crate::saha::P.tek_s);
    }
    b.b.ch = b.b.e;
    b.b.defl_ekle();
    b.b.log.push((b.b.t, TopOlay::Sek));
    Some(TemasSonuc { p: bi, took: false, pp, thru })
}

/// Topun bir tiki ve temas (JS stepAll).
pub fn step_all(b: &mut TopT, ps: &mut [Oyuncu], fz: &Fizik, rnd: &mut dyn FnMut() -> f64, k: &dyn Kural) -> Option<TemasSonuc> {
    if b.b.done.is_some() {
        return None;
    }
    let ox = b.b.x;
    let oy = b.b.y;
    let src: Vec<_> = ps.iter().map(|p| p.etki()).collect();
    adim(&mut b.b, &mut Kaynak::Sec(&src), fz);
    if b.b.done.is_some() {
        None
    } else {
        contact(b, ps, rnd, ox, oy, k)
    }
}

#[allow(dead_code)]
fn _kullan() {
    let _ = alan_k;
}

// ---- Kuyu ----

pub fn kuyu_mer(x: f64, y: f64) -> f64 {
    let d0 = hyp(x, y - CY);
    let d1 = hyp(x - W, y - CY);
    if d0 < d1 {
        d0
    } else {
        d1
    }
}
pub fn alan_ic(x: f64, y: f64) -> bool {
    kuyu_mer(x, y) < Q.kuyu_a
}

/// Oyuncu t tik sonra (x, y)'de olabilir mi: Bekçi dışı herkesin Kuyu alanında en çok 3 sn'si var (JS kuyuGirer).
pub fn kuyu_girer(p: &Oyuncu, x: f64, y: f64, t: f64, cik: bool, k: &dyn Kural) -> bool {
    if p.bekci() || (!alan_ic(x, y) && !cik) {
        return true;
    }
    if p.hal.sonuk {
        return !alan_ic(x, y);
    }
    let v = jmax(0.05, p.taban_hiz() * k.spr(p));
    let d_p = kuyu_mer(p.x, p.y);
    let ic_p = d_p < Q.kuyu_a;
    let d_x = kuyu_mer(x, y);
    if d_x >= Q.kuyu_a {
        return true;
    }
    let t_e = if ic_p { 0.0 } else { (d_p - Q.kuyu_a) / v };
    let ic = jmax(0.0, t - t_e) + if ic_p { p.hal.kuyuT.unwrap_or(0.0) } else { 0.0 };
    let t_c = if cik { (Q.kuyu_a - d_x + 0.3) / v + 1.0 / (0.18 * crate::yetenek::ah(&p.govde.yet)) } else { 0.0 };
    ic + t_c <= Q.kuyu_sn
}

// ---- gövde ----

/// Top bu tikte bir gövdeye çarpıyor mu (JS AlanCore.govde): ilk çarpılan gövde ve sekme.
pub fn govde(b: &mut TopT, x0: f64, y0: f64, ps: &[Oyuncu], skip: &dyn Fn(usize) -> bool, r: f64, e: f64) -> Option<usize> {
    let sx = b.b.x - x0;
    let sy = b.b.y - y0;
    let l2 = sx * sx + sy * sy;
    let mut hit: Option<usize> = None;
    let mut ht = 2.0;
    for (pi, p) in ps.iter().enumerate() {
        if skip(pi) {
            continue;
        }
        let fx = x0 - p.x;
        let fy = y0 - p.y;
        let c0 = fx * fx + fy * fy - r * r;
        let t;
        if c0 <= 0.0 {
            t = 0.0;
        } else {
            if l2 < 1e-9 {
                continue;
            }
            let bb = fx * sx + fy * sy;
            let disc = bb * bb - l2 * c0;
            if disc < 0.0 {
                continue;
            }
            t = (-bb - disc.sqrt()) / l2;
            if t < 0.0 || t > 1.0 {
                continue;
            }
        }
        if t < ht {
            ht = t;
            hit = Some(pi);
        }
    }
    let hi = hit?;
    let hp = &ps[hi];
    let px = x0 + sx * ht;
    let py = y0 + sy * ht;
    let mut nx = px - hp.x;
    let mut ny = py - hp.y;
    let mut nl = hypot(nx, ny);
    if nl < 1e-4 {
        let vl = {
            let h = hypot(b.b.vx, b.b.vy);
            if h == 0.0 || h != h {
                1.0
            } else {
                h
            }
        };
        nx = -b.b.vx / vl;
        ny = -b.b.vy / vl;
        nl = 1.0;
    }
    nx /= nl;
    ny /= nl;
    b.b.x = hp.x + nx * r;
    b.b.y = hp.y + ny * r;
    saha_ici(&mut b.b);
    let vn = (b.b.vx - hp.vx) * nx + (b.b.vy - hp.vy) * ny;
    let kac = b.t.cds.get(&hi).copied().unwrap_or(0.0) > b.b.t;
    if vn < 0.0 {
        if kac {
            b.b.vx -= vn * nx;
            b.b.vy -= vn * ny;
        } else {
            b.b.vx -= (1.0 + e) * vn * nx;
            b.b.vy -= (1.0 + e) * vn * ny;
            b.b.defl_dogru();
        }
    }
    Some(hi)
}

/// Topu gövdelerin dışına it (JS govdeIt).
pub fn govde_it(b: &mut Top, ps: &[Oyuncu], skip: &dyn Fn(usize) -> bool, r: f64) {
    for (pi, p) in ps.iter().enumerate() {
        if skip(pi) {
            continue;
        }
        let dx = b.x - p.x;
        let dy = b.y - p.y;
        let d = hypot(dx, dy);
        if d >= r {
            continue;
        }
        let vl = hypot(p.vx, p.vy);
        let nx = if d > 0.001 { dx / d } else if vl > 0.001 { p.vx / vl } else { 1.0 };
        let ny = if d > 0.001 { dy / d } else if vl > 0.001 { p.vy / vl } else { 0.0 };
        b.x = p.x + nx * r;
        b.y = p.y + ny * r;
        saha_ici(b);
        let vn = (b.vx - p.vx) * nx + (b.vy - p.vy) * ny;
        if vn < 0.0 {
            b.vx -= vn * nx;
            b.vy -= vn * ny;
        }
    }
}

/// Deney (ALAN_BEK_CD_IZ; ölçümde/wasm'da BEK_CD_ZORLA; analizci onaylı kural değişikliği): Bekçi'nin çeldikten sonraki yeniden
/// dokunma beklemesi topun şimdiki hızına göre sürekli (Q.cd × min(1, v / Q.cd_v), en az Q.cd_min); temas anındaki hızla sabit
/// değil. Varsayılan kapalı.
pub static BEK_CD_ZORLA: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub fn bek_cd() -> bool {
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    BEK_CD_ZORLA.load(std::sync::atomic::Ordering::Relaxed) || *A.get_or_init(|| crate::iz_acik() && std::env::var("ALAN_BEK_CD_IZ").is_ok())
}
