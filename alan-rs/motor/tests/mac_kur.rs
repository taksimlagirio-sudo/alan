//! Kayıttan maç kurulumu: JS ile bit bit aynılık (sina/mac-vektor.js; yerleşimin slotBase/bekci çağrıları kayıttan).
mod mac_yardim;
use alan_motor::mac::kayit::{macKur_ile, Kayit};
use alan_motor::mac::kurulum::{tl, KurDis};
use alan_motor::model::{Mac, Pid};
use mac_yardim::*;
use serde_json::Value;

struct Kayitli {
    c: Vec<Value>,
    i: usize,
    hata: Vec<String>,
}
impl Kayitli {
    fn al(&mut self, k: &str, m: &Mac, p: Pid, ex: f64, ey: f64, att: bool) -> (f64, f64) {
        let c = &self.c[self.i];
        self.i += 1;
        let pp = &m.ps[p];
        if c[0].as_str() != Some(k) || c[1].as_f64() != Some(pp.team) || c[2].as_f64() != Some(pp.id) || !ayni(h(&c[3]), ex) || !ayni(h(&c[4]), ey) || c[5].as_bool() != Some(att) {
            self.hata.push(format!("çağrı {}: rust {k} {} {} {ex} {ey} {att} js {c}", self.i - 1, pp.team, pp.id));
        }
        (h(&c[6]), h(&c[7]))
    }
}
impl KurDis for Kayitli {
    fn slotBase(&mut self, m: &Mac, p: Pid, ex: f64, ey: f64, att: bool) -> (f64, f64) {
        self.al("s", m, p, ex, ey, att)
    }
    fn bekci(&mut self, m: &Mac, p: Pid, ex: f64, ey: f64) -> (f64, f64) {
        self.al("b", m, p, ex, ey, false)
    }
}

#[test]
fn kayittan_kurulum_js_ile_ayni() {
    let Some(v) = vektor() else { return };
    let mek: Vec<String> = v["mek"].as_array().unwrap().iter().map(|x| x.as_str().unwrap().to_string()).collect();
    let mut hata: Vec<String> = vec![];
    for k in v["kur"].as_array().unwrap() {
        let ad = k["dosya"].as_str().unwrap();
        let d: Kayit = serde_json::from_value(k["kayit"].clone()).unwrap();
        let mut dis = Kayitli { c: k["cagri"].as_array().unwrap().clone(), i: 0, hata: vec![] };
        let m = macKur_ile(&d, &mut dis);
        if dis.i != dis.c.len() {
            hata.push(format!("{ad}: yerleşim çağrısı rust {} js {}", dis.i, dis.c.len()));
        }
        hata.extend(dis.hata.iter().map(|x| format!("{ad}: {x}")));
        let es = |a: f64, b: &Value| ayni(a, h(b));
        if m.holder != k["holder"].as_u64().map(|x| x as usize) || !es(m.ch, &k["ch"]) || !es(m.decT, &k["decT"]) || !es(m.len, &k["len"]) || m.devre != k["devre"].as_bool().unwrap() {
            hata.push(format!("{ad}: maç alanları"));
        }
        for (t, tj) in k["tac"].as_array().unwrap().iter().enumerate() {
            let tq = &m.tac[t];
            if tq.blok.as_deref() != tj["blok"].as_str() || !ayni_o(tq.pres, &tj["pres"]) || !ayni_o(tq.risk, &tj["risk"]) || !ayni_o(tq.tempo, &tj["tempo"]) || !ayni_o(tq.sabir, &tj["sabir"]) || tq.kazaninca.as_deref() != tj["kazaninca"].as_str() || tq.yz.unwrap_or(false) != tj["yz"].as_bool().unwrap() {
                hata.push(format!("{ad}: tac {t}: {tq:?} js {tj}"));
            }
        }
        for (i, pj) in k["ps"].as_array().unwrap().iter().enumerate() {
            let p = &m.ps[i];
            let mut f = |n: &str, ok: bool| {
                if !ok {
                    hata.push(format!("{ad}: oyuncu {i} ({}) {n}", p.name));
                }
            };
            f("ad", p.name == pj["name"].as_str().unwrap());
            f("rol", p.role == pj["role"].as_str().unwrap());
            f("id", p.id == pj["id"].as_f64().unwrap() && p.team == pj["team"].as_f64().unwrap() && p.i == pj["i"].as_f64().unwrap());
            f("slot", es(p.slot.d, &pj["slot"][0]) && es(p.slot.l, &pj["slot"][1]));
            let ik = |a: Option<(f64, f64)>, b: &Value| match (a, b.is_null()) { (None, true) => true, (Some(a), false) => ayni(a.0, h(&b[0])) && ayni(a.1, h(&b[1])), _ => false };
            f("rslot", ik(p.rslot.as_ref().map(|r| (r.d, r.l)), &pj["rslot"]));
            f("slot0", ik(p.slot0.as_ref().map(|r| (r.d, r.l)), &pj["slot0"]));
            f("rslot0", ik(p.rslot0.as_ref().map(|r| (r.d, r.l)), &pj["rslot0"]));
            f("eg", ik(p.eg.as_ref().map(|r| (r.ps, r.rk)), &pj["eg"]));
            f("rh", p.rh.as_deref() == pj["rh"].as_str());
            f("rs", p.rs.as_deref() == pj["rs"].as_str());
            f("markRef", p.markRef == pj["markRef"].as_u64().map(|x| x as usize));
            match (&p.tl, pj["tl"].is_null()) {
                (None, true) => {}
                (Some(t), false) => {
                    for (ev, mm) in [("huc", &t.huc), ("sav", &t.sav)] {
                        let o = pj["tl"][ev].as_object().unwrap();
                        f(&format!("tl.{ev} sayısı"), o.len() == mm.len());
                        for (kk, x) in o {
                            f(&format!("tl.{ev}.{kk}"), tl(p, ev, kk).map_or(false, |y| ayni(y, h(x))));
                        }
                    }
                }
                _ => f("tl", false),
            }
            for n in ["R", "D", "x", "y", "tx", "ty", "en", "_korT"] {
                let r = match n {
                    "R" => Some(p.R),
                    "D" => Some(p.D),
                    "x" => Some(p.x),
                    "y" => Some(p.y),
                    "tx" => Some(p.tx),
                    "ty" => Some(p.ty),
                    "en" => p.en,
                    _ => p._korT,
                };
                f(n, ayni_o(r, &pj["n"][n]));
            }
            for (j, y) in pj["Y"].as_array().unwrap().iter().enumerate() {
                let mi = alan_motor::yetenek::MEK_AD.iter().position(|x| *x == mek[j]).unwrap();
                f(&format!("Y.{}", mek[j]), es(p.yet.y(mi), y));
            }
            for (j, kk) in ["gorus", "tepki", "guc", "kesme", "hakimiyet", "yeralma"].iter().enumerate() {
                f(&format!("oz.{kk}"), es(p.yet.ozk(kk), &pj["oz"][j]));
            }
        }
    }
    eprintln!("kurulum: {} kayıt", v["kur"].as_array().unwrap().len());
    assert!(hata.is_empty(), "{} hata:\n{}", hata.len(), hata[..hata.len().min(30)].join("\n"));
}
