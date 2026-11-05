//! ALAN maç motoru.
//!
//! JS motorunun (alan-v5) Rust karşılığı. Yalnız bugünkü davranış taşınır: `_ESKI` deney
//! dalları yok. Yapı ilkeleri `alan-v5/TASARIM-yapi.md`'de: dünya tek veri yapısında,
//! gizli durum yok, önbellekler sıradan bağımsız, kafa = maç tek işlev, belirlenimli matematik.

pub mod alan;
pub mod core_sabit;
pub mod donus;
pub mod dunya;
pub mod kafa;
pub mod kural;
pub mod mat;
pub mod mac;
pub mod model;
pub mod oyun;
pub mod paket;
pub mod parmak;
pub mod paralel;
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

/// Önbellekler için hızlı karma (FxHash yöntemi). Yalnız arama yapılan sözlüklerde kullanılır;
/// gezme sırası maçı etkilemez, sonuçlar aynıdır. Varsayılan SipHash saldırıya dayanıklı ama yavaş.
#[derive(Default, Clone, Copy)]
pub struct HizliKarma(u64);
impl std::hash::Hasher for HizliKarma {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        for c in bytes.chunks(8) {
            let mut b = [0u8; 8];
            b[..c.len()].copy_from_slice(c);
            self.write_u64(u64::from_le_bytes(b));
        }
    }
    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.0 = (self.0.rotate_left(5) ^ i).wrapping_mul(0x51_7c_c1_b7_27_22_0a_95);
    }
    #[inline]
    fn write_u8(&mut self, i: u8) { self.write_u64(i as u64) }
    #[inline]
    fn write_u32(&mut self, i: u32) { self.write_u64(i as u64) }
    #[inline]
    fn write_i64(&mut self, i: i64) { self.write_u64(i as u64) }
    #[inline]
    fn write_usize(&mut self, i: usize) { self.write_u64(i as u64) }
    #[inline]
    fn finish(&self) -> u64 { self.0 }
}
/// Hızlı karmalı sözlük.
pub type HizliMap<K, V> = std::collections::HashMap<K, V, std::hash::BuildHasherDefault<HizliKarma>>;

/// JS Map anahtarındaki sayının yerine geçen bit anahtarı: JS'te `String(x)` iki sayı için ancak
/// değerler eşitse aynıdır; yalnız -0 ile 0 ("0") ve bütün NaN'lar ("NaN") birleşir. Metin kurmadan aynı eşitlik.
#[inline]
pub fn js_sayi_k(x: f64) -> u64 {
    if x == 0.0 {
        0
    } else if x.is_nan() {
        f64::NAN.to_bits()
    } else {
        x.to_bits()
    }
}
