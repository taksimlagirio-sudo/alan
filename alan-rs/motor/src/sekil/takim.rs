//! Takım tartıları: pas yolu (laneOpen, yolP, yerV), tehlike yönleri (tehlike), boşluğun takıma değeri
//! (teamV, teamBest, coverSpots, yolT, kurO), savunmanın hat kapatması (kapatHat, coverBoth), metrics.
#![allow(non_snake_case)]

use crate::mat::{atan2, cos, exp, hypot, jmax, jmin, sin};
use crate::model::{KafaD, Oyuncu, Pid};
use crate::oyuncu::Tepki;
use crate::temas::Kural;

use super::gecici_dis as dis;
use super::*;

/// JS laneOpen(w, att, A, B, v): A→B pas yolunun açıklığı (rakiplerin uzanmasıyla).
pub fn laneOpen(w: &[Varlik], oy: &[Oyuncu], att: u8, A: (f64, f64), B: (f64, f64), v: Option<f64>, k: &dyn Kural, d: &KafaD) -> f64 {
    let uc = v.map(|v| v == 0.0 || v != v).unwrap_or(true);
    let v = match v {
        Some(v) if v != 0.0 && v == v => v,
        _ => 1.0,
    };
    let L = {
        let l = hypot(B.0 - A.0, B.1 - A.1);
        if l != 0.0 && l == l {
            l
        } else {
            1.0
        }
    };
    let ux = (B.0 - A.0) / L;
    let uy = (B.1 - A.1) / L;
    let mut open = 1.0;
    for o in w {
        if o.team == att || o.bekci {
            continue;
        }
        let al = (o.x - A.0) * ux + (o.y - A.1) * uy;
        if al <= 0.0 || al >= L + 1.0 {
            continue;
        }
        let g = o.govde(oy);
        let pe = (-(o.x - A.0) * uy + (o.y - A.1) * ux).abs();
        let tb = if uc { dis::ucusT(L, d.arrive, Some(al)) } else { al / v };
        let R = 1.6 + jmax(0.0, tb - TEPKI(g, Tepki::Kovala, k)) * spd(g) * ALAN_SPRE(g, k);
        open *= 1.0 - 1.0 / (1.0 + exp((pe - R) / 0.5));
    }
    open
}

/// JS yolP(w, att, A, B) (ALAN_PRENSIP açık, bekYol var): kafanın pas yolu × Bekçi'nin yolu.
pub fn yolP(w: &[Varlik], oy: &[Oyuncu], att: u8, A: (f64, f64), B: (f64, f64), d: &KafaD) -> f64 {
    dis::laneOk(w, oy, att, A, B, d.arriveFirm, Some(d.sutHazir)) * dis::bekYol(w, oy, att, A, B, d.sutHazir)
}
/// JS yerV(w, att, x, y, ch): yerin değeri (kafanın yerDeger'i).
pub fn yerV(w: &[Varlik], oy: &[Oyuncu], att: u8, x: f64, y: f64, ch: f64) -> f64 {
    dis::yerDeger(w, oy, att, x, y, ch)
}

/// JS teamBest(w2, att, E, mates, ch): taşıyıcının en iyi seçeneği (sür ya da pas).
pub fn teamBest(w2: &[Varlik], oy: &[Oyuncu], att: u8, E: (f64, f64), mates: &[(f64, f64)], ch: f64, k: &dyn Kural, d: &KafaD) -> f64 {
    let gx = oppX(att);
    let gd = {
        let l = hypot(gx - E.0, GCY - E.1);
        if l != 0.0 && l == l {
            l
        } else {
            1.0
        }
    };
    let st = jmin(6.0, jmax(0.0, gd - 6.0));
    let F = (E.0 + (gx - E.0) / gd * st, E.1 + (GCY - E.1) / gd * st);
    let mut best = yerV(w2, oy, att, E.0, E.1, ch);
    if st > 0.0 {
        best = jmax(best, laneOpen(w2, oy, att, E, F, Some(0.3), k, d) * yerV(w2, oy, att, F.0, F.1, ch) * S.W.tvDrive);
    }
    for q in mates {
        best = jmax(best, yolP(w2, oy, att, E, *q, d) * yerV(w2, oy, att, q.0, q.1, dis::recvCh(w2, oy, att, q.0, q.1, ch, None)) * S.W.tvPass);
    }
    best
}

