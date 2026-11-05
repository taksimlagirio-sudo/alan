//! Kafa1 (decide.js 1–4716): JS ile bit bit aynılık (sina/kafa1-vektor.js, gerçek maçtan anlık durumlar).
//! Dış dünya (maç, yerleşim, enerji) kayıttan ya da buraya taşınmış küçük işlevlerden (KayitDis).
#![allow(non_snake_case)]
use alan_motor::kafa::gecici_dis1::*;
use alan_motor::kafa::kafa1_js::{nk, Nk, Op};
use alan_motor::kafa::kafa1_ucus::UcusB0;
use alan_motor::kafa::kafa1_varis::Lane;
use alan_motor::kafa::Kafa;
use alan_motor::mat::hypot;
use alan_motor::model::{JMShGs, JPHamle, JPPplan, JP_gvKaz, JP_sf, Oyuncu};
use alan_motor::yetenek::{Ham, Yetenek};
use serde_json::Value;
use std::collections::HashMap;
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
fn ayni(a: f64, b: f64) -> bool {
    a.to_bits() == b.to_bits() || (a != a && b != b)
}

#[derive(Clone)]
struct MacK {
    ch: Option<f64>,
    gs: Option<JMShGs>,
    tac: Vec<(Option<f64>, bool, f64)>,
    prT: Option<PrT>,
}
struct KayitDis {
    mac: Option<MacK>,
}
impl Dis for KayitDis {
    fn S(&self) -> Option<ShapeS> {
        Some(ShapeS { react: Some(0.5), rcvBuf: 3.0, marginT: 0.6, markOff: Some(2.5), bekOnce: Some(2.0), bekSerbest: true })
    }
    fn GR(&self) -> Option<ShapeGR> {
        Some(ShapeGR { yasH: 40.0 })
    }
    fn gside(&self, a: Nk, t: f64, off: f64) -> Nk {
        let gx = if t == 0.0 { 0.0 } else { 120.0 };
        let dx = gx - a.x;
        let dy = 30.0 - a.y;
        let L = {
            let l = hypot(dx, dy);
            if l != 0.0 && l == l {
                l
            } else {
                1.0
            }
        };
        nk(a.x + dx / L * off, a.y + dy / L * off)
    }
    fn AS_ilkPres_var(&self) -> bool {
        true
    }
    fn mac_var(&self) -> bool {
        self.mac.is_some()
    }
    fn mac_ch(&self) -> Option<f64> {
        self.mac.as_ref().and_then(|m| m.ch)
    }
    fn mac_holder(&self) -> Option<(f64, f64, f64, f64)> {
        None
    }
    fn tac_enHarcama(&self, _t: f64) -> Option<f64> {
        None
    }
    fn tac_siddet(&self, team: f64) -> Option<(Option<f64>, bool, f64)> {
        self.mac.as_ref().map(|m| m.tac[team as usize])
    }
    fn sh_gs(&self) -> Option<JMShGs> {
        self.mac.as_ref().and_then(|m| m.gs.clone())
    }
    fn prT(&self) -> Option<PrT> {
        self.mac.as_ref().and_then(|m| m.prT.clone())
    }
    fn algi_e(&self, _p: &Oyuncu) -> Option<f64> {
        None
    }
    fn algi_yas(&self, _p: &Oyuncu) -> Option<f64> {
        None
    }
}

