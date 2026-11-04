//! position'un hücum bölümü (güvence, kontra koşucuları, kanat, boşluk araması) ve son işler (yol kes,
//! geçilenler, Bekçi, yol ver, sınırlar).
#![allow(non_snake_case, clippy::needless_range_loop)]

use std::sync::Arc;

use crate::core_sabit::Q;
use crate::mat::{atan2, cos, exp, hypot, jmax, jmin, jsign, pow, sin, PI};
use crate::model::{JP_kv, JP_otE, Pid};
use crate::oyuncu::Tepki;
use crate::saha::{KR, MOUTH};
use crate::temas::{alan_ic, kuyu_girer};

use super::super::gecici_dis as dis;
use super::super::takim::{yerV, yolP, TvAday};
use super::super::*;
use super::savunma::Sv;
use super::yardim::*;

/// Hücumcu başına sabitler (JS: runV, roleY, leashK … kapanışlarının okudukları).
struct Hk {
    genisK: f64,
    kaleOnu: &'static str,
}

impl Sv {
    fn elOf(&self, p: Pid) -> f64 {
        match self.base[p].and_then(|b| b.el) {
            Some(e) if e == e => e,
            _ => 0.0,
        }
    }
    fn elDar(&self, c: &Mc, p: Pid) -> bool {
        self.elOf(p) > 0.5 && tl0(&c.ps[p], "huc", "serbest") < 0.5
    }
    fn genis(&self, c: &Mc, p: Pid) -> bool {
        rslotOf(&c.ps[p]).1.abs() >= 0.75 || self.kanatT[p].is_some()
    }
    fn yanOf(&self, c: &Mc, p: Pid) -> f64 {
        self.kanatT[p].unwrap_or_else(|| jsign(rslotOf(&c.ps[p]).1))
    }
    fn roleLeash(&self, c: &Mc, p: Pid) -> f64 {
        pow(2.0, 2.0 * (0.5 - tl0(&c.ps[p], "huc", "serbest")))
    }
    fn roleRun(&self, c: &Mc, p: Pid) -> f64 {
        if self.runners[p] {
            0.0
        } else {
            0.6 * S.runK * (tl0(&c.ps[p], "huc", "kosu") - 0.5) / 0.5
        }
    }
    fn vacL(&self, c: &Mc, p: Pid) -> f64 {
        if self.elOf(p) != 0.0 {
            1.0 + (S.vacLeash - 1.0) * jmin(1.0, 2.0 * tl0(&c.ps[p], "huc", "serbest"))
        } else {
            S.vacLeash
        }
    }
    fn leashK(&self, c: &Mc, p: Pid) -> f64 {
        (1.0 + S.durusEl * self.elOf(p)) * if self.win { if self.runners[p] { 0.01 } else { 2.5 } } else { self.roleLeash(c, p) }
    }
    fn dogL(&self, hk: &Hk) -> f64 {
        if hk.kaleOnu == "her" {
            0.0
        } else {
            S.vacLeash
        }
    }

    /// JS runV(p, c, sn): koşunun değeri (rakiplerden önce varılırsa tehdit).
    fn runV(&self, c: &Mc, p: Pid, a: &Aday, sn: &[Varlik]) -> f64 {
        let k = c.k;
        let pp = &c.ps[p];
        let ta = (match a.kv {
            Some(kv) => hypot(kv.0 - pp.x, kv.1 - pp.y) + hypot(a.x - kv.0, a.y - kv.1),
            None => hypot(a.x - pp.x, a.y - pp.y),
        }) / vSp(pp, k);
        let mut td: f64 = 1e9;
        for o in sn {
            if !o.bekci {
                let g = o.govde(c.ps);
                td = jmin(td, jmax(0.0, TEPKI(g, Tepki::Kovala, k)) + hypot(a.x - o.x, a.y - o.y) / vSp(g, k));
            }
        }
        dis::threat(self.att, a.x, a.y) / (1.0 + exp((ta - td) / 8.0))
    }
    /// JS blokDis(sn, y).
    fn blokDis(sn: &[Varlik], y: f64) -> f64 {
        let (mut lo, mut hi) = (1e9f64, -1e9f64);
        for o in sn {
            if !o.bekci {
                if o.y < lo {
                    lo = o.y;
                }
                if o.y > hi {
                    hi = o.y;
                }
            }
        }
        if lo > hi {
            0.0
        } else {
            jmax(jmax(0.0, lo - y), y - hi)
        }
    }
    /// JS roleY(p, c, sn).
    fn roleY(&self, c: &Mc, hk: &Hk, p: Pid, a: &Aday, sn: &[Varlik]) -> f64 {
        let ds = tl0(&c.ps[p], "huc", "destek");
        let E = self.E;
        (if self.genis(c, p) { hk.genisK * S.kanatK * jmin(jmax(0.0, 13.0 * GKY - (a.y - GCY).abs()) / (13.0 * GKY), 1.0 - cl(Self::blokDis(sn, a.y) / 8.0, 0.0, 1.0)) } else { 0.0 })
            + (if ds < 0.5 { jmin(1.5, (0.5 - ds) / 0.3) * S.kurucuK * jmax(0.0, (a.x - E.0) * dirOf(self.att) + 2.0) / 10.0 } else { 0.0 })
            + (if (ds - 0.5).abs() > 0.1 { S.destekK * jmin(4.0, pow((hypot(a.x - E.0, a.y - E.1) - (6.0 + 14.0 * ds)) / 8.0, 2.0)) } else { 0.0 })
    }