/// Taşıyıcının tehlikeli bir yönü (JS tehlike öğesi).
#[derive(Clone, Debug)]
pub struct Tehlike {
    pub an: f64,
    pub w: f64,
    pub w0: f64,
    pub x: f64,
    pub y: f64,
    pub k: String,
    pub L: f64,
}

/// JS tehlike(m, h): kafanın son seçeneklerinden taşıyıcının tehlikeli yönleri (açıyla birleşmiş, payları).
pub fn tehlike(m: &crate::model::Mac, ps: &[Oyuncu], h: Option<Pid>, d: &KafaD) -> Option<Vec<Tehlike>> {
    let h = h?;
    let sec = m._sec.as_ref()?;
    if sec.h != h || m.tick - sec.t > 45.0 {
        return None;
    }
    let O = &sec.o;
    if O.is_empty() {
        return None;
    }
    let hp = &ps[h];
    let dk = |o: &crate::model::JM_secOE| if o.dv != 0.0 && o.dv == o.dv { d.surKarar } else { 0.0 };
    let mut vm = f64::NEG_INFINITY;
    for o in O {
        vm = jmax(vm, o.v + dk(o));
    }
    let mut out: Vec<Tehlike> = Vec::new();
    let mut top = 0.0;
    for o in O {
        let L = hypot(o.x - hp.x, o.y - hp.y);
        if L < 1.0 {
            continue;
        }
        let w = exp((o.v + dk(o) - vm) / 0.04) * jmax(0.0, o.v - jmax(0.0, sec.tut));
        if w <= 1e-4 {
            continue;
        }
        let an = atan2(o.y - hp.y, o.x - hp.x);
        match out.iter_mut().find(|u| atan2(sin(u.an - an), cos(u.an - an)).abs() < 0.45) {
            Some(f) => {
                if w > f.w0 {
                    f.an = an;
                    f.x = o.x;
                    f.y = o.y;
                    f.k = o.k.clone();
                    f.w0 = w;
                }
                f.w += w;
            }
            None => out.push(Tehlike { an, w, w0: w, x: o.x, y: o.y, k: o.k.clone(), L }),
        }
        top += w;
    }
    if out.is_empty() || top <= 0.0 {
        return None;
    }
    for u in out.iter_mut() {
        u.w /= top;
    }
    out.sort_by(|a, b| b.w.partial_cmp(&a.w).unwrap_or(std::cmp::Ordering::Equal));
    Some(out)
}

/// JS coverSpots(o, att, E, targets, R): savunucunun `R` içinde gidebileceği kapatma noktaları.
pub fn coverSpots(o: (f64, f64), att: u8, E: (f64, f64), targets: &[(f64, f64)], R: f64) -> Vec<(f64, f64)> {
    let mut out = vec![o];
    let A = gside(E.0, E.1, 1 - att, 2.2);
    for P in targets {
        let B = gside(P.0, P.1, 1 - att, 2.2);
        for t in [0.0, 0.5, 1.0] {
            let g = (A.0 + (B.0 - A.0) * t, A.1 + (B.1 - A.1) * t);
            let d = hypot(g.0 - o.0, g.1 - o.1);
            out.push(if d <= R { g } else { (o.0 + (g.0 - o.0) / d * R, o.1 + (g.1 - o.1) / d * R) });
        }
    }
    out
}

/// Ayrı kurulmuş koşu bedeni (JS `{a: p.a, x, y, vx, vy}`): yetenek p'nin, kimlik/bar yok.
pub fn govdeF(p: &Oyuncu, x: f64, y: f64, vx: f64, vy: f64) -> Oyuncu {
    let mut f = Oyuncu::default();
    f.a = p.a.clone();
    f.yet = p.yet.clone();
    f.id = f64::NAN;
    f.x = x;
    f.y = y;
    f.vx = vx;
    f.vy = vy;
    f
}

