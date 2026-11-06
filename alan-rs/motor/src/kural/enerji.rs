//! Enerji, sprint ve koşu kuralları (saf): oyuncu ve maçın anlık görüntüsü (`MacKural`) girer, sayı çıkar.
//! Maç durumunu değiştiren yordamlar (enerji2, sprint kararı, alan seçimi) `mac::enerji`'de.
//!
//! JS'te sprint katı küresel `self.ALAN_SPR`'dır ve match.js onu sprint evresine göre sarar;
//! burada `ALAN_SPR(p, k)` açık işlev, maçın bağlamı `MacKural` (çekirdeğin `temas::Kural`'ı).
//! Bugünkü bayraklar: Enerji 2/3 açık, sürekli koşu açık (ALAN_SUREKLI), sprint evresi açık (SPR_FAZ),
//! dokunuşla sürme kapalı.
//!
//! Kafanın tartı sırasında verdiği sprint niyeti `MacKural::niyet`'te (açık bağlam).
use crate::mat::{dogru, exp, hypot, jmax, jmin, pow};
use crate::model::Oyuncu;
use crate::oyuncu::kim;
use crate::yetenek::{mek, spr_h};

/// JS ALAN_EN (match.js sprint ve dolT'u değiştirir: 1/300, 0,004)
pub struct En {
    pub dalis: f64,
    pub dalisDolYok: f64,
    pub alan: f64,
    pub alanMin: f64,
    pub sprint: f64,
    pub ivme: f64,
    pub ikili: f64,
    pub dol: f64,
    pub dolEsik: f64,
    pub hizAlt: f64,
    pub hizEsik: f64,
    pub sprEsik: f64,
    pub ikiliMin: f64,
    pub santra: f64,
    pub sakinUzak: f64,
    pub sakinEn: f64,
    pub surme: f64,
    pub fake: f64,
    pub fiske: f64,
    pub kalkan: f64,
    pub dolas: f64,
    pub dolT: f64,
    pub sprBos: f64,
    pub sprPres: f64,
    pub fizK: f64,
}
pub const EN: En = En { dalis: 0.3, dalisDolYok: 45.0, alan: 0.02, alanMin: 0.12, sprint: 1.0 / 300.0, ivme: 0.1, ikili: 0.08, dol: 1.0 / 90.0, dolEsik: 1.0, hizAlt: 0.82, hizEsik: 0.3, sprEsik: 0.2, ikiliMin: 0.1, santra: 0.3, sakinUzak: 10.0, sakinEn: 0.5, surme: 0.002, fake: 0.03, fiske: 0.02, kalkan: 0.0015, dolas: 1.0 / 400.0, dolT: 0.004, sprBos: 0.6, sprPres: 1.3, fizK: 1.5 };

/// JS ALAN_EN2 (pat: sürekli koşuda 0 — JS'te alıcı)
pub struct En2 {
    pub tepe: f64,
    pub pat: f64,
    pub dinlen: f64,
    pub tempoDol: f64,
    pub yuruDol: f64,
    pub yogTau: f64,
    pub yogK: f64,
    pub yogMin: f64,
    pub hizDus: f64,
    pub geriK: f64,
    pub uzan: f64,
    pub uzanDolYok: f64,
    pub refL: f64,
}
pub const EN2: En2 = En2 { tepe: 1.0 / 120.0, pat: 0.0, dinlen: 1.0 / 180.0, tempoDol: 0.25, yuruDol: 0.7, yogTau: 150.0, yogK: 0.6, yogMin: 0.4, hizDus: 0.1, geriK: 0.5, uzan: 0.08, uzanDolYok: 15.0, refL: 20.0 };

/// JS ALAN_EN3 (tanım temelde, `core_sabit`: temas da kullanır)
pub use crate::core_sabit::{En3, EN3};

/// JS ALAN_SPR_FAZ (sürekli koşuda alıcının döndürdüğü: patT 1, frenT 4, dolT 0, after 1, patK 1)
pub struct SprFaz {
    pub ust: bool,
    pub hkPay: f64,
    pub tepeT: f64,
    pub after: f64,
    pub dolT: f64,
    pub bedel: f64,
    pub patT: f64,
    pub frenT: f64,
    pub patK: f64,
    pub kes: bool,
    pub ayir: bool,
    pub itKos: bool,
    pub topla: bool,
    pub toplaBos: f64,
}
pub const SPR_FAZ: SprFaz = SprFaz { ust: true, hkPay: 0.4, tepeT: 45.0, after: 1.0, dolT: 0.0, bedel: 0.15, patT: 1.0, frenT: 4.0, patK: 1.0, kes: true, ayir: true, itKos: true, topla: true, toplaBos: 8.0 };

