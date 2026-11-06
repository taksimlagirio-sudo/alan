//! Pres: maçın durumuna göre pres katı (macPres), aynı anda kaç presçi (presLim), ilk presçinin işi (ilkPres).
#![allow(non_snake_case)]

use crate::core_sabit::Q;
use crate::donus::shielded;
use crate::mat::{acos, atan2, cos, hypot, jmax, jmin, jsign, sin};
use crate::model::{JPPplan, JpOnK, Pid};
use crate::yetenek::mek;

use super::gecici_dis as dis;
use super::*;

/// JS macPres(m, t): yapay zekâ takımında skor/süreye göre presi artır (+1) ya da azalt (−1).
pub fn mac_pres(c: &mut Mc, t: u8) -> f64 {
    match c.m.tac.get(t as usize) {
        Some(tq) if tq.yz == Some(true) => {}
        _ => return 0.0,
    }
    let o = dis::mac_eff_mac_k(c, t);
    let p0 = c.m.tac[t as usize].pres.unwrap_or(1.0);
    match o {
        None => 0.0,
        Some(o) if o < 0.6 => {
            if p0 < 2.0 {
                1.0
            } else {
                0.0
            }
        }
        Some(o) => {
            if o > 1.6 && p0 > 1.0 {
                -1.0
            } else {
                0.0
            }
        }
    }
}

/// S7 tehlike bölgesi: taşıyıcı kalemize bundan yakınken (br, kale ortasına) tek savunmacı yetmez.
pub const S7_D: f64 = 32.0 * GKX;
/// Deney (GOREV-savunma S7): taşıyıcıyı durdurmak. Parçalar ayrı ölçülsün diye: ALAN_S7A_IZ (kale tarafında değilsen şut
/// hattına geç), ALAN_S7B_IZ (tehlike bölgesinde ikinci presçi), ALAN_S7C_IZ (geçilen savunmacı adamına değil kale önüne);
/// ALAN_S7_IZ üçü birden. Sonradan: ALAN_S7D_IZ (bit 8; geniş dönüş: geçilen herkes, baskıcılar da, adamı topun önünde
/// değilse kale önüne), ALAN_S7E_IZ (bit 16; varılamayan şut hattı yerine kendi tarafından kale önüne çekil).
/// ALAN_S7F_IZ (bit 32): tehlike bölgesinde, adamı tehlike bölgesinde olan markajcı ikinci (ve sonraki) presçi olmaz:
/// adamını devretmeden buluşmaya / kale önüne gitmesin. Ölçümde S7_ZORLA (bitler). Varsayılan kapalı.
pub static S7_ZORLA: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);
pub fn s7(parca: u8) -> bool {
    static A: std::sync::OnceLock<u8> = std::sync::OnceLock::new();
    let e = *A.get_or_init(|| {
        if !crate::iz_acik() {
            return 0;
        }
        let v = |k: &str| std::env::var(k).is_ok();
        (if v("ALAN_S7_IZ") { 7 } else { 0 }) | (if v("ALAN_S7A_IZ") { 1 } else { 0 }) | (if v("ALAN_S7B_IZ") { 2 } else { 0 }) | (if v("ALAN_S7C_IZ") { 4 } else { 0 }) | (if v("ALAN_S7D_IZ") { 8 } else { 0 }) | (if v("ALAN_S7E_IZ") { 16 } else { 0 }) | (if v("ALAN_S7F_IZ") { 32 } else { 0 })
    });
    (e | S7_ZORLA.load(std::sync::atomic::Ordering::Relaxed)) & parca != 0
}

/// JS presLim(m, dt, h, att): tetik (geri pas, kanat, zayıf kontrol, arkası dönük, uzun pas) yoksa tek presçi.
pub fn pres_lim(c: &Mc, dt: &dis::TacE, h: Option<Pid>, att: u8) -> usize {
    let n = 2usize;
    let T: &[String] = dt.t.tetik.as_deref().unwrap_or(&[]);
    let h = match h {
        Some(h) => h,
        None => return n,
    };
    if T.is_empty() || dt.kp {
        return n;
    }
    let has = |s: &str| T.iter().any(|x| x == s);
    let hp = &c.ps[h];
    let ps = c.m._pas.as_ref();
    let yeni = matches!(ps, Some(p) if p.team == att as f64 && c.m.tick - p.t < 90.0);
    let dir = dir_of(att);
    let on = (has("Geri pas") && yeni && (ps.unwrap().tx - ps.unwrap().fx) * dir < -2.0)
        || (has("Kanatta") && (hp.y - GCY).abs() > 15.0 * GKY)
        || (has("Zayıf kontrol") && dis::slowT(hp) > c.m.tick)
        || (has("Arkası dönük") && hp.ca.is_some() && cos(hp.ca.unwrap()) * dir < -0.3)
        || (has("Uzun pas havada") && yeni && ps.unwrap().len > 25.0);
    if on {
        n
    } else {
        n.min(1)
    }
}

