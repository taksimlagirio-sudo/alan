//! Paralel kafa (JS paralel.js'in karşılığı). Aynı maçı oynayan işçiler kararın bağımsız
//! işlerini (dalga) bölüşür: her işçi dalgaya gelince sıradaki işi ortak sayaçtan alır, hesaplar,
//! sonucunu paketleyip ortak panoya yazar; sonra ötekilerin sonuçlarını açıp yerine koyar. İşler
//! sıradan bağımsız olduğu için maç tek çekirdektekiyle bit bit aynıdır.
use std::cell::RefCell;
use std::rc::Rc;

/// İş bölüştürücü. `f(i, None)`: i'yi burada hesapla, paketini döndür; `f(i, Some(b))`: başka
/// işçinin paketini yerine koy.
pub trait Dagitici {
    fn dagit(&self, n: usize, f: &mut dyn FnMut(usize, Option<&[u8]>) -> Option<Vec<u8>>);
}

thread_local! {
    static DAG: RefCell<Option<Rc<dyn Dagitici>>> = const { RefCell::new(None) };
    /// iç içe dalga sırayla (yalnız en dıştaki bölüşülür)
    static ICTE: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

pub fn kur(d: Option<Rc<dyn Dagitici>>) {
    DAG.with(|x| *x.borrow_mut() = d);
}

/// Dalgayı bölüştür; bölüştürücü yoksa (ya da iç içe dalgaysa) `false` döner, çağıran sırayla yapar.
pub fn dagit(n: usize, f: &mut dyn FnMut(usize, Option<&[u8]>) -> Option<Vec<u8>>) -> bool {
    if n < 2 || ICTE.with(|c| c.get()) {
        return false;
    }
    let d = match DAG.with(|x| x.borrow().clone()) {
        Some(d) => d,
        None => return false,
    };
    ICTE.with(|c| c.set(true));
    struct Geri;
    impl Drop for Geri {
        fn drop(&mut self) {
            ICTE.with(|c| c.set(false));
        }
    }
    let _g = Geri;
    d.dagit(n, f);
    true
}

/// Bu dalga bölüşülecek mi (bölüştürücü var, iç içe değil, en az iki iş)?
pub fn hazir(n: usize) -> bool {
    n >= 2 && !ICTE.with(|c| c.get()) && DAG.with(|x| x.borrow().is_some())
}

pub fn icte() -> bool {
    ICTE.with(|c| c.get())
}

#[cfg(not(target_family = "wasm"))]
pub mod yerel {
    //! Bilgisayarda: iş parçacıkları, her biri kendi maçıyla; pano Mutex + Condvar.
    use super::Dagitici;
    use std::collections::HashMap;
    use std::sync::{Arc, Condvar, Mutex};

    struct Dalga {
        n: usize,
        sira: usize,
        sonuc: Vec<Option<Arc<Vec<u8>>>>,
        biten: usize,
        okuyan: usize,
    }
    #[derive(Default)]
    pub struct Pano {
        d: Mutex<HashMap<u64, Dalga>>,
        c: Condvar,
    }
    pub struct Isci {
        pub pano: Arc<Pano>,
        pub n: usize,
        pub sayac: std::cell::Cell<u64>,
        /// ölçüm: (hesaplanan, açılan, bekleme sn)
        pub olc: std::cell::RefCell<(usize, usize, f64, f64, usize)>,
        /// dalgalarda geçen toplam süre
        pub olc_d: std::cell::Cell<f64>,
    }
    impl Isci {
        pub fn yeni(pano: Arc<Pano>, n: usize) -> Isci {
            Isci { pano, n, sayac: Default::default(), olc: Default::default(), olc_d: Default::default() }
        }
    }
    impl Dagitici for Isci {
        fn dagit(&self, n: usize, f: &mut dyn FnMut(usize, Option<&[u8]>) -> Option<Vec<u8>>) {
            let t_d = std::time::Instant::now();
            self.dagit0(n, f);
            self.olc_d.set(self.olc_d.get() + t_d.elapsed().as_secs_f64());
        }
    }
    impl Isci {
        fn dagit0(&self, n: usize, f: &mut dyn FnMut(usize, Option<&[u8]>) -> Option<Vec<u8>>) {
            let s = self.sayac.get();
            self.sayac.set(s + 1);
            let mut benim = vec![false; n];
            let mut g = self.pano.d.lock().unwrap();
            g.entry(s).or_insert_with(|| Dalga { n, sira: 0, sonuc: vec![None; n], biten: 0, okuyan: 0 });
            assert_eq!(g[&s].n, n, "işçiler ayrıştı (dalga {s})");
            loop {
                let d = g.get_mut(&s).unwrap();
                if d.sira >= n {
                    break;
                }
                let i = d.sira;
                d.sira += 1;
                drop(g);
                benim[i] = true;
                let b = f(i, None).expect("paket");
                {
                    let mut o = self.olc.borrow_mut();
                    o.0 += 1;
                    o.4 += b.len();
                }
                g = self.pano.d.lock().unwrap();
                let d = g.get_mut(&s).unwrap();
                d.sonuc[i] = Some(Arc::new(b));
                d.biten += 1;
                if d.biten == n {
                    self.pano.c.notify_all();
                }
            }
            let t0 = std::time::Instant::now();
            while g[&s].biten < n {
                g = self.pano.c.wait(g).unwrap();
            }
            self.olc.borrow_mut().2 += t0.elapsed().as_secs_f64();
            let pk: Vec<Option<Arc<Vec<u8>>>> = g[&s].sonuc.clone();
            drop(g);
            for i in 0..n {
                if !benim[i] {
                    let t1 = std::time::Instant::now();
                    f(i, Some(&pk[i].as_ref().unwrap()[..]));
                    let mut o = self.olc.borrow_mut();
                    o.1 += 1;
                    o.3 += t1.elapsed().as_secs_f64();
                }
            }
            let mut g = self.pano.d.lock().unwrap();
            let d = g.get_mut(&s).unwrap();
            d.okuyan += 1;
            if d.okuyan == self.n {
                g.remove(&s);
            }
        }
    }
}
