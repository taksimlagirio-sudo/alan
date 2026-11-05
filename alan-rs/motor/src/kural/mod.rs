//! Kural katmanı (MIMARI.md): maçın kuralları, saf işlevler. Yalnız temel katmana (mat, yetenek, temas,
//! oyuncu, model'in veri tipleri) bağlıdır; mac, sekil, kafa'ya bağlanmaz.
#![allow(non_snake_case, non_upper_case_globals, clippy::too_many_arguments)]
pub mod enerji;
pub mod gec;
pub mod hareket;
pub mod ikili;
pub mod kos;
pub mod sut;

/// Bekleyen taşıyıcının beklemeyi bozduğu en yakın rakip uzaklığı (br).
pub const BEKLE_BOZ: f64 = 3.5;
