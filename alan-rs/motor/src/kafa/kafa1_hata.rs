//! Vuruşun hatası (decide.js:202–206 gonEk, 547 pasEk/sogK, 735–790 hataF/SUT_TABAN/pasK/pasKv/
//! applyErrorSav/applyError).
#![allow(non_snake_case)]

use super::kafa1_baglam::Kafa;
use super::kafa1_js::*;
use super::kafa1_param::{GON, SAV};
use super::kafa1_tehlike::gx_of;
use crate::core_sabit::IN;
use crate::mat::{cos, hypot, jmax, jmin, sin};
use crate::model::{Oyuncu, SutBag};
use crate::saha::CY as GCY;
use crate::yetenek::{durum_pay, mek, Durum, DK_PAS, DK_SUT};

/// hataF'nin dönüşü: yön, hız, bırakma hatası.
#[derive(Clone, Copy, Debug)]
pub struct HataF {
    pub y: f64,
    pub v: f64,
    pub b: f64,
}

impl Kafa {
    /// Gönderme hatasının katı (decide.js:202; Yetenek 2 açık).
    pub fn gon_ek(&self, _h: &Oyuncu, _src: &[Op], shot: &str) -> f64 {
        GON.taban * (1.0 + if shot == "güçlü" { GON.guc } else { 0.0 })
    }
    /// decide.js:547 (Yetenek 2 açık: 1)
    pub fn pas_ek(&self, _h: &Oyuncu, _src: &[Op]) -> f64 {
        1.0
    }
    /// decide.js:547
    pub fn sog_k(&self, h: &Oyuncu) -> f64 {
        jmax(0.2, 1.0 - 0.06 * (h.govde.yet.ozk("sogukkanlilik") - 10.0))
    }
    /// D._sut'un metin hâli (şutun türü: "güçlü"/"plase"); sayıysa (taktik) yok.
    pub fn d_sut_tur(&self) -> Option<String> {
        match &self.bag._sut {
            Some(SutBag::Tur(s)) => Some(s.clone()),
            _ => None,
        }
    }
    /// Şutun taban hatası (decide.js:755).
    pub fn sut_taban(&self) -> f64 {
        10.0 * 0.087 / (SAV.tau * (SAV.w_max0 + 10.0 * SAV.w_max_sk) + self.D.aim_err * (SAV.u0 + 10.0 * SAV.u_sk))
    }
    /// Hatanın yetenek ve durum payları (decide.js:735).
    pub fn hata_f(&self, h: &Oyuncu, src: Option<&[Op]>, gon: bool, tur: Option<&str>, kenar: bool, lk: f64) -> HataF {
        let DZ = gon || h.id >= 0.0;
        let my = if gon {
            if tur == Some("plase") {
                mek::PLASE
            } else {
                mek::SUT_YON
            }
        } else if kenar {
            mek::KENAR_PAS_ISABET
        } else {
            mek::PAS_YON
        };
        let Sy = if my == mek::SUT_YON { h.govde.yet.sut_y(hypot(gx_of(h.team) - h.x, GCY - h.y)) } else { h.govde.yet.y(my) };
        let Sv = if gon { Sy } else { h.govde.yet.y(mek::PAS_HIZ) };
        let Sb = h.govde.yet.y(mek::BIRAKMA);
        if !DZ {
            return HataF { y: (20.0 - Sy) * lk, v: (20.0 - Sv) * lk, b: (20.0 - Sb) * lk };
        }
        let s = Durum { b: jmin(1.0, self.press_at(src.unwrap_or(&[]), h.team, h.x, h.y)), d: 1.0 - h.den.unwrap_or(1.0), h: self.har(h), hz: 0.0, e: 0.0 };
        let T = if gon { &DK_SUT } else { &DK_PAS };
        let l0 = lk / (1.0 + if gon && tur == Some("güçlü") { GON.guc } else { 0.0 });
        let ty = if gon { self.sut_taban() } else { 0.0 };
        let e_y = hypot(20.0 - Sy, ty);
        let e_b = hypot(20.0 - Sb, ty);
        HataF {
            y: e_y * lk + 10.0 * l0 * durum_pay(&h.govde.yet, my, T, &s),
            v: (20.0 - Sv) * lk + 10.0 * l0 * durum_pay(&h.govde.yet, if gon { my } else { mek::PAS_HIZ }, T, &s),
            b: e_b * lk + 10.0 * l0 * durum_pay(&h.govde.yet, mek::BIRAKMA, T, &s),
        }
    }
    /// Pasın sertliğiyle artan yön hatası (decide.js:758).
    pub fn pas_k(&self, gon: bool, vx: f64, vy: f64, h: Option<&Oyuncu>) -> f64 {
        if gon {
            return 1.0;
        }
        let g = self.vmax_of(h) / self.D.vmax;
        IN.p_aim * (1.0 + IN.p_sert * jmax(0.0, hypot(vx, vy) / g - 0.5) / 0.5)
    }
    pub fn pas_kv(&self, gon: bool) -> f64 {
        if gon {
            1.0
        } else {
            IN.p_hz
        }
    }
    fn hata_ortak(&self, h: &Oyuncu, gon: bool, src: Option<&[Op]>, tur: Option<&str>, kenar: Option<bool>, ot: f64) -> (bool, HataF) {
        let g = gon || self.bag._gon.unwrap_or(false);
        let st = self.d_sut_tur();
        let tur = match tur {
            Some(t) => Some(t.to_string()),
            None => {
                if g {
                    st
                } else {
                    None
                }
            }
        };
        let kenar = kenar.or(self.bag._kenar).unwrap_or(false);
        let lk = ya(ya(ot, self.bag._ot.unwrap_or(0.0)), 1.0);
        (g, self.hata_f(h, src, g, tur.as_deref(), kenar, lk))
    }
    /// Savurmalı vuruşun hatası (decide.js:763): hız vektörünü döndürür ve ölçekler.
    #[allow(clippy::too_many_arguments)]
    pub fn apply_error_sav(&self, h: &Oyuncu, v: Nk, rnd: &mut dyn FnMut() -> f64, ot: f64, gon: bool, w: f64, u_sh: f64, src: Option<&[Op]>, tur: Option<&str>, kenar: Option<bool>) -> Nk {
        let (g, F) = self.hata_ortak(h, gon, src, tur, kenar, ot);
        let r1 = rnd();
        let r2 = rnd();
        let ang = (r1 + r2 - 1.0) * (SAV.tau * w * F.b + self.D.aim_err * self.pas_k(g, v.x, v.y, Some(h)) * z0(u_sh) * F.y) / 10.0;
        let r3 = rnd();
        let r4 = rnd();
        let k = 1.0 + (r3 + r4 - 1.0) * F.v * self.D.v0_err * self.pas_kv(g) / 10.0;
        let c = cos(ang);
        let s = sin(ang);
        nk((v.x * c - v.y * s) * k, (v.x * s + v.y * c) * k)
    }
    /// Vuruşun hatası (decide.js:777).
    pub fn apply_error(&self, h: &Oyuncu, v: Nk, rnd: &mut dyn FnMut() -> f64, ot: f64, gon: bool, src: Option<&[Op]>, tur: Option<&str>, kenar: Option<bool>) -> Nk {
        let (g, F) = self.hata_ortak(h, gon, src, tur, kenar, ot);
        let r1 = rnd();
        let r2 = rnd();
        let ang = (r1 + r2 - 1.0) * F.y * self.D.aim_err * self.pas_k(g, v.x, v.y, Some(h)) / 10.0;
        let r3 = rnd();
        let r4 = rnd();
        let k = 1.0 + (r3 + r4 - 1.0) * F.v * self.D.v0_err * self.pas_kv(g) / 10.0;
        let c = cos(ang);
        let s = sin(ang);
        nk((v.x * c - v.y * s) * k, (v.x * s + v.y * c) * k)
    }
}
