//! Taşıyıcının seçenekleri (decide.js `options`, sade kaynak 2808–3491).
//!
//! Yalnız canlı yol taşındı: kafa her kararda sahiplik haritasını (`SAH_F`) kurar (ALAN_PRENSIP açık,
//! iki gerçek maçta 137 çağrının hepsinde harita vardı), bu yüzden JS'teki `TEK`/`HEP` koşulları hep doğru:
//! eski pas adayları (`!TEK`: düz pas, sert pas, araya, niyet), boşluk noktaları ve ağız/buluşma koşuları
//! (`!HEP`) ölü koddur. `TOPLA` (h.id ≥ 0) iki kolu da taşındı.
//!
//! JS'te seçenekler nesne kimliğiyle karşılaştırılır (`q === h`, `al.some(P => P === c.P)`): burada
//! oyuncu (takım, id) ile, nokta da üretildiği sıranın numarasıyla karşılaştırılır.
#![allow(non_snake_case)]

use crate::kafa::gecici_dis1::KuralErisim as _;
use super::kafa1_baglam::Kafa;
use super::kafa1_js::{nk, Nk};
use super::kafa1_tehlike::kuyuD;
use crate::mat::{cos, exp, hypot, jmax, jmin, kis, sin, PI};
use crate::model::{JOLaunch, JsDeger, Nokta, Oyuncu, Pid, Secenek};
use crate::oyuncu::Tepki;
use crate::saha::{CY as GCY, H as GH, W as GW};

/// Hayali dünyanın oyuncusu (kafa2 ile ortak tip; birleştirmede tek tanım).
pub type Op = std::sync::Arc<Oyuncu>;

/// options'ın dönüşü: seçenekler ve JS dizisine asılı yan bilgiler (out.tekH, out.tekS, out.tekC).
#[derive(Clone, Debug, Default)]
pub struct Secenekler {
    pub out: Vec<Secenek>,
    /// her alıcı için seçilen noktalar (JS out.tekH: Map alıcı → noktalar); alıcı = src sırası (taşıyıcı da)
    pub tekH: Vec<(Pid, Vec<Nk>)>,
    /// sürme noktaları (JS out.tekS) ve bütün sürme adayları (JS out.tekC)
    pub tekS: Vec<Nk>,
    pub tekC: Vec<TekC>,
}

/// Sürme adayı (JS C3 öğesi).
#[derive(Clone, Copy, Debug)]
pub struct TekC {
    pub P: Nk,
    pub u: Nk,
    pub r: f64,
    pub own: f64,
    pub ownS: Option<f64>,
    pub s: f64,
}

#[inline(always)]
/// Oyuncu kimliği: aynı takımın aynı numaralı oyuncusu mu (kopyası da sayılır). Nesne kimliğinden farklı:
/// MIMARI.md 3.2.
fn ayni_oyuncu(a: &Oyuncu, b: &Oyuncu) -> bool {
    a.team == b.team && a.id == b.id
}
#[inline(always)]
fn sig(x: f64) -> f64 {
    1.0 / (1.0 + exp(x))
}

/// JS `+v.toFixed(2)` (ondalık tam değerden, eşitlikte büyüğe).
pub fn to_fixed2(v: f64) -> f64 {
    if !v.is_finite() {
        return v;
    }
    let neg = v < 0.0;
    let s = format!("{:.40}", v.abs());
    let (ip, fp) = s.split_once('.').unwrap();
    let digits: Vec<u8> = fp.bytes().map(|c| c - b'0').collect();
    let mut cent: i128 = ip.parse::<i128>().unwrap() * 100 + (digits[0] as i128) * 10 + digits[1] as i128;
    let kalan = &digits[2..];
    if kalan[0] >= 5 {
        // ≥ 0,005: yukarı (eşitlikte de büyüğe: JS toFixed)
        cent += 1;
    }
    let r = cent as f64 / 100.0;
    if neg {
        -r
    } else {
        r
    }
}

/// Bir alıcı: src'deki oyuncu ya da (kendine pas) taşıyıcının kendisi.
#[derive(Clone, Copy)]
enum Al {
    Src(usize),
    H,
}

/// C2 adayı (kendine/arkadaşa pas noktası).
struct Aday {
    pi: usize,
    P: Nk,
    hz: f64,
    own: f64,
    onune: bool,
    s: f64,
    spr: bool,
    itK: Option<f64>,
}

