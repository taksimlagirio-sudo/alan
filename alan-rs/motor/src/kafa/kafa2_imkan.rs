//! Şut imkânı (decide.js 5427–5511, 5805–5866, 6073–6631): şut tablosu (sutTab, imkBol), imkân haritası
//! (imkan: kim nereye vaktinde varıp şut çeker), şut yolu (sutYol, sutS), şut vakti (sutVakit),
//! taşıma (tasima), sutEngel, bosluk.
#![allow(non_snake_case, clippy::too_many_arguments)]

use super::kafa2_bag as dis;
use super::kafa2_bag::{ALAN_HZ};
use super::kafa2_baglam::*;
use super::kafa2_dunya::{bekKay, bekYerW};
use crate::mat::{cos, exp, hypot, jmax, jmin, sin};
use crate::model::Oyuncu;
use crate::saha::{CY as GCY, H as GH, MOUTH, W as GW};

use std::sync::{Arc, Mutex};

// ------------------------------------------------------------------ şut tablosu (ALAN_IMK_SUT_VE)

/// Tablonun eksenleri (shot-table.js, "imk-11 · motorun kendi şut hesabı · parmak izi 104b9sy").
pub const IMK_DS: [f64; 9] = [10.0, 13.0, 16.0, 20.0, 24.0, 28.0, 32.0, 36.0, 40.0];
pub const IMK_SY: [f64; 7] = [0.0, 5.0, 10.0, 15.0, 20.0, 25.0, 30.0];
pub const IMK_KO: [f64; 9] = [-8.0, -6.0, -4.0, -2.0, 0.0, 2.0, 4.0, 6.0, 8.0];
pub const IMK_KD: [f64; 3] = [-2.0, 0.0, 2.0];
pub const IMK_CH: [f64; 4] = [0.1, 0.4, 0.7, 1.0];
pub const IMK_L: [f64; 4] = [5.0, 10.0, 15.0, 20.0];
const TAB_BOY: usize = 9 * 7 * 9 * 3 * 4;
/// 16 tablo (şutçu L × Bekçi L, sıra L[a]-L[b]), her biri DS×SY×KO×KD×CH, değer −1 (yok) ya da 0–100.
static IMK_SUT: &[u8; 16 * TAB_BOY] = include_bytes!("kafa2_imk_sut.bin");
#[inline(always)]
fn tab_v(tab: usize, i: usize) -> f64 {
    IMK_SUT[tab * TAB_BOY + i] as i8 as f64
}

/// JS fr(A, v): A'da v'nin hücresi ve payı
pub fn fr(A: &[f64], v: f64) -> (usize, f64) {
    if v <= A[0] {
        return (0, 0.0);
    }
    if v >= A[A.len() - 1] {
        return (A.len() - 1, 0.0);
    }
    let mut i = 0;
    while A[i + 1] < v {
        i += 1;
    }
    (i, (v - A[i]) / (A[i + 1] - A[i]))
}

/// JS imkBol'un `e`si: konuma göre DS/SY/CH'de ara değerlenmiş tablolar (KO × KD).
#[derive(Debug)]
pub struct ImkBol {
    di: usize,
    dt: f64,
    si: usize,
    st: f64,
    ci: usize,
    ct: f64,
    tabs: crate::HizliMap<usize, Arc<(Vec<f64>, Vec<f64>)>>,
}
impl ImkBol {
    fn tab(&mut self, tab: usize) -> Arc<(Vec<f64>, Vec<f64>)> {
        if let Some(a) = self.tabs.get(&tab) {
            return a.clone();
        }
        let (nS, nK, nD, nC, nDS) = (IMK_SY.len(), IMK_KO.len(), IMK_KD.len(), IMK_CH.len(), IMK_DS.len());
        let mut n = vec![0.0; nK * nD];
        let mut w = vec![0.0; nK * nD];
        for k in 0..nK {
            for q in 0..nD {
                let mut sn = 0.0;
                let mut sw = 0.0;
                for a in 0..2 {
                    for b in 0..2 {
                        for f in 0..2 {
                            let wgt = (if a == 1 { self.dt } else { 1.0 - self.dt }) * (if b == 1 { self.st } else { 1.0 - self.st }) * (if f == 1 { self.ct } else { 1.0 - self.ct });
                            if wgt <= 0.0 {
                                continue;
                            }
                            let ix = ((((self.di + a).min(nDS - 1) * nS + (self.si + b).min(nS - 1)) * nK + k) * nD + q) * nC + (self.ci + f).min(nC - 1);
                            let v = tab_v(tab, ix);
                            if !(v < 0.0) {
                                sn += wgt * v;
                                sw += wgt;
                            }
                        }
                    }
                }
                n[k * nD + q] = sn;
                w[k * nD + q] = sw;
            }
        }
        let a = Arc::new((n, w));
        self.tabs.insert(tab, a.clone());
        a
    }
}

/// JS sayısının anahtar biçimi (−0 → 0)
#[inline(always)]
fn nb(x: f64) -> u64 {
    if x == 0.0 {
        0
    } else {
        x.to_bits()
    }
}

/// JS imkBol(T, fr, team, x, y, dd, sy, ch) (decide.js:6074)
fn imkBol(k: &mut Kafa, team: f64, x: f64, y: f64, dd: f64, sy: f64, ch: f64) -> Arc<Mutex<ImkBol>> {
    if !k.k2.IMKBT {
        k.k2.IMKBT = true;
        k.k2.IMKB = Default::default();
    }
    let key = [nb(team), nb(x), nb(y), nb(ch)];
    if let Some(e) = k.k2.IMKB.get(&key) {
        return e.clone();
    }
    if k.k2.IMKB.len() > 4000 {
        k.k2.IMKB.clear();
    }
    let (di, dt) = fr(&IMK_DS, dd);
    let (si, st) = fr(&IMK_SY, sy);
    let (ci, ct) = fr(&IMK_CH, ch);
    let e = Arc::new(Mutex::new(ImkBol { di, dt, si, st, ci, ct, tabs: Default::default() }));
    k.k2.IMKB.insert(key, e.clone());
    e
}

