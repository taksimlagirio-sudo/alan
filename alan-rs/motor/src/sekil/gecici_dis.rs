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

use crate::model::Oyuncu;

use super::varlik::{KayipF, Mc, Varlik};

// ---- AlanDecide ----
use crate::kafa::kafa1_js::{nk, Op};
use crate::dunya::ow;
pub use crate::dunya::ow_paylas;
fn ops(ps: &[Oyuncu]) -> crate::dunya::Dunya {
    crate::dunya::op_dunya(ps)
}

pub fn threat(team: u8, x: f64, y: f64) -> f64 {
    // saf: kafa bağlamı aranmaz (kesişme döngüsünde milyonlarca kez çağrılır)
    crate::kafa::kafa1_tehlike::threat_d(crate::kafa::kafa2_baglam::ayar(), team as f64, x, y)
}
/// Saf (yetenek ve kuraldan): kafaya uğramaz.
pub fn fren_t(p: &Oyuncu, x: f64, y: f64, R: f64, T: f64, v: f64) -> f64 {
    crate::kafa::kafa1_varis::fren_t_p(p, x, y, R, T, v)
}
/// Saf (ayardan): kafaya uğramaz.
pub fn free_at(w: &[Varlik], oy: &[Oyuncu], team: f64, x: f64, y: f64) -> f64 {
    let o = ow(w, oy);
    crate::kafa::kafa1_tehlike::free_at_d(crate::kafa::kafa2_baglam::ayar(), o.iter().map(|p| &**p), team, x, y)
}
pub fn core_side(h: &Oyuncu) -> (f64, f64) {
    let c = crate::kafa::kafa1_ikili::core_side_p(h);
    (c.ux, c.uy)
}
/// Taşıyıcının şimdi çekebileceği en sert şutun hızı (tam şarj): kural katmanından, kafaya uğramadan
/// (eskiden yerleşim → kafa iç içe çağrısıydı; yedek kafaya düşen çağrıların neredeyse hepsi buradandı).
/// Maç kurulurken (kafa yokken) çağrılmaz: Bekçi'nin bağlamı (BekM::sut) bunu bilir.
pub fn sut_hiz(h: &Oyuncu) -> f64 {
    crate::kural::sut::sut_hiz(h)
}
pub fn top_k(h: &Oyuncu) -> f64 {
    crate::kafa::kafa1_ikili::top_k_p(h, crate::kafa::gecici_dis1::mac_dokunus())
}
/// JS'te oyuncular dünyadaki nesnelerin kendisidir (kafa `o !== q` ile karşılaştırır): kopya dünyada kimliğiyle bul.
fn kim<'a>(src: &'a [Op], p: &'a Oyuncu) -> &'a Oyuncu {
    src.iter().find(|q| q.id == p.id && q.team == p.team).map(|q| &**q).unwrap_or(p)
}
pub fn duel_p(kafa: KafaEl, q: &Oyuncu, h: &Oyuncu, ps: &[Oyuncu]) -> f64 {
    let src = ops(ps);
    kafa_ile(kafa, |k| k.duel_p(kim(&src, q), kim(&src, h), &src, None))
}
pub fn gir_thr(kafa: KafaEl, q: &Oyuncu, h: &Oyuncu, ps: &[Oyuncu]) -> f64 {
    let src = ops(ps);
    kafa_ile(kafa, |k| k.gir_thr(kim(&src, q), kim(&src, h), &src))
}
pub fn ucus_t(kafa: KafaEl, L: f64, arrive: f64, al: Option<f64>) -> f64 {
    kafa_ile(kafa, |k| k.ucus_t(L, arrive, al))
}
/// `arr`: None = JS'te verilmedi (undefined).
pub fn lane_ok(kafa: KafaEl, w: &[Varlik], oy: &[Oyuncu], team: u8, A: (f64, f64), B: (f64, f64), arr: f64, lag: Option<f64>) -> f64 {
    kafa_ile(kafa, |k| k.lane_ok(&ow(w, oy), team as f64, nk(A.0, A.1), nk(B.0, B.1), Some(arr), lag))
}
/// JS laneOk(src, team, A, B) (arr ve lag verilmeden).
pub fn lane_ok0(kafa: KafaEl, w: &[Varlik], oy: &[Oyuncu], team: u8, A: (f64, f64), B: (f64, f64)) -> f64 {
    kafa_ile(kafa, |k| k.lane_ok(&ow(w, oy), team as f64, nk(A.0, A.1), nk(B.0, B.1), None, None))
}
pub fn bek_yol(kafa: KafaEl, w: &[Varlik], oy: &[Oyuncu], team: u8, A: (f64, f64), B: (f64, f64), lag: f64) -> f64 {
    kafa_ile(kafa, |k| crate::kafa::kafa2_zemin::bek_yol(k, &ow(w, oy), team as f64, nk(A.0, A.1), nk(B.0, B.1), Some(lag)))
}
pub fn yer_deger(kafa: KafaEl, w: &[Varlik], oy: &[Oyuncu], team: u8, x: f64, y: f64, ch: f64) -> f64 {
    kafa_ile(kafa, |k| crate::kafa::kafa2_zemin::yer_deger(k, &ow(w, oy), team as f64, x, y, ch))
}
pub fn recv_ch(w: &[Varlik], oy: &[Oyuncu], team: u8, x: f64, y: f64, ch: f64, q: Option<&Oyuncu>) -> f64 {
    // saf: dünyayı ve kafayı okumaz
    let _ = (w, oy, team, x, y, q);
    crate::kafa::kafa1_tehlike::recv_ch_p(ch)
}
pub fn keep_p(kafa: KafaEl, h: &Oyuncu, w: &[Varlik], oy: &[Oyuncu], dir: Option<(f64, f64)>, T: f64) -> f64 {
    kafa_ile(kafa, |k| k.keep_p(h, &ow(w, oy), dir.map(|d| nk(d.0, d.1)), T))
}
pub fn run_on(kafa: KafaEl, w: &[Varlik], oy: &[Oyuncu], team: u8, h: &Oyuncu, at: (f64, f64)) -> f64 {
    kafa_ile(kafa, |k| k.run_on(&ow(w, oy), team as f64, h, nk(at.0, at.1)))
}
pub fn space(kafa: KafaEl, w: &[Varlik], oy: &[Oyuncu], team: u8, x: f64, y: f64) -> f64 {
    kafa_ile(kafa, |k| k.space(&ow(w, oy), team as f64, x, y))
}
pub fn front(kafa: KafaEl, w: &[Varlik], oy: &[Oyuncu], team: u8, x: f64, y: f64) -> f64 {
    kafa_ile(kafa, |k| k.front(&ow(w, oy), team as f64, x, y))
}
pub fn dagin(w: &[Varlik], oy: &[Oyuncu], team: u8, x: f64, vv: f64, y: f64) -> f64 {
    // saf: kafaya uğramaz
    let o = ow(w, oy);
    crate::kafa::kafa1_tehlike::dagin_p(o.iter().map(|p| &**p), team as f64, x, vv, Some(y))
}
pub fn kayip_yer(kafa: KafaEl, ps: &[Oyuncu], team: u8, ch: f64, h: &Oyuncu, tac: Option<&crate::mac_veri::Tac>) -> KayipF {
    let src = std::sync::Arc::new(ops(ps));
    let tac = tac.map(crate::kafa::kafa2_baglam::kafa_tac);
    let f = kafa_ile(kafa, |k| crate::kafa::kafa2_zemin::kayip_yer(k, &src, team as f64, ch, h, tac.as_ref()));
    let f = std::sync::Mutex::new(f);
    std::sync::Arc::new(move |kafa: KafaEl, x: f64, y: f64| kafa_ile(kafa, |k| f.lock().unwrap().deger(k, x, y)))
}
/// decide.js:6875 `PNC_GORUS`.
pub const PNC_GORUS: f64 = 48.0;
/// decide.js `DU.fakeT`, `DU.oyalaR`, `DU.engage`.
pub const DU_FAKE_T: f64 = 9.0;
pub const DU_ENGAGE: f64 = 2.4;

