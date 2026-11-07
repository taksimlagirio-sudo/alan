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
    /// son gönderilen karar kartının imzası (JS sonDec)
    son_karar: String,
    isler: IsSozluk,
    is_gonderilen: usize,
    kare: [f32; 174],
    metin: Vec<u8>,
}

thread_local! {
    static D: RefCell<Durum> = RefCell::new(Durum { mt: None, kurulum: None, gecmis: Vec::new(), kopyalar: Vec::new(), son_karar: String::new(), isler: IsSozluk::default(), is_gonderilen: 0, kare: [0.0; 174], metin: Vec::new() });
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
        d.son_karar.clear();
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
        let Durum { mt, isler, is_gonderilen, son_karar, .. } = &mut *d;
        let mut olay = Vec::new();
        let mut karar = Vec::new();
        let mut kafa = Vec::new();
        if let Some(mt) = mt.as_mut() {
            let t = mt.m.tick;
            for e in mt.yeni_olaylar() {
                olay.push(format!("{{\"tick\":{},\"t\":{},\"type\":{},\"team\":{},\"text\":{}}}", t, serde_json::to_string(&e.t).unwrap(), serde_json::to_string(&e.r#type).unwrap(), e.team, serde_json::to_string(&e.text).unwrap()));
            }
            if let Some(k) = karar_json(&mt.m, son_karar) {
                karar.push(k);
            }
            if let Some(K) = mt.m.mac._kafa.as_mut() {
                for r in K.L.drain(..) {
                    kafa.push(kafa_json(&r).to_string());
                }
            }
        }
        let is: Vec<String> = isler.yazilar[*is_gonderilen..].iter().map(|j| serde_json::to_string(j).unwrap()).collect();
        *is_gonderilen = isler.yazilar.len();
        format!("{{\"olay\":[{}],\"is\":[{}],\"karar\":[{}],\"kafa\":[{}]}}", olay.join(","), is.join(","), karar.join(","), kafa.join(","))
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
        let Durum { mt, kurulum, gecmis, kopyalar, son_karar, .. } = &mut *d;
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
        alan_motor::mac::kafa_bag::kafa_ac(&mut m);
        let mut s0 = String::new();
        karar_json(&m, &mut s0);
        *son_karar = s0;
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

/// sayıyı n basamağa yuvarla (JS +x.toFixed(n)); sonlu değilse null
fn yuv(x: f64, n: i32) -> serde_json::Value {
    if !x.is_finite() {
        return serde_json::Value::Null;
    }
    let k = 10f64.powi(n);
    serde_json::json!((x * k).round() / k)
}

/// Taşıyıcının son kararı (mac-isci.js kaydet'in karar kısmı): değişmişse karar kartı (JSON), değişmemişse yok.
fn karar_json(m: &MacTam, son: &mut String) -> Option<String> {
    use serde_json::{json, Map, Value};
    let d = m.last_dec.as_ref()?;
    let hi = m.ps.iter().position(|p| &*p.govde.name == d.name);
    let okuma = hi.and_then(|i| {
        let h = &m.mac.ham.get(i)?.0;
        let al = |k: &str| h.iter().find(|e| e.0 == k).and_then(|e| if let alan_motor::mac::HamV::S(x) = e.1 { Some(x) } else { None });
        Some(al("gorus").or_else(|| al("okuma")).unwrap_or(10.0))
    });
    let aday: Vec<Value> = d
        .opts
        .iter()
        .map(|o| {
            let mut a = Map::new();
            a.insert("kind".into(), json!(o.kind.clone() + &if o.q.is_empty() { String::new() } else { format!(" → {}", o.q) }));
            a.insert("v".into(), yuv(if o.v.is_finite() { o.v } else { 0.0 }, 3));
            a.insert("p".into(), if o.P.is_finite() { yuv(o.P, 2) } else { json!(0) });
            a.insert("to".into(), o.to.as_ref().map_or(Value::Null, |t| json!({"x": t.x, "y": t.y})));
            if let Some(dv) = o.devam.filter(|x| *x != 0.0) {
                a.insert("devam".into(), json!(dv));
            }
            if let Some(e) = &o.en {
                if e.ek != 0.0 {
                    a.insert("en".into(), json!({"ek": e.ek, "sab": 0, "hb": e.hizBiz, "hr": e.hizRakip}));
                }
                a.insert("bz".into(), yuv(e.biz, 3));
                if let Some(i) = hi {
                    let h = &m.ps[i];
                    let key = format!("{}:{}", h.team, h.id);
                    let v = e.per.iter().find(|x| x.0 == key).map_or(0.0, |x| x.1);
                    a.insert("bh".into(), yuv(v, 3));
                }
            }
            if let Some(x) = o.sjK.filter(|x| *x != 0.0) {
                a.insert("sjK".into(), json!(x));
            }
            if let Some(dk) = &o.dok {
                let mut r = Map::new();
                for (k, v) in &dk.k {
                    if v.abs() >= 0.0005 {
                        r.insert(k.clone(), yuv(*v, 4));
                    }
                }
                a.insert("dok".into(), Value::Object(r));
            }
            Value::Object(a)
        })
        .collect();
    let imza = format!("{}|{}|{}|{}", d.name, d.x, d.y, serde_json::to_string(&aday).unwrap_or_default());
    if *son == imza {
        return None;
    }
    *son = imza;
    Some(json!({"tip": "karar", "tick": m.tick, "ad": d.name, "team": d.team, "okuma": okuma, "x": d.x, "y": d.y, "aday": aday, "secilen": 0}).to_string())
}

/// Kafa ↔ gerçek kaydı (match.js kafaBasla/kafaBitir'in r'si) JSON olarak; JS'teki gibi boş alanlar yazılmaz.
fn kafa_json(r: &alan_motor::mac::kafa_bag::KafaKayit) -> serde_json::Value {
    use serde_json::{json, Map, Value};
    let mut o = Map::new();
    let mut koy = |k: &str, v: Option<Value>| {
        if let Some(v) = v {
            o.insert(k.into(), v);
        }
    };
    koy("t", Some(json!(r.t)));
    koy("ad", Some(json!(r.ad)));
    koy("id", Some(json!(r.id)));
    koy("team", Some(json!(r.team)));
    koy("kind", Some(json!(r.kind)));
    koy("kendine", r.kendine.then(|| json!(1)));
    koy("q", r.q.as_ref().map(|x| json!(x)));
    koy("qid", r.qid.map(|x| json!(x)));
    koy("shot", r.shot.as_ref().map(|x| json!(x)));
    koy("ot", r.ot.then(|| json!(1)));
    for (k, v) in [("v", r.v), ("P", r.P), ("Ps", r.Ps), ("Pk", r.Pk), ("hiz", r.hiz), ("savT", r.savT), ("tf", r.tf), ("x", r.x), ("y", r.y), ("cikis", r.cikis), ("hizC", r.hiz_c), ("hizG", r.hiz_g), ("sek", r.sek)] {
        koy(k, v.filter(|x| x.is_finite()).map(|x| json!(x)));
    }
    koy("alis", r.alis.map(|(x, y)| json!({"x": x, "y": y})));
    koy("bozuldu", r.bozuldu.as_ref().map(|x| json!(x)));
    if let Some(s) = &r.son {
        let mut so = Map::new();
        so.insert("t".into(), json!(s.t));
        if s.santra {
            so.insert("santra".into(), json!(1));
        }
        if s.devam {
            so.insert("devam".into(), json!(1));
        }
        for (k, v) in [("kimId", s.kim_id), ("kimT", s.kim_t), ("x", s.x), ("y", s.y), ("gol", s.gol)] {
            if let Some(v) = v {
                so.insert(k.into(), json!(v));
            }
        }
        if let Some(k) = &s.kim {
            so.insert("kim".into(), json!(k));
        }
        o.insert("son".into(), Value::Object(so));
    }
    o.insert("sapma".into(), Value::Array(r.sapma.iter().map(|x| json!({"k": x.k, "a": x.a, "y": x.y})).collect()));
    Value::Object(o)
}

/// Kafanın uçuşunda yoldaki gövde ve kovalamayan savunmacının yola koşabilmesi (kafa2_tarti::govde_yol): 1 açık, 0 kapalı.
#[no_mangle]
pub extern "C" fn govde_yol(acik: i32) {
    alan_motor::kafa::kafa2_tarti::GOVDE_YOL_KAPALI.store(acik == 0, std::sync::atomic::Ordering::Relaxed);
}

/// Yerleşik hücumda adam adama (sekil::konum::yerlesik_adam; deney): 1 açık, 0 kapalı (varsayılan).
#[no_mangle]
pub extern "C" fn yerlesik_adam(acik: i32) {
    alan_motor::sekil::konum::YERLESIK_ADAM_ZORLA.store(acik != 0, std::sync::atomic::Ordering::Relaxed);
}

/// Sürmenin tutma olasılığı değerin tartıldığı ara noktaya dek (kafa2_tarti::sur_e): 1 açık, 0 kapalı.
#[no_mangle]
pub extern "C" fn sur_e(acik: i32) {
    alan_motor::kafa::kafa2_tarti::SUR_E_KAPALI.store(acik == 0, std::sync::atomic::Ordering::Relaxed);
}

/// Kafanın Bekçi dalış modeli (kafa1_ucus::bek_dalis_k): 1 açık (varsayılan), 0 kapalı.
#[no_mangle]
pub extern "C" fn bek_dalis(acik: i32) {
    alan_motor::kafa::kafa1_ucus::BEK_DALIS_KAPALI.store(acik == 0, std::sync::atomic::Ordering::Relaxed);
}

/// Şut tablosunun Bekçi ideali boş sahada (kafa2_imkan::bek_yer_bos): 1 açık (varsayılan), 0 kapalı.
#[no_mangle]
pub extern "C" fn bek_yer_bos(acik: i32) {
    alan_motor::kafa::kafa2_imkan::BEK_YER_BOS_KAPALI.store(acik == 0, std::sync::atomic::Ordering::Relaxed);
}

/// sur_ufuk: 1 açık (varsayılan), 0 kapalı.
#[no_mangle]
pub extern "C" fn sur_ufuk(acik: i32) {
    alan_motor::kafa::kafa2_tarti::SUR_UFUK_KAPALI.store(acik == 0, std::sync::atomic::Ordering::Relaxed);
}

/// sah_yumusak: 1 açık (varsayılan), 0 kapalı.
#[no_mangle]
pub extern "C" fn sah_yumusak(acik: i32) {
    alan_motor::kafa::kafa2_tarti::SAH_YUMUSAK_KAPALI.store(acik == 0, std::sync::atomic::Ordering::Relaxed);
}

/// gurultu_surekli: 1 açık (varsayılan), 0 kapalı.
#[no_mangle]
pub extern "C" fn gurultu_surekli(acik: i32) {
    alan_motor::kafa::kafa1_secim::GURULTU_SUREKLI_KAPALI.store(acik == 0, std::sync::atomic::Ordering::Relaxed);
}

/// bölge değeri haritası (kafa2_tarti::bolge): 1 açık (varsayılan), 0 kapalı (eski sahiplik haritası).
#[no_mangle]
pub extern "C" fn bolge(acik: i32) {
    alan_motor::kafa::kafa2_tarti::BOLGE_KAPALI.store(acik == 0, std::sync::atomic::Ordering::Relaxed);
    alan_motor::kafa::kafa2_tarti::BOLGE_ZORLA.store(acik != 0, std::sync::atomic::Ordering::Relaxed);
}

/// sah_surekli: 1 açık (varsayılan), 0 kapalı.
#[no_mangle]
pub extern "C" fn sah_surekli(acik: i32) {
    alan_motor::kafa::kafa2_tarti::SAH_SUREKLI_KAPALI.store(acik == 0, std::sync::atomic::Ordering::Relaxed);
}

/// plan_sur: 1 açık (varsayılan), 0 kapalı.
#[no_mangle]
pub extern "C" fn plan_sur(acik: i32) {
    alan_motor::kafa::kafa1_secim::PLAN_SUR_KAPALI.store(acik == 0, std::sync::atomic::Ordering::Relaxed);
}

/// kovsuz: 1 açık (varsayılan), 0 kapalı.
#[no_mangle]
pub extern "C" fn kovsuz(acik: i32) {
    alan_motor::kafa::kafa2_tarti::KOVSUZ_SABIT_KAPALI.store(acik == 0, std::sync::atomic::Ordering::Relaxed);
}

/// kayip_sahip: 1 açık (varsayılan), 0 kapalı.
#[no_mangle]
pub extern "C" fn kayip_sahip(acik: i32) {
    alan_motor::kafa::kafa2_tarti::KAYIP_SAHIP_KAPALI.store(acik == 0, std::sync::atomic::Ordering::Relaxed);
}

/// plan: 1 açık, 0 kapalı (varsayılan).
#[no_mangle]
pub extern "C" fn plan(acik: i32) {
    alan_motor::kafa::kafa1_secim::PLAN_KAPALI.store(acik == 0, std::sync::atomic::Ordering::Relaxed);
    alan_motor::kafa::kafa1_secim::PLAN_ZORLA.store(acik != 0, std::sync::atomic::Ordering::Relaxed);
}

// ---- hücum ve savunma oturumlarının deneyleri (varsayılan kapalı; sitede adreste =1 ya da yeni=1 hepsini açar) ----
macro_rules! anahtar {
    ($ad:ident, $yol:path) => {
        #[no_mangle]
        pub extern "C" fn $ad(acik: i32) {
            $yol.store(acik != 0, std::sync::atomic::Ordering::Relaxed);
        }
    };
}
// H1: sahiplik haritası yakınsayana dek (sahyak) / ön eleme sırayla (onsira)
anahtar!(sah_yakinsa, alan_motor::kafa::kafa2_tarti::SAH_YAKINSA_ZORLA);
anahtar!(on_sira, alan_motor::kafa::kafa2_tarti::ON_SIRA_ZORLA);
// H2: savurma yolunda rakip tepkiden sonra kovalar (savtepki)
anahtar!(savur_tepki, alan_motor::kafa::kafa2_tarti::SAVUR_TEPKI_ZORLA);
// tut: eldeki en iyi seçenek × beklemenin indirimi (tutelde)
anahtar!(tut_elde, alan_motor::kafa::kafa2_tarti::TUT_ELDE_ZORLA);
// S1 baskıya kazanç, S2 atamada süreklilik, S3 savunma tabanı
anahtar!(pres_kazanc, alan_motor::sekil::konum::PRES_KAZANC_ZORLA);
anahtar!(ua_hist, alan_motor::sekil::konum::UA_HIST_ZORLA);
anahtar!(sav_taban, alan_motor::sekil::konum::SAV_TABAN_ZORLA);
/// S7 taşıyıcıyı durdurmak: bit 1 şut hattı, 2 ikinci presçi, 4 kale önüne dönüş (7 hepsi)
#[no_mangle]
pub extern "C" fn s7(bit: i32) {
    alan_motor::sekil::S7_ZORLA.store(bit as u32, std::sync::atomic::Ordering::Relaxed);
}
// H4a: hücum dizilişinin ön ucu toptan en çok bir pas menzili ileride (hucumboy)
anahtar!(hucum_boy, alan_motor::sekil::yer::HUCUM_BOY_ZORLA);
// haritada komşu hücreye varış yarışına rakip Bekçi de girer (sahbekci); Bekçi dar açıda yakın direği tutar (bekdirek)
anahtar!(sah_bekci, alan_motor::kafa::kafa2_tarti::SAH_BEKCI_ZORLA);
anahtar!(bek_direk, alan_motor::sekil::yer::BEK_DIREK_ZORLA);
// önde bekleyen pasa açılır (ondeac); Kuyu alanından paylı ve koşarak çıkış (kuyupay)
anahtar!(onde_ac, alan_motor::sekil::konum::ONDE_AC_ZORLA);
anahtar!(kuyu_pay, alan_motor::mac::kuyu::KUYU_PAY_ZORLA);
// harita kenarının indirimi eylemin süresine göre (sahzaman; yeni=1'de yok)
anahtar!(sah_zaman, alan_motor::kafa::kafa2_tarti::SAH_ZAMAN_ZORLA);
// bütün oyuncular için yürüme: hız seçiminde normal koşunun altı, bar dolumu hıza bağlı (yuru)
anahtar!(yuru, alan_motor::mac::enerji::YURU_ZORLA);
// kayıp bedelinde sahipliğin değeri en az bölgenin harita değeri (kayipharita)
anahtar!(kayip_harita, alan_motor::kafa::kafa2_tarti::KAYIP_HARITA_ZORLA);
// hız tercihi: koşu değeri varışla sürekli, savunmada tehlike anına karşı, bar değeri hedeften bağımsız (hiz2)
anahtar!(hiz2, alan_motor::mac::enerji::HIZ2_ZORLA);
// hiz2: tehlike anından sonraki varışın değer tabanı (hiz2taban)
anahtar!(hiz2_taban, alan_motor::mac::enerji::HIZ2_TABAN_ZORLA);
// keep_p hamle süresini ve bekleme süresini tartar (kphamle; B2)
anahtar!(kp_hamle, alan_motor::kafa::kafa1_ikili::KP_HAMLE_ZORLA);
// sahiplik haritasının tahmin dünyası topun gerçek varışında (sahtc) / hücre şutunda dibindeki rakip ikili kesmez (sahdibinde)
anahtar!(sah_tc, alan_motor::kafa::kafa2_tarti::SAH_TC_ZORLA);
anahtar!(sah_dibinde, alan_motor::kafa::kafa2_tarti::SAH_DIBINDE_ZORLA);
// A1 tutma bilgi tablosu kayıp bedelinde (bilgitutma) / yalnız şutun bedelinde (bilgitutmasut)
anahtar!(bilgi_tutma, alan_motor::kafa::kafa2_tarti::BILGI_TUTMA_ZORLA);
anahtar!(bilgi_tutma_sut, alan_motor::kafa::kafa2_tarti::BILGI_TUTMA_SUT_ZORLA);
anahtar!(bilgi_tutma_tut, alan_motor::kafa::kafa2_tarti::BILGI_TUTMA_TUT_ZORLA);
anahtar!(bilgi_tutma_olcek, alan_motor::kafa::kafa2_tarti::BILGI_TUTMA_OLCEK_ZORLA);
// kontra koşusunun değeri değer tablosundan (ktab; hiz2 ile)
anahtar!(ktab, alan_motor::mac::kontra_tablo::KTAB_ZORLA);
