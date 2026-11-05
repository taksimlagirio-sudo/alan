//! JS anlamının küçük yardımcıları (AJAN.md madde 5) ve kafanın ortak küçük tipleri.

use crate::alan::{Etki, KOnbellek, Kaynak};
use crate::model::Oyuncu;
use std::cell::RefCell;

/// Bir nokta ({x, y}): modelin ortak noktası (kafa2 ile tek tip).
pub use crate::model::Nokta as Nk;
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
/// Nesne kimliği (JS `a === b`): ikisi aynı nesne mi. Bir oyuncunun kopyası (hayali konum, cevap dünyası)
/// aslıyla aynı **değildir**; kimlik numarası `NaN` olan hayaletler de yalnız kendileriyle aynıdır.
/// `Op` (`Arc<Oyuncu>`) da verilebilir: aynı paylaşımlı nesne, aynı adres demektir. Kimlik envanteri: MIMARI.md 3.2.
pub fn ayni_nesne(a: &Oyuncu, b: &Oyuncu) -> bool {
    std::ptr::eq(a, b)
}
/// `src.indexOf(h)` (kimlikle).
pub fn sira(src: &[Op], h: &Oyuncu) -> Option<usize> {
    src.iter().position(|p| ayni_nesne(p, h))
}
/// `src.find(p => p.id === q.id && p.team === q.team)`.
pub fn bul<'a>(src: &'a [Op], q: &Oyuncu) -> Option<&'a Oyuncu> {
    src.iter().find(|p| p.id == q.id && p.team == q.team).map(|p| &**p)
}
pub fn bul_i(src: &[Op], q: &Oyuncu) -> Option<usize> {
    src.iter().position(|p| p.id == q.id && p.team == q.team)
}

