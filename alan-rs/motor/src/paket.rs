//! Paralel kafa: işçiler arasında tartı sonuçlarının aktarımı.
//! Her işçi aynı maçı oynar; dalganın başında kurulan başvuru tablosundaki (kararın dünyası,
//! taşıyıcı, maçın oyuncuları) oyuncular sıra numarasıyla gider, yenileri değerle. Pakette ikinci
//! kez geçen oyuncu ya da dünya ilkine başvurur (JS nesne kimliği korunur).
use crate::model::Oyuncu;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

type Op = Arc<Oyuncu>;
type W = Arc<Vec<Op>>;

#[derive(Default)]
pub struct Tablo {
    /// kodlama: adres → sıra
    po: HashMap<usize, u32>,
    pw: HashMap<usize, u32>,
    /// çözme: sıra → nesne
    lo: Vec<Op>,
    lw: Vec<W>,
    /// tabanın uzunlukları (paket başında buraya geri dönülür)
    no: usize,
    nw: usize,
    /// yetenek satırları (oyuncunun ortak, değişmeyen satırı): adres → sıra
    py: HashMap<usize, u32>,
    ly: Vec<Arc<crate::yetenek::Yetenek>>,
}

thread_local! {
    pub static TABLO: RefCell<Tablo> = RefCell::new(Tablo::default());
}

impl Tablo {
    fn oy(&mut self, o: &Op) {
        self.ye(&o.govde.yet);
        let a = Arc::as_ptr(o) as usize;
        if !self.po.contains_key(&a) {
            self.po.insert(a, self.lo.len() as u32);
            self.lo.push(o.clone());
        }
    }
    fn ye(&mut self, y: &Arc<crate::yetenek::Yetenek>) {
        let a = Arc::as_ptr(y) as usize;
        if !self.py.contains_key(&a) {
            self.py.insert(a, self.ly.len() as u32);
            self.ly.push(y.clone());
        }
    }
    fn du(&mut self, w: &W) {
        let a = Arc::as_ptr(w) as usize;
        if !self.pw.contains_key(&a) {
            self.pw.insert(a, self.lw.len() as u32);
            self.lw.push(w.clone());
        }
    }
    /// paketin eklediklerini at (her paket tabanla başlar)
    fn geri(&mut self) {
        for o in self.lo.drain(self.no..) {
            self.po.remove(&(Arc::as_ptr(&o) as usize));
        }
        for w in self.lw.drain(self.nw..) {
            self.pw.remove(&(Arc::as_ptr(&w) as usize));
        }
    }
}

/// Dalganın başında: başvuru tablosunu kur (her işçide aynı sırayla).
pub fn taban_kur<'a>(dunyalar: impl IntoIterator<Item = &'a W>, oyuncular: impl IntoIterator<Item = &'a Op>) {
    TABLO.with(|t| {
        let mut t = t.borrow_mut();
        *t = Tablo::default();
        for w in dunyalar {
            t.du(w);
            for o in w.iter() {
                t.oy(o);
            }
        }
        for o in oyuncular {
            t.oy(o);
        }
        t.no = t.lo.len();
        t.nw = t.lw.len();
    });
}
pub fn taban_bos() {
    TABLO.with(|t| *t.borrow_mut() = Tablo::default());
}

#[derive(Serialize, Deserialize)]
enum OyK {
    R(u32),
    V(Box<Oyuncu>),
}
#[derive(Serialize, Deserialize)]
enum DuK {
    R(u32),
    V(Vec<OyK>),
}

fn oy_kodla(t: &mut Tablo, o: &Op) -> OyK {
    let a = Arc::as_ptr(o) as usize;
    if let Some(&i) = t.po.get(&a) {
        return OyK::R(i);
    }
    t.oy(o);
    OyK::V(Box::new((**o).clone()))
}
fn oy_coz(t: &mut Tablo, k: OyK) -> Op {
    match k {
        OyK::R(i) => t.lo[i as usize].clone(),
        OyK::V(b) => {
            let o: Op = Arc::new(*b);
            t.oy(&o);
            o
        }
    }
}

pub mod dunya_kod {
    use super::*;
    pub fn serialize<S: Serializer>(w: &Option<W>, s: S) -> Result<S::Ok, S::Error> {
        let k: Option<DuK> = w.as_ref().map(|w| {
            TABLO.with(|t| {
                let mut t = t.borrow_mut();
                let a = Arc::as_ptr(w) as usize;
                if let Some(&i) = t.pw.get(&a) {
                    return DuK::R(i);
                }
                let v = w.iter().map(|o| oy_kodla(&mut t, o)).collect();
                t.du(w);
                DuK::V(v)
            })
        });
        k.serialize(s)
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Option<W>, D::Error> {
        let k: Option<DuK> = Option::deserialize(d)?;
        Ok(k.map(|k| {
            TABLO.with(|t| {
                let mut t = t.borrow_mut();
                match k {
                    DuK::R(i) => {
                        if i as usize >= t.lw.len() {
                            panic!("dünya başvurusu {} yok (tabloda {} dünya, {} oyuncu; taban {} / {})", i, t.lw.len(), t.lo.len(), t.nw, t.no);
                        }
                        t.lw[i as usize].clone()
                    }
                    DuK::V(v) => {
                        let w: W = Arc::new(v.into_iter().map(|o| oy_coz(&mut t, o)).collect());
                        t.du(&w);
                        w
                    }
                }
            })
        }))
    }
}

/// Bir değeri paketle (tablo pakete özgü eklerden temizlenir).
pub fn kodla<T: Serialize>(x: &T) -> Vec<u8> {
    // tek geçiş: bincode::serialize önce boyut ölçmek için bir kez daha kodlar, o geçiş tabloyu doldurur
    let mut b = Vec::with_capacity(1024);
    TABLO.with(|t| t.borrow_mut().geri());
    bincode::serialize_into(&mut b, x).expect("paket");
    TABLO.with(|t| t.borrow_mut().geri());
    b
}
pub fn coz<T: for<'de> Deserialize<'de>>(b: &[u8]) -> T {
    let x = bincode::deserialize(b).expect("paket çöz");
    TABLO.with(|t| t.borrow_mut().geri());
    x
}

pub mod yet_kod {
    use super::*;
    use crate::yetenek::Yetenek;
    pub fn serialize<S: Serializer>(y: &Arc<Yetenek>, s: S) -> Result<S::Ok, S::Error> {
        let i = TABLO.with(|t| t.borrow().py.get(&(Arc::as_ptr(y) as usize)).copied());
        i.expect("yetenek satırı tabanda yok").serialize(s)
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Arc<Yetenek>, D::Error> {
        let i = u32::deserialize(d)?;
        Ok(TABLO.with(|t| t.borrow().ly[i as usize].clone()))
    }
}
