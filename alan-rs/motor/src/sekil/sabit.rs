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
    pub runSprint: f64,
    pub deep: f64,
    pub tv: f64,
    pub tvPass: f64,
    pub tvDrive: f64,
    pub dlReach: f64,
    pub defReach: f64,
    pub leash: f64,
    pub hyst: f64,
    pub noise: f64,
}

/// JS `S` (AlanShape.S).
#[derive(Clone, Copy, Debug)]
pub struct Sp {
    pub bekSerbest: bool,
    pub errPos: f64,
    pub look: f64,
    pub lag0: f64,
    pub lagOk: f64,
    pub gizleT: f64,
    pub react: f64,
    pub reactLoose: f64,
    pub reactLooseOk: f64,
    pub marginT: f64,
    pub bekOnce: f64,
    pub bekCikR: f64,
    pub bekCikPay: f64,
    pub bekCikSur: f64,
    pub bekCikSaha: f64,
    pub bekSupL: f64,
    pub bekSupMax: f64,
    pub bekSupArka: f64,
    pub bekSupK: f64,
    pub chaseMargin: f64,
    pub destekK: f64,
    pub zoneR: f64,
    pub zoneOk: f64,
    pub zoneDng: f64,
    pub ayr: f64,
    pub tvEgim: f64,
    pub kosMin: f64,
    pub kosHiz: f64,
    pub evAyr: f64,
    pub evAyrBek: f64,
    pub evBolge: f64,
    pub sutKapatR: f64,
    pub enYerK: f64,
    pub enYerUst: f64,
    pub enYerDin: f64,
    pub ortakMin: f64,
    pub ortakR: f64,
    pub prT0: f64,
    pub prStill: f64,
    pub prCost: f64,
    pub prPmin: f64,
    pub prDis: f64,
    pub prHold: f64,
    pub markOff: f64,
    pub markW: f64,
    pub lock: f64,
    pub runK: f64,
    pub vacOk0: f64,
    pub vacOkStep: f64,
    pub vacLeft: f64,
    pub vacEmpty: f64,
    pub vacLeash: f64,
    pub durusEl: f64,
    pub kanatK: f64,
    pub cekK: f64,
    pub cekEtiket: f64,
    pub ucK: f64,
    pub bindK: f64,
    pub bindYol: f64,
    pub bindOn: f64,
    pub kurucuK: f64,
    pub pivotGap: f64,
    pub recT: f64,
    pub recOk: f64,
    pub rcvRisk: f64,
    pub rcvMarginT: f64,
    pub rcvBuf: f64,
    pub W: SW,
}

pub const S: Sp = Sp {
    bekSerbest: true,
    errPos: 0.14,
    look: 0.6,
    lag0: 10.0,
    lagOk: 0.4,
    gizleT: 4.0,
    react: 0.5,
    reactLoose: 6.0,
    reactLooseOk: 0.25,
    marginT: 0.6,
    bekOnce: 2.0,
    bekCikR: 22.0 * GKX,
    bekCikPay: 3.0,
    bekCikSur: 6.0,
    bekCikSaha: 4.0,
    bekSupL: 35.0 * GKX,
    bekSupMax: 18.0 * GKX,
    bekSupArka: 5.0,
    bekSupK: 0.4,
    chaseMargin: 0.015,
    destekK: 0.06,
    zoneR: 7.0,
    zoneOk: 0.5,
    zoneDng: 9.0,
    ayr: 4.5,
    tvEgim: 0.25,
    kosMin: 2.0,
    kosHiz: 8.0,
    evAyr: 6.0,
    evAyrBek: 14.0,
    evBolge: 5.0,
    sutKapatR: 34.0,
    enYerK: 0.03,
    enYerUst: 0.75,
    enYerDin: 0.2,
    ortakMin: 0.3,
    ortakR: 3.5,
    prT0: 22.0,
    prStill: 1.2,
    prCost: 0.55,
    prPmin: 0.3,
    prDis: 0.08,
    prHold: 120.0,
    markOff: 2.5,
    markW: 0.75,
    lock: 40.0,
    runK: 0.6,
    vacOk0: 6.0,
    vacOkStep: 4.0,
    vacLeft: 7.0,
    vacEmpty: 6.0,
    vacLeash: 0.4,
    durusEl: 0.5,
    kanatK: 0.12,
    cekK: 1.0,
    cekEtiket: 0.012,
    ucK: 0.05,
    bindK: 0.08,
    bindYol: 3.5,
    bindOn: 4.0,
    kurucuK: 0.08,
    pivotGap: 0.0,
    recT: 120.0,
    recOk: 1.2,
    rcvRisk: 1.0,
    rcvMarginT: 10.0,
    rcvBuf: 3.0,
    W: SW { runSprint: 4.0, deep: 6.0, tv: 1.0, tvPass: 0.9, tvDrive: 0.85, dlReach: 4.0, defReach: 5.0, leash: 0.12, hyst: 0.04, noise: 0.01 },
};

/// JS `S.prHat[blok]`: pres hattı (kale çizgisinden).
pub fn prHat(blok: &str) -> Option<f64> {
    match blok {
        "Düşük" => Some(42.0 * GKX),
        "Orta" => Some(62.0 * GKX),
        "Yüksek" => Some(100.0 * GKX),
        _ => None,
    }
}
/// JS `S.pressR[blok]`.
pub fn pressR(blok: &str) -> Option<f64> {
    match blok {
        "Düşük" => Some(12.0),
        "Orta" => Some(20.0),
        "Yüksek" => Some(32.0),
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
    pub tarK: f64,
    pub tarMin: f64,
    pub blok: f64,
    pub hata: f64,
    pub uz0: f64,
    pub yasH: f64,
    pub yasC: f64,
    pub yasB: f64,
}
pub const GR: Grp = Grp { aci: 100.0, tar0: 70.0, tarK: 3.0, tarMin: 12.0, blok: 15.0, hata: 0.07, uz0: 15.0, yasH: 40.0, yasC: 0.21, yasB: 12.0 };
impl Grp {
    /// JS `GR.yasHata(L)`.
    pub fn yasHata(&self, l: f64) -> f64 {
        self.yasC * l * l / (l + self.yasB)
    }
}

/// JS `DUR_A`: duran top dizilişinin üç dayanak derinliği.
pub const DUR_A: [f64; 3] = [20.0 * GKX, 50.0 * GKX, 80.0 * GKX];
