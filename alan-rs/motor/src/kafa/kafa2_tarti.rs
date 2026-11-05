//! Prensip tartısı (decide.js 6747–6772, 6811–7292): sürme değeri (surD), Bekçi'nin pas yolu (bekYol),
//! tartının dünyaları (pDunya), alıcının penceresi (pencere), en iyi pas (enIyiPas), düğüm değeri
//! (nodeV), durum (durumD), imkânlı zemin (zeminH), prensipDegerle / prensipDegerle0, prGam.
#![allow(non_snake_case, clippy::too_many_arguments)]

pub use super::kafa2_zemin::bekYol;
use super::kafa2_bag as dis;
use super::kafa2_bag::{ALAN_HZ};
use super::kafa2_baglam::*;
use super::kafa2_dunya::{bekYerW, geriCekil, predictWorld};
use super::kafa2_imkan::{imkUzak, imkan, sutS, sutVakit, SUR_BOS, SUR_PARCA};
use super::kafa2_zemin::{gam, zeminS, KayipF};
use crate::mat::{cos, exp, hypot, jmax, jmin, jround};
use crate::model::{Dok, Oyuncu, Secenek};
use crate::saha::{CY as GCY, H as GH, W as GW};
use std::cell::RefCell;

use std::rc::Rc;
use std::sync::Arc;

pub const PNC_DILIM: f64 = 12.0;
pub const PNC_UFUK: f64 = 120.0;
pub const PNC_GORUS: f64 = 48.0;

pub type KayipRef = Rc<RefCell<KayipF>>;
/// JS `lossAt(x, y)` / `kayip(x, y)`
#[inline]
pub fn kay(k: &mut Kafa, f: &KayipRef, x: f64, y: f64) -> f64 {
    f.borrow_mut().deger(k, x, y)
}


/// Prensip tartısının bağlamı (JS cx: { src, team, ch, kayip, h, P, IM })
pub struct Cx {
    pub src: Arc<Dunya>,
    pub team: f64,
    pub ch: f64,
    pub kayip: KayipRef,
    pub h: Op,
    /// pDunya önbelleği ("w|…")
    pub PW: crate::HizliMap<String, Arc<Dunya>>,
    /// pencere önbelleği
    pub PE: crate::HizliMap<String, Arc<Pencere>>,
    /// nodeV önbelleği ("v|…")
    pub PV: crate::HizliMap<String, f64>,
    /// durumD önbelleği ("d|…")
    pub PD: crate::HizliMap<String, Arc<DurumR>>,
    /// zeminH'nin imkân önbelleği (JS cx.IM; cxSiz kopyaları varsa paylaşır)
    pub IM: Option<Rc<RefCell<crate::HizliMap<String, f64>>>>,
}

/// JS pencere'nin girdisi
#[derive(Debug)]
pub struct Pencere {
    pub k: Op,
    pub B: Nokta,
    pub W: Arc<Dunya>,
    pub tb: f64,
    pub kA: f64,
    pub sut: f64,
    pub zem: f64,
    pub sur: f64,
    pub b: f64,
}

/// JS durumD'nin sonucu
#[derive(Debug, Clone)]
pub struct DurumR {
    pub v: f64,
    pub sut: f64,
    pub zem: f64,
    pub pas: f64,
    pub sur: f64,
    pub pasK: Option<String>,
}

/// JS sayısının yazısı (tam sayılar ".0"sız)
pub fn js_sayi(x: f64) -> String {
    if x == x.trunc() && x.abs() < 1e15 {
        format!("{}", x as i64)
    } else {
        format!("{}", x)
    }
}
/// JS `x.toFixed(n)`
pub fn to_fixed(x: f64, n: usize) -> String {
    format!("{:.*}", n, x)
}

/// JS pDunya(cx, B, T, kind, q) (decide.js:6932): top B'deyken T tik sonraki dünya (en yakın cevap
/// veren tek rakip cevabıyla, ötekiler öngörüyle; alıcı B'de).
pub fn pDunya(k: &mut Kafa, cx: &mut Cx, B: Nokta, T: f64, kind: &str, q: Option<&Oyuncu>) -> Arc<Dunya> {
    let key = format!("w|{}|{}|{}|{}|{}", kind, to_fixed(B.x, 2), to_fixed(B.y, 2), js_sayi(jround(T)), q.map(|q| js_sayi(q.id)).unwrap_or_default());
    if let Some(w) = cx.PW.get(&key) {
        return w.clone();
    }
    let src = cx.src.clone();
    let team = cx.team;
    let W: Arc<Dunya> = if T > 0.0 {
        let Rw = dis::respond(k, &src, team, B, T, kind);
        let resp: Vec<usize> = (0..src.len()).filter(|&i| !ayni(&Rw[i], &src[i])).collect();
        let tas = if kind == "drive" { q.map(|q| nk(q.x, q.y)) } else { None };
        let Pw = predictWorld(k, &src, team, B, T, true, Some(&resp), false, tas);
        let mut tekC: isize = -1;
        let mut bd = 1e9;
        for &i in &resp {
            let d = hypot(Rw[i].x - B.x, Rw[i].y - B.y);
            if d < bd {
                bd = d;
                tekC = i as isize;
            }
        }
        Arc::new(
            (0..src.len())
                .map(|i| {
                    let p = &src[i];
                    if !ayni(&Rw[i], p) && (tekC < 0 || i as isize == tekC) {
                        let mut c = (*Rw[i]).clone();
                        c.vx = 0.0;
                        c.vy = 0.0;
                        return Arc::new(c);
                    }
                    match q {
                        Some(q) if p.id == q.id && p.team == q.team => kxyv(p, B.x, B.y, 0.0, 0.0),
                        _ => Pw[i].clone(),
                    }
                })
                .collect(),
        )
    } else {
        match q {
            Some(q) => Arc::new(src.iter().map(|p| if p.id == q.id && p.team == q.team { kxyv(p, B.x, B.y, 0.0, 0.0) } else { p.clone() }).collect()),
            None => src.clone(),
        }
    };
    cx.PW.insert(key, W.clone());
    W
}

