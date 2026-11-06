//! Belirlenimli matematik (sabit-mat.js'in birebir karşılığı).
//!
//! Telefonda (ARM) ve bilgisayarda (x86) aynı maç çıksın diye motor `std`'nin
//! `exp`, `sin`… işlevlerini değil bunları kullanır: yalnız + − × ÷ ve √ (IEEE 754'te
//! her yerde bit bit aynı). Algoritmalar fdlibm'den, JS'teki işlem sırasıyla aynı.
//!
//! JS'in `Math.max`/`Math.min`/`Math.round` anlamı Rust'takinden farklı olduğu için
//! (NaN'ı yaymak, .5'i yukarı yuvarlamak) onların karşılıkları da burada.

#[inline(always)]
fn iki(k: i32) -> f64 {
    f64::from_bits(((k + 1023) as u64) << 52)
}
#[inline(always)]
fn sk(y: f64, k: i32) -> f64 {
    if k > 1023 {
        y * iki(1023) * iki(k - 1023)
    } else if k < -1022 {
        y * iki(-1022) * iki(k + 1022)
    } else {
        y * iki(k)
    }
}
#[inline(always)]
fn yuv(x: f64) -> f64 {
    if x >= 0.0 {
        (x + 0.5).floor()
    } else {
        (x - 0.5).ceil()
    }
}

const LN2H: f64 = 6.9314718036912381649e-1;
const LN2L: f64 = 1.90821492927058770002e-10;
const ILN2: f64 = 1.442695040888963387;
const P1: f64 = 1.66666666666666019037e-1;
const P2: f64 = -2.77777777770155933842e-3;
const P3: f64 = 6.61375632143793436117e-5;
const P4: f64 = -1.6533902205465251539e-6;
const P5: f64 = 4.13813679705723846039e-8;

pub fn exp(x: f64) -> f64 {
    if x != x {
        return x;
    }
    if x > 709.782712893384 {
        return f64::INFINITY;
    }
    if x < -745.1332191019411 {
        return 0.0;
    }
    if x.abs() < 3.725290298461914e-9 {
        return 1.0 + x;
    }
    let k = if x.abs() < 0.34657359027997264 { 0.0 } else { yuv(x * ILN2) };
    let hi = x - k * LN2H;
    let lo = k * LN2L;
    let r = hi - lo;
    let t = r * r;
    let c = r - t * (P1 + t * (P2 + t * (P3 + t * (P4 + t * P5))));
    let y = 1.0 - (lo - (r * c) / (2.0 - c) - hi);
    if k == 0.0 {
        y
    } else {
        sk(y, k as i32)
    }
}

const LG1: f64 = 6.66666666666673513e-1;
const LG2: f64 = 3.999999999940941908e-1;
const LG3: f64 = 2.857142874366239149e-1;
const LG4: f64 = 2.222219843214978396e-1;
const LG5: f64 = 1.818357216161805012e-1;
const LG6: f64 = 1.531383769920937332e-1;
const LG7: f64 = 1.479819860511658591e-1;

pub fn log(x: f64) -> f64 {
    if x != x || x < 0.0 {
        return f64::NAN;
    }
    if x == 0.0 {
        return f64::NEG_INFINITY;
    }
    if x == f64::INFINITY {
        return x;
    }
    let mut k: i32 = 0;
    let mut b = x.to_bits();
    let mut hx = (b >> 32) as u32;
    if hx < 0x0010_0000 {
        b = (x * 18014398509481984.0).to_bits();
        hx = (b >> 32) as u32;
        k -= 54;
    }
    k += ((hx >> 20) as i32) - 1023;
    hx &= 0x000f_ffff;
    let i = (hx.wrapping_add(0x95f64)) & 0x0010_0000;
    let yeni_hi = hx | (i ^ 0x3ff0_0000);
    b = ((yeni_hi as u64) << 32) | (b & 0xffff_ffff);
    k += (i >> 20) as i32;
    let f = f64::from_bits(b) - 1.0;
    let hf = 0.5 * f * f;
    let s = f / (2.0 + f);
    let z = s * s;
    let w = z * z;
    let r = z * (LG1 + w * (LG3 + w * (LG5 + w * LG7))) + w * (LG2 + w * (LG4 + w * LG6));
    let kf = k as f64;
    kf * LN2H - (hf - (s * (hf + r) + kf * LN2L) - f)
}

