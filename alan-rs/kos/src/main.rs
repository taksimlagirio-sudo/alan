//! alan-kos: kayıttan maç oynat. Kullanım: alan-kos <kayit.json> <tik> [iz|ozet]
//! iz: her tik bir satır (sina/istatistik.js okur); ozet: süre ve skor.

fn main() {
    let a: Vec<String> = std::env::args().collect();
    if a.len() < 3 {
        eprintln!("kullanım: alan-kos <kayit.json> <tik> [iz|ozet]");
        std::process::exit(2);
    }
    let _yol = &a[1];
    let _tik: u32 = a[2].parse().expect("tik");
    // Maç modülü birleşince: alan_motor::mac::Motor (MacMotoru) ile oynatılır.
    eprintln!("alan-kos: maç modülü henüz bağlı değil (birleştirme bekleniyor)");
    std::process::exit(1);
}