/// JS imkKov(p) (decide.js:6113): rakibin kovalama tepkisi ve hızı (maç tiki başına önbellek, team|id).
pub fn imkKov(k: &mut Kafa, p: &Oyuncu) -> (f64, f64) {
    let t = k.mac.as_ref().map(|m| m.tick);
    if k.k2.IMKKt != Some(t) {
        k.k2.IMKKt = Some(t);
        k.k2.IMKK = Default::default();
    }
    let key = (crate::js_sayi_k(p.team), crate::js_sayi_k(p.id));
    if let Some(r) = k.k2.IMKK.get(&key) {
        return *r;
    }
    let r = (dis::ALAN_TEPKI(k, p, "kovala"), (0.17 + 0.006 * ALAN_HZ(p)) * dis::ALAN_SPRE(k, p));
    k.k2.IMKK.insert(key, r);
    r
}

/// JS imkOrt(a) (decide.js:6139): ham özelliklerin ortalaması (yoksa 10). Sıra: kaydın alan sırası.
pub fn imkOrt(p: &Oyuncu) -> f64 {
    let a = &p.govde.a;
    let v = [a.hiz, a.ceviklik, a.guc, a.uzanma, a.alan, a.kontrol, a.surme, a.pas, a.gonderme, a.yukleme, a.tepki, a.gorus, a.yeralma, a.sogukkanlilik, a.kesme, a.refleks, a.yerlesim, a.hakimiyet, a.enerji, a.govde, a.caliskanlik];
    let mut s = 0.0;
    for x in v {
        s += x;
    }
    s / v.len() as f64
}

thread_local! {
    static SAHTE: std::cell::Cell<Option<Box<Oyuncu>>> = const { std::cell::Cell::new(None) };
}
/// `f(&sahte(team, x, y, id))` ile aynı, ama sahte oyuncu her çağrıda yeniden kurulmaz: ödünç alınıp
/// yalnız bu dört alan yazılır (geri kalanı hep varsayılan; f yalnız okur), sonra geri konur. İç içe çağrıda yenisi kurulur.
pub fn sahte_ile<R>(team: f64, x: f64, y: f64, id: f64, f: impl FnOnce(&Oyuncu) -> R) -> R {
    let mut o = SAHTE.with(|c| c.take()).unwrap_or_else(|| Box::new(Oyuncu::bos()));
    o.team = team;
    o.x = x;
    o.y = y;
    o.id = id;
    let r = f(&o);
    SAHTE.with(|c| c.set(Some(o)));
    r
}

/// Sahte oyuncu (JS `{ team, x, y, id: -1 }` gibi kısmi nesneler)
pub fn sahte(team: f64, x: f64, y: f64, id: f64) -> Oyuncu {
    let mut o = Oyuncu::bos();
    o.team = team;
    o.x = x;
    o.y = y;
    o.id = id;
    o
}

