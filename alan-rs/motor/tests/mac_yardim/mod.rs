//! Maç sınamalarının ortak yükleyicisi (sina/mac-vektor.js çıktısı).
#![allow(dead_code)]
use alan_motor::mac::{HamA, HamV};
use alan_motor::model::{Mac, MacTop, Oyuncu, JMFl, JP_sf};
use alan_motor::temas::{TopT, TopTemas};
use alan_motor::top::{Bitis, Top};
use serde_json::Value;

pub fn h(v: &Value) -> f64 {
    match v.as_str() {
        Some(s) => f64::from_bits(u64::from_str_radix(s, 16).unwrap()),
        None => f64::NAN,
    }
}
pub fn ho(v: &Value) -> Option<f64> {
    if v.is_null() { None } else { Some(h(v)) }
}
pub fn ayni(a: f64, b: f64) -> bool {
    a.to_bits() == b.to_bits() || (a != a && b != b)
}
pub fn ayni_o(a: Option<f64>, b: &Value) -> bool {
    match (a, b.is_null()) {
        (None, true) => true,
        (Some(x), false) => ayni(x, h(b)),
        _ => false,
    }
}
pub fn vektor() -> Option<Value> {
    let f = concat!(env!("CARGO_MANIFEST_DIR"), "/../sina/mac-vektor.json");
    match std::fs::read_to_string(f) {
        Ok(s) => Some(serde_json::from_str(&s).unwrap()),
        Err(_) => {
            eprintln!("sina/mac-vektor.json yok (node sina/mac-vektor.js ile üret): sınama atlandı");
            None
        }
    }
}

/// JS ser(p) → Oyuncu
pub fn oyuncu(v: &Value) -> Oyuncu {
    let mut p = Oyuncu::default();
    p.govde_mut().name = v["name"].as_str().unwrap_or("").into();
    p.govde_mut().role = v["role"].as_str().unwrap_or("").into();
    p.job = v["job"].as_str().map(|s| s.into());
    let mut a = HamA::default();
    for (k, x) in v["a"].as_object().unwrap() {
        match k.as_str() {
            "taraf" => a.koy(k, HamV::M(x.as_str().unwrap_or("").into())),
            _ => {
                if x.is_string() {
                    a.koy(k, HamV::S(h(x)));
                }
            }
        }
    }
    alan_motor::mac::kurulum::yetKur(&mut p, &a);
    p.D = 1.0;
    for (k, x) in v["n"].as_object().unwrap() {
        let x = h(x);
        match k.as_str() {
            "id" => p.id = x,
            "team" => p.team = x,
            "i" => p.govde_mut().i = x,
            "R" => p.R = x,
            "D" => p.D = x,
            "x" => p.x = x,
            "y" => p.y = x,
            "vx" => p.vx = x,
            "vy" => p.vy = x,
            "tx" => p.tx = x,
            "ty" => p.ty = x,
            "cd" => p.cd = x,
            "en" => p.en = Some(x),
            "den" => p.den = Some(x),
            "_evx" => p.hal_mut()._evx = Some(x),
            "_evy" => p.hal_mut()._evy = Some(x),
            "_yog" => p.hal_mut()._yog = Some(x),
            "_dolYok" => p.hal_mut()._dolYok = Some(x),
            "_Dl" => p.hal_mut()._Dl = Some(x),
            "_D0" => p.hal_mut()._D0 = Some(x),
            "_enOnce" => p.hal_mut()._enOnce = Some(x),
            "_bosT" => p.hal_mut()._bosT = Some(x),
            "kuyuT" => p.hal_mut().kuyuT = Some(x),
            "_hizK" => p.hal_mut()._hizK = Some(x),
            "_itK" => p.hal_mut()._itK = Some(x),
            "stun" => p.hal_mut().stun = Some(x),
            "adandi" => p.hal_mut().adandi = Some(x),
            "sendT" => p.hal_mut().sendT = Some(x),
            "sendCd" => p.hal_mut().sendCd = Some(x),
            "ca" => p.ca = Some(x),
            "_korT" => p.hal_mut()._korT = Some(x),
            "slowT" => p.hal_mut().slowT = Some(x),
            "fiske" => p.hal_mut().fiske = Some(x),
            "fiskeCd" => p.hal_mut().fiskeCd = Some(x),
            "_dke" => p._dke = Some(x),
            "_rcvAbs" => p.hal_mut()._rcvAbs = Some(x),
            _ => {}
        }
    }
    let bo = &v["bo"];
    p.noTouch = bo["noTouch"].as_bool().unwrap();
    p.hal_mut().sonuk = bo["sonuk"].as_bool().unwrap();
    p.sprint = bo["sprint"].as_bool().unwrap();
    p.hal_mut()._topSpr = bo["_topSpr"].as_bool().unwrap();
    p.hal_mut()._sprIst = bo["_sprIst"].as_bool().unwrap();
    p.hal_mut().amA = bo["amA"].as_bool().unwrap();
    p.press = bo["press"].as_bool().unwrap();
    p.hal_mut()._kuyuCik = bo["_kuyuCik"].as_bool().unwrap();
    if !v["sf"].is_null() {
        p._sf = Some(JP_sf { t0: h(&v["sf"]["t0"]), f: v["sf"]["f"].as_str().unwrap().into() });
    }
    p
}

/// JS mser(m) → Mac (yalnız anlık durumun okunan kısmı)
pub fn mac(v: &Value) -> Mac {
    let mut m = Mac::default();
    m.tick = h(&v["tick"]);
    m.len = h(&v["len"]);
    m.score = v["score"].as_array().unwrap().iter().map(|x| x.as_f64().unwrap()).collect();
    m.ps = v["ps"].as_array().unwrap().iter().map(oyuncu).collect();
    m.holder = v["holder"].as_u64().map(|x| x as usize);
    if !v["ball"].is_null() {
        let bv = &v["ball"];
        let mut b = Top::yeni(h(&bv["x"]), h(&bv["y"]), 0.0, 0.0, bv["team"].as_u64().unwrap() as u8, None, 0.2);
        b.x = h(&bv["x"]);
        b.y = h(&bv["y"]);
        b.done = match bv["done"].as_str() {
            Some("kuyu-sol") => Some(Bitis::KuyuSol),
            Some("kuyu-sag") => Some(Bitis::KuyuSag),
            Some("durdu") => Some(Bitis::Durdu),
            Some("tutuldu") => Some(Bitis::Tutuldu),
            _ => None,
        };
        m.ball = Some(MacTop { tt: TopT { b, t: TopTemas { kimlik: 1, ..Default::default() } }, ek: Default::default() });
    }
    if let Some(q) = v["flq"].as_u64() {
        m.fl = Some(JMFl { q: Some(q as usize), kind: "pas".into(), ..Default::default() });
    }
    if let Some(a) = v["mh"].as_array() {
        m.mac.__mH = Some((a[0].as_f64().unwrap(), a[1].as_f64().unwrap()));
    }
    m.st.loose = vec![0.0, 0.0];
    m
}
