//! Tutma ve ikili (decide.js:244–301 topK/donS/cevK/donT/dolanma, 302 fakeOkunmaz, 4017–4716 yanilT/kWof/
//! presRol/kesisK/keepP/govdeVurP/coreSide/trapped/ikKafa/ikHiz/ikDen/duelP2/duelP/coreAng/siddetQ/oyalaEk/
//! enIkiliEk/girEsik/girThr/girEk/commitThr).
#![allow(non_snake_case)]

use crate::kafa::gecici_dis1::KuralErisim as _;
use super::gecici_dis1::{yerDeger, Ik, PrT, TacEff, Tehlike};
use super::kafa1_baglam::Kafa;
use super::kafa1_js::*;
use super::kafa1_param::{DU, MIND};
use super::kafa1_tehlike::{ox_of, GH8, GW8};
use crate::core_sabit::{Q, TQ};
use crate::donus::{flick_miss, flick_w, omega, shielded};
use crate::oyuncu::Tepki;
macro_rules! kp_yaz {
    ($dis:expr, $e:expr) => {
        if $dis {
            KP_KAYIT.with(|c| if let Some(v) = c.borrow_mut().as_mut() { v.push($e); });
        }
    };
}
use crate::mat::{acos, atan2, cos, exp, hypot, jmax, jmin, jround, jsign, pow, sin, PI};
use crate::model::{JMShGs, Oyuncu};
use crate::saha::{CY as GCY, KR, W as GW};
use crate::yetenek::mek;

/// Çekirdeğin yönü (JS coreSide dönüşü).
#[derive(Clone, Copy, Debug)]
pub struct CoreSide {
    pub ux: f64,
    pub uy: f64,
    pub off: f64,
}

/// presRol'ün bir rakibe verdiği rol.
#[derive(Clone, Debug)]
pub enum PresRol {
    /// ilk presçi: yerleşimin ilkPres'iyle koşar
    Ilk { TH: Vec<Tehlike>, dtac: TacEff, gs: Option<JMShGs> },
    /// tehlike yönünü kapatan: açı ve uzaklık
    An { an: f64, r: f64 },
    /// yerinde
    Dur,
}

/// İkilinin son hesabı (JS DP_K, DP_SON).
#[derive(Clone, Copy, Debug)]
pub struct DuelSon {
    pub P: f64,
    pub omuz: bool,
    pub son: f64,
}

#[inline(always)]
fn w0(a: f64) -> f64 {
    atan2(sin(a), cos(a))
}
#[inline(always)]
fn cl1(v: f64) -> f64 {
    jmax(0.0, jmin(1.0, v))
}
/// JS `Math.sign(x) || 1`
#[inline(always)]
fn sg1(x: f64) -> f64 {
    let s = jsign(x);
    if s != 0.0 && s == s {
        s
    } else {
        1.0
    }
}
/// Tam sayı değerin JS dizgisi (anahtarlar için; -0 → "0").
pub fn js_int(v: f64) -> String {
    format!("{}", v + 0.0)
}

#[derive(Clone, Copy, Debug)]
struct Hm {
    tr: f64,
    pc: f64,
    omuz: bool,
}
#[derive(Clone, Copy, Debug)]
struct Ad {
    T: f64,
    x: f64,
    y: f64,
    g: bool,
}
#[derive(Clone, Copy, Debug)]
struct Bol {
    pc: f64,
    W: f64,
    omuz: bool,
}
/// kesisK'nın değişen durumu (JS'te kapanışın değişkenleri; `al()`/`ver()` anlık görüntüsü).
#[derive(Clone)]
struct Kd {
    hv: Oyuncu,
    qv: Oyuncu,
    keep: f64,
    tK: f64,
    dMin: f64,
    qMin: f64,
    fk: f64,
    fkD: f64,
    fkOk: bool,
    hm: Option<Hm>,
    ad: Option<Ad>,
    qS: f64,
    qCd: f64,
    kalS: bool,
    /// taşıyıcının ilk sendelediği an (ALAN_SEND_YARIS_IZ): serbest top yarışının olasılığı (rakip kazanır)
    sendY: Option<f64>,
}
/// kesisK'nın sabitleri.
struct Kc<'a> {
    h: &'a Oyuncu,
    q: &'a Oyuncu,
    src: &'a [Op],
    dir: Nk,
    T: f64,
    rol: Option<&'a PresRol>,
    IK: Ik,
    vB: f64,
    vQ0: f64,
    vQs: f64,
    spU: f64,
    spG: f64,
    kX: f64,
    la: f64,
    dlH: f64,
    dlQ: f64,
    dlQs: f64,
    mH: f64,
    mQ: f64,
    iH: f64,
    iQ: f64,
    rH: f64,
    rQ: f64,
    w: f64,
    ek: f64,
    cvQ: f64,
    vQf: f64,
    adS: f64,
    t0: f64,
}

