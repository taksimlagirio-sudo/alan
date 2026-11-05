//! position: bölge markajı, eşleşme işi (UA: kişi / taşıyıcı / bölge atamaları), yedek işler, takım düzeltmeleri.
#![allow(non_snake_case, clippy::needless_range_loop)]

use crate::core_sabit::Q;
use crate::mat::{exp, hypot, jmax, jmin};
use crate::model::Pid;
use crate::oyuncu::Tepki;
use crate::yetenek::ah;

use super::super::gecici_dis as dis;
use super::super::*;
use super::savunma::{dal_sinir, Sv};
use super::yardim::*;

impl Sv {
    /// JS icOmuz(q, T, off, r): adamın kale tarafı, koşusuyla T tik ileri (kenara açılan koşu yarım sayılır).
    pub fn icOmuz(&self, qx: f64, qy: f64, vx: f64, vy: f64, T: f64, off: f64) -> (f64, f64) {
        let gx = ownX(self.dt);
        let ux0 = gx - qx;
        let uy0 = GCY - qy;
        let ul = nz1(hypot(ux0, uy0));
        let ux = ux0 / ul;
        let uy = uy0 / ul;
        let vu = vx * ux + vy * uy;
        let lx = (vx - vu * ux) * T;
        let ly = (vy - vu * uy) * T;
        let dis = (qy + ly - GCY).abs() > (qy - GCY).abs();
        let w = if dis { if ul < 25.0 * GKX { 0.5 } else { 0.0 } } else { 1.0 };
        gside(qx + ux * jmax(0.0, vu) * T + lx * w, qy + uy * jmax(0.0, vu) * T + ly * w, self.dt, off)
    }
    /// JS yakin(q, g): hedef adamdan en çok 4 + 0,15·gD uzakta.
    pub fn yakin(&self, qx: f64, qy: f64, g: (f64, f64)) -> (f64, f64) {
        let dg = hypot(g.0 - qx, g.1 - qy);
        let dMx = 4.0 + 0.15 * self.gD(qx, qy);
        if dg > dMx {
            (qx + (g.0 - qx) / dg * dMx, qy + (g.1 - qy) / dg * dMx)
        } else {
            g
        }
    }

    /// Bölge markajı (JS zonePrev ve en yakın boştaki savunucu).
    pub fn bolge(&mut self, c: &mut Mc) {
        let att = self.att;
        let n = self.n;
        let mut markedRef = vec![false; n];
        for p in 0..n {
            if let Some(q) = self.mark[p] {
                markedRef[q] = true;
            }
            if let Some(Some(q)) = self.swT[p] {
                markedRef[q] = true;
            }
        }
        let mut zTak = vec![false; n];
        for &p in &self.free.clone() {
            let q = match c.ps[p].hal.zonePrev {
                Some(q) => q,
                None => continue,
            };
            if self.taken[p] || markedRef[q] || zTak[q] || !self.atk.contains(&q) {
                continue;
            }
            let sv = seen(c, p);
            let sn = sv.iter().find(|o| o.r == Some(q));
            let bs = self.base[p].unwrap();
            let dr = if self.dMax > 0.0 { jmin(1.0, self.dng(c, q) / self.dMax) } else { 0.0 };
            if let Some(sn) = sn {
                if hypot(sn.x - bs.x, sn.y - bs.y) < 1.5 * (S.zoneR + yerS(&c.ps[p]) * S.zoneOk + S.zoneDng * dr) {
                    self.taken[p] = true;
                    self.mark[p] = Some(q);
                    zTak[q] = true;
                }
            }
        }
        let mut aq: Vec<(Pid, f64)> = self.atk.iter().map(|&q| (q, dis::threat(att, c.ps[q].x, c.ps[q].y))).collect();
        aq.sort_by(|a, b| sira(b.1 - a.1));
        for (q, _) in aq {
            if markedRef[q] || zTak[q] {
                continue;
            }
            let dr = if self.dMax > 0.0 { jmin(1.0, self.dng(c, q) / self.dMax) } else { 0.0 };
            let mut bp: Option<Pid> = None;
            let mut bd = 1e9;
            for &p in &self.free.clone() {
                if self.taken[p] {
                    continue;
                }
                let sv = seen(c, p);
                let sn0 = sv.iter().find(|o| o.r == Some(q)).expect("bölge: görülmeyen rakip");
                let sn = ileriV(sn0, okuTp(&c.ps[p]), Some(&c.ps[q]), c.ps, self.sg);
                let bs = self.base[p].unwrap();
                let d = hypot(sn.0 - bs.x, sn.1 - bs.y);
                if d < S.zoneR + yerS(&c.ps[p]) * S.zoneOk + S.zoneDng * dr && d < bd {
                    bd = d;
                    bp = Some(p);
                }
            }
            if let Some(bp) = bp {
                self.taken[bp] = true;
                self.mark[bp] = Some(q);
            }
        }
        for &p in &self.free {
            c.ps[p].hal_mut().zonePrev = self.mark[p];
        }
    }

