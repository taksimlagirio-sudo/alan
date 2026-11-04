//! ALAN maç motoru.
//!
//! JS motorunun (alan-v5) Rust karşılığı. Yalnız bugünkü davranış taşınır: `_ESKI` deney
//! dalları yok. Yapı ilkeleri `alan-v5/TASARIM-yapi.md`'de: dünya tek veri yapısında,
//! gizli durum yok, önbellekler sıradan bağımsız, kafa = maç tek işlev, belirlenimli matematik.

pub mod alan;
pub mod core_sabit;
pub mod donus;
pub mod kafa;
pub mod mat;
pub mod mac;
pub mod model;
pub mod oyun;
pub mod oyuncu;
pub mod rng;
pub mod temas;
pub mod saha;
pub mod sekil;
pub mod top;
pub mod veri;
pub mod yetenek;
mod yetenek_tablo;

/// İz kancaları (ALAN_*_IZ / ALAN_IMK_HU) açık mı: bir kez okunur, sıcak yolda ortam değişkeni aranmaz.
pub fn iz_acik() -> bool {
    // wasm32-unknown-unknown'da ortam değişkeni listesi yok (std::env::vars panikler)
    if cfg!(target_family = "wasm") {
        return false;
    }
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *A.get_or_init(|| std::env::vars().any(|(k, _)| k.starts_with("ALAN_") && (k.ends_with("_IZ") || k == "ALAN_IMK_HU")))
}