/// JS zeminH(w, team, x, y, ch, k, cx, T) (decide.js:6748): zemin ile imkânın büyüğü (imkân T tik
/// sonrası için; cx varsa önbellekli).
pub fn zeminH(k: &mut Kafa, w: &[Op], team: f64, x: f64, y: f64, ch: f64, kq: Option<&Oyuncu>, cx: Option<&mut Cx>, T: Option<f64>) -> f64 {
    if imkUzak(team, x, y) {
        return zeminS(k, w, team, x, y, ch, kq);
    }
    let cx = match cx {
        None => {
            let a = imkan(k, w, team, nk(x, y), kq, ch);
            let b = zeminS(k, w, team, x, y, ch, kq);
            return jmax(a, b);
        }
        Some(c) => c,
    };
    let M = cx.IM.get_or_insert_with(|| Rc::new(RefCell::new(Default::default()))).clone();
    let key = format!(
        "{}|{}|{}|{}|{}",
        js_sayi(team),
        kq.map(|q| js_sayi(q.id)).unwrap_or_else(|| "-".into()),
        js_sayi(jround(x / 3.0)),
        js_sayi(jround(y / 3.0)),
        js_sayi(jround(veyo(T, 0.0) / PNC_DILIM))
    );
    if let Some(v) = M.borrow().get(&key) {
        return *v;
    }
    let o0 = k.k2.IMK_OFS;
    k.k2.IMK_OFS = veyo(T, 0.0);
    let v = imkan(k, w, team, nk(x, y), kq, ch);
    k.k2.IMK_OFS = o0;
    let v = jmax(v, zeminS(k, w, team, x, y, ch, kq));
    M.borrow_mut().insert(key, v);
    v
}

/// JS surD(w, team, k, at, ch, kayip, cx, T0) (decide.js:6811): at'tan kaleye ya da ileri sürmenin
/// değeri (8 br'lik parçalarla; her parçada tutma olasılığı, geri çekilen savunma; sonunda şut ya da pas).
pub fn surD(k: &mut Kafa, w: &[Op], team: f64, kq: &Oyuncu, at: Nokta, ch: f64, kayip: &KayipRef, mut cx: Option<&mut Cx>, T0: Option<f64>) -> f64 {
    let gx = if t0(team) { GW } else { 0.0 };
    let dx = gx - at.x;
    let dy = GCY - at.y;
    let dg = veya(hypot(dx, dy), 1.0);
    let ug = nk(dx / dg, dy / dg);
    let uf = nk(if t0(team) { 1.0 } else { -1.0 }, 0.0);
    let mut dirs = vec![(ug, true)];
    let vK = (0.17 + 0.006 * ALAN_HZ(kq)) * dis::topK(k, kq);
    if ug.x * uf.x < cos(0.35) {
        dirs.push((uf, false));
    }
    let mut best = -1e9;
    let T0v = veyo(T0, 0.0);
    for (u, gol) in dirs {
        let mut Lm = SUR_BOS;
        Lm = if gol { jmin(Lm, dg - 1.5) } else { jmin(Lm, if t0(team) { GW - 2.0 - at.x } else { at.x - 2.0 }) };
        if Lm < 2.0 {
            continue;
        }
        let mut pos = at;
        let mut ww: Dunya = w.to_vec();
        let mut Pc = 1.0;
        let mut T = 0.0;
        let mut yol = 0.0;
        let mut kayv = 0.0;
        while yol < Lm - 0.5 && Pc > 0.05 {
            let seg = jmin(SUR_PARCA, Lm - yol);
            let mut r = kq.clone();
            r.x = pos.x;
            r.y = pos.y;
            r.vx = if yol > 0.0 { u.x * vK } else { 0.0 };
            r.vy = if yol > 0.0 { u.y * vK } else { 0.0 };
            let Ts0 = jmax(4.0, dis::surSure(k, &r, seg, Some(u), None));
            let p = jmax(0.0, jmin(1.0, dis::keepP(k, &r, &ww, Some(u), Ts0)));
            let pr = k.keepP_son.get();
            let adv = if pr < seg { jmax(0.0, pr) } else { seg };
            kayv += Pc * (1.0 - p) * kay(k, kayip, pos.x + u.x * adv / 2.0, pos.y + u.y * adv / 2.0);
            Pc *= p;
            if adv < 1.0 {
                break;
            }
            let E = nk(pos.x + u.x * adv, pos.y + u.y * adv);
            let Ts = if adv < seg { jmax(4.0, dis::surSure(k, &r, adv, Some(u), None)) } else { Ts0 };
            ww = geriCekil(k, &ww, team, pos, E, Ts, yol == 0.0);
            T += Ts;
            yol += adv;
            pos = E;
            if adv < seg - 0.5 {
                break;
            }
        }
        if yol < 1.5 {
            continue;
        }
        let wE = bekYerW(k, &ww, team, pos, None);
        let sE = sutS(k, &wE, team, pos.x, pos.y, ch, Some(kq), Some(T0v + T)) * sutVakit(k, &wE, team, pos.x, pos.y, Some(kq));
        let sonra = match cx.as_deref_mut() {
            Some(c) => gam(k) * enIyiPas(k, c, &wE, pos, T0v + T, kq.id, 0.0).0,
            None => 0.0,
        };
        let v = Pc * jmax(jmax(sE, 0.0), sonra) - kayv;
        if v > best {
            best = v;
        }
    }
    if best > -1e9 {
        best
    } else {
        0.0
    }
}

