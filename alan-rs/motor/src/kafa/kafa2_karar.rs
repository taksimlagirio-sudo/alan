//! Karar (decide.js 5252–5270, 5302–5423, 7293–8890): decide, decide0, decide0K (adayların tartısı:
//! tut, sür, gönder, pas / önüne / kenardan; kenardan sekme ve kendine sekme adayları; çalım),
//! dalgaKos, decideOT, otFactor, riskOf, realize, kayipYer.
#![allow(non_snake_case, clippy::too_many_arguments)]

pub use super::kafa2_zemin::{kayipYer, riskOf};
use super::kafa2_bag as dis;
use super::kafa2_bag::{ALAN_HZ, ALAN_Y};
use super::kafa2_baglam::*;
use super::kafa2_imkan::tasima;
use super::kafa2_tarti::{kay, prensipDegerle, KayipRef};
use super::kafa2_zemin::{kayipF, sahiplikKur};
use crate::mat::{atan2, cos, dogru, exp, hypot, jmax, jmin, jround, sin, PI};
use crate::model::{Dok, JODet, JOLaunch, Oyuncu, Secenek};
use crate::saha::{CY as GCY, H as GH, KR, W as GW};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

const GW8: f64 = if GW == 100.0 { 99.2 } else { GW - 0.8 };
const GH8: f64 = if GH == 50.0 { 49.2 } else { GH - 0.8 };


/// JS otFactor(p, inSp) (decide.js:5252): tek dokunuşun hata katı (gelen topun hızına göre)
pub fn otFactor(k: &Kafa, p: &Oyuncu, inSp: f64) -> f64 {
    1.0 + k.D.otErr * jmin(1.0, inSp / 0.8) * jmax(0.4, 1.0 - 0.06 * (ALAN_Y(p, "tekDokunusIsabet") - 10.0))
}

/// JS decideOT(h, src, ch, rnd, tac, ot) (decide.js:5255): tek dokunuşla oynanacak en iyi vuruş.
pub fn decideOT(k: &mut Kafa, hi: Pid, src: &Arc<Dunya>, ch: f64, rnd: &mut Zar, tac: Option<&Tac>, ot: f64) -> Option<Secenek> {
    let ot0 = k.bag._ot;
    let td0 = k.bag._tekD;
    k.bag._ot = Some(ot);
    k.bag._tekD = true;
    let r = decide(k, hi, src, ch, rnd, tac);
    let opts: Vec<Secenek> = r.opts.into_iter().filter(|o| o.kind != "sür" && o.kind != "tut" && o.launch.is_some()).collect();
    let s = dis::secim(k, &src[hi], opts, r.T, src);
    // JS finally: D._ot = 0; D._tekD = false (öncekine dönmez)
    let _ = (ot0, td0);
    k.bag._ot = Some(0.0);
    k.bag._tekD = false;
    s
}


/// decide'ın sonucu (JS `{ best, opts, T }`)
#[derive(Clone, Debug, Default)]
pub struct Karar {
    pub best: Option<Secenek>,
    pub opts: Vec<Secenek>,
    pub T: f64,
}

/// JS decide(h, src, ch, rnd, tac) (decide.js:7293): topu tutan h'nin seçenekleri ve en iyisi.
/// `hi`: h'nin dünyadaki sırası; seçeneklerin `q`'su da bu dünyadaki sıra.
pub fn decide(k: &mut Kafa, hi: Pid, src: &Arc<Dunya>, ch: f64, rnd: &mut Zar, tac: Option<&Tac>) -> Karar {
    let tL0 = k.bag._tekL;
    k.bag._tekL = if dis::depoOn() && tac.map(|t| t.kontra).unwrap_or(false) { Some(k.D.kontraL) } else { None };
    let r = decide0(k, hi, src, ch, rnd, tac);
    k.bag._tekL = tL0;
    r
}

/// JS decide0 (decide.js:7388): sahiplik haritası kurulur (kararın süresince SAH_F).
pub fn decide0(k: &mut Kafa, hi: Pid, src: &Arc<Dunya>, ch: f64, rnd: &mut Zar, tac: Option<&Tac>) -> Karar {
    let s0 = k.SAH_F.clone();
    {
        let h = src[hi].clone();
        // ALAN_SAHSIZ_IZ: seçenek üretiminin ızgarası yerel değerle (yayılmasız)
        let f = Arc::new(if crate::kafa::kafa2_tarti::bolge() {
            crate::kafa::kafa2_zemin::sahiplikBolge(k, src, h.team, ch, Some(nk(h.x, h.y)))
        } else if crate::kafa::kafa2_tarti::sahsiz() {
            crate::kafa::kafa2_zemin::sahiplikYerel(k, src, ch)
        } else {
            sahiplikKur(k, src, h.team, ch, Some(nk(h.x, h.y)))
        });
        k.k2.sahF_iz = Some((k.bag._tick, h.team, f.clone()));
        k.SAH_F = Some(f);
    }
    // JS decide0K: srcC = Object.assign(src.slice(), {_kc: new Map}) (kararın K önbelleği)
    let kc0 = k.srcC_kc.replace(std::cell::RefCell::new(crate::alan::KOnbellek::yeni()));
    let r = decide0K(k, hi, src, ch, rnd, tac);
    k.srcC_kc = kc0;
    k.SAH_F = s0;
    r
}

thread_local! {
    static DALGA_DER: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
    pub static PAKET_OLC: std::cell::RefCell<(usize, usize)> = const { std::cell::RefCell::new((0, 0)) };
    /// ölçüm (ALAN_DALGA_IZ): [toplam, dalga sayısı, iş sayısı, 2 işçi, 3 işçi, 4 işçi, 6 işçi süresi]
    pub static DALGA_OLC: std::cell::RefCell<[f64; 7]> = const { std::cell::RefCell::new([0.0; 7]) };
}
fn dalga_olc(s: &[f64]) {
    DALGA_OLC.with(|o| {
        let mut o = o.borrow_mut();
        o[0] += s.iter().sum::<f64>();
        o[1] += 1.0;
        o[2] += s.len() as f64;
        for (j, n) in [2usize, 3, 4, 6].iter().enumerate() {
            // paylaşımlı sayaçla sırayla alma: boşalan işçi sıradakini alır
            let mut w = vec![0.0f64; *n];
            for &t in s {
                let m = (0..*n).min_by(|a, b| w[*a].partial_cmp(&w[*b]).unwrap()).unwrap();
                w[m] += t;
            }
            o[3 + j] += w.iter().cloned().fold(0.0, f64::max);
        }
    });
}
pub fn dalga_olc_yaz() {
    PAKET_OLC.with(|p| {
        let p = p.borrow();
        if p.0 > 0 {
            eprintln!("paket: {} sonuç, ort {} bayt", p.0, p.1 / p.0);
        }
    });
    DALGA_OLC.with(|o| {
        let o = o.borrow();
        if o[1] > 0.0 {
            eprintln!("dalga: toplam {:.2} s, {} dalga, ort {:.1} iş; 2 işçi {:.2} s, 3 işçi {:.2} s, 4 işçi {:.2} s, 6 işçi {:.2} s", o[0], o[1], o[2] / o[1], o[3], o[4], o[5], o[6]);
        }
    });
}

/// dalgaKos'un bir kaydı (JS `{ ek, dir }`)
#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct DalgaR {
    pub ek: Option<Vec<Secenek>>,
    pub dir: Option<(Pid, f64)>,
}

/// D'nin `_` alanlarının anlık görüntüsü (dalgaKos'un geri'si)
struct DAnlik {
    /// geçici bağlam (ayar değişmez, saklanmaz)
    bag: crate::model::KafaBag,
    noHold: bool,
    noFake: bool,
}
fn d_al(k: &Kafa) -> DAnlik {
    DAnlik { bag: k.bag.clone(), noHold: k.k2._noHold, noFake: k.k2._noFake }
}
fn d_geri(k: &mut Kafa, a: &DAnlik) {
    k.bag = a.bag.clone();
    k.k2._noHold = a.noHold;
    k.k2._noFake = a.noFake;
}


fn bul_bekci_y(src: &[Op], team: f64) -> Option<f64> {
    src.iter().find(|p| bekci(p) && p.team != team).map(|p| p.y)
}

/// Kenardan sekme adayının tarifi (bankBir'in `it`'i)
struct BankIt {
    q: Pid,
    T: Nokta,
    tk: &'static str,
    wv: f64,
    ek: &'static str,
}

/// decide0K'nın ortak durumu (JS kapanışın yerelleri)
struct Ctx0<'a> {
    h: Op,
    src: Arc<Dunya>,
    ch: f64,
    team: f64,
    T: f64,
    DOK: bool,
    risk: f64,
    tac: Option<&'a Tac>,
    lossAt: Option<KayipRef>,
    RV: Option<(f64, f64, f64, f64)>,
}

