//! position'un hücum bölümü (güvence, kontra koşucuları, kanat, boşluk araması) ve son işler (yol kes,
//! geçilenler, Bekçi, yol ver, sınırlar).
#![allow(non_snake_case, clippy::needless_range_loop)]

use std::sync::Arc;

use crate::core_sabit::Q;
use crate::mat::{atan2, cos, exp, hypot, jmax, jmin, jsign, pow, sin, PI};
use crate::model::{JpKv, JpOtE, Pid};
use crate::oyuncu::Tepki;
use crate::saha::{KR, MOUTH};
use crate::temas::{alan_ic, kuyu_girer};

use super::super::gecici_dis as dis;
use super::super::takim::{yer_v, yol_p, TvAday};
use super::super::*;
use super::savunma::Sv;
use super::yardim::*;

/// Hücumcu başına sabitler (JS: runV, roleY, leashK … kapanışlarının okudukları).
struct Hk {
    genis_k: f64,
    kaleOnu: &'static str,
}

impl Sv {
    fn el_of(&self, p: Pid) -> f64 {
        match self.base[p].and_then(|b| b.el) {
            Some(e) if e == e => e,
            _ => 0.0,
        }
    }
    fn el_dar(&self, c: &Mc, p: Pid) -> bool {
        self.el_of(p) > 0.5 && tl0(&c.ps[p], "huc", "serbest") < 0.5
    }
    fn genis(&self, c: &Mc, p: Pid) -> bool {
        rslot_of(&c.ps[p]).1.abs() >= 0.75 || self.kanat_t[p].is_some()
    }
    fn yan_of(&self, c: &Mc, p: Pid) -> f64 {
        self.kanat_t[p].unwrap_or_else(|| jsign(rslot_of(&c.ps[p]).1))
    }
    fn role_leash(&self, c: &Mc, p: Pid) -> f64 {
        pow(2.0, 2.0 * (0.5 - tl0(&c.ps[p], "huc", "serbest")))
    }
    fn role_run(&self, c: &Mc, p: Pid) -> f64 {
        if self.runners[p] {
            0.0
        } else {
            0.6 * S.run_k * (tl0(&c.ps[p], "huc", "kosu") - 0.5) / 0.5
        }
    }
    fn vac_l(&self, c: &Mc, p: Pid) -> f64 {
        if self.el_of(p) != 0.0 {
            1.0 + (S.vac_leash - 1.0) * jmin(1.0, 2.0 * tl0(&c.ps[p], "huc", "serbest"))
        } else {
            S.vac_leash
        }
    }
    fn leash_k(&self, c: &Mc, p: Pid) -> f64 {
        (1.0 + S.durus_el * self.el_of(p)) * if self.win { if self.runners[p] { 0.01 } else { 2.5 } } else { self.role_leash(c, p) }
    }
    fn dog_l(&self, hk: &Hk) -> f64 {
        if hk.kaleOnu == "her" {
            0.0
        } else {
            S.vac_leash
        }
    }