/// JS sutTab(w, team, x, y, ch, sh, sig, bkYer, bkPos, tOnce) (decide.js:6147): (x, y)'den şutun
/// gol olasılığı tablodan (Bekçi'nin yerine, şutçu ve Bekçi'nin düzeyine göre) × gövdelerin şut yolu.
pub fn sutTab(k: &mut Kafa, w: &[Op], team: f64, x: f64, y: f64, ch: f64, sh: Option<&Oyuncu>, sig: Option<f64>, bkYer: Option<Nokta>, bkPos: Option<Nokta>, tOnce: Option<f64>) -> f64 {
    let bk = match w.iter().find(|p| p.team != team && bekci(p)) {
        Some(b) => b.clone(),
        None => return 1.0,
    };
    let fx = |v: f64| if t0(team) { v } else { GW - v };
    let ayna = y < GCY;
    let fy = |v: f64| if ayna { 2.0 * GCY - v } else { v };
    let X = fx(x);
    let Y = fy(y);
    let dd = veya(hypot(GW - X, GCY - Y), 1.0);
    let sy = Y - GCY;
    let yer0 = match bkYer {
        Some(b) => b,
        None => {
            let tac = k.mac.as_ref().map(|m| m.tac.clone()).unwrap_or_default();
            // şutçu (x, y)'de: kopya yerine görünüm (yalnız yeri değişir)
            match sh {
                Some(s) => {
                    let fm = dis::Fm { tac: &tac, ps: w, holder: Some(crate::dunya::Yerde { govde: s, x, y }) };
                    dis::AS_bekci(k, &bk, nk(x, y), Some(&fm))
                }
                None => sahte_ile(team, x, y, -1.0, |hold| {
                    let fm = dis::Fm { tac: &tac, ps: w, holder: Some(crate::dunya::Yerde::oyuncu(hold)) };
                    dis::AS_bekci(k, &bk, nk(x, y), Some(&fm))
                }),
            }
        }
    };
    let bkP = bkPos.unwrap_or(nk(bk.x, bk.y));
    let (yx, yy, bx, by) = (fx(yer0.x), fy(yer0.y), fx(bkP.x), fy(bkP.y));
    let ux = (GW - X) / dd;
    let uy = (GCY - Y) / dd;
    let nx = -uy;
    let ny = ux;
    let sg = if ny >= 0.0 { 1.0 } else { -1.0 };
    let ko = sg * ((bx - yx) * nx + (by - yy) * ny);
    let kd = -((bx - yx) * ux + (by - yy) * uy);
    let ls = jmax(5.0, jmin(20.0, match sh {
        Some(s) => imkOrt(s),
        None => 10.0,
    }));
    let lk = jmax(5.0, jmin(20.0, imkOrt(&bk)));
    let (nK, nD) = (IMK_KO.len(), IMK_KD.len());
    let (ei, et) = fr(&IMK_KD, kd);
    let BK = imkBol(k, team, x, y, dd, sy, ch);
    let oku1 = |tab: usize, ki: usize, kt: f64| -> f64 {
        let A = BK.lock().unwrap().tab(tab);
        let mut s = 0.0;
        let mut wsum = 0.0;
        for c in 0..2 {
            for e in 0..2 {
                let wgt = (if c == 1 { kt } else { 1.0 - kt }) * (if e == 1 { et } else { 1.0 - et });
                if wgt <= 0.0 {
                    continue;
                }
                let j = (ki + c).min(nK - 1) * nD + (ei + e).min(nD - 1);
                s += wgt * A.0[j];
                wsum += wgt * A.1[j];
            }
        }
        if wsum > 0.0 {
            s / wsum / 100.0
        } else {
            0.0
        }
    };
    let (li, lt) = fr(&IMK_L, ls);
    let (mi, mt) = fr(&IMK_L, lk);
    let tb = |a: usize, b: usize| (li + a).min(3) * 4 + (mi + b).min(3);
    let mut LT: Vec<(f64, usize)> = Vec::new();
    let mut ek = |wgt: f64, a: usize, b: usize| {
        if wgt > 1e-9 {
            LT.push((wgt, tb(a, b)));
        }
    };
    if lt >= mt {
        ek(1.0 - lt, 0, 0);
        ek(lt - mt, 1, 0);
        ek(mt, 1, 1);
    } else {
        ek(1.0 - mt, 0, 0);
        ek(mt - lt, 0, 1);
        ek(lt, 1, 1);
    }
    let okuL = |kov: f64| -> f64 {
        let (ki, kt) = fr(&IMK_KO, kov);
        let mut s = 0.0;
        for &(wgt, tab) in &LT {
            s += wgt * oku1(tab, ki, kt);
        }
        s
    };
    let gx = if t0(team) { GW } else { 0.0 };
    let vSG = (match sh {
        Some(s) => dis::sutV(k, s),
        None => k.D.shotV,
    }) * crate::top::chg_mul(ch);
    let opps: Vec<Op> = w.iter().filter(|p| !(p.team == team || bekci(p) || p.noTouch)).cloned().collect();
    let govOf = |k: &mut Kafa, ty: f64| -> f64 {
        let dg2 = veya(hypot(gx - x, ty - y), 1.0);
        let ux2 = (gx - x) / dg2;
        let uy2 = (ty - y) / dg2;
        let mut g = 1.0;
        for p in &opps {
            let al = (p.x - x) * ux2 + (p.y - y) * uy2;
            if al <= 0.0 || al >= dg2 {
                continue;
            }
            let pe = (-(p.x - x) * uy2 + (p.y - y) * ux2).abs();
            let kv_ = imkKov(k, p);
            let R = crate::core_sabit::Q.reach + jmax(0.0, veyo(tOnce, 0.0) + al / vSG - kv_.0) * kv_.1;
            if pe < R + 0.8 {
                g *= k.D.shotThru;
            }
        }
        g
    };
    let MO = MOUTH;
    let mut gov = f64::NEG_INFINITY;
    for a in [k.D.aimPow, -k.D.aimPow, k.D.aimPlase, -k.D.aimPlase] {
        gov = jmax(gov, govOf(k, GCY + a * MO));
    }
    let sig = sig.unwrap_or(f64::NAN);
    if !(sig > 0.3) {
        return gov * okuL(ko);
    }
    let h3 = 1.7320508 * sig;
    gov * (2.0 / 3.0 * okuL(ko) + (okuL(ko - h3) + okuL(ko + h3)) / 6.0)
}

// ------------------------------------------------------------------ imkân

pub const IMK_HUC: f64 = 6.0;
const IMK_SAP: [(f64, f64); 5] = [(0.0, 1.2), (15.0, 2.0), (30.0, 2.7), (50.0, 7.0), (80.0, 10.9)];
pub const IMK_UFUK: f64 = 150.0;
pub const IMK_V: f64 = 0.42;

/// JS imkSap(t) (decide.js:6255): t tik sonrasının konum belirsizliği (br)
pub fn imkSap(t: f64) -> f64 {
    let mut a = IMK_SAP[0];
    for b in IMK_SAP {
        if t <= b.0 {
            let f = if b.0 > a.0 { (t - a.0) / (b.0 - a.0) } else { 1.0 };
            return a.1 + (b.1 - a.1) * f;
        }
        a = b;
    }
    a.1
}
/// JS imkInan(t) (decide.js:6267): uzak geleceğe güven azalır
pub fn imkInan(k: &Kafa, t: f64) -> f64 {
    exp(-(veya(t, 0.0) + k.k2.IMK_OFS) / k.k2.IMK_INAN_T)
}
/// JS imkTau(t)
pub fn imkTau(t: f64) -> f64 {
    hypot(1.5, imkSap(t) / IMK_V)
}
/// JS imkSg(z)
#[inline(always)]
pub fn imkSg(z: f64) -> f64 {
    1.0 / (1.0 + exp(-z))
}
/// JS imkUzak(team, x, y) (decide.js:6628)
pub fn imkUzak(team: f64, x: f64, y: f64) -> bool {
    hypot((if t0(team) { GW } else { 0.0 }) - x, GCY - y) > 60.0
}

