//! Maçın çağırdığı kafa (decide.js) ve yerleşim (shape.js) işlevleri: GEÇİCİ imzalar (todo!()).
//! Birleştirmede gerçeklerine bağlanacak. Adlar JS adları; kaynak satırları sade dosyaya (kaynak/) göre.
//!
//! | işlev | JS |
//! |---|---|
//! | slotBase | shape.js:642 |
//! | bekci | shape.js:684 |
//! | position | shape.js:993 |
//! | bak | shape.js:336 |
//! | metrics (swarm) | shape.js:3607 |
//! | gorulen | shape.js:388 |
//! | gercek | shape.js:434 |
//! | pressAt | decide.js:3949 |
//! | oppAt | decide.js:140 |
//! | threat | decide.js:136 |
//! | ucusT | decide.js:2736 |
//! | coreSide | decide.js:4561 |
//! | caOf | decide.js:730 |
//! | vurE | decide.js:564 |
//! | savurPlan | decide.js:565 |
//! | onPlan | decide.js:628 |
//! | fakeP, fakeT, fakeHedef, fakeIlgili | decide.js:303, 307, 308, 326 |
//! | gonEk, pasEk | decide.js:202, 549 |
//! | applyErrorSav, applyError | decide.js:763, 777 |
//! | sarjMal, recvCh, sarjKarar, chargeRate | decide.js:3968, 3962, 3970, 3990 |
//! | otFactor, decideOT | decide.js:5252, 5255 |
//! | decide | decide.js:7293 |
//! | secimW, secimN, secim | decide.js:162, 183, 188 |
//! | rakipKovala | decide.js:659 |
//! | govdeVurP, duelP, enIkiliEk, girEsik, commitThr | decide.js:4550, 4622, 4663, 4668, 4708 |
//! | V | decide.js:3666 |
//! | D (sabitler), DU (sabitler) | decide.js:6–100 (değerler kopyalandı, todo değil) |
#![allow(unused_variables)]
pub use crate::kafa::kafa2_baglam::kafa_tac;
pub use crate::mac_veri::{FakeShow, Karar};
use crate::kafa::gecici_dis1::KuralErisim as _;
use crate::model::{Oyuncu, Pid, Secenek, JpOnH};
use crate::mac::MacTam as Mac;
use crate::rng::Rng;



/// Savurma planı (JS savurPlan sonucu).
#[derive(Clone, Copy, Debug, Default)]
pub struct SavurPlan {
    pub T: f64,
    pub w_rel: f64,
    pub u_sh: f64,
    pub k: f64,
    pub A: f64,
    pub s: f64,
    pub a: f64,
    pub wMax: f64,
    pub wR: f64,
    pub w0: f64,
    /// JS `pl.P || 0`
    pub P: f64,
}

/// Fiskede rakibin kovalaması (JS {q, f, d, v}).
#[derive(Clone, Copy, Debug, Default)]
pub struct RakipK {
    pub q: Pid,
    pub f: f64,
    pub d: f64,
    pub v: f64,
}

/// JS AlanDecide.D (maçın okuduğu sabitler)
#[allow(non_upper_case_globals)]
pub mod D {
    pub const arriveFirm: f64 = 0.8;
    pub const OT_OK: f64 = 6.0;
    pub const otMargin: f64 = 0.02;
    pub const SARJ_KAP: f64 = 0.25;
    pub const surKarar: f64 = 0.025;
}
/// JS AlanDecide.DU (maçın okuduğu sabitler; slowK tanımsız → 0,55)
#[allow(non_upper_case_globals)]
pub mod DU {
    pub const enFark: f64 = crate::core_sabit::EN_FARK;
    pub const engage: f64 = 2.4;
    pub const cd: f64 = 14.0;
    pub const oyalaR: f64 = 26.0;
    pub const thrLast: f64 = 0.5;
    pub const oyala: f64 = 0.25;
    pub const oyalaC: f64 = 0.1;
    pub const SLOW_K: f64 = 0.55;
}

