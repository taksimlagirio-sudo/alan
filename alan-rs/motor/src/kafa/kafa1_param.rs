//! Kafa parametreleri (decide.js:6–130 DU, D; 148 GON; 547 PAS; 550 SAV; 1361 BKS, UC3, UC5; 4639 SDQ).
#![allow(non_upper_case_globals)]

use crate::model::KafaD;

/// İkili parametreleri (JS `DU`).
#[derive(Clone, Copy, Debug)]
pub struct Du {
    pub enFark: f64,
    pub base: f64,
    pub skill: f64,
    pub cage: f64,
    pub side: f64,
    pub core0: f64,
    pub coreV: f64,
    pub trap: f64,
    pub trap0: f64,
    pub trapR: f64,
    pub trapA: f64,
    pub thrCover: f64,
    pub thrLast: f64,
    pub stun: f64,
    pub stunOk: f64,
    pub engage: f64,
    pub cd: f64,
    pub shieldK: f64,
    pub openK: f64,
    pub fakeT: f64,
    pub oyala: f64,
    pub oyalaC: f64,
    pub oyalaR: f64,
    pub oyalaD: f64,
}
pub const DU: Du = Du {
    enFark: crate::core_sabit::EN_FARK,
    base: 0.3,
    skill: 0.03,
    cage: 0.14,
    side: 0.16,
    core0: 0.6,
    coreV: 3.0,
    trap: 0.6,
    trap0: 0.15,
    trapR: 4.5,
    trapA: 0.8,
    thrCover: 0.3,
    thrLast: 0.5,
    stun: 22.0,
    stunOk: 0.5,
    engage: 2.4,
    cd: 14.0,
    shieldK: 0.6,
    openK: 0.15,
    fakeT: 9.0,
    oyala: 0.25,
    oyalaC: 0.1,
    oyalaR: 26.0,
    oyalaD: 3.2,
};

pub use crate::kural::sut::{Sav, SAV};

/// Gönderme hatasının durum katları (JS `GON`).
pub struct Gon {
    pub taban: f64,
    pub baski: f64,
    pub hareket: f64,
    pub guc: f64,
}
pub const GON: Gon = Gon { taban: 1.0, baski: 0.6, hareket: 0.4, guc: 0.3 };
pub const BODY_R: f64 = 1.0;
pub const BODY_E: f64 = 0.35;
pub const MIND: f64 = 1.7;
pub const SECIM_P: f64 = 0.1;
pub const BKS: f64 = 0.12;
/// Uçuş belirsizliğinin örnek noktaları (JS UC3, UC5): [z, ağırlık, hız z'si]
pub const UC3: [[f64; 3]; 3] = [[0.0, 0.5833, 0.0], [0.632, 0.20835, 0.0], [-0.632, 0.20835, 0.0]];
pub const UC5: [[f64; 3]; 5] = [[0.0, 0.1666, 0.0], [0.632, 0.20835, 0.0], [-0.632, 0.20835, 0.0], [0.0, 0.20835, 0.632], [0.0, 0.20835, -0.632]];
pub const BEK_YON: [f64; 5] = [-60.0, -30.0, 0.0, 30.0, 60.0];

/// JS `D`'nin başlangıç değerleri.
pub fn d_js() -> KafaD {
    KafaD {
        simMax: 180.0,
        errOk: 0.05,
        errPos: 3.0,
        aimErr: 0.035,
        v0Err: 0.04,
        arrive: 0.3,
        arriveSpace: 0.08,
        vmax: 3.0,
        shotV: 2.6,
        plaseV: 1.7,
        riskBase: 1.0,
        surKarar: 0.025,
        asanPay: 3.0,
        sutHazir: 5.0,
        threatK: crate::core_sabit::TEHDIT_K,
        sideK: crate::core_sabit::TEHDIT_YAN,
        b0: 0.06,
        b1: 0.28,
        recvKeepT: 8.0,
        shotThru: 0.15,
        bekTempo: 0.05,
        otErr: 1.2,
        otMargin: 0.02,
        passMaxF: 80.0,
        useSpace: true,
        aimPow: 0.45,
        aimPlase: 0.75,
        arriveFirm: 0.8,
        vK: 0.35,
        combo: 0.9,
        chgMin: 0.1,
        sarjUfuk: 30.0,
        chg0: 0.0035,
        chgTek: 2.5,
        chgPasif: 0.4,
        enHizV: 0.3,
        enUfuk: 40.0,
        enTmin: 15.0,
        kontraL: 35.0,
        zoneMin: 0.45,
        zoneD: 32.0,
        bankOk: 8.0,
        bankIf: 0.8,
        bankN: 3.0,
        trials: 2.0,
        nFull: 9.0,
        yokD: 0.03,
        nMates: 6.0,
        shotRange: 34.0,
        ..Default::default()
    }
}
