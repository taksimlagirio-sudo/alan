//! Temas: JS ile bit bit aynılık (sina/temas-vektor.js, gerçek maçtan anlık durumlar).
use alan_motor::oyuncu::*;
use alan_motor::temas::*;
use alan_motor::top::Top;
use alan_motor::yetenek::{Ham, Yetenek};
use serde_json::Value;

fn h(v: &Value) -> f64 { match v.as_str() { Some("true") => 1.0, Some(s) => f64::from_bits(u64::from_str_radix(s, 16).unwrap()), None => f64::NAN } }
fn ho(v: &Value) -> Option<f64> { if v.is_null() { None } else { Some(h(v)) } }
fn ayni(a: f64, b: f64) -> bool { a.to_bits() == b.to_bits() || (a != a && b != b) }

struct KayitKural(Vec<(f64, f64, f64)>);
impl Kural for KayitKural { fn spr(&self, p: &Oyuncu) -> f64 { self.0.iter().find(|x| x.0 == p.team && x.1 == p.id).unwrap().2 } }

fn oyuncu(v: &Value) -> Oyuncu {
    let mut a = Ham::default();
    for (k, x) in v["a"].as_object().unwrap() { a.v.insert(k.clone(), h(x)); }
    let mut p = Oyuncu::default();
    p.id = v["id"].as_u64().unwrap() as f64; p.team = v["team"].as_u64().unwrap() as f64;
    p.role = if v["bek"].as_bool().unwrap() { "Bekçi".into() } else { "Orta".into() };
    p.yet = if v["a"].as_object().unwrap().is_empty() { Yetenek::bos() } else { Yetenek::kur(&a) };
    p.x = h(&v["x"]); p.y = h(&v["y"]); p.vx = h(&v["vx"]); p.vy = h(&v["vy"]);
    p.en = ho(&v["en"]); p.den = ho(&v["den"]);
    p.tx = h(&v["tx"]); p.ty = h(&v["ty"]); p.noTouch = v["noTouch"].as_bool().unwrap(); p.sonuk = v["sonuk"].as_bool().unwrap();
    p.kuyuT = ho(&v["kuyuT"]); p.R = ho(&v["R"]).unwrap_or(0.0); p.D = ho(&v["D"]).unwrap_or(1.0); p.amA = v["amA"].as_bool().unwrap(); p.am = ho(&v["am"]);
    p._rcvB = if v["rcvB"].as_bool().unwrap() { Some(1) } else { None }; p._dalis = ho(&v["dalis"]);
    p
}
fn top(v: &Value) -> TopT {
    let mut b = Top::yeni(0.0, 0.0, 0.0, 0.0, v["team"].as_u64().unwrap() as u8, None, 0.2);
    b.x = h(&v["x"]); b.y = h(&v["y"]); b.vx = h(&v["vx"]); b.vy = h(&v["vy"]); b.e = h(&v["e"]); b.ch = h(&v["ch"]); b.alive = v["alive"].as_bool().unwrap(); b.dead = h(&v["dead"]); b.t = h(&v["t"]); b.dist = h(&v["dist"]); b.walls = h(&v["walls"]);
    b.tp = ho(&v["tp"]); b.w0 = ho(&v["w0"]); b.d0 = if v["d0"].is_null() { None } else { Some((h(&v["d0"]), v["d0"].as_str() == Some("true"))) };
    b.defl = h(&v["defl"]); b.defl_b = v["defl"].as_str() == Some("true");
    let mut t = TopTemas::default(); t.kimlik = 1; t.kt = ho(&v["kt"]); t.kz = h(&v["kz"]); t.gon = v["gon"].as_bool().unwrap(); t.k_bek = v["kBek"].as_bool().unwrap();
    t.from = v["from"].as_u64().map(|x| x as usize); t.recv = v["recv"].as_u64().map(|x| x as usize); t.cue = h(&v["cue"]); t.bek_id = v["bekId"].as_u64().map(|x| x as u32); t.bek_m = h(&v["bekM"]); t.sap = v["sap"].as_bool().unwrap();
    for c in v["cds"].as_array().unwrap() { t.cds.insert(c[0].as_u64().unwrap() as usize, h(&c[1])); }
    TopT { b, t }
}

