//! Kafa 2'nin kafa1'e ve dış dünyaya bağı (eski `gecici_dis2.rs`).
//!
//! Kafa2'nin dosyaları bunu `dis` adıyla çağırır. Buradaki işlevler gerçeklerine bağlanmıştır:
//! - **kafa1** (`decide.js` 1–4716): `impl Kafa` yöntemleri (`k.X(..)`); burada yalnız imza farkları
//!   uydurulur (K önbelleği `Kc`, `Option` dönüşleri, kafa1'in tipleri).
//! - **maç / yerleşim / enerji**: kafa1'in dış dünyası [`Dis`] (`k.dis.X(..)`).
//! - Birkaç küçük JS yardımcısı kafa1'de yoktu (dokEk/teK/dokKar, sabit bayraklar KY/EN2/Z5/depoOn/holdF):
//!   gerçekleri burada.
//!
//! K'nin kipi: decide0 kararın başında `k.srcC_kc`'yi kurar (JS `srcC = Object.assign(src.slice(),
//! {_kc: new Map})`). JS'te `launchFor/ucusSure/bankSure/bankFor` decide0 içinde hep `srcC` ile,
//! `imagine/kenarIm` düz `src` ile çağrılır: burada da öyle.
#![allow(non_snake_case, dead_code)]

use crate::kafa::gecici_dis1::KuralErisim as _;
use super::kafa1_js::Kc;
use super::kafa1_ucus::{Ucus, UcusB0};
use super::kafa2_baglam::*;
use crate::mat::{dogru, jmax, jmin};
use crate::model::{JMTacE, JODet, JOLaunch, Oyuncu, Secenek};
use crate::oyuncu::Tepki;
use std::borrow::Cow;
use std::sync::Arc;

pub use super::gecici_dis1::{Fm, GecTab};
pub use super::kafa1_enerji::SarjTut as SarjT;
pub use super::kafa1_savur::SavPlan;
pub use super::kafa1_tehlike::{BosOy, SPT};
pub use super::kafa1_varis::Kovala;

/// Kafa 1'in nokta tipi (modelin `Nokta`'sı)
pub type Nk = Nokta;

#[inline(always)]
fn kc(k: &Kafa) -> Kc<'_> {
    k.srcC_kc.as_ref()
}

// ---------------------------------------------------------------- çekirdek / yetenek

/// JS ALAN_Y(p, k) (yetenek.js:322): mekanik adıyla.
pub fn ALAN_Y(p: &Oyuncu, k: &str) -> f64 {
    p.govde.yet.y(super::kafa1_secim::mek_sira(k))
}
/// JS ALAN_OZ(p, k) (core.js:30)
pub fn ALAN_OZ(p: &Oyuncu, k: &str) -> f64 {
    p.govde.yet.ozk(k)
}
/// JS ALAN_HZ (core.js:104): Enerji 3.0 açık → ham Hız.
pub fn ALAN_HZ(p: &Oyuncu) -> f64 {
    p.hz()
}
/// JS ALAN_SPR (match.js:2168, sprint evresiyle): maçın.
pub fn ALAN_SPR(k: &Kafa, p: &Oyuncu) -> f64 {
    k.dis.ALAN_SPR(p)
}
/// JS ALAN_SPRE(p) (core.js:102): maçın.
pub fn ALAN_SPRE(k: &Kafa, p: &Oyuncu) -> f64 {
    k.dis.ALAN_SPRE(p)
}
/// JS ALAN_TEPKI(p, k) (core.js:1278): kafa1'in `TEPKI`'si (yerleşimin S'siyle).
pub fn ALAN_TEPKI(k: &Kafa, p: &Oyuncu, ad: &str) -> f64 {
    let t = match ad {
        "alan" => Tepki::Alan,
        "algi" => Tepki::Algi,
        "karar" => Tepki::Karar,
        "yeniTop" => Tepki::YeniTop,
        "kovala" => Tepki::Kovala,
        "bosTop" => Tepki::BosTop,
        "uzanma" => Tepki::Uzanma,
        _ => panic!("ALAN_TEPKI: bilinmeyen tür {ad}"),
    };
    k.TEPKI(p, t)
}
/// JS ALAN_KOSU_T(p, d, v1) (match.js:2262)
pub fn ALAN_KOSU_T(k: &Kafa, p: &Oyuncu, d: f64, v1: f64) -> f64 {
    k.dis.ALAN_KOSU_T(p, d, v1)
}
/// JS ALAN_KOSU_M(p, t, v1) (match.js:2268)
pub fn ALAN_KOSU_M(k: &Kafa, p: &Oyuncu, t: f64, v1: f64) -> f64 {
    k.dis.ALAN_KOSU_M(p, t, v1)
}