    /// Hücum: güvence, kontra koşucuları, kanat, boşluk araması.
    pub fn hucum(&mut self, c: &mut Mc) {
        let k = c.k;
        let tick = c.m.tick;
        let n = self.n;
        let (att, dt, E, h) = (self.att, self.dt, self.E, self.h);
        let hang0: Vec<Pid> = (0..n).filter(|&q| c.ps[q].takim() == dt && !c.ps[q].bekci() && (c.ps[q].x - E.0) * dirOf(att) < -8.0).collect();
        let G = c.m.gc.as_ref().filter(|g| g.team == att as f64 && g.git).cloned();
        {
            let risk = c.m.tac[att as usize].risk.unwrap_or(f64::NAN);
            let nG = jmax(0.0, if G.is_some() { jmin(hang0.len() as f64, if risk > 0.7 { 0.0 } else { 1.0 }) } else { hang0.len() as f64 - if risk > 0.7 { 1.0 } else { 0.0 } }) as usize;
            let mut pool: Vec<Pid> = self.atk.iter().copied().filter(|&q| !self.chase[q] && !G.as_ref().map(|g| g.on.contains(&c.ps[q].id)).unwrap_or(false) && rslotOf(&c.ps[q]).0 < 0.6).collect();
            pool.sort_by(|&u, &v| sira(rslotOf(&c.ps[u]).0 - rslotOf(&c.ps[v]).0));
            let mut hang: Vec<(Pid, f64)> = hang0.iter().map(|&q| (q, dis::threat(dt, c.ps[q].x, c.ps[q].y))).collect();
            hang.sort_by(|a, b| sira(b.1 - a.1));
            for &(hq, _) in hang.iter().take(nG) {
                let mut bi: Option<Pid> = None;
                let mut bd = 1e9;
                for &q in &pool {
                    if self.guard[q].is_some() {
                        continue;
                    }
                    let bq = self.base[q].unwrap_or(Yer { x: c.ps[q].x, y: c.ps[q].y, el: None });
                    let d = hypot(bq.x - c.ps[hq].x, bq.y - c.ps[hq].y) + 30.0 * rslotOf(&c.ps[q]).0 + 40.0 * match bq.el {
                        Some(e) if e == e => e,
                        _ => 0.0,
                    };
                    if d < bd {
                        bd = d;
                        bi = Some(q);
                    }
                }
                if let Some(bi) = bi {
                    self.guard[bi] = Some(hq);
                }
            }
        }
        self.win = G.is_some();
        if let Some(G) = &G {
            for &q in &self.atk {
                if G.on.contains(&c.ps[q].id) && !self.chase[q] && self.guard[q].is_none() {
                    self.runners[q] = true;
                }
            }
        }
        let hk = Hk {
            genisK: match c.m.tac.get(att as usize).and_then(|t| t.genislik.as_deref()) {
                Some("Geniş") => 1.3,
                Some("Dar") => 0.7,
                _ => 1.0,
            },
            kaleOnu: match c.m.tac.get(att as usize).and_then(|t| t.kaleOnu.as_deref()) {
                Some("girme") => "girme",
                Some("her") => "her",
                _ => "firsat",
            },
        };
        if !self.win {
            if let Some(hh) = h.filter(|&hh| c.ps[hh].takim() == att) {
                for sd in [-1.0f64, 1.0] {
                    if self.atk.iter().any(|&q| !c.ps[q].bekci() && rslotOf(&c.ps[q]).1 * sd >= 0.75) {
                        continue;
                    }
                    let wx = cl(E.0 + 10.0 * dirOf(att), 3.0, GW - 3.0);
                    let wy = if sd < 0.0 { 4.0 } else { GH - 4.0 };
                    let mut bq: Option<Pid> = None;
                    let mut bc = 1e9;
                    for &q in &self.atk {
                        let qp = &c.ps[q];
                        let el = self.base[q].and_then(|b| b.el).map(|e| e != 0.0 && e == e).unwrap_or(false);
                        if q == hh || qp.bekci() || self.chase[q] || self.guard[q].is_some() || self.runners[q] || rslotOf(qp).1 * sd <= 0.0 || el {
                            continue;
                        }
                        let cc = hypot(wx - qp.x, wy - qp.y);
                        if cc < bc {
                            bc = cc;
                            bq = Some(q);
                        }
                    }
                    if let Some(bq) = bq {
                        self.kanatT[bq] = Some(sd);
                        let ks = self.atk.iter().copied().find(|&q| !c.ps[q].bekci() && rslotOf(&c.ps[q]).1 * -sd >= 0.75);
                        let ev = match ks {
                            Some(q) => {
                                let ka = rslotOf(&c.ps[q]);
                                (ka.0, -ka.1)
                            }
                            None => (0.75, 0.85 * sd),
                        };
                        let b = slotBase0(c.m, c.ps, &c.ps[bq], Some(ev), E.0, E.1, true);
                        self.base[bq] = Some(Yer { x: b.0, y: b.1, el: None });
                    }
                }
            }
        }
        if let Some(hh) = h {
            if c.ps[hh].takim() == att {
                c.ps[hh].otx = None;
            }
        }
        c.ek.kpC = None;
        {
            let fr_first = self.fr.as_ref().and_then(|f| f.first);
            let Hc = match h {
                Some(hh) => if c.ps[hh].takim() == att { Some(hh) } else { None },
                None => fr_first.filter(|&f| c.ps[f].takim() == att && !c.ps[f].bekci()),
            };
            if !c.lite {
                if let Some(hc) = Hc {
                    let mut Hr = c.ps[hc].clone();
                    Hr.x = E.0;
                    Hr.y = E.1;
                    Hr.vx = 0.0;
                    Hr.vy = 0.0;
                    let key = format!("{}|{}|{}", (tick / 4.0).floor(), att, idStr(c.ps[hc].id));
                    let gec = matches!(&c.ek.kayL, Some((kk, _)) if *kk == key);
                    if !gec {
                        let ch = yada(c.m.ch, 0.3);
                        let f = dis::kayipYer(c.ps, att, ch, &c.ps[hc], c.m.tac.get(att as usize));
                        c.ek.kayL = Some((key, f));
                    }
                    let L = c.ek.kayL.as_ref().unwrap().1.clone();
                    let LE = L(E.0, E.1);
                    if LE * S.W.tvPass > 0.003 {
                        c.ek.kpC = Some(KpC { L, LE, Hr, kH: 0.0, mut_: 0, kc: Default::default() });
                    }
                }
            }
        }
        for &p in &self.atk.clone() {
            self.hucumcu(c, &hk, p);
        }
        let _ = (tick, k);
    }

