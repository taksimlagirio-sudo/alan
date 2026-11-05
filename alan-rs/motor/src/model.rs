//! Ortak veri modeli: TASLAK, çalışma anı envanterinden üretildi (sina/model-uret.py, sina/ic-sekil2.txt).
//! Alan adları JS adlarıyla birebir (izlenebilirlik). Tipler gözlemden: Option = null/undefined görüldü;
//! Pid = m.ps içindeki oyuncunun sırası; TopRef = topun kimliği (JS nesne kimliği / kopya).
//! JsDeger = karışık tip görüldü: sahibi olan modül uygun tipe çevirir. Biçimi değiştirmek: model-sahiplik.md
#![allow(non_snake_case, non_camel_case_types, dead_code)]
pub type Pid = usize;
pub type TopRef = u64;
#[derive(Clone, Debug, Default)] pub struct JsFn;
/// Karışık tipli JS değeri (taslakta; sahibi düzeltir).
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)] pub enum JsDeger { #[default] Yok, Sayi(f64), Metin(String), Mantik(bool), Dizi(Vec<JsDeger>), Nesne(Vec<(String, JsDeger)>) }

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
    /// `_lc`: null
    pub _lc: Option<JsDeger>,
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
    pub kenarW: Option<crate::mac::KenarW>,
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
    /// `done`: null
    pub done: Option<JsDeger>,
    /// `log`: dizi
    pub log: Vec<JBLogE>,
    /// `from`: OYUNCU (sonradan eklenir)
    pub from: Option<Pid>,
    /// `recv`: OYUNCU|null
    pub recv: Option<Pid>,
    /// `vur`: OYUNCU (sonradan eklenir)
    pub vur: Option<Pid>,
    /// `kt`: number (sonradan eklenir)
    pub kt: Option<f64>,
    /// `kz`: number (sonradan eklenir)
    pub kz: Option<f64>,
    /// `cue`: number (sonradan eklenir)
    pub cue: Option<f64>,
    /// `bekId`: number (sonradan eklenir)
    pub bekId: Option<f64>,
    /// `bekM`: number (sonradan eklenir)
    pub bekM: Option<f64>,
    /// `_oku`: object (sonradan eklenir)
    pub _oku: Option<JB_oku>,
    /// `tp`: number (sonradan eklenir)
    pub tp: Option<f64>,
    /// `_w0`: number (sonradan eklenir)
    pub _w0: Option<f64>,
    /// `_d0`: number (sonradan eklenir)
    pub _d0: Option<f64>,
    /// `_tt`: number (sonradan eklenir)
    pub _tt: Option<f64>,
    /// `cds`: Map (sonradan eklenir)
    pub cds: Option<Vec<(Pid, f64)>>,
    /// `defl`: boolean|number (sonradan eklenir)
    pub defl: Option<JsDeger>,
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
    pub tac: Vec<crate::mac::Tac>,
    /// `st`: object
    pub st: JMSt,
    /// `diz`: dizi (sonradan eklenir)
    pub diz: Option<Vec<Option<crate::mac::Diz>>>,
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
    pub tac0: Option<Vec<Option<crate::mac::Tac>>>,
    /// `evre`: dizi (sonradan eklenir)
    pub evre: Option<Vec<Option<JMEvreE>>>,
    /// `_macC`: object (sonradan eklenir)
    pub _macC: Option<std::collections::BTreeMap<String, crate::mac::MacW>>,
    /// `macDurum`: dizi (sonradan eklenir)
    pub macDurum: Option<Vec<Option<JMMacDurumE>>>,
    /// `_sn`: Map (sonradan eklenir)
    pub _sn: Option<Vec<(Pid, Vec<JM_snVE>)>>,
    /// `_snT`: number (sonradan eklenir)
    pub _snT: Option<f64>,
    /// `_kayL`: object (sonradan eklenir)
    pub _kayL: Option<JM_kayL>,
    /// `E`: object (sonradan eklenir)
    pub E: Option<JME>,
    /// `att`: number (sonradan eklenir)
    pub att: Option<f64>,
    /// `_perde`: null
    pub _perde: Option<crate::mac::Perde>,
    /// `_kuyuTop`: number (sonradan eklenir)
    pub _kuyuTop: Option<f64>,
    /// `_temaslar`: object (sonradan eklenir)
    pub _temaslar: Option<JM_temaslar>,
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
    /// `_temas`: object (sonradan eklenir)
    pub _temas: Option<JM_temas>,
    /// `winTeam`: number (sonradan eklenir)
    pub winTeam: Option<f64>,
    /// `hhKul`: boolean
    pub hhKul: bool,
    /// `gecisGeri`: number (sonradan eklenir)
    pub gecisGeri: Option<f64>,
    /// `kp`: null|object
    pub kp: Option<JMKp>,
    /// `_yzBas`: number (sonradan eklenir)
    pub _yzBas: Option<f64>,
    /// `_yzBekle`: boolean
    pub _yzBekle: bool,
    /// `_acikK`: object (sonradan eklenir)
    pub _acikK: Option<JM_acikK>,
    /// (mac) `ps`: oyuncular (Pid = sıra)
    pub ps: Vec<Oyuncu>,
    /// (mac) `r`: maçın zar dizisi
    pub r: crate::rng::Rng,
    /// (mac) `events`: olaylar (en yenisi başta, en çok 120)
    pub events: Vec<crate::mac::Olay>,
    /// (mac) bugüne dek çıkan olay sayısı (events 120'de kesilir; yeni olayları bulmak için)
    pub ev_n: u64,
    /// (mac) taslakta olmayan maç alanları (_lite, _kafa, fake, __mH …)
    pub mac: crate::mac::MacEk,
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
    /// `riskCay`: number
    pub riskCay: f64,
    /// `riskP0`: number
    pub riskP0: f64,
    /// `riskPk`: number
    pub riskPk: f64,
    /// `surKarar`: number
    pub surKarar: f64,
    /// `asanPay`: number
    pub asanPay: f64,
    /// `amIleri`: number
    pub amIleri: f64,
    /// `amGec`: number
    pub amGec: f64,
    /// `amAc`: number
    pub amAc: f64,
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
    /// `disc`: number
    pub disc: f64,
    /// `istekK`: number
    pub istekK: f64,
    /// `planK`: number
    pub planK: f64,
    /// `belirsiz`: number
    pub belirsiz: f64,
    /// `bekCek`: number
    pub bekCek: f64,
    /// `bekleAtla`: number
    pub bekleAtla: f64,
    /// `bekBedel`: number
    pub bekBedel: f64,
    /// `bekTempo`: number
    pub bekTempo: f64,
    /// `cevTempo`: number
    pub cevTempo: f64,
    /// `otErr`: number
    pub otErr: f64,
    /// `otOk`: number
    pub otOk: f64,
    /// `otMargin`: number
    pub otMargin: f64,
    /// `passMax`: number
    pub passMax: f64,
    /// `passMaxF`: number
    pub passMaxF: f64,
    /// `spcBallV`: number
    pub spcBallV: f64,
    /// `useValTab`: boolean
    pub useValTab: bool,
    /// `useCurve`: boolean
    pub useCurve: bool,
    /// `useSpace`: boolean
    pub useSpace: bool,
    /// `look`: boolean
    pub look: bool,
    /// `lookReuse`: number
    pub lookReuse: f64,
    /// `lookMove`: number
    pub lookMove: f64,
    /// `lookAge`: number
    pub lookAge: f64,
    /// `lookR`: number
    pub lookR: f64,
    /// `lookN`: number
    pub lookN: f64,
    /// `lookT`: number
    pub lookT: f64,
    /// `lookMargin`: number
    pub lookMargin: f64,
    /// `laneLag`: number
    pub laneLag: f64,
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
    /// `chV`: number
    pub chV: f64,
    /// `pierceV`: number
    pub pierceV: f64,
    /// `chgMin`: number
    pub chgMin: f64,
    /// `enBedel`: number
    pub enBedel: f64,
    /// `sarjUfuk`: number
    pub sarjUfuk: f64,
    /// `sarjEgim`: number
    pub sarjEgim: f64,
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
    /// `sabirEn`: number
    pub sabirEn: f64,
    /// `direktK`: number
    pub direktK: f64,
    /// `kontraL`: number
    pub kontraL: f64,
    /// `sarjKap`: number
    pub sarjKap: f64,
    /// `zoneMin`: number
    pub zoneMin: f64,
    /// `zoneD`: number
    pub zoneD: f64,
    /// `chRecv`: number
    pub chRecv: f64,
    /// `threadMin`: number
    pub threadMin: f64,
    /// `threadLane`: number
    pub threadLane: f64,
    /// `threadOff`: dizi
    pub threadOff: Vec<f64>,
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
    /// `nSpots`: number
    pub nSpots: f64,
    /// `nFullLook`: number
    pub nFullLook: f64,
    /// `nMates`: number
    pub nMates: f64,
    /// `growMax`: number
    pub growMax: f64,
    /// `shotRange`: number
    pub shotRange: f64,
    /// `tPS`: number
    pub tPS: f64,
    /// `tPB`: number
    pub tPB: f64,
    /// `pbOrta`: number
    pub pbOrta: f64,
    /// `pbGen`: number
    pub pbGen: f64,
    /// `tSut`: number
    pub tSut: f64,
    /// `tSab`: number
    pub tSab: f64,
    /// `imkEski`: boolean
    pub imkEski: bool,
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
    pub _sut: Option<JsDeger>,
    /// `_zin`: null
    pub _zin: Option<JsDeger>,
    /// `_kuyruk`: undef
    pub _kuyruk: Option<JsDeger>,
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
    /// `_kayipYap`: undef
    pub _kayipYap: Option<JsDeger>,
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
    /// `_noFake`: boolean (kafa1 ekledi: fakeSecenek'in iç dünyası)
    pub _noFake: bool,
}

/// (kafa1'in adı) JS `D._kanca`
pub type JD_kanca = DKanca;