/// JS pencere(cx, k, T, R) (decide.js:6987): alıcı k topu T tik sonra (R'de ya da öngörülen yerinde)
/// alırsa: dünya, tutma olasılığı, şut / zemin / sürme değeri.
pub fn pencere(k: &mut Kafa, cx: &mut Cx, kq: &Op, T: f64, R: Option<Nokta>) -> Arc<Pencere> {
    let tb = jmin(PNC_UFUK, jmax(0.0, jround(T / PNC_DILIM) * PNC_DILIM));
    let key = format!("{}|{}{}", js_sayi(kq.id), js_sayi(tb), R.map(|r| format!("|{}|{}", js_sayi(jround(2.0 * r.x)), js_sayi(jround(2.0 * r.y)))).unwrap_or_default());
    if let Some(e) = cx.PE.get(&key) {
        return e.clone();
    }
    let team = cx.team;
    let ch = cx.ch;
    let B = match R {
        Some(r) => r,
        None => {
            if tb == 0.0 || ayni(kq, &cx.h) {
                nk(kq.x, kq.y)
            } else {
                let src = cx.src.clone();
                let h = cx.h.clone();
                let pw = predictWorld(k, &src, team, nk(h.x, h.y), tb, false, None, false, None);
                match bul_i(&pw, kq.id, kq.team) {
                    Some(i) => nk(pw[i].x, pw[i].y),
                    None => nk(kq.x, kq.y),
                }
            }
        }
    };
    let pw = pDunya(k, cx, B, tb, "pass", Some(kq));
    let W = Arc::new(bekYerW(k, &pw, team, B, None));
    let mut r = (**kq).clone();
    r.x = B.x;
    r.y = B.y;
    r.vx = 0.0;
    r.vy = 0.0;
    let kA = jmax(0.0, jmin(1.0, dis::keepP(k, &r, &W, None, k.D.recvKeepT)));
    let sut = if bekci(kq) { 0.0 } else { sutS(k, &W, team, B.x, B.y, ch, Some(kq), Some(tb)) * sutVakit(k, &W, team, B.x, B.y, Some(kq)) };
    let zem = zeminH(k, &W, team, B.x, B.y, ch, Some(&r), Some(cx), Some(tb));
    let sur = if bekci(kq) {
        0.0
    } else {
        let kayip = cx.kayip.clone();
        gam(k) * surD(k, &W, team, &r, B, ch, &kayip, None, None)
    };
    let b = jmax(jmax(sut, gam(k) * zem), sur);
    let e = Arc::new(Pencere { k: kq.clone(), B, W, tb, kA, sut, zem, sur, b });
    cx.PE.insert(key, e.clone());
    e
}

