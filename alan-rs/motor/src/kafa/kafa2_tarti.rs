//! Prensip tartısı (decide.js 6747–6772, 6811–7292): sürme değeri (surD), Bekçi'nin pas yolu (bekYol),
//! tartının dünyaları (pDunya), alıcının penceresi (pencere), en iyi pas (enIyiPas), düğüm değeri
//! (nodeV), durum (durumD), imkânlı zemin (zeminH), prensipDegerle / prensipDegerle0, prGam.
#![allow(non_snake_case, clippy::too_many_arguments)]

pub use super::kafa2_zemin::bek_yol;
use super::kafa2_bag as dis;
use super::kafa2_bag::{alan_hz};
use super::kafa2_baglam::*;
use super::kafa2_dunya::{bek_yer_w, geri_cekil, predict_world};
use super::kafa2_imkan::{imk_sg, imk_tau, imk_uzak, imkan, sut_s, sut_tab, sut_vakit, SUR_BOS, SUR_PARCA};
fn sut_tab_k(k: &mut Kafa, w: &[Op], team: f64, x: f64, y: f64, ch: f64, sh: Option<&Oyuncu>) -> f64 {
    sut_tab(k, w, team, x, y, ch, sh, None, None, None, None)
}
use super::kafa2_zemin::{gam, zemin_s, KayipF};
use crate::mat::{cos, exp, hypot, jmax, jmin, jround};
use crate::model::{Dok, Oyuncu, Secenek};
use crate::saha::{CY as GCY, H as GH, W as GW};
use std::cell::RefCell;

use std::rc::Rc;
use std::sync::Arc;

pub const PNC_DILIM: f64 = 12.0;
pub const PNC_UFUK: f64 = 120.0;
pub const PNC_GORUS: f64 = 48.0;
/// Alıcının tek dokunuş şutunun değeri (ALAN_TEK_SUT_IZ): pas A'dan `tv` tikte vurulur, topu `ta` tikte alıcıya
/// B'de varır; dünya varış anında, Bekçi kararın dünyasındaki yerinden topun her tikteki yerine göre idealine doğru
/// kayar (vuruşa dek top A'da); şut topun varışında, hazırlıksız. Uzak geleceğe güven indirimiyle.
pub fn tek_sut_deger(k: &mut Kafa, cx: &mut Cx, A: Nokta, B: Nokta, tv: f64, ta: f64, rq: &Op) -> f64 {
    let T0 = jmax(1.0, ta);
    let w_t = p_dunya(k, cx, B, T0, "pass", Some(&**rq));
    let mut w_t: Vec<Op> = w_t.to_vec();
    if let Some(bi) = w_t.iter().position(|z| z.team != cx.team && bekci(z) && !z.noTouch) {
        if let Some(b0) = cx.src.iter().find(|z| z.team == w_t[bi].team && z.id == w_t[bi].id).cloned() {
            let vb = (0.17 + 0.006 * alan_hz(&b0)) * dis::alan_spre(k, &b0) * crate::temas::bek_kay(&b0);
            let mut bp = nk(b0.x, b0.y);
            let mut t = 1.0;
            let uc = jmax(1.0, T0 - tv);
            while t <= T0 {
                let f = jmax(0.0, jmin(1.0, (t - tv) / uc));
                let top = nk(A.x + (B.x - A.x) * f, A.y + (B.y - A.y) * f);
                let g = dis::as_bekci(k, &kxy(&b0, bp.x, bp.y), top, None);
                let (dx, dy) = (g.x - bp.x, g.y - bp.y);
                let L = hypot(dx, dy);
                if L > 1e-6 {
                    let m = jmin(L, vb);
                    bp = nk(bp.x + dx / L * m, bp.y + dy / L * m);
                }
                t += 1.0;
            }
            w_t[bi] = kxy(&b0, bp.x, bp.y);
        }
    }
    // Kuyu kuralı: varış anında kuyuda olamayacak savunmacı (süresini doldurmuş) kuyunun kenarına
    {
        let kr = k.dis.kural();
        let gx = if t0(cx.team) { GW } else { 0.0 };
        for i in 0..w_t.len() {
            let o = &w_t[i];
            if o.team == cx.team || bekci(o) || o.noTouch {
                continue;
            }
            let Some(o0) = cx.src.iter().find(|z| z.team == o.team && z.id == o.id) else { continue };
            if !kuyu_olabilir(o0, o.x, o.y, T0, &kr) {
                let (dx, dy) = (o.x - gx, o.y - GCY);
                let L = jmax(1e-6, hypot(dx, dy));
                let r = crate::core_sabit::Q.kuyu_a + 0.5;
                w_t[i] = kxy(o, gx + dx / L * r, GCY + dy / L * r);
            }
        }
    }
    if !yol_olabilir() {
        return sut_s(k, &w_t, cx.team, B.x, B.y, cx.ch, Some(rq), Some(T0)) * guven(k, T0);
    }
    // ALAN_YOL_OLABILIR_IZ: yakın rakip ve şut yolu da olabilirlikle: rakipler pas vurulunca (tv) tepki verir;
    // varışa (T0) dek şutçunun dibine (2.4 br) ya da şut yoluna yetişebilir mi (kuyu kuralıyla)
    let src = cx.src.clone();
    let kr = k.dis.kural();
    let mut fB = 1.0;
    let atama = if atama_acik() { Some(savunma_atama(k, &src, cx.team)) } else { None };
    for (pi, p) in src.iter().enumerate() {
        if p.team == cx.team || p.noTouch || bekci(p) {
            continue;
        }
        let pb: Op;
        let p = if let Some(at) = &atama {
            if let Some(hid) = at[pi] {
                if hid != rq.id {
                    continue;
                }
            }
            pb = gorev_yeri(k, &src, cx.team, p, at[pi], A, None, tv);
            &pb
        } else {
            p
        };
        let G = dis::as_gec_tab(k, p);
        let v = jmax(0.05, dis::as_v_sp(k, p));
        let rc = jmax(0.0, dis::alan_tepki(k, p, "kovala"));
        let d = jmax(0.0, hypot(p.x - B.x, p.y - B.y) - 2.4);
        let tp = tv + rc + dis::as_gec_of(k, &G, B.x, B.y, d) + crate::kural::enerji::alan_kosu_t(p, d, v, &kr);
        if !kuyu_olabilir(p, B.x, B.y, tp, &kr) {
            continue;
        }
        fB *= 1.0 - imk_sg((T0 - tp) / olab_tau(k, p, v, rc, T0));
    }
    let gx = if t0(cx.team) { GW } else { 0.0 };
    let mut yl: f64 = 0.0;
    for a in [0.0, 0.6, -0.6] {
        let G = nk(gx, GCY + a * crate::saha::MOUTH);
        yl = jmax(yl, yol_olabilir_b(k, &src, cx.team, B, G, T0, tv, None, Some(rq.id), false, false));
    }
    let tab = sut_tab_k(k, &w_t, cx.team, B.x, B.y, cx.ch, Some(rq));
    if crate::iz_acik() && eniyi_iz(&rq.govde.name) {
        eprintln!("ENIYI     tek parçalar {} ({:.1},{:.1}) tablo {:.3} yakın {:.3} yol {:.3} güven {:.3}", rq.govde.name, B.x, B.y, tab, fB, yl, guven(k, T0));
    }
    tab * fB * yl * guven(k, T0)
}

/// Deney (ALAN_BELIRSIZ_KAYNAK_IZ; ölçümde BELIRSIZ_KAYNAK_ZORLA): belirsizlik kaynağından: uzak geleceğe güven
/// indirimi yok (riskler adım adım zaten çarpılıyor), olabilirlikteki zaman payı zamanla büyüyen tablo yerine
/// rakibin tepki zarından ve görenin onu görme hatasından (kafa1_ucus'taki alıcı/rakip payıyla aynı biçim).
pub static BELIRSIZ_KAYNAK_ZORLA: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub fn belirsiz_kaynak() -> bool {
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    BELIRSIZ_KAYNAK_ZORLA.load(std::sync::atomic::Ordering::Relaxed) || *A.get_or_init(|| crate::iz_acik() && std::env::var("ALAN_BELIRSIZ_KAYNAK_IZ").is_ok())
}
/// Uzak geleceğe güven: bugün exp(−T / IMK_INAN_T); ALAN_BELIRSIZ_KAYNAK_IZ ile 1.
pub fn guven(k: &Kafa, T: f64) -> f64 {
    if belirsiz_kaynak() { 1.0 } else { exp(-T / k.k2.imk_inan_t) }
}
/// Olabilirlikte yetişme olasılığının zaman payı (tik): bugün imkTau(t); ALAN_BELIRSIZ_KAYNAK_IZ ile rakibin tepki
/// zarı (rc · tepki_z, düzgün zarın ölçünü) ve görme hatası (algi_e / hız).
pub fn olab_tau(k: &Kafa, p: &Oyuncu, v: f64, rc: f64, t: f64) -> f64 {
    if !belirsiz_kaynak() {
        return imk_tau(t);
    }
    let e = k.dis.algi_e(p).unwrap_or(0.0);
    hypot(1.5 + e / jmax(0.05, v) * 0.5, rc * crate::core_sabit::IN.tepki_z / 2.449 / 1.814)
}

/// ALAN_ENIYI_IZ=<ad> ya da * : enIyiPas dökümü
fn eniyi_iz(ad: &str) -> bool {
    match std::env::var("ALAN_ENIYI_IZ") {
        Ok(v) => v == "*" || v == ad,
        Err(_) => false,
    }
}
/// Savunmacının vuruş anındaki (şimdiden t0 tik sonra) görev yeri (ALAN_ATAMA_IZ): Bekçi topun yerine (A) göre
/// idealinde; bir hücumcuyu karşılıyorsa (hid) adamının (gönderense A'da, değilse niyetine t0 boyunca koşmuş)
/// kale tarafına tepkisinden sonra yetişebildiği kadar; boştaysa yerinde.
pub fn gorev_yeri(k: &mut Kafa, src: &[Op], team: f64, p: &Op, hid: Option<f64>, A: Nokta, gonderen: Option<f64>, t0: f64) -> Op {
    let gx = if p.team == 0.0 { 0.0 } else { GW };
    if bekci(p) {
        let g = dis::as_bekci(k, p, A, None);
        return kxy(p, g.x, g.y);
    }
    let Some(hid) = hid else { return p.clone() };
    let Some(h) = src.iter().find(|o| o.team == team && o.id == hid) else { return p.clone() };
    let (hx, hy) = if Some(hid) == gonderen {
        (A.x, A.y)
    } else {
        let vh = jmax(0.05, dis::as_v_sp(k, h));
        let (dx, dy) = (h.tx - h.x, h.ty - h.y);
        let Lh = hypot(dx, dy);
        let m = jmin(Lh, vh * t0);
        if Lh > 1e-6 { (h.x + dx / Lh * m, h.y + dy / Lh * m) } else { (h.x, h.y) }
    };
    let L = jmax(1e-6, hypot(gx - hx, GCY - hy));
    let (gx2, gy2) = (hx + (gx - hx) / L * 1.5, hy + (GCY - hy) / L * 1.5);
    let vp = jmax(0.05, dis::as_v_sp(k, p));
    let rcp = jmax(0.0, dis::alan_tepki(k, p, "kovala"));
    let (dx, dy) = (gx2 - p.x, gy2 - p.y);
    let Lp = hypot(dx, dy);
    let m = jmin(Lp, vp * jmax(0.0, t0 - rcp));
    if Lp > 1e-6 { kxy(p, p.x + dx / Lp * m, p.y + dy / Lp * m) } else { p.clone() }
}

