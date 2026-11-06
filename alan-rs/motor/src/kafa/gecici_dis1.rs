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
    pub rcv_buf: f64,
    pub margin_t: f64,
    pub mark_off: Option<f64>,
    pub bek_once: Option<f64>,
    pub bek_serbest: bool,
}

/// Yerleşimin algı parametreleri (shape.js `GR`).
#[derive(Clone, Debug)]
pub struct ShapeGR {
    pub yas_h: f64,
}

/// Maçın ikili parametreleri (match.js:1836 `IK`).
#[derive(Clone, Copy, Debug)]
pub struct Ik {
    pub yerles: f64,
    pub esnek: f64,
    pub surt: f64,
    pub dengeK: f64,
    pub dolum: f64,
    pub dolum_s: f64,
    pub yavas: f64,
    pub sendele: f64,
    pub sendT: f64,
    pub sendCd: f64,
    pub kalkan: f64,
    pub kalkan_r: f64,
    pub poke_t: f64,
    pub poke_k: f64,
    pub poke_ac: f64,
    pub poke_dn: f64,
    pub omuz_v: f64,
}
pub const IK_JS: Ik = Ik { yerles: 1.6, esnek: 0.15, surt: 0.12, dengeK: 3.2, dolum: 0.012, dolum_s: 0.03, yavas: 0.35, sendele: 0.15, sendT: 12.0, sendCd: 40.0, kalkan: 0.7, kalkan_r: 3.2, poke_t: 0.22, poke_k: 0.03, poke_ac: 0.5, poke_dn: 0.35, omuz_v: 0.2 };

/// Sprint evreleri (match.js:2110 `ALAN_SPR_FAZ`; sürekli koşuda değiştirilmiş kopyası).
#[derive(Clone, Copy, Debug)]
pub struct SprFaz {
    pub ust: f64,
    pub hk_pay: f64,
    pub tepe_t: f64,
    pub after: f64,
    pub dol_t: f64,
    pub bedel: f64,
    pub pat_t: f64,
    pub fren_t: f64,
    pub pat_k: f64,
    pub ayir: f64,
    pub it_kos: f64,
    pub topla: f64,
    pub topla_bos: f64,
    /// `itBos` (tanımsız: 6 kullanılır), `peak` (tanımsız)
    pub it_bos: Option<f64>,
    pub peak: Option<f64>,
}

/// Enerji 2.0 sabitleri (core.js:113 `ALAN_EN2`; `pat` sürekli koşuda 0).
#[derive(Clone, Copy, Debug)]
pub struct En2 {
    pub tepe: f64,
    pub pat: f64,
    pub dinlen: f64,
    pub tempo_dol: f64,
    pub yog_k: f64,
    pub yog_min: f64,
}

/// Enerji sabitleri (core.js:66 `ALAN_EN`, match.js'in değiştirdikleriyle).
#[derive(Clone, Copy, Debug)]
pub struct En {
    pub dalis: f64,
    pub dalis_dol_yok: f64,
    pub spr_esik: f64,
    pub ivme: f64,
    pub surme: f64,
    pub dolas: f64,
    pub fake: f64,
    pub kalkan: f64,
    pub fiz_k: f64,
}

/// Top sürme dokunuşu (match.js:1430 `ALAN_DOKUNUS`).
#[derive(Clone, Copy, Debug)]
pub struct Dokunus {
    pub close: f64,
    pub close_s: f64,
    pub l_run: f64,
    pub l_run_s: f64,
    pub a_max: Option<f64>,
    pub l_max: f64,
    pub gor: Option<f64>,
    pub bos: f64,
    pub k: f64,
    pub topla_g: Option<f64>,
    pub topla: f64,
    pub risk_eski: bool,
}

/// Maçın pres tartısı (match.js `m._prT`): girEsik okur.
#[derive(Clone, Debug, Default)]
pub struct PrT {
    pub team: f64,
    pub h_id: Option<f64>,
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
    fn alan_en3_kul(&self, p: &Oyuncu, e: f64, bizde: bool, bx: f64, by: f64) -> f64 { todo!() }

