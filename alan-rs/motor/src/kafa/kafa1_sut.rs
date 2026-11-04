//! Şut (decide.js:1338–1360 TRF/TRI/TRG/TRM/sutUC, 2229–2314 sutT/triF/sutKapali, 2786–2807 acikAci/
//! sutHedef, 3769–3944 SHT/shotCell/shotTab/shotQ).
#![allow(non_snake_case)]

use super::kafa1_baglam::Kafa;
use super::kafa1_js::*;
use super::kafa1_param::{BKS, BODY_R, SAV};
use super::kafa1_sut_tablo::SHT_VE;
use super::kafa1_tehlike::gx_of;
use crate::alan::Kaynak;
use crate::core_sabit::{IN, Q};
use crate::mat::{atan2, cos, exp, hypot, jmax, jmin, jround, sin};
use crate::model::Oyuncu;
use crate::oyuncu::Tepki;
use crate::saha::{CY as GCY, MOUTH, W as GW};
use crate::temas::{bek_dl, bek_dv, bek_har, bek_kay, bek_oku_e, bek_re, bek_uc};
use crate::top::{adim, chg_mul, Top};
use std::sync::Arc;

#[inline(always)]
pub fn sgm(z: f64) -> f64 {
    1.0 / (1.0 + exp(-z))
}
/// Üçgen dağılımın birikimi (JS TRF).
pub fn TRF(z: f64) -> f64 {
    if z <= 0.0 {
        (1.0 + z) * (1.0 + z) / 2.0
    } else {
        1.0 - (1.0 - z) * (1.0 - z) / 2.0
    }
}
pub fn TRI(q: f64) -> f64 {
    let q = jmax(0.0, jmin(1.0, q));
    if q <= 0.5 {
        -1.0 + (2.0 * q).sqrt()
    } else {
        1.0 - (2.0 * (1.0 - q)).sqrt()
    }
}
pub fn TRG(z: f64) -> f64 {
    if z <= 0.0 {
        z * z / 2.0 + z * z * z / 3.0 - 1.0 / 6.0
    } else {
        z * z / 2.0 - z * z * z / 3.0 - 1.0 / 6.0
    }
}
pub fn TRM(a: f64, b: f64) -> f64 {
    let k = TRF(b) - TRF(a);
    if k > 1e-9 {
        (TRG(b) - TRG(a)) / k
    } else {
        (a + b) / 2.0
    }
}
/// JS triF(u, A)
pub fn triF(u: f64, A: f64) -> f64 {
    if A <= 1e-9 {
        if u >= 0.0 {
            1.0
        } else {
            0.0
        }
    } else if u <= -A {
        0.0
    } else if u >= A {
        1.0
    } else if u <= 0.0 {
        (u + A) * (u + A) / (2.0 * A * A)
    } else {
        1.0 - (A - u) * (A - u) / (2.0 * A * A)
    }
}

/// JS makeBall (tek enerji): `vur` verilmişse taban Vuruş tabanından.
pub fn top_yap(x: f64, y: f64, vx: f64, vy: f64, team: f64, ch: Option<f64>, vur: Option<&Oyuncu>) -> Top {
    let P = crate::saha::P;
    let taban = match vur {
        Some(p) => P.tek_taban * jmax(0.5, jmin(1.5, 1.0 + 0.05 * (p.yet.y(crate::yetenek::mek::VURUS_TABAN) - 10.0))),
        None => P.tek_taban,
    };
    Top::yeni(x, y, vx, vy, team as u8, ch, taban)
}

impl Kafa {
    /// Şutun s yolunu alma süresi (decide.js:2230; boş sahada, hız 1/20'ye yuvarlanır).
    pub fn sutT(&self, v0: f64, s: f64) -> f64 {
        let kq = jmax(1.0, jround(20.0 * v0)) as i64;
        let c = {
            let m = self.suty.borrow();
            m.get(&kq).cloned()
        };
        let c = match c {
            Some(c) => c,
            None => {
                let mut b = top_yap(5.0, GCY, kq as f64 / 20.0, 0.0, 0.0, Some(0.5), None);
                let mut c = vec![0.0];
                let bos: Vec<crate::alan::Etki> = Vec::new();
                let P = crate::saha::P;
                let mut t = 0;
                while t < 160 && b.done.is_none() {
                    adim(&mut b, &mut Kaynak::Sec(&bos), &P);
                    c.push(b.x - 5.0);
                    if b.x > 105.0 {
                        break;
                    }
                    t += 1;
                }
                let c = Arc::new(c);
                self.suty.borrow_mut().insert(kq, c.clone());
                c
            }
        };
        for i in 1..c.len() {
            if c[i] >= s {
                return i as f64;
            }
        }
        c.len() as f64 + (s - c[c.len() - 1]) / 0.3
    }

