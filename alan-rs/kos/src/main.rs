//! alan-kos: kayıttan maç oynat. Kullanım: alan-kos <kayit.json> <tik> [iz|ozet]
//! iz: her tik bir satır (sina/istatistik.js okur); ozet: süre ve skor.
use alan_motor::mac::motor::Motor;
use alan_motor::oyun::MacMotoru;
use std::io::Write;

fn main() {
    let a: Vec<String> = std::env::args().collect();
    if a.len() < 3 {
        eprintln!("kullanım: alan-kos <kayit.json> <tik> [iz|ozet]");
        std::process::exit(2);
    }
    let metin = std::fs::read_to_string(&a[1]).expect("kayıt okunamadı");
    let v: serde_json::Value = serde_json::from_str(&metin).expect("kayıt JSON değil");
    let tik: u32 = a[2].parse().expect("tik");
    let kip = a.get(3).map(|s| s.as_str()).unwrap_or("ozet");
    let mut mt = Motor::kayittan(&v).expect("maç kurulamadı");
    let t0 = std::time::Instant::now();
    let out = std::io::stdout();
    let mut out = std::io::BufWriter::new(out.lock());
    while mt.tick() < tik && !mt.bitti() {
        mt.adim();
        let s = mt.iz();
        if kip == "iz" {
            writeln!(out, "{}", s.json()).unwrap();
        }
    }
    let dt = t0.elapsed().as_secs_f64();
    let m = &mt.m;
    eprintln!("tik {} · skor {}-{} · {:.1} sn ({:.2} ms/tik)", m.tick, m.score[0], m.score[1], dt, 1e3 * dt / (m.tick.max(1.0)));
}
