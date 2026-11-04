//! Eski kafa (ALAN_PRENSIP açıkken yalnız hayali bakışta `_inLook` ile; bugünkü maçta erişilmez):
//! kontraKur, zinPay, alAt, bekDuelP, secondStep, tekSut (tek dokunuşla şut), taktik eğilimleri
//! (tercih, amac, kontra/risk/maç zamanı). decide.js 5065–5095, 5271–5422, 5574–5638, 7893–8008,
//! 8622–8705.
//!
//! Taşınmayanlar (bugünkü davranışta hiçbir yoldan erişilmez; NOT2.md): zincirKur (D._zin yalnız
//! `!PR && !_inLook` iken, ALAN_PRENSIP açıkken hiç), sonraDeger / surSonra / bekTart (aynı koşul ve
//! `holdF()`), lookAhead / reuseLook / saveLook (D.look kapalı), `!ALAN_PRENSIP` bloğu.
#![allow(non_snake_case, clippy::too_many_arguments)]

use super::gecici_dis2 as dis;
use super::gecici_dis2::{ALAN_HZ, ALAN_SPRE, ALAN_Y};
use super::kafa2_baglam::*;
use super::kafa2_dunya::bekKay;
use crate::mat::{atan2, dogru, exp, hypot, jmax, jmin, pow};
use crate::model::{Oyuncu, SecAmac, Secenek};
use crate::saha::{CY as GCY, W as GW};
use std::collections::HashMap;
use std::sync::Arc;

/// JS D._zin (zincir haritası: arkadaş id → değer)
#[derive(Clone, Debug, Default)]
pub struct Zincir {
    pub team: f64,
    pub m: Vec<(f64, f64)>,
}

/// JS zinPay(w, team, q, at, risk) (decide.js:5622): at'tan zincirdeki arkadaşa pas
pub fn zinPay(k: &mut Kafa, w: &[Op], team: f64, q: &Oyuncu, at: Nokta, risk: f64) -> f64 {
    let Z = match &k.k2.zin {
        Some(z) if z.team == team => z.clone(),
        _ => return -1e9,
    };
    let mut best = -1e9;
    for kq in w {
        if kq.team != team || kq.id == q.id || kq.noTouch {
            continue;
        }
        let z = match Z.m.iter().find(|x| x.0 == kq.id) {
            Some(x) => x.1,
            None => continue,
        };
        if z <= -1e8 {
            continue;
        }
        let L = hypot(kq.x - at.x, kq.y - at.y);
        if L < 6.0 || L > dis::pasMax(k) {
            continue;
        }
        let Lo = dis::laneOk(k, w, team, at, nk(kq.x, kq.y), None, None);
        if Lo < 0.05 {
            continue;
        }
        let v = Lo * z - (1.0 - Lo) * dis::V(k, w, 1.0 - team, (at.x + kq.x) / 2.0, (at.y + kq.y) / 2.0, 0.3) * risk;
        if v > best {
            best = v;
        }
    }
    if best > -1e9 {
        k.D.combo * best
    } else {
        -1e9
    }
}

/// JS alAt(w, q, at): alıcı at'a taşınmış dünya
pub fn alAt(w: &[Op], q: &Oyuncu, at: Nokta) -> Dunya {
    let i = match bul_i(w, q.id, q.team) {
        Some(i) => i,
        None => return w.to_vec(),
    };
    if hypot(w[i].x - at.x, w[i].y - at.y) <= 1.5 {
        return w.to_vec();
    }
    let mut w2 = w.to_vec();
    w2[i] = kxyv(&w[i], at.x, at.y, 0.0, 0.0);
    w2
}

/// bosluk (kafa2_imkan)
pub fn bosluk_(k: &mut Kafa, w: &[Op], team: f64, x: f64, y: f64) -> f64 {
    super::kafa2_imkan::bosluk(k, w, team, x, y)
}

