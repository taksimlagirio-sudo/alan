//! Kafa1'in çağırdığı yabancı işlevler: dış dünya (`Dis`) ve kafa2'ye bağ.
//!
//! İki tür var:
//! 1. **Dış modüller** (maç, yerleşim, enerji/sprint): [`Dis`] özelliğinin yöntemleri. Gövdeleri
//!    `todo!()`; maç modülü bu özelliği kendi bağlamında gerçekler. Sınamalar kendi kayıtlı
//!    gerçeklemelerini verir (JS'ten kaydedilmiş değerler ya da sınamaya taşınmış küçük işlevler).
//!    Kafa bunları `self.dis` üzerinden çağırır (gizli küresel yok).
//! 2. **Kafa2'nin işlevleri** (decide.js 4717 sonrası): bu dosyanın sonundaki serbest işlevler;
//!    kafa2'nin gerçeklerine bağlıdır (imza farkları burada uydurulur).
//!
//! Her yöntemin başında JS kaynağı (dosya:satır, sade kaynakta) yazılı.
#![allow(non_snake_case, unused_variables)]

use crate::temas::Kural as _;
use super::kafa1_baglam::Kafa;
use super::kafa1_js::{Nk, Op};
use crate::model::{JMShGs, Oyuncu};
use crate::oyuncu::TepkiS;

/// Geçiş tablosu: kuralın kendisi (kafanın ayrı biçimi ve çevirisi kalktı; MIMARI.md 4c-2c).
pub use crate::kural::gec::{GecTab, GecY};

/// Yerleşim parametreleri (shape.js `S`): kafanın okuduğu alanlar.
#[derive(Clone, Debug)]
pub struct ShapeS {
    pub react: Option<f64>,
    pub rcvBuf: f64,
    pub marginT: f64,
    pub markOff: Option<f64>,
    pub bekOnce: Option<f64>,
    pub bekSerbest: bool,
}

/// Yerleşimin algı parametreleri (shape.js `GR`).
#[derive(Clone, Debug)]
pub struct ShapeGR {
    pub yasH: f64,
}

/// Maçın ikili parametreleri (match.js:1836 `IK`).
#[derive(Clone, Copy, Debug)]
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
pub const IK_JS: Ik = Ik { yerles: 1.6, esnek: 0.15, surt: 0.12, dengeK: 3.2, dolum: 0.012, dolumS: 0.03, yavas: 0.35, sendele: 0.15, sendT: 12.0, sendCd: 40.0, kalkan: 0.7, kalkanR: 3.2, pokeT: 0.22, pokeK: 0.03, pokeAc: 0.5, pokeDn: 0.35, omuzV: 0.2 };

/// Sprint evreleri (match.js:2110 `ALAN_SPR_FAZ`; sürekli koşuda değiştirilmiş kopyası).
#[derive(Clone, Copy, Debug)]
pub struct SprFaz {
    pub ust: f64,
    pub hkPay: f64,
    pub tepeT: f64,
    pub after: f64,
    pub dolT: f64,
    pub bedel: f64,
    pub patT: f64,
    pub frenT: f64,
    pub patK: f64,
    pub ayir: f64,
    pub itKos: f64,
    pub topla: f64,
    pub toplaBos: f64,
    /// `itBos` (tanımsız: 6 kullanılır), `peak` (tanımsız)
    pub itBos: Option<f64>,
    pub peak: Option<f64>,
}

/// Enerji 2.0 sabitleri (core.js:113 `ALAN_EN2`; `pat` sürekli koşuda 0).
#[derive(Clone, Copy, Debug)]
pub struct En2 {
    pub tepe: f64,
    pub pat: f64,
    pub dinlen: f64,
    pub tempoDol: f64,
    pub yogK: f64,
    pub yogMin: f64,
}

/// Enerji sabitleri (core.js:66 `ALAN_EN`, match.js'in değiştirdikleriyle).
#[derive(Clone, Copy, Debug)]
pub struct En {
    pub dalis: f64,
    pub dalisDolYok: f64,
    pub sprEsik: f64,
    pub ivme: f64,
    pub surme: f64,
    pub dolas: f64,
    pub fake: f64,
    pub kalkan: f64,
    pub fizK: f64,
}

/// Top sürme dokunuşu (match.js:1430 `ALAN_DOKUNUS`).
#[derive(Clone, Copy, Debug)]
pub struct Dokunus {
    pub close: f64,
    pub closeS: f64,
    pub lRun: f64,
    pub lRunS: f64,
    pub aMax: Option<f64>,
    pub lMax: f64,
    pub gor: Option<f64>,
    pub bos: f64,
    pub k: f64,
    pub toplaG: Option<f64>,
    pub topla: f64,
    pub riskEski: bool,
}

