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
    pub a_sk: f64,
    pub w_max0: f64,
    pub w_max_sk: f64,
    pub tau: f64,
    pub ia0: f64,
    pub ia_sk: f64,
    pub u0: f64,
    pub u_sk: f64,
}
pub const SAV: Sav = Sav { a0: 0.18, a_sk: 0.012, w_max0: 2.3, w_max_sk: 0.03, tau: 0.08, ia0: 0.45, ia_sk: 0.025, u0: 1.0, u_sk: 0.03 };

/// Gövdenin verebileceği en büyük vuruş hızı (decide.js:561).
pub fn govde_v(h: &Oyuncu) -> f64 {
    let G = h.govde.yet.ozk("guc");
    hypot((SAV.w_max0 + (G - 10.0) * SAV.w_max_sk) * Q.body, jmax(0.3, SAV.u0 + (G - 10.0) * SAV.u_sk))
}
/// Şut gücü katı (decide.js:2497): yetenek, dengesi bozuksa denge katı.
pub fn sut_guc(h: &Oyuncu) -> f64 {
    (1.0 + 0.013 * (h.govde.yet.y(mek::SUT_GUCU) - 10.0))
        * match h.den {
            Some(d) if d < 1.0 && !h.bekci() => denge_k(Some(d), IK.yavas),
            _ => 1.0,
        }
}
/// Taşıyıcının şimdi çekebileceği en sert şutun hızı (tam şarj): gövde savurma hızı × vuruş enerjisi.
pub fn sut_hiz(h: &Oyuncu) -> f64 {
    govde_v(h) * (chg_mul(1.0) * sut_guc(h))
}