struct ImkO {
    p: Op,
    G: Option<dis::GecTab>,
    v: f64,
    R: f64,
    rc: f64,
}
struct ImkZ {
    i: usize,
    x: f64,
    y: f64,
    tr: Vec<f64>,
    yer: Option<Nokta>,
}
#[derive(Clone, Copy, Default)]
struct TopYol {
    t: f64,
    k: f64,
    s: Option<f64>,
}
#[derive(Clone, Copy)]
struct ImkE {
    ta: f64,
    k: f64,
    tv: f64,
    kalkis: f64,
    q: Option<Nokta>,
    S: Option<f64>,
}

/// JS imkan(w, team, B, h, ch, iz) (decide.js:6270): top B'deyken takımın şut imkânı. Her arkadaş için
/// şut hücrelerinden en iyisi: vaktinde varma olasılığı (rakiplerin varışıyla yarış) × topun oraya
/// gidebilmesi × oradan şutun değeri × geleceğe güven; rakipler arkadaşlara açgözlü eşlenir.
pub fn imkan(k: &mut Kafa, w: &[Op], team: f64, B: Nokta, h: Option<&Oyuncu>, ch: f64) -> f64 {
    let gx = if t0(team) { GW } else { 0.0 };
    let mut biz: Vec<ImkO> = Vec::new();
    let mut rak: Vec<ImkO> = Vec::new();
    for p in w {
        if p.noTouch {
            continue;
        }
        let o = ImkO { p: p.clone(), G: dis::AS_gecTab(k, p), v: dis::AS_vSp(k, p), R: jmin(2.0, crate::temas::menzil(p, 0.35)), rc: 0.0 };
        if p.team == team {
            if !bekci(p) {
                biz.push(o);
            }
        } else if !bekci(p) {
            let mut o = o;
            o.rc = jmax(0.0, dis::ALAN_TEPKI(k, p, "kovala"));
            rak.push(o);
        }
    }
    const IMK_BLOK: f64 = 2.4;
    // maçın kural görüntüsü imkân boyunca değişmez: bir kez alınır (varış başına kopyalanmaz)
    let kr = k.dis.kural();
    let varB = |k: &Kafa, o: &ImkO, x: f64, y: f64| -> f64 {
        let d = jmax(0.0, hypot(o.p.x - x, o.p.y - y) - IMK_BLOK);
        o.rc + dis::AS_gecOf(k, &o.G, x, y, d) + crate::kural::enerji::ALAN_KOSU_T(&o.p, d, o.v, &kr)
    };
    let var_ = |k: &Kafa, o: &ImkO, x: f64, y: f64, rc: f64| -> f64 {
        let d = jmax(0.0, hypot(o.p.x - x, o.p.y - y) - o.R);
        rc + dis::AS_gecOf(k, &o.G, x, y, d) + crate::kural::enerji::ALAN_KOSU_T(&o.p, d, o.v, &kr)
    };
    let rakVar = |k: &Kafa, x: f64, y: f64| -> f64 {
        let mut t = 1e9;
        for o in &rak {
            t = jmin(t, var_(k, o, x, y, o.rc));
        }
        t
    };
    let vh = match h {
        Some(h) => jmax(0.05, dis::AS_vSp(k, h) * dis::topK(k, h)),
        None => 0.3,
    };
    const KISA: f64 = 15.0;
    let bk: Option<Op> = w.iter().find(|p| p.team != team && bekci(p) && !p.noTouch).cloned();
    let tac = k.mac.as_ref().map(|m| m.tac.clone()).unwrap_or_default();
    let vk = bk.as_ref().map(|b| dis::AS_vSp(k, b)).unwrap_or(0.0);
    let rk = bk.as_ref().map(|b| jmax(0.0, dis::ALAN_TEPKI(k, b, "kovala"))).unwrap_or(0.0);
    let mut zs: Vec<ImkZ> = Vec::new();
    {
        let mut x = if gx == GW { GW - 4.0 } else { 4.0 };
        while (x - gx).abs() <= 40.0 {
            let mut y = GCY - 18.0;
            while y <= GCY + 18.0 {
                let tr = rak.iter().map(|o| varB(k, o, x, y)).collect();
                let yer = bk.as_ref().map(|b| {
                    sahte_ile(team, x, y, -1.0, |hold| {
                        let fm = dis::Fm { tac: &tac, ps: w, holder: Some(crate::dunya::Yerde::oyuncu(hold)) };
                        dis::AS_bekci(k, b, nk(x, y), Some(&fm))
                    })
                });
                zs.push(ImkZ { i: zs.len(), x, y, tr, yer });
                y += IMK_HUC;
            }
            x += if gx == GW { -IMK_HUC } else { IMK_HUC };
        }
    }
    let topYol = |k: &mut Kafa, x: f64, y: f64| -> TopYol {
        let L = hypot(x - B.x, y - B.y);
        let mut best = TopYol { t: 1e9, k: 0.0, s: None };
        let mut kk_ = 1.0;
        let mut s = 0.0;
        while s <= L + 0.01 {
            if s > 0.0 {
                let f = jmin(1.0, s / L);
                let tk = s / vh;
                kk_ = jmin(kk_, imkSg((rakVar(k, B.x + (x - B.x) * f, B.y + (y - B.y) * f) - tk) / imkTau(tk)));
            }
            let kalan = jmax(0.0, L - s);
            if kalan > jmax(KISA, if s == 0.0 { dis::pasMax(k) } else { KISA }) {
                s += IMK_HUC;
                continue;
            }
            let fq = if L != 0.0 { s / L } else { 0.0 };
            let q = nk(B.x + (x - B.x) * fq, B.y + (y - B.y) * fq);
            let lo = if kalan > 0.5 { dis::laneOk(k, w, team, q, nk(x, y), Some(k.D.arriveFirm), Some(k.D.sutHazir)) } else { 1.0 };
            let t = s / vh + if kalan > 0.5 { dis::ucusT(k, kalan, k.D.arriveFirm, None) + k.D.sutHazir } else { 0.0 };
            let kk = kk_ * lo;
            if x == 104.0 && y == 36.0 && crate::iz_acik() && std::env::var("ALAN_IMK_HU").is_ok() {
                eprintln!("    ty s {} kalan {:.3} k {:.6} lo {:.6} t {:.3}", s, kalan, kk_, lo, t);
            }
            if 1e3 * kk - t > 1e3 * best.k - best.t {
                best = TopYol { t, k: kk, s: Some(s) };
            }
            s += IMK_HUC;
        }
        best
    };
    let mut TY: Vec<Option<TopYol>> = vec![None; zs.len()];
    let bkAt = |z: &ImkZ, bas: Nokta, kalkis: f64, ta: f64| -> Option<Nokta> {
        if bk.is_none() {
            return None;
        }
        let yer = z.yer?;
        let dx = yer.x - bas.x;
        let dy = yer.y - bas.y;
        let L = hypot(dx, dy);
        let yol = jmax(0.0, ta - kalkis - rk) * vk;
        Some(if L < 1e-6 || yol >= L { yer } else { nk(bas.x + dx * yol / L, bas.y + dy * yol / L) })
    };
    // o.ic: arkadaş × hücre
    let mut IC: Vec<Vec<Option<ImkE>>> = vec![vec![None; zs.len()]; biz.len()];
    // imk0
    let imk0 = |k: &mut Kafa, TY: &mut Vec<Option<TopYol>>, oi: usize, zi: usize| -> ImkE {
        let o = &biz[oi];
        let z = &zs[zi];
        let own = h.map(|h| o.p.id == h.id && o.p.team == h.team).unwrap_or(false);
        let (ta, kk);
        let mut kalkis = 0.0;
        let mut q = None;
        if own {
            // topYol hücrenin (x, y)'sinin saf işlevi: taşıyıcı da arkadaşlarla aynı önbelleği kullanır
            let r = match TY[z.i] {
                Some(r) => r,
                None => {
                    let r = topYol(k, z.x, z.y);
                    TY[z.i] = Some(r);
                    r
                }
            };
            let Lz = hypot(z.x - B.x, z.y - B.y);
            if r.s.is_none() || (r.s.unwrap() - Lz).abs() > 0.5 {
                ta = Lz / vh;
                kk = if r.s.is_some() { r.k } else { 0.0 };
            } else {
                ta = r.t;
                kk = r.k;
            }
        } else {
            let r = match TY[z.i] {
                Some(r) => r,
                None => {
                    let r = topYol(k, z.x, z.y);
                    TY[z.i] = Some(r);
                    r
                }
            };
            ta = jmax(var_(k, o, z.x, z.y, 0.0), r.t);
            kk = r.k;
            if let Some(s) = r.s {
                kalkis = s / vh;
                let L = veya(hypot(z.x - B.x, z.y - B.y), 1.0);
                q = Some(nk(B.x + (z.x - B.x) * s / L, B.y + (z.y - B.y) * s / L));
            }
        }
        if ta > IMK_UFUK || kk <= 0.0 {
            ImkE { ta, k: 0.0, tv: f64::NAN, kalkis: 0.0, q: None, S: None }
        } else {
            ImkE { ta, k: kk, tv: ta + k.D.sutHazir, kalkis, q, S: None }
        }
    };
    // hücrenin şut yolu açıklığı oyuncuya bağlı değil (sutYol bu girdilerle saf): hücre başına bir kez
    let SY: Vec<std::cell::Cell<Option<f64>>> = vec![std::cell::Cell::new(None); zs.len()];
    // Sof: hücreden şutun değeri
    let Sof = |k: &mut Kafa, oi: usize, zi: usize, ta: f64, kalkis: f64, q: Option<Nokta>| -> f64 {
        let o = &biz[oi];
        let z = &zs[zi];
        let bas = match (&bk, q) {
            (Some(b), Some(q)) => {
                let fm = dis::Fm { tac: &tac, ps: w, holder: h.map(crate::dunya::Yerde::oyuncu) };
                dis::AS_bekci(k, b, q, Some(&fm))
            }
            (Some(b), None) => nk(b.x, b.y),
            (None, _) => nk(0.0, 0.0),
        };
        let r = sutTab(k, w, team, z.x, z.y, ch, Some(&o.p), Some(0.96), z.yer, bkAt(z, bas, kalkis, ta), Some(ta + k.D.sutHazir));
        let sy = match SY[zi].get() {
            Some(v) => v,
            None => {
                let v = sutYol(k, w, team, z.x, z.y, None);
                SY[zi].set(Some(v));
                v
            }
        };
        r * sy
    };
    let imk = |k: &mut Kafa, TY: &mut Vec<Option<TopYol>>, IC: &mut Vec<Vec<Option<ImkE>>>, oi: usize, zi: usize, tR: f64| -> (f64, ImkE) {
        let e = match IC[oi][zi] {
            Some(e) => e,
            None => {
                let e = imk0(k, TY, oi, zi);
                IC[oi][zi] = Some(e);
                e
            }
        };
        if e.ta > IMK_UFUK || e.k <= 0.0 {
            (0.0, e)
        } else {
            (e.k * imkSg((tR - e.tv) / imkTau(e.ta)), e)
        }
    };
    let imkS = |k: &mut Kafa, IC: &mut Vec<Vec<Option<ImkE>>>, oi: usize, zi: usize| -> f64 {
        let mut e = IC[oi][zi].unwrap();
        if e.S.is_none() {
            e.S = Some(if e.k > 0.0 && e.ta <= IMK_UFUK { Sof(k, oi, zi, e.ta, e.kalkis, e.q) } else { 0.0 });
            IC[oi][zi] = Some(e);
        }
        e.S.unwrap()
    };
    // açgözlü eşleme
    let mut bos: Vec<usize> = (0..rak.len()).collect();
    let mut atanan: Vec<Option<isize>> = vec![None; biz.len()];
    struct EnI {
        u: f64,
        z: Option<usize>,
        S: f64,
        ta: f64,
    }
    let enIyi = |k: &mut Kafa, TY: &mut Vec<Option<TopYol>>, IC: &mut Vec<Vec<Option<ImkE>>>, bos: &Vec<usize>, a: usize, di: Option<isize>| -> EnI {
        let mut r = EnI { u: 0.0, z: None, S: 0.0, ta: 0.0 };
        for zi in 0..zs.len() {
            let z = &zs[zi];
            let tR = match di {
                Some(d) => {
                    if d >= 0 {
                        z.tr[d as usize]
                    } else {
                        f64::NAN
                    }
                }
                None => {
                    let mut t = 1e9;
                    for &i in bos {
                        t = jmin(t, z.tr[i]);
                    }
                    t
                }
            };
            let (p, e) = imk(k, TY, IC, a, zi, tR);
            if di.is_none() && &*biz[a].p.govde.name == "Kaan" && z.x == 104.0 && z.y == 36.0 && crate::iz_acik() && std::env::var("ALAN_IMK_HU").is_ok() {
                eprintln!("    kaan ön z {} {} tR {:.3} p {:.6} ta {:.3} k {} kalkis {} q {:?}", z.x, z.y, tR, p, e.ta, e.k, e.kalkis, e.q.map(|q| (q.x, q.y)));
            }
            if p * imkInan(k, e.ta) <= r.u {
                continue;
            }
            let S = imkS(k, IC, a, zi);
            let u = p * S * imkInan(k, e.ta);
            if di.is_none() && &*biz[a].p.govde.name == "Kaan" && crate::iz_acik() && std::env::var("ALAN_IMK_HU").is_ok() {
                eprintln!("    kaan z {} {} tR {:.3} p {:.6} ta {:.3} S {:.6} u {:.6}", z.x, z.y, tR, p, e.ta, S, u);
            }
            if u > r.u {
                r = EnI { u, z: Some(zi), S, ta: e.ta };
            }
        }
        r
    };
    while !bos.is_empty() {
        let mut ba: Option<usize> = None;
        let mut bu = 0.002;
        for a in 0..biz.len() {
            if atanan[a].is_some() {
                continue;
            }
            let r = enIyi(k, &mut TY, &mut IC, &bos, a, None);
            if r.u > bu {
                bu = r.u;
                ba = Some(a);
            }
        }
        let ba = match ba {
            Some(a) => a,
            None => break,
        };
        let rz = enIyi(k, &mut TY, &mut IC, &bos, ba, None).z.unwrap();
        let mut di: isize = -1;
        let mut dt = 1e9;
        for &i in &bos {
            if zs[rz].tr[i] < dt {
                dt = zs[rz].tr[i];
                di = i as isize;
            }
        }
        if crate::iz_acik() && std::env::var("ALAN_IMK_HU").is_ok() {
            eprintln!("  açg {} u {:.6} z {} {} tr {} → {}", biz[ba].p.govde.name, bu, zs[rz].x, zs[rz].y, bos.iter().map(|&i| format!("{}:{:.3}", rak[i].p.govde.name, zs[rz].tr[i])).collect::<Vec<_>>().join(" "), if di >= 0 { rak[di as usize].p.govde.name.clone() } else { "-".into() });
        }
        atanan[ba] = Some(di);
        bos.retain(|&i| i as isize != di);
    }
    let mut v: f64 = 0.0;
    for a in 0..biz.len() {
        let r = enIyi(k, &mut TY, &mut IC, &bos, a, atanan[a]);
        let S = if r.z.is_some() { r.S } else { 0.0 };
        let p = if S > 0.0 { r.u / S } else { 0.0 };
        if crate::iz_acik() && std::env::var("ALAN_IMK_HU").is_ok() {
            eprintln!("  hu {} u={:.6} S={:.6} ta={:.3} z={:?} kar={:?}", biz[a].p.govde.name, r.u, S, r.ta, r.z.map(|z| (zs[z].x, zs[z].y)), atanan[a].and_then(|i| if i >= 0 { Some(rak[i as usize].p.govde.name.clone()) } else { None }));
        }
        v = jmax(v, p * S);
    }
    v
}