fn oyuncu(v: &Value) -> Oyuncu {
    let mut a = Ham::default();
    for (k, x) in v["a"].as_object().unwrap() {
        a.v.insert(k.clone(), h(x));
    }
    let mut p = Oyuncu::default();
    p.id = v["id"].as_f64().unwrap();
    p.team = v["team"].as_f64().unwrap();
    p.name = v["name"].as_str().unwrap_or("").into();
    p.role = if v["bek"].as_bool().unwrap() { "Bekçi".into() } else { "Orta".into() };
    p.yet = if v["a"].as_object().unwrap().is_empty() { Yetenek::bos().into() } else { Yetenek::kur(&a).into() };
    p.x = h(&v["x"]);
    p.y = h(&v["y"]);
    p.vx = h(&v["vx"]);
    p.vy = h(&v["vy"]);
    p.tx = h(&v["tx"]);
    p.ty = h(&v["ty"]);
    p.en = ho(&v["en"]);
    p.den = ho(&v["den"]);
    p.noTouch = v["noTouch"].as_bool().unwrap();
    p.hal_mut().sonuk = v["sonuk"].as_bool().unwrap();
    p.hal_mut().kuyuT = ho(&v["kuyuT"]);
    p.R = ho(&v["R"]).unwrap_or(0.0);
    p.D = ho(&v["D"]).unwrap_or(1.0);
    p.hal_mut().amA = v["amA"].as_bool().unwrap();
    p.hal_mut().am = ho(&v["am"]);
    p.ca = ho(&v["ca"]);
    p._don = ho(&v["don"]);
    p._dke = ho(&v["dke"]);
    p.sprint = v["sprint"].as_bool().unwrap();
    p.job = v["job"].as_str().map(|s| s.into());
    p.press = v["press"].as_bool().unwrap();
    p._sf = if v["sf"].is_null() { None } else { Some(JP_sf { t0: h(&v["sf"]["t0"]), f: v["sf"]["f"].as_str().unwrap().into() }) };
    p.hal_mut()._korT = ho(&v["korT"]);
    p.hal_mut().hamle = if v["hamle"].is_null() { None } else { Some(JPHamle { k: v["hamle"]["k"].as_str().unwrap().into(), hid: v["hamle"]["hid"].as_f64().unwrap(), t0: h(&v["hamle"]["t0"]), W: h(&v["hamle"]["W"]) }) };
    p.pplan = v["pplan"].as_str().map(|k| JPPplan { k: k.into(), ..Default::default() });
    p.hal_mut()._gvKaz = if v["gvKaz"].is_null() { None } else { Some(JP_gvKaz { h: h(&v["gvKaz"][0]), t: h(&v["gvKaz"][1]), kaz: h(&v["gvKaz"][2]) }) };
    p.hal_mut().fiskeCd = ho(&v["fiskeCd"]);
    p.hal_mut().sendT = ho(&v["sendT"]);
    p.hal_mut().sendCd = ho(&v["sendCd"]);
    p.hal_mut().stun = ho(&v["stun"]);
    p.hal_mut().adandi = ho(&v["adandi"]);
    p.hal_mut()._kal = v["kal"].as_bool().unwrap();
    p.hal_mut()._kalH = v["kalH"].as_u64().map(|x| x as usize);
    p.hal_mut().tutQ = v["tutQ"].as_u64().map(|x| x as usize);
    p.hal_mut().markRef = v["markRef"].as_u64().map(|x| x as usize);
    p.hal_mut().sarj = Some(v["sarj"].as_bool().unwrap_or(true));
    p.hal_mut()._yer = v["yer"].as_bool().unwrap();
    p
}

struct Hata {
    say: HashMap<&'static str, (usize, usize)>,
    ilk: Vec<String>,
}
impl Hata {
    fn bak(&mut self, ad: &'static str, rs: f64, js: f64, ek: &str) {
        let e = self.say.entry(ad).or_insert((0, 0));
        e.0 += 1;
        if !ayni(rs, js) {
            e.1 += 1;
            if self.ilk.len() < 400 {
                self.ilk.push(format!("{ad} {ek}: rust {rs:e} js {js:e}"));
            }
        }
    }
    fn evet(&mut self, ad: &'static str, ok: bool, ek: String) {
        let e = self.say.entry(ad).or_insert((0, 0));
        e.0 += 1;
        if !ok {
            e.1 += 1;
            if self.ilk.len() < 400 {
                self.ilk.push(format!("{ad} {ek}"));
            }
        }
    }
}

