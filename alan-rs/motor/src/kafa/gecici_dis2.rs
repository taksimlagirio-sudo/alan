//! Kafa 2'nin çağırdığı yabancı işlevler (geçici). Birleştirmede gerçeklerine bağlanacak.
//!
//! Kaynak (sade JS, `alan-rs/kaynak/`):
//! - Kafa 1 (`decide.js` 1–4716): threat:136, oppAt:140, pasMax:135, kuyuD:131, KY:131, gonEk:202,
//!   EN2:208, SPK:214, sprKop:219, topK:244, donS:253, fakeSecenek:334, vurE/sutV:564, savurPlan:565,
//!   caOf:730, applyError:777, chasePlan:808, recvPoint:849, kopyaDurum:1111, holdF:1317, alP/SAL0:1375,
//!   kenarIm:2315, imagine:2324, kuyrukBosalt:2368, imSonuc:2393, sutGuc:2497, enW:2564, enGec:2581,
//!   sfSonra:2593, enMuhasebe:2597, launchFor:2663, ucusT:2736, ucusSure:2743, bankSure:2749,
//!   bankFor:2757, sutHedef:2790, options:2808, sutKapali:2256, bosOy/SPT:3503, Vend:3598, runOn:3602,
//!   dokEk/teK/dokKar:3618, tercihEk:3656, V:3666, chKayip/depoOn:3673, laneOk:3680, respond:3714,
//!   shotQ:3932, zoneR:3945, pressAt:3949, freeAt:3959, recvCh:3962, sarjTut:3982, chargeRate:3990,
//!   surSure:3998, keepP (+ keepP.son):4429, coreSide:4561, ikKafa:4603, duelP:4622, coreAng:4635,
//!   kenarAkil:155, Z5:154.
//! - Yerleşim (`shape.js`, AlanShape): bekci, slotBase, gside, gecTab, gecOf, gec, vSp, S.prHat, S.rcvBuf.
//! - Maç (`match.js`, AlanMatch): tl, ivme, spd, sprK, IK, createMatch/kopya/step (holdWorld),
//!   ALAN_KOSU_T:2262, ALAN_KOSU_M:2268, ALAN_SPR_FAZ.
//! - Çekirdek (`core.js`): ALAN_OZ:30, ALAN_SPR:101, ALAN_SPRE:102, ALAN_HZ:104, ALAN_EN2_SPRE:192,
//!   ALAN_TEPKI:1278, makeBall/predict/stepAll (top.rs, temas.rs hazır).
//!
//! "GEÇİCİ KOPYA" yazanlar: JS'in birkaç satırlık saf işlevleri; kafa 2'nin sınamaları koşabilsin diye
//! burada aynen yazıldı. Birleştirmede kafa1'in / çekirdeğin gerçeğiyle değiştirilmeli (anlam aynı).
//! Geri kalanlar `todo!()`.
#![allow(non_snake_case, unused_variables, dead_code)]

use super::kafa2_baglam::*;
use crate::mat::{atan2, exp, hypot, jmax, jmin};
use crate::model::{JODet, JOLaunch, JMTacE, Oyuncu, Secenek};
use crate::saha::{CY as GCY, KR, W as GW};
use crate::yetenek::MEK_AD;

/// Kafa 1'in durumu (DP_K, GE, keepP.son, SHT…). Birleştirmede kafa1'in yapısı.
#[derive(Default)]
pub struct Kafa1Durum {
    /// JS `keepP.son` (keepP'nin yan çıktısı: topun güvenle taşınabildiği yol)
    pub keepP_son: f64,
}

// ---------------------------------------------------------------- çekirdek / yetenek (GEÇİCİ KOPYA)