/// JS sutYol(w, team, x, y, tS) (decide.js:6591): şutun gövdelerden geçme olasılığı (en açık nişan;
/// tS verilirse rakiplerin yeri belirsiz: yumuşak).
pub fn sutYol(k: &mut Kafa, w: &[Op], team: f64, x: f64, y: f64, tS: Option<f64>) -> f64 {
    let sg = match tS {
        None => 0.0,
        Some(t) => imkSap(t),
    };
    let gx = if t0(team) { GW } else { 0.0 };
    // sahte taşıyıcı (rolü boş: varsayılan zaten "")
    let hedef = sahte_ile(team, x, y, -9.0, |hh| dis::sutHedef(k, hh, w, gx));
    let mut en: f64 = 0.0;
    for yt in hedef {
        let d = veya(hypot(gx - x, yt - y), 1.0);
        let ux = (gx - x) / d;
        let uy = (yt - y) / d;
        let mut pass = 1.0;
        for p in w {
            if p.team == team || bekci(p) || p.noTouch {
                continue;
            }
            let al = (p.x - x) * ux + (p.y - y) * uy;
            if al <= 0.0 || al >= d {
                continue;
            }
            let pe = (-(p.x - x) * uy + (p.y - y) * ux).abs();
            let R = crate::core_sabit::Q.reach + jmax(0.0, al / 2.4 - dis::ALAN_TEPKI(k, p, "kovala")) * (0.17 + 0.006 * ALAN_HZ(p)) * dis::ALAN_SPRE(k, p);
            if sg > 0.0 {
                pass *= 1.0 - (1.0 - k.D.shotThru) * imkSg((R + 0.8 - pe) / sg);
            } else if pe < R + 0.8 {
                pass *= k.D.shotThru;
            }
        }
        if pass > en {
            en = pass;
        }
        if en >= 1.0 {
            break;
        }
    }
    en
}