    /// Bir hücumcunun işi (JS `for (const p of atk) {…}` gövdesi).
    fn hucumcu(&mut self, c: &mut Mc, hk: &Hk, p: Pid) {
        let k = c.k;
        let tick = c.m.tick;
        let (att, E, h) = (self.att, self.E, self.h);
        let lite = c.lite;
        if self.chase[p] {
            c.ps[p].otx = None;
            return;
        }
        if let Some(gq) = self.guard[p] {
            let g = gside(c.ps[gq].x, c.ps[gq].y, att, 3.0 + 0.2 * (20.0 - yerS(&c.ps[p])));
            let pp = &mut c.ps[p];
            pp.tx = g.0;
            pp.ty = g.1;
            pp.job = Some("güvence".into());
            pp.otx = None;
            return;
        }
        let every = jmax(8.0, jround(24.0 - yerF(&c.ps[p])));
        if lite && c.ps[p].otx.is_none() {
            let b = self.base[p].unwrap();
            let pp = &mut c.ps[p];
            pp.otx = Some(b.x);
            pp.oty = Some(b.y);
            pp._kv = None;
        }
        if self.win && c.ps[p]._winSeen != c.m.winT && !lite {
            c.ps[p]._winSeen = c.m.winT;
            c.ps[p].otx = None;
        }
        let fr_first = self.fr.as_ref().and_then(|f| f.first);
        if let Some(otx) = c.ps[p].otx {
            let oty = c.ps[p].oty.unwrap_or(f64::NAN);
            let pp = &c.ps[p];
            let kalabalik = !lite && self.atk.iter().any(|&q| q != p && Some(q) != h && !self.chase[q] && hypot(c.ps[q].tx - otx, c.ps[q].ty - oty) < S.ayr);
            let c2 = match h {
                Some(hh) if pp._otB.is_some() && !lite => {
                    let hb = BindH::of(&c.ps[hh], hh);
                    (otx - c.ps[hh].x) * dirOf(att) < S.bindOn || bindCak(pp, otx, oty, &hb, k) < S.bindYol
                }
                _ => false,
            };
            let c3 = match h {
                Some(hh) if !lite && c.ps[hh].takim() == att && hh != p => {
                    let hb = BindH::of(&c.ps[hh], hh);
                    bindCak(pp, otx, oty, &hb, k) < S.bindYol
                }
                _ => false,
            };
            let c4 = lite
                || (pp._otH == h
                    && (h.is_some() || (pp._otE.is_some() && pp._otF == fr_first && { let e = pp._otE.as_ref().unwrap(); hypot(E.0 - e.x, E.1 - e.y) < 8.0 }))
                    && hypot(otx - pp.x, oty - pp.y) > 1.5)
                || {
                    let r = (tick + pp.id) % every;
                    r != 0.0 && r == r
                };
            if !kalabalik && !c2 && !c3 && c4 {
                let runner = self.runners[p];
                let pp = &mut c.ps[p];
                pp.tx = otx;
                pp.ty = oty;
                if let Some(kv) = pp._kv.clone() {
                    if hypot(kv.x - pp.x, kv.y - pp.y) < 1.2 || tick > kv.t {
                        pp._kv = None;
                    } else {
                        pp.tx = kv.x;
                        pp.ty = kv.y;
                    }
                }
                pp.job = Some(if runner { "kontra koşusu".into() } else { pp._amacJob.clone().filter(|s| !s.is_empty()).unwrap_or_else(|| "boşluk".into()) });
                pp.sprint = runner || hypot(pp.tx - pp.x, pp.ty - pp.y) > S.W.runSprint;
                return;
            }
        }
        let bs = {
            let b0 = self.base[p].unwrap();
            match h {
                Some(hh) if c.ps[hh].takim() == att && hh != p => {
                    let hp = &c.ps[hh];
                    let ayr = if hp.bekci() { S.evAyrBek } else { S.evAyr };
                    let dx = b0.x - hp.x;
                    let dy = b0.y - hp.y;
                    let dd = hypot(dx, dy);
                    if dd >= ayr {
                        b0
                    } else {
                        let ux = if dd > 0.3 { dx / dd } else { dirOf(att) };
                        let uy = if dd > 0.3 { dy / dd } else { 0.0 };
                        Yer { x: cl(hp.x + ux * ayr, 3.0, GW - 3.0), y: cl(hp.y + uy * ayr, 3.0, GH - 3.0), el: b0.el }
                    }
                }
                _ => b0,
            }
        };
        let sn0 = seen(c, p);
        let mut sn: Vec<Varlik> = (*sn0).clone();
        let mut cands: Vec<Aday> = Vec::new();
        let pp = c.ps[p].clone();
        if let Some(otx) = pp.otx {
            cands.push(Aday::xy(otx, pp.oty.unwrap_or(f64::NAN)));
        }
        cands.push(Aday::xy(pp.x, pp.y));
        cands.push(Aday::xy(bs.x, bs.y));
        for kk in 0..10 {
            let a = kk as f64 * PI / 5.0;
            let ux = cos(a);
            let uy = sin(a);
            for r in [5.0, 10.0, 16.0, 24.0] {
                cands.push(Aday::xy(pp.x + ux * r, pp.y + uy * r));
            }
            for r in [8.0, 14.0, 20.0] {
                cands.push(Aday::xy(E.0 + ux * r, E.1 + uy * r));
            }
        }
        let lastD = sn.iter().filter(|o| !o.bekci).fold(0.0f64, |mx, o| jmax(mx, (o.x - ownX(att)) * dirOf(att)));
        let elDar = self.elDar(c, p);
        let kosu = tl0(&pp, "huc", "kosu");
        if !elDar && !pp.bekci() && kosu >= 0.25 {
            for yy in [pp.y, bs.y, 15.0 * GKY, 35.0 * GKY] {
                cands.push(Aday::xy(ownX(att) + dirOf(att) * jmin(GW - 6.0, lastD + 4.0), yy));
            }
        }
        let hAtt = h.filter(|&hh| c.ps[hh].takim() == att && hh != p);
        if !elDar && !pp.bekci() && hAtt.is_some() {
            let dr = dirOf(att);
            let dep = |o: &Varlik| (o.x - ownX(att)) * dr;
            let mut op: Vec<&Varlik> = sn.iter().filter(|o| !o.bekci).collect();
            op.sort_by(|u, v| sira(dep(v) - dep(u)));
            if op.len() >= 4 {
                let dL = (dep(op[0]) + dep(op[1])) / 2.0;
                let mL = (dep(op[2]) + dep(op[3])) / 2.0;
                if dL - mL >= 6.0 && (E.0 - ownX(att)) * dr < mL - 2.0 {
                    let mut ac: Vec<(f64, f64, f64)> = Vec::new();
                    for f in [0.4, 0.7] {
                        let xx = ownX(att) + dr * (mL + (dL - mL) * f);
                        let mut y = 4.0;
                        while y <= GH - 4.0 {
                            let mut nd: f64 = 1e9;
                            for o in &sn {
                                if !o.bekci {
                                    nd = jmin(nd, hypot(o.x - xx, o.y - y));
                                }
                            }
                            if nd >= 4.0 {
                                let lo = laneOpen(&sn, c.ps, att, E, (xx, y), None, k, c.d);
                                if lo >= 0.4 {
                                    ac.push((xx, y, lo * jmin(1.0, nd / 8.0)));
                                }
                            }
                            y += 4.0;
                        }
                    }
                    ac.sort_by(|a, b| sira(b.2 - a.2));
                    for a in ac.iter().take(3) {
                        cands.push(Aday { x: a.0, y: a.1, hat: true, ..Default::default() });
                    }
                }
            }
        }
        if hAtt.is_some() {
            let mut kd: Option<usize> = None;
            let mut kdd = 6.0;
            for (oi, o) in sn.iter().enumerate() {
                if o.bekci || o.r.is_none() || c.ps[o.r.unwrap()].tutQ != Some(p) {
                    continue;
                }
                let d = hypot(o.x - pp.x, o.y - pp.y);
                if d < kdd {
                    kdd = d;
                    kd = Some(oi);
                }
            }
            if let Some(kdi) = kd {
                let kdv = sn[kdi].clone();
                let bx = E.0 - pp.x;
                let by = E.1 - pp.y;
                let bl = nz1(hypot(bx, by));
                let gl = nz1(hypot(oppX(att) - kdv.x, GCY - kdv.y));
                let ux = (oppX(att) - kdv.x) / gl;
                let uy = (GCY - kdv.y) / gl;
                if bl > 9.0 {
                    let nx = -by / bl;
                    let ny = bx / bl;
                    for sd in [-1.0, 1.0] {
                        let kv = (cl(pp.x + bx / bl * 4.0 + nx * sd * 3.0, 3.0, GW - 3.0), cl(pp.y + by / bl * 4.0 + ny * sd * 3.0, 3.0, GH - 3.0));
                        cands.push(Aday { x: cl(kdv.x + 6.0 * ux - nx * sd * 5.0, 3.0, GW - 3.0), y: cl(kdv.y + 6.0 * uy - ny * sd * 5.0, 3.0, GH - 3.0), kv: Some(kv), kd: Some(kdi), ..Default::default() });
                    }
                }
                let ml = nz1(hypot(kdv.x - E.0, kdv.y - E.1));
                let mx = (kdv.x - E.0) / ml;
                let my = (kdv.y - E.1) / ml;
                for sd in [-1.0, 1.0] {
                    cands.push(Aday { x: cl(kdv.x + 2.5 * mx - my * sd * 4.0 + 1.5 * ux, 3.0, GW - 3.0), y: cl(kdv.y + 2.5 * my + mx * sd * 4.0 + 1.5 * uy, 3.0, GH - 3.0), kd: Some(kdi), kor: true, ..Default::default() });
                }
            }
        }
        {
            let nV = jmax(0.0, jround((yerF(&pp) - S.vacOk0) / S.vacOkStep)) as usize;
            if nV > 0 {
                let mut vac: Vec<(f64, f64, f64)> = Vec::new();
                for &d in &self.defs {
                    let bd = c.ps[d].past.as_ref().filter(|v| v.len() >= 3).map(|v| (v[0].x, v[0].y));
                    let sd = sn.iter().find(|o| o.r == Some(d));
                    let (bd, sd) = match (bd, sd) {
                        (Some(b), Some(s)) => (b, s),
                        _ => continue,
                    };
                    if hypot(sd.x - bd.0, sd.y - bd.1) < S.vacLeft {
                        continue;
                    }
                    let mut near: f64 = 1e9;
                    for o in &sn {
                        if !o.bekci {
                            near = jmin(near, hypot(o.x - bd.0, o.y - bd.1));
                        }
                    }
                    if !(near < S.vacEmpty) {
                        vac.push((bd.0, bd.1, dis::threat(att, bd.0, bd.1)));
                    }
                }
                vac.sort_by(|a, b| sira(b.2 - a.2));
                for v in vac.iter().take(nV) {
                    cands.push(Aday { x: v.0, y: v.1, vac: true, ..Default::default() });
                    cands.push(Aday { x: v.0 + 4.0 * dirOf(att), y: v.1, vac: true, ..Default::default() });
                }
            }
        }
        if self.genis(c, p) || (!pp.bekci() && !elDar) {
            let yy = if (if self.genis(c, p) { self.yanOf(c, p) } else { bs.y - GCY }) < 0.0 { 4.0 } else { GH - 4.0 };
            for dx in [-8.0, 0.0, 8.0, 16.0] {
                cands.push(Aday::xy(cl(bs.x + dirOf(att) * dx, 3.0, GW - 3.0), yy));
            }
        }
        let flq = c.m.fl.as_ref().and_then(|f| f.q);
        if h.is_none() && flq.map(|q| c.ps[q].takim() == att && q != p).unwrap_or(false) && self.fr.is_some() && fr_first == flq && !elDar {
            let B = flq.unwrap();
            let dr = dirOf(att);
            let ga = atan2(GCY - E.1, oppX(att) - E.0);
            let vg = c.ball.as_ref().map(|b| b.t.from == Some(p)).unwrap_or(false);
            for da in [-1.05, -0.5, 0.0, 0.5, 1.05] {
                for r in [10.0, 16.0, 22.0] {
                    let x = E.0 + cos(ga + da) * r;
                    let y = E.1 + sin(ga + da) * r;
                    if (x - pp.x) * dr < 2.0 || x < 2.0 || x > GW - 2.0 || y < 2.0 || y > GH - 2.0 {
                        continue;
                    }
                    cands.push(Aday { x, y, uc: Some(B), vg, ..Default::default() });
                }
            }
        }
        let bH: Option<Pid> = match h {
            Some(hh) => if c.ps[hh].takim() == att { Some(hh) } else { None },
            None => flq.filter(|&q| c.ps[q].takim() == att && self.fr.is_some() && fr_first == Some(q)),
        };
        if let Some(bh) = bH.filter(|&b| b != p && !c.ps[b].bekci()) {
            let hb = &c.ps[bh];
            let bAh = h == Some(bh);
            let hh = BindH { x: if bAh { hb.x } else { E.0 }, y: if bAh { hb.y } else { E.1 }, vx: if bAh { hb.vx } else { 0.0 }, vy: if bAh { hb.vy } else { 0.0 }, tx: if bAh { Some(hb.tx) } else { None }, ty: if bAh { Some(hb.ty) } else { None }, hz: hb.hz(), pid: bh };
            let dr = dirOf(att);
            let ileri = (hh.x - pp.x) * dr;
            let bt = dis::tl(&pp, "huc", "bindir");
            let tal = matches!(bt, Some(b) if b != 0.0 && b == b && hb.i == b);
            let side = if (hh.y - GCY).abs() > 4.0 { jsign(hh.y - GCY) } else { yada(jsign(pp.y - hh.y), 1.0) };
            let kosul = if tal { ileri > -5.0 && ileri < 40.0 && (hh.x - ownX(att)) * dr > 25.0 * GKX } else { !elDar && (hh.y - GCY).abs() > 8.0 * GKY && ileri > 3.0 && ileri < 25.0 && (pp.y - hh.y).abs() < 22.0 && (hh.x - ownX(att)) * dr > 35.0 * GKX };
            if kosul {
                let dis_ = jmax(1.5, jmin(7.0, if side > 0.0 { GH - 2.0 - hh.y } else { hh.y - 2.0 }));
                for f in [10.0, 16.0] {
                    let x = cl(hh.x + dr * f, 3.0, GW - 3.0);
                    for (y, ad) in [(hh.y + side * dis_, "dış"), (cl(hh.y - 7.0 * side, 3.0, GH - 3.0), "iç")] {
                        if bindCak(&pp, x, y, &hh, k) >= S.bindYol {
                            cands.push(Aday { x, y, bind: Some(ad), bh: Some(hh), bindT: tal, ..Default::default() });
                        }
                    }
                }
            }
        }
        let gx0 = (E.0 - oppX(att)).abs() < 40.0 * GKX;
        if (!elDar || hk.kaleOnu == "her") && gx0 && !pp.bekci() && kosu >= 0.25 && tl0(&pp, "huc", "destek") >= 0.3 {
            for r in [11.0, 19.0] {
                for an in [-0.8, -0.27, 0.27, 0.8] {
                    cands.push(Aday::xy(oppX(att) - dirOf(att) * r * cos(an), GCY + r * sin(an)));
                }
            }
        }
        if hk.kaleOnu != "girme" && hAtt.is_some() && !pp.bekci() && gx0 {
            let hh = hAtt.unwrap();
            let key = format!("{}|{}|{}", idStr(c.m.tick), att, idStr(c.ps[hh].id));
            let ak = match &c.ek.acikK {
                Some((kk, v)) if *kk == key => v.clone(),
                _ => {
                    let mut ak: Vec<(f64, f64, f64)> = Vec::new();
                    let gx = oppX(att);
                    let dr = dirOf(att);
                    for r in [11.0, 15.0, 19.0] {
                        for an in [-0.8, -0.4, 0.0, 0.4, 0.8] {
                            let P = (cl(gx - dr * r * cos(an), 2.0, GW - 2.0), cl(GCY + r * sin(an), 2.0, GH - 2.0));
                            if hypot(P.0 - gx, P.1 - GCY) < KR + 0.8 {
                                continue;
                            }
                            let yv = yolP(&sn, c.ps, att, E, P, c.d);
                            if yv > 0.3 {
                                ak.push((P.0, P.1, yv));
                            }
                        }
                    }
                    ak.sort_by(|a, b| sira(b.2 - a.2));
                    ak.truncate(4);
                    c.ek.acikK = Some((key, ak.clone()));
                    ak
                }
            };
            for P in ak {
                cands.push(Aday { x: P.0, y: P.1, dogal: true, ..Default::default() });
            }
        }
        if hk.kaleOnu != "girme" && hAtt.is_some() && !pp.bekci() && gx0 {
            let gx = oppX(att);
            let dr = dirOf(att);
            let yb = if E.1 >= GCY { 1.0 } else { -1.0 };
            let nok = |dy: f64, r: f64| {
                let xx = (jmax(0.0, r * r - dy * dy)).sqrt();
                Aday { x: cl(gx - dr * xx, 2.0, GW - 2.0), y: cl(GCY + dy, 2.0, GH - 2.0), dogal: true, ..Default::default() }
            };
            match &*pp.role {
                "Uç" => {
                    cands.push(nok(yb * MOUTH, KR + 1.0));
                    cands.push(nok(0.0, KR + 2.5));
                }
                "Kanat" => cands.push(nok(-yb * (MOUTH + 1.0), KR + 1.0)),
                "Orta" => cands.push(nok(0.0, KR + 9.0)),
                _ => {}
            }
        }
        let mateObjs: Vec<(f64, f64)> = self.atk.iter().copied().filter(|&q| q != p && !self.chase[q]).map(|q| (c.ps[q].tx, c.ps[q].ty)).collect();
        if let Some(hh) = hAtt {
            let hb = BindH::of(&c.ps[hh], hh);
            let iy: Vec<Aday> = cands.iter().filter(|a| a.x >= 2.0 && a.x <= GW - 2.0 && a.y >= 2.0 && a.y <= GH - 2.0 && bindCak(&pp, a.x, a.y, &hb, k) >= S.bindYol).cloned().collect();
            if !iy.is_empty() {
                cands = iy;
            }
        }
        let svi = p;
        let mut pre: Vec<(Aday, f64)> = Vec::new();
        for a in cands.iter().filter(|a| a.x >= 2.0 && a.x <= GW - 2.0 && a.y >= 2.0 && a.y <= GH - 2.0) {
            let lsh = jmax(0.0, hypot(a.x - bs.x, a.y - bs.y) - S.evBolge) / 12.0 * if a.dogal { self.dogL(hk) } else if a.vac || a.uc.is_some() || a.bind.is_some() || a.hat { self.vacL(c, p) * if a.bindT { 0.5 } else { 1.0 } } else { 1.0 };
            let rv = self.runV(c, p, a, &sn);
            let run = (if self.runners[p] { S.runK * rv } else { 0.0 }) + self.roleRun(c, p) * rv;
            let ry = self.roleY(c, hk, p, a, &sn);
            let tv = {
                let ta = TvAday { x: a.x, y: a.y, kv: a.kv, kd: a.kd, taban: false };
                match (a.kd, a.kv) {
                    (Some(kdi), Some(kv)) => {
                        let (x0, y0) = (sn[kdi].x, sn[kdi].y);
                        let g = gside(kv.0, kv.1, 1 - att, S.markOff);
                        sn[kdi].x = g.0;
                        sn[kdi].y = g.1;
                        if let Some(kp) = c.ek.kpC.as_mut() {
                            kp.mut_ += 1;
                        }
                        let v = teamV(c, &mut sn, svi, att, E, ta, &mateObjs, 0.0, Some(p), Some(p));
                        sn[kdi].x = x0;
                        sn[kdi].y = y0;
                        if let Some(kp) = c.ek.kpC.as_mut() {
                            kp.mut_ -= 1;
                        }
                        v
                    }
                    _ => teamV(c, &mut sn, svi, att, E, ta, &mateObjs, 0.0, Some(p), Some(p)),
                }
            };
            let kb = self.komb(c, p, a, &sn, hAtt);
            let q = run - ry + tv - S.W.leash * self.leashK(c, p) * lsh * lsh + kb;
            pre.push((a.clone(), q));
        }
        pre.sort_by(|a, b| sira(b.1 - a.1));
        {
            let deep = S.W.deep as usize;
            let kur: Vec<(Aday, f64)> = [true, false]
                .iter()
                .filter_map(|&kv| pre.iter().position(|it| (if kv { it.0.kv.is_some() } else { it.0.kor }) && it.0.kd.is_some()))
                .filter(|&i| i >= deep)
                .map(|i| pre[i].clone())
                .collect();
            pre.truncate(deep);
            pre.extend(kur);
        }
        let chC = yada(c.m.ch, 0.3);
        let KRf = self.KRf(c, p, &sn, hAtt);
        let mut best: Option<Aday> = None;
        let mut bv = -1e9;
        let mut bestCk: Option<(f64, Pid, Pid)> = None;
        let yH = yerH(&pp);
        for (a, _) in &pre {
            let crw = mateObjs.iter().fold(1.0f64, |acc, q| jmin(acc, hypot(q.0 - a.x, q.1 - a.y) / S.ayr));
            let lsh = jmax(0.0, hypot(a.x - bs.x, a.y - bs.y) - S.evBolge) / 12.0
                * if a.dogal {
                    1.0 - crw * (1.0 - self.dogL(hk))
                } else if a.vac || a.uc.is_some() || a.bind.is_some() || a.hat {
                    1.0 - crw * (1.0 - self.vacL(c, p) * if a.bindT { 0.5 } else { 1.0 })
                } else {
                    1.0
                };
            let ck = self.cekAmac(c, p, a, &sn, hAtt, chC);
            let kyF = if hAtt.is_some() { yolP(&sn, c.ps, att, E, (a.x, a.y), c.d) } else { 1.0 };
            let rv = self.runV(c, p, a, &sn);
            let run = (if self.runners[p] { S.runK * rv } else { 0.0 }) + self.roleRun(c, p) * rv;
            let kb = self.komb(c, p, a, &sn, hAtt);
            let ry = self.roleY(c, hk, p, a, &sn);
            let tv = teamV(c, &mut sn, svi, att, E, TvAday { x: a.x, y: a.y, kv: a.kv, kd: a.kd, taban: false }, &mateObjs, yH, Some(p), None);
            let v = crw * kb + crw * (1.0 - 0.0) * kyF * run - ry + S.W.tv * tv - S.W.leash * self.leashK(c, p) * lsh * lsh * (1.0 + 2.0 * 0.0) - 0.0 - match &KRf {
                Some(f) => f(a.x, a.y),
                None => 0.0,
            } + hsh(pp.id, jround(3.0 * a.x + 7.0 * a.y), (tick / 30.0).floor()) * (20.0 - yH) * S.W.noise * 2.0
                + match ck {
                    Some(ck) => crw * ck.0,
                    None => 0.0,
                };
            if v > bv {
                bv = v;
                best = Some(a.clone());
                bestCk = ck;
            }
        }
        let best = best.unwrap_or_else(|| Aday::xy(cl(pp.x, 2.0, GW - 2.0), cl(pp.y, 2.0, GH - 2.0)));
        let fr_first = self.fr.as_ref().and_then(|f| f.first);
        let genis = self.genis(c, p);
        let bdis = Self::blokDis(&sn, best.y);
        let pm = &mut c.ps[p];
        pm.otx = Some(best.x);
        pm.oty = Some(best.y);
        pm._otH = h;
        pm._otB = best.bh.map(|b| b.pid);
        pm._otE = Some(JP_otE { x: E.0, y: E.1 });
        pm._otF = if self.fr.is_some() { fr_first } else { None };
        pm.tx = best.x;
        pm.ty = best.y;
        pm._kv = best.kv.map(|kv| JP_kv { x: kv.0, y: kv.1, t: tick + 40.0 });
        if let Some(kv) = &pm._kv {
            pm.tx = kv.x;
            pm.ty = kv.y;
        }
        pm.job = Some(if best.vac {
            "açılan boşluk".into()
        } else {
            match &pm.rh {
                Some(rh) if !rh.is_empty() && &**rh != "Serbest" => rh.to_lowercase().into(),
                _ => "boşluk".into(),
            }
        });
        pm._amacJob = None;
        let ad = |c: &Mc, q: Pid| c.ps[q].name.clone();
        let yeni: Option<String> = if let Some(ck) = bestCk.filter(|ck| ck.0 >= S.cekEtiket) {
            Some(format!("açma · {} · {}", ad(c, ck.1), ad(c, ck.2)))
        } else if let Some(u) = best.uc {
            Some(format!("{}{}", if best.vg { "ver-kaç · " } else { "üçüncü adam · " }, ad(c, u)))
        } else if let Some(b) = best.bind {
            Some(format!("{}{}", if b == "dış" { "bindirme · " } else { "içeriden bindirme · " }, ad(c, best.bh.unwrap().pid)))
        } else if best.kd.is_some() && (best.kv.is_some() || best.kor) {
            let r = sn[best.kd.unwrap()].r.unwrap();
            Some(format!("{}{}", if best.kv.is_some() { "gel-dön · " } else { "kör taraf · " }, ad(c, r)))
        } else if best.hat {
            Some("hatlar arası".into())
        } else if genis && bdis > 2.0 {
            Some("genişlik".into())
        } else {
            None
        };
        let pm = &mut c.ps[p];
        if let Some(j) = yeni {
            pm.job = Some(j.into());
            pm._amacJob = pm.job.clone();
        }
        pm.sprint = hypot(pm.tx - pm.x, pm.ty - pm.y) > S.W.runSprint;
    }