/// Deney (ALAN_SAHSIZ_IZ; ölçümde SAHSIZ_ZORLA): sahiplik haritası değerlendirmeden tamamen çıkar: kaleye yakın
/// zemin yalnız imkân (varış yarışı), uzakta basit konum değeri (zemin: taşıma, ilerleme); budama da haritasız.
/// Harita yalnız seçenek adaylarının üretiminde kalır.
pub static SAHSIZ_ZORLA: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub fn sahsiz() -> bool {
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    SAHSIZ_ZORLA.load(std::sync::atomic::Ordering::Relaxed) || *A.get_or_init(|| crate::iz_acik() && std::env::var("ALAN_SAHSIZ_IZ").is_ok())
}
/// Deney (ALAN_BOLGE_IZ; ölçümde BOLGE_ZORLA): sahiplik haritası yerine bölge değeri haritası (kafa2_zemin::sahiplikBolge)
pub static BOLGE_ZORLA: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
/// Varsayılan açık (sitedeki kafa); kapatmak: BOLGE_KAPALI (wasm: ?bolge=0) ya da ALAN_BOLGE_ESKI=1.
pub static BOLGE_KAPALI: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub fn bolge() -> bool {
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    if BOLGE_KAPALI.load(std::sync::atomic::Ordering::Relaxed) {
        return false;
    }
    // sahiplik haritası (sürekli pas yolları) varsayılan olunca kapandı (20 maç: bölge orta sahada sahip olmanın değerini
    // görmüyor, uzak şut 67 / sahiplik + plan 30); açmak: ALAN_BOLGE_IZ (sitede ?bolge=1)
    BOLGE_ZORLA.load(std::sync::atomic::Ordering::Relaxed) || *A.get_or_init(|| crate::iz_acik() && std::env::var("ALAN_BOLGE_IZ").is_ok())
}
/// Deney (ALAN_KUYU_GEC_IZ; ölçümde KUYU_GEC_ZORLA): kafanın olabilirlik hesaplarında kuyu kuralı "t tik sonra
/// (x, y)'de olabilir mi" diye sorulur: savunmacı alana hemen girip beklemek zorunda değil, dışarıda bekleyip
/// zamanında girebilir (maçta alandan çıkınca sayaç sıfırlanır, mac/kuyu.rs). temas::kuyu_girer hemen girip
/// kalmayı tartar; uzak bir an için (pas sürmeden sonra gelince) alandaki herkesi dışarıda sayıyordu.
pub static KUYU_GEC_ZORLA: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub fn kuyu_gec() -> bool {
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    KUYU_GEC_ZORLA.load(std::sync::atomic::Ordering::Relaxed) || *A.get_or_init(|| crate::iz_acik() && std::env::var("ALAN_KUYU_GEC_IZ").is_ok())
}
/// Kafanın kuyu sorusu: p, t tik sonra (x, y)'de olabilir mi (kuyu_gec açıkken zamanında girişle).
pub fn kuyu_olabilir(p: &crate::model::Oyuncu, x: f64, y: f64, t: f64, kr: &dyn crate::temas::Kural) -> bool {
    use crate::temas::{kuyu_girer, kuyu_mer};
    if kuyu_girer(p, x, y, t, false, kr) {
        return true;
    }
    if !kuyu_gec() {
        return false;
    }
    let Q = &crate::core_sabit::Q;
    let v = jmax(0.05, p.taban_hiz() * kr.spr(p));
    let d_x = kuyu_mer(x, y);
    // kenardan noktaya içeride geçen süre
    let ic = jmax(0.0, Q.kuyu_a - d_x) / v;
    if ic > Q.kuyu_sn {
        return false;
    }
    let d_p = kuyu_mer(p.x, p.y);
    if d_p >= Q.kuyu_a {
        // dışarıda: girişini geciktirebilir
        return true;
    }
    // içeride: çıkıp (sayaç sıfırlanır) yeniden girmeye vakti var mı
    let cik = (Q.kuyu_a - d_p + 0.3) / v + 1.0 / (0.18 * crate::yetenek::ah(&p.govde.yet));
    cik + ic <= t
}
/// Deney (ALAN_GOVDE_YOL_IZ; ölçümde GOVDE_YOL_ZORLA): kafanın uçuşunda top yolundaki gövde (merkezi yola BODY_R'den
/// yakın; alıcı, Bekçi ve pası atan dışında, rakip ya da arkadaş) topa çarpar: tepkiden önce saf çarpma (sekme), sonra
/// topa ulaşmış sayılır (maçta temas::govde herkese uygulanır); eskiden yalnız adanmış ya da sendeleyen rakip için.
/// Kovalamayan savunmacı da (kafa onu o anki hızıyla yürütür) tepkisinden sonra yola koşabilir: görevi değişebilir.
pub static GOVDE_YOL_ZORLA: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
/// Yoldaki gövde ve kovalamayan savunmacının yola koşabilmesi. Varsayılan açık (sitedeki kafa); kapatmak: GOVDE_YOL_KAPALI (wasm: adres seçeneği) ya da ALAN_GOVDE_YOL_ESKI=1.
pub static GOVDE_YOL_KAPALI: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub fn govde_yol() -> bool {
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    if GOVDE_YOL_KAPALI.load(std::sync::atomic::Ordering::Relaxed) {
        return false;
    }
    GOVDE_YOL_ZORLA.load(std::sync::atomic::Ordering::Relaxed) || *A.get_or_init(|| std::env::var("ALAN_GOVDE_YOL_ESKI").is_err())
}
/// Deney (ALAN_SUR_E_IZ; ölçümde SUR_E_ZORLA): sürmenin tutma olasılığı değerin tartıldığı ara noktaya (toE) varış süresiyle;
/// eskiden sürmenin son hedefine (to) dek, değer ise toE'de tartılıyordu (k74-2536: kaleci 12–19. tiklerde yetişir sanıldı).
pub static SUR_E_ZORLA: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
/// Sürmenin tutma olasılığı ara noktaya dek. Varsayılan açık (sitedeki kafa); kapatmak: SUR_E_KAPALI (wasm: adres seçeneği) ya da ALAN_SUR_E_ESKI=1.
pub static SUR_E_KAPALI: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub fn sur_e() -> bool {
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    if SUR_E_KAPALI.load(std::sync::atomic::Ordering::Relaxed) {
        return false;
    }
    SUR_E_ZORLA.load(std::sync::atomic::Ordering::Relaxed) || *A.get_or_init(|| std::env::var("ALAN_SUR_E_ESKI").is_err())
}
thread_local! {
    static UFUK: std::cell::Cell<f64> = const { std::cell::Cell::new(12.0) };
}
/// Deney (ALAN_SUR_UFUK_IZ; ölçümde SUR_UFUK_ZORLA): sürme, taşıyıcının yeniden karar vereceği ana dek tartılır (tutma da değer
/// de oradaki ara noktada); uzun sürme bütün yolu boyunca, kısa hamle yalnız birkaç tik tartılıyordu (k85-1159: Cansu).
pub static SUR_UFUK_ZORLA: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
/// Sürme yeniden karar anına dek tartılır. Varsayılan açık (sitedeki kafa); kapatmak: SUR_UFUK_KAPALI (wasm: adres seçeneği) ya da ALAN_SUR_UFUK_ESKI=1.
pub static SUR_UFUK_KAPALI: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub fn sur_ufuk() -> bool {
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    if SUR_UFUK_KAPALI.load(std::sync::atomic::Ordering::Relaxed) {
        return false;
    }
    SUR_UFUK_ZORLA.load(std::sync::atomic::Ordering::Relaxed) || *A.get_or_init(|| std::env::var("ALAN_SUR_UFUK_ESKI").is_err())
}
/// Deney (ALAN_SAH_YUMUSAK_IZ; ölçümde SAH_YUMUSAK_ZORLA): sahiplik haritasında hücrenin değeri en iyi tek yoldan değil, en iyi
/// üç yolun ortalamasından (bir yol kapanınca değer çökmez; tek kaynağın değeri zincir boyunca söner).
pub static SAH_YUMUSAK_ZORLA: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
/// Sahiplik haritasında en iyi üç yolun ortalaması. Varsayılan açık (sitedeki kafa); kapatmak: SAH_YUMUSAK_KAPALI (wasm: adres seçeneği) ya da ALAN_SAH_YUMUSAK_ESKI=1.
pub static SAH_YUMUSAK_KAPALI: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub fn sah_yumusak() -> bool {
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    if SAH_YUMUSAK_KAPALI.load(std::sync::atomic::Ordering::Relaxed) {
        return false;
    }
    // bölge haritası varsayılan olunca kapandı (20 maç: tek iyi yolu sulandırıp uzak şut getiriyordu); açmak: ALAN_SAH_YUMUSAK_IZ
    SAH_YUMUSAK_ZORLA.load(std::sync::atomic::Ordering::Relaxed) || *A.get_or_init(|| crate::iz_acik() && std::env::var("ALAN_SAH_YUMUSAK_IZ").is_ok())
}
/// Deney (ALAN_SAH_BEKLENEN_IZ; ölçümde SAH_BEKLENEN_ZORLA): sahiplik haritasında hücrenin değeri taşıyıcının yollar arasından
/// gürültülü seçiminin beklenen değeri (yumuşak bölge değerinin yerine: tek iyi yolu sulandırmaz, bir yol kapanınca çökmez).
pub static SAH_BEKLENEN_ZORLA: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub fn sah_beklenen() -> bool {
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    SAH_BEKLENEN_ZORLA.load(std::sync::atomic::Ordering::Relaxed) || *A.get_or_init(|| crate::iz_acik() && std::env::var("ALAN_SAH_BEKLENEN_IZ").is_ok())
}
/// Deney (ALAN_IKI_KADEME_IZ; ölçümde IKI_KADEME_ZORLA): kaba değeri seçilemeyecek kadar geride kalan pas adayı tam tartılmaz.
pub static IKI_KADEME_ZORLA: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub fn iki_kademe() -> bool {
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    IKI_KADEME_ZORLA.load(std::sync::atomic::Ordering::Relaxed) || *A.get_or_init(|| crate::iz_acik() && std::env::var("ALAN_IKI_KADEME_IZ").is_ok())
}
/// Deney (ALAN_SAH_SUREKLI_IZ; ölçümde SAH_SUREKLI_ZORLA): sahiplik haritasında pas yolu zaman payıyla sürekli (kafa2_zemin::yol_sure).
pub static SAH_SUREKLI_ZORLA: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
/// Sahiplik haritasında pas yolu zaman payıyla sürekli. Varsayılan açık (sitedeki kafa); kapatmak: SAH_SUREKLI_KAPALI (wasm: adres seçeneği) ya da ALAN_SAH_SUREKLI_ESKI=1.
pub static SAH_SUREKLI_KAPALI: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub fn sah_surekli() -> bool {
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    if SAH_SUREKLI_KAPALI.load(std::sync::atomic::Ordering::Relaxed) {
        return false;
    }
    SAH_SUREKLI_ZORLA.load(std::sync::atomic::Ordering::Relaxed) || *A.get_or_init(|| std::env::var("ALAN_SAH_SUREKLI_ESKI").is_err())
}
/// Deney (ALAN_SEKME_BEDEL_IZ; ölçümde SEKME_BEDEL_ZORLA): şut yalnız gol olasılığıyla, pas yalnız alıcıya doğrudan varma
/// olasılığıyla değer kazanır; gol olmayan şut ve alıcının almadığı pas payı (sekip bize dönse de) kaybedilmiş gibi bedel öder.
pub static SEKME_BEDEL_ZORLA: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub fn sekme_bedel() -> bool {
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    SEKME_BEDEL_ZORLA.load(std::sync::atomic::Ordering::Relaxed) || *A.get_or_init(|| crate::iz_acik() && std::env::var("ALAN_SEKME_BEDEL_IZ").is_ok())
}
/// Deney (ALAN_KAYIP_SAHIP_IZ; ölçümde KAYIP_SAHIP_ZORLA): topu kaybetmenin bedeline elimizdeki sahipliğin değeri de
/// girer. Sahiplik değeri V0 = topu tutan seçeneklerin (şut dışı) en iyisinin değeri; her seçenek, topun takımda kalmama
/// olasılığı kadar V0 öder (şut: 1 − gol, pas: 1 − alıcıya doğrudan varma, sür/tut: 1 − tutma). Tutma şansı yoksa
/// (V0 ≈ 0) bedel kalkar, şut çekilir.
/// Varsayılan açık (20 maç: uzak şut 29 → 10, gol 10 → 9); kapatmak: KAYIP_SAHIP_KAPALI (wasm: ?kayipsahip=0) ya da ALAN_KAYIP_SAHIP_ESKI=1.
pub static KAYIP_SAHIP_ZORLA: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub static KAYIP_SAHIP_KAPALI: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub fn kayip_sahip() -> bool {
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    if KAYIP_SAHIP_KAPALI.load(std::sync::atomic::Ordering::Relaxed) {
        return false;
    }
    KAYIP_SAHIP_ZORLA.load(std::sync::atomic::Ordering::Relaxed) || *A.get_or_init(|| std::env::var("ALAN_KAYIP_SAHIP_ESKI").is_err())
}
/// Deney (ALAN_SAH_BELIRSIZ_IZ; ölçümde SAH_BELIRSIZ_ZORLA): sahiplik haritası hücrenin şutunu topun oraya varış
/// süresiyle (Tc) değerlendirir: savunmacıların o andaki yeri belirsiz (imk_sap), engel keskin eşik değil olasılık.
pub static SAH_BELIRSIZ_ZORLA: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub fn sah_belirsiz() -> bool {
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    SAH_BELIRSIZ_ZORLA.load(std::sync::atomic::Ordering::Relaxed) || *A.get_or_init(|| crate::iz_acik() && std::env::var("ALAN_SAH_BELIRSIZ_IZ").is_ok())
}
/// Deney (ALAN_KOVSUZ_SABIT_IZ; ölçümde KOVSUZ_SABIT_ZORLA): uçuşta kovalamayan savunmacının tepkiden sonra topun
/// yoluna tam hızla koşabileceği varsayımı kalkar (görevindeki yerine göre).
/// Varsayılan açık; kapatmak: KOVSUZ_SABIT_KAPALI (wasm: ?kovsuz=0) ya da ALAN_KOVSUZ_SABIT_ESKI=1.
pub static KOVSUZ_SABIT_ZORLA: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub static KOVSUZ_SABIT_KAPALI: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub fn kovsuz_sabit() -> bool {
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    if KOVSUZ_SABIT_KAPALI.load(std::sync::atomic::Ordering::Relaxed) {
        return false;
    }
    KOVSUZ_SABIT_ZORLA.load(std::sync::atomic::Ordering::Relaxed) || *A.get_or_init(|| std::env::var("ALAN_KOVSUZ_SABIT_ESKI").is_err())
}
/// Deney (ALAN_ON_SIRA_IZ; ölçümde ON_SIRA_ZORLA): pas/sürme adaylarının ön elemesi işarete değil sıraya bakar
/// (en iyi adaylar eksi değerde de tartıya girer; "tut"a karşı karşılaştırmayı tartı yapar).
pub static ON_SIRA_ZORLA: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub fn on_sira() -> bool {
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    ON_SIRA_ZORLA.load(std::sync::atomic::Ordering::Relaxed) || *A.get_or_init(|| crate::iz_acik() && std::env::var("ALAN_ON_SIRA_IZ").is_ok())
}
/// Deney (ALAN_SAH_YAKINSA_IZ; ölçümde SAH_YAKINSA_ZORLA): sahiplik haritası sabit SAH_ADIM yerine değerler
/// değişmeyene kadar yayılır (değer yinelemesinin sabit noktası).
pub static SAH_YAKINSA_ZORLA: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub fn sah_yakinsa() -> bool {
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    SAH_YAKINSA_ZORLA.load(std::sync::atomic::Ordering::Relaxed) || *A.get_or_init(|| crate::iz_acik() && std::env::var("ALAN_SAH_YAKINSA_IZ").is_ok())
}
/// Deney (ALAN_SAVUR_TEPKI_IZ; ölçümde SAVUR_TEPKI_ZORLA): savurma yolu denetiminde (kafa ve maç) yakındaki rakip
/// Çekirdeğe tepki süresinden sonra koşar; o süre boyunca kendi hızıyla sürer (eskisi: hemen, tam hızla).
pub static SAVUR_TEPKI_ZORLA: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub fn savur_tepki() -> bool {
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    SAVUR_TEPKI_ZORLA.load(std::sync::atomic::Ordering::Relaxed) || *A.get_or_init(|| crate::iz_acik() && std::env::var("ALAN_SAVUR_TEPKI_IZ").is_ok())
}
/// Deney (ALAN_IKI_HAMLE_IZ; ölçümde IKI_HAMLE_ZORLA): kafanın planı en çok iki top hareketi (kendi hamlesi + topu
/// alanın bir hamlesi); sonrası alıcının durumu (şut, tek dokunuş, zemin, sürme), gelecek kararlar sıralanmaz.
pub static IKI_HAMLE_ZORLA: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub fn iki_hamle() -> bool {
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    IKI_HAMLE_ZORLA.load(std::sync::atomic::Ordering::Relaxed) || *A.get_or_init(|| crate::iz_acik() && std::env::var("ALAN_IKI_HAMLE_IZ").is_ok())
}
/// Deney (ALAN_SECIM_BEKLENEN_IZ; ölçümde SECIM_BEKLENEN_ZORLA): gelecekteki kararın değeri, maçtaki gürültülü
/// seçimle (her adaya genişliği `w` üçgen gürültü, en yükseği seçilir) seçilenin beklenen gerçek değeri.
pub static SECIM_BEKLENEN_ZORLA: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub fn secim_beklenen() -> bool {
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    SECIM_BEKLENEN_ZORLA.load(std::sync::atomic::Ordering::Relaxed) || *A.get_or_init(|| crate::iz_acik() && std::env::var("ALAN_SECIM_BEKLENEN_IZ").is_ok())
}
/// Adayların değerleri `c`, gürültü genişliği `w`: seçilenin beklenen değeri (sabit tohumla 32 örnek, zar dizisi değişmez).
pub fn secimBeklenen(c: &[f64], w: f64, tohum: f64) -> f64 {
    if c.is_empty() {
        return 0.0;
    }
    if !(w > 0.0) {
        return c.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    }
    let n = 32;
    let mut top = 0.0;
    for j in 0..n {
        let mut bi = 0;
        let mut bv = f64::NEG_INFINITY;
        for (i, &v) in c.iter().enumerate() {
            let z = crate::yetenek::zar(tohum, j as f64, i as f64 * 2.0 + 1.0) + crate::yetenek::zar(tohum, j as f64, i as f64 * 2.0 + 2.0) - 1.0;
            if v + w * z > bv {
                bv = v + w * z;
                bi = i;
            }
        }
        top += c[bi];
    }
    top / n as f64
}