/// JS ALAN_Y(p, k) (yetenek.js:322): mekanik adıyla.
pub fn ALAN_Y(p: &Oyuncu, k: &str) -> f64 {
    match MEK_AD.iter().position(|a| *a == k) {
        Some(i) => p.yet.y(i),
        None => panic!("ALAN_Y: tabloda olmayan mekanik {k}"),
    }
}
/// JS ALAN_OZ(p, k) (core.js:30)
pub fn ALAN_OZ(p: &Oyuncu, k: &str) -> f64 {
    p.yet.ozk(k)
}
/// JS ALAN_HZ (core.js:104): Enerji 3.0 açık → ham Hız.
pub fn ALAN_HZ(p: &Oyuncu) -> f64 {
    p.yet.hiz
}
/// JS ALAN_SPR (core.js:101)
pub fn ALAN_SPR(p: &Oyuncu) -> f64 {
    crate::yetenek::spr_h(p.yet.hiz)
}
/// JS ALAN_SPRE(p) (core.js:102 → ALAN_EN2_SPRE:192, L tanımsız → refL 20). GEÇİCİ KOPYA.
pub fn ALAN_SPRE(p: &Oyuncu) -> f64 {
    let en = match p.en {
        None => return ALAN_SPR(p),
        Some(e) => e,
    };
    let sp = ALAN_SPR(p);
    if !(sp > 1.0) {
        return veya(sp, 1.0);
    }
    let en_s = 0.7 + 0.03 * ALAN_Y(p, "barBoy");
    let sb = jmax(0.5, jmin(1.6, 1.0 + 5.0 * (en_s - 1.0) / 3.0));
    let ts = jmax(0.0, en * sb / (1.0 / 120.0));
    if ts <= 0.0 {
        return 1.0;
    }
    let b = 0.17 + 0.006 * ALAN_HZ(p);
    let lr = 20.0;
    let reach = b * sp * ts;
    if lr <= reach {
        return sp;
    }
    let t = ts + (lr - reach) / b;
    lr / (b * t)
}
/// JS ALAN_TEPKI(p, k) (core.js:1278; enerji katı en dışta)
pub fn ALAN_TEPKI(p: &Oyuncu, k: &str) -> f64 {
    use crate::oyuncu::{tepki, Tepki, TepkiS};
    let t = match k {
        "alan" => Tepki::Alan,
        "algi" => Tepki::Algi,
        "karar" => Tepki::Karar,
        "yeniTop" => Tepki::YeniTop,
        "kovala" => Tepki::Kovala,
        "bosTop" => Tepki::BosTop,
        "uzanma" => Tepki::Uzanma,
        _ => todo!("ALAN_TEPKI {k}"),
    };
    tepki(p, t, &TepkiS::default())
}
/// JS ALAN_KOSU_T(p, d, v1) (match.js:2262): sprint evresiyle koşu süresi.
pub fn ALAN_KOSU_T(k: &Kafa, p: &Oyuncu, d: f64, v1: f64) -> f64 {
    todo!("match.js ALAN_KOSU_T")
}
/// JS ALAN_KOSU_M(p, t, v1) (match.js:2268): t tikte alınan yol.
pub fn ALAN_KOSU_M(k: &Kafa, p: &Oyuncu, t: f64, v1: f64) -> f64 {
    todo!("match.js ALAN_KOSU_M")
}

// ---------------------------------------------------------------- kafa 1: saf yardımcılar (GEÇİCİ KOPYA)

