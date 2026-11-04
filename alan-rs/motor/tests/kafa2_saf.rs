//! Kafa 2: yabancıya uğramayan saf işlevlerin JS ile bit bit aynılığı (sina/kafa2-vektor.js).
use alan_motor::kafa::gecici_dis2 as dis;
use alan_motor::kafa::kafa2_baglam::*;
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
            "hiz" => p.a.hiz = x,
            "ceviklik" => p.a.ceviklik = x,
            "guc" => p.a.guc = x,
            "uzanma" => p.a.uzanma = x,
            "alan" => p.a.alan = x,
            "kontrol" => p.a.kontrol = x,
            "surme" => p.a.surme = x,
            "pas" => p.a.pas = x,
            "gonderme" => p.a.gonderme = x,
            "yukleme" => p.a.yukleme = x,
            "tepki" => p.a.tepki = x,
            "gorus" => p.a.gorus = x,
            "yeralma" => p.a.yeralma = x,
            "sogukkanlilik" => p.a.sogukkanlilik = x,
            "kesme" => p.a.kesme = x,
            "refleks" => p.a.refleks = x,
            "yerlesim" => p.a.yerlesim = x,
            "hakimiyet" => p.a.hakimiyet = x,
            "enerji" => p.a.enerji = x,
            "govde" => p.a.govde = x,
            "caliskanlik" => p.a.caliskanlik = x,
            _ => {}
        }
    }
    p.id = v["id"].as_f64().unwrap();
    p.team = v["team"].as_f64().unwrap();
    p.role = v["role"].as_str().unwrap_or("").into();
    p.name = v["name"].as_str().unwrap_or("").into();
    p.yet = if a.v.is_empty() { Yetenek::bos() } else { Yetenek::kur(&a) };
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
        p.tl = Some(t);
    }
    if !v["egRk"].is_null() {
        p.eg = Some(JPEg { ps: 0.5, rk: h(&v["egRk"]) });
    }
    p
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
fn imk_egrileri() {
    let v = match vektor() {
        Some(v) => v,
        None => return,
    };
    let k = Kafa::yeni();
    let mut hata = Vec::new();
    for e in v["sap"].as_array().unwrap() {
        let t = h(&e[0]);
        let r = [kafa2_imkan::imkSap(t), kafa2_imkan::imkTau(t), kafa2_imkan::imkSg(t / 20.0 - 5.0), kafa2_imkan::imkInan(&k, t)];
        for (i, x) in r.iter().enumerate() {
            if !ayni_b(*x, h(&e[i + 1])) {
                hata.push(format!("t {t}: {i}. {x:e} js {:e}", h(&e[i + 1])));
            }
        }
    }
    assert!(hata.is_empty(), "{} hata: {:?}", hata.len(), &hata[..hata.len().min(10)]);
}

#[test]
fn js_ile_bit_bit_ayni() {
    let v = match vektor() {
        Some(v) => v,
        None => return,
    };
    let mut k = Kafa::yeni();
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
        k.D._tick = Some(tk as f64);
        let ps: Dunya = a["ps"].as_array().unwrap().iter().map(|p| Arc::new(oyuncu(p))).collect();
        // maçın sprint katı (match.js sarmalı) JS'ten
        dis::SINAMA_SPR.with(|m| *m.borrow_mut() = a["per"].as_array().unwrap().iter().enumerate().map(|(i, e)| (ps[i].id, ps[i].team, h(&e[1]))).collect());
        for (i, e) in a["per"].as_array().unwrap().iter().enumerate() {
            let p = &ps[i];
            bak("SPRE", tk, dis::ALAN_SPRE(p), &e[0], &mut hata, &mut say[0]);
            bak("TEPKI kovala", tk, dis::ALAN_TEPKI(p, "kovala"), &e[2], &mut hata, &mut say[0]);
            bak("TEPKI algi", tk, dis::ALAN_TEPKI(p, "algi"), &e[3], &mut hata, &mut say[0]);
            bak("TEPKI bosTop", tk, dis::ALAN_TEPKI(p, "bosTop"), &e[4], &mut hata, &mut say[0]);
            let _ = &e[5]; // otFactor: rastgele inSp, girdi yazılmadı
        }
        for e in a["nok"].as_array().unwrap() {
            let team = e[0].as_f64().unwrap();
            let (x, y) = (h(&e[1]), h(&e[2]));
            bak("threat", tk, dis::threat(&k, team, x, y), &e[3], &mut hata, &mut say[1]);
            bak("pressAt", tk, dis::pressAt(&k, &ps, team, x, y), &e[4], &mut hata, &mut say[1]);
            bak("freeAt", tk, dis::freeAt(&k, &ps, team, x, y), &e[5], &mut hata, &mut say[1]);
            bak("kuyuD", tk, dis::kuyuD(x, y), &e[6], &mut hata, &mut say[1]);
            bak("sutEngel", tk, kafa2_imkan::sutEngel(&k, &ps, team, x, y), &e[7], &mut hata, &mut say[2]);
        }
        for e in a["gc"].as_array().unwrap() {
            let team = e[0].as_f64().unwrap();
            let r = kafa2_dunya::geriCekil(&ps, team, nk(h(&e[1]), h(&e[2])), nk(h(&e[3]), h(&e[4])), h(&e[5]), e[6].as_bool().unwrap());
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
            k.k2.IMKK.clear();
            let r = kafa2_imkan::sutTab(&mut k, &ps, team, h(&e[1]), h(&e[2]), h(&e[3]), None, ho(&e[9]), Some(nk(h(&e[4]), h(&e[5]))), Some(nk(h(&e[6]), h(&e[7]))), ho(&e[8]));
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
            bak("riskOf", tk, kafa2_karar::riskOf(&k, p, tac.as_ref()), &e[2], &mut hata, &mut say[6]);
            bak("prGam", tk, kafa2_tarti::prGam(&k, p, tac.as_ref()), &e[3], &mut hata, &mut say[6]);
        }
    }
    eprintln!("sınanan: oyuncu {} nokta {} sutEngel {} geriCekil {} paylastir {} sutTab {} risk/prGam {}", say[0], say[1], say[2], say[3], say[4], say[5], say[6]);
    assert!(hata.is_empty(), "{} hata, ilkleri: {:#?}", hata.len(), &hata[..hata.len().min(15)]);
}