/// JS kontraKur(src, team, ch, h) (decide.js:5574): top (x, y)'de kaybedilirse rakibin hızlı hücum
/// pası (önbellekli, yarım birimlik ızgara).
pub struct KontraF {
    src: Arc<Dunya>,
    team: f64,
    ch: f64,
    h: Op,
    C2: HashMap<u64, f64>,
    Z: Option<Vec<(Op, f64)>>,
}
pub fn kontraKur(src: Arc<Dunya>, team: f64, ch: f64, h: Op) -> KontraF {
    KontraF { src, team, ch, h, C2: HashMap::new(), Z: None }
}
impl KontraF {
    fn zK(&mut self, k: &mut Kafa) -> Vec<(Op, f64)> {
        if let Some(z) = &self.Z {
            return z.clone();
        }
        let opp = 1.0 - self.team;
        let mut Z = Vec::new();
        let src = self.src.clone();
        for kq in src.iter() {
            if kq.team == self.team || kq.noTouch || bekci(kq) {
                continue;
            }
            let Tk = dis::ucusT(k, jmax(1.0, hypot(kq.x - self.h.x, kq.y - self.h.y)), k.D.arriveFirm, None);
            let mut wk = dis::respond(k, &src, opp, nk(kq.x, kq.y), Tk, "pass");
            if let Some(bi) = wk.iter().position(|o| o.team == self.team && bekci(o) && !o.noTouch) {
                let b2 = bekKay(k, &wk[bi], nk(kq.x, kq.y), Tk);
                if !ayni(&b2, &wk[bi]) {
                    wk[bi] = b2;
                }
            }
            let kA = dis::keepP(k, kq, &wk, None, k.D.recvKeepT);
            let z = kA * jmax(dis::V(k, &wk, opp, kq.x, kq.y, dis::chKayip(self.ch)), if k.D.useSpace { dis::runOn(k, &wk, opp, kq, nk(kq.x, kq.y)) } else { 0.0 });
            Z.push((kq.clone(), z));
        }
        self.Z = Some(Z.clone());
        Z
    }
    pub fn deger(&mut self, k: &mut Kafa, x: f64, y: f64) -> f64 {
        let X = crate::mat::jround(2.0 * x) / 2.0;
        let Y = crate::mat::jround(2.0 * y) / 2.0;
        let key = ((X * 1e4) + Y).to_bits() ^ (X.to_bits().rotate_left(17));
        if let Some(v) = self.C2.get(&key) {
            return *v;
        }
        let opp = 1.0 - self.team;
        let mut b = -1e9;
        let A = nk(X, Y);
        let src = self.src.clone();
        for (kq, z) in self.zK(k) {
            let L = hypot(kq.x - X, kq.y - Y);
            if L < 6.0 || L > dis::pasMax(k) {
                continue;
            }
            let Lo = dis::laneOk(k, &src, opp, A, nk(kq.x, kq.y), None, None);
            if Lo < 0.05 {
                continue;
            }
            let v1 = Lo * z - (1.0 - Lo) * dis::V(k, &src, self.team, (X + kq.x) / 2.0, (Y + kq.y) / 2.0, 0.3);
            if v1 > b {
                b = v1;
            }
        }
        let v = if b > -1e9 { k.D.combo * b } else { -1e9 };
        self.C2.insert(key, v);
        v
    }
}

