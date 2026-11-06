//! Dünya katmanı (MIMARI.md 2.1, 2.3): dünyanın gösterimleri ve aralarındaki çeviriler.
//!
//! Bugün iki gösterim var: yerleşimin görünümü (`Varlik`: konum/hız kendinde, beden maçtaki oyuncuya
//! başvuru ya da ayrı kopya; kimlik `kim: Option<Pid>`) ve kafanın dünyası (`Dunya` = `Vec<Op>`, `Op` =
//! paylaşımlı `Oyuncu`). Çeviriler yalnız burada:
//!
//! | çeviri | ne | kimlik |
//! |---|---|---|
//! | `op_dunya` | maçın oyuncuları → kafanın dünyası | her çağrıda yeni nesneler |
//! | `ow_kur` / `ow` | yerleşimin görünümü → kafanın dünyası (bedenin kopyası + görünümün konumu) | her çeviride yeni nesneler; `ow_paylas` kapsamında aynı dünyanın çevirisi paylaşılır |
//! | `dunya_kopya` | kafanın dünyası → düz oyuncu listesi (yerleşimin hizmetleri için) | aynı dünya (aynı nesneler) için önceki kopya |
//!
//! Kimlik kuralı (MIMARI.md 3.2): kafanın karşılaştırmaları nesne kimliğine dayanır; bir çevirinin
//! nesneleri başka bir çevirininkilerle aynı **değildir**. Çeviriler arasında nesne paylaşmak davranışı
//! değiştirir (ör. `dolanma`: kopya kendi aslını atlamaya başlar); bu yüzden yapılmaz.
//! Bağımlılık: yalnız temel katman ve `model` (katman kuralı `sina/katman.sh`).
#![allow(non_snake_case)]

use std::sync::Arc;

use crate::model::Oyuncu;

pub mod varlik;
pub use varlik::{hayal, Beden, Varlik, Yerde};

pub use crate::dunya_tip::Op;
/// Kafanın dünyası.
pub use crate::dunya_tip::Dunya;

/// Dünyayı kafanın biçimine çevir.
pub fn op_dunya(ps: &[Oyuncu]) -> Dunya {
    ps.iter().map(|p| Arc::new(p.clone())).collect()
}

/// Yerleşimin dünyasını kafanın dünyasına çevir (varlık: konum/hız kendinde, beden oyuncudan).
pub fn ow_kur(w: &[Varlik], oy: &[Oyuncu]) -> Vec<Op> {
    w.iter()
        .map(|v| {
            let mut o = v.govde(oy).clone();
            o.x = v.x;
            o.y = v.y;
            o.vx = v.vx;
            o.vy = v.vy;
            std::sync::Arc::new(o)
        })
        .collect()
}
thread_local! {
    /// ow_paylas kapsamındaki dünya: (w adresi, w uzunluğu, oy adresi, çevrilmiş dünya — ilk istekte kurulur)
    static OW: std::cell::RefCell<Option<(usize, usize, usize, Option<std::rc::Rc<Vec<Op>>>)>> = const { std::cell::RefCell::new(None) };
}
/// `f` boyunca (w, oy) dünyasının kafa çevirisini paylaştırır: aynı dünyayı soran her çağrı bir kez kurulmuş
/// çeviriyi alır. Kapsam boyunca w ve oy paylaşımlı ödünçte (değişemez), bu yüzden sonuç her seferinde
/// kurulanla birebir aynıdır. Dünyayı değiştirebilecek çağrılar (ör. &mut Mc alanlar) kapsam dışında kalmalı.
pub fn ow_paylas<R>(w: &[Varlik], oy: &[Oyuncu], f: impl FnOnce() -> R) -> R {
    let eski = OW.with(|o| o.replace(Some((w.as_ptr() as usize, w.len(), oy.as_ptr() as usize, None))));
    let r = f();
    OW.with(|o| *o.borrow_mut() = eski);
    r
}
pub fn ow(w: &[Varlik], oy: &[Oyuncu]) -> std::rc::Rc<Vec<Op>> {
    let a = (w.as_ptr() as usize, w.len(), oy.as_ptr() as usize);
    let hazir = OW.with(|o| match &*o.borrow() {
        Some((wp, wl, op, Some(v))) if (*wp, *wl, *op) == a => Some(v.clone()),
        _ => None,
    });
    if let Some(v) = hazir {
        return v;
    }
    let v = std::rc::Rc::new(ow_kur(w, oy));
    OW.with(|o| {
        if let Some((wp, wl, op, slot)) = &mut *o.borrow_mut() {
            if (*wp, *wl, *op) == a && w.len() > 0 {
                *slot = Some(v.clone());
            }
        }
    });
    v
}

thread_local! {
    /// Kafa dünyasının (Arc'lı oyuncular, değişmez) düz kopyası: aynı dünya için yeniden kopyalanmaz.
    /// Anahtar oyuncu nesnelerinin kendisi (Arc'lar tutulduğu için başka dünyayla karışmaz).
    static DUNYA_KOPYA: std::cell::RefCell<(Vec<Op>, Vec<Oyuncu>)> = const { std::cell::RefCell::new((Vec::new(), Vec::new())) };
}
/// `src`'nin düz kopyasıyla `f`'i çalıştır (dünya aynıysa önceki kopya).
pub fn dunya_kopya<R>(src: &[Op], f: impl FnOnce(&[Oyuncu]) -> R) -> R {
    DUNYA_KOPYA.with(|c| {
        let ayni = {
            let c = c.borrow();
            c.0.len() == src.len() && c.0.iter().zip(src).all(|(a, b)| Arc::ptr_eq(a, b))
        };
        if !ayni {
            match c.try_borrow_mut() {
                Ok(mut c) => {
                    c.0 = src.to_vec();
                    c.1 = src.iter().map(|q| (**q).clone()).collect();
                }
                // iç içe çağrıda (kopya ödünçteyken) başka dünya: geçici kopya
                Err(_) => {
                    let ps: Vec<Oyuncu> = src.iter().map(|q| (**q).clone()).collect();
                    return f(&ps);
                }
            }
        }
        let c = c.borrow();
        f(&c.1)
    })
}
