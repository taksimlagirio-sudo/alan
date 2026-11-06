//! Fake (decide.js:302–546 fakeP/fakeT/fakeHedef/fakeIlgili/fakeSecenek): bir vuruşu gösterip rakibi
//! aldatmak. Maçın uyguladığı kısım (fakeP/fakeT/fakeHedef/fakeIlgili). Kafanın fake seçeneği (fakeSecenek:
//! kandı/kanmadı dünyaları) yalnız eski kafa yolundan çağrılıyordu; o yol ölü olduğu için kaldırıldı.
#![allow(non_snake_case)]

use super::kafa1_baglam::Kafa;
use super::kafa1_js::*;
use crate::mat::{hypot, jmax, jmin, jround};
use crate::model::{JODet, Oyuncu};
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
pub fn al_p(d: Option<&JODet>) -> Option<f64> {
    d.and_then(|d| match (d.P, d.Ps) {
        (Some(P), Some(Ps)) => Some(jmax(0.0, P - Ps)),
        (P, _) => P,
    })
}

impl Kafa {
    /// Rakibin kanma olasılığı (decide.js:303).
    pub fn fake_p(&self, h: &Oyuncu, p: &Oyuncu, k: &str, c: Option<f64>) -> f64 {
        let ok = p.govde.yet.y(mek::FAKE_KAN);
        let sk = h.govde.yet.y(if k == "gövde" { mek::FAKE_GOVDE } else { mek::FAKE_INAN });
        let base = jmax(0.08, jmin(0.9, 0.5 + 0.04 * (sk - ok)));
        let cc = c.unwrap_or(1.0);
        let g = jmin(1.0, 0.5 + 0.03 * ok);
        base * (1.0 - g + g * cc)
    }
    /// Fake'in süresi (decide.js:307).
    pub fn fake_t(&self, h: &Oyuncu) -> f64 {
        jmax(8.0, jmin(18.0, jround(14.0 - 0.5 * (h.govde.yet.y(mek::FAKE_SURE) - 10.0))))
    }
    /// Kanan rakibin koşacağı yer (decide.js:308).
    pub fn fake_hedef(&self, p: &Oyuncu, h: &Oyuncu, sh: &FakeSh) -> Nk {
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
    pub fn fake_ilgili(&self, p: &Oyuncu, h: &Oyuncu, sh: &FakeSh) -> bool {
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
        let t = self.fake_hedef(p, h, sh);
        hypot(t.x - p.x, t.y - p.y) < 7.0
    }
}
