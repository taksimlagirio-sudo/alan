//! Kafa 2: yabancıya uğramayan saf işlevlerin JS ile bit bit aynılığı (sina/kafa2-vektor.js).
use alan_motor::kafa::gecici_dis1::Dis;
use alan_motor::kafa::kafa2_bag as dis;
use alan_motor::kafa::kafa2_baglam::*;
use std::cell::RefCell;
use alan_motor::kafa::{kafa2_dunya, kafa2_imkan, kafa2_karar, kafa2_tarti};
use alan_motor::model::{JPEg, JPTl, Oyuncu};
use alan_motor::yetenek::{Ham, Yetenek};
use serde_json::Value;
use std::sync::Arc;

fn h(v: &Value) -> f64 {
    match v.as_str() {
        Some(s) => f64::from_bits(u64::from_str_radix(s, 16).unwrap()),
        None => f64::NAN,
    }
}
fn ho(v: &Value) -> Option<f64> {
    if v.is_null() {
        None
    } else {
        Some(h(v))
    }
}
fn ayni_b(a: f64, b: f64) -> bool {
    a.to_bits() == b.to_bits() || (a != a && b != b)
}

fn oyuncu(v: &Value) -> Oyuncu {
    let mut a = Ham::default();
    let mut p = Oyuncu::default();
    for e in v["a"].as_array().unwrap() {
        let k = e[0].as_str().unwrap();
        let x = h(&e[1]);
        a.v.insert(k.to_string(), x);
        match k {
            "hiz" => p.govde_mut().a.hiz = x,
            "ceviklik" => p.govde_mut().a.ceviklik = x,
            "guc" => p.govde_mut().a.guc = x,
            "uzanma" => p.govde_mut().a.uzanma = x,
            "alan" => p.govde_mut().a.alan = x,
            "kontrol" => p.govde_mut().a.kontrol = x,
            "surme" => p.govde_mut().a.surme = x,
            "pas" => p.govde_mut().a.pas = x,
            "gonderme" => p.govde_mut().a.gonderme = x,
            "yukleme" => p.govde_mut().a.yukleme = x,
            "tepki" => p.govde_mut().a.tepki = x,
            "gorus" => p.govde_mut().a.gorus = x,
            "yeralma" => p.govde_mut().a.yeralma = x,
            "sogukkanlilik" => p.govde_mut().a.sogukkanlilik = x,
            "kesme" => p.govde_mut().a.kesme = x,
            "refleks" => p.govde_mut().a.refleks = x,
            "yerlesim" => p.govde_mut().a.yerlesim = x,
            "hakimiyet" => p.govde_mut().a.hakimiyet = x,
            "enerji" => p.govde_mut().a.enerji = x,
            "govde" => p.govde_mut().a.govde = x,
            "caliskanlik" => p.govde_mut().a.caliskanlik = x,
            _ => {}
        }
    }
    p.id = v["id"].as_f64().unwrap();
    p.team = v["team"].as_f64().unwrap();
    p.govde_mut().role = v["role"].as_str().unwrap_or("").into();
    p.govde_mut().name = v["name"].as_str().unwrap_or("").into();
    p.govde_mut().yet = if a.v.is_empty() { Yetenek::bos().into() } else { Yetenek::kur(&a).into() };
    p.x = h(&v["x"]);
    p.y = h(&v["y"]);
    p.vx = h(&v["vx"]);
    p.vy = h(&v["vy"]);
    p.tx = h(&v["tx"]);
    p.ty = h(&v["ty"]);
    p.en = ho(&v["en"]);
    p.den = ho(&v["den"]);
    p.noTouch = v["noTouch"].as_bool().unwrap();
    p.press = v["press"].as_bool().unwrap();
    p.R = ho(&v["R"]).unwrap_or(0.0);
    p.D = ho(&v["D"]).unwrap_or(1.0);
    if !v["tlRisk"].is_null() {
        let mut t = JPTl::default();
        t.huc.insert("risk".into(), h(&v["tlRisk"]));
        t.huc.insert("tempo".into(), h(&v["tlTempo"]));
        p.govde_mut().tl = Some(t.into());
    }
    if !v["egRk"].is_null() {
        p.govde_mut().eg = Some(JPEg { ps: 0.5, rk: h(&v["egRk"]) });
    }
    p
}

