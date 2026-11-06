//! Tarayıcı kabuğu: JS işçisi (alan-v5/rust-isci.js) maçı bu işlevlerle kurar ve oynatır.
//! Bellek: JS metni `ayir` ile ayrılan yere yazar; çıktı metinleri `metin_ptr`/`metin_len` ile okunur.
#![allow(clippy::missing_safety_doc)]

use alan_motor::mac::motor::Motor;
use alan_motor::oyun::{IsSozluk, MacMotoru};
use alan_motor::mac::kayit::{uygula, Degisim};
use alan_motor::mac::MacTam;
use std::cell::RefCell;

/// kopya aralığı (mac-isci.js KOP)
const KOP: f64 = 60.0;

struct Durum {
    mt: Option<Motor>,
    /// kurulum iletisi (baştan yeniden oynatmak için)
    kurulum: Option<serde_json::Value>,
    /// maç içi değişiklikler (tike göre sıralı)
    gecmis: Vec<Degisim>,
    /// her KOP tikte maçın kopyası: değişiklik gelince baştan değil en yakın kopyadan yeniden oynanır (mac-isci.js anlikK)
    kopyalar: Vec<(f64, MacTam)>,
    isler: IsSozluk,
    is_gonderilen: usize,
    kare: [f32; 174],
    metin: Vec<u8>,
}

thread_local! {
    static D: RefCell<Durum> = RefCell::new(Durum { mt: None, kurulum: None, gecmis: Vec::new(), kopyalar: Vec::new(), isler: IsSozluk::default(), is_gonderilen: 0, kare: [0.0; 174], metin: Vec::new() });
}

#[no_mangle]
pub extern "C" fn ayir(n: usize) -> *mut u8 {
    let mut v = Vec::<u8>::with_capacity(n);
    let p = v.as_mut_ptr();
    std::mem::forget(v);
    p
}

#[no_mangle]
pub unsafe extern "C" fn birak(p: *mut u8, n: usize) {
    drop(Vec::from_raw_parts(p, 0, n));
}

/// Maçı kur (JS `yeni` iletisinin JSON'u). 0: tamam, 1: hata (metin: neden).
#[no_mangle]
pub unsafe extern "C" fn kur(p: *const u8, n: usize) -> i32 {
    let s = std::str::from_utf8(std::slice::from_raw_parts(p, n)).unwrap_or("");
    D.with(|d| {
        let mut d = d.borrow_mut();
        d.isler = IsSozluk::default();
        d.is_gonderilen = 0;
        d.gecmis.clear();
        d.kopyalar.clear();
        let r = serde_json::from_str::<serde_json::Value>(s).map_err(|e| e.to_string()).and_then(|v| Motor::kayittan(&v).map(|mt| (v, mt)));
        match r {
            Ok((v, mt)) => {
                d.kurulum = Some(v);
                d.mt = Some(mt);
                0
            }
            Err(e) => {
                d.metin = e.into_bytes();
                1
            }
        }
    })
}

/// Bir tik ilerle; yeni tik.
#[no_mangle]
pub extern "C" fn adim() -> f64 {
    D.with(|d| {
        let mut d = d.borrow_mut();
        let Durum { mt, kopyalar, .. } = &mut *d;
        let mt = mt.as_mut().expect("maç kurulmadı");
        mt.adim();
        let t = mt.m.tick;
        if t % KOP == 0.0 && !kopyalar.iter().any(|k| k.0 == t) {
            kopyalar.push((t, mt.m.clone()));
        }
        t
    })
}

#[no_mangle]
pub extern "C" fn tik() -> f64 {
    D.with(|d| d.borrow().mt.as_ref().map_or(0.0, |m| m.m.tick))
}

#[no_mangle]
pub extern "C" fn bitti() -> i32 {
    D.with(|d| d.borrow().mt.as_ref().map_or(1, |m| m.bitti() as i32))
}

/// Şimdiki tikin karesini doldur; 174 sayının yeri.
#[no_mangle]
pub extern "C" fn kare() -> *const f32 {
    D.with(|d| {
        let mut d = d.borrow_mut();
        let Durum { mt, isler, kare, .. } = &mut *d;
        if let Some(mt) = mt.as_mut() {
            *kare = mt.kare174(isler);
        }
        kare.as_ptr()
    })
}

fn metin_koy(s: String) -> usize {
    D.with(|d| {
        let mut d = d.borrow_mut();
        d.metin = s.into_bytes();
        d.metin.len()
    })
}

/// Oyuncuların değişmeyen bilgisi (JSON) → metin
#[no_mangle]
pub extern "C" fn statik() -> usize {
    let s = D.with(|d| d.borrow().mt.as_ref().map(|m| m.statik_json()).unwrap_or_else(|| "[]".into()));
    metin_koy(s)
}

