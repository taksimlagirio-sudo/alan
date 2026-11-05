//! position'un savunma bölümü: pres, süpürücüler, markaj, eşleşme işi, yedek işler, takım düzeltmeleri.
#![allow(non_snake_case, clippy::needless_range_loop)]

use std::sync::Arc;

use crate::core_sabit::Q;
use crate::mat::{atan2, cos, exp, hypot, jmax, jmin, sin};
use crate::model::{Oyuncu, Pid};
use crate::oyuncu::Tepki;
use crate::yetenek::{agirlik, mek, oz};

use super::super::gecici_dis as dis;
use super::super::*;
use super::yardim::*;

/// position'un bir çağrısının durumu.
pub(in super::super) struct Sv {
    pub h: Option<Pid>,
    pub KT: Option<(Pid, f64)>,
    pub att: u8,
    pub dt: u8,
    pub dtac: dis::TacE,
    pub E: (f64, f64),
    pub fr: Option<Arc<Fr>>,
    pub chase: Vec<bool>,
    pub since: f64,
    pub n: usize,
    pub defs: Vec<Pid>,
    pub sweep: Vec<bool>,
    pub up: Vec<bool>,
    pub upHat: f64,
    pub press: Vec<Pid>,
    pub PRS: Option<Prs>,
    pub atkAll: Vec<Pid>,
    pub atk: Vec<Pid>,
    pub TH: Option<Vec<Tehlike>>,
    pub covered: Vec<f64>,
    pub Tt: f64,
    pub sg: bool,
    pub free: Vec<Pid>,
    pub base: Vec<Option<Yer>>,
    /// JS bul: alıcı ve buluşma (x, y, t)
    pub bul: Option<(Pid, f64, f64, f64)>,
    pub Eb: (f64, f64),
    pub swL: Vec<Pid>,
    pub tP: Option<TP>,
    pub dngC: Vec<Option<f64>>,
    pub dMax: f64,
    pub deepO: Vec<Pid>,
    /// JS swT: süpürücünün karşıladığı (Map: yok / null / oyuncu)
    pub swT: Vec<Option<Option<Pid>>>,
    pub YR: bool,
    pub mark: Vec<Option<Pid>>,
    pub taken: Vec<bool>,
    pub acik: f64,
    pub TA: f64,
    pub UA: Vec<Option<Ua>>,
    // hücum
    pub guard: Vec<Option<Pid>>,
    pub runners: Vec<bool>,
    pub kanatT: Vec<Option<f64>>,
    pub win: bool,
}

impl Sv {
    #[allow(clippy::too_many_arguments)]
    pub fn yeni(c: &mut Mc, h: Option<Pid>, KT: Option<(Pid, f64)>, att: u8, dt: u8, dtac: dis::TacE, E: (f64, f64), fr: Option<Arc<Fr>>, chase: Vec<bool>, since: f64) -> Sv {
        let n = c.ps.len();
        Sv {
            h,
            KT,
            att,
            dt,
            dtac,
            E,
            fr,
            chase,
            since,
            n,
            defs: vec![],
            sweep: vec![false; n],
            up: vec![false; n],
            upHat: 0.0,
            press: vec![],
            PRS: None,
            atkAll: vec![],
            atk: vec![],
            TH: None,
            covered: vec![],
            Tt: 0.0,
            sg: !c.lite,
            free: vec![],
            base: vec![None; n],
            bul: None,
            Eb: E,
            swL: vec![],
            tP: None,
            dngC: vec![None; n],
            dMax: 0.0,
            deepO: vec![],
            swT: vec![None; n],
            YR: false,
            mark: vec![None; n],
            taken: vec![false; n],
            acik: 0.0,
            TA: 0.0,
            UA: vec![None; n],
            guard: vec![None; n],
            runners: vec![false; n],
            kanatT: vec![None; n],
            win: false,
        }
    }

    /// JS gD(q): kendi kalesine (dt) uzaklık.
    #[inline]
    pub fn gD(&self, x: f64, y: f64) -> f64 {
        hypot(x - ownX(self.dt), y - GCY)
    }

    /// JS `swT.get(p)` (yoksa / null → None).
    #[inline]
    pub fn swTg(&self, p: Pid) -> Option<Pid> {
        self.swT[p].flatten()
    }

