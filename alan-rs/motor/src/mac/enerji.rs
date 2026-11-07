//! Enerji ve sprint (core.js 66–200'ün maçın kullandığı kısmı; match.js 311–716, 2109–2333).
//!
//! JS'te sprint katı küresel `self.ALAN_SPR`'dır ve match.js onu sprint evresine göre sarar;
//! burada `ALAN_SPR(p, k)` açık işlev, maçın bağlamı `MacKural` (çekirdeğin `temas::Kural`'ı).
//! Bugünkü bayraklar: Enerji 2/3 açık, sürekli koşu açık (ALAN_SUREKLI), sprint evresi açık (SPR_FAZ),
//! dokunuşla sürme kapalı.
pub use crate::mac_veri::EnSayac;
use super::*;
use crate::mat::{exp, hypot, jmax, jmin, pow};
use crate::model::JMSt;
use crate::yetenek::mek;
pub use crate::kural::enerji::*;

/// Maçın sprint bağlamı (eskiden `MacKural::of`; Aşama 7c: tip kural katmanında, kurucusu maçta).
pub fn mac_kurali(m: &Mac) -> MacKural {
    MacKural { niyet: None, mh: m.mac.__m_h, tick: m.tick, holder: m.holder.map(|h| kim(&m.ps[h])), flq: m.fl.as_ref().and_then(|f| f.q).map(|q| kim(&m.ps[q])) }
}


// ---- sayaçlar ----


fn ekle(s: &mut EnSayac, k: &str, v: f64) {
    if let Some(e) = s.iter_mut().find(|e| e.0 == k) {
        e.1 += v;
    } else {
        s.push((k.to_string(), v));
    }
}

/// JS enSay(m, p, k, v)
pub fn en_say_p(st: &mut JMSt, lite: bool, team: f64, _pi: Pid, k: &str, v: f64) {
    if lite || !(v > 0.0) {
        return;
    }
    if st.en_h.is_empty() {
        let z: EnSayac = ["sprint", "ivme", "ikili", "alan", "sarj", "dol"].iter().map(|k| (k.to_string(), 0.0)).collect();
        st.en_h = vec![z.clone(), z];
    }
    ekle(&mut st.en_h[team as usize], k, v);
}
pub fn en_say(m: &mut Mac, pi: Pid, k: &str, v: f64) {
    if crate::iz_acik() && !m.mac._lite && v > 0.0 && &*m.ps[pi].govde.name == "Cansu" && std::env::var("ALAN_EN_IZ").ok().and_then(|x| x.parse::<f64>().ok()).map_or(false, |x| m.tick >= x - 1.0 && m.tick <= x) { eprintln!("EN {} {} {} {}", m.tick, m.ps[pi].govde.name, k, v); }
    let t = m.ps[pi].team;
    en_say_p(&mut m.m.st, m.mac._lite, t, pi, k, v);
}

/// JS verim(p, ...oz): eski özelliklerin ortalamasıyla harcama katı
pub fn verim(p: &Oyuncu, oz: &[&str]) -> f64 {
    let mut s = 0.0;
    for o in oz {
        s += p.govde.yet.ozk(o);
    }
    jmax(0.5, jmin(1.4, 1.0 - 0.04 * (s / oz.len() as f64 - 10.0)))
}

/// JS enHarca(m, p, k, v)
pub fn en_harca_p(st: &mut JMSt, lite: bool, p: &mut Oyuncu, pi: Pid, k: &str, v: f64) {
    let Some(en) = p.en else { return };
    if !(v > 0.0) {
        return;
    }
    let d = jmin(en, v / alan_en2_sb(p));
    en_say_p(st, lite, p.team, pi, k, d);
    p.en = Some(en - d);
}
pub fn en_harca(m: &mut Mac, pi: Pid, k: &str, v: f64) {
    if crate::iz_acik() && !m.mac._lite && &*m.ps[pi].govde.name == "Cansu" && std::env::var("ALAN_EN_IZ").ok().and_then(|x| x.parse::<f64>().ok()).map_or(false, |x| m.tick >= x - 1.0 && m.tick <= x) { eprintln!("ENH {} {} {} {} en {:?}", m.tick, m.ps[pi].govde.name, k, v, m.ps[pi].en); }
    let lite = m.mac._lite;
    en_harca_p(&mut m.m.st, lite, &mut m.m.ps[pi], pi, k, v);
}