/// JS sutS(w, team, x, y, ch, k, tS) (decide.js:6618): (x, y)'den şutun değeri (dibindeki rakip
/// şutu keser; tS: ne kadar sonra).
pub fn sutS(k: &mut Kafa, w: &[Op], team: f64, x: f64, y: f64, ch: f64, kq: Option<&Oyuncu>, tS: Option<f64>) -> f64 {
    let sg = match tS {
        None => 0.0,
        Some(t) => imkSap(t),
    };
    let mut fB = 1.0;
    for p in w {
        if p.team != team && !bekci(p) && !p.noTouch {
            let dd = hypot(p.x - x, p.y - y);
            if sg > 0.0 {
                fB *= 1.0 - imkSg((2.4 - dd) / sg);
            } else if dd < 2.4 {
                return 0.0;
            }
        }
    }
    let r = sutTab(k, w, team, x, y, ch, kq, None, None, None, None);
    fB * (r * sutYol(k, w, team, x, y, tS))
}

/// JS sutVakit(w, team, x, y, k) (decide.js:5805): şuta hazırlanacak vakit (sutHazir tik topu tutma);
/// keepP.son korunur.
pub fn sutVakit(k: &mut Kafa, w: &[Op], team: f64, x: f64, y: f64, kq: Option<&Oyuncu>) -> f64 {
    let kk: Oyuncu = match kq {
        Some(q) => q.clone(),
        None => {
            let mut bd = 1e9;
            let mut b: Option<&Op> = None;
            for o in w {
                if o.team != team || o.noTouch || bekci(o) {
                    continue;
                }
                let d = hypot(o.x - x, o.y - y);
                if d < bd {
                    bd = d;
                    b = Some(o);
                }
            }
            match b {
                Some(b) => (**b).clone(),
                None => return 1.0,
            }
        }
    };
    let s = k.keepP_son.get();
    let mut r = kk;
    r.x = x;
    r.y = y;
    r.vx = 0.0;
    r.vy = 0.0;
    let p = jmax(0.0, jmin(1.0, dis::keepP(k, &r, w, None, k.D.sutHazir)));
    k.keepP_son.set(s);
    p
}

