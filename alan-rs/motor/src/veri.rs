//! Ölçülmüş tablolar (JS: shot-table.js, value-table.js). Bugünkü motorun kullandığı sürümler:
//! şut tablosu `ALAN_SHT_VE` (v5-22 · vuruş enerjisi), imkân haritasının şut tablosu `ALAN_IMK_SUT_VE`
//! (imk-11, parmak izi 104b9sy), topla oynamanın değeri `ALAN_VAL` (v5). Kütüphaneye gömülü.

use serde::Deserialize;
use std::collections::HashMap;
use std::sync::OnceLock;

/// Gönderme (şut) tablosu: hücre anahtarı → girme oranı (JS ALAN_SHT_VE.cells).
#[derive(Deserialize, Debug)]
pub struct SutTablosu {
    pub mouth: f64,
    pub kr: f64,
    pub direk: f64,
    pub surum: String,
    pub cells: Vec<(i64, f64)>,
}

/// İmkân haritasının şut tablosu (JS ALAN_IMK_SUT_VE): eksenler ve "mesafe-yan" anahtarlı değer dizileri.
#[derive(Deserialize, Debug)]
pub struct ImkanSut {
    pub surum: String,
    pub parmak: String,
    #[serde(rename = "DS")]
    pub ds: Vec<f64>,
    #[serde(rename = "SY")]
    pub sy: Vec<f64>,
    #[serde(rename = "KO")]
    pub ko: Vec<f64>,
    #[serde(rename = "KD")]
    pub kd: Vec<f64>,
    #[serde(rename = "CH")]
    pub ch: Vec<f64>,
    #[serde(rename = "L")]
    pub l: Vec<f64>,
    pub v: HashMap<String, Vec<f64>>,
}

/// Topla oynamanın değeri (JS ALAN_VAL).
#[derive(Deserialize, Debug)]
pub struct DegerTablosu {
    pub v: f64,
    pub nb: usize,
    pub ng: usize,
    pub cells: Vec<f64>,
}

pub fn sut_tablosu() -> &'static SutTablosu {
    static T: OnceLock<SutTablosu> = OnceLock::new();
    T.get_or_init(|| serde_json::from_str(include_str!("../veri/sut-tablosu.json")).expect("sut-tablosu.json"))
}
pub fn imkan_sut() -> &'static ImkanSut {
    static T: OnceLock<ImkanSut> = OnceLock::new();
    T.get_or_init(|| serde_json::from_str(include_str!("../veri/imkan-sut.json")).expect("imkan-sut.json"))
}
pub fn deger_tablosu() -> &'static DegerTablosu {
    static T: OnceLock<DegerTablosu> = OnceLock::new();
    T.get_or_init(|| serde_json::from_str(include_str!("../veri/deger-tablosu.json")).expect("deger-tablosu.json"))
}
