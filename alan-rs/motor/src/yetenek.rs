//! Yetenekler (yetenek.js): oyuncu kaydının özelliklerinden 34 özellik ve 101 mekanik.
//!
//! JS'te `ALAN_Y(p, 'mekanik')` her çağrıda kaydın önbelleğinden okur; burada `Yetenek`
//! maç başında bir kez kurulur (oyuncunun sabit satırı) ve doğrudan dizi okunur.

use crate::mat::{jmax, jmin, kis, sin};
pub use crate::yetenek_tablo::{mek, oz, MEK_AD, MEK_DURUM, MEK_PAY, N_MEK, N_OZ, OZ_AD};
use std::collections::HashMap;

/// Oyuncu kaydındaki ham özellikler (JS `p.a`): eski kayıtta `hiz, ceviklik, guc, …, gorus, tepki`,
/// yeni kayıtta OZ adları. Sayı olmayanlar (taraf) ayrı.
#[derive(Clone, Debug, Default)]
pub struct Ham {
    pub v: HashMap<String, f64>,
    pub taraf: Option<String>,
}

impl Ham {
    fn al(&self, k: &str) -> Option<f64> {
        self.v.get(k).copied()
    }
    /// JS ALAN_OZ: kendi adıyla, yoksa eski adının karşılığıyla, yoksa 10.
    pub fn oz_eski(&self, k: &str) -> f64 {
        if let Some(v) = self.al(k) {
            return v;
        }
        let e = match k {
            "tepki" | "gorus" | "yeralma" | "refleks" | "yerlesim" => Some("okuma"),
            "pas" | "gonderme" => Some("aktarim"),
            "kontrol" | "yukleme" | "hakimiyet" => Some("tutus"),
            "guc" | "uzanma" => Some("yogunluk"),
            _ => None,
        };
        if let Some(e) = e {
            if let Some(w) = self.al(e) {
                return w;
            }
        }
        10.0
    }
    /// JS `yeni(a, k)`: kayıtta varsa o, yoksa eşleme (ESLE), yoksa ALAN_OZ.
    fn yeni(&self, k: &str) -> f64 {
        if let Some(v) = self.al(k) {
            return v;
        }
        let o = |n: &str| self.oz_eski(n);
        match k {
            "patlayicilik" => o("hiz"),
            "denge" => (o("guc") + o("ceviklik")) / 2.0,
            "govde" => 10.0,
            "dayaniklilik" => o("enerji"),
            "toparlanma" => o("enerji"),
            "ilkKontrol" => o("kontrol"),
            "teknik" => (o("surme") + o("kontrol")) / 2.0,
            "bitiricilik" => o("gonderme"),
            "uzakSut" => o("gonderme"),
            "aldatma" => (o("surme") + (o("pas") + o("gorus")) / 2.0) / 2.0,
            "alanKontrol" => o("alan"),
            "okuma" => (o("tepki") + o("gorus")) / 2.0,
            "vizyon" => o("gorus"),
            "yaraticilik" => o("gorus"),
            "karar" => o("gorus"),
            "caliskanlik" => 10.0,
            "hucumYer" => o("yeralma"),
            "savunmaYer" => o("yeralma"),
            "markaj" => o("yeralma"),
            "birebir" => (o("refleks") + o("yerlesim")) / 2.0,
            _ => o(k),
        }
    }
}

/// Bir oyuncunun yetenek satırı: 34 özellik, 101 mekanik ve sık okunan ham değerler.
#[derive(Clone, Debug)]
pub struct Yetenek {
    pub oz: [f64; N_OZ],
    pub mek: [f64; N_MEK],
    /// ham Hız (JS `p.a.hiz ?? 10`): koşu hızı ve sprint katı bundan
    pub hiz: f64,
    /// ham Çeviklik (JS `p.a.ceviklik ?? 10`)
    pub ceviklik: f64,
    /// JS ALAN_OZ(p, k) ile okunan eski adlar (gorus, tepki, guc, enerji, alan…)
    pub eski: HashMap<String, f64>,
    pub taraf: Option<String>,
}