/// JS bekDuelP(h, src, to, dir) (decide.js:5390): sürülen yerde Bekçi ile ikiliye girme olasılığı
pub fn bekDuelP(k: &mut Kafa, h: &Op, src: &[Op], to: Nokta, dir: Nokta) -> Option<f64> {
    let team = h.team;
    let bk = src.iter().find(|q| q.team != team && bekci(q))?.clone();
    let hE = kxy(h, to.x, to.y);
    let ps: Dunya = src.iter().map(|q| if ayni(q, h) { hE.clone() } else { q.clone() }).collect();
    let bt = {
        let fm = dis::Fm { tac: &[], ps: &ps, holder: Some(&hE) };
        dis::AS_bekci(k, &bk, to, Some(&fm))
    };
    let dB = hypot(bt.x - to.x, bt.y - to.y);
    if dB < 2.4 + 1.4 {
        let vC = (0.17 + 0.006 * ALAN_HZ(h)) * ALAN_SPRE(h) * dis::topK(k, h);
        let tH = hypot(to.x - h.x, to.y - h.y) / vC;
        let vK = (0.17 + 0.006 * ALAN_HZ(&bk)) * ALAN_SPRE(&bk);
        let tK = 0.5 * jmax(0.0, 14.0 - 0.5 * ALAN_Y(&bk, "bekRefleks")) + hypot(bt.x - bk.x, bt.y - bk.y) / vK;
        if tK <= tH + 8.0 + 1.2 * hypot(bt.x - bk.x, bt.y - bk.y) {
            let hA = kxyv(h, to.x, to.y, dir.x * vC, dir.y * vC);
            let bA = kxy(&bk, bt.x, bt.y);
            let ps2: Dunya = src.iter().map(|q| if ayni(q, h) { hA.clone() } else if ayni(q, &bk) { bA.clone() } else { q.clone() }).collect();
            return Some(jmax(0.0, jmin(1.0, dis::duelP(k, &bA, &hA, &ps2))));
        }
    }
    None
}

/// JS secondStep(world, carrier, ch, rnd) (decide.js:5065): taşıyanın en iyi ikinci pası
pub fn secondStep(k: &mut Kafa, world: &[Op], ci: Pid, ch: f64, rnd: &mut Zar) -> (f64, Option<Pid>) {
    let carrier = world[ci].clone();
    let team = carrier.team;
    let mut cand: Vec<(Pid, f64)> = Vec::new();
    for qi in 0..world.len() {
        let q = &world[qi];
        let d = hypot(q.x - carrier.x, q.y - carrier.y);
        if q.team == team && qi != ci && !bekci(q) && d >= 4.0 && d <= 50.0 {
            let mut m: f64 = 0.0;
            for i in 1..=4 {
                let f = i as f64 / 5.0;
                m = jmax(m, dis::oppAt(k, world, team, carrier.x + (q.x - carrier.x) * f, carrier.y + (q.y - carrier.y) * f));
            }
            let s = exp(2.0 * -m) * dis::V(k, world, team, q.x, q.y, ch);
            cand.push((qi, s));
        }
    }
    cand.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    cand.truncate(2);
    let mut best = 0.0;
    let mut who = None;
    for (qi, _) in cand {
        let q = world[qi].clone();
        let lf = dis::launchFor(k, &carrier, nk(q.x, q.y), k.D.arrive, world, ch);
        if !lf.ok {
            continue;
        }
        let im = dis::imagine(k, &carrier, world, dis::Top0 { x: carrier.x, y: carrier.y, vx: lf.vx, vy: lf.vy, ch, aim: None }, rnd, 1.0, Some(&q), None);
        let v = im.P.unwrap_or(f64::NAN) * dis::V(k, world, team, q.x, q.y, jmax(0.15, 0.7 * ch));
        if v > best {
            best = v;
            who = Some(qi);
        }
    }
    (best, who)
}