/// Maçın pres tartısı (match.js `m._prT`): girEsik okur.
#[derive(Clone, Debug, Default)]
pub struct PrT {
    pub team: f64,
    pub hId: Option<f64>,
    pub t: f64,
    pub h: Option<f64>,
    /// `hm` Map'i (oyuncu id → değer) ve `hm.k` kümesi
    pub hm: Option<Vec<(f64, f64)>>,
    pub hmk: Option<Vec<f64>>,
}

/// Yerleşimin tehlike yönü (shape.js tehlike dönüşü).
#[derive(Clone, Debug, Default)]
pub struct Tehlike {
    pub an: f64,
    pub w: f64,
    pub w0: f64,
    pub x: f64,
    pub y: f64,
    pub k: String,
    pub L: f64,
}

/// Maç taktiğinin yerleşime verilen hali (match.js tacEff dönüşü): kafa yalnız taşır.
#[derive(Clone, Debug, Default)]
pub struct TacEff(pub Vec<(String, f64)>);

/// Yerleşimin baktığı maç görünümü (JS `fm`/`m`: `{ tac, ps, holder }`; kafa2 kurar).
pub struct Fm<'a> {
    pub tac: &'a [crate::model::JMTacE],
    pub ps: &'a [Op],
    pub holder: Option<crate::dunya::Yerde<'a>>,
}


/// Dış dünya: maç, yerleşim ve enerji modüllerinin kafaya açtığı işlevler.
pub trait Dis {
    /// Maçın tik başına anlık görüntüsü (sprint bağlamı, tepki parametreleri). Kafa maçın kurallarını
    /// `kural` katmanından bununla hesaplar (`KuralErisim`); yoksa (sınama) boş görüntü.
    fn kural(&self) -> crate::kural::enerji::MacKural {
        Default::default()
    }
    /// Tartı süresince taşıyıcının sprint niyetini anlık görüntüye koy; öncekini döndür (geri koymak için).
    /// Anlık görüntüsü olmayan dış dünyada (sınama) yok sayılır.
    fn niyet_koy(&self, n: Option<(f64, f64, bool, Option<f64>)>) -> Option<(f64, f64, bool, Option<f64>)> {
        let _ = n;
        None
    }
    // ---- enerji / sprint (core.js, match.js) ----
    /// core.js:169 ALAN_EN3_KUL(p, e, bizde, bx, by): enerjinin kullanım değeri
    fn ALAN_EN3_KUL(&self, p: &Oyuncu, e: f64, bizde: bool, bx: f64, by: f64) -> f64 { todo!() }

    // ---- yerleşim (shape.js, window.AlanShape) ----
    /// shape.js `S` (yoksa None)
    fn S(&self) -> Option<ShapeS> { todo!() }
    /// shape.js:321 `GR` (yoksa None)
    fn GR(&self) -> Option<ShapeGR> { todo!() }
    /// shape.js GR.yasHata(yas)
    fn GR_yasHata(&self, yas: f64) -> f64 { todo!() }
    /// shape.js:455 gside(a, t, off)
    fn gside(&self, a: Nk, t: f64, off: f64) -> Nk { todo!() }
    /// shape.js bekci(p, b, ...): Bekçi'nin yeri (yoksa None)
    fn bekci(&self, p: &Oyuncu, b: Nk) -> Option<Nk> { todo!() }
    /// shape.js:466 aliciDirenc(pts, p, rc, iMin, rP) (yoksa None)
    fn aliciDirenc(&self, pts: &[Nk], p: &Oyuncu, rc: f64, iMin: f64, rP: f64) -> Option<Nk> { todo!() }
    /// shape.js tehlike(m, h)
    fn tehlike(&self, h: &Oyuncu) -> Vec<Tehlike> { todo!() }
    /// shape.js:857 ilkPres(vm, qv, hx, team, TH, dtac, sh): qv.tx/ty'yi yazar. `tick`: vm.tick, `dtik`: kafanın karar tiki (D._tick)
    fn ilkPres(&self, dtik: f64, tick: f64, qv: &mut Oyuncu, hx: &Oyuncu, team: f64, TH: &[Tehlike], dtac: &TacEff, gs: &Option<JMShGs>) { todo!() }
    /// AlanShape var mı, ilkPres var mı (presRol'ün koşulu)
    fn AS_ilkPres_var(&self) -> bool { todo!() }
    /// (birleştirmede eklendi; kafa2) shape.js bekci(p, B, m): `m` verili maç görünümüyle (fm)
    fn bekciFm(&self, p: &Oyuncu, B: Nk, fm: &Fm) -> Nk { todo!() }
    /// (birleştirmede eklendi; kafa2) shape.js slotBase(m, p, B, hucum): oyuncunun yerleşimdeki yeri
    fn slotBase(&self, fm: &Fm, p: &Oyuncu, B: Nk, hucum: bool) -> Nk { todo!() }
    /// (birleştirmede eklendi; kafa2) shape.js S.prHat[blok] (yoksa None)
    fn S_prHat(&self, blok: &str) -> Option<f64> { todo!() }