/// JS SPK(): sürekli koşu
pub const fn SPK() -> bool {
    true
}
/// JS SBED(F): sprinte girişin bedeli (Enerji 2: EN2.pat)
pub const fn SBED() -> f64 {
    EN2.pat
}

// ---- çekirdeğin enerji işlevleri ----

pub fn ALAN_EN_S(p: &Oyuncu) -> f64 {
    0.7 + 0.03 * p.govde.yet.y(mek::BAR_BOY)
}
pub fn ALAN_EN_SD(p: &Oyuncu) -> f64 {
    0.7 + 0.03 * p.govde.yet.y(mek::BAR_DOLUM)
}
pub fn ALAN_EN2_SB(p: &Oyuncu) -> f64 {
    jmax(0.5, jmin(1.6, 1.0 + 5.0 * (ALAN_EN_S(p) - 1.0) / 3.0))
}
pub fn ALAN_EN2_SD(p: &Oyuncu) -> f64 {
    pow(1.71, ((ALAN_EN_SD(p) - 0.7) / 0.03 - 10.0) / 10.0)
}
/// JS ALAN_EN2_TS(p, e): tepe hızda kalınabilecek süre (tik)
pub fn ALAN_EN2_TS(p: &Oyuncu, e: Option<f64>) -> f64 {
    jmax(0.0, e.or(p.en).unwrap_or(1.0) * ALAN_EN2_SB(p) / EN2.tepe)
}
/// JS ALAN_EN3_TEPE(p, k): barla düşen tepe katı (en: p.en ya da {en: x})
pub fn ALAN_EN3_TEPE(en: Option<f64>, k: f64) -> f64 {
    match en {
        None => k,
        Some(e) => {
            let e = jmax(0.0, jmin(1.0, e));
            1.0 + (k - 1.0) * (EN3.tepeAlt + (1.0 - EN3.tepeAlt) * e)
        }
    }
}
pub fn ALAN_EN3_DOL(p: &Oyuncu, sahip: bool, nd: f64, yerK: f64) -> f64 {
    let bP = jmax(0.0, jmin(1.0, (nd - EN3.baskiR0) / (EN3.baskiR1 - EN3.baskiR0)));
    let r = if p.bekci() { EN3.rBek } else if sahip { EN3.rSahip } else { EN3.r0 + (EN3.rYer - EN3.r0) * jmax(0.0, jmin(1.0, yerK)) };
    r * ALAN_EN2_SD(p) * if p.bekci() { 1.0 } else { bP }
}
pub fn ALAN_EN3_SGIR(p: &Oyuncu, bed: f64) -> f64 {
    bed + SPR_FAZ.patT * EN2.tepe / ALAN_EN2_SB(p)
}
pub const fn ALAN_SPR_AFTER() -> f64 {
    1.0
}
/// JS ALAN_HZ (Enerji 3: ham Hız)
pub fn ALAN_HZ(p: &Oyuncu) -> f64 {
    p.govde.yet.hiz
}
pub fn ALAN_SPR_H(h: f64) -> f64 {
    spr_h(h)
}
/// Çekirdeğin sprint katı (JS SPR0 = sarılmamış ALAN_SPR)
pub fn SPR0(p: &Oyuncu) -> f64 {
    spr_h(p.govde.yet.hiz)
}
/// JS spd(p): temel koşu hızı
pub fn spd(p: &Oyuncu) -> f64 {
    0.17 + 0.006 * ALAN_HZ(p)
}

/// Maçın sprint bağlamı: JS `self.__mH` (son karar veren taşıyıcı), kosuPr'nin tik/taşıyıcı/alıcısı.
#[derive(Clone, Copy, Debug, Default)]
pub struct MacKural {
    /// JS self.__mH (team, id)
    pub mh: Option<(f64, f64)>,
    /// JS D._tick || _mt
    pub tick: f64,
    /// JS _mM.holder (team, id)
    pub holder: Option<(f64, f64)>,
    /// JS _mM.fl.q (team, id)
    pub flq: Option<(f64, f64)>,
    /// Kafanın bir seçeneği tartarken taşıyıcıya verdiği sprint niyeti (id, team, sprint mi, itK); tartı
    /// süresince dış dünyanın anlık görüntüsünde durur (JS `h._sfNiyet`, kopyaları da görür). Eskiden iş
    /// parçacığına özel bir değişkendi (SF_NIYET).
    pub niyet: Option<(f64, f64, bool, Option<f64>)>,
}
impl MacKural {
    /// JS sprKar(m, p)
    pub fn sprKar(&self, p: &Oyuncu) -> bool {
        p.hal._sprIst && (self.holder == Some(kim(p)) || (self.holder.is_none() && self.flq == Some(kim(p))))
    }
}

