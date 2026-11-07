//! Yerleşim (shape.js): topsuz herkesin işi ve hedefi (`position`), görüş (`bak`, `seen`, `gorulen`),
//! topun uçuşunu okuma (`flightRead`, `alici`), Bekçi'nin yeri (`bekci`), pres (`ilkPres`), savunma hattı
//! (`kapatHat`) ve varış süresi tabloları (`gec`, `gecTab`, `gecOf`).
//!
//! JS'te `window.AlanShape`. Kaynak: `alan-rs/kaynak/shape.js` (sade), anlam için `alan-v5/shape.js`.
//! Yabancı modüllerin (AlanDecide, AlanMatch) işlevleri `gecici_dis.rs`'te (`todo!()`).
#![allow(clippy::too_many_arguments)]

pub mod gecici_dis;
pub mod sabit;
pub mod varlik;
pub use crate::kural::gec;
pub mod gor;
pub mod ucus;
pub mod yer;
pub mod pres;
pub mod takim;
pub mod konum;

pub use gec::{gec, gec_of, gec_tab, gec_tab_v, gec_y, GecTab, GecY};
pub use gor::{bak, gorulen, seen, Gorulen};
pub use konum::position;
pub use pres::{ilk_pres, mac_pres, pres_lim, s7, S7H_ON, S7I_MAX, S7J_ARA, S7K, S7_D, S7_ZORLA};
pub use sabit::*;
pub use takim::{govde_f, kur_o, yer_v, yol_p, KurO, TvAday, cover_spots, kapat_hat, lane_open, metrics, team_v, tehlike, yol_t, Metrik, Tehlike};
pub use ucus::{menzil_o, menzil_p, okuT, alici, alici_direnc, flight_read, kendi_topu, Ar, Fr, Nokta};
pub use varlik::*;
pub use yer::{bekci, rslot_of, slot_of, BekM, Yer, alan_kosu_tp, hiz_yanilgi, pres_kesis_t, durus_def, durus_w, gside, koy_hedef, pres_kesis, slot_base, slot_base0, tas_hamle, yol_bag, yol_kesis, Kesis, YolBag, Yon};

use crate::mat::{jround, sin};
use crate::model::Oyuncu;
use crate::oyuncu::Tepki;
use crate::yetenek::mek;

/// JS `cl(v, a, b)`: `v < a ? a : v > b ? b : v` (NaN olduğu gibi kalır).
#[inline(always)]
pub fn cl(v: f64, a: f64, b: f64) -> f64 {
    if v < a {
        a
    } else if v > b {
        b
    } else {
        v
    }
}
#[inline(always)]
pub fn dir_of(t: u8) -> f64 {
    if t == 0 {
        1.0
    } else {
        -1.0
    }
}
#[inline(always)]
pub fn own_x(t: u8) -> f64 {
    if t == 0 {
        0.0
    } else {
        GW
    }
}
#[inline(always)]
pub fn opp_x(t: u8) -> f64 {
    if t == 0 {
        GW
    } else {
        0.0
    }
}

/// JS `hsh(a, b, c)`: tam sayı karması (Math.imul, >>> 0), [-0,5, 0,5).
pub fn hsh(a: f64, b: f64, c: f64) -> f64 {
    let ti = |x: f64| -> i32 { js_int32(x) };
    // JS: 73856093 * a ^ 19349663 * b ^ 83492791 * c (çarpımlar double, ^ ToInt32)
    let mut h: i32 = ti(73856093.0 * a) ^ ti(19349663.0 * b) ^ ti(83492791.0 * c);
    h = (h ^ ((h as u32) >> 13) as i32).wrapping_mul(1274126177);
    let u = (h ^ ((h as u32) >> 16) as i32) as u32;
    u as f64 / 4294967296.0 - 0.5
}

/// JS ToInt32.
pub fn js_int32(x: f64) -> i32 {
    if !x.is_finite() {
        return 0;
    }
    let t = x.trunc();
    let m = t.rem_euclid(4294967296.0);
    (m as u64 as u32) as i32
}
/// JS `x >>> 0`.
pub fn js_uint32(x: f64) -> u32 {
    js_int32(x) as u32
}

/// JS `(m.seed || 1) >>> 0`.
pub fn tohum(seed: f64) -> u32 {
    js_uint32(if seed != 0.0 && seed == seed { seed } else { 1.0 })
}

// Koşu, sprint ve tepki kuralları kural katmanında (MIMARI.md Aşama 1); eski adlarıyla buradan da görünür.
pub use crate::kural::enerji::{spd, alan_en2_sb as EN2_SB, alan_en2_ts as EN2_TS, alan_en_s as EN_S};
pub use crate::kural::kos::{lag_of, spre_k as alan_spre, tepki, v_sp, TEPKI};

// Yetenek kısayolları (ALAN_YET.acik(5/6/8) açık: hep yeni mekanikler).
#[inline(always)]
pub fn yer_s(p: &Oyuncu) -> f64 {
    p.govde.yet.y(mek::BOLGE)
}
#[inline(always)]
pub fn yer_m(p: &Oyuncu) -> f64 {
    p.govde.yet.y(mek::ADAM_YETIS)
}
#[inline(always)]
pub fn yer_h(p: &Oyuncu) -> f64 {
    p.govde.yet.y(mek::BOSLUK)
}
#[inline(always)]
pub fn yer_f(p: &Oyuncu) -> f64 {
    p.govde.yet.y(mek::BOS_FARK)
}
#[inline(always)]
pub fn niyet(p: &Oyuncu) -> f64 {
    p.govde.yet.y(mek::NIYET_SEZ)
}
#[inline(always)]
pub fn geri_k(p: &Oyuncu) -> f64 {
    1.0 + 0.015 * (p.govde.yet.y(mek::GERI_KOSU) - 10.0)
}
#[inline(always)]
pub fn kurtul(p: &Oyuncu) -> f64 {
    p.govde.yet.y(mek::MARKAJ_KURTUL)
}
#[inline(always)]
pub fn bek_y(p: &Oyuncu, m: usize) -> f64 {
    p.govde.yet.y(m)
}

/// JS `43758.5453 * sin(…)` kesir karması (yolBag/yolKesis'in hız yanılgısı).
pub fn kesir(s: f64) -> f64 {
    s - s.floor()
}
#[inline(always)]
pub fn sinh_k(a: f64) -> f64 {
    43758.5453 * sin(a)
}

