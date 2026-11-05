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

use crate::kural::kos::vSp;

/// JS `Y`: bir yön/hız için 80 tiklik ilerleme eğrisi (`m[n]`: n tikte alınan yol) ve tam gecikme `g`.
#[derive(Clone, Debug)]
pub struct GecY {
    pub g: f64,
    /// Aramanın özeti: `m[0], m[8], …, m[80]` (g ile aynı önbellek satırlarında; gecK önce burada arar)
    pub ozet: [f64; 11],
    /// `m` NaN'sız mı (azalmayan dizi: iki aşamalı arama ikili aramayla aynı yeri bulur)
    pub temiz: bool,
    pub m: [f64; 81],
}
impl GecY {
    pub fn yeni(g: f64, m: [f64; 81]) -> GecY {
        let ozet = std::array::from_fn(|i| m[8 * i]);
        let temiz = m.iter().all(|v| !v.is_nan());
        GecY { g, ozet, temiz, m }
    }
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
    [jround(1e4 * ah(&r0.govde.yet)) / 1e4, jround(100.0 * ag(&r0.govde.yet, r0.den, r0.bekci(), k.ik_yavas())) / 100.0]
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

/// gecYk'nin anahtarı ve nicelenmiş girdileri (rr, cc).
#[inline]
fn gec_anahtar(r: f64, c: f64, vm: f64, iv: [f64; 2]) -> (GecK, f64, f64) {
    let r10 = jround(10.0 * r);
    let c4 = jround(4.0 * c);
    (GecK { iv0: iv[0].to_bits(), iv1: iv[1].to_bits(), r10: r10 as i64, c4: c4 as i64, vmq: vm.to_bits() }, r10 / 10.0, c4 / 4.0)
}
/// Eğrileri hesapla: her biri 80 tik ivme yasasıyla (zincir: her adım bir öncekinin hızını bekler). Birden çok
/// eğri aynı adımda birlikte ilerler: zincirler birbirinden bağımsız, işlemci onları üst üste yürütür. Her
/// eğrinin işlemleri ve sırası tek başına hesaplanmasıyla aynı (sonuç bit bit aynı).
fn gec_egriler<const N: usize>(rr: f64, ccs: [f64; N], vm: f64, iv: [f64; 2]) -> [GecY; N] {
    let mut qx = [rr * vm; N];
    let mut qy = [0.0f64; N];
    let ux: [f64; N] = ccs;
    let uy: [f64; N] = std::array::from_fn(|k| (jmax(0.0, 1.0 - ccs[k] * ccs[k])).sqrt());
    let mut m = [[0.0f64; 81]; N];
    let mut d = [0.0f64; N];
    let mut e = [0.0f64; N];
    for n in 0..80 {
        for k in 0..N {
            ivme_iv(&mut qx[k], &mut qy[k], ux[k] * vm, uy[k] * vm, vm, iv);
            d[k] += qx[k] * ux[k] + qy[k] * uy[k];
            e[k] += qx[k] * uy[k] - qy[k] * ux[k];
            m[k][n + 1] = jmax(m[k][n], d[k] - e[k].abs());
        }
    }
    std::array::from_fn(|k| GecY::yeni(jmax(0.0, 80.0 - (d[k] - e[k].abs()) / vm), m[k]))
}
fn gec_koy(key: GecK, y: &Arc<GecY>) {
    GEC.with(|g| {
        let mut g = g.borrow_mut();
        // sınırsız büyüyordu (maç sonunda ~800 bin eğri, tarayıcıda 2 GB'ı aşıp işçiyi düşürüyordu).
        // Değer yalnız anahtardan hesaplanır (saf): temizlemek sonucu değiştirmez.
        if g.len() >= GEC_SINIR {
            g.clear();
        }
        g.insert(key, y.clone());
    });
}
/// JS gecYk: anahtarlı hesap (vm nicelenmiş).
fn gecYk(r: f64, c: f64, vm: f64, iv: [f64; 2]) -> Arc<GecY> {
    let (key, rr, cc) = gec_anahtar(r, c, vm, iv);
    if let Some(y) = GEC.with(|g| g.borrow().get(&key).cloned()) {
        return y;
    }
    let [y] = gec_egriler(rr, [cc], vm, iv);
    let y = Arc::new(y);
    gec_koy(key, &y);
    y
}
/// 9 yönün eğrileri (gecTabV): önbellekte olmayanlar birlikte hesaplanır.
fn gecYk9(r: f64, cs: [f64; 9], vm: f64, iv: [f64; 2]) -> [Arc<GecY>; 9] {
    let an: [(GecK, f64, f64); 9] = std::array::from_fn(|k| gec_anahtar(r, cs[k], vm, iv));
    let mut ys: [Option<Arc<GecY>>; 9] = std::array::from_fn(|k| GEC.with(|g| g.borrow().get(&an[k].0).cloned()));
    let eksik: Vec<usize> = (0..9).filter(|&k| ys[k].is_none() && !(0..k).any(|j| ys[j].is_none() && an[j].0 == an[k].0)).collect();
    if eksik.len() == 9 {
        let ccs: [f64; 9] = std::array::from_fn(|k| an[k].2);
        let yeni = gec_egriler(an[0].1, ccs, vm, iv);
        for (k, y) in yeni.into_iter().enumerate() {
            let y = Arc::new(y);
            gec_koy(an[k].0, &y);
            ys[k] = Some(y);
        }
    } else {
        for &k in &eksik {
            let [y] = gec_egriler(an[k].1, [an[k].2], vm, iv);
            let y = Arc::new(y);
            gec_koy(an[k].0, &y);
            ys[k] = Some(y);
        }
    }
    // aynı anahtarlı yönler (Bekçi'nin duran oyuncusu gibi: hepsi c = 1) ilk hesaplananı paylaşır
    std::array::from_fn(|k| match &ys[k] {
        Some(y) => y.clone(),
        None => ys[(0..k).find(|&j| an[j].0 == an[k].0).unwrap()].clone().unwrap(),
    })
}
/// Varış tablosu önbelleklerinin üst sınırı (girdi): aşılınca temizlenir (değerler saf, maç aynı kalır).
const GEC_SINIR: usize = 100_000;
const GTB_SINIR: usize = 20_000;

/// JS gecK: `s` yol için gecikme (eğriden ara değer).
pub fn gecK(Y: &GecY, s: f64, vm: f64) -> f64 {
    if !Y.temiz {
        return gecK_m(Y.g, &Y.m, s, vm);
    }
    if !(s > 0.0) {
        return 0.0;
    }
    // m[80] özetin sonunda: eğrinin kendisine dokunmadan
    if s >= Y.ozet[10] {
        return Y.g;
    }
    // m azalmayan ve NaN'sız: ikili aramanın bulduğu lo, m[lo] < s olan son sıra (m[0] = 0 < s, m[80] > s).
    // Önce özette (8'lik bloklar), sonra tek blokta: aynı lo, ama eğrinin 6 yerine 2 önbellek satırı.
    let mut b = 0usize;
    while b < 9 && Y.ozet[b + 1] < s {
        b += 1;
    }
    let mut lo = 8 * b;
    while lo < 79 && Y.m[lo + 1] < s {
        lo += 1;
    }
    let hi = lo + 1;
    jmax(0.0, lo as f64 + (s - Y.m[lo]) / jmax(1e-9, Y.m[hi] - Y.m[lo]) - s / vm)
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
            let cs: [f64; 9] = std::array::from_fn(|kk| if z { 1.0 } else { (kk as f64 - 4.0) / 4.0 });
            let ys = gecYk9(r, cs, vmQ, iv);
            let t: [f64; 9] = std::array::from_fn(|kk| ys[kk].g);
            let v = Arc::new((t, ys));
            GTB.with(|g| {
                let mut g = g.borrow_mut();
                if g.len() >= GTB_SINIR {
                    g.clear();
                }
                g.insert(key, v.clone());
            });
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