#[test]
fn js_ile_bit_bit_ayni() {
    let s = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../sina/temas-vektor.json")).unwrap();
    let v: Value = serde_json::from_str(&s).unwrap();
    let mut hata: Vec<String> = Vec::new();
    let mut say = [0usize; 3];
    for c in v.as_array().unwrap() {
        let tk = c["tick"].as_u64().unwrap();
        let mut ps: Vec<Oyuncu> = c["ps"].as_array().unwrap().iter().map(oyuncu).collect();
        let kk = KayitKural(c["ps"].as_array().unwrap().iter().map(|p| (p["team"].as_u64().unwrap() as f64, p["id"].as_u64().unwrap() as f64, h(&p["spr"]))).collect());
        for (i, pj) in c["per"].as_array().unwrap().iter().enumerate() {
            let mut b = top(&c["ball"]); let t = touch_cap(&mut b, i, &ps, &kk);
            for (n, x, j) in [("cap", t.cap, &pj["cap"]), ("vr", t.vr, &pj["vr"]), ("s", t.s, &pj["s"]), ("sk", t.sk, &pj["sk"])] { if !ayni(x, h(j)) { hata.push(format!("t{tk} oyuncu {i} touchCap.{n}: {x:e} js {:e}", h(j))); } }
            let mut b = top(&c["ball"]); let cp = ctrl_p(&mut b, i, &ps, Some(0.7), &kk); if !ayni(cp, h(&pj["cp"])) { hata.push(format!("t{tk} {i} ctrlP {cp} js {}", h(&pj["cp"]))); }
            let mz = menzil(&ps[i], 0.35); if !ayni(mz, h(&pj["mz"])) { hata.push(format!("t{tk} {i} menzil")); }
            if !pj["br"].is_null() { let mut b = top(&c["ball"]); let br = bek_r(&mut b, i, &ps); if !ayni(br, h(&pj["br"])) { hata.push(format!("t{tk} {i} bekR {br} js {}", h(&pj["br"]))); } }
            let b = top(&c["ball"]); if !ayni(thru_p(&b.b, &ps[i]), h(&pj["th"])) { hata.push(format!("t{tk} {i} thruP")); }
        }
        for g in c["kg"].as_array().unwrap() { let r = kuyu_girer(&ps[g[0].as_u64().unwrap() as usize], h(&g[1]), h(&g[2]), h(&g[3]), g[4].as_bool().unwrap(), &kk); if r != g[5].as_bool().unwrap() { hata.push(format!("t{tk} kuyuGirer")); } }
        let mut b = top(&c["ball"]); b.b.x = h(&c["bas"]["x"]); b.b.y = h(&c["bas"]["y"]);
        let rr: Vec<f64> = c["rnd"].as_array().unwrap().iter().map(h).collect(); let mut ri = 0;
        let mut rnd = || { let v = rr[ri]; ri += 1; v };
        let res = contact(&mut b, &mut ps, &mut rnd, h(&c["ox"]), h(&c["oy"]), &kk);
        match (&res, c["res"].is_null()) {
            (None, true) => say[0] += 1,
            (Some(r), false) => { say[if r.took { 1 } else { 2 }] += 1; let j = &c["res"]; if r.p != j["p"].as_u64().unwrap() as usize || r.took != j["took"].as_bool().unwrap() || !ayni(r.pp, h(&j["P"])) || r.thru != j["thru"].as_bool().unwrap() { hata.push(format!("t{tk} contact sonuç {r:?} js {j}")); } }
            _ => hata.push(format!("t{tk} contact var/yok farkı: rust {res:?} js {}", c["res"])),
        }
        let sj = &c["son"];
        for (n, x) in [("x", b.b.x), ("y", b.b.y), ("vx", b.b.vx), ("vy", b.b.vy), ("e", b.b.e), ("ch", b.b.ch), ("defl", b.b.defl)] { if !ayni(x, h(&sj[n])) { hata.push(format!("t{tk} contact top.{n}: {x:e} js {:e}", h(&sj[n]))); } }
        for (i, q) in c["pson"].as_array().unwrap().iter().enumerate() { if !ayni(ps[i].x, h(&q[0])) || !ayni(ps[i].y, h(&q[1])) || !ayni(ps[i]._dalis.unwrap_or(0.0), h(&q[2])) { hata.push(format!("t{tk} contact oyuncu {i} yeri")); } }
    }
    eprintln!("temassız {} tutulan {} seken {}", say[0], say[1], say[2]);
    assert!(hata.is_empty(), "{} hata:\n{}", hata.len(), hata[..hata.len().min(20)].join("\n"));
}
