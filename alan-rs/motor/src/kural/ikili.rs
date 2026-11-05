//! İkili fiziğinin kuralları (saf): gövdenin kütlesi, direnci, omuz ve dürtme katsayıları, sendeleme süresi,
//! denge kaybının koşuya bedeli. İkilinin maçtaki uygulaması (itme, ayırma, enerji) `mac::ikili`'de.
use crate::mat::{hypot, jmax, jmin, jround};
use crate::model::Oyuncu;
use crate::oyuncu::{oo, or0};
use crate::yetenek::mek;

/// JS IK: ikili parametreleri.
pub struct Ik {
    pub yerles: f64,
    pub esnek: f64,
    pub surt: f64,
    pub dengeK: f64,
    pub dolum: f64,
    pub dolumS: f64,
    pub yavas: f64,
    pub sendele: f64,
    pub sendT: f64,
    pub sendCd: f64,
    pub kalkan: f64,
    pub kalkanR: f64,
    pub pokeT: f64,
    pub pokeK: f64,
    pub pokeAc: f64,
    pub pokeDn: f64,
    pub omuzV: f64,
}

pub const IK: Ik = Ik { yerles: 1.6, esnek: 0.15, surt: 0.12, dengeK: 3.2, dolum: 0.012, dolumS: 0.03, yavas: 0.35, sendele: 0.15, sendT: 12.0, sendCd: 40.0, kalkan: 0.7, kalkanR: 3.2, pokeT: 0.22, pokeK: 0.03, pokeAc: 0.5, pokeDn: 0.35, omuzV: 0.2 };

pub const fn ikOn() -> bool {
    true
}

/// Enerjinin ikiliye etkisi (Enerji 3.0: barın yarısı kadar).
pub fn ikEn(p: &Oyuncu) -> f64 {
    match p.en {
        None => 1.0,
        Some(e) => 1.15 * (0.5 + 0.5 * jmax(0.0, jmin(1.0, e))),
    }
}

pub fn omuzK(q: &Oyuncu) -> f64 {
    (1.0 + 0.03 * (q.yet.y(mek::OMUZ) - 10.0)) * ikEn(q)
}

pub fn durtFark(q: &Oyuncu, h: &Oyuncu) -> f64 {
    q.yet.y(mek::DURTME) - h.yet.y(mek::DURTME_KORU)
}

/// JS kutle(p, h): ph = (p === h); yerinde duran (taşıyıcı dışı) daha ağır.
pub fn kutle(p: &Oyuncu, ph: bool) -> f64 {
    (1.0 + 0.05 * (p.yet.y(mek::KUTLE) - 10.0)) * if !ph && hypot(or0(p.vx), or0(p.vy)) < 0.06 { IK.yerles } else { 1.0 }
}

pub fn direnc(p: &Oyuncu, ph: bool) -> f64 {
    (0.8 + 0.05 * (p.yet.y(if ph { mek::TOPLU_DENGE } else { mek::DENGE_DIRENC }) - 10.0)) * if !ph && hypot(or0(p.vx), or0(p.vy)) < 0.06 { 1.8 } else { 1.0 } * ikEn(p)
}

/// Sendeleme süresi (tik).
pub fn sendS(p: &Oyuncu) -> f64 {
    jmax(2.0, jround(IK.sendT - 0.5 * (p.yet.y(mek::SENDELEME) - 10.0)))
}

pub(crate) fn dengeDolumK(p: &Oyuncu) -> f64 {
    1.0 + 0.0625 * (p.yet.y(mek::DENGE_DOLUM) - 10.0)
}

/// JS dengeKayip(p, t): dengesizliğin önümüzdeki tiklerde götüreceği koşu (tik cinsinden).
pub fn dengeKayip(p: &Oyuncu, t: f64) -> f64 {
    let Some(mut dn) = p.den else { return 0.0 };
    let mut l = 0.0;
    let dm = dengeDolumK(p);
    for k in 0..150 {
        let sd = oo(p.sendT) > t + k as f64;
        if !sd && dn >= IK.yavas {
            break;
        }
        let mut f = if sd { 0.3 } else { 1.0 };
        if dn < IK.yavas {
            f *= 0.45 + 0.55 * dn / IK.yavas;
        }
        l += 1.0 - f;
        dn = jmin(1.0, dn + (if sd { IK.dolumS } else { IK.dolum }) * dm);
    }
    l
}
