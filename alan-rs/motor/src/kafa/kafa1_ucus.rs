//! Uçuş (decide.js:1675–2228 ucusFormul, 2315–2326 kenarIm/imagine, 2411–2495 effFric/reachDist/solveV/
//! reachSpeed/reachTime/siddetFor, 2663–2785 launchFor/solveTo/ucusYol/ucusT/ucusSure/bankSure/bankFor).
#![allow(non_snake_case)]

use super::gecici_dis1::{bekSav, predictWorld};
use super::kafa1_baglam::{Kafa, KuralK, UcusYol};
use super::kafa1_js::*;
use super::kafa1_param::{BKS, BODY_R, SAV, UC3, UC5};
use super::kafa1_sut::{sgm, top_yap};
use super::kafa1_tehlike::ox_of;
use super::kafa1_varis::{ctrl_p_al, Kj, Sek, UA};
use crate::alan::{Etki, Kaynak};
use crate::core_sabit::{IN, Q};
use crate::mat::{atan2, cos, hypot, jmax, jmin, jround, jsign, sin};
use crate::model::{JOLaunch, Oyuncu};
use crate::oyuncu::Tepki;
use crate::saha::{CY as GCY, H as GH, KR, W as GW};
use crate::temas::{bek_dl, bek_dv, bek_har, bek_kay, bek_oku_e, bek_re, bek_uc, kuyu_girer, thru_p, TopT, TopTemas};
use crate::top::{adim, tahmin, Bitis, Top};
use crate::yetenek::mek;
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;

/// Uçuşun topu (JS ucusFormul'ün `b0`): çıkış yeri, hızı, şarjı ve (varsa) nişan noktası.
#[derive(Clone, Copy, Debug)]
pub struct UcusB0 {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub ch: f64,
    pub aim: Option<Nk>,
}

/// Uçuşun sonucu (JS ucusFormul dönüşü): arkadaşa (P), Kuyu'ya (Pk), rakibe (Pl) olasılıkları ve yerleri.
#[derive(Clone, Copy, Debug, Default)]
pub struct Ucus {
    pub P: f64,
    pub Pk: f64,
    pub Pl: f64,
    pub end: Nk,
    pub endM: Nk,
    pub endO: Nk,
    pub Ps: f64,
    pub endS: Nk,
    pub endD: Nk,
    pub Psek: f64,
    pub endSek: Nk,
}

/// Bir yerden vurulan topun başlangıcı (JS `{x, y, team}` ya da oyuncu): `vur` vuranın kendisi.
#[derive(Clone, Copy)]
pub struct Cikis<'a> {
    pub x: f64,
    pub y: f64,
    pub team: f64,
    pub vur: Option<&'a Oyuncu>,
}
impl<'a> Cikis<'a> {
    pub fn oyuncu(h: &'a Oyuncu) -> Cikis<'a> {
        Cikis { x: h.x, y: h.y, team: h.team, vur: Some(h) }
    }
    pub fn yer(x: f64, y: f64, team: f64) -> Cikis<'a> {
        Cikis { x, y, team, vur: None }
    }
}

