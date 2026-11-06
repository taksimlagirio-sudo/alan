//! Aşama 7c: katman ayrı crate. Kaynak dosyalar yerinde (`motor/src`), burası yalnız crate kökü:
//! temelin modülleri `#[path]` ile.
//! Bu crate'in bağımlılıkları (Cargo.toml) katman kuralının kendisi: üst katmana bağlanmak derlenmez.

#[path = "../../motor/src/alan.rs"]
pub mod alan;
#[path = "../../motor/src/core_sabit.rs"]
pub mod core_sabit;
#[path = "../../motor/src/donus.rs"]
pub mod donus;
#[path = "../../motor/src/dunya_tip.rs"]
pub mod dunya_tip;
#[path = "../../motor/src/mat.rs"]
pub mod mat;
#[path = "../../motor/src/mac_veri.rs"]
pub mod mac_veri;
#[path = "../../motor/src/model.rs"]
pub mod model;
#[path = "../../motor/src/oyun.rs"]
pub mod oyun;
#[path = "../../motor/src/paket.rs"]
pub mod paket;
#[path = "../../motor/src/parmak.rs"]
pub mod parmak;
#[path = "../../motor/src/paralel.rs"]
pub mod paralel;
#[path = "../../motor/src/oyuncu.rs"]
pub mod oyuncu;
#[path = "../../motor/src/rng.rs"]
pub mod rng;
#[path = "../../motor/src/temas.rs"]
pub mod temas;
#[path = "../../motor/src/saha.rs"]
pub mod saha;
#[path = "../../motor/src/top.rs"]
pub mod top;
#[path = "../../motor/src/veri.rs"]
pub mod veri;
#[path = "../../motor/src/yetenek.rs"]
pub mod yetenek;
#[path = "../../motor/src/yetenek_tablo.rs"]
mod yetenek_tablo;

/// İz kancaları (ALAN_*_IZ / ALAN_IMK_HU) açık mı: bir kez okunur, sıcak yolda ortam değişkeni aranmaz.
/// `#[inline]`: wasm'da sabit `false`; crate sınırını geçip açılınca iz kodu wasm'dan atılır (Aşama 7c'den beri gerekli).
#[inline]
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
