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
    pub dalis_dol_yok: f64,
    pub alan: f64,
    pub alan_min: f64,
    pub sprint: f64,
    pub ivme: f64,
    pub ikili: f64,
    pub dol: f64,
    pub dol_esik: f64,
    pub hiz_alt: f64,
    pub hiz_esik: f64,
    pub spr_esik: f64,
    pub ikili_min: f64,
    pub santra: f64,
    pub sakin_uzak: f64,
    pub sakin_en: f64,
    pub surme: f64,
    pub fake: f64,
    pub fiske: f64,
    pub kalkan: f64,
    pub dolas: f64,
    pub dol_t: f64,
    pub sprBos: f64,
    pub spr_pres: f64,
    pub fiz_k: f64,
}
pub const EN: En = En { dalis: 0.3, dalis_dol_yok: 45.0, alan: 0.02, alan_min: 0.12, sprint: 1.0 / 300.0, ivme: 0.1, ikili: 0.08, dol: 1.0 / 90.0, dol_esik: 1.0, hiz_alt: 0.82, hiz_esik: 0.3, spr_esik: 0.2, ikili_min: 0.1, santra: 0.3, sakin_uzak: 10.0, sakin_en: 0.5, surme: 0.002, fake: 0.03, fiske: 0.02, kalkan: 0.0015, dolas: 1.0 / 400.0, dol_t: 0.004, sprBos: 0.6, spr_pres: 1.3, fiz_k: 1.5 };

/// JS ALAN_EN2 (pat: sürekli koşuda 0 — JS'te alıcı)
pub struct En2 {
    pub tepe: f64,
    pub pat: f64,
    pub dinlen: f64,
    pub tempo_dol: f64,
    pub yuru_dol: f64,
    pub yog_tau: f64,
    pub yog_k: f64,
    pub yog_min: f64,
    pub hiz_dus: f64,
    pub geri_k: f64,
    pub uzan: f64,
    pub uzan_dol_yok: f64,
    pub ref_l: f64,
}
pub const en2: En2 = En2 { tepe: 1.0 / 120.0, pat: 0.0, dinlen: 1.0 / 180.0, tempo_dol: 0.25, yuru_dol: 0.7, yog_tau: 150.0, yog_k: 0.6, yog_min: 0.4, hiz_dus: 0.1, geri_k: 0.5, uzan: 0.08, uzan_dol_yok: 15.0, ref_l: 20.0 };

/// JS ALAN_EN3 (tanım temelde, `core_sabit`: temas da kullanır)
pub use crate::core_sabit::{En3, EN3};

/// JS ALAN_SPR_FAZ (sürekli koşuda alıcının döndürdüğü: patT 1, frenT 4, dolT 0, after 1, patK 1)
pub struct SprFaz {
    pub ust: bool,
    pub hk_pay: f64,
    pub tepe_t: f64,
    pub after: f64,
    pub dol_t: f64,
    pub bedel: f64,
    pub pat_t: f64,
    pub fren_t: f64,
    pub pat_k: f64,
    pub kes: bool,
    pub ayir: bool,
    pub it_kos: bool,
    pub topla: bool,
    pub topla_bos: f64,
}
pub const SPR_FAZ: SprFaz = SprFaz { ust: true, hk_pay: 0.4, tepe_t: 45.0, after: 1.0, dol_t: 0.0, bedel: 0.15, pat_t: 1.0, fren_t: 4.0, pat_k: 1.0, kes: true, ayir: true, it_kos: true, topla: true, topla_bos: 8.0 };

/// JS SPK(): sürekli koşu
pub const fn spk() -> bool {
    true
}
/// JS SBED(F): sprinte girişin bedeli (Enerji 2: EN2.pat)
pub const fn sbed() -> f64 {
    en2.pat
}

// ---- çekirdeğin enerji işlevleri ----

