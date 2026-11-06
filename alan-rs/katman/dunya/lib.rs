//! Aşama 7c: katman ayrı crate. Kaynak dosyalar yerinde (`motor/src`), burası yalnız crate kökü:
//! katmanın kendi modülü `#[path]` ile, alt katmanlar `crate::` yolları değişmesin diye yeniden dışa verilir.
//! Bu crate'in bağımlılıkları (Cargo.toml) katman kuralının kendisi: üst katmana bağlanmak derlenmez.

pub use alan_temel::*;

#[path = "../../motor/src/dunya/mod.rs"]
pub mod dunya;