/// JS enerji2(m) (Enerji 3): sprint harcar, dinlenme doldurur; bar alan gücünü (D) ölçekler.
pub fn enerji2(m: &mut Mac) {
    let hl = m.holder;
    let b0 = m.ball.as_ref().map(|b| (b.team as f64, b.done.is_some()));
    let tick = m.tick;
    let lite = m.mac._lite;
    for pi in 0..m.ps.len() {
        if m.ps[pi].en.is_none() {
            m.ps[pi].en = Some(1.0);
        }
        let p = &m.ps[pi];
        let Sb = alan_en2_sb(p);
        let Sd = alan_en2_sd(p);
        let vx = or0(p.vx);
        let vy = or0(p.vy);
        let v = hypot(vx, vy);
        let b = spd(p);
        let vmx = b * alan_spr_h(p.govde.yet.hiz);
        let mut d = 0.0;
        let mut st_ek: Vec<(&str, f64)> = vec![];
        if v > b {
            let ds = en2.tepe * pow(jmin(1.0, (v - b) / jmax(1e-6, vmx - b)), 1.0);
            d += ds;
            st_ek.push(("sprint", ds / Sb));
        }
        let r = v / b;
        let act = if v > b { 0.0 } else if r <= 0.6 { 1.0 - 0.5 * r } else { jmax(en2.tempo_dol, 0.7 - 1.125 * (r - 0.6)) };
        let y0 = p.hal._yog;
        let yog = y0 + (d / Sb - y0) / en2.yog_tau;
        let _yog_f = jmax(en2.yog_min, 1.0 - en2.yog_k * yog / (en2.tepe / Sb));
        let _ = (Sd, act);
        let mut dol;
        if v > 1.02 * b && !p.bekci() {
            dol = 0.0;
        } else {
            let sahip = match hl {
                Some(h) => m.ps[h].team == p.team,
                None => matches!(b0, Some((t, false)) if t == p.team),
            };
            let mut nd = 1e9;
            for q in &m.ps {
                if !(q.team == p.team || q.noTouch) {
                    nd = jmin(nd, hypot(q.x - p.x, q.y - p.y));
                }
            }
            let d_t = hypot(p.tx - p.x, p.ty - p.y);
            let yer_k = if sahip { 0.0 } else { jmax(0.0, 1.0 - d_t / EN3.yer_d) * jmax(0.0, 1.0 - v / b) };
            // yürü: dolum hıza bağlı (hedefe uzaklıktan bağımsız): yavaş hareket eden barını hızlı doldurur
            let yer_k = if yuru() && !sahip { jmax(yer_k, jmax(0.0, 1.0 - v / b)) } else { yer_k };
            dol = alan_en3_dol(p, sahip, nd, yer_k);
        }
        if gt(p.hal._dolYok, tick) {
            dol = 0.0;
        }
        let en = p.en.unwrap();
        st_ek.push(("dol", jmin(dol, 1.0 - en)));
        let team = p.team;
        for (k, x) in st_ek {
            en_say_p(&mut m.st, lite, team, pi, k, x);
        }
        let p = &mut m.ps[pi];
        p.hal_mut()._evx = vx;
        p.hal_mut()._evy = vy;
        p.hal_mut()._yog = yog;
        let en = jmax(0.0, jmin(1.0, en - d / Sb + dol));
        p.en = Some(en);
        if p.hal._Dl.map_or(true, |dl| p.D != dl) {
            p.hal_mut()._D0 = Some(p.D);
        }
        let dd = p.hal._D0.unwrap_or(f64::NAN) * en;
        p.D = dd;
        p.hal_mut()._Dl = Some(dd);
        if !lite {
            let bos = en < 0.1 && p.hal._enOnce.unwrap_or(1.0) >= 0.1 && !gt(p.hal._bosT, tick - 80.0);
            p.hal_mut()._enOnce = Some(en);
            if bos {
                p.hal_mut()._bosT = Some(tick);
                let (t, ad, bek) = (p.team, p.govde.name.clone(), p.bekci());
                ev(m, "pas", t, if bek { format!("Bekçi'nin barı boş · {ad}") } else { format!("Barı boş · {ad}") });
            }
        } else if p.hal._Dl.is_some() && Some(p.D) == p.hal._Dl {
            p.D = p.hal._D0.unwrap_or(f64::NAN);
            p.hal_mut()._Dl = None;
        }
    }
}
/// JS enerji(m) (Enerji 2 açık: enerji2)
pub fn enerji(m: &mut Mac) {
    enerji2(m)
}

/// JS enCal(m, kaz, kay, sert): ikiliyi kazanan kaybedenden bar çalar.
pub fn en_cal(m: &mut Mac, kaz: Pid, kay: Pid, sert: f64) {
    if kaz == kay {
        return;
    }
    let (a, b) = (&m.ps[kaz], &m.ps[kay]);
    if a.team == b.team || a.bekci() || b.bekci() || a.en.is_none() || b.en.is_none() {
        return;
    }
    let c = jmin(b.en.unwrap(), 0.05 + 0.05 * jmax(0.0, jmin(1.0, sert)));
    if !(c > 0.0) {
        return;
    }
    m.ps[kay].en = Some(m.ps[kay].en.unwrap() - c);
    en_say(m, kay, "calindi", c);
    let al = jmin(c, 1.0 - m.ps[kaz].en.unwrap());
    m.ps[kaz].en = Some(m.ps[kaz].en.unwrap() + al);
    en_say(m, kaz, "caldi", al);
    let t = m.ps[kaz].team;
    if !m.mac._lite {
        let (a, b) = (m.ps[kaz].govde.name.clone(), m.ps[kay].govde.name.clone());
        ev(m, "pas", t, format!("{a}, {b}'den bar çaldı"));
    }
    if m.st.en_cal.is_empty() {
        m.st.en_cal = vec![0.0, 0.0];
    }
    m.st.en_cal[t as usize] += c;
}
/// JS sertT(a, b): çarpışmanın sertliği
pub fn sert_t(a: &Oyuncu, b: &Oyuncu, k: &MacKural) -> f64 {
    hypot(or0(a.vx) - or0(b.vx), or0(a.vy) - or0(b.vy)) / jmax(0.05, spd(a) * alan_spr(a, k))
}

// ---- alan seçimi ----

const ALAN_YON: f64 = 2.0;

