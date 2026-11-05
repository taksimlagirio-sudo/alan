//! Alış ve varış (decide.js:791–873 firstTouch/chasePlan/recvPoint, 1361–1674 rbAt/sekYaris/bekCikar/frenT/
//! kacanAl/akHiz/alHiz/kosYer/kosHiz). Ayrıca topa dokunanın alma gücü: core.js touchCap/ctrlP/vurusOku'nun
//! kimlik (sıra) yerine başvuru alan kopyası (kafa sanal oyuncularla da çağırır; bkz. NOT1.md).
#![allow(non_snake_case)]

use crate::kafa::gecici_dis1::KuralErisim as _;
use super::gecici_dis1::GecTab;
use super::kafa1_baglam::Kafa;
use super::kafa1_js::*;
use super::kafa1_param::{BEK_YON, BODY_E, BODY_R};
use super::kafa1_sut::{sgm, top_yap};
use super::kafa1_tehlike::ox_of;
use crate::alan::Etki;
use crate::core_sabit::{IN, Q};
use crate::mat::{atan2, cos, exp, hypot, jmax, jmin, jround, kis, sin, PI};
use crate::model::Oyuncu;
use crate::oyuncu::Tepki;
use crate::saha::{CY as GCY, H as GH, KR, W as GW};
use crate::temas::{bek_dl, bek_dv, bek_kay, bek_re, kuyu_girer, uc_k, Kap, Kural, TopT};
use crate::top::{hyp, tahmin, Bitis};
use crate::yetenek::{mek, yum, zar, DK_ALIS};
use std::borrow::Cow;

// ---- core.js touchCap / vurusOku / ctrlP (başvuruyla) ----

fn yas_t(b: &TopT) -> f64 {
    b.b.tp.unwrap_or(b.b.t) + b.t.tt
}
/// core.js vurusOku(b, p): `from` vuranın kendisi.
pub fn vurus_oku_al(b: &mut TopT, p: &Oyuncu, from: Option<&Oyuncu>, bek0: bool) -> Option<(f64, f64)> {
    if b.t.kt.is_none() || b.b.defl != 0.0 || b.b.defl_b || b.b.walls != 0.0 || p.takim() == b.b.team || p.bekci() {
        return None;
    }
    let bek = bek0 || b.t.k_bek;
    let a = 100.0 * p.team + p.id;
    let ck = if bek { (-1.0 - a) as i64 } else { a as i64 };
    if let Some(r) = b.t.oku.get(&ck) {
        return Some(*r);
    }
    let k = b.t.kt.unwrap() + 1.618 * b.t.kz;
    let z = if bek { 0.0 } else { zar(a, k, 21.0) + zar(a, k, 23.0) - 1.0 };
    let mut on = 0.0;
    if let Some(f0) = from {
        if b.t.cue > 0.0 {
            let f = kis(0.5 + IN.sez * (p.yet.y(mek::VURUS_SEZ) - 10.0) - IN.giz * (f0.yet.y(mek::VURUS_GIZLE) - 10.0), 0.05, 0.95);
            on = b.t.cue * f * if bek { (IN.u_a + IN.u_b) / 2.0 } else { IN.u_a + (IN.u_b - IN.u_a) * zar(a, k, 29.0) };
        }
    }
    let r = (1.0 + IN.tepki_z * z, on);
    b.t.oku.insert(ck, r);
    Some(r)
}
/// core.js touchCap(b, p, src): p sanal (kopyası) olabilir; `from` topun vuranı.
pub fn kap_al(b: &mut TopT, p: &Oyuncu, from: Option<&Oyuncu>, ps: &[Op], k: &dyn Kural) -> Kap {
    let mate = p.takim() == b.b.team;
    let bek = p.bekci();
    let mut ark_k = 0.0;
    let mut sk = if bek { p.yet.y(mek::TUT_CEL) } else if mate { p.yet.y(mek::KONTROL_KAP) } else { p.yet.y(mek::KESME_KAP) };
    let mut pr = 0.0;
    for o in ps {
        if o.team != p.team {
            pr += o.etki().infl(p.x, p.y);
        }
    }
    {
        let ps0 = if mate && !bek { hyp(z0(p.vx), z0(p.vy)) } else { 0.0 };
        if ps0 > 0.03 {
            let rx = b.b.vx - p.vx;
            let ry = b.b.vy - p.vy;
            let rl = hyp(rx, ry);
            let ak = if rl > 1e-6 { jmax(0.0, (rx * p.vx + ry * p.vy) / (rl * ps0)) * jmin(1.0, ps0 / (p.taban_hiz() * k.spr(p))) } else { 0.0 };
            if ak > 0.0 {
                sk += IN.ark_w * ak * (p.yet.ozk("ceviklik") - sk);
                ark_k = IN.ark_k * ak;
            }
        }
    }
    let o = vurus_oku_al(b, p, from, false);
    let ok = if !bek { p.yet.y(mek::HAZIRLIK) } else { p.yet.ozk("refleks") };
    let tau = jmax(1.0, Q.tepki0 - ok * Q.tepki_ok) * o.map(|x| x.0).unwrap_or(1.0);
    let haz = jmin(1.0, (b.b.tp.unwrap_or(b.b.t) + o.map(|x| x.1).unwrap_or(0.0)) / tau);
    let pr_k = 1.0 - exp(-jmax(0.0, pr - 0.4) * Q.press_k / Q.press_max);
    let ch_t = if b.b.alive { b.b.ch * Q.cap_ch } else { 0.0 };
    let har = jmin(1.0, hypot(z0(p.vx), z0(p.vy)) / (p.taban_hiz() * p.spr0()));
    let cap = if !bek {
        let alis = DK_ALIS.b * yum(&p.yet, "B") * pr_k + DK_ALIS.d * (1.0 - p.den.unwrap_or(1.0)) + if mate { DK_ALIS.h * har * yum(&p.yet, "H") } else { 0.0 };
        Q.cap0 + (sk - 10.0) * Q.cap_sk - ch_t - Q.cap_pr * alis - ark_k
    } else {
        Q.cap0 + (sk - 10.0) * Q.cap_sk - ch_t - Q.cap_pr * pr_k - ark_k
    } * (Q.cap_haz + (1.0 - Q.cap_haz) * haz);
    let vrx = b.b.vx - z0(p.vx);
    let vry = b.b.vy - z0(p.vy);
    Kap {
        cap: jmax(0.08, cap * if !bek || mate { 1.0 } else { Q.bek_el * jmax(0.2, 1.0 + 0.05 * (sk - 10.0)) }),
        vr: hyp(vrx, vry),
        vrx,
        vry,
        s: Q.cap_s0 + (20.0 - sk) * Q.cap_ssk,
        sk,
    }
}
/// core.js ctrlP(b, p, src, undefined, x)
pub fn ctrl_p_al(b: &mut TopT, p: &Oyuncu, from: Option<&Oyuncu>, ps: &[Op], x: Option<f64>, k: &dyn Kural) -> f64 {
    let mut t = kap_al(b, p, from, ps, k);
    if let Some(x) = x {
        if !p.bekci() {
            t.cap *= uc_k(x, Some(p));
        }
    }
    kis(1.0 / (1.0 + exp((t.vr - t.cap) / t.s)), 0.02, 0.99)
}
#[allow(dead_code)]
fn _yas(b: &TopT) -> f64 {
    yas_t(b)
}

