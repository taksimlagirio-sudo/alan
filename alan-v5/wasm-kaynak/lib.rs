// ALAN çekirdekleri (WebAssembly deneyi). sabit-mat.js ve core.js'teki işlevlerin birebir aynı işlemleri, aynı sırayla.

const LN2H: f64 = 6.93147180369123816490e-01;
const LN2L: f64 = 1.90821492927058770002e-10;
const ILN2: f64 = 1.44269504088896338700e+00;
const P1: f64 = 1.66666666666666019037e-01;
const P2: f64 = -2.77777777770155933842e-03;
const P3: f64 = 6.61375632143793436117e-05;
const P4: f64 = -1.65339022054652515390e-06;
const P5: f64 = 4.13813679705723846039e-08;

#[inline(always)] fn floor(x: f64) -> f64 { x.floor() }
#[inline(always)] fn ceil(x: f64) -> f64 { x.ceil() }
#[inline(always)] fn abs(x: f64) -> f64 { f64::from_bits(x.to_bits() & 0x7fff_ffff_ffff_ffff) }
#[inline(always)] fn yuv(x: f64) -> f64 { if x >= 0.0 { floor(x + 0.5) } else { ceil(x - 0.5) } }
#[inline(always)] fn iki(k: i32) -> f64 { f64::from_bits(((k + 1023) as u64) << 52) }
#[inline(always)] fn sk(y: f64, k: i32) -> f64 { if k > 1023 { y * iki(1023) * iki(k - 1023) } else if k < -1022 { y * iki(-1022) * iki(k + 1022) } else { y * iki(k) } }

#[no_mangle]
pub extern "C" fn exp(x: f64) -> f64 {
    if x != x { return x; }
    if x > 709.782712893384 { return f64::INFINITY; }
    if x < -745.1332191019411 { return 0.0; }
    if abs(x) < 3.725290298461914e-9 { return 1.0 + x; }
    let k = if abs(x) < 0.34657359027997264 { 0.0 } else { yuv(x * ILN2) };
    let hi = x - k * LN2H; let lo = k * LN2L; let r = hi - lo; let t = r * r;
    let c = r - t * (P1 + t * (P2 + t * (P3 + t * (P4 + t * P5))));
    let y = 1.0 - ((lo - (r * c) / (2.0 - c)) - hi);
    if k == 0.0 { y } else { sk(y, k as i32) }
}

#[no_mangle]
pub extern "C" fn tanh(x: f64) -> f64 {
    if x != x { return x; }
    if x > 22.0 { return 1.0; }
    if x < -22.0 { return -1.0; }
    if abs(x) < 1e-8 { return x; }
    let e = exp(2.0 * x);
    (e - 1.0) / (e + 1.0)
}

#[inline(always)] fn dim_s(s: f64) -> f64 { if s <= 1.0 { s } else if s <= 2.0 { 1.0 + 0.5 * (s - 1.0) } else { 1.5 + 0.25 * (s - 2.0) } }
#[inline(always)] fn dogru(v: f64) -> bool { v != 0.0 && v == v } /* JS'te sayının doğruluğu: 0 ve NaN yanlış */

// paket alanı: 16 sayılık oyuncu kayıtları (core.js pack, secAcik)
const YUVA: usize = 256; /* bir paketin yeri: en çok 16 oyuncu × 16 */
const KAP: usize = 1024; /* halkadaki yuva sayısı */
static mut BUF: [f64; YUVA * KAP] = [0.0; YUVA * KAP];
#[no_mangle] pub extern "C" fn tampon() -> *mut f64 { unsafe { core::ptr::addr_of_mut!(BUF) as *mut f64 } }

#[no_mangle]
pub extern "C" fn kpk_sec(yuva: u32, n: u32, x: f64, y: f64, team: f64) -> f64 {
    let a: &[f64] = unsafe { core::slice::from_raw_parts((core::ptr::addr_of!(BUF) as *const f64).add((yuva as usize) * YUVA), YUVA) };
    let (mut sp, mut sn) = (0.0f64, 0.0f64);
    let mut o = 0usize;
    for _ in 0..n {
        let am = if dogru(a[o + 12]) { a[o + 13] } else if a[o + 7] != team && a[o + 14] > 0.15 { -1.0 } else { 0.0 };
        if !dogru(am) { o += 16; continue; }
        let dx = x - a[o]; let dy = y - a[o + 1]; let d2 = dx * dx + dy * dy;
        if d2 > a[o + 8] { o += 16; continue; }
        let q;
        if dogru(a[o + 4]) {
            let r = a[o + 2]; let fx = a[o + 5]; let fy = a[o + 6];
            let al = dx * fx + dy * fy; let pe = -dx * fy + dy * fx;
            let aa = if al > 0.0 { al / a[o + 10] } else { al / a[o + 11] };
            let pr = pe / r; q = aa * aa + pr * pr;
        } else { q = d2 / a[o + 9]; }
        let f = a[o + 3] * exp(-q * 1.2);
        if am > 0.0 { sp += f; } else { sn += f; }
        o += 16;
    }
    tanh(dim_s(sp) - dim_s(sn))
}