/// Deney (ALAN_ATAMA_IZ; ölçümde ATAMA_ZORLA): olabilirlikte bir savunmacı iki yerde olamaz: savunmacılar şimdiden
/// (topu ve oyuncuları görerek) hareket eder, ama her biri bir hücumcuyu karşılar (en tehlikeli hücumcuya en erken
/// yetişen); bir yolu yalnız boşta kalanlar, Bekçi, gönderenin ve alıcının karşılayıcısı kesebilir.
pub static ATAMA_ZORLA: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub fn atama_acik() -> bool {
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    ATAMA_ZORLA.load(std::sync::atomic::Ordering::Relaxed) || *A.get_or_init(|| crate::iz_acik() && std::env::var("ALAN_ATAMA_IZ").is_ok())
}
/// Savunmacı (src sırası) → karşıladığı hücumcunun kimliği (None: boşta). Hücumcular tehlikesine göre sırayla en erken
/// yetişen boştaki savunmacıyı alır (imkân haritasının açgözlü eşlemesi gibi).
pub fn savunma_atama(k: &mut Kafa, src: &[Op], team: f64) -> Vec<Option<f64>> {
    let kr = k.dis.kural();
    let mut atk: Vec<(f64, usize)> = src.iter().enumerate().filter(|(_, p)| p.team == team && !bekci(p) && !p.noTouch).map(|(i, p)| (crate::kafa::kafa1_tehlike::threat_d(&k.D, team, p.x, p.y), i)).collect();
    atk.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
    let mut out: Vec<Option<f64>> = vec![None; src.len()];
    let mut bos: Vec<usize> = (0..src.len()).filter(|&i| src[i].team != team && !bekci(&src[i]) && !src[i].noTouch).collect();
    for (_, ai) in atk {
        let a = &src[ai];
        let mut en: Option<(usize, f64)> = None;
        for (bi, &di) in bos.iter().enumerate() {
            let p = &src[di];
            let G = dis::as_gec_tab(k, p);
            let v = jmax(0.05, dis::as_v_sp(k, p));
            let d = jmax(0.0, hypot(p.x - a.x, p.y - a.y) - 2.0);
            let t = jmax(0.0, dis::alan_tepki(k, p, "kovala")) + dis::as_gec_of(k, &G, a.x, a.y, d) + crate::kural::enerji::alan_kosu_t(p, d, v, &kr);
            if en.map_or(true, |e| t < e.1) {
                en = Some((bi, t));
            }
        }
        if let Some((bi, _)) = en {
            let di = bos.remove(bi);
            out[di] = Some(a.id);
            if crate::iz_acik() && std::env::var("ALAN_ATAMA_DOK").is_ok() {
                eprintln!("ATAMA {} {} ({:.0},{:.0}) ← {} ({:.0},{:.0})", k.tick(), a.govde.name, a.x, a.y, src[di].govde.name, src[di].x, src[di].y);
            }
        }
    }
    out
}

/// Deney (ALAN_YOL_OLABILIR_IZ; ölçümde YOL_OLABILIR_ZORLA)
pub static YOL_OLABILIR_ZORLA: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub fn yol_olabilir() -> bool {
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    YOL_OLABILIR_ZORLA.load(std::sync::atomic::Ordering::Relaxed) || *A.get_or_init(|| crate::iz_acik() && std::env::var("ALAN_YOL_OLABILIR_IZ").is_ok())
}

/// A'dan B'ye, şimdiden t0 tik sonra vurulan pasın yolunun açık olma olasılığı, olabilirlikle: her rakip (Bekçi
/// dahil) pas vurulunca (t0) tepki verir, kararın dünyasındaki yerinden yolun her noktasına en erken ne zaman yetişebilir
/// (tepki + dönüş gecikmesi + koşu, menzil kadar yakına) ve top o noktaya ne zaman varır; alıcı o noktaya daha önce
/// gelebiliyorsa top onundur. Yetişme olasılığı zamanla büyüyen belirsizlikle; rakip başına yolun en kötü noktası.
pub fn yolOlabilir(k: &mut Kafa, src: &[Op], team: f64, A: Nokta, B: Nokta, t0: f64, alici: Option<&Oyuncu>, gonderen: Option<f64>, iz: bool) -> f64 {
    yol_olabilir_b(k, src, team, A, B, t0, t0, alici, gonderen, true, iz)
}
/// yolOlabilir'in genişi: top A'dan `t0`'da çıkar, rakipler `tt`'de tepki vermeye başlar; `bek`: Bekçi de sayılır mı
/// (şut yolunda Bekçi şut tablosunda zaten var: sayılmaz).
pub fn yol_olabilir_b(k: &mut Kafa, src: &[Op], team: f64, A: Nokta, B: Nokta, t0: f64, tt: f64, alici: Option<&Oyuncu>, gonderen: Option<f64>, bek_dahil: bool, iz: bool) -> f64 {
    let L = hypot(B.x - A.x, B.y - A.y);
    if L < 0.5 {
        return 1.0;
    }
    let atama = if atama_acik() { Some(savunma_atama(k, src, team)) } else { None };
    let alici_id = alici.map(|q| q.id);
    let kr = k.dis.kural();
    // alıcı yolun bir noktasına topla buluşmaya gelebilir: savunmacı oradan ancak alıcıdan önce varırsa keser
    let al = alici.map(|q| {
        let q = src.iter().find(|o| o.team == q.team && o.id == q.id).map(|o| (**o).clone()).unwrap_or_else(|| q.clone());
        let G = dis::as_gec_tab(k, &q);
        let v = jmax(0.05, dis::as_v_sp(k, &q));
        let R = jmin(2.0, crate::temas::menzil(&q, 0.35));
        (q, G, v, R)
    });
    let mut acik = 1.0;
    for p in src.iter() {
        if p.team == team || p.noTouch || (!bek_dahil && bekci(p)) {
            continue;
        }
        if let Some(at) = &atama {
            let pi = src.iter().position(|o| std::ptr::eq(&**o, &**p)).unwrap_or(0);
            if let Some(hid) = at[pi] {
                if Some(hid) != alici_id && Some(hid) != gonderen {
                    continue; // başka bir hücumcuyu karşılıyor
                }
            }
        }
        // vuruş anındaki görev yeri (ALAN_ATAMA_IZ): karşılayıcı adamının kale tarafında, Bekçi topun o anki yerine
        // (A) göre idealinde, boştaki yerinde; tepki vuruştan (tt) başlar
        let pb: Op = match &atama {
            Some(at) => {
                let pi = src.iter().position(|o| std::ptr::eq(&**o, &**p)).unwrap_or(0);
                gorev_yeri(k, src, team, p, at[pi], A, gonderen, t0)
            }
            None => p.clone(),
        };
        let p = &pb;
        let G = dis::as_gec_tab(k, p);
        let v = jmax(0.05, dis::as_v_sp(k, p));
        let bek = bekci(p);
        let R = if bek { crate::core_sabit::Q.bek_body + crate::temas::bek_dl(p) } else { jmin(2.0, crate::temas::menzil(p, 0.35)) };
        let rc = jmax(0.0, if bek { crate::temas::bek_re(p) } else { dis::alan_tepki(k, p, "kovala") });
        // koşu profilleri (kosuPr) yol boyunca sabit: bir kez
        let pr_p = crate::kural::enerji::kosuPr(p, &kr);
        let pr_q = al.as_ref().and_then(|a| crate::kural::enerji::kosuPr(&a.0, &kr));
        let mut en = 0.0f64;
        let mut s = 1.5;
        while s < L - 1.0 {
            let x = A.x + (B.x - A.x) * s / L;
            let y = A.y + (B.y - A.y) * s / L;
            let tb = t0 + dis::ucus_t(k, L, k.D.arriveFirm, Some(s));
            let d = jmax(0.0, hypot(p.x - x, p.y - y) - R);
            // savunmacı pası ancak vurulunca görür: koşusu vuruştan (t0) başlar; o ana dek yeri bilinmez (belirsizlik payı)
            let tp = tt + rc + dis::as_gec_of(k, &G, x, y, d) + crate::kural::enerji::alan_kosu_tp(pr_p, d, v);
            let tq = match &al {
                Some((q, Gq, vq, Rq)) => {
                    let dq = jmax(0.0, hypot(q.x - x, q.y - y) - Rq);
                    dis::as_gec_of(k, Gq, x, y, dq) + crate::kural::enerji::alan_kosu_tp(pr_q, dq, *vq)
                }
                None => f64::INFINITY,
            };
            // Kuyu kuralı (temas::kuyu_girer): kuyu süresini doldurmuş savunmacı o noktaya giremez
            if !bek && !kuyu_olabilir(p, x, y, tp, &kr) {
                s += 1.5;
                continue;
            }
            let pk = imk_sg((jmin(tb, tq) - tp) / olab_tau(k, p, v, rc, tb));
            if pk > en {
                en = pk;
            }
            s += 1.5;
        }
        if iz && en > 0.05 {
            eprintln!("ENIYI     olabilir {} ({:.1},{:.1}) kesme {:.3}", p.govde.name, p.x, p.y, en);
        }
        acik *= 1.0 - en;
    }
    acik
}

