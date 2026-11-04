//! Fake (decide.js:302–546 fakeP/fakeT/fakeHedef/fakeIlgili/fakeSecenek): bir vuruşu gösterip rakibi
//! aldatmak. Bugünkü davranışta (sonF) iki dünya (kandı/kanmadı) kafa2'nin sonraDeger'iyle tartılır.
#![allow(non_snake_case)]

use super::gecici_dis1::{riskOf, sonraDeger};
use super::kafa1_baglam::Kafa;
use super::kafa1_js::*;
use crate::mat::{exp, hypot, jmax, jmin, jround};
use crate::model::{JMTacE, JODet, JOShow, JOTo, Oyuncu, Secenek};
use crate::saha::{CY as GCY, MOUTH, W as GW};
use crate::yetenek::mek;

/// Gösterilen vuruş (JS `sh`): tür ("gönder", "pas", "gövde"), hedef, inanılırlık.
#[derive(Clone, Debug)]
pub struct FakeSh {
    pub k: String,
    pub to: Nk,
    pub c: f64,
}

/// JS alP(d): seçeneğin arkadaşa varma olasılığı (sekmeden gelen pay çıkar).
pub fn alP(d: Option<&JODet>) -> Option<f64> {
    d.and_then(|d| match (d.P, d.Ps) {
        (Some(P), Some(Ps)) => Some(jmax(0.0, P - Ps)),
        (P, _) => P,
    })
}

