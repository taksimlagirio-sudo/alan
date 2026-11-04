//! Kafa1 (decide.js 1–4716): JS ile bit bit aynılık (sina/kafa1-vektor.js, gerçek maçtan anlık durumlar).
//! Dış dünya (maç, yerleşim, enerji) kayıttan ya da buraya taşınmış küçük işlevlerden (KayitDis).
#![allow(non_snake_case)]
use alan_motor::kafa::gecici_dis1::*;
use alan_motor::kafa::kafa1_js::{nk, Nk, Op};
use alan_motor::kafa::kafa1_ucus::UcusB0;
use alan_motor::kafa::kafa1_varis::Lane;
use alan_motor::kafa::Kafa;
use alan_motor::mat::{hypot, jmax, jmin, jround};
use alan_motor::model::{JMShGs, JPHamle, JPPplan, JP_gvKaz, JP_sf, Oyuncu};
use alan_motor::oyuncu::TepkiS;
use alan_motor::yetenek::{ag, ah, mek, Ham, Yetenek};
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
fn z0(v: f64) -> f64 {
    if v != 0.0 && v == v {
        v
    } else {
        0.0
    }
}

#[derive(Clone)]
struct Ext {
    spre: f64,
    spr: f64,
    sprhk: f64,
    dk: f64,
    pr: Option<(f64, f64)>,
}
#[derive(Clone)]
struct MacK {
    ch: Option<f64>,
    gs: Option<JMShGs>,
    tac: Vec<(Option<f64>, bool, f64)>,
    prT: Option<PrT>,
}
struct KayitDis {
    oy: HashMap<(i64, i64), Ext>,
    mac: Option<MacK>,
}
impl KayitDis {
    fn e(&self, p: &Oyuncu) -> &Ext {
        &self.oy[&(p.team as i64, p.id as i64)]
    }
    fn vSp(&self, p: &Oyuncu) -> f64 {
        (0.17 + 0.006 * p.hz()) * self.e(p).spre
    }
    fn ikEn0(p: &Oyuncu) -> f64 {
        match p.en {
            None => 1.0,
            Some(e) => 1.15 * (0.5 + 0.5 * jmax(0.0, jmin(1.0, e))),
        }
    }
}
// shape.js gec* (sınama için taşındı)
fn ivme_iv(vx: &mut f64, vy: &mut f64, iv: (f64, f64), wx: f64, wy: f64, vmax: f64) {
    let sp = hypot(*vx, *vy);
    let k0 = 0.18;
    let kH = k0 * iv.0;
    if sp < 0.001 {
        *vx += (wx - *vx) * kH;
        *vy += (wy - *vy) * kH;
        return;
    }
    let ux = *vx / sp;
    let uy = *vy / sp;
    let dvx = wx - *vx;
    let dvy = wy - *vy;
    let al = dvx * ux + dvy * uy;
    let px = dvx - al * ux;
    let py = dvy - al * uy;
    let r = jmin(1.0, sp / jmax(0.05, vmax));
    let ag = iv.1;
    let kA = if al >= 0.0 { kH } else { k0 * ag * (1.0 - 0.55 * r) };
    let kT = k0 * ag * (1.0 - 0.45 * r);
    let mut ba = al * kA;
    let mut tx = px * kT;
    let mut ty = py * kT;
    if ba < 0.0 {
        ba = jmax(ba, -0.018 * ag);
    }
    let tl = hypot(tx, ty);
    let tm = 0.021 * ag;
    if tl > tm && sp > 0.1 {
        tx *= tm / tl;
        ty *= tm / tl;
    }
    *vx += ux * ba + tx;
    *vy += uy * ba + ty;
}
fn gecYk(r: f64, c: f64, vm: f64, iv: (f64, f64)) -> GecY {
    let rr = jround(10.0 * r) / 10.0;
    let cc = jround(4.0 * c) / 4.0;
    let (mut vx, mut vy) = (rr * vm, 0.0);
    let ux = cc;
    let uy = jmax(0.0, 1.0 - cc * cc).sqrt();
    let mut m = vec![0.0];
    let (mut d, mut e) = (0.0, 0.0);
    for n in 0..80 {
        ivme_iv(&mut vx, &mut vy, iv, ux * vm, uy * vm, vm);
        d += vx * ux + vy * uy;
        e += vx * uy - vy * ux;
        let v = jmax(m[n], d - f64::abs(e));
        m.push(v);
    }
    GecY { g: jmax(0.0, 80.0 - (d - e.abs()) / vm), m: Some(m) }
}
fn gecK(Y: &GecY, s: f64, vm: f64) -> f64 {
    if !(s > 0.0) {
        return 0.0;
    }
    let m = match &Y.m {
        Some(m) => m,
        None => return Y.g,
    };
    if s >= m[80] {
        return Y.g;
    }
    let (mut lo, mut hi) = (0usize, 80usize);
    while hi - lo > 1 {
        let md = (lo + hi) >> 1;
        if m[md] >= s {
            hi = md;
        } else {
            lo = md;
        }
    }
    jmax(0.0, lo as f64 + (s - m[lo]) / jmax(1e-9, m[hi] - m[lo]) - s / vm)
}
impl KayitDis {
    fn ivk(&self, p: &Oyuncu) -> (f64, f64) {
        (jround(1e4 * ah(&p.yet)) / 1e4, jround(100.0 * ag(&p.yet, p.den, p.bekci(), 0.35)) / 100.0)
    }
    fn gecY(&self, o: &Oyuncu, c: f64) -> GecY {
        let s0 = hypot(z0(o.vx), z0(o.vy));
        let vm = self.vSp(o);
        let r = jmin(1.0, s0 / vm);
        gecYk(r, c, jround(2e3 * vm) / 2e3, self.ivk(o))
    }
}
impl Dis for KayitDis {
    fn ALAN_SPRE(&self, p: &Oyuncu) -> f64 {
        self.e(p).spre
    }
    fn ALAN_SPR(&self, p: &Oyuncu) -> f64 {
        self.e(p).spr
    }
    fn ALAN_SPR_HK(&self, p: &Oyuncu) -> f64 {
        self.e(p).sprhk
    }
    fn ALAN_SPR_FAZ(&self) -> Option<SprFaz> {
        Some(SprFaz { ust: 1.0, hkPay: 0.4, tepeT: 45.0, after: 1.0, dolT: 0.0, bedel: 0.15, patT: 1.0, frenT: 4.0, patK: 1.0, ayir: 1.0, itKos: 1.0, topla: 1.0, toplaBos: 8.0, itBos: None, peak: None })
    }
    fn ALAN_KOSU_T(&self, p: &Oyuncu, d: f64, v1: f64) -> f64 {
        match self.e(p).pr {
            Some((Ts, k)) if d > 0.0 => {
                let dS = v1 * Ts;
                if d <= dS {
                    d / v1
                } else {
                    Ts + (d - dS) / (v1 * k)
                }
            }
            _ => d / v1,
        }
    }
    fn ALAN_KOSU_M(&self, p: &Oyuncu, t: f64, v1: f64) -> f64 {
        match self.e(p).pr {
            Some((Ts, k)) if t > 0.0 => {
                if t <= Ts {
                    v1 * t
                } else {
                    v1 * Ts + v1 * k * (t - Ts)
                }
            }
            _ => jmax(0.0, t) * v1,
        }
    }
    fn ALAN_EN2(&self) -> En2 {
        En2 { tepe: 1.0 / 120.0, pat: 0.0, dinlen: 1.0 / 180.0, tempoDol: 0.25, yogK: 0.6, yogMin: 0.4 }
    }
    fn ALAN_EN(&self) -> En {
        En { dalis: 0.3, dalisDolYok: 45.0, sprEsik: 0.2, ivme: 0.1, surme: 0.002, dolas: 0.0025, fake: 0.03, kalkan: 0.0015, fizK: 1.5 }
    }
    fn ALAN_DENGE_KAYIP(&self, p: &Oyuncu, _t: f64) -> f64 {
        self.e(p).dk
    }
    fn ALAN_DOKUNUS(&self) -> Option<Dokunus> {
        None
    }
    fn ALAN_YOL_VER_TEPKI(&self) -> bool {
        false
    }
    fn ALAN_TASIYICI_SONRA_YAVAS(&self) -> bool {
        false
    }
    fn tepki_s(&self) -> TepkiS {
        TepkiS::default()
    }
    fn S(&self) -> Option<ShapeS> {
        Some(ShapeS { react: Some(0.5), rcvBuf: 3.0, marginT: 0.6, markOff: Some(2.5), bekOnce: Some(2.0), bekSerbest: true })
    }
    fn GR(&self) -> Option<ShapeGR> {
        Some(ShapeGR { yasH: 40.0 })
    }
    fn gecTab(&self, o: &Oyuncu, _refsiz: bool) -> Option<GecTab> {
        let vx = z0(o.vx);
        let vy = z0(o.vy);
        let s0 = hypot(vx, vy);
        let vm = self.vSp(o);
        let r = jmin(1.0, s0 / vm);
        let iv = self.ivk(o);
        let z = s0 < 0.001;
        let vmQ = jround(2e3 * vm) / 2e3;
        let mut t = Vec::new();
        let mut Y = Vec::new();
        for k in 0..=8 {
            let c = if z { 1.0 } else { (k as f64 - 4.0) / 4.0 };
            let y = gecYk(r, c, vmQ, iv);
            t.push(y.g);
            Y.push(Arc::new(y));
        }
        Some(GecTab { vx, vy, s0, x: o.x, y: o.y, t, Y, vm })
    }
    fn gecOf(&self, G: &GecTab, x: f64, y: f64, s: Option<f64>) -> f64 {
        let dx = x - G.x;
        let dy = y - G.y;
        let L = hypot(dx, dy);
        if L < 0.5 {
            return 0.0;
        }
        let c = if G.s0 < 0.001 { 1.0 } else { (G.vx * dx + G.vy * dy) / (G.s0 * L) };
        let k = (jround(4.0 * jmax(-1.0, jmin(1.0, c))) + 4.0) as usize;
        match s {
            Some(s) => gecK(&G.Y[k], s, G.vm),
            None => G.t[k],
        }
    }
    fn gec(&self, o: &Oyuncu, x: f64, y: f64, s: Option<f64>) -> Option<f64> {
        let vx = z0(o.vx);
        let vy = z0(o.vy);
        let s0 = hypot(vx, vy);
        let dx = x - o.x;
        let dy = y - o.y;
        let L = hypot(dx, dy);
        if L < 0.5 {
            return Some(0.0);
        }
        let c = if s0 < 0.001 { 1.0 } else { (vx * dx + vy * dy) / (s0 * L) };
        let Y = self.gecY(o, c);
        Some(match s {
            None => Y.g,
            Some(s) => gecK(&Y, s, self.vSp(o)),
        })
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
    fn IK(&self) -> Option<Ik> {
        Some(IK_JS)
    }
    fn ivme(&self, p: &mut Oyuncu, wx: f64, wy: f64, vmax: f64) {
        let sp = hypot(p.vx, p.vy);
        let k0 = 0.18;
        let kH = k0 * ah(&p.yet);
        if sp < 0.001 {
            p.vx += (wx - p.vx) * kH;
            p.vy += (wy - p.vy) * kH;
            return;
        }
        let ux = p.vx / sp;
        let uy = p.vy / sp;
        let dvx = wx - p.vx;
        let dvy = wy - p.vy;
        let al = dvx * ux + dvy * uy;
        let px = dvx - al * ux;
        let py = dvy - al * uy;
        let r = jmin(1.0, sp / jmax(0.05, vmax));
        let ag = ag(&p.yet, p.den, p.bekci(), 0.35);
        let kA = if al >= 0.0 { kH } else { k0 * ag * (1.0 - 0.55 * r) };
        let kT = k0 * ag * (1.0 - 0.45 * r);
        let mut ba = al * kA;
        let mut tx = px * kT;
        let mut ty = py * kT;
        let bkA = if p.bekci() { 2.0 } else { 1.0 };
        if ba < 0.0 {
            ba = jmax(ba, -0.018 * ag * bkA);
        }
        let tl = hypot(tx, ty);
        let tm = 0.021 * ag * bkA;
        if tl > tm && sp > 0.1 {
            tx *= tm / tl;
            ty *= tm / tl;
        }
        p.vx += ux * ba + tx;
        p.vy += uy * ba + ty;
    }
    fn kutle(&self, p: &Oyuncu, a: bool) -> f64 {
        (1.0 + 0.05 * (p.yet.y(mek::KUTLE) - 10.0)) * if !a && hypot(z0(p.vx), z0(p.vy)) < 0.06 { 1.6 } else { 1.0 }
    }
    fn direnc(&self, p: &Oyuncu, a: bool) -> f64 {
        (0.8 + 0.05 * (p.yet.y(if a { mek::TOPLU_DENGE } else { mek::DENGE_DIRENC }) - 10.0)) * if !a && hypot(z0(p.vx), z0(p.vy)) < 0.06 { 1.8 } else { 1.0 } * Self::ikEn0(p)
    }
    fn omuzK(&self, q: &Oyuncu) -> f64 {
        (1.0 + 0.03 * (q.yet.y(mek::OMUZ) - 10.0)) * Self::ikEn0(q)
    }
    fn durtFark(&self, q: &Oyuncu, h: &Oyuncu) -> Option<f64> {
        Some(q.yet.y(mek::DURTME) - h.yet.y(mek::DURTME_KORU))
    }
    fn sendS(&self, p: &Oyuncu) -> Option<f64> {
        Some(jmax(2.0, jround(12.0 - 0.5 * (p.yet.y(mek::SENDELEME) - 10.0))))
    }
    fn ikEn(&self, p: &Oyuncu) -> Option<f64> {
        Some(Self::ikEn0(p))
    }
    fn BEKLE_BOZ(&self) -> Option<f64> {
        Some(3.5)
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
    p.yet = if v["a"].as_object().unwrap().is_empty() { Yetenek::bos() } else { Yetenek::kur(&a) };
    p.x = h(&v["x"]);
    p.y = h(&v["y"]);
    p.vx = h(&v["vx"]);
    p.vy = h(&v["vy"]);
    p.tx = h(&v["tx"]);
    p.ty = h(&v["ty"]);
    p.en = ho(&v["en"]);
    p.den = ho(&v["den"]);
    p.noTouch = v["noTouch"].as_bool().unwrap();
    p.sonuk = v["sonuk"].as_bool().unwrap();
    p.kuyuT = ho(&v["kuyuT"]);
    p.R = ho(&v["R"]).unwrap_or(0.0);
    p.D = ho(&v["D"]).unwrap_or(1.0);
    p.amA = v["amA"].as_bool().unwrap();
    p.am = ho(&v["am"]);
    p.ca = ho(&v["ca"]);
    p._don = ho(&v["don"]);
    p._dke = ho(&v["dke"]);
    p.sprint = v["sprint"].as_bool().unwrap();
    p.job = v["job"].as_str().map(|s| s.to_string());
    p.press = v["press"].as_bool().unwrap();
    p._sf = if v["sf"].is_null() { None } else { Some(JP_sf { t0: h(&v["sf"]["t0"]), f: v["sf"]["f"].as_str().unwrap().into() }) };
    p._korT = ho(&v["korT"]);
    p.hamle = if v["hamle"].is_null() { None } else { Some(JPHamle { k: v["hamle"]["k"].as_str().unwrap().into(), hid: v["hamle"]["hid"].as_f64().unwrap(), t0: h(&v["hamle"]["t0"]), W: h(&v["hamle"]["W"]) }) };
    p.pplan = v["pplan"].as_str().map(|k| JPPplan { k: k.into(), ..Default::default() });
    p._gvKaz = if v["gvKaz"].is_null() { None } else { Some(JP_gvKaz { h: h(&v["gvKaz"][0]), t: h(&v["gvKaz"][1]), kaz: h(&v["gvKaz"][2]) }) };
    p.fiskeCd = ho(&v["fiskeCd"]);
    p.sendT = ho(&v["sendT"]);
    p.sendCd = ho(&v["sendCd"]);
    p.stun = ho(&v["stun"]);
    p.adandi = ho(&v["adandi"]);
    p._kal = v["kal"].as_bool().unwrap();
    p._kalH = v["kalH"].as_u64().map(|x| x as usize);
    p.tutQ = v["tutQ"].as_u64().map(|x| x as usize);
    p.markRef = v["markRef"].as_u64().map(|x| x as usize);
    p.sarj = Some(v["sarj"].as_bool().unwrap_or(true));
    p._yer = v["yer"].as_bool().unwrap();
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
        let mut oy = HashMap::new();
        for x in pj {
            oy.insert(
                (x["team"].as_i64().unwrap(), x["id"].as_i64().unwrap()),
                Ext { spre: h(&x["spre"]), spr: h(&x["spr"]), sprhk: h(&x["sprhk"]), dk: h(&x["dk"]), pr: if x["pr"].is_null() { None } else { Some((h(&x["pr"][0]), h(&x["pr"][1]))) } },
            );
        }
        let dis = Arc::new(KayitDis { oy: oy.clone(), mac: None });
        let mut k = Kafa::yeni(dis);
        k.D._tick = Some(tk);
        k.D._gon = Some(false);
        k.D._sut = Some(alan_motor::model::JsDeger::Sayi(0.5));
        k.D._ot = Some(1.0);
        k.D._savT = Some(0.0);
        k.D._sYon = Some(0.0);
        k.D._kenar = Some(false);
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
            k.D._gon = Some(gon);
            k.D._sut = Some(if gon { alan_motor::model::JsDeger::Metin("güçlü".into()) } else { alan_motor::model::JsDeger::Sayi(0.5) });
            let r = k.ucusFormul(hh, &ps, UcusB0 { x: h(&b[0]), y: h(&b[1]), vx: h(&b[2]), vy: h(&b[3]), ch: h(&b[4]), aim: None }, None, None, None);
            k.D._gon = Some(false);
            k.D._sut = Some(alan_motor::model::JsDeger::Sayi(0.5));
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
            let dis2 = Arc::new(KayitDis { oy: oy.clone(), mac: Some(MacK { ch: ho(&mj["ch"]), gs, tac, prT }) });
            let k2 = Kafa::yeni(dis2);
            let mut k2 = k2;
            k2.D._tick = Some(c["mtick"].as_f64().unwrap() + 0.5);
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