/// JS tercih(h, opts, tac, src) (decide.js:5363): taktik eğilimleri (plan tarafı, istek, pas boyu, şut, sabır)
pub fn tercih(k: &mut Kafa, h: &Oyuncu, opts: &mut [Secenek], tac: Option<&Tac>, src: &[Op]) {
    let tac = match tac {
        Some(t) => t,
        None => return,
    };
    let ps = 0.0;
    let pb = tac.pasBoy.unwrap_or(0.5) - 0.5;
    let sb = tac.sabir.unwrap_or(0.5) - 0.5;
    let su = tac.sut.unwrap_or(0.5) - 0.5 + (h.tl.as_ref().and_then(|t| t.huc.get("gonder").copied()).unwrap_or(0.5) - 0.5);
    let dir = if t0(h.team) { 1.0 } else { -1.0 };
    let pT = tac.plan_taraf.clone();
    let team = h.team;
    for o in opts.iter_mut() {
        if o.v.is_none() {
            continue;
        }
        if let (Some(pt), Some(to)) = (&pT, o.to) {
            if o.kind != "gönder" && o.kind != "tut" {
                let sol = (to.y - GCY) * (if t0(team) { -1.0 } else { 1.0 }) > 4.0;
                let sag = (to.y - GCY) * (if t0(team) { 1.0 } else { -1.0 }) > 4.0;
                let isol = pt == "sol";
                let d = k.D.planK * if if isol { sol } else { sag } { 1.0 } else if if isol { sag } else { sol } { -0.5 } else { 0.0 };
                dis::teK(o, "tercih", d);
            }
        }
        if let Some(qi) = o.q {
            if let Some(istek) = src[qi].tl.as_ref().and_then(|t| t.huc.get("istek").copied()) {
                let ik = (istek - 0.5) * k.D.istekK;
                if o.kind == "önüne" {
                    dis::teK(o, "tercih", ik);
                } else if o.kind == "pas" {
                    dis::teK(o, "tercih", -ik);
                }
            }
        }
        let pas = o.kind == "pas" || o.kind == "önüne" || o.kind == "kenardan";
        let tp = o.to.or(o.q.map(|i| nk(src[i].x, src[i].y)));
        if o.kind == "sür" || o.kind == "çalım" {
            dis::teK(o, "tercih", ps * k.D.tPS);
        } else if pas {
            dis::teK(o, "tercih", -ps * k.D.tPS);
        }
        if pas && dogru(pb) {
            if let Some(tp) = tp {
                let u = jmax(-1.0, jmin(1.0, (hypot(tp.x - h.x, tp.y - h.y) - k.D.pbOrta) / k.D.pbGen));
                if pb * u < 0.0 {
                    dis::teK(o, "tercih", -2.0 * pb.abs() * u.abs() * k.D.tPB);
                }
            }
        }
        if o.kind == "gönder" {
            dis::teK(o, "tercih", su * k.D.tSut);
        }
        if pas || o.kind == "sür" {
            if let Some(tp) = tp {
                let adv = (tp.x - h.x) * dir;
                let SR = sb > 0.0;
                let rk = if SR { jmax(0.0, jmin(1.0, (1.0 - dis::alP(o.det.as_ref()).unwrap_or(1.0)) / 0.2)) } else { 1.0 };
                let bs = if SR { jmin(1.0, dis::pressAt(k, src, team, h.x, h.y)) } else { 1.0 };
                if adv > 6.0 {
                    dis::teK(o, "sabir", -sb * k.D.tSab * rk);
                } else if adv < -2.0 {
                    dis::teK(o, "sabir", sb * k.D.tSab * bs);
                }
            }
        }
    }
}