    /// JS UA: savunucuların eşleşme işi (kişi, taşıyıcıya pres, bölge) ortak atamayla.
    pub fn esles(&mut self, c: &mut Mc) {
        let k = c.k;
        let tick = c.m.tick;
        let (att, dt, E, h) = (self.att, self.dt, self.E, self.h);
        let n = self.n;
        let karsila: Vec<bool> = (0..n).map(|p| h.is_some() && self.sweep[p] && self.swTg(p) == h).collect();
        let mesgul = |s: &Sv, p: Pid| s.press.contains(&p) || s.up[p] || karsila[p];
        let pool: Vec<Pid> = self.defs.iter().copied().filter(|&p| !mesgul(self, p)).collect();
        let flq = c.m.fl.as_ref().and_then(|f| f.q);
        let alici = match (h, flq, self.bul) {
            (None, Some(q), Some(_)) if c.ps[q].takim() == att => Some(q),
            _ => None,
        };
        let kovalar = (0..n).any(|p| self.chase[p] && c.ps[p].takim() == dt);
        let yolu = |qa: (f64, f64)| {
            let dx = E.0 - qa.0;
            let dy = E.1 - qa.1;
            let L = nz1(hypot(dx, dy));
            let d = jmin(jmax(3.0, 0.35 * L), jmax(1.0, L - 2.0));
            (qa.0 + dx / L * d, qa.1 + dy / L * d)
        };
        let mut isler: Vec<Is> = Vec::new();
        let gEb = self.gD(self.Eb.0, self.Eb.1);
        for &q in &self.atk.clone() {
            let qp = c.ps[q].clone();
            if Some(q) == h || h.map(|hh| qp.id == c.ps[hh].id).unwrap_or(false) {
                continue;
            }
            let w = self.dng(c, q);
            if Some(q) == alici {
                if !kovalar {
                    let b = self.bul.unwrap();
                    isler.push(Is { q: IsQ::P(q), w, g: (b.1, b.2), job: "pres · buluşma".into(), derin: self.gD(b.1, b.2) < 30.0, bul: true, okuF: false, grp: None, car: None, zone: None, rq: vec![], thr: 0.0 });
                }
                continue;
            }
            let qa = ileriP(&qp, self.Tt);
            let iz = ileriP(&qp, jmax(30.0, self.Tt + self.TA));
            let derin = self.gD(iz.0, iz.1) < gEb + 2.0;
            let omuz = self.yakin(qp.x, qp.y, self.icOmuz(qp.x, qp.y, qp.vx, qp.vy, self.Tt + self.TA, S.markOff));
            isler.push(Is {
                q: IsQ::P(q),
                w: if derin { w } else { w * (1.0 - self.acik) },
                g: if derin { omuz } else { yolu(qa) },
                job: format!("{}{}", if derin { "arkada · " } else { "kapatma · " }, qp.govde.name),
                derin,
                bul: false,
                okuF: derin,
                grp: None,
                car: None,
                zone: None,
                rq: vec![],
                thr: 0.0,
            });
            if !derin {
                isler.last_mut().unwrap().grp = Some(qp.id);
                isler.push(Is { q: IsQ::P(q), w, g: omuz, job: format!("arkada · {}", qp.govde.name), derin: true, bul: false, okuF: true, grp: Some(qp.id), car: None, zone: None, rq: vec![], thr: 0.0 });
            }
        }
        isler.sort_by(|a, b| sira(b.w - a.w));
        let mut out: Vec<Option<Ua>> = vec![None; n];
        let aFs = c.d.arriveFirm;
        let SHs = jmax(1.0, yada(c.d.sutHazir, 1.0));
        let pr2 = matches!(h, Some(hh) if c.ps[hh].takim() == att) && self.PRS.is_some();
        let YR = self.YR;
        let mut Js: Vec<Is> = isler.into_iter().filter(|J| !(YR && !J.derin && !J.bul)).collect();
        let Ps: Vec<Pid> = if pr2 {
            let P = self.PRS.as_ref().unwrap();
            self.defs.iter().copied().filter(|&p| (!self.up[p] || self.upBas(c, p)) && !P.fixed.contains(&p) && (!self.press.contains(&p) || P.cand[p])).collect()
        } else {
            pool.clone()
        };
        if pr2 {
            let P = self.PRS.as_ref().unwrap();
            let hh = h.unwrap();
            let nf = P.fixed.len();
            let kac = P.pLim.saturating_sub(nf);
            for kk in 0..kac {
                let car = kk + nf;
                let w0 = match self.tP.as_ref().and_then(|t| t.h) {
                    Some(th) => th * P.K,
                    None => P.G,
                };
                Js.push(Is { q: IsQ::P(hh), w: w0 * if car == 0 { 1.0 } else { 1.0 - P.P0 }, g: (c.ps[hh].x, c.ps[hh].y), job: String::new(), derin: false, bul: false, okuF: false, grp: None, car: Some(car), zone: None, rq: vec![], thr: 0.0 });
            }
        }
        {
            let AQ: Vec<Pid> = self.atk.iter().copied().filter(|&q| { let qp = &c.ps[q]; !(qp.noTouch || qp.bekci() || h.map(|hh| qp.id == c.ps[hh].id).unwrap_or(false)) }).collect();
            for (kk, &p) in Ps.iter().enumerate() {
                let Z = match self.base[p] {
                    Some(z) => z,
                    None => continue,
                };
                let TZ = dis::ucusT(&*c.m, hypot(Z.x - E.0, Z.y - E.1), aFs, None) + SHs;
                let mut r: f64 = 0.0;
                let mut rq = Vec::new();
                for &q in &AQ {
                    let qp = &c.ps[q];
                    let rr = 1.0 / (1.0 + exp((hypot(qp.x - Z.x, qp.y - Z.y) / vSp(qp, k) - TZ) / 5.0));
                    rq.push((q, rr));
                    r = jmax(r, rr);
                }
                let thr = jmax(0.0, dis::threat(att, Z.x, Z.y)) * exp(-TZ / 60.0);
                let w = thr * r;
                if w > 0.0 {
                    Js.push(Is { q: IsQ::Bolge { id: -200.0 - kk as f64, x: Z.x, y: Z.y }, w, g: (Z.x, Z.y), job: "bölge".into(), derin: true, bul: false, okuF: false, grp: None, car: None, zone: Some(p), rq, thr });
                }
            }
        }
        let qxy = |J: &Is| -> (f64, f64) {
            match J.q {
                IsQ::P(q) => (c.ps[q].x, c.ps[q].y),
                IsQ::Bolge { x, y, .. } => (x, y),
            }
        };
        let qid = |J: &Is| -> f64 {
            match J.q {
                IsQ::P(q) => c.ps[q].id,
                IsQ::Bolge { id, .. } => id,
            }
        };
        let TW: Vec<f64> = Js
            .iter()
            .map(|J| {
                let b = if J.bul { self.bul.map(|b| b.3) } else { None };
                b.unwrap_or_else(|| { let q = qxy(J); dis::ucusT(&*c.m, hypot(q.0 - E.0, q.1 - E.1), aFs, None) }) + SHs
            })
            .collect();
        {
            let U: Vec<Pid> = (0..n).filter(|&u| self.up[u] && !c.ps[u].noTouch && !c.ps[u].bekci()).collect();
            if !U.is_empty() {
                for (j, J) in Js.iter_mut().enumerate() {
                    if J.car.is_some() {
                        continue;
                    }
                    let mut kk: f64 = 0.0;
                    for &u in &U {
                        let up = &c.ps[u];
                        let d = hypot(up.x - J.g.0, up.y - J.g.1);
                        let t = d / vSp(up, k) + gec(up, J.g.0, J.g.1, Some(d), k);
                        kk = jmax(kk, 1.0 / (1.0 + exp((t - TW[j]) / 5.0)));
                    }
                    J.w *= 1.0 - kk;
                }
            }
        }
        let kuyuVar = |p: Pid, G: (f64, f64), t: f64, v: f64| -> f64 {
            let pp = &c.ps[p];
            if pp.bekci() || c.m._kuyuTop == Some(if pp.x < GW / 2.0 { 0.0 } else { 1.0 }) {
                return t;
            }
            let cx = ownX(dt);
            let R = Q.kuyu_a;
            let dP = hypot(pp.x - cx, pp.y - GCY);
            if dP >= R {
                return t;
            }
            let kal = Q.kuyu_sn - pp.hal.kuyuT.filter(|x| *x == *x).unwrap_or(0.0);
            let dx = G.0 - pp.x;
            let dy = G.1 - pp.y;
            let L = { let l = hypot(dx, dy); if l != 0.0 && l == l { l } else { 1e-6 } };
            let ux = dx / L;
            let uy = dy / L;
            let bx = pp.x - cx;
            let by = pp.y - GCY;
            let bb = bx * ux + by * uy;
            let sL = -bb + (jmax(0.0, bb * bb - (bx * bx + by * by - R * R))).sqrt();
            let tCik = jmin(L, sL) / v + 1.0 / (0.18 * ah(&pp.govde.yet));
            if tCik <= kal {
                return t;
            }
            let rx = bx / nz1(dP);
            let ry = by / nz1(dP);
            let X = (cx + rx * (R + 0.3), GCY + ry * (R + 0.3));
            (R - dP + 0.3) / v + 1.0 / (0.18 * ah(&pp.govde.yet)) + hypot(G.0 - X.0, G.1 - X.1) / v
        };
        let tur = |J: &Is| -> u8 {
            if J.zone.is_some() {
                2
            } else if J.car.is_some() {
                1
            } else {
                0
            }
        };
        let mut GG: Vec<Vec<Option<(f64, f64)>>> = vec![vec![None; Js.len()]; Ps.len()];
        let mut Bs: Vec<Vec<f64>> = Vec::with_capacity(Ps.len());
        for (i, &p) in Ps.iter().enumerate() {
            let pp = &c.ps[p];
            let v = vSp(pp, k);
            let rc = jmax(0.0, TEPKI(pp, Tepki::Kovala, k));
            let mut row = Vec::with_capacity(Js.len());
            for (j, J) in Js.iter().enumerate() {
                let tr = tur(J);
                if self.up[p] && tr != 1 {
                    row.push(0.0);
                    continue;
                }
                if tr == 0 && self.sweep[p] && !J.derin {
                    row.push(0.0);
                    continue;
                }
                let val = match tr {
                    2 => {
                        if J.zone == Some(p) {
                            J.w * (1.0 / (1.0 + exp((hypot(pp.x - J.g.0, pp.y - J.g.1) / vSp(pp, k) - TW[j]) / 5.0)))
                        } else {
                            0.0
                        }
                    }
                    1 => {
                        let P = self.PRS.as_ref().unwrap();
                        let e = P.pv[p];
                        if e.is_none() && !self.sweep[p] {
                            0.0
                        } else {
                            let hp = &c.ps[h.unwrap()];
                            let PP = match e {
                                Some(e) => e.P,
                                None => 1.0 / (1.0 + exp((jmax(0.0, TEPKI(pp, Tepki::Kovala, k)) + jmax(0.0, hypot(pp.x - hp.x, pp.y - hp.y) - 2.4) / v - 8.0) / 8.0)),
                            };
                            match &self.tP {
                                // değerin tek anlamı: bu oyuncu yapmazsa kaybedilen (kafanın "o olmasa"sı ya da işin
                                // kendi ağırlığı, hangisi büyükse; eksi değil) × oraya zamanında yetişme
                                Some(t) if !uadeg_eski() => jmax(yada(t.hm_get(pp.id).unwrap_or(0.0), 0.0) * P.K, jmax(0.0, J.w)) * PP,
                                Some(t) => J.w * PP + yada(t.hm_get(pp.id).unwrap_or(0.0), 0.0) * P.K,
                                None => J.w * PP,
                            }
                        }
                    }
                    _ => {
                        let q = J.qp().unwrap();
                        let qp = &c.ps[q];
                        let Gp = if J.okuF { self.yakin(qp.x, qp.y, self.icOmuz(qp.x, qp.y, qp.vx, qp.vy, okuTp(pp) + 0.0, S.markOff)) } else { J.g };
                        GG[i][j] = Some(Gp);
                        let d = hypot(pp.x - Gp.0, pp.y - Gp.1);
                        let t = kuyuVar(p, Gp, d / v + gec(pp, Gp.0, Gp.1, Some(d), k), v) - if pp.hal.uaPrev == Some(q) || pp.hal.markRef == Some(q) { rc } else { 0.0 };
                        let kp = 1.0 / (1.0 + exp((t - TW[j]) / 5.0));
                        let mg = match &self.tP {
                            Some(tp) if qp.id >= 0.0 && !J.bul => yada(tp.mm_get(&format!("{}|{}", idStr(qp.id), idStr(pp.id))).unwrap_or(0.0), 0.0),
                            _ => 0.0,
                        };
                        if self.tP.is_some() && !uadeg_eski() {
                            jmax(mg, jmax(0.0, J.w)) * kp
                        } else if kp > 0.001 || mg > 0.0 {
                            J.w * kp + mg
                        } else {
                            0.0
                        }
                    }
                };
                row.push(val);
            }
            Bs.push(row);
        }
        let GI: Vec<usize> = Js
            .iter()
            .enumerate()
            .map(|(j, J)| match (J.grp, J.car) {
                (Some(g), None) => Js.iter().position(|K| K.grp == Some(g) && K.car.is_none()).unwrap(),
                _ => j,
            })
            .collect();
        let mut bst = dal_sinir(&Bs, &GI);
        if bst.is_some() && Js.iter().any(|J| J.zone.is_some()) {
            let b = bst.as_ref().unwrap();
            let mut cov: Vec<(f64, f64)> = Vec::new();
            for (i, _) in Ps.iter().enumerate() {
                let j = b[i];
                if j < 0 {
                    continue;
                }
                let J = &Js[j as usize];
                if J.zone.is_some() || !(J.w > 0.0) {
                    continue;
                }
                let kq = Bs[i][j as usize] / J.w;
                let id = qid(J);
                match cov.iter_mut().find(|x| x.0 == id) {
                    Some(x) => {
                        if !(x.1 >= kq) {
                            x.1 = kq;
                        }
                    }
                    None => cov.push((id, kq)),
                }
            }
            let mut deg = false;
            for j in 0..Js.len() {
                if Js[j].zone.is_none() {
                    continue;
                }
                let mut nw: f64 = 0.0;
                for &(q, rr) in &Js[j].rq {
                    let cv = cov.iter().find(|x| x.0 == c.ps[q].id).map(|x| x.1).filter(|v| *v != 0.0 && *v == *v).unwrap_or(0.0);
                    nw = jmax(nw, rr * (1.0 - jmin(1.0, cv)));
                }
                nw *= Js[j].thr;
                let f = if Js[j].w > 0.0 { nw / Js[j].w } else { 0.0 };
                if (f - 1.0).abs() > 1e-6 {
                    deg = true;
                    Js[j].w = nw;
                    for i in 0..Ps.len() {
                        Bs[i][j] *= f;
                    }
                }
            }
            if deg {
                bst = dal_sinir(&Bs, &GI);
            }
        }
        if crate::iz_acik() && !c.lite && std::env::var("ALAN_UA_IZ").ok().and_then(|x| x.parse::<f64>().ok()) == Some(c.m.tick) {
            if let Some(t) = &self.tP { eprintln!("TP {:?}", t.mm.iter().map(|(k, v)| format!("{}={:.4}", k, v)).collect::<Vec<_>>()); }
            eprintln!("UA dt {} Ps {} bst {:?}", dt, Ps.iter().map(|&p| c.ps[p].govde.name.clone()).collect::<Vec<_>>().join(","), bst);
            for (j, J) in Js.iter().enumerate() {
                let qn = match J.q { IsQ::P(q) => c.ps[q].govde.name.clone(), _ => "bölge".into() };
                eprintln!(" J {} {} w {} g {} {} derin {} bul {} car {:?} zone {:?} job {}", j, qn, J.w, J.g.0, J.g.1, J.derin, J.bul, J.car, J.zone.map(|z| c.ps[z].govde.name.clone()), J.job);
            }
            for (i, r) in Bs.iter().enumerate() {
                eprintln!(" B {} {}", c.ps[Ps[i]].govde.name, r.iter().map(|v| format!("{:.5}", v)).collect::<Vec<_>>().join(" "));
            }
        }
        for &p in &Ps {
            if self.adamT(c, p) {
                self.mark[p] = None;
            }
        }
        let mut yeniP: Option<Vec<Pid>> = if pr2 { Some(self.PRS.as_ref().unwrap().fixed.clone()) } else { None };
        if let Some(b) = &bst {
            for (i, &p) in Ps.iter().enumerate() {
                let j = b[i];
                if j < 0 || Js[j as usize].zone.is_some() {
                    continue;
                }
                let J = &Js[j as usize];
                c.ps[p].hal_mut()._savT = Some(tick);
                if J.derin {
                    c.ps[p].hal_mut()._savD = Some(tick);
                }
                if J.car.is_some() {
                    c.ps[p].hal_mut()._savCar = Some(tick);
                    yeniP.as_mut().unwrap().push(p);
                    continue;
                }
                let q = J.qp().unwrap();
                if self.adamT(c, p) && !J.bul {
                    self.mark[p] = Some(q);
                    c.ps[p].hal_mut().markRef = Some(q);
                    continue;
                }
                let Gq = GG[i][j as usize].unwrap_or(J.g);
                let pp = &c.ps[p];
                let d = hypot(pp.x - Gq.0, pp.y - Gq.1);
                out[p] = Some(Ua { q, g: Gq, job: J.job.clone(), derin: J.derin, bul: J.bul, sprint: J.bul || J.derin || d / vSp(pp, k) > 12.0 });
            }
        }
        if let Some(mut yeniP) = yeniP {
            for &p in &self.swL.clone() {
                let tas = Ps.iter().enumerate().any(|(i, &x)| x == p && bst.as_ref().map(|b| b[i] >= 0 && Js[b[i] as usize].car.is_some()).unwrap_or(false));
                if !(self.swTg(p) != h || tas) {
                    self.swT[p] = Some(None);
                }
            }
            let es = self.press.clone();
            if es.len() != yeniP.len() || es.iter().any(|p| !yeniP.contains(p)) {
                for &p in &es {
                    if !yeniP.contains(&p) {
                        c.ps[p].press = false;
                        c.ps[p].hal_mut().lock = Some(0.0);
                    }
                }
                let hp = (c.ps[h.unwrap()].x, c.ps[h.unwrap()].y);
                yeniP.sort_by(|&u, &v| {
                    let a = kilitAn(&c.ps[u], tick) - kilitAn(&c.ps[v], tick);
                    sira(yada(a, hypot(c.ps[u].x - hp.0, c.ps[u].y - hp.1) - hypot(c.ps[v].x - hp.0, c.ps[v].y - hp.1)))
                });
                self.press = yeniP.clone();
                self.covered.truncate(1);
                for (i, &p) in yeniP.iter().enumerate() {
                    self.presIs_(c, p, i);
                }
            }
        }
        for &p in &self.defs {
            c.ps[p].hal_mut().uaPrev = out[p].as_ref().map(|u| u.q);
        }
        self.UA = out;
    }

