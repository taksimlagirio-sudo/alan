//! ALAN maç motoru.
//!
//! JS motorunun (alan-v5) Rust karşılığı. Yalnız bugünkü davranış taşınır: `_ESKI` deney
//! dalları yok. Yapı ilkeleri `alan-v5/TASARIM-yapi.md`'de: dünya tek veri yapısında,
//! gizli durum yok, önbellekler sıradan bağımsız, kafa = maç tek işlev, belirlenimli matematik.
//!
//! Aşama 7c: katmanlar ayrı crate (`katman/`); bu crate hepsini eski yollarla yeniden dışa verir.

pub use alan_temel::*;
pub use alan_kural::kural;
pub use alan_dunya::dunya;
pub use alan_kafa::kafa;
pub use alan_sekil::sekil;
pub use alan_mac::mac;