    /// JS dng(q): hücumcunun tehlikesi (kafanın pres tartısı varsa ondan).
    pub fn dng(&mut self, c: &Mc, q: Pid) -> f64 {
        if let Some(v) = self.dngC[q] {
            return v;
        }
        let qp = &c.ps[q];
        let ozk = cl(1.0 + 0.035 * (qp.yet.y(mek::TEHLIKE) + agirlik(mek::TEHLIKE, oz::HIZ) * (qp.hz() - qp.yet.oz[oz::HIZ]) - 10.0), 0.7, 1.4);
        let v = match self.tP.as_ref().and_then(|t| t.m_get(qp.id)) {
            Some(m) => m * ozk,
            None => {
                let fx = cl(qp.x + 30.0 * qp.vx, 0.0, GW);
                let fy = cl(qp.y + 30.0 * qp.vy, 0.0, GH);
                let yer = dis::threat(self.att, fx, fy);
                let top = jmax(0.45, 1.0 - jmax(0.0, hypot(qp.x - self.E.0, qp.y - self.E.1) - 15.0) / 80.0);
                let gq = self.gD(qp.x, qp.y);
                let bos = if c.ps.iter().any(|o| o.takim() == self.dt && !o.bekci() && self.gD(o.x, o.y) < gq - 1.0 && (o.y - qp.y).abs() < 10.0) { 1.0 } else { 1.3 };
                yer * top * bos * ozk
            }
        };
        self.dngC[q] = Some(v);
        v
    }

    /// JS prV(p): presçi adayının değeri.
    fn prV(&self, c: &Mc, w: &[Varlik], pi: Pid, h: Pid, Th: f64, sd: f64, gainH: f64, hatK: f64) -> PrV {
        let k = c.k;
        let p = &c.ps[pi];
        let hp = &c.ps[h];
        let d = hypot(p.x - hp.x, p.y - hp.y);
        let tR = jmax(0.0, TEPKI(p, Tepki::Kovala, k)) + jmax(0.0, d - 2.4) / vSp(p, k) + gec(p, hp.x, hp.y, Some(d - 2.4), k);
        let sdp = jmax(0.0, jmin(1.0, sd + 0.8 * (tl0(p, "sav", "pres") - 0.5)));
        let ThP = Th / (0.6 + 0.8 * sd) * (0.6 + 0.8 * sdp);
        let P = 1.0 / (1.0 + exp((tR - ThP) / 8.0));
        let bs = slotBase(c.m, c.ps, p, self.E.0, self.E.1, false);
        let mut cost: f64 = 0.0;
        for &q in &self.atk {
            let qp = &c.ps[q];
            if hypot(qp.x - bs.x, qp.y - bs.y) < 9.0 || p.tutQ == Some(q) {
                cost = jmax(cost, dis::threat(self.att, qp.x, qp.y) * (0.3 + 0.7 * dis::laneOk0(&*c.m, w, c.ps, self.att, (hp.x, hp.y), (qp.x, qp.y))));
            }
        }
        let enV = {
            let dP = jmax(0.0, d - 2.4);
            let b0 = spd(p);
            let tK = dP / b0;
            let tS = jmax(0.0, tK - tR);
            let cP = jmin(p.en.unwrap_or(1.0), if tS > 0.0 { 0.03 + tR * (1.0 / 120.0) / EN2_SB(p) } else { 0.0 });
            let mut ndH: f64 = 1e9;
            for (qi, q) in c.ps.iter().enumerate() {
                if !(q.takim() != self.dt || qi == pi || q.noTouch) {
                    ndH = jmin(ndH, hypot(q.x - hp.x, q.y - hp.y));
                }
            }
            let dH = jmax(0.0, dis::ALAN_EN3_DOL(hp, true, ndH, 0.0) - dis::ALAN_EN3_DOL(hp, true, 2.4, 0.0)) * ThP;
            let eH = hp.en.unwrap_or(1.0);
            let eP = p.en.unwrap_or(1.0);
            dis::ALAN_EN3_KUL(hp, eH, true, hp.x, hp.y, c.k) - dis::ALAN_EN3_KUL(hp, eH - dH, true, hp.x, hp.y, c.k) - (dis::ALAN_EN3_KUL(p, eP, false, hp.x, hp.y, c.k) - dis::ALAN_EN3_KUL(p, eP - cP, false, hp.x, hp.y, c.k))
        };
        let adP = if p.markRef != Some(h) { 0.0 } else { tl0(p, "sav", "adam") * gainH * hatK };
        let cc = cost * S.prCost * (1.4 - 0.8 * sdp);
        PrV { P, v: P * (gainH * hatK + enV + adP) - cc, c: cc, tR }
    }

