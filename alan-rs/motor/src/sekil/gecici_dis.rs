//! Yerleşimin çağırdığı, henüz taşınmamış yabancı işlevler (AlanDecide, AlanMatch, core.js'in enerji parçası).
//! Birleştirmede gerçeklerine bağlanacak. Adlar JS adlarıyla aynı; tipler yerleşimin gördüğü kadarıyla.
//! Dünya (`w: &[Varlik]`) yerleşimin varlık listesidir; `oy` bedenlerin oyuncuları (m.ps).
//!
//! JS kaynakları (kaynak/*.js satırları):
//! - threat            decide.js:136   `threat(team, x, y)`: (x, y)'nin `team` için tehlikesi
//! - frenT             decide.js:1604  `frenT(p, x, y, R, T, v)`: koşucunun (x,y)'de durabilmesi için gereken süre
//! - freeAt            decide.js:3959  `freeAt(src, team, x, y)`: noktanın boşluğu
//! - coreSide          decide.js:4561  `coreSide(h)`: taşıyıcının Çekirdeği tuttuğu yön {ux, uy}
//! - govdeV            decide.js:561   `govdeV(h)`: taşıyıcının vuruş hızı
//! - savurPlan         decide.js:565   `savurPlan(h, V, ca0, w0, sYon, src, itme, E)`: savurmanın planı (yalnız `.T` okunur)
//! - topK              decide.js:244   `topK(h)`: topla koşu katı
//! - duelP             decide.js:4622  `duelP(q, h, src)`: q'nun h'den topu alma ihtimali
//! - girThr            decide.js:4701  `girThr(q, h, src)`: girme eşiği
//! - ucusT             decide.js:2736  `ucusT(L, arrive, al)`: pasın `al` yola varış süresi
//! - laneOk            decide.js:3680  `laneOk(src, team, A, B, arr, lag)`: pas yolunun açıklığı
//! - bekYol            decide.js:6876  `bekYol(w, team, A, B, lag)`: Bekçi'nin pas yoluna yetişmemesi
//! - yerDeger          decide.js:6773  `yerDeger(w, team, x, y, ch)`: yerin hücum değeri
//! - recvCh            decide.js:3962  `recvCh(src, team, x, y, ch, q)`: alıştan sonra topun şarjı
//! - keepP             decide.js:4429  `keepP(h, src, dir, T)`: T tik topu tutma ihtimali
//! - runOn             decide.js:3602  `runOn(w, team, h, at)`: topla koşunun değeri
//! - space             decide.js:3526  `space(src, team, bx, by)`; front decide.js:3567; dagin decide.js:3659
//! - kayipYer          decide.js:8801  `kayipYer(src, team, ch, h, tac)`: kaybın bedeli (x, y) → değer
//! - PNC_GORUS, DU     decide.js:6875, 20–30 (sabitler; değerleriyle)
//! - tl                match.js:939    `tl(p, ev, k)`: oyuncunun taktik rolü (huc/sav · adam/pres/kosu/…), yoksa null
//! - kpAkt             match.js:107    `kpAkt(m, t)`: kontrpres etkinse presçilerin id'leri
//! - tacEff            match.js:147    `tacEff(m, t)`: takımın etkin taktiği (`_kp`: kontrpres)
//! - macEff_macK       match.js:183    `macEff(m, t, {}).macK`: skor/süre katı
//! - kuyuSafe          match.js:2579   `kuyuSafe(x, y)`: Kuyu kapalıyken güvenli yer (Kuyu açık: hep null)
//! - ALAN_KOSU_PR      match.js:2219   `kosuPr(p)`: sprint profili {Ts, k} (maçın sprint evresinden)
//! - slowT             match.js:3760   `p.slowT || 0` (modelde alan yok)
//! - ALAN_EN3_DOL      core.js:159     barın dolumu
//! - ALAN_EN3_KUL      core.js:169     barın kullanım değeri
#![allow(non_snake_case, unused_variables, non_upper_case_globals)]

use crate::model::{JMTacE, Oyuncu};

use super::varlik::{KayipF, Mc, Varlik};

// ---- AlanDecide ----
use crate::kafa::kafa1_js::{nk, Op};
use crate::mac::kafa_dis::kafa_ile;
/// Yerleşimin dünyasını kafanın dünyasına çevir (varlık: konum/hız kendinde, beden oyuncudan).
fn ow(w: &[Varlik], oy: &[Oyuncu]) -> Vec<Op> {
    w.iter()
        .map(|v| {
            let mut o = v.govde(oy).clone();
            o.x = v.x;
            o.y = v.y;
            o.vx = v.vx;
            o.vy = v.vy;
            std::sync::Arc::new(o)
        })
        .collect()
}
fn ops(ps: &[Oyuncu]) -> Vec<Op> {
    crate::mac::kafa_dis::op_dunya(ps)
}

