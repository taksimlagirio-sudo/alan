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
pub fn sHash(s: &str) -> f64 {
    let mut x: i32 = 5381;
    for c in s.encode_utf16() {
        x = (x.wrapping_shl(5)).wrapping_add(x).wrapping_add(c as i32);
    }
    ((x as u32) % 100003) as f64
}

impl Kafa {
    /// Kenardan pası düşünür mü (decide.js:155).
    pub fn kenarAkil(&self, h: &Oyuncu) -> bool {
        zar(h.id, (self.tick() / 20.0).floor(), 13.0) < kademe(h.yet.y(mek::KENAR_PAS_DUSUN), self.D.bankOk)
    }
    /// Seçeneğin imzası (decide.js:156); `src`: seçeneklerin dünyası (o.q'nun sırası).
    pub fn sigY(&self, o: &Secenek, src: &[Op]) -> String {
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
    pub fn secimW(&self, h: &Oyuncu, B: f64, sut: bool, k: Option<&str>) -> f64 {
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
        self.D.otMargin * (20.0 - h.yet.y(m) + 10.0 * DK_SUT.b * B * yum(&h.yet, "B")) / 10.0
    }
    /// Seçeneğin gürültüsü (decide.js:162).
    pub fn secimZ(&self, h: &Oyuncu, o: &Secenek, w: f64, src: &[Op]) -> f64 {
        let k = sHash(&self.sigY(o, src));
        let t = (self.tick() / 20.0).floor();
        let z = zar(h.id, t, k) + zar(h.id + 101.0, t, k) - 1.0;
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
    /// Seçeneğin tartısını hangi mekanik bozar (decide.js:178).
    pub fn tartiK(&self, o: &Secenek) -> &'static str {
        if o.kind == "gönder" {
            "sutHedefSec"
        } else if o.kind == "kenardan" || o.kind == "çalım" || o.kind == "fake" || (o.kind == "sür" && o.duvar) {
            "secenekTur"
        } else {
            "tartiIsabet"
        }
    }
    /// Uzak gelecekteki seçeneğin gürültüsü büyür (decide.js:178).
    pub fn ufukK(&self, h: &Oyuncu, o: &Secenek, T: f64) -> f64 {
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
                let dT = o.det.as_ref().and_then(|d| d.T).map_or(0.0, z0);
                if dT != 0.0 {
                    dT
                } else {
                    jmax(8.0, ln / 0.8)
                }
            }
        };
        (dt / (T * jmax(1.0, h.yet.y(mek::ILERI_BAKIS)) / 10.0)).sqrt()
    }
    /// Tek seçeneğin gürültüsü (decide.js:183).
    pub fn secimN(&self, h: &Oyuncu, o: Option<&Secenek>, src: &[Op], T: f64) -> f64 {
        let o = match o {
            Some(o) => o,
            None => return 0.0,
        };
        if h.id < 0.0 {
            return 0.0;
        }
        let B = jmin(1.0, self.pressAt(src, h.team, h.x, h.y));
        let w = self.secimW(h, B, false, Some(self.tartiK(o))) * self.ufukK(h, o, T);
        if w > 0.0 {
            self.secimZ(h, o, w, src)
        } else {
            0.0
        }
    }
    /// Gürültülü seçim (decide.js:188): seçeneklerin sırası (yoksa `best`).
    pub fn secim(&self, h: &Oyuncu, opts: &[Secenek], best: Option<usize>, T: f64, src: &[Op]) -> Option<usize> {
        if h.id < 0.0 {
            return best;
        }
        let B = jmin(1.0, self.pressAt(src, h.team, h.x, h.y));
        let mut bi: Option<usize> = None;
        let mut bv = f64::NEG_INFINITY;
        for (i, o) in opts.iter().enumerate() {
            let ov = match o.v {
                Some(v) => v,
                None => continue,
            };
            let w = self.secimW(h, B, false, Some(self.tartiK(o))) * self.ufukK(h, o, T);
            let v = if w > 0.0 { ov + self.secimZ(h, o, w, src) } else { ov };
            if v > bv {
                bv = v;
                bi = Some(i);
            }
        }
        bi.or(best)
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
fn js_dogru_d(v: &Option<crate::model::JsDeger>) -> bool {
    super::kafa1_enerji::js_dogru(v)
}
