//! Ortak veri modeli: TASLAK, çalışma anı envanterinden üretildi (sina/model-uret.py, sina/ic-sekil2.txt).
//! Alan adları JS adlarıyla birebir (izlenebilirlik). Tipler gözlemden: Option = null/undefined görüldü;
//! Pid = m.ps içindeki oyuncunun sırası; TopRef = topun kimliği (JS nesne kimliği / kopya).
//! Karışık tipli JS değerleri (JsDeger) gerçek tiplerine çevrildi (MIMARI.md 3.6, Aşama 5).
#![allow(non_snake_case, non_camel_case_types, dead_code)]
pub type Pid = usize;
pub type TopRef = u64;
#[derive(Clone, Debug, Default)] pub struct JsFn;
/// Kafanın bağlamındaki şut bilgisi (JS `D._sut`: karışık tipliydi): kararın başında taktiğin şut eğilimi,
/// hayali bir şut tartılırken şutun türü ("güçlü" / "plase").
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum SutBag {
    Egilim(f64),
    Tur(String),
}

/// Dünyadaki bir varlığın kimliği: gerçek oyuncu (kopyaları dahil) `id`'siyle, hayaleti `HAYALET + id` ile;
/// `YOK` (sahte/boş oyuncu) hiçbir şeye eşit değil.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Kim(pub u16);
impl Kim {
    pub const YOK: Kim = Kim(u16::MAX);
    const HAYALET: u16 = 100;
    pub fn oyuncu(id: f64) -> Kim {
        Kim(id as u16)
    }
    pub fn hayalet(gercek: Kim) -> Kim {
        if gercek == Kim::YOK { Kim::YOK } else { Kim(gercek.0 + Kim::HAYALET) }
    }
    #[inline(always)]
    pub fn var(self) -> bool {
        self != Kim::YOK
    }
    #[inline(always)]
    pub fn hayalet_mi(self) -> bool {
        self.var() && self.0 >= Kim::HAYALET
    }
}
impl Default for Kim {
    fn default() -> Kim {
        Kim::YOK
    }
}

/// JS `p`
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Oyuncu {
    /// `id`: number
    pub id: f64,
    /// `team`: number
    pub team: f64,
    /// `R`: number
    pub R: f64,
    /// `D`: number
    pub D: f64,
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
    /// `vx`: number
    pub vx: f64,
    /// `vy`: number
    pub vy: f64,
    /// `tx`: number
    pub tx: f64,
    /// `ty`: number
    pub ty: f64,
    /// `cd`: number
    pub cd: f64,
    /// `en`: number (sonradan eklenir)
    pub en: Option<f64>,
    /// `noTouch`: boolean
    pub noTouch: bool,
    /// `sprint`: boolean
    pub sprint: bool,
    /// `press`: boolean
    pub press: bool,
    /// `job`: string (sonradan eklenir)
    pub job: Option<std::sync::Arc<str>>,
    /// `ca`: number (sonradan eklenir)
    pub ca: Option<f64>,
    /// `_don`: number (sonradan eklenir)
    pub _don: Option<f64>,
    /// `_dke`: number (sonradan eklenir)
    pub _dke: Option<f64>,
    /// `_sf`: null|object
    pub _sf: Option<JP_sf>,
    /// `den`: number (sonradan eklenir)
    pub den: Option<f64>,
    /// `pplan`: object (sonradan eklenir)
    pub pplan: Option<JPPplan>,
    /// Varlık kimliği (MIMARI.md 3.2, 4c-2): gerçek oyuncu ve kopyaları aynı, hayalet kendi gerçek oyuncusundan
    /// ayrı; sahte/boş oyuncuda yok (o zaman yalnız nesne kimliği). Kopyayla taşınır; kayda girmez.
    #[serde(skip)]
    pub kim: Kim,
    /// Maç durumu (MIMARI.md 3.6): yalnız maç ve yerleşimin yazdığı alanlar; kopyalar paylaşır,
    /// yazma `hal_mut()` ile (paylaşılıyorsa önce kopyalar).
    pub hal: std::sync::Arc<OyuncuMac>,
    /// Gövde (MIMARI.md 3.6): kurulumda (ve taktik değişiminde) yazılan, kopyaların paylaştığı alanlar;
    /// yazma `govde_mut()` ile (paylaşılıyorsa önce kopyalar).
    pub govde: std::sync::Arc<OyuncuGovde>,
}

/// Oyuncunun gövdesi (MIMARI.md 3.6): rol, slot, ad, yetenek, talimat. Maç boyunca (neredeyse) değişmez.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct OyuncuGovde {
    /// `i`: number
    pub i: f64,
    /// `role`: string
    pub role: std::sync::Arc<str>,
    /// `slot`: object
    pub slot: JPSlot,
    /// `name`: string
    pub name: std::sync::Arc<str>,
    /// `a`: object
    pub a: JPA,
    /// `rslot`: object (sonradan eklenir)
    pub rslot: Option<JPRslot>,
    /// `slot0`: object (sonradan eklenir)
    pub slot0: Option<JPSlot0>,
    /// `rslot0`: object (sonradan eklenir)
    pub rslot0: Option<JPRslot0>,
    /// `rh`: string (sonradan eklenir)
    pub rh: Option<std::sync::Arc<str>>,
    /// `rs`: string (sonradan eklenir)
    pub rs: Option<std::sync::Arc<str>>,
    /// `tl`: object (sonradan eklenir)
    pub tl: Option<std::sync::Arc<JPTl>>,
    /// `eg`: object (sonradan eklenir)
    pub eg: Option<JPEg>,
    /// (Rust) yetenek satırı: `a`dan maç başında kurulur (ALAN_Y)
    /// paylaşımlı: maç içinde değişmez, kopyası yalnız sayaç (her oyuncu kopyasında sözlük kurulmasın)
    #[serde(with = "crate::paket::yet_kod")]
    pub yet: std::sync::Arc<crate::yetenek::Yetenek>,
}