pub fn alan_en_s(p: &Oyuncu) -> f64 {
    0.7 + 0.03 * p.govde.yet.y(mek::BAR_BOY)
}
pub fn alan_en_sd(p: &Oyuncu) -> f64 {
    0.7 + 0.03 * p.govde.yet.y(mek::BAR_DOLUM)
}
pub fn alan_en2_sb(p: &Oyuncu) -> f64 {
    jmax(0.5, jmin(1.6, 1.0 + 5.0 * (alan_en_s(p) - 1.0) / 3.0))
}
pub fn alan_en2_sd(p: &Oyuncu) -> f64 {
    pow(1.71, ((alan_en_sd(p) - 0.7) / 0.03 - 10.0) / 10.0)
}
/// JS ALAN_EN2_TS(p, e): tepe hızda kalınabilecek süre (tik)
pub fn alan_en2_ts(p: &Oyuncu, e: Option<f64>) -> f64 {
    jmax(0.0, e.or(p.en).unwrap_or(1.0) * alan_en2_sb(p) / en2.tepe)
}
/// JS ALAN_EN3_TEPE(p, k): barla düşen tepe katı (en: p.en ya da {en: x})
pub fn alan_en3_tepe(en: Option<f64>, k: f64) -> f64 {
    match en {
        None => k,
        Some(e) => {
            let e = jmax(0.0, jmin(1.0, e));
            1.0 + (k - 1.0) * (EN3.tepe_alt + (1.0 - EN3.tepe_alt) * e)
        }
    }
}
pub fn alan_en3_dol(p: &Oyuncu, sahip: bool, nd: f64, yer_k: f64) -> f64 {
    let b_p = jmax(0.0, jmin(1.0, (nd - EN3.baski_r0) / (EN3.baski_r1 - EN3.baski_r0)));
    let r = if p.bekci() { EN3.r_bek } else if sahip { EN3.r_sahip } else { EN3.r0 + (EN3.r_yer - EN3.r0) * jmax(0.0, jmin(1.0, yer_k)) };
    r * alan_en2_sd(p) * if p.bekci() { 1.0 } else { b_p }
}
pub fn alan_en3_sgir(p: &Oyuncu, bed: f64) -> f64 {
    bed + SPR_FAZ.pat_t * en2.tepe / alan_en2_sb(p)
}
pub const fn alan_spr_after() -> f64 {
    1.0
}
/// JS ALAN_HZ (Enerji 3: ham Hız)
pub fn alan_hz(p: &Oyuncu) -> f64 {
    p.govde.yet.hiz
}
pub fn alan_spr_h(h: f64) -> f64 {
    spr_h(h)
}
/// Çekirdeğin sprint katı (JS SPR0 = sarılmamış ALAN_SPR)
pub fn SPR0(p: &Oyuncu) -> f64 {
    spr_h(p.govde.yet.hiz)
}
/// JS spd(p): temel koşu hızı
pub fn spd(p: &Oyuncu) -> f64 {
    0.17 + 0.006 * alan_hz(p)
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
    pub fn spr_kar(&self, p: &Oyuncu) -> bool {
        p.hal._sprIst && (self.holder == Some(kim(p)) || (self.holder.is_none() && self.flq == Some(kim(p))))
    }
}

/// JS sfPk(p): sprint tepesinin katı (barla)
pub fn sf_pk(p: &Oyuncu) -> f64 {
    alan_en3_tepe(p.en, SPR0(p))
}
/// JS hkK(p, k): hızlı koşunun katı (sprintin hkPay'ı)
pub fn hk_k(_p: &Oyuncu, k: f64) -> f64 {
    1.0 + (k - 1.0) * SPR_FAZ.hk_pay
}