/// JS alanYz(m, p, t0): alanın yönünü seçerken yanılma (baskı altında büyür).
pub fn alan_yz(m: &Mac, pi: Pid, t0: f64) -> (f64, f64) {
    let p = &m.ps[pi];
    let bb = jmin(1.0, gecici_dis::press_at(&m.ps, p.team, p.x, p.y));
    let w = ALAN_YON * (20.0 - p.govde.yet.y(mek::ALAN_YON) + 10.0 * crate::yetenek::DK_SUT.b * bb * crate::yetenek::yum(&p.govde.yet, "B")) / 10.0;
    if !(w > 0.0) {
        return (0.0, 0.0);
    }
    let z = |c: f64| (crate::yetenek::zar(p.id, t0, c) + crate::yetenek::zar(p.id + 101.0, t0, c) - 1.0) * w;
    (z(1.0), z(2.0))
}

/// JS alanSec(m): boştaki topa alan açma/kapama seçimi (am: +1 hızlandır, −1 yavaşlat) ve bedeli.
pub fn alan_sec(m: &mut Mac) {
    let ok = matches!(&m.ball, Some(b) if m.holder.is_none() && b.done.is_none());
    if !ok {
        for p in m.ps.iter_mut() {
            p.hal_mut().amA = false;
            p.hal_mut().am = Some(0.0);
        }
        return;
    }
    let b = m.ball.as_ref().unwrap();
    let (bx, by, bt) = (b.x, b.y, b.team as f64);
    let bdefl = dogru(b.defl) || b.defl_b;
    let fr = fr_top(m);
    let fl = m.fl.as_ref();
    let sut = fl.map_or(false, |f| f.kind == "gönder") && !bdefl;
    let q = if !sut && !bdefl { fl.and_then(|f| f.q) } else { None };
    let t0 = fl.map_or(m.tick, |f| f.t0);
    let (mut o_e, mut rq_e, mut rq_t, mut tE) = (1e9, 1e9, 0.0, [1e9, 1e9]);
    let n = fr.as_ref().map_or(0.0, |f| f.pts.len() as f64);
    if let Some(fr) = &fr {
        for (pi, r) in &fr.arr {
            let p = &m.ps[*pi];
            if !p.noTouch {
                if p.team != bt {
                    o_e = jmin(o_e, r.est);
                }
                tE[p.team as usize] = jmin(tE[p.team as usize], r.est);
                if Some(*pi) == q {
                    rq_e = r.est;
                    rq_t = r.t;
                }
            }
        }
    }
    let yz = !m.mac._lite;
    let (mut sp, mut sn) = (0.0, 0.0);
    let ts = crate::oyuncu::TepkiS::default();
    for pi in 0..m.ps.len() {
        let rt = jmax(2.0, crate::oyuncu::tepki(&m.ps[pi], crate::oyuncu::Tepki::Alan, &ts));
        if m.tick - t0 < rt || m.ps[pi].noTouch {
            let p = &mut m.m.ps[pi];
            p.hal_mut().amA = m.m.tick - t0 >= rt;
            p.hal_mut().am = Some(0.0);
            continue;
        }
        let f0 = m.ps[pi].etki().infl(bx, by);
        if f0 < 0.02 {
            m.ps[pi].hal_mut().amA = true;
            m.ps[pi].hal_mut().am = Some(0.0);
            continue;
        }
        let r = fr.as_ref().and_then(|f| f.arr.iter().find(|x| x.0 == pi)).is_some();
        let (e1, e2) = if yz { alan_yz(m, pi, t0) } else { (0.0, 0.0) };
        let p = &m.ps[pi];
        let pt = p.team as usize;
        let son = n != 0.0 && rq_t + e2 >= n - 3.0;
        let a = if sut {
            if p.team == bt { 1.0 } else { -1.0 }
        } else if q.is_some() {
            if p.team == bt {
                if rq_e >= 1e8 || son { -1.0 } else if rq_e + e1 >= o_e - 2.0 { 1.0 } else if son { -1.0 } else { 0.0 }
            } else if r && tE[pt] + e1 <= rq_e + 4.0 {
                -1.0
            } else if son {
                1.0
            } else {
                0.0
            }
        } else if tE[pt] + e1 < tE[1 - pt] {
            -1.0
        } else {
            0.0
        };
        let dd = p.D;
        let p = &mut m.ps[pi];
        p.hal_mut().amA = true;
        p.hal_mut().am = Some(a);
        if a > 0.0 {
            sp += f0 * dd;
        } else if a < 0.0 {
            sn += f0 * dd;
        }
    }
    let d_s = |s: f64| if s <= 1.0 { s } else if s <= 2.0 { 1.0 + 0.5 * (s - 1.0) } else { 1.5 + 0.25 * (s - 2.0) };
    for pi in 0..m.ps.len() {
        let am = oo(m.ps[pi].hal.am);
        if am == 0.0 {
            continue;
        }
        let p = &m.ps[pi];
        let f = p.etki().infl(bx, by);
        let s = if am > 0.0 { sp } else { sn };
        let pay = if s > 0.0 { d_s(s) / s } else { 1.0 };
        let c = EN.alan * f * pay / alan_en2_sb(p) * p.govde.yet.verim(mek::ALAN_BEDEL);
        let e = p.en.unwrap_or(1.0);
        en_say(m, pi, "alan", jmin(e, c));
        m.ps[pi].en = Some(jmax(0.0, e - c));
    }
}