// ---- shape.js ----

/// Yerleşimin bağlamını maçtan kur: oyuncular, top, zar ve yerleşim durumu geçici olarak maçtan alınır,
/// iş bitince geri konur (JS'te hepsi `m` üstünde).
pub fn mc_ile<T>(m: &mut Mac, f: impl FnOnce(&mut crate::sekil::varlik::Mc) -> T) -> T {
    use crate::sekil::varlik::Mc;
    crate::mac::kafa_dis::kafa_hazirla(m);
    crate::mac::kafa_dis::kural_tazele(m);
    let kural = crate::mac::mac_kurali(m);
    let mut ps = std::mem::take(&mut m.ps);
    // top maçta kalır (takımı okunur); yerleşim kopyasında çalışır, sonra geri yazılır
    let mut ball = m.ball.as_ref().map(|b| b.tt.clone());
    let mut ek = std::mem::take(&mut m.mac.sekil);
    let mut r = std::mem::take(&mut m.r);
    let lite = m.mac._lite;
    let d = kafa_d();
    let v = {
        let mut zar = || r.sonraki();
        // JS D._tick: gerçek maçın adımı yazar (match.js step). Kafanın hayali maçı (JS m._lite) Rust'ta kurulmaz.
        debug_assert!(!lite, "hayali maç (_lite) Rust'ta kurulmaz");
        let dt = Some(m.tick);
        let mut c = Mc { m: &mut m.m, ps: &mut ps, ball: ball.as_mut(), ek: &mut ek, kafa: m.mac.kafa.k.as_deref(), k: &kural, d, lite, rnd: Some(&mut zar), dtik: dt };
        f(&mut c)
    };
    m.ps = ps;
    if let (Some(b), Some(t)) = (m.ball.as_mut(), ball) {
        b.tt = t;
    }
    m.mac.sekil = ek;
    m.r = r;
    v
}

/// Kafanın sabitleri (JS AlanDecide.D)
pub fn kafa_d() -> &'static crate::model::KafaD {
    crate::kafa::kafa2_baglam::ayar()
}

/// Oyuncunun yerleşimdeki yeri (E: topun yeri, att: hücumda mı)
pub fn slot_base(m: &Mac, p: Pid, ex: f64, ey: f64, att: bool) -> (f64, f64) {
    let y = crate::sekil::yer::slot_base(m, &m.ps, &m.ps[p], ex, ey, att);
    (y.x, y.y)
}
/// Bekçi'nin yeri
pub fn bekci(m: &Mac, p: Pid, ex: f64, ey: f64) -> (f64, f64) {
    let kural = crate::mac::mac_kurali(m);
    let bm = crate::sekil::yer::BekM { ps: crate::sekil::yer::BekPs::Duz(&m.ps), holder: m.holder.map(|h| crate::dunya::Yerde::oyuncu(&m.ps[h])), sut: m.mac.kafa.k.is_some(), ben: Some(p) };
    crate::sekil::yer::bekci(&m.ps[p], ex, ey, Some(&bm), &kural)
}
/// Topsuz oyuncuların hedefleri ve işleri (tx, ty, job, sprint, press …)
pub fn position(m: &mut Mac) {
    mc_ile(m, |c| crate::sekil::position(c))
}
/// Bakış (p.ca vb.)
pub fn bak(m: &mut Mac) {
    mc_ile(m, |c| crate::sekil::bak(c))
}
/// JS metrics(m).swarm: topa yığılma ölçüsü
pub fn metrics_swarm(m: &mut Mac) -> f64 {
    mc_ile(m, |c| crate::sekil::takim::metrics(c).swarm as f64)
}
/// p'nin gördüğü dünya (m.ps ile aynı sıra); None = gerçek dünya (JS `=== m.ps`)
pub fn gorulen(m: &mut Mac, p: Pid) -> Option<Vec<Oyuncu>> {
    mc_ile(m, |c| {
        let g = crate::sekil::gorulen(c, p);
        Some(
            g.iter()
                .map(|q| {
                    let v = &q.v;
                    let mut o = v.govde(c.ps).clone();
                    o.x = v.x;
                    o.y = v.y;
                    o.vx = v.vx;
                    o.vy = v.vy;
                    o.hal_mut().__e = q.e;
                    o.hal_mut().__yas = q.yas;
                    o
                })
                .collect(),
        )
    })
}
/// Görülen dünyada kurulan seçeneği gerçek oyunculara bağla (yerinde): dünyalar sıra (Pid) ile
/// tutulduğu için iş yok.
pub fn gercek(_o: &mut Secenek) {}