/// Oyuncunun maç durumu (MIMARI.md 3.6): işler, hamleler, sayaçlar, yerleşimin anıları. Kafa bunlara yazmaz.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct OyuncuMac {
    /// `driveTo`: null|object
    pub driveTo: Option<JPDriveTo>,
    /// `_dur`: null|number
    pub _dur: Option<f64>,
    /// `_korT`: number (sonradan eklenir)
    pub _korT: Option<f64>,
    /// `bekBas`: null|number
    pub bekBas: Option<f64>,
    /// `hist`: dizi (sonradan eklenir)
    pub hist: Option<std::sync::Arc<Vec<JPHistE>>>,
    /// `lock`: number (sonradan eklenir)
    pub lock: Option<f64>,
    /// `kapPrev`: OYUNCU|null (yerleşim: kapatma işinin adamı)
    pub kapPrev: Option<Pid>,
    /// `zonePrev`: OYUNCU|null
    pub zonePrev: Option<Pid>,
    /// `swPrev`: OYUNCU|null
    pub swPrev: Option<Pid>,
    /// `uaPrev`: OYUNCU|null
    pub uaPrev: Option<Pid>,
    /// `rec`: number (sonradan eklenir)
    pub rec: Option<f64>,
    /// `tutQ`: OYUNCU|null
    pub tutQ: Option<Pid>,
    /// `otx`: null|number
    pub otx: Option<f64>,
    /// `_bk`: dizi (sonradan eklenir)
    pub _bk: Option<std::sync::Arc<Vec<Option<JP_bkE>>>>,
    /// `_ktB`: object (sonradan eklenir)
    pub _ktB: Option<JP_ktB>,
    /// `_topSpr`: boolean
    pub _topSpr: bool,
    /// `_kosuK`: null|string
    pub _kosuK: Option<std::sync::Arc<str>>,
    /// `_hizK`: null|number
    pub _hizK: Option<f64>,
    /// `_evx`: number (sonradan eklenir)
    pub _evx: Option<f64>,
    /// `_evy`: number (sonradan eklenir)
    pub _evy: Option<f64>,
    /// `_yog`: number (sonradan eklenir)
    pub _yog: Option<f64>,
    /// `_D0`: number (sonradan eklenir)
    pub _D0: Option<f64>,
    /// `_Dl`: number (sonradan eklenir)
    pub _Dl: Option<f64>,
    /// `_enOnce`: number (sonradan eklenir)
    pub _enOnce: Option<f64>,
    /// `drive`: boolean
    pub drive: bool,
    /// `markRef`: OYUNCU (sonradan eklenir)
    pub markRef: Option<Pid>,
    /// `oty`: number (sonradan eklenir)
    pub oty: Option<f64>,
    /// `_otH`: OYUNCU|null
    pub _otH: Option<Pid>,
    /// `_otB`: null|object (yerleşim: bindirmenin taşıyıcısı; yalnız var/yok okunur)
    pub _otB: Option<Pid>,
    /// `_otE`: object (sonradan eklenir)
    pub _otE: Option<JP_otE>,
    /// `_otF`: OYUNCU|null
    pub _otF: Option<Pid>,
    /// `_kv`: null|object
    pub _kv: Option<JP_kv>,
    /// `_amacJob`: null|string
    pub _amacJob: Option<std::sync::Arc<str>>,
    /// `_kt`: object (sonradan eklenir)
    pub _kt: Option<JP_kt>,
    /// `_hizF`: number (sonradan eklenir)
    pub _hizF: Option<f64>,
    /// `kuyuT`: number (sonradan eklenir)
    pub kuyuT: Option<f64>,
    /// `sonuk`: boolean
    pub sonuk: bool,
    /// `_kuyuCik`: boolean
    pub _kuyuCik: bool,
    /// `_yer`: boolean
    pub _yer: bool,
    /// `_savT`: number (sonradan eklenir)
    pub _savT: Option<f64>,
    /// `_savD`: number (sonradan eklenir)
    pub _savD: Option<f64>,
    /// `_blokT`: number (sonradan eklenir)
    pub _blokT: Option<f64>,
    /// `past`: dizi (sonradan eklenir)
    pub past: Option<std::sync::Arc<Vec<JPPastE>>>,
    /// `holdD`: boolean
    pub holdD: bool,
    /// `_plan`: null|object
    pub _plan: Option<JP_plan>,
    /// `_surK`: null|object
    pub _surK: Option<JP_surK>,
    /// `_sprIst`: boolean
    pub _sprIst: bool,
    /// `sarj`: boolean
    /// JS `p.sarj` (tanımsız: şarj pasif sayılmaz, `?? 1`)
    pub sarj: Option<bool>,
    /// `savur`: null|object
    pub savur: Option<JPSavur>,
    /// `lastWhy`: string (sonradan eklenir)
    pub lastWhy: Option<std::sync::Arc<str>>,
    /// `amA`: boolean
    pub amA: bool,
    /// `am`: number (sonradan eklenir)
    pub am: Option<f64>,
    /// `_rcvV`: null|number
    pub _rcvV: Option<f64>,
    /// `_rcvB`: TOP|TOPKOPYA|object (sonradan eklenir)
    pub _rcvB: Option<TopRef>,
    /// `_rcvAbs`: number (sonradan eklenir)
    pub _rcvAbs: Option<f64>,
    /// `_rcvX`: number (sonradan eklenir)
    pub _rcvX: Option<f64>,
    /// `_rcvY`: number (sonradan eklenir)
    pub _rcvY: Option<f64>,
    /// `chB`: TOP|TOPKOPYA|object (sonradan eklenir)
    pub chB: Option<TopRef>,
    /// `_preB`: TOP|TOPKOPYA|object (sonradan eklenir)
    pub _preB: Option<TopRef>,
    /// `_preT`: number (sonradan eklenir)
    pub _preT: Option<f64>,
    /// `_otA`: object (sonradan eklenir)
    pub _otA: Option<JP_otA>,
    /// `_onH`: null|object
    pub _onH: Option<JP_onH>,
    /// `_donDir`: number (sonradan eklenir)
    pub _donDir: Option<f64>,
    /// `_donT`: number (sonradan eklenir)
    pub _donT: Option<f64>,
    /// `_omuzT`: number (sonradan eklenir)
    pub _omuzT: Option<f64>,
    /// `_savCar`: number (sonradan eklenir)
    pub _savCar: Option<f64>,
    /// `lockAt`: number (sonradan eklenir)
    pub lockAt: Option<f64>,
    /// `_lockH`: OYUNCU (sonradan eklenir)
    pub _lockH: Option<Pid>,
    /// `_presT`: number (sonradan eklenir)
    pub _presT: Option<f64>,
    /// `_presH`: OYUNCU (sonradan eklenir)
    pub _presH: Option<Pid>,
    /// `_onK`: object (sonradan eklenir)
    pub _onK: Option<JP_onK>,
    /// `_omEv`: number (sonradan eklenir)
    pub _omEv: Option<f64>,
    /// `fiskeCd`: number (sonradan eklenir)
    pub fiskeCd: Option<f64>,
    /// `fiske`: number (sonradan eklenir)
    pub fiske: Option<f64>,
    /// `fiskeDir`: number (sonradan eklenir)
    pub fiskeDir: Option<f64>,
    /// `_kal`: boolean
    pub _kal: bool,
    /// `_kalH`: OYUNCU (sonradan eklenir)
    pub _kalH: Option<Pid>,
    /// `hamle`: null|object
    pub hamle: Option<JPHamle>,
    /// `_dolYok`: number (sonradan eklenir)
    pub _dolYok: Option<f64>,
    /// `adandi`: number (sonradan eklenir)
    pub adandi: Option<f64>,
    /// `adT`: object (sonradan eklenir)
    pub adT: Option<JPAdT>,
    /// `_gvKaz`: object (sonradan eklenir)
    pub _gvKaz: Option<JP_gvKaz>,
    /// `_winSeen`: number (sonradan eklenir)
    pub _winSeen: Option<f64>,
    /// `son`: object (sonradan eklenir)
    pub son: Option<JPSon>,
    /// `_kh`: object (sonradan eklenir)
    pub _kh: Option<JP_kh>,
    /// `_khT`: number (sonradan eklenir)
    pub _khT: Option<f64>,
    /// `_khQ`: OYUNCU (sonradan eklenir)
    pub _khQ: Option<Pid>,
    /// `_omuz`: object (sonradan eklenir)
    pub _omuz: Option<JP_omuz>,
    /// `adNeden`: string (sonradan eklenir)
    pub adNeden: Option<std::sync::Arc<str>>,
    /// `_kAn`: number (sonradan eklenir)
    pub _kAn: Option<f64>,
    /// `_kAnT`: number (sonradan eklenir)
    pub _kAnT: Option<f64>,
    /// `okudu`: number (sonradan eklenir)
    pub okudu: Option<f64>,
    /// `_bosT`: number (sonradan eklenir)
    pub _bosT: Option<f64>,
    /// `sendT`: number (sonradan eklenir)
    pub sendT: Option<f64>,
    /// `sendCd`: number (sonradan eklenir)
    pub sendCd: Option<f64>,
    /// `stun`: number (sonradan eklenir)
    pub stun: Option<f64>,
    /// `_itK`: null|number
    pub _itK: Option<f64>,
    /// `_koyEv`: number (sonradan eklenir)
    pub _koyEv: Option<f64>,
    /// `_dalis`: number (sonradan eklenir)
    pub _dalis: Option<f64>,
    /// (mac) `slowT`: omuzla yavaşlama bitişi
    pub slowT: Option<f64>,
    /// (yerleşim) görülen kopyanın konum hatası `__e` ve yaşı `__yas` (gerçek oyuncuda yok)
    pub __e: Option<f64>,
    pub __yas: Option<f64>,
    /// (mac) `kenarW`: kenardan çalımın yol noktası
    pub kenarW: Option<crate::mac_veri::KenarW>,
    /// (mac) `fkBlock`, `fkCool`: fake sonrası bekleme
    pub fkBlock: bool,
    pub fkCool: Option<f64>,
    /// (mac) `gvH`, `gvCd`: gövdeyle vurma hamlesi
    pub gvH: Option<JPHamle>,
    pub gvCd: Option<f64>,
}