    /// JS komb(c): üçüncü adam (uç) ve bindirme koşularının katkısı.
    fn komb(&self, c: &Mc, p: Pid, a: &Aday, sn: &[Varlik], _hAtt: Option<Pid>) -> f64 {
        let k = c.k;
        let pp = &c.ps[p];
        if let Some(u) = a.uc {
            let tB = self.fr.as_ref().map(|f| f.arr[u].t).unwrap_or(30.0);
            let d = hypot(a.x - pp.x, a.y - pp.y);
            let ta = gec(pp, a.x, a.y, Some(d), k) + d / vSp(pp, k);
            return if ta > tB + 20.0 { 0.0 } else { S.ucK * dis::laneOk0(sn, c.ps, self.att, self.E, (a.x, a.y)) };
        }
        if let (Some(_), Some(bh)) = (a.bind, a.bh) {
            if bindCak(pp, a.x, a.y, &bh, k) < S.bindYol {
                return 0.0;
            }
            let (mut nH, mut nC) = (0, 0);
            for o in sn {
                if !o.bekci {
                    if hypot(o.x - bh.x, o.y - bh.y) < 10.0 {
                        nH += 1;
                    }
                    if hypot(o.x - a.x, o.y - a.y) < 8.0 {
                        nC += 1;
                    }
                }
            }
            return if nC == 0 {
                S.bindK * (if a.bindT { 1.5 } else { 1.0 }) * if nH <= 1 { 1.0 } else if nH == 2 { 0.5 } else if a.bindT { 0.25 } else { 0.0 }
            } else {
                0.0
            };
        }
        0.0
    }