// ---- decide.js ----
use crate::kafa::kafa1_js::nk;
use crate::mac::kafa_dis::{kafa_mac, op_dunya};
/// JS `D._mac = m`: kafanın maç görünümü
pub fn mac_gor(m: &Mac) -> crate::kafa::kafa2_baglam::MacGor {
    let tac = (0..2)
        .map(|t| {
            let q = m.tac.get(t).cloned().unwrap_or_default();
            let n = f64::NAN;
            crate::model::JMTacE {
                blok: q.blok.unwrap_or_default(),
                pres: q.pres.unwrap_or(n),
                onde: q.onde.unwrap_or(n),
                genislik: q.genislik.unwrap_or_default(),
                risk: q.risk.unwrap_or(n),
                tempo: q.tempo.unwrap_or(n),
                kazaninca: q.kazaninca.unwrap_or_default(),
                siddet: q.siddet.unwrap_or(n),
                savGen: q.savGen.unwrap_or(n),
                pasSurme: q.pasSurme.unwrap_or(n),
                pasBoy: q.pasBoy.unwrap_or(n),
                sabir: q.sabir.unwrap_or(n),
                sut: q.sut.unwrap_or(n),
                enDinlen: q.enDinlen.unwrap_or(n),
                enYipratma: q.enYipratma.unwrap_or(n),
                yz: q.yz == Some(true),
                ..Default::default()
            }
        })
        .collect();
    crate::kafa::kafa2_baglam::MacGor { tick: m.tick, len: m.len, tac, ps: op_dunya(&m.ps), holder: m.holder }
}
fn sira(g: &[Oyuncu], p: &Oyuncu) -> Pid {
    g.iter().position(|q| q.id == p.id && q.team == p.team).expect("karar veren dünyada yok")
}
/// JS'te kafaya giden oyuncu dünyadaki nesnenin kendisidir (`o !== q` kimlikle karşılaştırır): kopya dünyada
/// aynı oyuncuyu (kimliğiyle) bul; yoksa verileni kullan.
fn ozne<'a>(g: &[Oyuncu], src: &'a [crate::kafa::kafa1_js::Op], p: &'a Oyuncu) -> &'a Oyuncu {
    match g.iter().position(|q| q.id == p.id && q.team == p.team) {
        Some(i) => &src[i],
        None => p,
    }
}
fn rng_fn(r: &mut Rng) -> impl FnMut() -> f64 + '_ {
    move || r.sonraki()
}