/// İş adı önekle mi başlıyor (JS `/^(a|b)/.test(job || "")`).
pub fn is_bas(job: &Option<std::sync::Arc<str>>, onek: &[&str]) -> bool {
    match job {
        Some(j) => onek.iter().any(|o| j.starts_with(o)),
        None => false,
    }
}
/// İş adında geçiyor mu (JS `/a|b/.test(job || "")`).
pub fn is_icer(job: &Option<std::sync::Arc<str>>, parca: &[&str]) -> bool {
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

/// JS `Math.tan` (sabit-mat'ta yok; V8'in fdlibm tan'ı: __kernel_tan + __ieee754_rem_pio2).
pub fn tan(x: f64) -> f64 {
    let hx = (x.to_bits() >> 32) as i32;
    let ix = hx & 0x7fffffff;
    if ix <= 0x3fe921fb {
        k_tan(x, 0.0, 1)
    } else if ix >= 0x7ff00000 {
        x - x
    } else {
        match rem_pio2(x) {
            Some((n, y0, y1)) => k_tan(y0, y1, 1 - ((n & 1) << 1)),
            None => x.tan(),
        }
    }
}
fn hi(x: f64) -> i32 {
    (x.to_bits() >> 32) as i32
}
fn lo0(x: f64) -> f64 {
    f64::from_bits(x.to_bits() & 0xffff_ffff_0000_0000)
}
const T: [f64; 13] = [
    3.33333333333334091986e-01,
    1.33333333333201242699e-01,
    5.39682539762260521377e-02,
    2.18694882948595424599e-02,
    8.86323982359930005737e-03,
    3.59207910759131235356e-03,
    1.45620945432529025516e-03,
    5.88041240820264096874e-04,
    2.46463134818469906812e-04,
    7.81794442939557092300e-05,
    7.14072491382608190305e-05,
    -1.85586374855275456654e-05,
    2.59073051863633712884e-05,
];
fn k_tan(mut x: f64, mut y: f64, iy: i32) -> f64 {
    let hx = hi(x);
    let ix = hx & 0x7fffffff;
    if ix < 0x3e300000 && (x as i32) == 0 {
        let low = x.to_bits() as u32;
        if ((ix as u32 | low) | (iy + 1) as u32) == 0 {
            return 1.0 / x.abs();
        } else if iy == 1 {
            return x;
        } else {
            let w = x + y;
            let z = lo0(w);
            let v = y - (z - x);
            let a = -1.0 / w;
            let t = lo0(a);
            let s = 1.0 + t * z;
            return t + a * (s + t * v);
        }
    }
    if ix >= 0x3FE59428 {
        if hx < 0 {
            x = -x;
            y = -y;
        }
        let z = 7.85398163397448278999e-01 - x;
        let w = 3.06161699786838301793e-17 - y;
        x = z + w;
        y = 0.0;
    }
    let z = x * x;
    let w = z * z;
    let mut r = T[1] + w * (T[3] + w * (T[5] + w * (T[7] + w * (T[9] + w * T[11]))));
    let v = z * (T[2] + w * (T[4] + w * (T[6] + w * (T[8] + w * (T[10] + w * T[12])))));
    let s = z * x;
    r = y + z * (s * (r + v) + y);
    r += T[0] * s;
    let w = x + r;
    if ix >= 0x3FE59428 {
        let v = iy as f64;
        return (1 - ((hx >> 30) & 2)) as f64 * (v - 2.0 * (x - (w * w / (w + v) - r)));
    }
    if iy == 1 {
        w
    } else {
        let z = lo0(w);
        let v = r - (z - x);
        let a = -1.0 / w;
        let t = lo0(a);
        let s = 1.0 + t * z;
        t + a * (s + t * v)
    }
}
fn rem_pio2(x: f64) -> Option<(i32, f64, f64)> {
    const PIO2_1: f64 = 1.57079632673412561417e+00;
    const PIO2_1T: f64 = 6.07710050650619224932e-11;
    const PIO2_2: f64 = 6.07710050630396597660e-11;
    const PIO2_2T: f64 = 2.02226624879595063154e-21;
    const PIO2_3: f64 = 2.02226624871116645580e-21;
    const PIO2_3T: f64 = 8.47842766036889956997e-32;
    const INVPIO2: f64 = 6.36619772367581382433e-01;
    let hx = hi(x);
    let ix = hx & 0x7fffffff;
    if ix < 0x4002d97c {
        if hx > 0 {
            let mut z = x - PIO2_1;
            if ix != 0x3ff921fb {
                let y0 = z - PIO2_1T;
                return Some((1, y0, (z - y0) - PIO2_1T));
            }
            z -= PIO2_2;
            let y0 = z - PIO2_2T;
            return Some((1, y0, (z - y0) - PIO2_2T));
        } else {
            let mut z = x + PIO2_1;
            if ix != 0x3ff921fb {
                let y0 = z + PIO2_1T;
                return Some((-1, y0, (z - y0) + PIO2_1T));
            }
            z += PIO2_2;
            let y0 = z + PIO2_2T;
            return Some((-1, y0, (z - y0) + PIO2_2T));
        }
    }
    if ix <= 0x413921fb {
        let t = x.abs();
        let n = (t * INVPIO2 + 0.5) as i32;
        let fnn = n as f64;
        let mut r = t - fnn * PIO2_1;
        let mut w = fnn * PIO2_1T;
        let j = ix >> 20;
        let mut y0 = r - w;
        let i = j - ((hi(y0) >> 20) & 0x7ff);
        if i > 16 {
            let t = r;
            w = fnn * PIO2_2;
            r = t - w;
            w = fnn * PIO2_2T - ((t - r) - w);
            y0 = r - w;
            let i = j - ((hi(y0) >> 20) & 0x7ff);
            if i > 49 {
                let t = r;
                w = fnn * PIO2_3;
                r = t - w;
                w = fnn * PIO2_3T - ((t - r) - w);
                y0 = r - w;
            }
        }
        let y1 = (r - y0) - w;
        return Some(if hx < 0 { (-n, -y0, -y1) } else { (n, y0, y1) });
    }
    None
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
