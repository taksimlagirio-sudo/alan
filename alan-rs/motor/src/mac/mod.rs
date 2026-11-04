//! Maç döngüsü (match.js): kurulum, taktik, enerji, hareket, ikili, Kuyu, vuruş, kafa günlüğü, ana döngü.
//!
//! JS'te maç `m` nesnesi ve modül düzeyindeki değişkenler (IKT, IKM, IKL, _mt, _mH, _mM, `self.__mH`)
//! burada açık: `Mac` (model) + `MacEk` (taslakta olmayan maç alanları) + işlevlere verilen bağlamlar.
//! Kafa (decide.js) ve yerleşim (shape.js) işlevleri `gecici_dis`'te.
#![allow(non_snake_case, clippy::too_many_arguments)]

pub mod dongu;
pub mod enerji;
pub mod gecici_dis;
pub mod hareket;
pub mod ikili;
pub mod kafa_bag;
pub mod kayit;
pub mod kurulum;
pub mod kuyu;
pub mod notlar;
pub mod tac;
pub mod kafa_dis;
pub mod vurus;

pub use dongu::{run, step, take};
pub use enerji::{EnSayac, MacKural};
pub use kurulum::{assignRoles, createMatch, dizSlots, dizUygula, kickoff, Diz, DizV, KurSecenek};
pub use tac::{macEff, tacEff, MacW, MacZ, Tac};

use crate::mat::{dogru, hypot};
use crate::model::{Mac, Oyuncu, Pid};
use crate::saha::{H, TPS, W};

/// JS ALAN_SAHA.KX, KY (saha ölçeği: 100 × 50'ye göre)
pub const KX: f64 = W / 100.0;
pub const KY: f64 = H / 50.0;