/// Saf (ayardan): kafaya uğramaz, dünya kopyalanmaz.
pub fn press_at(ps: &[Oyuncu], team: f64, x: f64, y: f64) -> f64 {
    crate::kafa::kafa1_tehlike::press_at_d(kafa_d(), ps, team, x, y)
}
/// Saf: kafaya uğramaz, dünya kopyalanmaz.
pub fn opp_at(ps: &[Oyuncu], team: f64, x: f64, y: f64) -> f64 {
    crate::kafa::kafa1_tehlike::opp_at_p(ps, team, x, y)
}
pub fn threat(team: f64, x: f64, y: f64) -> f64 {
    // saf: kafa bağlamı aranmaz
    crate::kafa::kafa1_tehlike::threat_d(kafa_d(), team, x, y)
}
/// Pasın d'ye uçuş süresi (aF: varış sertliği)
pub fn ucus_t(m: &Mac, d: f64, a_f: f64) -> f64 {
    kafa_mac(m, |k| k.ucus_t(d, a_f, None))
}
/// Çekirdeğin gövdedeki yönü (birim vektör)
pub fn core_side(m: &Mac, h: Pid) -> (f64, f64) {
    crate::mac::kafa_dis::kural_tazele(m);
    kafa_mac(m, |k| {
        let c = k.core_side(&m.ps[h]);
        (c.ux, c.uy)
    })
}
pub fn ca_of(m: &Mac, h: Pid) -> f64 {
    crate::mac::kafa_dis::kural_tazele(m);
    kafa_mac(m, |k| k.ca_of(&m.ps[h]))
}
pub fn vur_e(m: &Mac, h: Pid, ch: f64, gon: bool) -> f64 {
    crate::mac::kafa_dis::kural_tazele(m);
    kafa_mac(m, |k| k.vur_e(&m.ps[h], Some(ch), gon))
}
pub fn savur_plan(m: &Mac, h: Pid, launch: (f64, f64), ca0: f64, w0: f64, sYon: f64, plase: bool, v_e: f64) -> Option<SavurPlan> {
    crate::mac::kafa_dis::kural_tazele(m);
    let src = op_dunya(&m.ps);
    kafa_mac(m, |k| {
        k.savur_plan(&src[h], nk(launch.0, launch.1), ca0, w0, sYon, Some(&src), plase, v_e)
            .map(|p| SavurPlan { T: p.T, w_rel: p.w_rel, u_sh: p.u_sh, k: p.k, A: p.A, s: p.s, a: p.a, wMax: p.wMax, wR: p.wR, w0: p.w0, P: if crate::mat::dogru(p.P) { p.P } else { 0.0 } })
    })
}
/// Ön hareket planı (JS onPlan): {s, wS, phi, caR, kal, a, Tsw} (b, tArr, w maç doldurur)
pub fn on_plan(m: &Mac, q: Pid, launch: (f64, f64), plase: bool, v_e: f64) -> Option<JpOnH> {
    crate::mac::kafa_dis::kural_tazele(m);
    let b = m.ball.as_ref().map_or(nk(f64::NAN, f64::NAN), |b| nk(b.x, b.y));
    let top = m.ball.as_ref().map(|b| b.tt.t.kimlik).unwrap_or_default();
    kafa_mac(m, |k| {
        k.on_plan(&m.ps[q], nk(launch.0, launch.1), b, plase, v_e)
            .map(|p| JpOnH { b: top, s: p.s, wS: p.wS, phi: p.phi, caR: p.caR, kal: p.kal, a: p.a, Tsw: p.Tsw, ..Default::default() })
    })
}
pub fn fake_t(m: &Mac, h: Pid) -> f64 {
    crate::mac::kafa_dis::kural_tazele(m);
    kafa_mac(m, |k| k.fake_t(&m.ps[h]))
}
pub fn fake_ilgili(m: &Mac, p: Pid, h: Pid, sh: &FakeShow) -> bool {
    crate::mac::kafa_dis::kural_tazele(m);
    kafa_mac(m, |k| k.fake_ilgili(&m.ps[p], &m.ps[h], &crate::kafa::kafa1_fake::FakeSh { k: sh.k.clone(), to: nk(sh.to.0, sh.to.1), c: sh.c }))
}
pub fn fake_p(m: &Mac, h: Pid, p: Pid, k_: &str, c: f64) -> f64 {
    crate::mac::kafa_dis::kural_tazele(m);
    kafa_mac(m, |k| k.fake_p(&m.ps[h], &m.ps[p], k_, Some(c)))
}
pub fn fake_hedef(m: &Mac, p: Pid, h: Pid, sh: &FakeShow) -> (f64, f64) {
    crate::mac::kafa_dis::kural_tazele(m);
    kafa_mac(m, |k| {
        let t = k.fake_hedef(&m.ps[p], &m.ps[h], &crate::kafa::kafa1_fake::FakeSh { k: sh.k.clone(), to: nk(sh.to.0, sh.to.1), c: sh.c });
        (t.x, t.y)
    })
}
pub fn gon_ek(m: &Mac, h: Pid, shot: Option<&str>) -> f64 {
    crate::mac::kafa_dis::kural_tazele(m);
    let src = op_dunya(&m.ps);
    kafa_mac(m, |k| k.gon_ek(&src[h], &src, shot.unwrap_or("")))
}
pub fn pas_ek(m: &Mac, h: Pid) -> f64 {
    crate::mac::kafa_dis::kural_tazele(m);
    let src = op_dunya(&m.ps);
    kafa_mac(m, |k| k.pas_ek(&src[h], &src))
}
pub fn apply_error_sav(m: &Mac, h: Pid, launch: (f64, f64), r: &mut Rng, ek: f64, gon: bool, savW: f64, savU: f64, shot: Option<&str>, kenar: bool) -> (f64, f64) {
    crate::mac::kafa_dis::kural_tazele(m);
    let src = op_dunya(&m.ps);
    kafa_mac(m, |k| {
        let v = k.apply_error_sav(&src[h], nk(launch.0, launch.1), &mut rng_fn(r), ek, gon, savW, savU, Some(&src), shot, Some(kenar));
        (v.x, v.y)
    })
}
pub fn apply_error(m: &Mac, h: Pid, launch: (f64, f64), r: &mut Rng, ek: f64, gon: bool, shot: Option<&str>, kenar: bool) -> (f64, f64) {
    crate::mac::kafa_dis::kural_tazele(m);
    let src = op_dunya(&m.ps);
    kafa_mac(m, |k| {
        let v = k.apply_error(&src[h], nk(launch.0, launch.1), &mut rng_fn(r), ek, gon, Some(&src), shot, Some(kenar));
        (v.x, v.y)
    })
}
pub fn sarj_karar(m: &Mac, p: Pid, g: &[Oyuncu], ch: f64) -> bool {
    crate::mac::kafa_dis::kural_tazele(m);
    let src = op_dunya(g);
    kafa_mac(m, |k| k.sarj_karar(ozne(g, &src, &m.ps[p]), &src, ch))
}
pub fn recv_ch(m: &Mac, ps: &[Oyuncu], team: f64, x: f64, y: f64, ch: f64, p: &Oyuncu) -> f64 {
    crate::mac::kafa_dis::kural_tazele(m);
    kafa_mac(m, |k| k.recv_ch(&op_dunya(ps), team, x, y, ch, p))
}
pub fn ot_factor(m: &Mac, p: Pid, in_sp: f64) -> f64 {
    crate::mac::kafa_dis::kural_tazele(m);
    kafa_mac(m, |k| crate::kafa::kafa2_karar::ot_factor(k, &m.ps[p], in_sp))
}
/// Tek dokunuş kararı (kanca: JS D._kanca, kararın süresince)
pub fn decide_ot(m: &Mac, p: &Oyuncu, g: &[Oyuncu], ch: f64, r: &mut Rng, tE: &crate::mac::Tac, ot: f64, kanca: Option<crate::mac::hareket::Kanca>) -> Option<Secenek> {
    crate::mac::kafa_dis::kural_tazele(m);
    let src = std::sync::Arc::new(op_dunya(g));
    let hi = sira(g, p);
    let tac = kafa_tac(tE);
    let mg = mac_gor(m);
    kafa_mac(m, |k| {
        k.mac = Some(mg);
        let k0 = k.bag._kanca.take();
        k.bag._kanca = kanca.map(|c| crate::model::DKanca { w0: c.w0, s: c.s, ca: Some(c.ca), u: Some(c.u), on: c.on });
        let o = crate::kafa::kafa2_karar::decide_ot(k, hi, &src, ch, &mut rng_fn(r), Some(&tac), ot);
        k.bag._kanca = k0;
        o
    })
}
/// Karar: p (g içinde kendi sırasında) için seçenekler
pub fn decide(m: &Mac, p: &Oyuncu, g: &[Oyuncu], ch: f64, r: &mut Rng, tE: &crate::mac::Tac) -> Karar {
    crate::mac::kafa_dis::kural_tazele(m);
    let src = std::sync::Arc::new(op_dunya(g));
    let hi = sira(g, p);
    let tac = kafa_tac(tE);
    let mg = mac_gor(m);
    kafa_mac(m, |k| {
        k.mac = Some(mg);
        if crate::iz_acik() && std::env::var("ALAN_IMK_IZ").ok().map_or(false, |v| v == format!("{}", m.tick)) {
            eprintln!("vh parça vSp {} topK {} SPRE {} SPR {} spd {}", k.v_s(p), k.top_k(p), k.dis.alan_spre(p), k.dis.alan_spr(p), k.spd(p));
            for x in [20.0, 40.0, 60.0, 80.0, 100.0] {
                for y in [10.0, 30.0, 50.0] {
                    let i = crate::kafa::kafa2_imkan::imkan(k, &src, p.team, crate::model::Nokta { x, y }, Some(p), ch);
                    let yd = crate::kafa::kafa2_zemin::yer_deger(k, &src, p.team, x, y, ch);
                    eprintln!("imk {} {} {:.9} yd {:.9}", x, y, i, yd);
                }
            }
        }
        let r = crate::kafa::kafa2_karar::decide(k, hi, &src, ch, &mut rng_fn(r), Some(&tac));
        if crate::iz_acik() && std::env::var("ALAN_KARAR_IZ").ok().map_or(false, |v| v == "hepsi" || v == format!("{}", m.tick)) {
            eprintln!("tik {}\ndecide {} opts {}", m.tick, p.govde.name, r.opts.len());
            for q in g {
                eprintln!("  dunya {} {} {} {} {} {:?} {:?}", q.govde.name, q.x, q.y, q.vx, q.vy, q.hal.__e, q.hal.__yas);
            }
            let mut o: Vec<&crate::model::Secenek> = r.opts.iter().collect();
            o.sort_by(|a, b| b.v.unwrap_or(f64::NAN).partial_cmp(&a.v.unwrap_or(f64::NAN)).unwrap_or(std::cmp::Ordering::Equal));
            for o in o {
                eprintln!("  {:<34} {:<8} v={:.6} to={}", o.kind, o.q.map_or(String::new(), |q| g[q].govde.name.to_string()), o.v.unwrap_or(f64::NAN), o.to.as_ref().map_or("-".into(), |t| format!("{:.3},{:.3}", t.x, t.y)));
            }
        }
        if crate::iz_acik() && std::env::var("ALAN_SUR_IZ").ok().map_or(false, |v| v == format!("{}", m.tick)) {
            let mut o: Vec<&crate::model::Secenek> = r.opts.iter().filter(|o| o.kind == "sür").collect();
            o.sort_by(|a, b| b.v.unwrap_or(f64::NAN).partial_cmp(&a.v.unwrap_or(f64::NAN)).unwrap_or(std::cmp::Ordering::Equal));
            for o in o.iter().filter(|o| o.to.map_or(false, |t| (t.x - 68.126).abs() < 0.01 || std::env::var("ALAN_SUR_HEPSI").is_ok())).take(3) {
                eprintln!("SUR to {:?} spr {:?} sprBos {:?} v {:?} _own {:?} TsE {:?} toE {:?} _pT {:?}\n   dok {:?}\n   det {:?}\n   en {:?}", o.to, o.spr, o.sprBos, o.v, o._own, o.TsE, o.toE, o._pT, o.dok, o.det, o.en);
            }
        }
        Karar { best: if r.best.is_some() { Some(0) } else { None }, opts: r.opts, T: r.T }
    })
}
pub fn secim_n(m: &Mac, p: Pid, o: &Secenek, g: &[Oyuncu], T: f64) -> f64 {
    crate::mac::kafa_dis::kural_tazele(m);
    let src = op_dunya(g);
    kafa_mac(m, |k| k.secim_n(ozne(g, &src, &m.ps[p]), Some(o), &src, T))
}
/// Seçim: r.opts içindeki sıra
pub fn secim(m: &Mac, h: Pid, r: &Karar, g: &[Oyuncu]) -> usize {
    crate::mac::kafa_dis::kural_tazele(m);
    let src = op_dunya(g);
    kafa_mac(m, |k| k.secim(ozne(g, &src, &m.ps[h]), &r.opts, r.best, r.T, &src)).or(r.best).unwrap_or(0)
}
pub fn secim_w(m: &Mac, h: Pid, pr: f64, b: bool) -> f64 {
    crate::mac::kafa_dis::kural_tazele(m);
    kafa_mac(m, |k| k.secim_w(&m.ps[h], pr, b, None))
}
pub fn charge_rate(m: &Mac, h: Pid, still: f64, ch: f64) -> f64 {
    crate::mac::kafa_dis::kural_tazele(m);
    let src = op_dunya(&m.ps);
    kafa_mac(m, |k| k.charge_rate(&src[h], &src, still, ch, false))
}
pub fn sarj_mal(m: &Mac, h: Pid) -> f64 {
    crate::mac::kafa_dis::kural_tazele(m);
    kafa_mac(m, |k| k.sarj_mal(&m.ps[h]))
}
/// Fiskede rakibin bx, by'ye (taşıyıcıya göre) kovalaması
pub fn rakip_kovala(m: &Mac, o: &RakipK, bx: f64, by: f64, md: f64) -> (f64, f64) {
    crate::mac::kafa_dis::kural_tazele(m);
    todo!("decide.js rakipKovala")
}
pub fn govde_vur_p(m: &Mac, q: Pid, h: Pid) -> f64 {
    crate::mac::kafa_dis::kural_tazele(m);
    kafa_mac(m, |k| k.govde_vur_p(&m.ps[q], &m.ps[h], None, false))
}
pub fn duel_p(m: &Mac, q: Pid, h: Pid) -> f64 {
    crate::mac::kafa_dis::kural_tazele(m);
    let src = op_dunya(&m.ps);
    kafa_mac(m, |k| k.duel_p(&src[q], &src[h], &src, None))
}
pub fn en_ikili_ek(m: &Mac, q: Pid, h: Pid) -> f64 {
    crate::mac::kafa_dis::kural_tazele(m);
    kafa_mac(m, |k| k.en_ikili_ek(&m.ps[q], &m.ps[h]))
}
pub fn gir_esik(m: &Mac, q: Pid, h: Pid, ch: f64) -> Option<f64> {
    crate::mac::kafa_dis::kural_tazele(m);
    let src = op_dunya(&m.ps);
    kafa_mac(m, |k| k.gir_esik(&src[q], &src[h], &src, Some(ch)))
}
pub fn commit_thr(m: &Mac, q: Pid, h: Pid) -> f64 {
    crate::mac::kafa_dis::kural_tazele(m);
    let src = op_dunya(&m.ps);
    kafa_mac(m, |k| k.commit_thr(&src[q], &src[h], &src))
}
/// Alan değeri (not için)
pub fn V(m: &Mac, ps: &[Oyuncu], team: f64, x: f64, y: f64, ch: f64) -> f64 {
    kafa_mac(m, |k| k.V(&op_dunya(ps), team, x, y, ch))
}