/// JS yolT(p, c): gel-dön koşusu (önce c.kv'ye, sonra c'ye) süresi: (t1, t).
pub fn yolT(p: &Oyuncu, c: (f64, f64), kv: (f64, f64), k: &dyn Kural) -> (f64, f64) {
    let v = vSp(p, k);
    let d1 = hypot(kv.0 - p.x, kv.1 - p.y);
    let d2 = hypot(c.0 - kv.0, c.1 - kv.1);
    let ux = if d1 > 0.1 { (kv.0 - p.x) / d1 } else { 0.0 };
    let uy = if d1 > 0.1 { (kv.1 - p.y) / d1 } else { 0.0 };
    let t1 = gec(p, kv.0, kv.1, Some(d1), k) + d1 / v;
    let f = govdeF(p, kv.0, kv.1, ux * v * 0.8, uy * v * 0.8);
    (t1, t1 + gec(&f, c.0, c.1, Some(d2), k) + d2 / v)
}

/// JS kurO dönüşü: koşucuyu kovalayan markajcının yeri, varış bedeni ve başlangıcı.
pub struct KurO {
    pub x: f64,
    pub y: f64,
    /// None: bedeni hayaletin kendisi (JS `f: kd`)
    pub f: Option<Oyuncu>,
    pub t0: f64,
}

/// JS kurO(kd, att, p, c, yl).
pub fn kurO(kd: &Varlik, oy: &[Oyuncu], att: u8, p: &Oyuncu, kv: Option<(f64, f64)>, yl: Option<(f64, f64)>, k: &dyn Kural) -> KurO {
    let (yl, kv) = match (yl, kv) {
        (Some(y), Some(kv)) => (y, kv),
        _ => return KurO { x: kd.x, y: kd.y, f: None, t0: 0.0 },
    };
    let g = gside(kv.0, kv.1, 1 - att, S.markOff);
    let bx = kv.0 - p.x;
    let by = kv.1 - p.y;
    let bl = {
        let l = hypot(bx, by);
        if l != 0.0 && l == l {
            l
        } else {
            1.0
        }
    };
    let r = &oy[kd.r.unwrap()];
    let v = 0.8 * vSp(r, k);
    KurO { x: g.0, y: g.1, f: Some(govdeF(r, g.0, g.1, bx / bl * v, by / bl * v)), t0: yl.0 }
}

/// teamV'nin aday noktası (JS `c`): konum, gel-dön ara noktası (kv), kovalayan hayalet (kd: dünya sırası).
#[derive(Clone, Copy, Debug)]
pub struct TvAday {
    pub x: f64,
    pub y: f64,
    pub kv: Option<(f64, f64)>,
    pub kd: Option<usize>,
    pub taban: bool,
}

/// JS iş adı `/^(markaj|arkada)/`.
fn markajci(job: &Option<String>) -> bool {
    match job {
        Some(j) => j.starts_with("markaj") || j.starts_with("arkada"),
        None => false,
    }
}