/// Maçın zar dizisinin varsayılanı (Mac Default için; gerçek tohum createMatch'te verilir).
impl Default for crate::rng::Rng {
    fn default() -> Self {
        crate::rng::Rng::yeni(1)
    }
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
#[derive(Clone, Debug, Default)]
pub struct KenarW {
    pub x: f64,
    pub y: f64,
    pub sonra: Option<(f64, f64)>,
    pub until: f64,
}

/// JS `m.fake`.
#[derive(Clone, Debug, Default)]
pub struct Fake {
    pub team: f64,
    pub hid: f64,
    pub k: String,
    pub to: (f64, f64),
    pub t0: f64,
    pub until: f64,
    pub react: f64,
    /// aldananların id'leri (JS Set: ekleme sırası)
    pub fooled: Vec<f64>,
    pub tg: Vec<(f64, (f64, f64))>,
    pub ra: Vec<(f64, f64)>,
    pub ax: Option<f64>,
    pub ay: Option<f64>,
}

/// Oyuncunun ham özellikleri (JS `p.a`), ekleme sırasıyla (rastgele kadro sırayı kullanır).
#[derive(Clone, Debug, Default)]
pub struct HamA(pub Vec<(String, HamV)>);

#[derive(Clone, Debug)]
pub enum HamV {
    S(f64),
    M(String),
}

impl HamA {
    pub fn koy(&mut self, k: &str, v: HamV) {
        if let Some(e) = self.0.iter_mut().find(|e| e.0 == k) {
            e.1 = v;
        } else {
            self.0.push((k.to_string(), v));
        }
    }
    pub fn ham(&self) -> crate::yetenek::Ham {
        let mut h = crate::yetenek::Ham::default();
        for (k, v) in &self.0 {
            match v {
                HamV::S(x) => {
                    h.v.insert(k.clone(), *x);
                }
                HamV::M(s) => {
                    if k == "taraf" {
                        h.taraf = Some(s.clone());
                    }
                }
            }
        }
        h
    }
}

/// Taslakta olmayan maç alanları.
#[derive(Clone, Debug, Default)]
pub struct MacEk {
    /// JS `m._lite`: kafanın hafif kopyası (olay yazılmaz, sayaç tutulmaz)
    pub _lite: bool,
    /// JS `m._ic`
    pub _ic: bool,
    /// JS `m._ucus`
    pub _ucus: bool,
    /// JS `m._kafa` (kafaAc ile açılır)
    pub _kafa: Option<kafa_bag::KafaGunluk>,
    /// JS `self.__mH`: son karar veren taşıyıcı (team, id) — sprint katı bunu okur
    pub __mH: Option<(f64, f64)>,
    /// JS `m.fake`
    pub fake: Option<Fake>,
    /// JS `m._perdeQ`: id → tik
    pub _perdeQ: Vec<(f64, f64)>,
    /// JS `m.kosulAkt`
    pub kosulAkt: Option<[Vec<String>; 2]>,
    /// oyuncuların ham özellikleri (JS `p.a`, sırayla); Pid sırası
    pub ham: Vec<HamA>,
    /// top kimliği sayacı (JS nesne kimliği)
    pub topSay: u64,
    /// kafa günlüğü seçenek kimliği sayacı
    pub oSay: u64,
    /// yerleşimin maç üstündeki durumu (JS m._sn, m._kayL, m.sh …)
    pub sekil: crate::sekil::varlik::SekilDurum,
    /// kafanın pres tartısı (JS m._prT): kafa yazar, sonraki kararda kafa okur
    pub prT_k: Option<crate::kafa::gecici_dis1::PrT>,
}

// ---- küçük JS yardımcıları ----

#[inline(always)]
pub fn cl(v: f64, a: f64, b: f64) -> f64 {
    if v < a {
        a
    } else if v > b {
        b
    } else {
        v
    }
}
#[inline(always)]
pub fn dirOf(t: f64) -> f64 {
    if t == 0.0 {
        1.0
    } else {
        -1.0
    }
}
#[inline(always)]
pub fn ownX(t: f64) -> f64 {
    if t == 0.0 {
        0.0
    } else {
        W
    }
}
/// JS `SN()`: saniyedeki tik.
#[inline(always)]
pub fn SN() -> f64 {
    TPS
}
/// JS `x || 0`
#[inline(always)]
pub fn or0(x: f64) -> f64 {
    if dogru(x) {
        x
    } else {
        0.0
    }
}
/// JS `o || 0` (Option)
#[inline(always)]
pub fn oo(x: Option<f64>) -> f64 {
    match x {
        Some(v) if dogru(v) => v,
        _ => 0.0,
    }
}
/// JS `a > b` (a undefined → false)
#[inline(always)]
pub fn gt(a: Option<f64>, b: f64) -> bool {
    matches!(a, Some(v) if v > b)
}
#[inline(always)]
pub fn mesafe(p: &Oyuncu, q: &Oyuncu) -> f64 {
    hypot(q.x - p.x, q.y - p.y)
}
/// Oyuncunun kimliği (team, id): Kural yalnız `&Oyuncu` görür.
#[inline(always)]
pub fn kim(p: &Oyuncu) -> (f64, f64) {
    (p.team, p.id)
}
pub fn bas(j: &str, ler: &[&str]) -> bool {
    ler.iter().any(|x| j.starts_with(x))
}
pub fn icer(j: &str, ler: &[&str]) -> bool {
    ler.iter().any(|x| j.contains(x))
}
pub fn job(p: &Oyuncu) -> &str {
    p.job.as_deref().unwrap_or("")
}

/// JS `ev(m, type, team, text)`: olay günlüğü (en yenisi başta, en çok 120).
pub fn ev(m: &mut Mac, tip: &str, team: f64, text: String) {
    let s = (m.tick / SN()).floor() as i64;
    m.events.insert(0, Olay { t: format!("{:02}:{:02}", s / 60, s % 60), r#type: tip.to_string(), team, text });
    if m.events.len() > 120 {
        m.events.pop();
    }
}

/// İki ayrı oyuncuya aynı anda yazma.
pub fn iki(ps: &mut [Oyuncu], i: usize, j: usize) -> (&mut Oyuncu, &mut Oyuncu) {
    assert!(i != j);
    if i < j {
        let (a, b) = ps.split_at_mut(j);
        (&mut a[i], &mut b[0])
    } else {
        let (a, b) = ps.split_at_mut(i);
        (&mut b[0], &mut a[j])
    }
}

/// id'den oyuncu (JS `m.ps.find(p => p.id === id)`).
pub fn idden(m: &Mac, id: f64) -> Option<Pid> {
    m.ps.iter().position(|p| p.id == id)
}

/// JS `x.toFixed(n)` sonra `+` (sayıya): yarıda yukarı (tam ikili değerle).
pub fn to_fixed(x: f64, n: usize) -> f64 {
    let s = format!("{:.*}", n, x);
    let mut v: f64 = s.parse().unwrap_or(x);
    let k = 10f64.powi(n as i32);
    let y = x.abs() * k;
    if y.fract() == 0.5 && y < 1e15 {
        let up = (y.floor() + 1.0) / k;
        v = if x < 0.0 { -up } else { up };
    }
    v
}

/// JS `String(x)` (sayı), olay yazıları için.
pub fn js_num(x: f64) -> String {
    if x == x.trunc() && x.abs() < 1e21 {
        format!("{}", x as i64)
    } else {
        format!("{}", x)
    }
}

/// JS `null != b.gvx`: top tutulunca (temas) gelen hızı saklanır.
pub fn gv_var(b: &crate::model::MacTop) -> bool {
    b.tt.t.holder.is_some()
}
