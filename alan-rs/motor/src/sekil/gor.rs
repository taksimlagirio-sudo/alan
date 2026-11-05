//! Görüş: her oyuncunun ötekilerden aklında tuttuğu (JS bak, `p._bk`), rakipleri nasıl gördüğü (seen) ve
//! kafaya verilen dünya (gorulen, gercek).
#![allow(non_snake_case)]

use std::sync::Arc;

use crate::mat::{hypot, jmax, jmin, jround};
use crate::model::{JP_bkE, Oyuncu, Pid};
use crate::oyuncu::Tepki;
use crate::yetenek::{mek, zar};

use super::*;

/// Görenin bir oyuncudan aklındaki son kare (JS `B[j]`) ya da yoksa geçmişten okunan.
fn hist_oku(q: &Oyuncu, dl: f64) -> (f64, f64, f64, f64) {
    match &q.hal.hist {
        Some(h) if !h.is_empty() => {
            let i = jmax(0.0, h.len() as f64 - 1.0 - dl) as usize;
            let e = &h[i];
            (e.x, e.y, e.vx, e.vy)
        }
        _ => (q.x, q.y, q.vx, q.vy),
    }
}

/// JS seen(m, p): `p`nin gördüğü rakipler (hayaletler). Aynı tikte önbellekli.
pub fn seen(c: &mut Mc, pi: Pid) -> Arc<Vec<Varlik>> {
    let t = c.m.tick;
    if c.ek.snT == Some(t) {
        if let Some(Some(v)) = c.ek.sn.get(pi) {
            return v.clone();
        }
    } else {
        c.ek.sn = Vec::new();
        c.ek.snT = Some(t);
    }
    let n = c.ps.len();
    let mut out: Vec<Varlik> = Vec::with_capacity(n);
    let hy: Vec<Option<Arc<Oyuncu>>> = (0..n).map(|j| if c.ps[j].team != c.ps[pi].team { Some(c.hayal(j)) } else { None }).collect();
    let k = c.k;
    let ps = &*c.ps;
    let p = &ps[pi];
    let e = (20.0 - p.yet.y(mek::ALGI_HATA)) * S.errPos;
    let dl = jmax(1.0, jround(TEPKI(p, Tepki::Algi, k)));
    let la = dl + (niyet(p) - 6.0) * S.look;
    let b = (t / 20.0).floor();
    let hayalet = |q: &Oyuncu, j: Pid, x: f64, y: f64, gvx: f64, gvy: f64| Varlik {
        x,
        y,
        vx: 0.0,
        vy: 0.0,
        gvx: Some(gvx),
        gvy: Some(gvy),
        team: q.takim(),
        bekci: q.bekci(),
        press: q.press,
        id: f64::NAN,
        r: Some(j),
        kim: None,
        beden: Beden::Ayri(hy[j].clone().unwrap()),
    };
    let duz = |q: &Oyuncu, j: Pid| {
        let (hx, hyy, hvx, hvy) = hist_oku(q, dl);
        hayalet(q, j, hx + hvx * la + 2.0 * hsh(p.id, q.id, b) * e, hyy + hvy * la + 2.0 * hsh(q.id, p.id, b + 7.0) * e, hvx, hvy)
    };
    let B = if !c.lite { p.hal._bk.as_ref() } else { None };
    match B {
        Some(B) if B.len() == n => {
            let eg = (20.0 - p.yet.y(mek::ALGI_HATA)) * GR.hata;
            let a = 100.0 * p.team + p.id;
            let kz = (tohum(c.m.seed) % 9973) as f64 * 1.618;
            let la2 = (niyet(p) - 6.0) * S.look;
            let H = GR.yasH;
            for j in 0..n {
                let q = &ps[j];
                if q.team == p.team {
                    continue;
                }
                let r = match &B[j] {
                    None => {
                        out.push(duz(q, j));
                        continue;
                    }
                    Some(r) => r,
                };
                let yas = t - r.t;
                let f = if yas < H { yas - yas * yas / (2.0 * H) } else { H / 2.0 };
                let kk = jmax(0.0, 1.0 - yas / H);
                let x0 = r.x + r.vx * f;
                let y0 = r.y + r.vy * f;
                let s = eg * jmin(2.0, hypot(x0 - p.x, y0 - p.y) / GR.uz0);
                let bq = 100.0 * q.team + q.id;
                let vx = r.vx * kk;
                let vy = r.vy * kk;
                out.push(hayalet(q, j, x0 + (2.0 * zar(a, bq, r.z + 31.0 + kz) - 1.0) * s + vx * la2, y0 + (2.0 * zar(bq, a, r.z + 37.0 + kz) - 1.0) * s + vy * la2, vx, vy));
            }
        }
        _ => {
            for (j, q) in ps.iter().enumerate() {
                if q.team != p.team {
                    out.push(duz(q, j));
                }
            }
        }
    }
    let out = Arc::new(out);
    if c.ek.sn.len() < n {
        c.ek.sn.resize(n, None);
    }
    c.ek.sn[pi] = Some(out.clone());
    out
}