/// JS sfPk(p): sprint tepesinin katı (barla)
pub fn sfPk(p: &Oyuncu) -> f64 {
    ALAN_EN3_TEPE(p.en, SPR0(p))
}
/// JS hkK(p, k): hızlı koşunun katı (sprintin hkPay'ı)
pub fn hkK(_p: &Oyuncu, k: f64) -> f64 {
    1.0 + (k - 1.0) * SPR_FAZ.hkPay
}

pub fn sf_niyet(p: &Oyuncu, k: &MacKural) -> Option<(bool, Option<f64>)> {
    k.niyet.and_then(|(id, t, s, ik)| if id == p.id && t == p.team { Some((s, ik)) } else { None })
}
/// JS ALAN_SPR (match.js sarması): sprint evresine göre koşu katı.
pub fn ALAN_SPR(p: &Oyuncu, k: &MacKural) -> f64 {
    let F = &SPR_FAZ;
    if p._sf.is_none() {
        if let Some((spr, itK)) = sf_niyet(p, k) {
            return if spr { sfPk(p) } else { itK.unwrap_or_else(|| hkK(p, SPR0(p))) };
        }
    }
    match &p._sf {
        Some(s) => {
            if s.f == "dolum" {
                ALAN_SPR_AFTER()
            } else if s.f == "fren" {
                (sfPk(p) + ALAN_SPR_AFTER()) / 2.0
            } else {
                sfPk(p)
            }
        }
        None => {
            let e = p.en.unwrap_or(1.0);
            let gir = e >= ALAN_EN3_SGIR(p, SBED());
            if (F.itKos && gir && k.mh == Some(kim(p))) || (gir && !p.bekci()) {
                sfPk(p)
            } else {
                hkK(p, SPR0(p))
            }
        }
    }
}
/// JS ALAN_SPR_HK
pub fn ALAN_SPR_HK(p: &Oyuncu) -> f64 {
    hkK(p, SPR0(p))
}
/// JS sprK(p): barla kısılan sprint katı
pub fn sprK(p: &Oyuncu, k: &MacKural) -> f64 {
    match p.en {
        Some(e) if e < EN.sprEsik => 1.0 + (ALAN_SPR(p, k) - 1.0) * jmax(0.0, e) / EN.sprEsik,
        _ => ALAN_SPR(p, k),
    }
}
/// JS ALAN_EN2_SPRE(p, L): L boyunca ortalama sprint katı
pub fn ALAN_EN2_SPRE(p: &Oyuncu, l: Option<f64>, k: &MacKural) -> f64 {
    spre_bar(p, l, ALAN_SPR(p, k))
}
/// Barlı oyuncuda sprintin etkin katı: sprint katı `sp` bar süresince sürer, `l` (yoksa EN2.refL) yolunda
/// ortalama kat. ALAN_EN2_SPRE'nin ve yerleşimin ALAN_SPRE'sinin ortak çekirdeği.
pub fn spre_bar(p: &Oyuncu, l: Option<f64>, sp: f64) -> f64 {
    if !(sp > 1.0) {
        return if dogru(sp) { sp } else { 1.0 };
    }
    let ts = ALAN_EN2_TS(p, None);
    if ts <= 0.0 {
        return 1.0;
    }
    let b = spd(p);
    let lr = l.unwrap_or(EN2.refL);
    let reach = b * sp * ts;
    if lr <= reach {
        return sp;
    }
    let t = ts + (lr - reach) / b;
    lr / (b * t)
}
/// JS ALAN_SPRE(p, L)
pub fn ALAN_SPRE(p: &Oyuncu, l: Option<f64>, k: &MacKural) -> f64 {
    if p.en.is_some() {
        ALAN_EN2_SPRE(p, l, k)
    } else {
        ALAN_SPR(p, k)
    }
}