    /// JS presIs(p, k): k'nci presçinin işi.
    pub(super) fn presIs(&mut self, c: &mut Mc, pi: Pid, kk: usize) {
        let tick = c.m.tick;
        let h = self.h.unwrap();
        {
            let p = &mut c.ps[pi];
            if !kilitli(p, tick) {
                p.lock = Some(tick + S.lock);
                p.lockAt = Some(tick + 0.01 * kk as f64);
                p._lockH = Some(h);
            }
            p.press = true;
            p.sprint = true;
            p.job = Some("pres".into());
        }
        let sn = seen(c, pi);
        if kk == 0 {
            let TH = self.TH.clone();
            ilkPres(c, pi, h, self.dt, TH.as_deref(), &self.dtac);
            return;
        }
        let dA = |x: f64, y: f64| atan2(sin(x - y), cos(x - y)).abs();
        let (hx, hy) = (c.ps[h].x, c.ps[h].y);
        if let Some(TH) = &self.TH {
            if let Some(t2) = TH.iter().find(|u| self.covered.iter().all(|&cv| dA(u.an, cv) > 0.8)) {
                let an = t2.an;
                let r = jmax(2.5, jmin(8.0, 0.35 * t2.L));
                self.covered.push(an);
                let p = &mut c.ps[pi];
                p.tx = hx + cos(an) * r;
                p.ty = hy + sin(an) * r;
                p.job = Some("pres · ikinci yolu kes".into());
                return;
            }
        }
        let mut opts: Vec<(Pid, f64, f64)> = self
            .atk
            .iter()
            .map(|&q| {
                let qp = &c.ps[q];
                (q, dis::threat(self.att, qp.x, qp.y) * exp(2.0 * -laneIn(&sn, c.ps, (hx, hy), (qp.x, qp.y))), atan2(qp.y - hy, qp.x - hx))
            })
            .collect();
        opts.sort_by(|a, b| sira(b.1 - a.1));
        let q = opts.iter().find(|o| self.covered.iter().all(|&cv| dA(o.2, cv) > 0.8)).or(opts.first()).copied();
        let q = match q {
            Some(q) => q,
            None => return,
        };
        self.covered.push(q.2);
        let qp = &c.ps[q.0];
        let dx = qp.x - hx;
        let dy = qp.y - hy;
        let L = nz1(hypot(dx, dy));
        let p = &mut c.ps[pi];
        p.tx = hx + dx / L * 2.5;
        p.ty = hy + dy / L * 2.5;
    }

