//! Enerji ve sprint (decide.js:207–243 EN2/EN3S/sarjBd/SPK/SPRS/sprKop/itTab/itPk/itHk/SUR/itKf/itT/itV,
//! 2498–2596 enMuh2/enW/enGec/sfSonra/enMuhasebe, 3967–4016 sarjMal/sarjVerim/sarjKarar/sarjTut/
//! chargeRate/sprKafa/surSure).
#![allow(non_snake_case)]

use super::kafa1_baglam::Kafa;
use super::kafa1_js::*;
use crate::mat::{exp, hypot, jmax, jmin, sin};
use crate::model::{JOEn, JP_sf, JsDeger, Oyuncu, Secenek};
use crate::yetenek::{mek, spr_h, yum};
use std::borrow::Cow;

/// sarjTut'un dönüşü.
#[derive(Clone, Copy, Debug)]
pub struct SarjTut {
    pub c2: f64,
    pub bed: f64,
}

/// JS doğruluğu (karışık tip).
pub fn js_dogru(v: &Option<JsDeger>) -> bool {
    match v {
        None | Some(JsDeger::Yok) => false,
        Some(JsDeger::Sayi(x)) => *x != 0.0 && *x == *x,
        Some(JsDeger::Metin(s)) => !s.is_empty(),
        Some(JsDeger::Mantik(b)) => *b,
        Some(_) => true,
    }
}

