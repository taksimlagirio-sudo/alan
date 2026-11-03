//! ALAN maç motoru.
//!
//! JS motorunun (alan-v5) Rust karşılığı. Yalnız bugünkü davranış taşınır: `_ESKI` deney
//! dalları yok. Yapı ilkeleri `alan-v5/TASARIM-yapi.md`'de: dünya tek veri yapısında,
//! gizli durum yok, önbellekler sıradan bağımsız, kafa = maç tek işlev, belirlenimli matematik.

pub mod mat;
