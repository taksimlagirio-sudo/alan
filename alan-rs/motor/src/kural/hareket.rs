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
    let kH = k0 * ah(&p.govde.yet) * if p._sf.as_ref().map_or(false, |s| s.f == "patlama") { SPR_FAZ.patK } else { 1.0 };
    if sp < 0.001 {
        p.vx += (wx - p.vx) * kH;
        p.vy += (wy - p.vy) * kH;
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
    let kA = if al >= 0.0 { kH } else { k0 * g * (1.0 - 0.55 * r) };
    let kT = k0 * g * (1.0 - 0.45 * r);
    let mut ba = al * kA;
    let mut tx = px * kT;
    let mut ty = py * kT;
    {
        let bkA = if p.bekci() { 2.0 } else { 1.0 };
        if ba < 0.0 {
            ba = jmax(ba, -0.018 * g * bkA);
        }
        let tl = hypot(tx, ty);
        let tm = 0.021 * g * bkA;
        if tl > tm && sp > 0.1 {
            tx *= tm / tl;
            ty *= tm / tl;
        }
    }
    p.vx += ux * ba + tx;
    p.vy += uy * ba + ty;
}