pub fn sf_niyet(p: &Oyuncu, k: &MacKural) -> Option<(bool, Option<f64>)> {
    k.niyet.and_then(|(id, t, s, ik)| if id == p.id && t == p.team { Some((s, ik)) } else { None })
}
/// JS ALAN_SPR (match.js sarması): sprint evresine göre koşu katı.
pub fn alan_spr(p: &Oyuncu, k: &MacKural) -> f64 {
    let F = &SPR_FAZ;
    if p._sf.is_none() {
        if let Some((spr, itK)) = sf_niyet(p, k) {
            return if spr { sf_pk(p) } else { itK.unwrap_or_else(|| hk_k(p, SPR0(p))) };
        }
    }
    match &p._sf {
        Some(s) => {
            if s.f == "dolum" {
                alan_spr_after()
            } else if s.f == "fren" {
                (sf_pk(p) + alan_spr_after()) / 2.0
            } else {
                sf_pk(p)
            }
        }
        None => {
            let e = p.en.unwrap_or(1.0);
            let gir = e >= alan_en3_sgir(p, sbed());
            if (F.it_kos && gir && k.mh == Some(kim(p))) || (gir && !p.bekci()) {
                sf_pk(p)
            } else {
                hk_k(p, SPR0(p))
            }
        }
    }
}
/// JS ALAN_SPR_HK
pub fn alan_spr_hk(p: &Oyuncu) -> f64 {
    hk_k(p, SPR0(p))
}
/// JS sprK(p): barla kısılan sprint katı
pub fn spr_k(p: &Oyuncu, k: &MacKural) -> f64 {
    match p.en {
        Some(e) if e < EN.spr_esik => 1.0 + (alan_spr(p, k) - 1.0) * jmax(0.0, e) / EN.spr_esik,
        _ => alan_spr(p, k),
    }
}
/// JS ALAN_EN2_SPRE(p, L): L boyunca ortalama sprint katı
pub fn alan_en2_spre(p: &Oyuncu, l: Option<f64>, k: &MacKural) -> f64 {
    spre_bar(p, l, alan_spr(p, k))
}
/// Barlı oyuncuda sprintin etkin katı: sprint katı `sp` bar süresince sürer, `l` (yoksa EN2.refL) yolunda
/// ortalama kat. ALAN_EN2_SPRE'nin ve yerleşimin ALAN_SPRE'sinin ortak çekirdeği.
pub fn spre_bar(p: &Oyuncu, l: Option<f64>, sp: f64) -> f64 {
    if !(sp > 1.0) {
        return if dogru(sp) { sp } else { 1.0 };
    }
    let ts = alan_en2_ts(p, None);
    if ts <= 0.0 {
        return 1.0;
    }
    let b = spd(p);
    let lr = l.unwrap_or(en2.ref_l);
    let reach = b * sp * ts;
    if lr <= reach {
        return sp;
    }
    let t = ts + (lr - reach) / b;
    lr / (b * t)
}
/// JS ALAN_SPRE(p, L)
pub fn alan_spre(p: &Oyuncu, l: Option<f64>, k: &MacKural) -> f64 {
    if p.en.is_some() {
        alan_en2_spre(p, l, k)
    } else {
        alan_spr(p, k)
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
    let T = F.pat_t + F.tepe_t;
    let niyet = sf_niyet(p, k);
    if p._sf.is_none() && matches!(niyet, Some((false, _))) {
        return None;
    }
    let kar = niyet.is_some() || k.spr_kar(p) || p.hal._topSpr;
    let ts = if let Some(s) = &p._sf {
        if s.f != "patlama" && s.f != "tepe" {
            return None;
        }
        let b_t = alan_en2_ts(p, None);
        jmax(0.0, F.pat_t - (tk - s.t0)) + if kar { b_t } else { jmin(b_t, jmax(0.0, T - jmax(F.pat_t, tk - s.t0))) }
    } else {
        let e0 = p.en.unwrap_or(1.0);
        if !(e0 >= alan_en3_sgir(p, sbed())) {
            return None;
        }
        let b_t = alan_en2_ts(p, Some(e0 - sbed()));
        F.pat_t + if kar { b_t } else { jmin(b_t, F.tepe_t) }
    };
    let f = alan_spr(p, k);
    let h = hk_k(p, SPR0(p));
    Some(KosuPr { Ts: ts, k: if f > 0.0 { jmin(1.0, h / f) } else { 1.0 } })
}
pub fn alan_kosu_tp(pr: Option<KosuPr>, d: f64, v1: f64) -> f64 {
    match pr {
        Some(pr) if d > 0.0 => {
            let d_s = v1 * pr.Ts;
            if d <= d_s {
                d / v1
            } else {
                pr.Ts + (d - d_s) / (v1 * pr.k)
            }
        }
        _ => d / v1,
    }
}
/// JS ALAN_KOSU_T(p, d, v1): d'yi koşma süresi
pub fn alan_kosu_t(p: &Oyuncu, d: f64, v1: f64, k: &MacKural) -> f64 {
    alan_kosu_tp(kosuPr(p, k), d, v1)
}
/// JS ALAN_KOSU_M(p, t, v1): t tikte alınan yol
pub fn alan_kosu_m(p: &Oyuncu, t: f64, v1: f64, k: &MacKural) -> f64 {
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
        alan_spr(p, self)
    }
    fn kosu_pr(&self, p: &Oyuncu) -> Option<(f64, f64)> {
        kosuPr(p, self).map(|k| (k.Ts, k.k))
    }
    fn en3_kul(&self, p: &Oyuncu, e: f64, bizde: bool, b: Option<(f64, f64)>) -> f64 {
        alan_en3_kul(p, e, bizde, b, self)
    }
}


/// Kafanın tehdit ölçüsü, sabitleriyle (kafa `threat_d(ayar())` ile aynı değer).
#[inline]
fn tehdit_s(team: f64, x: f64, y: f64) -> f64 {
    crate::saha::tehdit(crate::core_sabit::TEHDIT_K, crate::core_sabit::TEHDIT_YAN, team, x, y)
}

/// JS ALAN_EN3_KUL(p, e, bizde, bx, by): barın e düzeyindeki kullanım değeri.
pub fn alan_en3_kul(p: &Oyuncu, e: f64, bizde: bool, bxy: Option<(f64, f64)>, k: &MacKural) -> f64 {
    let Some((bx, by)) = bxy else { return 0.0 };
    let ee = jmax(0.0, jmin(1.0, e));
    let yak = exp(-hypot(p.x - bx, p.y - by) / 10.0);
    let t_b = tehdit_s(if bizde { p.team } else { 1.0 - p.team }, bx, by);
    let mut u = yak * t_b * crate::core_sabit::EN_FARK * ee;
    if bizde {
        u += tehdit_s(p.team, p.x, p.y) * jmin(1.0, ee / 0.2);
    }
    if !p.bekci() {
        let b = spd(p);
        let pk = if SPR_FAZ.ust { alan_spr(p, k) } else { 1.3 };
        let d = jmax(3.0, jmin(25.0, hypot(p.x - bx, p.y - by)));
        let T = |x: f64| {
            if x < en2.pat {
                return d / b;
            }
            let v_s = b * alan_en3_tepe(Some(x), pk);
            let ts = alan_en2_ts(p, Some(x - en2.pat));
            let r = v_s * ts;
            if d <= r {
                d / v_s
            } else {
                ts + (d - r) / b
            }
        };
        let T1 = T(1.0);
        u += yak * (t_b + tehdit_s(if bizde { 1.0 - p.team } else { p.team }, bx, by)) / (1.0 + exp((T(ee) - T1) / 5.0));
    }
    u
}