/// Deney (ALAN_ZEMIN_YARIS_IZ, ölçümde ZEMIN_YARIS_ZORLA)
pub static ZEMIN_YARIS_ZORLA: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub fn zemin_yaris() -> bool {
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    ZEMIN_YARIS_ZORLA.load(std::sync::atomic::Ordering::Relaxed) || *A.get_or_init(|| crate::iz_acik() && std::env::var("ALAN_ZEMIN_YARIS_IZ").is_ok())
}
/// Deney (ALAN_TEK_SUT_IZ, ölçümde TEK_SUT_ZORLA): pasın değerinde alıcının tek dokunuş şutu.
pub static TEK_SUT_ZORLA: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub fn tek_sut() -> bool {
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    TEK_SUT_ZORLA.load(std::sync::atomic::Ordering::Relaxed) || *A.get_or_init(|| crate::iz_acik() && std::env::var("ALAN_TEK_SUT_IZ").is_ok())
}
/// Deney (ALAN_GORUS_IZ, ölçümde GORUS_ZORLA): görüş ufku (PNC_GORUS) her alıcının topu alacağı andan sayılır;
/// eskisi kararın anından sayar, uzun pasın alıcısının sonraki hamlesi hiç görülmez (belirsizlik zaten
/// exp(-T/IMK_INAN_T) indirimiyle tartılıyor).
pub static GORUS_ZORLA: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub fn gorus_goreli() -> bool {
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    GORUS_ZORLA.load(std::sync::atomic::Ordering::Relaxed) || *A.get_or_init(|| crate::iz_acik() && std::env::var("ALAN_GORUS_IZ").is_ok())
}

pub type KayipRef = Rc<RefCell<KayipF>>;
/// JS `lossAt(x, y)` / `kayip(x, y)`
#[inline]
pub fn kay(k: &mut Kafa, f: &KayipRef, x: f64, y: f64) -> f64 {
    f.borrow_mut().deger(k, x, y)
}


/// Prensip tartısının bağlamı (JS cx: { src, team, ch, kayip, h, P, IM })
pub struct Cx {
    pub src: Arc<Dunya>,
    pub team: f64,
    pub ch: f64,
    pub kayip: KayipRef,
    pub h: Op,
    /// pDunya önbelleği ("w|…")
    pub PW: crate::HizliMap<String, Arc<Dunya>>,
    /// pencere önbelleği
    pub PE: crate::HizliMap<String, Arc<Pencere>>,
    /// nodeV önbelleği ("v|…")
    pub PV: crate::HizliMap<String, f64>,
    /// durumD önbelleği ("d|…")
    pub PD: crate::HizliMap<String, Arc<DurumR>>,
    /// zeminH'nin imkân önbelleği (JS cx.IM; cxSiz kopyaları varsa paylaşır)
    pub IM: Option<Rc<RefCell<crate::HizliMap<String, f64>>>>,
}

/// JS pencere'nin girdisi
#[derive(Debug)]
pub struct Pencere {
    pub k: Op,
    pub B: Nokta,
    pub W: Arc<Dunya>,
    pub tb: f64,
    pub k_a: f64,
    pub sut: f64,
    pub zem: f64,
    pub sur: f64,
    pub b: f64,
}

/// JS durumD'nin sonucu
#[derive(Debug, Clone)]
pub struct DurumR {
    pub v: f64,
    pub sut: f64,
    pub zem: f64,
    pub pas: f64,
    pub sur: f64,
    pub pas_k: Option<String>,
}

/// JS sayısının yazısı (tam sayılar ".0"sız)
pub fn js_sayi(x: f64) -> String {
    if x == x.trunc() && x.abs() < 1e15 {
        format!("{}", x as i64)
    } else {
        format!("{}", x)
    }
}
/// JS `x.toFixed(n)`
pub fn to_fixed(x: f64, n: usize) -> String {
    format!("{:.*}", n, x)
}

/// JS pDunya(cx, B, T, kind, q) (decide.js:6932): top B'deyken T tik sonraki dünya (en yakın cevap
/// veren tek rakip cevabıyla, ötekiler öngörüyle; alıcı B'de).
pub fn p_dunya(k: &mut Kafa, cx: &mut Cx, B: Nokta, T: f64, kind: &str, q: Option<&Oyuncu>) -> Arc<Dunya> {
    let mut key = format!("w|{}|{}|{}|{}|{}", kind, to_fixed(B.x, 2), to_fixed(B.y, 2), js_sayi(jround(T)), q.map(|q| js_sayi(q.id)).unwrap_or_default());
    let (B, T) = if pr_sirasiz() {
        if kind == "drive" {
            if let Some(q) = q {
                key.push_str(&format!("|{:x}|{:x}", q.x.to_bits(), q.y.to_bits()));
            }
        }
        (nk(iki_bas(B.x), iki_bas(B.y)), jround(T))
    } else {
        (B, T)
    };
    if let Some(w) = cx.PW.get(&key) {
        return w.clone();
    }
    let src = cx.src.clone();
    let team = cx.team;
    let W: Arc<Dunya> = if T > 0.0 {
        let (rw, degisti) = dis::respond_d(k, &src, team, B, T, kind);
        let resp: Vec<usize> = (0..degisti.len()).filter(|&i| degisti[i]).collect();
        let tas = if kind == "drive" { q.map(|q| nk(q.x, q.y)) } else { None };
        let Pw = predict_world(k, &src, team, B, T, true, Some(&resp), false, tas);
        let mut tek_c: isize = -1;
        let mut bd = 1e9;
        for &i in &resp {
            let d = hypot(rw[i].x - B.x, rw[i].y - B.y);
            if d < bd {
                bd = d;
                tek_c = i as isize;
            }
        }
        Arc::new(
            (0..src.len())
                .map(|i| {
                    let p = &src[i];
                    if degisti[i] && (tek_c < 0 || i as isize == tek_c) {
                        let mut c = (*rw[i]).clone();
                        c.vx = 0.0;
                        c.vy = 0.0;
                        return Arc::new(c);
                    }
                    match q {
                        Some(q) if p.id == q.id && p.team == q.team => kxyv(p, B.x, B.y, 0.0, 0.0),
                        _ => Pw[i].clone(),
                    }
                })
                .collect(),
        )
    } else {
        match q {
            Some(q) => Arc::new(src.iter().map(|p| if p.id == q.id && p.team == q.team { kxyv(p, B.x, B.y, 0.0, 0.0) } else { p.clone() }).collect()),
            None => src.clone(),
        }
    };
    cx.PW.insert(key, W.clone());
    W
}

/// JS zeminH(w, team, x, y, ch, k, cx, T) (decide.js:6748): zemin ile imkânın büyüğü (imkân T tik
/// sonrası için; cx varsa önbellekli).
pub fn zemin_h(k: &mut Kafa, w: &[Op], team: f64, x: f64, y: f64, ch: f64, kq: Option<&Oyuncu>, cx: Option<&mut Cx>, T: Option<f64>) -> f64 {
    if imk_uzak(team, x, y) {
        return zemin_s(k, w, team, x, y, ch, kq);
    }
    let cx = match cx {
        None => {
            let a = imkan(k, w, team, nk(x, y), kq, ch);
            let b = if zemin_yaris() || sahsiz() { 0.0 } else { zemin_s(k, w, team, x, y, ch, kq) };
            return jmax(a, b);
        }
        Some(c) => c,
    };
    let M = cx.IM.get_or_insert_with(|| Rc::new(RefCell::new(Default::default()))).clone();
    let mut key = format!(
        "{}|{}|{}|{}|{}",
        js_sayi(team),
        kq.map(|q| js_sayi(q.id)).unwrap_or_else(|| "-".into()),
        js_sayi(jround(x / 3.0)),
        js_sayi(jround(y / 3.0)),
        js_sayi(jround(veyo(T, 0.0) / PNC_DILIM))
    );
    let (x, y, T) = if pr_sirasiz() {
        key.push_str(&format!("|{:x}", dunya_oz(w)));
        (jround(x / 3.0) * 3.0, jround(y / 3.0) * 3.0, T.map(|t| jround(t / PNC_DILIM) * PNC_DILIM))
    } else {
        (x, y, T)
    };
    if let Some(v) = M.borrow().get(&key) {
        if crate::iz_acik() && kq.map_or(false, |q| std::env::var("ALAN_SURD_IZ").ok().as_deref() == Some(&*q.govde.name)) {
            eprintln!("ZEMIN ({:.1},{:.1}) T {:.0} önbellek {:.3} anahtar {}", x, y, veyo(T, 0.0), v, key);
        }
        return *v;
    }
    let o0 = k.k2.imk_ofs;
    k.k2.imk_ofs = veyo(T, 0.0);
    let v = imkan(k, w, team, nk(x, y), kq, ch);
    k.k2.imk_ofs = o0;
    // ALAN_ZEMIN_YARIS_IZ: kaleye yakın bölgede zemin yalnız imkân (varış yarışı); sahiplik haritasının
    // donuk dünyalı, en iyisini yayan değeri katılmaz
    let zs = if zemin_yaris() || sahsiz() { 0.0 } else { zemin_s(k, w, team, x, y, ch, kq) };
    if crate::iz_acik() && kq.map_or(false, |q| std::env::var("ALAN_SURD_IZ").ok().as_deref() == Some(&*q.govde.name)) {
        eprintln!("ZEMIN ({:.1},{:.1}) T {:.0} imkan {:.3} zeminS {:.3}", x, y, veyo(T, 0.0), v, zs);
    }
    let v = jmax(v, zs);
    M.borrow_mut().insert(key, v);
    v
}

