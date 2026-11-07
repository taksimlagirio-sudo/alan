//! A2 kontra değer tablosu (analiz/TASARIM-BILGI.md; biçim: bilgi tabloları, bicim 1): kaleye uzaklık × topun önündeki rakip
//! saha oyuncusu × top kazanılalı geçen süre → "N sn içinde gol". Hız kararı (enerji::sprint_karar3, hiz2 + ALAN_BILGI_KONTRA_IZ
//! ya da ALAN_KTAB_IZ) kontra koşusunun değerini buradan okur. Dosya motor/veri/bilgi/kontra.json (derlemeye gömülü, wasm'da da);
//! üretim ve eskime denetimi sina/bilgi.sh (tarif sina/bilgi/kontra.conf, üretici sina/bilgi/kontra.py). Okuma ve harmanlama
//! kafa2_bilgi::Bilgi'nin (çoklu doğrusal; (n·tablo + k·model)/(n + k)); model: enerji::kontra_deger (0/1/2/3+ savunmacı).
//! Yerelde ALAN_KONTRA_TABLO=<yol> ile başka bir bicim 1 dosyası okunur.
use crate::kafa::kafa2_bilgi::Bilgi;

fn coz(metin: &str) -> Option<Bilgi> {
    let j: serde_json::Value = serde_json::from_str(metin).ok()?;
    if j.get("bicim").and_then(|b| b.as_i64()) != Some(1) {
        eprintln!("bilgi/kontra.json: bilinmeyen biçim");
        return None;
    }
    let sayi = |v: &serde_json::Value| -> Vec<f64> { v.as_array().map(|a| a.iter().filter_map(|x| x.as_f64()).collect()).unwrap_or_default() };
    let eksen: Vec<(String, Vec<f64>)> = j["eksen"].as_array()?.iter().filter_map(|e| Some((e["ad"].as_str()?.to_string(), sayi(&e["dugum"])))).collect();
    let (deger, n) = (sayi(&j["deger"]), sayi(&j["n"]));
    let boy: usize = eksen.iter().map(|e| e.1.len()).product();
    if boy == 0 || boy != deger.len() || boy != n.len() {
        eprintln!("bilgi/kontra.json: eksen ile değer/n boyu uyuşmuyor");
        return None;
    }
    Some(Bilgi { ad: "kontra".into(), eksen, deger, n, k: j["k"].as_f64().unwrap_or(30.0) })
}

/// Gömülü tablo (yerelde ALAN_KONTRA_TABLO=<yol> ile başka dosya).
pub fn tablo() -> Option<&'static Bilgi> {
    static T: std::sync::OnceLock<Option<Bilgi>> = std::sync::OnceLock::new();
    T.get_or_init(|| {
        if crate::iz_acik() {
            if let Ok(yol) = std::env::var("ALAN_KONTRA_TABLO") {
                if let Ok(m) = std::fs::read_to_string(&yol) {
                    return coz(&m);
                }
            }
        }
        coz(include_str!("../../veri/bilgi/kontra.json"))
    })
    .as_ref()
}

/// Kontra durumunun değeri: (kaleye uzaklık br, öndeki savunmacı (yumuşak sayı), geçen süre sn); `model`: tablosuz değer.
pub fn deger(uzak: f64, onde: f64, sure: f64, model: f64) -> f64 {
    match tablo() {
        Some(t) => t.harmanla(&[("uzaklik", uzak), ("onde", onde), ("sure", sure)], model),
        None => model,
    }
}

/// Deney (ALAN_BILGI_KONTRA_IZ ya da ALAN_KTAB_IZ; ölçümde/wasm'da KTAB_ZORLA): hiz2'de kontra koşusunun değeri bu tablodan.
/// Varsayılan kapalı.
pub static KTAB_ZORLA: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub fn ktab() -> bool {
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    KTAB_ZORLA.load(std::sync::atomic::Ordering::Relaxed)
        || *A.get_or_init(|| crate::iz_acik() && (std::env::var("ALAN_BILGI_KONTRA_IZ").is_ok() || std::env::var("ALAN_KTAB_IZ").is_ok()))
}