impl<'a> Ctx0<'a> {
    /// JS lossAt(x, y)
    fn loss(&mut self, k: &mut Kafa, x: f64, y: f64) -> f64 {
        match &self.lossAt {
            Some(f) => kay(k, f, x, y),
            None => {
                let src = self.src.clone();
                self.voK(k, &src, x, y) * self.risk
            }
        }
    }
    /// JS VoK(w, x, y)
    fn voK(&mut self, k: &mut Kafa, w: &[Op], x: f64, y: f64) -> f64 {
        let vo = dis::V(k, w, 1.0 - self.team, x, y, dis::chKayip(k, self.ch));
        vo
    }
    /// JS sonDunya(D0, Topt, kind)
    fn sonDunya(&mut self, k: &mut Kafa, D0: Nokta, Topt: f64, kind: &str) -> Arc<Dunya> {
        // yeni kafada (PR) son dünya başlangıç dünyasıdır; eski kafanın cevap dünyası kalktı
        let _ = (k, D0, Topt, kind);
        self.src.clone()
    }
    /// JS sekPay(im, tf)
    fn sekPay(&mut self, k: &mut Kafa, im: &JODet) -> f64 {
        // yeni kafada (PR) sekme payı yok; eski kafanın hesabı kalktı
        let _ = (k, im);
        0.0
    }
}

/// JS `x.toFixed(n)` sonra `+`
fn sabit(x: f64, n: i32) -> f64 {
    sabitle(x, n)
}

/// savurmanın kancası (JS `D._kanca`): (w0, s)
fn kanca_w0(k: &Kafa) -> f64 {
    k.bag._kanca.as_ref().map(|c| c.w0).unwrap_or(0.0)
}
fn kanca_s(k: &Kafa) -> f64 {
    k.bag._kanca.as_ref().map(|c| c.s).unwrap_or(0.0)
}
/// JS `D._kanca && null != D._kanca.u && !D._kanca.on`
fn kanca_u(k: &Kafa) -> Option<f64> {
    match &k.bag._kanca {
        Some(c) if !c.on => c.u,
        _ => None,
    }
}