fn yeni(kind: &str) -> Secenek {
    Secenek { kind: kind.to_string(), ..Default::default() }
}

impl Kafa {
    /// JS `varA` (TEK ve TOPLA kollarında aynı): o'nun (x, y)'ye varışı (koşu profili + dönüş kaybı).
    fn varA(&self, o: &Oyuncu, x: f64, y: f64, gt: &mut Vec<(f64, f64, Option<super::gecici_dis1::GecTab>)>) -> f64 {
        let d = hypot(x - o.x, y - o.y);
        let t0 = if self.dis.ALAN_SPR_FAZ().is_some() { self.dis.ALAN_KOSU_T(o, d, self.vS(o)) } else { d / self.vS(o) };
        if d < 1e-6 {
            return t0;
        }
        let i = match gt.iter().position(|g| g.0 == o.team && g.1 == o.id) {
            Some(i) => i,
            None => {
                gt.push((o.team, o.id, self.dis.gecTab(o, false)));
                gt.len() - 1
            }
        };
        match &gt[i].2 {
            Some(G) => t0 + self.dis.gecOf(G, x, y, Some(d)),
            None => t0,
        }
    }

    /// JS `lane(a, b)`: yolun dört ara noktasında rakibin en büyük etkisi.
    fn lane(&self, src: &[Op], team: f64, a: &Oyuncu, b: &Oyuncu) -> f64 {
        let mut m = 0.0;
        for i in 1..=4 {
            let f = i as f64 / 5.0;
            m = jmax(m, self.oppAt(src, team, a.x + (b.x - a.x) * f, a.y + (b.y - a.y) * f));
        }
        m
    }