    /// JS runV(p, c, sn): koşunun değeri (rakiplerden önce varılırsa tehdit).
    fn run_v(&self, c: &Mc, p: Pid, a: &Aday, sn: &[Varlik]) -> f64 {
        let k = c.k;
        let pp = &c.ps[p];
        let ta = (match a.kv {
            Some(kv) => hypot(kv.0 - pp.x, kv.1 - pp.y) + hypot(a.x - kv.0, a.y - kv.1),
            None => hypot(a.x - pp.x, a.y - pp.y),
        }) / v_sp(pp, k);
        let mut td: f64 = 1e9;
        for o in sn {
            if !o.bekci {
                let g = o.govde(c.ps);
                td = jmin(td, jmax(0.0, TEPKI(g, Tepki::Kovala, k)) + hypot(a.x - o.x, a.y - o.y) / v_sp(g, k));
            }
        }
        dis::threat(self.att, a.x, a.y) / (1.0 + exp((ta - td) / 8.0))
    }
    /// JS blokDis(sn, y).
    fn blok_dis(sn: &[Varlik], y: f64) -> f64 {
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
    fn role_y(&self, c: &Mc, hk: &Hk, p: Pid, a: &Aday, sn: &[Varlik]) -> f64 {
        let ds = tl0(&c.ps[p], "huc", "destek");
        let E = self.E;
        (if self.genis(c, p) { hk.genis_k * S.kanat_k * jmin(jmax(0.0, 13.0 * GKY - (a.y - GCY).abs()) / (13.0 * GKY), 1.0 - cl(Self::blok_dis(sn, a.y) / 8.0, 0.0, 1.0)) } else { 0.0 })
            + (if ds < 0.5 { jmin(1.5, (0.5 - ds) / 0.3) * S.kurucu_k * jmax(0.0, (a.x - E.0) * dir_of(self.att) + 2.0) / 10.0 } else { 0.0 })
            + (if (ds - 0.5).abs() > 0.1 { S.destek_k * jmin(4.0, pow((hypot(a.x - E.0, a.y - E.1) - (6.0 + 14.0 * ds)) / 8.0, 2.0)) } else { 0.0 })
    }

    /// Hücum: güvence, kontra koşucuları, kanat, boşluk araması.
    pub fn hucum(&mut self, c: &mut Mc) {
        let k = c.k;
        let tick = c.m.tick;
        let n = self.n;
        let (att, dt, E, h) = (self.att, self.dt, self.E, self.h);
        let hang0: Vec<Pid> = (0..n).filter(|&q| c.ps[q].takim() == dt && !c.ps[q].bekci() && (c.ps[q].x - E.0) * dir_of(att) < -8.0).collect();
        let G = c.m.gc.as_ref().filter(|g| g.team == att as f64 && g.git).cloned();
        {
            let risk = c.m.tac[att as usize].risk.unwrap_or(f64::NAN);
            let n_g = jmax(0.0, if G.is_some() { jmin(hang0.len() as f64, if risk > 0.7 { 0.0 } else { 1.0 }) } else { hang0.len() as f64 - if risk > 0.7 { 1.0 } else { 0.0 } }) as usize;
            let mut pool: Vec<Pid> = self.atk.iter().copied().filter(|&q| !self.chase[q] && !G.as_ref().map(|g| g.on.contains(&c.ps[q].id)).unwrap_or(false) && rslot_of(&c.ps[q]).0 < 0.6).collect();
            pool.sort_by(|&u, &v| sira(rslot_of(&c.ps[u]).0 - rslot_of(&c.ps[v]).0));
            let mut hang: Vec<(Pid, f64)> = hang0.iter().map(|&q| (q, dis::threat(dt, c.ps[q].x, c.ps[q].y))).collect();
            hang.sort_by(|a, b| sira(b.1 - a.1));
            for &(hq, _) in hang.iter().take(n_g) {
                let mut bi: Option<Pid> = None;
                let mut bd = 1e9;
                for &q in &pool {
                    if self.guard[q].is_some() {
                        continue;
                    }
                    let bq = self.base[q].unwrap_or(Yer { x: c.ps[q].x, y: c.ps[q].y, el: None });
                    let d = hypot(bq.x - c.ps[hq].x, bq.y - c.ps[hq].y) + 30.0 * rslot_of(&c.ps[q]).0 + 40.0 * match bq.el {
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
            genis_k: match c.m.tac.get(att as usize).and_then(|t| t.genislik.as_deref()) {
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
                    if self.atk.iter().any(|&q| !c.ps[q].bekci() && rslot_of(&c.ps[q]).1 * sd >= 0.75) {
                        continue;
                    }
                    let wx = cl(E.0 + 10.0 * dir_of(att), 3.0, GW - 3.0);
                    let wy = if sd < 0.0 { 4.0 } else { GH - 4.0 };
                    let mut bq: Option<Pid> = None;
                    let mut bc = 1e9;
                    for &q in &self.atk {
                        let qp = &c.ps[q];
                        let el = self.base[q].and_then(|b| b.el).map(|e| e != 0.0 && e == e).unwrap_or(false);
                        if q == hh || qp.bekci() || self.chase[q] || self.guard[q].is_some() || self.runners[q] || rslot_of(qp).1 * sd <= 0.0 || el {
                            continue;
                        }
                        let cc = hypot(wx - qp.x, wy - qp.y);
                        if cc < bc {
                            bc = cc;
                            bq = Some(q);
                        }
                    }
                    if let Some(bq) = bq {
                        self.kanat_t[bq] = Some(sd);
                        let ks = self.atk.iter().copied().find(|&q| !c.ps[q].bekci() && rslot_of(&c.ps[q]).1 * -sd >= 0.75);
                        let ev = match ks {
                            Some(q) => {
                                let ka = rslot_of(&c.ps[q]);
                                (ka.0, -ka.1)
                            }
                            None => (0.75, 0.85 * sd),
                        };
                        let b = slot_base0(c.m, c.ps, &c.ps[bq], Some(ev), E.0, E.1, true);
                        self.base[bq] = Some(Yer { x: b.0, y: b.1, el: None });
                    }
                }
            }
        }
        if let Some(hh) = h {
            if c.ps[hh].takim() == att {
                c.ps[hh].hal_mut().otx = None;
            }
        }
        c.ek.kp_c = None;
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
                    let key = format!("{}|{}|{}", (tick / 4.0).floor(), att, id_str(c.ps[hc].id));
                    let gec = matches!(&c.ek.kay_l, Some((kk, _)) if *kk == key);
                    if !gec {
                        let ch = yada(c.m.ch, 0.3);
                        let f = dis::kayip_yer(c.kafa, c.ps, att, ch, &c.ps[hc], c.m.tac.get(att as usize));
                        c.ek.kay_l = Some((key, f));
                    }
                    let L = c.ek.kay_l.as_ref().unwrap().1.clone();
                    let le = L(c.kafa, E.0, E.1);
                    if le * S.W.tv_pass > 0.003 {
                        c.ek.kp_c = Some(KpC { L, le, Hr, k_h: 0.0, mut_: 0, kc: Default::default() });
                    }
                }
            }
        }
        for &p in &self.atk.clone() {
            self.hucumcu(c, &hk, p);
        }
        let _ = (tick, k);
    }

    /// Seyrek yerleşim (yalnız Rust, davranış değişir): topa SEYREK_R'den uzak hücumcunun işi 3 tikte bir
    /// hesaplanır (oyuncular tiklere dağılır: (tik + id) % 3); aradaki tiklerde son hesabın hedefi, işi ve
    /// sprinti kalır (en çok 2 tik eski). Kovalayan, güvence veren, taşıyıcı ve kafanın hayali maçı her tik.
    /// Eskisi (her tik): ALAN_SEYREK_ESKI_IZ.
    fn hucumcu(&mut self, c: &mut Mc, hk: &Hk, p: Pid) {
        if !seyrek_acik() || c.lite {
            return self.hucumcu0(c, hk, p);
        }
        let tick = c.m.tick;
        let n = c.ps.len();
        if c.ek.seyrek.len() != n {
            c.ek.seyrek = vec![None; n];
        }
        if self.chase[p] || self.guard[p].is_some() || Some(p) == self.h {
            c.ek.seyrek[p] = None;
            return self.hucumcu0(c, hk, p);
        }
        let uzak = hypot(c.ps[p].x - self.E.0, c.ps[p].y - self.E.1) > SEYREK_R;
        let sira = (tick + c.ps[p].id) % 3.0;
        // ara tik: arama gerekmiyorsa her zamanki ucuz yol (eskisiyle aynı); gerekiyorsa atlanmaz, ertelenir:
        // otx silinir (bir sonraki hesap tikinde arama kesin yapılır), o zamana dek son hesabın hedefi kalır
        if uzak && sira != 0.0 && sira == sira && self.arama_gerek(c, p) {
            if let Some((t, tx, ty, job, sprint)) = c.ek.seyrek[p].clone() {
                if tick - t <= 2.0 {
                    let pp = &mut c.ps[p];
                    pp.hal_mut().otx = None;
                    pp.tx = tx;
                    pp.ty = ty;
                    pp.job = job;
                    pp.sprint = sprint;
                    return;
                }
            }
        }
        self.hucumcu0(c, hk, p);
        let pp = &c.ps[p];
        c.ek.seyrek[p] = Some((tick, pp.tx, pp.ty, pp.job.clone(), pp.sprint));
    }

