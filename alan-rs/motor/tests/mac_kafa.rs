//! Kafa günlüğü: kafaSapma'nın yazıları JS ile aynı (gerçek maçın kayıtlarından).
mod mac_yardim;
use alan_motor::mac::kafa_bag::{kafa_sapma, KafaKayit, Son};
use mac_yardim::*;
use serde_json::Value;

fn s(v: &Value) -> Option<String> {
    v.as_str().map(|x| x.to_string())
}
fn n(v: &Value) -> Option<f64> {
    v.as_f64()
}

#[test]
fn kafa_sapmasi_js_ile_ayni() {
    let Some(v) = vektor() else { return };
    let mut hata = vec![];
    let mut say = 0;
    for c in v["sapma"].as_array().unwrap() {
        let r = &c["r"];
        let so = &r["son"];
        let son = Son { t: so["t"].as_f64().unwrap_or(0.0), santra: so["santra"].is_number(), devam: so["devam"].is_number(), kim: s(&so["kim"]), kim_id: n(&so["kimId"]), kim_t: n(&so["kimT"]), x: n(&so["x"]), y: n(&so["y"]), gol: n(&so["gol"]) };
        let k = KafaKayit {
            t: r["t"].as_f64().unwrap(),
            ad: s(&r["ad"]).unwrap_or_default(),
            id: r["id"].as_f64().unwrap(),
            team: r["team"].as_f64().unwrap(),
            kind: s(&r["kind"]).unwrap_or_default(),
            kendine: r["kendine"].is_number(),
            q: s(&r["q"]),
            qid: n(&r["qid"]),
            shot: s(&r["shot"]),
            ot: r["ot"].is_number(),
            v: n(&r["v"]),
            P: n(&r["P"]),
            Ps: n(&r["Ps"]),
            Pk: n(&r["Pk"]),
            Pl: None,
            hiz: n(&r["hiz"]),
            savT: n(&r["savT"]),
            tf: n(&r["tf"]),
            alis: if r["alis"].is_object() { Some((n(&r["alis"]["x"]), n(&r["alis"]["y"]))) } else { None },
            x: n(&r["x"]),
            y: n(&r["y"]),
            cikis: n(&r["cikis"]),
            hiz_c: n(&r["hizC"]),
            hiz_g: n(&r["hizG"]),
            bozuldu: s(&r["bozuldu"]),
            sek: n(&r["sek"]),
            son: Some(son),
            sapma: vec![],
        };
        let rs = kafa_sapma(&k);
        let js = c["s"].as_array().unwrap();
        say += js.len();
        let a: Vec<String> = rs.iter().map(|x| format!("{}|{}|{}", x.k, x.a, x.y)).collect();
        let b: Vec<String> = js.iter().map(|x| format!("{}|{}|{}", x["k"].as_str().unwrap(), x["a"].as_f64().unwrap(), x["y"].as_str().unwrap())).collect();
        if a != b {
            hata.push(format!("{a:?} js {b:?}"));
        }
    }
    eprintln!("sapma: {} kayıt, {say} sapma", v["sapma"].as_array().unwrap().len());
    let _ = ho;
    assert!(hata.is_empty(), "{} hata:\n{}", hata.len(), hata[..hata.len().min(20)].join("\n"));
}
