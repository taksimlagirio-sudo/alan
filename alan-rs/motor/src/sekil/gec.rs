//! Varış gecikmesi (JS gec, gecTab, gecTabV, gecOf, gecC, gecY, gecYk, gecK): oyuncu bir noktaya düz koşu
//! süresinden ne kadar geç varır (dönüş, hızlanma). Hesap ivme yasasıyla 80 tik ileri oynatılır.
//!
//! Önbellek (JS GEC, GTB): değer yalnız anahtardan hesaplanır (anahtar = yetenek katları, hız oranı, yön,
//! nicelenmiş azami hız), bu yüzden sıradan bağımsızdır. İş parçacığı başına tutulur (görünmez memo).
#![allow(non_snake_case)]

use std::cell::RefCell;

use std::sync::Arc;

use crate::mat::{hypot, jmax, jmin, jround};
use crate::model::Oyuncu;
use crate::temas::Kural;
use crate::yetenek::{ag, ah};

use super::vSp;

/// JS `Y`: bir yön/hız için 80 tiklik ilerleme eğrisi (`m[n]`: n tikte alınan yol) ve tam gecikme `g`.
#[derive(Clone, Debug)]
pub struct GecY {
    pub g: f64,
    pub m: [f64; 81],
}

/// JS `gecTab` dönüşü: oyuncunun anlık hızı ve 9 yön için gecikme tablosu.
#[derive(Clone, Debug)]
pub struct GecTab {
    pub vx: f64,
    pub vy: f64,
    pub s0: f64,
    pub x: f64,
    pub y: f64,
    /// 9 yönün tablosu (önbellekteki paylaşımlı nesne; kopyalanmaz)
    pub tb: Arc<([f64; 9], [Arc<GecY>; 9])>,
    pub vm: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct GecK {
    iv0: u64,
    iv1: u64,
    r10: i64,
    c4: i64,
    vmq: u64,
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct GtbK {
    iv0: u64,
    iv1: u64,
    r10: i64,
    z: bool,
    vmq: u64,
}

thread_local! {
    static GEC: RefCell<crate::HizliMap<GecK, Arc<GecY>>> = RefCell::new(Default::default());
    static GTB: RefCell<crate::HizliMap<GtbK, Arc<([f64; 9], [Arc<GecY>; 9])>>> = RefCell::new(Default::default());
}

/// JS `VMQ`: azami hızın nicelenmesi.
#[inline]
pub fn VMQ(vm: f64) -> f64 {
    jround(2e3 * vm) / 2e3
}

/// JS IVK: [AH, AG] yuvarlanmış (ALAN_YET.acik(4) açık).
pub fn IVK(r0: &Oyuncu, k: &dyn Kural) -> [f64; 2] {
    [jround(1e4 * ah(&r0.yet)) / 1e4, jround(100.0 * ag(&r0.yet, r0.den, r0.bekci(), k.ik_yavas())) / 100.0]
}

/// match.js `ivme`, `_iv` verilmiş ve sprint evresiz hayali gövde için (gecYk'nin oynattığı `q`).
fn ivme_iv(vx: &mut f64, vy: &mut f64, wx: f64, wy: f64, vmax: f64, iv: [f64; 2]) {
    let sp = hypot(*vx, *vy);
    let k0 = 0.18;
    let kH = k0 * iv[0];
    if sp < 0.001 {
        *vx += (wx - *vx) * kH;
        *vy += (wy - *vy) * kH;
        return;
    }
    let ux = *vx / sp;
    let uy = *vy / sp;
    let dvx = wx - *vx;
    let dvy = wy - *vy;
    let al = dvx * ux + dvy * uy;
    let px = dvx - al * ux;
    let py = dvy - al * uy;
    let r = jmin(1.0, sp / jmax(0.05, vmax));
    let agv = iv[1];
    let kA = if al >= 0.0 { kH } else { k0 * agv * (1.0 - 0.55 * r) };
    let kT = k0 * agv * (1.0 - 0.45 * r);
    let mut ba = al * kA;
    let mut tx = px * kT;
    let mut ty = py * kT;
    let bkA = 1.0;
    if ba < 0.0 {
        ba = jmax(ba, -0.018 * agv * bkA);
    }
    let tl = hypot(tx, ty);
    let tm = 0.021 * agv * bkA;
    if tl > tm && sp > 0.1 {
        tx *= tm / tl;
        ty *= tm / tl;
    }
    *vx += ux * ba + tx;
    *vy += uy * ba + ty;
}

/// JS gecYk: anahtarlı hesap (vm nicelenmiş).
fn gecYk(r: f64, c: f64, vm: f64, iv: [f64; 2]) -> Arc<GecY> {
    let r10 = jround(10.0 * r);
    let c4 = jround(4.0 * c);
    let key = GecK { iv0: iv[0].to_bits(), iv1: iv[1].to_bits(), r10: r10 as i64, c4: c4 as i64, vmq: vm.to_bits() };
    if let Some(y) = GEC.with(|g| g.borrow().get(&key).cloned()) {
        return y;
    }
    let rr = r10 / 10.0;
    let cc = c4 / 4.0;
    let (mut qx, mut qy) = (rr * vm, 0.0);
    let ux = cc;
    let uy = (jmax(0.0, 1.0 - cc * cc)).sqrt();
    let mut m = [0.0f64; 81];
    let (mut d, mut e) = (0.0f64, 0.0f64);
    for n in 0..80 {
        ivme_iv(&mut qx, &mut qy, ux * vm, uy * vm, vm, iv);
        d += qx * ux + qy * uy;
        e += qx * uy - qy * ux;
        m[n + 1] = jmax(m[n], d - e.abs());
    }
    let y = Arc::new(GecY { g: jmax(0.0, 80.0 - (d - e.abs()) / vm), m });
    GEC.with(|g| g.borrow_mut().insert(key, y.clone()));
    y
}

/// JS gecK: `s` yol için gecikme (eğriden ara değer).
pub fn gecK(Y: &GecY, s: f64, vm: f64) -> f64 {
    gecK_m(Y.g, &Y.m, s, vm)
}
/// gecK'nın tablosu dilim olarak (kafanın tablosundan kopyasız okunur).
pub fn gecK_m(g: f64, m: &[f64; 81], s: f64, vm: f64) -> f64 {
    if !(s > 0.0) {
        return 0.0;
    }
    if s >= m[80] {
        return g;
    }
    let (mut lo, mut hi) = (0usize, 80usize);
    while hi - lo > 1 {
        let md = (lo + hi) >> 1;
        if m[md] >= s {
            hi = md;
        } else {
            lo = md;
        }
    }
    jmax(0.0, lo as f64 + (s - m[lo]) / jmax(1e-9, m[hi] - m[lo]) - s / vm)
}

/// JS gecY(o, c): `o` = varış bedeni (JS `o.ref || o`).
pub fn gecY(r0: &Oyuncu, c: f64, k: &dyn Kural) -> Arc<GecY> {
    let vx = r0.vx;
    let vy = r0.vy;
    let s0 = hypot(vx, vy);
    let vm = vSp(r0, k);
    let r = jmin(1.0, s0 / vm);
    gecYk(r, c, VMQ(vm), IVK(r0, k))
}

/// JS gec(o, x, y, s): `s` verilmezse (None) tam gecikme, verilirse `s` yol için.
pub fn gec(r0: &Oyuncu, x: f64, y: f64, s: Option<f64>, k: &dyn Kural) -> f64 {
    let vx = r0.vx;
    let vy = r0.vy;
    let s0 = hypot(vx, vy);
    let dx = x - r0.x;
    let dy = y - r0.y;
    let L = hypot(dx, dy);
    if L < 0.5 {
        return 0.0;
    }
    let c = if s0 < 0.001 { 1.0 } else { (vx * dx + vy * dy) / (s0 * L) };
    match s {
        None => gecY(r0, c, k).g,
        Some(s) => gecK(&gecY(r0, c, k), s, vSp(r0, k)),
    }
}

/// JS gecTab(o).
pub fn gecTab(r0: &Oyuncu, k: &dyn Kural) -> GecTab {
    gecTabV(r0, None, k)
}

/// JS gecTabV(o, vm0): azami hız verilebilir.
pub fn gecTabV(r0: &Oyuncu, vm0: Option<f64>, k: &dyn Kural) -> GecTab {
    let vx = r0.vx;
    let vy = r0.vy;
    let s0 = hypot(vx, vy);
    let vm = vm0.unwrap_or_else(|| vSp(r0, k));
    let r = jmin(1.0, s0 / vm);
    let iv = IVK(r0, k);
    let z = s0 < 0.001;
    let vmQ = VMQ(vm);
    let key = GtbK { iv0: iv[0].to_bits(), iv1: iv[1].to_bits(), r10: jround(10.0 * r) as i64, z, vmq: vmQ.to_bits() };
    let T = match GTB.with(|g| g.borrow().get(&key).cloned()) {
        Some(t) => t,
        None => {
            let mut t = [0.0; 9];
            let ys: [Arc<GecY>; 9] = std::array::from_fn(|kk| {
                let c = if z { 1.0 } else { (kk as f64 - 4.0) / 4.0 };
                let y = gecYk(r, c, vmQ, iv);
                t[kk] = y.g;
                y
            });
            let v = Arc::new((t, ys));
            GTB.with(|g| g.borrow_mut().insert(key, v.clone()));
            v
        }
    };
    GecTab { vx, vy, s0, x: r0.x, y: r0.y, tb: T, vm }
}

/// JS gecOf(G, x, y, s).
pub fn gecOf(G: &GecTab, x: f64, y: f64, s: Option<f64>) -> f64 {
    let dx = x - G.x;
    let dy = y - G.y;
    let L = hypot(dx, dy);
    if L < 0.5 {
        return 0.0;
    }
    let c = if G.s0 < 0.001 { 1.0 } else { (G.vx * dx + G.vy * dy) / (G.s0 * L) };
    let kk = (jround(4.0 * jmax(-1.0, jmin(1.0, c))) + 4.0) as usize;
    match s {
        Some(s) => gecK(&G.tb.1[kk], s, G.vm),
        None => G.tb.0[kk],
    }
}