/// tartG: bir adayı tartar (JS decide0K içinde tartG).
fn tartG(k: &mut Kafa, c: &mut Ctx0, o: &mut Secenek, r: &mut DalgaR, rnd: &mut Zar) {
    let h = c.h.clone();
    let src = c.src.clone();
    let team = c.team;
    let ch = c.ch;
    if o.kind == "tut" {
        let Th = veyo(o.hold, c.T);
        let w = c.sonDunya(k, nk(h.x, h.y), Th, "drive");
        let p1 = dis::keepP(k, &h, &src, None, Th / 2.0);
        let wR = dis::respond(k, &src, team, nk(h.x, h.y), Th / 2.0, "drive");
        let hR: Oyuncu = if dis::ikKafa(k) {
            let mut q: Option<&Op> = None;
            let mut qd = 1e9;
            for o2 in wR.iter() {
                if o2.team != team && !bekci(o2) {
                    let dq = hypot(o2.x - h.x, o2.y - h.y);
                    if dq < qd {
                        qd = dq;
                        q = Some(o2);
                    }
                }
            }
            match q {
                None => (*h).clone(),
                Some(q) => {
                    let mut v = (*h).clone();
                    let w0 = |a: f64| atan2(sin(a), cos(a));
                    let c0 = h.ca.unwrap_or_else(|| dis::coreAng(k, &h));
                    let dd = w0(atan2(h.y - q.y, h.x - q.x) - c0);
                    let om = crate::donus::omega(&h, qd < 4.0, 0.35);
                    v.ca = Some(c0 + crate::mat::jsign(dd) * jmin(dd.abs(), om * Th / 2.0));
                    v
                }
            }
        } else {
            (*h).clone()
        };
        let p2 = dis::keepP(k, &hR, &wR, None, Th / 2.0);
        let P = p1 * p2;
        let tS = dis::sarjTut(k, &h, &src, ch, 1.0, Th);
        let c2 = match tS {
            Some(t) => t.c2,
            None => jmin(1.0, ch + dis::chargeRate(k, &h, &src, 1.0, ch) * Th),
        };
        let vY = dis::V(k, &w, team, h.x, h.y, c2);
        let vE = { vY };
        let LH = c.loss(k, h.x, h.y);
        let kE = {
            1.0
        };
        let bed = tS.map(|t| t.bed).unwrap_or(0.0);
        let v = (if kE < 1.0 { P * (kE * vE - (1.0 - kE) * LH) } else { P * vE }) - (1.0 - P) * LH - bed;
        o.v = Some(v);
        if kE < 1.0 {
        }
        if c.DOK {
            o.dok = Some(Dok::yeni(&[("konum", P * kE * vY), ("acti", P * kE * (vE - vY)), ("risk", -(1.0 - P) * LH - P * (1.0 - kE) * LH), ("enBiz", -bed)]));
        }
        o.det = Some(JODet { P: Some(P), ch: Some(c2), ..Default::default() });
        o._w = Some(w);
        o._T = Some(Th);
        if tS.is_some() {
            o.sjK = Some(sabit(c2 - ch, 3));
        }
    } else if o.kind == "sür" {
        let to = o.to.unwrap();
        let Ts = jmax(6.0, dis::surSure(k, &h, hypot(to.x - h.x, to.y - h.y), o.dir, o.spr));
        // JS keepP(o.spr ? sprKop(h) : h, …): sprintsizde taşıyıcının kendisi (dünyadaki nesne; dolanma onu kimliğiyle atlar)
        let hk: Option<Oyuncu> = if o.spr.map(dogru).unwrap_or(false) { Some(dis::sprKop(k, &h)) } else { None };
        let kpq = crate::iz_acik() && std::env::var("ALAN_KPQ_IZ").is_ok();
        if kpq {
            crate::kafa::kafa1_ikili::KP_KAYIT.with(|c| *c.borrow_mut() = Some(Vec::new()));
        }
        let P = crate::kafa::kafa1_ikili::kp_kal(dis::keepP(k, hk.as_ref().unwrap_or(&*h), &src, o.dir, Ts));
        if kpq {
            let v = crate::kafa::kafa1_ikili::KP_KAYIT.with(|c| c.borrow_mut().take()).unwrap_or_default();
            let gx = if h.team == 0.0 { GW } else { 0.0 };
            if hypot(gx - h.x, GCY - h.y) < 35.0 {
                let q: Vec<String> = v.iter().map(|e| format!("[{:?},{:.2},{:.1},{:.5},{}]", e.0, e.1, e.2, e.3, e.4)).collect();
                eprintln!("KPQ [{},{:?},{:.2},{:.2},[{:.2},{:.2}],{},{:.1},{:.4},[{}]]", k.tick(), &*h.govde.name, h.x, h.y, to.x, to.y, o.spr.map_or(false, dogru), Ts, P, q.join(","));
            }
        }
        let pr = k.keepP_son.get();
        let gec = k.keepP_gec.get();
        let (A, A_to) = {
            let L = hypot(to.x - h.x, to.y - h.y);
            if pr < L {
                (nk(h.x + (to.x - h.x) / veya(L, 1.0) * jmax(0.0, pr), h.y + (to.y - h.y) / veya(L, 1.0) * jmax(0.0, pr)), false)
            } else {
                (to, true)
            }
        };
        let sS = dis::sarjTut(k, &h, &src, ch, 0.0, Ts);
        let c2 = match sS {
            Some(t) => t.c2,
            None => jmin(1.0, ch + dis::chargeRate(k, &h, &src, 0.0, ch) * Ts),
        };
        let w = c.sonDunya(k, A, Ts, "drive");
        if !A_to {
            o.toE = Some(A);
            o.TsE = Some(Ts);
            if gec > 0.0 && crate::kafa::kafa1_ikili::send_gec() {
                o.sendGec = Some(gec);
            }
        }
        let vY = 0.0;
        let Vend = 0.0;
        let vE0 = Vend;
        let kE = 1.0;
        let LA = c.loss(k, A.x, A.y);
        let v = P * Vend - (1.0 - P) * LA;
        o.v = Some(v);
        if c.DOK {
            o.dok = Some(Dok::yeni(&[("konum", P * kE * vY), ("acti", P * kE * (vE0 - vY)), ("risk", v - P * kE * vE0)]));
        }
        o.det = Some(JODet { P: Some(P), ch: Some(c2), ..Default::default() });
        o._w = Some(w);
        o._T = Some(Ts);
    } else if o.kind == "gönder" {
        let to = o.to.unwrap();
        let dx = to.x - h.x;
        let dy = to.y - h.y;
        let L = veya(hypot(dx, dy), 1.0);
        let v0 = o.v.unwrap_or(f64::NAN) * crate::top::chg_mul(ch) * dis::sutGuc(k, &h);
        let mut launch = JOLaunch { vx: dx / L * v0, vy: dy / L * v0, ok: false };
        let ot0 = k.bag._ot;
        let g0 = k.bag._gon;
        let s0 = k.bag._sut.clone();
        let ks = kanca_s(k);
        if o.sYon.map(dogru).unwrap_or(false) && k.bag._kanca.is_some() && dogru(ks) && o.sYon != Some(ks) {
            o.v = None;
            return;
        }
        let syO = if k.bag._kanca.is_some() && dogru(ks) { ks } else { veyo(o.sYon, 0.0) };
        let plase = o.shot.as_deref() == Some("plase");
        {
            let vE = dis::vurE(k, &h, Some(ch), true);
            let ca = dis::caOf(k, &h);
            let w0 = kanca_w0(k);
            let mut sv = match dis::savurPlan(k, &h, launch.vx, launch.vy, ca, w0, syO, &src, plase, vE) {
                Some(s) => s,
                None => {
                    o.v = None;
                    return;
                }
            };
            { let caR = sv.caR;
                let B = crate::core_sabit::Q.body;
                let g = jmax(0.0, veya(sv.T, 0.0));
                let rx = h.x + veya(h.vx, 0.0) * g * 0.5 + cos(caR) * B;
                let ry = h.y + veya(h.vy, 0.0) * g * 0.5 + sin(caR) * B;
                let ex = to.x - rx;
                let ey = to.y - ry;
                let eL = veya(hypot(ex, ey), 1.0);
                let L2 = JOLaunch { vx: ex / eL * v0, vy: ey / eL * v0, ok: false };
                let ca = dis::caOf(k, &h);
                if let Some(s2) = dis::savurPlan(k, &h, L2.vx, L2.vy, ca, kanca_w0(k), syO, &src, plase, vE) {
                    launch = L2;
                    sv = s2;
                }
            }
            if sv.k < 1.0 {
                launch = JOLaunch { vx: launch.vx * sv.k, vy: launch.vy * sv.k, ok: false };
            }
            o.savT = Some(sv.T);
            k.bag._savW = Some(sv.wRel);
            k.bag._savU = Some(sv.uSh);
            k.bag._itme = Some(plase);
            if let Some(u) = kanca_u(k) {
                if sv.T >= 1.0 {
                    let vO = hypot(launch.vx, launch.vy);
                    let mx = u * (1.0 - 0.35 * jmin(1.0, sv.A / PI)) + 0.4;
                    if vO > mx {
                        let L2 = JOLaunch { vx: launch.vx * mx / vO, vy: launch.vy * mx / vO, ok: false };
                        let ca = dis::caOf(k, &h);
                        let s2 = dis::savurPlan(k, &h, L2.vx, L2.vy, ca, kanca_w0(k), ks, &src, plase, vE);
                        launch = match s2 {
                            Some(s2) if s2.k < 1.0 => JOLaunch { vx: L2.vx * s2.k, vy: L2.vy * s2.k, ok: false },
                            _ => L2,
                        };
                        if let Some(s2) = s2 {
                            o.savT = Some(s2.T);
                            k.bag._savW = Some(s2.wRel);
                            k.bag._savU = Some(s2.uSh);
                        }
                    }
                }
            }
        }
        o.launch = Some(launch.clone());
        let shot = o.shot.clone().unwrap_or_default();
        k.bag._ot = Some(veyo(ot0, 1.0) * dis::gonEk(k, &h, &src, &shot));
        k.bag._gon = Some(true);
        k.bag._sut = Some(crate::model::SutBag::Tur(shot));
        k.bag._sYon = Some(syO);
        let tr = { 6.0 };
        let im = dis::imagine(k, &h, &src, dis::Top0 { x: h.x, y: h.y, vx: launch.vx, vy: launch.vy, ch, aim: None }, rnd, tr, None, None);
        k.bag._sYon = Some(0.0);
        k.bag._ot = ot0;
        k.bag._gon = g0;
        k.bag._sut = s0;
        k.bag._savW = None;
        k.bag._savU = None;
        k.bag._itme = None;
        let e = im.endO.or(im.end).unwrap_or_default();
        let rk = -im.Pl.unwrap_or(f64::NAN) * c.loss(k, e.x, e.y);
        let Pk = im.Pk.unwrap_or(f64::NAN);
        o.v = Some(Pk + rk);
        if c.DOK {
            o.dok = Some(Dok::yeni(&[("konum", Pk), ("acti", 0.0), ("risk", rk)]));
        }
        o.det = Some(im);
    } else {
        let to = o.to.unwrap();
        let lf = match &o.lf0 {
            Some(l) => l.clone(),
            None => {
                let arr = o.arrive.unwrap_or(f64::NAN);
                dis::launchFor(k, &h, to, arr, &src, ch)
            }
        };
        if !lf.ok {
            return;
        }
        o.launch = Some(lf.clone());
        let ks = kanca_s(k);
        let kS = if k.bag._kanca.is_some() && dogru(ks) { ks } else { 0.0 };
        let o0 = if dogru(kS) || o.sYon.map(dogru).unwrap_or(false) { None } else { Some(o.clone()) };
        let vE = dis::vurE(k, &h, Some(ch), false);
        let ca = dis::caOf(k, &h);
        let syon = if dogru(kS) { kS } else { veyo(o.sYon, 0.0) };
        let mut sv = match dis::savurPlan(k, &h, lf.vx, lf.vy, ca, kanca_w0(k), syon, &src, false, vE) {
            Some(s) => s,
            None => {
                o.v = None;
                return;
            }
        };
        if let Some(o0) = o0 {
            let sA = dis::savurPlan(k, &h, lf.vx, lf.vy, ca, kanca_w0(k), -sv.s, &src, false, vE);
            if let Some(sA) = sA {
                if sA.T <= sv.T + 1.0 {
                    o.sYon = Some(sv.s);
                    let mut ek = o0;
                    ek.lf0 = Some(lf.clone());
                    ek.sYon = Some(-sv.s);
                    r.ek.get_or_insert_with(Vec::new).push(ek);
                }
            }
        }
        let mut launch = lf.clone();
        if sv.k < 1.0 {
            launch = JOLaunch { vx: lf.vx * sv.k, vy: lf.vy * sv.k, ok: lf.ok };
        }
        o.savT = Some(sv.T);
        k.bag._savT = Some(sv.T);
        k.bag._savW = Some(sv.wRel);
        k.bag._savU = Some(sv.uSh);
        k.bag._savCa = Some(sv.caR);
        if let Some(u) = kanca_u(k) {
            if sv.T >= 1.0 {
                let vO = hypot(launch.vx, launch.vy);
                let mx = u * (1.0 - 0.35 * jmin(1.0, sv.A / PI)) + 0.4;
                if vO > mx {
                    let L2 = JOLaunch { vx: launch.vx * mx / vO, vy: launch.vy * mx / vO, ok: launch.ok };
                    let ca = dis::caOf(k, &h);
                    let s2 = dis::savurPlan(k, &h, L2.vx, L2.vy, ca, kanca_w0(k), ks, &src, false, vE);
                    launch = match s2 {
                        Some(s2) if s2.k < 1.0 => JOLaunch { vx: L2.vx * s2.k, vy: L2.vy * s2.k, ok: L2.ok },
                        _ => L2,
                    };
                    if let Some(s2) = s2 {
                        sv = s2;
                        o.savT = Some(s2.T);
                        k.bag._savT = Some(s2.T);
                        k.bag._savW = Some(s2.wRel);
                        k.bag._savU = Some(s2.uSh);
                        k.bag._savCa = Some(s2.caR);
                    }
                }
            }
        }
        let _ = sv;
        o.launch = Some(launch.clone());
        let onune = o.kind == "önüne";
        let q: Option<Oyuncu> = o.q.map(|i| (*src[i]).clone());
        let tr = { k.D.trials };
        let im = dis::imagine(k, &h, &src, dis::Top0 { x: h.x, y: h.y, vx: launch.vx, vy: launch.vy, ch, aim: Some(to) }, rnd, tr, q.as_ref(), if onune { Some(to) } else { None });
        k.bag._savT = None;
        k.bag._savW = None;
        k.bag._savU = None;
        k.bag._savCa = None;
        let imP = im.P.unwrap_or(f64::NAN);
        let sd = im.Ps.is_some();
        let Pd = if sd { jmax(0.0, imP - im.Ps.unwrap()) } else { imP };
        let eM = if sd { im.endD } else { im.endM };
        let eM_ok = eM.map(|e| e.x.is_finite()).unwrap_or(false);
        let qpos = q.as_ref().map(|q| nk(q.x, q.y));
        let (at, at_to) = if onune {
            if Pd > 1e-9 && eM_ok {
                (eM.unwrap(), false)
            } else if imP >= 0.5 && im.end.map(|e| e.x.is_finite()).unwrap_or(false) {
                (im.end.unwrap(), false)
            } else {
                (to, true)
            }
        } else if o.kind == "pas" && Pd > 1e-9 && eM_ok {
            (eM.unwrap(), false)
        } else {
            (qpos.unwrap_or(to), false)
        };
        let tf = dis::ucusSure(k, &h, at, &launch, &src, ch, o.savT);
        let w = c.sonDunya(k, at, tf, "pass");
        o._sek = Some(if sd { c.sekPay(k, &im) } else { 0.0 });
        let rv = {
            0.0
        };
        let eo = im.endO.or(im.end).unwrap_or_default();
        let Pl = im.Pl.unwrap_or(f64::NAN);
        let Pk = im.Pk.unwrap_or(f64::NAN);
        let lv = c.loss(k, eo.x, eo.y);
        let mut v = Some(Pk + Pd * rv + o._sek.unwrap() - Pl * lv);
        if c.DOK {
            if let Some((kA, vY, vM, RL)) = c.RV {
                let rk = -Pl * lv;
                let ac0 = Pd * kA * (vM - vY);
                o.dok = Some(Dok::yeni(&[("konum", v.unwrap() - ac0 + Pd * RL - rk), ("acti", ac0), ("risk", rk - Pd * RL)]));
            }
        }
        if onune && !at_to && imP > 1e-9 {
            let lvn = veya(hypot(launch.vx, launch.vy), 1.0);
            let ileri = ((at.x - to.x) * launch.vx + (at.y - to.y) * launch.vy) / lvn;
            if ileri > k.D.asanPay {
                v = None;
            }
        }
        o.v = v;
        if o.v.is_some() && onune && !at_to && Pd > 1e-9 {
            let tf0 = dis::ucusSure(k, &h, to, &launch, &src, ch, o.savT);
            let ek = jmax(0.0, tf - tf0);
            if ek > 0.0 {
                let tempo = match c.tac {
                    Some(t) => t.tempo.unwrap_or(0.5),
                    None => 0.5,
                };
                let zx = Pd * tempo * k.D.bekTempo * (ek / 30.0);
                o.v = Some(o.v.unwrap() - zx);
                dis::dokEk(o, "zaman", -zx);
            }
        }
        o.det = Some(im);
        o.tf = Some(tf);
        o._at = Some(at);
        o._w = Some(w);
        if o.kind == "pas" {
            r.dir = Some((o.q.unwrap(), Pd));
        }
    }
}