/// JS amac(h, opts, tac, src) (decide.js:5271): ilerleme, rakip geçme ve açılan seçenekler
pub fn amac(k: &mut Kafa, hi: Pid, opts: &mut [Secenek], tac: Option<&Tac>, src: &[Op]) {
    let h = src[hi].clone();
    let team = h.team;
    let gx = if t0(team) { GW } else { 0.0 };
    let gD = |x: f64, y: f64| hypot(gx - x, GCY - y);
    let sab = jmax(0.0, jmin(1.0, tac.and_then(|t| t.sabir).unwrap_or(0.5)));
    let secenek = |k: &mut Kafa, x: f64, y: f64, ben: &Oyuncu, w: &[Op]| -> f64 {
        let d0 = gD(x, y);
        let mut n = 0.0;
        for kq in w {
            if kq.team != team || bekci(kq) || kq.noTouch || kq.id == ben.id {
                continue;
            }
            let L = hypot(kq.x - x, kq.y - y);
            let ilerde = gD(kq.x, kq.y) <= d0 + 3.0;
            if L < 6.0 || L > dis::pasMax(k) {
                continue;
            }
            let mut yak: f64 = 1e9;
            for o in w {
                if !(o.team == team || bekci(o) || o.noTouch) {
                    yak = jmin(yak, hypot(o.x - kq.x, o.y - kq.y));
                }
            }
            let bos = jmax(0.0, jmin(1.0, (yak - 2.0) / 6.0));
            if bos > 0.0 {
                n += (if ilerde { 1.0 } else { 0.4 }) * bos * dis::laneOk(k, w, team, nk(x, y), nk(kq.x, kq.y), None, None);
            }
        }
        n
    };
    let mut Wk: HashMap<String, Dunya> = HashMap::new();
    let mut kosuDun = |k: &mut Kafa, B: Nokta, T: f64| -> Dunya {
        let key = format!("{}|{}|{}", crate::mat::jround(B.x), crate::mat::jround(B.y), crate::mat::jround(T / 5.0));
        if let Some(w) = Wk.get(&key) {
            return w.clone();
        }
        let w = super::kafa2_dunya::predictWorld(k, src, team, B, jmax(5.0, jmin(45.0, T)), false, None, false, None);
        Wk.insert(key, w.clone());
        w
    };
    let O: Vec<Op> = src.iter().filter(|o| o.team != team && !bekci(o) && !o.noTouch).cloned().collect();
    let n0 = secenek(k, h.x, h.y, &h, src);
    let gB = gD(h.x, h.y);
    let mut memo: HashMap<String, f64> = HashMap::new();
    for o in opts.iter_mut() {
        if o.v.is_none() || o.kind == "gönder" || o.kind == "fake" {
            continue;
        }
        if o.kind == "tut" {
            if bekci(&h) {
                continue;
            }
            match &o.hw {
                Some(hw) if !hw.kayip => {}
                _ => continue,
            }
            let T = veyo(o.hold, 15.0);
            let wk = kosuDun(k, nk(h.x, h.y), T);
            let n1 = secenek(k, h.x, h.y, &h, &wk);
            let P = jmax(0.0, jmin(1.0, o.hw.as_ref().and_then(|w| w.Pk).unwrap_or(1.0)));
            let ac = k.D.amAc * (n1 - n0) * (1.0 + sab);
            if ac > 0.0 {
                let v0 = o.v.unwrap();
                o.v = Some(dis::tercihEk(v0, P * ac));
                let dd = o.v.unwrap() - v0;
                dis::dokEk(o, "acti", dd / (1.0 + sab));
                dis::dokEk(o, "sabir", dd * sab / (1.0 + sab));
            }
            o.amac = Some(SecAmac { il: 0.0, ac: sabitle(P * jmax(0.0, ac), 3), gec: 0.0, n0: sabitle(n0, 1), n1: sabitle(n1, 1), bekle: true });
            continue;
        }
        let mut ben = h.clone();
        let R: Option<Nokta> = if o.kind == "sür" || o.kind == "çalım" {
            o.toE.or(o.to)
        } else if o.q.is_some() && o.q != Some(hi) {
            ben = src[o.q.unwrap()].clone();
            let e = match &o.det {
                Some(d) if dis::alP(Some(d)).unwrap_or(f64::NAN) > 0.5 && d.endM.map(|e| e.x.is_finite()).unwrap_or(false) => d.endM,
                _ => None,
            };
            e.or(if o.kind == "önüne" { o.to } else { Some(nk(ben.x, ben.y)) })
        } else if o.q == Some(hi) {
            o.lead.or(o.to)
        } else {
            None
        };
        let R = match R {
            Some(r) if r.x.is_finite() => r,
            _ => continue,
        };
        let P = jmax(0.0, jmin(1.0, dis::alP(o.det.as_ref()).unwrap_or(1.0)));
        let gR = gD(R.x, R.y);
        let mut gec = 0.0;
        for p in &O {
            let g = gD(p.x, p.y);
            if g < gB - 1.0 && g > gR + 1.0 {
                gec += 1.0;
            } else if g > gB + 1.0 && g < gR - 1.0 {
                gec -= 1.0;
            }
        }
        let mut il = k.D.amIleri * (gB - gR) / 30.0 + k.D.amGec * gec;
        let il0 = il;
        if il < 0.0 {
            il *= 1.0 - 0.6 * sab;
        }
        let key = format!("{}:{},{}", ben.id, crate::mat::jround(R.x), crate::mat::jround(R.y));
        let mut n1 = match &o._wE {
            Some(w) => Some(secenek(k, R.x, R.y, &ben, w)),
            None => memo.get(&key).copied(),
        };
        if n1.is_none() {
            let v = if o.kind != "sür" && o.kind != "çalım" {
                secenek(k, R.x, R.y, &ben, src)
            } else {
                let wk = kosuDun(k, R, hypot(R.x - h.x, R.y - h.y) / 0.25);
                secenek(k, R.x, R.y, &ben, &wk)
            };
            memo.insert(key, v);
            n1 = Some(v);
        }
        let n1 = n1.unwrap();
        let ac = k.D.amAc * (n1 - n0) * (1.0 + sab);
        let pay = P * (il + ac);
        let v0 = o.v.unwrap();
        o.v = Some(dis::tercihEk(v0, pay));
        if o.dok.is_some() && dogru(pay) {
            let f = (o.v.unwrap() - v0) / pay;
            dis::dokEk(o, "ilerle", f * P * il0);
            dis::dokEk(o, "sabir", f * P * (il - il0 + ac * sab / (1.0 + sab)));
            dis::dokEk(o, "acti", f * P * ac / (1.0 + sab));
        }
        o.amac = Some(SecAmac { il: sabitle(P * il, 3), ac: sabitle(P * ac, 3), gec, n0: sabitle(n0, 1), n1: sabitle(n1, 1), bekle: false });
    }
}

