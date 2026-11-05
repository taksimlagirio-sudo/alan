//! Yerleşimin bağlamı (JS `m` + modülün kendi durumu) ve "dünya" varlıkları.
//!
//! JS'te yerleşim üç tür nesneyle çalışır: gerçek oyuncu (`m.ps[i]`), görülen hayalet (`seen`: `{a, x, y,
//! R, D, team, role, press, ref, gvx, gvy}`) ve kopya (`{...p, x, y, vx, vy}`). Rust'ta hepsi `Varlik`:
//! konum/hız varlığın kendinde, yetenek/sprint/tepki için "beden" (gerçek oyuncu ya da ayrı kurulmuş bir
//! `Oyuncu`), `r` = JS `o.ref`, `kim` = nesnenin kimin kopyası olduğu (JS `o === q`, `o.id === d.id`).
#![allow(non_snake_case)]


use std::sync::Arc;

use crate::model::{KafaD, Mac, Oyuncu, Pid, TopRef};
use crate::temas::{Kural, TopT};

use super::ucus::Fr;

/// Dünyanın varlıkları dünya katmanında (MIMARI.md Aşama 4c); eski yolları buradan da görünür.
pub use crate::dunya::varlik::{hayal, Beden, Varlik};


/// JS `m.sh`: yerleşimin maç boyu durumu (model.rs'teki `JMSh` taslağının doğru tipli karşılığı).
#[derive(Clone, Debug, Default)]
pub struct Sh {
    pub fr: Option<Arc<Fr>>,
    pub frT: f64,
    pub frB: Option<TopRef>,
    pub att: Option<u8>,
    pub aday: Option<u8>,
    pub adayT: f64,
    pub gain: f64,
    /// JS `sh.ball` (top kimliği; `undefined` → None, `null` → Some(None))
    pub ball: Option<Option<TopRef>>,
    pub ballT: f64,
    pub defl: f64,
    pub yr: Option<Yr>,
    pub gs: Option<Gs>,
    pub yvBas: Option<(Pid, f64)>,
}
#[derive(Clone, Copy, Debug, Default)]
pub struct Yr {
    pub gain: f64,
    pub dt: u8,
    pub on: bool,
}
#[derive(Clone, Copy, Debug, Default)]
pub struct Gs {
    pub t: f64,
    pub dt: u8,
    pub on: bool,
    pub onP: bool,
    pub oy: bool,
}

/// Kafanın kayıp-yeri işlevi (JS `D.kayipYer(...)` dönüşü: `(x, y) → değer`).
/// Kayıp haritası (x, y) → değer. Çağrı anında maçı alır: değerlendirme maçın kafasıyla yapılır
/// (iş parçacığına özel kafa aranmaz; MIMARI.md Aşama 3).
/// Yerleşimin kafaya ulaşması (MIMARI.md 3.8): maçın kafası. Maç katmanı bağlamı (`Mc`) kurarken verir; temel
/// maç modeli kafayı tanımaz. Yoksa (kurulum, kafasız sınama) None.
pub type KafaEl<'a> = Option<&'a std::cell::RefCell<crate::kafa::Kafa>>;
pub type KayipF = Arc<dyn Fn(KafaEl, f64, f64) -> f64 + Send + Sync>;

/// JS modül değişkeni `_kpC`: topu kaybetme tartısı (position'da kurulur, teamV okur).
#[derive(Clone)]
pub struct KpC {
    pub L: KayipF,
    pub LE: f64,
    /// JS `Hr`: taşıyıcının E'deki kopyası
    pub Hr: Oyuncu,
    pub kH: f64,
    pub mut_: i32,
    /// JS `kc`: dünya (görenin sırası) × T → keepP
    pub kc: crate::HizliMap<(Pid, u64), f64>,
}

/// Yerleşimin kendi durumu: `m.sh`, görüş önbelleği (`m._sn`), JS modül değişkenleri (`_kpC`, `_tbC`),
/// `m._kayL`, `m._acikK`, `p._bkCik`.
#[derive(Clone, Default)]
pub struct SekilDurum {
    pub sh: Sh,
    /// JS `m._sn` / `m._snT`: görenin sırası → gördükleri
    pub sn: Vec<Option<Arc<Vec<Varlik>>>>,
    pub snT: Option<f64>,
    /// hayalet bedenleri (oyuncu sırasıyla; aynı tikte bir kez kurulur)
    pub hy: Vec<Option<Arc<Oyuncu>>>,
    pub hyT: Option<f64>,
    pub kpC: Option<KpC>,
    /// JS `_tbC`: (tik, p.id, H.id) → teamV tabanı
    pub tbC: Option<((u64, u64, u64), f64)>,
    /// JS `m._kayL`
    pub kayL: Option<(String, KayipF)>,
    /// JS `m._acikK`
    pub acikK: Option<(String, Vec<(f64, f64, f64)>)>,
    /// JS `p._bkCik` (oyuncu sırasıyla)
    pub bkCik: Vec<Option<TopRef>>,
    /// JS `b._pc` (hayali maçta topun yolu): top, tik, yol
    pub pc: Option<(TopRef, f64, Arc<Vec<super::ucus::Nokta>>)>,
    /// seyrek yerleşim: hücumcunun son tam hesabı (tik, tx, ty, job, sprint), oyuncu sırasıyla
    pub seyrek: Vec<Option<(f64, f64, f64, Option<Arc<str>>, bool)>>,
}