// ---- AlanMatch ----

/// JS tl(p, ev, k): `null` → None.
pub fn tl(p: &Oyuncu, ev: &str, k: &str) -> Option<f64> {
    crate::kural::tac::tl(p, ev, k)
}
/// JS kpAkt(m, t): kontrpresçilerin id'leri (m.kp'yi bitirebilir).
pub fn kp_akt(c: &mut Mc, t: u8) -> Option<Vec<f64>> {
    c.mac_ile(|m| crate::kural::tac::kp_akt(m, t as f64))
}
/// Etkin taktik (JS tacEff dönüşü): taktik + kontrpres (`_kp`) işareti. (`plan`, `enPresDalga` hiç kurulmaz.)
#[derive(Clone, Debug, Default)]
pub struct TacE {
    pub t: crate::mac_veri::Tac,
    pub kp: bool,
}
pub fn tac_eff(c: &mut Mc, t: u8) -> TacE {
    let t = c.mac_ile(|m| crate::kural::tac::tac_eff(m, t as f64));
    let kp = t._kp.is_some();
    TacE { t, kp }
}
/// JS `macEff(m, t, {}).macK`.
pub fn mac_eff_mac_k(c: &mut Mc, t: u8) -> Option<f64> {
    c.mac_ile(|m| crate::kural::tac::mac_eff(m, t as f64, Default::default()).macK)
}
pub fn kuyu_safe(x: f64, y: f64) -> Option<(f64, f64)> {
    crate::kural::hareket::kuyu_safe(x, y)
}
/// match.js kosuPr dönüşü.
#[derive(Clone, Copy, Debug)]
pub struct KosuPr {
    pub Ts: f64,
    pub k: f64,
}
pub fn alan_kosu_pr(p: &Oyuncu, k: &dyn crate::temas::Kural) -> Option<KosuPr> {
    k.kosu_pr(p).map(|(Ts, k)| KosuPr { Ts, k })
}
/// JS `p.slowT || 0`.
pub fn slowT(p: &Oyuncu) -> f64 {
    p.hal.slowT.unwrap_or(0.0)
}

