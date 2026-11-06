//! Aşama 7c: katman ayrı crate. Kaynak dosyalar yerinde (`motor/src`), burası yalnız crate kökü:
//! katmanın kendi modülü `#[path]` ile, alt katmanlar `crate::` yolları değişmesin diye yeniden dışa verilir.
//! Bu crate'in bağımlılıkları (Cargo.toml) katman kuralının kendisi: üst katmana bağlanmak derlenmez.

pub use alan_temel::*;
pub use alan_kural::kural;
pub use alan_dunya::dunya;
pub use alan_kafa::kafa;

#[path = "../../motor/src/sekil/mod.rs"]
pub mod sekil;