// ---------------------------------------------------------------- kafa 1: saf yardımcılar

pub fn threat(k: &Kafa, team: f64, x: f64, y: f64) -> f64 {
    k.threat(team, x, y)
}
pub fn pasMax(k: &Kafa) -> f64 {
    k.pasMax()
}
pub fn kuyuD(x: f64, y: f64) -> f64 {
    super::kafa1_tehlike::kuyuD(x, y)
}
/// JS KY() = C().KUYU() (core.js:1124): hep açık (kafa1 de sabitledi).
pub fn KY() -> bool {
    true
}
/// JS EN2() (decide.js:208): Enerji 3.0 açık.
pub fn EN2() -> bool {
    true
}
/// JS Z5() = ALAN_YET.acik(5): açık.
pub fn Z5() -> bool {
    true
}
/// JS depoOn() (decide.js:3673): tek enerji açık.
pub fn depoOn() -> bool {
    true
}
/// JS holdF (decide.js:1317)
pub fn holdF() -> bool {
    true
}
pub fn chKayip(k: &Kafa, ch: f64) -> f64 {
    k.chKayip(ch)
}
pub fn alP(d: Option<&JODet>) -> Option<f64> {
    super::kafa1_fake::alP(d)
}
pub fn zoneR(k: &Kafa, team: f64, x: f64, y: f64) -> f64 {
    k.zoneR(team, x, y)
}
pub fn pressAt(k: &Kafa, src: &[Op], team: f64, x: f64, y: f64) -> f64 {
    k.pressAt(src, team, x, y)
}
pub fn freeAt(k: &Kafa, src: &[Op], team: f64, x: f64, y: f64) -> f64 {
    k.freeAt(src, team, x, y)
}
pub fn tercihEk(k: &Kafa, v: f64, d: f64) -> f64 {
    k.tercihEk(v, d)
}
/// JS dokEk (decide.js:3618)
pub fn dokEk(o: &mut Secenek, ad: &str, d: f64) {
    if let Some(t) = o.dok.as_mut() {
        if d != 0.0 && d.is_finite() {
            let v = t.al(ad).map(|x| veya(x, 0.0)).unwrap_or(0.0) + d;
            t.koy(ad, v);
        }
    }
}
/// JS teK (decide.js:3621)
pub fn teK(k: &Kafa, o: &mut Secenek, ad: &str, d: f64) {
    let v0 = o.v.unwrap_or(f64::NAN);
    let v = k.tercihEk(v0, d);
    o.v = Some(v);
    dokEk(o, ad, v - v0);
}
/// JS dokKar (decide.js:3625)
pub fn dokKar(o: &mut Secenek, w: f64, P: &[(&str, f64)]) {
    if let Some(t) = o.dok.as_mut() {
        for x in t.k.iter_mut() {
            x.1 *= 1.0 - w;
        }
        for (ad, v) in P {
            if *v != 0.0 && *v == *v {
                let a = t.al(ad).map(|x| veya(x, 0.0)).unwrap_or(0.0) + w * v;
                t.koy(ad, a);
            }
        }
    }
}
/// JS coreSide (decide.js:4561): (ux, uy, off)
pub fn coreSide(k: &Kafa, h: &Oyuncu) -> (f64, f64, f64) {
    let c = k.coreSide(h);
    (c.ux, c.uy, c.off)
}
pub fn coreAng(k: &Kafa, h: &Oyuncu) -> f64 {
    k.coreAng(h)
}
pub fn kenarAkil(k: &Kafa, h: &Oyuncu) -> bool {
    k.kenarAkil(h)
}
/// JS SPK (decide.js:214): sprint evresi ayrı (ALAN_SPR_FAZ.ayir).
pub fn SPK(k: &Kafa) -> bool {
    k.SPK()
}
/// JS ikKafa (decide.js:4603): AlanMatch.IK var mı
pub fn ikKafa(k: &Kafa) -> bool {
    k.dis.IK().is_some()
}

// ---------------------------------------------------------------- kafa 1: tartı

