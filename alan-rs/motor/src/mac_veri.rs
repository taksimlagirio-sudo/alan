//! Maçın veri tipleri (MIMARI.md Aşama 7a): temel veri modeli (`model`) bunlara başvurur, bu yüzden temel
//! katmanda; maç katmanı eski yerlerinden yeniden dışa açar (`mac::Tac` gibi eski yollar çalışmaya devam eder).
#![allow(non_snake_case)]
use crate::model::Secenek;
use serde::Deserialize;

/// Diziliş yeri (JS {d, l}).
#[derive(Clone, Copy, Debug, Default, Deserialize)]
pub struct Yer {
    #[serde(default)]
    pub d: f64,
    #[serde(default)]
    pub l: f64,
}

/// Duruş noktası (JS tac.durus.huc[][]: {f, y}).
#[derive(Clone, Debug, Default, Deserialize)]
pub struct DurusN {
    #[serde(default)]
    pub f: Option<f64>,
    #[serde(default)]
    pub y: Option<f64>,
}
#[derive(Clone, Debug, Default, Deserialize)]
pub struct Durus {
    #[serde(default)]
    pub huc: Vec<Vec<DurusN>>,
    #[serde(default)]
    pub sav: Vec<Vec<DurusN>>,
}

/// Koşul (JS tac.kosul[]: {ne, son, ayar}).
#[derive(Clone, Debug, Default, Deserialize)]
pub struct Kosul {
    #[serde(default)]
    pub ne: Option<String>,
    #[serde(default)]
    pub son: Option<f64>,
    #[serde(default)]
    pub ayar: Option<Tac>,
}

/// Maçın kalan zamanının değeri (JS macEff'in `macZ` kapanışı): veri olarak.
#[derive(Clone, Copy, Debug, Default)]
pub struct MacZ {
    pub d: f64,
    pub kal: f64,
    pub birim: f64,
}

macro_rules! tac_yap {
    ($($a:ident : $t:ty),* $(,)?) => {
        /// Takım talimatı (JS `m.tac[t]` ve tacEff'in döndürdüğü kopya). Bütün alanlar seçimli:
        /// JS `{...a, ...b}` = `a.ust(&b)`; okuyan `??` varsayılanını kendisi verir.
        #[derive(Clone, Debug, Default, Deserialize)]
        pub struct Tac {
            $( #[serde(default)] pub $a: Option<$t>, )*
            /// (tacEff) karşı pres işareti
            #[serde(skip)]
            pub _kp: Option<String>,
            /// (tacEff) maç durumu katı ve zaman değeri
            #[serde(skip)]
            pub macK: Option<f64>,
            #[serde(skip)]
            pub macZ: Option<MacZ>,
        }
        impl Tac {
            /// JS `{...self, ...o}`
            pub fn ust(&self, o: &Tac) -> Tac {
                Tac {
                    $( $a: o.$a.clone().or_else(|| self.$a.clone()), )*
                    _kp: o._kp.clone().or_else(|| self._kp.clone()),
                    macK: o.macK.or(self.macK),
                    macZ: o.macZ.or(self.macZ),
                }
            }
        }
    };
}
tac_yap! {
    blok: String, pres: f64, onde: f64, arkada: f64, genislik: String, risk: f64, tempo: f64, kazaninca: String,
    tuzak: String, sistem: String, siddet: f64, savGen: f64, gecisH: String, kpYer: String, kpSure: f64,
    kaleOnu: String, pasSurme: f64, pasBoy: f64, sabir: f64, sut: f64, enDinlen: f64, enYipratma: f64,
    enHarcama: f64, tetik: Vec<String>, durus: Durus, yz: bool, kayip: f64, gecisEsik: f64,
    kosul: Vec<Kosul>, _ev: String, kontra: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MacW {
    pub g: f64,
    pub c: f64,
    pub e0: f64,
}

/// JS `m.events[]` öğesi.
#[derive(Clone, Debug, Default)]
pub struct Olay {
    pub t: String,
    pub r#type: String,
    pub team: f64,
    pub text: String,
}

/// JS `m._perde`.
#[derive(Clone, Debug, Default)]
pub struct Perde {
    pub cid: f64,
    pub qid: f64,
    pub until: f64,
}

/// JS `p.kenarW`.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct KenarW {
    pub x: f64,
    pub y: f64,
    pub sonra: Option<(f64, f64)>,
    pub until: f64,
}

/// Diziliş (JS "3-2-1" ya da [{d, l}…]).
#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum DizV {
    Dizi(Vec<Yer>),
    Metin(String),
}

/// JS `m.diz[t]` = {sav, huc}
#[derive(Clone, Debug, Default, Deserialize)]
pub struct Diz {
    #[serde(default)]
    pub sav: Option<DizV>,
    #[serde(default)]
    pub huc: Option<DizV>,
}

/// Kafanın kararı (JS r = {best, opts, T}): best, opts içindeki sıra (JS nesne kimliği).
#[derive(Clone, Debug, Default)]
pub struct Karar {
    pub opts: Vec<Secenek>,
    pub best: Option<usize>,
    pub T: f64,
}

/// Fake'in gösterdiği (JS o.show = {k, to, c}).
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct FakeShow {
    pub k: String,
    pub to: (f64, f64),
    pub c: f64,
}

/// Enerji sayacı (JS {sprint: 0, ivme: 0, …}): ekleme sırasıyla.
pub type EnSayac = Vec<(String, f64)>;
