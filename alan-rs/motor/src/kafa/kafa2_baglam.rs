//! Kafa 2'nin bağlamı: JS `decide.js` kapanışının (IIFE) 4717–8890 satırlarındaki modül düzeyi
//! durum ve önbellekler, açık yapı olarak. Ortak durum `D` → `KafaD` (`Kafa::D`).
//!
//! Dünya (JS `src`, `w`): oyuncu nesnelerinin dizisi. JS'te hayali dünyalar `{...p}` kopyalarıyla
//! kurulur ve değişmeyen oyuncular aynı nesne olarak paylaşılır; kimlik karşılaştırmaları
//! (`Rw[i] !== p`, `b2 !== w[i]`) bu paylaşıma dayanır. Burada aynısı: `Dunya = Vec<Arc<Oyuncu>>`,
//! kopya `Arc::new(..)`, kimlik `Arc::ptr_eq`. Sıra: `Pid` = dünyadaki sıra (JS'te `src.indexOf`).
#![allow(non_snake_case, non_upper_case_globals, dead_code)]

use crate::mat::dogru;
use crate::model::{JMTacE, KafaD, Oyuncu};

use std::sync::Arc;

pub type Op = Arc<Oyuncu>;
pub type Dunya = Vec<Op>;
pub type Pid = usize;

/// Zar (JS `rnd`: maçın zar dizisi ya da hayali dünyanın zarı).
pub type Zar<'a> = dyn FnMut() -> f64 + 'a;

pub use crate::model::Nokta;
#[inline(always)]
pub fn nk(x: f64, y: f64) -> Nokta {
    Nokta { x, y }
}
pub trait Konum {
    fn xy(&self) -> Nokta;
}
impl Konum for Nokta {
    fn xy(&self) -> Nokta {
        *self
    }
}
impl Konum for Oyuncu {
    fn xy(&self) -> Nokta {
        Nokta { x: self.x, y: self.y }
    }
}

/// JS `{...p, x, y}`
pub fn kxy(p: &Oyuncu, x: f64, y: f64) -> Op {
    let mut q = p.clone();
    q.x = x;
    q.y = y;
    Arc::new(q)
}
/// JS `{...p, x, y, vx, vy}`
pub fn kxyv(p: &Oyuncu, x: f64, y: f64, vx: f64, vy: f64) -> Op {
    let mut q = p.clone();
    q.x = x;
    q.y = y;
    q.vx = vx;
    q.vy = vy;
    Arc::new(q)
}
/// JS `{...p, vx: p.vx || 0, vy: p.vy || 0}` (hareket eden kopya; `_o` geri bağı sıra kimliğiyle)
pub fn kv(p: &Oyuncu) -> Oyuncu {
    let mut q = p.clone();
    q.vx = veya(p.vx, 0.0);
    q.vy = veya(p.vy, 0.0);
    q
}

/// JS `a || b` (sayı): 0 ve NaN yanlış.
#[inline(always)]
pub fn veya(a: f64, b: f64) -> f64 {
    if dogru(a) {
        a
    } else {
        b
    }
}
/// JS `a || b` (Option<sayı>)
#[inline(always)]
pub fn veyo(a: Option<f64>, b: f64) -> f64 {
    match a {
        Some(v) if dogru(v) => v,
        _ => b,
    }
}
#[inline(always)]
pub fn hyp2(x: f64, y: f64) -> f64 {
    crate::mat::hypot(x, y)
}
/// JS `Math.hypot(a.x - b.x, a.y - b.y)`
#[inline(always)]
pub fn uz(ax: f64, ay: f64, bx: f64, by: f64) -> f64 {
    crate::mat::hypot(ax - bx, ay - by)
}
/// JS `/^pres/.test(job || "")`
#[inline(always)]
pub fn pres_mi(job: &Option<std::sync::Arc<str>>) -> bool {
    job.as_deref().map(|j| j.starts_with("pres")).unwrap_or(false)
}
#[inline(always)]
pub fn bekci(p: &Oyuncu) -> bool {
    &*p.role == "Bekçi"
}
/// JS `0 === team`
#[inline(always)]
pub fn t0(team: f64) -> bool {
    team == 0.0
}
/// JS `(+x).toFixed(n)` sonra `+` (gösterim için yuvarlama)
pub fn sabitle(x: f64, n: i32) -> f64 {
    let s = format!("{:.*}", n as usize, x);
    s.parse().unwrap_or(x)
}
/// JS `src.find(z => z.id === r.id && z.team === r.team)` → sıra
pub fn bul_i(w: &[Op], id: f64, team: f64) -> Option<usize> {
    w.iter().position(|z| z.id == id && z.team == team)
}
/// Nesne kimliği (JS `a === b`); `Op`'lar için de geçerli (aynı paylaşımlı nesne, aynı adres).
pub use super::kafa1_js::ayni_nesne;

