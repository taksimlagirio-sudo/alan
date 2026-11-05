//! Dünyanın varlıkları: gerçek oyuncu, görülen hayalet ve kopya.
//!
//! JS'te yerleşim üç tür nesneyle çalışır: gerçek oyuncu (`m.ps[i]`), görülen hayalet (`seen`: `{a, x, y,
//! R, D, team, role, press, ref, gvx, gvy}`) ve kopya (`{...p, x, y, vx, vy}`). Rust'ta hepsi `Varlik`:
//! konum/hız varlığın kendinde, yetenek/sprint/tepki için "beden" (gerçek oyuncu ya da ayrı kurulmuş bir
//! `Oyuncu`), `r` = JS `o.ref`, `kim` = nesnenin kimin kopyası olduğu (JS `o === q`, `o.id === d.id`).
#![allow(non_snake_case)]

use std::sync::Arc;

use crate::model::{Oyuncu, Pid};

/// Varlığın yetenek/sprint/tepki için okunan bedeni.
#[derive(Clone, Debug)]
pub enum Beden {
    /// `m.ps` içindeki gerçek oyuncu
    Ps(Pid),
    /// ayrı kurulmuş beden (hayalet: barsız, sprint evresiz; kopya: alanları değişmiş oyuncu)
    Ayri(Arc<Oyuncu>),
}

/// Dünyadaki bir varlık (JS: oyuncu, hayalet ya da kopya).
#[derive(Clone, Debug)]
pub struct Varlik {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    /// hayaletin görülen hızı (JS gvx/gvy; gerçek oyuncuda yok)
    pub gvx: Option<f64>,
    pub gvy: Option<f64>,
    pub team: u8,
    pub bekci: bool,
    pub press: bool,
    /// JS `o.id` (hayalette yok: NaN, hiçbir şeye eşit değil)
    pub id: f64,
    /// JS `o.ref`: hayaletin gerçek oyuncusu
    pub r: Option<Pid>,
    /// nesne kimliği: bu varlık hangi oyuncunun kendisi ya da kopyası (hayalette None)
    pub kim: Option<Pid>,
    pub beden: Beden,
}

impl Varlik {
    /// Gerçek oyuncunun kendisi (JS: dünyada `m.ps[i]`).
    pub fn oyuncu(pi: Pid, p: &Oyuncu) -> Varlik {
        Varlik { x: p.x, y: p.y, vx: p.vx, vy: p.vy, gvx: None, gvy: None, team: p.takim(), bekci: p.bekci(), press: p.press, id: p.id, r: None, kim: Some(pi), beden: Beden::Ps(pi) }
    }
    /// Kopya (JS `{...p, x, y, vx, vy}`): bedeni konumu/hızı değişmiş ayrı bir oyuncu (varış tablosu bu
    /// hızla okunur); kimliği aynı oyuncu.
    pub fn kopya(pi: Pid, p: &Oyuncu, x: f64, y: f64, vx: f64, vy: f64) -> Varlik {
        let mut q = p.clone();
        q.x = x;
        q.y = y;
        q.vx = vx;
        q.vy = vy;
        let mut v = Varlik::oyuncu(pi, p);
        v.x = x;
        v.y = y;
        v.vx = vx;
        v.vy = vy;
        v.beden = Beden::Ayri(Arc::new(q));
        v
    }
    /// Yetenek/sprint/tepki bedeni.
    #[inline]
    pub fn govde<'a>(&'a self, ps: &'a [Oyuncu]) -> &'a Oyuncu {
        match &self.beden {
            Beden::Ps(i) => &ps[*i],
            Beden::Ayri(o) => o,
        }
    }
    /// JS `o.ref || o` (varış tablosunun bedeni: hız gerçek oyuncunun).
    #[inline]
    pub fn r0<'a>(&'a self, ps: &'a [Oyuncu]) -> &'a Oyuncu {
        match self.r {
            Some(i) => &ps[i],
            None => self.govde(ps),
        }
    }
}

/// Hayaletin bedeni (JS seen çıktısı: `{a, R, D, team, role, press}`; id/bar/sprint evresi yok).
pub fn hayal(q: &Oyuncu) -> Oyuncu {
    let mut o = Oyuncu::bos();
    o.a = q.a.clone();
    o.yet = q.yet.clone();
    o.R = q.R;
    o.D = q.D;
    o.team = q.team;
    o.role = q.role.clone();
    o.press = q.press;
    o.id = f64::NAN;
    o.x = q.x;
    o.y = q.y;
    o
}