/// JS decide0K'nın tek dokunuşla şut bloğu (decide.js:7893; `!PR && !D._gon`): pasın alıcısı topu
/// gelişine kaleye vurursa.
pub fn tekSut(k: &mut Kafa, h: &Op, src: &[Op], ch: f64, rnd: &mut Zar, opts: &mut Vec<Secenek>, loss: &mut dyn FnMut(&mut Kafa, f64, f64) -> f64) {
    let team = h.team;
    let gx = if t0(team) { GW } else { 0.0 };
    let mut sec: Vec<usize> = (0..opts.len())
        .filter(|&i| {
            let x = &opts[i];
            (x.kind == "pas" || x.kind == "önüne") && x.q.is_some() && x._at.is_some() && x.det.is_some() && dis::alP(x.det.as_ref()).unwrap_or(f64::NAN) >= 0.5 && hypot(gx - x._at.unwrap().x, GCY - x._at.unwrap().y) < k.D.shotRange
        })
        .collect();
    sec.sort_by(|&a, &b| opts[b].v.partial_cmp(&opts[a].v).unwrap_or(std::cmp::Ordering::Equal));
    sec.truncate(3);
    for oi in sec {
        let (at, w, launch, savT, qid) = {
            let o = &opts[oi];
            (o._at.unwrap(), o._w.clone().unwrap(), o.launch.clone().unwrap(), o.savT, src[o.q.unwrap()].id)
        };
        let bk = w.iter().find(|p| p.team != team && bekci(p)).cloned();
        let q0: Op = w.iter().find(|p| p.id == qid).cloned().unwrap_or_else(|| src[opts[oi].q.unwrap()].clone());
        let tf = dis::ucusSure(k, h, at, &launch, src, ch, savT);
        let bS = bk.as_ref().map(|bk| {
            let bk0 = match bul_i(src, bk.id, bk.team) {
                Some(i) => src[i].clone(),
                None => bk.clone(),
            };
            bekKay(k, &bk0, at, tf - k.D.sutHazir)
        });
        let hS = kxyv(&q0, at.x, at.y, 0.0, 0.0);
        let srcS: Dunya = w.iter().map(|p| if ayni(p, &q0) { hS.clone() } else if bk.as_ref().map(|b| ayni(p, b)).unwrap_or(false) { bS.clone().unwrap() } else { p.clone() }).collect();
        let qq = src[opts[oi].q.unwrap()].clone();
        let c2 = dis::recvCh(k, &w, team, at.x, at.y, ch, &qq);
        let mut best: f64 = 0.0;
        for sd in [-1.0, 1.0] {
            let ty = GCY + sd * crate::saha::MOUTH * k.D.aimPow;
            let dx = gx - at.x;
            let dy = ty - at.y;
            let l = veya(hypot(dx, dy), 1.0);
            let v0 = dis::sutV(k, &q0) * crate::top::chg_mul(c2) * dis::sutGuc(&q0);
            let ot0 = k.D._ot;
            let g0 = k.D._gon;
            let s0 = k.D._sut.clone();
            k.D._ot = Some(veyo(ot0, 1.0) * dis::gonEk(k, &hS, &srcS, "güçlü") * 1.2);
            k.D._gon = Some(true);
            k.D._sut = Some(crate::model::JsDeger::Metin("güçlü".into()));
            let kc0 = k.D._kanca.clone();
            let mut vS = v0;
            {
                let B = crate::core_sabit::Q.body;
                let uin = jmax(0.3, 0.7 * hypot(launch.vx, launch.vy));
                let ca = atan2(h.y - at.y, h.x - at.x);
                k.D._kanca = Some(crate::model::DKanca { w0: uin / B, s: 0.0, ca: Some(ca), u: None, on: false });
                let vE = dis::vurE(k, &q0, Some(c2), true);
                let sv = dis::savurPlan(k, &hS, dx / l * v0, dy / l * v0, ca, uin / B, 0.0, &srcS, false, vE);
                k.D._savW = sv.map(|s| s.wRel);
                k.D._savU = sv.map(|s| s.uSh);
                if let Some(sv) = sv {
                    if sv.T >= 1.0 {
                        let mx = uin * (1.0 - 0.35 * jmin(1.0, sv.A / crate::mat::PI)) + 0.4;
                        if v0 > mx {
                            vS = mx;
                            let s2 = dis::savurPlan(k, &hS, dx / l * vS, dy / l * vS, ca, uin / B, 0.0, &srcS, false, vE);
                            k.D._savW = s2.map(|s| s.wRel);
                            k.D._savU = s2.map(|s| s.uSh);
                        }
                    }
                }
            }
            let tr = if k.k2._inLook { 1.0 } else { 3.0 };
            let im2 = dis::imagine(k, &hS, &srcS, dis::Top0 { x: at.x, y: at.y, vx: dx / l * vS, vy: dy / l * vS, ch: c2, aim: None }, rnd, tr, None, None);
            k.D._ot = ot0;
            k.D._gon = g0;
            k.D._sut = s0;
            k.D._kanca = kc0;
            k.D._savW = None;
            k.D._savU = None;
            if im2.Pk.unwrap_or(f64::NAN) > best {
                best = im2.Pk.unwrap();
            }
        }
        let o = &mut opts[oi];
        o.tek = Some(sabitle(best, 3));
        let det = o.det.clone().unwrap();
        let sk = veyo(o._sek, 0.0);
        let pA = dis::alP(Some(&det)).unwrap_or(f64::NAN);
        let e = det.endO.or(det.end).unwrap_or_default();
        let lv = loss(k, e.x, e.y);
        let Pk = det.Pk.unwrap_or(f64::NAN);
        let Pl = det.Pl.unwrap_or(f64::NAN);
        let rv = (o.v.unwrap_or(f64::NAN) - Pk - sk + Pl * lv) / jmax(1e-6, pA);
        if best > rv {
            let v0 = o.v.unwrap_or(f64::NAN);
            let lv2 = loss(k, e.x, e.y);
            o.v = Some(Pk + pA * best + sk - Pl * lv2);
            dis::dokEk(o, "acti", o.v.unwrap() - v0);
        }
    }
}