impl Kafa {
    /// Rakibin kanma olasılığı (decide.js:303).
    pub fn fakeP(&self, h: &Oyuncu, p: &Oyuncu, k: &str, c: Option<f64>) -> f64 {
        let ok = p.yet.y(mek::FAKE_KAN);
        let sk = h.yet.y(if k == "gövde" { mek::FAKE_GOVDE } else { mek::FAKE_INAN });
        let base = jmax(0.08, jmin(0.9, 0.5 + 0.04 * (sk - ok)));
        let cc = c.unwrap_or(1.0);
        let g = jmin(1.0, 0.5 + 0.03 * ok);
        base * (1.0 - g + g * cc)
    }
    /// Fake'in süresi (decide.js:307).
    pub fn fakeT(&self, h: &Oyuncu) -> f64 {
        jmax(8.0, jmin(18.0, jround(14.0 - 0.5 * (h.yet.y(mek::FAKE_SURE) - 10.0))))
    }
    /// Kanan rakibin koşacağı yer (decide.js:308).
    pub fn fakeHedef(&self, p: &Oyuncu, h: &Oyuncu, sh: &FakeSh) -> Nk {
        if sh.k == "gönder" && p.bekci() {
            let gx = if p.team == 0.0 { 0.0 } else { GW };
            return nk(gx + if p.team == 0.0 { 1.4 } else { -1.4 }, jmax(GCY - MOUTH, jmin(GCY + MOUTH, GCY + 0.9 * (sh.to.y - GCY))));
        }
        if sh.k == "gövde" {
            return nk(p.x + 0.6 * (sh.to.x - h.x), p.y + 0.6 * (sh.to.y - h.y));
        }
        let ax = sh.to.x - h.x;
        let ay = sh.to.y - h.y;
        let L2 = ya(ax * ax + ay * ay, 1.0);
        let u = jmax(0.15, jmin(0.95, ((p.x - h.x) * ax + (p.y - h.y) * ay) / L2));
        nk(h.x + ax * u, h.y + ay * u)
    }
    /// Rakip bu fake'le ilgili mi (decide.js:326).
    pub fn fakeIlgili(&self, p: &Oyuncu, h: &Oyuncu, sh: &FakeSh) -> bool {
        if p.team == h.team {
            return false;
        }
        if sh.k == "gövde" {
            return !p.bekci() && hypot(p.x - h.x, p.y - h.y) < 6.0;
        }
        if sh.k == "gönder" && p.bekci() {
            return true;
        }
        if p.bekci() {
            return false;
        }
        let t = self.fakeHedef(p, h, sh);
        hypot(t.x - p.x, t.y - p.y) < 7.0
    }
    fn fake_aldat(&self, p: &mut Oyuncu, h: &Oyuncu, sh: &FakeSh, T: f64) {
        let t = self.fakeHedef(p, h, sh);
        let vm = self.vS(p);
        let Ty = jmax(3.0, jround(3.0 + 0.45 * jmax(0.0, 20.0 - p.yet.y(mek::FAKE_KAN)) - 0.2 * (p.yet.ceviklik - 10.0)));
        let bek = p.bekci();
        let (x0, y0) = (p.x, p.y);
        p.vx = z0(p.vx);
        p.vy = z0(p.vy);
        let mut k = 0.0;
        while k < T {
            let yan = k < Ty;
            let gx = if yan { t.x } else if bek { x0 } else { h.x };
            let gy = if yan { t.y } else if bek { y0 } else { h.y };
            let ex = gx - p.x;
            let ey = gy - p.y;
            let e = hypot(ex, ey);
            let L = e - if yan || bek { 0.0 } else { 1.7 };
            let v = if L < 0.3 { 0.0 } else { jmin(vm, 0.25 * L) };
            let (wx, wy) = if v > 0.0 { (ex / e * v, ey / e * v) } else { (0.0, 0.0) };
            self.dis.ivme(p, wx, wy, vm);
            p.x += p.vx;
            p.y += p.vy;
            k += 1.0;
        }
    }
    fn fake_dunya(&mut self, h: &Oyuncu, src: &[Op], ch: f64, sh: &FakeSh, kandi: bool, T: f64, tac: Option<&JMTacE>) -> Option<Secenek> {
        let mut src2: Vec<Oyuncu> = src
            .iter()
            .map(|p| {
                let mut c = (**p).clone();
                c.hist = None;
                c.past = None;
                c
            })
            .collect();
        let hi = sira(src, h)?;
        src2[hi].vx = 0.0;
        src2[hi].vy = 0.0;
        for p in src2.iter_mut() {
            if p.team == h.team {
                continue;
            }
            let vmx = self.vS(p);
            let il = kandi && self.fakeIlgili(p, h, sh);
            if il {
                self.fake_aldat(p, h, sh, T);
            } else if !p.bekci() {
                let dx = h.x - p.x;
                let dy = h.y - p.y;
                let L = hypot(dx, dy);
                if L < 10.0 && L > 1.7 {
                    let mv = jmin(L - 1.7, vmx * T * 0.8);
                    p.x += dx / L * mv;
                    p.y += dy / L * mv;
                }
            }
        }
        let risk = riskOf(self, h, tac);
        let src2: Vec<Op> = src2.into_iter().map(Op::new).collect();
        let h2 = (*src2[hi]).clone();
        sonraDeger(self, &h2, &src2, ch, risk)
    }
    /// Fake seçenekleri (decide.js:334): `opts`'a ekler. `kon`: kayıp yerinin değeri (kafa2'nin KF'si).
    #[allow(clippy::too_many_arguments)]
    pub fn fakeSecenek(&mut self, h: &Oyuncu, src: &[Op], ch: f64, tac: Option<&JMTacE>, opts: &mut Vec<Secenek>, kon: Option<&dyn Fn(f64, f64) -> f64>) {
        if self.D._noFake || self.D._inLook || h.bekci() {
            return;
        }
        let team = h.team;
        let gx = if team == 0.0 { GW } else { 0.0 };
        let gd = hypot(gx - h.x, GCY - h.y);
        let vB = enb(opts.iter().filter_map(|o| o.v));
        let inan = |v: f64| 1.0 / (1.0 + exp(-(v - vB + 0.06) / 0.03));
        let enIyi = |f: &dyn Fn(&Secenek) -> bool| -> Option<usize> {
            let mut b: Option<usize> = None;
            for (i, o) in opts.iter().enumerate() {
                if f(o) {
                    match b {
                        Some(j) if !(o.v.unwrap() > opts[j].v.unwrap()) => {}
                        _ => b = Some(i),
                    }
                }
            }
            b
        };
        let mut cand: Vec<FakeSh> = Vec::new();
        if let Some(gi) = enIyi(&|o| o.kind == "gönder" && o.to.is_some() && o.v.is_some()) {
            let g = &opts[gi];
            if gd < 32.0 {
                let t = g.to.as_ref().unwrap();
                cand.push(FakeSh { k: "gönder".into(), to: nk(t.x, t.y), c: jmin(inan(g.v.unwrap()), 0.2 + 4.0 * g.det.as_ref().and_then(|d| d.Pk).filter(|v| crate::mat::dogru(*v)).unwrap_or(0.0)) });
            }
        }
        if let Some(pi) = enIyi(&|o| (o.kind == "pas" || o.kind == "önüne" || o.kind == "kenardan") && o.to.is_some() && o.v.map_or(false, |v| v > -0.05)) {
            let p = &opts[pi];
            let t = p.to.as_ref().unwrap();
            cand.push(FakeSh { k: "pas".into(), to: nk(t.x, t.y), c: jmin(inan(p.v.unwrap()), 0.3 + 0.7 * alP(p.det.as_ref()).unwrap_or(0.5)) });
        }
        let mut nq: Option<&Oyuncu> = None;
        let mut nd = 1e9;
        for q in src {
            if q.team != team && !q.bekci() {
                let dd = hypot(q.x - h.x, q.y - h.y);
                if dd < nd {
                    nd = dd;
                    nq = Some(q);
                }
            }
        }
        if let Some(nq) = nq {
            if nd < 5.0 {
                let s0 = enIyi(&|o| o.kind == "sür" && o.to.is_some() && o.v.is_some());
                let ux = (h.x - nq.x) / ya(nd, 1.0);
                let uy = (h.y - nq.y) / ya(nd, 1.0);
                let mut px = -uy;
                let mut py = ux;
                if let Some(si) = s0 {
                    let t = opts[si].to.as_ref().unwrap();
                    if (t.x - h.x) * px + (t.y - h.y) * py > 0.0 {
                        px = -px;
                        py = -py;
                    }
                }
                let sv = enIyi(&|o| o.kind == "sür" && o.v.is_some() && o.to.as_ref().map_or(false, |t| (t.x - h.x) * px + (t.y - h.y) * py > 0.0));
                cand.push(FakeSh { k: "gövde".into(), to: nk(h.x + 3.0 * px, h.y + 3.0 * py), c: sv.map_or(0.25, |i| inan(opts[i].v.unwrap())) });
            }
        }
        let T = self.fakeT(h);
        // dalga: her aday için kandı/kanmadı dünyaları (sırası sonuçtan bağımsız)
        let mut FR: Vec<(usize, bool, Option<Secenek>)> = Vec::new();
        for (ci, sh) in cand.iter().enumerate() {
            if src.iter().any(|p| self.fakeIlgili(p, h, sh)) {
                FR.push((ci, true, None));
                FR.push((ci, false, None));
            }
        }
        for x in FR.iter_mut() {
            let sh = cand[x.0].clone();
            x.2 = self.fake_dunya(h, src, ch, &sh, x.1, T, tac);
        }
        let mut fi = 0;
        let mut k0: Option<f64> = None;
        let mut L0 = 0.0;
        for sh in &cand {
            let il: Vec<&Op> = src.iter().filter(|p| self.fakeIlgili(p, h, sh)).collect();
            if il.is_empty() {
                continue;
            }
            let pf = 1.0 - il.iter().fold(1.0, |a, p| a * (1.0 - self.fakeP(h, p, &sh.k, Some(sh.c))));
            let r1 = FR[fi].2.clone();
            fi += 1;
            let r0 = FR[fi].2.clone();
            fi += 1;
            let (r1, r0) = match (r1, r0) {
                (Some(a), Some(b)) => (a, b),
                _ => continue,
            };
            let r1v = r1.v.unwrap_or(f64::NAN);
            let r0v = jmin(r0.v.unwrap_or(f64::NAN), vB);
            let pAll = il.iter().fold(1.0, |a, p| a * self.fakeP(h, p, &sh.k, Some(sh.c)));
            let mut v = pAll * r1v + (pf - pAll) * (r1v + r0v) / 2.0 + (1.0 - pf) * r0v;
            if h.id >= 0.0 {
                if k0.is_none() {
                    k0 = Some(jmax(0.0, jmin(1.0, self.keepP(h, src, None, T))));
                    let Vk = self.V(src, 1.0 - team, h.x, h.y, 0.3);
                    L0 = (match kon {
                        Some(f) => jmax(Vk, f(h.x, h.y)),
                        None => Vk,
                    }) * riskOf(self, h, tac);
                }
                let k0v = k0.unwrap();
                let kY = (1.0 + k0v) / 2.0;
                v = pAll * r1v + (pf - pAll) * (kY * (r1v + r0v) / 2.0 - (1.0 - kY) * L0) + (1.0 - pf) * (k0v * r0v - (1.0 - k0v) * L0);
            }
            v -= tac.map_or(0.5, |t| t.tempo) * self.D.bekTempo * (T / 30.0);
            opts.push(Secenek {
                kind: "fake".into(),
                show: Some(crate::mac::gecici_dis::FakeShow { k: sh.k.clone(), to: (sh.to.x, sh.to.y), c: sh.c }),
                shot: Some(format!("{} göster → {} · inanılırlık %{}", sh.k, r1.kind, jround(100.0 * sh.c) + 0.0)),
                to: Some(crate::model::Nokta { x: sh.to.x, y: sh.to.y }),
                v: Some(v),
                det: Some(JODet { P: Some(pf), sonra: Some(r1.kind.clone()), T: Some(T), c: Some(sh.c), ..Default::default() }),
                ..Default::default()
            });
        }
    }
}