    /// Yedek işler (JS YEDEK, YEDEK2): pres, eşleşme işi; yoksa geri koşu, önde bekle, markaj, arkada, bölge.
    pub fn yedek(&mut self, c: &mut Mc) {
        let k = c.k;
        let tick = c.m.tick;
        let (att, dt, E, h) = (self.att, self.dt, self.E, self.h);
        for &p in &self.defs.clone() {
            if self.press.contains(&p) {
                continue;
            }
            if let Some(J) = self.UA[p].clone() {
                let mut g = J.g;
                {
                    let pp = &c.ps[p];
                    if let Some(b) = self.bul {
                        if J.bul && (hypot(pp.x - J.g.0, pp.y - J.g.1) - 1.5) / vSp(pp, k) > b.3 + 2.0 {
                            g = gside(J.g.0, J.g.1, dt, 2.2);
                        }
                    }
                }
                c.ps[p].tx = g.0;
                c.ps[p].ty = g.1;
                if J.derin && !J.bul && J.job.starts_with("arkada · ") {
                    let dB = self.base[p].map(|b| self.gD(b.x, b.y)).unwrap_or(1e9);
                    let dJ = self.gD(J.g.0, J.g.1);
                    if dJ > dB + 1.0 {
                        c.ps[p].hal_mut()._blokT = Some(tick);
                        let gx = ownX(dt);
                        let kk = dB / dJ;
                        c.ps[p].tx = gx + (J.g.0 - gx) * kk;
                        c.ps[p].ty = GCY + (J.g.1 - GCY) * kk;
                    }
                }
                c.ps[p].job = Some(J.job.clone().into());
                if c.ps[p].hal._blokT == Some(tick) {
                    let nm = c.ps[J.q].govde.name.clone();
                    c.ps[p].job = Some(format!("arkada · blokta ({})", nm).into());
                }
                c.ps[p].hal_mut().tutQ = Some(J.q);
                if J.sprint {
                    c.ps[p].sprint = true;
                }
                continue;
            }
            let bs = self.base[p].unwrap();
            let sn = seen(c, p);
            if self.geriKosu(c, p, &bs) {
                continue;
            }
            if self.ondeBekle(c, p) {
                continue;
            }
            if self.arkaMarkaj(c, p, &sn) {
                continue;
            }
            if self.arkada(c, p, &sn) {
                continue;
            }
            self.markajBolge(c, p, &bs, &sn);
        }
        let _ = (att, E, h);
    }

