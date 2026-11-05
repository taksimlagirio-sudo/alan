//! Enerji ve sprint (core.js 66–200'ün maçın kullandığı kısmı; match.js 311–716, 2109–2333).
//!
//! JS'te sprint katı küresel `self.ALAN_SPR`'dır ve match.js onu sprint evresine göre sarar;
//! burada `ALAN_SPR(p, k)` açık işlev, maçın bağlamı `MacKural` (çekirdeğin `temas::Kural`'ı).
//! Bugünkü bayraklar: Enerji 2/3 açık, sürekli koşu açık (ALAN_SUREKLI), sprint evresi açık (SPR_FAZ),
//! dokunuşla sürme kapalı.
use super::*;
use crate::mat::{exp, hypot, jmax, jmin, pow};
use crate::model::JMSt;
use crate::temas::Kural;
use crate::yetenek::mek;
pub use crate::kural::enerji::*;

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

impl MacKural {
    pub fn of(m: &Mac) -> MacKural {
        MacKural { mh: m.mac.__mH, tick: m.tick, holder: m.holder.map(|h| kim(&m.ps[h])), flq: m.fl.as_ref().and_then(|f| f.q).map(|q| kim(&m.ps[q])) }
    }
}


// ---- sayaçlar ----

/// Enerji sayacı (JS {sprint: 0, ivme: 0, …}): ekleme sırasıyla.
pub type EnSayac = Vec<(String, f64)>;

fn ekle(s: &mut EnSayac, k: &str, v: f64) {
    if let Some(e) = s.iter_mut().find(|e| e.0 == k) {
        e.1 += v;
    } else {
        s.push((k.to_string(), v));
    }
}

/// JS enSay(m, p, k, v)
pub fn enSay_p(st: &mut JMSt, lite: bool, team: f64, pi: Pid, k: &str, v: f64) {
    if lite || !(v > 0.0) {
        return;
    }
    if st.enH.is_empty() {
        let z: EnSayac = ["sprint", "ivme", "ikili", "alan", "sarj", "dol"].iter().map(|k| (k.to_string(), 0.0)).collect();
        st.enH = vec![z.clone(), z];
    }
    ekle(&mut st.enH[team as usize], k, v);
    ekle(st.enP.entry(pi).or_default(), k, v);
}
pub fn enSay(m: &mut Mac, pi: Pid, k: &str, v: f64) {
    if crate::iz_acik() && !m.mac._lite && v > 0.0 && &*m.ps[pi].name == "Cansu" && std::env::var("ALAN_EN_IZ").ok().and_then(|x| x.parse::<f64>().ok()).map_or(false, |x| m.tick >= x - 1.0 && m.tick <= x) { eprintln!("EN {} {} {} {}", m.tick, m.ps[pi].name, k, v); }
    let t = m.ps[pi].team;
    enSay_p(&mut m.st, m.mac._lite, t, pi, k, v);
}

/// JS verim(p, ...oz): eski özelliklerin ortalamasıyla harcama katı
pub fn verim(p: &Oyuncu, oz: &[&str]) -> f64 {
    let mut s = 0.0;
    for o in oz {
        s += p.yet.ozk(o);
    }
    jmax(0.5, jmin(1.4, 1.0 - 0.04 * (s / oz.len() as f64 - 10.0)))
}