/// JS enIyiPas(cx, w, A, T, haric, der) (decide.js:7021): A'dan (T tik sonra) en iyi pasın değeri ve
/// penceresi; der > 0 ise alıcıdan bir pas daha.
pub fn enIyiPas(k: &mut Kafa, cx: &mut Cx, w: &[Op], A: Nokta, T: f64, haric: f64, der: f64) -> (f64, Option<Arc<Pencere>>) {
    let mut best = 0.0;
    let mut bk: Option<Arc<Pencere>> = None;
    let src = cx.src.clone();
    for kq in src.iter() {
        if kq.team != cx.team || kq.noTouch || kq.id == haric {
            continue;
        }
        let k0: Op = match bul_i(w, kq.id, kq.team) {
            Some(i) => w[i].clone(),
            None => kq.clone(),
        };
        let L0 = hypot(k0.x - A.x, k0.y - A.y);
        if L0 < 5.0 || L0 > dis::pasMax(k) {
            continue;
        }
        let tf = dis::ucusT(k, L0, k.D.arriveFirm, None);
        if T + tf > PNC_GORUS {
            continue;
        }
        let P0 = pencere(k, cx, kq, T + tf, None);
        let mut P = P0.clone();
        let mut tR = tf;
        let mut R: Option<Nokta> = None;
        {
            let Lb = hypot(P0.B.x - A.x, P0.B.y - A.y);
            if Lb >= 5.0 && Lb <= dis::pasMax(k) {
                let rb = veya(dis::AS_S_rcvBuf(k), 0.0);
                let vk = (0.17 + 0.006 * ALAN_HZ(&k0)) * dis::ALAN_SPRE(k, &k0);
                let ux = (P0.B.x - A.x) / Lb;
                let uy = (P0.B.y - A.y) / Lb;
                let mut s = 5.0;
                while s < Lb {
                    let x = A.x + ux * s;
                    let y = A.y + uy * s;
                    let d = jmax(0.0, hypot(x - k0.x, y - k0.y) - 1.4);
                    let tk = dis::AS_gec(k, &k0, x, y, d) + d / vk + rb;
                    let tbb = dis::ucusT(k, Lb, k.D.arriveFirm, Some(s));
                    if tk <= tbb {
                        R = Some(nk(x, y));
                        tR = tbb;
                        break;
                    }
                    s += 1.0;
                }
                if R.is_some() {
                    if T + tR > PNC_GORUS {
                        continue;
                    }
                    P = pencere(k, cx, kq, T + tR, R);
                }
            }
        }
        let L = hypot(P.B.x - A.x, P.B.y - A.y);
        if L < 5.0 || L > dis::pasMax(k) {
            continue;
        }
        let lag = k.D.sutHazir;
        let Lo = dis::laneOk(k, w, cx.team, A, P.B, Some(k.D.arriveFirm), Some(lag)) * bekYol(k, w, cx.team, A, P.B, Some(lag));
        if Lo < 0.05 {
            continue;
        }
        let nv = nodeV(k, cx, kq, T + tR, der, R);
        let kayip = cx.kayip.clone();
        let e = Lo * nv - (1.0 - Lo) * kay(k, &kayip, (A.x + P.B.x) / 2.0, (A.y + P.B.y) / 2.0);
        if e > best {
            best = e;
            bk = Some(P);
        }
    }
    (best, bk)
}

/// JS nodeV(cx, k, T, der, R) (decide.js:7077): alıcı k'nin T tik sonraki değeri (tutma × en iyisi
/// (şut, zemin, sürme, sonraki pas) × güven − kayıp).
pub fn nodeV(k: &mut Kafa, cx: &mut Cx, kq: &Op, T: f64, der: f64, R: Option<Nokta>) -> f64 {
    let e = pencere(k, cx, kq, T, R);
    let key = format!("v|{}|{}|{}{}", js_sayi(kq.id), js_sayi(e.tb), js_sayi(der), R.map(|r| format!("|{}|{}", js_sayi(jround(2.0 * r.x)), js_sayi(jround(2.0 * r.y)))).unwrap_or_default());
    if let Some(v) = cx.PV.get(&key) {
        return *v;
    }
    let pas = if der > 0.0 { gam(k) * enIyiPas(k, cx, &e.W, e.B, e.tb, kq.id, der - 1.0).0 } else { 0.0 };
    let kayip = cx.kayip.clone();
    let v = e.kA * jmax(e.b, pas) * exp(-jmax(0.0, veya(T, 0.0)) / k.k2.IMK_INAN_T) - (1.0 - e.kA) * kay(k, &kayip, e.B.x, e.B.y);
    cx.PV.insert(key, v);
    v
}

/// JS durumD(cx, w, q, at, T0) (decide.js:7086): q topu at'ta (T0 tik sonra) alırsa durumu.
pub fn durumD(k: &mut Kafa, cx: &mut Cx, w: &[Op], q: &Oyuncu, at: Nokta, T0: f64) -> Arc<DurumR> {
    let key = format!("d|{}|{}|{}|{}", js_sayi(q.id), js_sayi(jround(2.0 * at.x)), js_sayi(jround(2.0 * at.y)), js_sayi(jround(T0 / PNC_DILIM)));
    if let Some(r) = cx.PD.get(&key) {
        return r.clone();
    }
    let team = cx.team;
    let ch = cx.ch;
    let kayip = cx.kayip.clone();
    let wb = bekYerW(k, w, team, at, None);
    let mut r = q.clone();
    r.x = at.x;
    r.y = at.y;
    r.vx = 0.0;
    r.vy = 0.0;
    let T0v = veya(T0, 0.0);
    let sut = sutS(k, &wb, team, at.x, at.y, ch, Some(q), Some(T0v)) * sutVakit(k, &wb, team, at.x, at.y, Some(q));
    let z0 = zeminH(k, &wb, team, at.x, at.y, ch, Some(&r), Some(cx), Some(T0));
    let sur = gam(k) * surD(k, &wb, team, &r, at, ch, &kayip, Some(cx), Some(T0));
    let ip = enIyiPas(k, cx, w, at, T0, q.id, 1.0);
    let pas = gam(k) * ip.0;
    let v0 = jmax(jmax(jmax(sut, gam(k) * z0), pas), sur);
    let v = v0 * exp(-jmax(0.0, T0v) / k.k2.IMK_INAN_T);
    let pasK = ip.1.as_ref().map(|P| format!("{} ({},{}) +{}t", P.k.name, to_fixed(P.B.x, 0), to_fixed(P.B.y, 0), js_sayi(P.tb)));
    let res = Arc::new(DurumR { v, sut, zem: z0, pas, sur, pasK });
    cx.PD.insert(key, res.clone());
    res
}