// ---- core.js enerji ----

pub fn alan_en3_dol(p: &Oyuncu, sahip: bool, nd: f64, yer_k: f64) -> f64 {
    crate::kural::enerji::alan_en3_dol(p, sahip, nd, yer_k)
}
pub fn alan_en3_kul(p: &Oyuncu, e: f64, bizde: bool, bx: f64, by: f64, k: &dyn crate::temas::Kural) -> f64 {
    k.en3_kul(p, e, bizde, Some((bx, by)))
}

pub use crate::sekil::varlik::KafaEl;
/// Maçın kafasıyla çalış (MIMARI.md 3.8; eskiden maç katmanının `kafa_mac`'i, kafayı maçın kutusundan alırdı).
/// İç içe çağrı (kafa meşgulken yerleşimden kafaya dönüş) yapı gereği olmaz: açık hatayla durulur.
pub fn kafa_ile<R>(kafa: KafaEl, f: impl FnOnce(&mut crate::kafa::Kafa) -> R) -> R {
    let k = kafa.expect("kafa_ile: maçın kafası kurulmadı (kafa_hazirla) ya da hafif kopyadan çağrıldı");
    let mut b = k.try_borrow_mut().expect("kafa_ile: iç içe kafa çağrısı (kafa meşgulken yerleşimden kafaya dönüldü)");
    f(&mut b)
}