    fn geriKosu(&self, c: &mut Mc, p: Pid, bs: &Yer) -> bool {
        let tick = c.m.tick;
        let pp = &c.ps[p];
        let rec = pp.hal.rec.unwrap_or(0.0);
        if !(!self.up[p] && (pp.hal.markRef.is_none() || !self.adamT(c, p)) && rec != 0.0 && rec == rec && tick >= rec && self.since < S.recT * geriK(pp) && (pp.x - self.E.0) * dirOf(pp.takim()) > -1.0) {
            return false;
        }
        let g = (self.E.0 + 0.35 * (ownX(self.dt) - self.E.0), cl(bs.y + 0.3 * (GCY - bs.y), 3.0, GH - 3.0));
        let pp = &mut c.ps[p];
        pp.tx = g.0;
        pp.ty = g.1;
        pp.sprint = true;
        pp.job = Some("geri koşu".into());
        true
    }

    fn ondeBekle(&self, c: &mut Mc, p: Pid) -> bool {
        if !self.up[p] {
            return false;
        }
        let dt = self.dt;
        let h = self.h;
        let ra: Vec<Pid> = self.atkAll.iter().copied().filter(|&q| Some(q) != h).collect();
        let rl = if !ra.is_empty() {
            let mut u = ra[0];
            for &v in &ra[1..] {
                if (c.ps[v].x - oppX(dt)).abs() < (c.ps[u].x - oppX(dt)).abs() {
                    u = v;
                }
            }
            c.ps[u].x
        } else {
            oppX(dt) - 30.0 * dirOf(dt)
        };
        let mut wx = rl - 3.0 * dirOf(dt);
        if (wx - oppX(dt)).abs() < 12.0 {
            wx = oppX(dt) - 12.0 * dirOf(dt);
        }
        {
            let pp = &mut c.ps[p];
            pp.tx = wx;
            pp.ty = GCY + 11.0 * pp.govde.slot.l * GKY;
            pp.job = Some("önde bekle".into());
        }
        // önde talimatlı oyuncu savunmaya katılmaz: işi kontra için önde beklemek. Taşıyıcının arkasındaki adama pas yolunu
        // kapatmak için geriye koşmak (eski 'geri pası kapat') onu hem kontradan koparıyordu hem de kaleden uzak, tehlikesiz bir pası kesiyordu.
        let _ = h;
        true
    }

