//! Dönüş (Turn): JS ile bit bit aynılık (sina/donus-vektor.js).
use alan_motor::donus::*;
use alan_motor::oyuncu::*;
use alan_motor::yetenek::{Ham, Yetenek};
use serde_json::Value;
fn h(v: &Value) -> f64 { match v.as_str() { Some(s) => f64::from_bits(u64::from_str_radix(s, 16).unwrap()), None => f64::NAN } }
fn ho(v: &Value) -> Option<f64> { if v.is_null() { None } else { Some(h(v)) } }
fn ayni(a: f64, b: f64) -> bool { a.to_bits() == b.to_bits() || (a != a && b != b) }
fn oyuncu(v: &Value) -> Oyuncu {
    let mut a = Ham::default();
    for (k, x) in v["a"].as_object().unwrap() { a.v.insert(k.clone(), h(x)); }
    let mut p = Oyuncu::default();
    p.id = v["id"].as_u64().unwrap() as f64; p.team = v["team"].as_u64().unwrap() as f64;
    p.role = if v["bek"].as_bool().unwrap() { "Bekçi".into() } else { "Orta".into() };
    p.yet = if v["a"].as_object().unwrap().is_empty() { Yetenek::bos() } else { Yetenek::kur(&a) };
    p.x = h(&v["x"]); p.y = h(&v["y"]); p.vx = h(&v["vx"]); p.vy = h(&v["vy"]);
    p.en = ho(&v["en"]); p.den = ho(&v["den"]);
    p
}
fn sonuc(s: &str) -> DonusSonuc { match s { "döndü" => DonusSonuc::Dondu, "fiske kaçtı" => DonusSonuc::FiskeKacti, _ => DonusSonuc::RakipAldi } }
#[test]
fn js_ile_bit_bit_ayni() {
    let s = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../sina/donus-vektor.json")).unwrap();
    let v: Value = serde_json::from_str(&s).unwrap();
    let mut hata = Vec::new();
    for (ci, c) in v.as_array().unwrap().iter().enumerate() {
        let hp = oyuncu(&c["h"]); let opps: Vec<Oyuncu> = c["opps"].as_array().unwrap().iter().map(oyuncu).collect();
        let (a0, a1) = (h(&c["a0"]), h(&c["a1"]));
        if !ayni(omega(&hp, false, 0.35), h(&c["om"][0])) || !ayni(omega(&hp, true, 0.35), h(&c["om"][1])) { hata.push(format!("{ci} omega")); }
        if !ayni(arc(a0, a1, 1.0), h(&c["arc"])) { hata.push(format!("{ci} arc")); }
        for (i, (m, d)) in [(Mod::Dolas, 1.0), (Mod::Dolas, -1.0), (Mod::Fiske, 1.0), (Mod::Fiske, -1.0)].iter().enumerate() {
            let (r, t, k) = simulate(&hp, a0, a1, *m, *d, &opps, None, 0.35); let j = &c["sims"][i];
            if r != sonuc(j[0].as_str().unwrap()) || t.map(|x| x.to_bits()) != ho(&j[1]).map(|x| x.to_bits()) || !ayni(k, h(&j[2])) { hata.push(format!("{ci} simulate {i}: {r:?} {t:?} {k} js {j}")); }
        }
        let rr: Vec<f64> = c["rnd2"].as_array().unwrap().iter().map(h).collect(); let mut ri = 0; let mut f = || { let v = rr[ri]; ri += 1; v };
        let _ = &mut f; let _ = rr.len();
        let rr: Vec<f64> = c["rnd"].as_array().unwrap().iter().map(h).collect(); let mut ri = 0; let mut rnd = || { let v = rr[ri]; ri += 1; v };
        let p = choose(&hp, a0, a1, &opps, &mut rnd, 0.35); let j = &c["ch"];
        let md = if j[0].as_str() == Some("fiske") { Mod::Fiske } else { Mod::Dolas };
        if p.mode != md || p.dir != j[1].as_f64().unwrap() || !ayni(p.keep, h(&j[2])) || !ayni(p.cost, h(&j[3])) { hata.push(format!("{ci} choose {p:?} js {j}")); }
    }
    assert!(hata.is_empty(), "{} hata:\n{}", hata.len(), hata[..hata.len().min(15)].join("\n"));
}
