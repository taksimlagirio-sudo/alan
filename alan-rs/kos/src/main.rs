//! alan-kos: kayıttan maç oynat. Kullanım: alan-kos <kayit.json> <tik> [iz|ozet]
//! iz: her tik bir satır (sina/istatistik.js okur); ozet: süre ve skor;
//! kare <cikti.json>: izleme ekranı için bütün kareler (oyuncu yerleri, top, işler, olaylar).
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
    let mut isler = alan_motor::oyun::IsSozluk::default();
    let mut kareler: Vec<String> = Vec::new();
    let mut olaylar: Vec<String> = Vec::new();
    let r2 = |v: f32| if v.is_finite() { format!("{}", (v * 100.0).round() / 100.0) } else { "null".into() };
    let t0 = std::time::Instant::now();
    let out = std::io::stdout();
    let mut out = std::io::BufWriter::new(out.lock());
    let mut olay_say = mt.m.events.len();
    if kip == "durum" {
        writeln!(out, "{}", durum(&mt.m)).unwrap();
    }
    while mt.tick() < tik && !mt.bitti() {
        mt.adim();
        let s = mt.iz();
        if kip == "iz" {
            writeln!(out, "{}", s.json()).unwrap();
        }
        if kip == "durum" {
            let m = &mt.m;
            writeln!(out, "{}", durum(m)).unwrap();
        }
        if kip == "kare" {
            let k = mt.kare(&mut isler);
            let mut t = format!("[{},{},{},{},{},{}", k.tick, k.sahip, r2(k.top_x), r2(k.top_y), k.skor[0], k.skor[1]);
            for o in &k.oyuncular {
                t.push_str(&format!(",{},{},{},{}", r2(o.x), r2(o.y), o.is, r2(o.en)));
            }
            t.push(']');
            kareler.push(t);
            let m = &mt.m;
            let yeni = m.events.len().saturating_sub(olay_say);
            for e in m.events.iter().take(yeni).rev() {
                olaylar.push(format!("[{},{},{}]", m.tick, e.team, serde_json::to_string(&e.text).unwrap()));
            }
            olay_say = m.events.len();
        }
    }
    let dt = t0.elapsed().as_secs_f64();
    if kip == "karar" {
        if let Some(d) = &mt.m.lastDec {
            println!("karar {} ({},{})", d.name, d.x, d.y);
            for o in &d.opts {
                println!("  {:<40} q={:<8} v={:.6} P={:.4} to={:?}", o.kind, o.q, o.v, o.P, o.to.as_ref().map(|t| (t.x, t.y)));
            }
        }
    }
    if kip == "oyuncu" {
        let k: usize = a[4].parse().unwrap();
        let mut p = mt.m.ps[k].clone();
        p.yet = Default::default();
        println!("{:#?}", p);
    }
    if kip == "kare" {
        let m = &mt.m;
        let yol = a.get(4).expect("kare <cikti.json>");
        let ad: Vec<String> = m.ps.iter().map(|p| serde_json::to_string(&p.name).unwrap()).collect();
        let tk: Vec<String> = m.ps.iter().map(|p| format!("{}", p.team)).collect();
        let rl: Vec<String> = m.ps.iter().map(|p| serde_json::to_string(&p.role).unwrap()).collect();
        let is: Vec<String> = isler.yazilar.iter().map(|j| serde_json::to_string(j).unwrap()).collect();
        let j = format!("{{\"motor\":\"Rust motoru\",\"ad\":[{}],\"takim\":[{}],\"rol\":[{}],\"isler\":[{}],\"olay\":[{}],\"k\":[{}]}}", ad.join(","), tk.join(","), rl.join(","), is.join(","), olaylar.join(","), kareler.join(","));
        std::fs::write(yol, j).expect("yazılamadı");
    }
    let m = &mt.m;
    eprintln!("tik {} · skor {}-{} · {:.1} sn ({:.2} ms/tik)", m.tick, m.score[0], m.score[1], dt, 1e3 * dt / (m.tick.max(1.0)));
}


/// Bir tikin tam durumu (sina karşılaştırması için; durum-js.js ile aynı biçim)
fn durum(m: &alan_motor::model::Mac) -> String {
    use serde_json::{json, Value};
    let o = |v: Option<f64>| v.map_or(Value::Null, |x| json!(x));
    let f = |x: f64| if x.is_finite() { json!(x) } else { Value::Null };
    let ps: Vec<Value> = m
        .ps
        .iter()
        .map(|p| json!([f(p.x), f(p.y), f(p.vx), f(p.vy), o(p.en), o(p.den), f(p.tx), f(p.ty), p.job.clone().unwrap_or_default(), p._sf.as_ref().map_or(String::new(), |s| s.f.clone()), o(p.ca)]))
        .collect();
    let b = m.ball.as_ref().map_or(Value::Null, |b| json!([b.x, b.y, b.vx, b.vy]));
    json!({"t": m.tick, "h": m.holder.map_or(-1, |h| h as i64), "ch": m.ch, "r": m.r.s, "b": b, "p": ps}).to_string()
}