pub fn V(k: &Kafa, src: &[Op], team: f64, x: f64, y: f64, ch: f64) -> f64 {
    k.V(src, team, x, y, ch)
}
pub fn Vend(k: &mut Kafa, src: &[Op], team: f64, x: f64, y: f64, ch: f64) -> f64 {
    k.Vend(src, team, x, y, ch)
}
pub fn runOn(k: &mut Kafa, w: &[Op], team: f64, h: &Oyuncu, at: Nokta) -> f64 {
    k.runOn(w, team, h, at)
}
/// JS laneOk(src, team, A, B, arr, lag)
pub fn laneOk(k: &Kafa, src: &[Op], team: f64, A: Nokta, B: Nokta, arr: Option<f64>, lag: Option<f64>) -> f64 {
    k.laneOk(src, team, A, B, arr, lag)
}
/// JS keepP(h, src, dir, T); yan çıktı `k.keepP_son`
pub fn keepP(k: &mut Kafa, h: &Oyuncu, src: &[Op], dir: Option<Nokta>, T: f64) -> f64 {
    k.keepP(h, src, dir, T)
}
pub fn recvCh(k: &Kafa, src: &[Op], team: f64, x: f64, y: f64, ch: f64, q: &Oyuncu) -> f64 {
    k.recvCh(src, team, x, y, ch, q)
}
pub fn shotQ(k: &Kafa, src: &[Op], team: f64, x: f64, y: f64, ch: f64) -> f64 {
    k.shotQ(src, team, x, y, ch)
}
pub fn sutKapali(k: &Kafa, src: &[Op], team: f64, x: f64, y: f64, ch: f64) -> f64 {
    k.sutKapali(src, team, x, y, ch)
}
/// JS respond(src, team, at, T, kind): sonuç dünyası (değişen oyuncular yeni `Arc`, JS `{...p, x, y}`).
pub fn respond(k: &Kafa, src: &[Op], team: f64, at: Nokta, T: f64, kind: &str) -> Dunya {
    respond_d(k, src, team, at, T, kind).0
}
/// `respond` ve hangi oyuncunun değiştiği (yeni yere gidip yeni kopya olduğu): "değişti mi" nesne kimliğinden
/// değil açıkça (MIMARI.md 3.2 B).
pub fn respond_d(k: &Kafa, src: &[Op], team: f64, at: Nokta, T: f64, kind: &str) -> (Dunya, Vec<bool>) {
    let r = k.respond(src, team, at, T, kind);
    let degisti: Vec<bool> = r.iter().map(|y| y.is_some()).collect();
    let w = src.iter().zip(r).map(|(p, y)| match y {
        Some(n) => kxy(p, n.x, n.y),
        None => p.clone(),
    }).collect();
    (w, degisti)
}
pub fn oppAt(k: &Kafa, src: &[Op], team: f64, x: f64, y: f64) -> f64 {
    k.oppAt(src, team, x, y)
}
/// JS ucusT(L, arrive, al)
pub fn ucusT(k: &Kafa, L: f64, arrive: f64, al: Option<f64>) -> f64 {
    k.ucusT(L, arrive, al)
}
/// JS ucusSure(h, at, lv, srcC, ch, savT)
pub fn ucusSure(k: &Kafa, h: &Oyuncu, at: Nokta, lv: &JOLaunch, src: &[Op], ch: f64, savT: Option<f64>) -> f64 {
    k.ucusSure(h, at, nk(lv.vx, lv.vy), src, ch, savT, kc(k))
}
/// JS bankSure(h, B, T, lv, srcC, ch, savT)
pub fn bankSure(k: &Kafa, h: &Oyuncu, B: Nokta, T: Nokta, lv: &JOLaunch, src: &[Op], ch: f64, savT: Option<f64>) -> f64 {
    k.bankSure(h, B, T, nk(lv.vx, lv.vy), src, ch, savT, kc(k))
}
/// JS launchFor(h, to, arrive, srcC, ch) (decide0'ın srcC'siyle)
pub fn launchFor(k: &Kafa, h: &Oyuncu, to: Nokta, arrive: f64, src: &[Op], ch: f64) -> JOLaunch {
    k.launchFor(h, to, arrive, src, ch, kc(k))
}
/// JS launchFor(h, to, arrive, w, ch): düz dünyayla (K önbelleksiz; eski kafanın kontrası)
pub fn launchFor0(k: &Kafa, h: &Oyuncu, to: Nokta, arrive: f64, src: &[Op], ch: f64) -> JOLaunch {
    k.launchFor(h, to, arrive, src, ch, None)
}
/// JS bankFor(h, B, q, arrive, srcC, ch)
pub fn bankFor(k: &Kafa, h: &Oyuncu, B: Nokta, q: Nokta, arrive: f64, src: &[Op], ch: f64) -> JOLaunch {
    k.bankFor(h, B, q, arrive, src, ch, kc(k))
}
/// imagine'in topu (JS b0: {x, y, vx, vy, ch, aim})
#[derive(Clone, Copy, Debug, Default)]
pub struct Top0 {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub ch: f64,
    pub aim: Option<Nokta>,
}
impl Top0 {
    fn b0(&self) -> UcusB0 {
        UcusB0 { x: self.x, y: self.y, vx: self.vx, vy: self.vy, ch: self.ch, aim: self.aim }
    }
}
fn det_of(u: Ucus) -> JODet {
    JODet {
        P: Some(u.P),
        Pk: Some(u.Pk),
        Pl: Some(u.Pl),
        end: Some(u.end),
        endM: Some(u.endM),
        endO: Some(u.endO),
        Ps: Some(u.Ps),
        endS: Some(u.endS),
        endD: Some(u.endD),
        Psek: Some(u.Psek),
        endSek: Some(u.endSek),
        ..Default::default()
    }
}
/// JS imagine(h, src, b0, rnd, trials, q, to) = ucusFormul(h, src, b0, q, to) (zar ve deneme sayısı kullanılmaz)
pub fn imagine(k: &mut Kafa, h: &Oyuncu, src: &[Op], b0: Top0, _rnd: &mut Zar, _trials: f64, q: Option<&Oyuncu>, to: Option<Nokta>) -> JODet {
    det_of(k.imagine(h, src, b0.b0(), q, to, None))
}
/// JS kenarIm (decide.js:2315): `D._kenar` açıkken imagine
pub fn kenarIm(k: &mut Kafa, h: &Oyuncu, src: &[Op], b0: Top0, _rnd: &mut Zar, _trials: f64, q: Option<&Oyuncu>, to: Option<Nokta>) -> JODet {
    det_of(k.kenarIm(h, src, b0.b0(), q, to, None))
}
/// JS savurPlan(h, {vx, vy}, ca0, w0, sYon, src, itme, E)
pub fn savurPlan(k: &Kafa, h: &Oyuncu, vx: f64, vy: f64, ca0: f64, w0: f64, sYon: f64, src: &[Op], itme: bool, E: f64) -> Option<SavPlan> {
    k.savurPlan(h, nk(vx, vy), ca0, w0, sYon, Some(src), itme, E)
}
pub fn caOf(k: &Kafa, h: &Oyuncu) -> f64 {
    k.caOf(h)
}
/// JS vurE(h, ch, gon)
pub fn vurE(k: &Kafa, h: &Oyuncu, ch: Option<f64>, gon: bool) -> f64 {
    k.vurE(h, ch, gon)
}
pub fn sutGuc(k: &Kafa, h: &Oyuncu) -> f64 {
    k.sutGuc(h)
}
pub fn sutV(k: &Kafa, h: &Oyuncu) -> f64 {
    k.sutV(h)
}
pub fn gonEk(k: &Kafa, h: &Oyuncu, src: &[Op], shot: &str) -> f64 {
    k.gonEk(h, src, shot)
}
/// JS sutHedef(h, src, gx): nişan y'leri. `h` sahte olabilir (sutYol: id −9).
pub fn sutHedef(k: &Kafa, h: &Oyuncu, src: &[Op], gx: f64) -> Vec<f64> {
    k.sutHedef(h, src, gx)
}
/// JS surSure(h, L, dir, spr) (`spr` JS doğruluğuyla)
pub fn surSure(k: &Kafa, h: &Oyuncu, L: f64, dir: Option<Nokta>, spr: Option<f64>) -> f64 {
    k.surSure(h, L, dir, spr.map(dogru).unwrap_or(false))
}
/// JS sprKop(h): sprint niyetli kopya
pub fn sprKop(k: &Kafa, h: &Oyuncu) -> Oyuncu {
    k.sprKop(h)
}
pub fn sarjTut(k: &Kafa, h: &Oyuncu, src: &[Op], ch: f64, still: f64, T: f64) -> Option<SarjT> {
    k.sarjTut(h, src, ch, still, T)
}
/// JS chargeRate(h, src, still, ch) (`ham` yok)
pub fn chargeRate(k: &Kafa, h: &Oyuncu, src: &[Op], still: f64, ch: f64) -> f64 {
    k.chargeRate(h, src, still, ch, false)
}
pub fn topK(k: &Kafa, h: &Oyuncu) -> f64 {
    k.topK(h)
}
pub fn donS(k: &Kafa, p: &Oyuncu, ux: f64, uy: f64) -> f64 {
    k.donS(p, ux, uy)
}
/// JS duelP(q, h, src) (zorK yok)
pub fn duelP(k: &mut Kafa, q: &Oyuncu, h: &Oyuncu, src: &[Op]) -> f64 {
    k.duelP(q, h, src, None)
}
/// JS enW(w, o): enerji muhasebesinin dünyası (değişmediyse aynı dünya)
pub fn enW(k: &Kafa, w: &Arc<Dunya>, o: &Secenek) -> Arc<Dunya> {
    match k.enW(w, Some(o)) {
        Cow::Borrowed(_) => w.clone(),
        Cow::Owned(v) => Arc::new(crate::dunya::Dunya::from(v)),
    }
}
/// JS enGec(q, w, f): `q.en`'in geçici değeri (None: değişmez). Çağıran q'nun kopyasıyla f'i çağırır.
pub fn enGec(k: &Kafa, q: &Oyuncu, w: &[Op]) -> Option<f64> {
    k.enGec_en(q, w)
}
pub fn enMuhasebe(k: &Kafa, h: &Oyuncu, src: &[Op], w: &[Op], T: f64, bit: Option<Nokta>, o: &Secenek) -> crate::model::JOEn {
    k.enMuhasebe(h, src, w, T, bit, Some(o))
}
/// options'ın döndürdüğü dizi ve diziye takılı ek listeler (JS `opts.tekH`, `opts.tekS`, `opts.tekC`).
/// JS'te tekS / tekC yalnız TOPLA'da (h.id ≥ 0) kurulur; yoksa (`undefined`) yedek adaylar: None.
/// `tekC` burada adayların noktaları (kafa2 yalnız `c.P`'yi okur).
#[derive(Default)]
pub struct Secenekler {
    pub out: Vec<Secenek>,
    /// alıcı (sıra) → tek dokunuş hedefleri
    pub tekH: Vec<(Pid, Vec<Nk>)>,
    pub tekS: Option<Vec<Nk>>,
    pub tekC: Option<Vec<Nk>>,
}
/// JS options(h, src, ch)
pub fn options(k: &Kafa, h: &Oyuncu, src: &[Op], ch: f64) -> Secenekler {
    let r = k.options(h, src, ch);
    let topla = h.id >= 0.0;
    Secenekler {
        out: r.out,
        tekH: r.tekH,
        tekS: if topla { Some(r.tekS) } else { None },
        tekC: if topla { Some(r.tekC.iter().map(|c| c.P).collect()) } else { None },
    }
}
pub fn bosOy(k: &Kafa, src: &[Op], team: f64, bx: f64, by: f64) -> BosOy {
    k.bosOy(src, team, bx, by)
}
/// JS applyError(h, v, rnd, undefined, gon, src, tur, kenar)
pub fn applyError(k: &Kafa, h: &Oyuncu, v: &JOLaunch, rnd: &mut Zar, gon: bool, src: &[Op], tur: Option<&str>, kenar: bool) -> (f64, f64) {
    let r = k.applyError(h, nk(v.vx, v.vy), rnd, 0.0, gon, Some(src), tur, Some(kenar));
    (r.x, r.y)
}
/// JS chasePlan(pts, ps, t0): oyuncu (ps sırası) → (x, y, rc)
pub fn chasePlan(k: &Kafa, pts: &[crate::top::YolNokta], ps: &[Op], t0: f64) -> Vec<(Pid, f64, f64, f64)> {
    let p: Vec<Nk> = pts.iter().map(|q| nk(q.x, q.y)).collect();
    k.chasePlan(&p, ps, t0, None).into_iter().map(|(i, c)| (i, c.x, c.y, c.rc)).collect()
}
pub fn recvPoint(k: &Kafa, pts: &[crate::top::YolNokta], p: &Oyuncu) -> Option<Nokta> {
    let q: Vec<Nk> = pts.iter().map(|q| nk(q.x, q.y)).collect();
    Some(k.recvPoint(&q, p))
}
/// JS secim(h, {opts, best: null, T}, src) (decideOT'nin seçimi)
pub fn secim(k: &Kafa, h: &Oyuncu, opts: Vec<Secenek>, T: f64, src: &[Op]) -> Option<Secenek> {
    let i = k.secim(h, &opts, None, T, src)?;
    opts.into_iter().nth(i)
}