    /// Savunma: pres, süpürücüler, markaj, eşleşme, yedek işler, takım düzeltmeleri.
    pub fn savunma(&mut self, c: &mut Mc) {
        let k = c.k;
        let tick = c.m.tick;
        let n = self.n;
        let (att, dt, E, h) = (self.att, self.dt, self.E, self.h);
        self.defs = (0..n).filter(|&i| { let p = &c.ps[i]; p.takim() == dt && !p.bekci() && !self.chase[i] && !p.noTouch }).collect();
        let nArk = self.defs.iter().filter(|&&p| c.ps[p].slot.d <= 0.25).count();
        {
            let mut s = self.defs.clone();
            s.sort_by(|&u, &v| sira(c.ps[u].slot.d - c.ps[v].slot.d));
            for &p in s.iter().take(nArk) {
                self.sweep[p] = true;
            }
            let mut u: Vec<Pid> = self.defs.iter().copied().filter(|&p| !self.sweep[p]).collect();
            u.sort_by(|&a, &b| sira(c.ps[b].slot.d - c.ps[a].slot.d));
            let onde = self.dtac.t.onde.unwrap_or(f64::NAN);
            let nu = if onde > 0.0 && onde == onde { onde as usize } else { 0 };
            for &p in u.iter().take(nu) {
                self.up[p] = true;
            }
        }
        self.upHat = {
            let ra: Vec<Pid> = (0..n).filter(|&q| c.ps[q].takim() == att && !c.ps[q].bekci() && Some(q) != h).collect();
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
            wx
        };
        let pLim = presLim(c, &self.dtac, h, att);
        if let Some(h) = h {
            self.presSec(c, h, pLim);
        }
        self.atkAll = (0..n).filter(|&q| c.ps[q].takim() == att && !c.ps[q].bekci()).collect();
        self.atk = self.atkAll.iter().copied().filter(|&q| Some(q) != h).collect();
        self.TH = tehlike(c.m, c.ps, h, c.d);
        self.covered = match h {
            Some(h) => vec![match &self.TH {
                Some(t) => t[0].an,
                None => atan2(GCY - c.ps[h].y, ownX(dt) - c.ps[h].x),
            }],
            None => vec![],
        };
        let pr = self.press.clone();
        for (i, &p) in pr.iter().enumerate() {
            self.presIs(c, p, i);
        }
        self.Tt = if !self.defs.is_empty() { self.defs.iter().fold(0.0, |a, &p| a + okuTp(&c.ps[p])) / self.defs.len() as f64 } else { 0.0 };
        self.free = self.defs.iter().copied().filter(|&p| !self.sweep[p] && !self.up[p] && !self.press.contains(&p)).collect();
        for p in 0..n {
            if self.chase[p] || Some(p) == h {
                continue;
            }
            let pp = &c.ps[p];
            self.base[p] = Some(if pp.bekci() {
                let bm = BekM { ps: crate::sekil::yer::BekPs::Duz(c.ps), holder: c.m.holder.map(|h| &c.ps[h]), sut: c.m.mac.kafa.k.is_some(), ben: Some(p) };
                let b = bekci(pp, E.0, E.1, Some(&bm), k);
                Yer { x: b.0, y: b.1, el: None }
            } else {
                slotBase(c.m, c.ps, pp, E.0, E.1, pp.takim() == att)
            });
        }
        self.bul = (|| {
            let fr = self.fr.as_ref()?;
            let fq = c.m.fl.as_ref()?.q?;
            let b = c.ball.as_ref()?;
            if b.b.defl != 0.0 || b.b.done.is_some() || c.ps[fq].takim() != att {
                return None;
            }
            let r = fr.arr[fq];
            Some((fq, r.x, r.y, r.est))
        })();
        self.Eb = match self.bul {
            Some(b) => (b.1, b.2),
            None => E,
        };
        {
            let mut s = self.defs.clone();
            s.sort_by(|&u, &v| sira(c.ps[u].slot.d - c.ps[v].slot.d));
            self.swL = s.into_iter().filter(|&p| self.sweep[p]).collect();
            let mut s2 = self.swL.clone();
            s2.sort_by(|&u, &v| sira(c.ps[u].slot.d - c.ps[v].slot.d));
            self.swL = s2;
        }
        self.tP = match (&c.m._prT, h) {
            (Some(T0), Some(hh)) if T0.team == att as f64 && c.ps[hh].takim() == att && tick - T0.t <= 20.0 => Some(TP { m: T0.m.clone(), mm: T0.mm.clone(), hm: T0.hm.clone(), h: if T0.h == T0.h { Some(T0.h) } else { None } }),
            _ => None,
        };
        let mkRef0: Vec<Pid> = self.free.iter().filter(|&&p| tl0(&c.ps[p], "sav", "adam") >= 0.5 && c.ps[p].markRef.map(|r| c.ps[r].takim() == att).unwrap_or(false)).map(|&p| c.ps[p].markRef.unwrap()).collect();
        let swPrevS: Vec<Pid> = self.defs.iter().filter_map(|&p| c.ps[p].swPrev).collect();
        let gE = self.gD(E.0, E.1);
        let mut deepA: Vec<Pid> = Vec::new();
        for &q in &self.atk.clone() {
            let qp = &c.ps[q];
            let ia = ileriP(qp, self.Tt);
            let onde = self.defs.iter().any(|&o| !self.sweep[o] && (c.ps[o].kapPrev == Some(q) || c.ps[o].zonePrev == Some(q)) && self.gD(c.ps[o].x, c.ps[o].y) < self.gD(qp.x, qp.y) + 2.0);
            if self.gD(ia.0, ia.1) < gE + if swPrevS.contains(&q) { 4.0 } else { 0.0 } && !onde && !(h.map(|hh| qp.id == c.ps[hh].id).unwrap_or(false)) && !mkRef0.contains(&q) && hypot(qp.x - E.0, qp.y - E.1) > 3.0 {
                deepA.push(q);
            }
        }
        for &q in &deepA {
            self.dng(c, q);
        }
        deepA.sort_by(|&a, &b| sira(self.dngC[b].unwrap() - self.dngC[a].unwrap()));
        self.dMax = if !deepA.is_empty() { self.dngC[deepA[0]].unwrap() } else { 0.0 };
        self.deepO = deepA.iter().copied().filter(|&q| self.dngC[q].unwrap() >= 0.35 * self.dMax).collect();
        self.supurucu(c);
        self.YR = {
            let gain = c.ek.sh.gain;
            let yeni = !matches!(&c.ek.sh.yr, Some(y) if y.gain == gain && y.dt == dt);
            if yeni {
                c.ek.sh.yr = Some(Yr { gain, dt, on: true });
            }
            let mut y = c.ek.sh.yr.unwrap();
            if y.on {
                let gE = self.gD(E.0, E.1);
                let nn: Vec<Pid> = self.defs.iter().copied().filter(|&p| !self.press.contains(&p) && !self.up[p]).collect();
                let ok = nn
                    .iter()
                    .filter(|&&p| {
                        let pp = &c.ps[p];
                        let b = self.base[p];
                        self.gD(pp.x, pp.y) < gE + 3.0 && (b.is_none() || hypot(pp.x - b.unwrap().x, pp.y - b.unwrap().y) < 8.0)
                    })
                    .count();
                if ok as f64 >= nn.len() as f64 - 1.0 || tick - gain > 180.0 {
                    y.on = false;
                }
                c.ek.sh.yr = Some(y);
            }
            y.on
        };
        self.markaj(c);
        self.bolge(c);
        self.acik = {
            let menzil = |g: f64| cl((62.0 * GKX - g) / (16.0 * GKX), 0.0, 1.0);
            match h {
                Some(hh) if c.ps[hh].takim() == att => {
                    let hp = &c.ps[hh];
                    let mut dm: f64 = 1e9;
                    let gh = self.gD(hp.x, hp.y);
                    for o in c.ps.iter() {
                        if o.takim() == dt && !o.bekci() {
                            dm = jmin(dm, hypot(o.x - hp.x, o.y - hp.y) + if self.gD(o.x, o.y) < gh + 1.0 { 0.0 } else { 2.0 });
                        }
                    }
                    cl((dm - 3.0) / 4.0, 0.0, 1.0) * menzil(gh)
                }
                _ => {
                    let f = c.ball.as_ref().and_then(|b| b.t.from);
                    match (h, self.bul, f) {
                        (None, Some(b), Some(f)) if c.ps[f].takim() == att && self.gD(b.1, b.2) < self.gD(c.ps[f].x, c.ps[f].y) - 1.0 => menzil(self.gD(b.1, b.2)),
                        _ => 0.0,
                    }
                }
            }
        };
        self.TA = 30.0 * self.acik;
        self.esles(c);
        for &p in &self.swL.clone() {
            if !(h.is_some() && self.swTg(p) == h) {
                self.swT[p] = Some(None);
            }
        }
        for p in 0..n {
            if self.mark[p].is_some() && !self.adamT(c, p) {
                self.mark[p] = None;
            }
        }
        self.yedek(c);
        self.takim(c);
    }