/// JS surD(w, team, k, at, ch, kayip, cx, T0) (decide.js:6811): at'tan kaleye ya da ileri sürmenin
/// değeri (8 br'lik parçalarla; her parçada tutma olasılığı, geri çekilen savunma; sonunda şut ya da pas).
pub fn sur_d(k: &mut Kafa, w: &[Op], team: f64, kq: &Oyuncu, at: Nokta, ch: f64, kayip: &KayipRef, mut cx: Option<&mut Cx>, T0: Option<f64>) -> f64 {
    let gx = if t0(team) { GW } else { 0.0 };
    let dx = gx - at.x;
    let dy = GCY - at.y;
    let dg = veya(hypot(dx, dy), 1.0);
    let ug = nk(dx / dg, dy / dg);
    let uf = nk(if t0(team) { 1.0 } else { -1.0 }, 0.0);
    let mut dirs = vec![(ug, true)];
    let v_k = (0.17 + 0.006 * alan_hz(kq)) * dis::top_k(k, kq);
    if ug.x * uf.x < cos(0.35) {
        dirs.push((uf, false));
    }
    let mut best = -1e9;
    let T0v = veyo(T0, 0.0);
    // ALAN_SURD_IZ=<ad>: o oyuncunun sürme değeri hesabı (parça parça), stderr'e
    let iz_ad: Option<String> = if crate::iz_acik() { std::env::var("ALAN_SURD_IZ").ok() } else { None };
    for (u, gol) in dirs {
        let mut lm = SUR_BOS;
        lm = if gol { jmin(lm, dg - 1.5) } else { jmin(lm, if t0(team) { GW - 2.0 - at.x } else { at.x - 2.0 }) };
        if lm < 2.0 {
            continue;
        }
        let mut pos = at;
        let mut ww: Dunya = crate::dunya::Dunya::from(w.to_vec());
        let mut Pc = 1.0;
        let mut T = 0.0;
        let mut yol = 0.0;
        let mut kayv = 0.0;
        while yol < lm - 0.5 && Pc > 0.05 {
            let seg = jmin(SUR_PARCA, lm - yol);
            let mut r = kq.clone();
            r.x = pos.x;
            r.y = pos.y;
            r.vx = if yol > 0.0 { u.x * v_k } else { 0.0 };
            r.vy = if yol > 0.0 { u.y * v_k } else { 0.0 };
            let ts0 = jmax(4.0, dis::sur_sure(k, &r, seg, Some(u), None));
            let p = crate::kafa::kafa1_ikili::kp_kal(jmax(0.0, jmin(1.0, dis::keep_p(k, &r, &ww, Some(u), ts0))));
            let pr = k.keep_p_son.get();
            let adv = if pr < seg { jmax(0.0, pr) } else { seg };
            if iz_ad.as_deref() == Some(&*kq.govde.name) {
                eprintln!("SURD   parça ({:.1},{:.1}) seg {:.1} Ts {:.0} p {:.3} ilerleme {:.1} Pc {:.3}", pos.x, pos.y, seg, ts0, p, adv, Pc * p);
            }
            kayv += Pc * (1.0 - p) * kay(k, kayip, pos.x + u.x * adv / 2.0, pos.y + u.y * adv / 2.0);
            Pc *= p;
            if adv < 1.0 {
                break;
            }
            let E = nk(pos.x + u.x * adv, pos.y + u.y * adv);
            let Ts = if adv < seg { jmax(4.0, dis::sur_sure(k, &r, adv, Some(u), None)) } else { ts0 };
            ww = geri_cekil(k, &ww, team, pos, E, Ts, yol == 0.0);
            T += Ts;
            yol += adv;
            pos = E;
            if adv < seg - 0.5 {
                break;
            }
        }
        if yol < 1.5 {
            continue;
        }
        let w_e = bek_yer_w(k, &ww, team, pos, None);
        let s_e = sut_s(k, &w_e, team, pos.x, pos.y, ch, Some(kq), Some(T0v + T)) * sut_vakit(k, &w_e, team, pos.x, pos.y, Some(kq));
        let sonra = match cx.as_deref_mut() {
            Some(c) => gam(k) * en_iyi_pas(k, c, &w_e, pos, T0v + T, kq.id, 0.0).0,
            None => 0.0,
        };
        let v = Pc * jmax(jmax(s_e, 0.0), sonra) - kayv;
        if iz_ad.as_deref() == Some(&*kq.govde.name) {
            eprintln!("SURD {} {} ({:.1},{:.1})->({:.1},{:.1}) gol {} T0 {:.0} T {:.0} Pc {:.3} şut {:.3} sonra {:.3} kayıp {:.3} v {:.3}", k.tick(), kq.govde.name, at.x, at.y, pos.x, pos.y, gol, T0v, T, Pc, s_e, sonra, kayv, v);
        }
        if v > best {
            best = v;
        }
    }
    if best > -1e9 {
        best
    } else {
        0.0
    }
}

/// JS pencere(cx, k, T, R) (decide.js:6987): alıcı k topu T tik sonra (R'de ya da öngörülen yerinde)
/// alırsa: dünya, tutma olasılığı, şut / zemin / sürme değeri.
pub fn pencere(k: &mut Kafa, cx: &mut Cx, kq: &Op, T: f64, R: Option<Nokta>) -> Arc<Pencere> {
    let tb = jmin(PNC_UFUK, jmax(0.0, jround(T / PNC_DILIM) * PNC_DILIM));
    let mut key = format!("{}|{}{}", js_sayi(kq.id), js_sayi(tb), R.map(|r| format!("|{}|{}", js_sayi(jround(2.0 * r.x)), js_sayi(jround(2.0 * r.y)))).unwrap_or_default());
    let R = if pr_sirasiz() {
        if R.is_none() {
            key.push_str(&format!("|{:x}|{:x}", kq.x.to_bits(), kq.y.to_bits()));
        }
        R.map(|r| nk(jround(2.0 * r.x) / 2.0, jround(2.0 * r.y) / 2.0))
    } else {
        R
    };
    if let Some(e) = cx.PE.get(&key) {
        return e.clone();
    }
    let team = cx.team;
    let ch = cx.ch;
    let B = match R {
        Some(r) => r,
        None => {
            if tb == 0.0 || ayni_varlik(kq, &cx.h) {
                nk(kq.x, kq.y)
            } else {
                let src = cx.src.clone();
                let h = cx.h.clone();
                let pw = predict_world(k, &src, team, nk(h.x, h.y), tb, false, None, false, None);
                match bul_i(&pw, kq.id, kq.team) {
                    Some(i) => nk(pw[i].x, pw[i].y),
                    None => nk(kq.x, kq.y),
                }
            }
        }
    };
    let pw = p_dunya(k, cx, B, tb, "pass", Some(kq));
    let W = Arc::new(bek_yer_w(k, &pw, team, B, None));
    let mut r = (**kq).clone();
    r.x = B.x;
    r.y = B.y;
    r.vx = 0.0;
    r.vy = 0.0;
    let k_a = jmax(0.0, jmin(1.0, dis::keep_p(k, &r, &W, None, k.D.recv_keep_t)));
    let sut = if bekci(kq) { 0.0 } else { sut_s(k, &W, team, B.x, B.y, ch, Some(kq), Some(tb)) * sut_vakit(k, &W, team, B.x, B.y, Some(kq)) };
    let zem = zemin_h(k, &W, team, B.x, B.y, ch, Some(&r), Some(cx), Some(tb));
    let sur = if bekci(kq) {
        0.0
    } else {
        let kayip = cx.kayip.clone();
        gam(k) * sur_d(k, &W, team, &r, B, ch, &kayip, None, None)
    };
    let b = jmax(jmax(sut, gam(k) * zem), sur);
    let e = Arc::new(Pencere { k: kq.clone(), B, W, tb, k_a, sut, zem, sur, b });
    cx.PE.insert(key, e.clone());
    e
}

thread_local! {
    /// ALAN_SECIM_BEKLENEN_IZ: son enIyiPas çağrısının adaylarının değerleri (durumD beklenen seçimde kullanır)
    static SON_ADAY: std::cell::RefCell<Vec<f64>> = const { std::cell::RefCell::new(Vec::new()) };
}
/// JS enIyiPas(cx, w, A, T, haric, der) (decide.js:7021): A'dan (T tik sonra) en iyi pasın değeri ve
/// penceresi; der > 0 ise alıcıdan bir pas daha.
pub fn en_iyi_pas(k: &mut Kafa, cx: &mut Cx, w: &[Op], A: Nokta, T: f64, haric: f64, der: f64) -> (f64, Option<Arc<Pencere>>) {
    let mut adaylar: Vec<f64> = Vec::new();
    let r = en_iyi_pas0(k, cx, w, A, T, haric, der, &mut adaylar);
    SON_ADAY.with(|c| *c.borrow_mut() = adaylar);
    r
}
fn en_iyi_pas0(k: &mut Kafa, cx: &mut Cx, w: &[Op], A: Nokta, T: f64, haric: f64, der: f64, adaylar: &mut Vec<f64>) -> (f64, Option<Arc<Pencere>>) {
    let mut best = 0.0;
    let mut bk: Option<Arc<Pencere>> = None;
    let src = cx.src.clone();
    for kq in src.iter() {
        if kq.team != cx.team || kq.noTouch || kq.id == haric {
            continue;
        }
        let k0: Op = match bul_i(w, kq.id, kq.team) {
            Some(i) => w[i].clone(),
            None => kq.clone(),
        };
        let L0 = hypot(k0.x - A.x, k0.y - A.y);
        let iz = crate::iz_acik() && eniyi_iz(&kq.govde.name);
        if L0 < 5.0 || L0 > dis::pas_max(k) {
            if iz { eprintln!("ENIYI A ({:.1},{:.1}) T {:.0} → {} ({:.1},{:.1}) L {:.1} pasMax {:.1}: uzaklık dışı", A.x, A.y, T, kq.govde.name, k0.x, k0.y, L0, dis::pas_max(k)); }
            continue;
        }
        let tf = dis::ucus_t(k, L0, k.D.arriveFirm, None);
        let goreli = gorus_goreli();
        if if goreli { tf } else { T + tf } > PNC_GORUS {
            if iz { eprintln!("ENIYI A ({:.1},{:.1}) T {:.0} → {} L {:.1} tf {:.0}: ufuk dışı", A.x, A.y, T, kq.govde.name, L0, tf); }
            continue;
        }
        if iz { eprintln!("ENIYI A ({:.1},{:.1}) T {:.0} → {} ({:.1},{:.1}) L {:.1} tf {:.0}: aday", A.x, A.y, T, kq.govde.name, k0.x, k0.y, L0, tf); }
        let P0 = pencere(k, cx, kq, T + tf, None);
        let mut P = P0.clone();
        let mut t_r = tf;
        let mut R: Option<Nokta> = None;
        {
            let Lb = hypot(P0.B.x - A.x, P0.B.y - A.y);
            if Lb >= 5.0 && Lb <= dis::pas_max(k) {
                let rb = veya(dis::as_s_rcv_buf(k), 0.0);
                let vk = (0.17 + 0.006 * alan_hz(&k0)) * dis::alan_spre(k, &k0);
                let ux = (P0.B.x - A.x) / Lb;
                let uy = (P0.B.y - A.y) / Lb;
                let mut s = 5.0;
                while s < Lb {
                    let x = A.x + ux * s;
                    let y = A.y + uy * s;
                    let d = jmax(0.0, hypot(x - k0.x, y - k0.y) - 1.4);
                    let tk = dis::as_gec(k, &k0, x, y, d) + d / vk + rb;
                    let tbb = dis::ucus_t(k, Lb, k.D.arriveFirm, Some(s));
                    if tk <= tbb {
                        R = Some(nk(x, y));
                        t_r = tbb;
                        break;
                    }
                    s += 1.0;
                }
                if R.is_some() {
                    if if goreli { t_r } else { T + t_r } > PNC_GORUS {
                        continue;
                    }
                    P = pencere(k, cx, kq, T + t_r, R);
                }
            }
        }
        let L = hypot(P.B.x - A.x, P.B.y - A.y);
        let iz = crate::iz_acik() && eniyi_iz(&kq.govde.name);
        if L < 5.0 || L > dis::pas_max(k) {
            if iz { eprintln!("ENIYI   x A ({:.1},{:.1}) T {:.0} → {} alış ({:.1},{:.1}) L {:.1}: alış uzaklık dışı", A.x, A.y, T, kq.govde.name, P.B.x, P.B.y, L); }
            continue;
        }
        let lag = k.D.sut_hazir;
        let (lo1, lo2) = if yol_olabilir() {
            // ALAN_YOL_OLABILIR_IZ: yolun açıklığı donmuş öngörü dünyasından değil, kararın dünyasından (cx.src):
            // her savunmacı (ve Bekçi) karar anındaki yerinden, tepkisi ve dönüş gecikmesiyle top geçmeden yola
            // yetişebilir mi
            (yolOlabilir(k, &cx.src.clone(), cx.team, A, P.B, T + lag, Some(&**kq), Some(haric), iz), 1.0)
        } else {
            (dis::lane_ok(k, w, cx.team, A, P.B, Some(k.D.arriveFirm), Some(lag)), bek_yol(k, w, cx.team, A, P.B, Some(lag)))
        };
        let Lo = lo1 * lo2;
        if Lo < 0.05 {
            if iz {
                let L = veya(hypot(P.B.x - A.x, P.B.y - A.y), 1.0);
                let (ux, uy) = ((P.B.x - A.x) / L, (P.B.y - A.y) / L);
                let yak: Vec<String> = w.iter().filter(|o| o.team != cx.team && !o.noTouch).filter_map(|o| {
                    let al = (o.x - A.x) * ux + (o.y - A.y) * uy;
                    let pe = (-(o.x - A.x) * uy + (o.y - A.y) * ux).abs();
                    if (bekci(o) || pe < 6.0) && al > -2.0 && al < L + 2.0 { Some(format!("{}({:.1},{:.1}) yan {:.1}", o.govde.name, o.x, o.y, pe)) } else { None }
                }).collect();
                eprintln!("ENIYI   x A ({:.1},{:.1}) T {:.0} → {} alış ({:.1},{:.1}) yol {:.3} bekçi {:.3}: yol kapalı | {:?}", A.x, A.y, T, kq.govde.name, P.B.x, P.B.y, lo1, lo2, yak);
            }
            continue;
        }
        let mut nv = node_v(k, cx, kq, T + t_r, der, R);
        if tek_sut() && !bekci(kq) {
            // alıcının tek dokunuş şutu: pas T'de A'dan vurulur, top T + tR'de P.B'de
            let ts = tek_sut_deger(k, cx, A, P.B, T, T + t_r, kq);
            if crate::iz_acik() && eniyi_iz(&kq.govde.name) {
                eprintln!("ENIYI     tek dokunuş {} ({:.1},{:.1}) T {:.0} tek {:.3} düğüm {:.3}", kq.govde.name, P.B.x, P.B.y, T + t_r, ts, nv);
            }
            nv = jmax(nv, ts);
        }
        let kayip = cx.kayip.clone();
        let e = Lo * nv - (1.0 - Lo) * kay(k, &kayip, (A.x + P.B.x) / 2.0, (A.y + P.B.y) / 2.0);
        adaylar.push(e);
        if crate::iz_acik() && eniyi_iz(&kq.govde.name) {
            eprintln!("ENIYI   = A ({:.1},{:.1}) T {:.0} → {} alış ({:.1},{:.1}) tR {:.0} yol {:.3} düğüm {:.3} e {:.3} | pencere tut {:.3} şut {:.3} zemin {:.3} sür {:.3}", A.x, A.y, T, kq.govde.name, P.B.x, P.B.y, t_r, Lo, nv, e, P.k_a, P.sut, P.zem, P.sur);
        }
        if e > best {
            best = e;
            bk = Some(P);
        }
    }
    (best, bk)
}

