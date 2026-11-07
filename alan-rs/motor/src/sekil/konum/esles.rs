//! position: bölge markajı, eşleşme işi (UA: kişi / taşıyıcı / bölge atamaları), yedek işler, takım düzeltmeleri.
#![allow(non_snake_case, clippy::needless_range_loop)]

use crate::core_sabit::Q;
use crate::mat::{exp, hypot, jmax, jmin};
use crate::model::Pid;
use crate::oyuncu::Tepki;
use crate::yetenek::ah;

use super::super::gecici_dis as dis;
use super::super::*;
use super::savunma::{dal_sinir, pres_kazanc, ua_hist, Sv, UA_HIST};
use super::yardim::*;

impl Sv {
    /// JS icOmuz(q, T, off, r): adamın kale tarafı, koşusuyla T tik ileri (kenara açılan koşu yarım sayılır).
    pub fn ic_omuz(&self, qx: f64, qy: f64, vx: f64, vy: f64, T: f64, off: f64) -> (f64, f64) {
        let gx = own_x(self.dt);
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
        let d_mx = 4.0 + 0.15 * self.g_d(qx, qy);
        if dg > d_mx {
            (qx + (g.0 - qx) / dg * d_mx, qy + (g.1 - qy) / dg * d_mx)
        } else {
            g
        }
    }