    /// JS cekAmac(c): koşu markajcısını çekip bir arkadaşın pas yolunu açıyor mu → (değer, arkadaş, çekilen).
    fn cekAmac(&self, c: &Mc, p: Pid, a: &Aday, sn: &[Varlik], hAtt: Option<Pid>, chC: f64) -> Option<(f64, Pid, Pid)> {
        let h = hAtt?;
        let (att, E) = (self.att, self.E);
        let pp = &c.ps[p];
        let mut d: Option<usize> = None;
        let mut dd = 1e9;
        for (oi, o) in sn.iter().enumerate() {
            if o.bekci || o.r.is_none() {
                continue;
            }
            let dist = hypot(o.x - pp.x, o.y - pp.y);
            if ((c.ps[o.r.unwrap()].tutQ == Some(p) && dist < 12.0) || dist < 5.0) && dist < dd {
                dd = dist;
                d = Some(oi);
            }
        }
        let mx = a.x - pp.x;
        let my = a.y - pp.y;
        if hypot(mx, my) < 3.0 {
            return None;
        }
        let mut gk: Option<(f64, f64)> = None;
        if d.is_none() && (E.0 - oppX(att)).abs() < 40.0 * GKX {
            let mut bz = 1e9;
            for (oi, o) in sn.iter().enumerate() {
                if o.bekci || o.team == att {
                    continue;
                }
                let dz = hypot(o.x - a.x, o.y - a.y);
                if dz < bz {
                    bz = dz;
                    d = Some(oi);
                }
            }
            if d.is_some() {
                gk = Some(gside(a.x, a.y, 1 - att, S.markOff));
            }
        }
        let di = d?;
        let mut sn2: Vec<Varlik> = sn.to_vec();
        match gk {
            Some(g) => {
                sn2[di].x = g.0;
                sn2[di].y = g.1;
            }
            None => {
                sn2[di].x += 0.8 * mx;
                sn2[di].y += 0.8 * my;
            }
        }
        let mut bg = 0.0;
        let mut bq: Option<Pid> = None;
        for &q in &self.atk {
            if q == p || q == h || c.ps[q].bekci() || self.chase[q] {
                continue;
            }
            let qp = &c.ps[q];
            let l0 = yolP(sn, c.ps, att, E, (qp.x, qp.y), c.d);
            let l1 = yolP(&sn2, c.ps, att, E, (qp.x, qp.y), c.d);
            if l1 - l0 <= 0.05 {
                continue;
            }
            let g = (l1 - l0) * yerV(&sn2, c.ps, att, qp.x, qp.y, chC);
            if g > bg {
                bg = g;
                bq = Some(q);
            }
        }
        let dref = sn[di].r?;
        bq.map(|q| (S.cekK * bg, q, dref))
    }