/// JS `b`
#[derive(Clone, Debug, Default)]
pub struct TopEk {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
    /// `vx`: number
    pub vx: f64,
    /// `vy`: number
    pub vy: f64,
    /// `e`: number
    pub e: f64,
    /// `ch`: number
    pub ch: f64,
    /// `team`: number
    pub team: f64,
    /// `alive`: boolean
    pub alive: bool,
    /// `dead`: number
    pub dead: f64,
    /// `t`: number
    pub t: f64,
    /// `dist`: number
    pub dist: f64,
    /// `walls`: number
    pub walls: f64,
    /// `log`: dizi
    pub log: Vec<JBLogE>,
    /// `from`: OYUNCU (sonradan eklenir)
    pub from: Option<Pid>,
    /// `recv`: OYUNCU|null
    pub recv: Option<Pid>,
    /// `kt`: number (sonradan eklenir)
    pub kt: Option<f64>,
    /// `kz`: number (sonradan eklenir)
    pub kz: Option<f64>,
    /// `cue`: number (sonradan eklenir)
    pub cue: Option<f64>,
    /// `bekId`: number (sonradan eklenir)
    pub bekId: Option<f64>,
    /// `tp`: number (sonradan eklenir)
    pub tp: Option<f64>,
    /// `cds`: Map (sonradan eklenir)
    pub cds: Option<Vec<(Pid, f64)>>,
    /// `sap`: boolean
    pub sap: bool,
    /// `gon`: boolean
    pub gon: bool,
    /// `shotPk`: number (sonradan eklenir)
    pub shotPk: Option<f64>,
    /// (mac) `_dk0`: direk sayacının son görülen değeri
    pub _dk0: f64,
}