/// Rakibin koşacağı yer (JS `A.kj`): hedef, tepki, hız.
#[derive(Clone, Copy, Debug)]
pub struct Kj {
    pub t: Nk,
    pub r: f64,
    pub v: f64,
}

/// Uçuşun bir aday alıcısı (JS ucusFormul'ün `ad` dizisinin elemanı).
#[derive(Clone)]
pub struct UA<'a> {
    /// oyuncu (sanal kopya olabilir)
    pub p: Cow<'a, Oyuncu>,
    /// p dünyanın (src) bir satırıysa sırası
    pub pi: Option<usize>,
    pub G: Option<GecTab>,
    /// JS `A.G0` (undefined: None)
    pub G0: Option<Option<GecTab>>,
    pub v: f64,
    pub rc: f64,
    pub dMax: f64,
    pub hm: f64,
    pub rakip: bool,
    pub yv: bool,
    pub bek: bool,
    pub R0: f64,
    pub rR: f64,
    pub rR0: f64,
    pub rcY: Option<f64>,
    pub dV: f64,
    pub kay: f64,
    pub tau: f64,
    pub ben: bool,
    pub bk0: bool,
    pub gvT: f64,
    // denemeler arası durum
    pub tc: f64,
    pub M: f64,
    pub Gk: f64,
    pub kov: Option<bool>,
    pub kovT: Option<f64>,
    pub kt: Option<Nk>,
    pub ktT: f64,
    pub kj: Option<Kj>,
    pub kj2t: Option<Nk>,
    pub est: f64,
    pub kx: f64,
    pub ky: f64,
    pub c: (f64, f64),
    pub Gy: Option<GecTab>,
    pub Gkk: Option<GecTab>,
    pub ec: f64,
    pub pG: Option<Oyuncu>,
    /// JS `A.pT` (alışa koşan kopyanın hızı)
    pub pT: Option<(f64, f64)>,
    pub ucX: Option<f64>,
    pub gvNow: bool,
}
impl<'a> UA<'a> {
    pub fn yeni(p: Cow<'a, Oyuncu>, pi: Option<usize>) -> UA<'a> {
        UA {
            p,
            pi,
            G: None,
            G0: None,
            v: f64::NAN,
            rc: f64::NAN,
            dMax: f64::NAN,
            hm: 0.0,
            rakip: false,
            yv: false,
            bek: false,
            R0: f64::NAN,
            rR: f64::NAN,
            rR0: f64::NAN,
            rcY: None,
            dV: f64::NAN,
            kay: f64::NAN,
            tau: f64::NAN,
            ben: false,
            bk0: false,
            gvT: 0.0,
            tc: 0.0,
            M: -1e9,
            Gk: 0.0,
            kov: None,
            kovT: None,
            kt: None,
            ktT: 0.0,
            kj: None,
            kj2t: None,
            est: f64::NAN,
            kx: f64::NAN,
            ky: f64::NAN,
            c: (0.0, 0.0),
            Gy: None,
            Gkk: None,
            ec: f64::NAN,
            pG: None,
            pT: None,
            ucX: None,
            gvNow: false,
        }
    }
    /// JS `A.pT || A.p`: hızı değişmiş kopyası
    pub fn pv(&self) -> Cow<'_, Oyuncu> {
        match self.pT {
            Some((vx, vy)) => {
                let mut q = (*self.p).clone();
                q.vx = vx;
                q.vy = vy;
                Cow::Owned(q)
            }
            None => Cow::Borrowed(&*self.p),
        }
    }
}

