//! Dış arayüz: uygulamaların (telefon, bilgisayar) ve komut satırının kullandığı yüz.
//!
//! Maç modülü `MacMotoru`'nu gerçekler; uygulama kayıttan maç kurar, tik ilerletir, her tik bir `Kare` alır.
//! `Kare`, JS işçisinin karesinin (mac-isci.js kaydet, 174 sayı) tipli karşılığıdır: çizim için gereken her şey.

use crate::model::Oyuncu;

/// Bir oyuncunun karedeki hâli.
#[derive(Clone, Debug, Default)]
pub struct KareOyuncu {
    pub x: f32,
    pub y: f32,
    /// hedef (yoksa NaN)
    pub tx: f32,
    pub ty: f32,
    /// görev yazısının sırası (`Kare::isler` sözlüğünde)
    pub is: u16,
    /// bar (yoksa NaN)
    pub en: f32,
    /// ALAN seçimi: +1 hızlandır, −1 yavaşlat, 0 kapalı
    pub am: i8,
    /// denge (yoksa NaN)
    pub den: f32,
    /// ikili durumu: 1 sendeliyor, 2 ayak basmış (bit)
    pub ikili: u8,
    /// sprint evresi (sürekli koşuda hep 0; 1 patlama, 2 tepe, 3 fren, 4 dolum)
    pub evre: u8,
}

/// Bir tikin karesi.
#[derive(Clone, Debug, Default)]
pub struct Kare {
    pub tick: u32,
    pub oyuncular: Vec<KareOyuncu>,
    /// Çekirdeğin yeri (yoksa NaN)
    pub top_x: f32,
    pub top_y: f32,
    /// taşıyıcı (m.ps sırası) ya da −1
    pub sahip: i32,
    /// şarj (taşıyıcıdaysa m.ch, değilse topun ch'si)
    pub sarj: f32,
    pub skor: [u16; 2],
    /// iki takımın evresi (EVKOD sırası, yoksa −1)
    pub evre: [i8; 2],
    /// taşıyıcı şarj ediyor
    pub sarj_ediyor: bool,
    /// itilmiş açık topun iteni (−1) ve duracağı yer
    pub iten: i32,
    pub it_x: f32,
    pub it_y: f32,
}

/// Görev yazıları sözlüğü: yazı bir kez gönderilir, karede sırası.
#[derive(Clone, Debug, Default)]
pub struct IsSozluk {
    pub yazilar: Vec<String>,
    sira: std::collections::HashMap<String, u16>,
}
impl IsSozluk {
    pub fn no(&mut self, j: &str) -> u16 {
        if let Some(n) = self.sira.get(j) {
            return *n;
        }
        let n = self.yazilar.len() as u16;
        self.yazilar.push(j.to_string());
        self.sira.insert(j.to_string(), n);
        n
    }
}

impl Kare {
    /// Oyunculardan kareyi kur (top, sahip, skor gibi maç değerleri çağırandan).
    #[allow(clippy::too_many_arguments)]
    pub fn kur(tick: u32, ps: &[Oyuncu], isler: &mut IsSozluk, top: Option<(f64, f64)>, sahip: Option<usize>, sarj: f64, skor: [f64; 2], evre: [i8; 2], sarj_ediyor: bool, holder_takim: Option<f64>, holder_xy: Option<(f64, f64)>) -> Kare {
        let nan = f32::NAN;
        let oyuncular = ps
            .iter()
            .map(|p| {
                let mut ik = 0u8;
                if p.sendT.unwrap_or(0.0) > tick as f64 {
                    ik |= 1;
                }
                if let (Some(ht), Some((hx, hy))) = (holder_takim, holder_xy) {
                    if ht != p.team && crate::mat::hypot(p.vx, p.vy) < 0.06 && crate::mat::hypot(hx - p.x, hy - p.y) < 4.0 {
                        ik |= 2;
                    }
                }
                KareOyuncu {
                    x: p.x as f32,
                    y: p.y as f32,
                    tx: p.tx as f32,
                    ty: p.ty as f32,
                    is: isler.no(p.job.as_deref().unwrap_or("")),
                    en: p.en.map(|e| e as f32).unwrap_or(nan),
                    am: if p.amA { p.am.unwrap_or(0.0) as i8 } else { 0 },
                    den: p.den.map(|e| e as f32).unwrap_or(nan),
                    ikili: ik,
                    evre: 0,
                }
            })
            .collect();
        Kare {
            tick,
            oyuncular,
            top_x: top.map(|t| t.0 as f32).unwrap_or(nan),
            top_y: top.map(|t| t.1 as f32).unwrap_or(nan),
            sahip: sahip.map(|s| s as i32).unwrap_or(-1),
            sarj: sarj as f32,
            skor: [skor[0] as u16, skor[1] as u16],
            evre,
            sarj_ediyor,
            iten: -1,
            it_x: nan,
            it_y: nan,
        }
    }
}

/// Maç izi satırı (sina/iz-js.js ile aynı biçim): istatistik.js iki motoru aynı ölçüyle okur.
#[derive(Clone, Debug)]
pub struct IzSatiri {
    pub tick: u32,
    pub sahip: i32,
    pub skor: [u16; 2],
    /// yeni uçuş: (tür, atan, hedef)
    pub yeni_ucus: Option<(String, i32, i32)>,
    pub sut: bool,
    pub bar: [f64; 2],
}
impl IzSatiri {
    pub fn json(&self) -> String {
        let y = match &self.yeni_ucus {
            Some((k, a, q)) => format!("[{},{},{}]", serde_json::to_string(k).unwrap(), a, q),
            None => "0".into(),
        };
        format!("[{},{},{},{},{},{},{},{}]", self.tick, self.sahip, self.skor[0], self.skor[1], y, self.sut as u8, (self.bar[0] * 1e4).round() / 1e4, (self.bar[1] * 1e4).round() / 1e4)
    }
}

/// Bir takımın bar ortalaması (yoksa 1).
pub fn bar_ort(ps: &[Oyuncu], takim: f64) -> f64 {
    let (mut s, mut n) = (0.0, 0.0);
    for p in ps {
        if p.team == takim {
            if let Some(e) = p.en {
                s += e;
                n += 1.0;
            }
        }
    }
    if n > 0.0 {
        s / n
    } else {
        1.0
    }
}

/// Maç motorunun yüzü: maç modülü gerçekler.
pub trait MacMotoru: Sized {
    /// Kayıttan (kayit-*.json'un `yeni` nesnesi) maç kur.
    fn kayittan(yeni: &serde_json::Value) -> Result<Self, String>;
    /// Bir tik ilerle.
    fn adim(&mut self);
    fn tick(&self) -> u32;
    fn bitti(&self) -> bool;
    fn kare(&self, isler: &mut IsSozluk) -> Kare;
    /// Bu tikin iz satırı (adımdan sonra çağrılır).
    fn iz(&mut self) -> IzSatiri;
}