    // ---- maç (match.js, window.AlanMatch) ----
    /// (birleştirmede eklendi; kafa2) match.js:939 tl(p, ev, k): takım talimatı (`null` → None)
    fn tl(&self, p: &Oyuncu, ev: &str, k: &str) -> Option<f64> { todo!() }

    // ---- maçın durumu (D._mac) ----
    /// D._mac var mı
    fn mac_var(&self) -> bool { todo!() }
    /// D._mac.ch
    fn mac_ch(&self) -> Option<f64> { todo!() }
    /// D._mac.holder: (id, team, x, y)
    fn mac_holder(&self) -> Option<(f64, f64, f64, f64)> { todo!() }
    /// D._mac.tac[team].enHarcama
    fn tac_enHarcama(&self, team: f64) -> Option<f64> { todo!() }
    /// siddetQ'nun ham değeri (decide.js:4644): m.tac[k].siddet ?? .5, yz ve macEff(m,k,{}).macK ?? 1 (macEff yoksa 1); None: maç/taktik yok
    fn tac_siddet(&self, team: f64) -> Option<(Option<f64>, bool, f64)> { todo!() }
    /// AlanMatch.tacEff(m, dt) (yoksa m.tac[dt])
    fn tacEff(&self, team: f64) -> TacEff { todo!() }
    /// D._mac.sh.gs
    fn sh_gs(&self) -> Option<JMShGs> { todo!() }
    /// D._mac._prT
    fn prT(&self) -> Option<PrT> { todo!() }

    // ---- algı kopyaları (shape.js seen) ----
    /// algılanan oyuncunun konum hatası `__e` (gerçek oyuncu: None)
    fn algi_e(&self, p: &Oyuncu) -> Option<f64> { todo!() }
    /// algılanan oyuncunun yaşı `__yas` (gerçek oyuncu: None)
    fn algi_yas(&self, p: &Oyuncu) -> Option<f64> { todo!() }
}