    /// Bölge markajı (JS zonePrev ve en yakın boştaki savunucu).
    pub fn bolge(&mut self, c: &mut Mc) {
        let att = self.att;
        let n = self.n;
        let mut marked_ref = vec![false; n];
        for p in 0..n {
            if let Some(q) = self.mark[p] {
                marked_ref[q] = true;
            }
            if let Some(Some(q)) = self.sw_t[p] {
                marked_ref[q] = true;
            }
        }
        let mut z_tak = vec![false; n];
        for &p in &self.free.clone() {
            let q = match c.ps[p].hal.zonePrev {
                Some(q) => q,
                None => continue,
            };
            if self.taken[p] || marked_ref[q] || z_tak[q] || !self.atk.contains(&q) {
                continue;
            }
            let sv = seen(c, p);
            let sn = sv.iter().find(|o| o.r == Some(q));
            let bs = self.base[p].unwrap();
            let dr = if self.d_max > 0.0 { jmin(1.0, self.dng(c, q) / self.d_max) } else { 0.0 };
            if let Some(sn) = sn {
                if self.yerlesik || hypot(sn.x - bs.x, sn.y - bs.y) < 1.5 * (S.zone_r + yer_s(&c.ps[p]) * S.zone_ok + S.zone_dng * dr) {
                    self.taken[p] = true;
                    self.mark[p] = Some(q);
                    z_tak[q] = true;
                }
            }
        }
        let mut aq: Vec<(Pid, f64)> = self.atk.iter().map(|&q| (q, dis::threat(att, c.ps[q].x, c.ps[q].y))).collect();
        aq.sort_by(|a, b| sira(b.1 - a.1));
        for (q, _) in aq {
            if marked_ref[q] || z_tak[q] {
                continue;
            }
            let dr = if self.d_max > 0.0 { jmin(1.0, self.dng(c, q) / self.d_max) } else { 0.0 };
            let mut bp: Option<Pid> = None;
            let mut bd = 1e9;
            for &p in &self.free.clone() {
                if self.taken[p] {
                    continue;
                }
                let sv = seen(c, p);
                let sn0 = sv.iter().find(|o| o.r == Some(q)).expect("bölge: görülmeyen rakip");
                let sn = ileri_v(sn0, oku_tp(&c.ps[p]), Some(&c.ps[q]), c.ps, self.sg);
                let bs = self.base[p].unwrap();
                let d = hypot(sn.0 - bs.x, sn.1 - bs.y);
                // yerleşik hücumda bölge yarıçapı yok: en yakın boştaki savunmacı
                if (self.yerlesik || d < S.zone_r + yer_s(&c.ps[p]) * S.zone_ok + S.zone_dng * dr) && d < bd {
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
        let karsila: Vec<bool> = (0..n).map(|p| h.is_some() && self.sweep[p] && self.sw_tg(p) == h).collect();
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
        let g_eb = self.g_d(self.Eb.0, self.Eb.1);
        for &q in &self.atk.clone() {
            let qp = c.ps[q].clone();
            if Some(q) == h || h.map(|hh| qp.id == c.ps[hh].id).unwrap_or(false) {
                continue;
            }
            let w = self.dng(c, q);
            if Some(q) == alici {
                if !kovalar {
                    let b = self.bul.unwrap();
                    isler.push(Is { q: IsQ::P(q), w, g: (b.1, b.2), job: "pres · buluşma".into(), derin: self.g_d(b.1, b.2) < 30.0, bul: true, oku_f: false, grp: None, car: None, zone: None, rq: vec![], thr: 0.0 });
                }
                continue;
            }
            let qa = ileri_p(&qp, self.Tt);
            let iz = ileri_p(&qp, jmax(30.0, self.Tt + self.TA));
            let derin = self.g_d(iz.0, iz.1) < g_eb + 2.0;
            let omuz = self.yakin(qp.x, qp.y, self.ic_omuz(qp.x, qp.y, qp.vx, qp.vy, self.Tt + self.TA, S.mark_off));
            isler.push(Is {
                q: IsQ::P(q),
                w: if derin { w } else { w * (1.0 - self.acik) },
                g: if derin { omuz } else { yolu(qa) },
                job: format!("{}{}", if derin { "arkada · " } else { "kapatma · " }, qp.govde.name),
                derin,
                bul: false,
                oku_f: derin,
                grp: None,
                car: None,
                zone: None,
                rq: vec![],
                thr: 0.0,
            });
            if !derin {
                isler.last_mut().unwrap().grp = Some(qp.id);
                isler.push(Is { q: IsQ::P(q), w, g: omuz, job: format!("arkada · {}", qp.govde.name), derin: true, bul: false, oku_f: true, grp: Some(qp.id), car: None, zone: None, rq: vec![], thr: 0.0 });
            }
        }
        isler.sort_by(|a, b| sira(b.w - a.w));
        let mut out: Vec<Option<Ua>> = vec![None; n];
        let a_fs = c.d.arriveFirm;
        let shs = jmax(1.0, yada(c.d.sut_hazir, 1.0));
        let pr2 = matches!(h, Some(hh) if c.ps[hh].takim() == att) && self.prs.is_some();
        let YR = self.YR;
        let mut Js: Vec<Is> = isler.into_iter().filter(|J| !(YR && !J.derin && !J.bul)).collect();
        let Ps: Vec<Pid> = if pr2 {
            let P = self.prs.as_ref().unwrap();
            self.defs.iter().copied().filter(|&p| (!self.up[p] || self.up_bas(c, p)) && !P.fixed.contains(&p) && (!self.press.contains(&p) || P.cand[p])).collect()
        } else {
            pool.clone()
        };
        if pr2 {
            let P = self.prs.as_ref().unwrap();
            let hh = h.unwrap();
            let nf = P.fixed.len();
            let kac = P.p_lim.saturating_sub(nf);
            for kk in 0..kac {
                let car = kk + nf;
                let w0 = match self.t_p.as_ref().and_then(|t| t.h) {
                    Some(th) => th * P.K,
                    None => P.G,
                };
                // ALAN_PRES_KAZANC_IZ (S1): baskının değerine kendi kazancımız da girer: topu taşıyıcının yerinde kazanırsak
                // rakip kalesine yakınlık (savunan takımın tehdidi) × taktiğin pres ayarı × hat katı. Kafanın "o olmasa"
                // değerinden (t_p.h, rakibin haritası) bağımsız; pres = 0 olan takımda 0.
                let w0 = if pres_kazanc() {
                    let hp = &c.ps[hh];
                    w0 + self.dtac.t.pres.unwrap_or(0.0).max(0.0) * jmax(0.0, dis::threat(dt, hp.x, hp.y)) * P.K
                } else {
                    w0
                };
                Js.push(Is { q: IsQ::P(hh), w: w0 * if car == 0 { 1.0 } else { 1.0 - P.P0 }, g: (c.ps[hh].x, c.ps[hh].y), job: String::new(), derin: false, bul: false, oku_f: false, grp: None, car: Some(car), zone: None, rq: vec![], thr: 0.0 });
            }
        }
        {
            let AQ: Vec<Pid> = self.atk.iter().copied().filter(|&q| { let qp = &c.ps[q]; !(qp.noTouch || qp.bekci() || h.map(|hh| qp.id == c.ps[hh].id).unwrap_or(false)) }).collect();
            for (kk, &p) in Ps.iter().enumerate() {
                let Z = match self.base[p] {
                    Some(z) => z,
                    None => continue,
                };
                let tz = dis::ucus_t(c.kafa, hypot(Z.x - E.0, Z.y - E.1), a_fs, None) + shs;
                let mut r: f64 = 0.0;
                let mut rq = Vec::new();
                for &q in &AQ {
                    let qp = &c.ps[q];
                    let rr = 1.0 / (1.0 + exp((hypot(qp.x - Z.x, qp.y - Z.y) / v_sp(qp, k) - tz) / 5.0));
                    rq.push((q, rr));
                    r = jmax(r, rr);
                }
                let thr = jmax(0.0, dis::threat(att, Z.x, Z.y)) * exp(-tz / 60.0);
                let w = thr * r;
                if w > 0.0 {
                    Js.push(Is { q: IsQ::Bolge { id: -200.0 - kk as f64, x: Z.x, y: Z.y }, w, g: (Z.x, Z.y), job: "bölge".into(), derin: true, bul: false, oku_f: false, grp: None, car: None, zone: Some(p), rq, thr });
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
                b.unwrap_or_else(|| { let q = qxy(J); dis::ucus_t(c.kafa, hypot(q.0 - E.0, q.1 - E.1), a_fs, None) }) + shs
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
                        let t = d / v_sp(up, k) + gec(up, J.g.0, J.g.1, Some(d), k);
                        kk = jmax(kk, 1.0 / (1.0 + exp((t - TW[j]) / 5.0)));
                    }
                    J.w *= 1.0 - kk;
                }
            }
        }
        let kuyu_var = |p: Pid, G: (f64, f64), t: f64, v: f64| -> f64 {
            let pp = &c.ps[p];
            if pp.bekci() || c.m._kuyu_top == Some(if pp.x < GW / 2.0 { 0.0 } else { 1.0 }) {
                return t;
            }
            let cx = own_x(dt);
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
            let s_l = -bb + (jmax(0.0, bb * bb - (bx * bx + by * by - R * R))).sqrt();
            let t_cik = jmin(L, s_l) / v + 1.0 / (0.18 * ah(&pp.govde.yet));
            if t_cik <= kal {
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
            let v = v_sp(pp, k);
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
                            J.w * (1.0 / (1.0 + exp((hypot(pp.x - J.g.0, pp.y - J.g.1) / v_sp(pp, k) - TW[j]) / 5.0)))
                        } else {
                            0.0
                        }
                    }
                    1 => {
                        let P = self.prs.as_ref().unwrap();
                        let e = P.pv[p];
                        if e.is_none() && !self.sweep[p] {
                            0.0
                        } else {
                            let hp = &c.ps[h.unwrap()];
                            let PP = match e {
                                Some(e) => e.P,
                                None => 1.0 / (1.0 + exp((jmax(0.0, TEPKI(pp, Tepki::Kovala, k)) + jmax(0.0, hypot(pp.x - hp.x, pp.y - hp.y) - 2.4) / v - 8.0) / 8.0)),
                            };
                            match &self.t_p {
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
                        let gp = if J.oku_f { self.yakin(qp.x, qp.y, self.ic_omuz(qp.x, qp.y, qp.vx, qp.vy, oku_tp(pp) + 0.0, S.mark_off)) } else { J.g };
                        GG[i][j] = Some(gp);
                        let d = hypot(pp.x - gp.0, pp.y - gp.1);
                        let t = kuyu_var(p, gp, d / v + gec(pp, gp.0, gp.1, Some(d), k), v) - if pp.hal.uaPrev == Some(q) || pp.hal.markRef == Some(q) { rc } else { 0.0 };
                        let kp = 1.0 / (1.0 + exp((t - TW[j]) / 5.0));
                        let mg = match &self.t_p {
                            Some(tp) if qp.id >= 0.0 && !J.bul => yada(tp.mm_get(&format!("{}|{}", id_str(qp.id), id_str(pp.id))).unwrap_or(0.0), 0.0),
                            _ => 0.0,
                        };
                        if self.t_p.is_some() && !uadeg_eski() {
                            jmax(mg, jmax(0.0, J.w)) * kp
                        } else if kp > 0.001 || mg > 0.0 {
                            J.w * kp + mg
                        } else {
                            0.0
                        }
                    }
                };
                // ALAN_UA_HIST_IZ (S2): önceki tikin işi sürüyorsa değeri UA_HIST kadar artar (iş değiştirme maliyeti):
                // birbirine yakın değerlerde atama her tik dönmesin, oyuncu iki işin arasında gidip gelmesin
                let val = if ua_hist() && val > 0.0 && {
                    let pp = &c.ps[p];
                    match tr {
                        1 => pp.hal._savCar == Some(tick - 1.0),
                        0 => J.qp().map_or(false, |q| pp.hal.uaPrev == Some(q) || pp.hal.markRef == Some(q)),
                        _ => false,
                    }
                } {
                    val * (1.0 + UA_HIST)
                } else {
                    val
                };
                // S7k: tehlike bölgesindeki (kaleye S7_D) adamını tutan markajcı onu devretmeden bırakmaz: önceki adamına
                // değeri S7K kat (başka adama ya da alana geçmesi ancak çok daha değerliyse)
                let val = if s7(1024) && tr == 0 && val > 0.0 && J.qp().map_or(false, |q| {
                    let pp = &c.ps[p];
                    (pp.hal.uaPrev == Some(q) || pp.hal.markRef == Some(q)) && self.g_d(c.ps[q].x, c.ps[q].y) < S7_D
                }) {
                    val * S7K
                } else {
                    val
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
            if let Some(t) = &self.t_p { eprintln!("TP {:?}", t.mm.iter().map(|(k, v)| format!("{}={:.4}", k, v)).collect::<Vec<_>>()); }
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
            if self.adam_t(c, p) {
                self.mark[p] = None;
            }
        }
        let mut yeni_p: Option<Vec<Pid>> = if pr2 { Some(self.prs.as_ref().unwrap().fixed.clone()) } else { None };
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
                    yeni_p.as_mut().unwrap().push(p);
                    continue;
                }
                let q = J.qp().unwrap();
                if self.adam_t(c, p) && !J.bul {
                    self.mark[p] = Some(q);
                    c.ps[p].hal_mut().markRef = Some(q);
                    continue;
                }
                let Gq = GG[i][j as usize].unwrap_or(J.g);
                let pp = &c.ps[p];
                let d = hypot(pp.x - Gq.0, pp.y - Gq.1);
                out[p] = Some(Ua { q, g: Gq, job: J.job.clone(), derin: J.derin, bul: J.bul, sprint: J.bul || J.derin || d / v_sp(pp, k) > 12.0 });
            }
        }
        if let Some(mut yeni_p) = yeni_p {
            for &p in &self.sw_l.clone() {
                let tas = Ps.iter().enumerate().any(|(i, &x)| x == p && bst.as_ref().map(|b| b[i] >= 0 && Js[b[i] as usize].car.is_some()).unwrap_or(false));
                if !(self.sw_tg(p) != h || tas) {
                    self.sw_t[p] = Some(None);
                }
            }
            let es = self.press.clone();
            if es.len() != yeni_p.len() || es.iter().any(|p| !yeni_p.contains(p)) {
                for &p in &es {
                    if !yeni_p.contains(&p) {
                        c.ps[p].press = false;
                        c.ps[p].hal_mut().lock = 0.0;
                    }
                }
                let hp = (c.ps[h.unwrap()].x, c.ps[h.unwrap()].y);
                yeni_p.sort_by(|&u, &v| {
                    let a = kilit_an(&c.ps[u], tick) - kilit_an(&c.ps[v], tick);
                    sira(yada(a, hypot(c.ps[u].x - hp.0, c.ps[u].y - hp.1) - hypot(c.ps[v].x - hp.0, c.ps[v].y - hp.1)))
                });
                self.press = yeni_p.clone();
                self.covered.truncate(1);
                for (i, &p) in yeni_p.iter().enumerate() {
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
                        if J.bul && (hypot(pp.x - J.g.0, pp.y - J.g.1) - 1.5) / v_sp(pp, k) > b.3 + 2.0 {
                            g = gside(J.g.0, J.g.1, dt, 2.2);
                        }
                    }
                }
                c.ps[p].tx = g.0;
                c.ps[p].ty = g.1;
                if J.bul && crate::iz_acik() && std::env::var("ALAN_BUL_IZ").is_ok() {
                    let pp = &c.ps[p];
                    eprintln!("BULJ {} {} yer ({:.1},{:.1}) J.g ({:.1},{:.1}) hedef ({:.1},{:.1}) job {}", tick, pp.govde.name, pp.x, pp.y, J.g.0, J.g.1, g.0, g.1, J.job);
                }
                // blok: adamın kale tarafı savunmacının blok derinliğinden öndeyse adamın peşinden çıkmaz, bloğunda kalır
                // (orta sahaya çekilmesin). Yerleşik hücumda (top kuyunun önünde) adam zaten ceza sahası ağzında: blok
                // yok, adamı kale tarafından tutar.
                if J.derin && !J.bul && J.job.starts_with("arkada · ") && !self.yerlesik {
                    let d_b = self.base[p].map(|b| self.g_d(b.x, b.y)).unwrap_or(1e9);
                    let d_j = self.g_d(J.g.0, J.g.1);
                    if d_j > d_b + 1.0 {
                        c.ps[p].hal_mut()._blokT = Some(tick);
                        let gx = own_x(dt);
                        let kk = d_b / d_j;
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
            if self.geri_kosu(c, p, &bs) {
                continue;
            }
            if self.onde_bekle(c, p) {
                continue;
            }
            if self.arka_markaj(c, p, &sn) {
                continue;
            }
            if self.arkada(c, p, &sn) {
                continue;
            }
            self.markaj_bolge(c, p, &bs, &sn);
        }
        let _ = (att, E, h);
    }

    fn geri_kosu(&self, c: &mut Mc, p: Pid, bs: &Yer) -> bool {
        let tick = c.m.tick;
        let pp = &c.ps[p];
        let rec = pp.hal.rec;
        if !(!self.up[p] && (pp.hal.markRef.is_none() || !self.adam_t(c, p)) && rec != 0.0 && rec == rec && tick >= rec && self.since < S.rec_t * geri_k(pp) && (pp.x - self.E.0) * dir_of(pp.takim()) > -1.0) {
            return false;
        }
        let g = (self.E.0 + 0.35 * (own_x(self.dt) - self.E.0), cl(bs.y + 0.3 * (GCY - bs.y), 3.0, GH - 3.0));
        let pp = &mut c.ps[p];
        pp.tx = g.0;
        pp.ty = g.1;
        pp.sprint = true;
        pp.job = Some("geri koşu".into());
        true
    }

    fn onde_bekle(&self, c: &mut Mc, p: Pid) -> bool {
        if !self.up[p] {
            return false;
        }
        let dt = self.dt;
        let h = self.h;
        let ra: Vec<Pid> = self.atk_all.iter().copied().filter(|&q| Some(q) != h).collect();
        let rl = if !ra.is_empty() {
            let mut u = ra[0];
            for &v in &ra[1..] {
                if (c.ps[v].x - opp_x(dt)).abs() < (c.ps[u].x - opp_x(dt)).abs() {
                    u = v;
                }
            }
            c.ps[u].x
        } else {
            opp_x(dt) - 30.0 * dir_of(dt)
        };
        let mut wx = rl - 3.0 * dir_of(dt);
        if (wx - opp_x(dt)).abs() < 12.0 {
            wx = opp_x(dt) - 12.0 * dir_of(dt);
        }
        let ty0 = GCY + 11.0 * c.ps[p].govde.slot.l * GKY;
        // deney (onde_ac): önde bekleyen yerinde durmaz, son çizgi boyunca açığa (rakipten uzak, toptan pas yolu açık) kayar;
        // kazanç küçükse önceki yerinde kalır (titreme yok)
        let ty = if onde_ac() {
            let E = self.E;
            let puan = |y: f64| -> f64 {
                let mut dmin: f64 = 1e9;
                let mut yol: f64 = 1e9;
                let (vx, vy) = (wx - E.0, y - E.1);
                let l2 = jmax(1e-9, vx * vx + vy * vy);
                for &q in &self.atk_all {
                    let (qx, qy) = (c.ps[q].x, c.ps[q].y);
                    dmin = jmin(dmin, hypot(qx - wx, qy - y));
                    let u = jmax(0.0, jmin(1.0, ((qx - E.0) * vx + (qy - E.1) * vy) / l2));
                    yol = jmin(yol, hypot(E.0 + u * vx - qx, E.1 + u * vy - qy));
                }
                jmin(dmin, 8.0) + 0.5 * jmin(yol, 6.0) - 0.1 * (y - ty0).abs()
            };
            let mut en = (ty0, puan(ty0));
            let mut k = -10.0;
            while k <= 10.0 {
                let y = cl(ty0 + k, 3.0, GH - 3.0);
                let v = puan(y);
                if v > en.1 {
                    en = (y, v);
                }
                k += 2.0;
            }
            // hedef uzun süre geçerli: oyuncu başına 30 tikte bir yeniden tartılır, yalnız belirgin kazançta (3) kayar
            let pp = &c.ps[p];
            let once = pp.job.as_deref() == Some("önde bekle") && (pp.ty - ty0).abs() <= 10.5;
            let tart = (c.m.tick as i64 + p as i64 * 7) % ONDE_AC_T == 0;
            if once && (!tart || puan(pp.ty) + 3.0 >= en.1) { pp.ty } else { en.0 }
        } else {
            ty0
        };
        {
            let pp = &mut c.ps[p];
            pp.tx = wx;
            pp.ty = ty;
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
        let (x, y) = ileri_v(o0, oku_tp(&c.ps[p]), Some(&c.ps[q]), c.ps, self.sg);
        // markajcı adamının yalnız kaleye doğru koşusunu önceden alır (geç kalmak arkaya kaçırmaktır); yana ya da geri
        // gidişini öne almaz, gördüğü yerinden izler: yoksa adam dönünce markajcı adamın gitmediği yerin kale tarafında,
        // adamının yanında/üstünde kalır ve ne pas yolunu ne kaleyi kapatır
        let (gx, gy) = (own_x(self.dt) - o0.x, GCY - o0.y);
        let gl = hypot(gx, gy);
        let (ux, uy) = if gl > 1e-9 { (gx / gl, gy / gl) } else { (0.0, 0.0) };
        let on = jmax(0.0, (x - o0.x) * ux + (y - o0.y) * uy);
        o.x = o0.x + ux * on;
        o.y = o0.y + uy * on;
        o
    }

    fn arka_markaj(&self, c: &mut Mc, p: Pid, sn: &[Varlik]) -> bool {
        if !self.sweep[p] || (self.h.is_some() && self.sw_tg(p) == self.h) {
            return false;
        }
        if let Some(q) = self.mark[p] {
            if self.adam_t(c, p) {
                if let Some(o0) = sn.iter().find(|o| o.r == Some(q)) {
                    let o = self.okunan(c, p, o0, q);
                    let g = kapat_hat(c, p, self.att, self.E, &o);
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
        let o = self.sw_tg(p);
        let so = o.and_then(|o| sn.iter().find(|z| z.r == Some(o)));
        let ys = yer_s(&c.ps[p]);
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
                    let g0 = self.ic_omuz(so.x, so.y, op.vx, op.vy, oku_tp(&c.ps[p]), 3.0 + 0.2 * ys);
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
        let mut F: Vec<Pid> = self.sw_l.iter().copied().filter(|&q| self.sw_tg(q).is_none()).collect();
        let Eb = self.Eb;
        F.sort_by(|&u, &v| sira(hypot(c.ps[u].x - Eb.0, c.ps[u].y - Eb.1) - hypot(c.ps[v].x - Eb.0, c.ps[v].y - Eb.1)));
        let n_f = F.len();
        let i = F.iter().position(|&x| x == p).map(|i| i as f64).unwrap_or(-1.0);
        let gx = own_x(dt);
        let d_e = self.g_d(Eb.0, Eb.1);
        let okp = oku_tp(&c.ps[p]);
        let d_r = if !self.atk.is_empty() {
            self.atk
                .iter()
                .map(|&q| match self.bul {
                    Some(b) if b.0 == q => self.g_d(b.1, b.2),
                    _ => {
                        let qi = ileri_p(&c.ps[q], okp + self.TA);
                        jmax(self.g_d(qi.0, qi.1), 0.85 * self.g_d(c.ps[q].x, c.ps[q].y) - 4.0)
                    }
                })
                .fold(f64::INFINITY, jmin)
        } else {
            d_e
        };
        let d_b = self.base[p].map(|b| self.g_d(b.x, b.y)).unwrap_or(1e9);
        let d = jmax(6.0, jmin(jmin(d_e - 3.0 - 0.2 * ys, d_r - 2.0), d_b));
        let f = if d_e > 0.1 { jmin(1.0, d / d_e) } else { 0.0 };
        let L = (gx + (Eb.0 - gx) * f, GCY + (Eb.1 - GCY) * f);
        let M = (gx + dir_of(dt) * d, GCY);
        let w = if n_f <= 1 {
            if d_e > 70.0 {
                0.6
            } else if d_e < 30.0 {
                0.2
            } else {
                0.2 + (d_e - 30.0) / 40.0 * 0.4
            }
        } else {
            i / (n_f as f64 - 1.0)
        };
        let pp = &mut c.ps[p];
        pp.tx = L.0 + (M.0 - L.0) * w;
        pp.ty = cl(L.1 + (M.1 - L.1) * w, 3.0, GH - 3.0);
        pp.job = Some("arkada · alan".into());
        true
    }

    fn markaj_bolge(&self, c: &mut Mc, p: Pid, bs: &Yer, sn: &[Varlik]) {
        let (att, dt, E) = (self.att, self.dt, self.E);
        let (mut tx, mut ty) = (bs.x, bs.y);
        let q = self.mark[p];
        if let Some(q) = q {
            let o0 = sn.iter().find(|o| o.r == Some(q)).expect("markaj: görülmeyen adam");
            let o = self.okunan(c, p, o0, q);
            let g = kapat_hat(c, p, att, E, &o);
            let nm = c.ps[q].govde.name.clone();
            if self.adam_t(c, p) {
                let pp = &mut c.ps[p];
                pp.job = Some(format!("markaj · {}", nm).into());
                pp.hal_mut().tutQ = Some(q);
                pp.tx = g.0;
                pp.ty = g.1;
                return;
            }
            let w = S.mark_w * (0.6 + 0.02 * yer_m(&c.ps[p]));
            tx += (g.0 - tx) * w;
            ty += (g.1 - ty) * w;
            c.ps[p].job = Some(format!("bölge · {}", nm).into());
            c.ps[p].hal_mut().tutQ = Some(q);
        } else {
            let dx = own_x(dt) - E.0;
            let dy = GCY - E.1;
            let L = nz1(hypot(dx, dy));
            let u = ((tx - E.0) * dx + (ty - E.1) * dy) / L;
            let px = E.0 + dx / L * u;
            let py = E.1 + dy / L * u;
            tx += 0.3 * (px - tx);
            ty += 0.3 * (py - ty);
            c.ps[p].job = Some("bölge".into());
        }
        let dk = hypot(E.0 - own_x(dt), E.1 - GCY);
        let pp = &c.ps[p];
        let st = 26.0 + 1.4 * yer_s(pp) - 0.3 * (yada(self.dtac.t.blok.as_deref().and_then(BLOK).unwrap_or(0.0), 34.0) - 34.0);
        let w = cl((st - dk) / jmax(6.0, st - 10.0), 0.0, 1.0);
        if w > 0.0 {
            let qo = q.and_then(|q| sn.iter().find(|o| o.r == Some(q)));
            let T = match qo {
                Some(o) => (o.x, o.y),
                None => E,
            };
            let d_t = hypot(T.0 - own_x(dt), T.1 - GCY);
            let hold = 6.0 + 10.0 * pp.govde.slot.d;
            let f = jmin(1.0, hold / jmax(d_t, 1.0));
            let gx = own_x(dt) + (T.0 - own_x(dt)) * f;
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
            let pr_c: Vec<f64> = (0..self.n)
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
            let pr = |p: Pid| -> f64 { pr_c[p] };
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
            let gx = own_x(dt);
            let dr = dir_of(dt);
            let kos = |vx: f64| cl((-vx * dr - 0.1) / 0.15, 0.0, 1.0);
            for &p in &self.defs.clone() {
                if self.press.contains(&p) || self.up[p] || bas(&c.ps[p].job, &["pres", "karşıla", "markaj", "geri koşu"]) {
                    continue;
                }
                let pp = &c.ps[p];
                let dP = (pp.x - gx) * dr;
                let d_t = (pp.tx - gx) * dr;
                if d_t <= dP {
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
                    let x1 = gx + dr * (dP + (d_t - dP) * (1.0 - w));
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
            let g_e = self.g_d(E.0, E.1);
            let gs0 = c.ek.sh.gs;
            let on0 = matches!(gs0, Some(g) if g.t == tick - 1.0 && g.dt == dt && g.on);
            let mut on = false;
            let mut on_p = false;
            if g_e < 64.0 * GKX {
                let n_a = (if h.is_some() { 1 } else { 0 }) + self.atk.iter().filter(|&&q| { let a = ileri_p(&c.ps[q], 30.0); self.g_d(a.0, a.1) < g_e + 1.0 }).count();
                let n_d = c.ps.iter().filter(|p| p.takim() == dt && !p.bekci() && !p.noTouch && self.g_d(p.x, p.y) < g_e - 1.0).count();
                let (n_a, n_d) = (n_a as f64, n_d as f64);
                on = n_d < n_a + 1.0 || (on0 && n_d < n_a + 2.0);
                let mp = mac_pres(c, dt);
                let mg = 1.0 + 2.0 * (0.5 - jmin(1.0, self.dtac.t.siddet.unwrap_or(0.5) + if mp > 0.0 { 0.25 } else { 0.0 }));
                let on_p0 = matches!(gs0, Some(g) if g.t == tick - 1.0 && g.dt == dt && g.on_p);
                on_p = (n_d < n_a + mg || (on_p0 && n_d < n_a + mg + 1.0)) && self.defs.iter().any(|&o| { let op = &c.ps[o]; !self.up[o] && !op.noTouch && !op.bekci() && (op.x - own_x(dt)).abs() > (E.0 - own_x(dt)).abs() + 1.0 });
            }
            let kp_a = dis::kp_akt(c, dt);
            c.ek.sh.gs = Some(Gs { t: tick, dt, on, on_p, oy: kp_a.is_none() && (on_p || self.YR) });
            let pay = if on { -1.0 } else { 3.0 };
            if on || (self.YR && g_e < 64.0 * GKX) {
                // S7d: bu tik tutulan adamlar (savunmacıların tutQ'su)
                // S7f: yalnız adamını gerçekten tutanlar (presçi değil, işi adam işi) sayılır; buluşmaya / prese giden adamını devreder
                let mut s7_tut: Vec<Pid> = if s7(8) || s7(64) {
                    self.defs.iter().filter(|&&d| !s7(32) || (!self.press.contains(&d) && bas(&c.ps[d].job, &["markaj", "arkada", "kapatma", "geri koşu", "yolu kes"]))).filter_map(|&d| c.ps[d].hal.tutQ).collect()
                } else {
                    Vec::new()
                };
                for &p in &self.defs.clone() {
                    if self.up[p] {
                        continue;
                    }
                    let pp = &c.ps[p];
                    // S7d: tehlike bölgesinde geçilmiş (topun gerisinde kalmış) savunmacı, ilk presçi dışında (baskıcılar da):
                    // adamı tehlikeliyse (topun önünde ya da kendisi tehlike bölgesinde, uzak taraf dahil) onu tutmayı sürdürür;
                    // değilse önce tehlike bölgesinde kimsenin tutmadığı adama (kaleye en yakın), o da yoksa kale önüne döner
                    let gecildi = self.g_d(pp.x, pp.y) >= g_e + 1.0;
                    // S7f: geçilmemiş bölgecisi de tehlike bölgesinde boşta kalan adamı alır (kale önüne gitmez)
                    // S7g: alçak blokta top uzaktayken de bölgeci kale önünde (tehlike bölgesinde) boş kalan adamı alır
                    let dusuk = s7(64) && self.dtac.t.blok.as_deref() == Some("Düşük");
                    let bolgeci = (s7(32) || dusuk) && !gecildi && !self.press.contains(&p) && bas(&pp.job, &["bölge"]);
                    if (s7(8) || dusuk) && (g_e < S7_D || (dusuk && bolgeci)) && self.press.first() != Some(&p) && (gecildi || bolgeci) && {
                        let q = pp.hal.tutQ.or(pp.hal.markRef);
                        q.map_or(true, |q| { let d = self.g_d(c.ps[q].x, c.ps[q].y); !(d < g_e - 2.0 || d < S7_D) })
                    } {
                        let bos = self.atk.iter().copied().filter(|&q| Some(q) != h && !s7_tut.contains(&q) && self.g_d(c.ps[q].x, c.ps[q].y) < S7_D).min_by(|&u, &v| sira(self.g_d(c.ps[u].x, c.ps[u].y) - self.g_d(c.ps[v].x, c.ps[v].y)));
                        if let Some(q) = bos {
                            s7_tut.push(q);
                            let (qx, qy) = (c.ps[q].x, c.ps[q].y);
                            let l = jmax(0.01, self.g_d(qx, qy));
                            let nm = c.ps[q].govde.name.clone();
                            let pp = &mut c.ps[p];
                            pp.tx = qx + 2.0 * (own_x(dt) - qx) / l;
                            pp.ty = qy + 2.0 * (GCY - qy) / l;
                            pp.sprint = true;
                            pp.job = Some(format!("geri koşu · {}", nm).into());
                            pp.hal_mut().tutQ = Some(q);
                            continue;
                        }
                        if gecildi && g_e < S7_D {
                            let bs = self.base[p].map(|b| (b.x, b.y)).unwrap_or((pp.x, pp.y));
                            // S7h: kutunun ağzını tutar (topun 6 br önü, kaleye 10–20 br; top 10 br içindeyse yarı yol), kale dibine inmez
                            let kk = if s7(128) { 1.0 - jmax(jmin(10.0, 0.5 * g_e), jmin(20.0, g_e - 6.0)) / jmax(1.0, g_e) } else { 0.55 };
                            let (lx, ly) = (E.0 + kk * (own_x(dt) - E.0), E.1 + kk * (GCY - E.1));
                            let pp = &mut c.ps[p];
                            pp.tx = lx;
                            pp.ty = cl(ly + 0.25 * (bs.1 - ly), 3.0, GH - 3.0);
                            pp.sprint = true;
                            pp.job = Some("geri koşu · kale önü".into());
                            pp.hal_mut().tutQ = None;
                            continue;
                        }
                    }
                    let pp = &c.ps[p];
                    let ex = self.press.contains(&p) || bas(&pp.job, &["pres", "karşıla", "kovala", "sekme", "alıcı"]) || pp.hal._savD == Some(tick) || pp.hal._savT == Some(tick);
                    if ex || self.g_d(pp.tx, pp.ty) < g_e + pay {
                        // S7c: tehlike bölgesinde geçilmiş (topun gerisinde kalmış) savunmacı adamına değil kale önüne,
                        // taşıyıcının şut hattına döner
                        if s7(4) && !ex && g_e < S7_D && self.g_d(pp.x, pp.y) >= g_e + 1.0 {
                            let bs = self.base[p].map(|b| (b.x, b.y)).unwrap_or((pp.x, pp.y));
                            let (lx, ly) = (E.0 + 0.55 * (own_x(dt) - E.0), E.1 + 0.55 * (GCY - E.1));
                            let pp = &mut c.ps[p];
                            pp.tx = lx;
                            pp.ty = cl(ly + 0.25 * (bs.1 - ly), 3.0, GH - 3.0);
                            pp.sprint = true;
                            pp.job = Some("geri koşu · kale önü".into());
                            pp.hal_mut().tutQ = None;
                            continue;
                        }
                        if self.g_d(pp.x, pp.y) >= g_e + pay && !pp.sprint {
                            let pp = &mut c.ps[p];
                            pp.sprint = true;
                            pp.job = Some(format!("{} · geri", pp.job.as_deref().unwrap_or("")).into());
                        }
                        continue;
                    }
                    let bs = self.base[p].map(|b| (b.x, b.y)).unwrap_or((pp.x, pp.y));
                    let g = (E.0 + 0.35 * (own_x(dt) - E.0), cl(bs.1 + 0.3 * (GCY - bs.1), 3.0, GH - 3.0));
                    let pp = &mut c.ps[p];
                    pp.tx = g.0;
                    pp.ty = g.1;
                    pp.sprint = true;
                    pp.job = Some("geri koşu · kale".into());
                    pp.hal_mut().tutQ = None;
                }
                if let (Some(hh), true) = (h, g_e < S7_D && g_e > 4.0) {
                    let (ux, uy) = ((own_x(dt) - E.0) / g_e, (GCY - E.1) / g_e);
                    // S7i: ilk presçi taşıyıcının gerisindeyse ya da yarışı kaybedip kale önüne çekildiyse, kale tarafındaki en
                    // yakın savunmacı şut hattına, taşıyıcının önüne çıkar
                    let f = self.press.first().copied();
                    let geride = f.map_or(true, |f| {
                        let fp = &c.ps[f];
                        self.g_d(fp.x, fp.y) >= g_e - 0.5 || fp.job.as_deref() == Some("pres · kale önü")
                    });
                    if s7(256) && geride {
                        let s = jmin(S7H_ON, g_e - 2.0);
                        let (qx, qy) = (E.0 + ux * s, E.1 + uy * s);
                        let ad = self.defs.iter().copied().filter(|&d| Some(d) != f && !c.ps[d].bekci() && !self.up[d] && self.g_d(c.ps[d].x, c.ps[d].y) < g_e - 1.0).min_by(|&u, &v| sira(hypot(c.ps[u].x - qx, c.ps[u].y - qy) - hypot(c.ps[v].x - qx, c.ps[v].y - qy)));
                        // varış sınırı: şut hattı noktasına S7I_MAX'tan uzaktaki çıkamaz (analizci 6850: 19 br'deki çağrılıyordu)
                        let ad = ad.filter(|&d| hypot(c.ps[d].x - qx, c.ps[d].y - qy) <= S7I_MAX);
                        if let Some(d) = ad {
                            let pp = &mut c.ps[d];
                            pp.tx = qx;
                            pp.ty = qy;
                            pp.sprint = true;
                            pp.job = Some("pres · öne çık".into());
                            pp.hal_mut().tutQ = None;
                        }
                    }
                    // S7j: tehlike bölgesinde markajcı adamının yanında değil, adamıyla kale arasında (hattın üstünde) durur
                    if s7(512) {
                        for &d in &self.defs.clone() {
                            let pp = &c.ps[d];
                            if pp.bekci() || !bas(&pp.job, &["markaj"]) {
                                continue;
                            }
                            let Some(q) = pp.hal.tutQ.or(pp.hal.markRef).filter(|&q| q != hh) else { continue };
                            let (mx, my) = (c.ps[q].x, c.ps[q].y);
                            let l = self.g_d(mx, my);
                            if l >= S7_D || l < 2.0 {
                                continue;
                            }
                            let pp = &mut c.ps[d];
                            pp.tx = mx + (own_x(dt) - mx) / l * S7J_ARA;
                            pp.ty = my + (GCY - my) / l * S7J_ARA;
                        }
                    }
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

/// onde_ac: önde bekleyenin yerini yeniden tartma aralığı (tik).
pub const ONDE_AC_T: i64 = 30;
/// Deney (ALAN_ONDE_AC_IZ; ölçümde/wasm'da ONDE_AC_ZORLA): önde bekleyen oyuncu son çizgi boyunca açığa kayar; rakip topu
/// tutarken yürüyerek (bar dolar), top havadayken normal koşar (mac/enerji.rs sprint_karar3). Varsayılan kapalı.
pub static ONDE_AC_ZORLA: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub fn onde_ac() -> bool {
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    ONDE_AC_ZORLA.load(std::sync::atomic::Ordering::Relaxed) || *A.get_or_init(|| crate::iz_acik() && std::env::var("ALAN_ONDE_AC_IZ").is_ok())
}