/// JS teamV(m, sn, att, E, c, mates, okP, p, pK): `p` c'ye koşarsa takımın kazancı; savunmanın en kötü
/// karşılığına göre (okuma `okP` ile karışır). `sn`: p'nin gördüğü dünya (geçici olarak oynatılır, geri konur).
pub fn teamV(c: &mut Mc, sn: &mut Vec<Varlik>, svi: Pid, att: u8, E: (f64, f64), cc: TvAday, mates: &[(f64, f64)], okP: f64, p: Option<Pid>, pK: Option<Pid>) -> f64 {
    let pK = pK.or(p);
    let ch = if c.m.ch != 0.0 && c.m.ch == c.m.ch { c.m.ch } else { 0.3 };
    let k = c.k;
    let d = c.d;
    let w = cl((okP - 6.0) / 10.0, 0.0, 1.0);
    let mut crowd: f64 = 1.0;
    for q in mates {
        crowd = jmin(crowd, hypot(q.0 - cc.x, q.1 - cc.y) / S.ayr);
    }
    let sut = c.m.tac.get(att as usize).and_then(|t| t.sut).unwrap_or(0.5);
    let myOpt = |c: &mut Mc, w2: &[Varlik]| -> f64 {
        let ps = &*c.ps;
        let Vc = yerV(w2, ps, att, cc.x, cc.y, dis::recvCh(w2, ps, att, cc.x, cc.y, ch, p.map(|i| &ps[i])));
        match p {
            None => {
                let yP = yolP(w2, ps, att, E, (cc.x, cc.y), d);
                crowd * (yP * Vc + kpOf(c, w2, svi, (cc.x, cc.y), pK, yP, 1.0)) * S.W.tvPass
            }
            Some(pi) => {
                let pp = &ps[pi];
                let kL = hypot(cc.x - pp.x, cc.y - pp.y);
                let kos = kL > S.kosMin;
                let kux = if kos { (cc.x - pp.x) / kL } else { 0.0 };
                let kuy = if kos { (cc.y - pp.y) / kL } else { 0.0 };
                let kva = if kos { spd(pp) * jmin(1.0, kL / S.kosHiz) } else { 0.0 };
                let mut r = pp.clone();
                r.x = cc.x;
                r.y = cc.y;
                r.vx = kux * kva;
                r.vy = kuy * kva;
                let sutM = hypot(cc.x - oppX(att), cc.y - GCY) < d.shotRange * (0.75 + 0.5 * sut);
                let kA = dis::keepP(&r, w2, ps, if kos && kux * dirOf(att) > -0.3 { Some((kux, kuy)) } else { None }, if sutM { d.sutHazir } else { d.recvKeepT });
                let yP = yolP(w2, ps, att, E, (cc.x, cc.y), d);
                let ro = if d.useSpace { jmax(Vc, dis::runOn(w2, ps, att, &r, (cc.x, cc.y))) } else { Vc };
                let kp = kpOf(c, w2, svi, (cc.x, cc.y), Some(pi), yP, kA);
                crowd * (yP * kA * ro + kp) * S.W.tvPass
            }
        }
    };
    if w <= 0.0 {
        return myOpt(c, sn);
    }
    let ps0 = &*c.ps;
    let opp: Vec<usize> = (0..sn.len()).filter(|&i| sn[i].team != att && !sn[i].bekci).collect();
    let mut byE = opp.clone();
    byE.sort_by(|&u, &v| (hypot(sn[u].x - E.0, sn[u].y - E.1) - hypot(sn[v].x - E.0, sn[v].y - E.1)).partial_cmp(&0.0).unwrap_or(std::cmp::Ordering::Equal));
    let mut byC = opp.clone();
    byC.sort_by(|&u, &v| (hypot(sn[u].x - cc.x, sn[u].y - cc.y) - hypot(sn[v].x - cc.x, sn[v].y - cc.y)).partial_cmp(&0.0).unwrap_or(std::cmp::Ordering::Equal));
    let kd = if p.is_some() { cc.kd.filter(|&i| i < sn.len()) } else { None };
    let refIs = |o: &Varlik, pi: Pid| -> bool { o.r.map(|r| ps0[r].tutQ == Some(pi) && markajci(&ps0[r].job)).unwrap_or(false) };
    let mk = match (p, kd) {
        (Some(pi), None) => opp.iter().copied().find(|&i| refIs(&sn[i], pi)),
        _ => None,
    };
    let mut rs: Vec<usize> = Vec::new();
    if let Some(&e0) = byE.first() {
        rs.push(e0);
    }
    match kd {
        Some(kdi) => {
            if Some(&kdi) != byE.first() {
                rs.push(kdi);
            } else if let Some(&o2) = byC.iter().find(|&&o| o != kdi) {
                rs.push(o2);
            }
        }
        None => {
            if mk.is_some() && mk != byE.first().copied() {
                rs.push(mk.unwrap());
            } else if let Some(&c0) = byC.first() {
                if Some(&c0) != byE.first() {
                    rs.push(c0);
                }
            }
        }
    }
    let yl = match (p, cc.kv) {
        (Some(pi), Some(kv)) => Some(yolT(&ps0[pi], (cc.x, cc.y), kv, k)),
        _ => None,
    };
    let ta = match p {
        Some(pi) => {
            let pp = &ps0[pi];
            match yl {
                Some(y) => y.1,
                None => {
                    let dd = hypot(cc.x - pp.x, cc.y - pp.y);
                    gec(pp, cc.x, cc.y, Some(dd - 0.3), k) + jmax(0.0, dd - 0.3) / vSp(pp, k)
                }
            }
        }
        None => 1e9,
    };
    let kO = kd.map(|kdi| kurO(&sn[kdi], ps0, att, &ps0[p.unwrap()], cc.kv, yl, k));
    let Rof = |sn: &[Varlik], oi: usize| -> f64 {
        let o = &sn[oi];
        match p {
            Some(_) => {
                if Some(oi) == kd {
                    let kO = kO.as_ref().unwrap();
                    let r = &ps0[o.r.unwrap()];
                    let gf = match &kO.f {
                        Some(f) => gec(f, cc.x, cc.y, None, k),
                        None => gec(o.r0(ps0), cc.x, cc.y, None, k),
                    };
                    jmax(0.0, ta - kO.t0 - 0.5 * lagOf(r, k) - gf) * vSp(r, k)
                } else {
                    let r0 = o.r0(ps0);
                    jmax(0.0, ta - tepki(r0, k) - gec(r0, cc.x, cc.y, None, k)) * vSp(r0, k)
                }
            }
            None => S.W.dlReach,
        }
    };
    let mResp = |sn: &[Varlik], oi: usize| -> Option<Vec<(f64, f64)>> {
        let pi = p?;
        let o = &sn[oi];
        if !refIs(o, pi) {
            return None;
        }
        let fr0 = if Some(oi) == kd { let k = kO.as_ref().unwrap(); (k.x, k.y) } else { (o.x, o.y) };
        let g = gside(cc.x, cc.y, 1 - att, S.markOff);
        let R0 = Rof(sn, oi);
        let d0 = hypot(g.0 - fr0.0, g.1 - fr0.1);
        Some(vec![if d0 <= R0 { g } else { (fr0.0 + (g.0 - fr0.0) / d0 * R0, fr0.1 + (g.1 - fr0.1) / d0 * R0) }])
    };
    let choices: Vec<Vec<(usize, (f64, f64))>> = rs
        .iter()
        .map(|&oi| {
            let v = match mResp(sn, oi) {
                Some(v) => v,
                None => {
                    let o0 = if Some(oi) == kd { let k = kO.as_ref().unwrap(); (k.x, k.y) } else { (sn[oi].x, sn[oi].y) };
                    coverSpots(o0, att, E, &[(cc.x, cc.y)], Rof(sn, oi))
                }
            };
            v.into_iter().map(|g| (oi, g)).collect()
        })
        .collect();
    let combos: Vec<Vec<(usize, (f64, f64))>> = if choices.len() == 1 {
        choices[0].iter().map(|k| vec![*k]).collect()
    } else if !choices.is_empty() {
        choices[0].iter().flat_map(|k| choices[1].iter().map(move |j| vec![*k, *j])).collect()
    } else {
        vec![vec![]]
    };
    let keep: Vec<(f64, f64)> = rs.iter().map(|&o| (sn[o].x, sn[o].y)).collect();
    let mut worst = 1e9;
    let H = c.m.holder.filter(|&h| c.ps[h].takim() == att);
    let hR = H.map(|h| {
        let mut r = c.ps[h].clone();
        r.x = E.0;
        r.y = E.1;
        r
    });
    let hOpt = |c: &mut Mc, w2: &[Varlik]| -> f64 {
        let ps = &*c.ps;
        let hR = hR.as_ref().unwrap();
        let Vh = yerV(w2, ps, att, E.0, E.1, ch);
        if d.recvKeepT != 0.0 {
            dis::keepP(hR, w2, ps, None, d.recvKeepT) * if d.useSpace { jmax(Vh, dis::runOn(w2, ps, att, hR, E)) } else { Vh }
        } else {
            d.recvKeepT
        }
    };
    let (mut hBase, mut myW) = (1e9, 1e9);
    if let Some(kp) = c.ek.kpC.as_mut() {
        kp.mut_ += 1;
    }
    for cmb in &combos {
        for (oi, g) in cmb {
            sn[*oi].x = g.0;
            sn[*oi].y = g.1;
        }
        if H.is_some() {
            let hv = hOpt(c, sn);
            let mv = myOpt(c, sn);
            hBase = jmin(hBase, hv);
            myW = jmin(myW, mv);
            worst = jmin(worst, jmax(mv, hv));
        } else {
            worst = jmin(worst, myOpt(c, sn));
        }
    }
    for (i, &o) in rs.iter().enumerate() {
        sn[o].x = keep[i].0;
        sn[o].y = keep[i].1;
    }
    if let Some(kp) = c.ek.kpC.as_mut() {
        kp.mut_ -= 1;
    }
    if H.is_some() && cc.taban {
        return worst;
    }
    if let (Some(h), Some(pi), false) = (H, p, cc.taban) {
        let key = (c.m.tick.to_bits(), c.ps[pi].id.to_bits(), c.ps[h].id.to_bits());
        let v = match c.ek.tbC {
            Some((kk, v)) if kk == key => v,
            _ => {
                let px = c.ps[pi].x;
                let py = c.ps[pi].y;
                let v = teamV(c, sn, svi, att, E, TvAday { x: px, y: py, kv: None, kd: None, taban: true }, mates, okP, p, pK);
                c.ek.tbC = Some((key, v));
                v
            }
        };
        worst = worst - v + if myW < 1e9 { S.tvEgim * myW } else { 0.0 };
    } else if H.is_some() {
        worst = jmax(0.0, worst - hBase) + if myW < 1e9 { S.tvEgim * myW } else { 0.0 };
    }
    let opened = if d.useSpace && okP > 0.0 {
        let ps = &*c.ps;
        let vv = jmax(dis::space(sn, ps, att, cc.x, cc.y), dis::front(sn, ps, att, cc.x, cc.y));
        yolP(sn, ps, att, E, (cc.x, cc.y), d) * dis::dagin(sn, ps, att, cc.x, vv, cc.y) * S.W.tvPass
    } else {
        0.0
    };
    jmax((1.0 - w) * myOpt(c, sn) + w * worst, crowd * opened)
}