/// Son çağrıdan bu yana olaylar ve yeni iş yazıları (JSON: {olay:[…], is:[…]}) → metin
#[no_mangle]
pub extern "C" fn yenilikler() -> usize {
    let s = D.with(|d| {
        let mut d = d.borrow_mut();
        let Durum { mt, isler, is_gonderilen, .. } = &mut *d;
        let mut olay = Vec::new();
        if let Some(mt) = mt.as_mut() {
            let t = mt.m.tick;
            for e in mt.yeni_olaylar() {
                olay.push(format!("{{\"tick\":{},\"t\":{},\"type\":{},\"team\":{},\"text\":{}}}", t, serde_json::to_string(&e.t).unwrap(), serde_json::to_string(&e.r#type).unwrap(), e.team, serde_json::to_string(&e.text).unwrap()));
            }
        }
        let is: Vec<String> = isler.yazilar[*is_gonderilen..].iter().map(|j| serde_json::to_string(j).unwrap()).collect();
        *is_gonderilen = isler.yazilar.len();
        format!("{{\"olay\":[{}],\"is\":[{}]}}", olay.join(","), is.join(","))
    });
    metin_koy(s)
}

#[no_mangle]
pub extern "C" fn metin_ptr() -> *const u8 {
    D.with(|d| d.borrow().metin.as_ptr())
}

#[no_mangle]
pub extern "C" fn metin_len() -> usize {
    D.with(|d| d.borrow().metin.len())
}

/// Seyrek yerleşim (uzak hücumcunun işi 3 tikte bir): 1 açık (varsayılan), 0 kapalı (eski davranış, her tik).
#[no_mangle]
pub extern "C" fn seyrek(acik: i32) {
    alan_motor::sekil::konum::SEYREK_KAPALI.store(acik == 0, std::sync::atomic::Ordering::Relaxed);
}

/// mac-isci.js seyrelt(T): oynatılanın (T) gerisindeki ara kopyalar atılır (300'ün katları ve son 60 tik kalır);
/// taktik değişikliği oynatılan andan geriye gidemez.
fn seyrelt_ic(k: &mut Vec<(f64, MacTam)>, T: f64) {
    k.retain(|a| a.0 % 300.0 == 0.0 || a.0 >= T - 60.0);
}

/// Sayfanın oynattığı tik (JS `izle`)
#[no_mangle]
pub extern "C" fn seyrelt(T: f64) {
    D.with(|d| seyrelt_ic(&mut d.borrow_mut().kopyalar, T));
}

/// Maç içi taktik değişikliği (JS `degistir` iletisinin JSON'u: {snapT, t, tac, diz, roller}). Maç snapT'ye en yakın
/// kopyadan (yoksa baştan) değişikliklerle yeniden oynanır (mac-isci.js yeniden). 0: tamam, 1: hata (metin: neden).
#[no_mangle]
pub unsafe extern "C" fn degistir(p: *const u8, n: usize) -> i32 {
    let s = std::str::from_utf8(std::slice::from_raw_parts(p, n)).unwrap_or("");
    let c: Degisim = match serde_json::from_str(s) {
        Ok(c) => c,
        Err(e) => {
            metin_koy(e.to_string());
            return 1;
        }
    };
    let r = D.with(|d| -> Result<(), String> {
        let mut d = d.borrow_mut();
        let Durum { mt, kurulum, gecmis, kopyalar, .. } = &mut *d;
        let T = c.tick;
        gecmis.retain(|g| g.tick < T);
        gecmis.push(c);
        kopyalar.retain(|a| a.0 <= T);
        seyrelt_ic(kopyalar, T);
        let mut m = match kopyalar.last() {
            Some(a) => a.1.clone(),
            None => Motor::kayittan(kurulum.as_ref().ok_or("maç kurulmadı")?)?.m,
        };
        let mut ci = 0;
        while ci < gecmis.len() && gecmis[ci].tick < m.tick {
            ci += 1;
        }
        while m.tick < T && !m.over {
            while ci < gecmis.len() && gecmis[ci].tick <= m.tick {
                uygula(&mut m, &gecmis[ci]);
                ci += 1;
            }
            alan_motor::mac::step(&mut m);
        }
        while ci < gecmis.len() && gecmis[ci].tick <= m.tick {
            uygula(&mut m, &gecmis[ci]);
            ci += 1;
        }
        // T'ye kadarki kayıtlar sayfada zaten var
        alan_motor::mac::kafa_bag::kafaAc(&mut m);
        let mut yeni = Motor::yeni(m);
        yeni.olaylar_gonderildi();
        *mt = Some(yeni);
        Ok(())
    });
    match r {
        Ok(()) => 0,
        Err(e) => {
            metin_koy(e);
            1
        }
    }
}