/// JS bak(m): her oyuncu ötekilerin algı gecikmesi kadar eski karesini aklına yazar (`p._bk`); topu
/// tutmayan takımın markajdan kurtulan oyuncusu (markajKurtul > görenin niyetSez) daha da geç görülür.
pub fn bak(c: &mut Mc) {
    if c.lite {
        return;
    }
    let n = c.ps.len();
    let t = c.m.tick;
    let hb = c.m.holder;
    let at: f64 = match hb {
        Some(h) => c.ps[h].team,
        None => match &c.ball {
            Some(b) => b.b.team as f64,
            None => -1.0,
        },
    };
    let kur: Vec<Option<f64>> = (0..n).map(|j| {
        let q = &c.ps[j];
        if Some(j) == hb || q.bekci() || q.team != at {
            None
        } else {
            Some(kurtul(q))
        }
    }).collect();
    let k = c.k;
    for i in 0..n {
        let dl = jmax(1.0, jround(TEPKI(&c.ps[i], Tepki::Algi, k)));
        let nyP = niyet(&c.ps[i]);
        let pteam = c.ps[i].team;
        let mut B = match c.ps[i].hal_mut()._bk.take() {
            Some(b) if b.len() == n => std::sync::Arc::unwrap_or_clone(b),
            _ => vec![None; n],
        };
        for j in 0..n {
            if j == i {
                continue;
            }
            let q = &c.ps[j];
            let (hl, kf) = match &q.hal.hist {
                Some(h) if !h.is_empty() => (h.len(), jmin(dl, h.len() as f64 - 1.0)),
                _ => (0, 0.0),
            };
            let kk = kf as usize;
            let (hx, hy, hvx0, hvy0) = if kk > 0 { let e = &q.hal.hist.as_ref().unwrap()[hl - 1 - kk]; (e.x, e.y, e.vx, e.vy) } else { (q.x, q.y, q.vx, q.vy) };
            let (mut hvx, mut hvy) = (hvx0, hvy0);
            if kk > 0 && q.team != pteam {
                if let Some(kj) = kur[j] {
                    let d2 = jround(S.gizleT * jmax(0.0, kj - nyP) / 10.0);
                    if d2 > 0.0 {
                        let H0 = q.hal.hist.as_ref().unwrap();
                        let ix = jmin(kf + d2, hl as f64 - 1.0) as usize;
                        let e = &H0[hl - 1 - ix];
                        hvx = e.vx;
                        hvy = e.vy;
                    }
                }
            }
            B[j] = Some(JP_bkE { x: hx, y: hy, vx: hvx, vy: hvy, t: t - kf, z: 2.0 * (t / GR.blok).floor() });
        }
        c.ps[i].hal_mut()._bk = Some(std::sync::Arc::new(B));
    }
}