/// JS prGam(h, tac) (decide.js:7122): tempo / sabır / kontraya göre sonraki adımın indirimi
pub fn prGam(k: &Kafa, h: &Oyuncu, tac: Option<&Tac>) -> f64 {
    let tempo = tac.and_then(|t| t.tempo).unwrap_or(0.5);
    let htempo = h.tl.as_ref().and_then(|t| t.huc.get("tempo").copied()).unwrap_or(0.5);
    let sabir = tac.and_then(|t| t.sabir).unwrap_or(0.5);
    let kontra = tac.map(|t| t.kontra).unwrap_or(false);
    let tp = tempo + 0.6 * (htempo - 0.5) + (0.5 - sabir) + if kontra { 0.2 } else { 0.0 };
    jmax(0.8, jmin(0.96, k.D.combo + 0.12 * (0.5 - jmax(0.0, jmin(1.0, tp)))))
}

/// JS prensipDegerle(h, src, ch, team, opts, lossAt, T, tac) (decide.js:7109)
pub fn prensipDegerle(k: &mut Kafa, hi: Pid, src: &Arc<Dunya>, ch: f64, team: f64, opts: &mut Vec<Secenek>, lossAt: &KayipRef, T: f64, tac: Option<&Tac>) {
    let g0 = k.bag._gPr;
    let w0 = k.bag._enWD;
    let t0_ = k.bag._enTm;
    k.bag._gPr = Some(prGam(k, &src[hi], tac));
    k.bag._enWD = Some(veyo(tac.and_then(|t| t.enDinlen), 0.0));
    k.bag._enTm = Some(team);
    prensipDegerle0(k, hi, src, ch, team, opts, lossAt, T);
    k.bag._gPr = g0;
    k.bag._enWD = w0;
    k.bag._enTm = t0_;
}

/// JS enGec(q, w, f): q'nun barı dünyadakine çekilerek f (tartının bağlamında q da değişir).
fn en_gec<R>(k: &mut Kafa, cx: &mut Cx, q: &Op, w: &[Op], f: impl FnOnce(&mut Kafa, &mut Cx, &Op) -> R) -> R {
    match dis::enGec(k, q, w) {
        None => f(k, cx, q),
        Some(e) => {
            let mut c = (**q).clone();
            c.en = Some(e);
            let q2 = Arc::new(c);
            let eski_src = cx.src.clone();
            let eski_h = cx.h.clone();
            if let Some(i) = cx.src.iter().position(|z| ayni(z, q)) {
                let mut s = (*cx.src).clone();
                s[i] = q2.clone();
                cx.src = Arc::new(s);
            }
            if ayni(&cx.h, q) {
                cx.h = q2.clone();
            }
            let r = f(k, cx, &q2);
            cx.src = eski_src;
            cx.h = eski_h;
            r
        }
    }
}

fn dok_dk(r: &DurumR, o: &mut Secenek) {
    let d = o.dok.get_or_insert_with(Dok::default);
    d.koy("sut", r.sut);
    d.koy("ileri", r.pas);
    d.koy("surme", r.sur);
    d.koy("zemin", r.zem);
    if let Some(p) = &r.pasK {
        d.sonra = Some(p.clone());
    }
}

const PR_BOL: bool = false;
/// Hız (yalnız Rust, JS'te yok): "bir rakip olmasa" tablosunda arkadaşından UZAK_R'den uzak rakip atlanır
/// (çıkarılmasının o arkadaşın değerine etkisi ölçüldü, bkz. ALAN_UZAK_IZ). Eskisi: ALAN_UZAK_ESKI_IZ.
const UZAK_R: f64 = 20.0;
#[allow(non_upper_case_globals)]
static UZAK_KIS_: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
fn uzak_kis() -> bool {
    *UZAK_KIS_.get_or_init(|| !(crate::iz_acik() && std::env::var("ALAN_UZAK_ESKI_IZ").is_ok()))
}
/// "Bir rakip olmasa" dünyaları fırsat önbelleğini (IM) paylaşırdı; eskisi: ALAN_CXIM_ESKI_IZ.
fn cxim_eski() -> bool {
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *A.get_or_init(|| crate::iz_acik() && std::env::var("ALAN_CXIM_ESKI_IZ").is_ok())
}
fn uzak_h() -> bool {
    crate::iz_acik() && std::env::var("ALAN_UZAKH_IZ").is_ok()
}
fn uzak_iz() -> bool {
    crate::iz_acik() && std::env::var("ALAN_UZAK_IZ").is_ok()
}

