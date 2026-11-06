//! Uçuş (decide.js:1675–2228 ucusFormul, 2315–2326 kenarIm/imagine, 2411–2495 effFric/reachDist/solveV/
//! reachSpeed/reachTime/siddetFor, 2663–2785 launchFor/solveTo/ucusYol/ucusT/ucusSure/bankSure/bankFor).
#![allow(non_snake_case)]

use crate::kafa::gecici_dis1::KuralErisim as _;
use super::gecici_dis1::{bek_sav, predict_world};
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
use crate::top::{adim, Bitis, Top};
use crate::yetenek::mek;
use std::borrow::Cow;

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
    pub fn eff_fric(&self, src: &[Op], team: f64, ch: f64, pts: &[Nk]) -> f64 {
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
    pub fn reach_dist(&self, v0: f64, va: f64, fr: f64) -> f64 {
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
    pub fn solve_v(&self, L: f64, va: f64, fr: f64) -> (f64, bool) {
        let mut lo = va;
        let mut hi = self.D.vmax;
        if self.reach_dist(hi, va, fr) < L {
            return (hi, false);
        }
        for _ in 0..16 {
            let mid = (lo + hi) / 2.0;
            if self.reach_dist(mid, va, fr) < L {
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
    pub fn reach_speed(&self, h: &Cikis, dx: f64, dy: f64, L: f64, v0: f64, et: &[Etki], kc: Kc, ch: f64) -> f64 {
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
    pub fn reach_time(&self, h: &Cikis, dx: f64, dy: f64, L: f64, v0: f64, et: &[Etki], kc: Kc, ch: f64) -> f64 {
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
    pub fn siddet_for(&self, h: &Oyuncu, to: Nk, t_r: f64, src: &[Op], ch: f64, kc: Kc) -> f64 {
        let et = etkiler(src);
        let c = Cikis::oyuncu(h);
        let L = ya(hypot(to.x - h.x, to.y - h.y), 1.0);
        let dx = (to.x - h.x) / L;
        let dy = (to.y - h.y) / L;
        let vm = self.vmax_of(Some(h));
        if self.reach_time(&c, dx, dy, L, vm, &et, kc, ch) > t_r {
            return vm;
        }
        let mut lo = 0.2;
        let mut hi = vm;
        if self.reach_time(&c, dx, dy, L, lo, &et, kc, ch) <= t_r {
            return lo;
        }
        for _ in 0..8 {
            let mid = (lo + hi) / 2.0;
            if self.reach_time(&c, dx, dy, L, mid, &et, kc, ch) > t_r {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        hi
    }
    /// `to`'ya `arrive` hızıyla varan çıkış (decide.js:2663).
    pub fn launch_for(&self, h: &Oyuncu, to: Nk, arrive: f64, src: &[Op], ch: f64, kc: Kc) -> JOLaunch {
        let et = etkiler(src);
        let c = Cikis::oyuncu(h);
        let L = ya(hypot(to.x - h.x, to.y - h.y), 1.0);
        let dx = (to.x - h.x) / L;
        let dy = (to.y - h.y) / L;
        let vm = self.vmax_of(Some(h));
        if self.reach_speed(&c, dx, dy, L, vm, &et, kc, ch) < arrive {
            return JOLaunch { vx: dx * vm, vy: dy * vm, ok: false };
        }
        let mut lo = arrive;
        let mut hi = vm;
        for _ in 0..7 {
            let mid = (lo + hi) / 2.0;
            if self.reach_speed(&c, dx, dy, L, mid, &et, kc, ch) < arrive {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        JOLaunch { vx: dx * hi, vy: dy * hi, ok: true }
    }
    /// decide.js:2682 (yetişmezse None)
    pub fn solve_to(&self, h: &Cikis, to: Nk, arrive: f64, et: &[Etki], kc: Kc, ch: f64) -> Option<f64> {
        let L = ya(hypot(to.x - h.x, to.y - h.y), 1.0);
        let dx = (to.x - h.x) / L;
        let dy = (to.y - h.y) / L;
        let vm = self.vmax_of(h.vur);
        if self.reach_speed(h, dx, dy, L, vm, et, kc, ch) < arrive {
            return None;
        }
        let mut lo = arrive;
        let mut hi = vm;
        for _ in 0..7 {
            let mid = (lo + hi) / 2.0;
            if self.reach_speed(h, dx, dy, L, mid, et, kc, ch) < arrive {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        Some(hi)
    }
    /// Boş sahada L'lik pasın yolu (decide.js:2694; uzunluk ve varış hızına göre önbellekli).
    pub fn ucus_yol(&self, L: f64, arrive: f64) -> Arc<UcusYol> {
        let Li = jmax(1.0, jmin(80.0, jround(L))) as usize;
        let key = arrive.to_bits();
        if let Some(r) = self.ob.ucus.borrow().get(&key).and_then(|a| a.get(Li).cloned().flatten()) {
            return r;
        }
        let h = Cikis::yer(5.0, GCY, 0.0);
        let bos: Vec<Etki> = Vec::new();
        let v0 = self.solve_to(&h, nk(h.x + Li as f64, GCY), arrive, &bos, None, 0.5).unwrap_or(self.D.vmax);
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
        let mut m = self.ob.ucus.borrow_mut();
        let a = m.entry(key).or_default();
        if a.len() <= Li {
            a.resize(Li + 1, None);
        }
        a[Li] = Some(r.clone());
        r
    }
    /// Pasın L'lik (ya da `al`'a dek) yolu alma süresi (decide.js:2736).
    pub fn ucus_t(&self, L: f64, arrive: f64, al: Option<f64>) -> f64 {
        let r = self.ucus_yol(L, arrive);
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
    pub fn ucus_sure(&self, h: &Oyuncu, at: Nk, lv: Nk, src: &[Op], ch: f64, savT: Option<f64>, kc: Kc) -> f64 {
        let L = hypot(at.x - h.x, at.y - h.y);
        let v = ya(hypot(lv.x, lv.y), 1.0);
        let sv = yao(savT, 0.0);
        if L < 0.5 {
            return 4.0 + sv;
        }
        let et = etkiler(src);
        let t = self.reach_time(&Cikis::oyuncu(h), (at.x - h.x) / L, (at.y - h.y) / L, L, v, &et, kc, ch);
        (if t < 1e9 { t } else { L / jmax(0.5, 0.7 * v) }) + 4.0 + sv
    }
    /// Kenardan sektirme pasının süresi (decide.js:2749).
    #[allow(clippy::too_many_arguments)]
    pub fn bank_sure(&self, h: &Oyuncu, B: Nk, T: Nk, lv: Nk, src: &[Op], ch: f64, savT: Option<f64>, kc: Kc) -> f64 {
        let P = crate::saha::P;
        let et = etkiler(src);
        let c = Cikis::oyuncu(h);
        let l1 = ya(hypot(B.x - h.x, B.y - h.y), 1.0);
        let l2 = ya(hypot(T.x - B.x, T.y - B.y), 1.0);
        let v = ya(hypot(lv.x, lv.y), 1.0);
        let t1 = self.reach_time(&c, (B.x - h.x) / l1, (B.y - h.y) / l1, l1, v, &et, kc, ch);
        let v_b = self.reach_speed(&c, (B.x - h.x) / l1, (B.y - h.y) / l1, l1, v, &et, kc, ch) * P.rest;
        let t2 = if v_b > 0.0 { self.reach_time(&Cikis::yer(B.x, B.y, h.team), (T.x - B.x) / l2, (T.y - B.y) / l2, l2, v_b, &et, kc, ch) } else { 1e9 };
        if t1 < 1e9 && t2 < 1e9 {
            t1 + t2 + 4.0 + yao(savT, 0.0)
        } else {
            (l1 + l2) / jmax(0.5, 0.6 * v) + 4.0
        }
    }
    /// Kenardan sektirip q'ya `arrive` hızıyla varan çıkış (decide.js:2757).
    pub fn bank_for(&self, h: &Oyuncu, B: Nk, q: Nk, arrive: f64, src: &[Op], ch: f64, kc: Kc) -> JOLaunch {
        let P = crate::saha::P;
        let et = etkiler(src);
        let v2 = match self.solve_to(&Cikis::yer(B.x, B.y, h.team), q, arrive, &et, kc, ch) {
            Some(v) => v,
            None => return JOLaunch { vx: 0.0, vy: 0.0, ok: false },
        };
        let v_b = v2 / P.rest;
        let vm = self.vmax_of(Some(h));
        if v_b > vm {
            return JOLaunch { vx: 0.0, vy: 0.0, ok: false };
        }
        let v1 = self.solve_to(&Cikis::oyuncu(h), B, v_b, &et, kc, ch);
        let l1 = ya(hypot(B.x - h.x, B.y - h.y), 1.0);
        match v1 {
            None => JOLaunch { vx: (B.x - h.x) / l1 * vm, vy: (B.y - h.y) / l1 * vm, ok: false },
            Some(v1) => JOLaunch { vx: (B.x - h.x) / l1 * v1, vy: (B.y - h.y) / l1 * v1, ok: true },
        }
    }

    /// Kafadaki uçuş denemesi (decide.js:2324): bugünkü davranışta formül (ucusFormul).
    pub fn imagine(&mut self, h: &Oyuncu, src: &[Op], b0: UcusB0, q: Option<&Oyuncu>, to: Option<Nk>, kc: Kc) -> Ucus {
        self.ucus_formul(h, src, b0, q, to, kc)
    }
    /// Kenardan sektirme denemesi (decide.js:2315): B._kenar açıkken imagine.
    pub fn kenar_im(&mut self, h: &Oyuncu, src: &[Op], b0: UcusB0, q: Option<&Oyuncu>, to: Option<Nk>, kc: Kc) -> Ucus {
        let k0 = self.bag._kenar;
        self.bag._kenar = Some(true);
        let r = self.imagine(h, src, b0, q, to, kc);
        self.bag._kenar = k0;
        r
    }

    /// Topun uçuşunun formülü (decide.js:1675): her tikte her adayın topa yetişme olasılığı.
    pub fn ucus_formul(&mut self, h: &Oyuncu, src0: &[Op], b0in: UcusB0, q: Option<&Oyuncu>, to: Option<Nk>, kc0: Kc) -> Ucus {
        // D ve dış dünya yerel kopyadan okunur: keepP / predictWorld `&mut self` ister (kafa2'nin önbellekleri)
        let D0 = self.D.clone();
        let D = &D0;
        let B0 = self.bag.clone();
        let B = &B0;
        let team = h.team;
        let gon = B._gon.unwrap_or(false);
        let dis0 = self.dis.clone();
        let kr = KuralK(&*dis0);
        let S_ = self.dis.S();
        let gr = self.dis.gr();
        let tick = self.tick();
        let qq_i: Option<usize> = q.and_then(|q| bul_i(src0, q));
        let kendi = qq_i.map_or(false, |i| src0[i].id == h.id);
        let h_i0 = sira(src0, h);
        let lk = ya(B._ot.unwrap_or(0.0), 1.0) * 1.0;
        let tur = if gon { self.d_sut_tur() } else { None };
        let yf = self.hata_f(h, Some(src0), gon, tur.as_deref(), B._kenar.unwrap_or(false), lk);
        let mut gecik = B._savT.unwrap_or(0.0);
        let mut caR = B._sav_ca;
        let mut savW = B._sav_w;
        let mut savU = B._sav_u;
        if B._savT.is_none() && h.id >= 0.0 {
            let ks = B._kanca.as_ref().map(|k| k.s).filter(|s| *s != 0.0 && *s == *s);
            let pl = self.savur_plan(
                h,
                nk(b0in.vx, b0in.vy),
                self.ca_of(h),
                B._kanca.as_ref().map_or(0.0, |k| k.w0),
                ks.unwrap_or(yao(B._s_yon, 0.0)),
                if gon { None } else { Some(src0) },
                gon && B._itme.unwrap_or(false),
                self.vur_e(h, Some(b0in.ch), gon),
            );
            if let Some(pl) = pl {
                if pl.T >= 1.0 {
                    gecik = pl.T;
                }
                caR = Some(pl.caR);
                if savW.is_none() {
                    savW = Some(pl.w_rel);
                    savU = Some(pl.u_sh);
                }
            }
        }
        let gecik = jmax(0.0, jround(gecik));
        let bsv = gecik >= 1.0 && h.id >= 0.0;
        let ws: Cow<[Op]> = if bsv {
            Cow::Owned(src0.iter().map(|p| if p.team != team && p.bekci() { bek_sav(self, p, h, gecik, src0) } else { p.clone() }).collect())
        } else {
            Cow::Borrowed(src0)
        };
        let src: &[Op] = &ws;
        // JS: eşlenen dünya yeni dizidir (önbelleği yok)
        let kc: Kc = if bsv { None } else { kc0 };
        let h_i = h_i0;
        let ok1 = h.id >= 0.0;
        let cue1 = if ok1 { (if B._tek_d { IN.cue_ot } else { IN.cue0 }) + gecik } else { 0.0 };
        let u_m1 = (IN.u_a + IN.u_b) / 2.0;
        let giz1 = if ok1 { h.govde.yet.y(mek::VURUS_GIZLE) - 10.0 } else { 0.0 };
        let h_ic = h.id >= 0.0 && h_i.is_some();
        let h_ki: Option<usize> = if h.id >= 0.0 { bul_i(src, h) } else { None };
        let pre = if gecik >= 1.0 && h_ic { jmax(0.0, jmin(1.0, self.keep_p(&src[h_i.unwrap()], src, None, gecik))) } else { 1.0 };
        let p_k1 = self.pas_k(gon, b0in.vx, b0in.vy, Some(h));
        let Ah = (match savW {
            Some(w) => SAV.tau * w * yf.b + D.aim_err * p_k1 * z0(savU.unwrap_or(0.0)) * yf.y,
            None => D.aim_err * p_k1 * yf.y,
        }) / 10.0;
        let ayniyer = h.id >= 0.0 && b0in.x == h.x && b0in.y == h.y;
        let o0 = if ayniyer {
            match caR {
                Some(ca) => nk(h.x + z0(h.vx) * gecik * 0.5 + cos(ca) * Q.body, h.y + z0(h.vy) * gecik * 0.5 + sin(ca) * Q.body),
                None => {
                    let cs = self.core_side(h);
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
        let err = (20.0 - h.govde.yet.y(mek::ALGI_HATA)) * D.err_ok * D.err_pos;
        let mut ad: Vec<UA> = Vec::new();
        for (i, p) in src.iter().enumerate() {
            if p.noTouch && (p.team == team || !(p.hal.stun.map_or(false, |s| s > tick) || p.hal.adandi.map_or(false, |s| s > tick))) {
                continue;
            }
            let rakip = p.team != team;
            let ph = h_i == Some(i);
            if !rakip && (qq_i != Some(i)) && h.id >= 0.0 && !ph && h_ki != Some(i) {
                let L0 = ya(hypot(b0.vx, b0.vy), 1.0);
                let ux = b0.vx / L0;
                let uy = b0.vy / L0;
                let ax = p.x - o0.x;
                let ay = p.y - o0.y;
                let al = ax * ux + ay * uy;
                let pe = -ax * uy + ay * ux;
                let v_a = z0(p.vx) * ux + z0(p.vy) * uy;
                let v_l = -z0(p.vx) * uy + z0(p.vy) * ux;
                if al > -1.0 && (pe + 20.0 * v_l).abs() < 6.0 && !(p.bekci() && hypot(p.x - ox_of(p.team), p.y - GCY) < KR) {
                    let t_a = gecik + jmax(0.0, al) / L0;
                    let YT = self.dis.alan_yol_ver_tepki();
                    let tD = jmax(0.0, t_a - if YT { jmax(0.0, self.TEPKI(p, Tepki::Kovala)) } else { gecik });
                    let lt0 = pe + v_l * t_a;
                    let sg = if YT && lt0.abs() > 0.05 {
                        jsign(lt0)
                    } else if pe.abs() > 0.05 {
                        jsign(pe)
                    } else if v_l >= 0.0 {
                        1.0
                    } else {
                        -1.0
                    };
                    let a_a = al + v_a * t_a;
                    let lat0 = if p.bekci() { pe + v_l * t_a } else { pe + v_l * t_a + 0.5 * sg * 0.02 * tD * tD };
                    let lx0 = o0.x + ux * a_a - uy * lat0;
                    let ly0 = o0.y + uy * a_a + ux * lat0;
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
                        v.x = o0.x + ux * a_a - uy * lat;
                        v.y = o0.y + uy * a_a + ux * lat;
                        let mut A = UA::yeni(Cow::Owned(v), None);
                        A.yv = true;
                        A.gv_t = 1e9;
                        ad.push(A);
                    }
                }
                continue;
            }
            let BEN = ph && !kendi && !gon;
            if !(rakip || qq_i == Some(i) || BEN) {
                continue;
            }
            if ph && !kendi && !BEN {
                continue;
            }
            if gon && !rakip {
                continue;
            }
            let v = self.v_s(p);
            let dK = self.dis.alan_denge_kayip(p, tick);
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
            // ALAN_BEK_DALIS_IZ: maçın dalış modeli (temas::bek_dalis): hareket cezası yok, Bekçi tepki süresince
            // kendi hareketini sürdürür (aşağıda yeri hızıyla ilerletilir)
            let hm_a = if bek && !bek_dalis_k() { bek_har(p, o0.x, o0.y, b0.vx, b0.vy, &kr) } else { 0.0 };
            let f_o = if ok1 && rakip && !bek { jmax(0.05, jmin(0.95, 0.5 + IN.sez * (p.govde.yet.y(mek::VURUS_SEZ) - 10.0) - IN.giz * giz1)) } else { 0.0 };
            let on_e = cue1 * f_o * u_m1;
            let sd_w = if f_o != 0.0 { hypot(rc * IN.tepki_z / 2.449, cue1 * f_o * (IN.u_b - IN.u_a) / 3.464) / 1.814 } else { 0.0 };
            let mut A = UA::yeni(Cow::Borrowed(&**p), Some(i));
            A.G = self.dis.gec_tab(p, false);
            A.v = v;
            A.rc = dK + (if f_o != 0.0 { gecik + jmax(0.0, rc - on_e) } else if bek && gon && ok1 { rc - bek_oku_e(p, h, cue1) } else { rc }) + if bek { Q.bek_har_t * hm_a } else { 0.0 };
            A.d_max = bek_dl(p) * (1.0 - Q.bek_har_d * hm_a);
            A.hm = hm_a;
            A.rakip = rakip;
            A.bek = bek;
            A.R0 = self.menzil(p);
            A.r_r = self.TEPKI(p, Tepki::Uzanma) - on_e;
            A.r_r0 = self.TEPKI(p, Tepki::Uzanma);
            A.rc_y = if f_o != 0.0 { Some(gecik + jmax(1.0, self.TEPKI(p, Tepki::YeniTop)) - on_e) } else { None };
            A.d_v = bek_dv(p);
            A.kay = bek_kay(p);
            let e_j = match (rakip, self.dis.algi_e(p), &gr) {
                (true, Some(e), Some(_)) => e + self.dis.gr_yas_hata(self.dis.algi_yas(p).unwrap_or(f64::NAN)),
                _ => err,
            };
            A.tau = if sd_w != 0.0 { hypot(1.5 + e_j / v * 0.5, sd_w) } else { 1.5 + if rakip { e_j / v * 0.5 } else { 0.0 } };
            A.ben = ph;
            A.bk0 = p.bekci();
            A.gv_t = if rakip && !bek && h.id >= 0.0 { jmax(jmax(0.0, z0(p.hal.adandi.unwrap_or(0.0)) - tick), z0(p.hal.stun.unwrap_or(0.0)) - tick) } else { 0.0 };
            ad.push(A);
        }
        let mut wt = 0.0;
        let (mut m, mut o, mut k, mut ex, mut ey, mut mx, mut my, mut ox, mut oy, mut sk, mut sx, mut sy, mut dk, mut dxs, mut dys) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let r#as = if h.id < 0.0 { 0.0 } else { yf.v * D.v0_err / 10.0 * self.pas_kv(gon) };
        let ucs: Vec<[f64; 3]> = match if gon { self.sut_uc(team, o0, (b0.vx, b0.vy), Ah) } else { None } {
            Some(u) => u,
            None => {
                if r#as > 1e-9 {
                    UC5.to_vec()
                } else {
                    UC3.to_vec()
                }
            }
        };
        let na = ad.len();
        let mut KOP: Vec<Option<Oyuncu>> = vec![None; na];
        let mut kop2: Vec<Option<Oyuncu>> = vec![None; na];
        let mut SK: Vec<Sek> = vec![Sek { v: [0.0; 6], n: 0 }; na];
        let mut ska = vec![0.0; na];
        let mut ski: Vec<Option<f64>> = vec![None; na];
        let mut skx = vec![0.0; na];
        let mut sky = vec![0.0; na];
        let mut sz: Vec<Sek> = vec![Sek { v: [0.0; 6], n: 0 }; na];
        let mut sza = vec![0.0; na];
        let mut szi: Vec<Option<f64>> = vec![None; na];
        let mut szx = vec![0.0; na];
        let mut szy = vec![0.0; na];
        let kuc = 0.002;
        let et_p = etkiler(src);
        let mut ucs_y: Vec<(f64, f64, f64, bool)> = ucs.iter().map(|u| (u[0], u[1], u[2], false)).collect();
        if !gon && h.id >= 0.0 {
            let B = to.or_else(|| qq_i.map(|i| nk(src[i].x, src[i].y)));
            if let Some(B) = B {
                let L = ya(hypot(B.x - o0.x, B.y - o0.y), 1.0);
                let v0 = ya(hypot(b0.vx, b0.vy), 1.0);
                let t1 = self.reach_time(&Cikis::yer(o0.x, o0.y, team), (B.x - o0.x) / L, (B.y - o0.y) / L, L, v0, &et_p, kc, b0.ch);
                let Tf = jmax(1.0, jround(gecik + if t1 < 1e9 { t1 } else { L / jmax(0.5, 0.7 * v0) }));
                let Pw = predict_world(self, src, team, B, Tf, false, true);
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
                    ucs_y = ucs.iter().flat_map(|u| [(u[0], u[1] / 2.0, u[2], false), (u[0], u[1] / 2.0, u[2], true)]).collect();
                }
            }
        }
        let mut okc: crate::HizliMap<i64, (f64, f64)> = Default::default();
        let hfrom: Option<&Oyuncu> = Some(h);
        // ALAN_UCUS_IZ=<alıcı>: o alıcıya pasın top yolu boyunca kim ne olasılıkla alıyor (yalnız en dış benzetim, ilk yol)
        let ucus_iz = crate::iz_acik() && ((!gon && qq_i.map_or(false, |i| std::env::var("ALAN_UCUS_IZ").ok().as_deref() == Some(&*src[i].govde.name)))
            || (gon && std::env::var("ALAN_UCUS_SUT_IZ").ok().as_deref() == Some(&*h.govde.name)));
        let _ucd = UcIzDer::gir();
        let mut ucus_iz = ucus_iz;
        if ucus_iz {
            eprintln!("UCUS {} {} -> {} hedef {:?} hız ({:.2},{:.2}) gecik {} çıkış ({:.1},{:.1})", self.tick(), h.govde.name, qq_i.map_or("ŞUT".to_string(), |i| src[i].govde.name.to_string()), to.map(|t| (t.x, t.y)), b0.vx, b0.vy, gecik, o0.x, o0.y);
            for p in src.iter().filter(|p| p.bekci() && p.team != team) {
                eprintln!("UCUS   bekçi {} ({:.1},{:.1}) hız ({:.2},{:.2})", p.govde.name, p.x, p.y, z0(p.vx), z0(p.vy));
            }
        }
        let mut uc_yol = 0;
        let sut_hep = gon && std::env::var("ALAN_UCUS_SUT_IZ").is_ok();
        let alis_kume = alis_kume();
        let (mut km_a, mut km_x, mut km_y) = (0.0f64, 0.0f64, 0.0f64);
        for &(z, w, zs, yol) in &ucs_y {
            uc_yol += 1;
            if uc_yol > 1 && !sut_hep { ucus_iz = false; }
            let an = z * Ah;
            let c = cos(an);
            let s = sin(an);
            let kv = 1.0 + z0(zs) * r#as;
            let b = top_yap(o0.x, o0.y, (b0.vx * c - b0.vy * s) * kv, (b0.vx * s + b0.vy * c) * kv, team, Some(b0.ch), Some(h));
            let (pts, end) = tahmin_ob(&self.ob, &b, &et_p, kc, D.sim_max as usize);
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
                    let RB = z0(S.rcv_buf);
                    let HY = gr.as_ref().map_or(40.0, |g| g.yas_h);
                    let is_p = |A: &UA, T: f64| -> (f64, f64) {
                        match &A.kj {
                            Some(kj) => self.kos_yer(&A.p, kj, T),
                            None => {
                                let f = if T < HY { T - T * T / (2.0 * HY) } else { HY / 2.0 };
                                (A.p.x + z0(A.p.vx) * f, A.p.y + z0(A.p.vy) * f)
                            }
                        }
                    };
                    let var_e = |A: &UA, c: f64, px: f64, py: f64, rc0: f64| -> f64 {
                        let buf = if A.rakip { 0.0 } else { RB };
                        let mut i = c as usize;
                        while i < n {
                            let fi = i as f64;
                            let q = pts[i];
                            let d = hypot(q.x - px, q.y - py);
                            let dr = d - if A.rakip { jmax(1.15, jmin(A.R0, Q.reach_body + jmax(0.0, fi - A.r_r) * Q.reach_grow)) } else { A.R0 };
                            let t0v = c + rc0 + jmax(0.0, dr) / A.v;
                            if t0v + buf > fi {
                                i += 2;
                                continue;
                            }
                            let t = jmax(t0v + A.G.as_ref().map_or(0.0, |G| self.dis.gec_of(G, q.x, q.y, Some(dr))), if A.rakip { 0.0 } else { self.fren_t(&A.p, q.x, q.y, A.R0, fi - rc0, A.v) }) + buf;
                            if t <= fi {
                                return fi;
                            }
                            i += 2;
                        }
                        1e9
                    };
                    let mut o_ek: f64 = 1e9;
                    for A in ad.iter_mut() {
                        A.kov = Some(true);
                        A.kov_t = Some(0.0);
                    }
                    for A in ad.iter() {
                        if !A.rakip && qq_i.is_some() && A.pi == qq_i && !A.yv {
                            o_ek = var_e(A, 0.0, A.p.x, A.p.y, 0.0);
                        }
                    }
                    let RV: Vec<usize> = (0..na).filter(|&j| ad[j].rakip && !ad[j].bek).collect();
                    for &j in &RV {
                        let A = &mut ad[j];
                        A.kov = Some(false);
                        A.kov_t = None;
                        A.kt = None;
                        A.kj = None;
                        let p: &Oyuncu = &A.p;
                        let tq = p.hal.tutQ.and_then(|t| src.get(t));
                        let tq_ok = match (tq, qq_i) {
                            (Some(tq), Some(qi)) => tq.govde.name == src[qi].govde.name && tq.team == src[qi].team && o_ek < n as f64,
                            _ => false,
                        };
                        let kj = if tq_ok {
                            let P0 = pts[o_ek as usize];
                            Some(Kj { t: self.dis.gside(nk(P0.x, P0.y), p.team, ya(S.mark_off.unwrap_or(0.0), 2.5)), r: jmax(0.0, A.rc), v: A.v })
                        } else if yol && A.kj2t.is_some() {
                            Some(Kj { t: A.kj2t.unwrap(), r: 0.0, v: self.spd(p) })
                        } else {
                            Some(Kj { t: nk(p.tx, p.ty), r: 0.0, v: if p.sprint { A.v } else { self.spd(p) } })
                        };
                        A.kj = kj;
                    }
                    let kural = |A: &UA, e: f64, b_s: f64| e <= b_s + (20.0 - A.p.govde.yet.y(mek::ALGI_HATA)) * S.margin_t + 1.0 && !(e > o_ek + 6.0) && !(e > o_ek + 1.0);
                    let mut c = 0.0;
                    while c < n as f64 && c <= o_ek {
                        let mut b_s: f64 = 1e9;
                        for &j in &RV {
                            if ad[j].kov_t.is_some() {
                                let e = ad[j].est;
                                if e < b_s && !(e < c) {
                                    b_s = e;
                                }
                                continue;
                            }
                            let (px, py) = is_p(&ad[j], gecik + c);
                            ad[j].c = (px, py);
                            let mut Gy: Option<crate::kafa::gecici_dis1::GecTab> = None;
                            ad[j].Gy = None;
                            if ad[j].kj.is_some() && ad[j].G.is_some() && gecik + c > 0.0 {
                                let kj = ad[j].kj.unwrap();
                                let (wx, wy) = self.kos_hiz(&ad[j].p, &kj, gecik + c);
                                let oo = kop2[j].get_or_insert_with(|| (*ad[j].p).clone());
                                oo.x = px;
                                oo.y = py;
                                oo.vx = wx;
                                oo.vy = wy;
                                Gy = self.dis.gec_tab(oo, true);
                                let A = &mut ad[j];
                                if A.G0.is_none() {
                                    A.G0 = Some(A.G.clone());
                                }
                                A.gkk = A.G.clone();
                                A.G = Gy.clone();
                            }
                            let ec = var_e(&ad[j], c, px, py, jmax(0.0, ad[j].rc - gecik - c));
                            let A = &mut ad[j];
                            A.ec = ec;
                            if Gy.is_some() {
                                A.Gy = Gy;
                                A.G = A.gkk.clone();
                            }
                            if A.ec < b_s {
                                b_s = A.ec;
                            }
                        }
                        let mut hepsi = true;
                        for &j in &RV {
                            if ad[j].kov_t.is_some() {
                                continue;
                            }
                            let ec = ad[j].ec;
                            if kural(&ad[j], ec, b_s) {
                                let A = &mut ad[j];
                                A.kov = Some(true);
                                A.kov_t = Some(c);
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
                                let ah = ad[j].p.govde.yet.y(mek::ALGI_HATA);
                                if ad[j].kt.is_none() && o_ek < n as f64 && ec <= b_s + (20.0 - ah) * S.margin_t + 1.0 && !(ec > o_ek + 6.0) && ec > o_ek + 1.0 {
                                    let P0 = pts[o_ek as usize];
                                    let t = self.dis.gside(nk(P0.x, P0.y), ad[j].p.team, 2.2);
                                    let A = &mut ad[j];
                                    A.kt = Some(t);
                                    A.kt_t = c;
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
            let rcv_b = S_.as_ref().map_or(0.0, |s| z0(s.rcv_buf));
            let mut DD = vec![0.0; na];
            let mut xm: Vec<Option<f64>> = vec![None; na];
            let mut RR = vec![0.0; na];
            let mut KF: Vec<Option<f64>> = vec![None; na];
            let mut CX = vec![0.0; na];
            let mut CY = vec![0.0; na];
            let mut S = 1.0;
            // ALAN_ALIS_KUME_IZ: bu yolda alıcının doğrudan alışları (kütle, yer) sırayla
            let mut dAl: Vec<(f64, f64, f64)> = Vec::new();
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
                    // ALAN_GOVDE_YOL_IZ: yoldaki gövde (alıcı, Bekçi ve pası atan dışında herkes) topa çarpar: tepkiden önce saf çarpma
                    let gyol = ok1 && !A.bek && dd < BODY_R && A.p.id != h.id && !qq_i.map_or(false, |qi| A.p.id == src[qi].id && A.p.team == src[qi].team) && crate::kafa::kafa2_tarti::govde_yol();
                    A.gv_now = A.gv_t > Tb || (gyol && Tb <= A.rc);
                    if A.gv_now {
                        if dd < BODY_R {
                            F[j] = 1.0;
                            fs += 1.0;
                        }
                        continue;
                    }
                    if A.yv {
                        continue;
                    }
                    let tp_e = tp + tt;
                    let mut R = if A.bek {
                        Q.bek_body + jmin(A.d_max, jmax(0.0, tp_e - A.rc) * A.d_v)
                    } else if A.rakip {
                        jmin(A.R0, Q.reach_body + jmax(0.0, tp_e - if son == 0 { A.r_r } else { A.r_r0 }) * Q.reach_grow)
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
                        let mut dd_b = dd;
                        if bek_dalis_k() {
                            let tE = jmin(Tb, jmax(0.0, A.rc));
                            let qx = p.x + z0(p.vx) * tE;
                            let qy = p.y + z0(p.vy) * tE;
                            let u0 = if sl > 1e-12 { ((qx - Pp.x) * bvx + (qy - Pp.y) * bvy) / sl } else { 1.0 };
                            let uu = if u0 < 0.0 { 0.0 } else if u0 > 1.0 { 1.0 } else { u0 };
                            dd_b = hypot(Pp.x + bvx * uu - qx, Pp.y + bvy * uu - qy);
                        }
                        if A.hm > 0.0 {
                            let tE = if bsv { jmax(0.0, jmin(Tb - gecik, Q.bek_har_t * A.hm)) } else { jmin(Tb, gecik + Q.bek_har_t * A.hm) };
                            let qx = p.x + z0(p.vx) * tE;
                            let qy = p.y + z0(p.vy) * tE;
                            let u0 = if sl > 1e-12 { ((qx - Pp.x) * bvx + (qy - Pp.y) * bvy) / sl } else { 1.0 };
                            let uu = if u0 < 0.0 { 0.0 } else if u0 > 1.0 { 1.0 } else { u0 };
                            dd_b = hypot(Pp.x + bvx * uu - qx, Pp.y + bvy * uu - qy);
                            if (u0 < 0.0) != (t0 < 0.0) {
                                R = jmax(1.15, (Q.bek_body + jmin(A.d_max, jmax(0.0, tp + uu - A.rc) * A.d_v)) * if u0 < 0.0 { Q.away_k } else { 1.0 });
                            }
                        }
                        let kay_t = if A.hm > 0.0 {
                            let s0 = A.rc + 2.0;
                            s0 + (jmax(A.rc, gecik + Q.bek_har_t * A.hm) + jmax(2.0, A.G.as_ref().map_or(2.0, |G| self.dis.gec_of(G, cx, cy, Some(jmax(0.0, dd_b - R))))) - s0) * jmin(1.0, A.hm / 0.5)
                        } else {
                            A.rc + 2.0
                        };
                        let kay = jmax(0.0, (if bsv { Tb - gecik } else { Tb }) - kay_t) * A.v * A.kay;
                        zz = (R + kay - dd_b) / BKS;
                        KF[j] = Some(bek_uc(dd_b - kay, R));
                        if ucus_iz && UC_IZ_DER.with(|c| c.get()) == 1 && uc_yol == 2 {
                            eprintln!("UCUS   ~bek i {} {} top ({:.1},{:.1}) dd {:.2} R {:.2} kay {:.2} zz {:.2} | rc {:.1} dMax {:.2} dV {:.3} hm {:.2} tpE {:.1} kayT {:.1} v {:.3} kayK {:.3}", i, A.p.govde.name, P.x, P.y, dd_b, R, kay, zz, A.rc, A.d_max, A.d_v, A.hm, tp_e, kay_t, A.v, A.kay);
                        }
                    } else {
                        let t = if A.rakip && A.kov == Some(false) && !gon {
                            1e9
                        } else if A.rakip && A.kov_t.map_or(false, |k| k > 0.0) && !gon {
                            gecik + A.kov_t.unwrap() + jmax(0.0, hypot(A.kx - cx, A.ky - cy) - R) / A.v
                        } else if !A.rakip && gecik >= 1.0 {
                            let g = gecik;
                            if A.p_g.is_none() {
                                let mut pg = (*A.p).clone();
                                pg.x = A.p.x + z0(A.p.vx) * g;
                                pg.y = A.p.y + z0(A.p.vy) * g;
                                A.p_g = Some(pg);
                            }
                            let p_g = A.p_g.as_ref().unwrap();
                            let dG = hypot(cx - p_g.x, cy - p_g.y);
                            g + jmax(A.rc + A.G.as_ref().map_or(0.0, |G| self.dis.gec_of(G, cx, cy, Some(dG - R))) + jmax(0.0, dG - R) / A.v, self.fren_t(p_g, cx, cy, R, Tb - g - A.rc, A.v))
                        } else {
                            jmax(A.rc + A.G.as_ref().map_or(0.0, |G| self.dis.gec_of(G, cx, cy, Some(dd - R))) + jmax(0.0, dd - R) / A.v, if A.rakip { 0.0 } else { self.fren_t(&A.p, cx, cy, R, Tb - A.rc, A.v) })
                        };
                        zz = (Tb - t - if !A.rakip { rcv_b } else { 0.0 }) / A.tau;
                        if ucus_iz && (!A.rakip || std::env::var("ALAN_UCUS_RAKIP").ok().as_deref() == Some(&*A.p.govde.name)) && UC_IZ_DER.with(|c| c.get()) == 1 && Tb < 40.0 {
                            eprintln!("UCUS   ~ i {:3} {:8} Tb {:.0} t {:.1} rcvB {:.1} tau {:.2} zz {:.2} dd {:.1} R {:.2} v {:.3} rc {:.1} gec {:.1} fren {:.1} yer ({:.1},{:.1}) hız ({:.2},{:.2}) iv {:?} vm {:.3}", i, &*A.p.govde.name, Tb, t, rcv_b, A.tau, zz, dd, R, A.v, A.rc, A.G.as_ref().map_or(0.0, |G| self.dis.gec_of(G, cx, cy, Some(dd - R))), self.fren_t(&A.p, cx, cy, R, Tb - A.rc, A.v), A.p.x, A.p.y, z0(A.p.vx), z0(A.p.vy), crate::kural::gec::ivk(&A.p, &kr), A.G.as_ref().map_or(f64::NAN, |G| G.vm));
                        }
                        let p: &Oyuncu = &A.p;
                        if ok1 {
                            RR[j] = R;
                            xm[j] = Some(jmax(0.0, jmin(if t0 >= 0.0 && sl > 1e-12 { ((p.x - Pp.x) * bvy - (p.y - Pp.y) * bvx).abs() / sl.sqrt() } else { dd }, R) - jmax(0.0, Tb - t) * A.v) / R);
                        }
                        if A.rakip {
                            let KS = A.kov == Some(false) && !gon && gr.is_some();
                            let HY = if KS { gr.as_ref().unwrap().yas_h } else { 0.0 };
                            let tc = if KS {
                                if Tb < HY {
                                    Tb - Tb * Tb / (2.0 * HY)
                                } else {
                                    HY / 2.0
                                }
                            } else {
                                jmin(Tb, A.rc)
                            };
                            let kjy = KS && A.kt.is_none() && A.kj.is_some();
                            let kty = KS && A.kt.is_some();
                            let k_l = if kty { hypot(A.kt.unwrap().x - A.kx, A.kt.unwrap().y - A.ky) } else { 0.0 };
                            let k_s = if kty { jmin(k_l, jmax(0.0, Tb - if A.kt_t < -1e8 { 0.0 } else { jmax(A.rc, gecik + A.kt_t) }) * A.v) } else { 0.0 };
                            let (qx, qy) = if kjy {
                                self.kos_yer(p, &A.kj.unwrap(), Tb)
                            } else if kty {
                                let kt = A.kt.unwrap();
                                (A.kx + if k_l > 1e-6 { (kt.x - A.kx) / k_l * k_s } else { 0.0 }, A.ky + if k_l > 1e-6 { (kt.y - A.ky) / k_l * k_s } else { 0.0 })
                            } else {
                                (p.x + z0(p.vx) * tc, p.y + z0(p.vy) * tc)
                            };
                            let u0 = if sl > 1e-12 { ((qx - Pp.x) * bvx + (qy - Pp.y) * bvy) / sl } else { 1.0 };
                            let uu = if u0 < 0.0 { 0.0 } else if u0 > 1.0 { 1.0 } else { u0 };
                            let d2 = hypot(Pp.x + bvx * uu - qx, Pp.y + bvy * uu - qy);
                            // ALAN_GOVDE_YOL_IZ: kovalamayan savunmacı da tepkisinden sonra yola koşabilir (görevi değişebilir): olabilirlik
                            let d2z = if crate::kafa::kafa2_tarti::govde_yol() { jmax(0.0, d2 - jmax(0.0, Tb - A.rc) * A.v) } else { d2 };
                            zz = jmax(zz, (R - d2z) / BKS);
                            if ok1 {
                                xm[j] = Some(jmin(xm[j].unwrap(), jmin(d2, R) / R));
                            }
                        }
                    }
                    if !(A.rakip || A.bek || qq_i.is_none() || A.p.id == src[qq_i.unwrap()].id || gon) {
                        zz = jmin(zz, (R - dd) / BKS);
                    }
                    if gyol {
                        // tepki verebilen ama gövdesi zaten yolda olan: topa ulaşır (kontrolü ayrıca)
                        zz = jmax(zz, 8.0);
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
                let oku_b = ok1 && son == 0;
                if oku_b {
                    tt.kt = Some(0.0);
                    tt.cue = cue1;
                    tt.k_bek = true;
                    tt.oku = std::mem::take(&mut okc);
                }
                let mut b_l = TopT { b: bb, t: tt };
                let from = if oku_b { hfrom } else { None };
                for j in 0..na {
                    if F[j] == 0.0 {
                        continue;
                    }
                    let dP = jmax(0.0, S - ad[j].tc) * F[j] * sc;
                    let (gv_now, yv, bek, rakip) = (ad[j].gv_now, ad[j].yv, ad[j].bek, ad[j].rakip);
                    let p_t: Option<(f64, f64)> = if !ok1 || yv || gv_now || bek {
                        None
                    } else if rakip {
                        self.ak_hiz(&ad[j].p, &ad[j], bvx, bvy, CX[j], CY[j], IN.v_ak)
                    } else {
                        Some(self.al_hiz(&ad[j].p, &ad[j], DD[j], RR[j], CX[j], CY[j], bvx, bvy, Tb, IN.v_set))
                    };
                    let Pc = if gv_now {
                        0.0
                    } else {
                        ad[j].uc_x = if ok1 && !bek { xm[j] } else { None };
                        // hız değişmiyorsa kopya yok; değişiyorsa oyuncu başına bir kez kopyalanır, yalnız vx/vy yazılır (ad[j].p döngüde değişmez)
                        match p_t {
                            None => ctrl_p_al(&mut b_l, &ad[j].p, from, src, ad[j].uc_x, &kr),
                            Some((vx, vy)) => {
                                let pp = KOP[j].get_or_insert_with(|| (*ad[j].p).clone());
                                pp.vx = vx;
                                pp.vy = vy;
                                ctrl_p_al(&mut b_l, pp, from, src, ad[j].uc_x, &kr)
                            }
                        }
                    };
                    ad[j].p_t = p_t;
                    if ucus_iz && UC_IZ_DER.with(|c| c.get()) == 1 {
                        eprintln!("UCUS   i {:3} top ({:.1},{:.1}) hız {:.2} S {:.3} | {:8} {} yer ({:.1},{:.1}) F {:.3} dP {:.3} Pc {:.3}", i, P.x, P.y, hypot(bvx, bvy), S, &*ad[j].p.govde.name, if rakip { "R" } else { "B" }, CX[j], CY[j], F[j], dP, Pc);
                    }
                    let p: &Oyuncu = &ad[j].p;
                    let pdx = atan2(p.y - CY[j], p.x - CX[j]);
                    if !rakip && !yv {
                        let rk;
                        let mut rk_x = P.x;
                        let mut rk_y = P.y;
                        if ok1 && Pc < 0.99 {
                            let rq = if ski[j].is_some() && (CX[j] - skx[j]).abs() + (CY[j] - sky[j]).abs() < 3.0 {
                                SK[j]
                            } else {
                                ski[j] = Some(fi);
                                skx[j] = CX[j];
                                sky[j] = CY[j];
                                ska[j] = pdx;
                                SK[j] = self.sek_yaris(src, team, &ad[j], &mut b_l, from, CX[j], CY[j], Tb, Some(P.e), h_ki, gecik, false, &et_p, kc);
                                SK[j]
                            };
                            rk = jmin(1.0, rq.v[0] + rq.v[1]);
                            if rq.n > 4 {
                                rk_x = rq.v[4];
                                rk_y = rq.v[5];
                            }
                        } else {
                            rk = if ok1 { self.kacan_al(&ad[j].p, &ad, nk(P.x, P.y), bvx, bvy, Tb) } else { 0.4 };
                        }
                        let mm = dP * (Pc + (1.0 - Pc) * rk);
                        let oo = dP * (1.0 - Pc) * (1.0 - rk);
                        m1 += mm;
                        o1 += oo;
                        mx1 += mm * P.x;
                        my1 += mm * P.y;
                        ox1 += oo * rk_x;
                        oy1 += oo * rk_y;
                        let benf = ad[j].ben && !(qq_i.is_some() && ad[j].pi == qq_i);
                        if alis_kume {
                            let kcd = if benf { mm } else if !yv { dP * (1.0 - Pc) * rk } else { 0.0 };
                            if mm - kcd > 1e-12 {
                                dAl.push((mm - kcd, P.x, P.y));
                            }
                        }
                        if benf || !yv {
                            let kc_ = if benf { dP * (Pc + (1.0 - Pc) * rk) } else { dP * (1.0 - Pc) * rk };
                            s1 += kc_;
                            sx1 += kc_ * rk_x;
                            sy1 += kc_ * rk_y;
                            d1 += kc_;
                            dx1 += kc_ * CX[j];
                            dy1 += kc_ * CY[j];
                        }
                        cik += dP;
                        continue;
                    }
                    let th = if DD[j] < BODY_R || (ok1 && rakip && !bek && xm[j].map_or(false, |x| x * RR[j] < BODY_R)) {
                        0.0
                    } else {
                        (1.0 - Pc) * thru_p(&b_l.b, p) * if bek { KF[j].unwrap_or(1.0) } else { 1.0 }
                    };
                    let rest = jmax(0.0, 1.0 - Pc - th);
                    let yeni = gv_now || ski[j].is_none() || !((CX[j] - skx[j]).abs() + (CY[j] - sky[j]).abs() < 3.0) || (ok1 && (pdx - ska[j]).abs() > IN.sk_aci);
                    let rq: Sek = if yeni {
                        if dP * rest < kuc {
                            if ski[j].is_none() || gv_now {
                                Sek { v: [self.rb_at(src, team, nk(P.x, P.y)), 0.0, 0.0, 0.0, 0.0, 0.0], n: 2 }
                            } else {
                                SK[j]
                            }
                        } else {
                            ski[j] = Some(fi);
                            skx[j] = CX[j];
                            sky[j] = CY[j];
                            ska[j] = pdx;
                            SK[j] = self.sek_yaris(src, team, &ad[j], &mut b_l, from, CX[j], CY[j], Tb, Some(P.e), h_ki, gecik, false, &et_p, kc);
                            SK[j]
                        }
                    } else {
                        SK[j]
                    };
                    let rb = rq.v[0];
                    let mm = dP * rest * rb;
                    let oo = dP * (Pc + rest * (1.0 - rb - rq.v[1]));
                    let SY = rq.n > 2;
                    let oo_s = if SY { dP * rest * jmax(0.0, 1.0 - rb - rq.v[1]) } else { 0.0 };
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
                    ox1 += (oo - oo_s) * P.x + if SY { oo_s * rq.v[4] } else { 0.0 };
                    oy1 += (oo - oo_s) * P.y + if SY { oo_s * rq.v[5] } else { 0.0 };
                    let rt: Option<Sek> = if th > 1e-6 {
                        if !gv_now && szi[j].is_some() && (CX[j] - szx[j]).abs() + (CY[j] - szy[j]).abs() < 3.0 && !(ok1 && (pdx - sza[j]).abs() > IN.sk_aci) {
                            Some(sz[j])
                        } else if dP * th < kuc {
                            if szi[j].is_none() || gv_now {
                                None
                            } else {
                                Some(sz[j])
                            }
                        } else {
                            szi[j] = Some(fi);
                            szx[j] = CX[j];
                            szy[j] = CY[j];
                            sza[j] = pdx;
                            sz[j] = self.sek_yaris(src, team, &ad[j], &mut b_l, from, CX[j], CY[j], Tb, Some(P.e), h_ki, gecik, true, &et_p, kc);
                            Some(sz[j])
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
                if oku_b {
                    okc = std::mem::take(&mut b_l.t.oku);
                }
                S -= cik;
                i += 1;
            }
            if ucus_iz && UC_IZ_DER.with(|c| c.get()) == 1 {
                eprintln!("UCUS   yol sonu ({:.1},{:.1}) n {} bitiş {:?} kalan S {:.3} (yol {:?})", pts[n - 1].x, pts[n - 1].y, n, end.done, S, (z, w, zs, yol));
            }
            if S > 0.003 {
                let pn = pts[n - 1];
                match end.done {
                    Some(d) if d == Bitis::KuyuSag || d == Bitis::KuyuSol => {
                        if (d == Bitis::KuyuSag) == (team == 0.0) {
                            k1 += S;
                        } else {
                            o1 += S;
                            ox1 += S * pn.x;
                            oy1 += S * pn.y;
                        }
                    }
                    _ => {
                        let (mut tm, mut t_o, mut t_b): (f64, f64, f64) = (1e9, 1e9, 1e9);
                        let mut bk: Option<&Oyuncu> = None;
                        for p in src {
                            if p.noTouch {
                                continue;
                            }
                            let a = self.arr(p, pn.x, pn.y, None);
                            let tt = if !p.bekci() && !kuyu_girer(p, pn.x, pn.y, a, false, &kr) { 1e9 } else { a };
                            if p.team == team {
                                tm = jmin(tm, tt);
                            } else if p.bekci() {
                                t_b = tt;
                                bk = Some(p);
                            } else {
                                t_o = jmin(t_o, tt + 0.5 * jmax(0.0, self.TEPKI(p, Tepki::Kovala)));
                            }
                        }
                        if let Some(bk) = bk {
                            if self.bek_cikar(bk, pn.x, pn.y, t_b, tm) {
                                t_o = jmin(t_o, t_b);
                            }
                        }
                        let sv = sgm((t_o - tm) / 12.0);
                        let mm = S * sv;
                        let oo = S * (1.0 - sv);
                        m1 += mm;
                        o1 += oo;
                        mx1 += mm * pn.x;
                        my1 += mm * pn.y;
                        s1 += mm;
                        sx1 += mm * pn.x;
                        sy1 += mm * pn.y;
                        ox1 += oo * pn.x;
                        oy1 += oo * pn.y;
                    }
                }
            }
            if alis_kume {
                let dt: f64 = dAl.iter().map(|x| x.0).sum();
                if dt > 1e-9 {
                    let mut c = 0.0;
                    for &(a, x, y) in &dAl {
                        c += a;
                        if c >= 0.5 * dt {
                            km_a += w * dt;
                            km_x += w * dt * x;
                            km_y += w * dt * y;
                            break;
                        }
                    }
                }
            }
            let tot = ya(m1 + o1 + k1, 1.0);
            wt += w;
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
                let pn = pts[n - 1];
                ex += w * pn.x * (1.0 - (m1 + o1) / tot);
                ey += w * pn.y * (1.0 - (m1 + o1) / tot);
            }
        }
        m /= wt;
        o /= wt;
        k /= wt;
        let end = nk(ex / wt, ey / wt);
        let endM = if m > 1e-9 { nk(mx / wt / m, my / wt / m) } else { end };
        let endO = if o > 1e-9 { nk(ox / wt / o, oy / wt / o) } else { end };
        let mut Ps = sk / wt;
        let mut endS = if sk > 1e-9 { nk(sx / sk, sy / sk) } else { end };
        let m_d = m - Ps;
        let mut endD = if m_d > 1e-9 { nk((mx - sx) / wt / m_d, (my - sy) / wt / m_d) } else { endM };
        // ALAN_ALIS_KUME_IZ: alış yeri, alışların ortalaması değil ortancası (yol boyunca alışların yarısının
        // toplandığı nokta): erken alış ile top kaçarsa çok sonraki alışın ortalaması kimsenin almadığı bir yerdir.
        if alis_kume && km_a > 1e-9 && m_d > 1e-9 {
            endD = nk(km_x / km_a, km_y / km_a);
        }
        // endD işaretli ağırlıkların (m − s) ortası: s bir yerde m'yi aşınca ağırlık eksi olur, orta sahadan
        // binlerce (bir kez 160 milyon) br uzağa düşer; arkasındaki kaleci derinliği hesabı belleği tüketiyordu.
        // Sahadan uzak bir ortanın anlamı yok: mD ≈ 0'daki gibi endM. Eski davranış ALAN_ENDD_ESKI ile.
        let endD = if endd_sinirla() && !(endD.x.abs() < 3.0 * GW && endD.y.abs() < 3.0 * GH) { endM } else { endD };
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
        let Psek = dk / wt * if pre < 1.0 { pre } else { 1.0 };
        let endSek = if dk > 1e-9 { nk(dxs / dk, dys / dk) } else { end };
        Ucus { P: m, Pk: k, Pl: o, end, endM, endO, Ps, endS, endD, Psek, endSek }
    }
}

pub static BEK_DALIS_ZORLA: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
/// Kafanın Bekçi dalış modeli (maçın kaleci kurallarıyla; hm cezası yok). Varsayılan açık (sitedeki kafa); kapatmak: BEK_DALIS_KAPALI (wasm: adres seçeneği) ya da ALAN_BEK_DALIS_ESKI=1.
pub static BEK_DALIS_KAPALI: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub fn bek_dalis_k() -> bool {
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    if BEK_DALIS_KAPALI.load(std::sync::atomic::Ordering::Relaxed) {
        return false;
    }
    BEK_DALIS_ZORLA.load(std::sync::atomic::Ordering::Relaxed) || *A.get_or_init(|| std::env::var("ALAN_BEK_DALIS_ESKI").is_err())
}
pub static ALIS_KUME_ZORLA: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub fn alis_kume() -> bool {
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    ALIS_KUME_ZORLA.load(std::sync::atomic::Ordering::Relaxed) || *A.get_or_init(|| crate::iz_acik() && std::env::var("ALAN_ALIS_KUME_IZ").is_ok())
}

thread_local! { static UC_IZ_DER: std::cell::Cell<u32> = const { std::cell::Cell::new(0) }; }
struct UcIzDer;
impl UcIzDer {
    fn gir() -> UcIzDer { UC_IZ_DER.with(|c| c.set(c.get() + 1)); UcIzDer }
}
impl Drop for UcIzDer {
    fn drop(&mut self) { UC_IZ_DER.with(|c| c.set(c.get() - 1)); }
}

#[inline(always)]
fn dogru0(v: f64) -> bool {
    v != 0.0 && v == v
}

/// `endD` sahadan uzaksa `endM` (varsayılan). ALAN_ENDD_ESKI: sınırsız (eski davranış; yalnız yerelde).
fn endd_sinirla() -> bool {
    if cfg!(target_family = "wasm") {
        return true;
    }
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *A.get_or_init(|| std::env::var("ALAN_ENDD_ESKI").is_err())
}