    /// JS KRf: topu kaybedersek rakibin kontrası; adayın onu kesebilmesi.
    fn KRf(&self, c: &Mc, p: Pid, sn: &[Varlik], hAtt: Option<Pid>) -> Option<Box<dyn Fn(f64, f64) -> f64>> {
        let k = c.k;
        let h = hAtt?;
        let att = self.att;
        let hp = &c.ps[h];
        let pk = 1.0 - cl(dis::keepP(hp, sn, c.ps, None, c.d.recvKeepT), 0.0, 1.0);
        if !(pk > 0.001) {
            return None;
        }
        let gO = ownX(att);
        let ch0 = yada(c.m.ch, 0.3);
        let our: Vec<(f64, f64, f64)> = self.atkAll.iter().copied().filter(|&q| q != p && !c.ps[q].bekci()).map(|q| if q == h { (hp.x, hp.y, vSp(hp, k)) } else { (c.ps[q].tx, c.ps[q].ty, vSp(&c.ps[q], k)) }).collect();
        let mut OP: Vec<((f64, f64), f64, f64, f64)> = Vec::new();
        for o in sn.iter().filter(|o| !o.bekci && o.team != att) {
            let gx = gO - o.x;
            let gy = GCY - o.y;
            let gl = nz1(hypot(gx, gy));
            let d = jmin(10.0, gl);
            let a = (o.x + gx / gl * d, o.y + gy / gl * d);
            let g = gside(a.0, a.1, att, 2.0);
            let tO = d / vSp(o.r0(c.ps), k);
            let V = jmax(0.0, yerV(sn, c.ps, 1 - att, a.0, a.1, ch0));
            let mut u0 = 1.0;
            for q in &our {
                u0 *= 1.0 - 1.0 / (1.0 + exp((hypot(g.0 - q.0, g.1 - q.1) / q.2 - tO) / 5.0));
            }
            if V > 0.0 {
                OP.push((g, tO, V, u0));
            }
        }
        let vp = vSp(&c.ps[p], k);
        let sk = S.W.tv * S.W.tvPass * pk;
        if OP.is_empty() {
            return None;
        }
        Some(Box::new(move |x: f64, y: f64| {
            let mut w: f64 = 0.0;
            for o in &OP {
                w = jmax(w, o.2 * o.3 * (1.0 - 1.0 / (1.0 + exp((hypot(o.0 .0 - x, o.0 .1 - y) / vp - o.1) / 5.0))));
            }
            sk * w
        }))
    }