/// Etkin taktik (JS `tacEff(m, t)`: `m.tac[t]` + maç durumu: `macK`, `macZ`, `kontra`, kp, yz).
/// Kafa yalnız şu alanları okur; maç döngüsü kurar.
#[derive(Clone, Default)]
pub struct Tac {
    pub risk: Option<f64>,
    pub tempo: Option<f64>,
    pub sabir: Option<f64>,
    pub sut: Option<f64>,
    pub pasSurme: Option<f64>,
    pub pasBoy: Option<f64>,
    pub kontra: bool,
    pub macK: Option<f64>,
    /// JS `tac.macZ(dt)` (maç zamanının değeri)
    pub macZ: Option<Arc<dyn Fn(f64) -> f64 + Send + Sync>>,
    pub enDinlen: Option<f64>,
    pub enYipratma: Option<f64>,
    /// JS `tac.plan.taraf` ("sol" | "sağ")
    pub plan_taraf: Option<String>,
    /// geri kalan alanlar (yerleşim için)
    pub temel: Option<JMTacE>,
}
impl std::fmt::Debug for Tac {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tac{{risk:{:?},tempo:{:?},sabir:{:?},kontra:{}}}", self.risk, self.tempo, self.sabir, self.kontra)
    }
}

/// JS `D._mac` (kafanın maçtan okudukları): maç döngüsü kararın önünde doldurur.
#[derive(Clone, Debug, Default)]
pub struct MacGor {
    pub tick: f64,
    pub len: f64,
    pub tac: Vec<JMTacE>,
    pub ps: Dunya,
    /// topu tutan (sıra)
    pub holder: Option<Pid>,
}

/// JS `m._prT` (prensip tartısının izi; kafa yazar, maç okur)
#[derive(Clone, Debug, Default)]
pub struct PrT {
    pub team: f64,
    pub hId: f64,
    pub t: Option<f64>,
    /// arkadaş id → değer
    pub m: Vec<(f64, f64)>,
    /// "kid|did" → kazanç
    pub mm: Vec<(String, f64)>,
    /// rakip id → kazanç (ve bakılan rakipler `k`)
    pub hm: Option<(Vec<(f64, f64)>, Vec<f64>)>,
    pub h: f64,
}

impl PrT {
    /// Kafa1'in (girEsik) ve maçın okuduğu biçim (`Dis::prT`): maç `k.prT`'yi bununla verir.
    pub fn d1(&self) -> super::gecici_dis1::PrT {
        super::gecici_dis1::PrT {
            team: self.team,
            hId: Some(self.hId),
            t: self.t.unwrap_or(f64::NAN),
            h: Some(self.h),
            hm: self.hm.as_ref().map(|x| x.0.clone()),
            hmk: self.hm.as_ref().map(|x| x.1.clone()),
        }
    }
}

/// pwKos önbelleğinin satırı (JS PWM'in `e`)
#[derive(Clone, Debug)]
pub struct PwE {
    pub x0: f64,
    pub y0: f64,
    pub vx0: f64,
    pub vy0: f64,
    pub den: Option<f64>,
    pub gx0: f64,
    pub gy0: f64,
    pub vmax: f64,
    pub rc: f64,
    pub sw0: Option<Sw>,
    pub q: Oyuncu,
    pub t: usize,
    pub gx: f64,
    pub gy: f64,
    pub sw: Option<Sw>,
    pub S: Vec<f64>,
}
/// Koşu hedefinin değişimi (JS `sw`: `yer` varsa hedefe varınca, yoksa `t` anında)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sw {
    pub yer: bool,
    pub t: Option<f64>,
    pub x: f64,
    pub y: f64,
}