    /// JS `adamT(p)`: adam adama savunma (paylaşım dalgası yokken).
    #[inline]
    pub fn adamT(&self, c: &Mc, p: Pid) -> bool {
        tl0(&c.ps[p], "sav", "adam") >= 0.5 && !self.YR
    }

    /// Pres seçimi (JS `if (h) {…}` bloğu): adaylar, değerleri, kilit, kontrpres, sıra.
    fn presSec(&mut self, c: &mut Mc, h: Pid, pLim: usize) {
        let k = c.k;
        let tick = c.m.tick;
        let (att, dt) = (self.att, self.dt);
        let mp = macPres(c, 1 - att);
        let BL = ["Düşük", "Orta", "Yüksek"];
        let bix = BL.iter().position(|b| Some(*b) == self.dtac.t.blok.as_deref()).map(|i| i as i64).unwrap_or(-1);
        let ek = if mp > 0.0 || (self.dtac.t.yz == Some(true) && dis::macEff_macK(c, 1 - att).unwrap_or(1.0) < 0.6) { 1 } else { 0 };
        let bi = (bix + ek).clamp(0, 2) as usize;
        let still = {
            let hp = &mut c.ps[h];
            if hypot(hp.vx, hp.vy) < 0.04 {
                let d = *hp._dur.get_or_insert(tick);
                tick - d
            } else {
                hp._dur = None;
                0.0
            }
        };
        let sd = jmin(1.0, self.dtac.t.siddet.unwrap_or(0.5) + if mp > 0.0 { 0.25 } else { 0.0 });
        let hp = c.ps[h].clone();
        let gecikme = match &hp.savur {
            Some(s) => (s.g + s.rem) / jmax(0.1, (s.w + s.wMax) / 2.0),
            None => match self.KT {
                Some((q, t)) if q == h => t,
                _ => jmax(0.0, yada(c.m.decT, 0.0)),
            },
        };
        let Th = (S.prT0 + gecikme + still * S.prStill) * (0.6 + 0.8 * sd);
        let gainH = 0.4 + dis::threat(att, hp.x, hp.y);
        let mut cand: Vec<Pid> = self.defs.iter().copied().filter(|&p| !self.up[p] || self.upBas(c, p)).collect();
        let adamH = |p: Pid| -> f64 { if c.ps[p].markRef == Some(h) && tl0(&c.ps[p], "sav", "adam") >= 0.5 { 1.0 } else { 0.0 } };
        cand.sort_by(|&u, &v| sira(1e4 * (adamH(v) - adamH(u)) + 1e3 * (tl0(&c.ps[v], "sav", "pres") - tl0(&c.ps[u], "sav", "pres")) + hypot(c.ps[u].x - hp.x, c.ps[u].y - hp.y) - hypot(c.ps[v].x - hp.x, c.ps[v].y - hp.y)));
        let dG = (hp.x - ownX(dt)).abs();
        let hat = prHat(BL[bi]).unwrap_or(62.0);
        let hatK = if dG < hat {
            1.0
        } else if still > if hp.bekci() { S.prHold } else { 2.5 * S.prHold } {
            0.9
        } else {
            S.prDis * jmax(0.0, 1.0 - (dG - hat) / 25.0) + if dG < hat + 8.0 { 0.25 } else { 0.0 }
        };
        self.atk = (0..self.n).filter(|&q| c.ps[q].takim() == att && !c.ps[q].bekci() && q != h).collect();
        let w = c.dunya();
        let pvs: Vec<(Pid, PrV)> = cand.iter().map(|&p| (p, self.prV(c, &w, p, h, Th, sd, gainH, hatK))).collect();
        let mut srt = pvs.clone();
        srt.sort_by(|a, b| sira(b.1.v - a.1.v));
        for (q, e) in &srt {
            if self.press.len() >= pLim {
                break;
            }
            let yet = e.P * jmin(1.0, 2.0 * hatK) > S.prPmin;
            // başlamak için hem yetişmek hem değer gerekir; başlamış (kilitli) presçi ikisinden biri sürdükçe bırakmaz:
            // eşikte her tik girip çıkmak oyuncuyu presle arka iş arasında titretiyor, ikisini de yaptırmıyordu
            if (kilitli(&c.ps[*q], tick) && hatK >= 0.5 && (yet || e.v > 0.0)) || (yet && e.v > 0.0) {
                self.press.push(*q);
            }
        }
        {
            let mut pv = vec![None; self.n];
            let mut P0: f64 = 0.0;
            let mut cs = vec![false; self.n];
            for (q, e) in &pvs {
                pv[*q] = Some(*e);
                cs[*q] = true;
                P0 = jmax(P0, e.P);
            }
            let fixed: Vec<Pid> = self.press.iter().copied().filter(|&q| kilitli(&c.ps[q], tick) && hatK >= 0.5 && hypot(c.ps[q].x - hp.x, c.ps[q].y - hp.y) <= dis::DU_engage + 0.6).collect();
            self.PRS = Some(Prs { cand: cs, pv, pLim, G: gainH * hatK, K: hatK, P0, fixed });
        }
        if let Some(kp) = dis::kpAkt(c, dt) {
            self.PRS = None;
            self.press.clear();
            for id in kp {
                if let Some(&q) = self.defs.iter().find(|&&d| c.ps[d].id == id) {
                    self.press.push(q);
                }
            }
        }
        self.press.sort_by(|&u, &v| {
            let a = kilitAn(&c.ps[u], tick) - kilitAn(&c.ps[v], tick);
            sira(yada(a, hypot(c.ps[u].x - hp.x, c.ps[u].y - hp.y) - hypot(c.ps[v].x - hp.x, c.ps[v].y - hp.y)))
        });
        if self.press.len() > 1 && hypot(hp.x - ownX(dt), hp.y - GCY) < 35.0 * GKX {
            let g0 = gside(hp.x, hp.y, dt, 2.0);
            let tt = |q: Pid| { let qp = &c.ps[q]; hypot(qp.x - g0.0, qp.y - g0.1) / vSp(qp, k) + gec(qp, g0.0, g0.1, None, k) };
            self.press.sort_by(|&u, &v| sira(tt(u) - tt(v)));
        } else if self.press.len() > 1 && hp.vx * (ownX(dt) - hp.x) + hp.vy * (GCY - hp.y) > 0.0 {
            let g0 = gside(hp.x + 10.0 * hp.vx, hp.y + 10.0 * hp.vy, dt, 2.0);
            let tt = |q: Pid| { let qp = &c.ps[q]; hypot(qp.x - g0.0, qp.y - g0.1) / vSp(qp, k) + gec(qp, g0.0, g0.1, None, k) };
            self.press.sort_by(|&u, &v| sira(tt(u) - tt(v)));
        }
        if hp.bekci() && hp._korT.map(|t| t > tick).unwrap_or(false) {
            self.PRS = None;
            self.press.clear();
        }
        for &p in &self.defs {
            if !self.press.contains(&p) {
                c.ps[p].lock = Some(0.0);
            }
        }
    }