// ---- sprint kararı ve evreleri ----

fn sp_topa(j: &str) -> bool {
    bas(j, &["alıcı", "kovala", "uzanıyor", "karşıla", "sekme", "yol kes"])
}
fn sp_zorla(j: &str) -> bool {
    bas(j, &["adandı", "omuz"])
}
pub fn sf_kesin_re(j: &str) -> bool {
    bas(j, &["alıcı", "uzanıyor", "omuz", "karşıla", "kovala", "geçildi", "pres · kale tarafına", "çalım", "fake"])
}

/// Boştaki topun uçuş tahmini (JS `m.sh.fr`, yalnız top boştayken ve bu topunsa). Yerleşim yazar
/// (`m.mac.sekil.sh`); maç okur.
pub fn fr_al(m: &Mac) -> Option<crate::model::JMShFr> {
    if m.holder.is_some() {
        return None;
    }
    fr_top(m)
}
/// JS `m.sh.fr` (`m.sh.frB === m.ball` ise): yerleşimin uçuş okuması maçın biçiminde.
pub fn fr_top(m: &Mac) -> Option<crate::model::JMShFr> {
    let b = m.ball.as_ref()?;
    let sh = &m.mac.sekil.sh;
    if sh.fr_b != Some(b.kimlik()) {
        return None;
    }
    let fr = sh.fr.as_ref()?;
    Some(crate::model::JMShFr {
        arr: fr.arr.iter().enumerate().map(|(i, a)| (i, crate::model::JMShFrArrV { t: a.t, x: a.x, y: a.y, est: a.est, direnc: a.direnc, ..Default::default() })).collect(),
        first: fr.first.unwrap_or(0),
        pts: fr.pts.iter().map(|q| crate::model::JMShFrPtsE { x: q.x, y: q.y, e: q.e, ..Default::default() }).collect(),
    })
}

/// hiz2: koşu değerinin varışla azalması (tik): her zaman (TA) ve tehlike anından sonra (TB).
pub const HIZ2_TA: f64 = 60.0;
pub const HIZ2_TB: f64 = 15.0;
/// hiz2: tehlike anı: taşıyıcının şut uzaklığı (br), kaleye yaklaşma hızı (br/tik), en erken (tik).
pub const HIZ2_SUT_D: f64 = 18.0;
pub const HIZ2_TASI_V: f64 = 0.3;
pub const HIZ2_TEH_MIN: f64 = 8.0;
/// hiz2: kontrada savunmacıya karşı varış payının yumuşaklığı (tik): erken varmak her savunmacıya karşı payı sürekli büyütür.
pub const HIZ2_KONTRA_W: f64 = 20.0;
/// hiz2: kontranın değeri, topun önüne dönebilmiş savunmacı sayısına göre (analiz/ZAMAN-DEGERI.md, 38 maç; aralar doğrusal).
pub const KONTRA_GOL: [f64; 4] = [0.73, 0.50, 0.33, 0.0];
pub fn kontra_deger(n: f64) -> f64 {
    let n = jmax(0.0, n);
    let i = n.floor() as usize;
    if i >= KONTRA_GOL.len() - 1 {
        return KONTRA_GOL[KONTRA_GOL.len() - 1];
    }
    let f = n - i as f64;
    KONTRA_GOL[i] + (KONTRA_GOL[i + 1] - KONTRA_GOL[i]) * f
}
/// hiz2: barın değer katı (bedel = HIZ2_BAR · ((1−e+c)² − (1−e)²)).
pub const HIZ2_BAR: f64 = 0.5;
/// Deney (ALAN_HIZ2_IZ; ölçümde/wasm'da HIZ2_ZORLA; analiz/HIZ-TERCIHI.md): hız seçiminde koşunun değeri varış süresiyle sürekli
/// azalır, savunmada yarış tehlike anına karşı ölçülür, barın değeri hedeften bağımsızdır. Varsayılan kapalı.
pub static HIZ2_ZORLA: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub fn hiz2() -> bool {
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    HIZ2_ZORLA.load(std::sync::atomic::Ordering::Relaxed) || *A.get_or_init(|| crate::iz_acik() && std::env::var("ALAN_HIZ2_IZ").is_ok())
}

/// yürü: en yavaş kat (normal koşunun).
pub const YURU_MIN: f64 = 0.35;
/// Deney (ALAN_YURU_IZ; ölçümde/wasm'da YURU_ZORLA): hız seçimine normal koşunun altı (yürüme) girer ve bar dolumu hıza
/// bağlanır (yavaş hareket eden hızlı dolar). Varsayılan kapalı.
pub static YURU_ZORLA: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub fn yuru() -> bool {
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    YURU_ZORLA.load(std::sync::atomic::Ordering::Relaxed) || *A.get_or_init(|| crate::iz_acik() && std::env::var("ALAN_YURU_IZ").is_ok())
}
/// yürü: taşıyıcı ya da topun alıcısı mı (onlar yürümeyi seçmez).
fn bizde_sahip(m: &Mac, pi: Pid) -> bool {
    m.holder == Some(pi) || m.fl.as_ref().map_or(false, |f| f.q == Some(pi))
}

/// onde_ac: önde bekleyenin yürüme katı (normal koşunun).
pub const ONDE_YURU: f64 = 0.5;

