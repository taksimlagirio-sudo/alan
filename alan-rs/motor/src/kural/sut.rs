//! Vuruşun kuralları (saf): gövdenin verebileceği vuruş hızı, şut gücü katı, taşıyıcının en sert şutu.
//! Kafa (şut ve pas tartısı) ve yerleşim (Bekçi'nin yeri) aynı işlevi çağırır.
#![allow(non_snake_case)]
use crate::core_sabit::Q;
use crate::kural::ikili::IK;
use crate::mat::{hypot, jmax};
use crate::model::Oyuncu;
use crate::top::chg_mul;
use crate::yetenek::{denge_k, mek};

/// Savurma (vuruş öncesi Çekirdeği döndürme) parametreleri (JS `SAV`).
#[derive(Clone, Copy, Debug)]
pub struct Sav {
    pub a0: f64,
    pub aSk: f64,
    pub wMax0: f64,
    pub wMaxSk: f64,
    pub tau: f64,
    pub ia0: f64,
    pub iaSk: f64,
    pub u0: f64,
    pub uSk: f64,
}
pub const SAV: Sav = Sav { a0: 0.18, aSk: 0.012, wMax0: 2.3, wMaxSk: 0.03, tau: 0.08, ia0: 0.45, iaSk: 0.025, u0: 1.0, uSk: 0.03 };

/// Gövdenin verebileceği en büyük vuruş hızı (decide.js:561).
pub fn govdeV(h: &Oyuncu) -> f64 {
    let G = h.yet.ozk("guc");
    hypot((SAV.wMax0 + (G - 10.0) * SAV.wMaxSk) * Q.body, jmax(0.3, SAV.u0 + (G - 10.0) * SAV.uSk))
}
/// Şut gücü katı (decide.js:2497): yetenek, dengesi bozuksa denge katı.
pub fn sutGuc(h: &Oyuncu) -> f64 {
    (1.0 + 0.013 * (h.yet.y(mek::SUT_GUCU) - 10.0))
        * match h.den {
            Some(d) if d < 1.0 && !h.bekci() => denge_k(Some(d), IK.yavas),
            _ => 1.0,
        }
}
/// Taşıyıcının şimdi çekebileceği en sert şutun hızı (tam şarj): gövde savurma hızı × vuruş enerjisi.
pub fn sutHiz(h: &Oyuncu) -> f64 {
    govdeV(h) * (chg_mul(1.0) * sutGuc(h))
}
