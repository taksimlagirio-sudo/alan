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
    k.src_c_kc.as_ref()
}

// ---------------------------------------------------------------- çekirdek / yetenek

/// JS ALAN_Y(p, k) (yetenek.js:322): mekanik adıyla.
pub fn alan_y(p: &Oyuncu, k: &str) -> f64 {
    p.govde.yet.y(super::kafa1_secim::mek_sira(k))
}
/// JS ALAN_OZ(p, k) (core.js:30)
pub fn alan_oz(p: &Oyuncu, k: &str) -> f64 {
    p.govde.yet.ozk(k)
}
/// JS ALAN_HZ (core.js:104): Enerji 3.0 açık → ham Hız.
pub fn alan_hz(p: &Oyuncu) -> f64 {
    p.hz()
}
/// JS ALAN_SPR (match.js:2168, sprint evresiyle): maçın.
pub fn alan_spr(k: &Kafa, p: &Oyuncu) -> f64 {
    k.dis.alan_spr(p)
}
/// JS ALAN_SPRE(p) (core.js:102): maçın.
pub fn alan_spre(k: &Kafa, p: &Oyuncu) -> f64 {
    k.dis.alan_spre(p)
}
/// JS ALAN_TEPKI(p, k) (core.js:1278): kafa1'in `TEPKI`'si (yerleşimin S'siyle).
pub fn alan_tepki(k: &Kafa, p: &Oyuncu, ad: &str) -> f64 {
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
pub fn alan_kosu_t(k: &Kafa, p: &Oyuncu, d: f64, v1: f64) -> f64 {
    k.dis.alan_kosu_t(p, d, v1)
}
/// JS ALAN_KOSU_M(p, t, v1) (match.js:2268)
pub fn alan_kosu_m(k: &Kafa, p: &Oyuncu, t: f64, v1: f64) -> f64 {
    k.dis.alan_kosu_m(p, t, v1)
}

// ---------------------------------------------------------------- kafa 1: saf yardımcılar

pub fn threat(k: &Kafa, team: f64, x: f64, y: f64) -> f64 {
    k.threat(team, x, y)
}
pub fn pas_max(k: &Kafa) -> f64 {
    k.pas_max()
}
pub fn kuyu_d(x: f64, y: f64) -> f64 {
    super::kafa1_tehlike::kuyu_d(x, y)
}
/// JS KY() = C().KUYU() (core.js:1124): hep açık (kafa1 de sabitledi).
pub fn KY() -> bool {
    true
}
/// JS EN2() (decide.js:208): Enerji 3.0 açık.
pub fn en2() -> bool {
    true
}
/// JS Z5() = ALAN_YET.acik(5): açık.
pub fn z5() -> bool {
    true
}
/// JS depoOn() (decide.js:3673): tek enerji açık.
pub fn depo_on() -> bool {
    true
}
/// JS holdF (decide.js:1317)
pub fn hold_f() -> bool {
    true
}
pub fn ch_kayip(k: &Kafa, ch: f64) -> f64 {
    k.ch_kayip(ch)
}
pub fn al_p(d: Option<&JODet>) -> Option<f64> {
    super::kafa1_fake::al_p(d)
}
pub fn zone_r(k: &Kafa, team: f64, x: f64, y: f64) -> f64 {
    k.zone_r(team, x, y)
}
pub fn press_at(k: &Kafa, src: &[Op], team: f64, x: f64, y: f64) -> f64 {
    k.press_at(src, team, x, y)
}
pub fn free_at(k: &Kafa, src: &[Op], team: f64, x: f64, y: f64) -> f64 {
    k.free_at(src, team, x, y)
}
pub fn tercih_ek(k: &Kafa, v: f64, d: f64) -> f64 {
    k.tercih_ek(v, d)
}
/// JS dokEk (decide.js:3618)
pub fn dok_ek(o: &mut Secenek, ad: &str, d: f64) {
    if let Some(t) = o.dok.as_mut() {
        if d != 0.0 && d.is_finite() {
            let v = t.al(ad).map(|x| veya(x, 0.0)).unwrap_or(0.0) + d;
            t.koy(ad, v);
        }
    }
}
/// JS teK (decide.js:3621)
pub fn te_k(k: &Kafa, o: &mut Secenek, ad: &str, d: f64) {
    let v0 = o.v.unwrap_or(f64::NAN);
    let v = k.tercih_ek(v0, d);
    o.v = Some(v);
    dok_ek(o, ad, v - v0);
}
/// JS dokKar (decide.js:3625)
pub fn dok_kar(o: &mut Secenek, w: f64, P: &[(&str, f64)]) {
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
pub fn core_side(k: &Kafa, h: &Oyuncu) -> (f64, f64, f64) {
    let c = k.core_side(h);
    (c.ux, c.uy, c.off)
}
pub fn core_ang(k: &Kafa, h: &Oyuncu) -> f64 {
    k.core_ang(h)
}
pub fn kenar_akil(k: &Kafa, h: &Oyuncu) -> bool {
    k.kenar_akil(h)
}
/// JS SPK (decide.js:214): sprint evresi ayrı (ALAN_SPR_FAZ.ayir).
pub fn spk(k: &Kafa) -> bool {
    k.spk()
}
/// JS ikKafa (decide.js:4603): AlanMatch.IK var mı
pub fn ik_kafa(k: &Kafa) -> bool {
    k.dis.IK().is_some()
}

// ---------------------------------------------------------------- kafa 1: tartı

pub fn V(k: &Kafa, src: &[Op], team: f64, x: f64, y: f64, ch: f64) -> f64 {
    k.V(src, team, x, y, ch)
}
pub fn vend(k: &mut Kafa, src: &[Op], team: f64, x: f64, y: f64, ch: f64) -> f64 {
    k.vend(src, team, x, y, ch)
}
pub fn run_on(k: &mut Kafa, w: &[Op], team: f64, h: &Oyuncu, at: Nokta) -> f64 {
    k.run_on(w, team, h, at)
}
/// JS laneOk(src, team, A, B, arr, lag)
pub fn lane_ok(k: &Kafa, src: &[Op], team: f64, A: Nokta, B: Nokta, arr: Option<f64>, lag: Option<f64>) -> f64 {
    k.lane_ok(src, team, A, B, arr, lag)
}
/// JS keepP(h, src, dir, T); yan çıktı `k.keepP_son`
pub fn keep_p(k: &mut Kafa, h: &Oyuncu, src: &[Op], dir: Option<Nokta>, T: f64) -> f64 {
    k.keep_p(h, src, dir, T)
}
pub fn recv_ch(k: &Kafa, src: &[Op], team: f64, x: f64, y: f64, ch: f64, q: &Oyuncu) -> f64 {
    k.recv_ch(src, team, x, y, ch, q)
}
pub fn shot_q(k: &Kafa, src: &[Op], team: f64, x: f64, y: f64, ch: f64) -> f64 {
    k.shot_q(src, team, x, y, ch)
}
pub fn sut_kapali(k: &Kafa, src: &[Op], team: f64, x: f64, y: f64, ch: f64) -> f64 {
    k.sut_kapali(src, team, x, y, ch)
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
pub fn opp_at(k: &Kafa, src: &[Op], team: f64, x: f64, y: f64) -> f64 {
    k.opp_at(src, team, x, y)
}
/// JS ucusT(L, arrive, al)
pub fn ucus_t(k: &Kafa, L: f64, arrive: f64, al: Option<f64>) -> f64 {
    k.ucus_t(L, arrive, al)
}
/// JS ucusSure(h, at, lv, srcC, ch, savT)
pub fn ucus_sure(k: &Kafa, h: &Oyuncu, at: Nokta, lv: &JOLaunch, src: &[Op], ch: f64, savT: Option<f64>) -> f64 {
    k.ucus_sure(h, at, nk(lv.vx, lv.vy), src, ch, savT, kc(k))
}
/// JS bankSure(h, B, T, lv, srcC, ch, savT)
pub fn bank_sure(k: &Kafa, h: &Oyuncu, B: Nokta, T: Nokta, lv: &JOLaunch, src: &[Op], ch: f64, savT: Option<f64>) -> f64 {
    k.bank_sure(h, B, T, nk(lv.vx, lv.vy), src, ch, savT, kc(k))
}
/// JS launchFor(h, to, arrive, srcC, ch) (decide0'ın srcC'siyle)
pub fn launch_for(k: &Kafa, h: &Oyuncu, to: Nokta, arrive: f64, src: &[Op], ch: f64) -> JOLaunch {
    k.launch_for(h, to, arrive, src, ch, kc(k))
}
/// JS launchFor(h, to, arrive, w, ch): düz dünyayla (K önbelleksiz; eski kafanın kontrası)
pub fn launch_for0(k: &Kafa, h: &Oyuncu, to: Nokta, arrive: f64, src: &[Op], ch: f64) -> JOLaunch {
    k.launch_for(h, to, arrive, src, ch, None)
}
/// JS bankFor(h, B, q, arrive, srcC, ch)
pub fn bank_for(k: &Kafa, h: &Oyuncu, B: Nokta, q: Nokta, arrive: f64, src: &[Op], ch: f64) -> JOLaunch {
    k.bank_for(h, B, q, arrive, src, ch, kc(k))
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
pub fn kenar_im(k: &mut Kafa, h: &Oyuncu, src: &[Op], b0: Top0, _rnd: &mut Zar, _trials: f64, q: Option<&Oyuncu>, to: Option<Nokta>) -> JODet {
    det_of(k.kenar_im(h, src, b0.b0(), q, to, None))
}
/// JS savurPlan(h, {vx, vy}, ca0, w0, sYon, src, itme, E)
pub fn savur_plan(k: &Kafa, h: &Oyuncu, vx: f64, vy: f64, ca0: f64, w0: f64, sYon: f64, src: &[Op], itme: bool, E: f64) -> Option<SavPlan> {
    k.savur_plan(h, nk(vx, vy), ca0, w0, sYon, Some(src), itme, E)
}
pub fn ca_of(k: &Kafa, h: &Oyuncu) -> f64 {
    k.ca_of(h)
}
/// JS vurE(h, ch, gon)
pub fn vur_e(k: &Kafa, h: &Oyuncu, ch: Option<f64>, gon: bool) -> f64 {
    k.vur_e(h, ch, gon)
}
pub fn sut_guc(k: &Kafa, h: &Oyuncu) -> f64 {
    k.sut_guc(h)
}
pub fn sut_v(k: &Kafa, h: &Oyuncu) -> f64 {
    k.sut_v(h)
}
pub fn gon_ek(k: &Kafa, h: &Oyuncu, src: &[Op], shot: &str) -> f64 {
    k.gon_ek(h, src, shot)
}
/// JS sutHedef(h, src, gx): nişan y'leri. `h` sahte olabilir (sutYol: id −9).
pub fn sut_hedef(k: &Kafa, h: &Oyuncu, src: &[Op], gx: f64) -> Vec<f64> {
    k.sut_hedef(h, src, gx)
}
/// JS surSure(h, L, dir, spr) (`spr` JS doğruluğuyla)
pub fn sur_sure(k: &Kafa, h: &Oyuncu, L: f64, dir: Option<Nokta>, spr: Option<f64>) -> f64 {
    k.sur_sure(h, L, dir, spr.map(dogru).unwrap_or(false))
}
/// JS sprKop(h): sprint niyetli kopya
pub fn spr_kop(k: &Kafa, h: &Oyuncu) -> Oyuncu {
    k.spr_kop(h)
}
pub fn sarj_tut(k: &Kafa, h: &Oyuncu, src: &[Op], ch: f64, still: f64, T: f64) -> Option<SarjT> {
    k.sarj_tut(h, src, ch, still, T)
}
/// JS chargeRate(h, src, still, ch) (`ham` yok)
pub fn charge_rate(k: &Kafa, h: &Oyuncu, src: &[Op], still: f64, ch: f64) -> f64 {
    k.charge_rate(h, src, still, ch, false)
}
pub fn top_k(k: &Kafa, h: &Oyuncu) -> f64 {
    k.top_k(h)
}
pub fn don_s(k: &Kafa, p: &Oyuncu, ux: f64, uy: f64) -> f64 {
    k.don_s(p, ux, uy)
}
/// JS duelP(q, h, src) (zorK yok)
pub fn duel_p(k: &mut Kafa, q: &Oyuncu, h: &Oyuncu, src: &[Op]) -> f64 {
    k.duel_p(q, h, src, None)
}
/// JS enW(w, o): enerji muhasebesinin dünyası (değişmediyse aynı dünya)
pub fn en_w(k: &Kafa, w: &Arc<Dunya>, o: &Secenek) -> Arc<Dunya> {
    match k.en_w(w, Some(o)) {
        Cow::Borrowed(_) => w.clone(),
        Cow::Owned(v) => Arc::new(crate::dunya::Dunya::from(v)),
    }
}
/// JS enGec(q, w, f): `q.en`'in geçici değeri (None: değişmez). Çağıran q'nun kopyasıyla f'i çağırır.
pub fn enGec(k: &Kafa, q: &Oyuncu, w: &[Op]) -> Option<f64> {
    k.en_gec_en(q, w)
}
pub fn en_muhasebe(k: &Kafa, h: &Oyuncu, src: &[Op], w: &[Op], T: f64, bit: Option<Nokta>, o: &Secenek) -> crate::model::JOEn {
    k.en_muhasebe(h, src, w, T, bit, Some(o))
}
/// options'ın döndürdüğü dizi ve diziye takılı ek listeler (JS `opts.tekH`, `opts.tekS`, `opts.tekC`).
/// JS'te tekS / tekC yalnız TOPLA'da (h.id ≥ 0) kurulur; yoksa (`undefined`) yedek adaylar: None.
/// `tekC` burada adayların noktaları (kafa2 yalnız `c.P`'yi okur).
#[derive(Default)]
pub struct Secenekler {
    pub out: Vec<Secenek>,
    /// alıcı (sıra) → tek dokunuş hedefleri
    pub tek_h: Vec<(Pid, Vec<Nk>)>,
    pub tekS: Option<Vec<Nk>>,
    pub tek_c: Option<Vec<Nk>>,
}
/// JS options(h, src, ch)
pub fn options(k: &Kafa, h: &Oyuncu, src: &[Op], ch: f64) -> Secenekler {
    let r = k.options(h, src, ch);
    let topla = h.id >= 0.0;
    Secenekler {
        out: r.out,
        tek_h: r.tek_h,
        tekS: if topla { Some(r.tekS) } else { None },
        tek_c: if topla { Some(r.tek_c.iter().map(|c| c.P).collect()) } else { None },
    }
}
pub fn bos_oy(k: &Kafa, src: &[Op], team: f64, bx: f64, by: f64) -> BosOy {
    k.bos_oy(src, team, bx, by)
}
/// JS applyError(h, v, rnd, undefined, gon, src, tur, kenar)
pub fn apply_error(k: &Kafa, h: &Oyuncu, v: &JOLaunch, rnd: &mut Zar, gon: bool, src: &[Op], tur: Option<&str>, kenar: bool) -> (f64, f64) {
    let r = k.apply_error(h, nk(v.vx, v.vy), rnd, 0.0, gon, Some(src), tur, Some(kenar));
    (r.x, r.y)
}
/// JS chasePlan(pts, ps, t0): oyuncu (ps sırası) → (x, y, rc)
pub fn chase_plan(k: &Kafa, pts: &[crate::top::YolNokta], ps: &[Op], t0: f64) -> Vec<(Pid, f64, f64, f64)> {
    let p: Vec<Nk> = pts.iter().map(|q| nk(q.x, q.y)).collect();
    k.chase_plan(&p, ps, t0, None).into_iter().map(|(i, c)| (i, c.x, c.y, c.rc)).collect()
}
pub fn recv_point(k: &Kafa, pts: &[crate::top::YolNokta], p: &Oyuncu) -> Option<Nokta> {
    let q: Vec<Nk> = pts.iter().map(|q| nk(q.x, q.y)).collect();
    Some(k.recv_point(&q, p))
}
/// JS secim(h, {opts, best: null, T}, src) (decideOT'nin seçimi)
pub fn secim(k: &Kafa, h: &Oyuncu, opts: Vec<Secenek>, T: f64, src: &[Op]) -> Option<Secenek> {
    let i = k.secim(h, &opts, None, T, src)?;
    opts.into_iter().nth(i)
}

// ---------------------------------------------------------------- yerleşim (AlanShape): Dis üstünden

/// JS AlanShape.bekci(p, B, m): `fm` yoksa `m = null`.
pub fn as_bekci(k: &Kafa, p: &Oyuncu, B: Nokta, fm: Option<&Fm>) -> Nokta {
    match fm {
        Some(fm) => k.dis.bekci_fm(p, B, fm),
        None => k.dis.bekci(p, B).unwrap_or(nk(p.x, p.y)),
    }
}
/// JS AlanShape.slotBase(m, p, B, hucum)
pub fn as_slot_base(k: &Kafa, fm: &Fm, p: &Oyuncu, B: Nokta, hucum: bool) -> Nokta {
    k.dis.slot_base(fm, p, B, hucum)
}
/// JS AlanShape.gside(E, team, d)
pub fn as_gside(k: &Kafa, E: Nokta, team: f64, d: f64) -> Nokta {
    k.dis.gside(E, team, d)
}
/// JS AlanShape.gecTab(p) (None: yerleşimde yok)
pub fn as_gec_tab(k: &Kafa, p: &Oyuncu) -> Option<GecTab> {
    k.dis.gec_tab(p, false)
}
/// JS AlanShape.gecOf(G, x, y, d) (`G` yoksa 0)
pub fn as_gec_of(k: &Kafa, G: &Option<GecTab>, x: f64, y: f64, d: f64) -> f64 {
    match G {
        Some(G) => k.dis.gec_of(G, x, y, Some(d)),
        None => 0.0,
    }
}
/// JS `AS && AS.gec ? AS.gec(p, x, y, d) : 0`
pub fn as_gec(k: &Kafa, p: &Oyuncu, x: f64, y: f64, d: f64) -> f64 {
    k.dis.gec(p, x, y, Some(d)).unwrap_or(0.0)
}
/// shape.js:11 vSp(p) = spd(p) * ALAN_SPRE(p)
pub fn as_v_sp(k: &Kafa, p: &Oyuncu) -> f64 {
    k.v_s(p)
}
/// JS AlanShape.S.prHat[blok]
pub fn as_s_pr_hat(k: &Kafa, blok: &str) -> Option<f64> {
    k.dis.s_pr_hat(blok)
}
/// JS AlanShape.S.rcvBuf (yoksa NaN: çağıran `|| 0` uygular)
pub fn as_s_rcv_buf(k: &Kafa) -> f64 {
    k.dis.S().map(|s| s.rcv_buf).unwrap_or(f64::NAN)
}

// ---------------------------------------------------------------- maç (AlanMatch): Dis üstünden

/// JS AlanMatch.tl(p, ev, k) (`null` → 0: yalnız `>= .5` ile karşılaştırılır)
pub fn am_tl(k: &Kafa, p: &Oyuncu, a: &str, b: &str) -> f64 {
    k.dis.tl(p, a, b).unwrap_or(0.0)
}
/// JS AlanMatch.ivme(q, wx, wy, vmax): q.vx, q.vy yazar
pub fn am_ivme(k: &Kafa, q: &mut Oyuncu, wx: f64, wy: f64, vmax: f64) {
    k.dis.ivme(q, wx, wy, vmax)
}
/// match.js:311 spd(p) = .17 + .006 * ALAN_HZ(p)
pub fn am_spd(k: &Kafa, p: &Oyuncu) -> f64 {
    k.spd(p)
}
/// match.js:311 sprK(p) (enerjiyle kısılmış sprint katı)
pub fn am_spr_k(k: &Kafa, p: &Oyuncu) -> f64 {
    k.dis.spr_k(p)
}

/// JS vurTaban(p) (core.js:410): vuruşun taban enerjisi (kafa1'in top_yap'ındakiyle aynı).
pub fn vur_taban(p: &Oyuncu) -> f64 {
    crate::saha::P.tek_taban * jmax(0.5, jmin(1.5, 1.0 + 0.05 * (p.govde.yet.y(crate::yetenek::mek::VURUS_TABAN) - 10.0)))
}

/// JMTacE'yi kullanmayan uyarıyı susturur (tip, imzalarda görünür)
#[allow(unused)]
type _T = JMTacE;