/// Maçın kurallarına kafadan erişim: `kural` katmanını dış dünyanın anlık görüntüsüyle (`Dis::kural`) çağırır.
/// Adlar ve dönüş biçimleri eski `Dis` yöntemleriyle aynı (çağıran yerler değişmedi); her dış dünya için geçerli.
pub trait KuralErisim: Dis {
    /// core.js:102 ALAN_SPRE(p): sprint evresinin etkin hız katı.
    fn ALAN_SPRE(&self, p: &Oyuncu) -> f64 {
        crate::kural::kos::spre_k(p, &self.kural())
    }
    /// match.js:2168 ALAN_SPR(p): sprint katı (sprint evresiyle).
    fn ALAN_SPR(&self, p: &Oyuncu) -> f64 {
        crate::kural::enerji::ALAN_SPR(p, &self.kural())
    }
    /// match.js:2252 ALAN_SPR_HK(p)
    fn ALAN_SPR_HK(&self, p: &Oyuncu) -> f64 {
        crate::kural::enerji::ALAN_SPR_HK(p)
    }
    /// match.js:2110 ALAN_SPR_FAZ (sürekli koşuda kopyası)
    fn ALAN_SPR_FAZ(&self) -> Option<SprFaz> {
        let F = &crate::kural::enerji::SPR_FAZ;
        let b = |v: bool| if v { 1.0 } else { 0.0 };
        Some(SprFaz { ust: b(F.ust), hkPay: F.hkPay, tepeT: F.tepeT, after: F.after, dolT: F.dolT, bedel: F.bedel, patT: F.patT, frenT: F.frenT, patK: F.patK, ayir: b(F.ayir), itKos: b(F.itKos), topla: b(F.topla), toplaBos: F.toplaBos, itBos: None, peak: None })
    }
    /// match.js:2262 ALAN_KOSU_T(p, d, v1): d yolunu koşma süresi (sprint profiliyle)
    fn ALAN_KOSU_T(&self, p: &Oyuncu, d: f64, v1: f64) -> f64 {
        crate::kural::enerji::ALAN_KOSU_T(p, d, v1, &self.kural())
    }
    /// match.js:2268 ALAN_KOSU_M(p, t, v1): t tikte alınan yol
    fn ALAN_KOSU_M(&self, p: &Oyuncu, t: f64, v1: f64) -> f64 {
        crate::kural::enerji::ALAN_KOSU_M(p, t, v1, &self.kural())
    }
    /// core.js:163 ALAN_EN3_SGIR(p, bed)
    fn ALAN_EN3_SGIR(&self, p: &Oyuncu, bed: f64) -> f64 {
        crate::kural::enerji::ALAN_EN3_SGIR(p, bed)
    }
    /// core.js:191 ALAN_EN2_TS(p, e)
    fn ALAN_EN2_TS(&self, p: &Oyuncu, e: Option<f64>) -> f64 {
        crate::kural::enerji::ALAN_EN2_TS(p, e)
    }
    /// core.js:188 ALAN_EN2_SB(p)
    fn ALAN_EN2_SB(&self, p: &Oyuncu) -> f64 {
        crate::kural::enerji::ALAN_EN2_SB(p)
    }
    /// core.js:159 ALAN_EN3_DOL(p, sahip, nd, yerK)
    fn ALAN_EN3_DOL(&self, p: &Oyuncu, sahip: bool, nd: f64, yerK: f64) -> f64 {
        crate::kural::enerji::ALAN_EN3_DOL(p, sahip, nd, yerK)
    }
    /// core.js:154 ALAN_EN3_TEPE({en}, k)
    fn ALAN_EN3_TEPE(&self, e: Option<f64>, k: f64) -> f64 {
        crate::kural::enerji::ALAN_EN3_TEPE(e, k)
    }
    /// core.js:98 ALAN_EN_S(p)
    fn ALAN_EN_S(&self, p: &Oyuncu) -> f64 {
        crate::kural::enerji::ALAN_EN_S(p)
    }
    /// core.js:113 ALAN_EN2
    fn ALAN_EN2(&self) -> En2 {
        let E = &crate::kural::enerji::EN2;
        En2 { tepe: E.tepe, pat: E.pat, dinlen: E.dinlen, tempoDol: E.tempoDol, yogK: E.yogK, yogMin: E.yogMin }
    }
    /// core.js:66 ALAN_EN
    fn ALAN_EN(&self) -> En {
        let E = &crate::kural::enerji::EN;
        En { dalis: E.dalis, dalisDolYok: E.dalisDolYok, sprEsik: E.sprEsik, ivme: E.ivme, surme: E.surme, dolas: E.dolas, fake: E.fake, kalkan: E.kalkan, fizK: E.fizK }
    }
    /// match.js:1418 ALAN_DENGE_KAYIP(p, t)
    fn ALAN_DENGE_KAYIP(&self, p: &Oyuncu, t: f64) -> f64 {
        crate::kural::ikili::dengeKayip(p, t)
    }
    /// match.js:1430 ALAN_DOKUNUS
    fn ALAN_DOKUNUS(&self) -> Option<Dokunus> {
        None
    }
    /// self.ALAN_YOL_VER_TEPKI (tanımsız: false)
    fn ALAN_YOL_VER_TEPKI(&self) -> bool {
        false
    }
    /// self.ALAN_TASIYICI_SONRA_YAVAS (tanımsız: false)
    fn ALAN_TASIYICI_SONRA_YAVAS(&self) -> bool {
        false
    }
    /// shape.js:11 tepki parametreleri (S.lag0, lagOk, react, reactLoose, reactLooseOk): ALAN_TEPKI'nin
    fn tepki_s(&self) -> TepkiS {
        self.kural().tepki_s()
    }
    /// shape.js:152 gecTab(o); `refsiz`: JS'te `ref: null`/`ref = undefined` verilmiş kopya (kendi yeri)
    fn gecTab(&self, o: &Oyuncu, _refsiz: bool) -> Option<GecTab> {
        Some(crate::kural::gec::gecTab(o, &self.kural()))
    }
    /// shape.js:183 gecOf(G, x, y, s)
    fn gecOf(&self, G: &GecTab, x: f64, y: f64, s: Option<f64>) -> f64 {
        crate::kural::gec::gecOf(G, x, y, s)
    }
    /// shape.js:145 gec(o, x, y, s) (yoksa None)
    fn gec(&self, o: &Oyuncu, x: f64, y: f64, s: Option<f64>) -> Option<f64> {
        Some(crate::kural::gec::gec(o, x, y, s, &self.kural()))
    }
    /// match.js:1836 IK (yoksa None: ikili kafası kapalı)
    fn IK(&self) -> Option<Ik> {
        Some(IK_JS)
    }
    /// match.js:1601 ivme(p, wx, wy, vmax)
    fn ivme(&self, p: &mut Oyuncu, wx: f64, wy: f64, vmax: f64) {
        crate::kural::hareket::ivme(p, wx, wy, vmax)
    }
    /// match.js:1857 kutle(p, h): `ayni` = (p === h)
    fn kutle(&self, p: &Oyuncu, ayni: bool) -> f64 {
        crate::kural::ikili::kutle(p, ayni)
    }
    /// match.js:1857 direnc(p, h): `ayni` = (p === h)
    fn direnc(&self, p: &Oyuncu, ayni: bool) -> f64 {
        crate::kural::ikili::direnc(p, ayni)
    }
    /// match.js:1856 omuzK(q)
    fn omuzK(&self, q: &Oyuncu) -> f64 {
        crate::kural::ikili::omuzK(q)
    }
    /// match.js:1856 durtFark(q, h) (yoksa None)
    fn durtFark(&self, q: &Oyuncu, h: &Oyuncu) -> Option<f64> {
        Some(crate::kural::ikili::durtFark(q, h))
    }
    /// match.js:1961 sendS(p) (yoksa None)
    fn sendS(&self, p: &Oyuncu) -> Option<f64> {
        Some(crate::kural::ikili::sendS(p))
    }
    /// match.js:1855 ikEn(p) (yoksa None)
    fn ikEn(&self, p: &Oyuncu) -> Option<f64> {
        Some(crate::kural::ikili::ikEn(p))
    }
    /// AlanMatch.BEKLE_BOZ (yoksa None)
    fn BEKLE_BOZ(&self) -> Option<f64> {
        Some(crate::kural::BEKLE_BOZ)
    }
    /// (birleştirmede eklendi; kafa2) match.js:311 sprK(p): enerjiyle kısılmış sprint katı
    fn sprK(&self, p: &Oyuncu) -> f64 {
        crate::kural::enerji::sprK(p, &self.kural())
    }
}
impl<T: Dis + ?Sized> KuralErisim for T {}



