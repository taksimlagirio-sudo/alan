//! Savurma: vuruştan önce Çekirdeği gövdenin etrafında döndürme (decide.js:561–734 govdeV/vurE/sutV/
//! savurPlan/onPlan/rakipKovala/savurYol/savurEngel/caOf; 2496–2497 vmaxOf/sutGuc).
#![allow(non_snake_case)]

use super::kafa1_baglam::Kafa;
use super::kafa1_js::*;
use super::kafa1_param::SAV;
use crate::core_sabit::{Q, TQ};
use crate::mat::{acos, atan2, cos, hypot, jmax, jmin, sin, PI};
use crate::model::Oyuncu;
use crate::top::chg_mul;
use crate::yetenek::mek;

/// savurPlan'ın dönüşü.
#[derive(Clone, Copy, Debug, Default)]
pub struct SavPlan {
    pub s: f64,
    pub caR: f64,
    pub A: f64,
    pub P: f64,
    pub T: f64,
    pub wR: f64,
    pub w_rel: f64,
    pub u_sh: f64,
    pub a: f64,
    pub wMax: f64,
    pub k: f64,
    pub w0: f64,
}
/// onPlan'ın dönüşü.
#[derive(Clone, Copy, Debug, Default)]
pub struct OnPlan {
    pub s: f64,
    pub wS: f64,
    pub phi: f64,
    pub caR: f64,
    pub kal: f64,
    pub a: f64,
    pub Tsw: f64,
}
/// rakipKovala'nın kovalayanı (JS {q, f, d, v, x?, y?}).
#[derive(Clone, Copy, Debug, Default)]
pub struct Kov {
    pub f: f64,
    pub d: f64,
    pub v: f64,
    pub x: Option<f64>,
    pub y: Option<f64>,
}

const TP: f64 = 2.0 * PI;
#[inline(always)]
fn md(x: f64) -> f64 {
    (x % TP + TP) % TP
}

/// Rakip Çekirdeğe koşar (decide.js:659): göreli konumunu günceller.
pub fn rakip_kovala(o: &mut Kov, bx: f64, by: f64, MD: f64) -> (f64, f64) {
    if o.x.is_none() {
        o.x = Some(cos(o.f) * o.d);
        o.y = Some(sin(o.f) * o.d);
    }
    let (mut ox, mut oy) = (o.x.unwrap(), o.y.unwrap());
    let tx = bx - ox;
    let ty = by - oy;
    let tl = hypot(tx, ty);
    if tl > 1e-6 {
        let st = jmin(o.v, tl);
        ox += tx / tl * st;
        oy += ty / tl * st;
    }
    let dq = hypot(ox, oy);
    if dq < MD {
        let sc = MD / ya(dq, 1e-6);
        ox *= sc;
        oy *= sc;
    }
    o.x = Some(ox);
    o.y = Some(oy);
    (ox, oy)
}

impl Kafa {
    /// Gövdenin verebileceği en büyük vuruş hızı (decide.js:561).
    pub fn govde_v(&self, h: &Oyuncu) -> f64 {
        crate::kural::sut::govde_v(h)
    }
    /// Vuruş enerjisi katı (decide.js:564).
    pub fn vur_e(&self, h: &Oyuncu, ch: Option<f64>, gon: bool) -> f64 {
        match ch {
            Some(ch) => chg_mul(ch) * if gon { self.sut_guc(h) } else { 1.0 },
            None => 1.0,
        }
    }
    pub fn sut_v(&self, h: &Oyuncu) -> f64 {
        self.govde_v(h)
    }
    /// decide.js:2496
    pub fn vmax_of(&self, h: Option<&Oyuncu>) -> f64 {
        self.D.vmax
            * match h {
                Some(h) if !(h.id < 0.0) => 0.8 + 0.02 * h.govde.yet.y(mek::PAS_SERT),
                _ => 1.0,
            }
    }
    /// decide.js:2497
    pub fn sut_guc(&self, h: &Oyuncu) -> f64 {
        crate::kural::sut::sut_guc(h)
    }
    /// Çekirdeğin açısı (decide.js:730).
    pub fn ca_of(&self, h: &Oyuncu) -> f64 {
        if let Some(k) = &self.bag._kanca {
            if let Some(ca) = k.ca {
                return ca;
            }
        }
        let cs = self.core_side(h);
        atan2(cs.uy, cs.ux)
    }