/// Seken topun yarışının sonucu (JS sekYaris dönüşü: 2 ya da 6 eleman).
#[derive(Clone, Copy, Debug)]
pub struct Sek {
    pub v: [f64; 6],
    pub n: usize,
}
impl Sek {
    fn alti(v: [f64; 6]) -> Sek {
        Sek { v, n: 6 }
    }
}

/// chasePlan'ın kovalayanı (JS {t, x, y, rc}).
#[derive(Clone, Copy, Debug)]
pub struct Kovala {
    pub t: f64,
    pub x: f64,
    pub y: f64,
    pub rc: f64,
}
/// chasePlan'ın `lane` seçeneği.
#[derive(Clone, Copy, Debug, Default)]
pub struct Lane {
    pub lag: f64,
    pub loose: bool,
}

/// Frenle varış (saf): p'nin (x, y)'ye R yakınlığa T tikte varamadığı ek süre (yetenek ve kuraldan).
pub fn frenT_p(p: &Oyuncu, x: f64, y: f64, R: f64, T: f64, v: f64) -> f64 {
    let vx = z0(p.vx);
    let vy = z0(p.vy);
    let s0 = hypot(vx, vy);
    if s0 < 0.1 || !(T > 0.0) {
        return 0.0;
    }
    let ux = vx / s0;
    let uy = vy / s0;
    let dx = x - p.x;
    let dy = y - p.y;
    let xa = dx * ux + dy * uy;
    let xl = (-dx * uy + dy * ux).abs();
    let ag = crate::yetenek::ag(&p.yet, p.den, p.bekci(), crate::kural::ikili::IK.yavas);
    let aB = 0.018 * ag;
    let aL = 0.021 * ag;
    let tS = s0 / aB;
    let aMin = if T <= tS { s0 * T - aB * T * T / 2.0 } else { s0 * tS / 2.0 - jmin(v * (T - tS), 0.025 * (T - tS) * (T - tS)) };
    let aMax = s0 * T + jmin((v - s0) * T, 0.18 * v * 0.5 * crate::yetenek::ah(&p.yet) * T * T);
    let rA = jmax(jmax(0.0, aMin - xa), xa - aMax);
    let rL = jmax(0.0, xl - jmin(v * T, aL * T * T / 2.0));
    let def = hypot(rA, rL) - R;
    if def > 0.0 {
        T + def / v
    } else {
        0.0
    }
}