    /// JS `{...o0, ...ileri(o0, okuT(p), q)}`: görülen adam, okuma ufku kadar ileride.
    fn okunan(&self, c: &Mc, p: Pid, o0: &Varlik, q: Pid) -> Varlik {
        let mut o = o0.clone();
        let (x, y) = ileriV(o0, okuTp(&c.ps[p]), Some(&c.ps[q]), c.ps, self.sg);
        // markajcı adamının yalnız kaleye doğru koşusunu önceden alır (geç kalmak arkaya kaçırmaktır); yana ya da geri
        // gidişini öne almaz, gördüğü yerinden izler: yoksa adam dönünce markajcı adamın gitmediği yerin kale tarafında,
        // adamının yanında/üstünde kalır ve ne pas yolunu ne kaleyi kapatır
        let (gx, gy) = (ownX(self.dt) - o0.x, GCY - o0.y);
        let gl = hypot(gx, gy);
        let (ux, uy) = if gl > 1e-9 { (gx / gl, gy / gl) } else { (0.0, 0.0) };
        let on = jmax(0.0, (x - o0.x) * ux + (y - o0.y) * uy);
        o.x = o0.x + ux * on;
        o.y = o0.y + uy * on;
        o
    }

    fn arkaMarkaj(&self, c: &mut Mc, p: Pid, sn: &[Varlik]) -> bool {
        if !self.sweep[p] || (self.h.is_some() && self.swTg(p) == self.h) {
            return false;
        }
        if let Some(q) = self.mark[p] {
            if self.adamT(c, p) {
                if let Some(o0) = sn.iter().find(|o| o.r == Some(q)) {
                    let o = self.okunan(c, p, o0, q);
                    let g = kapatHat(c, p, self.att, self.E, &o);
                    let nm = c.ps[q].govde.name.clone();
                    let pp = &mut c.ps[p];
                    pp.tx = g.0;
                    pp.ty = g.1;
                    pp.job = Some(format!("markaj · {}", nm).into());
                    pp.hal_mut().tutQ = Some(q);
                    return true;
                }
            }
        }
        false
    }

