//! Enerji ve sprint (decide.js:207–243 EN2/EN3S/sarjBd/SPK/SPRS/sprKop/itTab/itPk/itHk/SUR/itKf/itT/itV,
//! 2498–2596 enMuh2/enW/enGec/sfSonra/enMuhasebe, 3967–4016 sarjMal/sarjVerim/sarjKarar/sarjTut/
//! chargeRate/sprKafa/surSure).
#![allow(non_snake_case)]

use crate::kafa::gecici_dis1::KuralErisim as _;
use super::kafa1_baglam::Kafa;
use super::kafa1_js::*;
use crate::mat::{exp, hypot, jmax, jmin, sin};
use crate::model::{JOEn, JpSf, Oyuncu, Secenek};
use crate::yetenek::{mek, spr_h, yum};
use std::borrow::Cow;

/// sarjTut'un dönüşü.
#[derive(Clone, Copy, Debug)]
pub struct SarjTut {
    pub c2: f64,
    pub bed: f64,
}


impl Kafa {
    /// JS SPK(): sprint evreleri ayrık mı
    pub fn spk(&self) -> bool {
        self.dis.alan_spr_faz().map_or(false, |f| f.ayir != 0.0 && f.ayir == f.ayir)
    }
    /// Taşıyıcı sprinte başlayabilir mi (decide.js:215).
    pub fn SPRS(&self, h: &Oyuncu) -> bool {
        self.spk() && !h.bekci() && h._sf.is_none() && h.en.unwrap_or(1.0) >= self.dis.alan_en3_sgir(h, self.dis.alan_en2().pat)
    }
    /// Sprintteki kopyası (decide.js:219).
    pub fn spr_kop(&self, h: &Oyuncu) -> Oyuncu {
        let mut c = h.clone();
        c._sf = Some(JpSf { t0: self.tick(), f: "patlama".into() });
        c.sprint = true;
        c
    }
    pub fn it_tab(&self, h: &Oyuncu) -> f64 {
        self.spd(h)
    }
    pub fn it_pk(&self, h: &Oyuncu) -> f64 {
        spr_h(h.govde.yet.hiz)
    }
    pub fn it_hk(&self, h: &Oyuncu) -> f64 {
        self.dis.alan_spr_hk(h)
    }
    /// İtip koşmanın hız katı (decide.js:229; md 0 rahat, 1 sprint, 2 tepe, 3 ara).
    pub fn it_kf(&self, h: &Oyuncu, md: i32) -> f64 {
        match md {
            2 => 1.0,
            3 => self.it_hk(h) + 0.5 * (self.it_pk(h) - self.it_hk(h)),
            1 => self.it_pk(h),
            _ => self.it_hk(h),
        }
    }
    /// İtip koşarak P'ye varış süresi (decide.js:230).
    pub fn it_t(&self, h: &Oyuncu, P: Nk, md: i32) -> f64 {
        let F = self.dis.alan_spr_faz().unwrap();
        let b = self.it_tab(h);
        let d = hypot(P.x - h.x, P.y - h.y);
        let t = if md == 2 || md == 3 {
            d / (b * self.it_kf(h, md))
        } else if md == 1 {
            let v1 = b * self.it_pk(h);
            let T = F.pat_t + F.tepe_t;
            if d <= v1 * T {
                d / v1
            } else {
                T + (d - v1 * T) / (b * 1.0)
            }
        } else {
            d / (b * self.it_hk(h))
        };
        if d > 1e-6 {
            if let Some(G) = self.dis.gec_tab(h, false) {
                return t + self.dis.gec_of(&G, P.x, P.y, Some(d));
            }
        }
        t
    }
    /// decide.js:240
    pub fn it_v(&self, h: &Oyuncu, md: i32, ta: f64) -> f64 {
        let F = self.dis.alan_spr_faz().unwrap();
        let b = self.it_tab(h);
        if md == 2 || md == 3 {
            b * self.it_kf(h, md)
        } else if md == 1 {
            if ta <= F.pat_t + F.tepe_t {
                b * self.it_pk(h)
            } else {
                b * 1.0
            }
        } else {
            b * self.it_hk(h)
        }
    }
    /// Şarjın bar bedeli (decide.js:209).
    pub fn sarj_bd(&self, h: &Oyuncu, _src: &[Op], dc: f64) -> f64 {
        let e = jmax(0.0, jmin(1.0, h.en.unwrap_or(1.0)));
        let c = jmin(e, dc * self.sarj_mal(h));
        let hr = jmax(0.0, jmin(1.0, self.dis.tac_en_harcama(h.team).unwrap_or(0.5)));
        (1.0 - (hr - 0.5)) * (self.dis.alan_en3_kul(h, e, true, h.x, h.y) - self.dis.alan_en3_kul(h, e - c, true, h.x, h.y))
    }
    /// decide.js:3968
    pub fn sarj_mal(&self, h: &Oyuncu) -> f64 {
        1.0 / jmax(0.5, jmin(2.0, 0.8 + 0.05 * (h.govde.yet.y(mek::SARJ_BEDEL) - 10.0))) / self.dis.alan_en_s(h)
    }
    /// decide.js:3969
    pub fn sarj_verim(&self, h: &Oyuncu) -> f64 {
        match h.en {
            None => 1.0,
            Some(e) => jmin(1.0, jmax(0.0, e) / 0.2),
        }
    }
    /// Topu şarj etmek değer mi (decide.js:3970).
    pub fn sarj_karar(&self, h: &Oyuncu, src: &[Op], ch: f64) -> bool {
        if let Some(e) = h.en {
            if e <= 0.0 {
                return false;
            }
        }
        let dc = jmin(1.0 - ch, self.charge_rate(h, src, 1.0, ch, true) * self.D.sarj_ufuk);
        if dc < 0.02 {
            return false;
        }
        let kz = self.V(src, h.team, h.x, h.y, jmin(1.0, ch + dc)) - self.V(src, h.team, h.x, h.y, ch);
        let bd = self.sarj_bd(h, src, dc);
        kz > bd
    }
    /// T tik tutarken şarjın kazancı ve bedeli (decide.js:3982).
    pub fn sarj_tut(&self, h: &Oyuncu, src: &[Op], ch: f64, still: f64, T: f64) -> Option<SarjTut> {
        if h.en.map_or(false, |e| e <= 0.0) || h.id < 0.0 {
            return None;
        }
        let dc = jmin(1.0 - ch, self.charge_rate(h, src, still, ch, true) * T);
        if dc < 0.005 {
            None
        } else {
            Some(SarjTut { c2: ch + dc, bed: self.sarj_bd(h, src, dc) })
        }
    }
    /// Şarj hızı (decide.js:3990).
    pub fn charge_rate(&self, h: &Oyuncu, src: &[Op], still: f64, ch: f64, ham: bool) -> f64 {
        let pasif = !ham && !h.hal.sarj.unwrap_or(true);
        let pr = self.press_at(src, h.team, h.x, h.y);
        let yk = h.govde.yet.y(mek::SARJ_KOSU_PAY);
        let mn = jmin(0.9, self.D.chg_min * jmax(0.3, 1.0 + 0.08 * (yk - 10.0)));
        (if pasif { self.D.chg_pasif } else { 1.0 })
            * (if ham { 1.0 } else { self.sarj_verim(h) })
            * self.D.chg0
            * self.D.chg_tek
            * (mn + (1.0 - mn) * still)
            * (0.25 + 0.75 * exp(2.0 * -pr * yum(&h.govde.yet, "B")))
            * (1.0 - 0.5 * ch)
            * (1.0 + 0.05 * (h.govde.yet.y(mek::SARJ_HIZ) - 10.0))
    }
    /// Kafanın sprint katı (decide.js:3997).
    pub fn spr_kafa(&self, p: &Oyuncu) -> f64 {
        if p.en.is_some() {
            self.dis.alan_spre(p)
        } else {
            self.dis.alan_spr(p)
        }
    }
    /// Topla L yolunu sürme süresi (decide.js:3998).
    pub fn sur_sure(&self, h: &Oyuncu, L: f64, dir: Option<Nk>, spr: bool) -> f64 {
        let F = self.dis.alan_spr_faz();
        let YS = self.spk();
        if let (Some(F), Some(_)) = (F, dir) {
            let tutan = match self.dis.mac_holder() {
                Some((id, team, x, y)) => id == h.id && team == h.team && hypot(x - h.x, y - h.y) < 1.0,
                None => false,
            };
            let sf_e = h._sf.as_ref().map_or(false, |s| s.f == "patlama" || s.f == "tepe");
            let kos = sf_e || if YS { spr && h._sf.is_none() } else { h._sf.is_none() && h.en.unwrap_or(1.0) >= F.bedel && dir.unwrap().x * (if h.team == 0.0 { 1.0 } else { -1.0 }) > 0.5 && L >= 4.0 };
            if L == 10.0 && spr && crate::iz_acik() && std::env::var("ALAN_C3_IZ").ok().map_or(false, |x| x == format!("{}", self.tick())) { eprintln!("   SSX {} holder {:?} h {} {} {} topla {} YS {} sf {} kos {}", h.govde.name, self.dis.mac_holder(), h.id, h.x, h.y, F.topla, YS, h._sf.is_some(), kos); }
            if F.topla != 0.0 && !h.bekci() && tutan && kos {
                let v = self.spd(h) * self.top_k(h) * self.ik_den(h);
                let pk = spr_h(h.govde.yet.hiz);
                let E2 = self.dis.alan_en2();
                let EB = YS;
                let tk0 = self.tick();
                let e0 = if EB { jmax(0.0, h.en.unwrap_or(1.0) - if h._sf.is_some() { 0.0 } else { E2.pat }) } else { 0.0 };
                let Tt = if EB { self.dis.alan_en2_ts(h, Some(e0)) } else { 0.0 };
                let T = if EB {
                    (match &h._sf {
                        Some(s) => jmax(0.0, F.pat_t - (tk0 - s.t0)),
                        None => F.pat_t,
                    }) + Tt
                } else {
                    F.pat_t + F.tepe_t
                };
                let pk_e = if EB {
                    let P0 = self.dis.alan_en3_tepe(Some(e0), pk);
                    let t_y = jmin(Tt, L / jmax(1e-6, v * P0));
                    self.dis.alan_en3_tepe(Some(jmax(0.0, e0 - 0.5 * t_y * E2.tepe / self.dis.alan_en2_sb(h))), pk)
                } else {
                    pk
                };
                let aft = if EB && !self.dis.alan_tasiyici_sonra_yavas() { 1.0 } else { F.after };
                let d1 = v * pk_e * T;
                if L == 10.0 && crate::iz_acik() && std::env::var("ALAN_C3_IZ").ok().map_or(false, |x| x == format!("{}", self.tick())) { eprintln!("   SS {} v {} pk {} e0 {} Tt {} T {} pkE {} aft {} d1 {} en {:?} sf {}", h.govde.name, v, pk, e0, Tt, T, pk_e, aft, d1, h.en, h._sf.is_some()); }
                return if L <= d1 { L / (v * pk_e) + 2.0 } else { T + 2.0 + (L - d1) / (v * aft) };
            }
        }
        let dir = dir.unwrap_or_default();
        let v = self.spd(h) * if h.sprint { self.spr_kafa(h) } else { 1.0 } * self.top_k(h) * self.ik_den(h);
        let v0 = jmax(0.0, z0(h.vx) * dir.x + z0(h.vy) * dir.y);
        // ALAN_PLAN_IZ: hızlanma payı yerine maçın dönüş tablosundan dönme bedeli (o anki hız ve yönden hedefe)
        if crate::kafa::kafa1_secim::plan_acik() && L > 0.0 {
            if let Some(G) = self.dis.gec_tab(h, false) {
                return L / v + self.dis.gec_of(&G, h.x + dir.x * L, h.y + dir.y * L, Some(L));
            }
        }
        L / v + jmax(0.0, 1.0 - v0 / v) / (0.18 * self.AH(h))
    }