impl Kafa {
    /// Bekçi kendi alanının dışına çıkar mı (decide.js:1603).
    pub fn bekCikar(&self, bk: &Oyuncu, X: f64, Y: f64, tB: f64, tm: f64) -> bool {
        let S = self.dis.S();
        let bekOnce = S.as_ref().and_then(|s| s.bekOnce).unwrap_or(2.0);
        let marginT = S.as_ref().map_or(0.6, |s| s.marginT);
        hypot(X - ox_of(bk.team), Y - GCY) <= KR || tB + bekOnce + (20.0 - bk.yet.y(mek::CIKIS)) * marginT < tm
    }
    /// Kale önündeki sekmede topun bizde kalma payı (decide.js:1363).
    pub fn rbAt(&self, src: &[Op], team: f64, P: Nk) -> f64 {
        let gxk = if team == 0.0 { GW } else { 0.0 };
        if hypot(P.x - gxk, P.y - GCY) >= 16.0 {
            return 0.4;
        }
        let mut A = 0.3;
        let mut Dn = 0.3;
        let cx = gxk - if team == 0.0 { 7.0 } else { -7.0 };
        for p in src {
            let d = hypot(p.x - cx, p.y - GCY);
            if d < 12.0 {
                if p.team == team {
                    A += 1.0;
                } else {
                    Dn += 1.0;
                }
            }
        }
        A / (A + Dn)
    }
    /// Koşan oyuncunun durup (x, y)'ye dönmesinin gecikmesi (decide.js:1604).
    pub fn frenT(&self, p: &Oyuncu, x: f64, y: f64, R: f64, T: f64, v: f64) -> f64 {
        frenT_p(p, x, y, R, T, v)
    }
    /// JS ucusFormul'ün `arr(p, x, y, G)`: p'nin (x, y)'ye varış süresi.
    pub fn arr(&self, p: &Oyuncu, x: f64, y: f64, G: Option<&GecTab>) -> f64 {
        let d = hypot(p.x - x, p.y - y);
        (match G {
            Some(G) => self.dis.gecOf(G, x, y, Some(d)),
            None => self.dis.gec(p, x, y, Some(d)).unwrap_or(0.0),
        }) + d / self.vS(p)
    }
    /// Kaçan topu arkadaşın alma payı (decide.js:1610).
    pub fn kacanAl(&self, p: &Oyuncu, ad: &[UA], P: Nk, bvx: f64, bvy: f64, Tb: f64) -> f64 {
        let sp = hypot(bvx, bvy);
        let cdT = jmax(Q.cd_min, jround((Q.cd - 0.5 * (p.yet.y(mek::YENIDEN_DOKUN) - 10.0)) * jmin(1.0, sp / Q.cd_v)));
        let mut tO: f64 = 1e9;
        for A in ad {
            if A.rakip && !A.bek {
                tO = jmin(tO, A.rcY.unwrap_or(A.rc) + self.arr(&A.p, P.x, P.y, A.G.as_ref()));
            }
        }
        1.0 / (1.0 + exp(-(tO - Tb - cdT - 1.0) / 3.0))
    }
    /// Rakip topa yandan koşar (decide.js:1616): yeni hız (None: değişmez).
    pub fn akHiz(&self, p: &Oyuncu, A: &UA, bvx: f64, bvy: f64, cx: f64, cy: f64, vA: f64) -> Option<(f64, f64)> {
        let bl = hypot(bvx, bvy);
        if bl < 1e-6 {
            return None;
        }
        let ux = bvx / bl;
        let uy = bvy / bl;
        let dx = cx - p.x;
        let dy = cy - p.y;
        let da = dx * ux + dy * uy;
        let px = dx - da * ux;
        let py = dy - da * uy;
        let pl = hypot(px, py);
        if pl < 1e-6 {
            return None;
        }
        let vn = jmin(A.v, vA);
        Some((px / pl * vn, py / pl * vn))
    }
    /// Arkadaş alışa koşar (decide.js:1626): yeni hız.
    #[allow(clippy::too_many_arguments)]
    pub fn alHiz(&self, p: &Oyuncu, A: &UA, dd: f64, R: f64, cx: f64, cy: f64, bvx: f64, bvy: f64, Tb: f64, vS: f64) -> (f64, f64) {
        let mut vx = 0.0;
        let mut vy = 0.0;
        let bs = hypot(bvx, bvy);
        if dd > R && bs > 1e-6 {
            let mx = cx - p.x;
            let my = cy - p.y;
            let ml = ya(hypot(mx, my), 1.0);
            let vn = jmin(A.v, (dd - R) / jmax(1.0, Tb - A.rc) * 1.25 + 0.02);
            let ux = bvx / bs;
            let uy = bvy / bs;
            vx = mx / ml * vn;
            vy = my / ml * vn;
            let vt = -(vx * ux + vy * uy);
            if vt > vS {
                vx += (vt - vS) * ux;
                vy += (vt - vS) * uy;
            }
        }
        (vx, vy)
    }
    /// J'ye koşanın T tik sonraki yeri (decide.js:1644).
    pub fn kosYer(&self, p: &Oyuncu, J: &Kj, T: f64) -> (f64, f64) {
        let x0 = p.x;
        let y0 = p.y;
        let vx = z0(p.vx);
        let vy = z0(p.vy);
        let r = jmax(0.0, jmin(T, J.r));
        let ax = x0 + vx * r;
        let ay = y0 + vy * r;
        let t = T - r;
        if t <= 0.0 {
            return (ax, ay);
        }
        let ex = J.t.x - ax;
        let ey = J.t.y - ay;
        let E = hypot(ex, ey);
        if E < 1e-6 {
            return (ax, ay);
        }
        let dvx = ex / E * J.v - vx;
        let dvy = ey / E * J.v - vy;
        let dv = hypot(dvx, dvy);
        let k = 0.021 * self.AG(p);
        let tau = dv / k;
        let (qx, qy);
        if dv < 1e-9 {
            qx = ax + vx * t;
            qy = ay + vy * t;
        } else if t <= tau {
            qx = ax + vx * t + dvx / dv * k * t * t / 2.0;
            qy = ay + vy * t + dvy / dv * k * t * t / 2.0;
        } else {
            qx = ax + vx * tau + dvx / dv * k * tau * tau / 2.0 + ex / E * J.v * (t - tau);
            qy = ay + vy * tau + dvy / dv * k * tau * tau / 2.0 + ey / E * J.v * (t - tau);
        }
        let ox = qx - ax;
        let oy = qy - ay;
        let along = (ox * ex + oy * ey) / E;
        if along > E {
            (J.t.x, J.t.y)
        } else {
            (qx, qy)
        }
    }
    /// J'ye koşanın T tik sonraki hızı (decide.js:1665).
    pub fn kosHiz(&self, p: &Oyuncu, J: &Kj, T: f64) -> (f64, f64) {
        let vx = z0(p.vx);
        let vy = z0(p.vy);
        let r = jmax(0.0, jmin(T, J.r));
        let t = T - r;
        if t <= 0.0 {
            return (vx, vy);
        }
        let ax = p.x + vx * r;
        let ay = p.y + vy * r;
        let ex = J.t.x - ax;
        let ey = J.t.y - ay;
        let E = hypot(ex, ey);
        if E < 1e-6 {
            return (0.0, 0.0);
        }
        let dvx = ex / E * J.v - vx;
        let dvy = ey / E * J.v - vy;
        let dv = hypot(dvx, dvy);
        let k = 0.021 * self.AG(p);
        let tau = dv / k;
        if dv < 1e-9 {
            return (vx, vy);
        }
        let (qx, qy) = self.kosYer(p, J, T);
        if hypot(qx - J.t.x, qy - J.t.y) < 1e-6 {
            (0.0, 0.0)
        } else if t <= tau {
            (vx + dvx / dv * k * t, vy + dvy / dv * k * t)
        } else {
            (ex / E * J.v, ey / E * J.v)
        }
    }