/// JS kpOf(w2, c, q, yP, kA) (teamV içi): koşunun topu kaybetme riskine katkısı (`_kpC` kuruluysa).
fn kpOf(c: &mut Mc, w2: &[Varlik], svi: Pid, cc: (f64, f64), q: Option<Pid>, yP: f64, kA: f64) -> f64 {
    let q = match q {
        Some(q) => q,
        None => return 0.0,
    };
    match &c.ek.kpC {
        Some(kp) if kp.LE > 0.0 => {}
        _ => return 0.0,
    }
    let k = c.k;
    let d = c.d;
    let qp = &c.ps[q];
    let kL = hypot(cc.0 - qp.x, cc.1 - qp.y);
    let tK = gec(qp, cc.0, cc.1, Some(kL), k) + kL / vSp(qp, k);
    let R = d.recvKeepT;
    let ps = &*c.ps;
    let kp = c.ek.kpC.as_mut().unwrap();
    let mut kT = |T: f64| -> f64 {
        if kp.mut_ != 0 {
            return cl(dis::keepP(&kp.Hr, w2, ps, None, T), 0.0, 1.0);
        }
        let key = (svi, T.to_bits());
        if let Some(v) = kp.kc.get(&key) {
            return *v;
        }
        let v = cl(dis::keepP(&kp.Hr, w2, ps, None, T), 0.0, 1.0);
        kp.kc.insert(key, v);
        v
    };
    let k1 = kT(jmax(R, tK));
    if k1 < 0.001 || kT(100.0) > 0.98 {
        return 0.0;
    }
    let k2 = kT(jmax(R, tK) + R);
    kp.kH = k1;
    let LE = kp.LE;
    let ust = (1.0 - k2) * LE - ((1.0 - k1) * LE + k1 * ((1.0 - yP) * LE));
    let kat = k1 * yP * (1.0 - cl(kA, 0.0, 1.0));
    if ust <= 0.0 {
        0.0
    } else if kat <= 0.0 {
        ust
    } else {
        jmax(0.0, (1.0 - k2) * LE - ((1.0 - k1) * LE + k1 * ((1.0 - yP) * LE + yP * (1.0 - cl(kA, 0.0, 1.0)) * (kp.L)(cc.0, cc.1))))
    }
}