/// JS threat (decide.js:136)
pub fn threat(k: &Kafa, team: f64, x: f64, y: f64) -> f64 {
    let gx = if team == 0.0 { GW } else { 0.0 };
    let d = hypot(x - gx, y - GCY);
    exp(-d / k.D.threatK) * (1.0 - k.D.sideK * (y - GCY).abs() / GCY)
}
/// JS pasMax (decide.js:135)
pub fn pasMax(k: &Kafa) -> f64 {
    k.D.passMaxF
}
/// JS kuyuD (decide.js:131)
pub fn kuyuD(x: f64, y: f64) -> f64 {
    let r = KR + 0.6;
    let dy = y - GCY;
    if dy > r || dy < -r || (x > r && x < GW - r) {
        -1.0
    } else {
        r - jmin(hypot(x, dy), hypot(x - GW, dy))
    }
}
/// JS KY() = C().KUYU() (core.js:1124): hep açık.
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
/// JS chKayip (decide.js:3673)
pub fn chKayip(ch: f64) -> f64 {
    if depoOn() && ch >= 0.0 {
        jmax(0.05, ch)
    } else {
        0.3
    }
}
/// JS alP (decide.js:1375): `d.P - d.Ps` (sekme payı düşülmüş tutma).
pub fn alP(d: Option<&JODet>) -> Option<f64> {
    match d {
        None => None,
        Some(d) => match d.P {
            Some(p) => match d.Ps {
                Some(ps) => Some(jmax(0.0, p - ps)),
                None => Some(p),
            },
            None => None,
        },
    }
}
/// JS zoneR (decide.js:3945)
pub fn zoneR(k: &Kafa, team: f64, x: f64, y: f64) -> f64 {
    let gx = if team == 0.0 { GW } else { 0.0 };
    let d = hypot(gx - x, GCY - y);
    k.D.zoneMin + (1.0 - k.D.zoneMin) * jmin(1.0, jmax(0.0, (d - 8.0) / k.D.zoneD))
}
/// JS pressAt (decide.js:3949)
pub fn pressAt(k: &Kafa, src: &[Op], team: f64, x: f64, y: f64) -> f64 {
    let z = zoneR(k, team, x, y);
    let mut s = 0.0;
    for p in src {
        if p.team == team || p.noTouch {
            continue;
        }
        let d = hypot(p.x - x, p.y - y) / z;
        let r = 0.55 * veya(p.R, 8.0);
        s += veya(p.D, 1.0) * exp(-d * d / (r * r));
    }
    s
}
/// JS freeAt (decide.js:3959)
pub fn freeAt(k: &Kafa, src: &[Op], team: f64, x: f64, y: f64) -> f64 {
    exp(1.6 * -pressAt(k, src, team, x, y))
}
/// JS tercihEk (decide.js:3656)
pub fn tercihEk(v: f64, d: f64) -> f64 {
    if d <= 0.0 {
        v + d
    } else {
        v + d * jmax(0.0, 1.0 - v)
    }
}
/// JS dokEk (decide.js:3618)
pub fn dokEk(o: &mut Secenek, k: &str, d: f64) {
    if let Some(t) = o.dok.as_mut() {
        if d != 0.0 && d.is_finite() {
            let v = t.al(k).map(|x| veya(x, 0.0)).unwrap_or(0.0) + d;
            t.koy(k, v);
        }
    }
}
/// JS teK (decide.js:3621)
pub fn teK(o: &mut Secenek, k: &str, d: f64) {
    let v0 = o.v.unwrap_or(f64::NAN);
    let v = tercihEk(v0, d);
    o.v = Some(v);
    dokEk(o, k, v - v0);
}
/// JS dokKar (decide.js:3625)
pub fn dokKar(o: &mut Secenek, w: f64, P: &[(&str, f64)]) {
    if let Some(t) = o.dok.as_mut() {
        for x in t.k.iter_mut() {
            x.1 *= 1.0 - w;
        }
        for (k, v) in P {
            if *v != 0.0 && *v == *v {
                let a = t.al(k).map(|x| veya(x, 0.0)).unwrap_or(0.0) + w * v;
                t.koy(k, a);
            }
        }
    }
}
/// JS coreSide (decide.js:4561): (ux, uy, off)
pub fn coreSide(h: &Oyuncu) -> (f64, f64, f64) {
    let sp = hypot(veya(h.vx, 0.0), veya(h.vy, 0.0));
    let (ux, uy);
    if let Some(ca) = h.ca {
        ux = crate::mat::cos(ca);
        uy = crate::mat::sin(ca);
    } else if sp > 0.04 {
        ux = h.vx / sp;
        uy = h.vy / sp;
    } else if hypot(h.tx - h.x, h.ty - h.y) > 0.5 {
        let l = hypot(h.tx - h.x, h.ty - h.y);
        ux = (h.tx - h.x) / l;
        uy = (h.ty - h.y) / l;
    } else {
        let gx = if h.team == 0.0 { GW } else { 0.0 };
        let l = veya(hypot(gx - h.x, GCY - h.y), 1.0);
        ux = (gx - h.x) / l;
        uy = (GCY - h.y) / l;
    }
    (ux, uy, 0.6 + sp * 3.0)
}
/// JS coreAng (decide.js:4635)
pub fn coreAng(h: &Oyuncu) -> f64 {
    let (ux, uy, _) = coreSide(h);
    atan2(uy, ux)
}
/// JS SPT (decide.js:3503): boşluk noktaları (r, açı)
pub fn SPT() -> Vec<(f64, f64)> {
    let mut v = Vec::new();
    for r in [6.0, 10.0, 14.0, 18.0, 22.0, 26.0, 30.0] {
        for k in -3..=3 {
            v.push((r, 0.37 * k as f64));
        }
    }
    v
}
/// JS ALAN_YET.zar / kademe ile kenarAkil (decide.js:155)
pub fn kenarAkil(k: &Kafa, h: &Oyuncu) -> bool {
    crate::yetenek::zar(h.id, (k.tick0() / 20.0).floor(), 13.0) < crate::yetenek::kademe(ALAN_Y(h, "kenarPasDusun"), k.D.bankOk)
}
/// JS holdF (decide.js:1317)
pub fn holdF() -> bool {
    true
}
/// JS SPK (decide.js:214): sprint evresi ayrı (ALAN_SPR_FAZ.ayir).
pub fn SPK() -> bool {
    todo!("match.js ALAN_SPR_FAZ.ayir")
}
/// JS ikKafa (decide.js:4603): AlanMatch.IK var mı
pub fn ikKafa() -> bool {
    todo!("match.js AlanMatch.IK")
}