/// JS `o.show` (kafa1 ekledi): fake'in gösterdiği vuruş: tür, hedef, inanılırlık.
#[derive(Clone, Debug, Default)]
pub struct JOShow {
    pub k: String,
    pub to: JOTo,
    pub c: f64,
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
    pub enV: Option<f64>,
    pub prensip: Option<String>,
    /// tartının dünyası (JS `o._pw`)
    #[serde(with = "crate::paket::dunya_kod")]
    pub _pw: Option<std::sync::Arc<Vec<std::sync::Arc<Oyuncu>>>>,
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
    pub asim: Option<f64>,
    pub kendine: bool,
    pub lead: Option<Nokta>,
    pub hold: Option<f64>,
    /// (mac) maçın yazdığı / okuduğu, taslakta olmayan alanlar
    pub savW: Option<f64>,
    pub savU: Option<f64>,
    pub ot: Option<f64>,
    pub noErr: bool,
    pub devam: Option<f64>,
    pub show: Option<crate::mac::gecici_dis::FakeShow>,
    /// (mac) kafa günlüğü için seçeneğin kimliği (JS nesne kimliği)
    pub _kafaId: Option<u64>,
    pub shot: Option<String>,
    // --- Kafa 2'nin tartısının yazdıkları ---
    pub kE: Option<f64>,
    pub bekci: Option<f64>,
    pub asan: Option<f64>,
    pub _at: Option<Nokta>,
    #[serde(with = "crate::paket::dunya_kod")]
    pub _w: Option<std::sync::Arc<Vec<std::sync::Arc<Oyuncu>>>>,
    pub _T: Option<f64>,
    #[serde(with = "crate::paket::dunya_kod")]
    pub _wE: Option<std::sync::Arc<Vec<std::sync::Arc<Oyuncu>>>>,
    pub _en: Option<f64>,
    pub _enB: Option<f64>,
    pub _enR: Option<f64>,
    pub _enS: Option<f64>,
    pub look: Option<f64>,
    pub next: Option<f64>,
    pub hw: Option<SecHw>,
    pub amac: Option<SecAmac>,
    pub sonra: Option<String>,
    pub geri: Option<f64>,
    pub ust: Option<f64>,
    pub cek: Option<f64>,
    pub why: Option<String>,
    pub riskCay: Option<f64>,
    pub zaman: Option<f64>,
    pub yerlesme: Option<f64>,
    pub drk: Option<f64>,
    pub calim: bool,
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

/// JS `o.hw` (beklemenin hayali dünyası)
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct SecHw {
    pub kayip: bool,
    pub tik: Option<f64>,
    pub T: f64,
    pub v: Option<f64>,
    pub sonra: Option<String>,
    pub sonraP: Option<f64>,
    pub Pk: Option<f64>,
}

/// JS `o.amac`
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct SecAmac {
    pub il: f64,
    pub ac: f64,
    pub gec: f64,
    pub n0: f64,
    pub n1: f64,
    pub bekle: bool,
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

/// JS `p._otB`
#[derive(Clone, Debug, Default)]
pub struct JP_otB {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
    /// `i`: number
    pub i: f64,
    /// `name`: string
    pub name: String,
    /// `vx`: number
    pub vx: f64,
    /// `vy`: number
    pub vy: f64,
    /// `tx`: number
    pub tx: f64,
    /// `ty`: number
    pub ty: f64,
    /// `a`: object
    pub a: JP_otBA,
    /// `id`: number
    pub id: f64,
    /// `team`: number
    pub team: f64,
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
    /// `dur`: number
    pub dur: f64,
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

/// JS `b._oku`
#[derive(Clone, Debug, Default)]
pub struct JB_oku {
    /// `111`: object
    pub k111: JB_okuK111,
    /// `112`: object
    pub k112: JB_okuK112,
    /// `113`: object
    pub k113: JB_okuK113,
    /// `114`: object
    pub k114: JB_okuK114,
    /// `115`: object
    pub k115: JB_okuK115,
    /// `116`: object
    pub k116: JB_okuK116,
    /// `-112`: object
    pub _112: JB_oku_112,
    /// `-113`: object
    pub _113: JB_oku_113,
    /// `-114`: object
    pub _114: JB_oku_114,
    /// `-115`: object
    pub _115: JB_oku_115,
    /// `-116`: object
    pub _116: JB_oku_116,
    /// `-117`: object
    pub _117: JB_oku_117,
    /// `1`: object
    pub k1: JB_okuK1,
    /// `2`: object
    pub k2: JB_okuK2,
    /// `3`: object
    pub k3: JB_okuK3,
    /// `4`: object
    pub k4: JB_okuK4,
    /// `5`: object
    pub k5: JB_okuK5,
    /// `6`: object
    pub k6: JB_okuK6,
    /// `-2`: object
    pub _2: JB_oku_2,
    /// `-3`: object
    pub _3: JB_oku_3,
    /// `-4`: object
    pub _4: JB_oku_4,
    /// `-5`: object
    pub _5: JB_oku_5,
    /// `-6`: object
    pub _6: JB_oku_6,
    /// `-7`: object
    pub _7: JB_oku_7,
    /// `-2110`: object
    pub _2110: JB_oku_2110,
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
    /// `tuzak`: null
    pub tuzak: Option<JsDeger>,
    /// `sistem`: string
    pub sistem: String,
    /// `siddet`: number
    pub siddet: f64,
    /// `savGen`: number
    pub savGen: f64,
    /// `gecisH`: string
    pub gecisH: String,
    /// `kpYer`: string
    pub kpYer: String,
    /// `kpSure`: number
    pub kpSure: f64,
    /// `kaleOnu`: string
    pub kaleOnu: String,
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
    /// `tetik`: dizi
    pub tetik: Vec<String>,
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
    /// `swN`: number
    pub swN: f64,
    /// `enH`: dizi
    pub enH: Vec<crate::mac::EnSayac>,
    /// `enP`: object
    pub enP: std::collections::BTreeMap<usize, crate::mac::EnSayac>,
    /// `sprF`: dizi
    pub sprF: Vec<f64>,
    /// `sprT`: dizi
    pub sprT: Vec<f64>,
    /// `enCal`: dizi
    pub enCal: Vec<f64>,
    /// `calim`: dizi
    pub calim: Vec<f64>,
    /// `cal`: object
    pub cal: Option<JMStCal>,
    /// (mac) taslakta olmayan sayaçlar (boş = yok)
    pub fake: Vec<f64>,
    pub fakeOk: Vec<f64>,
    pub ot: Vec<f64>,
    pub combo: Vec<f64>,
    pub direk: Vec<f64>,
}

/// JS `m.diz[]`
#[derive(Clone, Debug, Default)]
pub struct JMDizE {
    /// `sav`: dizi|string
    pub sav: JsDeger,
    /// `huc`: dizi|string
    pub huc: JsDeger,
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
    /// `aday`: null|number
    pub aday: Option<f64>,
    /// `att`: number
    pub att: f64,
    /// `gain`: number
    pub gain: f64,
    /// `ball`: TOP|TOPKOPYA|null|object
    pub ball: Option<TopRef>,
    /// `ballT`: number
    pub ballT: f64,
    /// `defl`: number
    pub defl: f64,
    /// `yr`: object
    pub yr: JMShYr,
    /// `gs`: object
    pub gs: JMShGs,
    /// `yvBas`: object
    pub yvBas: JMShYvBas,
    /// `fr`: object
    pub fr: JMShFr,
    /// `frT`: number
    pub frT: f64,
    /// `frB`: TOP|TOPKOPYA|object
    pub frB: TopRef,
    /// `adayT`: number
    pub adayT: f64,
}

/// JS `m.ra[][]`
#[derive(Clone, Debug, Default)]
pub struct JMRaEE {
    /// `h`: dizi
    pub h: Vec<f64>,
    /// `s`: dizi
    pub s: Vec<f64>,
}

/// JS `m.tac0[]`
#[derive(Clone, Debug, Default)]
pub struct JMTac0E {
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
    /// `tuzak`: null
    pub tuzak: Option<JsDeger>,
    /// `sistem`: string
    pub sistem: String,
    /// `siddet`: number
    pub siddet: f64,
    /// `savGen`: number
    pub savGen: f64,
    /// `gecisH`: string
    pub gecisH: String,
    /// `kpYer`: string
    pub kpYer: String,
    /// `kpSure`: number
    pub kpSure: f64,
    /// `kaleOnu`: string
    pub kaleOnu: String,
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
    /// `tetik`: dizi
    pub tetik: Vec<String>,
    /// `durus`: object
    pub durus: JMTac0EDurus,
    /// `yz`: boolean
    pub yz: bool,
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

/// JS `m._macC`
#[derive(Clone, Debug, Default)]
pub struct JM_macC {
    /// `0:74`: object
    pub k0_74: JM_macCK0_74,
    /// `0:73`: object
    pub k0_73: JM_macCK0_73,
    /// `0:72`: object
    pub k0_72: JM_macCK0_72,
    /// `0:71`: object
    pub k0_71: JM_macCK0_71,
    /// `0:70`: object
    pub k0_70: JM_macCK0_70,
    /// `0:69`: object
    pub k0_69: JM_macCK0_69,
    /// `0:68`: object
    pub k0_68: JM_macCK0_68,
    /// `0:67`: object
    pub k0_67: JM_macCK0_67,
    /// `0:66`: object
    pub k0_66: JM_macCK0_66,
    /// `0:65`: object
    pub k0_65: JM_macCK0_65,
    /// `0:64`: object
    pub k0_64: JM_macCK0_64,
    /// `0:63`: object
    pub k0_63: JM_macCK0_63,
    /// `0:62`: object
    pub k0_62: JM_macCK0_62,
}

/// JS `m.macDurum[]`
#[derive(Clone, Debug, Default)]
pub struct JMMacDurumE {
    /// `oran`: number
    pub oran: f64,
    /// `g`: number
    pub g: f64,
    /// `c`: number
    pub c: f64,
}

/// JS `m._sn{v}[]`
#[derive(Clone, Debug, Default)]
pub struct JM_snVE {
    /// `a`: object
    pub a: JM_snVEA,
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
    /// `R`: number
    pub R: f64,
    /// `D`: number
    pub D: f64,
    /// `team`: number
    pub team: f64,
    /// `role`: string
    pub role: String,
    /// `press`: boolean
    pub press: bool,
    /// `ref`: OYUNCU
    pub r#ref: Pid,
    /// `gvx`: number
    pub gvx: f64,
    /// `gvy`: number
    pub gvy: f64,
}

/// JS `m._kayL`
#[derive(Clone, Debug, Default)]
pub struct JM_kayL {
    /// `key`: string
    pub key: String,
    /// `f`: function
    pub f: JsFn,
}

/// JS `m.E`
#[derive(Clone, Debug, Default)]
pub struct JME {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `m._temaslar`
#[derive(Clone, Debug, Default)]
pub struct JM_temaslar {
    /// `t`: number
    pub t: f64,
    /// `L`: dizi
    pub L: Vec<JM_temaslarLE>,
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
    pub r: crate::mac::gecici_dis::Karar,
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
    /// `nd`: number
    pub nd: f64,
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
    pub r: crate::mac::gecici_dis::Karar,
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

/// JS `m._acikK`
#[derive(Clone, Debug, Default)]
pub struct JM_acikK {
    /// `key`: string
    pub key: String,
    /// `v`: dizi
    pub v: Vec<JM_acikKVE>,
}

/// JS `o.to`
#[derive(Clone, Debug, Default)]
pub struct JOTo {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `o.dir`
#[derive(Clone, Debug, Default)]
pub struct JODir {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `o.toE`
#[derive(Clone, Debug, Default)]
pub struct JOToE {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `o.dok`
#[derive(Clone, Debug, Default)]
pub struct JODok {
    /// `tutar`: number
    pub tutar: f64,
    /// `durum`: number
    pub durum: f64,
    /// `kayip`: number
    pub kayip: f64,
    /// `sure`: number
    pub sure: f64,
    /// `sut`: number
    pub sut: f64,
    /// `ileri`: number
    pub ileri: f64,
    /// `surme`: number
    pub surme: f64,
    /// `zemin`: number
    pub zemin: f64,
    /// `sonra`: string
    pub sonra: String,
    /// `simdi`: number
    pub simdi: f64,
    /// `alici`: number
    pub alici: f64,
    /// `sekme`: number
    pub sekme: f64,
    /// `gol`: number
    pub gol: f64,
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
    /// holdWorld denemesi (JS `det.Psim`)
    pub Psim: Option<f64>,
    /// bakılacak arkadaş (JS `det.lookTo`)
    pub lookTo: Option<Pid>,
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

/// JS `o._pw[]`
#[derive(Clone, Debug, Default)]
pub struct JO_pwE {
    /// `id`: number
    pub id: f64,
    /// `team`: number
    pub team: f64,
    /// `i`: number
    pub i: f64,
    /// `role`: string
    pub role: String,
    /// `slot`: object
    pub slot: JO_pwESlot,
    /// `name`: string
    pub name: String,
    /// `a`: object
    pub a: JO_pwEA,
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
    /// `en`: number
    pub en: f64,
    /// `driveTo`: null|object
    pub driveTo: Option<JO_pwEDriveTo>,
    /// `_dur`: null|number
    pub _dur: Option<f64>,
    /// `_korT`: number
    pub _korT: f64,
    /// `bekBas`: null
    pub bekBas: Option<JsDeger>,
    /// `noTouch`: boolean
    pub noTouch: bool,
    /// `hist`: dizi
    pub hist: Vec<JO_pwEHistE>,
    /// `lock`: number
    pub lock: f64,
    /// `kapPrev`: null
    pub kapPrev: Option<JsDeger>,
    /// `zonePrev`: OYUNCU|null
    pub zonePrev: Option<Pid>,
    /// `swPrev`: OYUNCU|null
    pub swPrev: Option<Pid>,
    /// `uaPrev`: OYUNCU|null
    pub uaPrev: Option<Pid>,
    /// `rec`: number
    pub rec: f64,
    /// `tutQ`: OYUNCU|null
    pub tutQ: Option<Pid>,
    /// `sprint`: boolean
    pub sprint: bool,
    /// `press`: boolean
    pub press: bool,
    /// `job`: string
    pub job: String,
    /// `otx`: null|number
    pub otx: Option<f64>,
    /// `_bk`: dizi
    pub _bk: Vec<Option<JO_pwE_bkE>>,
    /// `_ktB`: object
    pub _ktB: JO_pwE_ktB,
    /// `_topSpr`: boolean
    pub _topSpr: bool,
    /// `_kosuK`: null|string
    pub _kosuK: Option<String>,
    /// `_hizK`: null|number
    pub _hizK: Option<f64>,
    /// `_evx`: number
    pub _evx: f64,
    /// `_evy`: number
    pub _evy: f64,
    /// `_yog`: number
    pub _yog: f64,
    /// `_D0`: number
    pub _D0: f64,
    /// `_Dl`: number
    pub _Dl: f64,
    /// `_enOnce`: number
    pub _enOnce: f64,
    /// `ca`: number
    pub ca: f64,
    /// `_don`: number
    pub _don: f64,
    /// `drive`: boolean
    pub drive: bool,
    /// `_dke`: number
    pub _dke: f64,
    /// `past`: dizi
    pub past: Vec<JO_pwEPastE>,
    /// `holdD`: boolean
    pub holdD: bool,
    /// `_plan`: null
    pub _plan: Option<JsDeger>,
    /// `_surK`: null
    pub _surK: Option<JsDeger>,
    /// `_sprIst`: boolean
    pub _sprIst: bool,
    /// `sarj`: boolean
    pub sarj: bool,
    /// `savur`: null
    pub savur: Option<JsDeger>,
    /// `lastWhy`: string
    pub lastWhy: String,
    /// `_yer`: boolean
    pub _yer: bool,
    /// `amA`: boolean
    pub amA: bool,
    /// `am`: number
    pub am: f64,
    /// `__g`: OYUNCU
    pub __g: Pid,
    /// `__yas`: number
    pub __yas: f64,
    /// `__e`: number
    pub __e: f64,
    /// `rslot`: object
    pub rslot: JO_pwERslot,
    /// `slot0`: object
    pub slot0: JO_pwESlot0,
    /// `rslot0`: object
    pub rslot0: JO_pwERslot0,
    /// `rh`: string
    pub rh: String,
    /// `rs`: string
    pub rs: String,
    /// `tl`: object
    pub tl: JO_pwETl,
    /// `markRef`: OYUNCU
    pub markRef: Pid,
    /// `eg`: object
    pub eg: JO_pwEEg,
    /// `oty`: number
    pub oty: f64,
    /// `_otH`: OYUNCU|null
    pub _otH: Option<Pid>,
    /// `_otB`: null
    pub _otB: Option<JsDeger>,
    /// `_otE`: object
    pub _otE: JO_pwE_otE,
    /// `_otF`: OYUNCU|null
    pub _otF: Option<Pid>,
    /// `_kv`: null
    pub _kv: Option<JsDeger>,
    /// `_amacJob`: null
    pub _amacJob: Option<JsDeger>,
    /// `_kt`: object
    pub _kt: JO_pwE_kt,
    /// `_hizF`: number
    pub _hizF: f64,
    /// `kuyuT`: number
    pub kuyuT: f64,
    /// `sonuk`: boolean
    pub sonuk: bool,
    /// `_kuyuCik`: boolean
    pub _kuyuCik: bool,
    /// `_rcvV`: null|number
    pub _rcvV: Option<f64>,
    /// `_rcvB`: TOPKOPYA|object
    pub _rcvB: TopRef,
    /// `_rcvAbs`: number
    pub _rcvAbs: f64,
    /// `_rcvX`: number
    pub _rcvX: f64,
    /// `_rcvY`: number
    pub _rcvY: f64,
    /// `chB`: TOPKOPYA|object
    pub chB: TopRef,
    /// `_preB`: TOPKOPYA|object
    pub _preB: TopRef,
    /// `_preT`: number
    pub _preT: f64,
    /// `_otA`: object
    pub _otA: JO_pwE_otA,
    /// `_onH`: null
    pub _onH: Option<JsDeger>,
    /// `_lc`: null
    pub _lc: Option<JsDeger>,
    /// `_o`: OYUNCU|object
    pub _o: Pid,
    /// `_donDir`: number
    pub _donDir: f64,
    /// `_donT`: number
    pub _donT: f64,
    /// `_savT`: number
    pub _savT: f64,
    /// `_savD`: number
    pub _savD: f64,
    /// `_blokT`: number
    pub _blokT: f64,
    /// `_omuzT`: number
    pub _omuzT: f64,
    /// `_omEv`: number
    pub _omEv: f64,
    /// `_savCar`: number
    pub _savCar: f64,
    /// `lockAt`: number
    pub lockAt: f64,
    /// `_lockH`: OYUNCU
    pub _lockH: Pid,
    /// `_presT`: number
    pub _presT: f64,
    /// `_presH`: OYUNCU
    pub _presH: Pid,
    /// `_kh`: object
    pub _kh: JO_pwE_kh,
    /// `_khT`: number
    pub _khT: f64,
    /// `_khQ`: OYUNCU
    pub _khQ: Pid,
    /// `_onK`: object
    pub _onK: JO_pwE_onK,
    /// `_kal`: boolean
    pub _kal: bool,
    /// `_kalH`: OYUNCU
    pub _kalH: Pid,
    /// `hamle`: null
    pub hamle: Option<JsDeger>,
    /// `_dolYok`: number
    pub _dolYok: f64,
    /// `adandi`: number
    pub adandi: f64,
    /// `adT`: object
    pub adT: JO_pwEAdT,
    /// `son`: object
    pub son: JO_pwESon,
    /// `adNeden`: string
    pub adNeden: String,
    /// `den`: number
    pub den: f64,
    /// `_sf`: null
    pub _sf: Option<JsDeger>,
    /// `_bosT`: number
    pub _bosT: f64,
    /// `_gvKaz`: object
    pub _gvKaz: JO_pwE_gvKaz,
    /// `pplan`: object
    pub pplan: JO_pwEPplan,
    /// `_kAn`: number
    pub _kAn: f64,
    /// `_kAnT`: number
    pub _kAnT: f64,
    /// `fiskeCd`: number
    pub fiskeCd: f64,
    /// `fiske`: number
    pub fiske: f64,
    /// `fiskeDir`: number
    pub fiskeDir: f64,
    /// `okudu`: number
    pub okudu: f64,
    /// `_winSeen`: number
    pub _winSeen: f64,
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

/// JS `o.lf0`
#[derive(Clone, Debug, Default)]
pub struct JOLf0 {
    /// `vx`: number
    pub vx: f64,
    /// `vy`: number
    pub vy: f64,
    /// `ok`: boolean
    pub ok: bool,
}

/// JS `o.lead`
#[derive(Clone, Debug, Default)]
pub struct JOLead {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `p.tl.huc`
#[derive(Clone, Debug, Default)]
pub struct JPTlHuc {
    /// `kosu`: number
    pub kosu: f64,
    /// `istek`: number
    pub istek: f64,
    /// `destek`: number
    pub destek: f64,
    /// `risk`: number
    pub risk: f64,
    /// `ritim`: number
    pub ritim: f64,
    /// `tempo`: number
    pub tempo: f64,
    /// `gonder`: number
    pub gonder: f64,
    /// `serbest`: number
    pub serbest: f64,
}

/// JS `p.tl.sav`
#[derive(Clone, Debug, Default)]
pub struct JPTlSav {
    /// `adam`: number
    pub adam: f64,
    /// `pres`: number
    pub pres: f64,
}

/// JS `p._otB.a`
#[derive(Clone, Debug, Default)]
pub struct JP_otBA {
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

/// JS `p.savur.o`
#[derive(Clone, Debug, Default)]
pub struct JPSavurO {
    /// `kind`: string
    pub kind: String,
    /// `to`: object
    pub to: JPSavurOTo,
    /// `q`: OYUNCU
    pub q: Pid,
    /// `arrive`: number
    pub arrive: f64,
    /// `tek`: number
    pub tek: f64,
    /// `_own`: number
    pub _own: f64,
    /// `firm`: boolean|undef
    pub firm: Option<bool>,
    /// `spr`: undef
    pub spr: Option<JsDeger>,
    /// `itK`: undef
    pub itK: Option<JsDeger>,
    /// `launch`: object
    pub launch: JPSavurOLaunch,
    /// `sYon`: number
    pub sYon: f64,
    /// `savT`: number
    pub savT: f64,
    /// `_sek`: number
    pub _sek: f64,
    /// `v`: number
    pub v: f64,
    /// `det`: object
    pub det: JPSavurODet,
    /// `tf`: number
    pub tf: f64,
    /// `en`: object
    pub en: JPSavurOEn,
    /// `enV`: number
    pub enV: f64,
    /// `dok`: object
    pub dok: JPSavurODok,
    /// `prensip`: string
    pub prensip: String,
    /// `_pw`: dizi
    pub _pw: Vec<JPSavurO_pwE>,
    /// `_pT`: number
    pub _pT: f64,
    /// `kh`: string
    pub kh: String,
    /// `kendine`: boolean
    pub kendine: bool,
    /// `lead`: object
    pub lead: JPSavurOLead,
    /// `lf0`: object
    pub lf0: JPSavurOLf0,
    /// `shot`: string
    pub shot: String,
}

/// JS `b._oku.111`
#[derive(Clone, Debug, Default)]
pub struct JB_okuK111 {
    /// `k`: number
    pub k: f64,
    /// `on`: number
    pub on: f64,
}

/// JS `b._oku.112`
#[derive(Clone, Debug, Default)]
pub struct JB_okuK112 {
    /// `k`: number
    pub k: f64,
    /// `on`: number
    pub on: f64,
}

/// JS `b._oku.113`
#[derive(Clone, Debug, Default)]
pub struct JB_okuK113 {
    /// `k`: number
    pub k: f64,
    /// `on`: number
    pub on: f64,
}

/// JS `b._oku.114`
#[derive(Clone, Debug, Default)]
pub struct JB_okuK114 {
    /// `k`: number
    pub k: f64,
    /// `on`: number
    pub on: f64,
}

/// JS `b._oku.115`
#[derive(Clone, Debug, Default)]
pub struct JB_okuK115 {
    /// `k`: number
    pub k: f64,
    /// `on`: number
    pub on: f64,
}

/// JS `b._oku.116`
#[derive(Clone, Debug, Default)]
pub struct JB_okuK116 {
    /// `k`: number
    pub k: f64,
    /// `on`: number
    pub on: f64,
}

/// JS `b._oku.-112`
#[derive(Clone, Debug, Default)]
pub struct JB_oku_112 {
    /// `k`: number
    pub k: f64,
    /// `on`: number
    pub on: f64,
}

/// JS `b._oku.-113`
#[derive(Clone, Debug, Default)]
pub struct JB_oku_113 {
    /// `k`: number
    pub k: f64,
    /// `on`: number
    pub on: f64,
}

/// JS `b._oku.-114`
#[derive(Clone, Debug, Default)]
pub struct JB_oku_114 {
    /// `k`: number
    pub k: f64,
    /// `on`: number
    pub on: f64,
}

/// JS `b._oku.-115`
#[derive(Clone, Debug, Default)]
pub struct JB_oku_115 {
    /// `k`: number
    pub k: f64,
    /// `on`: number
    pub on: f64,
}

/// JS `b._oku.-116`
#[derive(Clone, Debug, Default)]
pub struct JB_oku_116 {
    /// `k`: number
    pub k: f64,
    /// `on`: number
    pub on: f64,
}

/// JS `b._oku.-117`
#[derive(Clone, Debug, Default)]
pub struct JB_oku_117 {
    /// `k`: number
    pub k: f64,
    /// `on`: number
    pub on: f64,
}

/// JS `b._oku.1`
#[derive(Clone, Debug, Default)]
pub struct JB_okuK1 {
    /// `k`: number
    pub k: f64,
    /// `on`: number
    pub on: f64,
}

/// JS `b._oku.2`
#[derive(Clone, Debug, Default)]
pub struct JB_okuK2 {
    /// `k`: number
    pub k: f64,
    /// `on`: number
    pub on: f64,
}

/// JS `b._oku.3`
#[derive(Clone, Debug, Default)]
pub struct JB_okuK3 {
    /// `k`: number
    pub k: f64,
    /// `on`: number
    pub on: f64,
}

/// JS `b._oku.4`
#[derive(Clone, Debug, Default)]
pub struct JB_okuK4 {
    /// `k`: number
    pub k: f64,
    /// `on`: number
    pub on: f64,
}

/// JS `b._oku.5`
#[derive(Clone, Debug, Default)]
pub struct JB_okuK5 {
    /// `k`: number
    pub k: f64,
    /// `on`: number
    pub on: f64,
}

/// JS `b._oku.6`
#[derive(Clone, Debug, Default)]
pub struct JB_okuK6 {
    /// `k`: number
    pub k: f64,
    /// `on`: number
    pub on: f64,
}

/// JS `b._oku.-2`
#[derive(Clone, Debug, Default)]
pub struct JB_oku_2 {
    /// `k`: number
    pub k: f64,
    /// `on`: number
    pub on: f64,
}

/// JS `b._oku.-3`
#[derive(Clone, Debug, Default)]
pub struct JB_oku_3 {
    /// `k`: number
    pub k: f64,
    /// `on`: number
    pub on: f64,
}

/// JS `b._oku.-4`
#[derive(Clone, Debug, Default)]
pub struct JB_oku_4 {
    /// `k`: number
    pub k: f64,
    /// `on`: number
    pub on: f64,
}

/// JS `b._oku.-5`
#[derive(Clone, Debug, Default)]
pub struct JB_oku_5 {
    /// `k`: number
    pub k: f64,
    /// `on`: number
    pub on: f64,
}

/// JS `b._oku.-6`
#[derive(Clone, Debug, Default)]
pub struct JB_oku_6 {
    /// `k`: number
    pub k: f64,
    /// `on`: number
    pub on: f64,
}

/// JS `b._oku.-7`
#[derive(Clone, Debug, Default)]
pub struct JB_oku_7 {
    /// `k`: number
    pub k: f64,
    /// `on`: number
    pub on: f64,
}

/// JS `b._oku.-2110`
#[derive(Clone, Debug, Default)]
pub struct JB_oku_2110 {
    /// `k`: number
    pub k: f64,
    /// `on`: number
    pub on: f64,
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

/// JS `m.st.enH[]`
#[derive(Clone, Debug, Default)]
pub struct JMStEnHE {
    /// `sprint`: number
    pub sprint: f64,
    /// `ivme`: number
    pub ivme: f64,
    /// `ikili`: number
    pub ikili: f64,
    /// `alan`: number
    pub alan: f64,
    /// `sarj`: number
    pub sarj: f64,
    /// `dol`: number
    pub dol: f64,
    /// `fake`: number
    pub fake: f64,
    /// `calindi`: number
    pub calindi: f64,
    /// `caldi`: number
    pub caldi: f64,
    /// `dalış`: number
    pub dalis: f64,
}

/// JS `m.st.enP[]`
#[derive(Clone, Debug, Default)]
pub struct JMStEnPE {
    /// `sarj`: number
    pub sarj: f64,
    /// `dol`: number
    pub dol: f64,
    /// `sprint`: number
    pub sprint: f64,
    /// `alan`: number
    pub alan: f64,
    /// `dalış`: number
    pub dalis: f64,
    /// `ikili`: number
    pub ikili: f64,
    /// `fake`: number
    pub fake: f64,
    /// `calindi`: number
    pub calindi: f64,
    /// `caldi`: number
    pub caldi: f64,
}

/// JS `m.st.cal`
#[derive(Clone, Debug, Default)]
pub struct JMStCal {
    /// `n`: number
    pub n: f64,
    /// `pk`: number
    pub pk: f64,
    /// `g`: number
    pub g: f64,
}

/// JS `m.sh.yr`
#[derive(Clone, Debug, Default)]
pub struct JMShYr {
    /// `gain`: number
    pub gain: f64,
    /// `dt`: number
    pub dt: f64,
    /// `on`: boolean
    pub on: bool,
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

/// JS `m.sh.yvBas`
#[derive(Clone, Debug, Default)]
pub struct JMShYvBas {
    /// `h`: OYUNCU
    pub h: Pid,
    /// `t`: number
    pub t: f64,
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

/// JS `m.tac0[].durus`
#[derive(Clone, Debug, Default)]
pub struct JMTac0EDurus {
    /// `huc`: dizi
    pub huc: Vec<Vec<JMTac0EDurusHucEE>>,
    /// `sav`: dizi
    pub sav: Vec<Vec<JMTac0EDurusSavEE>>,
}

/// JS `m._macC.0:74`
#[derive(Clone, Debug, Default)]
pub struct JM_macCK0_74 {
    /// `g`: number
    pub g: f64,
    /// `c`: number
    pub c: f64,
    /// `e0`: number
    pub e0: f64,
}

/// JS `m._macC.0:73`
#[derive(Clone, Debug, Default)]
pub struct JM_macCK0_73 {
    /// `g`: number
    pub g: f64,
    /// `c`: number
    pub c: f64,
    /// `e0`: number
    pub e0: f64,
}

/// JS `m._macC.0:72`
#[derive(Clone, Debug, Default)]
pub struct JM_macCK0_72 {
    /// `g`: number
    pub g: f64,
    /// `c`: number
    pub c: f64,
    /// `e0`: number
    pub e0: f64,
}

/// JS `m._macC.0:71`
#[derive(Clone, Debug, Default)]
pub struct JM_macCK0_71 {
    /// `g`: number
    pub g: f64,
    /// `c`: number
    pub c: f64,
    /// `e0`: number
    pub e0: f64,
}

/// JS `m._macC.0:70`
#[derive(Clone, Debug, Default)]
pub struct JM_macCK0_70 {
    /// `g`: number
    pub g: f64,
    /// `c`: number
    pub c: f64,
    /// `e0`: number
    pub e0: f64,
}

/// JS `m._macC.0:69`
#[derive(Clone, Debug, Default)]
pub struct JM_macCK0_69 {
    /// `g`: number
    pub g: f64,
    /// `c`: number
    pub c: f64,
    /// `e0`: number
    pub e0: f64,
}

/// JS `m._macC.0:68`
#[derive(Clone, Debug, Default)]
pub struct JM_macCK0_68 {
    /// `g`: number
    pub g: f64,
    /// `c`: number
    pub c: f64,
    /// `e0`: number
    pub e0: f64,
}

/// JS `m._macC.0:67`
#[derive(Clone, Debug, Default)]
pub struct JM_macCK0_67 {
    /// `g`: number
    pub g: f64,
    /// `c`: number
    pub c: f64,
    /// `e0`: number
    pub e0: f64,
}

/// JS `m._macC.0:66`
#[derive(Clone, Debug, Default)]
pub struct JM_macCK0_66 {
    /// `g`: number
    pub g: f64,
    /// `c`: number
    pub c: f64,
    /// `e0`: number
    pub e0: f64,
}

/// JS `m._macC.0:65`
#[derive(Clone, Debug, Default)]
pub struct JM_macCK0_65 {
    /// `g`: number
    pub g: f64,
    /// `c`: number
    pub c: f64,
    /// `e0`: number
    pub e0: f64,
}

/// JS `m._macC.0:64`
#[derive(Clone, Debug, Default)]
pub struct JM_macCK0_64 {
    /// `g`: number
    pub g: f64,
    /// `c`: number
    pub c: f64,
    /// `e0`: number
    pub e0: f64,
}

/// JS `m._macC.0:63`
#[derive(Clone, Debug, Default)]
pub struct JM_macCK0_63 {
    /// `g`: number
    pub g: f64,
    /// `c`: number
    pub c: f64,
    /// `e0`: number
    pub e0: f64,
}

/// JS `m._macC.0:62`
#[derive(Clone, Debug, Default)]
pub struct JM_macCK0_62 {
    /// `g`: number
    pub g: f64,
    /// `c`: number
    pub c: f64,
    /// `e0`: number
    pub e0: f64,
}

/// JS `m._sn{v}[].a`
#[derive(Clone, Debug, Default)]
pub struct JM_snVEA {
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

/// JS `m._temaslar.L[]`
#[derive(Clone, Debug, Default)]
pub struct JM_temaslarLE {
    /// `i`: number
    pub i: f64,
    /// `j`: number
    pub j: f64,
    /// `J`: number
    pub J: f64,
    /// `o`: number
    pub o: f64,
    /// `s`: number
    pub s: f64,
}

/// JS `m.pre.r`
#[derive(Clone, Debug, Default)]
pub struct JMPreR {
    /// `best`: object
    pub best: JMPreRBest,
    /// `opts`: dizi
    pub opts: Vec<JMPreROptsE>,
    /// `T`: number
    pub T: f64,
}

/// JS `m.ctl.r`
#[derive(Clone, Debug, Default)]
pub struct JMCtlR {
    /// `best`: object
    pub best: JMCtlRBest,
    /// `opts`: dizi
    pub opts: Vec<JMCtlROptsE>,
    /// `T`: number
    pub T: f64,
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
    /// `hw`: undef
    pub hw: Option<JsDeger>,
    /// `bulus`: undef
    pub bulus: Option<JsDeger>,
    /// `am`: undef
    pub am: Option<JsDeger>,
    /// `en`: object|undef
    pub en: Option<JMLastDecOptsEEn>,
    /// `drk`: undef
    pub drk: Option<JsDeger>,
    /// `sjK`: number|undef
    pub sjK: Option<f64>,
    /// `kh`: string|undef
    pub kh: Option<String>,
    /// `rc`: undef
    pub rc: Option<JsDeger>,
    /// `cek`: undef
    pub cek: Option<JsDeger>,
    /// `devam`: number|undef
    pub devam: Option<f64>,
    /// `neden`: undef
    pub neden: Option<JsDeger>,
    /// `bak`: undef
    pub bak: Option<JsDeger>,
    /// `dok`: object
    pub dok: JMLastDecOptsEDok,
}

/// JS `m._acikK.v[]`
#[derive(Clone, Debug, Default)]
pub struct JM_acikKVE {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
    /// `yv`: number
    pub yv: f64,
}

/// JS `o.det.end`
#[derive(Clone, Debug, Default)]
pub struct JODetEnd {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `o.det.endM`
#[derive(Clone, Debug, Default)]
pub struct JODetEndM {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `o.det.endO`
#[derive(Clone, Debug, Default)]
pub struct JODetEndO {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `o.det.endS`
#[derive(Clone, Debug, Default)]
pub struct JODetEndS {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `o.det.endD`
#[derive(Clone, Debug, Default)]
pub struct JODetEndD {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `o.det.endSek`
#[derive(Clone, Debug, Default)]
pub struct JODetEndSek {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `o._pw[].slot`
#[derive(Clone, Debug, Default)]
pub struct JO_pwESlot {
    /// `role`: string
    pub role: String,
    /// `d`: number
    pub d: f64,
    /// `l`: number
    pub l: f64,
}

/// JS `o._pw[].a`
#[derive(Clone, Debug, Default)]
pub struct JO_pwEA {
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

/// JS `o._pw[].driveTo`
#[derive(Clone, Debug, Default)]
pub struct JO_pwEDriveTo {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `o._pw[].hist[]`
#[derive(Clone, Debug, Default)]
pub struct JO_pwEHistE {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
    /// `vx`: number
    pub vx: f64,
    /// `vy`: number
    pub vy: f64,
}

/// JS `o._pw[]._bk[]`
#[derive(Clone, Debug, Default)]
pub struct JO_pwE_bkE {
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

/// JS `o._pw[]._ktB`
#[derive(Clone, Debug, Default)]
pub struct JO_pwE_ktB {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `o._pw[].past[]`
#[derive(Clone, Debug, Default)]
pub struct JO_pwEPastE {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `o._pw[].rslot`
#[derive(Clone, Debug, Default)]
pub struct JO_pwERslot {
    /// `d`: number
    pub d: f64,
    /// `l`: number
    pub l: f64,
}

/// JS `o._pw[].slot0`
#[derive(Clone, Debug, Default)]
pub struct JO_pwESlot0 {
    /// `role`: string
    pub role: String,
    /// `d`: number
    pub d: f64,
    /// `l`: number
    pub l: f64,
}

/// JS `o._pw[].rslot0`
#[derive(Clone, Debug, Default)]
pub struct JO_pwERslot0 {
    /// `d`: number
    pub d: f64,
    /// `l`: number
    pub l: f64,
}

/// JS `o._pw[].tl`
#[derive(Clone, Debug, Default)]
pub struct JO_pwETl {
    /// `huc`: object
    pub huc: JO_pwETlHuc,
    /// `sav`: object
    pub sav: JO_pwETlSav,
}

/// JS `o._pw[].eg`
#[derive(Clone, Debug, Default)]
pub struct JO_pwEEg {
    /// `ps`: number
    pub ps: f64,
    /// `rk`: number
    pub rk: f64,
}

/// JS `o._pw[]._otE`
#[derive(Clone, Debug, Default)]
pub struct JO_pwE_otE {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `o._pw[]._kt`
#[derive(Clone, Debug, Default)]
pub struct JO_pwE_kt {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `o._pw[]._otA`
#[derive(Clone, Debug, Default)]
pub struct JO_pwE_otA {
    /// `b`: TOPKOPYA|object
    pub b: TopRef,
    /// `ok`: boolean
    pub ok: bool,
}

/// JS `o._pw[]._kh`
#[derive(Clone, Debug, Default)]
pub struct JO_pwE_kh {
    /// `dx`: number
    pub dx: f64,
    /// `dy`: number
    pub dy: f64,
}

/// JS `o._pw[]._onK`
#[derive(Clone, Debug, Default)]
pub struct JO_pwE_onK {
    /// `h`: number
    pub h: f64,
    /// `koy`: boolean
    pub koy: bool,
}

/// JS `o._pw[].adT`
#[derive(Clone, Debug, Default)]
pub struct JO_pwEAdT {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `o._pw[].son`
#[derive(Clone, Debug, Default)]
pub struct JO_pwESon {
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

/// JS `o._pw[]._gvKaz`
#[derive(Clone, Debug, Default)]
pub struct JO_pwE_gvKaz {
    /// `h`: number
    pub h: f64,
    /// `t`: number
    pub t: f64,
    /// `kaz`: number
    pub kaz: f64,
}

/// JS `o._pw[].pplan`
#[derive(Clone, Debug, Default)]
pub struct JO_pwEPplan {
    /// `k`: string
    pub k: String,
    /// `s`: number
    pub s: f64,
    /// `t0`: number
    pub t0: f64,
    /// `until`: number
    pub until: f64,
}

/// JS `p.savur.o.to`
#[derive(Clone, Debug, Default)]
pub struct JPSavurOTo {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `p.savur.o.launch`
#[derive(Clone, Debug, Default)]
pub struct JPSavurOLaunch {
    /// `vx`: number
    pub vx: f64,
    /// `vy`: number
    pub vy: f64,
    /// `ok`: boolean
    pub ok: bool,
}

/// JS `p.savur.o.det`
#[derive(Clone, Debug, Default)]
pub struct JPSavurODet {
    /// `P`: number
    pub P: f64,
    /// `Pk`: number
    pub Pk: f64,
    /// `Pl`: number
    pub Pl: f64,
    /// `end`: object
    pub end: JPSavurODetEnd,
    /// `endM`: object
    pub endM: JPSavurODetEndM,
    /// `endO`: object
    pub endO: JPSavurODetEndO,
    /// `Ps`: number
    pub Ps: f64,
    /// `endS`: object
    pub endS: JPSavurODetEndS,
    /// `endD`: object
    pub endD: JPSavurODetEndD,
    /// `Psek`: number
    pub Psek: f64,
    /// `endSek`: object
    pub endSek: JPSavurODetEndSek,
}

/// JS `p.savur.o.en`
#[derive(Clone, Debug, Default)]
pub struct JPSavurOEn {
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

/// JS `p.savur.o.dok`
#[derive(Clone, Debug, Default)]
pub struct JPSavurODok {
    /// `tutar`: number
    pub tutar: f64,
    /// `alici`: number
    pub alici: f64,
    /// `durum`: number
    pub durum: f64,
    /// `sekme`: number
    pub sekme: f64,
    /// `kayip`: number
    pub kayip: f64,
    /// `sure`: number
    pub sure: f64,
    /// `sut`: number
    pub sut: f64,
    /// `ileri`: number
    pub ileri: f64,
    /// `surme`: number
    pub surme: f64,
    /// `zemin`: number
    pub zemin: f64,
    /// `gol`: number
    pub gol: f64,
}

/// JS `p.savur.o._pw[]`
#[derive(Clone, Debug, Default)]
pub struct JPSavurO_pwE {
    /// `id`: number
    pub id: f64,
    /// `team`: number
    pub team: f64,
    /// `i`: number
    pub i: f64,
    /// `role`: string
    pub role: String,
    /// `slot`: object
    pub slot: JPSavurO_pwESlot,
    /// `name`: string
    pub name: String,
    /// `a`: object
    pub a: JPSavurO_pwEA,
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
    /// `en`: number
    pub en: f64,
    /// `driveTo`: null|object
    pub driveTo: Option<JPSavurO_pwEDriveTo>,
    /// `_dur`: null|number
    pub _dur: Option<f64>,
    /// `_korT`: number
    pub _korT: f64,
    /// `bekBas`: null|number
    pub bekBas: Option<f64>,
    /// `noTouch`: boolean
    pub noTouch: bool,
    /// `hist`: dizi
    pub hist: Vec<JPSavurO_pwEHistE>,
    /// `lock`: number
    pub lock: f64,
    /// `kapPrev`: null
    pub kapPrev: Option<JsDeger>,
    /// `zonePrev`: OYUNCU|null
    pub zonePrev: Option<Pid>,
    /// `swPrev`: OYUNCU|null
    pub swPrev: Option<Pid>,
    /// `uaPrev`: null
    pub uaPrev: Option<JsDeger>,
    /// `rec`: number
    pub rec: f64,
    /// `tutQ`: OYUNCU|null
    pub tutQ: Option<Pid>,
    /// `sprint`: boolean
    pub sprint: bool,
    /// `press`: boolean
    pub press: bool,
    /// `job`: string
    pub job: String,
    /// `otx`: null|number
    pub otx: Option<f64>,
    /// `_bk`: dizi
    pub _bk: Vec<Option<JPSavurO_pwE_bkE>>,
    /// `_ktB`: object
    pub _ktB: JPSavurO_pwE_ktB,
    /// `_topSpr`: boolean
    pub _topSpr: bool,
    /// `_kosuK`: null|string
    pub _kosuK: Option<String>,
    /// `_hizK`: null|number
    pub _hizK: Option<f64>,
    /// `_evx`: number
    pub _evx: f64,
    /// `_evy`: number
    pub _evy: f64,
    /// `_yog`: number
    pub _yog: f64,
    /// `_D0`: number
    pub _D0: f64,
    /// `_Dl`: number
    pub _Dl: f64,
    /// `_enOnce`: number
    pub _enOnce: f64,
    /// `ca`: number
    pub ca: f64,
    /// `_don`: number
    pub _don: f64,
    /// `drive`: boolean
    pub drive: bool,
    /// `_dke`: number
    pub _dke: f64,
    /// `past`: dizi
    pub past: Vec<JPSavurO_pwEPastE>,
    /// `holdD`: boolean
    pub holdD: bool,
    /// `rslot`: object
    pub rslot: JPSavurO_pwERslot,
    /// `slot0`: object
    pub slot0: JPSavurO_pwESlot0,
    /// `rslot0`: object
    pub rslot0: JPSavurO_pwERslot0,
    /// `rh`: string
    pub rh: String,
    /// `rs`: string
    pub rs: String,
    /// `tl`: object
    pub tl: JPSavurO_pwETl,
    /// `markRef`: OYUNCU
    pub markRef: Pid,
    /// `eg`: object
    pub eg: JPSavurO_pwEEg,
    /// `oty`: number
    pub oty: f64,
    /// `_otH`: OYUNCU|null
    pub _otH: Option<Pid>,
    /// `_otB`: null
    pub _otB: Option<JsDeger>,
    /// `_otE`: object
    pub _otE: JPSavurO_pwE_otE,
    /// `_otF`: OYUNCU|null
    pub _otF: Option<Pid>,
    /// `_kv`: null|object
    pub _kv: Option<JPSavurO_pwE_kv>,
    /// `_amacJob`: null|string
    pub _amacJob: Option<String>,
    /// `_kt`: object
    pub _kt: JPSavurO_pwE_kt,
    /// `_hizF`: number
    pub _hizF: f64,
    /// `kuyuT`: number
    pub kuyuT: f64,
    /// `sonuk`: boolean
    pub sonuk: bool,
    /// `_kuyuCik`: boolean
    pub _kuyuCik: bool,
    /// `__g`: OYUNCU
    pub __g: Pid,
    /// `__yas`: number
    pub __yas: f64,
    /// `__e`: number
    pub __e: f64,
    /// `_o`: OYUNCU|object
    pub _o: Pid,
    /// `_plan`: null
    pub _plan: Option<JsDeger>,
    /// `_surK`: null|object
    pub _surK: Option<JPSavurO_pwE_surK>,
    /// `_sprIst`: boolean
    pub _sprIst: bool,
    /// `sarj`: boolean
    pub sarj: bool,
    /// `savur`: null
    pub savur: Option<JsDeger>,
    /// `lastWhy`: string
    pub lastWhy: String,
    /// `_yer`: boolean
    pub _yer: bool,
    /// `amA`: boolean
    pub amA: bool,
    /// `am`: number
    pub am: f64,
    /// `_rcvV`: null
    pub _rcvV: Option<JsDeger>,
    /// `_rcvB`: TOPKOPYA|object
    pub _rcvB: TopRef,
    /// `_rcvAbs`: number
    pub _rcvAbs: f64,
    /// `_rcvX`: number
    pub _rcvX: f64,
    /// `_rcvY`: number
    pub _rcvY: f64,
    /// `chB`: TOPKOPYA|object
    pub chB: TopRef,
    /// `_preB`: TOPKOPYA|object
    pub _preB: TopRef,
    /// `_preT`: number
    pub _preT: f64,
    /// `_otA`: object
    pub _otA: JPSavurO_pwE_otA,
    /// `_onH`: null
    pub _onH: Option<JsDeger>,
    /// `_lc`: null
    pub _lc: Option<JsDeger>,
    /// `_donDir`: number
    pub _donDir: f64,
    /// `_donT`: number
    pub _donT: f64,
    /// `_savT`: number
    pub _savT: f64,
    /// `_savD`: number
    pub _savD: f64,
    /// `_blokT`: number
    pub _blokT: f64,
    /// `_savCar`: number
    pub _savCar: f64,
    /// `lockAt`: number
    pub lockAt: f64,
    /// `_lockH`: OYUNCU
    pub _lockH: Pid,
    /// `_presT`: number
    pub _presT: f64,
    /// `_presH`: OYUNCU
    pub _presH: Pid,
    /// `_kh`: object
    pub _kh: JPSavurO_pwE_kh,
    /// `_khT`: number
    pub _khT: f64,
    /// `_khQ`: OYUNCU
    pub _khQ: Pid,
    /// `_onK`: object
    pub _onK: JPSavurO_pwE_onK,
    /// `_kal`: boolean
    pub _kal: bool,
    /// `_kalH`: OYUNCU
    pub _kalH: Pid,
    /// `hamle`: null
    pub hamle: Option<JsDeger>,
    /// `_dolYok`: number
    pub _dolYok: f64,
    /// `adandi`: number
    pub adandi: f64,
    /// `adT`: object
    pub adT: JPSavurO_pwEAdT,
    /// `son`: object
    pub son: JPSavurO_pwESon,
    /// `adNeden`: string
    pub adNeden: String,
    /// `den`: number
    pub den: f64,
    /// `_omEv`: number
    pub _omEv: f64,
    /// `_omuzT`: number
    pub _omuzT: f64,
    /// `_sf`: null|object
    pub _sf: Option<JPSavurO_pwE_sf>,
    /// `fiskeCd`: number
    pub fiskeCd: f64,
    /// `fiske`: number
    pub fiske: f64,
    /// `fiskeDir`: number
    pub fiskeDir: f64,
    /// `_omuz`: object
    pub _omuz: JPSavurO_pwE_omuz,
    /// `sendT`: number
    pub sendT: f64,
    /// `sendCd`: number
    pub sendCd: f64,
    /// `stun`: number
    pub stun: f64,
    /// `okudu`: number
    pub okudu: f64,
    /// `_dalis`: number
    pub _dalis: f64,
}

/// JS `p.savur.o.lead`
#[derive(Clone, Debug, Default)]
pub struct JPSavurOLead {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `p.savur.o.lf0`
#[derive(Clone, Debug, Default)]
pub struct JPSavurOLf0 {
    /// `vx`: number
    pub vx: f64,
    /// `vy`: number
    pub vy: f64,
    /// `ok`: boolean
    pub ok: bool,
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
    /// `fren`: boolean
    pub fren: bool,
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

/// JS `m.tac0[].durus.huc[][]`
#[derive(Clone, Debug, Default)]
pub struct JMTac0EDurusHucEE {
}

/// JS `m.tac0[].durus.sav[][]`
#[derive(Clone, Debug, Default)]
pub struct JMTac0EDurusSavEE {
}

/// JS `m.pre.r.best`
#[derive(Clone, Debug, Default)]
pub struct JMPreRBest {
    /// `kind`: string
    pub kind: String,
    /// `to`: object
    pub to: JMPreRBestTo,
    /// `dir`: object
    pub dir: JMPreRBestDir,
    /// `_own`: number
    pub _own: f64,
    /// `uzun`: undef
    pub uzun: Option<JsDeger>,
    /// `duvar`: boolean|undef
    pub duvar: Option<bool>,
    /// `v`: number
    pub v: f64,
    /// `dok`: object
    pub dok: JMPreRBestDok,
    /// `det`: object
    pub det: JMPreRBestDet,
    /// `en`: object
    pub en: JMPreRBestEn,
    /// `enV`: number
    pub enV: f64,
    /// `prensip`: string
    pub prensip: String,
    /// `_pw`: dizi
    pub _pw: Vec<JMPreRBest_pwE>,
    /// `_pT`: number
    pub _pT: f64,
    /// `sjK`: number
    pub sjK: f64,
    /// `toE`: object
    pub toE: JMPreRBestToE,
    /// `TsE`: number
    pub TsE: f64,
    /// `q`: OYUNCU
    pub q: Pid,
    /// `arrive`: number
    pub arrive: f64,
    /// `tek`: number
    pub tek: f64,
    /// `firm`: boolean
    pub firm: bool,
    /// `spr`: undef
    pub spr: Option<JsDeger>,
    /// `itK`: undef
    pub itK: Option<JsDeger>,
    /// `siddet`: number
    pub siddet: f64,
    /// `lf0`: object
    pub lf0: JMPreRBestLf0,
    /// `launch`: object
    pub launch: JMPreRBestLaunch,
    /// `sYon`: number
    pub sYon: f64,
    /// `savT`: number
    pub savT: f64,
    /// `_sek`: number
    pub _sek: f64,
    /// `tf`: number
    pub tf: f64,
}

/// JS `m.pre.r.opts[]`
#[derive(Clone, Debug, Default)]
pub struct JMPreROptsE {
    /// `kind`: string
    pub kind: String,
    /// `to`: object
    pub to: JMPreROptsETo,
    /// `dir`: object
    pub dir: JMPreROptsEDir,
    /// `_own`: number
    pub _own: f64,
    /// `uzun`: number|undef
    pub uzun: Option<f64>,
    /// `duvar`: boolean|undef
    pub duvar: Option<bool>,
    /// `v`: number
    pub v: f64,
    /// `dok`: object
    pub dok: JMPreROptsEDok,
    /// `det`: object
    pub det: JMPreROptsEDet,
    /// `en`: object
    pub en: JMPreROptsEEn,
    /// `enV`: number
    pub enV: f64,
    /// `prensip`: string
    pub prensip: String,
    /// `_pw`: dizi
    pub _pw: Vec<JMPreROptsE_pwE>,
    /// `_pT`: number
    pub _pT: f64,
    /// `spr`: number|undef
    pub spr: Option<f64>,
    /// `sprBos`: number
    pub sprBos: f64,
    /// `sjK`: number
    pub sjK: f64,
    /// `q`: OYUNCU
    pub q: Pid,
    /// `arrive`: number
    pub arrive: f64,
    /// `tek`: number
    pub tek: f64,
    /// `firm`: boolean|undef
    pub firm: Option<bool>,
    /// `itK`: undef
    pub itK: Option<JsDeger>,
    /// `launch`: object
    pub launch: JMPreROptsELaunch,
    /// `sYon`: number
    pub sYon: f64,
    /// `savT`: number
    pub savT: f64,
    /// `_sek`: number
    pub _sek: f64,
    /// `tf`: number
    pub tf: f64,
    /// `lf0`: object
    pub lf0: JMPreROptsELf0,
    /// `toE`: object
    pub toE: JMPreROptsEToE,
    /// `TsE`: number
    pub TsE: f64,
    /// `siddet`: number
    pub siddet: f64,
    /// `hold`: number
    pub hold: f64,
    /// `shot`: string
    pub shot: String,
}

/// JS `m.ctl.r.best`
#[derive(Clone, Debug, Default)]
pub struct JMCtlRBest {
    /// `kind`: string
    pub kind: String,
    /// `to`: object
    pub to: JMCtlRBestTo,
    /// `dir`: object
    pub dir: JMCtlRBestDir,
    /// `_own`: number
    pub _own: f64,
    /// `uzun`: undef
    pub uzun: Option<JsDeger>,
    /// `duvar`: undef
    pub duvar: Option<JsDeger>,
    /// `toE`: object
    pub toE: JMCtlRBestToE,
    /// `TsE`: number
    pub TsE: f64,
    /// `v`: number
    pub v: f64,
    /// `dok`: object
    pub dok: JMCtlRBestDok,
    /// `det`: object
    pub det: JMCtlRBestDet,
    /// `en`: object
    pub en: JMCtlRBestEn,
    /// `enV`: number
    pub enV: f64,
    /// `prensip`: string
    pub prensip: String,
    /// `_pw`: dizi
    pub _pw: Vec<JMCtlRBest_pwE>,
    /// `_pT`: number
    pub _pT: f64,
    /// `sjK`: number
    pub sjK: f64,
    /// `spr`: number|undef
    pub spr: Option<f64>,
    /// `q`: OYUNCU
    pub q: Pid,
    /// `arrive`: number
    pub arrive: f64,
    /// `tek`: number
    pub tek: f64,
    /// `firm`: boolean
    pub firm: bool,
    /// `itK`: undef
    pub itK: Option<JsDeger>,
    /// `siddet`: number
    pub siddet: f64,
    /// `lf0`: object
    pub lf0: JMCtlRBestLf0,
    /// `launch`: object
    pub launch: JMCtlRBestLaunch,
    /// `sYon`: number
    pub sYon: f64,
    /// `savT`: number
    pub savT: f64,
    /// `_sek`: number
    pub _sek: f64,
    /// `tf`: number
    pub tf: f64,
}

/// JS `m.ctl.r.opts[]`
#[derive(Clone, Debug, Default)]
pub struct JMCtlROptsE {
    /// `kind`: string
    pub kind: String,
    /// `to`: object
    pub to: JMCtlROptsETo,
    /// `dir`: object
    pub dir: JMCtlROptsEDir,
    /// `_own`: number
    pub _own: f64,
    /// `uzun`: undef
    pub uzun: Option<JsDeger>,
    /// `duvar`: undef
    pub duvar: Option<JsDeger>,
    /// `toE`: object
    pub toE: JMCtlROptsEToE,
    /// `TsE`: number
    pub TsE: f64,
    /// `v`: number
    pub v: f64,
    /// `dok`: object
    pub dok: JMCtlROptsEDok,
    /// `det`: object
    pub det: JMCtlROptsEDet,
    /// `en`: object
    pub en: JMCtlROptsEEn,
    /// `enV`: number
    pub enV: f64,
    /// `prensip`: string
    pub prensip: String,
    /// `_pw`: dizi
    pub _pw: Vec<JMCtlROptsE_pwE>,
    /// `_pT`: number
    pub _pT: f64,
    /// `spr`: number|undef
    pub spr: Option<f64>,
    /// `sprBos`: number
    pub sprBos: f64,
    /// `sjK`: number
    pub sjK: f64,
    /// `q`: OYUNCU
    pub q: Pid,
    /// `arrive`: number
    pub arrive: f64,
    /// `tek`: number
    pub tek: f64,
    /// `firm`: boolean|undef
    pub firm: Option<bool>,
    /// `itK`: undef
    pub itK: Option<JsDeger>,
    /// `launch`: object
    pub launch: JMCtlROptsELaunch,
    /// `sYon`: number
    pub sYon: f64,
    /// `savT`: number
    pub savT: f64,
    /// `_sek`: number
    pub _sek: f64,
    /// `tf`: number
    pub tf: f64,
    /// `lf0`: object
    pub lf0: JMCtlROptsELf0,
    /// `siddet`: number
    pub siddet: f64,
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

/// JS `m.lastDec.opts[].dok`
#[derive(Clone, Debug, Default)]
pub struct JMLastDecOptsEDok {
    /// `tutar`: number
    pub tutar: f64,
    /// `alici`: number
    pub alici: f64,
    /// `durum`: number
    pub durum: f64,
    /// `sekme`: number
    pub sekme: f64,
    /// `kayip`: number
    pub kayip: f64,
    /// `sure`: number
    pub sure: f64,
    /// `sut`: number
    pub sut: f64,
    /// `ileri`: number
    pub ileri: f64,
    /// `surme`: number
    pub surme: f64,
    /// `zemin`: number
    pub zemin: f64,
    /// `simdi`: number
    pub simdi: f64,
    /// `sonra`: string
    pub sonra: String,
    /// `gol`: number
    pub gol: f64,
}

/// JS `o._pw[].tl.huc`
#[derive(Clone, Debug, Default)]
pub struct JO_pwETlHuc {
    /// `kosu`: number
    pub kosu: f64,
    /// `istek`: number
    pub istek: f64,
    /// `destek`: number
    pub destek: f64,
    /// `risk`: number
    pub risk: f64,
    /// `ritim`: number
    pub ritim: f64,
    /// `tempo`: number
    pub tempo: f64,
    /// `gonder`: number
    pub gonder: f64,
    /// `serbest`: number
    pub serbest: f64,
}

/// JS `o._pw[].tl.sav`
#[derive(Clone, Debug, Default)]
pub struct JO_pwETlSav {
    /// `adam`: number
    pub adam: f64,
    /// `pres`: number
    pub pres: f64,
}

/// JS `p.savur.o.det.end`
#[derive(Clone, Debug, Default)]
pub struct JPSavurODetEnd {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `p.savur.o.det.endM`
#[derive(Clone, Debug, Default)]
pub struct JPSavurODetEndM {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `p.savur.o.det.endO`
#[derive(Clone, Debug, Default)]
pub struct JPSavurODetEndO {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `p.savur.o.det.endS`
#[derive(Clone, Debug, Default)]
pub struct JPSavurODetEndS {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `p.savur.o.det.endD`
#[derive(Clone, Debug, Default)]
pub struct JPSavurODetEndD {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `p.savur.o.det.endSek`
#[derive(Clone, Debug, Default)]
pub struct JPSavurODetEndSek {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `p.savur.o._pw[].slot`
#[derive(Clone, Debug, Default)]
pub struct JPSavurO_pwESlot {
    /// `role`: string
    pub role: String,
    /// `d`: number
    pub d: f64,
    /// `l`: number
    pub l: f64,
}

/// JS `p.savur.o._pw[].a`
#[derive(Clone, Debug, Default)]
pub struct JPSavurO_pwEA {
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

/// JS `p.savur.o._pw[].driveTo`
#[derive(Clone, Debug, Default)]
pub struct JPSavurO_pwEDriveTo {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `p.savur.o._pw[].hist[]`
#[derive(Clone, Debug, Default)]
pub struct JPSavurO_pwEHistE {
}

/// JS `p.savur.o._pw[]._bk[]`
#[derive(Clone, Debug, Default)]
pub struct JPSavurO_pwE_bkE {
}

/// JS `p.savur.o._pw[]._ktB`
#[derive(Clone, Debug, Default)]
pub struct JPSavurO_pwE_ktB {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `p.savur.o._pw[].past[]`
#[derive(Clone, Debug, Default)]
pub struct JPSavurO_pwEPastE {
}

/// JS `p.savur.o._pw[].rslot`
#[derive(Clone, Debug, Default)]
pub struct JPSavurO_pwERslot {
    /// `d`: number
    pub d: f64,
    /// `l`: number
    pub l: f64,
}

/// JS `p.savur.o._pw[].slot0`
#[derive(Clone, Debug, Default)]
pub struct JPSavurO_pwESlot0 {
    /// `role`: string
    pub role: String,
    /// `d`: number
    pub d: f64,
    /// `l`: number
    pub l: f64,
}

/// JS `p.savur.o._pw[].rslot0`
#[derive(Clone, Debug, Default)]
pub struct JPSavurO_pwERslot0 {
    /// `d`: number
    pub d: f64,
    /// `l`: number
    pub l: f64,
}

/// JS `p.savur.o._pw[].tl`
#[derive(Clone, Debug, Default)]
pub struct JPSavurO_pwETl {
    /// `huc`: object
    pub huc: JPSavurO_pwETlHuc,
    /// `sav`: object
    pub sav: JPSavurO_pwETlSav,
}

/// JS `p.savur.o._pw[].eg`
#[derive(Clone, Debug, Default)]
pub struct JPSavurO_pwEEg {
    /// `ps`: number
    pub ps: f64,
    /// `rk`: number
    pub rk: f64,
}

/// JS `p.savur.o._pw[]._otE`
#[derive(Clone, Debug, Default)]
pub struct JPSavurO_pwE_otE {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `p.savur.o._pw[]._kv`
#[derive(Clone, Debug, Default)]
pub struct JPSavurO_pwE_kv {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
    /// `t`: number
    pub t: f64,
}

/// JS `p.savur.o._pw[]._kt`
#[derive(Clone, Debug, Default)]
pub struct JPSavurO_pwE_kt {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `p.savur.o._pw[]._surK`
#[derive(Clone, Debug, Default)]
pub struct JPSavurO_pwE_surK {
    /// `id`: number
    pub id: f64,
    /// `to`: object
    pub to: JPSavurO_pwE_surKTo,
    /// `t`: number
    pub t: f64,
    /// `T`: number
    pub T: f64,
    /// `nd0`: number
    pub nd0: f64,
}

/// JS `p.savur.o._pw[]._otA`
#[derive(Clone, Debug, Default)]
pub struct JPSavurO_pwE_otA {
    /// `b`: TOPKOPYA|object
    pub b: TopRef,
    /// `ok`: boolean
    pub ok: bool,
}

/// JS `p.savur.o._pw[]._kh`
#[derive(Clone, Debug, Default)]
pub struct JPSavurO_pwE_kh {
    /// `dx`: number
    pub dx: f64,
    /// `dy`: number
    pub dy: f64,
}

/// JS `p.savur.o._pw[]._onK`
#[derive(Clone, Debug, Default)]
pub struct JPSavurO_pwE_onK {
    /// `h`: number
    pub h: f64,
    /// `koy`: boolean
    pub koy: bool,
}

/// JS `p.savur.o._pw[].adT`
#[derive(Clone, Debug, Default)]
pub struct JPSavurO_pwEAdT {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `p.savur.o._pw[].son`
#[derive(Clone, Debug, Default)]
pub struct JPSavurO_pwESon {
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

/// JS `p.savur.o._pw[]._sf`
#[derive(Clone, Debug, Default)]
pub struct JPSavurO_pwE_sf {
    /// `t0`: number
    pub t0: f64,
    /// `f`: string
    pub f: String,
}

/// JS `p.savur.o._pw[]._omuz`
#[derive(Clone, Debug, Default)]
pub struct JPSavurO_pwE_omuz {
    /// `t`: number
    pub t: f64,
    /// `q`: number
    pub q: f64,
}

/// JS `m.pre.r.best.to`
#[derive(Clone, Debug, Default)]
pub struct JMPreRBestTo {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `m.pre.r.best.dir`
#[derive(Clone, Debug, Default)]
pub struct JMPreRBestDir {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `m.pre.r.best.dok`
#[derive(Clone, Debug, Default)]
pub struct JMPreRBestDok {
    /// `tutar`: number
    pub tutar: f64,
    /// `durum`: number
    pub durum: f64,
    /// `kayip`: number
    pub kayip: f64,
    /// `sure`: number
    pub sure: f64,
    /// `sut`: number
    pub sut: f64,
    /// `ileri`: number
    pub ileri: f64,
    /// `surme`: number
    pub surme: f64,
    /// `zemin`: number
    pub zemin: f64,
    /// `sonra`: string
    pub sonra: String,
    /// `simdi`: number
    pub simdi: f64,
    /// `alici`: number
    pub alici: f64,
    /// `sekme`: number
    pub sekme: f64,
}

/// JS `m.pre.r.best.det`
#[derive(Clone, Debug, Default)]
pub struct JMPreRBestDet {
    /// `P`: number
    pub P: f64,
    /// `ch`: number
    pub ch: f64,
    /// `Pk`: number
    pub Pk: f64,
    /// `Pl`: number
    pub Pl: f64,
    /// `end`: object
    pub end: JMPreRBestDetEnd,
    /// `endM`: object
    pub endM: JMPreRBestDetEndM,
    /// `endO`: object
    pub endO: JMPreRBestDetEndO,
    /// `Ps`: number
    pub Ps: f64,
    /// `endS`: object
    pub endS: JMPreRBestDetEndS,
    /// `endD`: object
    pub endD: JMPreRBestDetEndD,
    /// `Psek`: number
    pub Psek: f64,
    /// `endSek`: object
    pub endSek: JMPreRBestDetEndSek,
}

/// JS `m.pre.r.best.en`
#[derive(Clone, Debug, Default)]
pub struct JMPreRBestEn {
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

/// JS `m.pre.r.best._pw[]`
#[derive(Clone, Debug, Default)]
pub struct JMPreRBest_pwE {
    /// `id`: number
    pub id: f64,
    /// `team`: number
    pub team: f64,
    /// `i`: number
    pub i: f64,
    /// `role`: string
    pub role: String,
    /// `slot`: object
    pub slot: JMPreRBest_pwESlot,
    /// `name`: string
    pub name: String,
    /// `a`: object
    pub a: JMPreRBest_pwEA,
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
    /// `en`: number
    pub en: f64,
    /// `driveTo`: null|object
    pub driveTo: Option<JMPreRBest_pwEDriveTo>,
    /// `_dur`: null|number
    pub _dur: Option<f64>,
    /// `_korT`: number
    pub _korT: f64,
    /// `bekBas`: null
    pub bekBas: Option<JsDeger>,
    /// `noTouch`: boolean
    pub noTouch: bool,
    /// `hist`: dizi
    pub hist: Vec<JsDeger>,
    /// `lock`: number
    pub lock: f64,
    /// `kapPrev`: null
    pub kapPrev: Option<JsDeger>,
    /// `zonePrev`: OYUNCU|null
    pub zonePrev: Option<Pid>,
    /// `swPrev`: OYUNCU|null
    pub swPrev: Option<Pid>,
    /// `uaPrev`: OYUNCU|null
    pub uaPrev: Option<Pid>,
    /// `rec`: number
    pub rec: f64,
    /// `tutQ`: OYUNCU|null
    pub tutQ: Option<Pid>,
    /// `sprint`: boolean
    pub sprint: bool,
    /// `press`: boolean
    pub press: bool,
    /// `job`: string
    pub job: String,
    /// `otx`: null|number
    pub otx: Option<f64>,
    /// `_bk`: dizi
    pub _bk: Vec<JsDeger>,
    /// `_ktB`: object
    pub _ktB: JMPreRBest_pwE_ktB,
    /// `_topSpr`: boolean
    pub _topSpr: bool,
    /// `_kosuK`: null|string
    pub _kosuK: Option<String>,
    /// `_hizK`: null|number
    pub _hizK: Option<f64>,
    /// `_evx`: number
    pub _evx: f64,
    /// `_evy`: number
    pub _evy: f64,
    /// `_yog`: number
    pub _yog: f64,
    /// `_D0`: number
    pub _D0: f64,
    /// `_Dl`: number
    pub _Dl: f64,
    /// `_enOnce`: number
    pub _enOnce: f64,
    /// `ca`: number
    pub ca: f64,
    /// `_don`: number
    pub _don: f64,
    /// `drive`: boolean
    pub drive: bool,
    /// `_dke`: number
    pub _dke: f64,
    /// `past`: dizi
    pub past: Vec<JsDeger>,
    /// `holdD`: boolean
    pub holdD: bool,
    /// `_plan`: null
    pub _plan: Option<JsDeger>,
    /// `_surK`: null
    pub _surK: Option<JsDeger>,
    /// `_sprIst`: boolean
    pub _sprIst: bool,
    /// `sarj`: boolean
    pub sarj: bool,
    /// `savur`: null
    pub savur: Option<JsDeger>,
    /// `lastWhy`: string
    pub lastWhy: String,
    /// `_yer`: boolean
    pub _yer: bool,
    /// `amA`: boolean
    pub amA: bool,
    /// `am`: number
    pub am: f64,
    /// `__g`: OYUNCU
    pub __g: Pid,
    /// `__yas`: number
    pub __yas: f64,
    /// `__e`: number
    pub __e: f64,
    /// `rslot`: object
    pub rslot: JMPreRBest_pwERslot,
    /// `slot0`: object
    pub slot0: JMPreRBest_pwESlot0,
    /// `rslot0`: object
    pub rslot0: JMPreRBest_pwERslot0,
    /// `rh`: string
    pub rh: String,
    /// `rs`: string
    pub rs: String,
    /// `tl`: object
    pub tl: JMPreRBest_pwETl,
    /// `markRef`: OYUNCU
    pub markRef: Pid,
    /// `eg`: object
    pub eg: JMPreRBest_pwEEg,
    /// `oty`: number
    pub oty: f64,
    /// `_otH`: OYUNCU|null
    pub _otH: Option<Pid>,
    /// `_otB`: null
    pub _otB: Option<JsDeger>,
    /// `_otE`: object
    pub _otE: JMPreRBest_pwE_otE,
    /// `_otF`: OYUNCU|null
    pub _otF: Option<Pid>,
    /// `_kv`: null
    pub _kv: Option<JsDeger>,
    /// `_amacJob`: null|string
    pub _amacJob: Option<String>,
    /// `_kt`: object
    pub _kt: JMPreRBest_pwE_kt,
    /// `_hizF`: number
    pub _hizF: f64,
    /// `kuyuT`: number
    pub kuyuT: f64,
    /// `sonuk`: boolean
    pub sonuk: bool,
    /// `_kuyuCik`: boolean
    pub _kuyuCik: bool,
    /// `_o`: object
    pub _o: JMPreRBest_pwE_o,
    /// `_rcvV`: null|number
    pub _rcvV: Option<f64>,
    /// `_rcvB`: TOP|TOPKOPYA|object
    pub _rcvB: TopRef,
    /// `_rcvAbs`: number
    pub _rcvAbs: f64,
    /// `_rcvX`: number
    pub _rcvX: f64,
    /// `_rcvY`: number
    pub _rcvY: f64,
    /// `chB`: TOP|TOPKOPYA|object
    pub chB: TopRef,
    /// `_preB`: TOP|TOPKOPYA|object
    pub _preB: TopRef,
    /// `_preT`: number
    pub _preT: f64,
    /// `_otA`: object
    pub _otA: JMPreRBest_pwE_otA,
    /// `_onH`: null
    pub _onH: Option<JsDeger>,
    /// `_lc`: null
    pub _lc: Option<JsDeger>,
    /// `_donDir`: number
    pub _donDir: f64,
    /// `_donT`: number
    pub _donT: f64,
    /// `_savT`: number
    pub _savT: f64,
    /// `_savD`: number
    pub _savD: f64,
    /// `_blokT`: number
    pub _blokT: f64,
    /// `_omuzT`: number
    pub _omuzT: f64,
    /// `_omEv`: number
    pub _omEv: f64,
    /// `_savCar`: number
    pub _savCar: f64,
    /// `lockAt`: number
    pub lockAt: f64,
    /// `_lockH`: OYUNCU
    pub _lockH: Pid,
    /// `_presT`: number
    pub _presT: f64,
    /// `_presH`: OYUNCU
    pub _presH: Pid,
    /// `_kh`: object
    pub _kh: JMPreRBest_pwE_kh,
    /// `_khT`: number
    pub _khT: f64,
    /// `_khQ`: OYUNCU
    pub _khQ: Pid,
    /// `_onK`: object
    pub _onK: JMPreRBest_pwE_onK,
    /// `_kal`: boolean
    pub _kal: bool,
    /// `_kalH`: OYUNCU
    pub _kalH: Pid,
    /// `hamle`: null
    pub hamle: Option<JsDeger>,
    /// `adandi`: number
    pub adandi: f64,
    /// `adT`: object
    pub adT: JMPreRBest_pwEAdT,
    /// `_dolYok`: number
    pub _dolYok: f64,
    /// `son`: object
    pub son: JMPreRBest_pwESon,
    /// `adNeden`: string
    pub adNeden: String,
    /// `den`: number
    pub den: f64,
    /// `_sf`: null
    pub _sf: Option<JsDeger>,
    /// `_bosT`: number
    pub _bosT: f64,
    /// `_gvKaz`: object
    pub _gvKaz: JMPreRBest_pwE_gvKaz,
    /// `pplan`: object
    pub pplan: JMPreRBest_pwEPplan,
    /// `_kAn`: number
    pub _kAn: f64,
    /// `_kAnT`: number
    pub _kAnT: f64,
    /// `fiskeCd`: number
    pub fiskeCd: f64,
    /// `fiske`: number
    pub fiske: f64,
    /// `fiskeDir`: number
    pub fiskeDir: f64,
    /// `okudu`: number
    pub okudu: f64,
    /// `_winSeen`: number
    pub _winSeen: f64,
    /// `_omuz`: object
    pub _omuz: JMPreRBest_pwE_omuz,
    /// `sendT`: number
    pub sendT: f64,
    /// `sendCd`: number
    pub sendCd: f64,
    /// `stun`: number
    pub stun: f64,
    /// `_dalis`: number
    pub _dalis: f64,
}

/// JS `m.pre.r.best.toE`
#[derive(Clone, Debug, Default)]
pub struct JMPreRBestToE {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `m.pre.r.best.lf0`
#[derive(Clone, Debug, Default)]
pub struct JMPreRBestLf0 {
    /// `vx`: number
    pub vx: f64,
    /// `vy`: number
    pub vy: f64,
    /// `ok`: boolean
    pub ok: bool,
}

/// JS `m.pre.r.best.launch`
#[derive(Clone, Debug, Default)]
pub struct JMPreRBestLaunch {
    /// `vx`: number
    pub vx: f64,
    /// `vy`: number
    pub vy: f64,
    /// `ok`: boolean
    pub ok: bool,
}

/// JS `m.pre.r.opts[].to`
#[derive(Clone, Debug, Default)]
pub struct JMPreROptsETo {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `m.pre.r.opts[].dir`
#[derive(Clone, Debug, Default)]
pub struct JMPreROptsEDir {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `m.pre.r.opts[].dok`
#[derive(Clone, Debug, Default)]
pub struct JMPreROptsEDok {
    /// `tutar`: number
    pub tutar: f64,
    /// `durum`: number
    pub durum: f64,
    /// `kayip`: number
    pub kayip: f64,
    /// `sure`: number
    pub sure: f64,
    /// `sut`: number
    pub sut: f64,
    /// `ileri`: number
    pub ileri: f64,
    /// `surme`: number
    pub surme: f64,
    /// `zemin`: number
    pub zemin: f64,
    /// `sonra`: string
    pub sonra: String,
    /// `simdi`: number
    pub simdi: f64,
    /// `alici`: number
    pub alici: f64,
    /// `sekme`: number
    pub sekme: f64,
}

/// JS `m.pre.r.opts[].det`
#[derive(Clone, Debug, Default)]
pub struct JMPreROptsEDet {
    /// `P`: number
    pub P: f64,
    /// `ch`: number
    pub ch: f64,
    /// `Pk`: number
    pub Pk: f64,
    /// `Pl`: number
    pub Pl: f64,
    /// `end`: object
    pub end: JMPreROptsEDetEnd,
    /// `endM`: object
    pub endM: JMPreROptsEDetEndM,
    /// `endO`: object
    pub endO: JMPreROptsEDetEndO,
    /// `Ps`: number
    pub Ps: f64,
    /// `endS`: object
    pub endS: JMPreROptsEDetEndS,
    /// `endD`: object
    pub endD: JMPreROptsEDetEndD,
    /// `Psek`: number
    pub Psek: f64,
    /// `endSek`: object
    pub endSek: JMPreROptsEDetEndSek,
}

/// JS `m.pre.r.opts[].en`
#[derive(Clone, Debug, Default)]
pub struct JMPreROptsEEn {
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
    pub per: Vec<(JsDeger, JsDeger)>,
    /// `ek`: number
    pub ek: f64,
}

/// JS `m.pre.r.opts[]._pw[]`
#[derive(Clone, Debug, Default)]
pub struct JMPreROptsE_pwE {
}

/// JS `m.pre.r.opts[].launch`
#[derive(Clone, Debug, Default)]
pub struct JMPreROptsELaunch {
    /// `vx`: number
    pub vx: f64,
    /// `vy`: number
    pub vy: f64,
    /// `ok`: boolean
    pub ok: bool,
}

/// JS `m.pre.r.opts[].lf0`
#[derive(Clone, Debug, Default)]
pub struct JMPreROptsELf0 {
    /// `vx`: number
    pub vx: f64,
    /// `vy`: number
    pub vy: f64,
    /// `ok`: boolean
    pub ok: bool,
}

/// JS `m.pre.r.opts[].toE`
#[derive(Clone, Debug, Default)]
pub struct JMPreROptsEToE {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `m.ctl.r.best.to`
#[derive(Clone, Debug, Default)]
pub struct JMCtlRBestTo {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `m.ctl.r.best.dir`
#[derive(Clone, Debug, Default)]
pub struct JMCtlRBestDir {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `m.ctl.r.best.toE`
#[derive(Clone, Debug, Default)]
pub struct JMCtlRBestToE {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `m.ctl.r.best.dok`
#[derive(Clone, Debug, Default)]
pub struct JMCtlRBestDok {
    /// `tutar`: number
    pub tutar: f64,
    /// `durum`: number
    pub durum: f64,
    /// `kayip`: number
    pub kayip: f64,
    /// `sure`: number
    pub sure: f64,
    /// `sut`: number
    pub sut: f64,
    /// `ileri`: number
    pub ileri: f64,
    /// `surme`: number
    pub surme: f64,
    /// `zemin`: number
    pub zemin: f64,
    /// `sonra`: string
    pub sonra: String,
    /// `simdi`: number
    pub simdi: f64,
    /// `alici`: number
    pub alici: f64,
    /// `sekme`: number
    pub sekme: f64,
}

/// JS `m.ctl.r.best.det`
#[derive(Clone, Debug, Default)]
pub struct JMCtlRBestDet {
    /// `P`: number
    pub P: f64,
    /// `ch`: number
    pub ch: f64,
    /// `Pk`: number
    pub Pk: f64,
    /// `Pl`: number
    pub Pl: f64,
    /// `end`: object
    pub end: JMCtlRBestDetEnd,
    /// `endM`: object
    pub endM: JMCtlRBestDetEndM,
    /// `endO`: object
    pub endO: JMCtlRBestDetEndO,
    /// `Ps`: number
    pub Ps: f64,
    /// `endS`: object
    pub endS: JMCtlRBestDetEndS,
    /// `endD`: object
    pub endD: JMCtlRBestDetEndD,
    /// `Psek`: number
    pub Psek: f64,
    /// `endSek`: object
    pub endSek: JMCtlRBestDetEndSek,
}

/// JS `m.ctl.r.best.en`
#[derive(Clone, Debug, Default)]
pub struct JMCtlRBestEn {
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

/// JS `m.ctl.r.best._pw[]`
#[derive(Clone, Debug, Default)]
pub struct JMCtlRBest_pwE {
    /// `id`: number
    pub id: f64,
    /// `team`: number
    pub team: f64,
    /// `i`: number
    pub i: f64,
    /// `role`: string
    pub role: String,
    /// `slot`: object
    pub slot: JMCtlRBest_pwESlot,
    /// `name`: string
    pub name: String,
    /// `a`: object
    pub a: JMCtlRBest_pwEA,
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
    /// `en`: number
    pub en: f64,
    /// `driveTo`: null|object
    pub driveTo: Option<JMCtlRBest_pwEDriveTo>,
    /// `_dur`: null|number
    pub _dur: Option<f64>,
    /// `_korT`: number
    pub _korT: f64,
    /// `bekBas`: null
    pub bekBas: Option<JsDeger>,
    /// `noTouch`: boolean
    pub noTouch: bool,
    /// `hist`: dizi
    pub hist: Vec<JsDeger>,
    /// `lock`: number
    pub lock: f64,
    /// `kapPrev`: null
    pub kapPrev: Option<JsDeger>,
    /// `zonePrev`: OYUNCU|null
    pub zonePrev: Option<Pid>,
    /// `swPrev`: OYUNCU|null
    pub swPrev: Option<Pid>,
    /// `uaPrev`: OYUNCU|null
    pub uaPrev: Option<Pid>,
    /// `rec`: number
    pub rec: f64,
    /// `tutQ`: OYUNCU|null
    pub tutQ: Option<Pid>,
    /// `sprint`: boolean
    pub sprint: bool,
    /// `press`: boolean
    pub press: bool,
    /// `job`: string
    pub job: String,
    /// `otx`: null|number
    pub otx: Option<f64>,
    /// `_bk`: dizi
    pub _bk: Vec<JsDeger>,
    /// `_ktB`: object
    pub _ktB: JMCtlRBest_pwE_ktB,
    /// `_topSpr`: boolean
    pub _topSpr: bool,
    /// `_kosuK`: null|string
    pub _kosuK: Option<String>,
    /// `_hizK`: null|number
    pub _hizK: Option<f64>,
    /// `_evx`: number
    pub _evx: f64,
    /// `_evy`: number
    pub _evy: f64,
    /// `_yog`: number
    pub _yog: f64,
    /// `_D0`: number
    pub _D0: f64,
    /// `_Dl`: number
    pub _Dl: f64,
    /// `_enOnce`: number
    pub _enOnce: f64,
    /// `ca`: number
    pub ca: f64,
    /// `_don`: number
    pub _don: f64,
    /// `drive`: boolean
    pub drive: bool,
    /// `_dke`: number
    pub _dke: f64,
    /// `past`: dizi
    pub past: Vec<JsDeger>,
    /// `holdD`: boolean
    pub holdD: bool,
    /// `_plan`: null
    pub _plan: Option<JsDeger>,
    /// `_surK`: null
    pub _surK: Option<JsDeger>,
    /// `_sprIst`: boolean
    pub _sprIst: bool,
    /// `sarj`: boolean
    pub sarj: bool,
    /// `savur`: null
    pub savur: Option<JsDeger>,
    /// `lastWhy`: string
    pub lastWhy: String,
    /// `_yer`: boolean
    pub _yer: bool,
    /// `amA`: boolean
    pub amA: bool,
    /// `am`: number
    pub am: f64,
    /// `__g`: OYUNCU
    pub __g: Pid,
    /// `__yas`: number
    pub __yas: f64,
    /// `__e`: number
    pub __e: f64,
    /// `rslot`: object
    pub rslot: JMCtlRBest_pwERslot,
    /// `slot0`: object
    pub slot0: JMCtlRBest_pwESlot0,
    /// `rslot0`: object
    pub rslot0: JMCtlRBest_pwERslot0,
    /// `rh`: string
    pub rh: String,
    /// `rs`: string
    pub rs: String,
    /// `tl`: object
    pub tl: JMCtlRBest_pwETl,
    /// `markRef`: OYUNCU
    pub markRef: Pid,
    /// `eg`: object
    pub eg: JMCtlRBest_pwEEg,
    /// `oty`: number
    pub oty: f64,
    /// `_otH`: OYUNCU|null
    pub _otH: Option<Pid>,
    /// `_otB`: null
    pub _otB: Option<JsDeger>,
    /// `_otE`: object
    pub _otE: JMCtlRBest_pwE_otE,
    /// `_otF`: OYUNCU|null
    pub _otF: Option<Pid>,
    /// `_kv`: null
    pub _kv: Option<JsDeger>,
    /// `_amacJob`: null
    pub _amacJob: Option<JsDeger>,
    /// `_kt`: object
    pub _kt: JMCtlRBest_pwE_kt,
    /// `_hizF`: number
    pub _hizF: f64,
    /// `kuyuT`: number
    pub kuyuT: f64,
    /// `sonuk`: boolean
    pub sonuk: bool,
    /// `_kuyuCik`: boolean
    pub _kuyuCik: bool,
    /// `_rcvV`: null|number
    pub _rcvV: Option<f64>,
    /// `_rcvB`: TOPKOPYA|object
    pub _rcvB: TopRef,
    /// `_rcvAbs`: number
    pub _rcvAbs: f64,
    /// `_rcvX`: number
    pub _rcvX: f64,
    /// `_rcvY`: number
    pub _rcvY: f64,
    /// `chB`: TOPKOPYA|object
    pub chB: TopRef,
    /// `_preB`: TOPKOPYA|object
    pub _preB: TopRef,
    /// `_preT`: number
    pub _preT: f64,
    /// `_otA`: object
    pub _otA: JMCtlRBest_pwE_otA,
    /// `_onH`: null
    pub _onH: Option<JsDeger>,
    /// `_lc`: null
    pub _lc: Option<JsDeger>,
    /// `_o`: OYUNCU|object
    pub _o: Pid,
    /// `_donDir`: number
    pub _donDir: f64,
    /// `_donT`: number
    pub _donT: f64,
    /// `_savT`: number
    pub _savT: f64,
    /// `_savD`: number
    pub _savD: f64,
    /// `_blokT`: number
    pub _blokT: f64,
    /// `_omuzT`: number
    pub _omuzT: f64,
    /// `_omEv`: number
    pub _omEv: f64,
    /// `_savCar`: number
    pub _savCar: f64,
    /// `lockAt`: number
    pub lockAt: f64,
    /// `_lockH`: OYUNCU
    pub _lockH: Pid,
    /// `_presT`: number
    pub _presT: f64,
    /// `_presH`: OYUNCU
    pub _presH: Pid,
    /// `_kh`: object
    pub _kh: JMCtlRBest_pwE_kh,
    /// `_khT`: number
    pub _khT: f64,
    /// `_khQ`: OYUNCU
    pub _khQ: Pid,
    /// `_onK`: object
    pub _onK: JMCtlRBest_pwE_onK,
    /// `_kal`: boolean
    pub _kal: bool,
    /// `_kalH`: OYUNCU
    pub _kalH: Pid,
    /// `hamle`: null
    pub hamle: Option<JsDeger>,
    /// `_dolYok`: number
    pub _dolYok: f64,
    /// `adandi`: number
    pub adandi: f64,
    /// `adT`: object
    pub adT: JMCtlRBest_pwEAdT,
    /// `son`: object
    pub son: JMCtlRBest_pwESon,
    /// `adNeden`: string
    pub adNeden: String,
    /// `den`: number
    pub den: f64,
    /// `_sf`: null
    pub _sf: Option<JsDeger>,
    /// `_bosT`: number
    pub _bosT: f64,
    /// `_gvKaz`: object
    pub _gvKaz: JMCtlRBest_pwE_gvKaz,
    /// `pplan`: object
    pub pplan: JMCtlRBest_pwEPplan,
    /// `_kAn`: number
    pub _kAn: f64,
    /// `_kAnT`: number
    pub _kAnT: f64,
    /// `fiskeCd`: number
    pub fiskeCd: f64,
    /// `fiske`: number
    pub fiske: f64,
    /// `fiskeDir`: number
    pub fiskeDir: f64,
    /// `okudu`: number
    pub okudu: f64,
    /// `_winSeen`: number
    pub _winSeen: f64,
}

/// JS `m.ctl.r.best.lf0`
#[derive(Clone, Debug, Default)]
pub struct JMCtlRBestLf0 {
    /// `vx`: number
    pub vx: f64,
    /// `vy`: number
    pub vy: f64,
    /// `ok`: boolean
    pub ok: bool,
}

/// JS `m.ctl.r.best.launch`
#[derive(Clone, Debug, Default)]
pub struct JMCtlRBestLaunch {
    /// `vx`: number
    pub vx: f64,
    /// `vy`: number
    pub vy: f64,
    /// `ok`: boolean
    pub ok: bool,
}

/// JS `m.ctl.r.opts[].to`
#[derive(Clone, Debug, Default)]
pub struct JMCtlROptsETo {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `m.ctl.r.opts[].dir`
#[derive(Clone, Debug, Default)]
pub struct JMCtlROptsEDir {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `m.ctl.r.opts[].toE`
#[derive(Clone, Debug, Default)]
pub struct JMCtlROptsEToE {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `m.ctl.r.opts[].dok`
#[derive(Clone, Debug, Default)]
pub struct JMCtlROptsEDok {
    /// `tutar`: number
    pub tutar: f64,
    /// `durum`: number
    pub durum: f64,
    /// `kayip`: number
    pub kayip: f64,
    /// `sure`: number
    pub sure: f64,
    /// `sut`: number
    pub sut: f64,
    /// `ileri`: number
    pub ileri: f64,
    /// `surme`: number
    pub surme: f64,
    /// `zemin`: number
    pub zemin: f64,
    /// `sonra`: string
    pub sonra: String,
    /// `simdi`: number
    pub simdi: f64,
    /// `alici`: number
    pub alici: f64,
    /// `sekme`: number
    pub sekme: f64,
}

/// JS `m.ctl.r.opts[].det`
#[derive(Clone, Debug, Default)]
pub struct JMCtlROptsEDet {
    /// `P`: number
    pub P: f64,
    /// `ch`: number
    pub ch: f64,
    /// `Pk`: number
    pub Pk: f64,
    /// `Pl`: number
    pub Pl: f64,
    /// `end`: object
    pub end: JMCtlROptsEDetEnd,
    /// `endM`: object
    pub endM: JMCtlROptsEDetEndM,
    /// `endO`: object
    pub endO: JMCtlROptsEDetEndO,
    /// `Ps`: number
    pub Ps: f64,
    /// `endS`: object
    pub endS: JMCtlROptsEDetEndS,
    /// `endD`: object
    pub endD: JMCtlROptsEDetEndD,
    /// `Psek`: number
    pub Psek: f64,
    /// `endSek`: object
    pub endSek: JMCtlROptsEDetEndSek,
}

/// JS `m.ctl.r.opts[].en`
#[derive(Clone, Debug, Default)]
pub struct JMCtlROptsEEn {
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
    pub per: Vec<(JsDeger, JsDeger)>,
    /// `ek`: number
    pub ek: f64,
}

/// JS `m.ctl.r.opts[]._pw[]`
#[derive(Clone, Debug, Default)]
pub struct JMCtlROptsE_pwE {
}

/// JS `m.ctl.r.opts[].launch`
#[derive(Clone, Debug, Default)]
pub struct JMCtlROptsELaunch {
    /// `vx`: number
    pub vx: f64,
    /// `vy`: number
    pub vy: f64,
    /// `ok`: boolean
    pub ok: bool,
}

/// JS `m.ctl.r.opts[].lf0`
#[derive(Clone, Debug, Default)]
pub struct JMCtlROptsELf0 {
    /// `vx`: number
    pub vx: f64,
    /// `vy`: number
    pub vy: f64,
    /// `ok`: boolean
    pub ok: bool,
}

/// JS `p.savur.o._pw[].tl.huc`
#[derive(Clone, Debug, Default)]
pub struct JPSavurO_pwETlHuc {
}

/// JS `p.savur.o._pw[].tl.sav`
#[derive(Clone, Debug, Default)]
pub struct JPSavurO_pwETlSav {
}

/// JS `p.savur.o._pw[]._surK.to`
#[derive(Clone, Debug, Default)]
pub struct JPSavurO_pwE_surKTo {
}

/// JS `m.pre.r.best.det.end`
#[derive(Clone, Debug, Default)]
pub struct JMPreRBestDetEnd {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `m.pre.r.best.det.endM`
#[derive(Clone, Debug, Default)]
pub struct JMPreRBestDetEndM {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `m.pre.r.best.det.endO`
#[derive(Clone, Debug, Default)]
pub struct JMPreRBestDetEndO {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `m.pre.r.best.det.endS`
#[derive(Clone, Debug, Default)]
pub struct JMPreRBestDetEndS {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `m.pre.r.best.det.endD`
#[derive(Clone, Debug, Default)]
pub struct JMPreRBestDetEndD {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `m.pre.r.best.det.endSek`
#[derive(Clone, Debug, Default)]
pub struct JMPreRBestDetEndSek {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `m.pre.r.best._pw[].slot`
#[derive(Clone, Debug, Default)]
pub struct JMPreRBest_pwESlot {
}

/// JS `m.pre.r.best._pw[].a`
#[derive(Clone, Debug, Default)]
pub struct JMPreRBest_pwEA {
}

/// JS `m.pre.r.best._pw[].driveTo`
#[derive(Clone, Debug, Default)]
pub struct JMPreRBest_pwEDriveTo {
}

/// JS `m.pre.r.best._pw[]._ktB`
#[derive(Clone, Debug, Default)]
pub struct JMPreRBest_pwE_ktB {
}

/// JS `m.pre.r.best._pw[].rslot`
#[derive(Clone, Debug, Default)]
pub struct JMPreRBest_pwERslot {
}

/// JS `m.pre.r.best._pw[].slot0`
#[derive(Clone, Debug, Default)]
pub struct JMPreRBest_pwESlot0 {
}

/// JS `m.pre.r.best._pw[].rslot0`
#[derive(Clone, Debug, Default)]
pub struct JMPreRBest_pwERslot0 {
}

/// JS `m.pre.r.best._pw[].tl`
#[derive(Clone, Debug, Default)]
pub struct JMPreRBest_pwETl {
}

/// JS `m.pre.r.best._pw[].eg`
#[derive(Clone, Debug, Default)]
pub struct JMPreRBest_pwEEg {
}

/// JS `m.pre.r.best._pw[]._otE`
#[derive(Clone, Debug, Default)]
pub struct JMPreRBest_pwE_otE {
}

/// JS `m.pre.r.best._pw[]._kt`
#[derive(Clone, Debug, Default)]
pub struct JMPreRBest_pwE_kt {
}

/// JS `m.pre.r.best._pw[]._o`
#[derive(Clone, Debug, Default)]
pub struct JMPreRBest_pwE_o {
}

/// JS `m.pre.r.best._pw[]._otA`
#[derive(Clone, Debug, Default)]
pub struct JMPreRBest_pwE_otA {
}

/// JS `m.pre.r.best._pw[]._kh`
#[derive(Clone, Debug, Default)]
pub struct JMPreRBest_pwE_kh {
}

/// JS `m.pre.r.best._pw[]._onK`
#[derive(Clone, Debug, Default)]
pub struct JMPreRBest_pwE_onK {
}

/// JS `m.pre.r.best._pw[].adT`
#[derive(Clone, Debug, Default)]
pub struct JMPreRBest_pwEAdT {
}

/// JS `m.pre.r.best._pw[].son`
#[derive(Clone, Debug, Default)]
pub struct JMPreRBest_pwESon {
}

/// JS `m.pre.r.best._pw[]._gvKaz`
#[derive(Clone, Debug, Default)]
pub struct JMPreRBest_pwE_gvKaz {
}

/// JS `m.pre.r.best._pw[].pplan`
#[derive(Clone, Debug, Default)]
pub struct JMPreRBest_pwEPplan {
}

/// JS `m.pre.r.best._pw[]._omuz`
#[derive(Clone, Debug, Default)]
pub struct JMPreRBest_pwE_omuz {
}

/// JS `m.pre.r.opts[].det.end`
#[derive(Clone, Debug, Default)]
pub struct JMPreROptsEDetEnd {
}

/// JS `m.pre.r.opts[].det.endM`
#[derive(Clone, Debug, Default)]
pub struct JMPreROptsEDetEndM {
}

/// JS `m.pre.r.opts[].det.endO`
#[derive(Clone, Debug, Default)]
pub struct JMPreROptsEDetEndO {
}

/// JS `m.pre.r.opts[].det.endS`
#[derive(Clone, Debug, Default)]
pub struct JMPreROptsEDetEndS {
}

/// JS `m.pre.r.opts[].det.endD`
#[derive(Clone, Debug, Default)]
pub struct JMPreROptsEDetEndD {
}

/// JS `m.pre.r.opts[].det.endSek`
#[derive(Clone, Debug, Default)]
pub struct JMPreROptsEDetEndSek {
}

/// JS `m.ctl.r.best.det.end`
#[derive(Clone, Debug, Default)]
pub struct JMCtlRBestDetEnd {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `m.ctl.r.best.det.endM`
#[derive(Clone, Debug, Default)]
pub struct JMCtlRBestDetEndM {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `m.ctl.r.best.det.endO`
#[derive(Clone, Debug, Default)]
pub struct JMCtlRBestDetEndO {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `m.ctl.r.best.det.endS`
#[derive(Clone, Debug, Default)]
pub struct JMCtlRBestDetEndS {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `m.ctl.r.best.det.endD`
#[derive(Clone, Debug, Default)]
pub struct JMCtlRBestDetEndD {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `m.ctl.r.best.det.endSek`
#[derive(Clone, Debug, Default)]
pub struct JMCtlRBestDetEndSek {
    /// `x`: number
    pub x: f64,
    /// `y`: number
    pub y: f64,
}

/// JS `m.ctl.r.best._pw[].slot`
#[derive(Clone, Debug, Default)]
pub struct JMCtlRBest_pwESlot {
}

/// JS `m.ctl.r.best._pw[].a`
#[derive(Clone, Debug, Default)]
pub struct JMCtlRBest_pwEA {
}

/// JS `m.ctl.r.best._pw[].driveTo`
#[derive(Clone, Debug, Default)]
pub struct JMCtlRBest_pwEDriveTo {
}

/// JS `m.ctl.r.best._pw[]._ktB`
#[derive(Clone, Debug, Default)]
pub struct JMCtlRBest_pwE_ktB {
}

/// JS `m.ctl.r.best._pw[].rslot`
#[derive(Clone, Debug, Default)]
pub struct JMCtlRBest_pwERslot {
}

/// JS `m.ctl.r.best._pw[].slot0`
#[derive(Clone, Debug, Default)]
pub struct JMCtlRBest_pwESlot0 {
}

/// JS `m.ctl.r.best._pw[].rslot0`
#[derive(Clone, Debug, Default)]
pub struct JMCtlRBest_pwERslot0 {
}

/// JS `m.ctl.r.best._pw[].tl`
#[derive(Clone, Debug, Default)]
pub struct JMCtlRBest_pwETl {
}

/// JS `m.ctl.r.best._pw[].eg`
#[derive(Clone, Debug, Default)]
pub struct JMCtlRBest_pwEEg {
}

/// JS `m.ctl.r.best._pw[]._otE`
#[derive(Clone, Debug, Default)]
pub struct JMCtlRBest_pwE_otE {
}

/// JS `m.ctl.r.best._pw[]._kt`
#[derive(Clone, Debug, Default)]
pub struct JMCtlRBest_pwE_kt {
}

/// JS `m.ctl.r.best._pw[]._otA`
#[derive(Clone, Debug, Default)]
pub struct JMCtlRBest_pwE_otA {
}

/// JS `m.ctl.r.best._pw[]._kh`
#[derive(Clone, Debug, Default)]
pub struct JMCtlRBest_pwE_kh {
}

/// JS `m.ctl.r.best._pw[]._onK`
#[derive(Clone, Debug, Default)]
pub struct JMCtlRBest_pwE_onK {
}

/// JS `m.ctl.r.best._pw[].adT`
#[derive(Clone, Debug, Default)]
pub struct JMCtlRBest_pwEAdT {
}

/// JS `m.ctl.r.best._pw[].son`
#[derive(Clone, Debug, Default)]
pub struct JMCtlRBest_pwESon {
}

/// JS `m.ctl.r.best._pw[]._gvKaz`
#[derive(Clone, Debug, Default)]
pub struct JMCtlRBest_pwE_gvKaz {
}

/// JS `m.ctl.r.best._pw[].pplan`
#[derive(Clone, Debug, Default)]
pub struct JMCtlRBest_pwEPplan {
}

/// JS `m.ctl.r.opts[].det.end`
#[derive(Clone, Debug, Default)]
pub struct JMCtlROptsEDetEnd {
}

/// JS `m.ctl.r.opts[].det.endM`
#[derive(Clone, Debug, Default)]
pub struct JMCtlROptsEDetEndM {
}

/// JS `m.ctl.r.opts[].det.endO`
#[derive(Clone, Debug, Default)]
pub struct JMCtlROptsEDetEndO {
}

/// JS `m.ctl.r.opts[].det.endS`
#[derive(Clone, Debug, Default)]
pub struct JMCtlROptsEDetEndS {
}

/// JS `m.ctl.r.opts[].det.endD`
#[derive(Clone, Debug, Default)]
pub struct JMCtlROptsEDetEndD {
}

/// JS `m.ctl.r.opts[].det.endSek`
#[derive(Clone, Debug, Default)]
pub struct JMCtlROptsEDetEndSek {
}