    /// Topa değen oyuncudan seken topun yarışı (decide.js:1376): [biz, kuyu, bizim yer x, y, rakip yer x, y].
    #[allow(clippy::too_many_arguments)]
    pub fn sekYaris(&self, src: &[Op], team: f64, A: &UA, bL: &mut TopT, from: Option<&Oyuncu>, cx: f64, cy: f64, Tb: f64, e0: Option<f64>, hK: Option<usize>, gecik: f64, sizar: bool, etP: &[Etki], kc: Kc) -> Sek {
        let p: &Oyuncu = &A.p;
        let pv = A.pv();
        let kr = self.kural();
        let mut T = kap_al(bL, &pv, from, src, &kr);
        if let Some(x) = A.ucX {
            if !A.bek {
                T.cap *= uc_k(x, Some(p));
            }
        }
        let left = jmax(0.0, T.vr - jmax(0.0, T.cap));
        let (bvx, bvy) = (bL.b.vx, bL.b.vy);
        let sp = ya(hypot(bvx, bvy), 1e-6);
        let bkP = A.bek;
        let GV = A.gvNow;
        let pd = hypot(p.x - cx, p.y - cy);
        let TY = !GV && !A.bek && pd > Q.reach;
        let mut ex = if pd > 1e-9 { (p.x - cx) / pd } else { 0.0 };
        let mut ey = if pd > 1e-9 { (p.y - cy) / pd } else { 0.0 };
        if TY {
            let ux = bvx / sp;
            let uy = bvy / sp;
            let da = (p.x - cx) * ux + (p.y - cy) * uy;
            let qx = p.x - cx - da * ux;
            let qy = p.y - cy - da * uy;
            let ql = hypot(qx, qy);
            if ql > 0.3 {
                ex = qx / ql;
                ey = qy / ql;
            }
        }
        let px = if TY { cx + ex * Q.reach } else { p.x };
        let py = if TY { cy + ey * Q.reach } else { p.y };
        let mut qx = cx - px;
        let mut qy = cy - py;
        let mut ql = hypot(qx, qy);
        if ql < 0.001 {
            qx = -bvx;
            qy = -bvy;
            ql = sp;
        }
        qx /= ql;
        qy /= ql;
        let RV = T.vr > 1e-6;
        let ux = if RV { T.vrx / T.vr } else { bvx / sp };
        let uy = if RV { T.vry / T.vr } else { bvy / sp };
        let dn = ux * qx + uy * qy;
        let mut nx = ux;
        let mut ny = uy;
        if dn < 0.0 && p.takim() != bL.b.team {
            nx = ux - (1.0 + Q.rest2) * dn * qx;
            ny = uy - (1.0 + Q.rest2) * dn * qy;
        }
        let (ix, iy);
        let mut v = left;
        if bkP {
            let out = if ox_of(p.team) == 0.0 { 1.0 } else { -1.0 };
            v = jmax(left, 0.8 + 0.04 * (T.sk - 10.0));
            ix = out;
            iy = 0.0;
        } else {
            let gx = if p.team == 0.0 { GW } else { 0.0 };
            let il = ya(hypot(gx - p.x, GCY - p.y), 1.0);
            ix = (gx - p.x) / il;
            iy = (GCY - p.y) / il;
        }
        if GV {
            let rx = bvx - z0(pv.vx);
            let ry = bvy - z0(pv.vy);
            let vn = rx * qx + ry * qy;
            let mut ox = bvx;
            let mut oy = bvy;
            if vn < 0.0 {
                ox -= (1.0 + BODY_E) * vn * qx;
                oy -= (1.0 + BODY_E) * vn * qy;
            }
            v = hypot(ox, oy);
            nx = ox / ya(v, 1.0);
            ny = oy / ya(v, 1.0);
        }
        let (nx0, ny0) = (nx, ny);
        let _S = self.dis.S();
        let cdT = jmax(Q.cd_min, jround(Q.cd * jmin(1.0, sp / Q.cd_v)));
        let tv: Option<Oyuncu> = match A.pT {
            Some((vx, vy)) if !GV => {
                let mut o = (*A.p).clone();
                o.vx = vx;
                o.vy = vy;
                o.x = px + vx * cdT;
                o.y = py + vy * cdT;
                Some(o)
            }
            _ => None,
        };
        let tvG = tv.as_ref().and_then(|o| self.dis.gecTab(o, true));
        let bch = bL.b.ch;
        let bteam = bL.b.team as f64;
        let ayniP = |o: &Oyuncu, oi: usize| A.pi == Some(oi) && std::ptr::eq(o, p);
        struct H {
            siz: bool,
            nx: f64,
            ny: f64,
            v: Option<f64>,
            x: f64,
            y: f64,
        }
        let yaris = |ix: f64, iy: f64, Hh: Option<H>| -> [f64; 6] {
            let mut nx = Hh.as_ref().map_or(nx0, |h| h.nx);
            let mut ny = Hh.as_ref().map_or(ny0, |h| h.ny);
            let siz = Hh.as_ref().map_or(false, |h| h.siz);
            let st = if GV || siz { 0.0 } else { jmin(1.0, Q.steer * (if bkP { Q.bek_steer } else { jmax(0.0, jmin(1.0, T.cap / ya(T.vr, 1.0))) }) * (0.5 + 0.04 * (T.sk - 10.0))) };
            nx = nx * (1.0 - st) + ix * st;
            ny = ny * (1.0 - st) + iy * st;
            let nl = ya(hypot(nx, ny), 1.0);
            nx /= nl;
            ny /= nl;
            let bx = jmax(1.0, jmin(GW - 1.0, match &Hh { Some(h) => h.x, None => if GV { p.x + qx * BODY_R } else { px + qx * (Q.reach + 0.05) } }));
            let by = jmax(1.0, jmin(GH - 1.0, match &Hh { Some(h) => h.y, None => if GV { p.y + qy * BODY_R } else { py + qy * (Q.reach + 0.05) } }));
            let vv = Hh.as_ref().and_then(|h| h.v).unwrap_or(v);
            let b0x = if GV || siz { 0.0 } else { z0(pv.vx) };
            let b0y = if GV || siz { 0.0 } else { z0(pv.vy) };
            let mut b = top_yap(bx, by, b0x + nx * vv, b0y + ny * vv, bteam, Some(bch), None);
            if let Some(e0) = e0 {
                b.e = jmax(0.0, e0 - if GV { 0.0 } else { Q.defl_e });
                b.alive = b.e > 0.0;
            }
            b.defl = 1.0;
            b.defl_b = false;
            let (pts, end) = kaynakla(etP, kc, |k| tahmin(&b, k, &crate::saha::P, Some(100)));
            let n = pts.len();
            let (mut tm, mut tO, mut tB): (f64, f64, f64) = (1e9, 1e9, 1e9);
            let mut bk: Option<&Oyuncu> = None;
            let (mut bX, mut bY) = (0.0, 0.0);
            let (mut mX, mut mY, mut oX, mut oY): (Option<f64>, Option<f64>, Option<f64>, Option<f64>) = (None, None, None, None);
            let kos = |o: &Oyuncu, oi: usize, Bo: f64| -> (f64, f64, f64) {
                let op = ayniP(o, oi);
                let t0 = jmax(Tb + if op { if GV { jmax(0.0, A.gvT - Tb) } else { cdT } } else { jmax(0.0, self.TEPKI(o, Tepki::BosTop)) }, if hK == Some(oi) { z0(gecik) + 8.0 } else { 0.0 });
                if t0 >= Bo {
                    return (1e9, 0.0, 0.0);
                }
                let v = self.vS(o);
                let Gt: Option<GecTab> = if tv.is_some() && op { tvG.clone() } else { self.dis.gecTab(o, false) };
                let BD = o.bekci() && !op;
                let gxo = ox_of(o.team);
                let reo = if BD { bek_re(o) } else { 0.0 };
                let DLo = if BD { bek_dl(o) } else { 0.0 };
                let DVo = if BD { bek_dv(o) } else { 0.0 };
                let KAo = if BD { v * bek_kay(o) } else { 0.0 };
                let adim = if BD { 1 } else { 3 };
                let mut i = 0;
                while i < n {
                    let fi = i as f64;
                    if Tb + fi >= Bo {
                        return (1e9, 0.0, 0.0);
                    }
                    let P = pts[i];
                    if !o.bekci() && !kuyu_girer(o, P.x, P.y, Tb + fi, false, &kr) {
                        i += adim;
                        continue;
                    }
                    if BD && hypot(P.x - gxo, GCY - P.y) <= KR {
                        let Rd = Q.bek_body + jmin(DLo, jmax(0.0, fi - reo) * DVo);
                        let ta = Tb + jmax(0.0, reo) + jmax(0.0, hypot(o.x - P.x, o.y - P.y) - Rd) / KAo;
                        if ta <= Tb + fi || i + 3 >= n {
                            return (jmax(ta, Tb + fi), P.x, P.y);
                        }
                        i += adim;
                        continue;
                    }
                    if BD && i % 3 != 0 {
                        i += adim;
                        continue;
                    }
                    let (oX0, oY0) = if op {
                        match &tv {
                            Some(t) => (t.x, t.y),
                            None => (px, py),
                        }
                    } else {
                        (o.x, o.y)
                    };
                    let kX0 = jmax(0.0, hypot(oX0 - P.x, oY0 - P.y) - Q.reach) / v;
                    if i + 3 < n && t0 + kX0 > Tb + fi {
                        i += adim;
                        continue;
                    }
                    let ta = t0 + match &Gt { Some(G) => self.dis.gecOf(G, P.x, P.y, Some(hypot(oX0 - P.x, oY0 - P.y) - Q.reach)), None => 0.0 } + kX0;
                    if ta <= Tb + fi || i + 3 >= n {
                        return (jmax(ta, Tb + fi), P.x, P.y);
                    }
                    i += adim;
                }
                (1e9, 0.0, 0.0)
            };
            let mut bkL: Vec<usize> = Vec::new();
            for (oi, o) in src.iter().enumerate() {
                if o.noTouch {
                    continue;
                }
                if o.team == team {
                    let (tk, kX, kY) = kos(o, oi, tm);
                    if tk < tm {
                        tm = tk;
                        mX = Some(kX);
                        mY = Some(kY);
                    }
                } else if o.bekci() {
                    bkL.push(oi);
                } else {
                    let (tk, kX, kY) = kos(o, oi, tO);
                    if tk < tO {
                        tO = tk;
                        oX = Some(kX);
                        oY = Some(kY);
                    }
                }
            }
            for &oi in &bkL {
                let (tk, kX, kY) = kos(&src[oi], oi, tO);
                tB = tk;
                bk = Some(&src[oi]);
                bX = kX;
                bY = kY;
            }
            if let Some(bk) = bk {
                if self.bekCikar(bk, bX, bY, tB, tm) && tB < tO {
                    tO = tB;
                    oX = Some(bX);
                    oY = Some(bY);
                }
            }
            let mut pK = 0.0;
            if let Some(d) = end.done {
                if d == Bitis::KuyuSag || d == Bitis::KuyuSol {
                    let tE = Tb + n as f64 - 1.0;
                    let kz = sgm((jmin(tm, tO) - tE) / 4.0);
                    pK = if (d == Bitis::KuyuSag) == (team == 0.0) { kz } else { 0.0 };
                    if pK == 0.0 {
                        tO = jmin(tO, tE);
                    }
                }
            }
            let Pe = pts[n - 1];
            [(1.0 - pK) * sgm((tO - tm) / 4.0), pK, mX.unwrap_or(Pe.x), mY.unwrap_or(Pe.y), oX.unwrap_or(Pe.x), oY.unwrap_or(Pe.y)]
        };
        if sizar {
            let a0 = atan2(uy, ux);
            let (mut r0, mut r1, mut mx, mut my, mut ox, mut oy, mut w0, mut w2) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
            for da in [-0.2, -0.1, 0.0, 0.1, 0.2] {
                let vx = cos(a0 + da);
                let vy = sin(a0 + da);
                let R = yaris(vx, vy, Some(H { siz: true, nx: vx, ny: vy, v: Some(sp * Q.thru_keep), x: cx + 0.05 * vx, y: cy + 0.05 * vy }));
                let ro = jmax(0.0, 1.0 - R[0] - R[1]);
                r0 += R[0] / 5.0;
                r1 += R[1] / 5.0;
                mx += R[0] * R[2];
                my += R[0] * R[3];
                w0 += R[0];
                ox += ro * R[4];
                oy += ro * R[5];
                w2 += ro;
            }
            return Sek::alti([r0, r1, if w0 > 1e-9 { mx / w0 } else { cx }, if w0 > 1e-9 { my / w0 } else { cy }, if w2 > 1e-9 { ox / w2 } else { cx }, if w2 > 1e-9 { oy / w2 } else { cy }]);
        }
        if bkP {
            let out = if p.team == 0.0 { 1.0 } else { -1.0 };
            let own = ox_of(p.team);
            let Lq = jmax(3.0, jmin(14.0, 15.0 * v));
            let (mut a0, mut a1, mut sw, mut mx, mut my, mut ox, mut oy, mut mw, mut ow) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
            for a in BEK_YON {
                let r = a * PI / 180.0;
                let dx = cos(r) * out;
                let dy = sin(r);
                let ppx = jmax(1.0, jmin(GW - 1.0, p.x + dx * Lq));
                let ppy = jmax(1.0, jmin(GH - 1.0, p.y + dy * Lq));
                let mut od: f64 = 12.0;
                for o in src {
                    if !(o.team == p.team || o.noTouch) {
                        od = jmin(od, hypot(o.x - ppx, o.y - ppy));
                    }
                }
                let wa = exp((od - if (ppx - own).abs() < 22.0 && (ppy - GCY).abs() < 12.0 { 6.0 } else { 0.0 }) / 2.0);
                let R = yaris(dx, dy, None);
                sw += wa;
                a0 += wa * R[0];
                a1 += wa * R[1];
                mx += wa * R[0] * R[2];
                my += wa * R[0] * R[3];
                mw += wa * R[0];
                let oq = wa * jmax(0.0, 1.0 - R[0] - R[1]);
                ox += oq * R[4];
                oy += oq * R[5];
                ow += oq;
            }
            return Sek::alti([a0 / sw, a1 / sw, if mw > 1e-12 { mx / mw } else { p.x }, if mw > 1e-12 { my / mw } else { p.y }, if ow > 1e-12 { ox / ow } else { p.x }, if ow > 1e-12 { oy / ow } else { p.y }]);
        }
        if !GV && pd < BODY_R + 0.5 {
            let R1 = yaris(ix, iy, None);
            let R2 = yaris(ix, iy, Some(H { siz: false, nx: -ux, ny: -uy, v: None, x: cx - 0.05 * ux, y: cy - 0.05 * uy }));
            let w1 = R1[0] + R2[0];
            let w2 = 1.0 - R1[0] - R1[1] + (1.0 - R2[0] - R2[1]);
            return Sek::alti([
                (R1[0] + R2[0]) / 2.0,
                (R1[1] + R2[1]) / 2.0,
                if w1 > 1e-9 { (R1[0] * R1[2] + R2[0] * R2[2]) / w1 } else { R1[2] },
                if w1 > 1e-9 { (R1[0] * R1[3] + R2[0] * R2[3]) / w1 } else { R1[3] },
                if w2 > 1e-9 { ((1.0 - R1[0] - R1[1]) * R1[4] + (1.0 - R2[0] - R2[1]) * R2[4]) / w2 } else { R1[4] },
                if w2 > 1e-9 { ((1.0 - R1[0] - R1[1]) * R1[5] + (1.0 - R2[0] - R2[1]) * R2[5]) / w2 } else { R1[5] },
            ]);
        }
        Sek::alti(yaris(ix, iy, None))
    }