/// JS `m`
#[derive(Clone, Debug, Default)]
pub struct Mac {
    /// `seed`: number
    pub seed: f64,
    /// `tick`: number
    pub tick: f64,
    /// `len`: number
    pub len: f64,
    /// `devre`: boolean
    pub devre: bool,
    /// `devre2`: boolean
    pub devre2: bool,
    /// `score`: dizi
    pub score: Vec<f64>,
    /// `ball`: TOP|null
    pub ball: Option<MacTop>,
    /// `holder`: OYUNCU|null
    pub holder: Option<Pid>,
    /// `ch`: number
    pub ch: f64,
    /// `decT`: number
    pub decT: f64,
    /// `over`: boolean
    pub over: bool,
    /// `fl`: null|object
    pub fl: Option<JMFl>,
    /// `tac`: dizi
    pub tac: Vec<crate::mac_veri::Tac>,
    /// `st`: object
    pub st: JMSt,
    /// `winT`: null|number
    pub winT: Option<f64>,
    /// `gecis`: boolean
    pub gecis: bool,
    /// `gc`: null|object
    pub gc: Option<JMGc>,
    /// `sh`: object (sonradan eklenir)
    pub sh: Option<JMSh>,
    /// `ra`: dizi (sonradan eklenir)
    pub ra: Option<Vec<std::collections::BTreeMap<usize, JMRaEE>>>,
    /// `tac0`: dizi (sonradan eklenir)
    pub tac0: Option<Vec<Option<crate::mac_veri::Tac>>>,
    /// `evre`: dizi (sonradan eklenir)
    pub evre: Option<Vec<Option<JMEvreE>>>,
    /// `_macC`: object (sonradan eklenir)
    pub _macC: Option<std::collections::BTreeMap<String, crate::mac_veri::MacW>>,
    /// `E`: object (sonradan eklenir)
    pub E: Option<JME>,
    /// `att`: number (sonradan eklenir)
    pub att: Option<f64>,
    /// `_perde`: null
    pub _perde: Option<crate::mac_veri::Perde>,
    /// `_kuyuTop`: number (sonradan eklenir)
    pub _kuyuTop: Option<f64>,
    /// `not`: object (sonradan eklenir)
    pub not: Option<JMNot>,
    /// `pre`: null|object
    pub pre: Option<JMPre>,
    /// `ctl`: null|object
    pub ctl: Option<JMCtl>,
    /// `_prT`: object (sonradan eklenir)
    pub _prT: Option<JM_prT>,
    /// `_sec`: object (sonradan eklenir)
    pub _sec: Option<JM_sec>,
    /// `lastDec`: object (sonradan eklenir)
    pub lastDec: Option<JMLastDec>,
    /// `_pas`: object (sonradan eklenir)
    pub _pas: Option<JM_pas>,
    /// `_gelenB`: TOPKOPYA|null|object
    pub _gelenB: Option<Box<MacTop>>,
    /// `_onHk`: null
    pub _onHk: Option<JP_onH>,
    /// `_inSp`: number (sonradan eklenir)
    pub _inSp: Option<f64>,
    /// `hhKul`: boolean
    pub hhKul: bool,
    /// `kp`: null|object
    pub kp: Option<JMKp>,
    /// `_yzBas`: number (sonradan eklenir)
    pub _yzBas: Option<f64>,
    /// `_yzBekle`: boolean
    pub _yzBekle: bool,
    /// (mac) `ps`: oyuncular (Pid = sıra)
    pub ps: Vec<Oyuncu>,
    /// (mac) `r`: maçın zar dizisi
    pub r: crate::rng::Rng,
    /// (mac) `events`: olaylar (en yenisi başta, en çok 120)
    pub events: Vec<crate::mac_veri::Olay>,
    /// (mac) bugüne dek çıkan olay sayısı (events 120'de kesilir; yeni olayları bulmak için)
    pub ev_n: u64,
}

/// (mac) Maç topu `m.ball`: fizik ve temas `temas::TopT`'de, ek alanlar `TopEk`'te
/// (TopEk'in TopT'de karşılığı olan alanları kullanılmaz). `Deref` fiziğe (`Top`) iner.
#[derive(Clone, Debug)]
pub struct MacTop {
    pub tt: crate::temas::TopT,
    pub ek: TopEk,
}
impl std::ops::Deref for MacTop {
    type Target = crate::top::Top;
    fn deref(&self) -> &crate::top::Top { &self.tt.b }
}
impl std::ops::DerefMut for MacTop {
    fn deref_mut(&mut self) -> &mut crate::top::Top { &mut self.tt.b }
}
impl MacTop {
    /// topun kimliği (JS nesne kimliği)
    pub fn kimlik(&self) -> TopRef { self.tt.t.kimlik }
}

/// JS `D`
#[derive(Clone, Debug, Default)]
pub struct KafaD {
    /// `simMax`: number
    pub simMax: f64,
    /// `errOk`: number
    pub errOk: f64,
    /// `errPos`: number
    pub errPos: f64,
    /// `aimErr`: number
    pub aimErr: f64,
    /// `v0Err`: number
    pub v0Err: f64,
    /// `arrive`: number
    pub arrive: f64,
    /// `arriveSpace`: number
    pub arriveSpace: f64,
    /// `vmax`: number
    pub vmax: f64,
    /// `shotV`: number
    pub shotV: f64,
    /// `plaseV`: number
    pub plaseV: f64,
    /// `riskBase`: number
    pub riskBase: f64,
    /// `surKarar`: number
    pub surKarar: f64,
    /// `asanPay`: number
    pub asanPay: f64,
    /// `sutHazir`: number
    pub sutHazir: f64,
    /// `threatK`: number
    pub threatK: f64,
    /// `sideK`: number
    pub sideK: f64,
    /// `b0`: number
    pub b0: f64,
    /// `b1`: number
    pub b1: f64,
    /// `recvKeepT`: number
    pub recvKeepT: f64,
    /// `shotThru`: number
    pub shotThru: f64,
    /// `bekTempo`: number
    pub bekTempo: f64,
    /// `otErr`: number
    pub otErr: f64,
    /// `otMargin`: number
    pub otMargin: f64,
    /// `passMaxF`: number
    pub passMaxF: f64,
    /// `useSpace`: boolean
    pub useSpace: bool,
    /// `aimPow`: number
    pub aimPow: f64,
    /// `aimPlase`: number
    pub aimPlase: f64,
    /// `arriveFirm`: number
    pub arriveFirm: f64,
    /// `vK`: number
    pub vK: f64,
    /// `combo`: number
    pub combo: f64,
    /// `chgMin`: number
    pub chgMin: f64,
    /// `sarjUfuk`: number
    pub sarjUfuk: f64,
    /// `chg0`: number
    pub chg0: f64,
    /// `chgTek`: number
    pub chgTek: f64,
    /// `chgPasif`: number
    pub chgPasif: f64,
    /// `enHizV`: number
    pub enHizV: f64,
    /// `enUfuk`: number
    pub enUfuk: f64,
    /// `enTmin`: number
    pub enTmin: f64,
    /// `kontraL`: number
    pub kontraL: f64,
    /// `zoneMin`: number
    pub zoneMin: f64,
    /// `zoneD`: number
    pub zoneD: f64,
    /// `bankOk`: number
    pub bankOk: f64,
    /// `bankIf`: number
    pub bankIf: f64,
    /// `bankN`: number
    pub bankN: f64,
    /// `trials`: number
    pub trials: f64,
    /// `nFull`: number
    pub nFull: f64,
    /// `yokD`: number
    pub yokD: f64,
    /// `nMates`: number
    pub nMates: f64,
    /// `shotRange`: number
    pub shotRange: f64,
}

