//! Koşu ve tepki kuralları (saf): sprintle koşu hızı, algı/kovalama tepkisi. Maçın bağlamı `&dyn Kural`
//! (çekirdeğin arayüzü) olarak girer; yerleşim ve kafa aynı işlevi çağırır.
use crate::kural::enerji::{spd, spre_bar};
use crate::mat::{jmax, jround};
use crate::model::Oyuncu;
use crate::oyuncu::{tepki as tepki_k, Tepki};
use crate::temas::Kural;

/// JS ALAN_SPRE (core.js): barlı oyuncuda (Enerji 2/3 açık) sprintin süresi bara bağlı (EN2_SPRE), barsızda
/// maçın sprint katı (ALAN_SPR = `k.spr`).
pub fn spre_k(p: &Oyuncu, k: &dyn Kural) -> f64 {
    let sp = k.spr(p);
    if p.en.is_none() {
        return sp;
    }
    spre_bar(p, None, sp)
}

/// JS `vSp(p)`: sprintle koşu hızı (br/tik).
#[inline]
pub fn v_sp(p: &Oyuncu, k: &dyn Kural) -> f64 {
    spd(p) * spre_k(p, k)
}

/// JS ALAN_TEPKI(p, k) (Enerji 3 katıyla).
#[inline]
pub fn TEPKI(p: &Oyuncu, t: Tepki, k: &dyn Kural) -> f64 {
    tepki_k(p, t, &k.tepki_s())
}

/// JS AlanShape.tepki: algı gecikmesi + kovalama tepkisi.
pub fn tepki(o: &Oyuncu, k: &dyn Kural) -> f64 {
    jmax(1.0, jround(TEPKI(o, Tepki::Algi, k))) + jmax(0.0, TEPKI(o, Tepki::Kovala, k))
}
/// JS `lagOf`.
pub fn lag_of(o: &Oyuncu, k: &dyn Kural) -> f64 {
    jmax(1.0, jround(TEPKI(o, Tepki::Algi, k)))
}
