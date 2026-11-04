//! ALAN maç motoru.
//!
//! JS motorunun (alan-v5) Rust karşılığı. Yalnız bugünkü davranış taşınır: `_ESKI` deney
//! dalları yok. Yapı ilkeleri `alan-v5/TASARIM-yapi.md`'de: dünya tek veri yapısında,
//! gizli durum yok, önbellekler sıradan bağımsız, kafa = maç tek işlev, belirlenimli matematik.

pub mod alan;
pub mod core_sabit;
pub mod donus;
pub mod mat;
pub mod model;
pub mod oyuncu;
pub mod rng;
pub mod temas;
pub mod saha;
pub mod sekil;
pub mod top;
pub mod yetenek;
mod yetenek_tablo;