    /// hucumcu0 bu tik boşluk araması yapacak mı (yan etkisiz; yalnız gerçek maç, lite değil): otx yok, pencere
    /// açıldı, ya da eldeki hedefi tutma koşulları sağlanmıyor (hucumcu0'daki kalabalik/c2/c3/c4 ile aynı).
    fn arama_gerek(&self, c: &Mc, p: Pid) -> bool {
        let k = c.k;
        let tick = c.m.tick;
        let (att, E, h) = (self.att, self.E, self.h);
        if self.win && c.ps[p].hal._winSeen != c.m.win_t {
            return true;
        }
        let otx = match c.ps[p].hal.otx {
            Some(x) => x,
            None => return true,
        };
        let every = jmax(8.0, jround(24.0 - yer_f(&c.ps[p])));
        let fr_first = self.fr.as_ref().and_then(|f| f.first);
        let oty = c.ps[p].hal.oty.unwrap_or(f64::NAN);
        let pp = &c.ps[p];
        let kalabalik = self.atk.iter().any(|&q| q != p && Some(q) != h && !self.chase[q] && hypot(c.ps[q].tx - otx, c.ps[q].ty - oty) < S.ayr);
        let c2 = match h {
            Some(hh) if pp.hal._otB.is_some() => {
                let hb = BindH::of(&c.ps[hh], hh);
                (otx - c.ps[hh].x) * dir_of(att) < S.bind_on || bind_cak(pp, otx, oty, &hb, k) < S.bind_yol
            }
            _ => false,
        };
        let c3 = match h {
            Some(hh) if c.ps[hh].takim() == att && hh != p => {
                let hb = BindH::of(&c.ps[hh], hh);
                bind_cak(pp, otx, oty, &hb, k) < S.bind_yol
            }
            _ => false,
        };
        let c4 = (pp.hal._otH == h
            && (h.is_some() || (pp.hal._otE.is_some() && pp.hal._otF == fr_first && { let e = pp.hal._otE.as_ref().unwrap(); hypot(E.0 - e.x, E.1 - e.y) < 8.0 }))
            && hypot(otx - pp.x, oty - pp.y) > 1.5)
            || {
                let r = (tick + pp.id) % every;
                r != 0.0 && r == r
            };
        !(!kalabalik && !c2 && !c3 && c4)
    }