pub fn threat(team: u8, x: f64, y: f64) -> f64 {
    // saf: kafa bağlamı aranmaz (kesişme döngüsünde milyonlarca kez çağrılır)
    crate::kafa::kafa1_tehlike::threat_d(crate::mac::gecici_dis::kafa_d(), team as f64, x, y)
}
pub fn frenT(p: &Oyuncu, x: f64, y: f64, R: f64, T: f64, v: f64) -> f64 {
    kafa_ile(|k| k.frenT(p, x, y, R, T, v))
}
pub fn freeAt(w: &[Varlik], oy: &[Oyuncu], team: f64, x: f64, y: f64) -> f64 {
    kafa_ile(|k| k.freeAt(&ow(w, oy), team, x, y))
}
pub fn coreSide(h: &Oyuncu) -> (f64, f64) {
    let c = crate::kafa::kafa1_ikili::coreSide_p(h);
    (c.ux, c.uy)
}
pub fn govdeV(h: &Oyuncu) -> f64 {
    kafa_ile(|k| k.govdeV(h))
}
/// Taşıyıcının şimdi çekebileceği en sert şutun hızı (tam şarj): gövde savurma hızı × vuruş enerjisi (kafanın vurE'si).
pub fn sutHiz(h: &Oyuncu) -> Option<f64> {
    if !crate::mac::kafa_dis::kafa_var() {
        return None;
    }
    Some(kafa_ile(|k| k.govdeV(h) * k.vurE(h, Some(1.0), true)))
}
/// JS `savurPlan(h, {vx, vy}, ca0, w0, sYon, null, itme, E)` → `pl ? pl.T : None`.
pub fn savurPlan(h: &Oyuncu, vx: f64, vy: f64, ca0: f64, w0: f64, sYon: f64, itme: bool, E: f64) -> Option<f64> {
    kafa_ile(|k| k.savurPlan(h, nk(vx, vy), ca0, w0, sYon, None, itme, E).map(|p| p.T))
}
pub fn topK(h: &Oyuncu) -> f64 {
    crate::kafa::kafa1_ikili::topK_p(h, crate::mac::kafa_dis::mac_dokunus())
}
/// JS'te oyuncular dünyadaki nesnelerin kendisidir (kafa `o !== q` ile karşılaştırır): kopya dünyada kimliğiyle bul.
fn kim<'a>(src: &'a [Op], p: &'a Oyuncu) -> &'a Oyuncu {
    src.iter().find(|q| q.id == p.id && q.team == p.team).map(|q| &**q).unwrap_or(p)
}
pub fn duelP(q: &Oyuncu, h: &Oyuncu, ps: &[Oyuncu]) -> f64 {
    let src = ops(ps);
    kafa_ile(|k| k.duelP(kim(&src, q), kim(&src, h), &src, None))
}
pub fn girThr(q: &Oyuncu, h: &Oyuncu, ps: &[Oyuncu]) -> f64 {
    let src = ops(ps);
    kafa_ile(|k| k.girThr(kim(&src, q), kim(&src, h), &src))
}
pub fn ucusT(L: f64, arrive: f64, al: Option<f64>) -> f64 {
    kafa_ile(|k| k.ucusT(L, arrive, al))
}
/// `arr`: None = JS'te verilmedi (undefined).
pub fn laneOk(w: &[Varlik], oy: &[Oyuncu], team: u8, A: (f64, f64), B: (f64, f64), arr: f64, lag: Option<f64>) -> f64 {
    kafa_ile(|k| k.laneOk(&ow(w, oy), team as f64, nk(A.0, A.1), nk(B.0, B.1), Some(arr), lag))
}
/// JS laneOk(src, team, A, B) (arr ve lag verilmeden).
pub fn laneOk0(w: &[Varlik], oy: &[Oyuncu], team: u8, A: (f64, f64), B: (f64, f64)) -> f64 {
    kafa_ile(|k| k.laneOk(&ow(w, oy), team as f64, nk(A.0, A.1), nk(B.0, B.1), None, None))
}
pub fn bekYol(w: &[Varlik], oy: &[Oyuncu], team: u8, A: (f64, f64), B: (f64, f64), lag: f64) -> f64 {
    kafa_ile(|k| crate::kafa::kafa2_tarti::bekYol(k, &ow(w, oy), team as f64, nk(A.0, A.1), nk(B.0, B.1), Some(lag)))
}
pub fn yerDeger(w: &[Varlik], oy: &[Oyuncu], team: u8, x: f64, y: f64, ch: f64) -> f64 {
    kafa_ile(|k| crate::kafa::kafa2_zemin::yerDeger(k, &ow(w, oy), team as f64, x, y, ch))
}
pub fn recvCh(w: &[Varlik], oy: &[Oyuncu], team: u8, x: f64, y: f64, ch: f64, q: Option<&Oyuncu>) -> f64 {
    // kafanın recvCh'i dünyayı okumaz: köprü kurulmaz
    let _ = (w, oy);
    kafa_ile(|k| match q {
        Some(q) => k.recvCh(&[], team as f64, x, y, ch, q),
        None => k.recvCh(&[], team as f64, x, y, ch, &Oyuncu::bos()),
    })
}
pub fn keepP(h: &Oyuncu, w: &[Varlik], oy: &[Oyuncu], dir: Option<(f64, f64)>, T: f64) -> f64 {
    kafa_ile(|k| k.keepP(h, &ow(w, oy), dir.map(|d| nk(d.0, d.1)), T))
}
pub fn runOn(w: &[Varlik], oy: &[Oyuncu], team: u8, h: &Oyuncu, at: (f64, f64)) -> f64 {
    kafa_ile(|k| k.runOn(&ow(w, oy), team as f64, h, nk(at.0, at.1)))
}
pub fn space(w: &[Varlik], oy: &[Oyuncu], team: u8, x: f64, y: f64) -> f64 {
    kafa_ile(|k| k.space(&ow(w, oy), team as f64, x, y))
}
pub fn front(w: &[Varlik], oy: &[Oyuncu], team: u8, x: f64, y: f64) -> f64 {
    kafa_ile(|k| k.front(&ow(w, oy), team as f64, x, y))
}
pub fn dagin(w: &[Varlik], oy: &[Oyuncu], team: u8, x: f64, vv: f64, y: f64) -> f64 {
    kafa_ile(|k| k.dagin(&ow(w, oy), team as f64, x, vv, Some(y)))
}
pub fn kayipYer(ps: &[Oyuncu], team: u8, ch: f64, h: &Oyuncu, tac: Option<&crate::mac::Tac>) -> KayipF {
    let src = std::sync::Arc::new(ops(ps));
    let tac = tac.map(crate::mac::gecici_dis::kafa_tac);
    let f = kafa_ile(|k| crate::kafa::kafa2_karar::kayipYer(k, &src, team as f64, ch, h, tac.as_ref()));
    let f = std::sync::Mutex::new(f);
    std::sync::Arc::new(move |x: f64, y: f64| kafa_ile(|k| f.lock().unwrap().deger(k, x, y)))
}
/// decide.js:6875 `PNC_GORUS`.
pub const PNC_GORUS: f64 = 48.0;
/// decide.js `DU.fakeT`, `DU.oyalaR`, `DU.engage`.
pub const DU_fakeT: f64 = 9.0;
pub const DU_oyalaR: f64 = 26.0;
pub const DU_engage: f64 = 2.4;