    /// Son işler: yol kes, geçilenler, Bekçi, yol ver, sınırlar ve Kuyu.
    pub fn son(&mut self, c: &mut Mc) {
        let k = c.k;
        let tick = c.m.tick;
        let n = self.n;
        let (att, E, h) = (self.att, self.E, self.h);
        let loose = c.m.fl.is_none() || c.m.fl.as_ref().unwrap().q.is_none() || c.ball.as_ref().map(|b| b.b.defl != 0.0 || b.b.done.is_some()).unwrap_or(false);
        if h.is_none() && c.ball.is_some() && loose {
            if let Some(fr) = self.fr.clone() {
                let mut qO: Option<Pid> = None;
                let (mut eO, mut eB) = (1e9, 1e9);
                for (q, r) in fr.arr.iter().enumerate() {
                    let qp = &c.ps[q];
                    if qp.bekci() || qp.noTouch {
                        continue;
                    }
                    if qp.takim() == att {
                        eB = jmin(eB, r.est);
                    } else if r.est < eO {
                        eO = r.est;
                        qO = Some(q);
                    }
                }
                if let Some(qO) = qO.filter(|_| eB > eO + 6.0) {
                    let A = fr.arr[qO];
                    let mut al = vec![false; n];
                    let es: Vec<Pid> = (0..n).filter(|&o| { let op = &c.ps[o]; op.takim() != att && o != qO && !op.bekci() && !op.noTouch }).collect();
                    let mut ord: Vec<Pid> = self.atk.iter().copied().filter(|&p| !self.chase[p] && self.guard[p].is_none() && !c.ps[p].bekci()).collect();
                    ord.sort_by(|&u, &v| sira(hypot(c.ps[u].x - A.x, c.ps[u].y - A.y) - hypot(c.ps[v].x - A.x, c.ps[v].y - A.y)));
                    for p in ord {
                        let mut bo: Option<Pid> = None;
                        let mut bL = (0.0, 0.0);
                        let mut bd = 20.0;
                        for &o in &es {
                            let op = &c.ps[o];
                            if al[o] || hypot(op.x - A.x, op.y - A.y) < 6.0 {
                                continue;
                            }
                            let L = (A.x + 0.6 * (op.x - A.x), A.y + 0.6 * (op.y - A.y));
                            let d = hypot(L.0 - c.ps[p].x, L.1 - c.ps[p].y);
                            if d < bd {
                                bd = d;
                                bo = Some(o);
                                bL = L;
                            }
                        }
                        if let Some(bo) = bo {
                            al[bo] = true;
                            let nm = c.ps[bo].name.clone();
                            let pp = &mut c.ps[p];
                            pp.tx = bL.0;
                            pp.ty = bL.1;
                            pp.otx = None;
                            pp._kv = None;
                            pp.sprint = bd > 3.0;
                            let j = format!("yol kes · {}", nm);
                            pp.job = Some(j.into());
                            pp._amacJob = pp.job.clone();
                        }
                    }
                }
            }
        }
        for i in 0..n {
            if c.ps[i].noTouch && Some(i) != h {
                let g = gside(E.0, E.1, c.ps[i].takim(), 5.0);
                let pp = &mut c.ps[i];
                pp.tx = g.0;
                pp.ty = g.1;
                pp.sprint = true;
                pp.job = Some("geçildi · dönüyor".into());
            }
        }
        for i in 0..n {
            if !(c.ps[i].bekci() && !self.chase[i] && Some(i) != h) {
                continue;
            }
            let b = self.base[i].unwrap();
            let pp = &mut c.ps[i];
            pp.tx = b.x;
            pp.ty = b.y;
            pp.job = Some("Bekçi".into());
            let db = hypot(b.x - pp.x, b.y - pp.y);
            pp.sprint = db > 1.2;
            pp._yer = hypot(pp.vx, pp.vy) < 0.05 && db < 1.0;
            let gx0 = ownX(pp.takim());
            let ux = E.0 - gx0;
            let uy = E.1 - GCY;
            let L = nz1(hypot(ux, uy));
            let hatD = ((pp.x - gx0) * uy - (pp.y - GCY) * ux).abs() / L;
            if db > 3.0 && hatD > 1.5 {
                let tt = cl(((pp.x - gx0) * ux + (pp.y - GCY) * uy) / (L * L), 0.0, jmax(0.0, jmin((L - 2.0) / L, (hypot(b.x - gx0, b.y - GCY) + 2.0) / L)));
                pp.tx = gx0 + ux * tt;
                pp.ty = GCY + uy * tt;
                pp.job = Some("Bekçi · dönüş".into());
                pp.sprint = true;
            }
        }
        self.yolVer(c);
        for i in 0..n {
            let fq = c.m.fl.as_ref().and_then(|f| f.q);
            let kuyuTop = c.m._kuyuTop;
            let pp = &mut c.ps[i];
            let kx = if pp.bekci() { 1.0 } else { 1.5 };
            pp.tx = cl(pp.tx, kx, GW - kx);
            pp.ty = cl(pp.ty, 1.5, GH - 1.5);
            if !pp.bekci() {
                if let Some(q) = dis::kuyuSafe(pp.tx, pp.ty) {
                    pp.tx = q.0;
                    pp.ty = q.1;
                }
            }
            if !pp.bekci() && Some(i) != c.m.holder && (kuyuTop.is_none() || kuyuTop != Some(if pp.tx < GW / 2.0 { 0.0 } else { 1.0 })) {
                let v = jmax(0.05, spd(pp) * ALAN_SPRE(pp, k));
                let tv = hypot(pp.tx - pp.x, pp.ty - pp.y) / v;
                let cik = !(bas(&pp.job, &["kovala", "alıcı"]) || fq == Some(i));
                if alan_ic(pp.tx, pp.ty) && !kuyu_girer(pp, pp.tx, pp.ty, tv, cik, k) {
                    let gx = if hypot(pp.tx, pp.ty - GCY) < hypot(pp.tx - GW, pp.ty - GCY) { 0.0 } else { GW };
                    let dx = pp.tx - gx;
                    let dy = pp.ty - GCY;
                    let l = nz1(hypot(dx, dy));
                    let R = Q.kuyu_a + 0.5;
                    pp.tx = cl(gx + dx / l * R, 1.5, GW - 1.5);
                    pp.ty = cl(GCY + dy / l * R, 1.5, GH - 1.5);
                }
            }
        }
        let _ = (tick, att);
    }

