//! Maçın parmak izi: tik başına gözlemlenebilir durumun bit bit özeti (64 bit FNV-1a).
//!
//! Yapısal değişikliklerin güvenlik ağı. Bir değişiklik "davranış aynı" diyorsa iki motorun her tikteki
//! parmak izi aynı olmalı. Özet, `Mac`'in ham baytları değil, açıkça seçilmiş alanlardır: veri yapısı
//! değişse (alan taşınsa, tip değişse) de aynı kalır; yalnız maçın gidişi değişirse değişir.
//!
//! Kapsam: tik, skor, top sahibi, şarj, zar dizisinin durumu, en son olay; topun konumu, hızı, sahibi,
//! uçuş bilgisi; her oyuncunun konumu, hızı, hedefi, enerjisi, sprint/pres durumu ve işi.
//! `f64` değerleri bitleriyle girer (−0 ile 0 ayrı, NaN bitleri olduğu gibi).

use crate::model::Mac;

/// 64 bit FNV-1a.
struct Fnv(u64);
impl Fnv {
    fn yeni() -> Fnv {
        Fnv(0xcbf2_9ce4_8422_2325)
    }
    fn bayt(&mut self, b: &[u8]) {
        for &x in b {
            self.0 ^= x as u64;
            self.0 = self.0.wrapping_mul(0x0100_0000_01b3);
        }
    }
    fn u64(&mut self, v: u64) {
        self.bayt(&v.to_le_bytes());
    }
    fn f(&mut self, v: f64) {
        self.u64(v.to_bits());
    }
    fn of(&mut self, v: Option<f64>) {
        match v {
            Some(x) => {
                self.u64(1);
                self.f(x);
            }
            None => self.u64(0),
        }
    }
    fn ou(&mut self, v: Option<usize>) {
        self.u64(v.map_or(u64::MAX, |x| x as u64));
    }
    fn b(&mut self, v: bool) {
        self.u64(v as u64);
    }
    fn s(&mut self, v: Option<&str>) {
        match v {
            Some(s) => {
                self.u64(s.len() as u64);
                self.bayt(s.as_bytes());
            }
            None => self.u64(u64::MAX),
        }
    }
}

/// Maçın şimdiki durumunun parmak izi.
pub fn parmak_izi(m: &Mac) -> u64 {
    let mut h = Fnv::yeni();
    h.f(m.tick);
    for &s in &m.score {
        h.f(s);
    }
    h.ou(m.holder);
    h.f(m.ch);
    h.b(m.over);
    h.u64(m.r.s as u32 as u64);
    // en son olay (olay sayacı sürümler arasında değişti; metin değişmedi)
    match m.events.first() {
        Some(e) => {
            h.s(Some(&e.t));
            h.s(Some(&e.text));
            h.f(e.team);
        }
        None => h.s(None),
    }
    match &m.ball {
        Some(b) => {
            h.u64(1);
            h.f(b.x);
            h.f(b.y);
            h.f(b.vx);
            h.f(b.vy);
            h.f(b.e);
            h.f(b.ch);
            h.u64(b.team as u64);
            h.b(b.alive);
            h.f(b.t);
            h.ou(b.tt.t.from);
            h.ou(b.tt.t.recv);
        }
        None => h.u64(0),
    }
    for p in &m.ps {
        h.f(p.x);
        h.f(p.y);
        h.f(p.vx);
        h.f(p.vy);
        h.f(p.tx);
        h.f(p.ty);
        h.of(p.en);
        h.of(p.den);
        h.of(p.ca);
        h.b(p.sprint);
        h.b(p.press);
        h.b(p.noTouch);
        h.s(p.job.as_deref());
    }
    h.0
}
