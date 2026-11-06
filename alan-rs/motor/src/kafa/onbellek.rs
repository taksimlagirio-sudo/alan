//! Kafanın önbellekleri, ömürlerine göre (MIMARI.md 3e-4). Eskiden `Kafa`'ya ve `Kafa2Durum`'a dağınıktı; tik
//! ömürlüler her biri kendi "son tik" etiketiyle tembelce temizleniyordu.
//!
//! - **tik ömürlü** (`ge`, `rnb`, `pwm`, `bsk`): değer o tikin maç durumuna bağlı, anahtarda tik yok. Kafanın tiki
//!   (`bag._tick`) yalnız `kafa_hazirla`'da yazılır; orada `tik_basi` tik değiştiyse hepsini birlikte temizler
//!   (eskisiyle aynı: her biri ilk erişimde etiketi tikle karşılaştırıp temizliyordu).
//! - **görünüm ömürlü** (`imkk`, `it`): kafanın maç görünümünün (`k.mac`) tikine bağlı. Görünüm bir tik içinde
//!   geçici olarak değişebildiği için (kafa2_zemin::ileriKur tik 0'lık bir görünüm kurar) etiketleri kendinde.
//! - **kalıcı** (`ucus`, `suty`, `sht_ek`, `imkb`): değer yalnız anahtardan (saf); boyutu sınırlı, temizlemek
//!   sonucu değiştirmez.
use std::cell::{Cell, RefCell};
use std::sync::Arc;

use super::kafa1_baglam::UcusYol;
use super::kafa2_baglam::PwE;
use crate::dunya::Op;
use crate::HizliMap;

#[derive(Default)]
pub struct Onbellek {
    /// tik ömürlülerin tiki (`bag._tick`)
    pub tik: Option<f64>,
    /// JS `GEk` (girEsik), `GEt` (yer değeri)
    pub ge: RefCell<(HizliMap<[u64; 7], f64>, HizliMap<[u64; 3], f64>)>,
    /// girEsik iç içe çağrı koruması (önbellek değil)
    pub ge_ic: Cell<bool>,
    /// JS `RNB` (runSut)
    pub rnb: RefCell<HizliMap<[u64; 6], f64>>,
    /// pwKos: oyuncu nesnesi (kimlik) → koşu satırları
    pub pwm: HizliMap<usize, (Op, Vec<PwE>)>,
    /// bskTab
    pub bsk: HizliMap<String, Vec<f64>>,
    /// imkKov: görünümün tiki → "team|id" → (tepki, hız)
    pub imkk: (Option<Option<f64>>, HizliMap<(u64, u64), (f64, f64)>),
    /// ileriTab: (görünümün tiki, anahtar, tablo)
    pub it: Option<(f64, String, Option<Arc<super::kafa2_zemin::IleriTab>>)>,
    /// JS `UCUS`: varış hızı (bit) → uzunluk (1..80) → yol
    pub ucus: RefCell<HizliMap<u64, Vec<Option<Arc<UcusYol>>>>>,
    /// JS `SUTY`: hız (1/20) → yol birikimi
    pub suty: RefCell<HizliMap<i64, Arc<Vec<f64>>>>,
    /// JS `SHT`'nin tabloda olmayan hücreleri (hesaplanmış)
    pub sht_ek: RefCell<HizliMap<i64, f64>>,
    /// topun yolu (`top::tahmin`, seçimli K ile): girdinin tamamı (top + alan kaynakları + adım sınırı) → sonuç.
    /// Saf; bir tikte aynı top aynı dünyada çok kez tahmin ediliyor (ölçüm: m00, çağrıların %38'i).
    pub tahmin: RefCell<HizliMap<Vec<u64>, Arc<(Vec<crate::top::YolNokta>, crate::top::Top)>>>,
    /// imkBol (tablo × konum)
    pub imkb: HizliMap<[u64; 4], Arc<std::sync::Mutex<super::kafa2_imkan::ImkBol>>>,
}
impl Onbellek {
    /// Kafanın tiki `t` oldu: tik ömürlüler tik değiştiyse birlikte temizlenir.
    pub fn tik_basi(&mut self, t: Option<f64>) {
        if self.tik != t {
            let g = self.ge.get_mut();
            g.0.clear();
            g.1.clear();
            self.rnb.get_mut().clear();
            self.pwm.clear();
            self.bsk.clear();
            self.tahmin.get_mut().clear();
            self.tik = t;
        }
    }
}
