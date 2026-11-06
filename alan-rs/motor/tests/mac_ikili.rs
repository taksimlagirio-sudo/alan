//! İkili ve hareket: JS ile bit bit aynılık (ivme, kutle, direnc, omuzK, ikEn, durtFark, sendS, dengeKayip, dengeHiz, govdeAyir).
mod mac_yardim;
use alan_motor::mac::hareket::ivme;
use alan_motor::mac::ikili::*;
use alan_motor::mac::MacTam as Mac;
use mac_yardim::*;

#[test]
fn ivme_js_ile_ayni() {
    let Some(v) = vektor() else { return };
    let mut hata = vec![];
    for c in v["ivme"].as_array().unwrap() {
        let mut p = oyuncu(&c["p"]);
        ivme(&mut p, h(&c["wx"]), h(&c["wy"]), h(&c["vm"]));
        if !ayni(p.vx, h(&c["vx"])) || !ayni(p.vy, h(&c["vy"])) {
            hata.push(format!("ivme {} {}: {} {} js {} {}", p.govde.name, p.govde.role, p.vx, p.vy, h(&c["vx"]), h(&c["vy"])));
        }
    }
    eprintln!("ivme {}", v["ivme"].as_array().unwrap().len());
    assert!(hata.is_empty(), "{} hata:\n{}", hata.len(), hata[..hata.len().min(20)].join("\n"));
}

#[test]
fn ikili_katsayilari_js_ile_ayni() {
    let Some(v) = vektor() else { return };
    let mut hata = vec![];
    for c in v["ik"].as_array().unwrap() {
        let p = oyuncu(&c["p"]);
        let ph = c["ph"].as_bool().unwrap();
        let t = h(&c["m"]["tick"]);
        let mut f = |n: &str, a: f64, j: &serde_json::Value| {
            if !ayni(a, h(j)) {
                hata.push(format!("t{t} {} {n}: {a} js {}", p.govde.name, h(j)));
            }
        };
        f("kutle", kutle(&p, ph), &c["kutle"]);
        f("direnc", direnc(&p, ph), &c["direnc"]);
        f("omuzK", omuz_k(&p), &c["omuzK"]);
        f("ikEn", ik_en(&p), &c["ikEn"]);
        f("sendS", send_s(&p), &c["sendS"]);
        f("dengeKayip", denge_kayip(&p, t), &c["dk"]);
        if !c["h"].is_null() {
            let hh = oyuncu(&c["h"]);
            f("durtFark", durt_fark(&p, &hh), &c["durt"]);
        }
        if !ph {
            let mut m = Mac::default();
            m.tick = t;
            m.ps = vec![p.clone()];
            f("dengeHiz", denge_hiz(&m, 0), &c["dh"]);
        }
    }
    eprintln!("ikili {}", v["ik"].as_array().unwrap().len());
    assert!(hata.is_empty(), "{} hata:\n{}", hata.len(), hata[..hata.len().min(20)].join("\n"));
}

#[test]
fn govde_ayir_js_ile_ayni() {
    let Some(v) = vektor() else { return };
    let mut hata = vec![];
    for (k, c) in v["gov"].as_array().unwrap().iter().enumerate() {
        let mut ps: Vec<_> = c["ps"].as_array().unwrap().iter().map(oyuncu).collect();
        let hi = c["h"].as_i64().unwrap();
        let h_ = if hi < 0 { None } else { Some(hi as usize) };
        govde_ayir(&mut ps, Some(h_), &mut IkBag::default());
        for (i, s) in c["son"].as_array().unwrap().iter().enumerate() {
            let p = &ps[i];
            if !ayni(p.x, h(&s[0])) || !ayni(p.y, h(&s[1])) || !ayni(p.vx, h(&s[2])) || !ayni(p.vy, h(&s[3])) || !ayni_o(p.den, &s[4]) {
                hata.push(format!("govdeAyir {k} oyuncu {i}: ({} {} {} {} {:?}) js {s}", p.x, p.y, p.vx, p.vy, p.den));
            }
        }
    }
    eprintln!("govdeAyir {}", v["gov"].as_array().unwrap().len());
    assert!(hata.is_empty(), "{} hata:\n{}", hata.len(), hata[..hata.len().min(20)].join("\n"));
}