/// gorulen'in bir öğesi: gerçek oyuncu (`yas` yok) ya da görülen kopyası (JS `__g`, `__yas`, `__e`).
#[derive(Clone, Debug)]
pub struct Gorulen {
    pub v: Varlik,
    pub yas: Option<f64>,
    pub e: Option<f64>,
}

/// JS gorulen(m, p): kafaya verilen dünya: herkes `p`nin aklındaki yerinde (görülen kopya), kendisi ve
/// hiç görülmeyen gerçek. Rakip Bekçi'nin görülmeden geçen sürede kaleye döndüğü varsayılır.
pub fn gorulen(c: &mut Mc, pi: Pid) -> Vec<Gorulen> {
    let n = c.ps.len();
    let gercek = |c: &Mc, j: Pid| Gorulen { v: Varlik::oyuncu(j, &c.ps[j]), yas: None, e: None };
    if c.ps[pi].hal._bk.is_none() {
        return (0..n).map(|j| gercek(c, j)).collect();
    }
    let k = c.k;
    let t = c.m.tick;
    let mut out = Vec::with_capacity(n);
    let top = match &c.ball {
        Some(b) => Some((b.b.x, b.b.y)),
        None => c.m.holder.map(|h| (c.ps[h].x, c.ps[h].y)),
    };
    for j in 0..n {
        let ps = &*c.ps;
        let p = &ps[pi];
        let B = p.hal._bk.as_ref().unwrap();
        let q = &ps[j];
        let r = match B.get(j) {
            Some(Some(r)) if j != pi => r.clone(),
            _ => {
                out.push(gercek(c, j));
                continue;
            }
        };
        let e = (20.0 - p.yet.y(mek::ALGI_HATA)) * GR.hata;
        let a = 100.0 * p.team + p.id;
        let kz = (tohum(c.m.seed) % 9973) as f64 * 1.618;
        let yas = t - r.t;
        let H = GR.yasH;
        let f = if yas < H { yas - yas * yas / (2.0 * H) } else { H / 2.0 };
        let kk = jmax(0.0, 1.0 - yas / H);
        let x0 = r.x + r.vx * f;
        let y0 = r.y + r.vy * f;
        let s = e * jmin(2.0, hypot(x0 - p.x, y0 - p.y) / GR.uz0);
        let b = 100.0 * q.team + q.id;
        let (mut xs, mut ys, mut vxs, mut vys) = (x0, y0, r.vx * kk, r.vy * kk);
        if yas > 0.0 && q.bekci() && q.team != p.team {
            if let Some((tx, ty)) = top {
                let bm = BekM { ps: crate::sekil::yer::BekPs::Duz(ps), holder: c.m.holder.map(|h| crate::dunya::Yerde::oyuncu(&ps[h])), sut: c.m.mac.kafa.k.is_some(), ben: Some(j) };
                let hd = bekci(q, tx, ty, Some(&bm), k);
                let vm = spd(q) * ALAN_SPRE(q, k);
                let kH = 0.18 * crate::yetenek::ah(&q.yet);
                let (mut x, mut y, mut vx, mut vy) = (r.x, r.y, r.vx, r.vy);
                let mut u = 0.0;
                while u < jmin(yas, GR.yasH) {
                    let dx = hd.0 - x;
                    let dy = hd.1 - y;
                    let d = hypot(dx, dy);
                    let w = jmin(vm, d * kH * 2.0);
                    let wx = if d > 1e-6 { dx / d * w } else { 0.0 };
                    let wy = if d > 1e-6 { dy / d * w } else { 0.0 };
                    vx += (wx - vx) * kH;
                    vy += (wy - vy) * kH;
                    x += vx;
                    y += vy;
                    u += 1.0;
                }
                xs = x;
                ys = y;
                vxs = vx;
                vys = vy;
            }
        }
        let v = Varlik::kopya(j, q, xs + (2.0 * zar(a, b, r.z + 31.0 + kz) - 1.0) * s, ys + (2.0 * zar(b, a, r.z + 37.0 + kz) - 1.0) * s, vxs, vys);
        out.push(Gorulen { v, yas: Some(yas), e: Some(s) });
    }
    out
}