const ESKI_ADLAR: [&str; 22] = ["hiz", "ceviklik", "guc", "uzanma", "alan", "kontrol", "surme", "pas", "gonderme", "yukleme", "tepki", "gorus", "yeralma", "sogukkanlilik", "kesme", "refleks", "yerlesim", "hakimiyet", "enerji", "govde", "caliskanlik", "okuma"];

impl Yetenek {
    pub fn kur(a: &Ham) -> Yetenek {
        let mut z = [0.0; N_OZ];
        for (i, k) in OZ_AD.iter().enumerate() {
            z[i] = a.yeni(k);
        }
        let mut m = [0.0; N_MEK];
        for i in 0..N_MEK {
            let mut s = 0.0;
            for &(o, w) in MEK_PAY[i] {
                s += w * z[o];
            }
            m[i] = s / 100.0;
        }
        let mut eski = HashMap::new();
        for k in ESKI_ADLAR {
            eski.insert(k.to_string(), a.oz_eski(k));
        }
        Yetenek { oz: z, mek: m, hiz: a.al("hiz").unwrap_or(10.0), ceviklik: a.al("ceviklik").unwrap_or(10.0), eski, taraf: a.taraf.clone() }
    }
    /// Kaydı olmayan oyuncu (JS: `p.a` yok → her şey 10).
    pub fn bos() -> Yetenek {
        Yetenek { oz: [10.0; N_OZ], mek: [10.0; N_MEK], hiz: 10.0, ceviklik: 10.0, eski: HashMap::new(), taraf: None }
    }
    #[inline(always)]
    pub fn y(&self, m: usize) -> f64 {
        self.mek[m]
    }
    /// JS ALAN_OZ(p, k) (eski adlarla okuma).
    pub fn ozk(&self, k: &str) -> f64 {
        self.eski.get(k).copied().unwrap_or(10.0)
    }
    /// Şutun isabeti mesafeyle (JS sutY): yakında Bitiricilik, uzakta Uzaktan şut.
    pub fn sut_y(&self, d: f64) -> f64 {
        let b = kis((SUT_UZAK - d) / (SUT_UZAK - SUT_YAKIN), 0.0, 1.0);
        let (mut s, mut bu) = (0.0, 0.0);
        for &(o, w) in MEK_PAY[mek::SUT_YON] {
            if o == oz::BITIRICILIK || o == oz::UZAK_SUT {
                bu += w;
            } else {
                s += w * self.oz[o];
            }
        }
        (s + bu * (b * self.oz[oz::BITIRICILIK] + (1.0 - b) * self.oz[oz::UZAK_SUT])) / 100.0
    }
    /// Çalışkanlık katı (JS caK).
    pub fn ca_k(&self) -> f64 {
        1.0 - 0.015 * (self.mek[mek::HARCAMA_ISTEGI] - 10.0)
    }
    /// Bir işin enerji verimi (JS verim): yetenek yüksekse bedel düşük.
    pub fn verim(&self, m: usize) -> f64 {
        kis(1.0 - 0.04 * (self.mek[m] - 10.0), 0.5, 1.4)
    }
}

impl Default for Yetenek {
    fn default() -> Self {
        Yetenek::bos()
    }
}

pub const SUT_YAKIN: f64 = 12.0;
pub const SUT_UZAK: f64 = 30.0;

/// Belirlenimli zar (0–1), oyuncu ve olayla (JS ALAN_YET.zar).
#[inline(always)]
pub fn zar(a: f64, b: f64, c: f64) -> f64 {
    let s = 43758.5453 * sin(12.9898 * a + 78.233 * b + 37.719 * c);
    s - s.floor()
}

/// Eşik çevresinde 4 puanlık rampa (JS kademe).
#[inline(always)]
pub fn kademe(v: f64, esik: f64) -> f64 {
    kis((v - esik + 2.0) / 4.0, 0.0, 1.0)
}

/// Bir mekaniğin içinde bir özelliğin payı (0–1).
pub fn agirlik(m: usize, o: usize) -> f64 {
    MEK_PAY[m].iter().find(|x| x.0 == o).map(|x| x.1 / 100.0).unwrap_or(0.0)
}