    /// JS upBas(p): öndeki savunucu taşıyıcı kendi hattını geçince bastırır.
    pub(super) fn upBas(&self, c: &Mc, p: Pid) -> bool {
        match self.h {
            Some(h) if self.up[p] && c.ps[h].takim() == self.att => (c.ps[h].x - oppX(self.dt)).abs() <= (self.upHat - oppX(self.dt)).abs() + 2.0,
            _ => false,
        }
    }

    /// Süpürücülerin karşıladıkları (JS swT bloğu).
    fn supurucu(&mut self, c: &Mc) {
        let (dt, h) = (self.dt, self.h);
        let swL = self.swL.clone();
        let onuBos = match h {
            Some(hh) if c.ps[hh].takim() == self.att => {
                let hp = &c.ps[hh];
                let gh = self.gD(hp.x, hp.y);
                (gh < 40.0 || {
                    let g = gside(hp.x, hp.y, dt, 3.0);
                    let t = |o: &Oyuncu| hypot(o.x - g.0, o.y - g.1) / spd(o);
                    let tO = self.defs.iter().filter(|&&o| !self.sweep[o]).map(|&o| t(&c.ps[o])).fold(1e9, jmin);
                    let tS = swL.iter().map(|&o| t(&c.ps[o])).fold(1e9, jmin);
                    tO > tS
                }) && !self.defs.iter().any(|&o| !self.sweep[o] && self.gD(c.ps[o].x, c.ps[o].y) < gh && hypot(c.ps[o].x - hp.x, c.ps[o].y - hp.y) < 6.0)
            }
            _ => false,
        };
        let mut hdf: Vec<Pid> = if onuBos { std::iter::once(h.unwrap()).chain(self.deepO.iter().copied().filter(|&q| Some(q) != h)).collect() } else { self.deepO.clone() };
        hdf.truncate(swL.len());
        let mut bos: Vec<Pid> = hdf.clone();
        let sil = |bos: &mut Vec<Pid>, q: Pid| {
            if let Some(i) = bos.iter().position(|&x| x == q) {
                bos.remove(i);
            }
        };
        if swL.len() > 1 && hdf.len() > 1 {
            let sure = |p: Pid, q: Option<Pid>| -> f64 {
                match q {
                    Some(q) => {
                        let g = gside(c.ps[q].x, c.ps[q].y, dt, 3.0);
                        hypot(g.0 - c.ps[p].x, g.1 - c.ps[p].y) / spd(&c.ps[p])
                    }
                    None => 0.0,
                }
            };
            let mut Qv: Vec<Option<Pid>> = hdf.iter().map(|&q| Some(q)).collect();
            for _ in 0..swL.len().saturating_sub(hdf.len()) {
                Qv.push(None);
            }
            let olc = |per: &[Option<Pid>]| -> f64 {
                let (mut mx, mut top) = (0.0f64, 0.0);
                for (i, &p) in swL.iter().enumerate() {
                    let t = sure(p, per.get(i).copied().flatten());
                    mx = jmax(mx, t);
                    top += t;
                }
                1e3 * mx + 0.001 * top
            };
            let mut en: Option<Vec<Option<Pid>>> = None;
            let mut ev = 1e18;
            for per in perms(&Qv) {
                let v = olc(&per);
                if v < ev {
                    ev = v;
                    en = Some(per);
                }
            }
            let eski: Vec<Option<Pid>> = swL.iter().map(|&p| c.ps[p].swPrev.filter(|q| bos.contains(q))).collect();
            if eski.iter().all(|x| x.is_some()) {
                let ev0 = olc(&eski);
                if ev0 <= 1.15 * ev {
                    en = Some(eski);
                }
            }
            if let Some(en) = en {
                for (i, &p) in swL.iter().enumerate() {
                    let e = en.get(i).copied().flatten();
                    self.swT[p] = Some(e);
                    if let Some(e) = e {
                        sil(&mut bos, e);
                    }
                }
            }
        }
        for &p in &swL {
            if let Some(o) = c.ps[p].swPrev {
                if bos.contains(&o) {
                    self.swT[p] = Some(Some(o));
                    sil(&mut bos, o);
                }
            }
        }
        for &p in &swL {
            if self.swT[p].is_some() {
                continue;
            }
            let mut b: Option<Pid> = None;
            let mut bd = 1e9;
            for &q in &bos {
                let d = hypot(c.ps[q].x - c.ps[p].x, c.ps[q].y - c.ps[p].y);
                if d < bd {
                    bd = d;
                    b = Some(q);
                }
            }
            self.swT[p] = Some(b);
            if let Some(b) = b {
                sil(&mut bos, b);
            }
        }
    }

