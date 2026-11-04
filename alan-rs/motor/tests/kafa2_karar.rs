//! Kafa 2: karar ve dünyalar JS ile (sina/kafa2-karar-vektor.js). Kafa 1, yerleşim ve maç işlevleri
//! (gecici_dis2) gerçeklerine bağlanınca koşar: `cargo test --release -- --ignored kafa2`.
use alan_motor::kafa::kafa2_baglam::*;
use alan_motor::kafa::{kafa2_dunya, kafa2_imkan, kafa2_karar, kafa2_zemin};
use alan_motor::model::{JMTacE, JPEg, JPTl, JP_kv, Oyuncu};
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
fn nok(v: &Value) -> Option<Nokta> {
    if v.is_null() {
        None
    } else {
        Some(nk(h(&v["x"]), h(&v["y"])))
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
        let f = match k {
            "hiz" => &mut p.a.hiz,
            "ceviklik" => &mut p.a.ceviklik,
            "guc" => &mut p.a.guc,
            "uzanma" => &mut p.a.uzanma,
            "alan" => &mut p.a.alan,
            "kontrol" => &mut p.a.kontrol,
            "surme" => &mut p.a.surme,
            "pas" => &mut p.a.pas,
            "gonderme" => &mut p.a.gonderme,
            "yukleme" => &mut p.a.yukleme,
            "tepki" => &mut p.a.tepki,
            "gorus" => &mut p.a.gorus,
            "yeralma" => &mut p.a.yeralma,
            "sogukkanlilik" => &mut p.a.sogukkanlilik,
            "kesme" => &mut p.a.kesme,
            "refleks" => &mut p.a.refleks,
            "yerlesim" => &mut p.a.yerlesim,
            "hakimiyet" => &mut p.a.hakimiyet,
            "enerji" => &mut p.a.enerji,
            "govde" => &mut p.a.govde,
            "caliskanlik" => &mut p.a.caliskanlik,
            _ => continue,
        };
        *f = x;
    }
    p.id = v["id"].as_f64().unwrap();
    p.team = v["team"].as_f64().unwrap();
    p.i = v["i"].as_f64().unwrap_or(0.0);
    p.role = v["role"].as_str().unwrap_or("").into();
    p.name = v["name"].as_str().unwrap_or("").into();
    p.yet = if a.v.is_empty() { Yetenek::bos() } else { Yetenek::kur(&a) };
    p.x = h(&v["x"]);
    p.y = h(&v["y"]);
    p.vx = ho(&v["vx"]).unwrap_or(0.0);
    p.vy = ho(&v["vy"]).unwrap_or(0.0);
    p.tx = h(&v["tx"]);
    p.ty = h(&v["ty"]);
    p.en = ho(&v["en"]);
    p.den = ho(&v["den"]);
    p.R = ho(&v["R"]).unwrap_or(0.0);
    p.D = ho(&v["D"]).unwrap_or(1.0);
    p.noTouch = v["noTouch"].as_bool().unwrap();
    p.press = v["press"].as_bool().unwrap();
    p.sprint = v["sprint"].as_bool().unwrap();
    p.job = v["job"].as_str().map(|s| s.to_string());
    p.markRef = v["markRef"].as_u64().map(|x| x as usize);
    p.tutQ = v["tutQ"].as_u64().map(|x| x as usize);
    p._kv = nok(&v["kv"]).map(|n| JP_kv { x: n.x, y: n.y, t: 0.0 });
    p.otx = ho(&v["otx"]);
    p.oty = ho(&v["oty"]);
    p.ca = ho(&v["ca"]);
    p.bekBas = ho(&v["bekBas"]);
    p.adandi = ho(&v["adandi"]);
    p.adT = nok(&v["adT"]).map(|n| alan_motor::model::JPAdT { x: n.x, y: n.y });
    p._yer = v["yer"].as_bool().unwrap_or(false);
    if let Some(s) = v["slot"].as_object() {
        p.slot.role = s["role"].as_str().unwrap_or("").into();
        p.slot.d = h(&s["d"]);
        p.slot.l = h(&s["l"]);
    }
    if let Some(t) = v["tl"].as_object() {
        let mut tl = JPTl::default();
        let g = |k: &str| t.get(k).map(h).unwrap_or(0.5);
        tl.huc.kosu = g("kosu");
        tl.huc.istek = g("istek");
        tl.huc.destek = g("destek");
        tl.huc.risk = g("risk");
        tl.huc.ritim = g("ritim");
        tl.huc.tempo = g("tempo");
        tl.huc.gonder = g("gonder");
        tl.huc.serbest = g("serbest");
        if let Some(sv) = v["tlSav"].as_object() {
            tl.sav.adam = sv.get("adam").map(h).unwrap_or(0.0);
            tl.sav.pres = sv.get("pres").map(h).unwrap_or(0.0);
        }
        p.tl = Some(tl);
    }
    if !v["eg"].is_null() {
        p.eg = Some(JPEg { ps: h(&v["eg"]["ps"]), rk: h(&v["eg"]["rk"]) });
    }
    p
}
fn dunya(v: &Value) -> Dunya {
    v.as_array().unwrap().iter().map(|p| Arc::new(oyuncu(p))).collect()
}
fn tac_e(v: &Value) -> JMTacE {
    let mut t = JMTacE::default();
    t.blok = v["blok"].as_str().unwrap_or("").into();
    t.genislik = v["genislik"].as_str().unwrap_or("").into();
    for (k, f) in [("pres", &mut t.pres), ("onde", &mut t.onde), ("risk", &mut t.risk), ("tempo", &mut t.tempo), ("sabir", &mut t.sabir), ("sut", &mut t.sut), ("pasSurme", &mut t.pasSurme), ("pasBoy", &mut t.pasBoy), ("enDinlen", &mut t.enDinlen), ("enYipratma", &mut t.enYipratma), ("siddet", &mut t.siddet), ("savGen", &mut t.savGen)] {
        if let Some(x) = ho(&v[k]) {
            *f = x;
        }
    }
    t
}
fn kafa(g: &Value) -> Kafa {
    let mut k = Kafa::yeni();
    k.D._tick = ho(&g["D"]["_tick"]);
    if !g["mac"].is_null() {
        let m = &g["mac"];
        k.mac = Some(MacGor { tick: m["tick"].as_f64().unwrap_or(0.0), len: 9000.0, tac: m["tac"].as_array().map(|a| a.iter().map(tac_e).collect()).unwrap_or_default(), ps: dunya(&g["src"]), holder: m["holder"].as_u64().map(|x| x as usize) });
    }
    k
}
fn vektor() -> Option<Value> {
    let yol = concat!(env!("CARGO_MANIFEST_DIR"), "/../sina/kafa2-karar-vektor.json");
    std::fs::read_to_string(yol).ok().map(|s| serde_json::from_str(&s).unwrap())
}