    /// Topun yolunda kovalayanlar (decide.js:808): src sırasıyla (oyuncu sırası, hedef), takımca en erken
    /// varanlar ve yetişenler.
    pub fn chasePlan(&self, pts: &[Nk], ps: &[Op], t0: f64, lane: Option<Lane>) -> Vec<(usize, Kovala)> {
        let bekSerbest = self.dis.S().map_or(false, |s| s.bekSerbest);
        let mut arr: Vec<(usize, Kovala, bool)> = Vec::new();
        for (pi, p) in ps.iter().enumerate() {
            if p.bekci() && !bekSerbest && hypot(p.x - ox_of(p.team), p.y - GCY) > 12.0 {
                continue;
            }
            let rc = if lane.map_or(false, |l| l.loose) {
                jmax(0.0, self.TEPKI(p, Tepki::BosTop))
            } else {
                jmax(0.0, self.TEPKI(p, Tepki::Kovala) - z0(t0) + lane.map_or(0.0, |l| l.lag)) - self.D._savT.map_or(0.0, z0)
            };
            let v = self.vS(p);
            let (px, py) = (p.x, p.y);
            let mut g: Option<Kovala> = None;
            let mut i = jmax(0.0, 2.0 * (rc / 2.0).ceil()) as usize;
            while i < pts.len() {
                let q = pts[i];
                let ax = (q.x - px).abs();
                let ay = (q.y - py).abs();
                if rc + (0.999999 * (if ax > ay { ax } else { ay }) - 1.4) / v > i as f64 + 1e-6 {
                    i += 2;
                    continue;
                }
                let tr = rc + jmax(0.0, hypot(q.x - px, q.y - py) - 1.4) / v;
                if tr <= i as f64 {
                    g = Some(Kovala { t: i as f64, x: q.x, y: q.y, rc });
                    break;
                }
                i += 2;
            }
            let g = match g {
                Some(g) => g,
                None => {
                    let q = match pts.last() {
                        Some(q) => *q,
                        None => continue,
                    };
                    Kovala { t: jmax(pts.len() as f64, rc + hypot(q.x - p.x, q.y - p.y) / v), x: q.x, y: q.y, rc }
                }
            };
            arr.push((pi, g, g.t < pts.len() as f64));
        }
        let mut out = Vec::new();
        for tm in [0.0, 1.0] {
            let mine: Vec<&(usize, Kovala, bool)> = arr.iter().filter(|a| ps[a.0].team == tm).collect();
            if mine.is_empty() {
                continue;
            }
            let best = mine.iter().fold(f64::INFINITY, |m, a| jmin(m, a.1.t));
            for &&(pi, g, hit) in &mine {
                let p = &ps[pi];
                if g.t <= best + 0.6 * (20.0 - p.yet.y(mek::ALGI_HATA)) + 1.0 || (lane.is_some() && hit) {
                    out.push((pi, g));
                }
            }
        }
        out
    }
    /// Alıcının topu karşılayacağı yer (decide.js:849).
    pub fn recvPoint(&self, pts: &[Nk], p: &Oyuncu) -> Nk {
        if let Some(r) = self.dis.aliciDirenc(pts, p, 0.0, 0.0, 1.4) {
            return r;
        }
        let v = self.vS(p);
        let mut i = 0;
        while i < pts.len() {
            let q = pts[i];
            if jmax(0.0, hypot(q.x - p.x, q.y - p.y) - 1.4) / v <= i as f64 {
                return q;
            }
            i += 2;
        }
        match pts.last() {
            Some(q) => *q,
            None => nk(p.x, p.y),
        }
    }
}