/// JS sprintKarar3(m): her koşucu için koşu hızı (tempo → sprint arası sürekli kat, _hizK).
pub fn sprint_karar3(m: &mut Mac) {
    let F = &SPR_FAZ;
    let h = m.holder;
    let fr = fr_al(m);
    let a_f = gecici_dis::D::arriveFirm;
    let wf = |T: f64, tD: f64| if tD >= 1e8 { 1.0 } else { 1.0 / (1.0 + exp((T - tD) / 5.0)) };
    let kk = mac_kurali(m);
    // topun son el değiştirdiği an (kontra değer tablosunun 'top kazanılalı geçen süre'si)
    if let Some(hh) = h {
        let t = m.ps[hh].team;
        if m.mac.kazan.map_or(true, |k| k.0 != t) {
            m.mac.kazan = Some((t, m.tick));
        }
    }
    for pi in 0..m.ps.len() {
        {
            let p = &mut m.ps[pi];
            p.hal_mut()._topSpr = false;
            p.hal_mut()._kosuK = None;
            p.hal_mut()._hizK = None;
        }
        if m.ps[pi].hal._itK.is_some() {
            let top_ben = h.is_none() && m.fl.as_ref().map_or(false, |f| f.q == Some(pi)) && m.ball.as_ref().map_or(false, |b| b.tt.t.from == Some(pi));
            if spk() && top_ben {
                let p = &mut m.ps[pi];
                p.hal_mut()._hizK = p.hal._itK;
                p.sprint = p.hal._itK.unwrap() > 1.001;
                p.hal_mut()._kosuK = Some("it".into());
                continue;
            }
            m.ps[pi].hal_mut()._itK = None;
        }
        let p = &m.ps[pi];
        let sS = p._sf.as_ref().map_or(false, |s| s.f == "patlama" || s.f == "tepe");
        if Some(pi) == h || p.bekci() || p.en.is_none() || p.noTouch || kk.spr_kar(p) {
            continue;
        }
        let j = job(p).to_string();
        // onde_ac: önde bekleyen, rakip topu tutarken acelesi yok: yürür (bar dolar); top havadayken (kazanılabilir) normal hız
        if crate::sekil::konum::onde_ac() && j == "önde bekle" && h.map_or(false, |h| m.ps[h].team != p.team) {
            let p = &mut m.ps[pi];
            p.hal_mut()._hizK = Some(ONDE_YURU);
            p.sprint = false;
            p.hal_mut()._kosuK = Some("yürü".into());
            continue;
        }
        if sp_zorla(&j) {
            let p = &mut m.ps[pi];
            p.hal_mut()._topSpr = true;
            p.hal_mut()._kosuK = Some("zorunlu".into());
            continue;
        }
        let r = fr.as_ref().and_then(|f| f.arr.iter().find(|x| x.0 == pi).map(|x| x.1.clone()));
        let topa = r.is_some() && sp_topa(&j);
        let (tx, ty) = if topa { (r.as_ref().unwrap().x, r.as_ref().unwrap().y) } else { (p.tx, p.ty) };
        let d = hypot(tx - p.x, ty - p.y);
        if d < 1.5 {
            m.ps[pi].sprint = false;
            continue;
        }
        let bizde = match h {
            Some(h) => m.ps[h].team == p.team,
            None => m.ball.as_ref().map_or(false, |b| b.team as f64 == p.team && b.done.is_none()),
        };
        let mut tD = 1e9;
        if topa {
            for (qi, rq) in &fr.as_ref().unwrap().arr {
                let q = &m.ps[*qi];
                if q.team != p.team && !q.noTouch && rq.est < tD {
                    tD = rq.est;
                }
            }
            tD = jmin(tD, r.as_ref().unwrap().t);
        } else if bizde {
            if let Some(h) = h {
                let hp = &m.ps[h];
                tD = jmax(0.0, oo(Some(m.dec_t))) + gecici_dis::ucus_t(m, hypot(hp.x - tx, hp.y - ty), a_f);
            }
        } else {
            for q in &m.ps {
                if q.team == p.team || q.noTouch || q.bekci() {
                    continue;
                }
                let l = hypot(q.x - tx, q.y - ty);
                tD = jmin(tD, alan_kosu_t(q, l, spd(q), &kk));
            }
        }
        let b = spd(p);
        let Sb = alan_en2_sb(p);
        let vm = b * alan_spr_h(p.govde.yet.hiz);
        let hk = hk_k(p, SPR0(p));
        let pat = sbed();
        let e = jmax(0.0, jmin(1.0, p.en.unwrap_or(1.0)));
        let nd = {
            let mut nd = 1e9;
            for q in &m.ps {
                if !(q.team == p.team || q.noTouch) {
                    nd = jmin(nd, hypot(q.x - p.x, q.y - p.y));
                }
            }
            nd
        };
        let dol = alan_en3_dol(p, bizde, nd, 0.0);
        // yürü: normal koşunun altındaki katta tik başına fazladan dolum (enerji2'deki hıza bağlı dolumla aynı)
        let yuru_ek = |k: f64| -> f64 { alan_en3_dol(p, bizde, nd, jmax(0.0, jmin(1.0, 1.0 - k))) - dol };
        let yuru_a = yuru() && !bizde_sahip(m, pi);
        let dK = dol;
        let pk = sf_pk(p);
        let s0 = SPR0(p);
        let yol = |e0: f64, sur: bool| -> [(f64, f64); 3] {
            let T0 = d / b;
            let T1 = d / (b * hk);
            let c1 = T1 * (en2.tepe * pow(jmin(1.0, (b * hk - b) / jmax(1e-6, vm - b)), 1.0) / Sb + dK);
            let pa = if sur { 0.0 } else { pat };
            let (mut T2, mut c2) = (1e9, 1e9);
            if e0 >= (if sur { 0.0 } else { alan_en3_sgir(p, pa) }) {
                let ts = alan_en2_ts(p, Some(e0 - pa));
                let v_s = b * alan_en3_tepe(Some(0.5 * jmax(0.0, e0 - pa) + 0.5 * jmax(0.0, e0 - pa - jmin(ts, d / (b * pk)) * en2.tepe / Sb)), if F.ust { s0 } else { s0 });
                let rc = v_s * ts;
                let t_s = jmin(ts, d / v_s);
                T2 = if d <= rc { d / v_s } else { ts + (d - rc) / b };
                c2 = pa + t_s * (en2.tepe / Sb + dK);
            }
            [(T0, 0.0), (T1, jmin(c1, e0)), (T2, c2)]
        };
        let en_iyi = |e0: f64| {
            let mut w = 0.0;
            for (T, _) in yol(e0, false) {
                w = jmax(w, wf(T, tD));
            }
            w
        };
        let T0 = d / b;
        let TH = |t: f64, x: f64, y: f64| gecici_dis::threat(t, x, y);
        let Sy = if topa || !bizde { TH(p.team, tx, ty) + TH(1.0 - p.team, tx, ty) } else { TH(p.team, tx, ty) };
        let bxy = match h {
            Some(h) => Some((m.ps[h].x, m.ps[h].y)),
            None => m.ball.as_ref().map(|b| (b.x, b.y)),
        };
        let ku = |x: f64| alan_en3_kul(p, x, bizde, bxy, &kk);
        let tac_t = &m.tac[p.team as usize];
        let hr_k = jmax(0.0, jmin(1.0, tac_t.enHarcama.unwrap_or(0.5))) - 0.5 + 1.0;
        let sd_p = alan_en2_sd(p);
        // hiz2: savunmada (rakip topta, topun peşinde değil) yarış, taşıyıcının şut uzaklığına gireceği ana karşı
        let tD2 = if hiz2() && !bizde && !topa {
            match h {
                Some(hh) if m.ps[hh].team != p.team => {
                    let hp = &m.ps[hh];
                    let gx = if p.team == 0.0 { 0.0 } else { W };
                    let lh = hypot(hp.x - gx, hp.y - crate::saha::H / 2.0);
                    jmax(tD, jmax(HIZ2_TEH_MIN, (lh - HIZ2_SUT_D) / HIZ2_TASI_V))
                }
                _ => tD,
            }
        } else {
            tD
        };
        // hiz2: hücumda son tarih yoksa (pas havada, kontra) koşunun değeri, varışta henüz dönememiş rakip savunmacıların payı:
        // her tik erken = geri dönemeyen bir savunmacı daha (yalnız en yakınının varışı değil)
        // kontra tablosunun girdileri: hedefin rakip kaleye uzaklığı, top kazanılalı geçen süre (sn, şimdi)
        let ktab_uzak = hypot(tx - if p.team == 0.0 { W } else { 0.0 }, ty - crate::saha::H / 2.0);
        let ktab_t0 = m.mac.kazan.filter(|k| k.0 == p.team).map_or(0.0, |k| (m.tick - k.1) / crate::saha::TPS);
        let kontra: Option<Vec<f64>> = if hiz2() && bizde && !topa && (tD >= 1e8 || j.starts_with("kontra")) {
            let v: Vec<f64> = m.ps.iter().filter(|q| q.team != p.team && !q.noTouch && !q.bekci()).map(|q| alan_kosu_t(q, hypot(q.x - tx, q.y - ty), spd(q), &kk)).collect();
            if v.is_empty() { None } else { Some(v) }
        } else {
            None
        };
        let DEG = |T: f64, c: f64| -> f64 {
            if hiz2() {
                // hiz2: koşunun değeri varışla sürekli azalır (erken varmak hep biraz iyi, geç kalmak giderek kötü, sıfır
                // değil); barın değeri hedeften bağımsız: harcamanın birim bedeli bar azaldıkça artar (dolu barda da sıfır değil),
                // yürümenin kazancı aynası
                let g = 1.0 / (1.0 + T / HIZ2_TA) / (1.0 + jmax(0.0, T - tD2) / HIZ2_TB);
                // kontra: durumun kendi değeri (analiz/ZAMAN-DEGERI.md): varışta topun önüne dönebilmiş savunmacı sayısı
                // (varış süreleriyle yumuşak sayım) → gol olasılığı (0: %73, 1: %50, 2: %33, 3+: %0); pas gelmeden varmak
                // bir şey katmaz (varış en erken pasın varışı)
                let vk = match &kontra {
                    Some(v) => {
                        let Tv = if tD < 1e8 { jmax(T, tD) } else { T };
                        let n: f64 = v.iter().map(|&t| 1.0 / (1.0 + exp((t - Tv) / HIZ2_KONTRA_W))).sum();
                        // ALAN_KTAB_IZ: değer tablosundan (kaleye uzaklık × savunmacı × top kazanılalı geçen süre)
                        match super::kontra_tablo::tablo().filter(|_| super::kontra_tablo::ktab()) {
                            Some(tb) => tb.oku(ktab_uzak, n, ktab_t0 + Tv / crate::saha::TPS, &|j| kontra_deger(j)),
                            None => kontra_deger(n),
                        }
                    }
                    None => 0.0,
                };
                // yürümenin kazancı barın boş kısmını aşamaz (dolu barda yürümek bir şey kazandırmaz)
                let c = if c < 0.0 { jmax(c, -(1.0 - e)) } else { c };
                // barın değeri dışbükey: birim değer barın boş kısmıyla artar (2·(1−x)); bedel e−c'den e'ye integral:
                // dolu barda kısa harcama neredeyse bedava (fazlası boşa gidiyordu), uzun harcama giderek pahalı
                let bos = jmax(0.0, 1.0 - e);
                let bedel = HIZ2_BAR * ((bos + c) * (bos + c) - bos * bos) * hr_k;
                return Sy * g + vk - bedel;
            }
            let mut bd = 0.0;
            if c > 0.0 {
                let d_e = jmax(dol, EN3.r0 * sd_p) * jmax(1.0, T0);
                for i in 1..=20 {
                    let g = d_e * i as f64;
                    if g >= c {
                        break;
                    }
                    let e_a = jmin(1.0, e + g);
                    let e_b = jmin(1.0, e - c + g);
                    bd += Sy * (en_iyi(e_a) - en_iyi(e_b)) + ku(e_a) - ku(e_b);
                }
            }
            if c < 0.0 {
                // yürü: kazanılan bar (c < 0), harcamanın aynası: fark dolumla kapanana dek her adımda ilerideki yetişme
                // ve barın kendi değeri
                let d_e = jmax(dol, EN3.r0 * sd_p) * jmax(1.0, T0);
                for i in 0..20 {
                    let g = d_e * i as f64;
                    if g >= -c {
                        break;
                    }
                    let e_a = jmin(1.0, e - c + g);
                    let e_b = jmin(1.0, e + g);
                    bd -= Sy * (en_iyi(e_a) - en_iyi(e_b)) + ku(e_a) - ku(e_b);
                }
            }
            bd *= hr_k;
            Sy * wf(T, tD) - bd
        };
        // SPK: sürekli kat
        let kM = jmax(1.0, pk);
        let c_k = |f: f64| -> (f64, f64, f64) {
            if f < 0.0 {
                // yürü: f −1'de YURU_MIN katı; bedel eksi (kazanç), varışa dek fazladan dolum
                let k = 1.0 + f * (1.0 - YURU_MIN);
                let T = d / (b * k);
                return (k, T, -T * jmax(0.0, yuru_ek(k)));
            }
            let k = 1.0 + f * (kM - 1.0);
            let T = d / (b * k);
            (k, T, if f > 0.0 { T * (en2.tepe * jmin(1.0, (b * k - b) / jmax(1e-6, vm - b)) / Sb + dK) } else { 0.0 })
        };
        let (mut fB, mut v_b) = (0.0, -1e9);
        let dene = |f: f64, fB: &mut f64, v_b: &mut f64| {
            if !((if yuru_a { -1.0 } else { 0.0 })..=1.0).contains(&f) {
                return;
            }
            let (_, T, c) = c_k(f);
            if c > e + 1e-9 {
                return;
            }
            let V = DEG(T, c);
            if V > *v_b + 1e-12 {
                *v_b = V;
                *fB = f;
            }
        };
        for f in [0.0, 0.2, 0.4, 0.6, 0.8, 1.0] {
            dene(f, &mut fB, &mut v_b);
        }
        if yuru_a {
            for f in [-1.0, -0.6, -0.3] {
                dene(f, &mut fB, &mut v_b);
            }
        }
        let f0 = fB;
        dene(f0 - 0.1, &mut fB, &mut v_b);
        dene(f0 + 0.1, &mut fB, &mut v_b);
        let f1 = fB;
        dene(f1 - 0.05, &mut fB, &mut v_b);
        dene(f1 + 0.05, &mut fB, &mut v_b);
        let (k_s, _, _) = c_k(fB);
        let _ = sS;
        if crate::iz_acik() && std::env::var("ALAN_K3_IZ").ok().as_deref() == Some(&format!("{}:{}", m.tick, pi)) {
            eprintln!("K3 tD={tD} tD2={tD2} kontra={kontra:?} sS={sS} e={e} dol={dol} topa={topa} bizde={bizde} j={j} f={fB} b={b} vm={vm} hk={hk} kM={kM} Sy={Sy} decT={}", m.dec_t);
        }
        let p = &mut m.ps[pi];
        p.hal_mut()._hizK = Some(k_s);
        p.hal_mut()._hizF = Some(fB);
        p.hal_mut()._kosuK = Some(if fB < -0.05 { "yürü" } else if fB < 0.05 { "tempo" } else if fB < 0.7 { "hızlı" } else { "sprint" }.into());
        p.sprint = fB >= 0.05;
        p.hal_mut()._topSpr = false;
    }
}