    /// Şutun girme oranı (Bekçi'yle; decide.js:2256). Yer ve şarj yuvarlanır (yarım birim, 0,1).
    pub fn sutKapali(&self, src: &[Op], team: f64, x: f64, y: f64, ch: f64) -> f64 {
        if hypot(gx_of(team) - x, GCY - y) > 1.4 * self.D.shotRange {
            return 0.0;
        }
        let ix = jround(2.0 * x);
        let iy = jround(2.0 * y);
        let ic = jround(10.0 * ch);
        let x = ix / 2.0;
        let y = iy / 2.0;
        let ch = ic / 10.0;
        let gx = gx_of(team);
        let dxg = jmax(0.5, (gx - x).abs());
        let d = hypot(gx - x, GCY - y);
        if d > 1.4 * self.D.shotRange {
            return 0.0;
        }
        let bk = src.iter().find(|p| p.team != team && p.bekci());
        let M = MOUTH;
        let mut hh = Oyuncu::default();
        hh.x = x;
        hh.y = y;
        hh.team = team;
        hh.id = -9.0;
        hh.role = String::new();
        hh.vx = 0.0;
        hh.vy = 0.0;
        hh.D = 1.0;
        let v0 = self.sutV(&hh) * chg_mul(ch);
        let ca = atan2(GCY - y, gx - x);
        let yL = GCY - M + 0.3;
        let yH = GCY + M - 0.3;
        let kr = self.kural();
        let mut r: f64 = 0.0;
        for yt in self.sutHedef(&hh, src, gx) {
            let ux = gx - x;
            let uy = yt - y;
            let L = ya(hypot(ux, uy), 1.0);
            let pl = self.savurPlan(&hh, nk(ux / L * v0, uy / L * v0), ca, 0.0, 0.0, None, false, chg_mul(ch));
            let A = match &pl {
                Some(pl) => SAV.tau * z0(pl.wRel) + self.D.aimErr * z0(pl.uSh),
                None => self.D.aimErr,
            };
            let a0 = atan2(yt - y, dxg);
            let on = triF(atan2(yH - y, dxg) - a0, A) - triF(atan2(yL - y, dxg) - a0, A);
            let mut kur = 0.0;
            if let Some(bk) = bk {
                let kx = bk.x - x;
                let ky = bk.y - y;
                let s = (kx * ux + ky * uy) / L;
                if s > 0.0 {
                    let pd = (kx * uy - ky * ux).abs() / L;
                    let ts = self.sutT(v0, jmin(s, L));
                    let hm = bek_har(bk, x, y, ux, uy, &kr);
                    let re = bek_re(bk) + Q.bek_har_t * hm - bek_oku_e(bk, &hh, IN.cue0);
                    let Rk = Q.bek_body + jmin(bek_dl(bk) * (1.0 - Q.bek_har_d * hm), jmax(0.0, ts - re) * bek_dv(bk)) + jmax(0.0, ts - re - 2.0) * (0.17 + 0.006 * (bk.hz() - 10.0 + 10.0)) * self.dis.ALAN_SPR(bk) * bek_kay(bk);
                    let th = jmax(0.0, jmin(Q.thru_max * Q.bek_thru, (v0 - Q.thru_v) * Q.thru_k - 0.02 * (bk.yet.ozk("kesme") - 10.0)));
                    kur = sgm((jmax(Rk, 1.15) - pd) / BKS) * if pd < BODY_R { 1.0 } else { 1.0 - th * bek_uc(pd, Rk) };
                }
            }
            r = jmax(r, jmax(0.0, on) * (1.0 - kur));
        }
        r
    }

