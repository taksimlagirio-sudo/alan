//! Yetenekler: JS ile bit bit aynılık (sina/yetenek-vektor.js).
use alan_motor::yetenek::*;
use serde_json::Value;
fn h(v: &Value) -> f64 { f64::from_bits(u64::from_str_radix(v.as_str().unwrap(), 16).unwrap()) }
fn ayni(a: f64, b: f64) -> bool { a.to_bits() == b.to_bits() }
#[test]
#[ignore = "JS karşılaştırması: vektör sina/yetenek-vektor.js ile üretilince `cargo test --release -- --ignored`; Rust tek doğru kaynak (MIMARI.md Aşama 6)"]
fn js_ile_bit_bit_ayni() {
    let s = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../sina/yetenek-vektor.json")).unwrap();
    let v: Value = serde_json::from_str(&s).unwrap();
    let mut hata = Vec::new();
    for (i, c) in v.as_array().unwrap().iter().enumerate() {
        let mut a = Ham::default();
        for (k, x) in c["a"].as_object().unwrap() { a.v.insert(k.clone(), h(x)); }
        let y = Yetenek::kur(&a);
        for (j, x) in c["oz"].as_array().unwrap().iter().enumerate() { if !ayni(y.oz[j], h(x)) { hata.push(format!("{i} oz {}: {} js {}", OZ_AD[j], y.oz[j], h(x))); } }
        for (j, x) in c["mek"].as_array().unwrap().iter().enumerate() { if !ayni(y.mek[j], h(x)) { hata.push(format!("{i} mek {}: {} js {}", MEK_AD[j], y.mek[j], h(x))); } }
        for (j, d) in [5.0, 15.0, 22.0, 35.0].iter().enumerate() { if !ayni(y.sut_y(*d), h(&c["sut"][j])) { hata.push(format!("{i} sutY {d}")); } }
        if !ayni(ah(&y), h(&c["ah"])) { hata.push(format!("{i} AH {} js {}", ah(&y), h(&c["ah"]))); }
        if !ayni(ag(&y, None, false, 0.35), h(&c["ag"])) { hata.push(format!("{i} AG")); }
    }
    assert!(hata.is_empty(), "{} hata, ilkleri:\n{}", hata.len(), hata[..hata.len().min(15)].join("\n"));
}