#[test]
#[ignore = "kafa1 / yerleşim / maç işlevleri birleştirmede bağlanınca"]
fn kafa2_dunyalar_js_ile() {
    let v = match vektor() {
        Some(v) => v,
        None => return,
    };
    let mut hata = Vec::new();
    for g in v["bekKay"].as_array().into_iter().flatten() {
        let k = kafa(g);
        let p = Arc::new(oyuncu(&g["p"]));
        let r = kafa2_dunya::bekKay(&k, &p, nok(&g["B"]).unwrap(), h(&g["T"]));
        if !ayni_b(r.x, h(&g["R"][0])) || !ayni_b(r.y, h(&g["R"][1])) {
            hata.push(format!("bekKay t{}", g["tick"]));
        }
    }
    for g in v["predictWorld"].as_array().into_iter().flatten() {
        let mut k = kafa(g);
        let src = dunya(&g["src"]);
        let resp: Option<Vec<usize>> = g["resp"].as_array().map(|a| a.iter().map(|x| x.as_u64().unwrap() as usize).collect());
        let r = kafa2_dunya::predictWorld(&mut k, &src, g["team"].as_f64().unwrap(), nok(&g["B"]).unwrap(), h(&g["T"]), g["ca"].as_bool().unwrap(), resp.as_deref(), g["pasta"].as_bool().unwrap(), nok(&g["tas"]));
        for (i, q) in g["R"].as_array().unwrap().iter().enumerate() {
            if !ayni_b(r[i].x, h(&q[0])) || !ayni_b(r[i].y, h(&q[1])) {
                hata.push(format!("predictWorld t{} oyuncu {i}", g["tick"]));
            }
        }
    }
    for g in v["imkan"].as_array().into_iter().flatten() {
        let mut k = kafa(g);
        let src = dunya(&g["src"]);
        let hq = if g["h"].is_null() { None } else { Some(oyuncu(&g["h"])) };
        let r = kafa2_imkan::imkan(&mut k, &src, g["team"].as_f64().unwrap(), nok(&g["B"]).unwrap(), hq.as_ref(), h(&g["ch"]));
        if !ayni_b(r, h(&g["R"])) {
            hata.push(format!("imkan t{}: {r:e} js {:e}", g["tick"], h(&g["R"])));
        }
    }
    for g in v["sahiplikKur"].as_array().into_iter().flatten() {
        let mut k = kafa(g);
        let src = dunya(&g["src"]);
        let f = kafa2_zemin::sahiplikKur(&mut k, &src, g["team"].as_f64().unwrap(), h(&g["ch"]), nok(&g["B0"]));
        for e in g["R"].as_array().unwrap() {
            let x = f.oku(e[0].as_f64().unwrap(), h(&e[1]), h(&e[2]));
            if !ayni_b(x, h(&e[3])) {
                hata.push(format!("sahiplikKur t{}", g["tick"]));
                break;
            }
        }
    }
    assert!(hata.is_empty(), "{} hata: {:#?}", hata.len(), &hata[..hata.len().min(20)]);
}