    /// Bekçi'ye göre ağzın açık açısı (decide.js:2786).
    pub fn acikAci(&self, x: f64, y: f64, gx: f64, bk: &Oyuncu) -> f64 {
        let M = MOUTH;
        let dir = if gx > x { 1.0 } else { -1.0 };
        let dxg = jmax(0.5, (gx - x).abs());
        let a0 = atan2(GCY - M + 0.6 - y, dxg);
        let a1 = atan2(GCY + M - 0.6 - y, dxg);
        let dk = ya(hypot(bk.x - x, bk.y - y), 1.0);
        let ak = atan2(bk.y - y, jmax(0.3, (bk.x - x) * dir));
        let kr = self.kural();
        let dl = atan2((Q.reach + 0.6) * (1.0 - 0.4 * bek_har(bk, x, y, gx - x, GCY - M + 0.6 - y, &kr)) + 0.08 * dk, dk);
        let du = atan2((Q.reach + 0.6) * (1.0 - 0.4 * bek_har(bk, x, y, gx - x, GCY + M - 0.6 - y, &kr)) + 0.08 * dk, dk);
        jmax(jmax(0.0, jmin(a1, ak - dl) - a0), a1 - jmax(a0, ak + du))
    }

    /// Şutun hedef y'leri (decide.js:2790): Bekçi'nin bıraktığı boşlukların ortası ve köşeler.
    pub fn sutHedef(&self, h: &Oyuncu, src: &[Op], gx: f64) -> Vec<f64> {
        let M = MOUTH;
        let dir = if gx > h.x { 1.0 } else { -1.0 };
        let dxg = jmax(0.5, (gx - h.x).abs());
        let ang = |y: f64| atan2(y - h.y, dxg);
        let yOf = |a: f64| h.y + tan(a) * dxg;
        let a0 = ang(GCY - M + 0.6);
        let a1 = ang(GCY + M - 0.6);
        let bk = src.iter().find(|p| p.team != h.team && p.bekci());
        let mut gaps: Vec<[f64; 2]> = vec![[a0, a1]];
        if let Some(bk) = bk {
            let kr = self.kural();
            let dk = ya(hypot(bk.x - h.x, bk.y - h.y), 1.0);
            let ak = atan2(bk.y - h.y, jmax(0.3, (bk.x - h.x) * dir));
            let dl = atan2((Q.reach + 0.6) * (1.0 - 0.4 * bek_har(bk, h.x, h.y, gx - h.x, GCY - M + 0.6 - h.y, &kr)) + 0.08 * dk, dk);
            let du = atan2((Q.reach + 0.6) * (1.0 - 0.4 * bek_har(bk, h.x, h.y, gx - h.x, GCY + M - 0.6 - h.y, &kr)) + 0.08 * dk, dk);
            gaps = vec![[a0, jmin(a1, ak - dl)], [jmax(a0, ak + du), a1]].into_iter().filter(|g| g[1] - g[0] > 0.004).collect();
        }
        gaps.sort_by(|p, q| {
            let v = q[1] - q[0] - (p[1] - p[0]);
            if v < 0.0 {
                std::cmp::Ordering::Less
            } else if v > 0.0 {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
        if gaps.is_empty() {
            return vec![GCY - 0.7 * M, GCY + 0.7 * M];
        }
        let ys0: Vec<f64> = if gaps.len() == 1 {
            let g = gaps[0];
            let dGirdi = if (g[0] - a0).abs() < 1e-9 { g[0] + 0.25 * (g[1] - g[0]) } else { g[1] - 0.25 * (g[1] - g[0]) };
            vec![yOf((g[0] + g[1]) / 2.0), yOf(dGirdi)]
        } else {
            gaps.iter().take(2).map(|g| yOf((g[0] + g[1]) / 2.0)).collect()
        };
        let mut ys: Vec<f64> = Vec::new();
        for (i, &y) in ys0.iter().enumerate() {
            if ys0[..i].iter().all(|z| (z - y).abs() > 1.0) {
                ys.push(y);
            }
        }
        for yk in [GCY - M + 0.9, GCY + M - 0.9] {
            if ys.iter().all(|y| (y - yk).abs() > 1.0) {
                ys.push(yk);
            }
        }
        ys
    }

    /// Gönderme tablosunun hücresi (decide.js:3774). Tablo (shot-table.js) motorla aynı sürümde;
    /// tablo dışı hücre istenmez (shotTab sınırlar), istenirse 0.
    pub fn shotCell(&self, di: f64, ai: f64, ci: f64) -> f64 {
        let key = (1e4 * di + 100.0 * ai + ci) as i64;
        match SHT_VE.binary_search_by_key(&key, |c| c.0) {
            Ok(i) => SHT_VE[i].1,
            Err(_) => *self.sht_ek.borrow().get(&key).unwrap_or(&0.0),
        }
    }
    /// decide.js:3925
    pub fn shotTab(&self, x: f64, y: f64, team: f64, ch: f64) -> f64 {
        let gx = gx_of(team);
        let dist = hypot(gx - x, GCY - y);
        let ang = atan2((GCY - y).abs(), (gx - x).abs());
        if dist > 41.0 {
            return 0.0;
        }
        let fd = jmax(0.0, (dist - 3.0) / 2.0);
        let fc = jmax(0.0, jmin(6.0, (ch - 0.1) / 0.15));
        let ai = jmin(7.0, jround(ang / 0.2));
        let d0 = fd.floor();
        let c0 = fc.floor();
        let td = fd - d0;
        let tc = fc - c0;
        let g = |a: f64, b: f64| self.shotCell(jmin(19.0, a), ai, jmin(6.0, b));
        (g(d0, c0) * (1.0 - td) + g(d0 + 1.0, c0) * td) * (1.0 - tc) + (g(d0, c0 + 1.0) * (1.0 - td) + g(d0 + 1.0, c0 + 1.0) * td) * tc
    }
    /// Şutun değeri: girme × yolun açıklığı (decide.js:3932).
    pub fn shotQ(&self, src: &[Op], team: f64, x: f64, y: f64, ch: f64) -> f64 {
        let gx = gx_of(team);
        let d = ya(hypot(gx - x, GCY - y), 1.0);
        let mut pass = 1.0;
        let ux = (gx - x) / d;
        let uy = (GCY - y) / d;
        for p in src {
            if p.team == team || p.bekci() {
                continue;
            }
            let al = (p.x - x) * ux + (p.y - y) * uy;
            if al <= 0.0 || al >= d {
                continue;
            }
            let pe = (-(p.x - x) * uy + (p.y - y) * ux).abs();
            let reachT = jmax(0.0, al / 2.4 - self.TEPKI(p, Tepki::Kovala)) * self.vS(p);
            let R = Q.reach + reachT;
            if pe < R + 0.8 {
                pass *= self.D.shotThru;
            }
        }
        self.sutKapali(src, team, x, y, ch) * pass
    }

    /// Şutun açı dağılımının kaleye düşen dilimleri (decide.js:1345): [z, ağırlık, 0].
    pub fn sutUC(&self, team: f64, o0: Nk, b0: (f64, f64), Ah: f64) -> Option<Vec<[f64; 3]>> {
        if !(Ah > 1e-6) {
            return None;
        }
        let gxL = if team == 0.0 { GW - 0.5 } else { 0.5 };
        let th = atan2(b0.1, b0.0);
        let zA = |yp: f64| {
            let e = atan2(yp - o0.y, gxL - o0.x) - th;
            atan2(sin(e), cos(e)) / Ah
        };
        let z1 = zA(GCY - MOUTH);
        let z2 = zA(GCY + MOUTH);
        let lo = jmax(-1.0, jmin(z1, z2));
        let hi = jmin(1.0, jmax(z1, z2));
        if !(hi > lo) {
            return None;
        }
        let mut out = Vec::new();
        let fl = TRF(lo);
        let fh = TRF(hi);
        if fl > 1e-4 {
            out.push([TRM(-1.0, lo), fl, 0.0]);
        }
        for i in 0..3 {
            let a = TRI(fl + (fh - fl) * i as f64 / 3.0);
            let b = TRI(fl + (fh - fl) * (i + 1) as f64 / 3.0);
            out.push([TRM(a, b), (fh - fl) / 3.0, 0.0]);
        }
        if 1.0 - fh > 1e-4 {
            out.push([TRM(hi, 1.0), 1.0 - fh, 0.0]);
        }
        Some(out)
    }
}