pub fn pow(x: f64, y: f64) -> f64 {
    if y == 0.0 {
        return 1.0;
    }
    if x != x || y != y {
        return f64::NAN;
    }
    if y.fract() == 0.0 && y.is_finite() && y.abs() <= 64.0 {
        let mut n = y.abs() as u32;
        let mut b = x;
        let mut r = 1.0;
        while n != 0 {
            if n & 1 != 0 {
                r *= b;
            }
            b *= b;
            n >>= 1;
        }
        return if y < 0.0 { 1.0 / r } else { r };
    }
    if x == 0.0 {
        return if y > 0.0 { 0.0 } else { f64::INFINITY };
    }
    if x < 0.0 {
        return f64::NAN;
    }
    if x == f64::INFINITY {
        return if y > 0.0 { f64::INFINITY } else { 0.0 };
    }
    exp(y * log(x))
}

const S1: f64 = -1.66666666666666324348e-1;
const S2: f64 = 8.33333333332248946124e-3;
const S3: f64 = -1.98412698298579493134e-4;
const S4: f64 = 2.75573137070700676789e-6;
const S5: f64 = -2.50507602534068634195e-8;
const S6: f64 = 1.58969099521155010221e-10;
const C1: f64 = 4.16666666666666019037e-2;
const C2: f64 = -1.38888888888741095749e-3;
const C3: f64 = 2.48015872894767294178e-5;
const C4: f64 = -2.75573143513906633035e-7;
const C5: f64 = 2.0875723212981748279e-9;
const C6: f64 = -1.13596475577881948265e-11;

#[inline(always)]
fn ks(x: f64, y: f64) -> f64 {
    let z = x * x;
    let v = z * x;
    let r = S2 + z * (S3 + z * (S4 + z * (S5 + z * S6)));
    x - (z * (0.5 * y - v * r) - y - v * S1)
}
#[inline(always)]
fn kc(x: f64, y: f64) -> f64 {
    let z = x * x;
    let r = z * (C1 + z * (C2 + z * (C3 + z * (C4 + z * (C5 + z * C6)))));
    let hz = 0.5 * z;
    let w = 1.0 - hz;
    w + (1.0 - w - hz + (z * r - x * y))
}

const IPIO2: f64 = 6.36619772367581382433e-1;
const PIO2_1: f64 = 1.57079632673412561417;
const PIO2_1T: f64 = 6.07710050650619224932e-11;
const PIO2_2: f64 = 6.0771005063039659766e-11;
const PIO2_2T: f64 = 2.02226624879595063154e-21;

fn ind(x: f64) -> (i32, f64, f64) {
    let n = yuv(x * IPIO2);
    let mut r = x - n * PIO2_1;
    let mut w = n * PIO2_1T;
    let mut y0 = r - w;
    if n.abs() > 32.0 {
        let t = r;
        w = n * PIO2_2;
        r = t - w;
        w = n * PIO2_2T - (t - r - w);
        y0 = r - w;
    }
    (n as i64 as i32, y0, r - y0 - w)
}

pub fn sin(x: f64) -> f64 {
    if x != x || x.is_infinite() {
        return f64::NAN;
    }
    if x.abs() < 0.7853981633974483 {
        return if x.abs() < 7.450580596923828e-9 { x } else { ks(x, 0.0) };
    }
    let (n, a, b) = ind(x);
    match n & 3 {
        0 => ks(a, b),
        1 => kc(a, b),
        2 => -ks(a, b),
        _ => -kc(a, b),
    }
}

