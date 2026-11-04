//! Oyuncu satırı (JS'te maçın `p` nesnesi; envanter: 132 alan). Fizik çekirdeğinin okuduğu alanlar burada;
//! maç ve kafa alanları ilgili modüller taşındıkça eklenir.

use crate::alan::Etki;
use crate::mat::{jmax, jmin};
use crate::yetenek::{mek, spr_h, Yetenek};

pub use crate::model::Oyuncu;

impl Oyuncu {
    pub fn bekci(&self) -> bool {
        &*self.role == "Bekçi"
    }
    /// takım (0/1) tam sayı olarak
    #[inline(always)]
    pub fn takim(&self) -> u8 {
        self.team as u8
    }
    /// Alan kaynağı olarak görünüşü (JS: R || 8, D ?? 1, cone, fx, fy, amA, am, en).
    pub fn etki(&self) -> Etki {
        Etki {
            x: self.x,
            y: self.y,
            r: if self.R != 0.0 && self.R == self.R { self.R } else { 8.0 },
            d: self.D,
            koni: false,
            fx: 0.0,
            fy: 0.0,
            team: self.takim(),
            am_a: self.amA,
            am: self.am.unwrap_or(0.0),
            en: self.en.unwrap_or(1.0),
        }
    }
    /// Etkin Hız (Enerji 3.0: bar hızı düşürmez, ham Hız).
    #[inline(always)]
    pub fn hz(&self) -> f64 {
        self.yet.hiz
    }
    /// Temel koşu hızı (br/tik).
    #[inline(always)]
    pub fn taban_hiz(&self) -> f64 {
        0.17 + 0.006 * self.hz()
    }
    /// Sprint katı Hız'dan (çekirdek; maç sprint evresiyle sarar).
    #[inline(always)]
    pub fn spr0(&self) -> f64 {
        spr_h(self.yet.hiz)
    }
}

/// Tepki süreleri (JS ALAN_TEPKI): işe göre tik; bar azaldıkça uzar (Enerji 3.0 tepkiK).
#[derive(Clone, Copy, Debug)]
pub enum Tepki {
    Alan,
    Algi,
    Karar,
    YeniTop,
    Kovala,
    BosTop,
    Uzanma,
}

/// Yerleşimin tepki parametreleri (JS AlanShape.S'den: lag0, lagOk, react, reactLoose, reactLooseOk).
#[derive(Clone, Copy, Debug)]
pub struct TepkiS {
    pub lag0: f64,
    pub lag_ok: f64,
    pub react: f64,
    pub react_loose: f64,
    pub react_loose_ok: f64,
}
impl Default for TepkiS {
    fn default() -> Self {
        TepkiS { lag0: 10.0, lag_ok: 0.4, react: 0.5, react_loose: 6.0, react_loose_ok: 0.25 }
    }
}

pub const EN3_TEPKI_K: f64 = 0.25;

pub fn tepki(p: &Oyuncu, k: Tepki, s: &TepkiS) -> f64 {
    use crate::core_sabit::Q;
    let y = &p.yet;
    let t = match k {
        Tepki::Alan => s.lag0 - y.y(mek::ALAN_ACMA) * s.lag_ok,
        Tepki::Algi => s.lag0 - y.y(mek::ALGI_GECIKME) * s.lag_ok,
        Tepki::Karar => 9.0 - 0.4 * y.y(mek::KARAR_SURE),
        Tepki::YeniTop => Q.tepki0 - y.y(mek::ALGI_GECIKME) * Q.tepki_ok,
        Tepki::Kovala => 14.0 - y.y(mek::PRES_BASLA) * s.react,
        Tepki::BosTop => s.react_loose - y.y(mek::PRES_BASLA) * s.react_loose_ok,
        Tepki::Uzanma => Q.reach_react - y.y(mek::UZANMA_BASLA) * Q.reach_react_ok,
    };
    match p.en {
        Some(e) => t * (1.0 + EN3_TEPKI_K * (1.0 - jmax(0.0, jmin(1.0, e)))),
        None => t,
    }
}