    /// Adam adama atama (JS mark/taken bloğu: dal-sınır araması).
    fn markaj(&mut self, c: &mut Mc) {
        let k = c.k;
        let (dt, h, E) = (self.dt, self.h, self.E);
        let mut Mk: Vec<Pid> = self.free.clone();
        for &p in &self.defs {
            if self.sweep[p] && !self.press.contains(&p) && !(h.is_some() && self.swTg(p) == h) {
                Mk.push(p);
            }
        }
        Mk.retain(|&p| self.adamT(c, p));
        let At = self.atk.clone();
        let mut taken = vec![false; self.n];
        if !Mk.is_empty() && !At.is_empty() {
            let aF = c.d.arriveFirm;
            let SH = jmax(1.0, yada(c.d.sutHazir, 1.0));
            let TQ: Vec<f64> = At.iter().map(|&q| dis::ucusT(&*c.m, hypot(c.ps[q].x - E.0, c.ps[q].y - E.1), aF, None)).collect();
            let DG: Vec<f64> = At.iter().map(|&q| jmax(0.0, self.dng(c, q))).collect();
            let mut B: Vec<Vec<f64>> = Vec::new();
            for &p in &Mk {
                let sv = seen(c, p);
                let pp = &c.ps[p];
                let rc = jmax(0.0, TEPKI(pp, Tepki::Kovala, k));
                let v = vSp(pp, k);
                B.push(
                    At.iter()
                        .enumerate()
                        .map(|(j, &q)| {
                            let s0 = sv.iter().find(|o| o.r == Some(q)).map(|o| (o.x, o.y)).unwrap_or((c.ps[q].x, c.ps[q].y));
                            let g = gside(s0.0, s0.1, dt, yada(S.markOff, 2.5));
                            let t = hypot(g.0 - pp.x, g.1 - pp.y) / v - if pp.markRef == Some(q) { rc } else { 0.0 };
                            DG[j] * jmax(0.0, jmin(1.0, (TQ[j] + SH - t) / SH))
                        })
                        .collect(),
                );
            }
            let gi: Vec<usize> = (0..At.len()).collect();
            let best = dal_sinir(&B, &gi);
            if let Some(best) = best {
                for (i, &p) in Mk.iter().enumerate() {
                    let j = best[i];
                    if j >= 0 {
                        let q = At[j as usize];
                        c.ps[p].markRef = Some(q);
                        self.mark[p] = Some(q);
                        taken[p] = true;
                    }
                }
            }
        }
        self.taken = taken;
    }
}