    fn arkada(&self, c: &mut Mc, p: Pid, sn: &[Varlik]) -> bool {
        if !self.sweep[p] {
            return false;
        }
        let (att, dt, h) = (self.att, self.dt, self.h);
        let o = self.swTg(p);
        let so = o.and_then(|o| sn.iter().find(|z| z.r == Some(o)));
        let ys = yerS(&c.ps[p]);
        if let (Some(so), Some(o)) = (so, o) {
            if Some(o) == h {
                let hp = c.ps[o].clone();
                let g1 = gside(so.x, so.y, dt, 3.0 + 0.2 * ys);
                let mut q2: Vec<(Pid, f64)> = self.atk.iter().copied().filter(|&q| hypot(c.ps[q].x - hp.x, c.ps[q].y - hp.y) < 30.0).map(|q| (q, dis::threat(att, c.ps[q].x, c.ps[q].y))).collect();
                q2.sort_by(|a, b| sira(b.1 - a.1));
                let g2 = match q2.first() {
                    Some(&(q, _)) => (hp.x + 0.35 * (c.ps[q].x - hp.x), hp.y + 0.35 * (c.ps[q].y - hp.y)),
                    None => g1,
                };
                let pp = &mut c.ps[p];
                pp.tx = 0.7 * g1.0 + 0.3 * g2.0;
                pp.ty = 0.7 * g1.1 + 0.3 * g2.1;
                pp.job = Some(format!("karşıla · {}", hp.govde.name).into());
                pp.hal_mut().tutQ = Some(o);
                return true;
            }
            let bulq = self.bul.filter(|b| b.0 == o);
            let g = match bulq {
                Some(b) => gside(b.1, b.2, dt, 3.0 + 0.2 * ys),
                None => {
                    let op = &c.ps[o];
                    let g0 = self.icOmuz(so.x, so.y, op.vx, op.vy, okuTp(&c.ps[p]), 3.0 + 0.2 * ys);
                    self.yakin(so.x, so.y, g0)
                }
            };
            let nm = c.ps[o].govde.name.clone();
            let pp = &mut c.ps[p];
            pp.tx = g.0;
            pp.ty = g.1;
            pp.job = Some(format!("arkada · {}", nm).into());
            // JS `p.tutQ = o.ref || null`: o gerçek oyuncu, ref'i yok → null
            pp.hal_mut().tutQ = None;
            return true;
        }
        let mut F: Vec<Pid> = self.swL.iter().copied().filter(|&q| self.swTg(q).is_none()).collect();
        let Eb = self.Eb;
        F.sort_by(|&u, &v| sira(hypot(c.ps[u].x - Eb.0, c.ps[u].y - Eb.1) - hypot(c.ps[v].x - Eb.0, c.ps[v].y - Eb.1)));
        let nF = F.len();
        let i = F.iter().position(|&x| x == p).map(|i| i as f64).unwrap_or(-1.0);
        let gx = ownX(dt);
        let dE = self.gD(Eb.0, Eb.1);
        let okp = okuTp(&c.ps[p]);
        let dR = if !self.atk.is_empty() {
            self.atk
                .iter()
                .map(|&q| match self.bul {
                    Some(b) if b.0 == q => self.gD(b.1, b.2),
                    _ => {
                        let qi = ileriP(&c.ps[q], okp + self.TA);
                        jmax(self.gD(qi.0, qi.1), 0.85 * self.gD(c.ps[q].x, c.ps[q].y) - 4.0)
                    }
                })
                .fold(f64::INFINITY, jmin)
        } else {
            dE
        };
        let dB = self.base[p].map(|b| self.gD(b.x, b.y)).unwrap_or(1e9);
        let d = jmax(6.0, jmin(jmin(dE - 3.0 - 0.2 * ys, dR - 2.0), dB));
        let f = if dE > 0.1 { jmin(1.0, d / dE) } else { 0.0 };
        let L = (gx + (Eb.0 - gx) * f, GCY + (Eb.1 - GCY) * f);
        let M = (gx + dirOf(dt) * d, GCY);
        let w = if nF <= 1 {
            if dE > 70.0 {
                0.6
            } else if dE < 30.0 {
                0.2
            } else {
                0.2 + (dE - 30.0) / 40.0 * 0.4
            }
        } else {
            i / (nF as f64 - 1.0)
        };
        let pp = &mut c.ps[p];
        pp.tx = L.0 + (M.0 - L.0) * w;
        pp.ty = cl(L.1 + (M.1 - L.1) * w, 3.0, GH - 3.0);
        pp.job = Some("arkada · alan".into());
        true
    }