// ---------------------------------------------------------------- yerleşim (AlanShape): Dis üstünden

/// JS AlanShape.bekci(p, B, m): `fm` yoksa `m = null`.
pub fn AS_bekci(k: &Kafa, p: &Oyuncu, B: Nokta, fm: Option<&Fm>) -> Nokta {
    match fm {
        Some(fm) => k.dis.bekciFm(p, B, fm),
        None => k.dis.bekci(p, B).unwrap_or(nk(p.x, p.y)),
    }
}
/// JS AlanShape.slotBase(m, p, B, hucum)
pub fn AS_slotBase(k: &Kafa, fm: &Fm, p: &Oyuncu, B: Nokta, hucum: bool) -> Nokta {
    k.dis.slotBase(fm, p, B, hucum)
}
/// JS AlanShape.gside(E, team, d)
pub fn AS_gside(k: &Kafa, E: Nokta, team: f64, d: f64) -> Nokta {
    k.dis.gside(E, team, d)
}
/// JS AlanShape.gecTab(p) (None: yerleşimde yok)
pub fn AS_gecTab(k: &Kafa, p: &Oyuncu) -> Option<GecTab> {
    k.dis.gecTab(p, false)
}
/// JS AlanShape.gecOf(G, x, y, d) (`G` yoksa 0)
pub fn AS_gecOf(k: &Kafa, G: &Option<GecTab>, x: f64, y: f64, d: f64) -> f64 {
    match G {
        Some(G) => k.dis.gecOf(G, x, y, Some(d)),
        None => 0.0,
    }
}
/// JS `AS && AS.gec ? AS.gec(p, x, y, d) : 0`
pub fn AS_gec(k: &Kafa, p: &Oyuncu, x: f64, y: f64, d: f64) -> f64 {
    k.dis.gec(p, x, y, Some(d)).unwrap_or(0.0)
}
/// shape.js:11 vSp(p) = spd(p) * ALAN_SPRE(p)
pub fn AS_vSp(k: &Kafa, p: &Oyuncu) -> f64 {
    k.vS(p)
}
/// JS AlanShape.S.prHat[blok]
pub fn AS_S_prHat(k: &Kafa, blok: &str) -> Option<f64> {
    k.dis.S_prHat(blok)
}
/// JS AlanShape.S.rcvBuf (yoksa NaN: çağıran `|| 0` uygular)
pub fn AS_S_rcvBuf(k: &Kafa) -> f64 {
    k.dis.S().map(|s| s.rcvBuf).unwrap_or(f64::NAN)
}