// ---------------------------------------------------------------- kafa 1: tartı (todo)

pub fn V(k: &mut Kafa, src: &[Op], team: f64, x: f64, y: f64, ch: f64) -> f64 {
    todo!("kafa1 V")
}
pub fn Vend(k: &mut Kafa, src: &[Op], team: f64, x: f64, y: f64, ch: f64) -> f64 {
    todo!("kafa1 Vend")
}
pub fn runOn(k: &mut Kafa, w: &[Op], team: f64, h: &Oyuncu, at: Nokta) -> f64 {
    todo!("kafa1 runOn")
}
/// JS laneOk(src, team, A, B, arr, lag)
pub fn laneOk(k: &mut Kafa, src: &[Op], team: f64, A: Nokta, B: Nokta, arr: Option<f64>, lag: Option<f64>) -> f64 {
    todo!("kafa1 laneOk")
}
/// JS keepP(h, src, dir, T); yan çıktı `k.k1.keepP_son`
pub fn keepP(k: &mut Kafa, h: &Oyuncu, src: &[Op], dir: Option<Nokta>, T: f64) -> f64 {
    todo!("kafa1 keepP")
}
pub fn recvCh(k: &mut Kafa, src: &[Op], team: f64, x: f64, y: f64, ch: f64, q: &Oyuncu) -> f64 {
    todo!("kafa1 recvCh")
}
pub fn shotQ(k: &mut Kafa, src: &[Op], team: f64, x: f64, y: f64, ch: f64) -> f64 {
    todo!("kafa1 shotQ")
}
pub fn sutKapali(k: &mut Kafa, src: &[Op], team: f64, x: f64, y: f64, ch: f64) -> f64 {
    todo!("kafa1 sutKapali")
}
/// JS respond(src, team, at, T, kind): sonuç dünyası (değişmeyen oyuncular aynı `Arc`).
pub fn respond(k: &mut Kafa, src: &[Op], team: f64, at: Nokta, T: f64, kind: &str) -> Dunya {
    todo!("kafa1 respond")
}
pub fn oppAt(k: &mut Kafa, src: &[Op], team: f64, x: f64, y: f64) -> f64 {
    todo!("kafa1 oppAt")
}
/// JS ucusT(L, arrive, al)
pub fn ucusT(k: &mut Kafa, L: f64, arrive: f64, al: Option<f64>) -> f64 {
    todo!("kafa1 ucusT")
}
pub fn ucusSure(k: &mut Kafa, h: &Oyuncu, at: Nokta, lv: &JOLaunch, src: &[Op], ch: f64, savT: Option<f64>) -> f64 {
    todo!("kafa1 ucusSure")
}
pub fn bankSure(k: &mut Kafa, h: &Oyuncu, B: Nokta, T: Nokta, lv: &JOLaunch, src: &[Op], ch: f64, savT: Option<f64>) -> f64 {
    todo!("kafa1 bankSure")
}
pub fn launchFor(k: &mut Kafa, h: &Oyuncu, to: Nokta, arrive: f64, src: &[Op], ch: f64) -> JOLaunch {
    todo!("kafa1 launchFor")
}
pub fn bankFor(k: &mut Kafa, h: &Oyuncu, B: Nokta, q: Nokta, arrive: f64, src: &[Op], ch: f64) -> JOLaunch {
    todo!("kafa1 bankFor")
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
/// JS imagine(h, src, b0, rnd, trials, q, to) (= ucusFormul; kuyruk yok)
pub fn imagine(k: &mut Kafa, h: &Oyuncu, src: &[Op], b0: Top0, rnd: &mut Zar, trials: f64, q: Option<&Oyuncu>, to: Option<Nokta>) -> JODet {
    todo!("kafa1 imagine")
}
/// JS kenarIm (decide.js:2315): `D._kenar` açıkken imagine
pub fn kenarIm(k: &mut Kafa, h: &Oyuncu, src: &[Op], b0: Top0, rnd: &mut Zar, trials: f64, q: Option<&Oyuncu>, to: Option<Nokta>) -> JODet {
    todo!("kafa1 kenarIm")
}
/// JS savurPlan sonucu
#[derive(Clone, Copy, Debug, Default)]
pub struct SavPlan {
    pub k: f64,
    pub T: f64,
    pub wRel: f64,
    pub uSh: f64,
    pub caR: Option<f64>,
    pub A: f64,
    pub s: f64,
}
pub fn savurPlan(k: &mut Kafa, h: &Oyuncu, vx: f64, vy: f64, ca0: f64, w0: f64, sYon: f64, src: &[Op], itme: bool, E: f64) -> Option<SavPlan> {
    todo!("kafa1 savurPlan")
}
pub fn caOf(k: &Kafa, h: &Oyuncu) -> f64 {
    todo!("kafa1 caOf")
}
/// JS vurE(h, ch, gon)
pub fn vurE(k: &Kafa, h: &Oyuncu, ch: Option<f64>, gon: bool) -> f64 {
    todo!("kafa1 vurE")
}
pub fn sutGuc(h: &Oyuncu) -> f64 {
    todo!("kafa1 sutGuc")
}
pub fn sutV(k: &Kafa, h: &Oyuncu) -> f64 {
    todo!("kafa1 sutV")
}
pub fn gonEk(k: &mut Kafa, h: &Oyuncu, src: &[Op], shot: &str) -> f64 {
    todo!("kafa1 gonEk")
}
/// JS sutHedef(h, src, gx): nişan y'leri. `h` sahte olabilir (sutYol: id −9).
pub fn sutHedef(k: &mut Kafa, h: &Oyuncu, src: &[Op], gx: f64) -> Vec<f64> {
    todo!("kafa1 sutHedef")
}
/// JS surSure(h, L, dir, spr)
pub fn surSure(k: &Kafa, h: &Oyuncu, L: f64, dir: Option<Nokta>, spr: Option<f64>) -> f64 {
    todo!("kafa1 surSure")
}
/// JS sprKop(h): sprint niyetli kopya (`_sf` patlama, sprint)
pub fn sprKop(k: &Kafa, h: &Oyuncu) -> Oyuncu {
    todo!("kafa1 sprKop")
}
#[derive(Clone, Copy, Debug)]
pub struct SarjT {
    pub c2: f64,
    pub bed: f64,
}
pub fn sarjTut(k: &mut Kafa, h: &Oyuncu, src: &[Op], ch: f64, still: f64, T: f64) -> Option<SarjT> {
    todo!("kafa1 sarjTut")
}
pub fn chargeRate(k: &mut Kafa, h: &Oyuncu, src: &[Op], still: f64, ch: f64) -> f64 {
    todo!("kafa1 chargeRate")
}
pub fn topK(k: &Kafa, h: &Oyuncu) -> f64 {
    todo!("kafa1 topK")
}
pub fn donS(k: &Kafa, p: &Oyuncu, ux: f64, uy: f64) -> f64 {
    todo!("kafa1 donS")
}
pub fn duelP(k: &mut Kafa, q: &Oyuncu, h: &Oyuncu, src: &[Op]) -> f64 {
    todo!("kafa1 duelP")
}
/// JS enW(w, o): enerji muhasebesinin dünyası (o.en.per'e göre düşülmüş bar)
pub fn enW(k: &Kafa, w: &std::sync::Arc<Dunya>, o: &Secenek) -> std::sync::Arc<Dunya> {
    todo!("kafa1 enW")
}
/// JS enGec(q, w, f): `q.en`'i geçici olarak dünyadaki karşılığına çeker; burada çekilecek değer döner
/// (None: değişmez). Çağıran q'nun kopyasıyla f'i çağırır.
pub fn enGec(k: &Kafa, q: &Oyuncu, w: &[Op]) -> Option<f64> {
    todo!("kafa1 enGec")
}
pub fn enMuhasebe(k: &mut Kafa, h: &Oyuncu, src: &[Op], w: &[Op], T: f64, bit: Option<Nokta>, o: &Secenek) -> crate::model::JOEn {
    todo!("kafa1 enMuhasebe")
}
/// Kafa 1'in nokta tipi (kafa1'de `Nk`)
pub type Nk = Nokta;
/// JS options(h, src, ch) (kafa1: `impl Kafa { pub fn options(&self, h, src, ch) -> Secenekler }`).
pub fn options(k: &Kafa, h: &Oyuncu, src: &[Op], ch: f64) -> Secenekler {
    todo!("kafa1 options")
}
/// options'ın döndürdüğü dizi ve diziye takılı ek listeler (JS `opts.tekH`, `opts.tekS`, `opts.tekC`).
/// JS'te tekS / tekC yoksa (`undefined`) yedek adaylar kurulur: burada None.
#[derive(Default)]
pub struct Secenekler {
    pub out: Vec<Secenek>,
    /// alıcı (sıra) → tek dokunuş hedefleri
    pub tekH: Vec<(Pid, Vec<Nk>)>,
    pub tekS: Option<Vec<Nk>>,
    pub tekC: Option<Vec<Nk>>,
}
pub fn fakeSecenek(k: &mut Kafa, hi: Pid, src: &[Op], ch: f64, rnd: &mut Zar, tac: Option<&Tac>, opts: &mut Vec<Secenek>, kon: bool) {
    todo!("kafa1 fakeSecenek")
}
/// JS bosOy(src, team, bx, by)
pub struct BosOy {
    pub n: usize,
    pub X: Vec<f64>,
    pub Y: Vec<f64>,
    pub V: Vec<f64>,
    pub A: Vec<f64>,
    pub T: Vec<i8>,
}
pub fn bosOy(k: &Kafa, src: &[Op], team: f64, bx: f64, by: f64) -> BosOy {
    todo!("kafa1 bosOy")
}
pub fn applyError(k: &mut Kafa, h: &Oyuncu, v: &JOLaunch, rnd: &mut Zar, gon: bool, src: &[Op], tur: Option<&str>, kenar: bool) -> (f64, f64) {
    todo!("kafa1 applyError")
}
/// JS chasePlan(pts, ps, t0): oyuncu (sıra) → (x, y, rc)
pub fn chasePlan(k: &mut Kafa, pts: &[crate::top::YolNokta], ps: &[Op], t0: f64) -> Vec<(Pid, f64, f64, f64)> {
    todo!("kafa1 chasePlan")
}
pub fn recvPoint(k: &mut Kafa, pts: &[crate::top::YolNokta], p: &Oyuncu) -> Option<Nokta> {
    todo!("kafa1 recvPoint")
}
/// JS secim(h, r, src) (decideOT'nin seçimi)
pub fn secim(k: &mut Kafa, h: &Oyuncu, opts: Vec<Secenek>, T: f64, src: &[Op]) -> Option<Secenek> {
    todo!("kafa1 secim")
}

// ---------------------------------------------------------------- yerleşim (AlanShape)

/// Yerleşimin baktığı maç görünümü (JS fm: { tac, ps, holder })
pub struct Fm<'a> {
    pub tac: &'a [JMTacE],
    pub ps: &'a [Op],
    pub holder: Option<&'a Oyuncu>,
}
/// JS AlanShape.bekci(p, B, m)
pub fn AS_bekci(k: &Kafa, p: &Oyuncu, B: Nokta, fm: Option<&Fm>) -> Nokta {
    todo!("shape bekci")
}
/// JS AlanShape.slotBase(m, p, B, hucum)
pub fn AS_slotBase(k: &Kafa, fm: &Fm, p: &Oyuncu, B: Nokta, hucum: bool) -> Nokta {
    todo!("shape slotBase")
}
/// JS AlanShape.gside(E, team, d)
pub fn AS_gside(E: Nokta, team: f64, d: f64) -> Nokta {
    todo!("shape gside")
}
/// JS AlanShape.gecTab(p) (dönüş/geçiş tablosu)
#[derive(Clone, Debug, Default)]
pub struct GecTab;
pub fn AS_gecTab(p: &Oyuncu) -> GecTab {
    todo!("shape gecTab")
}
pub fn AS_gecOf(G: &GecTab, x: f64, y: f64, d: f64) -> f64 {
    todo!("shape gecOf")
}
pub fn AS_gec(p: &Oyuncu, x: f64, y: f64, d: f64) -> f64 {
    todo!("shape gec")
}
pub fn AS_vSp(p: &Oyuncu) -> f64 {
    todo!("shape vSp")
}
/// JS AlanShape.S.prHat[blok]
pub fn AS_S_prHat(blok: &str) -> Option<f64> {
    todo!("shape S.prHat")
}
/// JS AlanShape.S.rcvBuf
pub fn AS_S_rcvBuf() -> f64 {
    todo!("shape S.rcvBuf")
}