impl Kafa {
    /// JS SPK(): sprint evreleri ayrık mı
    pub fn SPK(&self) -> bool {
        self.dis.ALAN_SPR_FAZ().map_or(false, |f| f.ayir != 0.0 && f.ayir == f.ayir)
    }
    /// Taşıyıcı sprinte başlayabilir mi (decide.js:215).
    pub fn SPRS(&self, h: &Oyuncu) -> bool {
        self.SPK() && !h.bekci() && h._sf.is_none() && h.en.unwrap_or(1.0) >= self.dis.ALAN_EN3_SGIR(h, self.dis.ALAN_EN2().pat)
    }
    /// Sprintteki kopyası (decide.js:219).
    pub fn sprKop(&self, h: &Oyuncu) -> Oyuncu {
        let mut c = h.clone();
        c._sf = Some(JP_sf { t0: self.tick(), f: "patlama".into() });
        c.sprint = true;
        c
    }
    pub fn itTab(&self, h: &Oyuncu) -> f64 {
        self.spd(h)
    }
    pub fn itPk(&self, h: &Oyuncu) -> f64 {
        spr_h(h.yet.hiz)
    }
    pub fn itHk(&self, h: &Oyuncu) -> f64 {
        self.dis.ALAN_SPR_HK(h)
    }
    /// İtip koşmanın hız katı (decide.js:229; md 0 rahat, 1 sprint, 2 tepe, 3 ara).
    pub fn itKf(&self, h: &Oyuncu, md: i32) -> f64 {
        match md {
            2 => 1.0,
            3 => self.itHk(h) + 0.5 * (self.itPk(h) - self.itHk(h)),
            1 => self.itPk(h),
            _ => self.itHk(h),
        }
    }
    /// İtip koşarak P'ye varış süresi (decide.js:230).
    pub fn itT(&self, h: &Oyuncu, P: Nk, md: i32) -> f64 {
        let F = self.dis.ALAN_SPR_FAZ().unwrap();
        let b = self.itTab(h);
        let d = hypot(P.x - h.x, P.y - h.y);
        let t = if md == 2 || md == 3 {
            d / (b * self.itKf(h, md))
        } else if md == 1 {
            let v1 = b * self.itPk(h);
            let T = F.patT + F.tepeT;
            if d <= v1 * T {
                d / v1
            } else {
                T + (d - v1 * T) / (b * 1.0)
            }
        } else {
            d / (b * self.itHk(h))
        };
        if d > 1e-6 {
            if let Some(G) = self.dis.gecTab(h, false) {
                return t + self.dis.gecOf(&G, P.x, P.y, Some(d));
            }
        }
        t
    }
    /// decide.js:240
    pub fn itV(&self, h: &Oyuncu, md: i32, ta: f64) -> f64 {
        let F = self.dis.ALAN_SPR_FAZ().unwrap();
        let b = self.itTab(h);
        if md == 2 || md == 3 {
            b * self.itKf(h, md)
        } else if md == 1 {
            if ta <= F.patT + F.tepeT {
                b * self.itPk(h)
            } else {
                b * 1.0
            }
        } else {
            b * self.itHk(h)
        }
    }
    /// Şarjın bar bedeli (decide.js:209).
    pub fn sarjBd(&self, h: &Oyuncu, _src: &[Op], dc: f64) -> f64 {
        let e = jmax(0.0, jmin(1.0, h.en.unwrap_or(1.0)));
        let c = jmin(e, dc * self.sarjMal(h));
        let hr = jmax(0.0, jmin(1.0, self.dis.tac_enHarcama(h.team).unwrap_or(0.5)));
        (1.0 - (hr - 0.5)) * (self.dis.ALAN_EN3_KUL(h, e, true, h.x, h.y) - self.dis.ALAN_EN3_KUL(h, e - c, true, h.x, h.y))
    }
    /// decide.js:3968
    pub fn sarjMal(&self, h: &Oyuncu) -> f64 {
        1.0 / jmax(0.5, jmin(2.0, 0.8 + 0.05 * (h.yet.y(mek::SARJ_BEDEL) - 10.0))) / self.dis.ALAN_EN_S(h)
    }
    /// decide.js:3969
    pub fn sarjVerim(&self, h: &Oyuncu) -> f64 {
        match h.en {
            None => 1.0,
            Some(e) => jmin(1.0, jmax(0.0, e) / 0.2),
        }
    }
    /// Topu şarj etmek değer mi (decide.js:3970).
    pub fn sarjKarar(&self, h: &Oyuncu, src: &[Op], ch: f64) -> bool {
        if let Some(e) = h.en {
            if e <= 0.0 {
                return false;
            }
        }
        let dc = jmin(1.0 - ch, self.chargeRate(h, src, 1.0, ch, true) * self.D.sarjUfuk);
        if dc < 0.02 {
            return false;
        }
        let kz = self.V(src, h.team, h.x, h.y, jmin(1.0, ch + dc)) - self.V(src, h.team, h.x, h.y, ch);
        let bd = self.sarjBd(h, src, dc);
        kz > bd
    }
    /// T tik tutarken şarjın kazancı ve bedeli (decide.js:3982).
    pub fn sarjTut(&self, h: &Oyuncu, src: &[Op], ch: f64, still: f64, T: f64) -> Option<SarjTut> {
        if h.en.map_or(false, |e| e <= 0.0) || h.id < 0.0 {
            return None;
        }
        let dc = jmin(1.0 - ch, self.chargeRate(h, src, still, ch, true) * T);
        if dc < 0.005 {
            None
        } else {
            Some(SarjTut { c2: ch + dc, bed: self.sarjBd(h, src, dc) })
        }
    }
    /// Şarj hızı (decide.js:3990).
    pub fn chargeRate(&self, h: &Oyuncu, src: &[Op], still: f64, ch: f64, ham: bool) -> f64 {
        let pasif = !ham && !h.sarj.unwrap_or(true);
        let pr = self.pressAt(src, h.team, h.x, h.y);
        let yk = h.yet.y(mek::SARJ_KOSU_PAY);
        let mn = jmin(0.9, self.D.chgMin * jmax(0.3, 1.0 + 0.08 * (yk - 10.0)));
        (if pasif { self.D.chgPasif } else { 1.0 })
            * (if ham { 1.0 } else { self.sarjVerim(h) })
            * self.D.chg0
            * self.D.chgTek
            * (mn + (1.0 - mn) * still)
            * (0.25 + 0.75 * exp(2.0 * -pr * yum(&h.yet, "B")))
            * (1.0 - 0.5 * ch)
            * (1.0 + 0.05 * (h.yet.y(mek::SARJ_HIZ) - 10.0))
    }
    /// Kafanın sprint katı (decide.js:3997).
    pub fn sprKafa(&self, p: &Oyuncu) -> f64 {
        if p.en.is_some() {
            self.dis.ALAN_SPRE(p)
        } else {
            self.dis.ALAN_SPR(p)
        }
    }
    /// Topla L yolunu sürme süresi (decide.js:3998).
    pub fn surSure(&self, h: &Oyuncu, L: f64, dir: Option<Nk>, spr: bool) -> f64 {
        let F = self.dis.ALAN_SPR_FAZ();
        let YS = self.SPK();
        if let (Some(F), Some(_)) = (F, dir) {
            let tutan = match self.dis.mac_holder() {
                Some((id, team, x, y)) => id == h.id && team == h.team && hypot(x - h.x, y - h.y) < 1.0,
                None => false,
            };
            let sfE = h._sf.as_ref().map_or(false, |s| s.f == "patlama" || s.f == "tepe");
            let kos = sfE || if YS { spr && h._sf.is_none() } else { h._sf.is_none() && h.en.unwrap_or(1.0) >= F.bedel && dir.unwrap().x * (if h.team == 0.0 { 1.0 } else { -1.0 }) > 0.5 && L >= 4.0 };
            if L == 10.0 && spr && crate::iz_acik() && std::env::var("ALAN_C3_IZ").ok().map_or(false, |x| x == format!("{}", self.tick())) { eprintln!("   SSX {} holder {:?} h {} {} {} topla {} YS {} sf {} kos {}", h.name, self.dis.mac_holder(), h.id, h.x, h.y, F.topla, YS, h._sf.is_some(), kos); }
            if F.topla != 0.0 && !h.bekci() && tutan && kos {
                let v = self.spd(h) * self.topK(h) * self.ikDen(h);
                let pk = spr_h(h.yet.hiz);
                let E2 = self.dis.ALAN_EN2();
                let EB = YS;
                let tk0 = self.tick();
                let e0 = if EB { jmax(0.0, h.en.unwrap_or(1.0) - if h._sf.is_some() { 0.0 } else { E2.pat }) } else { 0.0 };
                let Tt = if EB { self.dis.ALAN_EN2_TS(h, Some(e0)) } else { 0.0 };
                let T = if EB {
                    (match &h._sf {
                        Some(s) => jmax(0.0, F.patT - (tk0 - s.t0)),
                        None => F.patT,
                    }) + Tt
                } else {
                    F.patT + F.tepeT
                };
                let pkE = if EB {
                    let P0 = self.dis.ALAN_EN3_TEPE(Some(e0), pk);
                    let tY = jmin(Tt, L / jmax(1e-6, v * P0));
                    self.dis.ALAN_EN3_TEPE(Some(jmax(0.0, e0 - 0.5 * tY * E2.tepe / self.dis.ALAN_EN2_SB(h))), pk)
                } else {
                    pk
                };
                let aft = if EB && !self.dis.ALAN_TASIYICI_SONRA_YAVAS() { 1.0 } else { F.after };
                let d1 = v * pkE * T;
                if L == 10.0 && crate::iz_acik() && std::env::var("ALAN_C3_IZ").ok().map_or(false, |x| x == format!("{}", self.tick())) { eprintln!("   SS {} v {} pk {} e0 {} Tt {} T {} pkE {} aft {} d1 {} en {:?} sf {}", h.name, v, pk, e0, Tt, T, pkE, aft, d1, h.en, h._sf.is_some()); }
                return if L <= d1 { L / (v * pkE) + 2.0 } else { T + 2.0 + (L - d1) / (v * aft) };
            }
        }
        let dir = dir.unwrap_or_default();
        let v = self.spd(h) * if h.sprint { self.sprKafa(h) } else { 1.0 } * self.topK(h) * self.ikDen(h);
        let v0 = jmax(0.0, z0(h.vx) * dir.x + z0(h.vy) * dir.y);
        L / v + jmax(0.0, 1.0 - v0 / v) / (0.18 * self.AH(h))
    }