impl Kafa {
    /// Ortalama sürtünme (decide.js:2411).
    pub fn effFric(&self, src: &[Op], team: f64, ch: f64, pts: &[Nk]) -> f64 {
        let P = crate::saha::P;
        let et = etkiler(src);
        let mut k = 0.0;
        for p in pts {
            k += crate::alan::k(&mut Kaynak::Sec(&et), p.x, p.y, team as u8);
        }
        k /= pts.len() as f64;
        if k < 0.0 {
            P.fric + -k * P.opp_drag * (1.0 - P.pierce * ch) * P.tek_drag
        } else {
            P.fric * (1.0 - P.own_flow * k)
        }
    }
    /// decide.js:2418
    pub fn reachDist(&self, v0: f64, va: f64, fr: f64) -> f64 {
        let roll = crate::saha::P.roll;
        let mut v = v0;
        let mut d = 0.0;
        let mut i = 0;
        while i < 900 && v > va {
            v = jmax(0.0, v * (1.0 - fr) - roll);
            d += v;
            i += 1;
        }
        d
    }
    /// decide.js:2427
    pub fn solveV(&self, L: f64, va: f64, fr: f64) -> (f64, bool) {
        let mut lo = va;
        let mut hi = self.D.vmax;
        if self.reachDist(hi, va, fr) < L {
            return (hi, false);
        }
        for _ in 0..16 {
            let mid = (lo + hi) / 2.0;
            if self.reachDist(mid, va, fr) < L {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        (hi, true)
    }
    fn top_cik(&self, h: &Cikis, vx: f64, vy: f64, ch: f64) -> Top {
        top_yap(h.x, h.y, vx, vy, h.team, Some(ch), h.vur)
    }
    /// L yoluna varınca topun hızı (decide.js:2450; yetişmezse -1).
    pub fn reachSpeed(&self, h: &Cikis, dx: f64, dy: f64, L: f64, v0: f64, et: &[Etki], kc: Kc, ch: f64) -> f64 {
        let mut b = self.top_cik(h, dx * v0, dy * v0, ch);
        let P = crate::saha::P;
        kaynakla(et, kc, |k| {
            let mut t = 0;
            while t < 400 && b.done.is_none() {
                adim(&mut b, k, &P);
                if hypot(b.x - h.x, b.y - h.y) >= L {
                    return hypot(b.vx, b.vy);
                }
                t += 1;
            }
            -1.0
        })
    }
    /// L yoluna varış süresi (decide.js:2467; yetişmezse 1e9).
    pub fn reachTime(&self, h: &Cikis, dx: f64, dy: f64, L: f64, v0: f64, et: &[Etki], kc: Kc, ch: f64) -> f64 {
        let mut b = self.top_cik(h, dx * v0, dy * v0, ch);
        let P = crate::saha::P;
        kaynakla(et, kc, |k| {
            let mut t = 0;
            while t < 300 && b.done.is_none() {
                adim(&mut b, k, &P);
                if hypot(b.x - h.x, b.y - h.y) >= L {
                    return (t + 1) as f64;
                }
                t += 1;
            }
            1e9
        })
    }
    /// tR tikte varan en yumuşak pas hızı (decide.js:2484).
    pub fn siddetFor(&self, h: &Oyuncu, to: Nk, tR: f64, src: &[Op], ch: f64, kc: Kc) -> f64 {
        let et = etkiler(src);
        let c = Cikis::oyuncu(h);
        let L = ya(hypot(to.x - h.x, to.y - h.y), 1.0);
        let dx = (to.x - h.x) / L;
        let dy = (to.y - h.y) / L;
        let vm = self.vmaxOf(Some(h));
        if self.reachTime(&c, dx, dy, L, vm, &et, kc, ch) > tR {
            return vm;
        }
        let mut lo = 0.2;
        let mut hi = vm;
        if self.reachTime(&c, dx, dy, L, lo, &et, kc, ch) <= tR {
            return lo;
        }
        for _ in 0..8 {
            let mid = (lo + hi) / 2.0;
            if self.reachTime(&c, dx, dy, L, mid, &et, kc, ch) > tR {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        hi
    }
    /// `to`'ya `arrive` hızıyla varan çıkış (decide.js:2663).
    pub fn launchFor(&self, h: &Oyuncu, to: Nk, arrive: f64, src: &[Op], ch: f64, kc: Kc) -> JOLaunch {
        let et = etkiler(src);
        let c = Cikis::oyuncu(h);
        let L = ya(hypot(to.x - h.x, to.y - h.y), 1.0);
        let dx = (to.x - h.x) / L;
        let dy = (to.y - h.y) / L;
        let vm = self.vmaxOf(Some(h));
        if self.reachSpeed(&c, dx, dy, L, vm, &et, kc, ch) < arrive {
            return JOLaunch { vx: dx * vm, vy: dy * vm, ok: false };
        }
        let mut lo = arrive;
        let mut hi = vm;
        for _ in 0..7 {
            let mid = (lo + hi) / 2.0;
            if self.reachSpeed(&c, dx, dy, L, mid, &et, kc, ch) < arrive {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        JOLaunch { vx: dx * hi, vy: dy * hi, ok: true }
    }
    /// decide.js:2682 (yetişmezse None)
    pub fn solveTo(&self, h: &Cikis, to: Nk, arrive: f64, et: &[Etki], kc: Kc, ch: f64) -> Option<f64> {
        let L = ya(hypot(to.x - h.x, to.y - h.y), 1.0);
        let dx = (to.x - h.x) / L;
        let dy = (to.y - h.y) / L;
        let vm = self.vmaxOf(h.vur);
        if self.reachSpeed(h, dx, dy, L, vm, et, kc, ch) < arrive {
            return None;
        }
        let mut lo = arrive;
        let mut hi = vm;
        for _ in 0..7 {
            let mid = (lo + hi) / 2.0;
            if self.reachSpeed(h, dx, dy, L, mid, et, kc, ch) < arrive {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        Some(hi)
    }
    /// Boş sahada L'lik pasın yolu (decide.js:2694; uzunluk ve varış hızına göre önbellekli).
    pub fn ucusYol(&self, L: f64, arrive: f64) -> Arc<UcusYol> {
        let Li = jmax(1.0, jmin(80.0, jround(L))) as usize;
        let key = arrive.to_bits();
        if let Some(r) = self.ucus.borrow().get(&key).and_then(|a| a.get(Li).cloned().flatten()) {
            return r;
        }
        let h = Cikis::yer(5.0, GCY, 0.0);
        let bos: Vec<Etki> = Vec::new();
        let v0 = self.solveTo(&h, nk(h.x + Li as f64, GCY), arrive, &bos, None, 0.5).unwrap_or(self.D.vmax);
        let mut b = top_yap(h.x, h.y, v0, 0.0, 0.0, Some(0.5), None);
        let mut cum = vec![0.0];
        let mut T = 1e9;
        let P = crate::saha::P;
        let mut t = 0;
        while t < 400 && b.done.is_none() {
            adim(&mut b, &mut Kaynak::Sec(&bos), &P);
            let d = b.x - h.x;
            cum.push(d);
            if T == 1e9 && d >= Li as f64 {
                T = (t + 1) as f64;
            }
            if d >= Li as f64 + 30.0 || hypot(b.vx, b.vy) < 0.02 {
                break;
            }
            t += 1;
        }
        let r = Arc::new(UcusYol { v0, cum, T });
        let mut m = self.ucus.borrow_mut();
        let a = m.entry(key).or_default();
        if a.len() <= Li {
            a.resize(Li + 1, None);
        }
        a[Li] = Some(r.clone());
        r
    }
    /// Pasın L'lik (ya da `al`'a dek) yolu alma süresi (decide.js:2736).
    pub fn ucusT(&self, L: f64, arrive: f64, al: Option<f64>) -> f64 {
        let r = self.ucusYol(L, arrive);
        match al {
            Some(al) if al < L => {
                let c = &r.cum;
                for i in 1..c.len() {
                    if c[i] >= al {
                        return i as f64;
                    }
                }
                r.T
            }
            _ => r.T,
        }
    }
    /// Pasın varış süresi (decide.js:2743).
    pub fn ucusSure(&self, h: &Oyuncu, at: Nk, lv: Nk, src: &[Op], ch: f64, savT: Option<f64>, kc: Kc) -> f64 {
        let L = hypot(at.x - h.x, at.y - h.y);
        let v = ya(hypot(lv.x, lv.y), 1.0);
        let sv = yao(savT, 0.0);
        if L < 0.5 {
            return 4.0 + sv;
        }
        let et = etkiler(src);
        let t = self.reachTime(&Cikis::oyuncu(h), (at.x - h.x) / L, (at.y - h.y) / L, L, v, &et, kc, ch);
        (if t < 1e9 { t } else { L / jmax(0.5, 0.7 * v) }) + 4.0 + sv
    }
    /// Kenardan sektirme pasının süresi (decide.js:2749).
    #[allow(clippy::too_many_arguments)]
    pub fn bankSure(&self, h: &Oyuncu, B: Nk, T: Nk, lv: Nk, src: &[Op], ch: f64, savT: Option<f64>, kc: Kc) -> f64 {
        let P = crate::saha::P;
        let et = etkiler(src);
        let c = Cikis::oyuncu(h);
        let l1 = ya(hypot(B.x - h.x, B.y - h.y), 1.0);
        let l2 = ya(hypot(T.x - B.x, T.y - B.y), 1.0);
        let v = ya(hypot(lv.x, lv.y), 1.0);
        let t1 = self.reachTime(&c, (B.x - h.x) / l1, (B.y - h.y) / l1, l1, v, &et, kc, ch);
        let vB = self.reachSpeed(&c, (B.x - h.x) / l1, (B.y - h.y) / l1, l1, v, &et, kc, ch) * P.rest;
        let t2 = if vB > 0.0 { self.reachTime(&Cikis::yer(B.x, B.y, h.team), (T.x - B.x) / l2, (T.y - B.y) / l2, l2, vB, &et, kc, ch) } else { 1e9 };
        if t1 < 1e9 && t2 < 1e9 {
            t1 + t2 + 4.0 + yao(savT, 0.0)
        } else {
            (l1 + l2) / jmax(0.5, 0.6 * v) + 4.0
        }
    }
    /// Kenardan sektirip q'ya `arrive` hızıyla varan çıkış (decide.js:2757).
    pub fn bankFor(&self, h: &Oyuncu, B: Nk, q: Nk, arrive: f64, src: &[Op], ch: f64, kc: Kc) -> JOLaunch {
        let P = crate::saha::P;
        let et = etkiler(src);
        let v2 = match self.solveTo(&Cikis::yer(B.x, B.y, h.team), q, arrive, &et, kc, ch) {
            Some(v) => v,
            None => return JOLaunch { vx: 0.0, vy: 0.0, ok: false },
        };
        let vB = v2 / P.rest;
        let vm = self.vmaxOf(Some(h));
        if vB > vm {
            return JOLaunch { vx: 0.0, vy: 0.0, ok: false };
        }
        let v1 = self.solveTo(&Cikis::oyuncu(h), B, vB, &et, kc, ch);
        let l1 = ya(hypot(B.x - h.x, B.y - h.y), 1.0);
        match v1 {
            None => JOLaunch { vx: (B.x - h.x) / l1 * vm, vy: (B.y - h.y) / l1 * vm, ok: false },
            Some(v1) => JOLaunch { vx: (B.x - h.x) / l1 * v1, vy: (B.y - h.y) / l1 * v1, ok: true },
        }
    }

    /// Kafadaki uçuş denemesi (decide.js:2324): bugünkü davranışta formül (ucusFormul).
    pub fn imagine(&mut self, h: &Oyuncu, src: &[Op], b0: UcusB0, q: Option<&Oyuncu>, to: Option<Nk>, kc: Kc) -> Ucus {
        self.ucusFormul(h, src, b0, q, to, kc)
    }
    /// Kenardan sektirme denemesi (decide.js:2315): D._kenar açıkken imagine.
    pub fn kenarIm(&mut self, h: &Oyuncu, src: &[Op], b0: UcusB0, q: Option<&Oyuncu>, to: Option<Nk>, kc: Kc) -> Ucus {
        let k0 = self.D._kenar;
        self.D._kenar = Some(true);
        let r = self.imagine(h, src, b0, q, to, kc);
        self.D._kenar = k0;
        r
    }

    /// Topun uçuşunun formülü (decide.js:1675): her tikte her adayın topa yetişme olasılığı.
    pub fn ucusFormul(&mut self, h: &Oyuncu, src0: &[Op], b0in: UcusB0, q: Option<&Oyuncu>, to: Option<Nk>, kc0: Kc) -> Ucus {
        // D ve dış dünya yerel kopyadan okunur: keepP / predictWorld `&mut self` ister (kafa2'nin önbellekleri)
        let D0 = self.D.clone();
        let D = &D0;
        let team = h.team;
        let gon = D._gon.unwrap_or(false);
        let dis0 = self.dis.clone();
        let kr = KuralK(&*dis0);
        let S_ = self.dis.S();
        let GR = self.dis.GR();
        let tick = self.tick();
        let qqI: Option<usize> = q.and_then(|q| bul_i(src0, q));
        let kendi = qqI.map_or(false, |i| src0[i].id == h.id);
        let hI0 = sira(src0, h);
        let lk = ya(D._ot.unwrap_or(0.0), 1.0) * 1.0;
        let tur = if gon { self.d_sut_tur() } else { None };
        let YF = self.hataF(h, Some(src0), gon, tur.as_deref(), D._kenar.unwrap_or(false), lk);
        let mut gecik = D._savT.unwrap_or(0.0);
        let mut caR = D._savCa;
        let mut savW = D._savW;
        let mut savU = D._savU;
        if D._savT.is_none() && h.id >= 0.0 {
            let ks = D._kanca.as_ref().map(|k| k.s).filter(|s| *s != 0.0 && *s == *s);
            let pl = self.savurPlan(
                h,
                nk(b0in.vx, b0in.vy),
                self.caOf(h),
                D._kanca.as_ref().map_or(0.0, |k| k.w0),
                ks.unwrap_or(yao(D._sYon, 0.0)),
                if gon { None } else { Some(src0) },
                gon && D._itme.unwrap_or(false),
                self.vurE(h, Some(b0in.ch), gon),
            );
            if let Some(pl) = pl {
                if pl.T >= 1.0 {
                    gecik = pl.T;
                }
                caR = Some(pl.caR);
                if savW.is_none() {
                    savW = Some(pl.wRel);
                    savU = Some(pl.uSh);
                }
            }
        }
        let gecik = jmax(0.0, jround(gecik));
        let BSV = gecik >= 1.0 && h.id >= 0.0;
        let ws: Cow<[Op]> = if BSV {
            Cow::Owned(src0.iter().map(|p| if p.team != team && p.bekci() { bekSav(self, p, h, gecik, src0) } else { p.clone() }).collect())
        } else {
            Cow::Borrowed(src0)
        };
        let src: &[Op] = &ws;
        // JS: eşlenen dünya yeni dizidir (önbelleği yok)
        let kc: Kc = if BSV { None } else { kc0 };
        let hI = hI0;
        let OK1 = h.id >= 0.0;
        let cue1 = if OK1 { (if D._tekD { IN.cue_ot } else { IN.cue0 }) + gecik } else { 0.0 };
        let uM1 = (IN.u_a + IN.u_b) / 2.0;
        let giz1 = if OK1 { h.yet.y(mek::VURUS_GIZLE) - 10.0 } else { 0.0 };
        let hIc = h.id >= 0.0 && hI.is_some();
        let hKI: Option<usize> = if h.id >= 0.0 { bul_i(src, h) } else { None };
        let pre = if gecik >= 1.0 && hIc { jmax(0.0, jmin(1.0, self.keepP(&src[hI.unwrap()], src, None, gecik))) } else { 1.0 };
        let pK1 = self.pasK(gon, b0in.vx, b0in.vy, Some(h));
        let Ah = (match savW {
            Some(w) => SAV.tau * w * YF.b + D.aimErr * pK1 * z0(savU.unwrap_or(0.0)) * YF.y,
            None => D.aimErr * pK1 * YF.y,
        }) / 10.0;
        let ayniyer = h.id >= 0.0 && b0in.x == h.x && b0in.y == h.y;
        let o0 = if ayniyer {
            match caR {
                Some(ca) => nk(h.x + z0(h.vx) * gecik * 0.5 + cos(ca) * Q.body, h.y + z0(h.vy) * gecik * 0.5 + sin(ca) * Q.body),
                None => {
                    let cs = self.coreSide(h);
                    nk(h.x + cs.ux * Q.body, h.y + cs.uy * Q.body)
                }
            }
        } else {
            nk(b0in.x, b0in.y)
        };
        let mut b0 = b0in;
        if let Some(aim) = b0in.aim {
            if ayniyer {
                let sp = hypot(b0.vx, b0.vy);
                let dx = aim.x - o0.x;
                let dy = aim.y - o0.y;
                let L = hypot(dx, dy);
                if L > 1.0 && sp > 0.0 {
                    b0.vx = dx / L * sp;
                    b0.vy = dy / L * sp;
                }
            }
        }
        let err = (20.0 - h.yet.y(mek::ALGI_HATA)) * D.errOk * D.errPos;
        let mut ad: Vec<UA> = Vec::new();
        for (i, p) in src.iter().enumerate() {
            if p.noTouch && (p.team == team || !(p.stun.map_or(false, |s| s > tick) || p.adandi.map_or(false, |s| s > tick))) {
                continue;
            }
            let rakip = p.team != team;
            let ph = hI == Some(i);
            if !rakip && (qqI != Some(i)) && h.id >= 0.0 && !ph && hKI != Some(i) {
                let L0 = ya(hypot(b0.vx, b0.vy), 1.0);
                let ux = b0.vx / L0;
                let uy = b0.vy / L0;
                let ax = p.x - o0.x;
                let ay = p.y - o0.y;
                let al = ax * ux + ay * uy;
                let pe = -ax * uy + ay * ux;
                let vA = z0(p.vx) * ux + z0(p.vy) * uy;
                let vL = -z0(p.vx) * uy + z0(p.vy) * ux;
                if al > -1.0 && (pe + 20.0 * vL).abs() < 6.0 && !(p.bekci() && hypot(p.x - ox_of(p.team), p.y - GCY) < KR) {
                    let tA = gecik + jmax(0.0, al) / L0;
                    let YT = self.dis.ALAN_YOL_VER_TEPKI();
                    let tD = jmax(0.0, tA - if YT { jmax(0.0, self.TEPKI(p, Tepki::Kovala)) } else { gecik });
                    let lt0 = pe + vL * tA;
                    let sg = if YT && lt0.abs() > 0.05 {
                        jsign(lt0)
                    } else if pe.abs() > 0.05 {
                        jsign(pe)
                    } else if vL >= 0.0 {
                        1.0
                    } else {
                        -1.0
                    };
                    let aA = al + vA * tA;
                    let lat0 = if p.bekci() { pe + vL * tA } else { pe + vL * tA + 0.5 * sg * 0.02 * tD * tD };
                    let lx0 = o0.x + ux * aA - uy * lat0;
                    let ly0 = o0.y + uy * aA + ux * lat0;
                    let lat = {
                        let cx0 = jmax(1.5, jmin(GW - 1.5, lx0));
                        let cy0 = jmax(1.5, jmin(GH - 1.5, ly0));
                        if cx0 == lx0 && cy0 == ly0 {
                            lat0
                        } else {
                            -(cx0 - o0.x) * uy + (cy0 - o0.y) * ux
                        }
                    };
                    if lat.abs() < BODY_R + 0.05 {
                        let mut v = (**p).clone();
                        v.x = o0.x + ux * aA - uy * lat;
                        v.y = o0.y + uy * aA + ux * lat;
                        let mut A = UA::yeni(Cow::Owned(v), None);
                        A.yv = true;
                        A.gvT = 1e9;
                        ad.push(A);
                    }
                }
                continue;
            }
            let BEN = ph && !kendi && !gon;
            if !(rakip || qqI == Some(i) || BEN) {
                continue;
            }
            if ph && !kendi && !BEN {
                continue;
            }
            if gon && !rakip {
                continue;
            }
            let v = self.vS(p);
            let dK = self.dis.ALAN_DENGE_KAYIP(p, tick);
            let bek = rakip && p.bekci();
            let rc = if rakip {
                if bek {
                    bek_re(p)
                } else {
                    jmax(0.0, self.TEPKI(p, Tepki::Kovala))
                }
            } else {
                0.0
            };
            let hmA = if bek { bek_har(p, o0.x, o0.y, b0.vx, b0.vy, &kr) } else { 0.0 };
            let fO = if OK1 && rakip && !bek { jmax(0.05, jmin(0.95, 0.5 + IN.sez * (p.yet.y(mek::VURUS_SEZ) - 10.0) - IN.giz * giz1)) } else { 0.0 };
            let onE = cue1 * fO * uM1;
            let sdW = if fO != 0.0 { hypot(rc * IN.tepki_z / 2.449, cue1 * fO * (IN.u_b - IN.u_a) / 3.464) / 1.814 } else { 0.0 };
            let mut A = UA::yeni(Cow::Borrowed(&**p), Some(i));
            A.G = self.dis.gecTab(p, false);
            A.v = v;
            A.rc = dK + (if fO != 0.0 { gecik + jmax(0.0, rc - onE) } else if bek && gon && OK1 { rc - bek_oku_e(p, h, cue1) } else { rc }) + if bek { Q.bek_har_t * hmA } else { 0.0 };
            A.dMax = bek_dl(p) * (1.0 - Q.bek_har_d * hmA);
            A.hm = hmA;
            A.rakip = rakip;
            A.bek = bek;
            A.R0 = self.menzil(p);
            A.rR = self.TEPKI(p, Tepki::Uzanma) - onE;
            A.rR0 = self.TEPKI(p, Tepki::Uzanma);
            A.rcY = if fO != 0.0 { Some(gecik + jmax(1.0, self.TEPKI(p, Tepki::YeniTop)) - onE) } else { None };
            A.dV = bek_dv(p);
            A.kay = bek_kay(p);
            let eJ = match (rakip, self.dis.algi_e(p), &GR) {
                (true, Some(e), Some(_)) => e + self.dis.GR_yasHata(self.dis.algi_yas(p).unwrap_or(f64::NAN)),
                _ => err,
            };
            A.tau = if sdW != 0.0 { hypot(1.5 + eJ / v * 0.5, sdW) } else { 1.5 + if rakip { eJ / v * 0.5 } else { 0.0 } };
            A.ben = ph;
            A.bk0 = p.bekci();
            A.gvT = if rakip && !bek && h.id >= 0.0 { jmax(jmax(0.0, z0(p.adandi.unwrap_or(0.0)) - tick), z0(p.stun.unwrap_or(0.0)) - tick) } else { 0.0 };
            ad.push(A);
        }
        let mut Wt = 0.0;
        let (mut m, mut o, mut k, mut ex, mut ey, mut mx, mut my, mut ox, mut oy, mut sk, mut sx, mut sy, mut dk, mut dxs, mut dys) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let As = if h.id < 0.0 { 0.0 } else { YF.v * D.v0Err / 10.0 * self.pasKv(gon) };
        let UCs: Vec<[f64; 3]> = match if gon { self.sutUC(team, o0, (b0.vx, b0.vy), Ah) } else { None } {
            Some(u) => u,
            None => {
                if As > 1e-9 {
                    UC5.to_vec()
                } else {
                    UC3.to_vec()
                }
            }
        };
        let na = ad.len();
        let mut SK: Vec<Sek> = vec![Sek { v: [0.0; 6], n: 0 }; na];
        let mut SKa = vec![0.0; na];
        let mut SKi: Vec<Option<f64>> = vec![None; na];
        let mut SKx = vec![0.0; na];
        let mut SKy = vec![0.0; na];
        let mut SZ: Vec<Sek> = vec![Sek { v: [0.0; 6], n: 0 }; na];
        let mut SZa = vec![0.0; na];
        let mut SZi: Vec<Option<f64>> = vec![None; na];
        let mut SZx = vec![0.0; na];
        let mut SZy = vec![0.0; na];
        let KUC = 0.002;
        let etP = etkiler(src);
        let mut UCsY: Vec<(f64, f64, f64, bool)> = UCs.iter().map(|u| (u[0], u[1], u[2], false)).collect();
        if !gon && h.id >= 0.0 {
            let B = to.or_else(|| qqI.map(|i| nk(src[i].x, src[i].y)));
            if let Some(B) = B {
                let L = ya(hypot(B.x - o0.x, B.y - o0.y), 1.0);
                let v0 = ya(hypot(b0.vx, b0.vy), 1.0);
                let t1 = self.reachTime(&Cikis::yer(o0.x, o0.y, team), (B.x - o0.x) / L, (B.y - o0.y) / L, L, v0, &etP, kc, b0.ch);
                let Tf = jmax(1.0, jround(gecik + if t1 < 1e9 { t1 } else { L / jmax(0.5, 0.7 * v0) }));
                let Pw = predictWorld(self, src, team, B, Tf, false, true);
                let mut var2 = false;
                for A in ad.iter_mut() {
                    A.kj2t = None;
                    if !A.rakip || A.bek {
                        continue;
                    }
                    if let Some(i) = A.pi {
                        if let Some(Some(w)) = Pw.get(i) {
                            A.kj2t = Some(*w);
                            var2 = true;
                        }
                    }
                }
                if var2 {
                    UCsY = UCs.iter().flat_map(|u| [(u[0], u[1] / 2.0, u[2], false), (u[0], u[1] / 2.0, u[2], true)]).collect();
                }
            }
        }
        let mut okc: HashMap<i64, (f64, f64)> = HashMap::new();
        let hfrom: Option<&Oyuncu> = Some(h);
        for &(z, w, zs, yol) in &UCsY {
            let an = z * Ah;
            let c = cos(an);
            let s = sin(an);
            let kv = 1.0 + z0(zs) * As;
            let b = top_yap(o0.x, o0.y, (b0.vx * c - b0.vy * s) * kv, (b0.vx * s + b0.vy * c) * kv, team, Some(b0.ch), Some(h));
            let (pts, end) = kaynakla(&etP, kc, |kk| tahmin(&b, kk, &crate::saha::P, Some(D.simMax as usize)));
            let n = pts.len();
            for A in ad.iter_mut() {
                if let Some(g0) = A.G0.take() {
                    A.G = g0;
                }
                A.tc = 0.0;
                A.M = -1e9;
                A.Gk = 0.0;
            }
            if !gon {
                if let Some(S) = &S_ {
                    let RB = z0(S.rcvBuf);
                    let HY = GR.as_ref().map_or(40.0, |g| g.yasH);
                    let isP = |A: &UA, T: f64| -> (f64, f64) {
                        match &A.kj {
                            Some(kj) => self.kosYer(&A.p, kj, T),
                            None => {
                                let f = if T < HY { T - T * T / (2.0 * HY) } else { HY / 2.0 };
                                (A.p.x + z0(A.p.vx) * f, A.p.y + z0(A.p.vy) * f)
                            }
                        }
                    };
                    let varE = |A: &UA, c: f64, px: f64, py: f64, rc0: f64| -> f64 {
                        let buf = if A.rakip { 0.0 } else { RB };
                        let mut i = c as usize;
                        while i < n {
                            let fi = i as f64;
                            let q = pts[i];
                            let d = hypot(q.x - px, q.y - py);
                            let dr = d - if A.rakip { jmax(1.15, jmin(A.R0, Q.reach_body + jmax(0.0, fi - A.rR) * Q.reach_grow)) } else { A.R0 };
                            let t0v = c + rc0 + jmax(0.0, dr) / A.v;
                            if t0v + buf > fi {
                                i += 2;
                                continue;
                            }
                            let t = jmax(t0v + A.G.as_ref().map_or(0.0, |G| self.dis.gecOf(G, q.x, q.y, Some(dr))), if A.rakip { 0.0 } else { self.frenT(&A.p, q.x, q.y, A.R0, fi - rc0, A.v) }) + buf;
                            if t <= fi {
                                return fi;
                            }
                            i += 2;
                        }
                        1e9
                    };
                    let mut oEk: f64 = 1e9;
                    for A in ad.iter_mut() {
                        A.kov = Some(true);
                        A.kovT = Some(0.0);
                    }
                    for A in ad.iter() {
                        if !A.rakip && qqI.is_some() && A.pi == qqI && !A.yv {
                            oEk = varE(A, 0.0, A.p.x, A.p.y, 0.0);
                        }
                    }
                    let RV: Vec<usize> = (0..na).filter(|&j| ad[j].rakip && !ad[j].bek).collect();
                    for &j in &RV {
                        let A = &mut ad[j];
                        A.kov = Some(false);
                        A.kovT = None;
                        A.kt = None;
                        A.kj = None;
                        let p: &Oyuncu = &A.p;
                        let tq = p.tutQ.and_then(|t| src.get(t));
                        let tqOk = match (tq, qqI) {
                            (Some(tq), Some(qi)) => tq.name == src[qi].name && tq.team == src[qi].team && oEk < n as f64,
                            _ => false,
                        };
                        let kj = if tqOk {
                            let P0 = pts[oEk as usize];
                            Some(Kj { t: self.dis.gside(nk(P0.x, P0.y), p.team, ya(S.markOff.unwrap_or(0.0), 2.5)), r: jmax(0.0, A.rc), v: A.v })
                        } else if yol && A.kj2t.is_some() {
                            Some(Kj { t: A.kj2t.unwrap(), r: 0.0, v: self.spd(p) })
                        } else {
                            Some(Kj { t: nk(p.tx, p.ty), r: 0.0, v: if p.sprint { A.v } else { self.spd(p) } })
                        };
                        A.kj = kj;
                    }
                    let kural = |A: &UA, e: f64, bS: f64| e <= bS + (20.0 - A.p.yet.y(mek::ALGI_HATA)) * S.marginT + 1.0 && !(e > oEk + 6.0) && !(e > oEk + 1.0);
                    let mut c = 0.0;
                    while c < n as f64 && c <= oEk {
                        let mut bS: f64 = 1e9;
                        for &j in &RV {
                            if ad[j].kovT.is_some() {
                                let e = ad[j].est;
                                if e < bS && !(e < c) {
                                    bS = e;
                                }
                                continue;
                            }
                            let (px, py) = isP(&ad[j], gecik + c);
                            ad[j].c = (px, py);
                            let mut Gy: Option<crate::kafa::gecici_dis1::GecTab> = None;
                            ad[j].Gy = None;
                            if ad[j].kj.is_some() && ad[j].G.is_some() && gecik + c > 0.0 {
                                let kj = ad[j].kj.unwrap();
                                let (wx, wy) = self.kosHiz(&ad[j].p, &kj, gecik + c);
                                let mut oo = (*ad[j].p).clone();
                                oo.x = px;
                                oo.y = py;
                                oo.vx = wx;
                                oo.vy = wy;
                                Gy = self.dis.gecTab(&oo, true);
                                let A = &mut ad[j];
                                if A.G0.is_none() {
                                    A.G0 = Some(A.G.clone());
                                }
                                A.Gkk = A.G.clone();
                                A.G = Gy.clone();
                            }
                            let ec = varE(&ad[j], c, px, py, jmax(0.0, ad[j].rc - gecik - c));
                            let A = &mut ad[j];
                            A.ec = ec;
                            if Gy.is_some() {
                                A.Gy = Gy;
                                A.G = A.Gkk.clone();
                            }
                            if A.ec < bS {
                                bS = A.ec;
                            }
                        }
                        let mut hepsi = true;
                        for &j in &RV {
                            if ad[j].kovT.is_some() {
                                continue;
                            }
                            let ec = ad[j].ec;
                            if kural(&ad[j], ec, bS) {
                                let A = &mut ad[j];
                                A.kov = Some(true);
                                A.kovT = Some(c);
                                A.est = ec;
                                A.kx = A.c.0;
                                A.ky = A.c.1;
                                A.kt = None;
                                if A.Gy.is_some() {
                                    if A.G0.is_none() {
                                        A.G0 = Some(A.G.clone());
                                    }
                                    A.G = A.Gy.clone();
                                }
                            } else {
                                hepsi = false;
                                let ah = ad[j].p.yet.y(mek::ALGI_HATA);
                                if ad[j].kt.is_none() && oEk < n as f64 && ec <= bS + (20.0 - ah) * S.marginT + 1.0 && !(ec > oEk + 6.0) && ec > oEk + 1.0 {
                                    let P0 = pts[oEk as usize];
                                    let t = self.dis.gside(nk(P0.x, P0.y), ad[j].p.team, 2.2);
                                    let A = &mut ad[j];
                                    A.kt = Some(t);
                                    A.ktT = c;
                                    A.kx = A.c.0;
                                    A.ky = A.c.1;
                                }
                            }
                        }
                        if hepsi {
                            break;
                        }
                        c += 4.0;
                    }
                }
            }
            let rcvB = S_.as_ref().map_or(0.0, |s| z0(s.rcvBuf));
            let mut DD = vec![0.0; na];
            let mut XM: Vec<Option<f64>> = vec![None; na];
            let mut RR = vec![0.0; na];
            let mut KF: Vec<Option<f64>> = vec![None; na];
            let mut CX = vec![0.0; na];
            let mut CY = vec![0.0; na];
            let mut S = 1.0;
            let (mut m1, mut o1, mut k1, mut mx1, mut my1, mut ox1, mut oy1, mut son, mut s1, mut sx1, mut sy1, mut d1, mut dx1, mut dy1) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0usize, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
            let mut F = vec![0.0; na];
            let mut i = 1;
            while i < n && S > 0.003 {
                let fi = i as f64;
                let P = pts[i];
                let Pp = pts[i - 1];
                let bvx = P.x - Pp.x;
                let bvy = P.y - Pp.y;
                if i >= 2 {
                    let Z = pts[i - 2];
                    let ax = Pp.x - Z.x;
                    let ay = Pp.y - Z.y;
                    let la = hypot(ax, ay);
                    let lc = hypot(bvx, bvy);
                    if la > 0.02 && lc > 0.02 && (ax * bvx + ay * bvy) / (la * lc) < 0.85 {
                        son = i - 1;
                    }
                }
                let tp = jmax(0.0, fi - son as f64 - if son == 0 { 1.0 } else { 0.0 });
                let Tb = gecik + fi;
                let mut fs = 0.0;
                for j in 0..na {
                    F[j] = 0.0;
                    let A = &mut ad[j];
                    if A.ben && i < 8 {
                        continue;
                    }
                    let p: &Oyuncu = &A.p;
                    let sl = bvx * bvx + bvy * bvy;
                    let t0 = if sl > 1e-12 { ((p.x - Pp.x) * bvx + (p.y - Pp.y) * bvy) / sl } else { 1.0 };
                    let tt = if t0 < 0.0 { 0.0 } else if t0 > 1.0 { 1.0 } else { t0 };
                    let cx = Pp.x + bvx * tt;
                    let cy = Pp.y + bvy * tt;
                    let dd = hypot(cx - p.x, cy - p.y);
                    DD[j] = dd;
                    KF[j] = None;
                    CX[j] = cx;
                    CY[j] = cy;
                    A.gvNow = A.gvT > Tb;
                    if A.gvNow {
                        if dd < BODY_R {
                            F[j] = 1.0;
                            fs += 1.0;
                        }
                        continue;
                    }
                    if A.yv {
                        continue;
                    }
                    let tpE = tp + tt;
                    let mut R = if A.bek {
                        Q.bek_body + jmin(A.dMax, jmax(0.0, tpE - A.rc) * A.dV)
                    } else if A.rakip {
                        jmin(A.R0, Q.reach_body + jmax(0.0, tpE - if son == 0 { A.rR } else { A.rR0 }) * Q.reach_grow)
                    } else {
                        A.R0
                    };
                    if t0 < 0.0 {
                        R *= Q.away_k;
                    }
                    R = jmax(R, 1.15);
                    if !A.bk0 && !kuyu_girer(p, cx, cy, Tb, false, &kr) {
                        continue;
                    }
                    if dd > R + 40.0 {
                        continue;
                    }
                    let mut zz;
                    if A.bek {
                        let mut ddB = dd;
                        if A.hm > 0.0 {
                            let tE = if BSV { jmax(0.0, jmin(Tb - gecik, Q.bek_har_t * A.hm)) } else { jmin(Tb, gecik + Q.bek_har_t * A.hm) };
                            let qx = p.x + z0(p.vx) * tE;
                            let qy = p.y + z0(p.vy) * tE;
                            let u0 = if sl > 1e-12 { ((qx - Pp.x) * bvx + (qy - Pp.y) * bvy) / sl } else { 1.0 };
                            let uu = if u0 < 0.0 { 0.0 } else if u0 > 1.0 { 1.0 } else { u0 };
                            ddB = hypot(Pp.x + bvx * uu - qx, Pp.y + bvy * uu - qy);
                            if (u0 < 0.0) != (t0 < 0.0) {
                                R = jmax(1.15, (Q.bek_body + jmin(A.dMax, jmax(0.0, tp + uu - A.rc) * A.dV)) * if u0 < 0.0 { Q.away_k } else { 1.0 });
                            }
                        }
                        let kayT = if A.hm > 0.0 {
                            let s0 = A.rc + 2.0;
                            s0 + (jmax(A.rc, gecik + Q.bek_har_t * A.hm) + jmax(2.0, A.G.as_ref().map_or(2.0, |G| self.dis.gecOf(G, cx, cy, Some(jmax(0.0, ddB - R))))) - s0) * jmin(1.0, A.hm / 0.5)
                        } else {
                            A.rc + 2.0
                        };
                        let kay = jmax(0.0, (if BSV { Tb - gecik } else { Tb }) - kayT) * A.v * A.kay;
                        zz = (R + kay - ddB) / BKS;
                        KF[j] = Some(bek_uc(ddB - kay, R));
                    } else {
                        let t = if A.rakip && A.kov == Some(false) && !gon {
                            1e9
                        } else if A.rakip && A.kovT.map_or(false, |k| k > 0.0) && !gon {
                            gecik + A.kovT.unwrap() + jmax(0.0, hypot(A.kx - cx, A.ky - cy) - R) / A.v
                        } else if !A.rakip && gecik >= 1.0 {
                            let g = gecik;
                            if A.pG.is_none() {
                                let mut pg = (*A.p).clone();
                                pg.x = A.p.x + z0(A.p.vx) * g;
                                pg.y = A.p.y + z0(A.p.vy) * g;
                                A.pG = Some(pg);
                            }
                            let pG = A.pG.as_ref().unwrap();
                            let dG = hypot(cx - pG.x, cy - pG.y);
                            g + jmax(A.rc + A.G.as_ref().map_or(0.0, |G| self.dis.gecOf(G, cx, cy, Some(dG - R))) + jmax(0.0, dG - R) / A.v, self.frenT(pG, cx, cy, R, Tb - g - A.rc, A.v))
                        } else {
                            jmax(A.rc + A.G.as_ref().map_or(0.0, |G| self.dis.gecOf(G, cx, cy, Some(dd - R))) + jmax(0.0, dd - R) / A.v, if A.rakip { 0.0 } else { self.frenT(&A.p, cx, cy, R, Tb - A.rc, A.v) })
                        };
                        zz = (Tb - t - if !A.rakip { rcvB } else { 0.0 }) / A.tau;
                        let p: &Oyuncu = &A.p;
                        if OK1 {
                            RR[j] = R;
                            XM[j] = Some(jmax(0.0, jmin(if t0 >= 0.0 && sl > 1e-12 { ((p.x - Pp.x) * bvy - (p.y - Pp.y) * bvx).abs() / sl.sqrt() } else { dd }, R) - jmax(0.0, Tb - t) * A.v) / R);
                        }
                        if A.rakip {
                            let KS = A.kov == Some(false) && !gon && GR.is_some();
                            let HY = if KS { GR.as_ref().unwrap().yasH } else { 0.0 };
                            let tc = if KS {
                                if Tb < HY {
                                    Tb - Tb * Tb / (2.0 * HY)
                                } else {
                                    HY / 2.0
                                }
                            } else {
                                jmin(Tb, A.rc)
                            };
                            let KJy = KS && A.kt.is_none() && A.kj.is_some();
                            let KTy = KS && A.kt.is_some();
                            let kL = if KTy { hypot(A.kt.unwrap().x - A.kx, A.kt.unwrap().y - A.ky) } else { 0.0 };
                            let kS = if KTy { jmin(kL, jmax(0.0, Tb - if A.ktT < -1e8 { 0.0 } else { jmax(A.rc, gecik + A.ktT) }) * A.v) } else { 0.0 };
                            let (qx, qy) = if KJy {
                                self.kosYer(p, &A.kj.unwrap(), Tb)
                            } else if KTy {
                                let kt = A.kt.unwrap();
                                (A.kx + if kL > 1e-6 { (kt.x - A.kx) / kL * kS } else { 0.0 }, A.ky + if kL > 1e-6 { (kt.y - A.ky) / kL * kS } else { 0.0 })
                            } else {
                                (p.x + z0(p.vx) * tc, p.y + z0(p.vy) * tc)
                            };
                            let u0 = if sl > 1e-12 { ((qx - Pp.x) * bvx + (qy - Pp.y) * bvy) / sl } else { 1.0 };
                            let uu = if u0 < 0.0 { 0.0 } else if u0 > 1.0 { 1.0 } else { u0 };
                            let d2 = hypot(Pp.x + bvx * uu - qx, Pp.y + bvy * uu - qy);
                            zz = jmax(zz, (R - d2) / BKS);
                            if OK1 {
                                XM[j] = Some(jmin(XM[j].unwrap(), jmin(d2, R) / R));
                            }
                        }
                    }
                    if !(A.rakip || A.bek || qqI.is_none() || A.p.id == src[qqI.unwrap()].id || gon) {
                        zz = jmin(zz, (R - dd) / BKS);
                    }
                    let f;
                    if zz > A.M {
                        let g = sgm(zz);
                        f = if A.Gk < 1.0 { jmax(0.0, g - A.Gk) / (1.0 - A.Gk) } else { 0.0 };
                        A.M = zz;
                        A.Gk = g;
                    } else {
                        f = 0.0;
                    }
                    if !(f < 0.002) {
                        F[j] = f;
                        fs += f;
                    }
                }
                if fs <= 0.0 {
                    i += 1;
                    continue;
                }
                let sc = if fs > 1.0 { 1.0 / fs } else { 1.0 };
                let mut cik = 0.0;
                let mut bb = Top::yeni(P.x, P.y, bvx, bvy, team as u8, Some(b0.ch), 0.0);
                bb.x = P.x;
                bb.y = P.y;
                bb.vx = bvx;
                bb.vy = bvy;
                bb.ch = jmax(0.0, P.e);
                bb.alive = P.e > 0.0;
                bb.tp = Some(tp);
                bb.t = fi;
                let mut tt = TopTemas::default();
                tt.gon = gon;
                let okuB = OK1 && son == 0;
                if okuB {
                    tt.kt = Some(0.0);
                    tt.cue = cue1;
                    tt.k_bek = true;
                    tt.oku = std::mem::take(&mut okc);
                }
                let mut bL = TopT { b: bb, t: tt };
                let from = if okuB { hfrom } else { None };
                for j in 0..na {
                    if F[j] == 0.0 {
                        continue;
                    }
                    let dP = jmax(0.0, S - ad[j].tc) * F[j] * sc;
                    let (gvNow, yv, bek, rakip) = (ad[j].gvNow, ad[j].yv, ad[j].bek, ad[j].rakip);
                    let pT: Option<(f64, f64)> = if !OK1 || yv || gvNow || bek {
                        None
                    } else if rakip {
                        self.akHiz(&ad[j].p, &ad[j], bvx, bvy, CX[j], CY[j], IN.v_ak)
                    } else {
                        Some(self.alHiz(&ad[j].p, &ad[j], DD[j], RR[j], CX[j], CY[j], bvx, bvy, Tb, IN.v_set))
                    };
                    let Pc = if gvNow {
                        0.0
                    } else {
                        ad[j].ucX = if OK1 && !bek { XM[j] } else { None };
                        let mut pp = (*ad[j].p).clone();
                        if let Some((vx, vy)) = pT {
                            pp.vx = vx;
                            pp.vy = vy;
                        }
                        ctrl_p_al(&mut bL, &pp, from, src, ad[j].ucX, &kr)
                    };
                    ad[j].pT = pT;
                    let p: &Oyuncu = &ad[j].p;
                    let pdx = atan2(p.y - CY[j], p.x - CX[j]);
                    if !rakip && !yv {
                        let rk;
                        let mut rkX = P.x;
                        let mut rkY = P.y;
                        if OK1 && Pc < 0.99 {
                            let rq = if SKi[j].is_some() && (CX[j] - SKx[j]).abs() + (CY[j] - SKy[j]).abs() < 3.0 {
                                SK[j]
                            } else {
                                SKi[j] = Some(fi);
                                SKx[j] = CX[j];
                                SKy[j] = CY[j];
                                SKa[j] = pdx;
                                SK[j] = self.sekYaris(src, team, &ad[j], &mut bL, from, CX[j], CY[j], Tb, Some(P.e), hKI, gecik, false, &etP, kc);
                                SK[j]
                            };
                            rk = jmin(1.0, rq.v[0] + rq.v[1]);
                            if rq.n > 4 {
                                rkX = rq.v[4];
                                rkY = rq.v[5];
                            }
                        } else {
                            rk = if OK1 { self.kacanAl(&ad[j].p, &ad, nk(P.x, P.y), bvx, bvy, Tb) } else { 0.4 };
                        }
                        let mm = dP * (Pc + (1.0 - Pc) * rk);
                        let oo = dP * (1.0 - Pc) * (1.0 - rk);
                        m1 += mm;
                        o1 += oo;
                        mx1 += mm * P.x;
                        my1 += mm * P.y;
                        ox1 += oo * rkX;
                        oy1 += oo * rkY;
                        let BENF = ad[j].ben && !(qqI.is_some() && ad[j].pi == qqI);
                        if BENF || !yv {
                            let kc_ = if BENF { dP * (Pc + (1.0 - Pc) * rk) } else { dP * (1.0 - Pc) * rk };
                            s1 += kc_;
                            sx1 += kc_ * rkX;
                            sy1 += kc_ * rkY;
                            d1 += kc_;
                            dx1 += kc_ * CX[j];
                            dy1 += kc_ * CY[j];
                        }
                        cik += dP;
                        continue;
                    }
                    let th = if DD[j] < BODY_R || (OK1 && rakip && !bek && XM[j].map_or(false, |x| x * RR[j] < BODY_R)) {
                        0.0
                    } else {
                        (1.0 - Pc) * thru_p(&bL.b, p) * if bek { KF[j].unwrap_or(1.0) } else { 1.0 }
                    };
                    let rest = jmax(0.0, 1.0 - Pc - th);
                    let yeni = gvNow || SKi[j].is_none() || !((CX[j] - SKx[j]).abs() + (CY[j] - SKy[j]).abs() < 3.0) || (OK1 && (pdx - SKa[j]).abs() > IN.sk_aci);
                    let rq: Sek = if yeni {
                        if dP * rest < KUC {
                            if SKi[j].is_none() || gvNow {
                                Sek { v: [self.rbAt(src, team, nk(P.x, P.y)), 0.0, 0.0, 0.0, 0.0, 0.0], n: 2 }
                            } else {
                                SK[j]
                            }
                        } else {
                            SKi[j] = Some(fi);
                            SKx[j] = CX[j];
                            SKy[j] = CY[j];
                            SKa[j] = pdx;
                            SK[j] = self.sekYaris(src, team, &ad[j], &mut bL, from, CX[j], CY[j], Tb, Some(P.e), hKI, gecik, false, &etP, kc);
                            SK[j]
                        }
                    } else {
                        SK[j]
                    };
                    let rb = rq.v[0];
                    let mm = dP * rest * rb;
                    let oo = dP * (Pc + rest * (1.0 - rb - rq.v[1]));
                    let SY = rq.n > 2;
                    let ooS = if SY { dP * rest * jmax(0.0, 1.0 - rb - rq.v[1]) } else { 0.0 };
                    m1 += mm;
                    o1 += oo;
                    mx1 += mm * if SY { rq.v[2] } else { P.x };
                    my1 += mm * if SY { rq.v[3] } else { P.y };
                    s1 += mm;
                    sx1 += mm * if SY { rq.v[2] } else { P.x };
                    sy1 += mm * if SY { rq.v[3] } else { P.y };
                    if !yv {
                        d1 += mm;
                        dx1 += mm * CX[j];
                        dy1 += mm * CY[j];
                    }
                    ox1 += (oo - ooS) * P.x + if SY { ooS * rq.v[4] } else { 0.0 };
                    oy1 += (oo - ooS) * P.y + if SY { ooS * rq.v[5] } else { 0.0 };
                    let rt: Option<Sek> = if th > 1e-6 {
                        if !gvNow && SZi[j].is_some() && (CX[j] - SZx[j]).abs() + (CY[j] - SZy[j]).abs() < 3.0 && !(OK1 && (pdx - SZa[j]).abs() > IN.sk_aci) {
                            Some(SZ[j])
                        } else if dP * th < KUC {
                            if SZi[j].is_none() || gvNow {
                                None
                            } else {
                                Some(SZ[j])
                            }
                        } else {
                            SZi[j] = Some(fi);
                            SZx[j] = CX[j];
                            SZy[j] = CY[j];
                            SZa[j] = pdx;
                            SZ[j] = self.sekYaris(src, team, &ad[j], &mut bL, from, CX[j], CY[j], Tb, Some(P.e), hKI, gecik, true, &etP, kc);
                            Some(SZ[j])
                        }
                    } else {
                        None
                    };
                    match rt {
                        Some(rt) => {
                            let a0 = dP * th * rt.v[0];
                            let a1 = dP * th * rt.v[1];
                            let a2 = dP * th * jmax(0.0, 1.0 - rt.v[0] - rt.v[1]);
                            if bek {
                                k1 += a1;
                            }
                            m1 += a0;
                            o1 += a2;
                            mx1 += a0 * rt.v[2];
                            my1 += a0 * rt.v[3];
                            s1 += a0;
                            sx1 += a0 * rt.v[2];
                            sy1 += a0 * rt.v[3];
                            ox1 += a2 * rt.v[4];
                            oy1 += a2 * rt.v[5];
                            cik += dP;
                        }
                        None => {
                            cik += dP * (1.0 - th);
                            ad[j].tc += dP * th;
                        }
                    }
                }
                if okuB {
                    okc = std::mem::take(&mut bL.t.oku);
                }
                S -= cik;
                i += 1;
            }
            if S > 0.003 {
                let Pn = pts[n - 1];
                match end.done {
                    Some(d) if d == Bitis::KuyuSag || d == Bitis::KuyuSol => {
                        if (d == Bitis::KuyuSag) == (team == 0.0) {
                            k1 += S;
                        } else {
                            o1 += S;
                            ox1 += S * Pn.x;
                            oy1 += S * Pn.y;
                        }
                    }
                    _ => {
                        let (mut tm, mut tO, mut tB): (f64, f64, f64) = (1e9, 1e9, 1e9);
                        let mut bk: Option<&Oyuncu> = None;
                        for p in src {
                            if p.noTouch {
                                continue;
                            }
                            let a = self.arr(p, Pn.x, Pn.y, None);
                            let tt = if !p.bekci() && !kuyu_girer(p, Pn.x, Pn.y, a, false, &kr) { 1e9 } else { a };
                            if p.team == team {
                                tm = jmin(tm, tt);
                            } else if p.bekci() {
                                tB = tt;
                                bk = Some(p);
                            } else {
                                tO = jmin(tO, tt + 0.5 * jmax(0.0, self.TEPKI(p, Tepki::Kovala)));
                            }
                        }
                        if let Some(bk) = bk {
                            if self.bekCikar(bk, Pn.x, Pn.y, tB, tm) {
                                tO = jmin(tO, tB);
                            }
                        }
                        let sv = sgm((tO - tm) / 12.0);
                        let mm = S * sv;
                        let oo = S * (1.0 - sv);
                        m1 += mm;
                        o1 += oo;
                        mx1 += mm * Pn.x;
                        my1 += mm * Pn.y;
                        s1 += mm;
                        sx1 += mm * Pn.x;
                        sy1 += mm * Pn.y;
                        ox1 += oo * Pn.x;
                        oy1 += oo * Pn.y;
                    }
                }
            }
            let tot = ya(m1 + o1 + k1, 1.0);
            Wt += w;
            m += w * m1;
            o += w * o1;
            k += w * k1;
            mx += w * mx1;
            my += w * my1;
            sk += w * s1;
            sx += w * sx1;
            sy += w * sy1;
            dk += w * d1;
            dxs += w * dx1;
            dys += w * dy1;
            ox += w * ox1;
            oy += w * oy1;
            ex += w * (mx1 + ox1) / tot;
            ey += w * (my1 + oy1) / tot;
            if !dogru0(mx1 + ox1) {
                let Pn = pts[n - 1];
                ex += w * Pn.x * (1.0 - (m1 + o1) / tot);
                ey += w * Pn.y * (1.0 - (m1 + o1) / tot);
            }
        }
        m /= Wt;
        o /= Wt;
        k /= Wt;
        let end = nk(ex / Wt, ey / Wt);
        let endM = if m > 1e-9 { nk(mx / Wt / m, my / Wt / m) } else { end };
        let endO = if o > 1e-9 { nk(ox / Wt / o, oy / Wt / o) } else { end };
        let mut Ps = sk / Wt;
        let mut endS = if sk > 1e-9 { nk(sx / sk, sy / sk) } else { end };
        let mD = m - Ps;
        let endD = if mD > 1e-9 { nk((mx - sx) / Wt / mD, (my - sy) / Wt / mD) } else { endM };
        if pre < 1.0 {
            m = pre * m + 0.35 * (1.0 - pre);
            o = pre * o + 0.65 * (1.0 - pre);
            k *= pre;
            let s2 = pre * Ps + 0.35 * (1.0 - pre);
            if s2 > 1e-9 {
                endS = nk((pre * Ps * endS.x + 0.35 * (1.0 - pre) * h.x) / s2, (pre * Ps * endS.y + 0.35 * (1.0 - pre) * h.y) / s2);
            }
            Ps = s2;
        }
        let Psek = dk / Wt * if pre < 1.0 { pre } else { 1.0 };
        let endSek = if dk > 1e-9 { nk(dxs / dk, dys / dk) } else { end };
        Ucus { P: m, Pk: k, Pl: o, end, endM, endO, Ps, endS, endD, Psek, endSek }
    }
}

#[inline(always)]
fn dogru0(v: f64) -> bool {
    v != 0.0 && v == v
}