    /// Taşıyıcının seçenekleri (JS options(h, src, ch)).
    pub fn options(&self, h: &Oyuncu, src: &[Op], ch: f64) -> Secenekler {
        let team = h.team;
        let gx = if team == 0.0 { GW } else { 0.0 };
        let D = &self.D;
        let B = &self.bag;
        let saf = self.SAH_F.clone().expect("options: SAH_F (kafa her kararda kurar)");
        let SAH = |t: f64, x: f64, y: f64| -> f64 { saf.oku(t, x, y) };
        let TOPLA = h.id >= 0.0;
        let mut res = Secenekler::default();
        let hi = src.iter().position(|q| ayni_oyuncu(q, h));
        let qid = |a: Al| -> Option<Pid> {
            match a {
                Al::Src(i) => Some(i),
                Al::H => hi,
            }
        };
        let passMaxF = D.passMaxF;
        let pasMax = self.pasMax();
        // arkadaşlar (pas alıcıları): yolun rakipsizliği × sahiplik değeri, en iyi nMates
        let mut mates: Vec<(usize, f64)> = Vec::new();
        for (i, q) in src.iter().enumerate() {
            if q.team != team || ayni_oyuncu(q, h) || q.bekci() {
                continue;
            }
            let d = hypot(q.x - h.x, q.y - h.y);
            if !(d >= 4.0 && d <= passMaxF) {
                continue;
            }
            mates.push((i, exp(2.0 * -self.lane(src, team, h, q)) * (jmax(0.0, SAH(team, q.x, q.y)) + 0.02)));
        }
        mates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        mates.truncate(D.nMates as usize);
        let mut mates: Vec<usize> = mates.into_iter().map(|m| m.0).collect();
        if !h.bekci() {
            if let Some(ki) = src.iter().position(|q| q.team == team && q.bekci() && !ayni_oyuncu(q, h) && !q.noTouch) {
                let kp = &src[ki];
                let dk = hypot(kp.x - h.x, kp.y - h.y);
                if dk >= 4.0 && dk <= jmin(passMaxF, 40.0 * GW / 100.0) && (h.x - if team == 0.0 { 0.0 } else { GW }).abs() <= GW / 2.0 {
                    mates.push(ki);
                }
            }
        }
        let rak: Vec<usize> = (0..src.len()).filter(|&i| src[i].team != team && !src[i].noTouch).collect();
        let mut gt = Vec::new();
        let F = self.dis.ALAN_SPR_FAZ();
        let g = if team == 0.0 { 1.0 } else { -1.0 };
        let HZ = [D.arriveSpace, D.arrive, D.arriveFirm, 1.2, 1.6];
        // JS KUYU_HED() ve KY() = C().KUYU(): bugün ikisi de açık
        let kuyu_hed = true;
        let ky = true;
        // ---- TEK: her alıcı için noktalar (kendine pas dahil) ----
        {
            let mut alicilar: Vec<Al> = mates.iter().map(|&i| Al::Src(i)).collect();
            if F.as_ref().map(|f| f.itKos != 0.0 && f.itKos == f.itKos).unwrap_or(false) && !h.bekci() {
                alicilar.push(Al::H);
            }
            for (i, q) in src.iter().enumerate() {
                if q.team != team || ayni_oyuncu(q, h) || q.bekci() || q.noTouch || alicilar.iter().any(|a| matches!(a, Al::Src(j) if *j == i)) {
                    continue;
                }
                let d = hypot(q.x - h.x, q.y - h.y);
                if d >= 4.0 && d <= pasMax && (q.vx * g > 0.08 || (q.tx - q.x) * g > 6.0) {
                    alicilar.push(Al::Src(i));
                }
            }
            for &a in &alicilar {
                let q: &Oyuncu = match a {
                    Al::Src(i) => &src[i],
                    Al::H => h,
                };
                let qH = matches!(a, Al::H) || ayni_oyuncu(q, h);
                let mut dirs: Vec<Nk> = Vec::new();
                let ek = |dirs: &mut Vec<Nk>, x: f64, y: f64, esik: f64| {
                    let l = hypot(x, y);
                    if l > 1e-6 && !dirs.iter().any(|u| u.x * x / l + u.y * y / l > esik) {
                        dirs.push(nk(x / l, y / l));
                    }
                };
                let sv = hypot(q.vx, q.vy);
                let kos = if sv > 0.08 {
                    Some(nk(q.vx, q.vy))
                } else if hypot(q.tx - q.x, q.ty - q.y) > 2.0 {
                    Some(nk(q.tx - q.x, q.ty - q.y))
                } else {
                    None
                };
                let taban = kos.unwrap_or(nk(gx - q.x, GCY - q.y));
                let tl = {
                    let v = hypot(taban.x, taban.y);
                    if v == 0.0 || v != v {
                        1.0
                    } else {
                        v
                    }
                };
                if let Some(k) = kos {
                    ek(&mut dirs, k.x, k.y, 0.94);
                }
                ek(&mut dirs, gx - q.x, GCY - q.y, 0.94);
                {
                    let (mut nq, mut nd) = (None::<usize>, 1e9);
                    for &o in &rak {
                        if src[o].bekci() {
                            continue;
                        }
                        let d = hypot(src[o].x - q.x, src[o].y - q.y);
                        if d < nd {
                            nd = d;
                            nq = Some(o);
                        }
                    }
                    if let Some(o) = nq {
                        if nd < 12.0 {
                            ek(&mut dirs, q.x - src[o].x, q.y - src[o].y, 0.94);
                        }
                    }
                }
                ek(&mut dirs, h.x - q.x, h.y - q.y, 0.94);
                for k in 0..10 {
                    let a = k as f64 * PI / 5.0;
                    ek(&mut dirs, cos(a), sin(a), 0.94);
                }
                for a in [0.6, -0.6, 1.2, -1.2] {
                    let c = cos(a);
                    let sn = sin(a);
                    ek(&mut dirs, (taban.x * c - taban.y * sn) / tl, (taban.x * sn + taban.y * c) / tl, 0.94);
                }
                let mut nok = vec![nk(q.x, q.y)];
                for u in &dirs {
                    for r in [1.5, 3.0, 5.0, 8.0, 12.0, 17.0, 21.0] {
                        nok.push(nk(q.x + u.x * r, q.y + u.y * r));
                    }
                }
                let mut C2: Vec<Aday> = Vec::new();
                for (pi, &P) in nok.iter().enumerate() {
                    if P.x < 2.0 || P.x > GW - 2.0 || P.y < 2.0 || P.y > GH - 2.0 {
                        continue;
                    }
                    let L = hypot(P.x - h.x, P.y - h.y);
                    if L < 4.0 || L > pasMax || (kuyu_hed && !ky && kuyuD(P.x, P.y) > -0.3) {
                        continue;
                    }
                    let ta0 = self.varA(q, P.x, P.y, &mut gt);
                    let mut td = 1e9;
                    for &o in &rak {
                        td = jmin(td, self.varA(&src[o], P.x, P.y, &mut gt) + jmax(0.0, self.TEPKI(&src[o], Tepki::Kovala)));
                    }
                    let sf = SAH(team, P.x, P.y);
                    let mds: &[i32] = if qH && self.SPK() {
                        if true /* JS SUR(): sürekli koşu açık */ {
                            if self.SPRS(h) {
                                &[2, 0, 3, 1]
                            } else {
                                &[2, 0]
                            }
                        } else if self.SPRS(h) {
                            &[0, 1]
                        } else {
                            &[0]
                        }
                    } else {
                        &[-1]
                    };
                    for &md in mds {
                        let ta = if md < 0 { ta0 } else { self.itT(h, P, md) };
                        let hzs: Vec<f64> = if qH {
                            (|| -> Vec<f64> {
                                if (P.x - h.x) * g < 3.0 {
                                    return vec![];
                                }
                                {
                                    let ux = (P.x - h.x) / L;
                                    let uy = (P.y - h.y) / L;
                                    let ib = F.as_ref().and_then(|f| f.itBos).filter(|v| *v != 0.0 && v == v).unwrap_or(6.0);
                                    let ls = jmin(L, ib);
                                    for &o in &rak {
                                        let o = &src[o];
                                        let al = kis((o.x - h.x) * ux + (o.y - h.y) * uy, 0.0, ls);
                                        if hypot(h.x + ux * al - o.x, h.y + uy * al - o.y) < 2.5 {
                                            return vec![];
                                        }
                                    }
                                }
                                if md < 0 {
                                    let f = F.as_ref().expect("ALAN_SPR_FAZ");
                                    let kal = match &h._sf {
                                        Some(s) => f.patT + f.tepeT - (self.tick() - s.t0),
                                        None => f.patT + f.tepeT,
                                    };
                                    if ta > kal {
                                        return vec![];
                                    }
                                }
                                let (mut lo, mut hi) = (0.02, 2.5);
                                if self.ucusT(L, hi, None) > ta {
                                    return vec![];
                                }
                                for _ in 0..18 {
                                    let mid = (lo + hi) / 2.0;
                                    if self.ucusT(L, mid, None) > ta {
                                        lo = mid;
                                    } else {
                                        hi = mid;
                                    }
                                }
                                let _ = lo;
                                if hi > 0.9 * if md < 0 { self.vS(h) } else { self.itV(h, md, ta) } {
                                    vec![]
                                } else {
                                    vec![hi]
                                }
                            })()
                        } else {
                            HZ.to_vec()
                        };
                        for hz in hzs {
                            let tb = self.ucusT(L, hz, None);
                            if hz > D.arriveSpace && tb < ta - 3.0 {
                                continue;
                            }
                            let own = sig((jmax(ta, tb) - td) / 5.0);
                            C2.push(Aday { pi, P, hz, own, onune: ta - tb > 3.0, s: own * sf, spr: md == 1, itK: if qH && md >= 0 && true /* JS SUR(): sürekli koşu açık */ { Some(self.itKf(h, md)) } else { None } });
                        }
                    }
                }
                // JS sort kararlı: b.s - a.s
                C2.sort_by(|a, b| b.s.partial_cmp(&a.s).unwrap_or(std::cmp::Ordering::Equal));
                let mut secT: Vec<Nk> = Vec::new();
                let mut secT_i: Vec<usize> = Vec::new();
                for onune in [false, true] {
                    let mut al: Vec<(usize, Nk)> = Vec::new();
                    for c in &C2 {
                        if c.onune != onune || !(c.s > 0.0) {
                            continue;
                        }
                        if !al.iter().any(|a| a.0 == c.pi) {
                            if al.len() >= 2 || al.iter().any(|a| hypot(a.1.x - c.P.x, a.1.y - c.P.y) < 2.0) {
                                continue;
                            }
                            al.push((c.pi, c.P));
                            if !secT_i.contains(&c.pi) {
                                secT_i.push(c.pi);
                                secT.push(c.P);
                            }
                        }
                        let mut o = yeni(if onune { "önüne" } else { "pas" });
                        o.to = Some(Nokta { x: c.P.x, y: c.P.y });
                        o.q = qid(a);
                        o.arrive = Some(c.hz);
                        o.tek = Some(1.0);
                        o._own = Some(c.own);
                        o.firm = if c.hz >= D.arriveFirm { Some(true) } else { None };
                        o.spr = if c.spr { Some(1.0) } else { None };
                        o.itK = c.itK;
                        res.out.push(o);
                    }
                }
                if let Some(id) = qid(a) {
                    res.tekH.push((id, secT));
                }
            }
        }
        // ---- şut ----
        let gd = hypot(gx - h.x, GCY - h.y);
        if gd < D.shotRange * (0.75 + 0.5 * match &B._sut { Some(JsDeger::Sayi(v)) => *v, _ => 0.5 }) {
            for ty in self.sutHedef(h, src, gx) {
                for sy in [1.0, -1.0] {
                    let mut o = yeni("gönder");
                    o.shot = Some("güçlü".into());
                    o.to = Some(Nokta { x: gx, y: ty });
                    o.v = Some(self.sutV(h));
                    o.sYon = Some(sy);
                    res.out.push(o);
                    let mut o = yeni("gönder");
                    o.shot = Some("plase".into());
                    o.to = Some(Nokta { x: gx, y: ty });
                    o.v = Some(D.plaseV);
                    o.sYon = Some(sy);
                    res.out.push(o);
                }
            }
        }
        let fx = gx - h.x;
        let fy = GCY - h.y;
        let fn_ = {
            let v = hypot(fx, fy);
            if v == 0.0 || v != v {
                1.0
            } else {
                v
            }
        };
        // ---- sürme ----
        if TOPLA {
            let vH = (0.17 + 0.006 * h.hz()) * if h.sprint { self.sprKafa(h) } else { 1.0 } * self.topK(h) * self.ikDen(h);
            let mut dirs: Vec<Nk> = Vec::new();
            let ek = |dirs: &mut Vec<Nk>, x: f64, y: f64| {
                let l = hypot(x, y);
                if l > 1e-6 && !dirs.iter().any(|u| u.x * x / l + u.y * y / l > 0.97) {
                    dirs.push(nk(x / l, y / l));
                }
            };
            for a in [0.0, 0.7, -0.7, 1.3, -1.3] {
                let c = cos(a);
                let sn = sin(a);
                ek(&mut dirs, fx * c - fy * sn, fx * sn + fy * c);
            }
            for k in 0..10 {
                let a = k as f64 * PI / 5.0;
                ek(&mut dirs, cos(a), sin(a));
            }
            for wy in [0.0, GH] {
                if (h.y - wy).abs() < 10.0 {
                    ek(&mut dirs, 0.0, wy - h.y);
                }
            }
            {
                let (mut nq, mut nd) = (None::<usize>, 1e9);
                for &o in &rak {
                    if src[o].bekci() {
                        continue;
                    }
                    let d = hypot(src[o].x - h.x, src[o].y - h.y);
                    if d < nd {
                        nd = d;
                        nq = Some(o);
                    }
                }
                if let Some(o) = nq {
                    if nd < 12.0 {
                        ek(&mut dirs, h.x - src[o].x, h.y - src[o].y);
                    }
                }
            }
            if hypot(h.vx, h.vy) > 0.08 {
                ek(&mut dirs, h.vx, h.vy);
            }
            let sprs = self.SPRS(h);
            let mut C3: Vec<TekC> = Vec::new();
            for u in &dirs {
                for r in [3.0, 6.0, 10.0, 15.0, 20.0] {
                    let P = nk(h.x + u.x * r, h.y + u.y * r);
                    if P.x < 2.0 || P.x > GW - 2.0 || P.y < 1.8 || P.y > GH - 1.8 {
                        continue;
                    }
                    let tH = r / vH + jmax(0.0, self.donS(h, u.x, u.y));
                    let mut tD = 1e9;
                    for &o in &rak {
                        let o = &src[o];
                        let d = hypot(P.x - o.x, P.y - o.y);
                        tD = jmin(tD, self.varA(o, P.x, P.y, &mut gt) - jmin(d, 1.4) / self.vS(o) + jmax(0.0, self.TEPKI(o, Tepki::Kovala)));
                    }
                    let own = sig((tH - tD) / 5.0);
                    let ownS = if sprs { Some(sig((self.surSure(h, r, Some(*u), true) - tD) / 5.0)) } else { None };
                    C3.push(TekC { P, u: *u, r, own, ownS, s: jmax(own, ownS.unwrap_or(0.0)) * SAH(team, P.x, P.y) });
                }
            }
            C3.sort_by(|a, b| b.s.partial_cmp(&a.s).unwrap_or(std::cmp::Ordering::Equal));
            if crate::iz_acik() && std::env::var("ALAN_C3_IZ").ok().map_or(false, |v| v == format!("{}", self.tick())) {
                eprintln!("  C3 {} vH {} n {}", h.name, vH, C3.len());
                for c in C3.iter().take(12) {
                    eprintln!("    P {:.3} {:.3} r {} own {:.6} ownS {} s {:.6} sah {:.6}", c.P.x, c.P.y, c.r, c.own, c.ownS.map_or("-".into(), |v| format!("{:.6}", v)), c.s, c.s / jmax(c.own, c.ownS.unwrap_or(0.0)));
                }
            }
            let mut secS: Vec<Nk> = Vec::new();
            for c in &C3 {
                if secS.len() >= 6 {
                    break;
                }
                if c.s > 0.0 && !secS.iter().any(|P| hypot(P.x - c.P.x, P.y - c.P.y) < 2.5) {
                    secS.push(c.P);
                    let duvar = c.P.y < 2.6 || c.P.y > GH - 2.6;
                    let sur = |own: f64, spr: bool| {
                        let mut o = yeni("sür");
                        o.to = Some(Nokta { x: c.P.x, y: c.P.y });
                        o.dir = Some(Nokta { x: c.u.x, y: c.u.y });
                        o._own = Some(own);
                        if spr {
                            o.spr = Some(1.0);
                        }
                        o.uzun = if c.r >= 12.0 { Some(c.r) } else { None };
                        o.duvar = duvar;
                        o
                    };
                    res.out.push(sur(c.own, false));
                    if let Some(s) = c.ownS {
                        res.out.push(sur(s, true));
                    }
                }
            }
            res.tekS = secS;
            res.tekC = C3;
        } else {
            for a in if hypot(gx - h.x, GCY - h.y) < 22.0 { &[0.0, 0.7, -0.7, 1.3, -1.3][..] } else { &[0.0, 0.7, -0.7][..] } {
                let ca = cos(*a);
                let sa = sin(*a);
                let dx = (fx * ca - fy * sa) / fn_;
                let dy = (fx * sa + fy * ca) / fn_;
                let to = nk(h.x + 6.0 * dx, h.y + 6.0 * dy);
                if !(to.x < 2.0 || to.x > GW - 2.0 || to.y < 2.0 || to.y > GH - 2.0) {
                    let mut o = yeni("sür");
                    o.to = Some(Nokta { x: to.x, y: to.y });
                    o.dir = Some(Nokta { x: dx, y: dy });
                    res.out.push(o);
                }
            }
            {
                let g = if team == 0.0 { 1.0 } else { -1.0 };
                let (mut nq, mut nd) = (None::<usize>, 1e9);
                for (i, q) in src.iter().enumerate() {
                    if q.team != team && !q.bekci() {
                        let d = hypot(q.x - h.x, q.y - h.y);
                        if d < nd {
                            nd = d;
                            nq = Some(i);
                        }
                    }
                }
                if let Some(qi) = nq {
                    if nd < 7.0 && (src[qi].x - h.x) * g > -1.0 {
                        for wy in [0.0, GH] {
                            let dw = (h.y - wy).abs();
                            if dw > 10.0 || dw < 2.5 {
                                continue;
                            }
                            let to = nk(kis(h.x + 2.5 * g, 2.0, GW - 2.0), if wy == 0.0 { 1.8 } else { GH - 1.8 });
                            let L = {
                                let v = hypot(to.x - h.x, to.y - h.y);
                                if v == 0.0 || v != v {
                                    1.0
                                } else {
                                    v
                                }
                            };
                            let mut o = yeni("sür");
                            o.to = Some(Nokta { x: to.x, y: to.y });
                            o.dir = Some(Nokta { x: (to.x - h.x) / L, y: (to.y - h.y) / L });
                            o.duvar = true;
                            res.out.push(o);
                        }
                    }
                }
            }
            for L in [12.0, 20.0] {
                for a in [0.0, 0.4, -0.4] {
                    let ca = cos(a);
                    let sa = sin(a);
                    let dx = (fx * ca - fy * sa) / fn_;
                    let dy = (fx * sa + fy * ca) / fn_;
                    let to = nk(h.x + dx * L, h.y + dy * L);
                    if to.x < 2.0 || to.x > GW - 2.0 || to.y < 2.0 || to.y > GH - 2.0 {
                        continue;
                    }
                    let engel = src.iter().any(|q| {
                        if q.team == team || q.bekci() {
                            return false;
                        }
                        let t = kis(((q.x - h.x) * dx + (q.y - h.y) * dy) / L, 0.0, 1.0);
                        hypot(h.x + dx * L * t - q.x, h.y + dy * L * t - q.y) < 5.0
                    });
                    if !engel {
                        let mut o = yeni("sür");
                        o.to = Some(Nokta { x: to.x, y: to.y });
                        o.dir = Some(Nokta { x: dx, y: dy });
                        o.uzun = Some(L);
                        res.out.push(o);
                    }
                }
            }
        }
        // ---- önüne paslara şiddet: alıcının koşusuna göre sert pas (üç hız) ----
        {
            let n0 = res.out.len();
            for i in 0..n0 {
                let o = &res.out[i];
                let qi = match o.q {
                    Some(qi) => qi,
                    None => continue,
                };
                if o.kind != "önüne" || o.firm == Some(true) || Some(qi) == hi {
                    continue;
                }
                let q = &src[qi];
                let to = o.to.as_ref().unwrap();
                let vq = (0.17 + 0.006 * q.hz()) * self.dis.ALAN_SPRE(q);
                let dq = hypot(to.x - q.x, to.y - q.y);
                let tR = dq / vq + self.dis.gec(q, to.x, to.y, Some(dq)).unwrap_or(0.0);
                {
                    // TODO(bağ): kafa2 srcC'nin alan önbelleğini (JS src._kc) buraya verebilir
                    let v0 = self.siddetFor(h, nk(to.x, to.y), tR, src, ch, None);
                    if v0 != 0.0 && v0 == v0 {
                        let L = {
                            let v = hypot(to.x - h.x, to.y - h.y);
                            if v == 0.0 || v != v {
                                1.0
                            } else {
                                v
                            }
                        };
                        let (tx, ty) = (to.x, to.y);
                        let base = o.clone();
                        for k in [0.85, 1.0, 1.15] {
                            let vv = jmin(self.vmaxOf(Some(h)), v0 * k);
                            let mut c = base.clone();
                            c.firm = Some(true);
                            c.siddet = Some(to_fixed2(vv));
                            c.lf0 = Some(JOLaunch { vx: (tx - h.x) / L * vv, vy: (ty - h.y) / L * vv, ok: true });
                            res.out.push(c);
                        }
                    }
                }
            }
        }
        // ---- Kuyu'ya hedeflenen sürme/önüne yok ----
        if kuyu_hed {
            let mut i = res.out.len();
            while i > 0 {
                i -= 1;
                let o = &res.out[i];
                if let Some(to) = &o.to {
                    let ilgili = (o.kind == "önüne" && o.q.map(|qi| !src[qi].bekci()).unwrap_or(false)) || (o.kind == "sür" && !h.bekci());
                    if ilgili && !ky && kuyuD(to.x, to.y) > -0.3 {
                        res.out.remove(i);
                    }
                }
            }
        }
        // ---- tut ----
        res.out.push(yeni("tut"));
        {
            let r0 = self.dis.BEKLE_BOZ();
            let mut nd = 1e9;
            if r0.is_some() {
                for q in src {
                    if q.team != team && !q.noTouch {
                        nd = jmin(nd, hypot(q.x - h.x, q.y - h.y));
                    }
                }
            }
            if r0.map(|r| !(r != 0.0 && r == r) || nd >= r).unwrap_or(true) {
                let mut a = yeni("tut");
                a.hold = Some(30.0);
                a.shot = Some("0,5 sn bekle".into());
                let mut b = yeni("tut");
                b.hold = Some(60.0);
                b.shot = Some("1 sn bekle".into());
                res.out.push(a);
                res.out.push(b);
            }
        }
        res
    }
}