pub fn cos(x: f64) -> f64 {
    if x != x || x.is_infinite() {
        return f64::NAN;
    }
    if x.abs() < 0.7853981633974483 {
        return kc(x, 0.0);
    }
    let (n, a, b) = ind(x);
    match n & 3 {
        0 => kc(a, b),
        1 => -ks(a, b),
        2 => -kc(a, b),
        _ => ks(a, b),
    }
}

const AH: [f64; 4] = [4.63647609000806093515e-1, 7.85398163397448278999e-1, 9.82793723247329054082e-1, 1.570796326794896558];
const AL: [f64; 4] = [2.26987774529616870924e-17, 3.06161699786838301793e-17, 1.39033110312309984516e-17, 6.12323399573676603587e-17];
const AT: [f64; 11] = [
    3.33333333333329318027e-1,
    -1.99999999998764832476e-1,
    1.42857142725034663711e-1,
    -1.1111110405462355788e-1,
    9.09088713343650656196e-2,
    -7.69187620504482999495e-2,
    6.66107313738753120669e-2,
    -5.83357013379057348645e-2,
    4.97687799461593236017e-2,
    -3.6531572744216915527e-2,
    1.62858201153657823623e-2,
];

pub fn atan(x: f64) -> f64 {
    if x != x {
        return x;
    }
    let ng = x < 0.0;
    let mut a = x.abs();
    if a >= 7.378697629483821e19 {
        return if ng { -AH[3] - AL[3] } else { AH[3] + AL[3] };
    }
    let id: i32;
    if a < 0.4375 {
        if a < 1.862645149230957e-9 {
            return x;
        }
        id = -1;
    } else if a < 1.1875 {
        if a < 0.6875 {
            id = 0;
            a = (2.0 * a - 1.0) / (2.0 + a);
        } else {
            id = 1;
            a = (a - 1.0) / (a + 1.0);
        }
    } else if a < 2.4375 {
        id = 2;
        a = (a - 1.5) / (1.0 + 1.5 * a);
    } else {
        id = 3;
        a = -1.0 / a;
    }
    let z = a * a;
    let w = z * z;
    let s1 = z * (AT[0] + w * (AT[2] + w * (AT[4] + w * (AT[6] + w * (AT[8] + w * AT[10])))));
    let s2 = w * (AT[1] + w * (AT[3] + w * (AT[5] + w * (AT[7] + w * AT[9]))));
    if id < 0 {
        return x - x * (s1 + s2);
    }
    let i = id as usize;
    let r = AH[i] - (a * (s1 + s2) - AL[i] - a);
    if ng {
        -r
    } else {
        r
    }
}

pub const PI: f64 = 3.141592653589793116;
const PI_LO: f64 = 1.2246467991473531772e-16;

pub fn atan2(y: f64, x: f64) -> f64 {
    if x != x || y != y {
        return f64::NAN;
    }
    if x == 1.0 {
        return atan(y);
    }
    if y == 0.0 {
        if x > 0.0 || (x == 0.0 && (1.0 / x) > 0.0) {
            return y;
        }
        return if (1.0 / y) > 0.0 { PI } else { -PI };
    }
    if x == 0.0 {
        return if y > 0.0 { PI / 2.0 } else { -PI / 2.0 };
    }
    if x.is_infinite() || y.is_infinite() {
        let ix = if x == f64::INFINITY { 1.0 } else if x == f64::NEG_INFINITY { -1.0 } else { 0.0 };
        let iy = if y == f64::INFINITY { 1.0 } else if y == f64::NEG_INFINITY { -1.0 } else { 0.0 };
        if ix != 0.0 && iy != 0.0 {
            return iy * if ix > 0.0 { PI / 4.0 } else { (3.0 * PI) / 4.0 };
        }
        if ix != 0.0 {
            return if ix > 0.0 {
                if y > 0.0 {
                    0.0
                } else {
                    -0.0
                }
            } else if y > 0.0 {
                PI
            } else {
                -PI
            };
        }
        return (iy * PI) / 2.0;
    }
    let z = atan((y / x).abs());
    if x > 0.0 {
        return if y > 0.0 { z } else { -z };
    }
    if y > 0.0 {
        PI - (z - PI_LO)
    } else {
        -(PI - (z - PI_LO))
    }
}

