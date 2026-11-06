//! Kafanın bağlamı: JS kapanışının (decide.js IIFE) değişkenleri açık bir yapıda.
//!
//! - `D` (JS `AlanDecide.D`): ortak durum, `model::KafaD`.
//! - Modül düzeyi önbellekler (JS `UCUS`, `SUTY`, `GE`/`GEk`, `RNB`): `RefCell` içinde; değerleri
//!   girdilerinden belirlenir (JS ile aynı anahtarlar), yalnız `GE`/`RNB` JS'teki gibi ilk hesaplayanın
//!   değerini tutar (JS davranışı; bkz. NOT1.md).
//! - Fonksiyon özellikleri (JS `keepP.son`, `kesisK.son`): `Cell`.
//! - Kafa2'nin kurduğu sahiplik haritası (JS `SAH_F`): `SAH_F`.
//! - Dış dünya: `dis` ([`Dis`]).
//! - Kafa2'nin durumu: `mac` (JS `D._mac`), `prT` (JS `m._prT`), `k2` (önbellekler), `srcC_kc`.
#![allow(non_snake_case)]

use crate::kafa::gecici_dis1::KuralErisim as _;
use super::gecici_dis1::{Dis, Ik, IK_JS};
use super::kafa1_param::d_js;
use super::kafa2_baglam::{Kafa2Durum, MacGor, PrT};
use crate::alan::KOnbellek;
use crate::model::{KafaD, Oyuncu};
use crate::oyuncu::{tepki, Tepki, TepkiS};
use crate::temas::Kural;
use std::cell::{Cell, RefCell};

use std::sync::Arc;

/// Uçuşun yol tablosu (JS ucusYol dönüşü).
#[derive(Clone, Debug)]
pub struct UcusYol {
    pub v0: f64,
    pub cum: Vec<f64>,
    pub T: f64,
}

/// Sahiplik haritası (JS `SAH_F(team, x, y)`; kafa2'nin sahiplikKur'u kurar, `oku` ile okunur).
pub type SahF = Arc<super::kafa2_zemin::Sahiplik>;

pub struct Kafa {
    pub D: KafaD,
    /// geçici bağlam (JS `D`'nin `_` alanları)
    pub bag: crate::model::KafaBag,
    pub dis: Arc<dyn Dis>,
    pub SAH_F: Option<SahF>,
    /// JS `keepP.son`: tutmanın son ilerleyişi (ilk kesişmeye dek)
    pub keepP_son: Cell<f64>,
    /// JS `kesisK.son`
    pub kesisK_son: Cell<f64>,
    /// sendelemeyle biten benzetimde topu geri almaya dek beklenen süre (tik); keepP_gec: ilk kesişmenin ki
    pub kesisK_gec: Cell<f64>,
    pub keepP_gec: Cell<f64>,
    /// JS `D._mac` (kafa2'nin okuduğu maç görünümü; maç döngüsü kararın önünde doldurur)
    pub mac: Option<MacGor>,
    /// JS `m._prT` çıktısı (kafa2 yazar; maç ve kafa1'in girEsik'i `Dis::prT` ile okur, bkz. `PrT::d1`)
    pub prT: Option<PrT>,
    /// kafa2'nin modül düzeyi önbellekleri ve bayrakları
    pub k2: Kafa2Durum,
    /// decide0'ın `srcC._kc`'si (kararın K önbelleği; launchFor/ucusSure/bankSure/bankFor buna bakar)
    pub srcC_kc: Option<RefCell<KOnbellek>>,
    /// önbellekler, ömürlerine göre (MIMARI.md 3e-4)
    pub ob: super::onbellek::Onbellek,
}

impl Kafa {
    pub fn yeni(dis: Arc<dyn Dis>) -> Kafa {
        Kafa {
            D: d_js(),
            bag: Default::default(),
            dis,
            SAH_F: None,
            keepP_son: Cell::new(f64::INFINITY),
            kesisK_son: Cell::new(0.0),
            kesisK_gec: Cell::new(0.0),
            keepP_gec: Cell::new(0.0),
            mac: None,
            prT: None,
            k2: Kafa2Durum { IMK_INAN_T: 60.0, ..Default::default() },
            srcC_kc: None,
            ob: Default::default(),
        }
    }
    /// JS `D._tick || 0`
    #[inline(always)]
    pub fn tick(&self) -> f64 {
        super::kafa1_js::yao(self.bag._tick, 0.0)
    }
    /// JS `D._tick ?? 0`
    #[inline(always)]
    pub fn tick0(&self) -> f64 {
        self.bag._tick.unwrap_or(0.0)
    }
    /// Maçın IK'sı ya da JS değerleri (ikKafa kapalıysa çağıran ayrıca sorar).
    pub fn ik(&self) -> Ik {
        self.dis.IK().unwrap_or(IK_JS)
    }
    pub fn yavas(&self) -> f64 {
        match self.dis.IK() {
            Some(k) => k.yavas,
            None => 0.35,
        }
    }
    /// Çekirdeğin kuralı (sprint katı maçtan).
    pub fn kural(&self) -> KuralK<'_> {
        KuralK(&*self.dis)
    }
    /// `.17 + .006 * ALAN_HZ(p)` (Enerji 3.0: ham Hız)
    #[inline(always)]
    pub fn spd(&self, p: &Oyuncu) -> f64 {
        0.17 + 0.006 * p.hz()
    }
    /// `(.17 + .006 * ALAN_HZ(p)) * ALAN_SPRE(p)`
    #[inline(always)]
    pub fn vS(&self, p: &Oyuncu) -> f64 {
        self.spd(p) * self.dis.ALAN_SPRE(p)
    }
    /// JS ALAN_TEPKI(p, k)
    pub fn TEPKI(&self, p: &Oyuncu, k: Tepki) -> f64 {
        let s: TepkiS = self.dis.tepki_s();
        tepki(p, k, &s)
    }
    /// JS ALAN_AG(p)
    pub fn AG(&self, p: &Oyuncu) -> f64 {
        crate::yetenek::ag(&p.govde.yet, p.den, p.bekci(), self.yavas())
    }
    /// JS ALAN_AH(p)
    pub fn AH(&self, p: &Oyuncu) -> f64 {
        crate::yetenek::ah(&p.govde.yet)
    }
    /// JS ALAN_DURUM.dengeK(den)
    pub fn dengeK(&self, den: Option<f64>) -> f64 {
        crate::yetenek::denge_k(den, self.yavas())
    }
    /// JS ALAN_DURUM.har(p): hızın tepe hızına oranı
    pub fn har(&self, p: &Oyuncu) -> f64 {
        crate::mat::jmin(1.0, crate::mat::hypot(super::kafa1_js::z0(p.vx), super::kafa1_js::z0(p.vy)) / (self.spd(p) * crate::yetenek::spr_h(p.govde.yet.hiz)))
    }
    /// JS `K.menzil(p, Q)`
    pub fn menzil(&self, p: &Oyuncu) -> f64 {
        crate::temas::menzil(p, self.yavas())
    }
}

/// Çekirdeğin kuralı olarak dış dünya (ALAN_SPR maçtan).
pub struct KuralK<'a>(pub &'a dyn Dis);
impl<'a> Kural for KuralK<'a> {
    fn spr(&self, p: &Oyuncu) -> f64 {
        self.0.ALAN_SPR(p)
    }
    fn ik_yavas(&self) -> f64 {
        match self.0.IK() {
            Some(k) => k.yavas,
            None => 0.35,
        }
    }
    fn tepki_s(&self) -> TepkiS {
        self.0.tepki_s()
    }
}