/// Kafa 2'nin modül düzeyi önbellekleri (JS: PWM/PWT, BSK/BSKt, BSC, IMKB/IMKBT,
/// IMKK/IMKKt, IMKO, IT, SAH_F, IMK_OFS, IMK_INAN_T, DRM).
#[derive(Default)]
pub struct Kafa2Durum {
    /// pwKos: oyuncu nesnesi (kimlik) → koşu satırları
    pub PWM: crate::HizliMap<usize, (Op, Vec<PwE>)>,
    pub PWT: Option<f64>,
    /// bskTab
    pub BSK: crate::HizliMap<String, Vec<f64>>,
    pub BSKt: Option<f64>,
    /// bosluk: dünya (kimlik) → anahtar → değer
    pub BSC: crate::HizliMap<usize, (Arc<Dunya>, f64, crate::HizliMap<String, f64>)>,
    /// imkBol önbelleği (tablo × konum)
    pub IMKB: crate::HizliMap<[u64; 4], Arc<std::sync::Mutex<crate::kafa::kafa2_imkan::ImkBol>>>,
    pub IMKBT: bool,
    /// imkKov: "team|id" → (tepki, hız)
    pub IMKK: crate::HizliMap<(u64, u64), (f64, f64)>,
    pub IMKKt: Option<Option<f64>>,
    /// ileriTab: (tick, anahtar, tablo)
    pub IT: Option<(f64, String, Option<Arc<crate::kafa::kafa2_zemin::IleriTab>>)>,
    pub IMK_OFS: f64,
    pub IMK_INAN_T: f64,
    /// JS `self.__sahF` (görünüm için)
    pub sahF_iz: Option<(Option<f64>, f64, Arc<crate::kafa::kafa2_zemin::Sahiplik>)>,
    /// JS `_noHold` / `_noFake` / `_dalgada` / `_kayipYap` (KafaD'de yok)
    pub _noHold: bool,
    pub _noFake: bool,
    pub _dalgada: bool,
    /// JS `h._sfNiyet` / `h._sfNiyetK` (tartı süresince taşıyanın sprint niyeti): (id, team, niyet, itK)
    pub sfNiyet: Option<(f64, f64, String, Option<f64>)>,
}

/// Kafanın bağlamı tektir: kafa1'in yapısı (`kafa1_baglam::Kafa`); kafa2'nin alanları (`mac`, `prT`, `k2`)
/// ona eklendi, sahiplik haritası `Kafa::SAH_F`.
pub use super::kafa1_baglam::Kafa;

/// JS `D`'nin başlangıç değerleri (tek kaynak: `kafa1_param::d_js`).
/// Kafanın değişmez ayarları (JS AlanDecide.D'nin sabit alanları): bir kez kurulur. Saf değerlendirmeler
/// (`pressAt_d`, `threat_d` …) bunu açıkça alır; kafanın kendi `D`'si bunun kopyası + geçici bağlam alanları.
pub fn ayar() -> &'static KafaD {
    static D: std::sync::OnceLock<KafaD> = std::sync::OnceLock::new();
    D.get_or_init(kafa_d)
}
pub fn kafa_d() -> KafaD {
    super::kafa1_param::d_js()
}

/// D'nin `_` alanlarını (`_tick` dışında) boşaltır (JS: `for (k of Object.keys(D)) if (k[0] === "_" && k !== "_tick") delete D[k]`).
pub fn d_alt_bosalt(d: &mut crate::model::KafaBag) {
    d._tekL = None;
    d._sut = None;
    d._zin = None;
    d._kuyruk = None;
    d._savT = None;
    d._savW = None;
    d._savU = None;
    d._savCa = None;
    d._kenar = None;
    d._kayipYap = None;
    d._gPr = None;
    d._enWD = None;
    d._enTm = None;
    d._kanca = None;
    d._ot = None;
    d._tekD = false;
    d._itme = None;
    d._gon = None;
    d._sYon = None;
}