/// JS nodeV(cx, k, T, der, R) (decide.js:7077): alıcı k'nin T tik sonraki değeri (tutma × en iyisi
/// (şut, zemin, sürme, sonraki pas) × güven − kayıp).
pub fn node_v(k: &mut Kafa, cx: &mut Cx, kq: &Op, T: f64, der: f64, R: Option<Nokta>) -> f64 {
    let e = pencere(k, cx, kq, T, R);
    let mut key = format!("v|{}|{}|{}{}", js_sayi(kq.id), js_sayi(e.tb), js_sayi(der), R.map(|r| format!("|{}|{}", js_sayi(jround(2.0 * r.x)), js_sayi(jround(2.0 * r.y)))).unwrap_or_default());
    if pr_sirasiz() {
        key.push_str(&format!("|{:x}", T.to_bits()));
        if R.is_none() {
            key.push_str(&format!("|{:x}|{:x}", kq.x.to_bits(), kq.y.to_bits()));
        }
    }
    if let Some(v) = cx.PV.get(&key) {
        return *v;
    }
    let pas = if der > 0.0 { gam(k) * en_iyi_pas(k, cx, &e.W, e.B, e.tb, kq.id, der - 1.0).0 } else { 0.0 };
    let kayip = cx.kayip.clone();
    let v = e.k_a * jmax(e.b, pas) * guven(k, jmax(0.0, veya(T, 0.0))) - (1.0 - e.k_a) * kay(k, &kayip, e.B.x, e.B.y);
    cx.PV.insert(key, v);
    v
}

/// JS durumD(cx, w, q, at, T0) (decide.js:7086): q topu at'ta (T0 tik sonra) alırsa durumu.
pub fn durum_d(k: &mut Kafa, cx: &mut Cx, w: &[Op], q: &Oyuncu, at: Nokta, T0: f64) -> Arc<DurumR> {
    let mut key = format!("d|{}|{}|{}|{}", js_sayi(q.id), js_sayi(jround(2.0 * at.x)), js_sayi(jround(2.0 * at.y)), js_sayi(jround(T0 / PNC_DILIM)));
    let (at, T0) = if pr_sirasiz() {
        key.push_str(&format!("|{:x}", dunya_oz(w)));
        (nk(jround(2.0 * at.x) / 2.0, jround(2.0 * at.y) / 2.0), jround(T0 / PNC_DILIM) * PNC_DILIM)
    } else {
        (at, T0)
    };
    if let Some(r) = cx.PD.get(&key) {
        return r.clone();
    }
    let team = cx.team;
    let ch = cx.ch;
    let kayip = cx.kayip.clone();
    let wb = bek_yer_w(k, w, team, at, None);
    let mut r = q.clone();
    r.x = at.x;
    r.y = at.y;
    r.vx = 0.0;
    r.vy = 0.0;
    let T0v = veya(T0, 0.0);
    let sS = sut_s(k, &wb, team, at.x, at.y, ch, Some(q), Some(T0v));
    let s_v = sut_vakit(k, &wb, team, at.x, at.y, Some(q));
    let sut = sS * s_v;
    if crate::iz_acik() && std::env::var("ALAN_SURD_IZ").ok().as_deref() == Some(&*q.govde.name) {
        eprintln!("DURUM {} {} ({:.1},{:.1}) T0 {:.0} şutS {:.3} vakit {:.3}", k.tick(), q.govde.name, at.x, at.y, T0v, sS, s_v);
    }
    let z0 = zemin_h(k, &wb, team, at.x, at.y, ch, Some(&r), Some(cx), Some(T0));
    let sur = gam(k) * sur_d(k, &wb, team, &r, at, ch, &kayip, Some(cx), Some(T0));
    // ALAN_IKI_HAMLE_IZ: plan en çok iki top hareketi: q'nun pası alıcının durumuyla biter (alıcının pası yok)
    let der = if iki_hamle() { 0.0 } else { 1.0 };
    let ip = en_iyi_pas(k, cx, w, at, T0, q.id, der);
    let pas = gam(k) * ip.0;
    let mut v0 = jmax(jmax(jmax(sut, gam(k) * z0), pas), sur);
    if secim_beklenen() {
        // gelecekteki karar maçtaki gibi gürültülü (kafa1_secim): seçilenin beklenen değeri, en iyisinin değil
        let mut c: Vec<f64> = vec![sut, gam(k) * z0, sur];
        SON_ADAY.with(|a| c.extend(a.borrow().iter().map(|e| gam(k) * e)));
        let B = jmin(1.0, k.press_at(w, q.team, at.x, at.y));
        let gw = k.secim_w(q, B, false, None);
        v0 = secimBeklenen(&c, gw, q.id + 1000.0 * at.x + at.y);
    }
    let v = v0 * guven(k, jmax(0.0, T0v));
    let pas_k = ip.1.as_ref().map(|P| format!("{} ({},{}) +{}t", P.k.govde.name, to_fixed(P.B.x, 0), to_fixed(P.B.y, 0), js_sayi(P.tb)));
    let res = Arc::new(DurumR { v, sut, zem: z0, pas, sur, pas_k });
    cx.PD.insert(key, res.clone());
    res
}

/// JS prGam(h, tac) (decide.js:7122): tempo / sabır / kontraya göre sonraki adımın indirimi
pub fn pr_gam(k: &Kafa, h: &Oyuncu, tac: Option<&Tac>) -> f64 {
    let tempo = tac.and_then(|t| t.tempo).unwrap_or(0.5);
    let htempo = h.govde.tl.as_ref().and_then(|t| t.huc.get("tempo").copied()).unwrap_or(0.5);
    let sabir = tac.and_then(|t| t.sabir).unwrap_or(0.5);
    let kontra = tac.map(|t| t.kontra).unwrap_or(false);
    let tp = tempo + 0.6 * (htempo - 0.5) + (0.5 - sabir) + if kontra { 0.2 } else { 0.0 };
    jmax(0.8, jmin(0.96, k.D.combo + 0.12 * (0.5 - jmax(0.0, jmin(1.0, tp)))))
}

/// JS prensipDegerle(h, src, ch, team, opts, lossAt, T, tac) (decide.js:7109)
pub fn prensip_degerle(k: &mut Kafa, hi: Pid, src: &Arc<Dunya>, ch: f64, team: f64, opts: &mut Vec<Secenek>, loss_at: &KayipRef, T: f64, tac: Option<&Tac>) {
    let g0 = k.bag._g_pr;
    let w0 = k.bag._en_wd;
    let t0_ = k.bag._en_tm;
    k.bag._g_pr = Some(pr_gam(k, &src[hi], tac));
    k.bag._en_wd = Some(veyo(tac.and_then(|t| t.enDinlen), 0.0));
    k.bag._en_tm = Some(team);
    // ALAN_SUR_UFUK_IZ: taşıyıcının yeniden karar vereceği ortalama süre (maçın decT'si: 5 + (1 − tempo)·8 + zar 0–3)
    let u0 = UFUK.with(|u| u.replace(5.0 + (1.0 - tac.and_then(|t| t.tempo).unwrap_or(0.5)) * 8.0 + 1.5));
    prensip_degerle0(k, hi, src, ch, team, opts, loss_at, T);
    UFUK.with(|u| u.set(u0));
    k.bag._g_pr = g0;
    k.bag._en_wd = w0;
    k.bag._en_tm = t0_;
}

/// JS enGec(q, w, f): q'nun barı dünyadakine çekilerek f (tartının bağlamında q da değişir).
fn en_gec<R>(k: &mut Kafa, cx: &mut Cx, q: &Op, w: &[Op], f: impl FnOnce(&mut Kafa, &mut Cx, &Op) -> R) -> R {
    match dis::enGec(k, q, w) {
        None => f(k, cx, q),
        Some(e) => {
            let mut c = (**q).clone();
            c.en = Some(e);
            let q2 = Arc::new(c);
            let eski_src = cx.src.clone();
            let eski_h = cx.h.clone();
            if let Some(i) = cx.src.iter().position(|z| ayni_varlik(z, q)) {
                let mut s = (*cx.src).clone();
                s.koy(i, q2.clone());
                cx.src = Arc::new(s);
            }
            if ayni_varlik(&cx.h, q) {
                cx.h = q2.clone();
            }
            let r = f(k, cx, &q2);
            cx.src = eski_src;
            cx.h = eski_h;
            r
        }
    }
}

fn dok_dk(r: &DurumR, o: &mut Secenek) {
    let d = o.dok.get_or_insert_with(Dok::default);
    d.koy("sut", r.sut);
    d.koy("ileri", r.pas);
    d.koy("surme", r.sur);
    d.koy("zemin", r.zem);
    if let Some(p) = &r.pas_k {
        d.sonra = Some(p.clone());
    }
}

const PR_BOL: bool = false;