/// Sınamanın dış dünyası: maçın sprint katı (match.js ALAN_SPR sarmalı) JS'ten; ALAN_SPRE onun
/// üstünde çekirdeğin EN2_SPRE'si (core.js:192, refL 20); yerleşimin tepki parametreleri JS'tekiler.
#[derive(Default)]
struct SinamaDis {
    spr: RefCell<Vec<(f64, f64, f64)>>,
}
impl Dis for SinamaDis {
}
fn kafa_kur() -> (Kafa, Arc<SinamaDis>) {
    let d = Arc::new(SinamaDis::default());
    (Kafa::yeni(d.clone()), d)
}

fn vektor() -> Option<Value> {
    let yol = concat!(env!("CARGO_MANIFEST_DIR"), "/../sina/kafa2-vektor.json");
    match std::fs::read_to_string(yol) {
        Ok(s) => Some(serde_json::from_str(&s).unwrap()),
        Err(_) => {
            eprintln!("kafa2-vektor.json yok (node sina/kafa2-vektor.js ile üretilir): atlandı");
            None
        }
    }
}

#[test]
#[ignore = "JS karşılaştırması: vektör sina/kafa2-vektor.js ile üretilince `cargo test --release -- --ignored`; Rust tek doğru kaynak (MIMARI.md Aşama 6)"]
fn imk_egrileri() {
    let v = match vektor() {
        Some(v) => v,
        None => return,
    };
    let (k, _) = kafa_kur();
    let mut hata = Vec::new();
    for e in v["sap"].as_array().unwrap() {
        let t = h(&e[0]);
        let r = [kafa2_imkan::imk_sap(t), kafa2_imkan::imk_tau(t), kafa2_imkan::imk_sg(t / 20.0 - 5.0), kafa2_imkan::imk_inan(&k, t)];
        for (i, x) in r.iter().enumerate() {
            if !ayni_b(*x, h(&e[i + 1])) {
                hata.push(format!("t {t}: {i}. {x:e} js {:e}", h(&e[i + 1])));
            }
        }
    }
    assert!(hata.is_empty(), "{} hata: {:?}", hata.len(), &hata[..hata.len().min(10)]);
}