/// Kafanın geçici bağlamı (JS `D`'nin `_` alanları): bir hesabın süresince kurulup geri alınan değerler
/// (`_tick`, `_gPr`, `_sav*`, `_kenar` …). Değişmez ayar (`KafaD`) bundan ayrıldı (MIMARI.md 3e-3):
/// saf değerlendirmeler ve yerleşim yalnız ayarı görür, geçici bağlam kafanın kendisinde (`Kafa::bag`).
#[derive(Clone, Debug, Default)]
pub struct KafaBag {
    /// `_tick`: number (sonradan eklenir)
    pub _tick: Option<f64>,
    /// `_tekL`: undef
    pub _tekL: Option<f64>,
    /// `_sut`: number (sonradan eklenir)
    pub _sut: Option<SutBag>,
    /// `_savT`: undef
    pub _savT: Option<f64>,
    /// `_savW`: undef
    pub _savW: Option<f64>,
    /// `_savU`: undef
    pub _savU: Option<f64>,
    /// `_savCa`: undef
    pub _savCa: Option<f64>,
    /// `_kenar`: undef
    pub _kenar: Option<bool>,
    /// `_gPr`: undef
    pub _gPr: Option<f64>,
    /// `_enWD`: undef
    pub _enWD: Option<f64>,
    /// `_enTm`: undef
    pub _enTm: Option<f64>,
    /// `_kanca`: undef
    pub _kanca: Option<DKanca>,
    /// `_ot`: number (sonradan eklenir)
    pub _ot: Option<f64>,
    /// `_tekD`: boolean
    pub _tekD: bool,
    /// `_itme`: undef
    pub _itme: Option<bool>,
    /// `_gon`: undef
    pub _gon: Option<bool>,
    /// `_sYon`: undef
    pub _sYon: Option<f64>,
}



/// JS `D._kanca` (tek dokunuşun kancası: gelen topun savurması; maç kurar, kafa okur). Kafa 2 tipledi.
#[derive(Clone, Debug, Default)]
pub struct DKanca {
    pub w0: f64,
    pub s: f64,
    pub ca: Option<f64>,
    pub u: Option<f64>,
    pub on: bool,
}

/// JS `{x, y}` (nokta; seçeneğin hedefleri, hayalin bitiş yerleri)
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Nokta {
    pub x: f64,
    pub y: f64,
}

/// JS `o` (kafanın seçeneği). Kafa 2 düzeltti: noktalar `Nokta`, `det` alanları isteğe bağlı,
/// `dok` sıralı sözlük, hayali dünyalar paylaşımlı (`Arc`); tartının yazdığı alanlar eklendi.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Secenek {
    /// `kind`: string ("pas" | "önüne" | "kenardan" | "sür" | "çalım" | "tut" | "gönder" | "fake")
    pub kind: String,
    pub to: Option<Nokta>,
    pub dir: Option<Nokta>,
    pub _own: Option<f64>,
    pub uzun: Option<f64>,
    pub duvar: bool,
    pub toE: Option<Nokta>,
    pub TsE: Option<f64>,
    /// sendelemeyle kesilen sürmede topu geri almaya dek beklenen süre (ALAN_SEND_GEC_IZ)
    pub sendGec: Option<f64>,
    /// değer (JS null → None)
    pub v: Option<f64>,
    pub dok: Option<Dok>,
    pub det: Option<JODet>,
    pub en: Option<JOEn>,
    pub _pT: Option<f64>,
    /// JS `o.spr` (sprint niyeti: true ya da kat)
    pub spr: Option<f64>,
    pub sprBos: Option<f64>,
    pub sjK: Option<f64>,
    /// alıcı (kararın dünyasındaki sıra)
    pub q: Option<Pid>,
    pub arrive: Option<f64>,
    pub tek: Option<f64>,
    pub firm: Option<bool>,
    pub itK: Option<f64>,
    pub launch: Option<JOLaunch>,
    pub sYon: Option<f64>,
    pub savT: Option<f64>,
    pub _sek: Option<f64>,
    pub tf: Option<f64>,
    pub kh: Option<String>,
    pub lf0: Option<JOLaunch>,
    pub siddet: Option<f64>,
    pub kendine: bool,
    pub lead: Option<Nokta>,
    pub hold: Option<f64>,
    /// (mac) maçın yazdığı / okuduğu, taslakta olmayan alanlar
    pub savW: Option<f64>,
    pub savU: Option<f64>,
    pub ot: Option<f64>,
    pub noErr: bool,
    pub devam: Option<f64>,
    pub show: Option<crate::mac_veri::FakeShow>,
    /// (mac) kafa günlüğü için seçeneğin kimliği (JS nesne kimliği)
    pub _kafaId: Option<u64>,
    pub shot: Option<String>,
    // --- Kafa 2'nin tartısının yazdıkları ---
    pub bekci: Option<f64>,
    pub _at: Option<Nokta>,
    #[serde(with = "crate::paket::dunya_kod")]
    pub _w: Option<std::sync::Arc<Vec<std::sync::Arc<Oyuncu>>>>,
    pub _T: Option<f64>,
    pub _en: Option<f64>,
    pub sonra: Option<String>,
    pub ust: Option<f64>,
    pub zaman: Option<f64>,
    pub wp: Option<Nokta>,
    pub wp1: Option<Nokta>,
}

/// JS `o.dok`: değer dökümü (anahtar sırası JS ekleme sırası) ve `sonra` yazısı.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Dok {
    pub k: Vec<(String, f64)>,
    pub sonra: Option<String>,
}
impl Dok {
    pub fn al(&self, k: &str) -> Option<f64> {
        self.k.iter().find(|x| x.0 == k).map(|x| x.1)
    }
    /// JS `t[k] = v`
    pub fn koy(&mut self, k: &str, v: f64) {
        match self.k.iter_mut().find(|x| x.0 == k) {
            Some(x) => x.1 = v,
            None => self.k.push((k.to_string(), v)),
        }
    }
    pub fn yeni(kv: &[(&str, f64)]) -> Dok {
        let mut d = Dok::default();
        for (k, v) in kv {
            d.koy(k, *v);
        }
        d
    }
}



/// JS `p.slot`
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct JPSlot {
    /// `role`: string
    pub role: std::sync::Arc<str>,
    /// `d`: number
    pub d: f64,
    /// `l`: number
    pub l: f64,
}

/// JS `p.a`
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct JPA {
    /// `hiz`: number
    pub hiz: f64,
    /// `ceviklik`: number
    pub ceviklik: f64,
    /// `guc`: number
    pub guc: f64,
    /// `uzanma`: number
    pub uzanma: f64,
    /// `alan`: number
    pub alan: f64,
    /// `kontrol`: number
    pub kontrol: f64,
    /// `surme`: number
    pub surme: f64,
    /// `pas`: number
    pub pas: f64,
    /// `gonderme`: number
    pub gonderme: f64,
    /// `yukleme`: number
    pub yukleme: f64,
    /// `tepki`: number
    pub tepki: f64,
    /// `gorus`: number
    pub gorus: f64,
    /// `yeralma`: number
    pub yeralma: f64,
    /// `sogukkanlilik`: number
    pub sogukkanlilik: f64,
    /// `kesme`: number
    pub kesme: f64,
    /// `refleks`: number
    pub refleks: f64,
    /// `yerlesim`: number
    pub yerlesim: f64,
    /// `hakimiyet`: number
    pub hakimiyet: f64,
    /// `taraf`: string
    pub taraf: String,
    /// `enerji`: number
    pub enerji: f64,
    /// `govde`: number
    pub govde: f64,
    /// `caliskanlik`: number
    pub caliskanlik: f64,
}