/// JS enHarca(m, p, k, v)
pub fn enHarca_p(st: &mut JMSt, lite: bool, p: &mut Oyuncu, pi: Pid, k: &str, v: f64) {
    let Some(en) = p.en else { return };
    if !(v > 0.0) {
        return;
    }
    let d = jmin(en, v / ALAN_EN2_SB(p));
    enSay_p(st, lite, p.team, pi, k, d);
    p.en = Some(en - d);
}
pub fn enHarca(m: &mut Mac, pi: Pid, k: &str, v: f64) {
    if crate::iz_acik() && !m.mac._lite && &*m.ps[pi].name == "Cansu" && std::env::var("ALAN_EN_IZ").ok().and_then(|x| x.parse::<f64>().ok()).map_or(false, |x| m.tick >= x - 1.0 && m.tick <= x) { eprintln!("ENH {} {} {} {} en {:?}", m.tick, m.ps[pi].name, k, v, m.ps[pi].en); }
    let lite = m.mac._lite;
    enHarca_p(&mut m.st, lite, &mut m.ps[pi], pi, k, v);
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
        let Sb = ALAN_EN2_SB(p);
        let Sd = ALAN_EN2_SD(p);
        let vx = or0(p.vx);
        let vy = or0(p.vy);
        let v = hypot(vx, vy);
        let b = spd(p);
        let vmx = b * ALAN_SPR_H(p.yet.hiz);
        let mut d = 0.0;
        let mut st_ek: Vec<(&str, f64)> = vec![];
        if v > b {
            let ds = EN2.tepe * pow(jmin(1.0, (v - b) / jmax(1e-6, vmx - b)), 1.0);
            d += ds;
            st_ek.push(("sprint", ds / Sb));
        }
        let r = v / b;
        let act = if v > b { 0.0 } else if r <= 0.6 { 1.0 - 0.5 * r } else { jmax(EN2.tempoDol, 0.7 - 1.125 * (r - 0.6)) };
        let y0 = p._yog.unwrap_or(0.0);
        let yog = y0 + (d / Sb - y0) / EN2.yogTau;
        let _yogF = jmax(EN2.yogMin, 1.0 - EN2.yogK * yog / (EN2.tepe / Sb));
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
            let dT = hypot(p.tx - p.x, p.ty - p.y);
            let yerK = if sahip { 0.0 } else { jmax(0.0, 1.0 - dT / EN3.yerD) * jmax(0.0, 1.0 - v / b) };
            dol = ALAN_EN3_DOL(p, sahip, nd, yerK);
        }
        if gt(p._dolYok, tick) {
            dol = 0.0;
        }
        let en = p.en.unwrap();
        st_ek.push(("dol", jmin(dol, 1.0 - en)));
        let team = p.team;
        for (k, x) in st_ek {
            enSay_p(&mut m.st, lite, team, pi, k, x);
        }
        let p = &mut m.ps[pi];
        p._evx = Some(vx);
        p._evy = Some(vy);
        p._yog = Some(yog);
        let en = jmax(0.0, jmin(1.0, en - d / Sb + dol));
        p.en = Some(en);
        if p._Dl.map_or(true, |dl| p.D != dl) {
            p._D0 = Some(p.D);
        }
        let dd = p._D0.unwrap_or(f64::NAN) * en;
        p.D = dd;
        p._Dl = Some(dd);
        if !lite {
            let bos = en < 0.1 && p._enOnce.unwrap_or(1.0) >= 0.1 && !gt(p._bosT, tick - 80.0);
            p._enOnce = Some(en);
            if bos {
                p._bosT = Some(tick);
                let (t, ad, bek) = (p.team, p.name.clone(), p.bekci());
                ev(m, "pas", t, if bek { format!("Bekçi'nin barı boş · {ad}") } else { format!("Barı boş · {ad}") });
            }
        } else if p._Dl.is_some() && Some(p.D) == p._Dl {
            p.D = p._D0.unwrap_or(f64::NAN);
            p._Dl = None;
        }
    }
}
/// JS enerji(m) (Enerji 2 açık: enerji2)
pub fn enerji(m: &mut Mac) {
    enerji2(m)
}