/// Deney (ALAN_PR_SIRASIZ_IZ, PLAN-paralel.md Adım 1): prensip tartısının önbellekleri (PW, PE, PV, PD, IM) sıradan
/// bağımsız: değer anahtarın temsil ettiği girdilerle hesaplanır (nokta ve süre anahtarın ızgarasında, anahtarda olmayan
/// dünya ve yerler anahtara eklenir). Davranışı değiştirir; açıkken prensip döngüsü işçilere bölüşülür.
pub fn pr_sirasiz() -> bool {
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *A.get_or_init(|| crate::iz_acik() && std::env::var("ALAN_PR_SIRASIZ_IZ").is_ok())
}
/// Dünyanın özeti (önbellek anahtarı için): oyuncuların kimliği, yeri, hızı, hedefi.
fn dunya_oz(w: &[Op]) -> u64 {
    use std::hash::Hasher;
    let mut h = crate::HizliKarma::default();
    for p in w {
        for v in [p.id, p.team, p.x, p.y, p.vx, p.vy, p.tx, p.ty, p.en.unwrap_or(f64::NAN)] {
            h.write_u64(v.to_bits());
        }
        h.write_u8(p.noTouch as u8);
        h.write_u8(p.sprint as u8);
    }
    h.finish()
}
/// `to_fixed(x, 2)`'nin gösterdiği sayı
fn iki_bas(x: f64) -> f64 {
    to_fixed(x, 2).parse().unwrap_or(x)
}
/// Hız (yalnız Rust, JS'te yok): "bir rakip olmasa" tablosunda arkadaşından UZAK_R'den uzak rakip atlanır
/// (çıkarılmasının o arkadaşın değerine etkisi ölçüldü, bkz. ALAN_UZAK_IZ). Eskisi: ALAN_UZAK_ESKI_IZ.
const UZAK_R: f64 = 20.0;
#[allow(non_upper_case_globals)]
static UZAK_KIS_: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
fn uzak_kis() -> bool {
    *UZAK_KIS_.get_or_init(|| !(crate::iz_acik() && std::env::var("ALAN_UZAK_ESKI_IZ").is_ok()))
}
/// "Bir rakip olmasa" dünyaları fırsat önbelleğini (IM) paylaşırdı; eskisi: ALAN_CXIM_ESKI_IZ.
fn cxim_eski() -> bool {
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *A.get_or_init(|| crate::iz_acik() && std::env::var("ALAN_CXIM_ESKI_IZ").is_ok())
}
fn uzak_h() -> bool {
    crate::iz_acik() && std::env::var("ALAN_UZAKH_IZ").is_ok()
}
fn uzak_iz() -> bool {
    crate::iz_acik() && std::env::var("ALAN_UZAK_IZ").is_ok()
}