/// JS ilkPres(m, p, h, dt, TH, dtac, sh): ilk presçinin hedefi ve işi (yolu kes, kale tarafı, önüne gövde
/// koy, yandan gir, fake).
pub fn ilk_pres(c: &mut Mc, pi: Pid, hi: Pid, dt: u8, TH: Option<&[Tehlike]>, dtac: &dis::TacE) {
    let h = c.ps[hi].clone();
    ilk_pres_h(c, pi, &h, dt, TH, dtac)
}
/// ilkPres, taşıyıcı dışarıdan (yalnız okunur; kafanın pres görünümü kopyalamadan verir).
pub fn ilk_pres_h(c: &mut Mc, pi: Pid, h: &Oyuncu, dt: u8, TH: Option<&[Tehlike]>, dtac: &dis::TacE) {
    let k = c.k;
    let tick = c.m.tick;
    let cs = dis::core_side(&h);
    let B = Q.body;
    let dke = h._dke.unwrap_or(0.0);
    let co = (h.x + cs.0 * (B + dke), h.y + cs.1 * (B + dke));
    let (px, py) = (c.ps[pi].x, c.ps[pi].y);
    let d0 = hypot(px - h.x, py - h.y);
    let trap = dtac.t.sistem.as_deref() == Some("Kenara sıkıştır");
    let okp = c.ps[pi].govde.yet.y(mek::PRES_FAKE);
    let set = |c: &mut Mc, x: f64, y: f64| {
        c.ps[pi].tx = x;
        c.ps[pi].ty = y;
    };
    let job = |c: &mut Mc, j: String| c.ps[pi].job = Some(j.into());
    if !trap {
        let ox = own_x(dt);
        let KS = pres_kesis(&c.ps[pi], &h, dt, k, c.d, c.dtik);
        let (lx, ly) = match KS {
            Some(s) => (s.x, s.y),
            None => (h.x + 10.0 * h.vx, h.y + 10.0 * h.vy),
        };
        // T1: (an, sut, k)
        let T1: Option<(f64, bool, Option<String>)> = if hypot(h.x - ox, h.y - GCY) < 35.0 * GKX {
            Some((atan2(GCY - ly, ox - lx), true, None))
        } else {
            TH.and_then(|t| t.first()).map(|t| (t.an, false, Some(t.k.clone())))
        };
        let g = match &T1 {
            Some(t) => (lx + 2.0 * cos(t.0), ly + 2.0 * sin(t.0)),
            None => gside(lx, ly, dt, 2.0),
        };
        let kt = match &T1 {
            Some(t) => (px - h.x) * cos(t.0) + (py - h.y) * sin(t.0) > 1.0 && (!t.1 || (-(px - h.x) * sin(t.0) + (py - h.y) * cos(t.0)).abs() < 1.5),
            None => hypot(px - ox, py - GCY) < hypot(h.x - ox, h.y - GCY) - 0.5,
        };
        if crate::kafa::kafa1_ikili::ks_iz() { eprintln!("      IP tick {} KS {:?} T1 {:?} g {:?} kt {} d0 {} h {} {} {} {} p {} {} {} {}", tick, KS, T1.as_ref().map(|t| t.0), g, kt, d0, h.x, h.y, h.vx, h.vy, px, py, c.ps[pi].vx, c.ps[pi].vy); }
        if !kt && (!(d0 < 1.6) || shielded(h.x, h.y, px, py, co.0, co.1)) {
            // S7a: tehlike bölgesinde kale tarafında değilsen yandan dolanma; taşıyıcıdan önce varabildiğin ilk şut hattı
            // noktasına (taşıyıcı → kale ortası) geç
            let lg = hypot(ox - h.x, GCY - h.y);
            if s7(1) && lg < S7_D && lg > 4.0 {
                let (ux, uy) = ((ox - h.x) / lg, (GCY - h.y) / lg);
                let vp = jmax(0.05, v_sp(&c.ps[pi], k));
                let vh = jmax(0.02, hypot(h.vx, h.vy));
                let sm = jmax(2.0, lg - 4.0);
                let mut s = 2.0;
                while s < sm && hypot(h.x + ux * s - px, h.y + uy * s - py) / vp > s / vh {
                    s += 0.5;
                }
                if s >= sm && s7(16) {
                    // S7e: şut hattına taşıyıcıdan önce varamıyor: arkasından kovalama, kendi tarafından dolaşıp kale önünü tut
                    let ko = jmin(lg - 2.0, 8.0 * GKX);
                    let (nx, ny) = (-uy, ux);
                    let sg = if (px - h.x) * nx + (py - h.y) * ny >= 0.0 { 1.0 } else { -1.0 };
                    set(c, ox - ux * ko + nx * sg * 2.5, GCY - uy * ko + ny * sg * 2.5);
                    job(c, "pres · kale önü".into());
                    c.ps[pi].sprint = true;
                    return;
                }
                let s = jmin(s, sm);
                set(c, h.x + ux * s, h.y + uy * s);
                job(c, "pres · şut hattı".into());
                c.ps[pi].sprint = true;
                return;
            }
            let hk = (h.vx * (ox - h.x) + h.vy * (GCY - h.y)) / {
                let l = hypot(ox - h.x, GCY - h.y);
                if l != 0.0 && l == l {
                    l
                } else {
                    1.0
                }
            };
            if hk > 0.0 && d0 < 4.0 {
                let vp = v_sp(&c.ps[pi], k);
                let tg = hypot(g.0 - px, g.1 - py) / jmax(0.02, vp - hk);
                let tc = hypot(co.0 - px, co.1 - py) / vp;
                if tc < tg {
                    set(c, co.0, co.1);
                    job(c, "pres · yandan giriyor".into());
                    return;
                }
            }
            if hk > 0.9 * v_sp(&c.ps[pi], k) {
                let ux = ox - lx;
                let uy = GCY - ly;
                let ul = {
                    let l = hypot(ux, uy);
                    if l != 0.0 && l == l {
                        l
                    } else {
                        1.0
                    }
                };
                let sx = -uy / ul;
                let sy = ux / ul;
                let sg = if (px - lx) * sx + (py - ly) * sy >= 0.0 { 1.0 } else { -1.0 };
                set(c, lx + sx * sg * 1.8 + ux / ul * 0.6, ly + sy * sg * 1.8 + uy / ul * 0.6);
                job(c, "pres · yanında".into());
                return;
            }
            let vx = g.0 - px;
            let vy = g.1 - py;
            let vl = {
                let l = hypot(vx, vy);
                if l != 0.0 && l == l {
                    l
                } else {
                    1.0
                }
            };
            let t = jmax(0.0, jmin(1.0, ((lx - px) * vx + (ly - py) * vy) / (vl * vl)));
            let cx = px + vx * t - lx;
            let cy = py + vy * t - ly;
            if hypot(cx, cy) < 2.5 && t > 0.0 && t < 1.0 {
                let nx = -vy / vl;
                let ny = vx / vl;
                let sg = if (px - lx) * nx + (py - ly) * ny >= 0.0 { 1.0 } else { -1.0 };
                set(c, lx + nx * sg * 3.5 + 0.5 * (g.0 - lx), ly + ny * sg * 3.5 + 0.5 * (g.1 - ly));
            } else {
                set(c, g.0, g.1);
            }
            let gs = match &T1 {
                Some((_, _, kk)) => {
                    let s = kk.clone().unwrap_or_else(|| "undefined".into());
                    !(s.contains("gönder") || s.contains("sür"))
                }
                None => false,
            };
            job(c, if gs { "pres · yolu kapat".into() } else { "pres · kale tarafına".into() });
            return;
        }
        {
            // JS `if (m.ps)`: kafanın ileri bakışındaki görünümde (vm, maç listesi yok, zar yok) önceki kararı okur
            // girmek mi yolu kesmek mi: taşıyıcı başına bir kez karar verilir (o ikili boyunca sürer); taşıyıcı değişince
            // yeniden tartılır. Eşikte her tik yeniden tartmak oyuncuyu ikisi arasında titretiyor, hiçbirini yapmıyordu
            let tik = c.m.tick;
            let suren = c.ps[pi].hal._onK.as_ref().filter(|o| o.h == h.id).map(|o| o.koy);
            let _ = (tik, Tepki::Karar);
            let koy = if let (Some(kk), true) = (suren, c.rnd.is_some()) {
                kk
            } else if c.rnd.is_some() {
                let (Pd, th) = {
                    let ps = &*c.ps;
                    (dis::duel_p(c.kafa, &ps[pi], &h, ps), dis::gir_thr(c.kafa, &ps[pi], &h, ps))
                };
                let koy = !(Pd >= th);
                c.ps[pi].hal_mut()._onK = Some(JpOnK { h: h.id, koy, t: tik });
                koy
            } else {
                c.ps[pi].hal._onK.as_ref().map_or(false, |o| o.h == h.id && o.koy)
            };
            if koy {
                let an = match &T1 {
                    Some(t) => t.0,
                    None => atan2(GCY - ly, ox - lx),
                };
                let kh = koy_hedef(&c.ps[pi], &h, cos(an), sin(an));
                set(c, kh.0, kh.1);
                job(c, format!("yolu kesiyor · {}", h.govde.name));
                let p = &mut c.ps[pi];
                p.sprint = hypot(p.tx - p.x, p.ty - p.y) > 1.5;
                return;
            }
        }
        if d0 > 4.0 {
            set(c, g.0, g.1);
            return;
        }
    }
    if d0 > 4.0 {
        let g = if trap { (h.x - 1.2 * dir_of(dt), h.y + if h.y < GCY { 2.0 } else { -2.0 }) } else { gside(h.x, h.y, dt, 2.0) };
        set(c, g.0, g.1);
        return;
    }
    let plan_bitti = match &c.ps[pi].pplan {
        None => true,
        Some(pl) => tick >= pl.until,
    };
    if plan_bitti {
        let shd = shielded(h.x, h.y, px, py, co.0, co.1);
        let base = atan2(py - h.y, px - h.x);
        let ca = atan2(cs.1, cs.0);
        let side = {
            let s = jsign(atan2(sin(ca - base), cos(ca - base)));
            if s != 0.0 && s == s {
                s
            } else {
                1.0
            }
        };
        let rr = c.zar();
        let fake = shd && rr < 0.08 * jmax(0.0, okp - 6.0);
        c.ps[pi].pplan = Some(if fake {
            JPPplan { k: "fake".into(), s: -side, t0: tick, until: tick + 2.0 * dis::DU_FAKE_T }
        } else {
            JPPplan { k: if trap { "iç".into() } else if shd { "yan".into() } else { "direkt".into() }, s: side, t0: tick, until: tick + 8.0 }
        });
    }
    let pl = c.ps[pi].pplan.clone().unwrap();
    let base = atan2(py - h.y, px - h.x);
    let s2 = if pl.k == "fake" {
        if tick - pl.t0 < dis::DU_FAKE_T {
            pl.s
        } else {
            -pl.s
        }
    } else {
        pl.s
    };
    if pl.k == "direkt" || (pl.k == "iç" && d0 < 2.4) {
        set(c, co.0, co.1);
    } else if pl.k == "iç" {
        set(c, h.x - 1.2 * dir_of(dt), h.y + if h.y < GCY { 1.6 } else { -1.6 });
    } else {
        set(c, h.x + 1.9 * cos(base + 1.25 * s2), h.y + 1.9 * sin(base + 1.25 * s2));
    }
    let mut ksb = false;
    if !trap && pl.k != "iç" {
        let an0 = match TH.and_then(|t| t.first()) {
            Some(t) => t.an,
            None => atan2(GCY - h.y, own_x(dt) - h.x),
        };
        let p = &mut c.ps[pi];
        p.hal_mut()._kAn = Some(an0);
        p.hal_mut()._kAnT = Some(tick);
        let rx = p.tx - h.x;
        let ry = p.ty - h.y;
        let r = jmax(1.3, hypot(rx, ry));
        let a = atan2(ry, rx);
        let dv = atan2(sin(a - an0), cos(a - an0));
        let mx = acos(jmin(1.0, 1.2 / jmax(1.9, r)));
        if dv.abs() > mx {
            let a2 = an0 + jsign(dv) * mx;
            let r2 = jmax(1.9, r);
            p.tx = h.x + cos(a2) * r2;
            p.ty = h.y + sin(a2) * r2;
            ksb = true;
        }
    }
    let j = if pl.k == "fake" {
        if tick - pl.t0 < dis::DU_FAKE_T {
            "fake: gösteriyor"
        } else {
            "fake: giriyor"
        }
    } else if pl.k == "iç" {
        "kenara itiyor"
    } else if ksb {
        "pres · kale tarafında bekliyor"
    } else {
        "pres"
    };
    c.ps[pi].job = Some(j.into());
}