/// JS sprintBasla(m, p, F, yazi)
pub fn sprint_basla(m: &mut Mac, pi: Pid, yazi: Option<String>) {
    m.ps[pi]._sf = Some(crate::model::JpSf { t0: m.tick, f: "patlama".into() });
    if let Some(e) = m.ps[pi].en {
        en_say(m, pi, "sprint", sbed());
        m.ps[pi].en = Some(jmax(0.0, e - sbed()));
    }
    let t = m.ps[pi].team as usize;
    if m.st.spr_f.is_empty() {
        m.st.spr_f = vec![0.0, 0.0];
    }
    m.st.spr_f[t] += 1.0;
    if m.st.spr_t.is_empty() {
        m.st.spr_t = vec![0.0, 0.0];
    }
    m.st.spr_t[t] += 1.0;
    if !m.mac._lite {
        if let Some(y) = yazi {
            ev(m, "pas", t as f64, y);
        }
    }
}

/// JS sfKesin(m, p, F): sprinte çıkmak gerçekten gerekli mi (yarış).
pub fn sf_kesin(m: &Mac, pi: Pid) -> bool {
    let F = &SPR_FAZ;
    let p = &m.ps[pi];
    let j = job(p);
    if !sf_kesin_re(j) {
        return false;
    }
    let d = hypot(p.tx - p.x, p.ty - p.y);
    let tepe = alan_en2_ts(p, None);
    let r = spd(p) * sf_pk(p) * (F.pat_t + tepe) * 1.0;
    if d < 2.0 || d > r {
        return false;
    }
    if bas(j, &["alıcı", "kovala"]) {
        if let Some(fr) = fr_al(m) {
            if let Some((_, rr)) = fr.arr.iter().find(|x| x.0 == pi) {
                let mut rv = 1e9;
                for (qi, rq) in &fr.arr {
                    let q = &m.ps[*qi];
                    if q.team != p.team && !q.noTouch && rq.est < rv {
                        rv = rq.est;
                    }
                }
                if rr.est > F.pat_t + tepe || rv > rr.est + 4.0 {
                    return false;
                }
            }
        }
    }
    true
}