#[test]
#[ignore = "kafa1 / yerleşim / maç işlevleri birleştirmede bağlanınca"]
fn kafa2_decide_js_ile() {
    let v = match vektor() {
        Some(v) => v,
        None => return,
    };
    let mut hata = Vec::new();
    for g in v["decide"].as_array().into_iter().flatten() {
        let mut k = kafa(g);
        let src = Arc::new(dunya(&g["src"]));
        let hi = g["hi"].as_i64().unwrap();
        if hi < 0 {
            continue;
        }
        let zar: Vec<f64> = g["zar"].as_array().unwrap().iter().map(h).collect();
        let mut zi = 0;
        let mut rnd = || {
            let v = zar.get(zi).copied().unwrap_or(0.5);
            zi += 1;
            v
        };
        let tac = if g["tac"].is_null() {
            None
        } else {
            let t = &g["tac"];
            Some(Tac { risk: ho(&t["risk"]), tempo: ho(&t["tempo"]), sabir: ho(&t["sabir"]), sut: ho(&t["sut"]), pasSurme: ho(&t["pasSurme"]), pasBoy: ho(&t["pasBoy"]), kontra: t["kontra"].as_bool().unwrap_or(false), macK: ho(&t["macK"]), enDinlen: ho(&t["enDinlen"]), enYipratma: ho(&t["enYipratma"]), temel: Some(tac_e(t)), ..Default::default() })
        };
        let r = kafa2_karar::decide(&mut k, hi as usize, &src, h(&g["ch"]), &mut rnd, tac.as_ref());
        let jo = g["R"]["opts"].as_array().unwrap();
        if r.opts.len() != jo.len() {
            hata.push(format!("decide t{}: {} seçenek, js {}", g["tick"], r.opts.len(), jo.len()));
            continue;
        }
        for (o, j) in r.opts.iter().zip(jo) {
            if o.kind != j["kind"].as_str().unwrap() || !ayni_b(o.v.unwrap(), h(&j["v"])) {
                hata.push(format!("decide t{}: {} {:?} js {} {:e}", g["tick"], o.kind, o.v, j["kind"], h(&j["v"])));
                break;
            }
        }
    }
    assert!(hata.is_empty(), "{} hata: {:#?}", hata.len(), &hata[..hata.len().min(20)]);
}