    /// Bir seçeneğin enerji muhasebesi (decide.js:2498 enMuh2; Enerji 2.0/3.0 açık).
    pub fn en_muhasebe(&self, h: &Oyuncu, src: &[Op], w: &[Op], T: f64, bit: Option<Nk>, o: Option<&Secenek>) -> JOEn {
        self.en_muh2(h, src, w, T, bit, o)
    }
    pub fn en_muh2(&self, h: &Oyuncu, src: &[Op], w: &[Op], T: f64, bit: Option<Nk>, o: Option<&Secenek>) -> JOEn {
        let E = self.dis.alan_en();
        let E2 = self.dis.alan_en2();
        let mut out = JOEn::default();
        let har_r = |p: &Oyuncu, v: f64, b: f64, vm: f64| if v > b { E2.tepe * jmin(1.0, (v - b) / jmax(1e-6, vm - b)) / self.dis.alan_en2_sb(p) } else { 0.0 };
        for p in src {
            let pen = match p.en {
                Some(e) if !p.bekci() => e,
                _ => continue,
            };
            let q = match bul(w, p) {
                Some(q) => q,
                None => continue,
            };
            let b = self.spd(p);
            let vm = b * spr_h(p.govde.yet.hiz);
            let Sb = self.dis.alan_en2_sb(p);
            let dw = hypot(q.x - p.x, q.y - p.y);
            let nd_q = {
                let mut n: f64 = 1e9;
                for z in w {
                    if !(z.team == p.team || z.noTouch) {
                        n = jmin(n, hypot(z.x - q.x, z.y - q.y));
                    }
                }
                n
            };
            let bizden = p.team == h.team;
            let d_r = |r: f64| {
                if r > 1.02 && !p.bekci() {
                    0.0
                } else {
                    self.dis.alan_en3_dol(p, bizden, nd_q, if bizden { 0.0 } else { jmax(0.0, 1.0 - r) })
                }
            };
            let mut de;
            if bizden && !ayni_varlik(p, h) && dw < 0.3 {
                let sx = bit.map_or(0.0, |b| 0.5 * (b.x - h.x));
                let sy = bit.map_or(0.0, |b| 0.5 * (b.y - h.y));
                let L = hypot(p.tx + sx - p.x, p.ty + sy - p.y);
                let spr = L / T > b;
                let v = if spr { vm } else { b };
                let t_k = jmin(T, L / jmax(0.01, v));
                let tD = T - t_k;
                de = har_r(p, v, b, vm) * t_k - (if spr { 0.0 } else { d_r(1.0) * t_k }) - d_r(0.0) * tD;
            } else {
                let need = dw / jmax(1.0, T);
                de = if need > b { har_r(p, need, b, vm) * T } else { -d_r(need / b) * T };
            }
            if ayni_varlik(p, h) {
                if let Some(o) = o {
                    let mut ek = 0.0;
                    if let Some(st) = o.savT {
                        if st > 0.0 {
                            ek += E.dolas * st * p.govde.yet.verim(mek::DONDURME_BEDEL);
                        }
                    }
                    if o.kind == "fake" {
                        ek += E.fake * p.govde.yet.verim(mek::FAKE_BEDEL);
                    }
                    if o.kind == "tut" {
                        let mut nd: f64 = 1e9;
                        for z in w {
                            if z.team != h.team && !z.bekci() {
                                nd = jmin(nd, hypot(z.x - q.x, z.y - q.y));
                            }
                        }
                        if nd < 2.4 {
                            ek += E.kalkan * E.fiz_k * p.govde.yet.verim(mek::TEMAS_BEDEL) * T * 0.5;
                        }
                    }
                    de += ek / Sb;
                    if o.spr.map_or(false, crate::mat::dogru) && self.spk() {
                        de += E2.pat;
                    }
                }
            }
            de = jmax(de, -(1.0 - pen));
            let ck = if bizden { p.govde.yet.ca_k() } else { 1.0 };
            if bizden {
                out.biz += de * ck;
            } else {
                out.rakip += de * ck;
            }
            out.per.push((format!("{}:{}", p.team, p.id), de * ck));
            if !bizden {
                out.hizRakipY += 0.0;
            }
        }
        let g = h.govde.yet.y(mek::ENERJI_TAHMIN);
        let hs = 43758.5453 * sin(12.9898 * (h.id + 1.0) + 78.233 * T);
        let u = hs - hs.floor() - 0.5;
        let hata = 1.0 + u * (20.0 - g) * 0.04;
        out.rakip *= hata;
        out.hizRakip *= hata;
        out.hizRakipD *= hata;
        out.hizRakipY *= hata;
        for e in out.per.iter_mut() {
            let tm: f64 = e.0[..1].parse().unwrap_or(f64::NAN);
            if tm != h.team {
                e.1 *= hata;
            }
        }
        out.hizBiz = to_fixed(out.hizBiz, 4);
        out.hizRakip = to_fixed(out.hizRakip, 4);
        out.hizRakipD = to_fixed(out.hizRakipD, 4);
        out.hizRakipY = to_fixed(out.hizRakipY, 4);
        out.biz = to_fixed(out.biz, 3);
        out.rakip = to_fixed(out.rakip, 3);
        out
    }
    /// Seçeneğin enerjisiyle dünya (decide.js:2564): barları harcanmış oyuncular.
    pub fn en_w<'a>(&self, w: &'a [Op], o: Option<&Secenek>) -> Cow<'a, [Op]> {
        let P = match o.and_then(|o| o.en.as_ref()) {
            Some(en) if !en.per.is_empty() => &en.per,
            _ => return Cow::Borrowed(w),
        };
        let wD = jmax(0.0, self.bag._en_wd.unwrap_or(0.0));
        let tm = self.bag._en_tm;
        let mut deg = false;
        let r: Vec<Op> = w
            .iter()
            .map(|z| {
                let zen = match z.en {
                    Some(e) => e,
                    None => return z.clone(),
                };
                let k0 = format!("{}:{}", z.team, z.id);
                let de = P.iter().find(|e| e.0 == k0).map_or(0.0, |e| e.1);
                if !(de != 0.0 && de == de) {
                    return z.clone();
                }
                deg = true;
                let k = if Some(z.team) == tm { 1.0 + 2.0 * wD } else { 1.0 };
                let mut c = (**z).clone();
                c.en = Some(jmax(0.0, jmin(1.0, zen - de * k)));
                Op::new(c)
            })
            .collect();
        if deg {
            Cow::Owned(r)
        } else {
            Cow::Borrowed(w)
        }
    }
    /// enGec (decide.js:2581): q'nun barı w'dekiyle değişecekse yeni değeri (çağıran q.en'i
    /// geçici olarak bununla değiştirip işi yapar; JS q'yu yerinde değiştirip geri alır).
    pub fn en_gec_en(&self, q: &Oyuncu, w: &[Op]) -> Option<f64> {
        let qen = q.en?;
        let z = bul(w, q)?;
        match z.en {
            Some(e) if e != qen => Some(e),
            _ => None,
        }
    }
}