/// JS sprFaz(m): sprint evreleri (patlama → tepe → fren → dolum) ve sprint başlangıcı.
pub fn spr_faz(m: &mut Mac) {
    let F = &SPR_FAZ;
    let t = m.tick;
    let T1 = F.pat_t;
    let T2 = T1 + F.tepe_t;
    let t3 = T2 + F.fren_t;
    let t4 = t3 + F.dol_t;
    let kk = mac_kurali(m);
    for pi in 0..m.ps.len() {
        let holder = m.holder == Some(pi);
        let sk = kk.spr_kar(&m.ps[pi]);
        let p = &mut m.ps[pi];
        if let Some(s) = p._sf.as_mut() {
            let pt = s.f == "patlama" || s.f == "tepe";
            if pt {
                if p.en.unwrap_or(1.0) <= 0.002 {
                    if t - s.t0 < T2 {
                        s.t0 = t - T2;
                    }
                } else if (sk || p.hal._topSpr) && t - s.t0 >= T2 - 1.0 {
                    s.t0 = t - (T2 - 1.0);
                }
            }
            if pt && !(p.sprint && holder) && !(sk || p.hal._topSpr) && F.kes && t - s.t0 >= T1 {
                s.t0 = t - T2;
            }
            let e = t - s.t0;
            let f = if e < T1 { "patlama" } else if e < T2 { "tepe" } else if e < t3 { "fren" } else if e < t4 { "dolum" } else { "" };
            if f.is_empty() {
                p._sf = None;
            } else {
                s.f = f.into();
            }
        }
        let p = &m.ps[pi];
        if p._sf.is_none() && !p.bekci() && p.en.unwrap_or(1.0) >= alan_en3_sgir(p, sbed()) && p.hal._topSpr && !holder {
            m.ps[pi]._sf = Some(crate::model::JpSf { t0: t, f: "patlama".into() });
            if let Some(e) = m.ps[pi].en {
                en_say(m, pi, "sprint", sbed());
                m.ps[pi].en = Some(jmax(0.0, e - sbed()));
            }
            if m.st.spr_f.is_empty() {
                m.st.spr_f = vec![0.0, 0.0];
            }
            m.m.st.spr_f[m.m.ps[pi].team as usize] += 1.0;
        }
    }
}