// ---- şut tablosu üreteci (alan-kos imktablo) ----

/// Bütün ham özellikleri `L` olan oyuncu (şablonun kimliği, takımı, rolü korunur): imkOrt = L.
pub fn duzey_oyuncu(sablon: &Oyuncu, L: f64) -> Oyuncu {
    let mut p = sablon.clone();
    {
        let g = p.govde_mut();
        let a = &mut g.a;
        for f in [&mut a.hiz, &mut a.ceviklik, &mut a.guc, &mut a.uzanma, &mut a.alan, &mut a.kontrol, &mut a.surme, &mut a.pas, &mut a.gonderme, &mut a.yukleme, &mut a.tepki, &mut a.gorus, &mut a.yeralma, &mut a.sogukkanlilik, &mut a.kesme, &mut a.refleks, &mut a.yerlesim, &mut a.hakimiyet, &mut a.enerji, &mut a.govde, &mut a.caliskanlik] {
            *f = L;
        }
        let mut h = crate::yetenek::Ham::default();
        for k in ["hiz", "ceviklik", "guc", "uzanma", "alan", "kontrol", "surme", "pas", "gonderme", "yukleme", "tepki", "gorus", "yeralma", "sogukkanlilik", "kesme", "refleks", "yerlesim", "hakimiyet", "enerji", "govde", "caliskanlik", "okuma"] {
            h.v.insert(k.to_string(), L);
        }
        g.yet = std::sync::Arc::new(crate::yetenek::Yetenek::kur(&h));
    }
    p
}