/// JS enCal(m, kaz, kay, sert): ikiliyi kazanan kaybedenden bar çalar.
pub fn enCal(m: &mut Mac, kaz: Pid, kay: Pid, sert: f64) {
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
    enSay(m, kay, "calindi", c);
    let al = jmin(c, 1.0 - m.ps[kaz].en.unwrap());
    m.ps[kaz].en = Some(m.ps[kaz].en.unwrap() + al);
    enSay(m, kaz, "caldi", al);
    let t = m.ps[kaz].team;
    if !m.mac._lite {
        let (a, b) = (m.ps[kaz].name.clone(), m.ps[kay].name.clone());
        ev(m, "pas", t, format!("{a}, {b}'den bar çaldı"));
    }
    if m.st.enCal.is_empty() {
        m.st.enCal = vec![0.0, 0.0];
    }
    m.st.enCal[t as usize] += c;
}
/// JS sertT(a, b): çarpışmanın sertliği
pub fn sertT(a: &Oyuncu, b: &Oyuncu, k: &MacKural) -> f64 {
    hypot(or0(a.vx) - or0(b.vx), or0(a.vy) - or0(b.vy)) / jmax(0.05, spd(a) * ALAN_SPR(a, k))
}

// ---- alan seçimi ----

const ALAN_YON: f64 = 2.0;

/// JS alanYz(m, p, t0): alanın yönünü seçerken yanılma (baskı altında büyür).
pub fn alanYz(m: &Mac, pi: Pid, t0: f64) -> (f64, f64) {
    let p = &m.ps[pi];
    let bb = jmin(1.0, gecici_dis::pressAt(&m.ps, p.team, p.x, p.y));
    let w = ALAN_YON * (20.0 - p.yet.y(mek::ALAN_YON) + 10.0 * crate::yetenek::DK_SUT.b * bb * crate::yetenek::yum(&p.yet, "B")) / 10.0;
    if !(w > 0.0) {
        return (0.0, 0.0);
    }
    let z = |c: f64| (crate::yetenek::zar(p.id, t0, c) + crate::yetenek::zar(p.id + 101.0, t0, c) - 1.0) * w;
    (z(1.0), z(2.0))
}