/// Sprint katı Hız'dan (JS ALAN_SPR_H): 10 → 1,25, 20 → 1,45, 0 → 1,05.
#[inline(always)]
pub fn spr_h(h: f64) -> f64 {
    jmax(1.05, jmin(1.45, 1.25 + 0.02 * (h - 10.0)))
}

/// İkili sonrası dengenin hıza ve dönüşe etkisi (JS ALAN_DURUM.dengeK; yavas: maçın IK.yavas).
#[inline(always)]
pub fn denge_k(den: Option<f64>, yavas: f64) -> f64 {
    match den {
        None => 1.0,
        Some(d) if d >= yavas => 1.0,
        Some(d) => 0.45 + 0.55 * jmax(0.0, d) / yavas,
    }
}

/// Dönüş çarpanı (JS ALAN_AG): Fren ve yön değiştirme; dengesi bozuk (Bekçi dışı) oyuncu yavaş döner.
pub fn ag(y: &Yetenek, den: Option<f64>, bekci: bool, yavas: f64) -> f64 {
    let g = kis(1.0 + 0.05 * (y.mek[mek::DONUS] - 10.0), 0.6, 1.35);
    match den {
        Some(d) if d < 1.0 && !bekci => g * denge_k(Some(d), yavas),
        _ => g,
    }
}

/// İlk itişin ölçeği: Patlayıcılık Hız'dan farklıysa onun koşu hızının oranı (JS itis).
fn itis(y: &Yetenek) -> f64 {
    let h = y.hiz;
    let x = y.oz[oz::PATLAYICILIK];
    if x == h {
        return 1.0;
    }
    let vr = |h: f64| (0.17 + 0.006 * h) * spr_h(h);
    vr(x) / vr(h)
}

/// Hızlanma çarpanı (JS ALAN_AH).
pub fn ah(y: &Yetenek) -> f64 {
    let k = kis(1.0 + 0.05 * (y.mek[mek::HIZLANMA] - 10.0), 0.6, 1.35);
    k * itis(y)
}

/// Durum katmanının katsayıları (JS ALAN_DURUM.DK): baskı B, denge D, hareket H.
pub struct Dk {
    pub b: f64,
    pub d: f64,
    pub h: f64,
}
pub const DK_PAS: Dk = Dk { b: 1.5, d: 1.5, h: 0.4 };
pub const DK_SUT: Dk = Dk { b: 0.6, d: 0.6, h: 0.4 };
pub const DK_ALIS: Dk = Dk { b: 1.0, d: 1.0, h: 2.0 / 3.0 };

/// Durumun etkisini yumuşatan özellik (B: Soğukkanlılık, H: Teknik).
pub fn yum(y: &Yetenek, d: &str) -> f64 {
    match d {
        "B" => jmax(0.2, 1.0 - 0.06 * (y.oz[oz::SOGUKKANLILIK] - 10.0)),
        "H" => jmax(0.2, 1.0 - 0.06 * (y.oz[oz::TEKNIK] - 10.0)),
        _ => 1.0,
    }
}

/// Durum değerleri (B baskı, D dengesizlik, H hareket, Hz, E).
#[derive(Clone, Copy, Default, Debug)]
pub struct Durum {
    pub b: f64,
    pub d: f64,
    pub h: f64,
    pub hz: f64,
    pub e: f64,
}

/// Bir mekaniğe durumun payı (JS ALAN_DURUM.pay).
pub fn durum_pay(y: &Yetenek, m: usize, k: &Dk, s: &Durum) -> f64 {
    let mut t = 0.0;
    for &d in MEK_DURUM[m] {
        let (c, v) = match d {
            "B" => (k.b, s.b),
            "D" => (k.d, s.d),
            "H" => (k.h, s.h),
            _ => (0.0, 0.0),
        };
        if c != 0.0 && v != 0.0 && v == v {
            t += c * v * yum(y, d);
        }
    }
    t
}