/// JS prensipDegerle0 (decide.js:7126): seçenekleri prensiple tartar (Vur, İlerlet, Güvene al, Çevir,
/// Alana sür, Çalım, Kalkan, Bekle); sonra arkadaşların ve taşıyanın değerini m._prT'ye yazar.
pub fn prensip_degerle0(k: &mut Kafa, hi: Pid, src: &Arc<Dunya>, ch: f64, team: f64, opts: &mut Vec<Secenek>, loss_at: &KayipRef, T: f64) {
    let h = src[hi].clone();
    let mut nd: f64 = 1e9;
    for q in src.iter() {
        if !(q.team == team || q.noTouch || bekci(q)) {
            nd = jmin(nd, hypot(q.x - h.x, q.y - h.y));
        }
    }
    let mut cx = Cx { src: src.clone(), team, ch, kayip: loss_at.clone(), h: h.clone(), PW: Default::default(), PE: Default::default(), PV: Default::default(), PD: Default::default(), IM: None };
    let L = |k: &mut Kafa, x: f64, y: f64| kay(k, loss_at, x, y);
    let govde = |k: &mut Kafa, mut cx: &mut Cx, o: &mut Secenek| {
        let d = o.det.clone().unwrap_or_default();
        let kind = o.kind.as_str();
        if kind == "gönder" {
            let e_o = d.endO.or(d.end).unwrap_or(nk(h.x, h.y));
            let Pl = d.Pl.unwrap_or(0.0);
            let Lv = L(k, e_o.x, e_o.y);
            let P = d.Pk.unwrap_or(0.0);
            // ALAN_SEKME_BEDEL_IZ: şutun değeri yalnız golden; gol olmayan her sonuç (sekip bize dönse de) kaybedilmiş gibi bedel
            let Pl = if sekme_bedel() { jmax(0.0, 1.0 - P) } else { Pl };
            o.v = Some(P - Pl * Lv);
            o.dok = Some(Dok::yeni(&[("gol", P), ("kayip", -Pl * Lv)]));
            if crate::iz_acik() && std::env::var("ALAN_SURV_IZ").is_ok() {
                eprintln!("GONV {} {} ->({:.1},{:.1}) Pk {:.3} Pl {:.3} kayip {:.3} v {:.3}", k.tick(), h.govde.name, e_o.x, e_o.y, P, Pl, Lv, P - Pl * Lv);
            }
        } else if kind == "pas" || kind == "önüne" || kind == "kenardan" {
            let at = o._at.or(d.endD).or(d.endM).or(o.to).unwrap_or_else(|| {
                let q = &src[o.q.unwrap()];
                nk(q.x, q.y)
            });
            let Pd = dis::al_p(Some(&d)).or(d.P).unwrap_or(0.0);
            let Ps = veyo(d.Ps, 0.0);
            let Pl = d.Pl.unwrap_or_else(|| jmax(0.0, 1.0 - d.P.unwrap_or(0.0)));
            let rq: Op = match o.q {
                Some(qi) => match bul_i(src, src[qi].id, src[qi].team) {
                    Some(i) => src[i].clone(),
                    None => src[qi].clone(),
                },
                None => h.clone(),
            };
            let tf = match o.tf {
                Some(t) if crate::mat::dogru(t) => t,
                _ => dis::ucus_t(k, hypot(at.x - h.x, at.y - h.y), k.D.arriveFirm, None),
            };
            let pw = p_dunya(k, &mut cx, at, tf, "pass", Some(&rq));
            let w_e = dis::en_w(k, &pw, o);
            let t_a = en_gec(k, &mut cx, &rq, &w_e, |k, cx, rq| {
                let r = durum_d(k, cx, &w_e, rq, at, veya(tf, 0.0));
                if ayni_varlik(rq, &cx.h) {
                    (r.v, r, 1.0)
                } else {
                    let mut c = (**rq).clone();
                    c.x = at.x;
                    c.y = at.y;
                    c.vx = 0.0;
                    c.vy = 0.0;
                    let k_a = jmax(0.0, jmin(1.0, dis::keep_p(k, &c, &w_e, None, k.D.recv_keep_t)));
                    let lv = kay(k, &cx.kayip.clone(), at.x, at.y);
                    let mut v = k_a * r.v - (1.0 - k_a) * lv;
                    // ALAN_TEK_SUT_IZ: alıcı tek dokunuşla vurabilir: topu tutması ve şuta hazırlanması gerekmez,
                    // kaleci de topun geldiği andaki (benzetilen dünyadaki) yerindedir; şutun ayrıca yerleşmiş
                    // kaleciye göre tartılması (durumD) tek dokunuşun kazancını görmez.
                    if tek_sut() && !bekci(rq) {
                        // topun alıcıya varışı: ucusSure'nin topu yerleştirme payı (4 tik) tek dokunuşta yok
                        let T0 = jmax(1.0, veya(tf, 0.0) - 4.0);
                        let hb = nk(cx.h.x, cx.h.y);
                        let ts = tek_sut_deger(k, cx, hb, at, 0.0, T0, rq);
                        if crate::iz_acik() && std::env::var("ALAN_SURV_IZ").is_ok() {
                            eprintln!("TEKSUT {} → {} ({:.1},{:.1}) T {:.0} tek {:.3} tutup {:.3}", k.tick(), rq.govde.name, at.x, at.y, T0, ts, v);
                        }
                        v = jmax(v, ts);
                    }
                    (v, r, k_a)
                }
            });
            let t_s = 0.0;
            let e_o = d.endO.or(d.end).unwrap_or(at);
            let Lv = L(k, e_o.x, e_o.y);
            // ALAN_SEKME_BEDEL_IZ: pas yalnız alıcıya doğrudan varınca değerli; geri kalan her pay (sekip bize dönen de) başarısız pas
            let Pl = if sekme_bedel() { jmax(0.0, 1.0 - Pd) } else { Pl };
            o.v = Some(Pd * t_a.0 + Ps * t_s - Pl * Lv);
            if crate::iz_acik() && std::env::var("ALAN_SURV_IZ").is_ok() {
                eprintln!("PASV {} {} [{}{}→{} hız {:?}] ->({:.1},{:.1}) Pd {:.3} alici {:.3} durum {:.3} Ps {:.3} Pl {:.3} kayip {:.3} v {:.3} | şut {:.3} zemin {:.3} pas {:.3} sür {:.3} {:?} T {:.0}", k.tick(), h.govde.name, o.kind, if o.tek.map_or(false, crate::mat::dogru) { "·tek" } else { "" }, rq.govde.name, o.launch.as_ref().map(|l| ((l.vx * 100.0).round() / 100.0, (l.vy * 100.0).round() / 100.0)), at.x, at.y, Pd, t_a.2, t_a.1.v, Ps, Pl, Lv, Pd * t_a.0 + Ps * t_s - Pl * Lv, t_a.1.sut, t_a.1.zem, t_a.1.pas, t_a.1.sur, t_a.1.pas_k, tf);
            }
            o.dok = Some(Dok::yeni(&[("tutar", Pd), ("alici", t_a.2), ("durum", t_a.1.v), ("sekme", Ps * t_s), ("kayip", -Pl * Lv), ("kayp", Pl), ("sure", tf)]));
            o._pT = Some(tf);
            dok_dk(&t_a.1, o);
        } else if kind == "sür" || kind == "çalım" {
            let mut E = o.toE.or(o.to).unwrap_or(nk(h.x, h.y));
            let mut le = hypot(E.x - h.x, E.y - h.y);
            let mut Ts = jmax(6.0, match o.dir {
                Some(dr) => dis::sur_sure(k, &h, le, Some(dr), None),
                None => le / 0.2,
            });
            // ALAN_SUR_UFUK_IZ: yeniden karar anına (H) dek: yolun o ana dek gidilen kısmı
            if sur_ufuk() {
                let H = UFUK.with(|u| u.get());
                if Ts > H && le > 0.5 {
                    let f = H / Ts;
                    E = nk(h.x + (E.x - h.x) * f, h.y + (E.y - h.y) * f);
                    le *= f;
                    Ts = H;
                }
            }
            // ALAN_SUR_E_IZ: tutma, değerin tartıldığı ara noktaya (E) dek; sürmenin son hedefine dek değil (orada yeniden karar verilir)
            let P = if sur_e() && le > 0.5 {
                let hk: Option<Oyuncu> = if o.spr.map(crate::mat::dogru).unwrap_or(false) { Some(dis::spr_kop(k, &h)) } else { None };
                crate::kafa::kafa1_ikili::kp_kal(dis::keep_p(k, hk.as_ref().unwrap_or(&*h), src, o.dir, Ts))
            } else {
                match d.P {
                    Some(p) => p,
                    None => crate::kafa::kafa1_ikili::kp_kal(dis::keep_p(k, &h, src, o.dir, Ts)),
                }
            };
            // ALAN_SEND_GEC_IZ: sendelemeyle kesilen sürmede varış, topu geri alana dek sonrasının dünyası
            let ts_d = Ts + o.sendGec.unwrap_or(0.0);
            let pw = p_dunya(k, &mut cx, E, ts_d, "drive", Some(&h));
            let w_e = dis::en_w(k, &pw, o);
            let tE = en_gec(k, &mut cx, &h.clone(), &w_e, |k, cx, hq| durum_d(k, cx, &w_e, hq, E, ts_d));
            let Lv = L(k, (h.x + E.x) / 2.0, (h.y + E.y) / 2.0);
            o.v = Some(P * tE.v - (1.0 - P) * Lv);
            if crate::iz_acik() && std::env::var("ALAN_SURV_IZ").is_ok() {
                eprintln!("SURV {} {} ({:.1},{:.1})->({:.1},{:.1}) spr {} gec {:.0} Ts {:.0} P {:.3} durum {:.3} kayip {:.3} v {:.3}", k.tick(), h.govde.name, h.x, h.y, E.x, E.y, o.spr.map_or(false, crate::mat::dogru), o.sendGec.unwrap_or(0.0), Ts, P, tE.v, Lv, P * tE.v - (1.0 - P) * Lv);
                eprintln!("     şut {:.3} zemin {:.3} pas {:.3} sür {:.3} {:?}", tE.sut, tE.zem, tE.pas, tE.sur, tE.pas_k);
            }
            o.dok = Some(Dok::yeni(&[("tutar", P), ("durum", tE.v), ("kayip", -(1.0 - P) * Lv), ("sure", Ts)]));
            o._pT = Some(Ts);
            dok_dk(&tE, o);
        } else if kind == "tut" && bekci(&h) && h.hal.bekBas.is_some() && veyo(o.hold, 15.0) >= 90.0 - (k.tick() - h.hal.bekBas.unwrap()) - 4.0 {
            let v = -L(k, h.x, h.y);
            o.v = Some(v);
            o.dok = Some(Dok::yeni(&[("kayip", v)]));
        } else if kind != "tut" {
            o.v = None;
        } else {
            let Th = veyo(o.hold, T);
            let P = match d.P {
                Some(p) => p,
                None => dis::keep_p(k, &h, src, None, Th),
            };
            let pw = p_dunya(k, &mut cx, nk(h.x, h.y), Th, "drive", Some(&h));
            let w_e = dis::en_w(k, &pw, o);
            let t_h = en_gec(k, &mut cx, &h.clone(), &w_e, |k, cx, hq| durum_d(k, cx, &w_e, hq, nk(h.x, h.y), Th));
            let Lv = L(k, h.x, h.y);
            o.v = Some(P * t_h.v - (1.0 - P) * Lv);
            let simdi = durum_d(k, &mut cx, src, &h, nk(h.x, h.y), 0.0).v;
            if crate::iz_acik() && std::env::var("ALAN_SURV_IZ").is_ok() {
                eprintln!("TUTV {} {} Th {:.0} P {:.3} durum {:.3} (şut {:.3} zemin {:.3} pas {:.3} sür {:.3} {:?}) kayıp {:.3} v {:.3}", k.tick(), h.govde.name, Th, P, t_h.v, t_h.sut, t_h.zem, t_h.pas, t_h.sur, t_h.pas_k, Lv, P * t_h.v - (1.0 - P) * Lv);
            }
            o.dok = Some(Dok::yeni(&[("tutar", P), ("durum", t_h.v), ("simdi", simdi), ("kayip", -(1.0 - P) * Lv), ("sure", Th)]));
            o._pT = Some(Th);
            dok_dk(&t_h, o);
        }
    };
    // seçenekler birbirinden bağımsız (paralel kafada işçilere bölüşülür; sıra sınaması ALAN_TERS_IZ)
    let ix: Vec<usize> = (0..opts.len()).filter(|&i| opts[i].v.is_some()).collect();
    let n = ix.len();
    let ters = crate::iz_acik() && std::env::var("ALAN_TERS_PR_IZ").is_ok();
    let taze = crate::iz_acik() && std::env::var("ALAN_PR_TAZE_IZ").is_ok();
    // DİKKAT: önbellekler (PW/PE/PV/PD) yuvarlanmış anahtarla ilk soranın sonucunu verir (JS gibi): sıra
    // sınaması 289. tikte ayrışır, bu yüzden bölüşülmez (PR_BOL kapalı).
    let bol = (PR_BOL || taze || pr_sirasiz()) && !ters && crate::paralel::hazir(n);
    if bol {
        let mp: Dunya = k.mac.as_ref().map(|m| m.ps.clone()).unwrap_or_default();
        crate::paket::taban_kur([src], mp.iter());
    }
    {
        let mut bir = |j: usize, pb: Option<&[u8]>| -> Option<Vec<u8>> {
            let i = ix[j];
            if let Some(pb) = pb {
                opts[i] = crate::paket::coz(pb);
                return None;
            }
            if taze {
                cx.PW.clear();
                cx.PE.clear();
                cx.PV.clear();
                cx.PD.clear();
                if let Some(m) = &cx.IM {
                    m.borrow_mut().clear();
                }
            }
            govde(k, &mut cx, &mut opts[i]);
            if bol {
                let b = crate::paket::kodla(&opts[i]);
                if crate::iz_acik() && std::env::var("ALAN_PKS_IZ").is_ok() {
                    let _o: Secenek = crate::paket::coz(&b);
                }
                Some(b)
            } else {
                None
            }
        };
        if !(bol && crate::paralel::dagit(n, &mut bir)) {
            for j in 0..n {
                bir(if ters { n - 1 - j } else { j }, None);
            }
        }
    }
    if bol {
        crate::paket::taban_bos();
    }
    if taze {
        // seçeneklerin önbellekleri sonrasına taşınmaz (hangi işçi en son hangisini tarttıysa ondan bağımsız)
        cx.PW.clear();
        cx.PE.clear();
        cx.PV.clear();
        cx.PD.clear();
        if let Some(m) = &cx.IM {
            m.borrow_mut().clear();
        }
    }
    if kayip_sahip() {
        let v0 = opts.iter().filter(|o| o.kind != "gönder").filter_map(|o| o.v).fold(0.0, jmax);
        for o in opts.iter_mut() {
            let Some(v) = o.v else { continue };
            // pas: rakibe geçme olasılığı (Pl; sekip bizde kalan kayıp değil). ALAN_KAYIP_SAHIP_PD_IZ: eski (1 − doğrudan varma)
            let pd_eski = crate::iz_acik() && std::env::var("ALAN_KAYIP_SAHIP_PD_IZ").is_ok();
            let tut = match o.dok.as_ref() {
                Some(d) if o.kind == "gönder" => d.al("gol"),
                Some(d) if !pd_eski && d.al("kayp").is_some() => d.al("kayp").map(|l| 1.0 - l),
                Some(d) => d.al("tutar"),
                None => None,
            };
            let Some(tut) = tut else { continue };
            let bedel = jmax(0.0, 1.0 - tut) * v0;
            o.v = Some(v - bedel);
            if let Some(d) = o.dok.as_mut() {
                d.koy("sahip", -bedel);
            }
            if crate::iz_acik() && std::env::var("ALAN_SURV_IZ").is_ok() {
                eprintln!("SAHIP {} {} {} tut {:.3} V0 {:.3} v {:.3} -> {:.3}", k.tick(), h.govde.name, o.kind, tut, v0, v, v - bedel);
            }
        }
    }
    // m._prT: arkadaşların ve taşıyanın değeri, bir rakip olmasa ne kazanılırdı
    {
        let mut tm: Vec<(f64, f64)> = Vec::new();
        let mut mm: Vec<(String, f64)> = Vec::new();
        let mut cxs: Vec<(Op, Cx)> = Vec::new();
        // karşılaştırmanın gerçek dünyası da temiz bağlamda: seçenek tartısının önbellekleri başka (bitiş) dünyalarından
        // dolu; "o olmasa" farkı iki taraf aynı biçimde hesaplanmazsa önbellek artığını ölçer
        let mut cx0 = Cx { src: cx.src.clone(), team: cx.team, ch: cx.ch, kayip: cx.kayip.clone(), h: cx.h.clone(), PW: Default::default(), PE: Default::default(), PV: Default::default(), PD: Default::default(), IM: None };
        let mac_var = k.mac.is_some();
        fn cx_siz<'a>(cxs: &'a mut Vec<(Op, Cx)>, cx: &Cx, d: &Op) -> &'a mut Cx {
            let i = match cxs.iter().position(|(z, _)| ayni_varlik(z, d)) {
                Some(i) => i,
                None => {
                    let c = Cx {
                        // sıra korunur (kafada sırayla tutulan bir şey kaymasın): çıkarılan savunmacı silinmez, etkisizleşir
                        // (dokunamaz ve sahanın çok dışında). Eskisi (listeden silmek): ALAN_CXIM_ESKI_IZ
                        src: Arc::new(if cxim_eski() {
                            cx.src.iter().filter(|z| !ayni_varlik(z, d)).cloned().collect()
                        } else {
                            cx.src.iter().map(|z| if ayni_varlik(z, d) { let mut o = (**z).clone(); o.noTouch = true; o.x = -1000.0; o.y = -1000.0; o.vx = 0.0; o.vy = 0.0; Arc::new(o) } else { z.clone() }).collect()
                        }),
                        team: cx.team,
                        ch: cx.ch,
                        kayip: cx.kayip.clone(),
                        h: cx.h.clone(),
                        PW: Default::default(),
                        PE: Default::default(),
                        PV: Default::default(),
                        PD: Default::default(),
                        IM: if cxim_eski() { cx.IM.clone() } else { None }, /* her varsayım dünyasının kendi fırsat önbelleği (anahtarda dünya yok: paylaşılınca ilk dünyanın değeri bütün savunmacılara gidiyordu) */
                    };
                    cxs.push((d.clone(), c));
                    cxs.len() - 1
                }
            };
            &mut cxs[i].1
        }
        for kq in src.iter() {
            if kq.team != team || ayni_varlik(kq, &h) || kq.noTouch || bekci(kq) {
                continue;
            }
            let Tk = dis::ucus_t(k, jmax(5.0, hypot(kq.x - h.x, kq.y - h.y)), k.D.arriveFirm, None) + k.D.sut_hazir;
            let vk = node_v(k, if cxim_eski() { &mut cx } else { &mut cx0 }, kq, Tk, 0.0, None);
            tm.push((kq.id, vk));
            if mac_var {
                for d in src.iter() {
                    if d.team == team || bekci(d) || d.noTouch {
                        continue;
                    }
                    let uz = hypot(d.x - kq.x, d.y - kq.y);
                    if uzak_kis() && uz > UZAK_R {
                        continue;
                    }
                    let c = cx_siz(&mut cxs, &cx, d);
                    let v2 = node_v(k, c, kq, Tk, 0.0, None);
                    if uzak_iz() {
                        // atamadaki yetişme olasılığının yaklaşığı (esles: 1/(1+e^((t−TW)/5)), t ≈ uzaklık/hız, TW ≈ Tk)
                        let kp = 1.0 / (1.0 + exp((uz / crate::kafa::kafa1_js::ya(k.v_s(d), 0.05) - Tk) / 5.0));
                        eprintln!("UZAK {:.2} {:.6} t={:?} kq={} d={} dx={:.0} dy={:.0} kx={:.0} kp={:.6}", uz, jmax(0.0, v2 - vk), k.bag._tick, kq.id, d.id, d.x, d.y, kq.x, kp);
                    }
                    if v2 > vk {
                        mm.push((format!("{}|{}", js_sayi(kq.id), js_sayi(d.id)), v2 - vk));
                    }
                }
            }
        }
        let hm = if !mac_var {
            None
        } else {
            let mut M: Vec<(f64, f64)> = Vec::new();
            let mut Mk: Vec<f64> = Vec::new();
            let mut base: Option<f64> = None;
            for d in src.iter() {
                if d.team == team || bekci(d) || d.noTouch {
                    continue;
                }
                if base.is_none() {
                    base = Some(node_v(k, if cxim_eski() { &mut cx } else { &mut cx0 }, &h, k.D.sut_hazir, 0.0, None));
                }
                let uz = hypot(d.x - h.x, d.y - h.y);
                if uzak_h() && uz > UZAK_R {
                    // listede kalır (Mk: tartılan rakipler), değeri tartılmaz: uzak rakip taşıyıcıya etki etmiyor
                    if !Mk.contains(&d.id) {
                        Mk.push(d.id);
                    }
                    continue;
                }
                let c = cx_siz(&mut cxs, &cx, d);
                let v = node_v(k, c, &h, k.D.sut_hazir, 0.0, None);
                if uzak_iz() {
                    eprintln!("UZAKH {:.2} {:.6}", uz, jmax(0.0, v - base.unwrap()));
                }
                if !Mk.contains(&d.id) {
                    Mk.push(d.id);
                }
                if v > base.unwrap() {
                    match M.iter_mut().find(|x| x.0 == d.id) {
                        Some(x) => x.1 = v - base.unwrap(),
                        None => M.push((d.id, v - base.unwrap())),
                    }
                }
            }
            Some((M, Mk))
        };
        if mac_var {
            let hv = node_v(k, &mut cx, &h, k.D.sut_hazir, 0.0, None);
            k.pr_t = Some(PrT { team, h_id: h.id, t: k.bag._tick, m: tm, mm, hm, h: hv });
        }
    }
}

/// JS sonraki GH sabiti (yalnız sınırlar için)
pub const _GH: f64 = GH;