/// JS prensipDegerle0 (decide.js:7126): seçenekleri prensiple tartar (Vur, İlerlet, Güvene al, Çevir,
/// Alana sür, Çalım, Kalkan, Bekle); sonra arkadaşların ve taşıyanın değerini m._prT'ye yazar.
pub fn prensipDegerle0(k: &mut Kafa, hi: Pid, src: &Arc<Dunya>, ch: f64, team: f64, opts: &mut Vec<Secenek>, lossAt: &KayipRef, T: f64) {
    let h = src[hi].clone();
    let dir = if t0(team) { 1.0 } else { -1.0 };
    let mut nd: f64 = 1e9;
    for q in src.iter() {
        if !(q.team == team || q.noTouch || bekci(q)) {
            nd = jmin(nd, hypot(q.x - h.x, q.y - h.y));
        }
    }
    let mut cx = Cx { src: src.clone(), team, ch, kayip: lossAt.clone(), h: h.clone(), PW: Default::default(), PE: Default::default(), PV: Default::default(), PD: Default::default(), IM: None };
    let L = |k: &mut Kafa, x: f64, y: f64| kay(k, lossAt, x, y);
    let mut govde = |k: &mut Kafa, mut cx: &mut Cx, o: &mut Secenek| {
        let d = o.det.clone().unwrap_or_default();
        let kind = o.kind.as_str();
        if kind == "gönder" {
            let eO = d.endO.or(d.end).unwrap_or(nk(h.x, h.y));
            let Pl = d.Pl.unwrap_or(0.0);
            let Lv = L(k, eO.x, eO.y);
            let P = d.Pk.unwrap_or(0.0);
            o.v = Some(P - Pl * Lv);
            o.prensip = Some("Vur".into());
            o.dok = Some(Dok::yeni(&[("gol", P), ("kayip", -Pl * Lv)]));
        } else if kind == "pas" || kind == "önüne" || kind == "kenardan" {
            let at = o._at.or(d.endD).or(d.endM).or(o.to).unwrap_or_else(|| {
                let q = &src[o.q.unwrap()];
                nk(q.x, q.y)
            });
            let Pd = dis::alP(Some(&d)).or(d.P).unwrap_or(0.0);
            let Ps = veyo(d.Ps, 0.0);
            let Pl = d.Pl.unwrap_or_else(|| jmax(0.0, 1.0 - d.P.unwrap_or(0.0)));
            let adv = (at.x - h.x) * dir;
            let rq: Op = match o.q {
                Some(qi) => match bul_i(src, src[qi].id, src[qi].team) {
                    Some(i) => src[i].clone(),
                    None => src[qi].clone(),
                },
                None => h.clone(),
            };
            let tf = match o.tf {
                Some(t) if crate::mat::dogru(t) => t,
                _ => dis::ucusT(k, hypot(at.x - h.x, at.y - h.y), k.D.arriveFirm, None),
            };
            let pw = pDunya(k, &mut cx, at, tf, "pass", Some(&rq));
            let wE = dis::enW(k, &pw, o);
            let tA = en_gec(k, &mut cx, &rq, &wE, |k, cx, rq| {
                let r = durumD(k, cx, &wE, rq, at, veya(tf, 0.0));
                if ayni(rq, &cx.h) {
                    (r.v, r, 1.0)
                } else {
                    let mut c = (**rq).clone();
                    c.x = at.x;
                    c.y = at.y;
                    c.vx = 0.0;
                    c.vy = 0.0;
                    let kA = jmax(0.0, jmin(1.0, dis::keepP(k, &c, &wE, None, k.D.recvKeepT)));
                    let lv = kay(k, &cx.kayip.clone(), at.x, at.y);
                    (kA * r.v - (1.0 - kA) * lv, r, kA)
                }
            });
            let tS = 0.0;
            let eO = d.endO.or(d.end).unwrap_or(at);
            let Lv = L(k, eO.x, eO.y);
            o.v = Some(Pd * tA.0 + Ps * tS - Pl * Lv);
            o.prensip = Some(if adv >= 6.0 { "İlerlet" } else if nd < 4.0 { "Güvene al" } else { "Çevir" }.into());
            o.dok = Some(Dok::yeni(&[("tutar", Pd), ("alici", tA.2), ("durum", tA.1.v), ("sekme", Ps * tS), ("kayip", -Pl * Lv), ("sure", tf)]));
            o._pw = Some(wE);
            o._pT = Some(tf);
            dok_dk(&tA.1, o);
        } else if kind == "sür" || kind == "çalım" {
            let E = o.toE.or(o.to).unwrap_or(nk(h.x, h.y));
            let LE = hypot(E.x - h.x, E.y - h.y);
            let Ts = jmax(6.0, match o.dir {
                Some(dr) => dis::surSure(k, &h, LE, Some(dr), None),
                None => LE / 0.2,
            });
            let P = match d.P {
                Some(p) => p,
                None => dis::keepP(k, &h, src, o.dir, Ts),
            };
            let adv = (E.x - h.x) * dir;
            let pw = pDunya(k, &mut cx, E, Ts, "drive", Some(&h));
            let wE = dis::enW(k, &pw, o);
            let tE = en_gec(k, &mut cx, &h.clone(), &wE, |k, cx, hq| durumD(k, cx, &wE, hq, E, Ts));
            let Lv = L(k, (h.x + E.x) / 2.0, (h.y + E.y) / 2.0);
            o.v = Some(P * tE.v - (1.0 - P) * Lv);
            o.prensip = Some(if kind == "çalım" { "Çalım" } else if adv > 3.0 { "Alana sür" } else { "Sür (yana/geri)" }.into());
            o.dok = Some(Dok::yeni(&[("tutar", P), ("durum", tE.v), ("kayip", -(1.0 - P) * Lv), ("sure", Ts)]));
            o._pw = Some(wE);
            o._pT = Some(Ts);
            dok_dk(&tE, o);
        } else if kind == "tut" && bekci(&h) && h.bekBas.is_some() && veyo(o.hold, 15.0) >= 90.0 - (k.tick() - h.bekBas.unwrap()) - 4.0 {
            let v = -L(k, h.x, h.y);
            o.v = Some(v);
            o.prensip = Some("Bekle".into());
            o.dok = Some(Dok::yeni(&[("kayip", v)]));
        } else if kind != "tut" {
            o.v = None;
        } else {
            let Th = veyo(o.hold, T);
            let P = match d.P {
                Some(p) => p,
                None => dis::keepP(k, &h, src, None, Th),
            };
            let pw = pDunya(k, &mut cx, nk(h.x, h.y), Th, "drive", Some(&h));
            let wE = dis::enW(k, &pw, o);
            let tH = en_gec(k, &mut cx, &h.clone(), &wE, |k, cx, hq| durumD(k, cx, &wE, hq, nk(h.x, h.y), Th));
            let Lv = L(k, h.x, h.y);
            o.v = Some(P * tH.v - (1.0 - P) * Lv);
            o.prensip = Some(if nd < 4.0 { "Kalkan" } else { "Bekle" }.into());
            let simdi = durumD(k, &mut cx, src, &h, nk(h.x, h.y), 0.0).v;
            o.dok = Some(Dok::yeni(&[("tutar", P), ("durum", tH.v), ("simdi", simdi), ("kayip", -(1.0 - P) * Lv), ("sure", Th)]));
            o._pw = Some(wE);
            o._pT = Some(Th);
            dok_dk(&tH, o);
        }
    };
    // seçenekler birbirinden bağımsız (paralel kafada işçilere bölüşülür; sıra sınaması ALAN_TERS_IZ)
    let ix: Vec<usize> = (0..opts.len()).filter(|&i| opts[i].v.is_some()).collect();
    let n = ix.len();
    let ters = crate::iz_acik() && std::env::var("ALAN_TERS_PR_IZ").is_ok();
    let taze = crate::iz_acik() && std::env::var("ALAN_PR_TAZE_IZ").is_ok();
    // DİKKAT: önbellekler (PW/PE/PV/PD) yuvarlanmış anahtarla ilk soranın sonucunu verir (JS gibi): sıra
    // sınaması 289. tikte ayrışır, bu yüzden bölüşülmez (PR_BOL kapalı).
    let bol = (PR_BOL || taze) && !ters && crate::paralel::hazir(n);
    if bol {
        let mp: Vec<Op> = k.mac.as_ref().map(|m| m.ps.clone()).unwrap_or_default();
        crate::paket::taban_kur([src], mp.iter());
    }
    {
        let mut bir = |j: usize, pb: Option<&[u8]>| -> Option<Vec<u8>> {
            let i = ix[j];
            if let Some(pb) = pb {
                opts[i] = crate::paket::coz(pb);
                return None;
            }
            if taze {
                cx.PW.clear();
                cx.PE.clear();
                cx.PV.clear();
                cx.PD.clear();
                if let Some(m) = &cx.IM {
                    m.borrow_mut().clear();
                }
            }
            govde(k, &mut cx, &mut opts[i]);
            if bol {
                let b = crate::paket::kodla(&opts[i]);
                if crate::iz_acik() && std::env::var("ALAN_PKS_IZ").is_ok() {
                    let _o: Secenek = crate::paket::coz(&b);
                }
                Some(b)
            } else {
                None
            }
        };
        if !(bol && crate::paralel::dagit(n, &mut bir)) {
            for j in 0..n {
                bir(if ters { n - 1 - j } else { j }, None);
            }
        }
    }
    if bol {
        crate::paket::taban_bos();
    }
    if taze {
        // seçeneklerin önbellekleri sonrasına taşınmaz (hangi işçi en son hangisini tarttıysa ondan bağımsız)
        cx.PW.clear();
        cx.PE.clear();
        cx.PV.clear();
        cx.PD.clear();
        if let Some(m) = &cx.IM {
            m.borrow_mut().clear();
        }
    }
    // m._prT: arkadaşların ve taşıyanın değeri, bir rakip olmasa ne kazanılırdı
    {
        let mut tm: Vec<(f64, f64)> = Vec::new();
        let mut mm: Vec<(String, f64)> = Vec::new();
        let mut CXS: Vec<(Op, Cx)> = Vec::new();
        // karşılaştırmanın gerçek dünyası da temiz bağlamda: seçenek tartısının önbellekleri başka (bitiş) dünyalarından
        // dolu; "o olmasa" farkı iki taraf aynı biçimde hesaplanmazsa önbellek artığını ölçer
        let mut cx0 = Cx { src: cx.src.clone(), team: cx.team, ch: cx.ch, kayip: cx.kayip.clone(), h: cx.h.clone(), PW: Default::default(), PE: Default::default(), PV: Default::default(), PD: Default::default(), IM: None };
        let mac_var = k.mac.is_some();
        let inL = k.k2._inLook;
        fn cx_siz<'a>(CXS: &'a mut Vec<(Op, Cx)>, cx: &Cx, d: &Op) -> &'a mut Cx {
            let i = match CXS.iter().position(|(z, _)| ayni(z, d)) {
                Some(i) => i,
                None => {
                    let c = Cx {
                        // sıra korunur (kafada sırayla tutulan bir şey kaymasın): çıkarılan savunmacı silinmez, etkisizleşir
                        // (dokunamaz ve sahanın çok dışında). Eskisi (listeden silmek): ALAN_CXIM_ESKI_IZ
                        src: Arc::new(if cxim_eski() {
                            cx.src.iter().filter(|z| !ayni(z, d)).cloned().collect()
                        } else {
                            cx.src.iter().map(|z| if ayni(z, d) { let mut o = (**z).clone(); o.noTouch = true; o.x = -1000.0; o.y = -1000.0; o.vx = 0.0; o.vy = 0.0; Arc::new(o) } else { z.clone() }).collect()
                        }),
                        team: cx.team,
                        ch: cx.ch,
                        kayip: cx.kayip.clone(),
                        h: cx.h.clone(),
                        PW: Default::default(),
                        PE: Default::default(),
                        PV: Default::default(),
                        PD: Default::default(),
                        IM: if cxim_eski() { cx.IM.clone() } else { None }, /* her varsayım dünyasının kendi fırsat önbelleği (anahtarda dünya yok: paylaşılınca ilk dünyanın değeri bütün savunmacılara gidiyordu) */
                    };
                    CXS.push((d.clone(), c));
                    CXS.len() - 1
                }
            };
            &mut CXS[i].1
        }
        for kq in src.iter() {
            if kq.team != team || ayni(kq, &h) || kq.noTouch || bekci(kq) {
                continue;
            }
            let Tk = dis::ucusT(k, jmax(5.0, hypot(kq.x - h.x, kq.y - h.y)), k.D.arriveFirm, None) + k.D.sutHazir;
            let vk = nodeV(k, if cxim_eski() { &mut cx } else { &mut cx0 }, kq, Tk, 0.0, None);
            tm.push((kq.id, vk));
            if !inL && mac_var {
                for d in src.iter() {
                    if d.team == team || bekci(d) || d.noTouch {
                        continue;
                    }
                    let uz = hypot(d.x - kq.x, d.y - kq.y);
                    if uzak_kis() && uz > UZAK_R {
                        continue;
                    }
                    let c = cx_siz(&mut CXS, &cx, d);
                    let v2 = nodeV(k, c, kq, Tk, 0.0, None);
                    if uzak_iz() {
                        // atamadaki yetişme olasılığının yaklaşığı (esles: 1/(1+e^((t−TW)/5)), t ≈ uzaklık/hız, TW ≈ Tk)
                        let kp = 1.0 / (1.0 + exp((uz / crate::kafa::kafa1_js::ya(k.vS(d), 0.05) - Tk) / 5.0));
                        eprintln!("UZAK {:.2} {:.6} t={:?} kq={} d={} dx={:.0} dy={:.0} kx={:.0} kp={:.6}", uz, jmax(0.0, v2 - vk), k.bag._tick, kq.id, d.id, d.x, d.y, kq.x, kp);
                    }
                    if v2 > vk {
                        mm.push((format!("{}|{}", js_sayi(kq.id), js_sayi(d.id)), v2 - vk));
                    }
                }
            }
        }
        let hm = if inL || !mac_var {
            None
        } else {
            let mut M: Vec<(f64, f64)> = Vec::new();
            let mut Mk: Vec<f64> = Vec::new();
            let mut base: Option<f64> = None;
            for d in src.iter() {
                if d.team == team || bekci(d) || d.noTouch {
                    continue;
                }
                if base.is_none() {
                    base = Some(nodeV(k, if cxim_eski() { &mut cx } else { &mut cx0 }, &h, k.D.sutHazir, 0.0, None));
                }
                let uz = hypot(d.x - h.x, d.y - h.y);
                if uzak_h() && uz > UZAK_R {
                    // listede kalır (Mk: tartılan rakipler), değeri tartılmaz: uzak rakip taşıyıcıya etki etmiyor
                    if !Mk.contains(&d.id) {
                        Mk.push(d.id);
                    }
                    continue;
                }
                let c = cx_siz(&mut CXS, &cx, d);
                let v = nodeV(k, c, &h, k.D.sutHazir, 0.0, None);
                if uzak_iz() {
                    eprintln!("UZAKH {:.2} {:.6}", uz, jmax(0.0, v - base.unwrap()));
                }
                if !Mk.contains(&d.id) {
                    Mk.push(d.id);
                }
                if v > base.unwrap() {
                    match M.iter_mut().find(|x| x.0 == d.id) {
                        Some(x) => x.1 = v - base.unwrap(),
                        None => M.push((d.id, v - base.unwrap())),
                    }
                }
            }
            Some((M, Mk))
        };
        if mac_var {
            let hv = nodeV(k, &mut cx, &h, k.D.sutHazir, 0.0, None);
            k.prT = Some(PrT { team, hId: h.id, t: k.bag._tick, m: tm, mm, hm, h: hv });
        }
    }
}

/// JS sonraki GH sabiti (yalnız sınırlar için)
pub const _GH: f64 = GH;