pub const SUR_BOS: f64 = 24.0;
pub const SUR_PARCA: f64 = 8.0;

/// JS tasima(w, team, k, x, y, ch) (decide.js:5828): topu (x, y)'den kaleye ya da ileri boş alana
/// sürüp şut: şut değerinin en iyisi.
pub fn tasima(k: &mut Kafa, w: &[Op], team: f64, kq: Option<&Oyuncu>, x: f64, y: f64, ch: f64) -> f64 {
    let gx = if t0(team) { GW } else { 0.0 };
    let dx = gx - x;
    let dy = GCY - y;
    let dg = veya(hypot(dx, dy), 1.0);
    let ug = nk(dx / dg, dy / dg);
    let uf = nk(if t0(team) { 1.0 } else { -1.0 }, 0.0);
    let mut dirs = vec![(ug, true)];
    let r: Option<Oyuncu> = kq.map(|q| {
        let mut c = q.clone();
        c.x = x;
        c.y = y;
        c.vx = 0.0;
        c.vy = 0.0;
        c
    });
    let vC = 0.1955;
    let opp: Vec<Op> = w.iter().filter(|o| o.team != team && !o.noTouch).cloned().collect();
    if ug.x * uf.x < cos(0.35) {
        dirs.push((uf, false));
    }
    let wb = bekYerW(k, w, team, nk(x, y), None);
    let mut best = dis::shotQ(k, &wb, team, x, y, ch) * sutVakit(k, w, team, x, y, r.as_ref());
    for (u, gol) in dirs {
        let mut L = SUR_BOS;
        L = if gol { jmin(L, dg - 1.5) } else { jmin(L, if t0(team) { GW - 2.0 - x } else { x - 2.0 }) };
        let mut son: Option<Nokta> = None;
        let mut s0 = 2.0;
        while s0 <= L {
            let P = nk(x + u.x * s0, y + u.y * s0);
            if P.y < 2.0 || P.y > GH - 2.0 {
                break;
            }
            let tc = match &r {
                Some(r) => dis::surSure(k, r, s0, Some(u), None),
                None => s0 / vC,
            };
            let mut td: f64 = 1e9;
            for o in &opp {
                td = jmin(td, jmax(0.0, dis::ALAN_TEPKI(k, o, "kovala")) + jmax(0.0, hypot(o.x - P.x, o.y - P.y) - 2.4) / ((0.17 + 0.006 * ALAN_HZ(o)) * dis::ALAN_SPRE(k, o)));
            }
            if td - tc < k.D.sutHazir {
                break;
            }
            son = Some(P);
            s0 += 2.0;
        }
        if let Some(son) = son {
            let wb = bekYerW(k, w, team, son, None);
            best = jmax(best, dis::shotQ(k, &wb, team, son.x, son.y, ch));
        }
    }
    best
}