pub fn acos(x: f64) -> f64 {
    if x != x || x > 1.0 || x < -1.0 {
        f64::NAN
    } else {
        atan2(((1.0 - x) * (1.0 + x)).sqrt(), x)
    }
}

pub fn tanh(x: f64) -> f64 {
    if x != x {
        return x;
    }
    if x > 22.0 {
        return 1.0;
    }
    if x < -22.0 {
        return -1.0;
    }
    if x.abs() < 1e-8 {
        return x;
    }
    let e = exp(2.0 * x);
    (e - 1.0) / (e + 1.0)
}

#[inline(always)]
pub fn hypot(a: f64, b: f64) -> f64 {
    if a.is_infinite() || b.is_infinite() {
        return f64::INFINITY;
    }
    (a * a + b * b).sqrt()
}

// ---- JS anlamıyla yardımcılar ----

/// `Math.max(a, b)`: NaN yayılır (Rust `f64::max` NaN'ı yutar).
#[inline(always)]
pub fn jmax(a: f64, b: f64) -> f64 {
    if a != a || b != b {
        f64::NAN
    } else if a > b {
        a
    } else if b > a {
        b
    } else if a == 0.0 && b == 0.0 {
        if a.is_sign_negative() && b.is_sign_negative() {
            -0.0
        } else {
            0.0
        }
    } else {
        a
    }
}
/// `Math.min(a, b)`.
#[inline(always)]
pub fn jmin(a: f64, b: f64) -> f64 {
    if a != a || b != b {
        f64::NAN
    } else if a < b {
        a
    } else if b < a {
        b
    } else if a == 0.0 && b == 0.0 {
        if a.is_sign_negative() || b.is_sign_negative() {
            -0.0
        } else {
            0.0
        }
    } else {
        a
    }
}
/// `Math.max(lo, Math.min(hi, x))`.
#[inline(always)]
pub fn kis(x: f64, lo: f64, hi: f64) -> f64 {
    jmax(lo, jmin(hi, x))
}
/// `Math.round(x)`: .5 yukarı (+∞ yönüne).
#[inline(always)]
pub fn jround(x: f64) -> f64 {
    if x != x || x.is_infinite() {
        return x;
    }
    let f = x.floor();
    let r = if x - f >= 0.5 { f + 1.0 } else { f };
    if r == 0.0 && x < 0.0 {
        -0.0
    } else {
        r
    }
}
/// `Math.sign(x)`.
#[inline(always)]
pub fn jsign(x: f64) -> f64 {
    if x > 0.0 {
        1.0
    } else if x < 0.0 {
        -1.0
    } else {
        x
    }
}
/// JS'te sayının doğruluğu: 0 ve NaN yanlış.
#[inline(always)]
pub fn dogru(v: f64) -> bool {
    v != 0.0 && v == v
}

// Maç modülünden taşındı (Aşama 7b).
/// JS `x.toFixed(n)` sonra `+` (sayıya): yarıda yukarı (tam ikili değerle).
pub fn to_fixed(x: f64, n: usize) -> f64 {
    let s = format!("{:.*}", n, x);
    let mut v: f64 = s.parse().unwrap_or(x);
    let k = 10f64.powi(n as i32);
    let y = x.abs() * k;
    if y.fract() == 0.5 && y < 1e15 {
        let up = (y.floor() + 1.0) / k;
        v = if x < 0.0 { -up } else { up };
    }
    v
}


/// JS `String(x)` (sayı), olay yazıları için.
pub fn js_num(x: f64) -> String {
    if x == x.trunc() && x.abs() < 1e21 {
        format!("{}", x as i64)
    } else {
        format!("{}", x)
    }
}