/// JS alanSec(m): boştaki topa alan açma/kapama seçimi (am: +1 hızlandır, −1 yavaşlat) ve bedeli.
pub fn alanSec(m: &mut Mac) {
    let ok = matches!(&m.ball, Some(b) if m.holder.is_none() && b.done.is_none());
    if !ok {
        for p in m.ps.iter_mut() {
            p.amA = false;
            p.am = Some(0.0);
        }
        return;
    }
    let b = m.ball.as_ref().unwrap();
    let (bx, by, bt) = (b.x, b.y, b.team as f64);
    let bdefl = dogru(b.defl) || b.defl_b;
    let fr = frTop(m);
    let fl = m.fl.as_ref();
    let sut = fl.map_or(false, |f| f.kind == "gönder") && !bdefl;
    let q = if !sut && !bdefl { fl.and_then(|f| f.q) } else { None };
    let t0 = fl.map_or(m.tick, |f| f.t0);
    let (mut oE, mut rqE, mut rqT, mut tE) = (1e9, 1e9, 0.0, [1e9, 1e9]);
    let n = fr.as_ref().map_or(0.0, |f| f.pts.len() as f64);
    if let Some(fr) = &fr {
        for (pi, r) in &fr.arr {
            let p = &m.ps[*pi];
            if !p.noTouch {
                if p.team != bt {
                    oE = jmin(oE, r.est);
                }
                tE[p.team as usize] = jmin(tE[p.team as usize], r.est);
                if Some(*pi) == q {
                    rqE = r.est;
                    rqT = r.t;
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
            let p = &mut m.ps[pi];
            p.amA = m.tick - t0 >= rt;
            p.am = Some(0.0);
            continue;
        }
        let f0 = m.ps[pi].etki().infl(bx, by);
        if f0 < 0.02 {
            m.ps[pi].amA = true;
            m.ps[pi].am = Some(0.0);
            continue;
        }
        let r = fr.as_ref().and_then(|f| f.arr.iter().find(|x| x.0 == pi)).is_some();
        let (e1, e2) = if yz { alanYz(m, pi, t0) } else { (0.0, 0.0) };
        let p = &m.ps[pi];
        let pt = p.team as usize;
        let son = n != 0.0 && rqT + e2 >= n - 3.0;
        let a = if sut {
            if p.team == bt { 1.0 } else { -1.0 }
        } else if q.is_some() {
            if p.team == bt {
                if rqE >= 1e8 || son { -1.0 } else if rqE + e1 >= oE - 2.0 { 1.0 } else if son { -1.0 } else { 0.0 }
            } else if r && tE[pt] + e1 <= rqE + 4.0 {
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
        p.amA = true;
        p.am = Some(a);
        if a > 0.0 {
            sp += f0 * dd;
        } else if a < 0.0 {
            sn += f0 * dd;
        }
    }
    let dS = |s: f64| if s <= 1.0 { s } else if s <= 2.0 { 1.0 + 0.5 * (s - 1.0) } else { 1.5 + 0.25 * (s - 2.0) };
    for pi in 0..m.ps.len() {
        let am = oo(m.ps[pi].am);
        if am == 0.0 {
            continue;
        }
        let p = &m.ps[pi];
        let f = p.etki().infl(bx, by);
        let s = if am > 0.0 { sp } else { sn };
        let pay = if s > 0.0 { dS(s) / s } else { 1.0 };
        let c = EN.alan * f * pay / ALAN_EN2_SB(p) * p.yet.verim(mek::ALAN_BEDEL);
        let e = p.en.unwrap_or(1.0);
        enSay(m, pi, "alan", jmin(e, c));
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
pub fn frAl(m: &Mac) -> Option<crate::model::JMShFr> {
    if m.holder.is_some() {
        return None;
    }
    frTop(m)
}
/// JS `m.sh.fr` (`m.sh.frB === m.ball` ise): yerleşimin uçuş okuması maçın biçiminde.
pub fn frTop(m: &Mac) -> Option<crate::model::JMShFr> {
    let b = m.ball.as_ref()?;
    let sh = &m.mac.sekil.sh;
    if sh.frB != Some(b.kimlik()) {
        return None;
    }
    let fr = sh.fr.as_ref()?;
    Some(crate::model::JMShFr {
        arr: fr.arr.iter().enumerate().map(|(i, a)| (i, crate::model::JMShFrArrV { t: a.t, x: a.x, y: a.y, est: a.est, fren: a.fren, direnc: a.direnc, ..Default::default() })).collect(),
        first: fr.first.unwrap_or(0),
        pts: fr.pts.iter().map(|q| crate::model::JMShFrPtsE { x: q.x, y: q.y, e: q.e, ..Default::default() }).collect(),
    })
}

/// JS sprintKarar3(m): her koşucu için koşu hızı (tempo → sprint arası sürekli kat, _hizK).
pub fn sprintKarar3(m: &mut Mac) {
    let F = &SPR_FAZ;
    let h = m.holder;
    let fr = frAl(m);
    let aF = gecici_dis::D::arriveFirm;
    let Wf = |T: f64, tD: f64| if tD >= 1e8 { 1.0 } else { 1.0 / (1.0 + exp((T - tD) / 5.0)) };
    let kk = MacKural::of(m);
    for pi in 0..m.ps.len() {
        {
            let p = &mut m.ps[pi];
            p._topSpr = false;
            p._kosuK = None;
            p._hizK = None;
        }
        if m.ps[pi]._itK.is_some() {
            let topBen = h.is_none() && m.fl.as_ref().map_or(false, |f| f.q == Some(pi)) && m.ball.as_ref().map_or(false, |b| b.tt.t.from == Some(pi));
            if SPK() && topBen {
                let p = &mut m.ps[pi];
                p._hizK = p._itK;
                p.sprint = p._itK.unwrap() > 1.001;
                p._kosuK = Some("it".into());
                continue;
            }
            m.ps[pi]._itK = None;
        }
        let p = &m.ps[pi];
        let sS = p._sf.as_ref().map_or(false, |s| s.f == "patlama" || s.f == "tepe");
        if Some(pi) == h || p.bekci() || p.en.is_none() || p.noTouch || kk.sprKar(p) {
            continue;
        }
        let j = job(p).to_string();
        if sp_zorla(&j) {
            let p = &mut m.ps[pi];
            p._topSpr = true;
            p._kosuK = Some("zorunlu".into());
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
                tD = jmax(0.0, oo(Some(m.decT))) + gecici_dis::ucusT(m, hypot(hp.x - tx, hp.y - ty), aF);
            }
        } else {
            for q in &m.ps {
                if q.team == p.team || q.noTouch || q.bekci() {
                    continue;
                }
                let l = hypot(q.x - tx, q.y - ty);
                tD = jmin(tD, ALAN_KOSU_T(q, l, spd(q), &kk));
            }
        }
        let b = spd(p);
        let Sb = ALAN_EN2_SB(p);
        let vm = b * ALAN_SPR_H(p.yet.hiz);
        let hk = hkK(p, SPR0(p));
        let pat = SBED();
        let e = jmax(0.0, jmin(1.0, p.en.unwrap_or(1.0)));
        let dol = {
            let mut nd = 1e9;
            for q in &m.ps {
                if !(q.team == p.team || q.noTouch) {
                    nd = jmin(nd, hypot(q.x - p.x, q.y - p.y));
                }
            }
            ALAN_EN3_DOL(p, bizde, nd, 0.0)
        };
        let dK = dol;
        let pk = sfPk(p);
        let s0 = SPR0(p);
        let yol = |e0: f64, sur: bool| -> [(f64, f64); 3] {
            let T0 = d / b;
            let T1 = d / (b * hk);
            let c1 = T1 * (EN2.tepe * pow(jmin(1.0, (b * hk - b) / jmax(1e-6, vm - b)), 1.0) / Sb + dK);
            let pa = if sur { 0.0 } else { pat };
            let (mut T2, mut c2) = (1e9, 1e9);
            if e0 >= (if sur { 0.0 } else { ALAN_EN3_SGIR(p, pa) }) {
                let ts = ALAN_EN2_TS(p, Some(e0 - pa));
                let vS = b * ALAN_EN3_TEPE(Some(0.5 * jmax(0.0, e0 - pa) + 0.5 * jmax(0.0, e0 - pa - jmin(ts, d / (b * pk)) * EN2.tepe / Sb)), if F.ust { s0 } else { s0 });
                let rc = vS * ts;
                let tS = jmin(ts, d / vS);
                T2 = if d <= rc { d / vS } else { ts + (d - rc) / b };
                c2 = pa + tS * (EN2.tepe / Sb + dK);
            }
            [(T0, 0.0), (T1, jmin(c1, e0)), (T2, c2)]
        };
        let enIyi = |e0: f64| {
            let mut w = 0.0;
            for (T, _) in yol(e0, false) {
                w = jmax(w, Wf(T, tD));
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
        let KU = |x: f64| ALAN_EN3_KUL(p, x, bizde, bxy, &kk);
        let tac_t = &m.tac[p.team as usize];
        let hrK = jmax(0.0, jmin(1.0, tac_t.enHarcama.unwrap_or(0.5))) - 0.5 + 1.0;
        let sd_p = ALAN_EN2_SD(p);
        let DEG = |T: f64, c: f64| -> f64 {
            let mut bd = 0.0;
            if c > 0.0 {
                let dE = jmax(dol, EN3.r0 * sd_p) * jmax(1.0, T0);
                for i in 1..=20 {
                    let g = dE * i as f64;
                    if g >= c {
                        break;
                    }
                    let eA = jmin(1.0, e + g);
                    let eB = jmin(1.0, e - c + g);
                    bd += Sy * (enIyi(eA) - enIyi(eB)) + KU(eA) - KU(eB);
                }
            }
            bd *= hrK;
            Sy * Wf(T, tD) - bd
        };
        // SPK: sürekli kat
        let kM = jmax(1.0, pk);
        let cK = |f: f64| -> (f64, f64, f64) {
            let k = 1.0 + f * (kM - 1.0);
            let T = d / (b * k);
            (k, T, if f > 0.0 { T * (EN2.tepe * jmin(1.0, (b * k - b) / jmax(1e-6, vm - b)) / Sb + dK) } else { 0.0 })
        };
        let (mut fB, mut vB) = (0.0, -1e9);
        let dene = |f: f64, fB: &mut f64, vB: &mut f64| {
            if !(0.0..=1.0).contains(&f) {
                return;
            }
            let (_, T, c) = cK(f);
            if c > e + 1e-9 {
                return;
            }
            let V = DEG(T, c);
            if V > *vB + 1e-12 {
                *vB = V;
                *fB = f;
            }
        };
        for f in [0.0, 0.2, 0.4, 0.6, 0.8, 1.0] {
            dene(f, &mut fB, &mut vB);
        }
        let f0 = fB;
        dene(f0 - 0.1, &mut fB, &mut vB);
        dene(f0 + 0.1, &mut fB, &mut vB);
        let f1 = fB;
        dene(f1 - 0.05, &mut fB, &mut vB);
        dene(f1 + 0.05, &mut fB, &mut vB);
        let (kS, _, _) = cK(fB);
        let _ = sS;
        if crate::iz_acik() && std::env::var("ALAN_K3_IZ").ok().as_deref() == Some(&format!("{}:{}", m.tick, pi)) {
            eprintln!("K3 tD={tD} sS={sS} e={e} dol={dol} topa={topa} bizde={bizde} j={j} f={fB} b={b} vm={vm} hk={hk} kM={kM} Sy={Sy} decT={}", m.decT);
        }
        let p = &mut m.ps[pi];
        p._hizK = Some(kS);
        p._hizF = Some(fB);
        p._kosuK = Some(if fB < 0.05 { "tempo" } else if fB < 0.7 { "hızlı" } else { "sprint" }.into());
        p.sprint = fB >= 0.05;
        p._topSpr = false;
    }
}

/// JS ALAN_EN3_KUL(p, e, bizde, bx, by): barın e düzeyindeki kullanım değeri.
pub fn ALAN_EN3_KUL(p: &Oyuncu, e: f64, bizde: bool, bxy: Option<(f64, f64)>, k: &MacKural) -> f64 {
    let Some((bx, by)) = bxy else { return 0.0 };
    let ee = jmax(0.0, jmin(1.0, e));
    let yak = exp(-hypot(p.x - bx, p.y - by) / 10.0);
    let tB = gecici_dis::threat(if bizde { p.team } else { 1.0 - p.team }, bx, by);
    let mut u = yak * tB * gecici_dis::DU::enFark * ee;
    if bizde {
        u += gecici_dis::threat(p.team, p.x, p.y) * jmin(1.0, ee / 0.2);
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
        u += yak * (tB + gecici_dis::threat(if bizde { 1.0 - p.team } else { p.team }, bx, by)) / (1.0 + exp((T(ee) - T1) / 5.0));
    }
    u
}

/// JS sprintBasla(m, p, F, yazi)
pub fn sprintBasla(m: &mut Mac, pi: Pid, yazi: Option<String>) {
    m.ps[pi]._sf = Some(crate::model::JP_sf { t0: m.tick, f: "patlama".into() });
    if let Some(e) = m.ps[pi].en {
        enSay(m, pi, "sprint", SBED());
        m.ps[pi].en = Some(jmax(0.0, e - SBED()));
    }
    let t = m.ps[pi].team as usize;
    if m.st.sprF.is_empty() {
        m.st.sprF = vec![0.0, 0.0];
    }
    m.st.sprF[t] += 1.0;
    if m.st.sprT.is_empty() {
        m.st.sprT = vec![0.0, 0.0];
    }
    m.st.sprT[t] += 1.0;
    if !m.mac._lite {
        if let Some(y) = yazi {
            ev(m, "pas", t as f64, y);
        }
    }
}

/// JS sfKesin(m, p, F): sprinte çıkmak gerçekten gerekli mi (yarış).
pub fn sfKesin(m: &Mac, pi: Pid) -> bool {
    let F = &SPR_FAZ;
    let p = &m.ps[pi];
    let j = job(p);
    if !sf_kesin_re(j) {
        return false;
    }
    let d = hypot(p.tx - p.x, p.ty - p.y);
    let tepe = ALAN_EN2_TS(p, None);
    let r = spd(p) * sfPk(p) * (F.patT + tepe) * 1.0;
    if d < 2.0 || d > r {
        return false;
    }
    if bas(j, &["alıcı", "kovala"]) {
        if let Some(fr) = frAl(m) {
            if let Some((_, rr)) = fr.arr.iter().find(|x| x.0 == pi) {
                let mut rv = 1e9;
                for (qi, rq) in &fr.arr {
                    let q = &m.ps[*qi];
                    if q.team != p.team && !q.noTouch && rq.est < rv {
                        rv = rq.est;
                    }
                }
                if rr.est > F.patT + tepe || rv > rr.est + 4.0 {
                    return false;
                }
            }
        }
    }
    true
}

/// JS sprFaz(m): sprint evreleri (patlama → tepe → fren → dolum) ve sprint başlangıcı.
pub fn sprFaz(m: &mut Mac) {
    let F = &SPR_FAZ;
    let t = m.tick;
    let T1 = F.patT;
    let T2 = T1 + F.tepeT;
    let T3 = T2 + F.frenT;
    let T4 = T3 + F.dolT;
    let kk = MacKural::of(m);
    for pi in 0..m.ps.len() {
        let holder = m.holder == Some(pi);
        let sk = kk.sprKar(&m.ps[pi]);
        let p = &mut m.ps[pi];
        if let Some(s) = p._sf.as_mut() {
            let pt = s.f == "patlama" || s.f == "tepe";
            if pt {
                if p.en.unwrap_or(1.0) <= 0.002 {
                    if t - s.t0 < T2 {
                        s.t0 = t - T2;
                    }
                } else if (sk || p._topSpr) && t - s.t0 >= T2 - 1.0 {
                    s.t0 = t - (T2 - 1.0);
                }
            }
            if pt && !(p.sprint && holder) && !(sk || p._topSpr) && F.kes && t - s.t0 >= T1 {
                s.t0 = t - T2;
            }
            let e = t - s.t0;
            let f = if e < T1 { "patlama" } else if e < T2 { "tepe" } else if e < T3 { "fren" } else if e < T4 { "dolum" } else { "" };
            if f.is_empty() {
                p._sf = None;
            } else {
                s.f = f.into();
            }
        }
        let p = &m.ps[pi];
        if p._sf.is_none() && !p.bekci() && p.en.unwrap_or(1.0) >= ALAN_EN3_SGIR(p, SBED()) && p._topSpr && !holder {
            m.ps[pi]._sf = Some(crate::model::JP_sf { t0: t, f: "patlama".into() });
            if let Some(e) = m.ps[pi].en {
                enSay(m, pi, "sprint", SBED());
                m.ps[pi].en = Some(jmax(0.0, e - SBED()));
            }
            if m.st.sprF.is_empty() {
                m.st.sprF = vec![0.0, 0.0];
            }
            m.st.sprF[m.ps[pi].team as usize] += 1.0;
        }
    }
}

/// JS vK(p): koşu katı (sprint evresi, sürekli koşu katı _hizK). mH: hareketin başındaki taşıyıcı, mt: tik.
pub fn vK(p: &Oyuncu, ph: bool, mt: f64, k: &MacKural) -> f64 {
    let F = &SPR_FAZ;
    let s = p._sf.as_ref();
    if let Some(hk) = p._hizK {
        if s.is_none() && !ph && SPK() {
            return jmin(hk, jmax(1.0, sfPk(p)));
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
                sprK(p, k)
            } else {
                hkK(p, sprK(p, k))
            }
        } else {
            1.0
        };
    };
    if s.f == "patlama" || s.f == "tepe" {
        return sfPk(p);
    }
    let aS = 1.0;
    if s.f == "fren" {
        let T2 = F.patT + F.tepeT;
        let kf = jmin(1.0, jmax(0.0, (mt - s.t0 - T2) / F.frenT));
        let kk = kf * kf * (3.0 - 2.0 * kf);
        return sfPk(p) + (aS - sfPk(p)) * kk;
    }
    aS
}