/// Ortak buluşma süresi (hareketli hedefe yetişme; savunma presi, hücumun varış hesapları aynı formülü okusun): (px, py)'deki
/// oyuncu `gecikme` tik (tepki + dönüş/hızlanma; çağıran kendi modeliyle verir) bekleyip sonra sabit v hızla gider; hedef
/// (hx, hy)'den sabit (vx, vy) ile gider (sabit hedefte 0). R erişimle en erken buluşma (tik); yetişilemiyorsa ufuk.
/// Gecikme içinde hedef erişime kendisi girerse o an.
pub fn bulus_t(px: f64, py: f64, v: f64, hx: f64, hy: f64, vx: f64, vy: f64, R: f64, ufuk: f64, gecikme: f64) -> f64 {
    let (rx, ry) = (hx - px, hy - py);
    let c0 = rx * rx + ry * ry - R * R;
    if c0 <= 0.0 {
        return 0.0;
    }
    let t0 = jmax(0.0, gecikme);
    // |r + vh t| = Rt + w t'nin [alt, ∞) içindeki en küçük kökü
    let kok = |Rt: f64, w: f64, alt: f64, ust: f64| -> f64 {
        let a = vx * vx + vy * vy - w * w;
        let b = 2.0 * (rx * vx + ry * vy - Rt * w);
        let c = rx * rx + ry * ry - Rt * Rt;
        let ok = |t: f64| t >= alt && t <= ust;
        if a.abs() < 1e-9 {
            if b.abs() < 1e-12 { return f64::INFINITY; }
            let t = -c / b;
            return if ok(t) { t } else { f64::INFINITY };
        }
        let D = b * b - 4.0 * a * c;
        if D < 0.0 {
            return f64::INFINITY;
        }
        let q = D.sqrt();
        let (t1, t2) = ((-b - q) / (2.0 * a), (-b + q) / (2.0 * a));
        let (lo, hi) = if t1 < t2 { (t1, t2) } else { (t2, t1) };
        if ok(lo) { lo } else if ok(hi) { hi } else { f64::INFINITY }
    };
    // gecikmede hedef kendiliğinden erişime girer mi
    let t_a = if t0 > 0.0 { kok(R, 0.0, 0.0, t0) } else { f64::INFINITY };
    let t_b = if t_a.is_finite() { t_a } else { kok(R - v * t0, v, t0, f64::INFINITY) };
    // gecikme bittiğinde zaten erişimdeyse
    let t = if !t_b.is_finite() && t0 > 0.0 && { let (x, y) = (rx + vx * t0, ry + vy * t0); x * x + y * y <= R * R } { t0 } else { t_b };
    jmin(t, ufuk)
}
#[cfg(test)]
mod bulus_sina {
    use super::bulus_t;
    #[test]
    fn bulus() {
        // durgun hedef: (d - R) / v
        assert!((bulus_t(0.0, 0.0, 0.5, 10.0, 0.0, 0.0, 0.0, 2.0, 90.0, 0.0) - 16.0).abs() < 1e-9);
        // gecikme eklenir
        assert!((bulus_t(0.0, 0.0, 0.5, 10.0, 0.0, 0.0, 0.0, 2.0, 90.0, 4.0) - 20.0).abs() < 1e-9);
        // karşıdan gelen: (10 - 2) / (0.5 + 0.3) = 10
        assert!((bulus_t(0.0, 0.0, 0.5, 10.0, 0.0, -0.3, 0.0, 2.0, 90.0, 0.0) - 10.0).abs() < 1e-9);
        // kaçan, aynı hız: ufuk
        assert_eq!(bulus_t(0.0, 0.0, 0.3, 10.0, 0.0, 0.3, 0.0, 2.0, 90.0, 0.0), 90.0);
        // gecikmede hedef erişime girer: 10 br, hedef 1 br/tik geliyor, R 2 → 8
        assert!((bulus_t(0.0, 0.0, 0.5, 10.0, 0.0, -1.0, 0.0, 2.0, 90.0, 20.0) - 8.0).abs() < 1e-9);
    }
}
