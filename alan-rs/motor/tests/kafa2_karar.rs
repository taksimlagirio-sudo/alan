//! Kafa 2: karar ve dünyalar JS ile (sina/kafa2-karar-vektor.js): `cargo test --release -- --ignored kafa2`.
//!
//! Dış dünya: vektörün dünyasından kurulan bir maçın `MacDis`'i (maç ve yerleşimin gerçek işlevleri);
//! `Dis`'e birleştirmede eklenen yöntemler (bekciFm, slotBase, S_prHat, tl, sprK) burada yerleşimin ve
//! maçın gerçek işlevleriyle verilir. Vektörde maçın bütün durumu yok (sprint isteği, top uçuşu, kayıp
//! zamanı…): sapmalar bu eksiklerden olabilir.
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
    for e in v["a"].as_array().into_iter().flatten() {
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
    p.id = v["id"].as_f64().unwrap_or(f64::NAN); // kısmi nesne (JS {...}, id yok)
    p.team = v["team"].as_f64().unwrap_or(0.0);
    p.i = v["i"].as_f64().unwrap_or(0.0);
    p.role = v["role"].as_str().unwrap_or("").into();
    p.name = v["name"].as_str().unwrap_or("").into();
    p.yet = if a.v.is_empty() { Yetenek::bos().into() } else { Yetenek::kur(&a).into() };
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
    p.noTouch = v["noTouch"].as_bool().unwrap_or(false);
    p.press = v["press"].as_bool().unwrap_or(false);
    p.sprint = v["sprint"].as_bool().unwrap_or(false);
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
        tl.huc.insert("kosu".into(), g("kosu"));
        tl.huc.insert("istek".into(), g("istek"));
        tl.huc.insert("destek".into(), g("destek"));
        tl.huc.insert("risk".into(), g("risk"));
        tl.huc.insert("ritim".into(), g("ritim"));
        tl.huc.insert("tempo".into(), g("tempo"));
        tl.huc.insert("gonder".into(), g("gonder"));
        tl.huc.insert("serbest".into(), g("serbest"));
        if let Some(sv) = v["tlSav"].as_object() {
            tl.sav.insert("adam".into(), sv.get("adam").map(h).unwrap_or(0.0));
            tl.sav.insert("pres".into(), sv.get("pres").map(h).unwrap_or(0.0));
        }
        p.tl = Some(tl);
    }
    if !v["eg"].is_null() {
        p.eg = Some(JPEg { ps: h(&v["eg"]["ps"]), rk: h(&v["eg"]["rk"]) });
    }
    if let Some(sf) = v["sf"].as_object() {
        p._sf = Some(alan_motor::model::JP_sf { t0: h(&sf["t0"]), f: sf["f"].as_str().unwrap_or("").into() });
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
mod tdis {
    use alan_motor::kafa::gecici_dis1::*;
    use alan_motor::kafa::kafa1_js::{nk, Nk, Op};
    use alan_motor::mac::kafa_dis::MacDis;
    use alan_motor::model::{JMShGs, Oyuncu};
    use alan_motor::oyuncu::TepkiS;
    pub struct TDis {
        pub m: MacDis,
    }
    fn kisi(ps: &[Op]) -> Vec<Oyuncu> {
        ps.iter().map(|p| (**p).clone()).collect()
    }
    impl Dis for TDis {
    fn ALAN_SPRE(&self, p: &Oyuncu) -> f64 { self.m.ALAN_SPRE(p) }
    fn ALAN_SPR(&self, p: &Oyuncu) -> f64 { self.m.ALAN_SPR(p) }
    fn ALAN_SPR_HK(&self, p: &Oyuncu) -> f64 { self.m.ALAN_SPR_HK(p) }
    fn ALAN_SPR_FAZ(&self) -> Option<SprFaz> { self.m.ALAN_SPR_FAZ() }
    fn ALAN_KOSU_T(&self, p: &Oyuncu, d: f64, v1: f64) -> f64 { self.m.ALAN_KOSU_T(p, d, v1) }
    fn ALAN_KOSU_M(&self, p: &Oyuncu, t: f64, v1: f64) -> f64 { self.m.ALAN_KOSU_M(p, t, v1) }
    fn ALAN_EN3_KUL(&self, p: &Oyuncu, e: f64, bizde: bool, bx: f64, by: f64) -> f64 { self.m.ALAN_EN3_KUL(p, e, bizde, bx, by) }
    fn ALAN_EN3_SGIR(&self, p: &Oyuncu, bed: f64) -> f64 { self.m.ALAN_EN3_SGIR(p, bed) }
    fn ALAN_EN2_TS(&self, p: &Oyuncu, e: Option<f64>) -> f64 { self.m.ALAN_EN2_TS(p, e) }
    fn ALAN_EN2_SB(&self, p: &Oyuncu) -> f64 { self.m.ALAN_EN2_SB(p) }
    fn ALAN_EN2_SD(&self, p: &Oyuncu) -> f64 { self.m.ALAN_EN2_SD(p) }
    fn ALAN_EN3_DOL(&self, p: &Oyuncu, sahip: bool, nd: f64, yerK: f64) -> f64 { self.m.ALAN_EN3_DOL(p, sahip, nd, yerK) }
    fn ALAN_EN3_TEPE(&self, en: Option<f64>, k: f64) -> f64 { self.m.ALAN_EN3_TEPE(en, k) }
    fn ALAN_EN_S(&self, p: &Oyuncu) -> f64 { self.m.ALAN_EN_S(p) }
    fn ALAN_EN2(&self) -> En2 { self.m.ALAN_EN2() }
    fn ALAN_EN(&self) -> En { self.m.ALAN_EN() }
    fn ALAN_DENGE_KAYIP(&self, p: &Oyuncu, t: f64) -> f64 { self.m.ALAN_DENGE_KAYIP(p, t) }
    fn ALAN_DOKUNUS(&self) -> Option<Dokunus> { self.m.ALAN_DOKUNUS() }
    fn ALAN_YOL_VER_TEPKI(&self) -> bool { self.m.ALAN_YOL_VER_TEPKI() }
    fn ALAN_TASIYICI_SONRA_YAVAS(&self) -> bool { self.m.ALAN_TASIYICI_SONRA_YAVAS() }
    fn tepki_s(&self) -> TepkiS { self.m.tepki_s() }
    fn S(&self) -> Option<ShapeS> { self.m.S() }
    fn GR(&self) -> Option<ShapeGR> { self.m.GR() }
    fn GR_yasHata(&self, yas: f64) -> f64 { self.m.GR_yasHata(yas) }
    fn gecTab(&self, o: &Oyuncu, refsiz: bool) -> Option<GecTab> { self.m.gecTab(o, refsiz) }
    fn gecOf(&self, G: &GecTab, x: f64, y: f64, s: Option<f64>) -> f64 { self.m.gecOf(G, x, y, s) }
    fn gec(&self, o: &Oyuncu, x: f64, y: f64, s: Option<f64>) -> Option<f64> { self.m.gec(o, x, y, s) }
    fn gside(&self, a: Nk, t: f64, off: f64) -> Nk { self.m.gside(a, t, off) }
    fn bekci(&self, p: &Oyuncu, b: Nk) -> Option<Nk> { self.m.bekci(p, b) }
    fn aliciDirenc(&self, pts: &[Nk], p: &Oyuncu, rc: f64, iMin: f64, rP: f64) -> Option<Nk> { self.m.aliciDirenc(pts, p, rc, iMin, rP) }
    fn tehlike(&self, h: &Oyuncu) -> Vec<Tehlike> { self.m.tehlike(h) }
    fn AS_ilkPres_var(&self) -> bool { self.m.AS_ilkPres_var() }
    fn IK(&self) -> Option<Ik> { self.m.IK() }
    fn kutle(&self, p: &Oyuncu, ayni: bool) -> f64 { self.m.kutle(p, ayni) }
    fn direnc(&self, p: &Oyuncu, ayni: bool) -> f64 { self.m.direnc(p, ayni) }
    fn omuzK(&self, q: &Oyuncu) -> f64 { self.m.omuzK(q) }
    fn durtFark(&self, q: &Oyuncu, h: &Oyuncu) -> Option<f64> { self.m.durtFark(q, h) }
    fn sendS(&self, p: &Oyuncu) -> Option<f64> { self.m.sendS(p) }
    fn ikEn(&self, p: &Oyuncu) -> Option<f64> { self.m.ikEn(p) }
    fn BEKLE_BOZ(&self) -> Option<f64> { self.m.BEKLE_BOZ() }
    fn mac_var(&self) -> bool { self.m.mac_var() }
    fn mac_ch(&self) -> Option<f64> { self.m.mac_ch() }
    fn mac_holder(&self) -> Option<(f64, f64, f64, f64)> { self.m.mac_holder() }
    fn tac_enHarcama(&self, team: f64) -> Option<f64> { self.m.tac_enHarcama(team) }
    fn tac_siddet(&self, team: f64) -> Option<(Option<f64>, bool, f64)> { self.m.tac_siddet(team) }
    fn tacEff(&self, team: f64) -> TacEff { self.m.tacEff(team) }
    fn sh_gs(&self) -> Option<JMShGs> { self.m.sh_gs() }
    fn prT(&self) -> Option<PrT> { self.m.prT() }
    fn algi_e(&self, p: &Oyuncu) -> Option<f64> { self.m.algi_e(p) }
    fn algi_yas(&self, p: &Oyuncu) -> Option<f64> { self.m.algi_yas(p) }
    fn ilkPres(&self, tick: f64, qv: &mut Oyuncu, hx: &Oyuncu, team: f64, TH: &[Tehlike], dtac: &TacEff, gs: &Option<JMShGs>) { self.m.ilkPres(tick, qv, hx, team, TH, dtac, gs) }
    fn ivme(&self, p: &mut Oyuncu, wx: f64, wy: f64, vmax: f64) { self.m.ivme(p, wx, wy, vmax) }
        fn bekciFm(&self, p: &Oyuncu, B: Nk, fm: &Fm) -> Nk {
            let ps = kisi(fm.ps);
            let ben = fm.ps.iter().position(|q| q.id == p.id && q.team == p.team);
            let bm = alan_motor::sekil::yer::BekM { ps: alan_motor::sekil::yer::BekPs::Duz(&ps), holder: fm.holder, ben };
            let (x, y) = alan_motor::sekil::yer::bekci(p, B.x, B.y, Some(&bm), &self.m.kural.get());
            nk(x, y)
        }
        fn slotBase(&self, fm: &Fm, p: &Oyuncu, B: Nk, hucum: bool) -> Nk {
            let ps = kisi(fm.ps);
            let y = alan_motor::sekil::yer::slotBase(&self.m.mac, &ps, p, B.x, B.y, hucum);
            nk(y.x, y.y)
        }
        fn S_prHat(&self, blok: &str) -> Option<f64> {
            alan_motor::sekil::sabit::prHat(blok)
        }
        fn tl(&self, p: &Oyuncu, ev: &str, k: &str) -> Option<f64> {
            alan_motor::mac::kurulum::tl(p, ev, k)
        }
        fn sprK(&self, p: &Oyuncu) -> f64 {
            alan_motor::mac::enerji::sprK(p, &self.m.kural.get())
        }
    }
}

fn mac_tac(v: &Value) -> alan_motor::mac::Tac {
    let mut t = alan_motor::mac::Tac::default();
    t.blok = v["blok"].as_str().map(|s| s.into());
    t.genislik = v["genislik"].as_str().map(|s| s.into());
    t.pres = ho(&v["pres"]);
    t.onde = ho(&v["onde"]);
    t.risk = ho(&v["risk"]);
    t.tempo = ho(&v["tempo"]);
    t.sabir = ho(&v["sabir"]);
    t.sut = ho(&v["sut"]);
    t.pasSurme = ho(&v["pasSurme"]);
    t.pasBoy = ho(&v["pasBoy"]);
    t.enDinlen = ho(&v["enDinlen"]);
    t.enYipratma = ho(&v["enYipratma"]);
    t.siddet = ho(&v["siddet"]);
    t.savGen = ho(&v["savGen"]);
    t
}

fn kafa(g: &Value) -> Kafa {
    // vektörün dünyasından maç: oyuncular, tik, topu tutan, şarj, taktik
    let mut m = alan_motor::model::Mac::default();
    if let Some(src) = g["src"].as_array() {
        m.ps = src.iter().map(oyuncu).collect();
    }
    let src_var = g["src"].is_array();
    m.tick = g["mac"]["tick"].as_f64().or(g["tick"].as_f64()).unwrap_or(0.0);
    m.holder = g["mac"]["holder"].as_u64().map(|x| x as usize).filter(|&i| i < m.ps.len());
    m.ch = ho(&g["ch"]).unwrap_or(0.3);
    m.tac = match g["mac"]["tac"].as_array() {
        Some(a) if a.len() == 2 => a.iter().map(mac_tac).collect(),
        _ => vec![alan_motor::mac::tac::TAC0(), alan_motor::mac::tac::TAC0()],
    };
    let md = alan_motor::mac::kafa_dis::MacDis::kur(&mut m);
    let mut k = Kafa::yeni(Arc::new(tdis::TDis { m: md }));
    k.D._tick = ho(&g["D"]["_tick"]);
    if !g["mac"].is_null() {
        let m = &g["mac"];
        k.mac = Some(MacGor { tick: m["tick"].as_f64().unwrap_or(0.0), len: 9000.0, tac: m["tac"].as_array().map(|a| a.iter().map(tac_e).collect()).unwrap_or_default(), ps: if src_var { dunya(&g["src"]) } else { Vec::new() }, holder: m["holder"].as_u64().map(|x| x as usize) });
    }
    // yerleşimden kafaya dönen çağrılar (govdeV, threat …) için kapsamlı bağlam
    alan_motor::mac::kafa_dis::kafa_bagla(k.dis.clone(), k.D._tick);
    k
}
fn vektor() -> Option<Value> {
    let yol = concat!(env!("CARGO_MANIFEST_DIR"), "/../sina/kafa2-karar-vektor.json");
    std::fs::read_to_string(yol).ok().map(|s| serde_json::from_str(&s).unwrap())
}

#[test]
#[ignore = "uzun; JS vektörüyle: cargo test --release -- --ignored kafa2"]
fn kafa2_dunyalar_js_ile() {
    let v = match vektor() {
        Some(v) => v,
        None => return,
    };
    let mut hata = Vec::new();
    let mut say = 0;
    for g in v["bekKay"].as_array().into_iter().flatten() {
        let k = kafa(g);
        let p = Arc::new(oyuncu(&g["p"]));
        let r = kafa2_dunya::bekKay(&k, &p, nok(&g["B"]).unwrap(), h(&g["T"]));
        if !ayni_b(r.x, h(&g["R"][0])) || !ayni_b(r.y, h(&g["R"][1])) {
            let g0 = alan_motor::kafa::kafa2_bag::AS_bekci(&k, &p, nok(&g["B"]).unwrap(), None);
            let (jx, jy) = (h(&g["R"][0]), h(&g["R"][1]));
            let L = ((g0.x - p.x).powi(2) + (g0.y - p.y).powi(2)).sqrt();
            let mvj = ((jx - p.x).powi(2) + (jy - p.y).powi(2)).sqrt();
            let mvr = ((r.x - p.x).powi(2) + (r.y - p.y).powi(2)).sqrt();
            hata.push(format!("bekKay t{}: L {L} mv rs {mvr} js {mvj}; yön rs ({}, {}) js ({}, {}); SPRE {}", g["tick"], r.x - p.x, r.y - p.y, jx - p.x, jy - p.y, k.dis.ALAN_SPRE(&p)));
        }
        say += 1;
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
        say += 1;
    }
    eprintln!("bekKay + predictWorld: {say} çağrı");
    assert!(hata.is_empty(), "{} hata: {:#?}", hata.len(), &hata[..hata.len().min(20)]);
}

#[test]
#[ignore = "uzun; JS vektörüyle: cargo test --release -- --ignored kafa2"]
fn kafa2_imkan_sahiplik_js_ile() {
    let v = match vektor() {
        Some(v) => v,
        None => return,
    };
    let mut hata = Vec::new();
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
#[ignore = "uzun; JS vektörüyle: cargo test --release -- --ignored kafa2"]
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
