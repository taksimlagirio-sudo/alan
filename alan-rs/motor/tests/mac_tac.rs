//! Taktik: tacEff (geçiş, karşı pres, maç durumu, yetişme uyumu), evreBul, gecisGuncelle JS ile aynı.
mod mac_yardim;
use alan_motor::mac::tac::{evreBul, gecisGuncelle, tacEff};
use alan_motor::mac::Tac;
use alan_motor::model::{JMGc, JMKp};
use mac_yardim::*;
use serde_json::Value;

fn gc(v: &Value) -> Option<JMGc> {
    if v.is_null() {
        return None;
    }
    Some(JMGc { team: v["team"].as_f64().unwrap(), t0: h(&v["t0"]), kendi: v["kendi"].as_bool().unwrap(), git: v["git"].as_bool().unwrap(), on: v["on"].as_array().unwrap().iter().map(|x| x.as_f64().unwrap()).collect(), biz: ho(&v["biz"]).unwrap_or(0.0), rakip: ho(&v["rakip"]).unwrap_or(0.0), hizli: v["hizli"].as_bool().unwrap() })
}
fn gc_ayni(a: &Option<JMGc>, v: &Value) -> bool {
    match (a, v.is_null()) {
        (None, true) => true,
        (Some(g), false) => g.team == v["team"].as_f64().unwrap() && ayni(g.t0, h(&v["t0"])) && g.kendi == v["kendi"].as_bool().unwrap() && g.git == v["git"].as_bool().unwrap() && g.hizli == v["hizli"].as_bool().unwrap() && g.on == v["on"].as_array().unwrap().iter().map(|x| x.as_f64().unwrap()).collect::<Vec<_>>() && (v["biz"].is_null() || ayni(g.biz, h(&v["biz"]))) && (v["rakip"].is_null() || ayni(g.rakip, h(&v["rakip"]))),
        _ => false,
    }
}

#[test]
fn taceff_evre_gecis_js_ile_ayni() {
    let Some(v) = vektor() else { return };
    let Some(l) = v["tac"].as_array() else { return };
    let mut hata = vec![];
    for c in l {
        let s = &c["S"];
        let mut m = mac(&s["m"]);
        m.devre = s["devre"].as_bool().unwrap();
        m.tac = s["tac"].as_array().unwrap().iter().map(|t| serde_json::from_value::<Tac>(t.clone()).unwrap()).collect();
        m.tac0 = s["tac0"].as_array().map(|a| a.iter().map(|t| if t.is_null() { None } else { Some(serde_json::from_value::<Tac>(t.clone()).unwrap()) }).collect());
        m.kp = if s["kp"].is_null() { None } else { let k = &s["kp"]; Some(JMKp { team: k["team"].as_f64().unwrap(), ids: k["ids"].as_array().unwrap().iter().map(|x| x.as_f64().unwrap()).collect(), bitis: h(&k["bitis"]), sahip: k["sahip"].as_u64().map(|x| x as usize) }) };
        m.gc = gc(&s["gc"]);
        m.hhKul = s["hhKul"].as_bool().unwrap();
        m._yzBas = ho(&s["yzBas"]);
        m._yzBekle = s["yzBekle"].as_bool().unwrap();
        let mut cc = std::collections::BTreeMap::new();
        for e in s["macC"].as_array().unwrap() {
            cc.insert(e[0].as_str().unwrap().to_string(), alan_motor::mac::MacW { g: h(&e[1]), c: h(&e[2]), e0: h(&e[3]) });
        }
        m._macC = Some(cc);
        let tk = m.tick;
        for t in 0..2 {
            let e = tacEff(&mut m, t as f64);
            let j = &c["r"][t];
            if !ayni_o(e.risk, &j["risk"]) || !ayni_o(e.tempo, &j["tempo"]) || !ayni_o(e.sabir, &j["sabir"]) || e.kontra.unwrap_or(false) != j["kontra"].as_bool().unwrap() || e._kp.as_deref() != j["kp"].as_str() || !ayni_o(e.macK, &j["macK"]) {
                hata.push(format!("t{tk} tacEff {t}: {:?} {:?} {:?} {:?} {:?} {:?} js {j}", e.risk, e.tempo, e.sabir, e.kontra, e._kp, e.macK));
            }
        }
        for t in 0..2 {
            let ev = evreBul(&mut m, t as f64);
            if Some(ev) != c["ev"][t].as_str() {
                hata.push(format!("t{tk} evreBul {t}: {ev} js {}", c["ev"][t]));
            }
        }
        gecisGuncelle(&mut m);
        if !gc_ayni(&m.gc, &c["gc"]) || m.gecis != c["gecis"].as_bool().unwrap() || m.hhKul != c["hhKul"].as_bool().unwrap() || m._yzBekle != c["yzBekle"].as_bool().unwrap() || m.kp.is_some() != c["kpVar"].as_bool().unwrap() {
            hata.push(format!("t{tk} gecisGuncelle: {:?} gecis {} js {} {}", m.gc, m.gecis, c["gc"], c["gecis"]));
        }
    }
    eprintln!("tac {}", l.len());
    assert!(hata.is_empty(), "{} hata:\n{}", hata.len(), hata[..hata.len().min(20)].join("\n"));
}