// ---- AlanMatch ----

/// JS tl(p, ev, k): `null` → None.
pub fn tl(p: &Oyuncu, ev: &str, k: &str) -> Option<f64> {
    crate::mac::kurulum::tl(p, ev, k)
}
/// JS kpAkt(m, t): kontrpresçilerin id'leri (m.kp'yi bitirebilir).
pub fn kpAkt(c: &mut Mc, t: u8) -> Option<Vec<f64>> {
    c.mac_ile(|m| crate::mac::tac::kpAkt(m, t as f64))
}
/// Etkin taktik (JS tacEff dönüşü): taktik + kontrpres (`_kp`) işareti. (`plan`, `enPresDalga` hiç kurulmaz.)
#[derive(Clone, Debug, Default)]
pub struct TacE {
    pub t: crate::mac::Tac,
    pub kp: bool,
}
pub fn tacEff(c: &mut Mc, t: u8) -> TacE {
    let t = c.mac_ile(|m| crate::mac::tac::tacEff(m, t as f64));
    let kp = t._kp.is_some();
    TacE { t, kp }
}
/// JS `macEff(m, t, {}).macK`.
pub fn macEff_macK(c: &mut Mc, t: u8) -> Option<f64> {
    c.mac_ile(|m| crate::mac::tac::macEff(m, t as f64, Default::default()).macK)
}
pub fn kuyuSafe(x: f64, y: f64) -> Option<(f64, f64)> {
    crate::mac::hareket::kuyuSafe(x, y)
}
/// match.js kosuPr dönüşü.
#[derive(Clone, Copy, Debug)]
pub struct KosuPr {
    pub Ts: f64,
    pub k: f64,
}
pub fn ALAN_KOSU_PR(p: &Oyuncu, k: &dyn crate::temas::Kural) -> Option<KosuPr> {
    k.kosu_pr(p).map(|(Ts, k)| KosuPr { Ts, k })
}
/// JS `p.slowT || 0`.
pub fn slowT(p: &Oyuncu) -> f64 {
    p.slowT.unwrap_or(0.0)
}

// ---- core.js enerji ----

pub fn ALAN_EN3_DOL(p: &Oyuncu, sahip: bool, nd: f64, yerK: f64) -> f64 {
    crate::mac::enerji::ALAN_EN3_DOL(p, sahip, nd, yerK)
}
pub fn ALAN_EN3_KUL(p: &Oyuncu, e: f64, bizde: bool, bx: f64, by: f64, k: &dyn crate::temas::Kural) -> f64 {
    k.en3_kul(p, e, bizde, Some((bx, by)))
}