/// JS ara(i, c) dal-sınır araması: satır başına en çok bir sütun, toplamı en büyük atama. `gi`: sütunun
/// kullanım grubu (JS GI). Dönüş: satır başına sütun (−1: atanmadı).
pub(super) fn dal_sinir(B: &[Vec<f64>], gi: &[usize]) -> Option<Vec<i64>> {
    let nr = B.len();
    let nc = gi.len();
    let mx: Vec<f64> = B.iter().map(|r| r.iter().fold(0.0f64, |a, &b| jmax(a, b))).collect();
    let mut kal = vec![0.0; nr + 1];
    for i in (0..nr).rev() {
        kal[i] = kal[i + 1] + mx[i];
    }
    let mut best: Option<Vec<i64>> = None;
    let mut bb = 0.0;
    let mut cur: Vec<i64> = Vec::new();
    let mut used = vec![false; nc];
    fn ara(i: usize, c: f64, B: &[Vec<f64>], gi: &[usize], kal: &[f64], bb: &mut f64, best: &mut Option<Vec<i64>>, cur: &mut Vec<i64>, used: &mut [bool]) {
        if c + kal[i] <= *bb {
            return;
        }
        if i != B.len() {
            for j in 0..gi.len() {
                if !used[gi[j]] && B[i][j] > 0.0 {
                    used[gi[j]] = true;
                    cur.push(j as i64);
                    ara(i + 1, c + B[i][j], B, gi, kal, bb, best, cur, used);
                    cur.pop();
                    used[gi[j]] = false;
                }
            }
            cur.push(-1);
            ara(i + 1, c, B, gi, kal, bb, best, cur, used);
            cur.pop();
        } else {
            *bb = c;
            *best = Some(cur.clone());
        }
    }
    ara(0, 0.0, B, gi, &kal, &mut bb, &mut best, &mut cur, &mut used);
    best
}

#[allow(dead_code)]
fn _q() -> f64 {
    Q.body
}