    /// Bir seçeneğin enerji muhasebesi (decide.js:2498 enMuh2; Enerji 2.0/3.0 açık).
    pub fn enMuhasebe(&self, h: &Oyuncu, src: &[Op], w: &[Op], T: f64, bit: Option<Nk>, o: Option<&Secenek>) -> JOEn {
        self.enMuh2(h, src, w, T, bit, o)
    }
    pub fn enMuh2(&self, h: &Oyuncu, src: &[Op], w: &[Op], T: f64, bit: Option<Nk>, o: Option<&Secenek>) -> JOEn {
        let E = self.dis.ALAN_EN();
        let E2 = self.dis.ALAN_EN2();
        let mut out = JOEn::default();
        let harR = |p: &Oyuncu, v: f64, b: f64, vm: f64| if v > b { E2.tepe * jmin(1.0, (v - b) / jmax(1e-6, vm - b)) / self.dis.ALAN_EN2_SB(p) } else { 0.0 };
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
            let vm = b * spr_h(p.yet.hiz);
            let Sb = self.dis.ALAN_EN2_SB(p);
            let dw = hypot(q.x - p.x, q.y - p.y);
            let ndQ = {
                let mut n: f64 = 1e9;
                for z in w {
                    if !(z.team == p.team || z.noTouch) {
                        n = jmin(n, hypot(z.x - q.x, z.y - q.y));
                    }
                }
                n
            };
            let bizden = p.team == h.team;
            let dR = |r: f64| {
                if r > 1.02 && !p.bekci() {
                    0.0
                } else {
                    self.dis.ALAN_EN3_DOL(p, bizden, ndQ, if bizden { 0.0 } else { jmax(0.0, 1.0 - r) })
                }
            };
            let mut de;
            if bizden && !ayni(p, h) && dw < 0.3 {
                let sx = bit.map_or(0.0, |b| 0.5 * (b.x - h.x));
                let sy = bit.map_or(0.0, |b| 0.5 * (b.y - h.y));
                let L = hypot(p.tx + sx - p.x, p.ty + sy - p.y);
                let spr = L / T > b;
                let v = if spr { vm } else { b };
                let tK = jmin(T, L / jmax(0.01, v));
                let tD = T - tK;
                de = harR(p, v, b, vm) * tK - (if spr { 0.0 } else { dR(1.0) * tK }) - dR(0.0) * tD;
            } else {
                let need = dw / jmax(1.0, T);
                de = if need > b { harR(p, need, b, vm) * T } else { -dR(need / b) * T };
            }
            if ayni(p, h) {
                if let Some(o) = o {
                    let mut ek = 0.0;
                    if let Some(st) = o.savT {
                        if st > 0.0 {
                            ek += E.dolas * st * p.yet.verim(mek::DONDURME_BEDEL);
                        }
                    }
                    if o.kind == "fake" {
                        ek += E.fake * p.yet.verim(mek::FAKE_BEDEL);
                    }
                    if o.kind == "tut" {
                        let mut nd: f64 = 1e9;
                        for z in w {
                            if z.team != h.team && !z.bekci() {
                                nd = jmin(nd, hypot(z.x - q.x, z.y - q.y));
                            }
                        }
                        if nd < 2.4 {
                            ek += E.kalkan * E.fizK * p.yet.verim(mek::TEMAS_BEDEL) * T * 0.5;
                        }
                    }
                    de += ek / Sb;
                    if o.spr.map_or(false, crate::mat::dogru) && self.SPK() {
                        de += E2.pat;
                    }
                }
            }
            de = jmax(de, -(1.0 - pen));
            let ck = if bizden { p.yet.ca_k() } else { 1.0 };
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
        let g = h.yet.y(mek::ENERJI_TAHMIN);
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
    pub fn enW<'a>(&self, w: &'a [Op], o: Option<&Secenek>) -> Cow<'a, [Op]> {
        let P = match o.and_then(|o| o.en.as_ref()) {
            Some(en) if !en.per.is_empty() => &en.per,
            _ => return Cow::Borrowed(w),
        };
        let wD = jmax(0.0, self.D._enWD.unwrap_or(0.0));
        let tm = self.D._enTm;
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
    pub fn enGec_en(&self, q: &Oyuncu, w: &[Op]) -> Option<f64> {
        let qen = q.en?;
        let z = bul(w, q)?;
        match z.en {
            Some(e) if e != qen => Some(e),
            _ => None,
        }
    }
}