impl Kafa {
    /// Top sürerken hız katı (decide.js:244).
    pub fn topK(&self, h: &Oyuncu) -> f64 {
        topK_p(h, self.dis.ALAN_DOKUNUS())
    }
    #[allow(dead_code)]
    fn _topK_eski(&self, h: &Oyuncu) -> f64 {
        let s = h.yet.y(mek::TOPLU_KOSU);
        let b = match self.dis.ALAN_DOKUNUS() {
            Some(C0) => {
                let close = jmin(1.0, C0.close + (s - 10.0) * C0.closeS);
                let Lr = jmax(0.8, C0.lRun - (s - 10.0) * C0.lRunS);
                // `_dkP` modelde yok (çalışırken görülmedi): 0
                close + (1.0 - close) * jmin(1.0, 0.0 / Lr)
            }
            None => jmin(1.0, 0.68 + 0.02 * s),
        };
        // `slowT` modelde yok: yavaşlama yok
        b * 1.0
    }
    /// Yön değiştirmenin süre bedeli (decide.js:253).
    pub fn donS(&self, p: &Oyuncu, ux: f64, uy: f64) -> f64 {
        let sp0 = hypot(z0(p.vx), z0(p.vy));
        if sp0 < 0.05 {
            return 0.0;
        }
        let vm = self.vS(p);
        let ag = self.AG(p);
        let k0 = 0.18;
        let kH = k0 * self.AH(p);
        let yur = |mut vx: f64, mut vy: f64| -> f64 {
            let mut x = 0.0;
            let mut t = 0;
            while t < 60 && x < 6.0 {
                let sp = hypot(vx, vy);
                let wx = ux * vm;
                let wy = uy * vm;
                if sp < 0.001 {
                    vx += (wx - vx) * kH;
                    vy += (wy - vy) * kH;
                } else {
                    let ex = vx / sp;
                    let ey = vy / sp;
                    let dvx = wx - vx;
                    let dvy = wy - vy;
                    let al = dvx * ex + dvy * ey;
                    let px = dvx - al * ex;
                    let py = dvy - al * ey;
                    let r = jmin(1.0, sp / vm);
                    let kA = if al >= 0.0 { kH } else { k0 * ag * (1.0 - 0.55 * r) };
                    let kT = k0 * ag * (1.0 - 0.45 * r);
                    let mut ba = al * kA;
                    let mut tx = px * kT;
                    let mut ty = py * kT;
                    if ba < 0.0 {
                        ba = jmax(ba, -0.018 * ag);
                    }
                    let tl = hypot(tx, ty);
                    let tm = 0.021 * ag;
                    if tl > tm && sp > 0.1 {
                        tx *= tm / tl;
                        ty *= tm / tl;
                    }
                    vx += ex * ba + tx;
                    vy += ey * ba + ty;
                }
                x += vx * ux + vy * uy;
                t += 1;
            }
            t as f64
        };
        jmax(0.0, yur(z0(p.vx), z0(p.vy)) - yur(0.0, 0.0))
    }
    /// decide.js:283
    pub fn cevK(&self, p: &Oyuncu) -> f64 {
        self.AG(p)
    }
    /// Dönüş süresi (decide.js:283).
    pub fn donT(&self, p: &Oyuncu, ux: f64, uy: f64) -> f64 {
        let sp = hypot(z0(p.vx), z0(p.vy));
        if sp < 0.05 {
            return 0.0;
        }
        let c = jmax(-1.0, jmin(1.0, (z0(p.vx) * ux + z0(p.vy) * uy) / sp));
        acos(c) / PI / (0.099 * self.cevK(p))
    }
    /// Yoldaki gövdelerin etrafından dolanma payı (decide.js:289).
    pub fn dolanma(&self, p: &Oyuncu, tx: f64, ty: f64, src: &[Op], skip: Option<&Oyuncu>) -> f64 {
        let ax = tx - p.x;
        let ay = ty - p.y;
        let L = hypot(ax, ay);
        if L < 0.5 {
            return 0.0;
        }
        let mut ek = 0.0;
        let kimlik = dolanma_kimlik();
        // Bayrak açıkken varsayımsal kopya (p ya da skip src'deki aslının kopyası) kendi aslını engel saymaz.
        // Hayaletlerin id'si NaN: NaN == NaN yanlış, onlara dokunmaz.
        let ayni = |c: &Op, o: &Oyuncu| ayni_nesne(c, o) || (kimlik && c.team == o.team && c.id == o.id);
        for c in src {
            if ayni(c, p) || skip.map_or(false, |s| ayni(c, s)) || c.noTouch {
                continue;
            }
            let u = ((c.x - p.x) * ax + (c.y - p.y) * ay) / (L * L);
            if u <= 0.0 || u >= 1.0 {
                continue;
            }
            let dd = hypot(p.x + ax * u - c.x, p.y + ay * u - c.y);
            if dd < MIND {
                ek += 2.0 * (MIND - dd);
            }
        }
        ek
    }
    /// decide.js:302
    pub fn fakeOkunmaz(&self, q: &Oyuncu, h: &Oyuncu) -> f64 {
        match &q.pplan {
            Some(pp) if pp.k == "fake" => jmax(0.1, jmin(0.9, 0.5 + 0.04 * (q.yet.ozk("ceviklik") - h.yet.ozk("tepki")))),
            _ => 0.0,
        }
    }
    /// decide.js:4017
    pub fn yanilT(&self, p: &Oyuncu) -> f64 {
        jmax(3.0, jround(3.0 + 0.45 * jmax(0.0, 20.0 - p.yet.y(mek::FAKE_KAN)) - 0.2 * (p.yet.ceviklik - 10.0)))
    }
    /// İkili kazanılınca topu kapma katı (decide.js:4018).
    pub fn kWof(&self, omuz: Option<bool>, h: &Oyuncu, q: &Oyuncu, src: &[Op], ikK: bool) -> f64 {
        if !ikK {
            return 1.0;
        }
        let sS = match self.dis.sendS(h) {
            Some(s) => s,
            None => return 1.0,
        };
        if omuz == Some(true) {
            1.0 / (1.0 + exp(-(sS - jmax(3.0, jround(8.0 - 0.25 * (q.yet.y(mek::HAMLE) - 10.0)))) / 2.0))
        } else {
            0.6 + 0.4 * if src.iter().any(|o| !ayni_nesne(o, q) && o.team == q.team && !o.bekci() && !o.noTouch && hypot(o.x - h.x, o.y - h.y) < 12.0) { 0.5 } else { 0.0 }
        }
    }
    /// Pres rolleri (decide.js:4022): src sırasıyla (rolsüz: None).
    pub fn presRol(&self, h: &Oyuncu, src: &[Op], dir: Nk) -> Vec<Option<PresRol>> {
        let mut R: Vec<Option<PresRol>> = vec![None; src.len()];
        if !self.dis.AS_ilkPres_var() || !self.dis.mac_var() {
            return R;
        }
        let mut opp: Vec<usize> = (0..src.len()).filter(|&i| {
            let q = &src[i];
            q.team != h.team && !q.noTouch && !q.bekci()
        }).collect();
        if opp.is_empty() {
            return R;
        }
        let dt = src[opp[0]].team;
        let kx = if dt == 0.0 { 0.0 } else { GW };
        let near = hypot(h.x - kx, h.y - GCY) < 35.0 * (GW / 100.0);
        let tow = z0(h.vx) * (kx - h.x) + z0(h.vy) * (GCY - h.y) > 0.0;
        let g0 = if near {
            Some(self.dis.gside(nk(h.x, h.y), dt, 2.0))
        } else if tow {
            Some(self.dis.gside(nk(h.x + 10.0 * z0(h.vx), h.y + 10.0 * z0(h.vy)), dt, 2.0))
        } else {
            None
        };
        let tt = |q: &Oyuncu| match g0 {
            Some(g) => hypot(q.x - g.x, q.y - g.y) / self.vS(q) + self.dis.gec(q, g.x, g.y, None).unwrap_or(0.0),
            None => hypot(q.x - h.x, q.y - h.y),
        };
        let a = atan2(dir.y, dir.x);
        let t0 = self.dis.tehlike(h);
        let mut TH = vec![Tehlike { an: a, w: 1.0, w0: 1.0, x: h.x + 8.0 * dir.x, y: h.y + 8.0 * dir.y, k: "sür".into(), L: 8.0 }];
        for u in t0 {
            if atan2(sin(u.an - a), cos(u.an - a)).abs() >= 0.45 {
                TH.push(u);
            }
        }
        let dtac = self.dis.tacEff(dt);
        let gs = self.dis.sh_gs().map(|mut g| {
            g.t = f64::INFINITY;
            g
        });
        let mut cov = vec![TH[0].an];
        let dA = |x: f64, y: f64| atan2(sin(x - y), cos(x - y)).abs();
        let tts: Vec<f64> = (0..src.len()).map(|i| if opp.contains(&i) { tt(&src[i]) } else { 0.0 }).collect();
        opp.sort_by(|&a, &b| {
            let v = tts[a] - tts[b];
            if v < 0.0 {
                std::cmp::Ordering::Less
            } else if v > 0.0 {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
        for (i, &qi) in opp.iter().enumerate() {
            if i == 0 {
                R[qi] = Some(PresRol::Ilk { TH: TH.clone(), dtac: dtac.clone(), gs: gs.clone() });
                continue;
            }
            let t2 = TH.iter().find(|u| cov.iter().all(|&c| dA(u.an, c) > 0.8)).cloned();
            match t2 {
                Some(t2) => {
                    cov.push(t2.an);
                    R[qi] = Some(PresRol::An { an: t2.an, r: jmax(2.5, jmin(8.0, 0.35 * t2.L)) });
                }
                None => R[qi] = Some(PresRol::Dur),
            }
        }
        R
    }

    /// Sürerken bir rakiple kesişmeyi tik tik oynatır (decide.js:4071): topu tutma olasılığı.
    /// `kesisK_son`'a ilerleyişi yazar (JS kesisK.son).
    #[allow(clippy::too_many_arguments)]
    pub fn kesisK(&mut self, h: &Oyuncu, q: &Oyuncu, dir: Nk, T: f64, src: &[Op], _d0: f64, rol: Option<&PresRol>) -> f64 {
        let IK = self.ik();
        let spK = |p: &Oyuncu| self.sprKafa(p);
        let vB = self.spd(h) * if h.sprint { spK(h) } else { 1.0 } * self.topK(h);
        let vQ0 = self.spd(q);
        let vQs = vQ0 * spK(q);
        let spU = 4.0 + 8.0 * (1.0 - cl1(q.en.unwrap_or(1.0))) * q.yet.ca_k();
        let kX = ox_of(q.team);
        let la = 2.0 + 0.4 * h.yet.y(mek::KALKAN);
        let dlK = |p: &Oyuncu, b: f64| b * (1.0 + 0.0625 * (p.yet.y(mek::DENGE_DOLUM) - 10.0));
        let mH = self.dis.kutle(h, true);
        let mQ = self.dis.kutle(q, false);
        let rH = self.dis.direnc(h, true);
        let rQ = self.dis.direnc(q, false);
        let w = jmax(0.01, 0.03 * (20.0 - q.yet.y(mek::IKILI_KARAR)));
        let thr = self.girThr(q, h, src);
        let ek = self.enIkiliEk(q, h) + w / 2.0 - thr + self.girEk(q, h, thr, src);
        let t0 = self.tick0();
        let cvQ = 0.25 * (q.yet.y(mek::HAMLE) - 10.0);
        let c = Kc {
            h,
            q,
            src,
            dir,
            T,
            rol,
            IK,
            vB,
            vQ0,
            vQs,
            spU,
            spG: 12.0 + 10.0 * (1.0 - cl1(q.en.unwrap_or(1.0))) * q.yet.ca_k(),
            kX,
            la,
            dlH: dlK(h, IK.dolum),
            dlQ: dlK(q, IK.dolum),
            dlQs: dlK(q, IK.dolumS),
            mH,
            mQ,
            iH: 1.0 / mH,
            iQ: 1.0 / mQ,
            rH,
            rQ,
            w,
            ek,
            cvQ,
            vQf: self.vS(q),
            adS: jmax(3.0, jround(8.0 - cvQ)),
            t0,
        };
        let mut hv = h.clone();
        let mut qv = q.clone();
        hv.vx = z0(h.vx);
        hv.vy = z0(h.vy);
        hv.ca = Some(match h.ca {
            Some(a) => a,
            None => self.coreAng(h),
        });
        hv.den = Some(h.den.unwrap_or(1.0));
        qv.vx = z0(q.vx);
        qv.vy = z0(q.vy);
        qv.den = Some(q.den.unwrap_or(1.0));
        if let Some(rol) = rol {
            qv.pplan = None;
            let yas = self.dis.algi_yas(q);
            if let (Some(GRk), Some(yas)) = (self.dis.GR(), yas) {
                if yas > 0.0 && yas <= GRk.yasH / 2.0 {
                    let H2 = GRk.yasH;
                    let f = yas - yas * yas / (2.0 * H2);
                    let k = 1.0 - yas / H2;
                    let svx = qv.vx / k;
                    let svy = qv.vy / k;
                    qv.vx = svx;
                    qv.vy = svy;
                    qv.x -= svx * f;
                    qv.y -= svy * f;
                    let mut hg = hv.clone();
                    let mut j = 1.0;
                    while j <= yas {
                        hg.x = hv.x - hv.vx * (yas - j);
                        hg.y = hv.y - hv.vy * (yas - j);
                        let r0 = self.hedefQ(&c, rol, &mut qv, &hg, j - yas);
                        let ux = r0.x - qv.x;
                        let uy = r0.y - qv.y;
                        let L = hypot(ux, uy);
                        let vq = if L > spU && !is_icer(&qv.job, &["oyala", "bekliyor"]) { vQs } else { vQ0 };
                        let wq = if L < 0.3 { 0.0 } else { jmin(vq, 0.25 * L) };
                        let (wx, wy) = if L != 0.0 { (ux / L * wq, uy / L * wq) } else { (0.0, 0.0) };
                        self.dis.ivme(&mut qv, wx, wy, vq);
                        qv.x += qv.vx;
                        qv.y += qv.vy;
                        if KS_IZ.with(|f| f.get()) { eprintln!("     j {} r0 {} {} q {} {} {} {} hg {} {} job {:?}", j, r0.x, r0.y, qv.x, qv.y, qv.vx, qv.vy, hg.x, hg.y, qv.job); }
                        j += 1.0;
                    }
                    qv.pplan = None;
                }
            }
        }
        let hm = match &q.hamle {
            Some(hm) if hm.hid == h.id => Some(Hm { tr: jmax(0.0, hm.t0 + hm.W - t0), pc: 1.0, omuz: hm.k == "omuz" }),
            _ => None,
        };
        let kalS = q._kal && q._kalH.and_then(|i| src.get(i)).map_or(false, |x| x.id == h.id && x.team == h.team);
        let mut s = Kd {
            hv,
            qv,
            keep: 1.0,
            tK: -1e9,
            dMin: 1.0,
            qMin: 1.0,
            fk: 0.0,
            fkD: 0.0,
            fkOk: !(h.fiskeCd.map_or(false, |f| f > t0)) && !h.bekci(),
            hm,
            ad: None,
            qS: z0(q.sendT.unwrap_or(0.0)) - t0,
            qCd: z0(q.sendCd.unwrap_or(0.0)) - t0,
            kalS,
            sendY: None,
        };
        let mut son = 0.0;
        let mut bolundu = false;
        let kk = self.kes_oyna(&c, &mut s, 1.0, &mut son, &mut bolundu);
        self.kesisK_son.set(son);
        kk
    }
    fn hedefQ(&self, c: &Kc, rol: &PresRol, qv: &mut Oyuncu, hx: &Oyuncu, t: f64) -> Nk {
        match rol {
            PresRol::Ilk { TH, dtac, gs } => {
                self.dis.ilkPres(c.t0, c.t0 + t, qv, hx, c.q.team, TH, dtac, gs);
                nk(qv.tx, qv.ty)
            }
            PresRol::An { an, r } => nk(hx.x + cos(*an) * r, hx.y + sin(*an) * r),
            PresRol::Dur => nk(qv.x, qv.y),
        }
    }
    fn kes_core(hv: &Oyuncu) -> (f64, f64) {
        let ca = hv.ca.unwrap_or(0.0);
        (hv.x + cos(ca) * Q.body, hv.y + sin(ca) * Q.body)
    }
    fn kes_adan(c: &Kc, s: &mut Kd, t: f64, W: f64, omuz: bool) {
        s.hm = Some(Hm { tr: t + W, pc: 1.0, omuz });
        s.tK = t + W;
        let (cx, cy) = Self::kes_core(&s.hv);
        s.ad = Some(Ad { T: t + W + c.adS, x: s.qv.x + 1.6 * (cx - s.qv.x), y: s.qv.y + 1.6 * (cy - s.qv.y), g: false });
    }
    fn kes_oyna(&self, c: &Kc, s: &mut Kd, t1: f64, son: &mut f64, bolundu: &mut bool) -> f64 {
        let IK = &c.IK;
        let MD = TQ.min_d;
        let B = Q.body;
        let ENG = DU.engage + 0.6;
        let CRk = MD / 2.0 + 0.3;
        let yv = self.yavas();
        let h = c.h;
        let dir = c.dir;
        let mut t = t1;
        while t <= c.T {
            let mut g = ya(hypot(s.hv.x - s.qv.x, s.hv.y - s.qv.y), 1e-6);
            let mut tem = false;
            let nT = s.qS > t;
            {
                let (mut tx, mut ty);
                if let Some(rol) = c.rol {
                    let r0 = self.hedefQ(c, rol, &mut s.qv, &s.hv, t);
                    tx = r0.x;
                    ty = r0.y;
                } else if g > 4.0 {
                    let lx = s.hv.x + 10.0 * s.hv.vx;
                    let ly = s.hv.y + 10.0 * s.hv.vy;
                    let kl = ya(hypot(c.kX - lx, GCY - ly), 1.0);
                    tx = lx + (c.kX - lx) / kl * jmin(2.0, kl);
                    ty = ly + (GCY - ly) / kl * jmin(2.0, kl);
                } else {
                    let (cx, cy) = Self::kes_core(&s.hv);
                    if shielded(s.hv.x, s.hv.y, s.qv.x, s.qv.y, cx, cy) {
                        let bs = atan2(s.qv.y - s.hv.y, s.qv.x - s.hv.x);
                        let sg = sg1(w0(s.hv.ca.unwrap() - bs));
                        tx = s.hv.x + 1.9 * cos(bs + 1.25 * sg);
                        ty = s.hv.y + 1.9 * sin(bs + 1.25 * sg);
                    } else {
                        tx = cx;
                        ty = cy;
                    }
                    let an0 = atan2(GCY - s.hv.y, c.kX - s.hv.x);
                    let rx = tx - s.hv.x;
                    let ry = ty - s.hv.y;
                    let r = jmax(1.3, hypot(rx, ry));
                    let dv = w0(atan2(ry, rx) - an0);
                    let mx = acos(jmin(1.0, 1.2 / jmax(1.9, r)));
                    if dv.abs() > mx {
                        let a2 = an0 + jsign(dv) * mx;
                        let r2 = jmax(1.9, r);
                        tx = s.hv.x + cos(a2) * r2;
                        ty = s.hv.y + sin(a2) * r2;
                    }
                }
                let adK = match s.ad {
                    Some(ad) => s.hm.is_none() && t < ad.T,
                    None => false,
                };
                if adK {
                    let ad = s.ad.unwrap();
                    tx = ad.x;
                    ty = ad.y;
                } else if nT {
                    let sx = c.kX - s.hv.x;
                    let sy = GCY - s.hv.y;
                    let sl = ya(hypot(sx, sy), 1.0);
                    tx = s.hv.x + sx / sl * 5.0;
                    ty = s.hv.y + sy / sl * 5.0;
                }
                let gK = if adK { s.ad.unwrap().g } else { nT };
                let ux = tx - s.qv.x;
                let uy = ty - s.qv.y;
                let L = hypot(ux, uy);
                let hizli = (adK && !s.ad.unwrap().g)
                    || if gK {
                        (L > 2.0 && (s.qv.x - c.kX).abs() > (s.hv.x + cos(s.hv.ca.unwrap()) * B - c.kX).abs() - 1.0) || L > c.spG
                    } else {
                        L > c.spU && (c.rol.is_none() || !is_icer(&s.qv.job, &["oyala", "bekliyor"]))
                    };
                let vq = if hizli { c.vQs } else { c.vQ0 };
                let vqD = vq * if s.qS > t { 0.3 } else { 1.0 } * self.ikDen(&s.qv);
                let wq = if L < 0.3 { 0.0 } else { jmin(vqD, 0.25 * L) };
                let (wx, wy) = if L != 0.0 { (ux / L * wq, uy / L * wq) } else { (0.0, 0.0) };
                self.dis.ivme(&mut s.qv, wx, wy, vqD);
                if KS_IZ.with(|f| f.get()) { eprintln!("     t {} tx {} {} q {} {} {} {} h {} {} L {} vqD {} job {:?}", t, tx, ty, s.qv.x, s.qv.y, s.qv.vx, s.qv.vy, s.hv.x, s.hv.y, L, vqD, s.qv.job); }
            }
            {
                let vW = c.vB * self.ikDen(&s.hv) * if g < IK.kalkanR { IK.kalkan } else { 1.0 };
                self.dis.ivme(&mut s.hv, dir.x * vW, dir.y * vW, vW);
            }
            s.hv.x += s.hv.vx;
            s.hv.y += s.hv.vy;
            s.qv.x += s.qv.vx;
            s.qv.y += s.qv.vy;
            {
                let (cx, cy) = Self::kes_core(&s.hv);
                let CR = MD / 2.0 + 0.3;
                let dx = s.qv.x - cx;
                let dy = s.qv.y - cy;
                let dc = hypot(dx, dy);
                if dc < CR {
                    let ca = s.hv.ca.unwrap();
                    let ux = if dc > 0.001 { dx / dc } else { cos(ca) };
                    let uy = if dc > 0.001 { dy / dc } else { sin(ca) };
                    let vn = s.qv.vx * ux + s.qv.vy * uy;
                    s.qv.x = cx + ux * CR;
                    s.qv.y = cy + uy * CR;
                    if vn < 0.0 {
                        s.qv.vx -= vn * ux;
                        s.qv.vy -= vn * uy;
                    }
                }
            }
            let mut gx = s.hv.x - s.qv.x;
            let mut gy = s.hv.y - s.qv.y;
            g = ya(hypot(gx, gy), 1e-6);
            if g < MD {
                let ux = gx / g;
                let uy = gy / g;
                let o = MD - g;
                let sQ = c.iQ / (c.iQ + c.iH);
                s.qv.x -= ux * o * sQ;
                s.qv.y -= uy * o * sQ;
                s.hv.x += ux * o * (1.0 - sQ);
                s.hv.y += uy * o * (1.0 - sQ);
                let (avx, avy, bvx, bvy) = (s.qv.vx, s.qv.vy, s.hv.vx, s.hv.vy);
                let vr = (avx - bvx) * ux + (avy - bvy) * uy;
                let mut J = 0.0;
                if vr > 0.0 {
                    J = (1.0 + IK.esnek) * vr / (c.iQ + c.iH);
                    s.qv.vx -= J * c.iQ * ux;
                    s.qv.vy -= J * c.iQ * uy;
                    s.hv.vx += J * c.iH * ux;
                    s.hv.vy += J * c.iH * uy;
                }
                let nx = -uy;
                let ny = ux;
                let tq = s.qv.vx * nx + s.qv.vy * ny;
                let th = s.hv.vx * nx + s.hv.vy * ny;
                s.qv.vx -= tq * IK.surt * nx;
                s.qv.vy -= tq * IK.surt * ny;
                s.hv.vx -= th * IK.surt * nx;
                s.hv.vy -= th * IK.surt * ny;
                let yuk = J + if vr > 0.0 { 0.0015 } else { 0.0 } + 0.006 * (tq - th).abs();
                let gA = avx * ux + avy * uy;
                let gB = -(bvx * ux + bvy * uy);
                let fA = if gA > 0.02 && gA > gB { 0.5 } else { 1.0 };
                let fB = if gB > 0.02 && gB > gA { 0.5 } else { 1.0 };
                s.qv.den = Some(jmax(0.0, s.qv.den.unwrap() - yuk * fA * IK.dengeK / (c.mQ * c.rQ)));
                s.hv.den = Some(jmax(0.0, s.hv.den.unwrap() - kes_yuk() * yuk * fB * IK.dengeK / (c.mH * c.rH)));
                tem = true;
            }
            {
                let sd = s.qS > t;
                s.hv.den = Some(jmin(1.0, s.hv.den.unwrap() + c.dlH));
                s.qv.den = Some(jmin(1.0, s.qv.den.unwrap() + if sd { c.dlQs } else { c.dlQ }));
                if tem {
                    s.dMin = jmin(s.dMin, s.hv.den.unwrap());
                    s.qMin = jmin(s.qMin, s.qv.den.unwrap());
                }
                // maçın kuralı (mac/ikili.rs): taşıyıcı sendeleyince Çekirdek çekirdeğin yerinden taşıyıcının hızının
                // yarısıyla serbest kalır, taşıyıcı sendS boyunca hareketsiz; kim önce yetişirse top onun.
                if send_yaris() && s.sendY.is_none() && s.hv.den.unwrap() < IK.sendele {
                    let ca = s.hv.ca.unwrap();
                    let (bx, by) = (s.hv.x + cos(ca) * B + 6.0 * (0.5 * s.hv.vx + 0.14 * cos(ca)), s.hv.y + sin(ca) * B + 6.0 * (0.5 * s.hv.vy + 0.14 * sin(ca)));
                    let var = |p: &Oyuncu, x: f64, y: f64, bek: f64| bek + jmax(0.0, self.TEPKI(p, Tepki::BosTop)) + jmax(0.0, hypot(bx - x, by - y) - Q.reach) / jmax(0.05, self.spd(p));
                    // bizim taraf: sendeleyen taşıyıcı (sendS boyunca hareketsiz) ve takım arkadaşları (dünyadaki yerlerinden, t tik ilerlemiş)
                    let mut tBiz = self.dis.sendS(h).unwrap_or(IK.sendT) + hypot(bx - s.hv.x, by - s.hv.y) / jmax(0.05, self.spd(h));
                    // rakip taraf: benzetimdeki savunmacı (sendeliyorsa kalan süresi) ve öteki rakipler
                    // maçta Çekirdek'in `from`'u en yakın rakip: ilk 8 tik ona dokunamaz (temas.rs)
                    let dq = hypot(s.qv.x - s.hv.x, s.qv.y - s.hv.y);
                    let mut yakin_o: Option<usize> = None;
                    let mut yd = dq;
                    for (oi, o) in c.src.iter().enumerate() {
                        if o.team != h.team && !(o.team == c.q.team && o.id == c.q.id) {
                            let d = hypot(o.x - s.hv.x, o.y - s.hv.y);
                            if d < yd {
                                yd = d;
                                yakin_o = Some(oi);
                            }
                        }
                    }
                    let mut tRak = var(c.q, s.qv.x, s.qv.y, jmax(if yakin_o.is_none() { 8.0 } else { 0.0 }, s.qS - t));
                    for (oi, o) in c.src.iter().enumerate() {
                        if o.noTouch || o.bekci() || (o.team == h.team && o.id == h.id) || (o.team == c.q.team && o.id == c.q.id) {
                            continue;
                        }
                        let to = jmax(if yakin_o == Some(oi) { 8.0 } else { 0.0 }, var(o, o.x, o.y, 0.0) - t);
                        if o.team == h.team {
                            tBiz = jmin(tBiz, to);
                        } else {
                            tRak = jmin(tRak, to);
                        }
                    }
                    s.sendY = Some(1.0 / (1.0 + exp((tRak - tBiz) / 5.0)));
                    // maçta sürme burada biter (top serbest): benzetim de biter
                    break;
                }
                if !sd && s.qv.den.unwrap() < IK.sendele && t > s.qCd {
                    s.qS = t + self.dis.sendS(c.q).unwrap_or(IK.sendT);
                    s.qCd = t + IK.sendCd;
                }
            }
            gx = s.hv.x - s.qv.x;
            gy = s.hv.y - s.qv.y;
            g = ya(hypot(gx, gy), 1e-6);
            {
                let sp = hypot(s.hv.vx, s.hv.vy);
                let mv = if sp > 0.04 { atan2(s.hv.vy, s.hv.vx) } else { atan2(dir.y, dir.x) };
                let mut ta = mv;
                if g < 6.0 {
                    let aw = atan2(s.hv.y - (s.qv.y + s.qv.vy * c.la), s.hv.x - (s.qv.x + s.qv.vx * c.la));
                    ta = if sp > 0.04 { mv + 0.7 * jmax(-1.0, jmin(1.0, w0(aw - mv))) } else { aw };
                    if sp > 0.04 {
                        ta += w0(mv - ta) * cl1((sp / self.ikHiz(h) - 0.5) / 0.35);
                    }
                }
                let ca = s.hv.ca.unwrap();
                let dd = w0(ta - ca);
                s.hv._don = Some(jmin(1.0, dd.abs() / 1.2));
                if s.fk > 0.0 || dd.abs() > 0.02 {
                    let mut dr = if s.fk > 0.0 { s.fkD } else { jsign(dd) };
                    if s.fk <= 0.0 && g < 6.0 {
                        let pa = atan2(s.qv.y + s.qv.vy * c.la - s.hv.y, s.qv.x + s.qv.vx * c.la - s.hv.x);
                        let m1 = w0(ca + dd / 2.0);
                        let m2 = w0(ca + (dd - 2.0 * dr * PI) / 2.0);
                        if w0(m1 - pa).abs() < 0.9 && w0(m2 - pa).abs() > w0(m1 - pa).abs() {
                            dr = -dr;
                        }
                    }
                    let mut om = if s.fk > 0.0 { flick_w(h) } else { omega(&s.hv, g < 4.0, yv) };
                    let arc = if dr == jsign(dd) { dd.abs() } else { 2.0 * PI - dd.abs() };
                    if s.fk <= 0.0 && s.fkOk && g < 4.0 && arc > 1.2 && jmax(0.0, g - Q.reach) / c.vQf < arc / om {
                        let fw = flick_w(h);
                        let (hx, hy, qx, qy, qvx, qvy) = (s.hv.x, s.hv.y, s.qv.x, s.qv.y, s.qv.vx, s.qv.vy);
                        let vQf = c.vQf;
                        let yol = |d1: f64, a1: f64| -> bool {
                            let mut od = g;
                            let mut of = atan2(qy - hy, qx - hx);
                            let n = (a1 / fw).ceil() as i64;
                            for k in 1..=n {
                                let a = ca + d1 * jmin(a1, k as f64 * fw);
                                let (mut ox, mut oy);
                                if od < 3.0 {
                                    od = jmax(MD, od - vQf);
                                    of += jsign(w0(a - of)) * jmin(w0(a - of).abs(), vQf / od);
                                    ox = cos(of) * od;
                                    oy = sin(of) * od;
                                } else {
                                    ox = qx + qvx * k as f64 - hx;
                                    oy = qy + qvy * k as f64 - hy;
                                    let ol = ya(hypot(ox, oy), 1e-6);
                                    if ol < MD {
                                        ox *= MD / ol;
                                        oy *= MD / ol;
                                    }
                                }
                                if hypot(cos(a) * B - ox, sin(a) * B - oy) < CRk {
                                    return false;
                                }
                            }
                            true
                        };
                        let mut a1 = arc;
                        let mut d1 = dr;
                        let mut acik = true;
                        if !yol(d1, a1) {
                            if yol(-d1, 2.0 * PI - a1) {
                                d1 = -d1;
                                a1 = 2.0 * PI - a1;
                            } else {
                                acik = false;
                            }
                        }
                        if acik {
                            dr = d1;
                            s.fkOk = false;
                            s.keep *= 1.0 - 0.5 * flick_miss(h);
                            KES_SAY.with(|k| { let v = k.get(); k.set((v.0 + 1, v.1, v.2)); });
                            s.fk = (a1 / fw).ceil();
                            s.fkD = dr;
                            om = fw;
                        }
                    }
                    let ca1 = w0(ca + dr * if s.fk > 0.0 { om } else if dr == jsign(dd) { jmin(om, dd.abs()) } else { om });
                    if hypot(s.hv.x + cos(ca1) * B - s.qv.x, s.hv.y + sin(ca1) * B - s.qv.y) < CRk {
                        if s.fk > 0.0 {
                            s.keep *= 0.5;
                            KES_SAY.with(|k| { let v = k.get(); k.set((v.0, v.1 + 1, v.2)); });
                            s.fk = 0.0;
                        }
                    } else {
                        s.hv.ca = Some(ca1);
                    }
                    if s.fk > 0.0 {
                        s.fk -= 1.0;
                    }
                }
            }
            let mut bol: Option<Bol> = None;
            {
                if let Some(hm) = s.hm {
                    if hm.pc == 1.0 && t < hm.tr {
                        let ca = s.hv.ca.unwrap();
                        let ox = s.hv.x + if hm.omuz { 0.0 } else { cos(ca) * B } - s.qv.x;
                        let oy = s.hv.y + if hm.omuz { 0.0 } else { sin(ca) * B } - s.qv.y;
                        let ol = ya(hypot(ox, oy), 1.0);
                        if ol > MD + 0.1 {
                            let e = jmin(0.09 * self.dis.ikEn(&s.qv).unwrap_or(1.0), ol - MD - 0.1);
                            s.qv.x += ox / ol * e;
                            s.qv.y += oy / ol * e;
                        }
                    }
                }
                if let Some(hm) = s.hm {
                    if t >= hm.tr {
                        let ca = s.hv.ca.unwrap();
                        let yet = if hm.omuz { g < MD + 0.45 } else { hypot(s.qv.x - s.hv.x - cos(ca) * B, s.qv.y - s.hv.y - sin(ca) * B) < Q.reach + 1.0 };
                        if yet {
                            let d = self.duelP_k(&s.qv, &s.hv, c.src, Some(hm.omuz));
                            // omuz: maçta yalnız dengeyi düşürür, top sendelemeden sonraki serbest top yarışıyla gider
                            // (ALAN_SEND_YARIS_IZ açıkken doğrudan kayıp yok; yarış sendeleme anında)
                            let dus = if hm.omuz && send_yaris() { 0.0 } else { hm.pc * d.P * self.kWof(Some(hm.omuz), h, c.q, c.src, true) };
                            s.keep *= 1.0 - dus;
                            KES_SAY.with(|k| { let v = k.get(); k.set((v.0, v.1, v.2 + 1)); });
                            if KP_DER.with(|c| c.get()) == 1 {
                                KP_DENEME.with(|c| { let (n, a, b) = c.get(); c.set((n + 1, a + d.P, b + dus)); });
                                if crate::iz_acik() && std::env::var("ALAN_KDUEL_IZ").is_ok() {
                                    eprintln!("KDUEL [{},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4}]", hm.omuz, d.P, hm.pc, hypot(z0(s.hv.vx), z0(s.hv.vy)), z0(s.hv._don.unwrap_or(0.0)), s.hv.den.unwrap_or(1.0), self.ikHiz(h), self.kWof(Some(hm.omuz), h, c.q, c.src, true));
                                }
                            }
                            if hm.omuz {
                                let den = s.hv.den.unwrap();
                                s.hv.den = Some(den - hm.pc * jmax(0.0, den - d.son));
                            }
                        }
                        if hm.pc == 1.0 && !hm.omuz {
                            let ox = s.hv.x + cos(ca) * B - s.qv.x;
                            let oy = s.hv.y + sin(ca) * B - s.qv.y;
                            let ol = ya(hypot(ox, oy), 1.0);
                            let T0 = match s.ad {
                                Some(a) => a.T,
                                None => 0.0,
                            };
                            let ad = Ad { T: jmax(T0, t + self.yanilT(c.q)), x: s.qv.x + ox / ol * 2.6, y: s.qv.y + oy / ol * 2.6, g: true };
                            s.ad = Some(ad);
                            s.tK = jmax(s.tK, ad.T - DU.cd);
                        }
                        s.hm = None;
                    }
                }
                let acil = || {
                    let (cx, cy) = Self::kes_core(&s.hv);
                    s.kalS && !shielded(s.hv.x, s.hv.y, s.qv.x, s.qv.y, cx, cy)
                };
                if s.hm.is_none() && g <= ENG && (t - s.tK >= DU.cd || acil()) && !nT {
                    s.tK = t;
                    let d = self.duelP_k(&s.qv, &s.hv, c.src, None);
                    let pc = cl1((d.P + c.ek) / c.w);
                    let omuz = d.omuz;
                    s.kalS = omuz && pc < 1.0;
                    if pc > 0.0 {
                        let W = jmax(2.0, jround(if omuz { 4.0 } else { 5.0 } - c.cvQ));
                        if pc >= 1.0 || !*bolundu {
                            if pc >= 1.0 {
                                Self::kes_adan(c, s, t, W, omuz);
                            } else {
                                bol = Some(Bol { pc, W, omuz });
                            }
                        } else {
                            s.hm = Some(Hm { tr: t + W, pc, omuz });
                            s.tK = t + W * pc;
                        }
                    }
                }
            }
            if let Some(b) = bol {
                *bolundu = true;
                let z = s.clone();
                Self::kes_adan(c, s, t, b.W, b.omuz);
                let kG = self.kes_oyna(c, s, t + 1.0, son, bolundu);
                let sG = *son;
                *s = z;
                let kY = self.kes_oyna(c, s, t + 1.0, son, bolundu);
                *son = b.pc * sG + (1.0 - b.pc) * *son;
                return b.pc * kG + (1.0 - b.pc) * kY;
            }
            t += 1.0;
        }
        let pS = cl1((IK.sendele + 0.1 - s.dMin) / 0.2);
        let pQ = cl1((IK.sendele + 0.1 - s.qMin) / 0.2);
        *son = (s.hv.x - h.x) * dir.x + (s.hv.y - h.y) * dir.y;
        let son_k = match (s.sendY, kes_send()) {
            // sendelemeyle biten benzetim: o anki tutma × yarışı bizim kazanmamız (sondaki en düşük denge çarpanı yok)
            (Some(py), _) => s.keep * (1.0 - py),
            (None, Some(ks)) => s.keep * (1.0 - pS * ks),
            (None, None) => s.keep * (1.0 - pS * (1.0 - 0.5 * pQ)),
        };
        if son_k < 0.6 && KP_DER.with(|c| c.get()) == 1 && crate::iz_acik() && std::env::var("ALAN_KES_IZ").is_ok() {
            eprintln!("KES [{:.4},{:.4},{:.4},{:.4},{:.4},{:.2},{},{}]", son_k, s.keep, pS, s.dMin, pQ, hypot(h.x - c.q.x, h.y - c.q.y), s.sendY.map_or(-1.0, |v| v), KES_SAY.with(|k| { let v = k.get(); k.set((0, 0, 0)); format!("[{},{},{}]", v.0, v.1, v.2) }));
        }
        son_k
    }

    /// Topu T tik tutabilme olasılığı (decide.js:4429); `dir`: sürüş yönü (yok: yerinde tutar).
    /// `keepP_son`'a ilk kesişmeye dek ilerleyişi yazar.
    pub fn keepP(&mut self, h: &Oyuncu, src: &[Op], dir: Option<Nk>, T: f64) -> f64 {
        let (_kpd, kp_dis) = KpDer::gir();
        if kp_dis {
            KP_DENEME.with(|c| c.set((0, 0.0, 0.0)));
        }
        self.keepP_son.set(f64::INFINITY);
        if h.bekci() {
            if let Some(k) = h._korT {
                if k - self.tick() >= T {
                    return 1.0;
                }
            }
        }
        let mut ilS = f64::INFINITY;
        let mut keep = 1.0;
        let mut RL: Option<Vec<Option<PresRol>>> = None;
        let ikK = self.dis.IK().is_some();
        let IK = self.ik();
        let vC = self.spd(h) * if dir.is_some() { self.dis.ALAN_SPR(h) } else { 1.0 } * self.topK(h) * if ikK { self.ikDen(h) } else { 1.0 };
        let vCk = if ikK { vC * IK.kalkan } else { vC };
        let kapa = |yol: f64, vO: f64, away: f64| -> f64 {
            if !ikK {
                return yol / jmax(0.03, vO - vC * away);
            }
            let ky = jmin(yol, jmax(0.0, IK.kalkanR - DU.engage));
            (yol - ky) / jmax(0.03, vO - vC * away) + ky / jmax(0.03, vO - vCk * away)
        };
        let hT = match dir {
            Some(d) => self.donT(h, d.x, d.y),
            None => 0.0,
        };
        let tick0 = self.tick0();
        let mut hS: Option<Oyuncu> = None;
        for (qi, q) in src.iter().enumerate() {
            let q: &Oyuncu = q;
            if q.team == h.team || q.noTouch {
                continue;
            }
            let dx = h.x - q.x;
            let dy = h.y - q.y;
            let d = ya(hypot(dx, dy), 1e-6);
            let vO = self.vS(q);
            let away = match dir {
                Some(di) => (di.x * dx + di.y * dy) / d,
                None => 0.0,
            };
            let qT = if dir.is_some() { self.donT(q, dx / d, dy / d) } else { 0.0 };
            let yol = jmax(0.0, d - DU.engage) + self.dolanma(q, h.x, h.y, src, Some(h));
            let reachT = match dir {
                Some(di) => {
                    let hD = jmax(hT, self.donS(h, di.x, di.y));
                    let vTo = jmax(0.0, -(z0(h.vx) * dx + z0(h.vy) * dy) / d);
                    let c1 = vO + 0.5 * vTo;
                    qT + if yol <= c1 * hD { yol / c1 } else { hD + kapa(yol - c1 * hD, vO, away) }
                }
                None => jmax(0.0, qT - 0.5 * hT + kapa(yol, vO, away)),
            };
            let KIZ = crate::iz_acik() && dir.map_or(false, |di| std::env::var("ALAN_KIZ").ok().map_or(false, |v| { let (a, b) = v.split_once(":").unwrap_or(("", "0")); &*h.name == a && (di.x - b.parse::<f64>().unwrap_or(9.0)).abs() < 1e-4 })) && std::env::var("ALAN_C3_IZ").ok().map_or(false, |x| x == format!("{}", self.tick()));
            if KIZ && &*q.name == "Hendek" { let di = dir.unwrap(); eprintln!("   RT hT {} donS {} qT {} yol {} vO {} away {} d {}", hT, self.donS(h, di.x, di.y), qT, yol, vO, away, d); }
            if KIZ { eprintln!("   KQ {} reach {} T {} keep0 {} e {:?} x {} {}", q.name, reachT, T, keep, self.dis.algi_e(q), q.x, q.y); }
            kp_yaz!(kp_dis, (q.name.to_string(), d, reachT, keep, if q.bekci() { 9.0 } else { 0.0 }));
            if reachT > T {
                continue;
            }
            if ikK && !q.bekci() {
                let side = match dir {
                    Some(di) if hypot(z0(h.vx), z0(h.vy)) > 0.04 => Some((di.x, di.y)),
                    _ => None,
                };
                let pv = self.govdeVurP(q, h, side, false);
                let gk = match &q._gvKaz {
                    Some(g) if g.h == h.id && tick0 - g.t <= 4.0 => g.kaz,
                    _ => 1.0,
                };
                if pv > 0.0 {
                    keep *= 1.0 - pv * gk;
                }
            }
            if ikK && !q.bekci() {
                if let Some(di) = dir {
                    if RL.is_none() {
                        RL = Some(self.presRol(h, src, di));
                    }
                    let rl = RL.as_ref().unwrap()[qi].as_ref();
                    let e = z0(self.dis.algi_e(q).unwrap_or(0.0));
                    if e > 0.02 {
                        let a = e / 3f64.sqrt();
                        let mut k4 = 0.0;
                        let mut s4 = 0.0;
                        // örnek noktalar için rakibin tek kopyası (kesisK onu yalnız okur)
                        let mut qs = q.clone();
                        for (sx, sy) in [(-1.0, -1.0), (-1.0, 1.0), (1.0, -1.0), (1.0, 1.0)] {
                            qs.x = q.x + sx * a;
                            qs.y = q.y + sy * a;
                            let ks0 = KS_IZ.with(|f| f.get());
                            KS_IZ.with(|f| f.set(ks0 || (KIZ && sx == 1.0 && sy == 1.0 && !h.sprint)));
                            if KIZ && sx == 1.0 && sy == 1.0 && !h.sprint { eprintln!("    QS ic={} {} {} {} {} yas {:?} job {:?} pplan {}", ks0, qs.x, qs.y, qs.vx, qs.vy, qs.__yas, qs.job, qs.pplan.is_some()); }
                            let kk = self.kesisK(h, &qs, di, T, src, hypot(h.x - qs.x, h.y - qs.y), rl);
                            KS_IZ.with(|f| f.set(ks0));
                            k4 += kk;
                            s4 += self.kesisK_son.get();
                            if KIZ { eprintln!("    S {} {} k {} son {} rl {:?} hspr {} {:?} hca {:?} hv {} {}", sx, sy, kk, self.kesisK_son.get(), rl, h.sprint, h._sf.as_ref().map(|s| s.f.clone()), h.ca, h.vx, h.vy); }
                        }
                        keep *= k4 / 4.0;
                        if s4 / 4.0 < ilS {
                            ilS = s4 / 4.0;
                        }
                        if KIZ { eprintln!("   KK4 {} keep {} son {}", q.name, keep, s4 / 4.0); }
                    } else {
                        let kk = self.kesisK(h, q, di, T, src, d, rl);
                        keep *= kk;
                        if self.kesisK_son.get() < ilS {
                            ilS = self.kesisK_son.get();
                        }
                    }
                    continue;
                }
            }
            let mut qv = q.clone();
            qv.vx = 0.0;
            qv.vy = 0.0;
            let C0dk = self.dis.ALAN_DOKUNUS();
            let (eAt, dkKap) = match &C0dk {
                Some(C0) => {
                    let dkAg = |L0: f64| jmin(C0.aMax.unwrap_or(1.1), jmax(0.0, jmin(C0.lMax, L0)) * C0.gor.unwrap_or(0.4));
                    // `_dkP` modelde yok: 0
                    let dkE0 = if dir.is_some() { 0.5 * dkAg(0.0) } else { z0(h._dke.unwrap_or(0.0)) };
                    let eAt = if C0.riskEski { None } else { Some(jmax(0.5 * dkAg((DU.engage - C0.bos) * C0.k), dkE0 - C0.toplaG.unwrap_or(0.1) * jmax(0.0, reachT))) };
                    (eAt, dkE0 / C0.toplaG.unwrap_or(0.1))
                }
                None => (None, 0.0),
            };
            // taşıyıcının tek kopyası (rakip başına yalnız değişen alanları tazelenir)
            let hArg = hS.get_or_insert_with(|| h.clone());
            hArg.vx = h.vx;
            hArg.vy = h.vy;
            hArg._don = h._don;
            hArg.ca = h.ca;
            hArg._dke = h._dke;
            let hArg = &mut *hArg;
            if let Some(di) = dir {
                let mv = atan2(di.y, di.x);
                let aw = atan2(h.y - q.y, h.x - q.x);
                let mut ta = mv + 0.7 * jmax(-1.0, jmin(1.0, w0(aw - mv)));
                if ikK {
                    ta += w0(mv - ta) * jmax(0.0, jmin(1.0, (vC / self.ikHiz(h) - 0.5) / 0.35));
                    hArg.vx = di.x * vC;
                    hArg.vy = di.y * vC;
                }
                let c0 = match h.ca {
                    Some(a) => a,
                    None => self.coreAng(h),
                };
                let dd = w0(ta - c0);
                if ikK {
                    hArg._don = Some(jmin(1.0, dd.abs() / 1.2));
                }
                let om = omega(h, false, self.yavas());
                hArg.ca = Some(c0 + jsign(dd) * jmin(dd.abs(), om * jmax(0.0, reachT - dkKap)));
            } else if ikK {
                let c0 = match h.ca {
                    Some(a) => a,
                    None => self.coreAng(h),
                };
                let dd = w0(atan2(h.y - q.y, h.x - q.x) - c0);
                let om = omega(h, d < 4.0, self.yavas());
                hArg.ca = Some(c0 + jsign(dd) * jmin(dd.abs(), om * jmax(0.0, reachT - dkKap)));
            }
            if let Some(e) = eAt {
                hArg._dke = Some(e);
            }
            let d0 = self.duelP_k(&qv, hArg, src, None);
            let P = d0.P;
            let tries = 1.0 + ((T - reachT) / DU.cd).floor();
            let w = jmax(0.01, 0.03 * (20.0 - q.yet.y(mek::IKILI_KARAR)));
            let thr = self.girThr(q, h, src);
            let pcOf = |s: &mut Kafa, Pk: f64| jmax(0.0, jmin(1.0, (Pk + s.enIkiliEk(q, h) + w / 2.0 - thr + s.girEk(q, h, thr, src)) / w));
            let pc = pcOf(self, P);
            let hvB = &*hArg;
            let mut dCarp: Option<f64> = None;
            let mut Pd = P;
            let mut pcd = pc;
            let mut kd = if ikK { Some(d0.omuz) } else { None };
            let mut sd0 = d0.son;
            let BD = TQ.min_d;
            if let Some(di) = dir {
                if ikK && away < -0.2 && d <= BD + 2.0 && (di.x * (q.y - h.y) - di.y * (q.x - h.x)).abs() < BD {
                    let ux0 = dx / d;
                    let uy0 = dy / d;
                    let hA = -(z0(h.vx) * ux0 + z0(h.vy) * uy0);
                    let qA = z0(q.vx) * ux0 + z0(q.vy) * uy0;
                    let kv = |v0: f64, vm: f64, p: &Oyuncu| jmax(0.0, v0) + (vm - jmax(0.0, v0)) * (1.0 - pow(1.0 - 0.18 * self.AH(p), jmax(0.0, reachT)));
                    let vA = kv(hA, vC * -away, h);
                    let vQ = jmax(0.0, if qA > 0.05 { qA } else { kv(qA, vO, q) }) * if vA > 0.05 { 0.64 } else { 0.09 };
                    let vr = vA + vQ;
                    let mQ = self.dis.kutle(q, false);
                    let mH = self.dis.kutle(h, true);
                    let J = (1.0 + IK.esnek) * vr / (1.0 / mQ + 1.0 / mH);
                    let yk = J + 0.0015;
                    let dH = h.den.unwrap_or(1.0) - yk * if vA > vQ { 0.5 } else { 1.0 } * IK.dengeK / (mH * self.dis.direnc(h, true));
                    let dQ = q.den.unwrap_or(1.0) - yk * if vQ > vA { 0.5 } else { 1.0 } * IK.dengeK / (mQ * self.dis.direnc(q, false));
                    let pS = cl1((IK.sendele + 0.1 - dH) / 0.2);
                    let pQ = cl1((IK.sendele + 0.1 - dQ) / 0.2);
                    keep *= 1.0 - pS * (1.0 - 0.5 * pQ);
                    dCarp = Some(jmax(0.0, dH));
                }
            }
            if let (Some(dc), true) = (dCarp, ikK) {
                let mut hvC = hvB.clone();
                hvC.den = Some(dc);
                let d1 = self.duelP_k(&qv, &hvC, src, None);
                Pd = d1.P;
                pcd = pcOf(self, Pd);
                kd = Some(d1.omuz);
                sd0 = d1.son;
            }
            let HS = q.hamle.as_ref().map_or(false, |hm| hm.hid == h.id);
            let trK = if HS { tries - 1.0 } else { tries };
            if HS {
                let hk = q.hamle.as_ref().unwrap().k == "omuz";
                keep *= 1.0 - self.duelP_k(q, h, src, Some(hk)).P * self.kWof(Some(hk), h, q, src, ikK);
                if trK <= 0.0 {
                    continue;
                }
            }
            if pcd <= 0.0 {
                continue;
            }
            let kW = self.kWof(kd, h, q, src, ikK);
            if ikK && kd == Some(true) && trK > 1.0 {
                let dl = IK.dolum * (1.0 + 0.0625 * (h.yet.y(mek::DENGE_DOLUM) - 10.0)) * DU.cd;
                let mut Pk = Pd;
                let mut ck = pcd;
                let mut dk = sd0;
                let mut k = 0.0;
                while k < trK {
                    keep *= 1.0 - ck * Pk * kW;
                    if k == trK - 1.0 {
                        break;
                    }
                    let mut hv2 = hvB.clone();
                    hv2.den = Some(jmin(1.0, jmax(0.0, dk) + dl));
                    let d2 = self.duelP_k(&qv, &hv2, src, None);
                    Pk = d2.P;
                    ck = pcOf(self, Pk);
                    dk = if d2.omuz { d2.son } else { hv2.den.unwrap() };
                    k += 1.0;
                }
            } else {
                keep *= pow(1.0 - pcd * Pd * kW, trK);
            }
        }
        self.keepP_son.set(ilS);
        kp_yaz!(kp_dis, ("=".into(), 0.0, 0.0, keep, 0.0));
        if kp_dis {
            let (n, a, b) = KP_DENEME.with(|c| c.get());
            kp_yaz!(kp_dis, ("#".into(), n as f64, a, b, 0.0));
        }
        keep
    }

    /// Gövdeyle vurup topu düşürme olasılığı (decide.js:4550).
    pub fn govdeVurP(&self, q: &Oyuncu, h: &Oyuncu, side: Option<(f64, f64)>, vurus: bool) -> f64 {
        if q.bekci() {
            return 0.0;
        }
        let dF = match self.dis.durtFark(q, h) {
            Some(v) => v,
            None => return 0.0,
        };
        let (ux, uy) = match side {
            Some(s) => s,
            None => {
                let cs = self.coreSide(h);
                (cs.ux, cs.uy)
            }
        };
        let B = Q.body;
        let kx = h.x + ux * B;
        let ky = h.y + uy * B;
        if shielded(h.x, h.y, q.x, q.y, kx, ky) {
            return 0.0;
        }
        {
            let MD = TQ.min_d;
            let RC = Q.reach;
            let rx = q.x - h.x;
            let ry = q.y - h.y;
            let rl = ya(hypot(rx, ry), 1e-6);
            if (rx * ux + ry * uy) / rl < (MD * MD + B * B - RC * RC) / (2.0 * MD * B) {
                return 0.0;
            }
        }
        let bx = kx - q.x;
        let by = ky - q.y;
        let bl = ya(hypot(bx, by), 1e-6);
        let vr = ((z0(q.vx) - z0(h.vx)) * bx + (z0(q.vy) - z0(h.vy)) * by) / bl;
        let hz = if vurus { jmax(0.0, jmin(1.0, vr / 0.25)) } else { 0.7 };
        jmax(0.05, jmin(0.85, 0.25 + 0.03 * dF + 0.3 * hz + 0.3 * (1.0 - h.den.unwrap_or(1.0))))
    }
    /// Çekirdeğin bulunduğu yön (decide.js:4561).
    pub fn coreSide(&self, h: &Oyuncu) -> CoreSide {
        coreSide_p(h)
    }
    /// decide.js:4635
    pub fn coreAng(&self, h: &Oyuncu) -> f64 {
        let cs = self.coreSide(h);
        atan2(cs.uy, cs.ux)
    }
    /// Kapanmışlık (decide.js:4585): 16 yönün kaçı takımın gövdeleriyle (ya da sahanın sınırıyla) kapalı.
    pub fn trapped(&self, h: &Oyuncu, src: &[Op], team: f64) -> f64 {
        let mut blocked = 0;
        for k in 0..16 {
            let a = k as f64 * PI / 8.0;
            let dx = cos(a);
            let dy = sin(a);
            let mut b = false;
            let ex = h.x + 3.0 * dx;
            let ey = h.y + 3.0 * dy;
            if ex < 0.8 || ex > GW8 || ey < 0.8 || ey > GH8 {
                b = true;
            } else {
                for o in src {
                    if o.team != team || o.bekci() || o.noTouch {
                        continue;
                    }
                    let ox = o.x - h.x;
                    let oy = o.y - h.y;
                    let od = hypot(ox, oy);
                    if !(od > DU.trapR || od < 0.01) && (ox * dx + oy * dy) / od > cos(DU.trapA) {
                        b = true;
                        break;
                    }
                }
            }
            if b {
                blocked += 1;
            }
        }
        blocked as f64 / 16.0
    }
    /// decide.js:4604
    pub fn ikHiz(&self, h: &Oyuncu) -> f64 {
        self.vS(h)
    }
    /// Dengesi bozuk oyuncunun hız katı (decide.js:4604).
    pub fn ikDen(&self, h: &Oyuncu) -> f64 {
        let IK = self.ik();
        let d = h.den.unwrap_or(1.0);
        if d < IK.yavas {
            0.45 + 0.55 * d / IK.yavas
        } else {
            1.0
        }
    }
    /// İkili olasılığı ve türü (decide.js:4609 duelP2 / 4622 duelP); `zorK`: türü zorla (omuz mu).
    pub fn duelP_k(&self, q: &Oyuncu, h: &Oyuncu, src: &[Op], zorK: Option<bool>) -> DuelSon {
        match self.dis.IK() {
            Some(IK) => {
                let cl0 = |v: f64, a: f64, b: f64| jmax(a, jmin(b, v));
                let cs = self.coreSide(h);
                let B = Q.body;
                let Bo = B + z0(h._dke.unwrap_or(0.0));
                let ox = h.x + cs.ux * Bo;
                let oy = h.y + cs.uy * Bo;
                let kG = ox_of(h.team);
                let uzan = h.bekci() && hypot(ox - kG, oy - GCY) < KR + 0.5 && hypot(ox - q.x, oy - q.y) < Q.reach + 0.4;
                let den = h.den.unwrap_or(1.0);
                let omuz = match zorK {
                    Some(z) => z,
                    None => shielded(h.x, h.y, q.x, q.y, ox, oy) && !uzan,
                };
                if omuz {
                    let nx = h.x - q.x;
                    let ny = h.y - q.y;
                    let nl = ya(hypot(nx, ny), 1.0);
                    let ux = nx / nl;
                    let uy = ny / nl;
                    let gq = self.dis.omuzK(q);
                    let vr = jmax(0.0, (z0(q.vx) + ux * IK.omuzV * gq - z0(h.vx)) * ux + (z0(q.vy) + uy * IK.omuzV * gq - z0(h.vy)) * uy);
                    let mQ = self.dis.kutle(q, ayni_nesne(q, h));
                    let mH = self.dis.kutle(h, true);
                    let J = (1.0 + IK.esnek) * vr / (1.0 / mQ + 1.0 / mH);
                    let son = den - (J + 0.0015) * IK.dengeK / (mH * self.dis.direnc(h, true));
                    return DuelSon { P: cl0((IK.sendele + 0.1 - son) / 0.2, 0.05, 0.9), omuz: true, son };
                }
                let sp = hypot(z0(h.vx), z0(h.vy));
                let hz = cl0((sp / self.ikHiz(h) - 0.5) / 0.5, 0.0, 1.0);
                let acik = cl0((0.55 * (1.0 - den) + hz * z0(h._don.unwrap_or(0.0)) * 0.7 + 0.3 * hz) / 0.8, 0.0, 1.0);
                let dF = self.dis.durtFark(q, h).unwrap_or(0.0);
                DuelSon { P: cl0(IK.pokeT + dF * IK.pokeK + acik * IK.pokeAc + (1.0 - den) * IK.pokeDn, 0.05, 0.95), omuz: false, son: 1.0 }
            }
            None => {
                let cs = self.coreSide(h);
                let qx = q.x - h.x;
                let qy = q.y - h.y;
                let qd = ya(hypot(qx, qy), 1.0);
                let cosA = (qx * cs.ux + qy * cs.uy) / qd;
                let mut P = DU.base + (q.yet.ozk("kesme") - h.yet.ozk("surme")) * DU.skill + DU.side * cosA * (cs.off / DU.core0) * 0.5;
                {
                    let B = Q.body;
                    let ox = h.x + cs.ux * B;
                    let oy = h.y + cs.uy * B;
                    let kG = ox_of(h.team);
                    let uzan = h.bekci() && hypot(ox - kG, oy - GCY) < KR + 0.5 && hypot(ox - q.x, oy - q.y) < Q.reach + 0.4;
                    let fk = self.fakeOkunmaz(q, h);
                    let shd = shielded(h.x, h.y, q.x, q.y, ox, oy) && !uzan;
                    let gk = jmax(0.5, jmin(1.5, 1.0 - 0.04 * (h.yet.ozk("guc") - q.yet.ozk("guc"))));
                    let Pk = if shd { jmin(P, P * DU.shieldK * gk) } else { P + (1.0 - P) * DU.openK };
                    let Pf = if shd { P + (1.0 - P) * DU.openK } else { Pk };
                    P = (1.0 - fk) * Pk + fk * Pf;
                }
                let tr = self.trapped(h, src, q.team);
                P += (1.0 - P) * DU.trap * jmax(0.0, tr - DU.trap0) / (1.0 - DU.trap0);
                DuelSon { P: jmin(0.9, jmax(0.05, P)), omuz: false, son: 1.0 }
            }
        }
    }
    /// İkili olasılığı (decide.js:4622).
    pub fn duelP(&self, q: &Oyuncu, h: &Oyuncu, src: &[Op], zorK: Option<bool>) -> f64 {
        self.duelP_k(q, h, src, zorK).P
    }
    /// İkilinin sertliği (decide.js:4644): taktiğin şiddeti; yapay zekâ takımı geriyse sertleşir.
    pub fn siddetQ(&self, team: f64) -> f64 {
        match self.dis.tac_siddet(team) {
            None => 0.5,
            Some((s, yz, macK)) => jmin(1.0, s.unwrap_or(0.5) + if yz && macK < 0.6 { 0.25 } else { 0.0 }),
        }
    }
    /// Oyalama (decide.js:4658).
    pub fn oyalaEk(&self, q: &Oyuncu, h: &Oyuncu, thr: f64) -> f64 {
        if q.bekci() {
            return 0.0;
        }
        match self.dis.sh_gs() {
            Some(g) if g.oy && g.dt == q.team && !(g.t < self.tick0() - 1.0) && hypot(h.x - ox_of(q.team), h.y - GCY) > DU.oyalaR => {
                if thr >= DU.thrLast {
                    DU.oyala
                } else {
                    DU.oyalaC
                }
            }
            _ => 0.0,
        }
    }
    /// decide.js:4663
    pub fn enIkiliEk(&self, q: &Oyuncu, h: &Oyuncu) -> f64 {
        match (q.en, h.en) {
            (Some(a), Some(b)) => DU.enFark * (a - b),
            _ => 0.0,
        }
    }
    /// Maçın pres tartısından ikiliye girme eşiği (decide.js:4668; JS gibi tik başına önbellekli).
    pub fn girEsik(&mut self, q: &Oyuncu, h: &Oyuncu, src: &[Op], ch: Option<f64>) -> Option<f64> {
        if self.ge_ic.get() {
            return None;
        }
        let t = self.tick();
        {
            let mut g = self.ge.borrow_mut();
            if g.0 != Some(t) {
                g.1.clear();
                g.2.clear();
                g.0 = Some(t);
            }
        }
        // JS anahtarı `${q.id}|${q.team}|${h.id}|...`: metin yerine aynı eşitlikte sayı anahtarı (crate::js_sayi_k)
        let sk = crate::js_sayi_k;
        let k = [sk(q.id), sk(q.team), sk(h.id), sk(jround(2.0 * q.x)), sk(jround(2.0 * q.y)), sk(jround(2.0 * h.x)), sk(jround(2.0 * h.y))];
        if let Some(&v) = self.ge.borrow().1.get(&k) {
            return if v < 0.0 { None } else { Some(v) };
        }
        self.ge_ic.set(true);
        let v = (|| {
            let c = ch.or_else(|| self.dis.mac_ch()).unwrap_or(0.3);
            let cs = self.coreSide(h);
            let B = Q.body;
            let kx = h.x + cs.ux * B;
            let ky = h.y + cs.uy * B;
            let hz: Option<PrT> = self.dis.prT().filter(|tP| {
                tP.team == h.team
                    && tP.hm.is_some()
                    && tP.hmk.as_ref().map_or(false, |s| s.contains(&q.id))
                    && tP.h.is_some()
                    && t - tP.t <= 20.0
                    && (!tP.hId.map_or(false, |x| x != 0.0 && x == x) || tP.hId == Some(h.id))
            });
            let hz = match hz {
                Some(z) => z,
                None => return -1.0,
            };
            let Vn = hz.h.unwrap();
            let G = z0(hz.hm.as_ref().unwrap().iter().find(|e| e.0 == q.id).map_or(0.0, |e| e.1));
            let vk = [sk(q.team), sk(jround(kx)), sk(jround(ky))];
            let Vk = {
                let o = self.ge.borrow().2.get(&vk).copied();
                match o {
                    Some(v) => v,
                    None => {
                        let v = yerDeger(self, src, q.team, kx, ky, c);
                        self.ge.borrow_mut().2.insert(vk, v);
                        v
                    }
                }
            };
            let den = Vn + Vk + G;
            if den > 1e-6 {
                G / den
            } else {
                0.5
            }
        })();
        self.ge_ic.set(false);
        self.ge.borrow_mut().1.insert(k, v);
        if v < 0.0 {
            None
        } else {
            Some(v)
        }
    }
    /// girEsik'in yer değeri önbelleğine (JS `GEk`) bu tik için değer koy: sınama ve birleştirme kancası
    /// (yerDeger kafa2'nin; sınamada JS'teki değeri verir).
    pub fn ge_vk_koy(&self, key: String, v: f64) {
        let t = self.tick();
        let mut g = self.ge.borrow_mut();
        if g.0 != Some(t) {
            g.1.clear();
            g.2.clear();
            g.0 = Some(t);
        }
        // JS anahtarı "team|x,y" → sayı anahtarı
        let n: Vec<f64> = key.split(|c| c == '|' || c == ',').map(|t| t.parse::<f64>().unwrap_or(f64::NAN)).collect();
        g.2.insert([crate::js_sayi_k(n[0]), crate::js_sayi_k(n[1]), crate::js_sayi_k(n[2])], v);
    }
    /// decide.js:4701
    pub fn girThr(&mut self, q: &Oyuncu, h: &Oyuncu, src: &[Op]) -> f64 {
        match self.girEsik(q, h, src, None) {
            Some(g) => g,
            None => self.commitThr(q, h, src),
        }
    }
    /// decide.js:4705
    pub fn girEk(&mut self, q: &Oyuncu, h: &Oyuncu, thr: f64, src: &[Op]) -> f64 {
        if self.girEsik(q, h, src, None).is_some() {
            0.0
        } else {
            -self.oyalaEk(q, h, thr) + 0.2 * (self.siddetQ(q.team) - 0.5)
        }
    }
    /// İkiliye girme eşiği (decide.js:4708): arkada örten varsa düşük.
    pub fn commitThr(&self, q: &Oyuncu, h: &Oyuncu, src: &[Op]) -> f64 {
        let gx = ox_of(q.team);
        for o in src {
            if !ayni_nesne(o, q) && o.team == q.team && !o.bekci() && (o.x - gx).abs() < (h.x - gx).abs() - 1.0 && hypot(o.x - h.x, o.y - h.y) < 14.0 {
                return DU.thrCover;
            }
        }
        DU.thrLast
    }
}

thread_local! { static KS_IZ: std::cell::Cell<bool> = const { std::cell::Cell::new(false) }; }
thread_local! {
    /// ölçüm (ALAN_KPQ_IZ): keepP'de her rakibin başında (ad, uzaklık, yetişme, o anki tutma, 9 = Bekçi); sonda ("=", …, tutma)
    pub static KP_KAYIT: std::cell::RefCell<Option<Vec<(String, f64, f64, f64, f64)>>> = const { std::cell::RefCell::new(None) };
    /// keepP'nin iç içe derinliği: kayıt yalnız en dıştaki çağrıdan
    static KP_DER: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
    /// ölçüm (ALAN_KES_IZ): kes_oyna'da (fiske denemesi, fiske çarpışması, kapma girişimi) sayıları
    static KES_SAY: std::cell::Cell<(u32, u32, u32)> = const { std::cell::Cell::new((0, 0, 0)) };
    /// en dıştaki keepP'de kapma girişimleri: (sayı, d.P toplamı, düşülen olasılık toplamı)
    static KP_DENEME: std::cell::Cell<(u32, f64, f64)> = const { std::cell::Cell::new((0, 0.0, 0.0)) };
}
struct KpDer;
impl KpDer {
    fn gir() -> (KpDer, bool) {
        let d = KP_DER.with(|c| { let d = c.get(); c.set(d + 1); d });
        (KpDer, d == 0)
    }
}
impl Drop for KpDer {
    fn drop(&mut self) {
        KP_DER.with(|c| c.set(c.get() - 1));
    }
}
pub fn ks_iz() -> bool { KS_IZ.with(|f| f.get()) }

/// coreSide (decide.js): kafa durumuna bağlı değil, yerleşim de doğrudan çağırır.
pub fn coreSide_p(h: &Oyuncu) -> CoreSide {
        let sp = hypot(z0(h.vx), z0(h.vy));
        let (ux, uy);
        if let Some(ca) = h.ca {
            ux = cos(ca);
            uy = sin(ca);
        } else if sp > 0.04 {
            ux = h.vx / sp;
            uy = h.vy / sp;
        } else if hypot(h.tx - h.x, h.ty - h.y) > 0.5 {
            let L = hypot(h.tx - h.x, h.ty - h.y);
            ux = (h.tx - h.x) / L;
            uy = (h.ty - h.y) / L;
        } else {
            let gx = if h.team == 0.0 { GW } else { 0.0 };
            let L = ya(hypot(gx - h.x, GCY - h.y), 1.0);
            ux = (gx - h.x) / L;
            uy = (GCY - h.y) / L;
        }
        CoreSide { ux, uy, off: DU.core0 + sp * DU.coreV }
}
/// topK (decide.js): dokunuş ayarıyla, kafa durumuna bağlı değil.
pub fn topK_p(h: &Oyuncu, dk: Option<crate::kafa::gecici_dis1::Dokunus>) -> f64 {
    let s = h.yet.y(mek::TOPLU_KOSU);
    match dk {
        Some(C0) => {
            let close = jmin(1.0, C0.close + (s - 10.0) * C0.closeS);
            let Lr = jmax(0.8, C0.lRun - (s - 10.0) * C0.lRunS);
            close + (1.0 - close) * jmin(1.0, 0.0 / Lr)
        }
        None => jmin(1.0, 0.68 + 0.02 * s),
    }
}

/// Deneme (ayar arayışı, ALAN_KES_YUK=<katsayı>): benzetimde taşıyıcının gövde temasından kaybettiği denge ölçeklenir.
fn kes_yuk() -> f64 {
    static A: std::sync::OnceLock<f64> = std::sync::OnceLock::new();
    *A.get_or_init(|| if crate::iz_acik() { std::env::var("ALAN_KES_YUK").ok().and_then(|v| v.parse().ok()).unwrap_or(1.0) } else { 1.0 })
}
/// Deneme (ayar arayışı, ALAN_KES_SEND=<olasılık>): benzetimin sonunda taşıyıcı sendelediyse topun gitme olasılığı
/// (özgün: 1 − 0,5·pQ; gerçek maçta sendeleyen taşıyıcının topu kaybı ~0,4).
fn kes_send() -> Option<f64> {
    static A: std::sync::OnceLock<Option<f64>> = std::sync::OnceLock::new();
    *A.get_or_init(|| if crate::iz_acik() { std::env::var("ALAN_KES_SEND").ok().and_then(|v| v.parse().ok()) } else { None })
}

/// Deney (varsayılan kapalı, ALAN_KP_KAL_IZ): sürmenin tutma kestirimi ölçülen gerçekle eşlenir.
/// alan-kos tutma (kilitli, 4 maç, kaleye 35 br içi): kestirim 0,03 → gerçek 0,74; 0,30 → 0,75; 0,53 → 0,90;
/// 0,69 → 0,83; 1,00 → 0,97. Doğrusal uydurma: P' = 0,74 + 0,23 P.
/// alan-kos an: deneyi maçın ortasında açmak için (yalnız ölçüm aracı yazar)
pub static KP_KAL_ZORLA: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub fn kp_kal(p: f64) -> f64 {
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    let acik = KP_KAL_ZORLA.load(std::sync::atomic::Ordering::Relaxed) || *A.get_or_init(|| crate::iz_acik() && std::env::var("ALAN_KP_KAL_IZ").is_ok());
    if acik && p.is_finite() {
        0.74 + 0.23 * jmax(0.0, jmin(1.0, p))
    } else {
        p
    }
}

/// MIMARI.md 3.3 "dolanma": ALAN_DOLANMA_KIMLIK açıkken dolanma, kopyanın kendi aslını da atlar
/// (nesne kimliği yerine oyuncu kimliği). Varsayılan kapalı; tarayıcıda hep kapalı.
fn dolanma_kimlik() -> bool {
    if cfg!(target_family = "wasm") {
        return false;
    }
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *A.get_or_init(|| std::env::var("ALAN_DOLANMA_KIMLIK").is_ok())
}

/// Deney (varsayılan kapalı, ALAN_SEND_YARIS_IZ): benzetimde taşıyıcı sendeleyince maçın kuralıyla serbest top yarışı
/// (Çekirdek çekirdeğin yerinden çıkar, taşıyıcı sendS boyunca hareketsiz); özgünü: sondaki en düşük dengeye göre kayıp.
fn send_yaris() -> bool {
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *A.get_or_init(|| crate::iz_acik() && std::env::var("ALAN_SEND_YARIS_IZ").is_ok())
}