/// JS vK(p): koşu katı (sprint evresi, sürekli koşu katı _hizK). mH: hareketin başındaki taşıyıcı, mt: tik.
pub fn v_k(p: &Oyuncu, ph: bool, mt: f64, k: &MacKural) -> f64 {
    let F = &SPR_FAZ;
    let s = p._sf.as_ref();
    if let Some(hk) = p.hal._hizK {
        if s.is_none() && !ph && spk() {
            return jmin(hk, jmax(1.0, sf_pk(p)));
        }
    }
    if let Some(s) = s {
        if ph && (s.f == "fren" || s.f == "dolum") {
            return 1.0;
        }
    }
    let Some(s) = s else {
        return if p.sprint {
            if p.bekci() {
                spr_k(p, k)
            } else {
                hk_k(p, spr_k(p, k))
            }
        } else {
            1.0
        };
    };
    if s.f == "patlama" || s.f == "tepe" {
        return sf_pk(p);
    }
    let a_s = 1.0;
    if s.f == "fren" {
        let T2 = F.pat_t + F.tepe_t;
        let kf = jmin(1.0, jmax(0.0, (mt - s.t0 - T2) / F.fren_t));
        let kk = kf * kf * (3.0 - 2.0 * kf);
        return sf_pk(p) + (a_s - sf_pk(p)) * kk;
    }
    a_s
}