/// Yerleşimin bağlamı: maç, oyuncular, top, kendi durumu, maçın kuralı (sprint katı), kafanın sabitleri.
pub struct Mc<'a> {
    pub m: &'a mut Mac,
    /// JS `m.ps`
    pub ps: &'a mut Vec<Oyuncu>,
    /// JS `m.ball`
    pub ball: Option<&'a mut TopT>,
    pub ek: &'a mut SekilDurum,
    /// maçın kafası (yerleşimin kafaya sorduğu değerlendirmeler için)
    pub kafa: KafaEl<'a>,
    pub k: &'a dyn Kural,
    /// JS `AlanDecide.D`
    pub d: &'a KafaD,
    /// JS `m._lite` (kafanın hayali maçı)
    pub lite: bool,
    /// JS `m.r` (maçın zarı); yoksa 0,5
    pub rnd: Option<&'a mut dyn FnMut() -> f64>,
    /// Kafanın karar tiki (JS `AlanDecide.bag._tick`): gerçek maçta maçın tiki, kafanın ileri bakışında kararın
    /// verildiği tik (benzetilen tik `m.tick`'ten farklı). Eskiden iş parçacığına özel bir değişkendi (D_TICK).
    pub dtik: Option<f64>,
}

impl<'a> Mc<'a> {
    /// Maçın kendi işlevini çağır: oyuncular ve top bir anlığına maça geri konur (JS'te ikisi de `m`
    /// üstünde; burada yerleşim çalışırken ayrı tutuluyorlar).
    pub fn mac_ile<T>(&mut self, f: impl FnOnce(&mut Mac) -> T) -> T {
        std::mem::swap(self.ps, &mut self.m.ps);
        if let (Some(b), Some(mb)) = (self.ball.as_deref_mut(), self.m.ball.as_mut()) {
            std::mem::swap(b, &mut mb.tt);
        }
        let v = f(self.m);
        std::mem::swap(self.ps, &mut self.m.ps);
        if let (Some(b), Some(mb)) = (self.ball.as_deref_mut(), self.m.ball.as_mut()) {
            std::mem::swap(b, &mut mb.tt);
        }
        v
    }
    #[inline]
    pub fn tick(&self) -> f64 {
        self.m.tick
    }
    #[inline]
    pub fn holder(&self) -> Option<Pid> {
        self.m.holder
    }
    /// topun kimliği (JS nesne kimliği)
    #[inline]
    pub fn top_id(&self) -> Option<TopRef> {
        self.ball.as_ref().map(|b| b.t.kimlik)
    }
    /// JS `m.r ? m.r() : .5`
    pub fn zar(&mut self) -> f64 {
        match self.rnd.as_mut() {
            Some(r) => r(),
            None => 0.5,
        }
    }
    /// Hayalet bedeni (aynı tikte önbellekli).
    pub fn hayal(&mut self, j: Pid) -> Arc<Oyuncu> {
        let t = self.m.tick;
        if self.ek.hyT != Some(t) || self.ek.hy.len() != self.ps.len() {
            self.ek.hy = vec![None; self.ps.len()];
            self.ek.hyT = Some(t);
        }
        if let Some(h) = &self.ek.hy[j] {
            return h.clone();
        }
        let h = Arc::new(hayal(&self.ps[j]));
        self.ek.hy[j] = Some(h.clone());
        h
    }
    /// Gerçek oyuncuların dünyası (JS `m.ps` dünya olarak).
    pub fn dunya(&self) -> Vec<Varlik> {
        self.ps.iter().enumerate().map(|(i, p)| Varlik::oyuncu(i, p)).collect()
    }
}

impl std::fmt::Debug for SekilDurum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SekilDurum").field("snT", &self.snT).field("hyT", &self.hyT).finish_non_exhaustive()
    }
}
