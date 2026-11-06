//! Seçim hatası (decide.js:155–201 kenarAkil/sigY/sHash/secimW/secimZ/tartiK/ufukK/secimN/secim): oyuncunun
//! tartısı yeteneğine göre gürültülüdür; gürültü seçeneğin imzasından ve 20 tiklik dilimden (belirlenimli).
#![allow(non_snake_case)]

use super::kafa1_baglam::Kafa;
use super::kafa1_ikili::js_int;
use super::kafa1_js::*;
use super::kafa1_param::SECIM_P;
use crate::mat::{exp, hypot, jmax, jmin, jround, log};
use crate::model::{Oyuncu, Secenek};
use crate::yetenek::{kademe, mek, yum, zar, DK_SUT, MEK_AD};

/// Mekaniğin sırası (adıyla).
pub fn mek_sira(ad: &str) -> usize {
    MEK_AD.iter().position(|m| *m == ad).unwrap_or_else(|| panic!("ALAN_Y: tabloda olmayan mekanik {ad}"))
}

/// JS sHash: dizginin UTF-16 birimleriyle djb2, 100003'e göre.
pub fn s_hash(s: &str) -> f64 {
    let mut x: i32 = 5381;
    for c in s.encode_utf16() {
        x = (x.wrapping_shl(5)).wrapping_add(x).wrapping_add(c as i32);
    }
    ((x as u32) % 100003) as f64
}