    /// Vuruş hızına ulaşmak için Çekirdeği döndürme planı (decide.js:565).
    #[allow(clippy::too_many_arguments)]
    pub fn savur_plan(&self, h: &Oyuncu, V: Nk, ca0: f64, w0: f64, sYon: f64, src: Option<&[Op]>, itme: bool, E: f64) -> Option<SavPlan> {
        let B = Q.body;
        let v = ya(hypot(V.x, V.y), 1e-9) / ya(E, 1.0);
        let G = h.govde.yet.ozk("guc");
        let a = jmax(0.08, SAV.ia0 + (G - 10.0) * SAV.ia_sk);
        let wMax = SAV.w_max0 + (G - 10.0) * SAV.w_max_sk;
        let u_max = jmax(0.3, SAV.u0 + (G - 10.0) * SAV.u_sk);
        let y_i = itme;
        let v_cap = if y_i { u_max } else { hypot(wMax * B, u_max) };
        let mut k = 1.0;
        let mut vv = v;
        if v > v_cap {
            k = v_cap / v;
            vv = v_cap;
        }
        let w_lo = jmax(0.0, vv * vv - u_max * u_max).sqrt() / B;
        let w_hi = jmin(vv / B, wMax);
        let th = atan2(V.y, V.x);
        let mut best: Option<SavPlan> = None;
        let w0 = jmin(z0(w0), wMax);
        let hedef_w: Vec<f64> = if y_i {
            vec![0.0]
        } else if u_max <= 0.0 {
            vec![w_hi]
        } else if w_lo > 0.0 {
            vec![w_lo, (w_lo + w_hi) / 2.0, w_hi]
        } else {
            vec![0.0, w_hi / 2.0, w_hi]
        };
        let geri_ok = !(w0 > 0.0);
        let yonler: Vec<f64> = if dogru0(sYon) { vec![sYon] } else { vec![1.0, -1.0] };
        for &s in &yonler {
            for &wS in &hedef_w {
                let bet = atan2(wS * B, jmax(0.0, vv * vv - wS * wS * B * B).sqrt());
                let caR = th - s * bet;
                let Pg = md(s * (ca0 - (caR - s * (wS * wS / (2.0 * a) + 0.5 * wS))));
                let mut Ps = vec![0.0];
                if geri_ok && Pg > 0.05 && Pg <= 3.0 * PI / 4.0 {
                    Ps.push(Pg);
                    Ps.push(jmin(3.0 * PI / 4.0, Pg + 0.25));
                }
                for &P in &Ps {
                    let ca_g = ca0 - s * P;
                    let mut t_g = 0.0;
                    if P > 0.0 {
                        if self.savur_engel(h, src, ca0, -s, P) {
                            continue;
                        }
                        let mut w: f64 = 0.0;
                        let mut rem = P;
                        while rem > 1e-9 && t_g < 300.0 {
                            w = jmin(jmin(w + a, wMax), (2.0 * a * rem).sqrt());
                            rem -= jmin(w, rem);
                            t_g += 1.0;
                        }
                    }
                    let mut A = md(s * (caR - ca_g));
                    for tur in 0..4 {
                        let w_b = if P > 0.0 { 0.0 } else { w0 };
                        let mut w = w_b;
                        let mut rem = A;
                        let mut T = 0.0;
                        while rem > 1e-9 && T < 300.0 {
                            w = jmin(jmin(w + a, wMax), (wS * wS + 2.0 * a * rem).sqrt());
                            rem -= jmin(w, rem);
                            T += 1.0;
                        }
                        let w_e = if T != 0.0 { jmin(w, jmax(wS, 1e-9)) } else { w_b };
                        if w_e >= 0.97 * w_lo || tur == 3 {
                            let u_sh = jmax(0.0, vv * vv - w_e * w_e * B * B).sqrt() / vv;
                            let TT = t_g + T;
                            let iyi = match &best {
                                None => true,
                                Some(b) => TT < b.T || (TT == b.T && (u_sh > b.u_sh + 1e-9 || (u_sh > b.u_sh - 1e-9 && P + A < b.P + b.A))),
                            };
                            if !self.savur_engel(h, src, ca_g, s, A) && iyi && !self.savur_yol(h, src, ca0, s, P, A, a, wMax, wS, if P > 0.0 { 0.0 } else { w0 }) {
                                best = Some(SavPlan { s, caR, A, P, T: TT, wR: wS, w_rel: w_e, u_sh, a, wMax, k, w0 });
                            }
                            break;
                        }
                        A += TP;
                    }
                }
            }
        }
        best
    }

