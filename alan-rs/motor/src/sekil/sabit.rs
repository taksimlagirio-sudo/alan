//! Yerleşimin parametreleri (JS `S`, `GR`, `BLOK`, `GEN`, `DUR_A`).
#![allow(non_snake_case, non_upper_case_globals)]

use crate::saha;

pub const GW: f64 = saha::W;
pub const GH: f64 = saha::H;
pub const GCY: f64 = saha::CY;
pub const GKX: f64 = saha::W / 100.0;
pub const GKY: f64 = saha::H / 50.0;

/// JS `S.W`: boşluk aramasının tartıları.
#[derive(Clone, Copy, Debug)]
pub struct SW {
    pub run_sprint: f64,
    pub deep: f64,
    pub tv: f64,
    pub tv_pass: f64,
    pub tv_drive: f64,
    pub dl_reach: f64,
    pub def_reach: f64,
    pub leash: f64,
    pub hyst: f64,
    pub noise: f64,
}

/// JS `S` (AlanShape.S).
#[derive(Clone, Copy, Debug)]
pub struct Sp {
    pub bek_serbest: bool,
    pub err_pos: f64,
    pub look: f64,
    pub lag0: f64,
    pub lagOk: f64,
    pub gizle_t: f64,
    pub react: f64,
    pub reactLoose: f64,
    pub reactLooseOk: f64,
    pub margin_t: f64,
    pub bek_once: f64,
    pub bek_cik_r: f64,
    pub bek_cik_pay: f64,
    pub bek_cik_sur: f64,
    pub bek_cik_saha: f64,
    pub bek_sup_l: f64,
    pub bek_sup_max: f64,
    pub bek_sup_arka: f64,
    pub bek_sup_k: f64,
    pub chase_margin: f64,
    pub destek_k: f64,
    pub zone_r: f64,
    pub zone_ok: f64,
    pub zone_dng: f64,
    pub ayr: f64,
    pub tv_egim: f64,
    pub kos_min: f64,
    pub kos_hiz: f64,
    pub ev_ayr: f64,
    pub ev_ayr_bek: f64,
    pub ev_bolge: f64,
    pub sut_kapat_r: f64,
    pub en_yer_k: f64,
    pub en_yer_ust: f64,
    pub en_yer_din: f64,
    pub ortak_min: f64,
    pub ortak_r: f64,
    pub pr_t0: f64,
    pub pr_still: f64,
    pub pr_cost: f64,
    pub pr_pmin: f64,
    pub pr_dis: f64,
    pub pr_hold: f64,
    pub mark_off: f64,
    pub mark_w: f64,
    pub lock: f64,
    pub run_k: f64,
    pub vac_ok0: f64,
    pub vac_ok_step: f64,
    pub vac_left: f64,
    pub vac_empty: f64,
    pub vac_leash: f64,
    pub durus_el: f64,
    pub kanat_k: f64,
    pub cek_k: f64,
    pub cek_etiket: f64,
    pub ucK: f64,
    pub bind_k: f64,
    pub bind_yol: f64,
    pub bind_on: f64,
    pub kurucu_k: f64,
    pub pivot_gap: f64,
    pub rec_t: f64,
    pub rec_ok: f64,
    pub rcv_risk: f64,
    pub rcv_margin_t: f64,
    pub rcv_buf: f64,
    pub W: SW,
}

