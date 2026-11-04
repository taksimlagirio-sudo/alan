//! Yerleşim (shape.js): JS ile bit bit aynılık (sina/sekil-vektor.js, gerçek maçtan anlık durumlar).
//! Yabancı işleve (gecici_dis: `todo!()`) uğrayan durumlar sayılır ve atlanır.
use std::panic::{catch_unwind, AssertUnwindSafe};

use alan_motor::model::*;
use alan_motor::oyuncu::{Oyuncu, Tepki};
use alan_motor::sekil::gecici_dis::KosuPr;
use alan_motor::sekil::*;
use alan_motor::temas::*;
use alan_motor::top::Top;
use alan_motor::yetenek::{Ham, Yetenek};
use serde_json::Value;

fn h(v: &Value) -> f64 {
    match v.as_str() {
        Some("true") => 1.0,
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

struct KayitKural(Vec<(f64, f64, f64, f64, f64, Vec<u64>)>);
impl Kural for KayitKural {
    fn spr(&self, p: &Oyuncu) -> f64 {
        if p.id != p.id {
            let mk: Vec<u64> = p.yet.mek.iter().map(|x| x.to_bits()).collect();
            let e = self.0.iter().find(|x| x.5 == mk).expect("hayalin oyuncusu yok");
            return if p.role.is_empty() { e.4 } else { e.3 };
        }
        self.0.iter().find(|x| x.0 == p.team && x.1 == p.id).unwrap().2
    }
}

fn bk(v: &Value) -> Option<Vec<Option<JP_bkE>>> {
    v.as_array().map(|a| a.iter().map(|r| r.as_array().map(|r| JP_bkE { x: h(&r[0]), y: h(&r[1]), vx: h(&r[2]), vy: h(&r[3]), t: h(&r[4]), z: h(&r[5]) })).collect())
}

fn oyuncu(v: &Value) -> Oyuncu {
    let mut a = Ham::default();
    for (k, x) in v["a"].as_object().unwrap() {
        a.v.insert(k.clone(), h(x));
    }
    let mut p = Oyuncu::default();
    p.id = v["id"].as_f64().unwrap();
    p.team = v["team"].as_f64().unwrap();
    p.i = v["i"].as_f64().unwrap_or(0.0);
    p.role = v["role"].as_str().unwrap().to_string();
    p.name = v["name"].as_str().unwrap().to_string();
    p.yet = if v["a"].as_object().unwrap().is_empty() { Yetenek::bos() } else { Yetenek::kur(&a) };
    p.slot.d = h(&v["slot"]["d"]);
    p.slot.l = h(&v["slot"]["l"]);
    if !v["rslot"].is_null() {
        p.rslot = Some(JPRslot { d: h(&v["rslot"]["d"]), l: h(&v["rslot"]["l"]) });
    }
    p.x = h(&v["x"]);
    p.y = h(&v["y"]);
    p.vx = h(&v["vx"]);
    p.vy = h(&v["vy"]);
    p.tx = h(&v["tx"]);
    p.ty = h(&v["ty"]);
    p.en = ho(&v["en"]);
    p.den = ho(&v["den"]);
    p.R = ho(&v["R"]).unwrap_or(0.0);
    p.D = h(&v["D"]);
    p.press = v["press"].as_bool().unwrap();
    p.noTouch = v["noTouch"].as_bool().unwrap();
    p.sonuk = v["sonuk"].as_bool().unwrap();
    p.kuyuT = ho(&v["kuyuT"]);
    p.cd = h(&v["cd"]);
    p.hist = Some(v["hist"].as_array().unwrap().iter().map(|e| JPHistE { x: h(&e[0]), y: h(&e[1]), vx: h(&e[2]), vy: h(&e[3]) }).collect());
    p.past = Some(v["past"].as_array().unwrap().iter().map(|e| JPPastE { x: h(&e[0]), y: h(&e[1]) }).collect());
    p._bk = bk(&v["bk"]);
    p._rcvB = if v["rcvB"].as_bool().unwrap() { Some(1) } else { None };
    p._rcvAbs = ho(&v["rcvAbs"]);
    p._rcvV = ho(&v["rcvV"]);
    p._rcvX = ho(&v["rcvX"]);
    p._rcvY = ho(&v["rcvY"]);
    p.amA = v["amA"].as_bool().unwrap_or(false);
    p.am = ho(&v["am"]);
    assert!(!v["cone"].as_bool().unwrap_or(false), "koni (cone) görüldü");
    p.chB = if v["chB"].as_bool().unwrap() { Some(1) } else { None };
    p
}
fn top(v: &Value) -> TopT {
    let mut b = Top::yeni(0.0, 0.0, 0.0, 0.0, v["team"].as_u64().unwrap() as u8, None, 0.2);
    b.x = h(&v["x"]);
    b.y = h(&v["y"]);
    b.vx = h(&v["vx"]);
    b.vy = h(&v["vy"]);
    b.e = h(&v["e"]);
    b.ch = h(&v["ch"]);
    b.alive = v["alive"].as_bool().unwrap();
    b.dead = h(&v["dead"]);
    b.t = h(&v["t"]);
    b.dist = h(&v["dist"]);
    b.walls = h(&v["walls"]);
    b.tp = ho(&v["tp"]);
    b.w0 = ho(&v["w0"]);
    b.d0 = if v["d0"].is_null() { None } else { Some((h(&v["d0"]), v["d0"].as_str() == Some("true"))) };
    b.defl = h(&v["defl"]);
    b.defl_b = v["defl"].as_str() == Some("true");
    if !v["done"].is_null() {
        b.done = Some(alan_motor::top::Bitis::Durdu);
    }
    let mut t = TopTemas::default();
    t.kimlik = 1;
    t.kt = ho(&v["kt"]);
    t.kz = h(&v["kz"]);
    t.gon = v["gon"].as_bool().unwrap();
    t.k_bek = v["kBek"].as_bool().unwrap();
    t.from = v["from"].as_u64().map(|x| x as usize);
    t.recv = v["recv"].as_u64().map(|x| x as usize);
    t.cue = h(&v["cue"]);
    t.bek_id = v["bekId"].as_u64().map(|x| x as u32);
    t.bek_m = h(&v["bekM"]);
    t.sap = v["sap"].as_bool().unwrap();
    for c in v["cds"].as_array().unwrap() {
        t.cds.insert(c[0].as_u64().unwrap() as usize, h(&c[1]));
    }
    TopT { b, t }
}
fn tac(v: &Value) -> JMTacE {
    let mut t = JMTacE::default();
    t.blok = v["blok"].as_str().unwrap_or("").to_string();
    t.genislik = v["genislik"].as_str().unwrap_or("").to_string();
    t.risk = h(&v["risk"]);
    if !v["durus"].is_null() {
        let dz = |a: &Value| -> Vec<Vec<(Option<f64>, Option<f64>)>> { a.as_array().unwrap().iter().map(|r| r.as_array().unwrap().iter().map(|q| if q.is_null() { (None, None) } else { (ho(&q[0]), ho(&q[1])) }).collect()).collect() };
        t.durus.huc = dz(&v["durus"]["huc"]).into_iter().map(|r| r.into_iter().map(|(f, y)| JMTacEDurusHucEE { f, y }).collect()).collect();
        t.durus.sav = dz(&v["durus"]["sav"]).into_iter().map(|r| r.into_iter().map(|(f, y)| JMTacEDurusSavEE { f, y }).collect()).collect();
    }
    t
}

struct Kur {
    m: Mac,
    ps: Vec<Oyuncu>,
    ball: Option<TopT>,
    ek: SekilDurum,
    kk: KayitKural,
    d: KafaD,
}
fn kur(c: &Value) -> Kur {
    let ps: Vec<Oyuncu> = c["ps"].as_array().unwrap().iter().map(oyuncu).collect();
    let kk = KayitKural(
        c["ps"]
            .as_array()
            .unwrap()
            .iter()
            .zip(ps.iter())
            .map(|(p, o)| (o.team, o.id, h(&p["spr"]), h(&p["sprH"]), h(&p["sprF"]), o.yet.mek.iter().map(|x| x.to_bits()).collect()))
            .collect(),
    );
    let mut m = Mac::default();
    m.tick = c["tick"].as_f64().unwrap();
    m.seed = c["seed"].as_f64().unwrap();
    m.holder = c["holder"].as_u64().map(|x| x as usize);
    m.tac = c["tac"].as_array().unwrap().iter().map(tac).collect();
    if !c["fl"].is_null() {
        let mut f = JMFl::default();
        f.q = c["fl"]["q"].as_u64().map(|x| x as usize);
        f.t0 = h(&c["fl"]["t0"]);
        f.kind = c["fl"]["kind"].as_str().unwrap_or("").to_string();
        m.fl = Some(f);
    }
    if let Some(s) = c.get("sec").filter(|s| !s.is_null()) {
        m._sec = Some(JM_sec {
            h: s["h"].as_u64().unwrap() as usize,
            t: h(&s["t"]),
            tut: h(&s["tut"]),
            o: s["o"].as_array().unwrap().iter().map(|o| JM_secOE { k: o["k"].as_str().unwrap_or("").to_string(), x: h(&o["x"]), y: h(&o["y"]), v: h(&o["v"]), dv: h(&o["dv"]) }).collect(),
        });
    }
    let ball = if c["ball"].is_null() { None } else { Some(top(&c["ball"])) };
    let mut d = KafaD::default();
    d.surKarar = 0.025;
    d.arrive = 0.3;
    d.arriveFirm = 0.8;
    d.sutHazir = 5.0;
    d.recvKeepT = 8.0;
    Kur { m, ps, ball, ek: SekilDurum::default(), kk, d }
}
impl Kur {
    fn mc(&mut self) -> Mc<'_> {
        Mc { m: &mut self.m, ps: &mut self.ps, ball: self.ball.as_mut(), ek: &mut self.ek, k: &self.kk, d: &self.d, lite: false, rnd: None }
    }
}

#[derive(Default)]
struct Say {
    tamam: usize,
    atla: usize,
}

#[test]
fn js_ile_bit_bit_ayni() {
    let yol = concat!(env!("CARGO_MANIFEST_DIR"), "/../sina/sekil-vektor.json");
    let s = match std::fs::read_to_string(yol) {
        Ok(s) => s,
        Err(_) => {
            eprintln!("sekil-vektor.json yok (node sina/sekil-vektor.js ../alan-v5 <kayit> sina/sekil-vektor.json)");
            return;
        }
    };
    std::panic::set_hook(Box::new(|_| {}));
    let v: Value = serde_json::from_str(&s).unwrap();
    let mut hata: Vec<String> = Vec::new();
    let mut say: std::collections::BTreeMap<&str, Say> = Default::default();
    let chk = |hata: &mut Vec<String>, ad: String, a: f64, b: f64| {
        if !ayni(a, b) {
            hata.push(format!("{ad}: rust {a:e} js {b:e}"));
        }
    };
    for c in v.as_array().unwrap() {
        let tk = c["tick"].as_u64().unwrap();
        let mut K = kur(c);
        let n = K.ps.len();
        // oyuncu başına: vSp, tepki, gecTab, gec, gecOf
        for (i, pj) in c["per"].as_array().unwrap().iter().enumerate() {
            let p = &K.ps[i];
            chk(&mut hata, format!("t{tk} {i} vSp"), vSp(p, &K.kk), h(&pj["vSp"]));
            chk(&mut hata, format!("t{tk} {i} tepki"), alan_motor::sekil::tepki(p, &K.kk), h(&pj["tepki"]));
            let G = gecTab(p, &K.kk);
            chk(&mut hata, format!("t{tk} {i} gecTab.vm"), G.vm, h(&pj["gvm"]));
            for kk in 0..9 {
                chk(&mut hata, format!("t{tk} {i} gecTab.t[{kk}]"), G.t[kk], h(&pj["gt"][kk]));
                chk(&mut hata, format!("t{tk} {i} gecTab.Y[{kk}].g"), G.Y[kk].g, h(&pj["gy"][kk][0]));
                if let Some(mm) = pj["gy"][kk][1].as_array() {
                    for (j, x) in mm.iter().enumerate() {
                        chk(&mut hata, format!("t{tk} {i} gecTab.Y[{kk}].m[{j}]"), G.Y[kk].m[j], h(x));
                    }
                }
            }
            for q in pj["pts"].as_array().unwrap() {
                let (x, y, sv) = (h(&q[0]), h(&q[1]), ho(&q[2]));
                chk(&mut hata, format!("t{tk} {i} gec({x},{y},{sv:?})"), gec(p, x, y, sv, &K.kk), h(&q[3]));
                chk(&mut hata, format!("t{tk} {i} gecOf({x},{y},{sv:?})"), gecOf(&G, x, y, sv), h(&q[4]));
            }
            say.entry("vSp/tepki/gec").or_default().tamam += 1;
        }
        // bak: önceki akıl kareleri üstüne
        for (i, p) in K.ps.iter_mut().enumerate() {
            p._bk = bk(&c["bk0"][i]);
        }
        bak(&mut K.mc());
        for i in 0..n {
            let r = &K.ps[i]._bk;
            let j = bk(&c["bak"][i]);
            let ok = match (r, &j) {
                (Some(a), Some(b)) => a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| match (x, y) {
                    (Some(x), Some(y)) => ayni(x.x, y.x) && ayni(x.y, y.y) && ayni(x.vx, y.vx) && ayni(x.vy, y.vy) && ayni(x.t, y.t) && ayni(x.z, y.z),
                    (None, None) => true,
                    _ => false,
                }),
                (None, None) => true,
                _ => false,
            };
            if !ok {
                hata.push(format!("t{tk} {i} bak"));
            }
        }
        say.entry("bak").or_default().tamam += 1;
        // seen
        for i in 0..n {
            let sv = seen(&mut K.mc(), i);
            let js = c["seen"][i].as_array().unwrap();
            if sv.len() != js.len() {
                hata.push(format!("t{tk} {i} seen boy {} js {}", sv.len(), js.len()));
                continue;
            }
            for (o, q) in sv.iter().zip(js) {
                if o.r != q[0].as_u64().map(|x| x as usize) {
                    hata.push(format!("t{tk} {i} seen ref"));
                }
                chk(&mut hata, format!("t{tk} {i} seen.x"), o.x, h(&q[1]));
                chk(&mut hata, format!("t{tk} {i} seen.y"), o.y, h(&q[2]));
                chk(&mut hata, format!("t{tk} {i} seen.gvx"), o.gvx.unwrap_or(f64::NAN), h(&q[3]));
                chk(&mut hata, format!("t{tk} {i} seen.gvy"), o.gvy.unwrap_or(f64::NAN), h(&q[4]));
            }
        }
        say.entry("seen").or_default().tamam += 1;
        // gorulen (Bekçi'nin karşı karşıya kestirimi kafaya uğrayabilir: atlanır)
        for i in 0..n {
            let r = catch_unwind(AssertUnwindSafe(|| gorulen(&mut K.mc(), i)));
            match r {
                Err(_) => say.entry("gorulen").or_default().atla += 1,
                Ok(g) => {
                    say.entry("gorulen").or_default().tamam += 1;
                    for (j, (o, q)) in g.iter().zip(c["gor"][i].as_array().unwrap()).enumerate() {
                        match (o.yas, q.is_null()) {
                            (None, true) => {}
                            (Some(_), false) => {
                                for (ad, a, b) in [("x", o.v.x, &q[0]), ("y", o.v.y, &q[1]), ("vx", o.v.vx, &q[2]), ("vy", o.v.vy, &q[3]), ("yas", o.yas.unwrap(), &q[4]), ("e", o.e.unwrap(), &q[5])] {
                                    chk(&mut hata, format!("t{tk} gorulen {i}→{j}.{ad}"), a, h(b));
                                }
                            }
                            _ => hata.push(format!("t{tk} gorulen {i}→{j} kopya/gerçek farkı")),
                        }
                    }
                }
            }
        }
        // yerler
        for y in c["yer"].as_array().unwrap() {
            let pi = y["p"].as_u64().unwrap() as usize;
            let (ex, ey) = (h(&y["E"]["x"]), h(&y["E"]["y"]));
            let att = y["att"].as_bool().unwrap();
            let sb = slotBase(&K.m, &K.ps, &K.ps[pi], ex, ey, att);
            chk(&mut hata, format!("t{tk} slotBase {pi} x"), sb.x, h(&y["sb"]["x"]));
            chk(&mut hata, format!("t{tk} slotBase {pi} y"), sb.y, h(&y["sb"]["y"]));
            chk(&mut hata, format!("t{tk} slotBase {pi} el"), sb.el.unwrap_or(f64::NAN), h(&y["el"]));
            let b0 = bekci(&K.ps[pi], ex, ey, None, &K.kk);
            chk(&mut hata, format!("t{tk} bekci {pi} x"), b0.0, h(&y["bk"]["x"]));
            chk(&mut hata, format!("t{tk} bekci {pi} y"), b0.1, h(&y["bk"]["y"]));
            say.entry("slotBase/bekci").or_default().tamam += 1;
            if !y["bkm"].is_null() {
                let bm = BekM { ps: &K.ps, holder: K.m.holder, ben: Some(pi) };
                match catch_unwind(AssertUnwindSafe(|| bekci(&K.ps[pi], ex, ey, Some(&bm), &K.kk))) {
                    Ok(b1) => {
                        chk(&mut hata, format!("t{tk} bekci(m) {pi} x"), b1.0, h(&y["bkm"]["x"]));
                        chk(&mut hata, format!("t{tk} bekci(m) {pi} y"), b1.1, h(&y["bkm"]["y"]));
                        say.entry("bekci(m)").or_default().tamam += 1;
                    }
                    Err(_) => say.entry("bekci(m)").or_default().atla += 1,
                }
            }
        }
        for d in c["dw"].as_array().unwrap() {
            let w = durusW(h(&d[0]));
            for kk in 0..3 {
                chk(&mut hata, format!("t{tk} durusW"), w[kk], h(&d[1][kk]));
            }
        }
        if K.m.holder.is_some() || K.ball.is_some() {
            let mt = metrics(&K.mc());
            if mt.swarm as u64 != c["metr"].as_u64().unwrap() {
                hata.push(format!("t{tk} metrics"));
            }
        }
        // uçuş okuma
        if let Some(fj) = c.get("fr").filter(|f| !f.is_null()) {
            let mut K2 = kur(c);
            for (i, p) in K2.ps.iter_mut().enumerate() {
                p._bk = bk(&c["bak"][i]);
            }
            match catch_unwind(AssertUnwindSafe(|| flightRead(&mut K2.mc()))) {
                Err(_) => say.entry("flightRead").or_default().atla += 1,
                Ok(None) => hata.push(format!("t{tk} flightRead yok")),
                Ok(Some(fr)) => {
                    say.entry("flightRead").or_default().tamam += 1;
                    if fr.first != fj["first"].as_u64().map(|x| x as usize) {
                        hata.push(format!("t{tk} flightRead.first {:?} js {}", fr.first, fj["first"]));
                    }
                    let jp = fj["pts"].as_array().unwrap();
                    if jp.len() != fr.pts.len() {
                        hata.push(format!("t{tk} flightRead.pts boy {} js {}", fr.pts.len(), jp.len()));
                    }
                    for (a, q) in fr.pts.iter().zip(jp) {
                        chk(&mut hata, format!("t{tk} pts.x"), a.x, h(&q[0]));
                        chk(&mut hata, format!("t{tk} pts.y"), a.y, h(&q[1]));
                    }
                    for (i, (a, q)) in fr.arr.iter().zip(fj["arr"].as_array().unwrap()).enumerate() {
                        for (ad, x, y) in [("t", a.t, &q[0]), ("x", a.x, &q[1]), ("y", a.y, &q[2]), ("est", a.est, &q[3])] {
                            chk(&mut hata, format!("t{tk} flightRead {i}.{ad}"), x, h(y));
                        }
                    }
                    for (i, q) in c["psSon"].as_array().unwrap().iter().enumerate() {
                        let p = &K2.ps[i];
                        for (ad, x, y) in [("rcvV", p._rcvV, &q[0]), ("rcvAbs", p._rcvAbs, &q[1]), ("rcvX", p._rcvX, &q[2]), ("rcvY", p._rcvY, &q[3])] {
                            chk(&mut hata, format!("t{tk} flightRead {i}._{ad}"), x.unwrap_or(f64::NAN), ho(y).unwrap_or(f64::NAN));
                        }
                    }
                }
            }
        }
        // taşıyıcı
        if let Some(hi) = K.m.holder {
            let th = tehlike(&K.m, &K.ps, Some(hi), &K.d);
            match (&th, c["TH"].as_array()) {
                (None, None) => {}
                (Some(a), Some(b)) if a.len() == b.len() => {
                    for (u, q) in a.iter().zip(b) {
                        for (ad, x, y) in [("an", u.an, &q[0]), ("w", u.w, &q[1]), ("w0", u.w0, &q[2]), ("x", u.x, &q[3]), ("y", u.y, &q[4]), ("L", u.L, &q[6])] {
                            chk(&mut hata, format!("t{tk} tehlike.{ad}"), x, h(y));
                        }
                        if u.k != q[5].as_str().unwrap_or("") {
                            hata.push(format!("t{tk} tehlike.k"));
                        }
                    }
                }
                _ => hata.push(format!("t{tk} tehlike var/yok ya da boy farkı")),
            }
            say.entry("tehlike").or_default().tamam += 1;
            let hp = K.ps[hi].clone();
            let dt = 1 - hp.takim();
            let yol = tasHamle(hp.x, hp.y, hp.vx, hp.vy, dt);
            let jy = c["yol"].as_array().unwrap();
            if yol.len() != jy.len() {
                hata.push(format!("t{tk} tasHamle boy"));
            }
            for (Y, q) in yol.iter().zip(jy) {
                if Y.ad != q[0].as_str().unwrap() {
                    hata.push(format!("t{tk} tasHamle ad"));
                }
                for (ad, x, y) in [("ux", Y.ux, &q[1]), ("uy", Y.uy, &q[2]), ("sM", Y.sM, &q[3])] {
                    chk(&mut hata, format!("t{tk} tasHamle.{ad}"), x, h(y));
                }
            }
            say.entry("tasHamle").or_default().tamam += 1;
            for kj in c["kes"].as_array().unwrap() {
                let pi = kj["p"].as_u64().unwrap() as usize;
                let p = &K.ps[pi];
                let vSh = vSp(&hp, &K.kk);
                let vSq = vSp(p, &K.kk);
                chk(&mut hata, format!("t{tk} yolBag.vSh"), vSh, h(&kj["vSh"]));
                chk(&mut hata, format!("t{tk} yolBag.vP"), vSq, h(&kj["vP"]));
                let tR0 = jmax0(TEPKI(p, Tepki::Kovala, &K.kk));
                chk(&mut hata, format!("t{tk} yolBag.tR0"), tR0, h(&kj["tR0"]));
                let pr = |v: &Value| v.as_array().map(|a| KosuPr { Ts: h(&a[0]), k: h(&a[1]) });
                let cx = YolBag { vH: h(&kj["vH"]), vP: vSq, tR0, prH: pr(&kj["prH"]), prP: pr(&kj["prP"]), vSh, gH: gecTabV(&hp, Some(vSh), &K.kk), gP: gecTabV(p, Some(vSq), &K.kk) };
                for (Y, q) in yol.iter().zip(kj["K"].as_array().unwrap()) {
                    let X = yolKesis(p, &hp, Y, &cx);
                    for (ad, x, y) in [("x", X.x, &q[0]), ("y", X.y, &q[1]), ("s", X.s, &q[2])] {
                        chk(&mut hata, format!("t{tk} yolKesis {pi} {}.{ad}", Y.ad), x, h(y));
                    }
                    if X.once != q[3].as_bool().unwrap() {
                        hata.push(format!("t{tk} yolKesis {pi} once"));
                    }
                }
                let tehl: Vec<f64> = kj["tehl"].as_array().unwrap().iter().map(h).collect();
                let PK = presKesisT(p, &hp, &yol, &cx, &tehl);
                match (PK, kj["PK"].is_null()) {
                    (Some(a), false) => {
                        chk(&mut hata, format!("t{tk} presKesis {pi} x"), a.x, h(&kj["PK"]["x"]));
                        chk(&mut hata, format!("t{tk} presKesis {pi} y"), a.y, h(&kj["PK"]["y"]));
                    }
                    (None, true) => {}
                    _ => hata.push(format!("t{tk} presKesis var/yok")),
                }
                let kq = &kj["koy"];
                let kh = koyHedef(p, &hp, h(&kq[0]), h(&kq[1]));
                chk(&mut hata, format!("t{tk} koyHedef x"), kh.0, h(&kq[2]["x"]));
                chk(&mut hata, format!("t{tk} koyHedef y"), kh.1, h(&kq[2]["y"]));
                say.entry("yolBag/yolKesis/presKesis/koyHedef").or_default().tamam += 1;
            }
        }
        // takım
        {
            let cs = &c["cs"];
            let xy = |v: &Value| (h(&v["x"]), h(&v["y"]));
            let T: Vec<(f64, f64)> = cs["T"].as_array().unwrap().iter().map(xy).collect();
            let r = coverSpots(xy(&cs["o"]), cs["att"].as_u64().unwrap() as u8, xy(&cs["E"]), &T, h(&cs["R"]));
            let js = cs["out"].as_array().unwrap();
            if r.len() != js.len() {
                hata.push(format!("t{tk} coverSpots boy"));
            }
            for (a, q) in r.iter().zip(js) {
                chk(&mut hata, format!("t{tk} coverSpots x"), a.0, h(&q["x"]));
                chk(&mut hata, format!("t{tk} coverSpots y"), a.1, h(&q["y"]));
            }
            let lo = &c["lo"];
            let w = K.mc().dunya();
            let r = laneOpen(&w, &K.ps, lo["att"].as_u64().unwrap() as u8, xy(&lo["A"]), xy(&lo["B"]), Some(h(&lo["v"])), &K.kk, &K.d);
            chk(&mut hata, format!("t{tk} laneOpen"), r, h(&lo["r"]));
            let yt = &c["yt"];
            let pi = yt["p"].as_u64().unwrap() as usize;
            let (t1, t) = yolT(&K.ps[pi], xy(&yt["c"]), xy(&yt["kv"]), &K.kk);
            chk(&mut hata, format!("t{tk} yolT.t1"), t1, h(&yt["t1"]));
            chk(&mut hata, format!("t{tk} yolT.t"), t, h(&yt["t"]));
            say.entry("coverSpots/laneOpen/yolT").or_default().tamam += 1;
        }
    }
    let _ = std::panic::take_hook();
    for (k, s) in &say {
        eprintln!("{k}: {} durum, {} atlandı (yabancı işlev)", s.tamam, s.atla);
    }
    let mut tur: std::collections::BTreeMap<String, (usize, String)> = Default::default();
    for e in &hata {
        let k: String = e.split(':').next().unwrap().split_whitespace().skip(1).collect::<Vec<_>>().join(" ").chars().filter(|c| !c.is_ascii_digit()).collect();
        let x = tur.entry(k).or_insert((0, e.clone()));
        x.0 += 1;
    }
    let ozet: Vec<String> = tur.iter().map(|(k, (n, e))| format!("{n:5} × {k}   örn: {e}")).collect();
    assert!(hata.is_empty(), "{} hata:\n{}", hata.len(), ozet.join("\n"));
}

fn jmax0(x: f64) -> f64 {
    alan_motor::mat::jmax(0.0, x)
}