/// JS coverBoth(m, d, att, E, h): taşıyıcıya en yakın savunucunun iki seçeneği birden kapattığı yer.
/// (Bugünkü maçta çağrılmaz: `!ONIS()` dalında; bütünlük için taşındı.)
pub fn coverBoth(c: &mut Mc, di: Pid, att: u8, E: (f64, f64), h: Option<Pid>) -> Option<(f64, f64)> {
    let okD = yerS(&c.ps[di]);
    let w = cl((okD - 6.0) / 10.0, 0.0, 1.0);
    if w <= 0.0 {
        return None;
    }
    let sn = seen(c, di);
    let k = c.k;
    let d = c.d;
    let ps = &*c.ps;
    let atk: Vec<(f64, f64)> = ps.iter().enumerate().filter(|(i, q)| q.takim() == att && Some(*i) != h && !q.bekci() && hypot(q.tx - E.0, q.ty - E.1) < 22.0).map(|(_, q)| (q.x, q.y)).collect();
    if atk.is_empty() {
        return None;
    }
    let dp = &ps[di];
    let me = (dp.x, dp.y);
    let spots = coverSpots(me, att, E, &atk, S.W.defReach);
    let ch = if c.m.ch != 0.0 && c.m.ch == c.m.ch { c.m.ch } else { 0.3 };
    let mut w0: Vec<Varlik> = (*sn).clone();
    for (i, q) in ps.iter().enumerate() {
        if q.team == dp.team && i != di {
            w0.push(Varlik::oyuncu(i, q));
        }
    }
    let mut best = me;
    let mut bv = 1e9;
    for g in &spots {
        let mut w2 = w0.clone();
        w2.push(Varlik::kopya(di, dp, g.0, g.1, 0.0, 0.0));
        let v = teamBest(&w2, ps, att, E, &atk, ch, k, d) + 0.004 * hypot(g.0 - me.0, g.1 - me.1);
        if v < bv {
            bv = v;
            best = *g;
        }
    }
    let g0 = gside(E.0, E.1, 1 - att, 2.2);
    Some((g0.0 + (best.0 - g0.0) * w, g0.1 + (best.1 - g0.1) * w))
}

