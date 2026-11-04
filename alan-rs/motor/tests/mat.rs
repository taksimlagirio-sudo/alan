//! sabit-mat.js ile bit bit aynılık (sina/mat-vektor.js'in ürettiği vektörler).
use alan_motor::mat::*;

fn h(s: &str) -> f64 { f64::from_bits(u64::from_str_radix(s, 16).unwrap()) }
fn ayni(a: f64, b: f64) -> bool { a.to_bits() == b.to_bits() || (a != a && b != b) }

#[test]
fn js_ile_bit_bit_ayni() {
    let s = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../sina/mat-vektor.txt")).unwrap();
    let ad = ["exp", "log", "pow", "powi", "sin", "cos", "atan", "atan2", "acos", "tanh", "hypot", "round", "max", "min"];
    let mut hata = [0usize; 14];
    let mut n = 0;
    for l in s.lines() {
        let v: Vec<f64> = l.split(' ').map(h).collect();
        let (x, y) = (v[0], v[1]);
        let r = [exp(x), log(x.abs()), pow(x.abs(), y % 7.0), pow(x, jround(y) % 9.0), sin(x), cos(x), atan(x), atan2(x, y), acos(kis(x / 10.0, -1.0, 1.0)), tanh(x), hypot(x, y), jround(x), jmax(x, y), jmin(x, y)];
        for i in 0..14 { if !ayni(r[i], v[i + 2]) { if hata[i] < 3 { eprintln!("{} x={:e} y={:e} rust={:e} js={:e}", ad[i], x, y, r[i], v[i + 2]); } hata[i] += 1; } }
        n += 1;
    }
    for i in 0..14 { assert_eq!(hata[i], 0, "{}: {} / {} farklı", ad[i], hata[i], n); }
}