// ======================================================================
// Kafa2'nin işlevleri (decide.js 4717 sonrası): kafa2'nin gerçeklerine bağlı.
// ======================================================================

/// Kafa2'nin `predictWorld` dönüşündeki bir oyuncunun kestirilen yeri (null: değişmedi).
pub type DunyaYer = Option<Nk>;

/// decide.js:4920 predictWorld(src, team, B, T, cevapAyri, undefined, pasta): oyuncuların T tik
/// sonraki yerleri (src sırasıyla; JS'te her öğe bir oyuncu nesnesi, değişmeyen de: hep Some).
pub fn predictWorld(k: &mut Kafa, src: &[Op], team: f64, B: Nk, T: f64, cevapAyri: bool, pasta: bool) -> Vec<DunyaYer> {
    let w = super::kafa2_dunya::predictWorld(k, src, team, B, T, cevapAyri, None, pasta, None);
    w.iter().map(|q| Some(Nk { x: q.x, y: q.y })).collect()
}
/// decide.js:4789 bekSav(p, h, T, src): şut savrulurken Bekçi'nin yeri (kopya).
pub fn bekSav(k: &Kafa, p: &Op, h: &Oyuncu, T: f64, src: &[Op]) -> Op {
    super::kafa2_dunya::bekSav(k, p, h, T, src)
}
/// decide.js:5454 bosluk(src, team, bx, by)
pub fn bosluk(k: &mut Kafa, src: &[Op], team: f64, bx: f64, by: f64) -> f64 {
    super::kafa2_imkan::bosluk(k, src, team, bx, by)
}
/// decide.js:5440 sutEngel(src, team, x, y)
pub fn sutEngel(k: &Kafa, src: &[Op], team: f64, x: f64, y: f64) -> f64 {
    super::kafa2_imkan::sutEngel(k, src, team, x, y)
}
/// decide.js:6773 yerDeger(w, team, x, y, ch)
pub fn yerDeger(k: &mut Kafa, w: &[Op], team: f64, x: f64, y: f64, ch: f64) -> f64 {
    super::kafa2_zemin::yerDeger(k, w, team, x, y, ch)
}

/// Maçın dokunuş ayarı (JS ALAN_DOKUNUS): bugün kapalı (maç taşımadı)
pub fn mac_dokunus() -> Option<Dokunus> {
    None
}