// ---------------------------------------------------------------- maç (AlanMatch): Dis üstünden

/// JS AlanMatch.tl(p, ev, k) (`null` → 0: yalnız `>= .5` ile karşılaştırılır)
pub fn AM_tl(k: &Kafa, p: &Oyuncu, a: &str, b: &str) -> f64 {
    k.dis.tl(p, a, b).unwrap_or(0.0)
}
/// JS AlanMatch.ivme(q, wx, wy, vmax): q.vx, q.vy yazar
pub fn AM_ivme(k: &Kafa, q: &mut Oyuncu, wx: f64, wy: f64, vmax: f64) {
    k.dis.ivme(q, wx, wy, vmax)
}
/// match.js:311 spd(p) = .17 + .006 * ALAN_HZ(p)
pub fn AM_spd(k: &Kafa, p: &Oyuncu) -> f64 {
    k.spd(p)
}
/// match.js:311 sprK(p) (enerjiyle kısılmış sprint katı)
pub fn AM_sprK(k: &Kafa, p: &Oyuncu) -> f64 {
    k.dis.sprK(p)
}

/// JS vurTaban(p) (core.js:410): vuruşun taban enerjisi (kafa1'in top_yap'ındakiyle aynı).
pub fn vurTaban(p: &Oyuncu) -> f64 {
    crate::saha::P.tek_taban * jmax(0.5, jmin(1.5, 1.0 + 0.05 * (p.govde.yet.y(crate::yetenek::mek::VURUS_TABAN) - 10.0)))
}

/// JMTacE'yi kullanmayan uyarıyı susturur (tip, imzalarda görünür)
#[allow(unused)]
type _T = JMTacE;