impl Kafa {
    /// Kenardan pası düşünür mü (decide.js:155).
    pub fn kenar_akil(&self, h: &Oyuncu) -> bool {
        zar(h.id, (self.tick() / 20.0).floor(), 13.0) < kademe(h.govde.yet.y(mek::KENAR_PAS_DUSUN), self.D.bank_ok)
    }
    /// Seçeneğin imzası (decide.js:156); `src`: seçeneklerin dünyası (o.q'nun sırası).
    pub fn sig_y(&self, o: &Secenek, src: &[Op]) -> String {
        let q = match o.q.and_then(|i| src.get(i)) {
            Some(q) => js_int(q.id),
            None => String::new(),
        };
        let to = match &o.to {
            Some(t) => format!("{},{}", js_int(jround(t.x / 3.0)), js_int(jround(t.y / 3.0))),
            None => String::new(),
        };
        let shot = o.shot.clone().unwrap_or_default();
        let sy = match o.sYon {
            Some(v) if v != 0.0 && v == v => js_num(v),
            _ => String::new(),
        };
        format!("{}:{}:{}:{}:{}", o.kind, q, to, shot, sy)
    }
    /// Tartının gürültüsünün genişliği (decide.js:162); `k`: mekaniğin adı.
    pub fn secim_w(&self, h: &Oyuncu, B: f64, sut: bool, k: Option<&str>) -> f64 {
        let m = match k {
            Some(k) => mek_sira(k),
            None => {
                if sut {
                    mek::SUT_HEDEF_SEC
                } else {
                    mek::TARTI_ISABET
                }
            }
        };
        self.D.otMargin * (20.0 - h.govde.yet.y(m) + 10.0 * DK_SUT.b * B * yum(&h.govde.yet, "B")) / 10.0
    }
    /// Seçeneğin gürültüsü (decide.js:162).
    pub fn secim_z(&self, h: &Oyuncu, o: &Secenek, w: f64, src: &[Op]) -> f64 {
        let z = if gurultu_surekli() {
            self.surekli_z(h, o, src)
        } else {
            let k = s_hash(&self.sig_y(o, src));
            let t = (self.tick() / 20.0).floor();
            zar(h.id, t, k) + zar(h.id + 101.0, t, k) - 1.0
        };
        let dk = o.dok.as_ref();
        let d = match dk.map(|d| (d.al("tutar"), d.al("durum"), d.al("kayip"), d.al("gol"))) {
            Some((Some(t), Some(du), ka, _)) => Some((t, du, ka.unwrap_or(f64::NAN))),
            Some((_, _, ka, Some(g))) if o.kind == "gönder" => Some((g, 1.0, z0(ka.unwrap_or(f64::NAN)))),
            _ => match (&o.det, o.v) {
                (Some(det), Some(v)) if o.kind != "gönder" && det.P.is_some() => {
                    let P = det.P.unwrap();
                    Some((P, if P > 0.01 { v / P } else { 0.0 }, 0.0))
                }
                _ => None,
            },
        };
        let (tutar, durum, kayip) = match d {
            Some(d) => d,
            None => return z * w,
        };
        let P = jmax(0.0, jmin(1.0, tutar));
        let L = if P < 0.999 { jmax(0.0, -z0(kayip) / (1.0 - P)) } else { 0.0 };
        let Pc = jmax(1e-9, jmin(1.0 - 1e-9, P));
        let Pg = 1.0 / (1.0 + exp(-(log(Pc / (1.0 - Pc)) + 4.0 * z * w / self.D.otMargin * SECIM_P)));
        (Pg - P) * (durum + L)
    }
    /// ALAN_GURULTU_SUREKLI_IZ: oyuncunun bir seçeneği yanlış tartması kalıcıdır. Gürültünün imzası seçeneğin anlamından:
    /// sürmede oyuncuya göre yön (16 dilim), pasta alıcının kimliği (yeri değil); zamanda 20 tiklik dilimler, yönde de
    /// dilimler arası düzgün geçiş (genişlik korunur: ağırlıkların kareleri toplamıyla). Oyuncu ilerledikçe ya da dilim
    /// sınırında tercihleri birden yeniden çekilmez.
    pub fn surekli_z(&self, h: &Oyuncu, o: &Secenek, src: &[Op]) -> f64 {
        let q = match o.q.and_then(|i| src.get(i)) {
            Some(q) => js_int(q.id),
            None => String::new(),
        };
        let yonlu = o.q.is_none() && o.to.is_some() && o.kind != "gönder";
        let (a0, fa) = if yonlu {
            let t = o.to.as_ref().unwrap();
            let a = crate::mat::atan2(t.y - h.y, t.x - h.x);
            let u = (a / (2.0 * std::f64::consts::PI) + 1.0) * 16.0;
            (u.floor(), u - u.floor())
        } else {
            (0.0, 0.0)
        };
        let shot = o.shot.clone().unwrap_or_default();
        let sy = match o.sYon {
            Some(v) if v != 0.0 && v == v => js_num(v),
            _ => String::new(),
        };
        let tt = self.tick() / 20.0;
        let (t0, ft) = (tt.floor(), tt - tt.floor());
        let zz = |t: f64, a: f64| -> f64 {
            let k = s_hash(&format!("{}:{}:{}:{}:{}", o.kind, q, (a as i64).rem_euclid(16), shot, sy));
            zar(h.id, t, k) + zar(h.id + 101.0, t, k) - 1.0
        };
        let mut s = 0.0;
        let mut w2 = 0.0;
        for (dt, wt) in [(0.0, 1.0 - ft), (1.0, ft)] {
            for (da, wa) in [(0.0, 1.0 - fa), (1.0, fa)] {
                let w = wt * wa;
                if w <= 0.0 {
                    continue;
                }
                s += w * zz(t0 + dt, a0 + da);
                w2 += w * w;
            }
        }
        if w2 > 0.0 { s / w2.sqrt() } else { 0.0 }
    }
    /// Seçeneğin tartısını hangi mekanik bozar (decide.js:178).
    pub fn tarti_k(&self, o: &Secenek) -> &'static str {
        if o.kind == "gönder" {
            "sutHedefSec"
        } else if o.kind == "kenardan" || o.kind == "çalım" || o.kind == "fake" || (o.kind == "sür" && o.duvar) {
            "secenekTur"
        } else {
            "tartiIsabet"
        }
    }
    /// Uzak gelecekteki seçeneğin gürültüsü büyür (decide.js:178).
    pub fn ufuk_k(&self, h: &Oyuncu, o: &Secenek, T: f64) -> f64 {
        if o.kind == "gönder" || !(T > 0.0) {
            return 1.0;
        }
        let ln = match &o.to {
            Some(t) => hypot(t.x - h.x, t.y - h.y),
            None => 0.0,
        };
        let dt = if o.kind == "tut" {
            yao(o.hold, T)
        } else if o.kind == "sür" || o.kind == "çalım" {
            jmax(T, ln / 0.2)
        } else {
            let tf = yao(o.tf, 0.0);
            if tf != 0.0 {
                tf
            } else {
                let d_t = o.det.as_ref().and_then(|d| d.T).map_or(0.0, z0);
                if d_t != 0.0 {
                    d_t
                } else {
                    jmax(8.0, ln / 0.8)
                }
            }
        };
        (dt / (T * jmax(1.0, h.govde.yet.y(mek::ILERI_BAKIS)) / 10.0)).sqrt()
    }
    /// Tek seçeneğin gürültüsü (decide.js:183).
    pub fn secim_n(&self, h: &Oyuncu, o: Option<&Secenek>, src: &[Op], T: f64) -> f64 {
        let o = match o {
            Some(o) => o,
            None => return 0.0,
        };
        if h.id < 0.0 {
            return 0.0;
        }
        let B = jmin(1.0, self.press_at(src, h.team, h.x, h.y));
        let w = self.secim_w(h, B, false, Some(self.tarti_k(o))) * self.ufuk_k(h, o, T);
        if w > 0.0 {
            self.secim_z(h, o, w, src)
        } else {
            0.0
        }
    }
    /// Gürültülü seçim (decide.js:188): seçeneklerin sırası (yoksa `best`).
    pub fn secim(&self, h: &Oyuncu, opts: &[Secenek], best: Option<usize>, T: f64, src: &[Op]) -> Option<usize> {
        if h.id < 0.0 {
            return best;
        }
        let B = jmin(1.0, self.press_at(src, h.team, h.x, h.y));
        let mut bi: Option<usize> = None;
        let mut bv = f64::NEG_INFINITY;
        let mut bs: Option<usize> = None;
        let mut bs_v = f64::NEG_INFINITY;
        // ALAN_PLAN_IZ: plana devam eden seçenek: sürmede önceki yöne en yakın sürme (90°'den az sapan), diğerlerinde aynı tür ve alıcı
        let psur = plan_sur() && !plan_acik();
        let devam: Vec<bool> = if plan_acik() || psur {
            PLAN_SIMDI.with(|p| {
                let p = p.borrow();
                let Some(p) = p.as_ref() else { return vec![false; opts.len()] };
                let ps: Vec<PlanIz> = opts.iter().map(|o| plan_of(h, o, src, 0.0)).collect();
                match p.aci {
                    Some(a0) => {
                        let fark = |q: &PlanIz| -> f64 {
                            if q.tur != p.tur || q.q != p.q {
                                return f64::INFINITY;
                            }
                            match q.aci {
                                Some(x) => {
                                    let mut d = (x - a0).abs() % (2.0 * std::f64::consts::PI);
                                    if d > std::f64::consts::PI {
                                        d = 2.0 * std::f64::consts::PI - d;
                                    }
                                    d
                                }
                                None => f64::INFINITY,
                            }
                        };
                        let en = ps.iter().map(|q| fark(q)).fold(f64::INFINITY, f64::min);
                        ps.iter().map(|q| { let d = fark(q); d < std::f64::consts::FRAC_PI_2 && d <= en + 1e-9 }).collect()
                    }
                    None => ps.iter().map(|q| plan_uyar(p, q)).collect(),
                }
            })
        } else {
            Vec::new()
        };
        for (i, o) in opts.iter().enumerate() {
            let ov = match o.v {
                Some(v) => v,
                None => continue,
            };
            let w = self.secim_w(h, B, false, Some(self.tarti_k(o))) * self.ufuk_k(h, o, T);
            let mut v = if w > 0.0 { ov + self.secim_z(h, o, w, src) } else { ov };
            if psur {
                // ALAN_PLAN_SUR_IZ: tür seçimi plansız; plan yalnız sürmelerin arasında yön seçer
                if v > bv {
                    bv = v;
                    bi = Some(i);
                }
                if matches!(o.kind.as_str(), "sür" | "çalım") {
                    let vs = v + if devam.get(i).copied().unwrap_or(false) { self.secim_w(h, B, false, Some(self.tarti_k(o))) } else { 0.0 };
                    if vs > bs_v {
                        bs_v = vs;
                        bs = Some(i);
                    }
                }
                continue;
            }
            if devam.get(i).copied().unwrap_or(false) {
                v += self.secim_w(h, B, false, Some(self.tarti_k(o)));
            }
            if v > bv {
                bv = v;
                bi = Some(i);
            }
        }
        if psur {
            if let (Some(i), Some(j)) = (bi, bs) {
                if matches!(opts[i].kind.as_str(), "sür" | "çalım") {
                    return Some(j);
                }
            }
        }
        bi.or(best)
    }
}

