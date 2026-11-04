#![allow(non_snake_case, non_upper_case_globals)]
//! Kafa (decide.js): karar modülü.
//!
//! JS'te `decide.js` tek bir kapanıştır; ortak durum `D` ve modül düzeyi önbellekler/sabitler orada
//! gizlidir. Burada hepsi açık bir bağlam yapısındadır: [`Kafa`] (`kafa1_baglam.rs`). Kafa'nın iki
//! yarısı aynı yapının yöntemleridir: kafa1 (satır 1–4716: imkân, uçuş, şut, tutma/ikili, seçenekler)
//! ve kafa2 (satır 4717–: dünyalar, tartı, karar). Dış modüller (maç, yerleşim, enerji) [`Dis`]
//! özelliğinden çağrılır (`gecici_dis1.rs`).

pub mod gecici_dis1;
pub mod gecici_dis2;
pub mod kafa2_baglam;
pub mod kafa2_dunya;
pub mod kafa2_eski;
pub mod kafa2_imkan;
pub mod kafa2_karar;
pub mod kafa2_tarti;
pub mod kafa2_zemin;
pub mod kafa1_baglam;
pub mod kafa1_enerji;
pub mod kafa1_fake;
pub mod kafa1_hata;
pub mod kafa1_ikili;
pub mod kafa1_js;
pub mod kafa1_param;
pub mod kafa1_savur;
pub mod kafa1_secenek;
pub mod kafa1_secim;
pub mod kafa1_sut;
pub mod kafa1_sut_tablo;
pub mod kafa1_tehlike;
pub mod kafa1_ucus;
pub mod kafa1_varis;

pub use gecici_dis1::Dis;
pub use kafa1_baglam::Kafa;
pub use kafa1_js::Nk;