    /// Topu önüne almak için kısa dönüş (decide.js:628).
    pub fn on_plan(&self, h: &Oyuncu, V: Nk, b: Nk, itme: bool, E: f64) -> Option<OnPlan> {
        let B = Q.body;
        let v = ya(hypot(V.x, V.y), 1e-9) / ya(E, 1.0);
        let G = h.govde.yet.ozk("guc");
        let a = jmax(0.08, SAV.ia0 + (G - 10.0) * SAV.ia_sk);
        let wMax = SAV.w_max0 + (G - 10.0) * SAV.w_max_sk;
        let u_max = jmax(0.3, SAV.u0 + (G - 10.0) * SAV.u_sk);
        let y_i = itme;
        let v_cap = if y_i { u_max } else { hypot(wMax * B, u_max) };
        let vv = jmin(v, v_cap);
        let w_lo = jmax(0.0, vv * vv - u_max * u_max).sqrt() / B;
        let w_hi = jmin(vv / B, wMax);
        let th = atan2(V.y, V.x);
        let psi = atan2(-b.y, -b.x);
        let fark = |x: f64| {
            let x = md(x);
            if x > PI {
                x - TP
            } else {
                x
            }
        };
        let mut best: Option<OnPlan> = None;
        let ws: Vec<f64> = if y_i {
            vec![0.0]
        } else if w_lo > 0.0 {
            vec![w_lo, (w_lo + w_hi) / 2.0, w_hi]
        } else {
            vec![0.0, w_hi / 2.0, w_hi]
        };
        for s in [1.0, -1.0] {
            for &wS in &ws {
                let bet = atan2(wS * B, jmax(0.0, vv * vv - wS * wS * B * B).sqrt());
                let caR = th - s * bet;
                let mut phi = caR;
                let mut kal = 0.0;
                if fark(caR - psi).abs() > PI / 2.0 {
                    kal = 1e9;
                    for c in [psi + PI / 2.0, psi - PI / 2.0] {
                        let A = md(s * (caR - c));
                        if A < kal {
                            kal = A;
                            phi = c;
                        }
                    }
                }
                let iyi = match &best {
                    None => true,
                    Some(b) => kal < b.kal - 1e-9 || (kal < b.kal + 1e-9 && wS < b.wS - 1e-9),
                };
                if iyi {
                    best = Some(OnPlan { s, wS, phi, caR, kal, a, Tsw: (wS / a - 1e-9).ceil() });
                }
            }
        }
        best
    }

    /// Döndürme yolunda rakip Çekirdeğe yetişir mi (decide.js:678).
    #[allow(clippy::too_many_arguments)]
    pub fn savur_yol(&self, h: &Oyuncu, src: Option<&[Op]>, ca0: f64, s: f64, P: f64, A: f64, a: f64, wMax: f64, wR: f64, w0: f64) -> bool {
        let src = match src {
            Some(s) => s,
            None => return false,
        };
        let B = Q.body;
        let CR = TQ.min_d / 2.0 + 0.3;
        let MD = TQ.min_d;
        let mut qs: Vec<Kov> = Vec::new();
        for q in src {
            if q.team == h.team || q.noTouch || ayni_varlik(q, h) {
                continue;
            }
            let d = hypot(q.x - h.x, q.y - h.y);
            if !(d > 10.0) {
                qs.push(Kov { f: atan2(q.y - h.y, q.x - h.x), d, v: self.v_s(q), x: None, y: None });
            }
        }
        if qs.is_empty() {
            return false;
        }
        let mut ca = ca0;
        let mut g = P;
        let mut rem = A;
        let mut w = if P > 1e-9 { 0.0 } else { z0(w0) };
        let mut k = 1;
        while k <= 300 && (g > 1e-9 || rem > 1e-9) {
            let geri = g > 1e-9;
            w = if geri { jmin(jmin(w + a, wMax), (2.0 * a * g).sqrt()) } else { jmin(jmin(w + a, wMax), (wR * wR + 2.0 * a * rem).sqrt()) };
            let st = jmin(w, if geri { g } else { rem });
            ca += (if geri { -s } else { s }) * st;
            if geri {
                g -= st;
                if g <= 1e-9 {
                    g = 0.0;
                    w = 0.0;
                }
            } else {
                rem -= st;
            }
            let bx = cos(ca) * B;
            let by = sin(ca) * B;
            for o in qs.iter_mut() {
                let (qx, qy) = rakip_kovala(o, bx, by, MD);
                if hypot(bx - qx, by - qy) < CR {
                    return true;
                }
            }
            k += 1;
        }
        false
    }

    /// Döndürme yayının üstünde rakip var mı (decide.js:716).
    pub fn savur_engel(&self, h: &Oyuncu, src: Option<&[Op]>, ca0: f64, s: f64, A: f64) -> bool {
        let src = match src {
            Some(s) => s,
            None => return false,
        };
        let B = Q.body;
        let CR = TQ.min_d / 2.0 + 0.3;
        for q in src {
            if q.team == h.team || q.noTouch || ayni_varlik(q, h) {
                continue;
            }
            let d = hypot(q.x - h.x, q.y - h.y);
            if d > B + CR || d < B - CR {
                continue;
            }
            if A >= 2.0 * PI - 1e-9 {
                return true;
            }
            let aq = atan2(q.y - h.y, q.x - h.x);
            let yar = acos(jmax(-1.0, jmin(1.0, (B * B + d * d - CR * CR) / (2.0 * B * d))));
            let rel = md(s * (aq - ca0));
            if rel - yar <= A || rel + yar >= TP {
                return true;
            }
        }
        false
    }
}

/// JS sayı doğruluğu (0 ve NaN yanlış).
#[inline(always)]
pub fn dogru0(v: f64) -> bool {
    v != 0.0 && v == v
}
