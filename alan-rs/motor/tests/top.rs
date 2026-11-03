//! Top fiziği: JS ile bit bit aynılık (sina/top-vektor.js).
use alan_motor::alan::{k, Etki, Kaynak};
use alan_motor::saha::P;
use alan_motor::top::*;
use serde_json::Value;

fn h(v: &Value) -> f64 { match v.as_str() { Some(s) => f64::from_bits(u64::from_str_radix(s, 16).unwrap()), None => f64::NAN } }
fn ayni(a: f64, b: f64) -> bool { a.to_bits() == b.to_bits() || (a != a && b != b) }

fn etki(v: &Value) -> Etki {
    Etki { x: h(&v["x"]), y: h(&v["y"]), r: h(&v["r"]), d: h(&v["d"]), koni: v["koni"].as_bool().unwrap(), fx: h(&v["fx"]), fy: h(&v["fy"]), team: v["team"].as_u64().unwrap() as u8, am_a: v["am_a"].as_bool().unwrap(), am: h(&v["am"]), en: h(&v["en"]) }
}
fn top(v: &Value, team: u8) -> Top {
    let mut t = Top::yeni(0.0, 0.0, 0.0, 0.0, team, None, 0.2);
    t.x = h(&v["x"]); t.y = h(&v["y"]); t.vx = h(&v["vx"]); t.vy = h(&v["vy"]); t.e = h(&v["e"]); t.ch = h(&v["ch"]);
    t.alive = v["alive"].as_bool().unwrap(); t.dead = h(&v["dead"]); t.t = h(&v["t"]); t.dist = h(&v["dist"]); t.walls = h(&v["walls"]);
    t
}
fn karsila(ad: &str, t: &Top, v: &Value) -> Result<(), String> {
    let al = [("x", t.x), ("y", t.y), ("vx", t.vx), ("vy", t.vy), ("e", t.e), ("ch", t.ch), ("dead", t.dead), ("t", t.t), ("dist", t.dist), ("walls", t.walls), ("direk", t.direk)];
    for (n, x) in al { if !ayni(x, h(&v[n])) { return Err(format!("{ad}.{n}: rust {x:e} js {:e}", h(&v[n]))); } }
    if t.alive != v["alive"].as_bool().unwrap() { return Err(format!("{ad}.alive")); }
    let d = t.done.map(|d| d.ad().to_string()); let dj = v["done"].as_str().map(|s| s.to_string());
    if d != dj { return Err(format!("{ad}.done rust {d:?} js {dj:?}")); }
    Ok(())
}

#[test]
fn js_ile_bit_bit_ayni() {
    let s = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../sina/top-vektor.json")).expect("önce: node sina/top-vektor.js ../alan-v5 sina/top-vektor.json");
    let v: Value = serde_json::from_str(&s).unwrap();
    let mut hata = Vec::new();
    for (ci, c) in v.as_array().unwrap().iter().enumerate() {
        let src: Vec<Etki> = c["src"].as_array().unwrap().iter().map(etki).collect();
        for kv in c["k"].as_array().unwrap() { let (x, y, tm) = (h(&kv[0]), h(&kv[1]), kv[2].as_u64().unwrap() as u8); let r = k(&mut Kaynak::Sec(&src), x, y, tm); if !ayni(r, h(&kv[3])) { hata.push(format!("vaka {ci} K: rust {r:e} js {:e}", h(&kv[3]))); break; } }
        let team = c["top"]["team"].as_u64().unwrap() as u8;
        let t0 = top(&c["top"], team);
        let mut t = t0.kopya();
        for (i, a) in c["adim"].as_array().unwrap().iter().enumerate() {
            adim(&mut t, &mut Kaynak::Sec(&src), &P);
            if let Err(e) = karsila(&format!("vaka {ci} adım {i}"), &t, a) { hata.push(e); break; }
        }
        let n = c["pr_n"].as_u64().unwrap() as usize;
        let (pts, son) = tahmin(&t0, &mut Kaynak::Sec(&src), &P, Some(120));
        if pts.len() != n { hata.push(format!("vaka {ci} predict nokta {} js {n}", pts.len())); }
        if let Err(e) = karsila(&format!("vaka {ci} predict son"), &son, &c["pr_son"]) { hata.push(e); }
        for (i, pj) in c["pr_pts"].as_array().unwrap().iter().enumerate() { let p = pts[i * 7]; if !ayni(p.x, h(&pj[0])) || !ayni(p.y, h(&pj[1])) || (i > 0 && !ayni(p.e, h(&pj[2]))) { hata.push(format!("vaka {ci} predict nokta {}", i * 7)); break; } }
        let to = (h(&c["sl_to"][0]), h(&c["sl_to"][1]));
        let (v0, ok) = cikis_coz(&t0, to, h(&c["sl_arr"]), &mut Kaynak::Sec(&src), &P, 2.5);
        if !ayni(v0, h(&c["sl"][0])) || ok != c["sl"][1].as_bool().unwrap() { hata.push(format!("vaka {ci} solveLaunch rust {v0:e} {ok} js {:e}", h(&c["sl"][0]))); }
        let rt = roll_to(t0.x, t0.y, t0.vx, t0.vy, team, t0.ch, &mut Kaynak::Sec(&src), t0.x, t0.y, h(&c["rt_l"]), 200.0);
        if !ayni(rt, h(&c["rt"])) { hata.push(format!("vaka {ci} rollTo rust {rt:e} js {:e}", h(&c["rt"]))); }
    }
    assert!(hata.is_empty(), "{}", hata.join("\n"));
}