    // ---- yerleşim (shape.js, window.AlanShape) ----
    /// shape.js `S` (yoksa None)
    fn S(&self) -> Option<ShapeS> { todo!() }
    /// shape.js:321 `GR` (yoksa None)
    fn gr(&self) -> Option<ShapeGR> { todo!() }
    /// shape.js GR.yasHata(yas)
    fn gr_yas_hata(&self, yas: f64) -> f64 { todo!() }
    /// shape.js:455 gside(a, t, off)
    fn gside(&self, a: Nk, t: f64, off: f64) -> Nk { todo!() }
    /// shape.js bekci(p, b, ...): Bekçi'nin yeri (yoksa None)
    fn bekci(&self, p: &Oyuncu, b: Nk) -> Option<Nk> { todo!() }
    /// shape.js:466 aliciDirenc(pts, p, rc, iMin, rP) (yoksa None)
    fn alici_direnc(&self, pts: &[Nk], p: &Oyuncu, rc: f64, i_min: f64, r_p: f64) -> Option<Nk> { todo!() }
    /// shape.js tehlike(m, h)
    fn tehlike(&self, h: &Oyuncu) -> Vec<Tehlike> { todo!() }
    /// shape.js:857 ilkPres(vm, qv, hx, team, TH, dtac, sh): qv.tx/ty'yi yazar. `tick`: vm.tick, `dtik`: kafanın karar tiki (D._tick)
    fn ilk_pres(&self, dtik: f64, tick: f64, qv: &mut Oyuncu, hx: &Oyuncu, team: f64, TH: &[Tehlike], dtac: &TacEff, gs: &Option<JMShGs>) { todo!() }
    /// AlanShape var mı, ilkPres var mı (presRol'ün koşulu)
    fn as_ilk_pres_var(&self) -> bool { todo!() }
    /// (birleştirmede eklendi; kafa2) shape.js bekci(p, B, m): `m` verili maç görünümüyle (fm)
    fn bekci_fm(&self, p: &Oyuncu, B: Nk, fm: &Fm) -> Nk { todo!() }
    /// (birleştirmede eklendi; kafa2) shape.js slotBase(m, p, B, hucum): oyuncunun yerleşimdeki yeri
    fn slot_base(&self, fm: &Fm, p: &Oyuncu, B: Nk, hucum: bool) -> Nk { todo!() }
    /// (birleştirmede eklendi; kafa2) shape.js S.prHat[blok] (yoksa None)
    fn s_pr_hat(&self, blok: &str) -> Option<f64> { todo!() }

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
    fn tac_en_harcama(&self, team: f64) -> Option<f64> { todo!() }
    /// siddetQ'nun ham değeri (decide.js:4644): m.tac[k].siddet ?? .5, yz ve macEff(m,k,{}).macK ?? 1 (macEff yoksa 1); None: maç/taktik yok
    fn tac_siddet(&self, team: f64) -> Option<(Option<f64>, bool, f64)> { todo!() }
    /// AlanMatch.tacEff(m, dt) (yoksa m.tac[dt])
    fn tac_eff(&self, team: f64) -> TacEff { todo!() }
    /// D._mac.sh.gs
    fn sh_gs(&self) -> Option<JMShGs> { todo!() }
    /// D._mac._prT
    fn pr_t(&self) -> Option<PrT> { todo!() }

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
    fn alan_spre(&self, p: &Oyuncu) -> f64 {
        crate::kural::kos::spre_k(p, &self.kural())
    }
    /// match.js:2168 ALAN_SPR(p): sprint katı (sprint evresiyle).
    fn alan_spr(&self, p: &Oyuncu) -> f64 {
        crate::kural::enerji::alan_spr(p, &self.kural())
    }
    /// match.js:2252 ALAN_SPR_HK(p)
    fn alan_spr_hk(&self, p: &Oyuncu) -> f64 {
        crate::kural::enerji::alan_spr_hk(p)
    }
    /// match.js:2110 ALAN_SPR_FAZ (sürekli koşuda kopyası)
    fn alan_spr_faz(&self) -> Option<SprFaz> {
        let F = &crate::kural::enerji::SPR_FAZ;
        let b = |v: bool| if v { 1.0 } else { 0.0 };
        Some(SprFaz { ust: b(F.ust), hk_pay: F.hk_pay, tepe_t: F.tepe_t, after: F.after, dol_t: F.dol_t, bedel: F.bedel, pat_t: F.pat_t, fren_t: F.fren_t, pat_k: F.pat_k, ayir: b(F.ayir), it_kos: b(F.it_kos), topla: b(F.topla), topla_bos: F.topla_bos, it_bos: None, peak: None })
    }
    /// match.js:2262 ALAN_KOSU_T(p, d, v1): d yolunu koşma süresi (sprint profiliyle)
    fn alan_kosu_t(&self, p: &Oyuncu, d: f64, v1: f64) -> f64 {
        crate::kural::enerji::alan_kosu_t(p, d, v1, &self.kural())
    }
    /// match.js:2268 ALAN_KOSU_M(p, t, v1): t tikte alınan yol
    fn alan_kosu_m(&self, p: &Oyuncu, t: f64, v1: f64) -> f64 {
        crate::kural::enerji::alan_kosu_m(p, t, v1, &self.kural())
    }
    /// core.js:163 ALAN_EN3_SGIR(p, bed)
    fn alan_en3_sgir(&self, p: &Oyuncu, bed: f64) -> f64 {
        crate::kural::enerji::alan_en3_sgir(p, bed)
    }
    /// core.js:191 ALAN_EN2_TS(p, e)
    fn alan_en2_ts(&self, p: &Oyuncu, e: Option<f64>) -> f64 {
        crate::kural::enerji::alan_en2_ts(p, e)
    }
    /// core.js:188 ALAN_EN2_SB(p)
    fn alan_en2_sb(&self, p: &Oyuncu) -> f64 {
        crate::kural::enerji::alan_en2_sb(p)
    }
    /// core.js:159 ALAN_EN3_DOL(p, sahip, nd, yerK)
    fn alan_en3_dol(&self, p: &Oyuncu, sahip: bool, nd: f64, yer_k: f64) -> f64 {
        crate::kural::enerji::alan_en3_dol(p, sahip, nd, yer_k)
    }
    /// core.js:154 ALAN_EN3_TEPE({en}, k)
    fn alan_en3_tepe(&self, e: Option<f64>, k: f64) -> f64 {
        crate::kural::enerji::alan_en3_tepe(e, k)
    }
    /// core.js:98 ALAN_EN_S(p)
    fn alan_en_s(&self, p: &Oyuncu) -> f64 {
        crate::kural::enerji::alan_en_s(p)
    }
    /// core.js:113 ALAN_EN2
    fn alan_en2(&self) -> En2 {
        let E = &crate::kural::enerji::en2;
        En2 { tepe: E.tepe, pat: E.pat, dinlen: E.dinlen, tempo_dol: E.tempo_dol, yog_k: E.yog_k, yog_min: E.yog_min }
    }
    /// core.js:66 ALAN_EN
    fn alan_en(&self) -> En {
        let E = &crate::kural::enerji::EN;
        En { dalis: E.dalis, dalis_dol_yok: E.dalis_dol_yok, spr_esik: E.spr_esik, ivme: E.ivme, surme: E.surme, dolas: E.dolas, fake: E.fake, kalkan: E.kalkan, fiz_k: E.fiz_k }
    }
    /// match.js:1418 ALAN_DENGE_KAYIP(p, t)
    fn alan_denge_kayip(&self, p: &Oyuncu, t: f64) -> f64 {
        crate::kural::ikili::denge_kayip(p, t)
    }
    /// match.js:1430 ALAN_DOKUNUS
    fn alan_dokunus(&self) -> Option<Dokunus> {
        None
    }
    /// self.ALAN_YOL_VER_TEPKI (tanımsız: false)
    fn alan_yol_ver_tepki(&self) -> bool {
        false
    }
    /// self.ALAN_TASIYICI_SONRA_YAVAS (tanımsız: false)
    fn alan_tasiyici_sonra_yavas(&self) -> bool {
        false
    }
    /// shape.js:11 tepki parametreleri (S.lag0, lagOk, react, reactLoose, reactLooseOk): ALAN_TEPKI'nin
    fn tepki_s(&self) -> TepkiS {
        self.kural().tepki_s()
    }
    /// shape.js:152 gecTab(o); `refsiz`: JS'te `ref: null`/`ref = undefined` verilmiş kopya (kendi yeri)
    fn gec_tab(&self, o: &Oyuncu, _refsiz: bool) -> Option<GecTab> {
        Some(crate::kural::gec::gec_tab(o, &self.kural()))
    }
    /// shape.js:183 gecOf(G, x, y, s)
    fn gec_of(&self, G: &GecTab, x: f64, y: f64, s: Option<f64>) -> f64 {
        crate::kural::gec::gec_of(G, x, y, s)
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
    fn omuz_k(&self, q: &Oyuncu) -> f64 {
        crate::kural::ikili::omuz_k(q)
    }
    /// match.js:1856 durtFark(q, h) (yoksa None)
    fn durt_fark(&self, q: &Oyuncu, h: &Oyuncu) -> Option<f64> {
        Some(crate::kural::ikili::durt_fark(q, h))
    }
    /// match.js:1961 sendS(p) (yoksa None)
    fn send_s(&self, p: &Oyuncu) -> Option<f64> {
        Some(crate::kural::ikili::send_s(p))
    }
    /// match.js:1855 ikEn(p) (yoksa None)
    fn ik_en(&self, p: &Oyuncu) -> Option<f64> {
        Some(crate::kural::ikili::ik_en(p))
    }
    /// AlanMatch.BEKLE_BOZ (yoksa None)
    fn bekle_boz(&self) -> Option<f64> {
        Some(crate::kural::bekle_boz)
    }
    /// (birleştirmede eklendi; kafa2) match.js:311 sprK(p): enerjiyle kısılmış sprint katı
    fn spr_k(&self, p: &Oyuncu) -> f64 {
        crate::kural::enerji::spr_k(p, &self.kural())
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
pub fn predict_world(k: &mut Kafa, src: &[Op], team: f64, B: Nk, T: f64, cevap_ayri: bool, pasta: bool) -> Vec<DunyaYer> {
    let w = super::kafa2_dunya::predict_world(k, src, team, B, T, cevap_ayri, None, pasta, None);
    w.iter().map(|q| Some(Nk { x: q.x, y: q.y })).collect()
}
/// decide.js:4789 bekSav(p, h, T, src): şut savrulurken Bekçi'nin yeri (kopya).
pub fn bek_sav(k: &Kafa, p: &Op, h: &Oyuncu, T: f64, src: &[Op]) -> Op {
    super::kafa2_dunya::bek_sav(k, p, h, T, src)
}
/// decide.js:5454 bosluk(src, team, bx, by)
pub fn bosluk(k: &mut Kafa, src: &[Op], team: f64, bx: f64, by: f64) -> f64 {
    super::kafa2_imkan::bosluk(k, src, team, bx, by)
}
/// decide.js:5440 sutEngel(src, team, x, y)
pub fn sut_engel(k: &Kafa, src: &[Op], team: f64, x: f64, y: f64) -> f64 {
    super::kafa2_imkan::sut_engel(k, src, team, x, y)
}
/// decide.js:6773 yerDeger(w, team, x, y, ch)
pub fn yer_deger(k: &mut Kafa, w: &[Op], team: f64, x: f64, y: f64, ch: f64) -> f64 {
    super::kafa2_zemin::yer_deger(k, w, team, x, y, ch)
}

/// Maçın dokunuş ayarı (JS ALAN_DOKUNUS): bugün kapalı (maç taşımadı)
pub fn mac_dokunus() -> Option<Dokunus> {
    None
}
