//! Maçın enerji ve sprint işlevleri: JS ile bit bit aynılık (macW, macEff, enerji2, sprint katı, koşu süresi, vK, Kuyu çıkışı).
mod mac_yardim;
use alan_motor::mac::enerji::*;
use alan_motor::mac::kuyu::{kuyu_cik_t, kuyu_cik_yon};
use alan_motor::mac::tac::{mac_eff, mac_w};
use alan_motor::mac::Tac;
use alan_motor::model::Mac;
use mac_yardim::*;

#[test]
fn macw_maceff_js_ile_ayni() {
    let Some(v) = vektor() else { return };
    let mut hata = vec![];
    for c in v["macW"].as_array().unwrap() {
        let w = mac_w(c[0].as_f64().unwrap(), c[1].as_f64().unwrap());
        if !ayni(w.g, h(&c[2])) || !ayni(w.c, h(&c[3])) || !ayni(w.e0, h(&c[4])) {
            hata.push(format!("macW {c}: {w:?}"));
        }
    }
    for c in v["macEff"].as_array().unwrap() {
        let mut m = Mac::default();
        m.len = h(&c[0]);
        m.tick = h(&c[1]);
        m.score = c[2].as_array().unwrap().iter().map(|x| x.as_f64().unwrap()).collect();
        let r = mac_eff(&mut m, c[3].as_f64().unwrap(), Tac::default());
        if !ayni_o(r.macK, &c[4]) {
            hata.push(format!("macEff.macK {c}: {:?}", r.macK));
        }
        let z = r.macZ.unwrap();
        for (i, dt) in [0.0, 30.0, 300.0, 1200.0].iter().enumerate() {
            if !ayni(z.zaman(*dt), h(&c[5][i])) {
                hata.push(format!("macZ({dt}) {c}: {}", z.zaman(*dt)));
            }
        }
    }
    eprintln!("macW {} macEff {}", v["macW"].as_array().unwrap().len(), v["macEff"].as_array().unwrap().len());
    assert!(hata.is_empty(), "{} hata:\n{}", hata.len(), hata[..hata.len().min(20)].join("\n"));
}

#[test]
fn enerji2_bir_tik_js_ile_ayni() {
    let Some(v) = vektor() else { return };
    let mut hata = vec![];
    for c in v["en2"].as_array().unwrap() {
        let mut m = mac(&c["m"]);
        m.mac._lite = c["lite"].as_bool().unwrap();
        enerji2(&mut m);
        for (i, s) in c["son"].as_array().unwrap().iter().enumerate() {
            let p = &m.ps[i];
            let r = [("en", p.en), ("D", Some(p.D)), ("_Dl", p.hal._Dl), ("_D0", p.hal._D0), ("_yog", Some(p.hal._yog)), ("_enOnce", p.hal._enOnce), ("_bosT", p.hal._bosT), ("_evx", Some(p.hal._evx)), ("_evy", Some(p.hal._evy))];
            for ((ad, x), j) in r.iter().zip(s.as_array().unwrap()) {
                if !ayni_o(*x, j) {
                    hata.push(format!("t{} oyuncu {i} {ad}: {:?} js {:?}", h(&c["m"]["tick"]), x, ho(j)));
                }
            }
        }
    }
    eprintln!("enerji2 {} anlık", v["en2"].as_array().unwrap().len());
    assert!(hata.is_empty(), "{} hata:\n{}", hata.len(), hata[..hata.len().min(20)].join("\n"));
}

#[test]
fn sprint_kati_ve_kosu_js_ile_ayni() {
    let Some(v) = vektor() else { return };
    let mut hata = vec![];
    let mut n = 0;
    for c in v["spr"].as_array().unwrap() {
        let m = mac(&c["m"]);
        let k = alan_motor::mac::mac_kurali(&m);
        for (i, pj) in c["per"].as_array().unwrap().iter().enumerate() {
            let p = &m.ps[i];
            let d = h(&pj["d"]);
            let pr = kosuPr(p, &k);
            let prj = &pj["pr"];
            let pr_ok = match (pr, prj.is_null()) {
                (None, true) => true,
                (Some(r), false) => ayni(r.Ts, h(&prj[0])) && ayni(r.k, h(&prj[1])),
                _ => false,
            };
            let mut f = |a: &str, ok: bool| {
                n += 1;
                if !ok {
                    hata.push(format!("t{} oyuncu {i} {a}", h(&c["m"]["tick"])));
                }
            };
            f("ALAN_SPR", ayni(alan_spr(p, &k), h(&pj["spr"])));
            f("sprK", ayni(spr_k(p, &k), h(&pj["sprK"])));
            f("ALAN_SPRE(L)", ayni(alan_spre(p, Some(d), &k), h(&pj["spre"])));
            f("ALAN_SPRE", ayni(alan_spre(p, None, &k), h(&pj["spre0"])));
            f("kosuPr", pr_ok);
            f("ALAN_KOSU_T", ayni(alan_kosu_t(p, d, spd(p), &k), h(&pj["kt"])));
            f("ALAN_KOSU_M", ayni(alan_kosu_m(p, d / 3.0, spd(p), &k), h(&pj["km"])));
            f("vK", ayni(v_k(p, m.holder == Some(i), m.tick, &k), h(&pj["vK"])));
            f("SGIR", ayni(alan_en3_sgir(p, 0.0), h(&pj["sgir"])));
        }
    }
    eprintln!("sprint: {n} karşılaştırma");
    assert!(hata.is_empty(), "{} hata:\n{}", hata.len(), hata[..hata.len().min(20)].join("\n"));
}

#[test]
fn kuyu_cikisi_js_ile_ayni() {
    let Some(v) = vektor() else { return };
    let mut hata = vec![];
    for c in v["kuyu"].as_array().unwrap() {
        let m = mac(&c["m"]);
        let k = alan_motor::mac::mac_kurali(&m);
        let pi = c["p"].as_u64().unwrap() as usize;
        let y = kuyu_cik_yon(&m, pi);
        let ok = match (y, c["Y"].is_null()) {
            (None, true) => true,
            (Some(y), false) => [y.k, y.ux, y.uy, y.L, y.x, y.y].iter().enumerate().all(|(i, x)| ayni(*x, h(&c["Y"][i]))),
            _ => false,
        };
        if !ok {
            hata.push(format!("kuyuCikYon {:?} js {}", y, c["Y"]));
        }
        let t = kuyu_cik_t(&m, pi, None, &k);
        if !ayni(t, h(&c["T"])) {
            hata.push(format!("kuyuCikT {t} js {}", h(&c["T"])));
        }
    }
    eprintln!("kuyu {}", v["kuyu"].as_array().unwrap().len());
    assert!(hata.is_empty(), "{} hata:\n{}", hata.len(), hata[..hata.len().min(20)].join("\n"));
}