/// Şut tablosunun bir hücresi (JS shotCell / imk tablosunun tanımı): boş sahada şutçu (takım 0, kale GW'de) kaleye
/// `dd` uzakta, `sy` yanda (y > CY); Bekçi şutçuya göre yerleşim kuralının yerinden (sekil::yer::bekci) şut doğrultusuna
/// dik `ko`, doğrultu boyunca `kd` (artı: geride) kaymış; kafanın kendi kararının gönder seçeneklerinde en iyi gol
/// olasılığı (det.Pk). Geçersiz yer: None.
pub fn imk_hucre(m: &Mac, sut: &Oyuncu, bek: &Oyuncu, dd: f64, sy: f64, ko: f64, kd: f64, ch: f64) -> Option<f64> {
    use crate::saha::{CY, W};
    if sy >= dd - 0.5 {
        return None;
    }
    let x = W - (dd * dd - sy * sy).sqrt();
    let y = CY + sy;
    if x > W - 0.8 || y > crate::saha::H - 0.8 {
        return None;
    }
    let mut s = sut.clone();
    s.team = 0.0;
    s.x = x;
    s.y = y;
    s.vx = 0.0;
    s.vy = 0.0;
    let mut b = bek.clone();
    b.team = 1.0;
    b.vx = 0.0;
    b.vy = 0.0;
    b.x = W - 1.0;
    b.y = CY;
    let kural = crate::mac::mac_kurali(m);
    let g0 = vec![s.clone(), b.clone()];
    let bm = crate::sekil::yer::BekM { ps: crate::sekil::yer::BekPs::Duz(&g0), holder: Some(crate::dunya::Yerde::oyuncu(&g0[0])), sut: true, ben: Some(1) };
    let (yx, yy) = crate::sekil::yer::bekci(&g0[1], x, y, Some(&bm), &kural);
    let (ux, uy) = ((W - x) / dd, (CY - y) / dd);
    let (nx, ny) = (-uy, ux);
    let sg = if ny >= 0.0 { 1.0 } else { -1.0 };
    b.x = (yx + sg * ko * nx - kd * ux).clamp(0.8, W - 0.8);
    b.y = (yy + sg * ko * ny - kd * uy).clamp(0.8, crate::saha::H - 0.8);
    let g = vec![s.clone(), b];
    let mut r = Rng::yeni(12345);
    let tac = m.tac.get(0).cloned().unwrap_or_default();
    let k = decide(m, &g[0], &g, ch, &mut r, &tac);
    let v = k.opts.iter().filter(|o| o.kind == "gönder").filter_map(|o| o.det.as_ref().and_then(|d| d.Pk)).fold(0.0, f64::max);
    Some(v)
}
