//! JS anlamının küçük yardımcıları (AJAN.md madde 5) ve kafanın ortak küçük tipleri.

use crate::alan::{Etki, KOnbellek, Kaynak};
use crate::model::Oyuncu;
use std::cell::RefCell;

/// Bir nokta ({x, y}).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Nk {
    pub x: f64,
    pub y: f64,
}
#[inline(always)]
pub fn nk(x: f64, y: f64) -> Nk {
    Nk { x, y }
}

/// JS `v || 0` (NaN ve ±0 → +0).
#[inline(always)]
pub fn z0(v: f64) -> f64 {
    if v != 0.0 && v == v {
        v
    } else {
        0.0
    }
}
/// JS `a || b` (sayılar).
#[inline(always)]
pub fn ya(a: f64, b: f64) -> f64 {
    if a != 0.0 && a == a {
        a
    } else {
        b
    }
}
/// JS `x || 1e-6` gibi: Option<f64> → `(x ?? undefined) || b`.
#[inline(always)]
pub fn yao(a: Option<f64>, b: f64) -> f64 {
    match a {
        Some(v) => ya(v, b),
        None => b,
    }
}

/// JS kimlik karşılaştırması (`p === q`): iki başvuru aynı satır mı.
#[inline(always)]
pub fn ayni(a: &Oyuncu, b: &Oyuncu) -> bool {
    std::ptr::eq(a, b)
}
/// `src.indexOf(h)` (kimlikle).
pub fn sira(src: &[Op], h: &Oyuncu) -> Option<usize> {
    src.iter().position(|p| ayni(p, h))
}
/// `src.find(p => p.id === q.id && p.team === q.team)`.
pub fn bul<'a>(src: &'a [Op], q: &Oyuncu) -> Option<&'a Oyuncu> {
    src.iter().find(|p| p.id == q.id && p.team == q.team).map(|p| &**p)
}
pub fn bul_i(src: &[Op], q: &Oyuncu) -> Option<usize> {
    src.iter().position(|p| p.id == q.id && p.team == q.team)
}

/// İş adı önekle mi başlıyor (JS `/^(a|b)/.test(job || "")`).
pub fn is_bas(job: &Option<String>, onek: &[&str]) -> bool {
    match job {
        Some(j) => onek.iter().any(|o| j.starts_with(o)),
        None => false,
    }
}
/// İş adında geçiyor mu (JS `/a|b/.test(job || "")`).
pub fn is_icer(job: &Option<String>, parca: &[&str]) -> bool {
    match job {
        Some(j) => parca.iter().any(|o| j.contains(o)),
        None => false,
    }
}

/// JS `+x.toFixed(f)`: ondalık açılımın tam değeriyle yuvarlama, eşitlikte büyüğe (mutlak değerde).
pub fn to_fixed(x: f64, f: usize) -> f64 {
    if x != x || x.is_infinite() || x.abs() >= 1e21 {
        return x;
    }
    let ax = x.abs();
    let uzun = format!("{:.*}", f + 40, ax);
    let nokta = uzun.find('.').unwrap();
    let kuyruk = &uzun[nokta + 1 + f..];
    let es = kuyruk.starts_with('5') && kuyruk[1..].bytes().all(|c| c == b'0');
    let s = if es {
        // tam yarı: büyüğe yuvarla
        let kes = &uzun[..nokta + 1 + f];
        let kes = kes.trim_end_matches('.');
        let v: f64 = kes.parse().unwrap();
        let birim = 10f64.powi(-(f as i32));
        let r = format!("{:.*}", f, v + birim);
        r
    } else {
        format!("{:.*}", f, ax)
    };
    let v: f64 = s.parse().unwrap();
    if x < 0.0 {
        -v
    } else {
        v
    }
}

/// Alan değerinin (K) kipi: düz liste (JS paket: seçimli K) ya da dünyanın yarım birimlik önbelleği (JS `_kc`).
pub type Kc<'a> = Option<&'a RefCell<KOnbellek>>;

/// Oyuncu listesinin alan kaynakları.
pub fn etkiler(src: &[Op]) -> Vec<Etki> {
    src.iter().map(|p| p.etki()).collect()
}

/// `f`'i src'nin K kaynağıyla çalıştır (JS `pack(src)`: `_kc` varsa önbellekli).
pub fn kaynakla<R>(et: &[Etki], kc: Kc, f: impl FnOnce(&mut Kaynak) -> R) -> R {
    match kc {
        Some(c) => {
            let mut c = c.borrow_mut();
            let mut k = Kaynak::Onbellek(et, &mut c);
            f(&mut k)
        }
        None => {
            let mut k = Kaynak::Sec(et);
            f(&mut k)
        }
    }
}

/// JS `Math.tan` (sabit-mat'ta yok; JS de yerel `Math.tan` kullanır).
#[inline(always)]
pub fn tan(x: f64) -> f64 {
    x.tan()
}

/// JS `Math.max(...dizi)` (boşsa -Infinity).
pub fn enb(v: impl Iterator<Item = f64>) -> f64 {
    let mut m = f64::NEG_INFINITY;
    for x in v {
        m = crate::mat::jmax(m, x);
    }
    m
}

/// Dünyanın bir oyuncusu (kafa2 ile ortak): dünya kopyası yalnız değişen oyuncuyu kopyalar.
pub type Op = std::sync::Arc<Oyuncu>;
/// Hayali dünya.
pub type Dunya = Vec<Op>;
