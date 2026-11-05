//! Birebir aynılık: maçı kütüphane arayüzünden oynatıp her tikin parmak izini (motor::parmak) kayıtlı
//! başvuruyla (sina/ayni/*.pk, `sina/ayni.sh --yaz` üretir) karşılaştırır. `sina/ayni.sh` ile aynı ölçüt;
//! bu sınama onun kısa sürümüdür (cargo test'in de davranış korumasını denetlemesi için).
//! Davranışı bilerek değiştiren bir commit başvuruları `sina/ayni.sh --yaz` ile yeniler.
use alan_motor::mac::motor::Motor;
use alan_motor::oyun::MacMotoru;
use alan_motor::parmak::parmak_izi;

fn kok(yol: &str) -> String {
    format!("{}/../{}", env!("CARGO_MANIFEST_DIR"), yol)
}

fn denetle(mac: &str, tik: u32) {
    let an: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(kok(&format!("../anlar/{mac}.json"))).unwrap()).unwrap();
    let kurulum = an.get("kurulum").cloned().unwrap_or(an);
    let basvuru = std::fs::read_to_string(kok(&format!("sina/ayni/{mac}.pk"))).unwrap();
    let mut mt = Motor::kayittan(&kurulum).expect("maç kurulamadı");
    for satir in basvuru.lines().take(tik as usize) {
        mt.adim();
        let beklenen = satir.split_whitespace().nth(1).unwrap();
        let bulunan = format!("{:016x}", parmak_izi(&mt.m));
        assert_eq!(bulunan, beklenen, "{mac}: tik {} ayrıştı (sina/ayni.sh ile ayrıntı)", mt.tick());
    }
}

/// Sürüm derlemesinde 300 tik (~4 sn); hata ayıklama derlemesinde kısa (motor orada çok yavaş).
fn tik() -> u32 {
    if cfg!(debug_assertions) { 20 } else { 300 }
}

#[test]
fn kontra_parmak_izi_basvuruyla_ayni() {
    denetle("a-107-1-3464-kontra", tik());
}
