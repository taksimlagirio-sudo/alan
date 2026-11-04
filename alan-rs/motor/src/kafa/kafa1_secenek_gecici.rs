//! GEÇİCİ: options'ın kafa1'den beklediği, henüz yazılmamış yöntemler (birleştirmede silinir).
#![allow(non_snake_case, unused)]
use super::kafa1_baglam::Kafa;
use super::kafa1_js::Nk;
use crate::model::Oyuncu;
impl Kafa {
    /// decide.js ucusT(L, arrive)
    pub fn ucusT(&self, L: f64, arrive: f64, al: Option<f64>) -> f64 { todo!() }
    /// decide.js KUYU_HED()
    pub fn KUYU_HED(&self) -> bool { todo!() }
    /// decide.js KY()
    pub fn KY(&self) -> bool { todo!() }
    /// decide.js SUR()
    pub fn SUR(&self) -> bool { todo!() }
    /// decide.js siddetFor(h, to, tR, src, ch)
    pub fn siddetFor(&self, h: &Oyuncu, to: Nk, tR: f64, src: &[Oyuncu], ch: f64) -> Option<f64> { todo!() }
}