/// Deney (ALAN_GURULTU_SUREKLI_IZ; ölçümde GURULTU_SUREKLI_ZORLA): seçim gürültüsü zamanda ve seçeneğin anlamında sürekli.
pub static GURULTU_SUREKLI_ZORLA: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
/// Seçim gürültüsü zamanda ve seçeneğin anlamında sürekli. Varsayılan açık (sitedeki kafa); kapatmak: GURULTU_SUREKLI_KAPALI (wasm: adres seçeneği) ya da ALAN_GURULTU_SUREKLI_ESKI=1.
pub static GURULTU_SUREKLI_KAPALI: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub fn gurultu_surekli() -> bool {
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    if GURULTU_SUREKLI_KAPALI.load(std::sync::atomic::Ordering::Relaxed) {
        return false;
    }
    GURULTU_SUREKLI_ZORLA.load(std::sync::atomic::Ordering::Relaxed) || *A.get_or_init(|| std::env::var("ALAN_GURULTU_SUREKLI_ESKI").is_err())
}

/// Deney (ALAN_PLAN_IZ; ölçümde PLAN_ZORLA): oyuncu planına sadıktır: son kararının planına (tür; pasta alıcı, sürmede yön)
/// karşılık gelen seçenek, seçimde oyuncunun karar gürültüsünün genişliği kadar önde sayılır (ayırt edemediği farkla planını
/// bozmaz; belirgin biçimde iyi bir şey çıkınca bozar). Yalnız seçim anında; değere karışmaz. Ayrıca sürmenin süresine
/// hızlanma payı yerine maçın dönüş tablosundan dönme bedeli (kafa1_enerji::sur_sure).
pub static PLAN_ZORLA: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
/// Plan sadakati. Varsayılan açık (sitedeki kafa); kapatmak: PLAN_KAPALI (wasm: adres seçeneği) ya da ALAN_PLAN_ESKI=1.
pub static PLAN_KAPALI: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
/// Deney (ALAN_PLAN_SUR_IZ; ölçümde PLAN_SUR_ZORLA): plan yalnız sürmeyi kapsar; pas/şut/tut seçimine dokunmaz,
/// plansız seçimin kazananı sürmeyse planla uyumlu sürme yönü seçilir.
/// Varsayılan açık (20 maç: pas 327 → 344, dönüş 0.51 → 0.44); kapatmak: PLAN_SUR_KAPALI (wasm: ?plansur=0) ya da ALAN_PLAN_SUR_ESKI=1.
pub static PLAN_SUR_ZORLA: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub static PLAN_SUR_KAPALI: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub fn plan_sur() -> bool {
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    if PLAN_SUR_KAPALI.load(std::sync::atomic::Ordering::Relaxed) {
        return false;
    }
    PLAN_SUR_ZORLA.load(std::sync::atomic::Ordering::Relaxed) || *A.get_or_init(|| std::env::var("ALAN_PLAN_SUR_ESKI").is_err())
}
pub fn plan_acik() -> bool {
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    if PLAN_KAPALI.load(std::sync::atomic::Ordering::Relaxed) {
        return false;
    }
    // varsayılan kapandı (20 maç: pas 355 → 217; devam ödülü hep sürmeye gidiyor); açmak: ALAN_PLAN_IZ (sitede ?plan=1)
    PLAN_ZORLA.load(std::sync::atomic::Ordering::Relaxed) || *A.get_or_init(|| crate::iz_acik() && std::env::var("ALAN_PLAN_IZ").is_ok())
}
/// Bir taşıyıcı kararının planı: tür öbeği, pasın alıcısı (kimlik), sürmenin yönü (açı), karar tiki.
#[derive(Clone, Debug, Default)]
pub struct PlanIz {
    pub tur: String,
    pub q: Option<f64>,
    pub aci: Option<f64>,
    pub t: f64,
}
thread_local! {
    /// seçim süresince geçerli plan (maç kafaya seçimden hemen önce verir, sonra alır)
    pub static PLAN_SIMDI: std::cell::RefCell<Option<PlanIz>> = const { std::cell::RefCell::new(None) };
}
/// Seçeneğin planı (tür öbeği: pas türleri "pas", sürme ve çalım "sür", diğerleri kendi adı).
pub fn plan_of(h: &Oyuncu, o: &Secenek, src: &[Op], t: f64) -> PlanIz {
    let tur = match o.kind.as_str() {
        "pas" | "önüne" | "kenardan" => "pas",
        "sür" | "çalım" => "sür",
        k => k,
    }
    .to_string();
    let q = o.q.and_then(|i| src.get(i)).map(|q| q.id + 1000.0 * q.team);
    let kendine = o.q.and_then(|i| src.get(i)).map_or(false, |q| q.id == h.id && q.team == h.team);
    let aci = if (o.q.is_none() || kendine) && tur != "gönder" { o.to.as_ref().map(|t2| crate::mat::atan2(t2.y - h.y, t2.x - h.x)) } else { None };
    PlanIz { tur, q: if kendine { None } else { q }, aci, t }
}
fn plan_uyar(a: &PlanIz, b: &PlanIz) -> bool {
    if a.tur != b.tur || a.q != b.q {
        return false;
    }
    match (a.aci, b.aci) {
        (Some(x), Some(y)) => {
            let mut d = (x - y).abs() % (2.0 * std::f64::consts::PI);
            if d > std::f64::consts::PI {
                d = 2.0 * std::f64::consts::PI - d;
            }
            d < std::f64::consts::PI / 8.0
        }
        (None, None) => true,
        _ => false,
    }
}

/// Sayının JS dizgisi (tam sayılar ve kısa ondalıklar için yeterli).
pub fn js_num(v: f64) -> String {
    if v == v.trunc() && v.abs() < 1e15 {
        js_int(v)
    } else {
        format!("{}", v)
    }
}