/// JS kosuPr(p): sprint profili {Ts: tepe süresi, k: sonrasının katı}
#[derive(Clone, Copy, Debug)]
pub struct KosuPr {
    pub Ts: f64,
    pub k: f64,
}
pub fn kosuPr(p: &Oyuncu, k: &MacKural) -> Option<KosuPr> {
    let F = &SPR_FAZ;
    if p.bekci() {
        return None;
    }
    let tk = k.tick;
    let T = F.patT + F.tepeT;
    let niyet = sf_niyet(p, k);
    if p._sf.is_none() && matches!(niyet, Some((false, _))) {
        return None;
    }
    let kar = niyet.is_some() || k.sprKar(p) || p.hal._topSpr;
    let ts = if let Some(s) = &p._sf {
        if s.f != "patlama" && s.f != "tepe" {
            return None;
        }
        let bT = ALAN_EN2_TS(p, None);
        jmax(0.0, F.patT - (tk - s.t0)) + if kar { bT } else { jmin(bT, jmax(0.0, T - jmax(F.patT, tk - s.t0))) }
    } else {
        let e0 = p.en.unwrap_or(1.0);
        if !(e0 >= ALAN_EN3_SGIR(p, SBED())) {
            return None;
        }
        let bT = ALAN_EN2_TS(p, Some(e0 - SBED()));
        F.patT + if kar { bT } else { jmin(bT, F.tepeT) }
    };
    let f = ALAN_SPR(p, k);
    let h = hkK(p, SPR0(p));
    Some(KosuPr { Ts: ts, k: if f > 0.0 { jmin(1.0, h / f) } else { 1.0 } })
}
pub fn ALAN_KOSU_TP(pr: Option<KosuPr>, d: f64, v1: f64) -> f64 {
    match pr {
        Some(pr) if d > 0.0 => {
            let dS = v1 * pr.Ts;
            if d <= dS {
                d / v1
            } else {
                pr.Ts + (d - dS) / (v1 * pr.k)
            }
        }
        _ => d / v1,
    }
}
/// JS ALAN_KOSU_T(p, d, v1): d'yi koşma süresi
pub fn ALAN_KOSU_T(p: &Oyuncu, d: f64, v1: f64, k: &MacKural) -> f64 {
    ALAN_KOSU_TP(kosuPr(p, k), d, v1)
}
/// JS ALAN_KOSU_M(p, t, v1): t tikte alınan yol
pub fn ALAN_KOSU_M(p: &Oyuncu, t: f64, v1: f64, k: &MacKural) -> f64 {
    match kosuPr(p, k) {
        Some(pr) if t > 0.0 => {
            if t <= pr.Ts {
                v1 * t
            } else {
                v1 * pr.Ts + v1 * pr.k * (t - pr.Ts)
            }
        }
        _ => jmax(0.0, t) * v1,
    }
}

// Çekirdeğin kural arayüzü (Aşama 7c: tipiyle aynı crate'te; eskiden mac/enerji.rs).
use crate::temas::Kural;
impl Kural for MacKural {
    fn spr(&self, p: &Oyuncu) -> f64 {
        ALAN_SPR(p, self)
    }
    fn kosu_pr(&self, p: &Oyuncu) -> Option<(f64, f64)> {
        kosuPr(p, self).map(|k| (k.Ts, k.k))
    }
    fn en3_kul(&self, p: &Oyuncu, e: f64, bizde: bool, b: Option<(f64, f64)>) -> f64 {
        ALAN_EN3_KUL(p, e, bizde, b, self)
    }
}


/// Kafanın tehdit ölçüsü, sabitleriyle (kafa `threat_d(ayar())` ile aynı değer).
#[inline]
fn tehdit_s(team: f64, x: f64, y: f64) -> f64 {
    crate::saha::tehdit(crate::core_sabit::TEHDIT_K, crate::core_sabit::TEHDIT_YAN, team, x, y)
}

/// JS ALAN_EN3_KUL(p, e, bizde, bx, by): barın e düzeyindeki kullanım değeri.
pub fn ALAN_EN3_KUL(p: &Oyuncu, e: f64, bizde: bool, bxy: Option<(f64, f64)>, k: &MacKural) -> f64 {
    let Some((bx, by)) = bxy else { return 0.0 };
    let ee = jmax(0.0, jmin(1.0, e));
    let yak = exp(-hypot(p.x - bx, p.y - by) / 10.0);
    let tB = tehdit_s(if bizde { p.team } else { 1.0 - p.team }, bx, by);
    let mut u = yak * tB * crate::core_sabit::EN_FARK * ee;
    if bizde {
        u += tehdit_s(p.team, p.x, p.y) * jmin(1.0, ee / 0.2);
    }
    if !p.bekci() {
        let b = spd(p);
        let pk = if SPR_FAZ.ust { ALAN_SPR(p, k) } else { 1.3 };
        let d = jmax(3.0, jmin(25.0, hypot(p.x - bx, p.y - by)));
        let T = |x: f64| {
            if x < EN2.pat {
                return d / b;
            }
            let vS = b * ALAN_EN3_TEPE(Some(x), pk);
            let ts = ALAN_EN2_TS(p, Some(x - EN2.pat));
            let r = vS * ts;
            if d <= r {
                d / vS
            } else {
                ts + (d - r) / b
            }
        };
        let T1 = T(1.0);
        u += yak * (tB + tehdit_s(if bizde { 1.0 - p.team } else { p.team }, bx, by)) / (1.0 + exp((T(ee) - T1) / 5.0));
    }
    u
}