    /// Bir hücumcunun işi (JS `for (const p of atk) {…}` gövdesi).
    fn hucumcu0(&mut self, c: &mut Mc, hk: &Hk, p: Pid) {
        let k = c.k;
        let tick = c.m.tick;
        let (att, E, h) = (self.att, self.E, self.h);
        let lite = c.lite;
        if self.chase[p] {
            c.ps[p].hal_mut().otx = None;
            return;
        }
        if let Some(gq) = self.guard[p] {
            let g = gside(c.ps[gq].x, c.ps[gq].y, att, 3.0 + 0.2 * (20.0 - yer_s(&c.ps[p])));
            let pp = &mut c.ps[p];
            pp.tx = g.0;
            pp.ty = g.1;
            pp.job = Some("güvence".into());
            pp.hal_mut().otx = None;
            return;
        }
        let every = jmax(8.0, jround(24.0 - yer_f(&c.ps[p])));
        if lite && c.ps[p].hal.otx.is_none() {
            let b = self.base[p].unwrap();
            let pp = &mut c.ps[p];
            pp.hal_mut().otx = Some(b.x);
            pp.hal_mut().oty = Some(b.y);
            pp.hal_mut()._kv = None;
        }
        if self.win && c.ps[p].hal._winSeen != c.m.win_t && !lite {
            c.ps[p].hal_mut()._winSeen = c.m.win_t;
            c.ps[p].hal_mut().otx = None;
        }
        let fr_first = self.fr.as_ref().and_then(|f| f.first);
        if let Some(otx) = c.ps[p].hal.otx {
            let oty = c.ps[p].hal.oty.unwrap_or(f64::NAN);
            let pp = &c.ps[p];
            let kalabalik = !lite && self.atk.iter().any(|&q| q != p && Some(q) != h && !self.chase[q] && hypot(c.ps[q].tx - otx, c.ps[q].ty - oty) < S.ayr);
            let c2 = match h {
                Some(hh) if pp.hal._otB.is_some() && !lite => {
                    let hb = BindH::of(&c.ps[hh], hh);
                    (otx - c.ps[hh].x) * dir_of(att) < S.bind_on || bind_cak(pp, otx, oty, &hb, k) < S.bind_yol
                }
                _ => false,
            };
            let c3 = match h {
                Some(hh) if !lite && c.ps[hh].takim() == att && hh != p => {
                    let hb = BindH::of(&c.ps[hh], hh);
                    bind_cak(pp, otx, oty, &hb, k) < S.bind_yol
                }
                _ => false,
            };
            let c4 = lite
                || (pp.hal._otH == h
                    && (h.is_some() || (pp.hal._otE.is_some() && pp.hal._otF == fr_first && { let e = pp.hal._otE.as_ref().unwrap(); hypot(E.0 - e.x, E.1 - e.y) < 8.0 }))
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
                if let Some(kv) = pp.hal._kv.clone() {
                    if hypot(kv.x - pp.x, kv.y - pp.y) < 1.2 || tick > kv.t {
                        pp.hal_mut()._kv = None;
                    } else {
                        pp.tx = kv.x;
                        pp.ty = kv.y;
                    }
                }
                pp.job = Some(if runner { "kontra koşusu".into() } else { pp.hal._amacJob.clone().filter(|s| !s.is_empty()).unwrap_or_else(|| "boşluk".into()) });
                pp.sprint = runner || hypot(pp.tx - pp.x, pp.ty - pp.y) > S.W.run_sprint;
                return;
            }
        }
        let bs = {
            let b0 = self.base[p].unwrap();
            match h {
                Some(hh) if c.ps[hh].takim() == att && hh != p => {
                    let hp = &c.ps[hh];
                    let ayr = if hp.bekci() { S.ev_ayr_bek } else { S.ev_ayr };
                    let dx = b0.x - hp.x;
                    let dy = b0.y - hp.y;
                    let dd = hypot(dx, dy);
                    if dd >= ayr {
                        b0
                    } else {
                        let ux = if dd > 0.3 { dx / dd } else { dir_of(att) };
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
        if let Some(otx) = pp.hal.otx {
            cands.push(Aday::xy(otx, pp.hal.oty.unwrap_or(f64::NAN)));
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
        let last_d = sn.iter().filter(|o| !o.bekci).fold(0.0f64, |mx, o| jmax(mx, (o.x - own_x(att)) * dir_of(att)));
        let el_dar = self.el_dar(c, p);
        let kosu = tl0(&pp, "huc", "kosu");
        if !el_dar && !pp.bekci() && kosu >= 0.25 {
            for yy in [pp.y, bs.y, 15.0 * GKY, 35.0 * GKY] {
                cands.push(Aday::xy(own_x(att) + dir_of(att) * jmin(GW - 6.0, last_d + 4.0), yy));
            }
        }
        let h_att = h.filter(|&hh| c.ps[hh].takim() == att && hh != p);
        if !el_dar && !pp.bekci() && h_att.is_some() {
            let dr = dir_of(att);
            let dep = |o: &Varlik| (o.x - own_x(att)) * dr;
            let mut op: Vec<&Varlik> = sn.iter().filter(|o| !o.bekci).collect();
            op.sort_by(|u, v| sira(dep(v) - dep(u)));
            if op.len() >= 4 {
                let d_l = (dep(op[0]) + dep(op[1])) / 2.0;
                let m_l = (dep(op[2]) + dep(op[3])) / 2.0;
                if d_l - m_l >= 6.0 && (E.0 - own_x(att)) * dr < m_l - 2.0 {
                    let mut ac: Vec<(f64, f64, f64)> = Vec::new();
                    for f in [0.4, 0.7] {
                        let xx = own_x(att) + dr * (m_l + (d_l - m_l) * f);
                        let mut y = 4.0;
                        while y <= GH - 4.0 {
                            let mut nd: f64 = 1e9;
                            for o in &sn {
                                if !o.bekci {
                                    nd = jmin(nd, hypot(o.x - xx, o.y - y));
                                }
                            }
                            if nd >= 4.0 {
                                let lo = lane_open(c.kafa, &sn, c.ps, att, E, (xx, y), None, k, c.d);
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
        if h_att.is_some() {
            let mut kd: Option<usize> = None;
            let mut kdd = 6.0;
            for (oi, o) in sn.iter().enumerate() {
                if o.bekci || o.r.is_none() || c.ps[o.r.unwrap()].hal.tutQ != Some(p) {
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
                let gl = nz1(hypot(opp_x(att) - kdv.x, GCY - kdv.y));
                let ux = (opp_x(att) - kdv.x) / gl;
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
            let n_v = jmax(0.0, jround((yer_f(&pp) - S.vac_ok0) / S.vac_ok_step)) as usize;
            if n_v > 0 {
                let mut vac: Vec<(f64, f64, f64)> = Vec::new();
                for &d in &self.defs {
                    let bd = c.ps[d].hal.past.as_ref().filter(|v| v.len() >= 3).map(|v| (v[0].x, v[0].y));
                    let sd = sn.iter().find(|o| o.r == Some(d));
                    let (bd, sd) = match (bd, sd) {
                        (Some(b), Some(s)) => (b, s),
                        _ => continue,
                    };
                    if hypot(sd.x - bd.0, sd.y - bd.1) < S.vac_left {
                        continue;
                    }
                    let mut near: f64 = 1e9;
                    for o in &sn {
                        if !o.bekci {
                            near = jmin(near, hypot(o.x - bd.0, o.y - bd.1));
                        }
                    }
                    if !(near < S.vac_empty) {
                        vac.push((bd.0, bd.1, dis::threat(att, bd.0, bd.1)));
                    }
                }
                vac.sort_by(|a, b| sira(b.2 - a.2));
                for v in vac.iter().take(n_v) {
                    cands.push(Aday { x: v.0, y: v.1, vac: true, ..Default::default() });
                    cands.push(Aday { x: v.0 + 4.0 * dir_of(att), y: v.1, vac: true, ..Default::default() });
                }
            }
        }
        if self.genis(c, p) || (!pp.bekci() && !el_dar) {
            let yy = if (if self.genis(c, p) { self.yan_of(c, p) } else { bs.y - GCY }) < 0.0 { 4.0 } else { GH - 4.0 };
            for dx in [-8.0, 0.0, 8.0, 16.0] {
                cands.push(Aday::xy(cl(bs.x + dir_of(att) * dx, 3.0, GW - 3.0), yy));
            }
        }
        let flq = c.m.fl.as_ref().and_then(|f| f.q);
        if h.is_none() && flq.map(|q| c.ps[q].takim() == att && q != p).unwrap_or(false) && self.fr.is_some() && fr_first == flq && !el_dar {
            let B = flq.unwrap();
            let dr = dir_of(att);
            let ga = atan2(GCY - E.1, opp_x(att) - E.0);
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
        let b_h: Option<Pid> = match h {
            Some(hh) => if c.ps[hh].takim() == att { Some(hh) } else { None },
            None => flq.filter(|&q| c.ps[q].takim() == att && self.fr.is_some() && fr_first == Some(q)),
        };
        if let Some(bh) = b_h.filter(|&b| b != p && !c.ps[b].bekci()) {
            let hb = &c.ps[bh];
            let b_ah = h == Some(bh);
            let hh = BindH { x: if b_ah { hb.x } else { E.0 }, y: if b_ah { hb.y } else { E.1 }, vx: if b_ah { hb.vx } else { 0.0 }, vy: if b_ah { hb.vy } else { 0.0 }, tx: if b_ah { Some(hb.tx) } else { None }, ty: if b_ah { Some(hb.ty) } else { None }, hz: hb.hz(), pid: bh };
            let dr = dir_of(att);
            let ileri = (hh.x - pp.x) * dr;
            let bt = dis::tl(&pp, "huc", "bindir");
            let tal = matches!(bt, Some(b) if b != 0.0 && b == b && hb.govde.i == b);
            let side = if (hh.y - GCY).abs() > 4.0 { jsign(hh.y - GCY) } else { yada(jsign(pp.y - hh.y), 1.0) };
            let kosul = if tal { ileri > -5.0 && ileri < 40.0 && (hh.x - own_x(att)) * dr > 25.0 * GKX } else { !el_dar && (hh.y - GCY).abs() > 8.0 * GKY && ileri > 3.0 && ileri < 25.0 && (pp.y - hh.y).abs() < 22.0 && (hh.x - own_x(att)) * dr > 35.0 * GKX };
            if kosul {
                let dis_ = jmax(1.5, jmin(7.0, if side > 0.0 { GH - 2.0 - hh.y } else { hh.y - 2.0 }));
                for f in [10.0, 16.0] {
                    let x = cl(hh.x + dr * f, 3.0, GW - 3.0);
                    for (y, ad) in [(hh.y + side * dis_, "dış"), (cl(hh.y - 7.0 * side, 3.0, GH - 3.0), "iç")] {
                        if bind_cak(&pp, x, y, &hh, k) >= S.bind_yol {
                            cands.push(Aday { x, y, bind: Some(ad), bh: Some(hh), bind_t: tal, ..Default::default() });
                        }
                    }
                }
            }
        }
        let gx0 = (E.0 - opp_x(att)).abs() < 40.0 * GKX;
        if (!el_dar || hk.kaleOnu == "her") && gx0 && !pp.bekci() && kosu >= 0.25 && tl0(&pp, "huc", "destek") >= 0.3 {
            for r in [11.0, 19.0] {
                for an in [-0.8, -0.27, 0.27, 0.8] {
                    cands.push(Aday::xy(opp_x(att) - dir_of(att) * r * cos(an), GCY + r * sin(an)));
                }
            }
        }
        if hk.kaleOnu != "girme" && h_att.is_some() && !pp.bekci() && gx0 {
            let hh = h_att.unwrap();
            let key = format!("{}|{}|{}", id_str(c.m.tick), att, id_str(c.ps[hh].id));
            let ak = match &c.ek.acik_k {
                Some((kk, v)) if *kk == key => v.clone(),
                _ => {
                    let mut ak: Vec<(f64, f64, f64)> = Vec::new();
                    let gx = opp_x(att);
                    let dr = dir_of(att);
                    for r in [11.0, 15.0, 19.0] {
                        for an in [-0.8, -0.4, 0.0, 0.4, 0.8] {
                            let P = (cl(gx - dr * r * cos(an), 2.0, GW - 2.0), cl(GCY + r * sin(an), 2.0, GH - 2.0));
                            if hypot(P.0 - gx, P.1 - GCY) < KR + 0.8 {
                                continue;
                            }
                            let yv = yol_p(c.kafa, &sn, c.ps, att, E, P, c.d);
                            if yv > 0.3 {
                                ak.push((P.0, P.1, yv));
                            }
                        }
                    }
                    ak.sort_by(|a, b| sira(b.2 - a.2));
                    ak.truncate(4);
                    c.ek.acik_k = Some((key, ak.clone()));
                    ak
                }
            };
            for P in ak {
                cands.push(Aday { x: P.0, y: P.1, dogal: true, ..Default::default() });
            }
        }
        if hk.kaleOnu != "girme" && h_att.is_some() && !pp.bekci() && gx0 {
            let gx = opp_x(att);
            let dr = dir_of(att);
            let yb = if E.1 >= GCY { 1.0 } else { -1.0 };
            let nok = |dy: f64, r: f64| {
                let xx = (jmax(0.0, r * r - dy * dy)).sqrt();
                Aday { x: cl(gx - dr * xx, 2.0, GW - 2.0), y: cl(GCY + dy, 2.0, GH - 2.0), dogal: true, ..Default::default() }
            };
            match &*pp.govde.role {
                "Uç" => {
                    cands.push(nok(yb * MOUTH, KR + 1.0));
                    cands.push(nok(0.0, KR + 2.5));
                }
                "Kanat" => cands.push(nok(-yb * (MOUTH + 1.0), KR + 1.0)),
                "Orta" => cands.push(nok(0.0, KR + 9.0)),
                _ => {}
            }
        }
        let mate_objs: Vec<(f64, f64)> = self.atk.iter().copied().filter(|&q| q != p && !self.chase[q]).map(|q| (c.ps[q].tx, c.ps[q].ty)).collect();
        if let Some(hh) = h_att {
            let hb = BindH::of(&c.ps[hh], hh);
            let iy: Vec<Aday> = cands.iter().filter(|a| a.x >= 2.0 && a.x <= GW - 2.0 && a.y >= 2.0 && a.y <= GH - 2.0 && bind_cak(&pp, a.x, a.y, &hb, k) >= S.bind_yol).cloned().collect();
            if !iy.is_empty() {
                cands = iy;
            }
        }
        let svi = p;
        let mut pre: Vec<(Aday, f64)> = Vec::new();
        for a in cands.iter().filter(|a| a.x >= 2.0 && a.x <= GW - 2.0 && a.y >= 2.0 && a.y <= GH - 2.0) {
            let lsh = jmax(0.0, hypot(a.x - bs.x, a.y - bs.y) - S.ev_bolge) / 12.0 * if a.dogal { self.dog_l(hk) } else if a.vac || a.uc.is_some() || a.bind.is_some() || a.hat { self.vac_l(c, p) * if a.bind_t { 0.5 } else { 1.0 } } else { 1.0 };
            let rv = self.run_v(c, p, a, &sn);
            let run = (if self.runners[p] { S.run_k * rv } else { 0.0 }) + self.role_run(c, p) * rv;
            let ry = self.role_y(c, hk, p, a, &sn);
            let tv = {
                let ta = TvAday { x: a.x, y: a.y, kv: a.kv, kd: a.kd, taban: false };
                match (a.kd, a.kv) {
                    (Some(kdi), Some(kv)) => {
                        let (x0, y0) = (sn[kdi].x, sn[kdi].y);
                        let g = gside(kv.0, kv.1, 1 - att, S.mark_off);
                        sn[kdi].x = g.0;
                        sn[kdi].y = g.1;
                        if let Some(kp) = c.ek.kp_c.as_mut() {
                            kp.mut_ += 1;
                        }
                        let v = team_v(c, &mut sn, svi, att, E, ta, &mate_objs, 0.0, Some(p), Some(p));
                        sn[kdi].x = x0;
                        sn[kdi].y = y0;
                        if let Some(kp) = c.ek.kp_c.as_mut() {
                            kp.mut_ -= 1;
                        }
                        v
                    }
                    _ => team_v(c, &mut sn, svi, att, E, ta, &mate_objs, 0.0, Some(p), Some(p)),
                }
            };
            let kb = self.komb(c, p, a, &sn, h_att);
            let q = run - ry + tv - S.W.leash * self.leash_k(c, p) * lsh * lsh + kb;
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
        let ch_c = yada(c.m.ch, 0.3);
        let krf = self.krf(c, p, &sn, h_att);
        let mut best: Option<Aday> = None;
        let mut bv = -1e9;
        let mut best_ck: Option<(f64, Pid, Pid)> = None;
        let y_h = yer_h(&pp);
        for (a, _) in &pre {
            let crw = mate_objs.iter().fold(1.0f64, |acc, q| jmin(acc, hypot(q.0 - a.x, q.1 - a.y) / S.ayr));
            let lsh = jmax(0.0, hypot(a.x - bs.x, a.y - bs.y) - S.ev_bolge) / 12.0
                * if a.dogal {
                    1.0 - crw * (1.0 - self.dog_l(hk))
                } else if a.vac || a.uc.is_some() || a.bind.is_some() || a.hat {
                    1.0 - crw * (1.0 - self.vac_l(c, p) * if a.bind_t { 0.5 } else { 1.0 })
                } else {
                    1.0
                };
            let ck = self.cek_amac(c, p, a, &sn, h_att, ch_c);
            let ky_f = if h_att.is_some() { yol_p(c.kafa, &sn, c.ps, att, E, (a.x, a.y), c.d) } else { 1.0 };
            let rv = self.run_v(c, p, a, &sn);
            let run = (if self.runners[p] { S.run_k * rv } else { 0.0 }) + self.role_run(c, p) * rv;
            let kb = self.komb(c, p, a, &sn, h_att);
            let ry = self.role_y(c, hk, p, a, &sn);
            let tv = team_v(c, &mut sn, svi, att, E, TvAday { x: a.x, y: a.y, kv: a.kv, kd: a.kd, taban: false }, &mate_objs, y_h, Some(p), None);
            let v = crw * kb + crw * (1.0 - 0.0) * ky_f * run - ry + S.W.tv * tv - S.W.leash * self.leash_k(c, p) * lsh * lsh * (1.0 + 2.0 * 0.0) - 0.0 - match &krf {
                Some(f) => f(a.x, a.y),
                None => 0.0,
            } + hsh(pp.id, jround(3.0 * a.x + 7.0 * a.y), (tick / 30.0).floor()) * (20.0 - y_h) * S.W.noise * 2.0
                + match ck {
                    Some(ck) => crw * ck.0,
                    None => 0.0,
                };
            if v > bv {
                bv = v;
                best = Some(a.clone());
                best_ck = ck;
            }
        }
        let best = best.unwrap_or_else(|| Aday::xy(cl(pp.x, 2.0, GW - 2.0), cl(pp.y, 2.0, GH - 2.0)));
        let fr_first = self.fr.as_ref().and_then(|f| f.first);
        let genis = self.genis(c, p);
        let bdis = Self::blok_dis(&sn, best.y);
        let pm = &mut c.ps[p];
        pm.hal_mut().otx = Some(best.x);
        pm.hal_mut().oty = Some(best.y);
        pm.hal_mut()._otH = h;
        pm.hal_mut()._otB = best.bh.map(|b| b.pid);
        pm.hal_mut()._otE = Some(JpOtE { x: E.0, y: E.1 });
        pm.hal_mut()._otF = if self.fr.is_some() { fr_first } else { None };
        pm.tx = best.x;
        pm.ty = best.y;
        pm.hal_mut()._kv = best.kv.map(|kv| JpKv { x: kv.0, y: kv.1, t: tick + 40.0 });
        if let Some(kv) = &pm.hal._kv {
            pm.tx = kv.x;
            pm.ty = kv.y;
        }
        pm.job = Some(if best.vac {
            "açılan boşluk".into()
        } else {
            match &pm.govde.rh {
                Some(rh) if !rh.is_empty() && &**rh != "Serbest" => rh.to_lowercase().into(),
                _ => "boşluk".into(),
            }
        });
        pm.hal_mut()._amacJob = None;
        let ad = |c: &Mc, q: Pid| c.ps[q].govde.name.clone();
        let yeni: Option<String> = if let Some(ck) = best_ck.filter(|ck| ck.0 >= S.cek_etiket) {
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
            pm.hal_mut()._amacJob = pm.job.clone();
        }
        pm.sprint = hypot(pm.tx - pm.x, pm.ty - pm.y) > S.W.run_sprint;
    }

    /// JS komb(c): üçüncü adam (uç) ve bindirme koşularının katkısı.
    fn komb(&self, c: &Mc, p: Pid, a: &Aday, sn: &[Varlik], _h_att: Option<Pid>) -> f64 {
        let k = c.k;
        let pp = &c.ps[p];
        if let Some(u) = a.uc {
            let t_b = self.fr.as_ref().map(|f| f.arr[u].t).unwrap_or(30.0);
            let d = hypot(a.x - pp.x, a.y - pp.y);
            let ta = gec(pp, a.x, a.y, Some(d), k) + d / v_sp(pp, k);
            return if ta > t_b + 20.0 { 0.0 } else { S.ucK * dis::lane_ok0(c.kafa, sn, c.ps, self.att, self.E, (a.x, a.y)) };
        }
        if let (Some(_), Some(bh)) = (a.bind, a.bh) {
            if bind_cak(pp, a.x, a.y, &bh, k) < S.bind_yol {
                return 0.0;
            }
            let (mut n_h, mut n_c) = (0, 0);
            for o in sn {
                if !o.bekci {
                    if hypot(o.x - bh.x, o.y - bh.y) < 10.0 {
                        n_h += 1;
                    }
                    if hypot(o.x - a.x, o.y - a.y) < 8.0 {
                        n_c += 1;
                    }
                }
            }
            return if n_c == 0 {
                S.bind_k * (if a.bind_t { 1.5 } else { 1.0 }) * if n_h <= 1 { 1.0 } else if n_h == 2 { 0.5 } else if a.bind_t { 0.25 } else { 0.0 }
            } else {
                0.0
            };
        }
        0.0
    }

    /// JS cekAmac(c): koşu markajcısını çekip bir arkadaşın pas yolunu açıyor mu → (değer, arkadaş, çekilen).
    fn cek_amac(&self, c: &Mc, p: Pid, a: &Aday, sn: &[Varlik], h_att: Option<Pid>, ch_c: f64) -> Option<(f64, Pid, Pid)> {
        let h = h_att?;
        let (att, E) = (self.att, self.E);
        let pp = &c.ps[p];
        let mut d: Option<usize> = None;
        let mut dd = 1e9;
        for (oi, o) in sn.iter().enumerate() {
            if o.bekci || o.r.is_none() {
                continue;
            }
            let dist = hypot(o.x - pp.x, o.y - pp.y);
            if ((c.ps[o.r.unwrap()].hal.tutQ == Some(p) && dist < 12.0) || dist < 5.0) && dist < dd {
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
        if d.is_none() && (E.0 - opp_x(att)).abs() < 40.0 * GKX {
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
                gk = Some(gside(a.x, a.y, 1 - att, S.mark_off));
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
            let l0 = yol_p(c.kafa, sn, c.ps, att, E, (qp.x, qp.y), c.d);
            let l1 = yol_p(c.kafa, &sn2, c.ps, att, E, (qp.x, qp.y), c.d);
            if l1 - l0 <= 0.05 {
                continue;
            }
            let g = (l1 - l0) * yer_v(c.kafa, &sn2, c.ps, att, qp.x, qp.y, ch_c);
            if g > bg {
                bg = g;
                bq = Some(q);
            }
        }
        let dref = sn[di].r?;
        bq.map(|q| (S.cek_k * bg, q, dref))
    }

    /// JS KRf: topu kaybedersek rakibin kontrası; adayın onu kesebilmesi.
    fn krf(&self, c: &Mc, p: Pid, sn: &[Varlik], h_att: Option<Pid>) -> Option<Box<dyn Fn(f64, f64) -> f64>> {
        let k = c.k;
        let h = h_att?;
        let att = self.att;
        let hp = &c.ps[h];
        let pk = 1.0 - cl(dis::keep_p(c.kafa, hp, sn, c.ps, None, c.d.recv_keep_t), 0.0, 1.0);
        if !(pk > 0.001) {
            return None;
        }
        let g_o = own_x(att);
        let ch0 = yada(c.m.ch, 0.3);
        let our: Vec<(f64, f64, f64)> = self.atk_all.iter().copied().filter(|&q| q != p && !c.ps[q].bekci()).map(|q| if q == h { (hp.x, hp.y, v_sp(hp, k)) } else { (c.ps[q].tx, c.ps[q].ty, v_sp(&c.ps[q], k)) }).collect();
        let mut OP: Vec<((f64, f64), f64, f64, f64)> = Vec::new();
        for o in sn.iter().filter(|o| !o.bekci && o.team != att) {
            let gx = g_o - o.x;
            let gy = GCY - o.y;
            let gl = nz1(hypot(gx, gy));
            let d = jmin(10.0, gl);
            let a = (o.x + gx / gl * d, o.y + gy / gl * d);
            let g = gside(a.0, a.1, att, 2.0);
            let t_o = d / v_sp(o.r0(c.ps), k);
            let V = jmax(0.0, yer_v(c.kafa, sn, c.ps, 1 - att, a.0, a.1, ch0));
            let mut u0 = 1.0;
            for q in &our {
                u0 *= 1.0 - 1.0 / (1.0 + exp((hypot(g.0 - q.0, g.1 - q.1) / q.2 - t_o) / 5.0));
            }
            if V > 0.0 {
                OP.push((g, t_o, V, u0));
            }
        }
        let vp = v_sp(&c.ps[p], k);
        let sk = S.W.tv * S.W.tv_pass * pk;
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
                let mut q_o: Option<Pid> = None;
                let (mut e_o, mut e_b) = (1e9, 1e9);
                for (q, r) in fr.arr.iter().enumerate() {
                    let qp = &c.ps[q];
                    if qp.bekci() || qp.noTouch {
                        continue;
                    }
                    if qp.takim() == att {
                        e_b = jmin(e_b, r.est);
                    } else if r.est < e_o {
                        e_o = r.est;
                        q_o = Some(q);
                    }
                }
                if let Some(q_o) = q_o.filter(|_| e_b > e_o + 6.0) {
                    let A = fr.arr[q_o];
                    let mut al = vec![false; n];
                    let es: Vec<Pid> = (0..n).filter(|&o| { let op = &c.ps[o]; op.takim() != att && o != q_o && !op.bekci() && !op.noTouch }).collect();
                    let mut ord: Vec<Pid> = self.atk.iter().copied().filter(|&p| !self.chase[p] && self.guard[p].is_none() && !c.ps[p].bekci()).collect();
                    ord.sort_by(|&u, &v| sira(hypot(c.ps[u].x - A.x, c.ps[u].y - A.y) - hypot(c.ps[v].x - A.x, c.ps[v].y - A.y)));
                    for p in ord {
                        let mut bo: Option<Pid> = None;
                        let mut b_l = (0.0, 0.0);
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
                                b_l = L;
                            }
                        }
                        if let Some(bo) = bo {
                            al[bo] = true;
                            let nm = c.ps[bo].govde.name.clone();
                            let pp = &mut c.ps[p];
                            pp.tx = b_l.0;
                            pp.ty = b_l.1;
                            pp.hal_mut().otx = None;
                            pp.hal_mut()._kv = None;
                            pp.sprint = bd > 3.0;
                            let j = format!("yol kes · {}", nm);
                            pp.job = Some(j.into());
                            pp.hal_mut()._amacJob = pp.job.clone();
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
            pp.hal_mut()._yer = hypot(pp.vx, pp.vy) < 0.05 && db < 1.0;
            let gx0 = own_x(pp.takim());
            let ux = E.0 - gx0;
            let uy = E.1 - GCY;
            let L = nz1(hypot(ux, uy));
            let hat_d = ((pp.x - gx0) * uy - (pp.y - GCY) * ux).abs() / L;
            if db > 3.0 && hat_d > 1.5 {
                let tt = cl(((pp.x - gx0) * ux + (pp.y - GCY) * uy) / (L * L), 0.0, jmax(0.0, jmin((L - 2.0) / L, (hypot(b.x - gx0, b.y - GCY) + 2.0) / L)));
                pp.tx = gx0 + ux * tt;
                pp.ty = GCY + uy * tt;
                pp.job = Some("Bekçi · dönüş".into());
                pp.sprint = true;
            }
        }
        self.yol_ver(c);
        for i in 0..n {
            let fq = c.m.fl.as_ref().and_then(|f| f.q);
            let kuyu_top = c.m._kuyu_top;
            let pp = &mut c.ps[i];
            let kx = if pp.bekci() { 1.0 } else { 1.5 };
            pp.tx = cl(pp.tx, kx, GW - kx);
            pp.ty = cl(pp.ty, 1.5, GH - 1.5);
            if !pp.bekci() {
                if let Some(q) = dis::kuyu_safe(pp.tx, pp.ty) {
                    pp.tx = q.0;
                    pp.ty = q.1;
                }
            }
            if !pp.bekci() && Some(i) != c.m.holder && (kuyu_top.is_none() || kuyu_top != Some(if pp.tx < GW / 2.0 { 0.0 } else { 1.0 })) {
                let v = jmax(0.05, spd(pp) * alan_spre(pp, k));
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
    fn yol_ver(&mut self, c: &mut Mc) {
        let h_s = c.m.holder;
        let so = h_s.and_then(|h| c.ps[h].hal.savur.as_ref().map(|s| (s.o.clone(), s.t0)));
        if let (Some(h), Some((_, t0))) = (h_s, &so) {
            c.ek.sh.yv_bas = Some((h, *t0));
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
        let b: Option<B> = match (&so, h_s) {
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
            let lat_a = pe + (-pp.vx * uy + pp.vy * ux) * jmax(0.0, al) / sp;
            if pe.abs() > 2.0 && lat_a.abs() > 2.0 {
                continue;
            }
            let sg = if lat_a.abs() > 0.05 {
                jsign(lat_a)
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

/// Seyrek yerleşimin uzaklık eşiği (br): topa (E) bundan uzak hücumcunun işi 3 tikte bir hesaplanır.
const SEYREK_R: f64 = 30.0;
fn seyrek_acik() -> bool {
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    !SEYREK_KAPALI.load(std::sync::atomic::Ordering::Relaxed) && *A.get_or_init(|| !(crate::iz_acik() && std::env::var("ALAN_SEYREK_ESKI_IZ").is_ok()))
}
/// Tarayıcıda (ortam değişkeni yok) seyrek yerleşimi kapatmak için: rust-isci.js adreste seyrek=0 görürse kurar.
pub static SEYREK_KAPALI: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