/// JS kancaK(lf): kenardan vuruşun savurma planı (yoksa vuruş yok)
fn kancaK(k: &mut Kafa, h: &Oyuncu, src: &[Op], ch: f64, lf: JOLaunch) -> Option<JOLaunch> {
    let kc = kanca_u(k);
    let ca = dis::caOf(k, h);
    let vE = dis::vurE(k, h, Some(ch), false);
    let pl = dis::savurPlan(k, h, lf.vx, lf.vy, ca, kanca_w0(k), kanca_s(k), src, false, vE)?;
    let u = match kc {
        Some(u) if pl.T >= 1.0 => u,
        _ => return Some(lf),
    };
    let vO = hypot(lf.vx, lf.vy);
    let mx = u * (1.0 - 0.35 * jmin(1.0, pl.A / PI)) + 0.4;
    Some(if vO > mx { JOLaunch { vx: lf.vx * mx / vO, vy: lf.vy * mx / vO, ok: lf.ok } } else { lf })
}

/// JS segD(p, A, B): noktanın AB parçasına uzaklığı
fn segD(p: Nokta, A: Nokta, B: Nokta) -> f64 {
    let ax = B.x - A.x;
    let ay = B.y - A.y;
    let L2 = veya(ax * ax + ay * ay, 1.0);
    let u = jmax(0.0, jmin(1.0, ((p.x - A.x) * ax + (p.y - A.y) * ay) / L2));
    hypot(A.x + ax * u - p.x, A.y + ay * u - p.y)
}