    fn markajBolge(&self, c: &mut Mc, p: Pid, bs: &Yer, sn: &[Varlik]) {
        let (att, dt, E) = (self.att, self.dt, self.E);
        let (mut tx, mut ty) = (bs.x, bs.y);
        let q = self.mark[p];
        if let Some(q) = q {
            let o0 = sn.iter().find(|o| o.r == Some(q)).expect("markaj: görülmeyen adam");
            let o = self.okunan(c, p, o0, q);
            let g = kapatHat(c, p, att, E, &o);
            let nm = c.ps[q].govde.name.clone();
            if self.adamT(c, p) {
                let pp = &mut c.ps[p];
                pp.job = Some(format!("markaj · {}", nm).into());
                pp.hal_mut().tutQ = Some(q);
                pp.tx = g.0;
                pp.ty = g.1;
                return;
            }
            let w = S.markW * (0.6 + 0.02 * yerM(&c.ps[p]));
            tx += (g.0 - tx) * w;
            ty += (g.1 - ty) * w;
            c.ps[p].job = Some(format!("bölge · {}", nm).into());
            c.ps[p].hal_mut().tutQ = Some(q);
        } else {
            let dx = ownX(dt) - E.0;
            let dy = GCY - E.1;
            let L = nz1(hypot(dx, dy));
            let u = ((tx - E.0) * dx + (ty - E.1) * dy) / L;
            let px = E.0 + dx / L * u;
            let py = E.1 + dy / L * u;
            tx += 0.3 * (px - tx);
            ty += 0.3 * (py - ty);
            c.ps[p].job = Some("bölge".into());
        }
        let dk = hypot(E.0 - ownX(dt), E.1 - GCY);
        let pp = &c.ps[p];
        let st = 26.0 + 1.4 * yerS(pp) - 0.3 * (yada(self.dtac.t.blok.as_deref().and_then(BLOK).unwrap_or(0.0), 34.0) - 34.0);
        let w = cl((st - dk) / jmax(6.0, st - 10.0), 0.0, 1.0);
        if w > 0.0 {
            let qo = q.and_then(|q| sn.iter().find(|o| o.r == Some(q)));
            let T = match qo {
                Some(o) => (o.x, o.y),
                None => E,
            };
            let dT = hypot(T.0 - ownX(dt), T.1 - GCY);
            let hold = 6.0 + 10.0 * pp.govde.slot.d;
            let f = jmin(1.0, hold / jmax(dT, 1.0));
            let gx = ownX(dt) + (T.0 - ownX(dt)) * f;
            let gy = GCY + (T.1 - GCY) * f + if qo.is_some() { 0.0 } else { 8.0 * pp.govde.slot.l };
            let kk = if qo.is_some() { 0.6 } else { 1.0 };
            tx += (gx - tx) * w * kk;
            ty += (gy - ty) * w * kk;
        }
        let pp = &mut c.ps[p];
        pp.tx = tx;
        pp.ty = ty;
    }

