use alan_motor::rng::Rng;
#[test]
#[ignore = "JS karşılaştırması: vektör sina/rng-vektor.js ile üretilince `cargo test --release -- --ignored`; Rust tek doğru kaynak (MIMARI.md Aşama 6)"]
fn js_ile_ayni() {
    let s = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../sina/rng-vektor.json")).unwrap();
    let v: Vec<String> = serde_json::from_str(&s).unwrap();
    let mut r = Rng::yeni(909623);
    for i in 0..2000 { assert_eq!(r.sonraki().to_bits(), u64::from_str_radix(&v[i], 16).unwrap(), "{i}"); }
    let mut r = Rng::yeni(1);
    for i in 0..10 { assert_eq!(r.sonraki().to_bits(), u64::from_str_radix(&v[2000 + i], 16).unwrap()); }
}