/// Kenardan sekme adayının hedefleri (JS bankHedef)
fn bankHedef(ad: &dis::Secenekler, src: &[Op], team: f64, qi: Pid) -> Vec<(Nokta, &'static str, f64, &'static str)> {
    let q = &src[qi];
    let duvarlar: [(f64, &'static str); 3] = [(0.0, "y"), (GH, "y"), (if t0(team) { GW } else { 0.0 }, "x")];
    if let Some(tH) = ad.tekH.iter().find(|x| x.0 == qi).map(|x| &x.1) {
        if !tH.is_empty() {
            let mut A = Vec::new();
            for P in tH {
                let tk = if hypot(P.x - q.x, P.y - q.y) < 0.5 { "ayak" } else { "nokta" };
                for (wv, ek) in duvarlar {
                    A.push((*P, tk, wv, ek));
                }
            }
            return A;
        }
    }
    let mut hedefler: Vec<(Nokta, &'static str)> = vec![(nk(q.x, q.y), "ayak")];
    if hypot(q.tx - q.x, q.ty - q.y) > 3.0 {
        hedefler.push((nk(jmax(1.0, jmin(GW - 1.0, q.tx)), jmax(1.0, jmin(GH - 1.0, q.ty))), "koşu"));
    }
    let gx0 = if t0(team) { GW } else { 0.0 };
    let dq = hypot(gx0 - q.x, GCY - q.y);
    if dq < 40.0 && dq > 6.0 {
        for kk in [4.0, 8.0, 12.0] {
            if kk >= dq - 2.0 {
                break;
            }
            hedefler.push((nk(q.x + (gx0 - q.x) / dq * kk, q.y + (GCY - q.y) / dq * kk), "orta"));
        }
    }
    let mut A = Vec::new();
    for (T, tk) in hedefler {
        for (wv, ek) in duvarlar {
            A.push((T, tk, wv, ek));
        }
    }
    A
}

/// JS decide0K(h, src, ch, rnd, tac) (decide.js:7402): adayları kurar, budar, tartar; kenardan ve
/// çalım adaylarını ekler; enerji muhasebesi; prensip tartısı. Sonuç değere göre sıralı.
pub fn decide0K(k: &mut Kafa, hi: Pid, src: &Arc<Dunya>, ch: f64, rnd: &mut Zar, tac: Option<&Tac>) -> Karar {
    k.bag._sut = Some(crate::model::SutBag::Egilim(tac.and_then(|t| t.sut).unwrap_or(0.5)));
    let h = src[hi].clone();
    let team = h.team;
    let mut ad = dis::options(k, &h, src, ch);
    let risk = riskOf(k, &h, tac);
    let htempo = h.govde.tl.as_ref().and_then(|t| t.huc.get("tempo").copied()).unwrap_or(0.5);
    let T = jround(10.0 + 14.0 * (1.0 - jmax(0.0, jmin(1.0, tac.and_then(|t| t.tempo).unwrap_or(0.5) + 0.6 * (htempo - 0.5)))) + 3.0);
    let DOK = !k.k2._noHold && !k.k2._noFake;
    let mut c = Ctx0 {
        h: h.clone(),
        src: src.clone(),
        ch,
        team,
        T,
        DOK,
        risk,
        tac,
        lossAt: None,
        RV: None,
    };
    {
        let mut kf = kayipF(k, src.clone(), team, ch, risk);
        kf.karar_icin(k);
        c.lossAt = Some(Rc::new(RefCell::new(kf)));
    }
    let mut direct: Vec<(Pid, f64)> = Vec::new();
    let dirYaz = |direct: &mut Vec<(Pid, f64)>, q: Pid, v: f64| match direct.iter_mut().find(|x| x.0 == q) {
        Some(x) => x.1 = jmax(x.1, v),
        None => direct.push((q, v)),
    };
    // budama
    {
        let nP = k.D.nFull;
        let nS = 4.0;
        let opts = &mut ad.out;
        let rough = |k: &mut Kafa, o: &mut Secenek| -> f64 {
            let at = if o.kind == "önüne" || o.tek.map(dogru).unwrap_or(false) { o.to.unwrap() } else { o.q.map(|i| nk(src[i].x, src[i].y)).or(o.to).unwrap() };
            if o.kind == "gönder" {
                return dis::laneOk(k, src, team, nk(h.x, h.y), at, None, None) * dis::shotQ(k, src, team, h.x, h.y, ch) * if o.shot.as_deref() == Some("plase") { 1.0 } else { 1.02 };
            }
            {
                let z = match if crate::kafa::kafa2_tarti::sahsiz() { &None } else { &k.SAH_F } {
                    Some(f) => f.oku(team, at.x, at.y),
                    None => {
                        let q = o.q.map(|i| (*src[i]).clone());
                        tasima(k, src, team, q.as_ref(), at.x, at.y, ch)
                    }
                };
                return dis::laneOk(k, src, team, nk(h.x, h.y), at, Some(k.D.arriveFirm), None) * o._own.unwrap_or(1.0) * jmax(0.002, z);
            }
        };
        let mut pl: Vec<(usize, f64)> = Vec::new();
        for i in 0..opts.len() {
            if opts[i].q.is_some() {
                let r = rough(k, &mut opts[i]);
                pl.push((i, r));
            }
        }
        pl.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        let bky = bul_bekci_y(src, team);
        let mut sh: Vec<(usize, f64)> = Vec::new();
        for i in 0..opts.len() {
            if opts[i].kind == "gönder" {
                let r = rough(k, &mut opts[i]);
                let s = if bky.map(|y| y > GCY).unwrap_or(false) { -1.0 } else { 1.0 };
                sh.push((i, r + (opts[i].to.unwrap().y - GCY) * s * 0.001));
            }
        }
        sh.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        let mut keep: Vec<usize> = Vec::new();
        let mut seenQ: Vec<String> = Vec::new();
        {
            for &(i, _) in &pl {
                let o = &opts[i];
                let kq = format!("{}:{}{}{}", src[o.q.unwrap()].id, o.kind, if o.firm.unwrap_or(false) { "+" } else { "" }, if o.arrive.map(|a| a > k.D.arriveFirm).unwrap_or(false) { "!" } else { "" });
                if !seenQ.contains(&kq) {
                    seenQ.push(kq);
                    keep.push(i);
                }
            }
        }
        for &(i, _) in &pl {
            if keep.len() as f64 >= jmax(nP, seenQ.len() as f64) {
                break;
            }
            if !keep.contains(&i) {
                keep.push(i);
            }
        }
        {
            let mut hy: Vec<u64> = Vec::new();
            let mut on = Vec::new();
            let mut ar = Vec::new();
            for x in sh.iter() {
                let y = opts[x.0].to.unwrap().y.to_bits();
                if hy.contains(&y) {
                    ar.push(*x);
                } else {
                    hy.push(y);
                    on.push(*x);
                }
            }
            sh = on;
            sh.extend(ar);
        }
        let gk = |o: &Secenek| format!("{}:{}", o.shot.clone().unwrap_or_else(|| "undefined".into()), super::kafa2_tarti::to_fixed(o.to.unwrap().y, 2));
        let mut kG: Vec<String> = Vec::new();
        for &(i, _) in &sh {
            if kG.len() as f64 >= nS {
                break;
            }
            let g = gk(&opts[i]);
            if !kG.contains(&g) {
                kG.push(g);
            }
        }
        {
            if let Some(&(i, _)) = sh.iter().find(|x| opts[x.0].shot.as_deref() == Some("plase")) {
                let g = gk(&opts[i]);
                if !kG.contains(&g) {
                    kG.push(g);
                }
            }
        }
        let mut drop: Vec<usize> = pl.iter().filter(|x| !keep.contains(&x.0)).map(|x| x.0).collect();
        drop.extend(sh.iter().filter(|x| !kG.contains(&gk(&opts[x.0]))).map(|x| x.0));
        let mut i = opts.len();
        while i > 0 {
            i -= 1;
            if drop.contains(&i) {
                opts.remove(i);
            }
        }
    }
    // tartı dalgaları
    let SPKv = dis::SPK(k);
    {
        let mut tart = |k: &mut Kafa, o: &mut Secenek, r: &mut DalgaR, rnd: &mut Zar| {
            let nY = SPKv && o.q == Some(hi);
            if nY {
                k.k2.sfNiyet = Some((h.id, h.team, if o.spr.map(dogru).unwrap_or(false) { "sprint".into() } else { "yok".into() }, o.itK));
            }
            let n0 = if nY { k.dis.niyet_koy(Some((h.id, h.team, o.spr.map(dogru).unwrap_or(false), o.itK))) } else { None };
            tartG(k, &mut c, o, r, rnd);
            if nY {
                k.k2.sfNiyet = None;
                k.dis.niyet_koy(n0);
            }
        };
        let mut bekle: Option<Vec<(usize, Secenek)>> = Some(Vec::new());
        let mut dalga = |k: &mut Kafa, opts: &mut Vec<Secenek>, bekle: &mut Option<Vec<(usize, Secenek)>>, direct: &mut Vec<(Pid, f64)>, a0: usize, rnd: &mut Zar| {
            let mut a = a0;
            while a < opts.len() {
                let b = opts.len();
                let kay = {
                    let items = &mut opts[a..b];
                    let snap = d_al(k);
                    let mut kay: Vec<DalgaR> = (0..items.len()).map(|_| DalgaR::default()).collect();
                    // sıra sınaması (ALAN_TERS_IZ): işler ters sırayla tartılır, sonuç sırası aynı
                    let ters = crate::iz_acik() && std::env::var("ALAN_TERS_IZ").is_ok();
                    let n = items.len();
                    let olc = crate::iz_acik() && std::env::var("ALAN_DALGA_IZ").is_ok() && DALGA_DER.with(|d| d.get()) == 0;
                    DALGA_DER.with(|d| d.set(d.get() + 1));
                    let mut sure = Vec::new();
                    let pk = crate::iz_acik() && std::env::var("ALAN_PAKET_IZ").is_ok() && DALGA_DER.with(|d| d.get()) == 1;
                    if pk {
                        let mp: Vec<Op> = k.mac.as_ref().map(|m| m.ps.clone()).unwrap_or_default();
                        crate::paket::taban_kur([src], mp.iter());
                    }
                    let bol = !ters && !pk && crate::paralel::hazir(n);
                    if bol {
                        let mp: Vec<Op> = k.mac.as_ref().map(|m| m.ps.clone()).unwrap_or_default();
                        crate::paket::taban_kur([src], mp.iter());
                    }
                    let mut bir = |i: usize, pb: Option<&[u8]>| -> Option<Vec<u8>> {
                        if let Some(pb) = pb {
                            let (o2, r2): (Secenek, DalgaR) = crate::paket::coz(pb);
                            items[i] = o2;
                            kay[i] = r2;
                            return None;
                        }
                        d_geri(k, &snap);
                        let mut r = DalgaR::default();
                        tart(k, &mut items[i], &mut r, rnd);
                        d_geri(k, &snap);
                        kay[i] = r;
                        if bol {
                            return Some(crate::paket::kodla(&(&items[i], &kay[i])));
                        }
                        if pk {
                            // paket sınaması: sonuç paketlenip geri açılır (başka işçiden gelmiş gibi)
                            let b = crate::paket::kodla(&(&items[i], &kay[i]));
                            let (o2, r2): (Secenek, DalgaR) = crate::paket::coz(&b);
                            items[i] = o2;
                            kay[i] = r2;
                            PAKET_OLC.with(|p| {
                                let mut p = p.borrow_mut();
                                p.0 += 1;
                                p.1 += b.len();
                            });
                        }
                        None
                    };
                    if !(bol && crate::paralel::dagit(n, &mut bir)) {
                        for j in 0..n {
                            let i = if ters { n - 1 - j } else { j };
                            // ölçüm yalnız bilgisayarda (wasm'da zaman ölçümü yok, Instant panikler)
                            let t0 = if olc { Some(std::time::Instant::now()) } else { None };
                            bir(i, None);
                            if let Some(t0) = t0 {
                                sure.push(t0.elapsed().as_secs_f64());
                            }
                        }
                    }
                    if bol {
                        crate::paket::taban_bos();
                    }
                    DALGA_DER.with(|d| d.set(d.get() - 1));
                    if olc {
                        dalga_olc(&sure);
                    }
                    kay
                };
                for (i, r) in kay.into_iter().enumerate() {
                    if let Some(ek) = r.ek {
                        for x in ek {
                            match bekle.as_mut() {
                                Some(bk) => bk.push((a + i, x)),
                                None => opts.push(x),
                            }
                        }
                    }
                    if let Some((q, v)) = r.dir {
                        dirYaz(direct, q, v);
                    }
                }
                a = b;
            }
        };
        dalga(k, &mut ad.out, &mut bekle, &mut direct, 0, rnd);
        if let Some(B) = bekle.take() {
            let mut b1 = f64::NEG_INFINITY;
            for o in &ad.out {
                if let Some(v) = o.v {
                    if v > b1 {
                        b1 = v;
                    }
                }
            }
            let n0 = ad.out.len();
            let Bq = &crate::core_sabit::Q;
            let yak = |p: &Secenek| -> bool {
                let t = veyo(p.savT, 0.0) + 2.0;
                src.iter().any(|q| q.team != team && !q.noTouch && hypot(q.x - h.x, q.y - h.y) < Bq.body + Bq.reach + (0.17 + 0.006 * ALAN_HZ(q)) * dis::ALAN_SPRE(k, q) * t)
            };
            for (pi, x) in B {
                let p = &ad.out[pi];
                if p.v.is_none() || p.v.unwrap() >= b1 - k.D.yokD || yak(p) {
                    ad.out.push(x);
                }
            }
            dalga(k, &mut ad.out, &mut bekle, &mut direct, n0, rnd);
        }
    }
    let basR = {
        let mut s = 0.0;
        for q in src.iter() {
            if q.team == team || bekci(q) {
                continue;
            }
            let dx = h.x - q.x;
            let dy = h.y - q.y;
            let L = veya(hypot(dx, dy), 1.0);
            let b = 0.17 + 0.006 * ALAN_HZ(q);
            s += jmax(0.0, jmin(1.0, (veya(q.vx, 0.0) * dx + veya(q.vy, 0.0) * dy) / L / b));
        }
        jmin(1.0, s / 2.0)
    };
    if SPKv {
        let n = ad.out.len();
        for i in 0..n {
            let o = &ad.out[i];
            if !o.spr.map(dogru).unwrap_or(false) || o.v.is_none() {
                continue;
            }
            let tw = (0..n).find(|&j| {
                let y = &ad.out[j];
                j != i && !y.spr.map(dogru).unwrap_or(false) && y.kind == o.kind && y.q == o.q && y.to.is_some() && o.to.is_some() && hypot(y.to.unwrap().x - o.to.unwrap().x, y.to.unwrap().y - o.to.unwrap().y) < 0.01 && y.v.is_some()
            });
            if let Some(j) = tw {
                let twv = ad.out[j].v.unwrap();
                if !(ad.out[i].v.unwrap() > twv) {
                    ad.out[i].sprBos = Some(1.0);
                    ad.out[i].v = Some(twv - 1e-4);
                }
            }
        }
    }
    // enerji muhasebesi
    {
        for i in 0..ad.out.len() {
            let w = match &ad.out[i]._w {
                Some(w) => w.clone(),
                None => continue,
            };
            let o = &ad.out[i];
            let Tt = match o._T {
                Some(t) if dogru(t) => t,
                _ => match o.to {
                    Some(to) => dis::ucusT(k, hypot(to.x - h.x, to.y - h.y), o.arrive.unwrap_or(k.D.arrive), None) + 4.0,
                    None => 0.0,
                },
            };
            if Tt > 0.0 {
                let bit = if o.kind == "tut" { None } else { o.toE.or(o.to) };
                let en = dis::enMuhasebe(k, &h, src, &w, Tt, bit, &ad.out[i]);
                let o = &mut ad.out[i];
                o.en = Some(en);
                let wY = veyo(tac.and_then(|t| t.enYipratma), 0.0);
                let kT = k.D.enUfuk / jmax(k.D.enTmin, Tt);
                if dogru(wY) && o.v.is_some() {
                    let e = o.en.as_ref().unwrap();
                    let ek = kT * k.D.enHizV * (0.0 * e.hizBiz - wY * (e.hizRakipD + basR * e.hizRakipY));
                    o.v = Some(o.v.unwrap() + ek);
                    o.en.as_mut().unwrap().ek = sabit(ek, 4);
                    let eb = kT * k.D.enHizV * (0.0 * o.en.as_ref().unwrap().hizBiz);
                    dis::dokEk(o, "enBiz", eb);
                    dis::dokEk(o, "enRakip", ek - eb);
                    o._en = Some(veyo(o._en, 0.0) + ek);
                }
            }
        }
    }
    for o in ad.out.iter_mut() {
        if o._w.is_some() && o.q.is_some() {
        }
        o._w = None;
        o._at = None;
        o._T = None;
    }
    // kenardan sekme (arkadaşa)
    let bankQ: Vec<(Pid, f64)> = if dis::kenarAkil(k, &h) {
        let mut v: Vec<(Pid, f64, f64)> = Vec::new();
        for qi in 0..src.len() {
            let q = &src[qi];
            if q.team != team || qi == hi || bekci(q) || hypot(q.x - h.x, q.y - h.y) < 10.0 {
                continue;
            }
            let Pd = match direct.iter().find(|x| x.0 == qi) {
                Some(x) => x.1,
                None => dis::laneOk(k, src, team, nk(h.x, h.y), nk(q.x, q.y), None, None),
            };
            if Pd < k.D.bankIf {
                v.push((qi, Pd, dis::threat(k, team, q.x, q.y)));
            }
        }
        v.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
        v.into_iter().take(k.D.bankN as usize).map(|x| (x.0, x.1)).collect()
    } else {
        Vec::new()
    };
    {
        let mut items: Vec<BankIt> = Vec::new();
        for (q, _) in &bankQ {
            for (T, tk, wv, ek) in bankHedef(&ad, src, team, *q) {
                items.push(BankIt { q: *q, T, tk, wv, ek });
            }
        }
        let mut yeni: Vec<Secenek> = Vec::new();
        let snap = d_al(k);
        for it in &items {
            d_geri(k, &snap);
            if let Some(o) = bankBir(k, &mut c, it, rnd) {
                yeni.push(o);
            }
            d_geri(k, &snap);
        }
        ad.out.extend(yeni);
    }
    // kenardan kendine
    {
        let g = if t0(team) { 1.0 } else { -1.0 };
        let opp: Vec<Op> = src.iter().filter(|q| q.team != team && !bekci(q)).cloned().collect();
        let yakv: Vec<&Op> = opp.iter().filter(|q| (q.x - h.x) * g > -1.0 && hypot(q.x - h.x, q.y - h.y) < 12.0).collect();
        if !yakv.is_empty() {
            let hp = nk(h.x, h.y);
            for wy in [0.0, GH] {
                let dw = (h.y - wy).abs();
                if dw > 9.0 {
                    continue;
                }
                let Ts: Vec<Nokta> = match &ad.tekS {
                    Some(v) => v.iter().filter(|P| (P.y - wy).abs() < (h.y - wy).abs() + 6.0).cloned().collect(),
                    None => {
                        let mut v = Vec::new();
                        for L in [7.0, 10.0, 13.0] {
                            for off in [0.0, 2.5, 5.0] {
                                v.push(nk(jmax(2.0, jmin(GW - 2.0, h.x + g * L)), if wy == 0.0 { jmax(1.5, jmin(GH - 1.5, h.y + off)) } else { jmin(GH - 1.5, jmax(1.5, h.y - off)) }));
                            }
                        }
                        v
                    }
                };
                for T in Ts {
                    let tk = k.tick();
                    if opp.iter().any(|q| segD(nk(q.x, q.y), hp, T) < 2.0 || (q.hal.adT.is_some() && veyo(q.hal.adandi, 0.0) > tk && segD(nk(q.hal.adT.as_ref().unwrap().x, q.hal.adT.as_ref().unwrap().y), hp, T) < 2.0)) {
                        continue;
                    }
                    let Ty = if wy == 0.0 { -T.y } else { 2.0 * GH - T.y };
                    let tt = (wy - h.y) / (Ty - h.y);
                    if tt <= 0.1 || tt >= 0.9 {
                        continue;
                    }
                    let B = nk(h.x + (T.x - h.x) * tt, if wy == 0.0 { 0.8 } else { GH8 });
                    if opp.iter().any(|q| segD(nk(q.x, q.y), hp, B) < 1.3 || segD(nk(q.x, q.y), B, T) < 1.3) {
                        continue;
                    }
                    let lf0 = dis::bankFor(k, &h, B, T, 0.6 * k.D.arrive, src, ch);
                    if !lf0.ok {
                        continue;
                    }
                    let lf = match kancaK(k, &h, src, ch, lf0) {
                        Some(l) => l,
                        None => continue,
                    };
                    let im = dis::kenarIm(k, &h, src, dis::Top0 { x: h.x, y: h.y, vx: lf.vx, vy: lf.vy, ch, aim: Some(B) }, rnd, k.D.trials, Some(&h), Some(T));
                    let tf = dis::bankSure(k, &h, B, T, &lf, src, ch, None);
                    let imP = im.P.unwrap_or(f64::NAN);
                    let PdK = if im.Ps.is_some() { jmax(0.0, imP - im.Ps.unwrap()) } else { imP };
                    let rv = {
                        0.0
                    };
                    let sk = c.sekPay(k, &im);
                    let eo = im.endO.or(im.end).unwrap_or_default();
                    let lv = c.loss(k, eo.x, eo.y);
                    let Pl = im.Pl.unwrap_or(f64::NAN);
                    let v = im.Pk.unwrap_or(f64::NAN) + PdK * rv + sk - Pl * lv;
                    let mut oK = Secenek { kind: "kenardan".into(), kendine: true, to: Some(B), lead: Some(T), q: Some(hi), launch: Some(lf), det: Some(im), tf: Some(tf), v: Some(v), ..Default::default() };
                    if c.DOK {
                        if let Some((kA, vY, vM, RL)) = c.RV {
                            let rk = -Pl * lv;
                            let ac0 = PdK * kA * (vM - vY);
                            oK.dok = Some(Dok::yeni(&[("konum", v - ac0 + PdK * RL - rk), ("acti", ac0), ("risk", rk - PdK * RL)]));
                        }
                    }
                    ad.out.push(oK);
                }
            }
        }
    }
    // çalım (topu rakibin yanından duvara vurup arkasında almak)
    calim(k, &mut c, &mut ad, src, hi, rnd);
    if bekci(&h) && h.hal.bekBas.is_some() {
        let kal = 150.0 - (k.tick() - h.hal.bekBas.unwrap());
        for o in ad.out.iter_mut() {
            if o.kind != "tut" && o.kind != "fake" {
                continue;
            }
            let dt = if o.kind == "fake" { 15.0 } else { veyo(o.hold, 15.0) };
            if dt >= kal - 4.0 {
                let v = -c.loss(k, h.x, h.y);
                o.v = Some(v);
                if o.dok.is_some() {
                    o.dok = Some(Dok::yeni(&[("risk", v)]));
                }
            }
        }
    }
    // eski kafa: taktik eğilimleri, yerleşme, fake
    for o in ad.out.iter_mut() {
        o._en = None;
        if DOK {
            if let Some(v) = o.v {
                match o.dok.as_mut() {
                    Some(d) => {
                        let mut t = 0.0;
                        for x in &d.k {
                            t += x.1;
                        }
                        if (v - t).abs() > 1e-9 {
                            let dg = d.al("diger").map(|x| veya(x, 0.0)).unwrap_or(0.0) + (v - t);
                            d.koy("diger", dg);
                        }
                    }
                    None => {
                        o.dok = Some(Dok::yeni(&[(if o.kind == "fake" { "acti" } else { "konum" }, v)]));
                    }
                }
            }
        }
    }
    {
        let la = c.lossAt.clone().unwrap();
        prensipDegerle(k, hi, src, ch, team, &mut ad.out, &la, T, tac);
    }
    let mut ok: Vec<Secenek> = ad.out.into_iter().filter(|o| o.v.is_some()).collect();
    ok.sort_by(|a, b| b.v.unwrap().partial_cmp(&a.v.unwrap()).unwrap_or(std::cmp::Ordering::Equal));
    Karar { best: ok.first().cloned(), opts: ok, T }
}

/// JS bankBir(it, r): bir kenardan sekme adayı (arkadaşa)
fn bankBir(k: &mut Kafa, c: &mut Ctx0, it: &BankIt, rnd: &mut Zar) -> Option<Secenek> {
    let h = c.h.clone();
    let src = c.src.clone();
    let ch = c.ch;
    let q = src[it.q].clone();
    let T = it.T;
    let B;
    if it.ek == "y" {
        let wy = it.wv;
        let qy2 = if wy == 0.0 { -T.y } else { 2.0 * GH - T.y };
        let tt = (wy - h.y) / (qy2 - h.y);
        if tt <= 0.15 || tt >= 0.85 {
            return None;
        }
        B = nk(h.x + (T.x - h.x) * tt, if wy == 0.0 { 0.8 } else { GH8 });
    } else {
        let wx = it.wv;
        let qx2 = if wx == 0.0 { -T.x } else { 2.0 * GW - T.x };
        let tt = (wx - h.x) / (qx2 - h.x);
        if tt <= 0.15 || tt >= 0.85 {
            return None;
        }
        B = nk(if wx == 0.0 { 0.8 } else { GW8 }, h.y + (T.y - h.y) * tt);
        if (B.y - GCY).abs() < KR + 2.0 || B.y < 1.0 || B.y > GH - 1.0 || hypot(B.x - h.x, B.y - h.y) + hypot(T.x - B.x, T.y - B.y) > 60.0 {
            return None;
        }
    }
    let hedef = if it.tk == "ayak" { nk(q.x, q.y) } else { T };
    let lf0 = dis::bankFor(k, &h, B, hedef, k.D.arrive, &src, ch);
    if !lf0.ok {
        return None;
    }
    let lf = kancaK(k, &h, &src, ch, lf0)?;
    let im = dis::kenarIm(k, &h, &src, dis::Top0 { x: h.x, y: h.y, vx: lf.vx, vy: lf.vy, ch, aim: Some(B) }, rnd, k.D.trials, Some(&q), None);
    let tf = dis::bankSure(k, &h, B, nk(q.x, q.y), &lf, &src, ch, None);
    let imP = im.P.unwrap_or(f64::NAN);
    let sd = im.Ps.is_some();
    let Pd = if sd { jmax(0.0, imP - im.Ps.unwrap()) } else { imP };
    let rv = {
        0.0
    };
    let sk = if sd { c.sekPay(k, &im) } else { 0.0 };
    let eo = im.endO.or(im.end).unwrap_or_default();
    let lv = c.loss(k, eo.x, eo.y);
    let Pl = im.Pl.unwrap_or(f64::NAN);
    let v = Pd * rv + sk - Pl * lv;
    let mut o = Secenek { kind: "kenardan".into(), to: Some(B), q: Some(it.q), launch: Some(lf), det: Some(im), kh: Some(it.tk.into()), v: Some(v), ..Default::default() };
    if c.DOK {
        if let Some((kA, vY, vM, RL)) = c.RV {
            let rk = -Pl * lv;
            let ac0 = Pd * kA * (vM - vY);
            o.dok = Some(Dok::yeni(&[("konum", v - ac0 + Pd * RL - rk), ("acti", ac0), ("risk", rk - Pd * RL)]));
        }
    }
    o.tf = Some(tf);
    Some(o)
}

/// Çalım adayları (JS decide0K'da `nq && nd < 6` bloğu): topu en yakın rakibin yanından duvara
/// vurup arkasında yeniden almak.
fn calim(k: &mut Kafa, c: &mut Ctx0, ad: &mut dis::Secenekler, src: &Arc<Dunya>, hi: Pid, _rnd: &mut Zar) {
    let h = c.h.clone();
    let team = c.team;
    let ch = c.ch;
    let mut nq: Option<Op> = None;
    let mut nd = 1e9;
    let g = if t0(team) { 1.0 } else { -1.0 };
    for q in src.iter() {
        if q.team != team && !bekci(q) {
            let dd = hypot(q.x - h.x, q.y - h.y);
            if dd < nd {
                nd = dd;
                nq = Some(q.clone());
            }
        }
    }
    let hc = {
        let (ux, uy, _) = dis::coreSide(k, &h);
        let B = crate::core_sabit::Q.body;
        let mut o = (*h).clone();
        o.x = h.x + ux * B;
        o.y = h.y + uy * B;
        o
    };
    let nq = match nq {
        Some(q) if nd < 6.0 && (q.x - h.x) * g > -1.0 => q,
        _ => return,
    };
    let tk0 = k.tick();
    let adK = |q: &Oyuncu| jmax(0.0, veyo(q.hal.adandi, 0.0) - tk0);
    let qP = |q: &Oyuncu| -> Nokta {
        if adK(q) > 0.0 {
            if let Some(a) = &q.hal.adT {
                return nk(a.x, a.y);
            }
        }
        nk(q.x, q.y)
    };
    let opp: Vec<Op> = src.iter().filter(|q| q.team != team && !bekci(q)).cloned().collect();
    for wy in [0.0, GH] {
        let adaylar: Vec<(f64, f64, Option<Nokta>)> = match &ad.tekC {
            Some(tc) => {
                let mut v: Vec<(Nokta, f64)> = tc
                    .iter()
                    .filter(|P| (P.x - nq.x) * g > 1.0 && hypot(P.x - nq.x, P.y - nq.y) < 9.0 && (P.y - wy).abs() < (nq.y - wy).abs() + 4.0)
                    .map(|P| (*P, k.SAH_F.as_ref().map(|f| f.oku(team, P.x, P.y)).unwrap_or(0.0)))
                    .collect();
                v.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
                v.into_iter().take(6).map(|x| (0.0, 0.0, Some(x.0))).collect()
            }
            None => [(3.3, 2.8), (5.0, 2.8), (5.0, 1.2), (7.0, 2.8), (7.0, 1.2), (4.0, 4.0)].iter().map(|&(a, b)| (a, b, None)).collect(),
        };
        for (fwd, lat, TP) in adaylar {
            for ar in [0.7, 0.1] {
                let dw = (h.y - wy).abs();
                let dq = (nq.y - wy).abs();
                if dw > 15.0 || dq > 18.0 {
                    continue;
                }
                let T = match TP {
                    Some(p) => p,
                    None => nk(jmax(2.0, jmin(GW - 2.0, nq.x + g * fwd)), if wy == 0.0 { jmin(GH - 2.0, nq.y + lat) } else { jmax(2.0, nq.y - lat) }),
                };
                let Ty = if wy == 0.0 { -T.y } else { 2.0 * GH - T.y };
                let tt = (wy - hc.y) / (Ty - hc.y);
                if tt <= 0.1 || tt >= 0.9 {
                    continue;
                }
                let B = nk(hc.x + (T.x - hc.x) * tt, if wy == 0.0 { 0.8 } else { GH8 });
                let A = nk(hc.x, hc.y);
                let yolGovde = |q: &Oyuncu| -> f64 {
                    let kal = jmax(0.0, veyo(q.hal.adandi, 0.0) - tk0);
                    let mut mn: f64 = 1e9;
                    for kk in 0..=8 {
                        let u = kk as f64 / 8.0;
                        let x = A.x + (B.x - A.x) * u;
                        let y = A.y + (B.y - A.y) * u;
                        let tb = hypot(x - A.x, y - A.y) / 0.9;
                        let f = if kal > 0.0 && q.hal.adT.is_some() { jmin(1.0, tb / kal) } else { 0.0 };
                        let (ax, ay) = match &q.hal.adT {
                            Some(a) => (a.x, a.y),
                            None => (q.x, q.y),
                        };
                        let qx = q.x + (ax - q.x) * f;
                        let qy = q.y + (ay - q.y) * f;
                        mn = jmin(mn, hypot(qx - x, qy - y));
                    }
                    mn
                };
                if opp.iter().any(|q| yolGovde(q) < 1.35) {
                    continue;
                }
                let lf = dis::bankFor(k, &hc, B, T, k.D.arrive * ar, src, ch);
                if !lf.ok {
                    continue;
                }
                let sy = if wy == 0.0 { 1.0 } else { -1.0 };
                let W1 = nk(nq.x - 0.6 * g, nq.y + 2.2 * sy);
                let Wp = nk(nq.x + 1.0 * g, nq.y + 2.2 * sy);
                let vH = (0.17 + 0.006 * ALAN_HZ(&h)) * dis::ALAN_SPRE(k, &h);
                let dW = hypot(W1.x - h.x, W1.y - h.y) + hypot(Wp.x - W1.x, Wp.y - W1.y);
                let pts = {
                    let taban = dis::vurTaban(&h);
                    let b0 = crate::top::Top::yeni(hc.x, hc.y, lf.vx, lf.vy, team as u8, Some(ch), taban);
                    let et: Vec<crate::alan::Etki> = src.iter().map(|p| p.etki()).collect();
                    crate::top::tahmin(&b0, &mut crate::alan::Kaynak::Sec(&et), &crate::saha::P, Some(160)).0
                };
                {
                    let mut carp = false;
                    let n = jmin(18.0, pts.len() as f64) as usize;
                    'dis: for kk in 1..n {
                        for q in &opp {
                            let kal = jmax(0.0, veyo(q.hal.adandi, 0.0) - tk0);
                            let (qx, qy);
                            if kal > 0.0 && q.hal.adT.is_some() {
                                let a = q.hal.adT.as_ref().unwrap();
                                let f = jmin(1.0, kk as f64 / kal);
                                qx = q.x + (a.x - q.x) * f;
                                qy = q.y + (a.y - q.y) * f;
                            } else {
                                let tr = jmin(kk as f64, 0.6 * jmax(0.0, dis::ALAN_TEPKI(k, q, "kovala")));
                                qx = q.x + veya(q.vx, 0.0) * tr;
                                qy = q.y + veya(q.vy, 0.0) * tr;
                            }
                            if hypot(pts[kk].x - qx, pts[kk].y - qy) < 1.35 {
                                carp = true;
                                break 'dis;
                            }
                        }
                    }
                    if carp {
                        continue;
                    }
                }
                let mut cut = pts.len();
                for j in 2..pts.len() {
                    let pj = pts[j];
                    let mut tq: f64 = 1e9;
                    for q in &opp {
                        let qp = qP(q);
                        let qd = veya(hypot(pj.x - qp.x, pj.y - qp.y), 1.0);
                        tq = jmin(tq, jmax(adK(q), 0.6 * jmax(0.0, dis::ALAN_TEPKI(k, q, "kovala"))) + dis::donS(k, q, (pj.x - qp.x) / qd, (pj.y - qp.y) / qd) + jmax(0.0, hypot(qp.x - pj.x, qp.y - pj.y) - 1.4) / ((0.17 + 0.006 * ALAN_HZ(q)) * dis::ALAN_SPRE(k, q)));
                    }
                    if tq < j as f64 {
                        cut = j;
                        break;
                    }
                }
                let dS = |k: &Kafa, p: &Oyuncu, x: f64, y: f64| {
                    let L = veya(hypot(x - p.x, y - p.y), 1.0);
                    dis::donS(k, p, (x - p.x) / L, (y - p.y) / L)
                };
                let kapiG = {
                    let tHW = dS(k, &h, W1.x, W1.y) + dW / (0.82 * vH);
                    let kq = qP(&nq);
                    let tKW = jmax(adK(&nq), 0.6 * jmax(0.0, dis::ALAN_TEPKI(k, &nq, "kovala"))) + dS(k, &nq, Wp.x, Wp.y) + jmax(0.0, hypot(Wp.x - kq.x, Wp.y - kq.y) - 1.7) / ((0.17 + 0.006 * ALAN_HZ(&nq)) * dis::ALAN_SPRE(k, &nq));
                    if tKW < tHW {
                        tHW - tKW + 8.0
                    } else {
                        0.0
                    }
                };
                let mut best: Option<(f64, f64, Nokta, f64)> = None;
                let mut i = 6;
                while i < cut {
                    let pt = pts[i];
                    let l1 = veya(hypot(W1.x - h.x, W1.y - h.y), 1.0);
                    let tH = jmax(i as f64, dis::donS(k, &h, (W1.x - h.x) / l1, (W1.y - h.y) / l1) + (dW + hypot(pt.x - Wp.x, pt.y - Wp.y)) / (0.82 * vH));
                    let tHv = tH + kapiG;
                    if tHv > i as f64 + 3.0 {
                        i += 3;
                        continue;
                    }
                    let mut tD: f64 = 1e9;
                    for q in &opp {
                        let ql = veya(hypot(pt.x - q.x, pt.y - q.y), 1.0);
                        let back = dis::donS(k, q, (pt.x - q.x) / ql, (pt.y - q.y) / ql);
                        let qp = qP(q);
                        tD = jmin(tD, jmax(adK(q), jmax(0.0, dis::ALAN_TEPKI(k, q, "kovala"))) + back + jmax(0.0, hypot(qp.x - pt.x, qp.y - pt.y) - 1.4) / ((0.17 + 0.006 * ALAN_HZ(q)) * dis::ALAN_SPRE(k, q)));
                    }
                    let pw = 1.0 / (1.0 + exp((tHv - tD) / 5.0)) * (1.0 - 0.3 * jmax(0.0, 1.0 - (ALAN_Y(&h, "kenarCalim") - 6.0) / 10.0));
                    let vV = dis::V(k, src, team, pt.x, pt.y, 0.5 * ch);
                    let v = pw * vV - (1.0 - pw) * c.loss(k, pt.x, pt.y);
                    if best.map(|b| v > b.0).unwrap_or(true) {
                        best = Some((v, pw, nk(pt.x, pt.y), pw * vV));
                    }
                    i += 3;
                }
                if let Some((v, pw, pt, kon)) = best {
                    let mut oC = Secenek { kind: "çalım".into(), to: Some(pt), wp: Some(Wp), wp1: Some(W1), q: Some(hi), launch: Some(lf), v: Some(v), det: Some(JODet { P: Some(pw), ..Default::default() }), ..Default::default() };
                    if c.DOK {
                        oC.dok = Some(Dok::yeni(&[("konum", kon), ("risk", v - kon)]));
                    }
                    ad.out.push(oC);
                }
            }
        }
    }
}