/// JS `p.driveTo`
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct JPDriveTo {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `p.hist[]`
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct JPHistE {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
    /// `vx`: number
    pub vx: f64,
    /// `vy`: number
    pub vy: f64,
}

/// JS `p._bk[]`
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct JP_bkE {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
    /// `vx`: number
    pub vx: f64,
    /// `vy`: number
    pub vy: f64,
    /// `t`: number
    pub t: f64,
    /// `z`: number
    pub z: f64,
}

/// JS `p._ktB`
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct JP_ktB {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `p.rslot`
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct JPRslot {
    /// `d`: number
    pub d: f64,
    /// `l`: number
    pub l: f64,
}

/// JS `p.slot0`
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct JPSlot0 {
    /// `role`: string
    pub role: std::sync::Arc<str>,
    /// `d`: number
    pub d: f64,
    /// `l`: number
    pub l: f64,
}

/// JS `p.rslot0`
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct JPRslot0 {
    /// `d`: number
    pub d: f64,
    /// `l`: number
    pub l: f64,
}

/// JS `p.tl`
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct JPTl {
    /// (mac) hücum talimatları: yalnız verilenler (JS'te eksik anahtar varsayılana düşer: mac::kurulum::tl)
    pub huc: std::collections::BTreeMap<String, f64>,
    /// (mac) savunma talimatları
    pub sav: std::collections::BTreeMap<String, f64>,
}

/// JS `p.eg`
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct JPEg {
    /// `ps`: number
    pub ps: f64,
    /// `rk`: number
    pub rk: f64,
}


/// JS `p._otE`
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct JP_otE {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `p._kv`
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct JP_kv {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
    /// `t`: number
    pub t: f64,
}

/// JS `p._kt`
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct JP_kt {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `p.past[]`
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct JPPastE {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `p._plan`
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct JP_plan {
    /// `kind`: string
    pub kind: String,
    /// `q`: OYUNCU
    pub q: Pid,
    /// `to`: object
    pub to: JP_planTo,
    /// `P`: number
    pub P: f64,
    /// `t`: number
    pub t: f64,
}

/// JS `p._surK`
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct JP_surK {
    /// `id`: number
    pub id: f64,
    /// `to`: object
    pub to: JP_surKTo,
    /// `t`: number
    pub t: f64,
    /// `T`: number
    pub T: f64,
    /// `nd0`: number
    pub nd0: f64,
}

/// JS `p.savur`
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct JPSavur {
    /// `t0`: number
    pub t0: f64,
    /// `o`: object
    pub o: Box<Secenek>,
    /// `s`: number
    pub s: f64,
    /// `a`: number
    pub a: f64,
    /// `wMax`: number
    pub wMax: f64,
    /// `wR`: number
    pub wR: f64,
    /// `rem`: number
    pub rem: f64,
    /// `w`: number
    pub w: f64,
    /// `g`: number
    pub g: f64,
    /// `u`: number
    pub u: f64,
}

/// JS `p._otA`
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct JP_otA {
    /// `b`: TOP|TOPKOPYA|object
    pub b: TopRef,
    /// `ok`: boolean
    pub ok: bool,
}

/// JS `p._onH`
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct JP_onH {
    /// `b`: TOP
    pub b: TopRef,
    /// `tArr`: number
    pub tArr: f64,
    /// `s`: number
    pub s: f64,
    /// `wS`: number
    pub wS: f64,
    /// `phi`: number
    pub phi: f64,
    /// `caR`: number
    pub caR: f64,
    /// `kal`: number
    pub kal: f64,
    /// `a`: number
    pub a: f64,
    /// `Tsw`: number
    pub Tsw: f64,
    /// `w`: number
    pub w: f64,
}

/// JS `p._sf`
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct JP_sf {
    /// `t0`: number
    pub t0: f64,
    /// `f`: string
    pub f: String,
}

/// JS `p._onK`
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct JP_onK {
    /// `h`: number
    pub h: f64,
    /// `koy`: boolean
    pub koy: bool,
    /// kararın verildiği tik
    #[serde(default)]
    pub t: f64,
}

/// JS `p.hamle`
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct JPHamle {
    /// `k`: string
    pub k: String,
    /// `hid`: number
    pub hid: f64,
    /// `t0`: number
    pub t0: f64,
    /// `W`: number
    pub W: f64,
}

/// JS `p.adT`
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct JPAdT {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `p._gvKaz`
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct JP_gvKaz {
    /// `h`: number
    pub h: f64,
    /// `t`: number
    pub t: f64,
    /// `kaz`: number
    pub kaz: f64,
}

/// JS `p.son`
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct JPSon {
    /// `k`: string
    pub k: String,
    /// `t`: number
    pub t: f64,
    /// `ok`: boolean
    pub ok: bool,
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `p._kh`
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct JP_kh {
    /// `dx`: number
    pub dx: f64,
    /// `dy`: number
    pub dy: f64,
}

/// JS `p._omuz`
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct JP_omuz {
    /// `t`: number
    pub t: f64,
    /// `q`: number
    pub q: f64,
}

/// JS `p.pplan`
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct JPPplan {
    /// `k`: string
    pub k: String,
    /// `s`: number
    pub s: f64,
    /// `t0`: number
    pub t0: f64,
    /// `until`: number
    pub until: f64,
}

/// JS `b.log[]`
#[derive(Clone, Debug, Default)]
pub struct JBLogE {
    /// `t`: number
    pub t: f64,
    /// `k`: string
    pub k: String,
    /// `txt`: string
    pub txt: String,
}


/// JS `m.fl`
#[derive(Clone, Debug, Default)]
pub struct JMFl {
    /// `t0`: number
    pub t0: f64,
    /// `kind`: string
    pub kind: String,
    /// `q`: OYUNCU|null
    pub q: Option<Pid>,
    /// `to`: object
    pub to: Option<JMFlTo>,
    /// `end`: object
    pub end: JMFlEnd,
    /// `shot`: string|undef
    pub shot: Option<String>,
}

/// JS `m.tac[]`
#[derive(Clone, Debug, Default)]
pub struct JMTacE {
    /// `blok`: string
    pub blok: String,
    /// `pres`: number
    pub pres: f64,
    /// `onde`: number
    pub onde: f64,
    /// `genislik`: string
    pub genislik: String,
    /// `risk`: number
    pub risk: f64,
    /// `tempo`: number
    pub tempo: f64,
    /// `kazaninca`: string
    pub kazaninca: String,
    /// `siddet`: number
    pub siddet: f64,
    /// `savGen`: number
    pub savGen: f64,
    /// `pasSurme`: number
    pub pasSurme: f64,
    /// `pasBoy`: number
    pub pasBoy: f64,
    /// `sabir`: number
    pub sabir: f64,
    /// `sut`: number
    pub sut: f64,
    /// `enDinlen`: number
    pub enDinlen: f64,
    /// `enYipratma`: number
    pub enYipratma: f64,
    /// `durus`: object
    pub durus: JMTacEDurus,
    /// `yz`: boolean
    pub yz: bool,
}

/// JS `m.st`
#[derive(Clone, Debug, Default)]
pub struct JMSt {
    /// `duel`: dizi
    pub duel: Vec<f64>,
    /// `duelW`: dizi
    pub duelW: Vec<f64>,
    /// `pass`: dizi
    pub pass: Vec<f64>,
    /// `passOk`: dizi
    pub passOk: Vec<f64>,
    /// `shot`: dizi
    pub shot: Vec<f64>,
    /// `steal`: dizi
    pub steal: Vec<f64>,
    /// `poss`: dizi
    pub poss: Vec<f64>,
    /// `bank`: dizi
    pub bank: Vec<f64>,
    /// `lead`: dizi
    pub lead: Vec<f64>,
    /// `loose`: dizi
    pub loose: Vec<f64>,
    /// `swarm`: number
    pub swarm: f64,
    /// `enH`: dizi
    pub enH: Vec<crate::mac_veri::EnSayac>,
    /// `sprF`: dizi
    pub sprF: Vec<f64>,
    /// `sprT`: dizi
    pub sprT: Vec<f64>,
    /// `enCal`: dizi
    pub enCal: Vec<f64>,
    /// `calim`: dizi
    pub calim: Vec<f64>,
    /// (mac) taslakta olmayan sayaçlar (boş = yok)
    pub fake: Vec<f64>,
    pub fakeOk: Vec<f64>,
    pub ot: Vec<f64>,
    pub combo: Vec<f64>,
    pub direk: Vec<f64>,
}


/// JS `m.gc`
#[derive(Clone, Debug, Default)]
pub struct JMGc {
    /// `team`: number
    pub team: f64,
    /// `t0`: number
    pub t0: f64,
    /// `kendi`: boolean
    pub kendi: bool,
    /// `git`: boolean
    pub git: bool,
    /// `on`: dizi
    pub on: Vec<f64>,
    /// `biz`: number
    pub biz: f64,
    /// `rakip`: number
    pub rakip: f64,
    /// `hizli`: boolean
    pub hizli: bool,
}

/// JS `m.sh`
#[derive(Clone, Debug, Default)]
pub struct JMSh {
    /// `att`: number
    pub att: f64,
    /// `ball`: TOP|TOPKOPYA|null|object
    pub ball: Option<TopRef>,
    /// `defl`: number
    pub defl: f64,
    /// `gs`: object
    pub gs: JMShGs,
    /// `fr`: object
    pub fr: JMShFr,
}

/// JS `m.ra[][]`
#[derive(Clone, Debug, Default)]
pub struct JMRaEE {
    /// `h`: dizi
    pub h: Vec<f64>,
    /// `s`: dizi
    pub s: Vec<f64>,
}


/// JS `m.evre[]`
#[derive(Clone, Debug, Default)]
pub struct JMEvreE {
    /// `key`: string
    pub key: String,
    /// `ev`: string
    pub ev: String,
    /// `ad`: string
    pub ad: String,
    /// `t0`: number
    pub t0: f64,
}





/// JS `m.E`
#[derive(Clone, Debug, Default)]
pub struct JME {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}


/// JS `m.not`
#[derive(Clone, Debug, Default)]
pub struct JMNot {
    /// `p`: object
    pub p: std::collections::BTreeMap<usize, f64>,
    /// `s`: number|null
    pub s: Option<f64>,
    /// `actor`: OYUNCU|null
    pub actor: Option<Pid>,
    /// `team`: number|null
    pub team: Option<f64>,
}

/// JS `m.pre`
#[derive(Clone, Debug, Default)]
pub struct JMPre {
    /// `b`: TOP|TOPKOPYA|object
    pub b: TopRef,
    /// `q`: OYUNCU
    pub q: Pid,
    /// `r`: object
    pub r: crate::mac_veri::Karar,
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
    /// `t`: number
    pub t: f64,
}

/// JS `m.ctl`
#[derive(Clone, Debug, Default)]
pub struct JMCtl {
    /// `p`: OYUNCU
    pub p: Pid,
    /// `t`: number
    pub t: f64,
    /// `r`: object
    pub r: crate::mac_veri::Karar,
}

/// JS `m._prT`
#[derive(Clone, Debug, Default)]
pub struct JM_prT {
    /// `team`: number
    pub team: f64,
    /// `hId`: number
    pub hId: f64,
    /// `t`: number
    pub t: f64,
    /// `m`: Map
    pub m: Vec<(f64, f64)>,
    /// `mm`: Map
    pub mm: Vec<(String, f64)>,
    /// `hm`: Map
    pub hm: Vec<(f64, f64)>,
    /// `h`: number
    pub h: f64,
}

/// JS `m._sec`
#[derive(Clone, Debug, Default)]
pub struct JM_sec {
    /// `h`: OYUNCU
    pub h: Pid,
    /// `t`: number
    pub t: f64,
    /// `o`: dizi
    pub o: Vec<JM_secOE>,
    /// `tut`: number
    pub tut: f64,
}

/// JS `m.lastDec`
#[derive(Clone, Debug, Default)]
pub struct JMLastDec {
    /// `name`: string
    pub name: String,
    /// `team`: number
    pub team: f64,
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
    /// `opts`: dizi
    pub opts: Vec<JMLastDecOptsE>,
}

/// JS `m._pas`
#[derive(Clone, Debug, Default)]
pub struct JM_pas {
    /// `team`: number
    pub team: f64,
    /// `fx`: number
    pub fx: f64,
    /// `tx`: number
    pub tx: f64,
    /// `len`: number
    pub len: f64,
    /// `t`: number
    pub t: f64,
}

/// JS `m._temas`
#[derive(Clone, Debug, Default)]
pub struct JM_temas {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
    /// `a`: number
    pub a: f64,
    /// `J`: number
    pub J: f64,
    /// `t`: number
    pub t: f64,
}

/// JS `m.kp`
#[derive(Clone, Debug, Default)]
pub struct JMKp {
    /// `team`: number
    pub team: f64,
    /// `ids`: dizi
    pub ids: Vec<f64>,
    /// `bitis`: number
    pub bitis: f64,
    /// `sahip`: OYUNCU
    pub sahip: Option<Pid>,
}






/// JS `o.det`: hayalin sonucu (imagine) ya da tartının {P, ch}. Kafa 2 düzeltti: alanlar isteğe bağlı.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct JODet {
    pub P: Option<f64>,
    pub ch: Option<f64>,
    pub Pk: Option<f64>,
    pub Pl: Option<f64>,
    pub end: Option<Nokta>,
    pub endM: Option<Nokta>,
    pub endO: Option<Nokta>,
    pub Ps: Option<f64>,
    pub endS: Option<Nokta>,
    pub endD: Option<Nokta>,
    pub Psek: Option<f64>,
    pub endSek: Option<Nokta>,
    /// hayalin süresi (JS `det.T`)
    pub T: Option<f64>,
    /// `sonra`: string (kafa1 ekledi: fake'ten sonraki seçeneğin türü)
    pub sonra: Option<String>,
    /// `c`: number (kafa1 ekledi: fake'in inanılırlığı)
    pub c: Option<f64>,
}

/// JS `o.en`
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct JOEn {
    /// `biz`: number
    pub biz: f64,
    /// `rakip`: number
    pub rakip: f64,
    /// `hizBiz`: number
    pub hizBiz: f64,
    /// `hizRakip`: number
    pub hizRakip: f64,
    /// `hizRakipD`: number
    pub hizRakipD: f64,
    /// `hizRakipY`: number
    pub hizRakipY: f64,
    /// `per`: Map
    pub per: Vec<(String, f64)>,
    /// `ek`: number
    pub ek: f64,
}


/// JS `o.launch`
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct JOLaunch {
    /// `vx`: number
    pub vx: f64,
    /// `vy`: number
    pub vy: f64,
    /// `ok`: boolean
    pub ok: bool,
}






/// JS `p._plan.to`
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct JP_planTo {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `p._surK.to`
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct JP_surKTo {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}



























/// JS `m.fl.to`
#[derive(Clone, Debug, Default)]
pub struct JMFlTo {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `m.fl.end`
#[derive(Clone, Debug, Default)]
pub struct JMFlEnd {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `m.tac[].durus`
#[derive(Clone, Debug, Default)]
pub struct JMTacEDurus {
    /// `huc`: dizi
    pub huc: Vec<Vec<JMTacEDurusHucEE>>,
    /// `sav`: dizi
    pub sav: Vec<Vec<JMTacEDurusSavEE>>,
}





/// JS `m.sh.gs`
#[derive(Clone, Debug, Default)]
pub struct JMShGs {
    /// `t`: number
    pub t: f64,
    /// `dt`: number
    pub dt: f64,
    /// `on`: boolean
    pub on: bool,
    /// `onP`: boolean
    pub onP: bool,
    /// `oy`: boolean
    pub oy: bool,
}


/// JS `m.sh.fr`
#[derive(Clone, Debug, Default)]
pub struct JMShFr {
    /// `arr`: Map
    pub arr: Vec<(Pid, JMShFrArrV)>,
    /// `first`: OYUNCU
    pub first: Pid,
    /// `pts`: dizi
    pub pts: Vec<JMShFrPtsE>,
}
















/// JS `m._temaslar.L[]`
#[derive(Clone, Debug, Default)]
pub struct JM_temaslarLE {
    /// `i`: number
    pub i: f64,
    /// `j`: number
    pub j: f64,
    /// `o`: number
    pub o: f64,
    /// `s`: number
    pub s: f64,
}



/// JS `m._sec.o[]`
#[derive(Clone, Debug, Default)]
pub struct JM_secOE {
    /// `k`: string
    pub k: String,
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
    /// `v`: number
    pub v: f64,
    /// `dv`: number
    pub dv: f64,
}

/// JS `m.lastDec.opts[]`
#[derive(Clone, Debug, Default)]
pub struct JMLastDecOptsE {
    /// `kind`: string
    pub kind: String,
    /// `to`: object|undef
    pub to: Option<JMLastDecOptsETo>,
    /// `q`: string
    pub q: String,
    /// `P`: number
    pub P: f64,
    /// `v`: number
    pub v: f64,
    /// `en`: object|undef
    pub en: Option<JMLastDecOptsEEn>,
    /// `kh`: string|undef
    pub kh: Option<String>,
    /// `devam`: number|undef
    pub devam: Option<f64>,
}





































/// JS `m.tac[].durus.huc[][]`
#[derive(Clone, Debug, Default)]
pub struct JMTacEDurusHucEE {
    /// `f`: number (duran topta derinlik, saha yüzdesi; yerleşim okur)
    pub f: Option<f64>,
    /// `y`: number
    pub y: Option<f64>,
}

/// JS `m.tac[].durus.sav[][]`
#[derive(Clone, Debug, Default)]
pub struct JMTacEDurusSavEE {
    /// `f`: number (duran topta derinlik, saha yüzdesi; yerleşim okur)
    pub f: Option<f64>,
    /// `y`: number
    pub y: Option<f64>,
}

/// JS `m.sh.fr.arr{v}`
#[derive(Clone, Debug, Default)]
pub struct JMShFrArrV {
    /// `t`: number
    pub t: f64,
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
    /// `est`: number
    pub est: f64,
    /// `direnc`: boolean
    pub direnc: bool,
}

/// JS `m.sh.fr.pts[]`
#[derive(Clone, Debug, Default)]
pub struct JMShFrPtsE {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
    /// `e`: number
    pub e: f64,
}







/// JS `m.lastDec.opts[].to`
#[derive(Clone, Debug, Default)]
pub struct JMLastDecOptsETo {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `m.lastDec.opts[].en`
#[derive(Clone, Debug, Default)]
pub struct JMLastDecOptsEEn {
    /// `biz`: number
    pub biz: f64,
    /// `rakip`: number
    pub rakip: f64,
    /// `per`: Map
    pub per: Vec<(String, f64)>,
    /// `ek`: number
    pub ek: f64,
}





































































































