    /// Takım düzeltmeleri (JS TAKIM): alan paylaşımı, açık top, geçiş savunması.
    pub fn takim(&mut self, c: &mut Mc) {
        let tick = c.m.tick;
        let (att, dt, E, h) = (self.att, self.dt, self.E, self.h);
        // alan paylaşımı
        {
            let zs: Vec<Pid> = self.defs.iter().copied().filter(|&p| !self.press.contains(&p) && !self.up[p] && c.ps[p].job.as_deref() != Some("geri koşu")).collect();
            let prC: Vec<f64> = (0..self.n)
                .map(|p| match self.mark[p] {
                    Some(m) => dis::threat(att, c.ps[m].x, c.ps[m].y) + 1.0,
                    None => {
                        if self.sweep[p] {
                            0.5
                        } else {
                            0.0
                        }
                    }
                })
                .collect();
            // JS pr(p) her karşılaştırmada hesaplanır; tehdit yalnız yere bağlı (aynı değer)
            let pr = |p: Pid| -> f64 { prC[p] };
            for _ in 0..2 {
                for i in 0..zs.len() {
                    for j in (i + 1)..zs.len() {
                        let (a, b) = (zs[i], zs[j]);
                        let dx = c.ps[b].tx - c.ps[a].tx;
                        let dy = c.ps[b].ty - c.ps[a].ty;
                        let dd = hypot(dx, dy);
                        if dd >= S.ayr {
                            continue;
                        }
                        let mv = if pr(a) >= pr(b) { b } else { a };
                        let st = if mv == a { b } else { a };
                        if self.mark[mv].is_some() && icerir(&c.ps[mv].job, "markaj") {
                            continue;
                        }
                        let (mut nx, mut ny) = (c.ps[mv].tx - c.ps[st].tx, c.ps[mv].ty - c.ps[st].ty);
                        let mut nl = hypot(nx, ny);
                        if nl < 0.3 {
                            nx = 0.0;
                            ny = if c.ps[mv].ty < GCY { -1.0 } else { 1.0 };
                            nl = 1.0;
                        }
                        let kk = S.ayr - dd;
                        let m = &mut c.ps[mv];
                        m.tx = cl(m.tx + nx / nl * kk, 2.0, GW - 2.0);
                        m.ty = cl(m.ty + ny / nl * kk, 2.0, GH - 2.0);
                    }
                }
            }
        }
        // açık top
        {
            let gx = ownX(dt);
            let dr = dirOf(dt);
            let kos = |vx: f64| cl((-vx * dr - 0.1) / 0.15, 0.0, 1.0);
            for &p in &self.defs.clone() {
                if self.press.contains(&p) || self.up[p] || bas(&c.ps[p].job, &["pres", "karşıla", "markaj", "geri koşu"]) {
                    continue;
                }
                let pp = &c.ps[p];
                let dP = (pp.x - gx) * dr;
                let dT = (pp.tx - gx) * dr;
                if dT <= dP {
                    continue;
                }
                let q = pp.hal.tutQ.filter(|&q| !c.ps[q].govde.name.is_empty() && icerir(&pp.job, &c.ps[q].govde.name));
                let mut w = self.acik;
                match q {
                    Some(q) => w = jmax(w, kos(c.ps[q].vx)),
                    None => {
                        for &r in &self.atk {
                            if Some(r) == h {
                                continue;
                            }
                            let dq = (c.ps[r].x - gx) * dr;
                            if dq > dP && dq - dP < 25.0 * GKX {
                                w = jmax(w, kos(c.ps[r].vx));
                            }
                        }
                    }
                }
                if w > 0.0 {
                    let x1 = gx + dr * (dP + (dT - dP) * (1.0 - w));
                    if let Some(q) = q {
                        if bas(&c.ps[p].job, &["arkada · "]) {
                            let (qx, qy) = (c.ps[q].x, c.ps[q].y);
                            let t = (x1 - qx) / yada(gx - qx, 1e-9);
                            if t > 0.0 && t < 1.0 {
                                let pp = &mut c.ps[p];
                                pp.ty += (qy + (GCY - qy) * t - pp.ty) * w;
                            }
                        }
                    }
                    c.ps[p].tx = x1;
                }
            }
        }
        // geçiş savunması
        if let Some(hh) = h {
            let _ = hh;
            for &q in &self.press {
                c.ps[q].hal_mut()._presT = Some(tick);
                c.ps[q].hal_mut()._presH = h;
            }
        }
        {
            let gE = self.gD(E.0, E.1);
            let gs0 = c.ek.sh.gs;
            let on0 = matches!(gs0, Some(g) if g.t == tick - 1.0 && g.dt == dt && g.on);
            let mut on = false;
            let mut onP = false;
            if gE < 64.0 * GKX {
                let nA = (if h.is_some() { 1 } else { 0 }) + self.atk.iter().filter(|&&q| { let a = ileriP(&c.ps[q], 30.0); self.gD(a.0, a.1) < gE + 1.0 }).count();
                let nD = c.ps.iter().filter(|p| p.takim() == dt && !p.bekci() && !p.noTouch && self.gD(p.x, p.y) < gE - 1.0).count();
                let (nA, nD) = (nA as f64, nD as f64);
                on = nD < nA + 1.0 || (on0 && nD < nA + 2.0);
                let mp = macPres(c, dt);
                let mg = 1.0 + 2.0 * (0.5 - jmin(1.0, self.dtac.t.siddet.unwrap_or(0.5) + if mp > 0.0 { 0.25 } else { 0.0 }));
                let onP0 = matches!(gs0, Some(g) if g.t == tick - 1.0 && g.dt == dt && g.onP);
                onP = (nD < nA + mg || (onP0 && nD < nA + mg + 1.0)) && self.defs.iter().any(|&o| { let op = &c.ps[o]; !self.up[o] && !op.noTouch && !op.bekci() && (op.x - ownX(dt)).abs() > (E.0 - ownX(dt)).abs() + 1.0 });
            }
            let kpA = dis::kpAkt(c, dt);
            c.ek.sh.gs = Some(Gs { t: tick, dt, on, onP, oy: kpA.is_none() && (onP || self.YR) });
            let pay = if on { -1.0 } else { 3.0 };
            if on || (self.YR && gE < 64.0 * GKX) {
                for &p in &self.defs.clone() {
                    if self.up[p] {
                        continue;
                    }
                    let pp = &c.ps[p];
                    let ex = self.press.contains(&p) || bas(&pp.job, &["pres", "karşıla", "kovala", "sekme", "alıcı"]) || pp.hal._savD == Some(tick) || pp.hal._savT == Some(tick);
                    if ex || self.gD(pp.tx, pp.ty) < gE + pay {
                        if self.gD(pp.x, pp.y) >= gE + pay && !pp.sprint {
                            let pp = &mut c.ps[p];
                            pp.sprint = true;
                            pp.job = Some(format!("{} · geri", pp.job.as_deref().unwrap_or("")).into());
                        }
                        continue;
                    }
                    let bs = self.base[p].map(|b| (b.x, b.y)).unwrap_or((pp.x, pp.y));
                    let g = (E.0 + 0.35 * (ownX(dt) - E.0), cl(bs.1 + 0.3 * (GCY - bs.1), 3.0, GH - 3.0));
                    let pp = &mut c.ps[p];
                    pp.tx = g.0;
                    pp.ty = g.1;
                    pp.sprint = true;
                    pp.job = Some("geri koşu · kale".into());
                    pp.hal_mut().tutQ = None;
                }
            }
        }
    }

    /// presIs (eşleşmeden sonra yeniden).
    pub fn presIs_(&mut self, c: &mut Mc, p: Pid, kk: usize) {
        self.presIs(c, p, kk);
    }
}

/// Eşleşme işinin değeri eskisi gibi (ağırlık × yetişme + "o olmasa", toplam): ALAN_UADEG_ESKI_IZ.
fn uadeg_eski() -> bool {
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *A.get_or_init(|| crate::iz_acik() && std::env::var("ALAN_UADEG_ESKI_IZ").is_ok())
}