pub const S: Sp = Sp {
    bek_serbest: true,
    err_pos: 0.14,
    look: 0.6,
    lag0: 10.0,
    lagOk: 0.4,
    gizle_t: 4.0,
    react: 0.5,
    reactLoose: 6.0,
    reactLooseOk: 0.25,
    margin_t: 0.6,
    bek_once: 2.0,
    bek_cik_r: 22.0 * GKX,
    bek_cik_pay: 3.0,
    bek_cik_sur: 6.0,
    bek_cik_saha: 4.0,
    bek_sup_l: 35.0 * GKX,
    bek_sup_max: 18.0 * GKX,
    bek_sup_arka: 5.0,
    bek_sup_k: 0.4,
    chase_margin: 0.015,
    destek_k: 0.06,
    zone_r: 7.0,
    zone_ok: 0.5,
    zone_dng: 9.0,
    ayr: 4.5,
    tv_egim: 0.25,
    kos_min: 2.0,
    kos_hiz: 8.0,
    ev_ayr: 6.0,
    ev_ayr_bek: 14.0,
    ev_bolge: 5.0,
    sut_kapat_r: 34.0,
    en_yer_k: 0.03,
    en_yer_ust: 0.75,
    en_yer_din: 0.2,
    ortak_min: 0.3,
    ortak_r: 3.5,
    pr_t0: 22.0,
    pr_still: 1.2,
    pr_cost: 0.55,
    pr_pmin: 0.3,
    pr_dis: 0.08,
    pr_hold: 120.0,
    mark_off: 2.5,
    mark_w: 0.75,
    lock: 40.0,
    run_k: 0.6,
    vac_ok0: 6.0,
    vac_ok_step: 4.0,
    vac_left: 7.0,
    vac_empty: 6.0,
    vac_leash: 0.4,
    durus_el: 0.5,
    kanat_k: 0.12,
    cek_k: 1.0,
    cek_etiket: 0.012,
    ucK: 0.05,
    bind_k: 0.08,
    bind_yol: 3.5,
    bind_on: 4.0,
    kurucu_k: 0.08,
    pivot_gap: 0.0,
    rec_t: 120.0,
    rec_ok: 1.2,
    rcv_risk: 1.0,
    rcv_margin_t: 10.0,
    rcv_buf: 3.0,
    W: SW { run_sprint: 4.0, deep: 6.0, tv: 1.0, tv_pass: 0.9, tv_drive: 0.85, dl_reach: 4.0, def_reach: 5.0, leash: 0.12, hyst: 0.04, noise: 0.01 },
};

/// JS `S.prHat[blok]`: pres hattı (kale çizgisinden).
pub fn pr_hat(blok: &str) -> Option<f64> {
    match blok {
        "Düşük" => Some(42.0 * GKX),
        "Orta" => Some(62.0 * GKX),
        "Yüksek" => Some(100.0 * GKX),
        _ => None,
    }
}
/// JS `BLOK[blok]`: savunma bloğunun derinliği.
pub fn BLOK(blok: &str) -> Option<f64> {
    match blok {
        "Düşük" => Some(22.0 * GKX),
        "Orta" => Some(34.0 * GKX),
        "Yüksek" => Some(50.0 * GKX),
        _ => None,
    }
}
/// JS `GEN[genislik]`: hücum genişliği.
pub fn GEN(g: &str) -> Option<f64> {
    match g {
        "Dar" => Some(28.0 * GKY),
        "Normal" => Some(38.0 * GKY),
        "Geniş" => Some(46.0 * GKY),
        _ => None,
    }
}

/// JS `GR`: görüşün parametreleri.
#[derive(Clone, Copy, Debug)]
pub struct Grp {
    pub aci: f64,
    pub tar0: f64,
    pub tar_k: f64,
    pub tar_min: f64,
    pub blok: f64,
    pub hata: f64,
    pub uz0: f64,
    pub yas_h: f64,
    pub yas_c: f64,
    pub yas_b: f64,
}
pub const gr: Grp = Grp { aci: 100.0, tar0: 70.0, tar_k: 3.0, tar_min: 12.0, blok: 15.0, hata: 0.07, uz0: 15.0, yas_h: 40.0, yas_c: 0.21, yas_b: 12.0 };
impl Grp {
    /// JS `GR.yasHata(L)`.
    pub fn yas_hata(&self, l: f64) -> f64 {
        self.yas_c * l * l / (l + self.yas_b)
    }
}

/// JS `DUR_A`: duran top dizilişinin üç dayanak derinliği.
pub const DUR_A: [f64; 3] = [20.0 * GKX, 50.0 * GKX, 80.0 * GKX];
