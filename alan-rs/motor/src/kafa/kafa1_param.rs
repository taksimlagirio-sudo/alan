//! Kafa parametreleri (decide.js:6–130 DU, D; 148 GON; 547 PAS; 550 SAV; 1361 BKS, UC3, UC5; 4639 SDQ).
#![allow(non_upper_case_globals)]

use crate::model::KafaD;

/// İkili parametreleri (JS `DU`). `enFark`, `oyalaC`, `oyalaR`, `thrLast` maçın `DU` sabitleriyle aynı token: adlandırmada atlandı.
#[allow(non_snake_case)]
#[derive(Clone, Copy, Debug)]
pub struct Du {
    pub enFark: f64,
    pub base: f64,
    pub skill: f64,
    pub cage: f64,
    pub side: f64,
    pub core0: f64,
    pub core_v: f64,
    pub trap: f64,
    pub trap0: f64,
    pub trap_r: f64,
    pub trap_a: f64,
    pub thr_cover: f64,
    pub thrLast: f64,
    pub stun: f64,
    pub stun_ok: f64,
    pub engage: f64,
    pub cd: f64,
    pub shield_k: f64,
    pub open_k: f64,
    pub fake_t: f64,
    pub oyala: f64,
    pub oyalaC: f64,
    pub oyalaR: f64,
    pub oyala_d: f64,
}
pub const DU: Du = Du {
    enFark: crate::core_sabit::EN_FARK,
    base: 0.3,
    skill: 0.03,
    cage: 0.14,
    side: 0.16,
    core0: 0.6,
    core_v: 3.0,
    trap: 0.6,
    trap0: 0.15,
    trap_r: 4.5,
    trap_a: 0.8,
    thr_cover: 0.3,
    thrLast: 0.5,
    stun: 22.0,
    stun_ok: 0.5,
    engage: 2.4,
    cd: 14.0,
    shield_k: 0.6,
    open_k: 0.15,
    fake_t: 9.0,
    oyala: 0.25,
    oyalaC: 0.1,
    oyalaR: 26.0,
    oyala_d: 3.2,
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
        sim_max: 180.0,
        err_ok: 0.05,
        err_pos: 3.0,
        aim_err: 0.035,
        v0_err: 0.04,
        arrive: 0.3,
        arrive_space: 0.08,
        vmax: 3.0,
        shot_v: 2.6,
        plase_v: 1.7,
        risk_base: 1.0,
        surKarar: 0.025,
        asan_pay: 3.0,
        sut_hazir: 5.0,
        threat_k: crate::core_sabit::TEHDIT_K,
        side_k: crate::core_sabit::TEHDIT_YAN,
        b0: 0.06,
        b1: 0.28,
        recv_keep_t: 8.0,
        shot_thru: 0.15,
        bek_tempo: 0.05,
        ot_err: 1.2,
        otMargin: 0.02,
        pass_max_f: 80.0,
        use_space: true,
        aim_pow: 0.45,
        aim_plase: 0.75,
        arriveFirm: 0.8,
        v_k: 0.35,
        combo: 0.9,
        chg_min: 0.1,
        sarj_ufuk: 30.0,
        chg0: 0.0035,
        chg_tek: 2.5,
        chg_pasif: 0.4,
        en_hiz_v: 0.3,
        en_ufuk: 40.0,
        en_tmin: 15.0,
        kontra_l: 35.0,
        zone_min: 0.45,
        zone_d: 32.0,
        bank_ok: 8.0,
        bank_if: 0.8,
        bank_n: 3.0,
        trials: 2.0,
        n_full: 9.0,
        yok_d: 0.03,
        n_mates: 6.0,
        shot_range: 34.0,
        ..Default::default()
    }
}