    /// JS yol ver: pas/şut yolundaki takım arkadaşı yana çekilir.
    fn yolVer(&mut self, c: &mut Mc) {
        let hS = c.m.holder;
        let so = hS.and_then(|h| c.ps[h].savur.as_ref().map(|s| (s.o.clone(), s.t0)));
        if let (Some(h), Some((_, t0))) = (hS, &so) {
            c.ek.sh.yvBas = Some((h, *t0));
        }
        // b: savurmanın pası (yeni top) ya da maçın topu
        struct B {
            x: f64,
            y: f64,
            vx: f64,
            vy: f64,
            recv: Option<Pid>,
            gon: bool,
            team: u8,
            from: Option<Pid>,
            t: f64,
            done: bool,
            defl: bool,
            sap: bool,
        }
        let b: Option<B> = match (&so, hS) {
            (Some((o, _)), Some(h)) if ["pas", "önüne", "kenardan", "gönder"].contains(&o.kind.as_str()) => {
                let hp = &c.ps[h];
                Some(B { x: hp.x, y: hp.y, vx: o.launch.as_ref().map_or(f64::NAN, |l| l.vx), vy: o.launch.as_ref().map_or(f64::NAN, |l| l.vy), recv: if o.kind == "gönder" { None } else { o.q }, gon: o.kind == "gönder", team: hp.takim(), from: Some(h), t: 0.0, done: false, defl: false, sap: false })
            }
            _ => c.ball.as_ref().map(|b| B { x: b.b.x, y: b.b.y, vx: b.b.vx, vy: b.b.vy, recv: b.t.recv, gon: b.t.gon, team: b.b.team, from: b.t.from, t: b.b.t, done: b.b.done.is_some(), defl: b.b.defl != 0.0, sap: b.t.sap }),
        };
        let b = match b {
            Some(b) => b,
            None => return,
        };
        if !((c.m.holder.is_none() || b.from == c.m.holder) && !b.done && (b.recv.is_some() || b.gon) && !b.defl && !b.sap) {
            return;
        }
        let sp = hypot(b.vx, b.vy);
        if !(sp > 0.05) {
            return;
        }
        let ux = b.vx / sp;
        let uy = b.vy / sp;
        for i in 0..self.n {
            let pp = &c.ps[i];
            if pp.takim() != b.team || Some(i) == b.recv || pp.bekci() || pp.noTouch || (Some(i) == b.from && b.t < 4.0) {
                continue;
            }
            let ax = pp.x - b.x;
            let ay = pp.y - b.y;
            let al = ax * ux + ay * uy;
            if al <= -1.0 || al / sp > 40.0 {
                continue;
            }
            let pe = -ax * uy + ay * ux;
            let latA = pe + (-pp.vx * uy + pp.vy * ux) * jmax(0.0, al) / sp;
            if pe.abs() > 2.0 && latA.abs() > 2.0 {
                continue;
            }
            let sg = if latA.abs() > 0.05 {
                jsign(latA)
            } else if pe.abs() > 0.05 {
                jsign(pe)
            } else if (pp.tx - pp.x) * -uy + (pp.ty - pp.y) * ux >= 0.0 {
                1.0
            } else {
                -1.0
            };
            let lt = sg * jmax(pe.abs(), 2.2);
            let ga = jmax(0.0, al);
            let pm = &mut c.ps[i];
            pm.tx = b.x + ux * ga - uy * lt;
            pm.ty = b.y + uy * ga + ux * lt;
            pm.sprint = true;
            pm.job = Some("yol ver".into());
        }
    }
}

#[allow(dead_code)]
fn _kullan() -> Arc<()> {
    Arc::new(())
}