#[test]
#[ignore = "JS karşılaştırması: vektör sina/kafa2-vektor.js ile üretilince `cargo test --release -- --ignored`; Rust tek doğru kaynak (MIMARI.md Aşama 6)"]
fn js_ile_bit_bit_ayni() {
    let v = match vektor() {
        Some(v) => v,
        None => return,
    };
    let (mut k, sd) = kafa_kur();
    let mut hata: Vec<String> = Vec::new();
    let mut say = [0usize; 8];
    let bak = |ad: &str, tk: u64, x: f64, j: &Value, hata: &mut Vec<String>, s: &mut usize| {
        *s += 1;
        if !ayni_b(x, h(j)) {
            hata.push(format!("t{tk} {ad}: {x:e} js {:e}", h(j)));
        }
    };
    for a in v["anlar"].as_array().unwrap() {
        let tk = a["tick"].as_u64().unwrap();
        k.bag._tick = Some(tk as f64);
        let ps: Dunya = a["ps"].as_array().unwrap().iter().map(|p| Arc::new(oyuncu(p))).collect();
        // maçın sprint katı (match.js sarmalı) JS'ten
        *sd.spr.borrow_mut() = a["per"].as_array().unwrap().iter().enumerate().map(|(i, e)| (ps[i].id, ps[i].team, h(&e[1]))).collect();
        for (i, e) in a["per"].as_array().unwrap().iter().enumerate() {
            let p = &ps[i];
            bak("SPRE", tk, dis::alan_spre(&k, p), &e[0], &mut hata, &mut say[0]);
            bak("TEPKI kovala", tk, dis::alan_tepki(&k, p, "kovala"), &e[2], &mut hata, &mut say[0]);
            bak("TEPKI algi", tk, dis::alan_tepki(&k, p, "algi"), &e[3], &mut hata, &mut say[0]);
            bak("TEPKI bosTop", tk, dis::alan_tepki(&k, p, "bosTop"), &e[4], &mut hata, &mut say[0]);
            let _ = &e[5]; // otFactor: rastgele inSp, girdi yazılmadı
        }
        for e in a["nok"].as_array().unwrap() {
            let team = e[0].as_f64().unwrap();
            let (x, y) = (h(&e[1]), h(&e[2]));
            bak("threat", tk, dis::threat(&k, team, x, y), &e[3], &mut hata, &mut say[1]);
            bak("pressAt", tk, dis::press_at(&k, &ps, team, x, y), &e[4], &mut hata, &mut say[1]);
            bak("freeAt", tk, dis::free_at(&k, &ps, team, x, y), &e[5], &mut hata, &mut say[1]);
            bak("kuyuD", tk, dis::kuyu_d(x, y), &e[6], &mut hata, &mut say[1]);
            bak("sutEngel", tk, kafa2_imkan::sut_engel(&k, &ps, team, x, y), &e[7], &mut hata, &mut say[2]);
        }
        for e in a["gc"].as_array().unwrap() {
            let team = e[0].as_f64().unwrap();
            let r = kafa2_dunya::geri_cekil(&k, &ps, team, nk(h(&e[1]), h(&e[2])), nk(h(&e[3]), h(&e[4])), h(&e[5]), e[6].as_bool().unwrap());
            for (i, q) in e[7].as_array().unwrap().iter().enumerate() {
                for (j, x) in [r[i].x, r[i].y, r[i].vx, r[i].vy].iter().enumerate() {
                    bak(&format!("geriCekil {i}.{j}"), tk, *x, &q[j], &mut hata, &mut say[3]);
                }
            }
        }
        for e in a["pl"].as_array().unwrap() {
            let team = e[0].as_f64().unwrap();
            let r = kafa2_dunya::paylastir(&k, &ps, team, h(&e[1]), &[]);
            for (i, q) in e[2].as_array().unwrap().iter().enumerate() {
                bak(&format!("paylastir {i}.x"), tk, r[i].x, &q[0], &mut hata, &mut say[4]);
                bak(&format!("paylastir {i}.y"), tk, r[i].y, &q[1], &mut hata, &mut say[4]);
            }
        }
        for e in a["st"].as_array().unwrap() {
            let team = e[0].as_f64().unwrap();
            // imkKov'un önbelleği maçın tikine bağlı; JS vektörü her çağrıya taze tik verdi
            k.ob.imkk.1.clear();
            let r = kafa2_imkan::sut_tab(&mut k, &ps, team, h(&e[1]), h(&e[2]), h(&e[3]), None, ho(&e[9]), Some(nk(h(&e[4]), h(&e[5]))), Some(nk(h(&e[6]), h(&e[7]))), ho(&e[8]));
            bak("sutTab", tk, r, &e[10], &mut hata, &mut say[5]);
        }
        for e in a["rk"].as_array().unwrap() {
            let p = &ps[e[0].as_u64().unwrap() as usize];
            let tac = if e[1].is_null() {
                None
            } else {
                let t = &e[1];
                Some(Tac { risk: ho(&t["risk"]), tempo: ho(&t["tempo"]), sabir: ho(&t["sabir"]), kontra: t["kontra"].as_bool().unwrap(), macK: ho(&t["macK"]), ..Default::default() })
            };
            bak("riskOf", tk, kafa2_karar::risk_of(&k, p, tac.as_ref()), &e[2], &mut hata, &mut say[6]);
            bak("prGam", tk, kafa2_tarti::pr_gam(&k, p, tac.as_ref()), &e[3], &mut hata, &mut say[6]);
        }
    }
    eprintln!("sınanan: oyuncu {} nokta {} sutEngel {} geriCekil {} paylastir {} sutTab {} risk/prGam {}", say[0], say[1], say[2], say[3], say[4], say[5], say[6]);
    assert!(hata.is_empty(), "{} hata, ilkleri: {:#?}", hata.len(), &hata[..hata.len().min(15)]);
}