#[test]
#[ignore = "JS karşılaştırması: vektör sina/kafa1-vektor.js ile üretilince `cargo test --release -- --ignored`; Rust tek doğru kaynak (MIMARI.md Aşama 6)"]
fn js_ile_bit_bit_ayni() {
    let yol = concat!(env!("CARGO_MANIFEST_DIR"), "/../sina/kafa1-vektor.json");
    let s = match std::fs::read_to_string(yol) {
        Ok(s) => s,
        Err(_) => {
            eprintln!("kafa1-vektor.json yok (sina/kafa1-vektor.js ile üretilir): sınama atlandı");
            return;
        }
    };
    let v: Value = serde_json::from_str(&s).unwrap();
    let mut H = Hata { say: HashMap::new(), ilk: Vec::new() };
    for c in v.as_array().unwrap() {
        let tk = c["tick"].as_f64().unwrap();
        let pj = c["ps"].as_array().unwrap();
        let ps: Vec<Op> = pj.iter().map(|x| Arc::new(oyuncu(x))).collect();
        let dis = Arc::new(KayitDis { mac: None });
        let mut k = Kafa::yeni(dis);
        k.bag._tick = Some(tk);
        k.bag._gon = Some(false);
        k.bag._sut = Some(alan_motor::model::JsDeger::Sayi(0.5));
        k.bag._ot = Some(1.0);
        k.bag._savT = Some(0.0);
        k.bag._sYon = Some(0.0);
        k.bag._kenar = Some(false);
        let hi = c["h"].as_u64().unwrap() as usize;
        let hh: &Oyuncu = &ps[hi];
        let team = hh.team;
        let gx = if team == 0.0 { 120.0 } else { 0.0 };
        let et = format!("t{tk}");
        for t in c["tan"].as_array().unwrap() {
            H.bak("tan", alan_motor::kafa::kafa1_js::tan(h(&t[0])), h(&t[1]), &et);
        }
        for t in c["thr"].as_array().unwrap() {
            H.bak("threat", k.threat(t[0].as_f64().unwrap(), h(&t[1]), h(&t[2])), h(&t[3]), &et);
        }
        for t in c["ut"].as_array().unwrap() {
            H.bak("ucusT", k.ucusT(h(&t[0]), h(&t[1]), ho(&t[2])), h(&t[3]), &et);
        }
        for t in c["yer"].as_array().unwrap() {
            let (x, y, ch) = (h(&t[0]), h(&t[1]), h(&t[2]));
            H.bak("pressAt", k.pressAt(&ps, team, x, y), h(&t[3]), &et);
            H.bak("freeAt", k.freeAt(&ps, team, x, y), h(&t[4]), &et);
            H.bak("holdVal", k.holdVal(&ps, team, x, y), h(&t[5]), &et);
            H.bak("V", k.V(&ps, team, x, y, ch), h(&t[6]), &et);
            H.bak("shotQ", k.shotQ(&ps, team, x, y, ch), h(&t[7]), &et);
            H.bak("sutKapali", k.sutKapali(&ps, team, x, y, ch), h(&t[8]), &et);
            H.bak("oppAt", k.oppAt(&ps, team, x, y), h(&t[9]), &et);
        }
        {
            let r = k.sutHedef(hh, &ps, gx);
            let j: Vec<f64> = c["sh"].as_array().unwrap().iter().map(h).collect();
            H.evet("sutHedef", r.len() == j.len() && r.iter().zip(&j).all(|(a, b)| ayni(*a, *b)), format!("{et} {r:?} js {j:?}"));
        }
        for t in c["lo"].as_array().unwrap() {
            let q: &Oyuncu = &ps[t[0].as_u64().unwrap() as usize];
            H.bak("laneOk", k.laneOk(&ps, team, nk(hh.x, hh.y), nk(q.x, q.y), Some(h(&t[1])), Some(h(&t[2]))), h(&t[3]), &et);
        }
        H.bak("space", k.space(&ps, team, hh.x, hh.y), h(&c["spf"][0]), &et);
        H.bak("front", k.front(&ps, team, hh.x, hh.y), h(&c["spf"][1]), &et);
        for t in c["rs"].as_array().unwrap() {
            let r = k.respond(&ps, team, nk(h(&t[1]), h(&t[2])), h(&t[3]), t[0].as_str().unwrap());
            let j = t[4].as_array().unwrap();
            let ok = r.len() == j.len() && r.iter().zip(j).all(|(a, b)| match (a, b.is_null()) {
                (None, true) => true,
                (Some(p), false) => ayni(p.x, h(&b[0])) && ayni(p.y, h(&b[1])),
                _ => false,
            });
            H.evet("respond", ok, format!("{et} {r:?} js {j:?}"));
        }
        {
            let cs = k.coreSide(hh);
            H.bak("coreSide", cs.ux, h(&c["cs"][0]), &et);
            H.bak("coreSide", cs.uy, h(&c["cs"][1]), &et);
            H.bak("coreSide", cs.off, h(&c["cs"][2]), &et);
            H.bak("trapped", k.trapped(hh, &ps, 1.0 - team), h(&c["trap"]), &et);
        }
        for t in c["ik"].as_array().unwrap() {
            let q: &Oyuncu = &ps[t[0].as_u64().unwrap() as usize];
            let e2 = format!("{et} q{}", t[0]);
            H.bak("duelP", k.duelP(q, hh, &ps, None), h(&t[1]), &e2);
            H.bak("govdeVurP", k.govdeVurP(q, hh, None, false), h(&t[2]), &e2);
            H.bak("commitThr", k.commitThr(q, hh, &ps), h(&t[3]), &e2);
            H.bak("girThr", k.girThr(q, hh, &ps), h(&t[4]), &e2);
        }
        for t in c["kp"].as_array().unwrap() {
            let dir = if t[0].is_null() { None } else { Some(nk(h(&t[0][0]), h(&t[0][1]))) };
            let ad = if dir.is_some() { "keepP(yön, kesisK)" } else { "keepP" };
            let r = k.keepP(hh, &ps, dir, h(&t[1]));
            H.bak(ad, r, h(&t[2]), &et);
            H.bak(if dir.is_some() { "keepP.son(yön)" } else { "keepP.son" }, k.keepP_son.get(), h(&t[3]), &et);
        }
        for u in c["uf"].as_array().unwrap() {
            let gon = u["gon"].as_bool().unwrap();
            let b = &u["b0"];
            k.bag._gon = Some(gon);
            k.bag._sut = Some(if gon { alan_motor::model::JsDeger::Metin("güçlü".into()) } else { alan_motor::model::JsDeger::Sayi(0.5) });
            let r = k.ucusFormul(hh, &ps, UcusB0 { x: h(&b[0]), y: h(&b[1]), vx: h(&b[2]), vy: h(&b[3]), ch: h(&b[4]), aim: None }, None, None, None);
            k.bag._gon = Some(false);
            k.bag._sut = Some(alan_motor::model::JsDeger::Sayi(0.5));
            let rv = [r.P, r.Pk, r.Pl, r.end.x, r.end.y, r.endM.x, r.endM.y, r.endO.x, r.endO.y, r.Ps, r.endS.x, r.endS.y, r.endD.x, r.endD.y, r.Psek, r.endSek.x, r.endSek.y];
            let ad = if gon { "ucusFormul(şut)" } else { "ucusFormul(pas)" };
            for (i, x) in rv.iter().enumerate() {
                H.bak(ad, *x, h(&u["r"][i]), &format!("{et} alan {i}"));
            }
        }
        for t in c["lf"].as_array().unwrap() {
            let r = k.launchFor(hh, nk(h(&t[0]), h(&t[1])), h(&t[2]), &ps, h(&t[3]), None);
            H.bak("launchFor", r.vx, h(&t[4]), &et);
            H.bak("launchFor", r.vy, h(&t[5]), &et);
            H.evet("launchFor.ok", r.ok == t[6].as_bool().unwrap(), et.clone());
        }
        for t in c["bf"].as_array().unwrap() {
            let r = k.bankFor(hh, nk(h(&t[0]), h(&t[1])), nk(h(&t[2]), h(&t[3])), h(&t[4]), &ps, h(&t[5]), None);
            H.bak("bankFor", r.vx, h(&t[6]), &et);
            H.bak("bankFor", r.vy, h(&t[7]), &et);
            H.evet("bankFor.ok", r.ok == t[8].as_bool().unwrap(), et.clone());
        }
        for t in c["sp"].as_array().unwrap() {
            let V = nk(h(&t[0]), h(&t[1]));
            let it = t[3].as_bool().unwrap();
            let r = k.savurPlan(hh, V, h(&t[2]), 0.0, 0.0, Some(&ps), it, 1.0);
            match (r, t[4].is_null()) {
                (None, true) => H.evet("savurPlan", true, String::new()),
                (Some(r), false) => {
                    for (i, x) in [r.s, r.caR, r.A, r.P, r.T, r.wR, r.wRel, r.uSh, r.k].iter().enumerate() {
                        H.bak("savurPlan", *x, h(&t[4][i]), &format!("{et} {i}"));
                    }
                }
                _ => H.evet("savurPlan", false, format!("{et} var/yok")),
            }
            let a = alan_motor::mat::atan2(V.y, V.x);
            let _ = a;
            let r2 = k.onPlan(hh, V, nk(h(&t[6]), h(&t[7])), it, 1.0);
            match (r2, t[5].is_null()) {
                (None, true) => H.evet("onPlan", true, String::new()),
                (Some(r), false) => {
                    for (i, x) in [r.s, r.wS, r.phi, r.caR, r.kal, r.Tsw].iter().enumerate() {
                        H.bak("onPlan", *x, h(&t[5][i]), &format!("{et} {i}"));
                    }
                }
                _ => H.evet("onPlan", false, format!("{et} var/yok")),
            }
        }
        {
            let ae = &c["ae"];
            let rr: Vec<f64> = ae[0].as_array().unwrap().iter().map(h).collect();
            let mut ri = 0;
            let mut rnd = || {
                let v = rr[ri];
                ri += 1;
                v
            };
            let r1 = k.applyError(hh, nk(1.2, 0.4), &mut rnd, 1.0, false, Some(&ps), None, None);
            let r2 = k.applyErrorSav(hh, nk(1.2, 0.4), &mut rnd, 1.0, true, 0.7, 0.8, Some(&ps), Some("güçlü"), None);
            H.bak("applyError", r1.x, h(&ae[1]), &et);
            H.bak("applyError", r1.y, h(&ae[2]), &et);
            H.bak("applyErrorSav", r2.x, h(&ae[3]), &et);
            H.bak("applyErrorSav", r2.y, h(&ae[4]), &et);
        }
        {
            let fk = &c["fk"];
            H.bak("fakeT", k.fakeT(hh), h(&fk["T"]), &et);
            for t in fk["p"].as_array().unwrap() {
                let q: &Oyuncu = &ps[t[0].as_u64().unwrap() as usize];
                let sh = alan_motor::kafa::kafa1_fake::FakeSh { k: t[1].as_str().unwrap().into(), to: nk(h(&t[2]), h(&t[3])), c: h(&t[4]) };
                H.bak("fakeP", k.fakeP(hh, q, &sh.k, Some(sh.c)), h(&t[5]), &et);
                let fh = k.fakeHedef(q, hh, &sh);
                H.bak("fakeHedef", fh.x, h(&t[6]), &et);
                H.bak("fakeHedef", fh.y, h(&t[7]), &et);
                H.evet("fakeIlgili", k.fakeIlgili(q, hh, &sh) == t[8].as_bool().unwrap(), et.clone());
            }
        }
        {
            let sj = &c["sec"];
            let T = h(&sj["T"]);
            let os: Vec<alan_motor::model::Secenek> = sj["os"].as_array().unwrap().iter().map(|x| alan_motor::model::Secenek {
                kind: x[0].as_str().unwrap().into(),
                to: if x[1].is_null() { None } else { Some(alan_motor::model::Nokta { x: h(&x[1][0]), y: h(&x[1][1]) }) },
                q: x[2].as_u64().map(|i| i as usize),
                v: Some(h(&x[3])),
                det: if x[4].is_null() { None } else { Some(alan_motor::model::JODet { P: Some(h(&x[4])), ..Default::default() }) },
                shot: x[5].as_str().map(|s| s.into()),
                sYon: x[6].as_f64(),
                hold: ho(&x[7]),
                ..Default::default()
            }).collect();
            for (o, n) in os.iter().zip(sj["n"].as_array().unwrap()) {
                H.bak("secimN", k.secimN(hh, Some(o), &ps, T), h(n), &format!("{et} {}", o.kind));
            }
            H.bak("secimW", k.secimW(hh, 0.4, true, None), h(&sj["w"][0]), &et);
            H.bak("secimW", k.secimW(hh, 0.7, false, Some("secenekTur")), h(&sj["w"][1]), &et);
            let i = k.secim(hh, &os, Some(0), T, &ps);
            H.evet("secim", i == sj["i"].as_u64().map(|x| x as usize), format!("{et} {i:?} js {}", sj["i"]));
        }
        for t in c["cr"].as_array().unwrap() {
            H.bak("chargeRate", k.chargeRate(hh, &ps, h(&t[0]), h(&t[2]), t[1].as_bool().unwrap()), h(&t[3]), &et);
        }
        {
            let cp = &c["cp"];
            let pts: Vec<Nk> = cp["pts"].as_array().unwrap().iter().map(|p| nk(h(&p[0]), h(&p[1]))).collect();
            let opp: Vec<Op> = ps.iter().filter(|q| q.team != team).cloned().collect();
            let r = k.chasePlan(&pts, &opp, 0.0, Some(Lane { lag: 0.0, loose: false }));
            let j = cp["r"].as_array().unwrap();
            let ok = r.len() == j.len()
                && r.iter().zip(j).all(|((pi, g), jj)| {
                    let p = &opp[*pi];
                    let jp = &ps[jj[0].as_u64().unwrap() as usize];
                    p.id == jp.id && p.team == jp.team && ayni(g.t, h(&jj[1])) && ayni(g.x, h(&jj[2])) && ayni(g.y, h(&jj[3])) && ayni(g.rc, h(&jj[4]))
                });
            H.evet("chasePlan", ok, format!("{et} {} js {}", r.len(), j.len()));
        }
        {
            let mj = &c["mac"];
            let prT = if mj["prT"].is_null() {
                None
            } else {
                let p = &mj["prT"];
                Some(PrT {
                    team: p["team"].as_f64().unwrap(),
                    hId: p["hId"].as_f64(),
                    t: h(&p["t"]),
                    h: ho(&p["h"]),
                    hm: p["hm"].as_array().map(|a| a.iter().map(|e| (e[0].as_f64().unwrap(), h(&e[1]))).collect()),
                    hmk: p["hmk"].as_array().map(|a| a.iter().map(|e| e.as_f64().unwrap()).collect()),
                })
            };
            let gs = if mj["gs"].is_null() { None } else { Some(JMShGs { t: h(&mj["gs"]["t"]), dt: mj["gs"]["dt"].as_f64().unwrap(), oy: mj["gs"]["oy"].as_bool().unwrap(), ..Default::default() }) };
            let tac = mj["tac"].as_array().unwrap().iter().map(|t| (ho(&t[0]), t[1].as_bool().unwrap(), h(&t[2]))).collect();
            let dis2 = Arc::new(KayitDis { mac: Some(MacK { ch: ho(&mj["ch"]), gs, tac, prT }) });
            let k2 = Kafa::yeni(dis2);
            let mut k2 = k2;
            k2.bag._tick = Some(c["mtick"].as_f64().unwrap() + 0.5);
            for e in mj["vk"].as_array().unwrap() {
                k2.ge_vk_koy(e[0].as_str().unwrap().into(), h(&e[1]));
            }
            for t in mj["ge"].as_array().unwrap() {
                let q: &Oyuncu = &ps[t[0].as_u64().unwrap() as usize];
                let e2 = format!("{et} q{}", t[0]);
                let g = k2.girEsik(q, hh, &ps, None);
                H.bak("girEsik(maç)", g.unwrap_or(f64::NAN), ho(&t[1]).unwrap_or(f64::NAN), &e2);
                H.bak("girThr(maç)", k2.girThr(q, hh, &ps), h(&t[2]), &e2);
                H.bak("commitThr(maç)", k2.commitThr(q, hh, &ps), h(&t[3]), &e2);
            }
        }
    }
    let mut ad: Vec<_> = H.say.iter().collect();
    ad.sort_by_key(|x| *x.0);
    for (a, (n, e)) in &ad {
        eprintln!("{a:22} {n:6} sınama, {e} fark");
    }
    assert!(H.ilk.is_empty(), "{} fark:\n{}", H.ilk.len(), H.ilk.join("\n"));
}
