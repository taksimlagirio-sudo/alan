//! ALAN etkisi: oyuncuların sahaya yaydığı alan ve topun bir noktadaki K değeri
//! (core.js: infl, amOf, dimS, Ksec, pack/KpkSec, KcT/_kc, makeGrid/gridS, K).
//!
//! JS'te K dört kipte çalışır: düz liste (seçimli, `Ksec`), paketli (`KpkSec`, aynı sonuç),
//! yarım birimlik ızgara önbelleği (`_kc`: seçimsiz takım toplamı, ızgara köşesinde) ve
//! tam birimlik ızgara (`_g`, çift doğrusal). Burada hepsi `Kaynak` üstünde.

use crate::mat::{exp, tanh};
use crate::saha::*;

/// Bir oyuncunun alan kaynağı olarak görünüşü.
#[derive(Clone, Copy, Debug, Default)]
pub struct Etki {
    pub x: f64,
    pub y: f64,
    /// yarıçap (JS: `p.R || 8`)
    pub r: f64,
    /// şiddet (JS: `p.D ?? 1`)
    pub d: f64,
    pub koni: bool,
    pub fx: f64,
    pub fy: f64,
    pub team: u8,
    /// ALAN seçimi açık mı (`amA`) ve yönü (`am`: +1 kendi alanı, −1 yavaşlatma, 0 yok)
    pub am_a: bool,
    pub am: f64,
    /// bar (JS: `p.en`, yoksa 1)
    pub en: f64,
}

impl Etki {
    #[inline(always)]
    pub fn infl(&self, x: f64, y: f64) -> f64 {
        let dx = x - self.x;
        let dy = y - self.y;
        let r = self.r;
        if dx * dx + dy * dy > r * r * 9.0 {
            return 0.0;
        }
        let q = if self.koni {
            let al = dx * self.fx + dy * self.fy;
            let pe = -dx * self.fy + dy * self.fx;
            let a = if al > 0.0 { al / (1.4 * r) } else { al / (0.6 * r) };
            let pr = pe / r;
            a * a + pr * pr
        } else {
            (dx * dx + dy * dy) / (r * r)
        };
        self.d * exp(1.2 * -q)
    }
    /// JS amOf: seçimi açıksa kendi seçimi, değilse rakip (barı %15'ten büyük) yavaşlatır.
    #[inline(always)]
    pub fn am_of(&self, team: u8) -> f64 {
        if self.am_a {
            self.am
        } else if self.team != team && self.en > 0.15 {
            -1.0
        } else {
            0.0
        }
    }
}

#[inline(always)]
pub fn dim_s(s: f64) -> f64 {
    if s <= 1.0 {
        s
    } else if s <= 2.0 {
        1.0 + 0.5 * (s - 1.0)
    } else {
        1.5 + 0.25 * (s - 2.0)
    }
}

/// Seçimli K (JS Ksec ve KpkSec): topun takımı için kendi alanı (+) ve yavaşlatma (−) ayrı ayrı sönümlenip farkı.
pub fn k_sec(src: &[Etki], x: f64, y: f64, team: u8) -> f64 {
    let (mut sp, mut sn) = (0.0f64, 0.0f64);
    for p in src {
        let a = p.am_of(team);
        if a == 0.0 || a != a {
            continue;
        }
        let f = p.infl(x, y);
        if a > 0.0 {
            sp += f;
        } else {
            sn += f;
        }
    }
    tanh(dim_s(sp) - dim_s(sn))
}

/// Takım toplamı (seçimsiz; 0. takım +, 1. takım −) bir noktada.
#[inline(always)]
fn takim_toplam(src: &[Etki], x: f64, y: f64) -> f64 {
    let mut v = 0.0;
    for p in src {
        v += (if p.team == 0 { 1.0 } else { -1.0 }) * p.infl(x, y);
    }
    v
}

/// Yarım birimlik ızgara önbelleği (JS `_kc` + `KcT`): değer ızgara köşesinde hesaplanır (sıradan bağımsız).
#[derive(Clone)]
pub struct KOnbellek {
    v: Vec<f64>,
    f: Vec<u8>,
    dis: std::collections::HashMap<i64, f64>,
}
impl KOnbellek {
    pub fn yeni() -> Self {
        KOnbellek { v: vec![0.0; GX1 * GY1], f: vec![0; GX1 * GY1], dis: Default::default() }
    }
}

/// Tam birimlik ızgara (JS makeGrid, f32 saklanır).
pub fn izgara_kur(src: &[Etki]) -> Vec<f32> {
    let mut g = vec![0f32; GW1 * GH1];
    for j in 0..=(H as usize) {
        for i in 0..=(W as usize) {
            let mut s = 0.0f64;
            for p in src {
                let dx = i as f64 - p.x;
                let dy = j as f64 - p.y;
                if dx * dx + dy * dy > 400.0 {
                    continue;
                }
                s += (if p.team == 0 { 1.0 } else { -1.0 }) * p.infl(i as f64, j as f64);
            }
            g[j * GW1 + i] = s as f32;
        }
    }
    g
}

pub fn izgara_oku(g: &[f32], x: f64, y: f64) -> f64 {
    let x = crate::mat::kis(x, 0.0, GXM);
    let y = crate::mat::kis(y, 0.0, GYM);
    let i = x as usize;
    let j = y as usize;
    let fx = x - i as f64;
    let fy = y - j as f64;
    let a = g[j * GW1 + i] as f64;
    let b = g[j * GW1 + i + 1] as f64;
    let c = g[(j + 1) * GW1 + i] as f64;
    let d = g[(j + 1) * GW1 + i + 1] as f64;
    a + (b - a) * fx + (c - a) * fy + (a - b - c + d) * fx * fy
}

/// K'nin kaynağı: hangi kipte okunacağıyla birlikte.
pub enum Kaynak<'a> {
    /// düz liste ya da paket: seçimli K
    Sec(&'a [Etki]),
    /// yarım birimlik önbellekli takım toplamı
    Onbellek(&'a [Etki], &'a mut KOnbellek),
    /// tam birimlik ızgara
    Izgara(&'a [f32]),
}

/// Topun (team takımının) (x, y)'deki K değeri: + kendi alanında hızlanır, − rakip alanında yavaşlar.
pub fn k(src: &mut Kaynak, x: f64, y: f64, team: u8) -> f64 {
    match src {
        Kaynak::Sec(s) => k_sec(s, x, y, team),
        Kaynak::Onbellek(s, c) => {
            let i = (2.0 * x + 0.5) as i32;
            let j = (2.0 * y + 0.5) as i32;
            let v = if i >= 0 && (i as usize) <= GX1 - 1 && j >= 0 && (j as usize) <= GY1 - 1 {
                let ci = i as usize * GY1 + j as usize;
                if c.f[ci] != 0 {
                    c.v[ci]
                } else {
                    let v = takim_toplam(s, i as f64 / 2.0, j as f64 / 2.0);
                    c.v[ci] = v;
                    c.f[ci] = 1;
                    v
                }
            } else {
                let key = 1000 * i as i64 + j as i64;
                if let Some(v) = c.dis.get(&key) {
                    *v
                } else {
                    let v = takim_toplam(s, i as f64 / 2.0, j as f64 / 2.0);
                    c.dis.insert(key, v);
                    v
                }
            };
            tanh(if team == 0 { v } else { -v })
        }
        Kaynak::Izgara(g) => tanh(izgara_oku(g, x, y) * if team == 0 { 1.0 } else { -1.0 }),
    }
}
