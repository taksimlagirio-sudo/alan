//! Saha ölçüleri ve top fiziğinin parametreleri (core.js: ALAN_SAHA, P, W/H/KR/MOUTH/DIREK).
#![allow(non_snake_case)]

/// Saha boyu (br). JS: `ALAN_SAHA_BOY || [120, 60]`.
pub const W: f64 = 120.0;
pub const H: f64 = 60.0;
pub const CY: f64 = H / 2.0;
/// Kuyu yarıçapı (çizim), ağız yarı genişliği, direk yarıçapı.
pub const KR: f64 = 9.0;
pub const MOUTH: f64 = 5.0;
pub const DIREK: f64 = 0.3;

/// Alan ızgaraları (yarım birim ve tam birim).
pub const GX1: usize = (2.0 * W) as usize + 1;
pub const GY1: usize = (2.0 * H) as usize + 1;
pub const GW1: usize = W as usize + 1;
pub const GH1: usize = H as usize + 1;
pub const GXM: f64 = if W == 100.0 { 99.999 } else { W - 0.001 };
pub const GYM: f64 = if H == 50.0 { 49.999 } else { H - 0.001 };

/// Saniyedeki tik, maçın boyu (tik).
pub const TPS: f64 = 30.0;
pub const MAC_LEN: f64 = 300.0 * TPS;

/// Top (Çekirdek) fiziği: sürtünme, rakip alanının yavaşlatması, enerji kaybı, duvar sekmesi.
#[derive(Clone, Debug)]
pub struct Fizik {
    pub fric: f64,
    pub roll: f64,
    pub opp_drag: f64,
    pub pierce: f64,
    pub chg_v: f64,
    pub own_flow: f64,
    pub attn: f64,
    pub drain: f64,
    pub dead_ramp: f64,
    pub dead_max: f64,
    pub rest: f64,
    pub wall_loss: f64,
    pub e0: f64,
    pub tek_s: f64,
    pub tek_gain: f64,
    pub tek_drain: f64,
    pub tek_drag: f64,
    pub tek_taban: f64,
}

pub const P: Fizik = Fizik {
    fric: 0.01,
    roll: 0.003,
    opp_drag: 0.07,
    pierce: 1.0,
    chg_v: 0.9,
    own_flow: 0.55,
    attn: 0.003,
    drain: 0.09,
    dead_ramp: 0.004,
    dead_max: 0.06,
    rest: 0.72,
    wall_loss: 0.08,
    e0: 1.0,
    tek_s: 0.35,
    tek_gain: 0.025,
    tek_drain: 0.6,
    tek_drag: 0.35,
    tek_taban: 0.2,
};

impl Default for Fizik {
    fn default() -> Self {
        P
    }
}

// Maç modülünden taşındı (Aşama 7b): sahanın ve zamanın temel yardımcıları.
/// JS ALAN_SAHA.KX, KY (saha ölçeği: 100 × 50'ye göre)
pub const KX: f64 = W / 100.0;
pub const KY: f64 = H / 50.0;
#[inline(always)]
pub fn dir_of(t: f64) -> f64 {
    if t == 0.0 {
        1.0
    } else {
        -1.0
    }
}

#[inline(always)]
pub fn own_x(t: f64) -> f64 {
    if t == 0.0 {
        0.0
    } else {
        W
    }
}

/// JS `SN()`: saniyedeki tik.
#[inline(always)]
pub fn SN() -> f64 {
    TPS
}

/// threat (decide.js): team takımının kalesine (atak yönü) yakınlık; k: uzaklık ölçeği, yan: kenar cezası.
#[inline]
pub fn tehdit(k: f64, yan: f64, team: f64, x: f64, y: f64) -> f64 {
    let gx = if team == 0.0 { W } else { 0.0 };
    let d = crate::mat::hypot(x - gx, y - CY);
    crate::mat::exp(-d / k) * (1.0 - yan * (y - CY).abs() / CY)
}