// ---------------------------------------------------------------- maç (AlanMatch)

/// JS AlanMatch.tl(p, "sav", "adam") (oyuncunun takım talimatı)
pub fn AM_tl(p: &Oyuncu, a: &str, b: &str) -> f64 {
    todo!("match tl")
}
/// JS AlanMatch.ivme(q, wx, wy, vmax): hızı isteğe doğru çevirir (q.vx, q.vy yazar)
pub fn AM_ivme(k: &Kafa, q: &mut Oyuncu, wx: f64, wy: f64, vmax: f64) {
    todo!("match ivme")
}
pub fn AM_spd(p: &Oyuncu) -> f64 {
    todo!("match spd")
}
pub fn AM_sprK(p: &Oyuncu) -> f64 {
    todo!("match sprK")
}
/// holdWorld'ün hayali maçı (JS createMatch + kopya + kopyaDurum + step). Yalnız eski kafa (ALAN_PRENSIP kapalı).
pub struct HayaliMac {
    pub ps: Dunya,
    pub holder: Option<Pid>,
    pub ch: f64,
}
pub fn AM_hayali(k: &mut Kafa, tohum: f64, src: &[Op], hi: Pid, ch: f64, tac: Option<&Tac>) -> HayaliMac {
    todo!("match createMatch/kopya/kopyaDurum")
}
/// JS M.step(m) (hayali maçta bir tik; m.decT = 99)
pub fn AM_step(k: &mut Kafa, m: &mut HayaliMac) {
    todo!("match step")
}

/// JS vurTaban(p) (core.js:410): vuruşun taban enerjisi. GEÇİCİ KOPYA.
pub fn vurTaban(p: &Oyuncu) -> f64 {
    crate::saha::P.tek_taban * jmax(0.5, jmin(1.5, 1.0 + 0.05 * (ALAN_Y(p, "vurusTaban") - 10.0)))
}
