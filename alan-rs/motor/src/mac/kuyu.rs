//! Kuyu (match.js 2009–2094): Kuyu alanında süre, sönme, alandan çıkış yönü ve süresi.
use super::enerji::{spd, sprK, vK, MacKural};
use super::*;
use crate::core_sabit::Q;
use crate::mat::{atan2, cos, hypot, jmax, jmin, sin};
use crate::saha::{CY, H};
use crate::temas::{alan_ic, kuyu_girer, kuyu_mer};

/// JS kuyuCikYon sonucu
#[derive(Clone, Copy, Debug)]
pub struct CikYon {
    pub k: f64,
    pub ux: f64,
    pub uy: f64,
    pub L: f64,
    pub x: f64,
    pub y: f64,
}

/// JS kuyuCikYon(m, p): Kuyu alanından en kısa (önü açık) çıkış.
pub fn kuyuCikYon(m: &Mac, pi: Pid) -> Option<CikYon> {
    let p = &m.ps[pi];
    let gx = if hypot(p.x, p.y - CY) < hypot(p.x - W, p.y - CY) { 0.0 } else { W };
    let a0 = atan2(p.y - CY, p.x - gx);
    let mut en: Option<CikYon> = None;
    for da in [0.0, -0.5, 0.5, -1.0, 1.0] {
        let a = a0 + da;
        let ux = cos(a);
        let uy = sin(a);
        let rx = p.x - gx;
        let ry = p.y - CY;
        let b = rx * ux + ry * uy;
        let c = rx * rx + ry * ry - Q.kuyu_a * Q.kuyu_a;
        let l = -b + jmax(0.0, b * b - c).sqrt() + 0.5;
        let ex = p.x + ux * l;
        let ey = p.y + uy * l;
        if ex < 1.0 || ex > W - 1.0 || ey < 1.0 || ey > H - 1.0 {
            continue;
        }
        let mut kap = false;
        for (qi, q) in m.ps.iter().enumerate() {
            if qi == pi {
                continue;
            }
            let t = jmax(0.0, jmin(l, (q.x - p.x) * ux + (q.y - p.y) * uy));
            if hypot(p.x + ux * t - q.x, p.y + uy * t - q.y) < 1.4 {
                kap = true;
                break;
            }
        }
        let k = l + if kap { 6.0 } else { 0.0 };
        if en.map_or(true, |e| k < e.k) {
            en = Some(CikYon { k, ux, uy, L: l, x: ex, y: ey });
        }
    }
    en
}

/// JS kuyuCikT(m, p, Y): alandan çıkmanın süresi (tik), ivmeyle benzetim.
pub fn kuyuCikT(m: &Mac, pi: Pid, y: Option<CikYon>, k: &MacKural) -> f64 {
    let p = &m.ps[pi];
    let gx = if hypot(p.x, p.y - CY) < hypot(p.x - W, p.y - CY) { 0.0 } else { W };
    let mut q = p.clone();
    q.vx = or0(p.vx);
    q.vy = or0(p.vy);
    let vS = spd(p) * sprK(p, k);
    let Some(y) = y.or_else(|| kuyuCikYon(m, pi)) else { return 40.0 };
    for t in 1..=40 {
        super::hareket::ivme(&mut q, y.ux * vS, y.uy * vS, vS);
        q.x += q.vx;
        q.y += q.vy;
        if hypot(q.x - gx, q.y - CY) >= Q.kuyu_a {
            return t as f64 + if y.k > y.L { 4.0 } else { 0.0 };
        }
    }
    40.0
}