/// JS kapatHat(m, d, att, E, qo): markajcının hattı: adamının kale tarafı, adamına ve arkasındaki koşuya pas
/// yolunu en çok kapatan noktaya (yerleşim yeteneğiyle) kayar. 4 tik önbellekli (`d._kh`).
pub fn kapatHat(c: &mut Mc, di: Pid, att: u8, E: (f64, f64), qo: &Varlik) -> (f64, f64) {
    let okD = yerS(&c.ps[di]);
    let w = cl((okD - 6.0) / 10.0, 0.0, 1.0);
    let g0 = gside(qo.x, qo.y, 1 - att, S.markOff);
    if w <= 0.0 || c.m.holder.is_none() {
        return g0;
    }
    {
        let dp = &c.ps[di];
        if let (Some(kh), Some(t)) = (&dp._kh, dp._khT) {
            if t > c.m.tick - 4.0 && dp._khQ == qo.r {
                return (qo.x + kh.dx, qo.y + kh.dy);
            }
        }
    }
    let sn0 = seen(c, di);
    let mut sn: Vec<Varlik> = (*sn0).clone();
    let ch = if c.m.ch != 0.0 && c.m.ch == c.m.ch { c.m.ch } else { 0.3 };
    let gx = ownX(1 - att);
    let gl = {
        let l = hypot(gx - qo.x, GCY - qo.y);
        if l != 0.0 && l == l {
            l
        } else {
            1.0
        }
    };
    let run = (qo.x + (gx - qo.x) / gl * 7.0, qo.y + (GCY - qo.y) / gl * 7.0);
    let T = [(qo.x, qo.y), run];
    let mut spots = vec![g0];
    let dMx = 4.0 + 0.15 * gl;
    for t in [0.55, 0.72, 0.86] {
        let lx = E.0 + (qo.x - E.0) * t;
        let ly = E.1 + (qo.y - E.1) * t;
        for g in [(lx, ly), ((lx + g0.0) / 2.0, (ly + g0.1) / 2.0)] {
            if hypot(g.0 - qo.x, g.1 - qo.y) <= dMx {
                spots.push(g);
            }
        }
    }
    let d = c.d;
    let (dx0, dy0) = (c.ps[di].x, c.ps[di].y);
    // JS hayalet {x, y, a, team, role, ref: d}: R/D/press yok (8, 1, false), id yok.
    let mut gb = hayal(&c.ps[di]);
    gb.R = 0.0;
    gb.D = 1.0;
    gb.press = false;
    let gb = std::sync::Arc::new(gb);
    sn.push(Varlik { x: dx0, y: dy0, vx: 0.0, vy: 0.0, gvx: None, gvy: None, team: c.ps[di].takim(), bekci: c.ps[di].bekci(), press: false, id: f64::NAN, r: Some(di), kim: None, beden: Beden::Ayri(gb) });
    let gi = sn.len() - 1;
    let mut best = g0;
    let mut bv = 1e9;
    let ps = &*c.ps;
    for g in &spots {
        sn[gi].x = g.0;
        sn[gi].y = g.1;
        let mut v: f64 = 0.0;
        for P in T {
            v = jmax(v, yolP(&sn, ps, att, E, P, d) * yerV(&sn, ps, att, P.0, P.1, dis::recvCh(&sn, ps, att, P.0, P.1, ch, None)));
        }
        v += 0.004 * hypot(g.0 - dx0, g.1 - dy0);
        if v < bv {
            bv = v;
            best = *g;
        }
    }
    let r = (g0.0 + (best.0 - g0.0) * w, g0.1 + (best.1 - g0.1) * w);
    let dp = &mut c.ps[di];
    dp._kh = Some(crate::model::JP_kh { dx: r.0 - qo.x, dy: r.1 - qo.y });
    dp._khT = Some(c.m.tick);
    dp._khQ = qo.r;
    r
}

/// JS metrics(m) dönüşü.
#[derive(Clone, Copy, Debug)]
pub struct Metrik {
    pub swarm: u32,
}

/// JS metrics(m): topun 8 br yakınındaki (pres dışı) oyuncu sayısı.
pub fn metrics(c: &Mc) -> Metrik {
    let (bx, by) = match c.m.holder {
        Some(h) => (c.ps[h].x, c.ps[h].y),
        None => {
            let b = c.ball.as_ref().expect("metrics: top yok");
            (b.b.x, b.b.y)
        }
    };
    let mut n = 0;
    for (i, p) in c.ps.iter().enumerate() {
        if Some(i) != c.m.holder && !p.bekci() && !p.press && hypot(p.x - bx, p.y - by) < 8.0 {
            n += 1;
        }
    }
    Metrik { swarm: n }
}

#[allow(dead_code)]
fn _kullan(_: &Oyuncu) {}