/// JS sutEngel(src, team, x, y) (decide.js:5440): şut çizgisinde kesebilecek gövdeler.
pub fn sutEngel(k: &Kafa, src: &[Op], team: f64, x: f64, y: f64) -> f64 {
    let gx = if t0(team) { GW } else { 0.0 };
    let d = veya(hypot(gx - x, GCY - y), 1.0);
    let mut pass = 1.0;
    let ux = (gx - x) / d;
    let uy = (GCY - y) / d;
    for p in src {
        if p.team == team || bekci(p) {
            continue;
        }
        let al = (p.x - x) * ux + (p.y - y) * uy;
        if al <= 0.0 || al >= d {
            continue;
        }
        let pe = (-(p.x - x) * uy + (p.y - y) * ux).abs();
        let reachT = jmax(0.0, al / 2.4 - dis::ALAN_TEPKI(k, p, "kovala")) * (0.17 + 0.006 * ALAN_HZ(p)) * dis::ALAN_SPRE(k, p);
        let R = crate::core_sabit::Q.reach + reachT;
        if pe < R + 0.8 {
            pass *= k.D.shotThru;
        }
    }
    pass
}

/// JS bskTab(team, bk) (decide.js:5427): boşluk noktalarının şut değeri önbelleği (tik başına).
fn bskTab<'a>(k: &'a mut Kafa, team: f64, bk: Option<&Oyuncu>) -> &'a mut Vec<f64> {
    if k.k2.BSKt != k.bag._tick || k.k2.BSKt.is_none() {
        k.k2.BSK.clear();
        k.k2.BSKt = k.bag._tick;
    }
    let key = match bk {
        Some(b) => format!("{}|{}|{:?}|{}", team, b.id, b.en, if b.hal._yer { 1 } else { 0 }),
        None => format!("{}|-", team),
    };
    let n = dis::SPT().len();
    k.k2.BSK.entry(key).or_insert_with(|| vec![-1.0; n])
}

/// JS bosluk(src, team, bx, by) (decide.js:5454; eski kafa): top (bx, by)'deyken kale önündeki
/// boşluğa koşup şut çekme değeri.
pub fn bosluk(k: &mut Kafa, src: &[Op], team: f64, bx: f64, by: f64) -> f64 {
    let gx = if t0(team) { GW } else { 0.0 };
    let dir = if t0(team) { -1.0 } else { 1.0 };
    let O = dis::bosOy(k, src, team, bx, by);
    let bk: Option<Op> = src.iter().find(|p| p.team != team && bekci(p) && !p.noTouch).cloned();
    let B = nk(bx, by);
    let mut best = 0.0;
    let spt = dis::SPT();
    for (kk, &(r, an)) in spt.iter().enumerate() {
        let x = gx + dir * r * cos(an);
        let y = GCY + r * sin(an);
        if y < 1.0 || y > GH - 1.0 {
            continue;
        }
        let mut ta: f64 = 1e9;
        let mut td: f64 = 1e9;
        for j in 0..O.x.len() {
            let tk = O.t[j];
            if tk == 0 {
                continue;
            }
            let t = hypot(O.x[j] - x, O.y[j] - y) / O.v[j];
            if tk == 1 {
                if t < ta {
                    ta = t;
                }
            } else {
                let tt = t + 6.0 - O.a[j];
                if tt < td {
                    td = tt;
                }
            }
        }
        let tb = dis::ucusT(k, hypot(bx - x, by - y), k.D.arriveFirm, None);
        let own = 1.0 / (1.0 + exp((jmax(ta, tb) - td) / 6.0));
        if own <= best {
            continue;
        }
        let mut sk = bskTab(k, team, bk.as_deref())[kk];
        if sk < 0.0 {
            sk = match &bk {
                Some(b) => {
                    let g = dis::AS_bekci(k, b, nk(x, y), None);
                    let bw = vec![kxyv(b, g.x, g.y, 0.0, 0.0)];
                    dis::sutKapali(k, &bw, team, x, y, 0.4)
                }
                None => dis::sutKapali(k, src, team, x, y, 0.4),
            };
            bskTab(k, team, bk.as_deref())[kk] = sk;
        }
        if own * sk <= best {
            continue;
        }
        let Lo = dis::laneOk(k, src, team, B, nk(x, y), None, None);
        if own * Lo * sk <= best {
            continue;
        }
        let v = own * Lo * sk * sutEngel(k, src, team, x, y);
        if v > best {
            best = v;
        }
    }
    best
}

/// bekKay'in yeniden dışa açılışı (kafa2_imkan içinden kullananlar için)
pub fn _bekKay(k: &Kafa, p: &Op, B: Nokta, T: f64) -> Op {
    bekKay(k, p, B, T)
}