/// JS kuyuSaat(m): Kuyu alanındaki süre; dolunca sönük, dolmadan çıkış; hedefi alanın içindeyse dışına al.
pub fn kuyuSaat(m: &mut Mac, mt: f64, mH: Option<Pid>) {
    let tb = match m.holder {
        Some(h) => Some((m.ps[h].x, m.ps[h].y)),
        None => m.ball.as_ref().map(|b| (b.x, b.y)),
    };
    let topK = match tb {
        Some((x, y)) if kuyu_mer(x, y) < Q.kuyu_a => {
            if x < W / 2.0 {
                0.0
            } else {
                1.0
            }
        }
        _ => -1.0,
    };
    m._kuyuTop = Some(topK);
    let kk = MacKural::of(m);
    for pi in 0..m.ps.len() {
        if m.ps[pi].bekci() {
            continue;
        }
        let holder = m.holder == Some(pi);
        let (px, py) = (m.ps[pi].x, m.ps[pi].y);
        let d = kuyu_mer(px, py);
        let ic = d < Q.kuyu_a;
        let dur = ic && topK == if px < W / 2.0 { 0.0 } else { 1.0 };
        if dur {
            m.ps[pi].hal_mut()._kuyuCik = false;
            if !m.ps[pi].hal.sonuk {
                continue;
            }
        }
        if ic {
            if !holder {
                m.ps[pi].hal_mut().kuyuT = Some(oo(m.ps[pi].hal.kuyuT) + 1.0);
            }
            if !m.ps[pi].hal.sonuk && !holder && oo(m.ps[pi].hal.kuyuT) >= Q.kuyu_sn {
                m.ps[pi].hal_mut().sonuk = true;
                m.ps[pi].hal_mut().hamle = None;
                let (t, ad) = (m.ps[pi].team, m.ps[pi].govde.name.clone());
                ev(m, "pas", t, format!("{ad} Kuyu alanında süreyi doldurdu · sönük"));
            }
        } else {
            m.ps[pi].hal_mut().kuyuT = Some(0.0);
            m.ps[pi].hal_mut().sonuk = false;
        }
        let p = &m.ps[pi];
        if !ic || p.hal.sonuk || holder || p.hal._kuyuCik {
            if !ic {
                m.ps[pi].hal_mut()._kuyuCik = false;
            }
        } else if oo(p.hal.kuyuT) + kuyuCikT(m, pi, None, &kk) + 1.0 >= Q.kuyu_sn {
            m.ps[pi].hal_mut()._kuyuCik = true;
        }
        let p = &m.ps[pi];
        if p.hal.sonuk || (p.hal._kuyuCik && !holder) {
            if let Some(y) = kuyuCikYon(m, pi) {
                m.ps[pi].tx = cl(y.x + 0.7 * y.ux, 1.0, W - 1.0);
                m.ps[pi].ty = cl(y.y + 0.7 * y.uy, 1.0, H - 1.0);
            }
            let p = &mut m.ps[pi];
            p.sprint = true;
            p.job = Some(if p.hal.sonuk { "sönük · alandan çıkıyor" } else { "Kuyu alanından çıkıyor" }.into());
        } else if !holder && (ic || alan_ic(p.tx, p.ty)) {
            let v = jmax(0.05, spd(p) * vK(p, mH == Some(pi), mt, &kk));
            let tv = hypot(p.tx - p.x, p.ty - p.y) / v;
            let topa = bas(job(p), &["kovala", "alıcı"]) || m.fl.as_ref().map_or(false, |f| f.q == Some(pi));
            if !kuyu_girer(p, p.tx, p.ty, tv, !topa, &kk) {
                let gx = if hypot(p.tx, p.ty - CY) < hypot(p.tx - W, p.ty - CY) { 0.0 } else { W };
                let dx = p.tx - gx;
                let dy = p.ty - CY;
                let l = { let l = hypot(dx, dy); if dogru(l) { l } else { 1.0 } };
                let r = Q.kuyu_a + 0.5;
                let p = &mut m.ps[pi];
                p.tx = cl(gx + dx / l * r, 1.5, W - 1.5);
                p.ty = cl(CY + dy / l * r, 1.5, H - 1.5);
            }
        }
    }
}