/// decide0K'nın eski kafa sonu (decide.js 8622–8705, `!PR`): kontra, risk, maç zamanı, tercih, amac, fake.
pub fn eskiSonra(k: &mut Kafa, hi: Pid, src: &Arc<Dunya>, ch: f64, rnd: &mut Zar, tac: Option<&Tac>, opts: &mut Vec<Secenek>, _risk: f64, T: f64, _loss: &mut dyn FnMut(&mut Kafa, f64, f64) -> f64) {
    let h = src[hi].clone();
    let team = h.team;
    let dir0 = if t0(team) { 1.0 } else { -1.0 };
    let pasre = |s: &str| s.contains("pas") || s.contains("önüne") || s.contains("kenardan");
    if let Some(t) = tac {
        if t.kontra {
            for o in opts.iter_mut() {
                let sur = o.kind == "sür";
                if o.v.is_none() || o.to.is_none() || !(sur || (o.q.is_some() && pasre(&o.kind))) {
                    continue;
                }
                let adv = (o.to.unwrap().x - h.x) * dir0;
                if adv >= 6.0 {
                    let d = 0.5 * k.D.cevTempo * jmin(1.0, adv / 20.0) * dis::alP(o.det.as_ref()).unwrap_or(0.8);
                    dis::teK(o, "tercih", d);
                }
            }
        }
        if let Some(r) = t.risk {
            let kk = 1.0 + 0.8 * jmax(0.0, r - 0.5);
            if kk > 1.0 {
                for o in opts.iter_mut() {
                    let v = match o.v {
                        Some(v) => v,
                        None => continue,
                    };
                    if v <= 0.0 || o.to.is_none() || (!pasre(&o.kind) && o.kind != "sür") || (o.to.unwrap().x - h.x) * dir0 < 6.0 {
                        continue;
                    }
                    let nv = 1.0 - pow(1.0 - jmin(1.0, v), kk);
                    o.v = Some(nv);
                    dis::dokEk(o, "tercih", nv - v);
                }
            }
            let pr = h.tl.as_ref().and_then(|t| t.huc.get("risk").copied()).or(h.eg.as_ref().map(|e| e.rk)).unwrap_or(0.5);
            let rE = jmax(0.0, jmin(1.0, r + 0.6 * (pr - 0.5)));
            let P0 = k.D.riskP0 - k.D.riskPk * rE;
            for o in opts.iter_mut() {
                if o.v.is_none() || !(pasre(&o.kind) || o.kind == "çalım" || o.kind == "sür" || o.kind == "tut") {
                    continue;
                }
                let d = match &o.det {
                    Some(d) if d.P.is_some() => d.clone(),
                    _ => continue,
                };
                let ac = jmax(0.0, P0 - (d.P.unwrap() - d.Ps.unwrap_or(0.0)));
                if ac > 0.0 {
                    o.v = Some(o.v.unwrap() - k.D.riskCay * ac);
                    dis::dokEk(o, "risk", -k.D.riskCay * ac);
                    o.riskCay = Some(sabitle(k.D.riskCay * ac, 3));
                }
            }
        }
        if let Some(mz) = &t.macZ {
            for o in opts.iter_mut() {
                if o.v.is_none() || o.kind == "gönder" {
                    continue;
                }
                let ln = o.to.map(|to| hypot(to.x - h.x, to.y - h.y)).unwrap_or(0.0);
                let dt = if o.kind == "tut" {
                    veyo(o.hold, T)
                } else if o.kind == "sür" {
                    jmax(T, ln / 0.2)
                } else {
                    match o.det.as_ref().and_then(|d| d.T) {
                        Some(t) if dogru(t) => t,
                        _ => jmax(8.0, ln / 0.8),
                    }
                };
                let pk = if o.kind == "sür" || o.kind == "tut" { 1.0 } else { dis::alP(o.det.as_ref()).unwrap_or(0.7) };
                let z = mz(dt) * pk;
                dis::teK(o, "zaman", z);
                o.zaman = Some(z);
            }
        }
    }
    tercih(k, &h, opts, tac, src);
    amac(k, hi, opts, tac, src);
    dis::fakeSecenek(k, hi, src, ch, rnd, tac, opts, true);
}
