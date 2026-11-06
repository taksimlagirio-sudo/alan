//! Hareketin kuralları (saf): istenen hıza ivmelenme (öne Hızlanma, frene ve yana Dönüş).
//! Tikin hareket adımı ve ilişkiler `mac::hareket`'te.
use crate::kural::enerji::SPR_FAZ;
use crate::kural::ikili::IK;
use crate::mat::{hypot, jmax, jmin};
use crate::model::Oyuncu;
use crate::yetenek::{ag, ah};

/// JS ivme(p, wx, wy, vmax): istenen hıza doğru ivmelen (öne Hızlanma, frene ve yana Dönüş).
pub fn ivme(p: &mut Oyuncu, wx: f64, wy: f64, vmax: f64) {
    let sp = hypot(p.vx, p.vy);
    let k0 = 0.18;
    let k_h = k0 * ah(&p.govde.yet) * if p._sf.as_ref().map_or(false, |s| s.f == "patlama") { SPR_FAZ.pat_k } else { 1.0 };
    if sp < 0.001 {
        p.vx += (wx - p.vx) * k_h;
        p.vy += (wy - p.vy) * k_h;
        return;
    }
    let ux = p.vx / sp;
    let uy = p.vy / sp;
    let dvx = wx - p.vx;
    let dvy = wy - p.vy;
    let al = dvx * ux + dvy * uy;
    let px = dvx - al * ux;
    let py = dvy - al * uy;
    let r = jmin(1.0, sp / jmax(0.05, vmax));
    let g = ag(&p.govde.yet, p.den, p.bekci(), IK.yavas);
    let k_a = if al >= 0.0 { k_h } else { k0 * g * (1.0 - 0.55 * r) };
    let k_t = k0 * g * (1.0 - 0.45 * r);
    let mut ba = al * k_a;
    let mut tx = px * k_t;
    let mut ty = py * k_t;
    {
        let bk_a = if p.bekci() { 2.0 } else { 1.0 };
        if ba < 0.0 {
            ba = jmax(ba, -0.018 * g * bk_a);
        }
        let tl = hypot(tx, ty);
        let tm = 0.021 * g * bk_a;
        if tl > tm && sp > 0.1 {
            tx *= tm / tl;
            ty *= tm / tl;
        }
    }
    p.vx += ux * ba + tx;
    p.vy += uy * ba + ty;
}

/// JS kuyuSafe: Kuyu açıkken null.
pub fn kuyu_safe(_x: f64, _y: f64) -> Option<(f64, f64)> {
    None
}
