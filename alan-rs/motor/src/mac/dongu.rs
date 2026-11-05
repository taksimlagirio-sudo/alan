//! Ana döngü (match.js 3049–4079, 4133–4138): take (topu alma), step (bir tik), run.
use super::enerji::*;
use super::gecici_dis::{self as dis, Karar};
use super::hareket::*;
use super::ikili::{ikEn, ikOn, ikiliTemas, durtFark, omuzK, IkBag, IkHedef, IK};
use super::kafa_bag::{k3, kafaBasla, kafaBitir, Son};
use super::kurulum::tlv;
use super::tac::{gecisBasla, gecisGuncelle, evreUygula, macEff, tacEff};
use super::vurus::{launch, makeBall, TopIstek};
use super::*;
use crate::alan::{Etki, Kaynak};
use crate::core_sabit::{Q, TQ};
use crate::donus::{flick_miss, flick_w, omega, shielded};
use crate::mat::{atan2, cos, exp, hypot, jmax, jmin, jround, jsign, sin, PI};
use crate::model::{JMCtl, JMPre, JP_plan, JP_planTo, JP_surK, JP_surKTo, JPHamle, JPSon, Secenek};
use crate::oyuncu::{tepki, Tepki, TepkiS};
use crate::saha::{CY, H, KR, P as FZ};
use crate::top::tahmin;
use crate::yetenek::mek;

pub use crate::kural::BEKLE_BOZ;

fn ts() -> TepkiS {
    TepkiS::default()
}

/// JS amBitir(m)
pub fn amBitir(m: &mut Mac) {
    for p in m.ps.iter_mut() {
        if p.hal.amA {
            p.hal_mut().amA = false;
            p.hal_mut().am = Some(0.0);
        }
    }
}
/// JS amYok(m, f): alan seçimlerini kapatıp f'i çalıştır, sonra geri koy.
pub fn amYok<T>(m: &mut Mac, f: impl FnOnce(&mut Mac) -> T) -> T {
    let mut s = vec![];
    for (i, p) in m.ps.iter_mut().enumerate() {
        if p.hal.amA {
            s.push((i, p.hal.am));
            p.hal_mut().amA = false;
            p.hal_mut().am = Some(0.0);
        }
    }
    let r = f(m);
    for (i, a) in s {
        m.ps[i].hal_mut().amA = true;
        m.ps[i].hal_mut().am = a;
    }
    r
}

/// JS gor(m, p): p'nin gördüğü dünya (None = gerçek)
pub fn gor(m: &mut Mac, p: Pid) -> Option<Vec<Oyuncu>> {
    if m.mac._lite {
        None
    } else {
        dis::gorulen(m, p)
    }
}

fn rng_ile<T>(m: &mut Mac, f: impl FnOnce(&Mac, &mut crate::rng::Rng) -> T) -> T {
    let mut r = std::mem::take(&mut m.r);
    let v = f(m, &mut r);
    m.r = r;
    v
}

fn karar_gercek(r: &mut Karar) {
    if let Some(b) = r.best {
        dis::gercek(&mut r.opts[b]);
    }
    for o in r.opts.iter_mut() {
        dis::gercek(o);
    }
}

fn tepki_re(j: &str) -> bool {
    j.contains("tepki")
}

/// JS take(m, p, how): p Çekirdeği alır.
pub fn take(m: &mut Mac, p: Pid, how: Option<String>) {
    amBitir(m);
    m.ctl = None;
    if m.ps[p].bekci() {
        m.ps[p].hal_mut()._korT = Some(m.tick + Q.bek_koru);
    }
    if let Some(b) = &m.ball {
        m.ch = if b.alive { jmax(0.05, b.ch) } else { 0.05 };
    }
    for (i, q) in m.ps.iter_mut().enumerate() {
        if i != p {
            q.hal_mut().sarj = Some(false);
        }
    }
    let g = gor(m, p);
    m.ps[p].hal_mut().sarj = Some(dis::sarjKarar(m, p, g.as_deref().unwrap_or(&m.ps), m.ch));
    let pp = &m.ps[p];
    let son = Son { kim: Some(pp.name.to_string()), kimId: Some(pp.id), kimT: Some(pp.team), x: k3(Some(pp.x)), y: k3(Some(pp.y)), ..Default::default() };
    kafaBitir(m, son);
    m._gelenB = m.ball.clone().map(Box::new);
    if let Some(gb) = &m.ball {
        if let Some(fr) = gb.tt.t.from {
            if m.ps[fr].team != m.ps[p].team && !(dogru(gb.defl) || gb.defl_b) && gb.done.is_none() {
                let s = hypot(or0(gb.vx), or0(gb.vy)) / 1.2;
                enCal(m, p, fr, s);
            }
        }
    }
    for q in m.ps.iter_mut() {
        q.hal_mut().savur = None;
    }
    m._onHk = None;
    m._inSp = Some(match &m.ball {
        Some(b) => {
            if gv_var(b) {
                hypot(b.tt.t.gvx, b.tt.t.gvy)
            } else {
                hypot(b.vx, b.vy)
            }
        }
        None => 0.0,
    });
    let prev = m.ball.as_ref().map(|b| b.team as f64);
    let fl = m.fl.clone();
    let ca = if let Some(b) = &m.ball {
        atan2(b.y - m.ps[p].y, b.x - m.ps[p].x)
    } else if let Some(hh) = m.holder {
        let hp = &m.ps[hh];
        let hc = hp.ca.unwrap_or(0.0);
        atan2(hp.y + sin(hc) * Q.body - m.ps[p].y, hp.x + cos(hc) * Q.body - m.ps[p].x)
    } else {
        m.ps[p].ca.unwrap_or(0.0)
    };
    m.ps[p].ca = Some(ca);
    {
        let hh = m.ps[p].hal_mut()._onH.take();
        let gk = m._gelenB.as_ref().map(|b| b.kimlik());
        if let Some(hh) = hh {
            if !m.mac._lite && Some(hh.b) == gk && (m.tick - hh.tArr).abs() <= 3.0 {
                if m.ball.is_some() {
                    let a0 = ca;
                    let mut d = hh.phi - a0;
                    d = atan2(sin(d), cos(d));
                    m.ps[p].ca = Some(if d.abs() <= PI / 2.0 { hh.phi } else { a0 + jsign(d) * PI / 2.0 });
                } else {
                    m.ps[p].ca = Some(hh.phi);
                }
                m._onHk = Some(hh);
            }
        }
    }
    m.holder = Some(p);
    m.ps[p].hal_mut()._dur = None;
    m.ball = None;
    let z = m.r.sonraki();
    m.decT = jmax(2.0, jround(tepki(&m.ps[p], Tepki::Karar, &ts()) + 3.0 * z));
    let gk = m._gelenB.as_ref().map(|b| b.kimlik());
    if m.pre.as_ref().map_or(false, |pr| Some(pr.b) != gk) {
        m.pre = None;
    }
    if let Some(pr) = m.pre.as_mut().filter(|pr| pr.q == p) {
        pr.t = m.tick;
        m.decT = 1.0;
    }
    m.fl = None;
    m.ps[p].hal_mut().driveTo = None;
    m.ps[p].hal_mut()._lc = None;
    let pt = m.ps[p].team;
    if prev == Some(pt) {
        if let Some(fl) = fl.filter(|f| f.kind != "gönder") {
            let _ = fl;
            m.st.passOk[pt as usize] += 1.0;
            let inSp = oo(m._inSp);
            let chFull = m.ch;
            let chCtrl = chFull;
            let ot = dis::otFactor(m, p, inSp);
            let mut one: Option<Secenek> = None;
            let mut kn = None;
            if !m.mac._lite && gk.map_or(false, |g| otAkil(m, p, g)) {
                let tE = tacEff(m, pt);
                let g0 = gor(m, p);
                kn = kancaOf(m, p);
                let me = m.ps[p].clone();
                let mut r = rng_ile(m, |m, r| dis::decideOT(m, &me, g0.as_deref().unwrap_or(&m.ps), chFull, r, &tE, ot, kn));
                crate::mac::kafa_dis::prT_esitle(m);
                if let Some(o) = r.as_mut() {
                    dis::gercek(o);
                }
                let mut rc = rng_ile(m, |m, r| dis::decide(m, &me, g0.as_deref().unwrap_or(&m.ps), chCtrl, r, &tE));
                crate::mac::kafa_dis::prT_esitle(m);
                if g0.is_some() {
                    karar_gercek(&mut rc);
                }
                let ctrlBest = rc.opts.first().map_or(0.0, |o| o.v.unwrap_or(f64::NAN));
                let b0 = rc.best.map(|i| rc.opts[i].clone());
                m.ps[p].hal_mut()._plan = b0.as_ref().filter(|b| b.q.is_some() && b.to.is_some() && b.det.is_some() && matches!(b.kind.as_str(), "pas" | "önüne" | "kenardan")).map(|b| JP_plan { kind: b.kind.clone(), q: b.q.unwrap(), to: JP_planTo { x: b.to.as_ref().unwrap().x, y: b.to.as_ref().unwrap().y }, P: b.det.as_ref().unwrap().P.unwrap_or(f64::NAN), t: m.tick });
                if let Some(r) = r {
                    let gg = g0.as_deref().unwrap_or(&m.ps);
                    let a = r.v.unwrap_or(f64::NAN) + dis::secimN(m, p, &r, gg, rc.T);
                    let bsn = b0.as_ref().map_or(f64::NAN, |b| dis::secimN(m, p, b, gg, rc.T));
                    if a > ctrlBest + bsn + (dis::D::otMargin + 0.06 * (tlv(&m.ps[p], "huc", "ritim") - 0.5)) {
                        one = Some(r);
                    }
                }
                m.ctl = Some(JMCtl { p, t: m.tick, r: rc });
            }
            if let Some(mut one) = one {
                m.ch = chFull;
                one.ot = Some(ot);
                kafaBasla(m, p, &mut one, true);
                match kn {
                    Some(k) if one.launch.is_some() => savurBasla(m, p, one, k.w0, k.s, if k.on { None } else { Some(k.u) }),
                    _ => launch(m, p, one),
                }
                return;
            }
            m.ch = chCtrl;
        }
    } else {
        m.st.steal[pt as usize] += 1.0;
        m.winT = Some(m.tick);
        m.winTeam = Some(pt);
        gecisBasla(m, p);
        let t = how.unwrap_or_else(|| format!("{} Çekirdeği aldı", m.ps[p].name));
        ev(m, "kesme", pt, t);
    }
}

/// JS step(m) (notlarla sarılı): bir tik.
pub fn step(m: &mut Mac) {
    let s0 = m.score[0];
    let s1 = m.score[1];
    let son = m.not.as_ref().and_then(|n| n.actor);
    _step(m);
    if m.score[0] > s0 {
        let a = son.filter(|&a| m.ps[a].team == 0.0);
        super::notlar::notSayi(m, 0.0, a);
    } else if m.score[1] > s1 {
        let a = son.filter(|&a| m.ps[a].team == 1.0);
        super::notlar::notSayi(m, 1.0, a);
    } else {
        super::notlar::notTik(m);
    }
}

/// JS run(m)
pub fn run(m: &mut Mac) {
    while !m.over {
        step(m);
    }
}

fn kxy(m: &Mac, h: Pid) -> (f64, f64) {
    let hp = &m.ps[h];
    let ca = hp.ca.unwrap_or(0.0);
    let r = Q.body + oo(hp._dke);
    (hp.x + cos(ca) * r, hp.y + sin(ca) * r)
}
fn w0(mut a: f64) -> f64 {
    while a > PI {
        a -= 2.0 * PI;
    }
    while a < -PI {
        a += 2.0 * PI;
    }
    a
}

/// Topu boşa düşür (JS m.ball = makeBall(...); holder = null; fl = null; loose++).
fn bosaDus(m: &mut Mac, o: TopIstek, takim: f64) {
    let b = makeBall(m, o);
    m.ball = Some(b);
    m.holder = None;
    m.fl = None;
    m.st.loose[takim as usize] += 1.0;
}

/// JS _step(m)
pub fn _step(m: &mut Mac) {
    crate::mac::kafa_dis::kafa_hazirla(m);
    if m.over {
        return;
    }
    if m.devre && !m.devre2 && m.tick >= ((m.len as i64 as i32) >> 1) as f64 {
        m.devre2 = true;
        let t = format!("DEVRE ARASI · {}–{}", js_num(m.score[0]), js_num(m.score[1]));
        ev(m, "devre", -1.0, t);
        super::kurulum::kickoff(m, 1.0);
    }
    m.tick += 1.0;
    let ts = ts();
    // alıcının önceden kararı
    if !m.mac._lite && m.holder.is_none() {
        let ok = match (&m.fl, &m.ball) {
            (Some(fl), Some(b)) => fl.q.is_some() && fl.kind != "gönder" && b.done.is_none() && b.team as f64 == m.ps[fl.q.unwrap()].team,
            _ => false,
        };
        if ok {
            let q = m.fl.as_ref().unwrap().q.unwrap();
            let bk = m.ball.as_ref().unwrap().kimlik();
            if m.ps[q].hal._rcvB == Some(bk) && m.ps[q].hal._rcvX.is_some() && m.pre.as_ref().map_or(true, |p| p.b != bk) {
                if m.ps[q].hal._preB != Some(bk) {
                    m.ps[q].hal_mut()._preB = Some(bk);
                    let z = m.r.sonraki();
                    m.ps[q].hal_mut()._preT = Some(jmax(2.0, jround(tepki(&m.ps[q], Tepki::Karar, &ts) + 3.0 * z)));
                }
                let ek = if otAkil(m, q, bk) { jmax(0.0, jround(0.5 * (m.ps[q].yet.y(mek::ON_HAREKET) - 10.0))) } else { 0.0 };
                if m.ps[q].hal._rcvAbs.unwrap_or(f64::NAN) - m.tick <= m.ps[q].hal._preT.unwrap_or(f64::NAN) + ek {
                    amYok(m, |m| {
                        let b = m.ball.as_ref().unwrap();
                        let (bvx, bvy, bch) = (b.vx, b.vy, b.ch);
                        let mut hc = m.ps[q].clone();
                        hc.x = hc.hal._rcvX.unwrap();
                        hc.y = hc.hal._rcvY.unwrap_or(f64::NAN);
                        hc.tx = hc.x;
                        hc.ty = hc.y;
                        if hypot(bvx, bvy) > 1e-6 {
                            hc.ca = Some(atan2(-bvy, -bvx));
                        }
                        let mut ps2 = gor(m, q).unwrap_or_else(|| m.ps.clone());
                        ps2[q] = hc.clone();
                        let mut nd = 1e9;
                        for o in &m.ps {
                            if !(o.team == hc.team || o.noTouch) {
                                nd = jmin(nd, hypot(o.x - hc.x, o.y - hc.y));
                            }
                        }
                        let ch2 = dis::recvCh(m, &ps2, hc.team, hc.x, hc.y, bch, &hc);
                        let tE = tacEff(m, hc.team);
                        let mut r = rng_ile(m, |m, rr| dis::decide(m, &hc, &ps2, ch2, rr, &tE));
                crate::mac::kafa_dis::prT_esitle(m);
                        if r.best.is_some() {
                            karar_gercek(&mut r);
                            m.pre = Some(JMPre { b: bk, q, r, x: hc.x, y: hc.y, nd, t: 0.0 });
                        }
                    });
                }
            }
        }
    }
    onHareket(m);
    for p in m.ps.iter_mut() {
        if !p.bekci() {
            let al = p.yet.y(mek::ALAN_MENZIL);
            let ak = p.yet.y(mek::ALAN_KUVVET);
            p.R = 8.0 + (al - 10.0) * 0.25;
            p.D = 1.0 + (ak - 10.0) * 0.03;
        } else {
            p.D = 1.2 * (1.0 + (p.yet.y(mek::BEKCI_ALAN) - 10.0) * 0.03);
        }
    }
    gecisGuncelle(m);
    evreUygula(m);
    if let Some(h) = m.holder.filter(|&h| m.ps[h].bekci()) {
        let kg = if m.ps[h].team == 0.0 { 0.0 } else { W };
        let ic = hypot(m.ps[h].x - kg, m.ps[h].y - CY) < Q.kuyu_a;
        if ic {
            if m.ps[h].hal.bekBas.is_none() {
                m.ps[h].hal_mut().bekBas = Some(m.tick);
            }
            if m.tick - m.ps[h].hal.bekBas.unwrap() >= 90.0 {
                m.ps[h].hal_mut().bekBas = None;
                let (t, ad) = (m.ps[h].team, m.ps[h].name.clone());
                ev(m, "kesme", 1.0 - t, format!("{ad} süreyi aştı · santra rakipte"));
                super::kurulum::kickoff(m, 1.0 - t);
                return;
            }
        } else {
            m.ps[h].hal_mut().bekBas = None;
        }
    }
    let holder = m.holder;
    let tick = m.tick;
    for (i, p) in m.ps.iter_mut().enumerate() {
        if Some(i) != holder {
            p.hal_mut().bekBas = None;
        }
    }
    for p in m.ps.iter_mut() {
        p.noTouch = gt(p.hal.stun, tick) || (gt(p.hal.adandi, tick) && holder.is_none());
        if gt(p.hal.adandi, tick) && p.hal.hamle.is_none() && p.hal.adT.is_some() {
            let a = p.hal.adT.clone().unwrap();
            p.tx = a.x;
            p.ty = a.y;
            p.sprint = true;
            p.job = Some(p.hal.adNeden.clone().filter(|s| !s.is_empty()).unwrap_or_else(|| "adandı · dönemiyor".into()));
        }
        if let Some(hm) = &p.hal.hamle {
            if holder.map_or(true, |_| false) || tick > hm.t0 + 30.0 {
                p.hal_mut().hamle = None;
            }
        }
    }
    if let Some(h) = holder {
        let hid = m.ps[h].id;
        for p in m.ps.iter_mut() {
            if p.hal.hamle.as_ref().map_or(false, |hm| hm.hid != hid) {
                p.hal_mut().hamle = None;
            }
        }
    }
    dis::position(m);
    dis::bak(m);
    iliski(m);
    topaYigilma(m);
    m._perde = None;
    dolan(m);
    fakeYonet(m);
    {
        let tick = m.tick;
        let hold = m.holder.is_some();
        let bb = m.ball.as_ref().map(|b| (b.tt.t.bek_id, b.tt.t.bek_m, dogru(b.defl) || b.defl_b, b.t, b.tp, b.team as f64));
        for pi in 0..m.ps.len() {
            if m.ps[pi].bekci() {
                let p = &m.ps[pi];
                let tep = match bb {
                    Some((bid, _bm, defl, bt, _, _)) => !hold && bid == Some(p.id as u32) && !defl && p.hal._ktB.is_some() && m.ball.as_ref().map_or(false, |b| b.tt.t.gon && b.team as f64 != p.team) && bt < crate::temas::bek_re(p), /* dalış modelinde Bekçi tepki süresince eski hedefini sürdürür (şutu henüz görmedi); sonra dalış onu taşır. Önce süre hareket cezasıydı (4 tik × bekHar) */
                    None => false,
                };
                let p = &mut m.ps[pi];
                if tep {
                    let k = p.hal._ktB.clone().unwrap();
                    p.tx = k.x;
                    p.ty = k.y;
                    if !tepki_re(job(p)) {
                        p.job = Some(format!("{} · tepki", job(p)).into());
                    }
                } else {
                    p.hal_mut()._ktB = Some(crate::model::JP_ktB { x: p.tx, y: p.ty });
                }
                continue;
            }
            let opp = matches!(bb, Some((.., bteam)) if !hold && m.ps[pi].team != bteam);
            if !opp {
                let p = &mut m.ps[pi];
                p.hal_mut()._kt = Some(crate::model::JP_kt { x: p.tx, y: p.ty });
                continue;
            }
            let d = jmax(1.0, tepki(&m.ps[pi], Tepki::YeniTop, &ts));
            let tau = {
                let b = m.ball.as_mut().unwrap();
                crate::temas::oku_t(&mut b.tt, pi, &m.ps, d, false)
            };
            let (_, _, _, bt, btp, _) = bb.unwrap();
            let p = &mut m.ps[pi];
            if btp.unwrap_or(bt) < tau && p.hal._kt.is_some() && !gt(p.hal.adandi, tick) {
                let k = p.hal._kt.clone().unwrap();
                p.tx = k.x;
                p.ty = k.y;
                if !tepki_re(job(p)) {
                    p.job = Some(format!("{} · tepki", job(p)).into());
                }
            } else {
                p.hal_mut()._kt = Some(crate::model::JP_kt { x: p.tx, y: p.ty });
            }
        }
    }
    let tick = m.tick;
    for p in m.ps.iter_mut() {
        if gt(p.hal.adandi, tick) && p.hal.hamle.is_none() && p.hal.adT.is_some() {
            let a = p.hal.adT.clone().unwrap();
            p.tx = a.x;
            p.ty = a.y;
            p.sprint = true;
            p.job = Some(p.hal.adNeden.clone().filter(|s| !s.is_empty()).unwrap_or_else(|| "adandı".into()));
        }
    }
    let hold = m.holder.is_some();
    for p in m.ps.iter_mut() {
        let Some(kw) = p.hal.kenarW.clone() else { continue };
        if hold || tick > kw.until {
            p.hal_mut().kenarW = None;
        } else if hypot(kw.x - p.x, kw.y - p.y) < 1.2 {
            if let Some(s) = kw.sonra {
                p.hal_mut().kenarW = Some(KenarW { x: s.0, y: s.1, sonra: None, until: kw.until });
                p.tx = s.0;
                p.ty = s.1;
                p.sprint = true;
                p.job = Some("çalım: arkasına".into());
            } else {
                p.hal_mut().kenarW = None;
            }
        } else {
            p.tx = kw.x;
            p.ty = kw.y;
            p.sprint = true;
            p.job = Some("çalım: dolaşıyor".into());
        }
    }
    if m.holder.is_none() {
        if let (Some(fl), Some(b)) = (&m.fl, &m.ball) {
            if let Some(q) = fl.q {
                if !(dogru(b.defl) || b.defl_b) && b.done.is_none() {
                    m.ps[q].sprint = true;
                }
            }
        }
    }
    move_(m);
    if m.ra.is_some() {
        super::kurulum::roleSample(m);
    }
    {
        let sw = dis::metrics_swarm(m);
        m.st.swarm += sw;
        m.st.swN += 1.0;
    }
    if let Some(h) = m.holder {
        if tasiyici(m, h) {
            return;
        }
        let hala = m.holder == Some(h);
        let hp = &mut m.ps[h];
        if hala && !hp.hal.drive {
            if let Some(dt) = hp.hal.driveTo.clone().filter(|d| hypot(d.x - hp.x, d.y - hp.y) > 0.8) {
                hp.tx = dt.x;
                hp.ty = dt.y;
            } else {
                hp.hal_mut().driveTo = None;
                hp.tx = hp.x;
                hp.ty = hp.y;
            }
        }
        hp.hal_mut().drive = false;
        return;
    }
    bostaTop(m);
    for p in m.ps.iter_mut() {
        p.hal_mut().drive = false;
    }
    if m.tick >= m.len {
        m.over = true;
    }
}

fn move_(m: &mut Mac) {
    super::hareket::r#move(m)
}

/// Taşıyıcının tiki (şarj, savurma, dönüş ve fiske, gövdeyle vurma, omuz/dürtme, ikiliye giriş, karar).
/// true: tik burada bitti (top koptu, vuruş çıktı …).
fn tasiyici(m: &mut Mac, h: Pid) -> bool {
    amBitir(m);
    let ht = m.ps[h].team;
    m.st.poss[ht as usize] += 1.0;
    let kk = MacKural::of(m);
    // JS `pr = D.oppAt(...)`: hesaplanır ama kullanılmaz (taşınmadı)
    let hp = &m.ps[h];
    let still = cl(1.0 - hypot(hp.vx, hp.vy) / (spd(hp) * ALAN_SPR(hp, &kk)), 0.0, 1.0);
    {
        let c0 = m.ch;
        m.ch = jmin(1.0, m.ch + dis::chargeRate(m, h, still, m.ch));
        if let Some(en) = m.ps[h].en {
            if m.ch > c0 {
                let mal = dis::sarjMal(m, h);
                enSay(m, h, "sarj", jmin(en, (m.ch - c0) * mal));
                m.ps[h].en = Some(jmax(0.0, en - (m.ch - c0) * mal));
            }
        }
    }
    if m.ps[h].hal.savur.is_some() {
        if savurAdim(m, h) {
            return true;
        }
    } else if donus(m, h, &kk) {
        return true;
    }
    let tick = m.tick;
    if !m.mac._lite && hypot(or0(m.ps[h].vx), or0(m.ps[h].vy)) < 0.04 {
        let (hx, hy) = (m.ps[h].x, m.ps[h].y);
        let q = (0..m.ps.len()).find(|&qi| {
            let q = &m.ps[qi];
            !(q.team == ht || !job(q).starts_with("yolu kesiyor") || hypot(q.x - hx, q.y - hy) > TQ.min_d + 0.1 || oo(q.hal._koyEv) > tick)
        });
        if let Some(q) = q {
            m.ps[q].hal_mut()._koyEv = Some(tick + 60.0);
            let (qt, qn, hn) = (m.ps[q].team, m.ps[q].name.clone(), m.ps[h].name.clone());
            ev(m, "pas", qt, format!("{qn} gövdesini koydu, {hn} durdu"));
        }
    }
    if ikOn() && !(m.ps[h].bekci() && gt(m.ps[h].hal._korT, m.tick)) {
        if govdeVur(m, h, &kk) {
            return true;
        }
    }
    if hamleler(m, h, &kk) {
        return true;
    }
    ikiliGiris(m, h);
    if m.ps[h].hal.holdD {
        let mut nd = 1e9;
        let hp = &m.ps[h];
        for q in &m.ps {
            if !(q.team == ht || q.noTouch) {
                nd = jmin(nd, hypot(q.x - hp.x, q.y - hp.y));
            }
        }
        if nd < BEKLE_BOZ {
            m.decT = jmin(m.decT, 1.0);
            m.ps[h].hal_mut().holdD = false;
        }
    }
    m.decT -= 1.0;
    if m.decT <= 0.0 && m.ps[h].hal.savur.is_none() && !m.mac._ucus {
        if karar(m, h) {
            return true;
        }
    }
    false
}

/// Taşıyıcının dönüşü ve fiskesi. true: top koptu.
fn donus(m: &mut Mac, h: Pid, kk: &MacKural) -> bool {
    let ht = m.ps[h].team;
    let mut nq: Option<Pid> = None;
    let mut nd = 1e9;
    {
        let hp = &m.ps[h];
        for (qi, q) in m.ps.iter().enumerate() {
            if q.team != ht && !q.bekci() {
                let dd = hypot(q.x - hp.x, q.y - hp.y);
                if dd < nd {
                    nd = dd;
                    nq = Some(qi);
                }
            }
        }
    }
    let hp = &m.ps[h];
    let sp = hypot(hp.vx, hp.vy);
    let mv = if sp > 0.04 {
        atan2(hp.vy, hp.vx)
    } else if hypot(hp.tx - hp.x, hp.ty - hp.y) > 0.5 {
        atan2(hp.ty - hp.y, hp.tx - hp.x)
    } else {
        atan2(CY - hp.y, (if ht == 0.0 { W } else { 0.0 }) - hp.x)
    };
    let mut ta = mv;
    let mut pa: Option<f64> = None;
    let yakin4 = nq.is_some() && nd < 4.0;
    if let Some(q) = nq.filter(|_| nd < 6.0) {
        let qp = &m.ps[q];
        let la = 2.0 + 0.4 * hp.yet.y(mek::KALKAN);
        let aw = atan2(hp.y - (qp.y + or0(qp.vy) * la), hp.x - (qp.x + or0(qp.vx) * la));
        pa = Some(w0(aw + PI));
        ta = if sp > 0.04 { mv + 0.7 * cl(w0(aw - mv), -1.0, 1.0) } else { aw };
        if ikOn() && sp > 0.04 {
            let wS = cl((sp / (spd(hp) * ALAN_SPR(hp, kk)) - 0.5) / 0.35, 0.0, 1.0);
            ta += w0(mv - ta) * wS;
        }
    }
    if m.ps[h].ca.is_none() {
        m.ps[h].ca = Some(mv);
    }
    let tick = m.tick;
    let hp = &m.ps[h];
    let ca = hp.ca.unwrap();
    let dd = w0(ta - ca);
    let fiskede = |hp: &Oyuncu| hp.hal.fiske.map_or(false, |f| dogru(f) && tick < f);
    m.ps[h]._don = Some(jmin(1.0, dd.abs() / 1.2));
    let hp = &m.ps[h];
    let olu = if fiskede(hp) || (hp.hal._donT == Some(tick - 1.0) && hp.hal._donDir.map_or(false, dogru)) { 0.02 } else if yakin4 { 0.12 } else { 0.1 };
    if !(dd.abs() > olu) {
        return false;
    }
    let mut dir = jsign(dd);
    if let Some(pa) = pa {
        let m1 = w0(ca + dd / 2.0);
        let m2 = w0(ca + (dd - 2.0 * dir * PI) / 2.0);
        if w0(m1 - pa).abs() < 0.9 && w0(m2 - pa).abs() > w0(m1 - pa).abs() {
            dir = -dir;
        }
    }
    if fiskede(hp) && hp.hal.fiskeDir.map_or(false, dogru) {
        dir = hp.hal.fiskeDir.unwrap();
    }
    if !fiskede(&m.ps[h]) && sp < 0.15 {
        let hp = &mut m.ps[h];
        if hp.hal._donDir.map_or(false, dogru) && hp.hal._donT == Some(tick - 1.0) && dd.abs() > 0.5 {
            dir = hp.hal._donDir.unwrap();
        }
        hp.hal_mut()._donDir = Some(if dd.abs() > 0.5 { dir } else { 0.0 });
        hp.hal_mut()._donT = Some(tick);
    }
    let hp = &m.ps[h];
    let mut w = omega(hp, yakin4, IK.yavas);
    let mut arc = if dir == jsign(dd) { dd.abs() } else { 2.0 * PI - dd.abs() };
    if fiskede(hp) {
        w = flick_w(hp);
    } else if yakin4 && arc > 1.2 && !gt(hp.hal.fiskeCd, tick) && !hp.bekci() {
        let qp = &m.ps[nq.unwrap()];
        let vq = spd(qp);
        let tq = jmax(0.0, nd - Q.reach) / (vq * ALAN_SPR(qp, kk));
        let tDol = arc / w;
        if tq < 1.0 * tDol {
            let mut acik = true;
            {
                let fw = flick_w(hp);
                let b = Q.body;
                let cr = TQ.min_d / 2.0 + 0.3;
                let md = TQ.min_d;
                // JS: kovalayanlar (Q) her yön denemesinde yeniden kurulur; göreli yerleri (o.x, o.y) bir denemenin adımları boyunca birikir
                let qs0: Vec<crate::kafa::kafa1_savur::Kov> = (0..m.ps.len()).filter(|&qi| m.ps[qi].team != ht && !m.ps[qi].noTouch).map(|qi| {
                    let q = &m.ps[qi];
                    crate::kafa::kafa1_savur::Kov { f: atan2(q.y - hp.y, q.x - hp.x), d: hypot(q.x - hp.x, q.y - hp.y), v: spd(q) * ALAN_SPR(q, kk), x: None, y: None }
                }).collect();
                let yol = |dr: f64, ar: f64| -> bool {
                    let mut qs = qs0.clone();
                    let n = (ar / fw).ceil() as i64;
                    for k in 1..=n {
                        let a = ca + dr * jmin(ar, k as f64 * fw);
                        let bx = cos(a) * b;
                        let by = sin(a) * b;
                        for o in qs.iter_mut() {
                            let (qx, qy) = crate::kafa::kafa1_savur::rakipKovala(o, bx, by, md);
                            if hypot(bx - qx, by - qy) < cr {
                                return false;
                            }
                        }
                    }
                    true
                };
                if !yol(dir, arc) {
                    let ar2 = 2.0 * PI - arc;
                    if yol(-dir, ar2) {
                        dir = -dir;
                        arc = ar2;
                    } else {
                        acik = false;
                    }
                }
            }
            if acik {
                m.ps[h].hal_mut().fiskeCd = Some(tick + 40.0);
                let z = m.r.sonraki();
                if z < flick_miss(&m.ps[h]) {
                    let a = ca + dir * arc * 0.5;
                    let hp = &m.ps[h];
                    let o = TopIstek { x: hp.x + cos(a) * Q.body, y: hp.y + sin(a) * Q.body, vx: 0.35 * cos(a), vy: 0.35 * sin(a), team: ht, ch: Some(m.ch), from: Some(h), defl: 1.0, ..Default::default() };
                    bosaDus(m, o, ht);
                    let n = m.ps[h].name.clone();
                    ev(m, "kesme", ht, format!("{n} fiskeyi kaçırdı"));
                    return true;
                }
                let fw = flick_w(&m.ps[h]);
                m.ps[h].hal_mut().fiske = Some(tick + (arc / fw).ceil());
                let v = EN.fiske * m.ps[h].yet.verim(mek::FAKE_BEDEL);
                enHarca(m, h, "fake", v);
                w = flick_w(&m.ps[h]);
                let n = m.ps[h].name.clone();
                ev(m, "pas", ht, format!("{n} fiske"));
                m.ps[h].hal_mut().fiskeDir = Some(dir);
            }
        }
    }
    let ca0 = ca;
    let ca1 = w0(ca + dir * if dir == jsign(dd) { jmin(w, dd.abs()) } else { w });
    let b = Q.body;
    let cr = TQ.min_d / 2.0 + 0.3;
    let (hx, hy) = (m.ps[h].x, m.ps[h].y);
    let cx1 = hx + cos(ca1) * b;
    let cy1 = hy + sin(ca1) * b;
    if yakin4 && !fiskede(&m.ps[h]) {
        let v = EN.dolas * jmin(1.0, w0(ca1 - ca0).abs() / jmax(0.01, omega(&m.ps[h], true, IK.yavas))) * m.ps[h].yet.verim(mek::DONDURME_BEDEL);
        enHarca(m, h, "ikili", v);
    }
    let carp = (0..m.ps.len()).find(|&qi| {
        let q = &m.ps[qi];
        q.team != ht && !q.noTouch && hypot(cx1 - q.x, cy1 - q.y) < cr
    });
    match carp {
        Some(c) => {
            if fiskede(&m.ps[h]) {
                let cx0 = hx + cos(ca0) * b;
                let cy0 = hy + sin(ca0) * b;
                let (cxp, cyp) = (m.ps[c].x, m.ps[c].y);
                let nx = cx0 - cxp;
                let ny = cy0 - cyp;
                let nl = { let l = hypot(nx, ny); if dogru(l) { l } else { 1.0 } };
                let tx = -sin(ca0) * dir;
                let ty = cos(ca0) * dir;
                let vn = tx * nx / nl + ty * ny / nl;
                let sp = 0.32;
                let vx = (tx - 1.6 * jmin(0.0, vn) * nx / nl) * sp;
                let vy = (ty - 1.6 * jmin(0.0, vn) * ny / nl) * sp;
                m.ps[h].hal_mut().fiske = Some(0.0);
                m.ps[h].hal_mut().fiskeCd = Some(tick + 30.0);
                let o = TopIstek { x: cx0, y: cy0, vx, vy, team: ht, ch: Some(m.ch), from: Some(c), defl: 1.0, ..Default::default() };
                bosaDus(m, o, ht);
                let (ct, hn, cn) = (m.ps[c].team, m.ps[h].name.clone(), m.ps[c].name.clone());
                ev(m, "kesme", ct, format!("{hn}'in fiskesi {cn}'e çarptı"));
                return true;
            }
        }
        None => m.ps[h].ca = Some(ca1),
    }
    false
}

/// Boştaki topa yarış (JS yaris): taşıyıcının takımı ile rakibin topa varış farkı.
fn yaris(m: &Mac, h: Pid, q: Pid, kx: f64, ky: f64, sg: f64, kk: &MacKural) -> f64 {
    let qp = &m.ps[q];
    let ax = kx - qp.x;
    let ay = ky - qp.y;
    let al = { let l = hypot(ax, ay); if dogru(l) { l } else { 1.0 } };
    let nx = -ay / al;
    let ny = ax / al;
    let vx = nx * sg * 0.55 + 0.2 * or0(qp.vx);
    let vy = ny * sg * 0.55 + 0.2 * or0(qp.vy);
    let tb = crate::top::Top::yeni(kx, ky, vx, vy, m.ps[h].team as u8, Some(m.ch), FZ.tek_taban);
    let mut tb = tb;
    tb.defl = 1.0;
    let src: Vec<Etki> = m.ps.iter().map(|p| p.etki()).collect();
    let pts = tahmin(&tb, &mut Kaynak::Sec(&src), &FZ, Some(60)).0;
    let r = Q.reach;
    let ht = m.ps[h].team;
    let (mut tA, mut tD) = (1e9, 1e9);
    let ts = ts();
    for (zi, z) in m.ps.iter().enumerate() {
        if z.noTouch || z.hal.sonuk {
            continue;
        }
        let v = jmax(0.05, spd(z) * ALAN_SPRE(z, None, kk));
        let rc = jmax(0.0, tepki(z, Tepki::BosTop, &ts));
        let iM = if zi == q { 8 } else { 0 };
        let mut t = 1e9;
        for i in iM..pts.len() {
            if hypot(z.x - pts[i].x, z.y - pts[i].y) - r <= v * jmax(0.0, i as f64 - rc) {
                t = i as f64;
                break;
            }
        }
        if t == 1e9 && !pts.is_empty() {
            let l = pts[pts.len() - 1];
            t = jmax(jmax(pts.len() as f64, iM as f64), rc + jmax(0.0, hypot(z.x - l.x, z.y - l.y) - r) / v);
        }
        if z.team == ht {
            tA = jmin(tA, t);
        } else {
            tD = jmin(tD, t);
        }
    }
    tA - tD
}
fn yarisEn(m: &Mac, h: Pid, q: Pid, kx: f64, ky: f64, kk: &MacKural) -> (f64, f64) {
    let a = yaris(m, h, q, kx, ky, -1.0, kk);
    let b = yaris(m, h, q, kx, ky, 1.0, kk);
    if a >= b {
        (-1.0, a)
    } else {
        (1.0, b)
    }
}

/// Gövdeyle vurma hamleleri (JS q.gvH). true: Çekirdek koptu.
fn govdeVur(m: &mut Mac, h: Pid, kk: &MacKural) -> bool {
    let (kx, ky) = kxy(m, h);
    let hid = m.ps[h].id;
    let ht = m.ps[h].team;
    for qi in 0..m.ps.len() {
        let Some(g) = m.ps[qi].hal.gvH.clone() else { continue };
        if g.hid != hid {
            continue;
        }
        if m.ps[qi].hal.sonuk || m.ps[qi].noTouch {
            m.ps[qi].hal_mut().gvH = None;
            continue;
        }
        if m.tick < g.t0 + g.W {
            let q = &mut m.ps[qi];
            let ux = kx - q.x;
            let uy = ky - q.y;
            let ul = { let l = hypot(ux, uy); if dogru(l) { l } else { 1.0 } };
            q.tx = kx;
            q.ty = ky;
            q.sprint = true;
            let ek = jmin(0.3, jmax(0.0, ul - 0.5 * Q.reach));
            q.x += ux / ul * ek;
            q.y += uy / ul * ek;
            q.job = Some("vuruyor".into());
            continue;
        }
        m.ps[qi].hal_mut().gvH = None;
        let qt = m.ps[qi].team as usize;
        m.st.duel[qt] += 1.0;
        m.ps[qi].hal_mut().son = Some(JPSon { k: "vur".into(), t: m.tick, ok: false, x: kx, y: ky });
        let yet = hypot(m.ps[qi].x - kx, m.ps[qi].y - ky) < Q.reach + 0.2;
        let P = if yet { dis::govdeVurP(m, qi, h) } else { 0.0 };
        if P > 0.0 && m.r.sonraki() < P {
            let q = &m.ps[qi];
            let ax = kx - q.x;
            let ay = ky - q.y;
            let al = { let l = hypot(ax, ay); if dogru(l) { l } else { 1.0 } };
            let ux = ax / al;
            let uy = ay / al;
            let nx = -uy;
            let ny = ux;
            let sB = yarisEn(m, h, qi, kx, ky, kk).0;
            let q = &m.ps[qi];
            let vx = nx * sB * 0.55 + 0.2 * or0(q.vx);
            let vy = ny * sB * 0.55 + 0.2 * or0(q.vy);
            let yon = if ux * (ny * sB) - uy * (nx * sB) > 0.0 { "sağa" } else { "sola" };
            let o = TopIstek { x: kx, y: ky, vx, vy, team: ht, ch: Some(m.ch), from: Some(qi), defl: 1.0, ..Default::default() };
            let b = makeBall(m, o);
            m.ball = Some(b);
            m.holder = None;
            m.fl = None;
            m.ps[h].hal_mut().savur = None;
            m.st.loose[ht as usize] += 1.0;
            m.st.duelW[qt] += 1.0;
            m.ps[qi].hal_mut().son.as_mut().unwrap().ok = true;
            let (qn, hn) = (m.ps[qi].name.clone(), m.ps[h].name.clone());
            ev(m, "kesme", qt as f64, format!("{qn} gövdesiyle {yon} vurdu, Çekirdek {hn}'den koptu"));
            return true;
        }
        m.ps[qi].hal_mut().adandi = Some(m.tick + 4.0);
        let (qn, hn) = (m.ps[qi].name.clone(), m.ps[h].name.clone());
        ev(m, "pas", ht, if P > 0.0 { format!("{hn} Çekirdeği korudu, {qn} boşa vurdu") } else { format!("{hn} Çekirdeği gövdesinin arkasına aldı, {qn} boşa vurdu") });
    }
    let (hx, hy) = (m.ps[h].x, m.ps[h].y);
    for qi in 0..m.ps.len() {
        let q = &m.ps[qi];
        if q.team == ht || q.noTouch || q.hal.sonuk || q.hal.hamle.is_some() || q.hal.gvH.is_some() || q.bekci() || oo(q.hal.gvCd) > m.tick || oo(q.hal.adandi) > m.tick {
            continue;
        }
        if hypot(q.x - hx, q.y - hy) > dis::DU::engage + 0.6 {
            continue;
        }
        let W0 = jmax(2.0, jround(4.0 - 0.15 * (q.yet.y(mek::HAMLE) - 10.0)));
        if hypot(q.x - kx, q.y - ky) > Q.reach + 0.3 * W0 {
            continue;
        }
        let P = dis::govdeVurP(m, qi, h);
        if !(P < 0.15) {
            let (_, d) = yarisEn(m, h, qi, kx, ky, kk);
            let kaz = 1.0 / (1.0 + exp(-d / 3.0));
            m.ps[qi].hal_mut()._gvKaz = Some(crate::model::JP_gvKaz { h: m.ps[h].id, t: m.tick, kaz });
            if P * kaz < 0.15 {
                continue;
            }
            let tick = m.tick;
            let q = &mut m.ps[qi];
            q.hal_mut().gvCd = Some(tick + 14.0);
            q.hal_mut().gvH = Some(JPHamle { k: String::new(), hid, t0: tick, W: W0 });
        }
    }
    false
}

/// Omuz ve uzanma hamlelerinin sonucu (JS q.hamle). true: Çekirdek el değiştirdi/koptu.
fn hamleler(m: &mut Mac, h: Pid, kk: &MacKural) -> bool {
    let hid = m.ps[h].id;
    let ht = m.ps[h].team;
    for qi in 0..m.ps.len() {
        let Some(hm) = m.ps[qi].hal.hamle.clone() else { continue };
        if hm.hid != hid {
            continue;
        }
        if m.ps[qi].hal.sonuk {
            m.ps[qi].hal_mut().hamle = None;
            continue;
        }
        let (kx, ky) = kxy(m, h);
        let (hx, hy) = (m.ps[h].x, m.ps[h].y);
        let omuz = hm.k == "omuz";
        if m.tick < hm.t0 + hm.W {
            let q = &mut m.ps[qi];
            let ux = (if omuz { hx } else { kx }) - q.x;
            let uy = (if omuz { hy } else { ky }) - q.y;
            let ul = { let l = hypot(ux, uy); if dogru(l) { l } else { 1.0 } };
            q.tx = q.x + ux / ul * 2.0;
            q.ty = q.y + uy / ul * 2.0;
            q.sprint = true;
            if ul > TQ.min_d + 0.1 {
                let ek = jmin(0.09 * ikEn(q), ul - TQ.min_d - 0.1);
                q.x += ux / ul * ek;
                q.y += uy / ul * ek;
            }
            q.job = Some(if omuz { "omuz veriyor" } else { "uzanıyor" }.into());
            continue;
        }
        m.ps[qi].hal_mut().hamle = None;
        let q = &m.ps[qi];
        let yet = if omuz { hypot(q.x - hx, q.y - hy) < TQ.min_d + 0.45 } else { hypot(q.x - kx, q.y - ky) < Q.reach + 1.0 };
        let qt = q.team;
        let (qn, hn) = (q.name.clone(), m.ps[h].name.clone());
        if ikOn() && omuz {
            m.st.duel[qt as usize] += 1.0;
            m.ps[qi].hal_mut().son = Some(JPSon { k: "omuz".into(), t: m.tick, ok: false, x: kx, y: ky });
            if !yet {
                ev(m, "pas", ht, format!("{hn} Çekirdeği çekti, {qn} boşa yüklendi"));
                continue;
            }
            let q = &m.ps[qi];
            let nx = hx - q.x;
            let ny = hy - q.y;
            let nl = { let l = hypot(nx, ny); if dogru(l) { l } else { 1.0 } };
            let ux = nx / nl;
            let uy = ny / nl;
            let ok = omuzK(q);
            let q = &mut m.ps[qi];
            q.vx = or0(q.vx) + ux * IK.omuzV * ok;
            q.vy = or0(q.vy) + uy * IK.omuzV * ok;
            if q.den.is_none() {
                q.den = Some(1.0);
            }
            if m.ps[h].den.is_none() {
                m.ps[h].den = Some(1.0);
            }
            {
                let lite = m.mac._lite;
                let mut bag = IkBag { ikt: None, ikm: Some(IkHedef { st: &mut m.st, lite }), ikl: None };
                ikiliTemas(&mut m.ps, qi, h, ux, uy, jmax(0.0, TQ.min_d - nl), true, Some(Some(h)), &mut bag);
            }
            m.ps[h].hal_mut()._omuz = Some(crate::model::JP_omuz { t: m.tick, q: m.ps[qi].id });
            let hd = m.ps[h].den.unwrap();
            ev(m, "pas", ht, if hd < IK.sendele { format!("{qn} omuz verdi, {hn} dengesini kaybetti") } else { format!("{qn} omuz verdi, {hn} dengede · %{}", js_num(jround(100.0 * hd))) });
            continue;
        }
        let hp = &m.ps[h];
        let acik = if ikOn() && !omuz {
            let sp = hypot(or0(hp.vx), or0(hp.vy));
            let hz = cl((sp / (spd(hp) * ALAN_SPR(hp, kk)) - 0.5) / 0.5, 0.0, 1.0);
            cl((0.55 * (1.0 - hp.den.unwrap_or(1.0)) + hz * oo(hp._don) * 0.7 + 0.3 * hz) / 0.8, 0.0, 1.0)
        } else {
            0.0
        };
        let P = if yet && ikOn() && !omuz {
            cl(IK.pokeT + durtFark(&m.ps[qi], hp) * IK.pokeK + acik * IK.pokeAc + (1.0 - hp.den.unwrap_or(1.0)) * IK.pokeDn, 0.05, 0.95)
        } else if yet {
            dis::duelP(m, qi, h) * if omuz { 0.7 + 0.03 * (m.ps[qi].yet.ozk("guc") - hp.yet.ozk("guc")) } else { 1.0 }
        } else {
            0.0
        };
        m.st.duel[qt as usize] += 1.0;
        m.ps[qi].hal_mut().son = Some(JPSon { k: hm.k.clone(), t: m.tick, ok: false, x: kx, y: ky });
        // zar yalnız erişince çekilir (özgün kısa devreyle aynı sıra)
        let zr = if yet { m.r.sonraki() } else { 1.0 };
        // ölçüm (ALAN_DUEL_IZ): gerçek kapma girişimi: tik, taşıyıcı, kaleye uzaklık, omuz mu, erişti mi, P, kazandı mı, taşıyıcının açıklığı, dengesi, hızı
        if crate::iz_acik() && std::env::var("ALAN_DUEL_IZ").is_ok() && !m.mac._lite {
            let hp = &m.ps[h];
            let gx = if hp.team == 0.0 { W } else { 0.0 };
            eprintln!("DUEL [{},{:?},{:.2},{},{},{:.4},{},{:.4},{:.4},{:.4},{:.4}]", m.tick, &*hp.name, hypot(gx - hp.x, H / 2.0 - hp.y), omuz, yet, P, yet && zr < P, acik, hp.den.unwrap_or(1.0), hypot(or0(hp.vx), or0(hp.vy)), oo(hp._don));
        }
        if yet && zr < P {
            m.st.duelW[qt as usize] += 1.0;
            m.ps[qi].hal_mut().son.as_mut().unwrap().ok = true;
            if omuz {
                let q = &m.ps[qi];
                let (qvx, qvy) = (or0(q.vx), or0(q.vy));
                let nx = hx - q.x;
                let ny = hy - q.y;
                let nl = { let l = hypot(nx, ny); if dogru(l) { l } else { 1.0 } };
                let tick = m.tick;
                let hp = &mut m.ps[h];
                hp.x += nx / nl * 0.7;
                hp.y += ny / nl * 0.7;
                hp.vx = 0.3 * or0(hp.vx) + nx / nl * 0.22;
                hp.vy = 0.3 * or0(hp.vy) + ny / nl * 0.22;
                hp.hal_mut().slowT = Some(tick + 25.0);
                let (hvx, hvy) = (hp.vx, hp.vy);
                let o = TopIstek { x: kx, y: ky, vx: 0.2 * hvx + 0.5 * qvx, vy: 0.2 * hvy + 0.5 * qvy, team: ht, ch: Some(m.ch), from: Some(h), defl: 1.0, ..Default::default() };
                bosaDus(m, o, ht);
                let s = sertT(&m.ps[qi], &m.ps[h], kk);
                enCal(m, qi, h, s);
                ev(m, "kesme", qt, format!("{qn} omuz verdi, {hn} Çekirdekten ayrıldı"));
                return true;
            }
            if m.r.sonraki() < 0.6 {
                let q = &m.ps[qi];
                m.ps[qi].ca = Some(atan2(ky - q.y, kx - q.x));
                let s = sertT(&m.ps[qi], &m.ps[h], kk);
                enCal(m, qi, h, s);
                take(m, qi, Some(format!("İkili · {qn}, {hn}'den Çekirdeği aldı")));
                return true;
            }
            let q = &m.ps[qi];
            let mut a = atan2(ky - q.y, kx - q.x) + 1.6 * (m.r.sonraki() - 0.5);
            {
                let mut bm: Option<Pid> = None;
                let mut bv = -1e9;
                for (zi, z) in m.ps.iter().enumerate() {
                    if z.team != qt || zi == qi || z.bekci() || z.noTouch {
                        continue;
                    }
                    let dz = hypot(z.x - kx, z.y - ky);
                    if dz > 12.0 {
                        continue;
                    }
                    let mut nd = 1e9;
                    for o in &m.ps {
                        if o.team == ht {
                            nd = jmin(nd, hypot(o.x - z.x, o.y - z.y));
                        }
                    }
                    let v = nd - 0.4 * dz;
                    if v > bv {
                        bv = v;
                        bm = Some(zi);
                    }
                }
                if let Some(bm) = bm {
                    let ta = atan2(m.ps[bm].y - ky, m.ps[bm].x - kx);
                    let ok2 = jmax(0.0, jmin(1.0, m.ps[qi].yet.y(mek::DURTME) / 20.0));
                    let df = w0(ta - a);
                    a += df * ok2 * 0.8;
                }
            }
            let o = TopIstek { x: kx + 0.3 * cos(a), y: ky + 0.3 * sin(a), vx: 0.5 * cos(a), vy: 0.5 * sin(a), team: ht, ch: Some(m.ch), from: Some(qi), defl: 1.0, ..Default::default() };
            bosaDus(m, o, ht);
            ev(m, "kesme", qt, format!("{qn} uzanıp dürttü, Çekirdek boşta"));
            return true;
        }
        {
            let q = &m.ps[qi];
            let dx = (if omuz { hx } else { kx }) - q.x;
            let dy = (if omuz { hy } else { ky }) - q.y;
            let dl = { let l = hypot(dx, dy); if dogru(l) { l } else { 1.0 } };
            let (x, y) = (q.x + dx / dl * 2.6, q.y + dy / dl * 2.6);
            yanil(m, qi, x, y, "geçildi");
        }
        if yet {
            let s = sertT(&m.ps[h], &m.ps[qi], kk);
            enCal(m, h, qi, s);
        }
        ev(m, "pas", ht, if yet { format!("{hn}, {qn}'i geçti") } else { format!("{hn} Çekirdeği çekti, {qn} boşa {}", if omuz { "yüklendi" } else { "uzandı" }) });
    }
    false
}

/// Rakiplerin ikiliye girişi (omuz ya da uzanma hamlesi).
fn ikiliGiris(m: &mut Mac, h: Pid) {
    let ht = m.ps[h].team;
    let hid = m.ps[h].id;
    for qi in 0..m.ps.len() {
        let q = &m.ps[qi];
        if q.team == ht || q.noTouch || q.hal.hamle.is_some() {
            continue;
        }
        if m.ps[h].bekci() && gt(m.ps[h].hal._korT, m.tick) {
            continue;
        }
        if q.hal.sonuk {
            continue;
        }
        let hp = &m.ps[h];
        if hypot(q.x - hp.x, q.y - hp.y) > dis::DU::engage + 0.6 {
            continue;
        }
        let (kx, ky) = kxy(m, h);
        if q.cd > m.tick && (!q.hal._kal || q.hal._kalH != Some(h) || shielded(hp.x, hp.y, q.x, q.y, kx, ky)) {
            continue;
        }
        if q.en.map_or(false, |e| e < EN.ikiliMin) {
            continue;
        }
        let P = dis::duelP(m, qi, h);
        let z = m.r.sonraki();
        let q = &m.ps[qi];
        let kaz = m.tac.get(q.team as usize).and_then(|t| t.kazaninca.clone());
        let est = P + (z - 0.5) * (20.0 - q.yet.y(mek::IKILI_KARAR)) * 0.03 + dis::D::sarjKap * jmax(0.0, m.ch - 0.3) * if kaz.as_deref() == Some("Kontra") { 1.0 } else { 0.5 } + dis::enIkiliEk(m, qi, h);
        let tick = m.tick;
        let (hx, hy) = (m.ps[h].x, m.ps[h].y);
        {
            let q = &mut m.ps[qi];
            q.cd = tick + dis::DU::cd;
            q.hal_mut()._kal = shielded(hx, hy, q.x, q.y, kx, ky);
            q.hal_mut()._kalH = Some(h);
        }
        let qt = m.ps[qi].team;
        let tq = m.tac.get(qt as usize).cloned().unwrap_or_default();
        let yzk = if tq.yz == Some(true) { macEff(m, qt, Default::default()).macK.unwrap_or(1.0) < 0.6 } else { false };
        let sdq = jmin(1.0, tq.siddet.unwrap_or(0.5) + if yzk { 0.25 } else { 0.0 });
        let thr0 = dis::commitThr(m, qi, h);
        let oya = {
            let q = &m.ps[qi];
            let gs = m.mac.sekil.sh.gs.as_ref();
            !q.bekci() && gs.map_or(false, |g| g.oy && g.dt as f64 == q.team && g.t >= tick - 1.0) && hypot(hx - if q.team == 0.0 { 0.0 } else { W }, hy - H / 2.0) > dis::DU::oyalaR
        };
        {
            let ge = dis::girEsik(m, qi, h, m.ch);
            if crate::iz_acik() && &*m.ps[qi].name == "Cansu" && std::env::var("ALAN_EN_IZ").ok().and_then(|x| x.parse::<f64>().ok()) == Some(m.tick) { eprintln!("IKE P {} est {} ge {:?} thr0 {} enEk {} ch {}", P, est, ge, thr0, dis::enIkiliEk(m, qi, h), m.ch); }
            let gec = match ge {
                Some(g) => est < g,
                None => est < thr0 + (if oya { if thr0 >= dis::DU::thrLast { dis::DU::oyala } else { dis::DU::oyalaC } } else { 0.0 }) - 0.2 * (sdq - 0.5),
            };
            if gec {
                continue;
            }
        }
        let q = &m.ps[qi];
        let shd = shielded(hx, hy, q.x, q.y, kx, ky);
        let cv = 0.25 * (q.yet.y(mek::HAMLE) - 10.0);
        let W0 = jmax(2.0, jround((if shd { 4.0 } else { 5.0 }) - cv));
        {
            let q = &mut m.ps[qi];
            q.hal_mut()._kal = false;
            q.hal_mut().hamle = Some(JPHamle { k: if shd { "omuz" } else { "uzan" }.into(), hid, t0: tick, W: W0 });
            if !shd {
                q.hal_mut()._dolYok = Some(jmax(oo(q.hal._dolYok), tick + EN2.uzanDolYok));
            }
        }
        if m.ps[qi].en.is_some() {
            let v = EN.ikili * EN.fizK * m.ps[qi].yet.verim(mek::TEMAS_BEDEL);
            enHarca(m, qi, "ikili", v);
        }
        {
            let q = &mut m.ps[qi];
            q.cd = tick + W0 + dis::DU::cd;
            q.hal_mut().adandi = Some(tick + W0 + jmax(3.0, jround(8.0 - cv)));
            q.hal_mut().adT = Some(crate::model::JPAdT { x: q.x + 1.6 * (kx - q.x), y: q.y + 1.6 * (ky - q.y) });
        }
        let okH = m.ps[h].yet.y(mek::IKILI_KARAR);
        let okQ = m.ps[qi].yet.y(mek::IKILI_KARAR);
        if m.r.sonraki() < jmax(0.1, jmin(0.9, 0.35 + 0.04 * (okH - okQ) + 0.02 * (okH - 10.0))) {
            m.decT = jmin(m.decT, 1.0);
            m.ps[h].hal_mut().okudu = Some(m.ps[qi].id);
        }
    }
}

/// Taşıyıcının kararı. true: tik bitti (JS'te karar sonrası return yok; yalnız launch/savur kendi içinde döner).
fn karar(m: &mut Mac, h: Pid) -> bool {
    m.ps[h].hal_mut().holdD = false;
    if !m.mac._lite {
        m.mac.__mH = Some(kim(&m.ps[h]));
    }
    let ht = m.ps[h].team;
    let tE = tacEff(m, ht);
    m.pre = None;
    let g = gor(m, h);
    let mut r: Option<Karar> = None;
    if let Some(c) = m.ctl.take() {
        if c.p == h && m.tick - c.t <= 1.0 {
            r = Some(c.r);
        }
    }
    let mut r = match r {
        Some(r) => r,
        None => {
            let me = m.ps[h].clone();
            let ch = m.ch;
            let mut r = rng_ile(m, |m, rr| dis::decide(m, &me, g.as_deref().unwrap_or(&m.ps), ch, rr, &tE));
                crate::mac::kafa_dis::prT_esitle(m);
            if g.is_some() {
                karar_gercek(&mut r);
            }
            r
        }
    };
    let gg: Vec<Oyuncu> = g.clone().unwrap_or_else(|| m.ps.clone());
    let mut oi = dis::secim(m, h, &r, &gg);
    dis::gercek(&mut r.opts[oi]);
    let mut dvC: Option<usize> = None;
    if let Some(sk) = m.ps[h].hal._surK.clone().filter(|k| k.id == m.ps[h].id) {
        if r.opts[oi].kind != "gönder" {
            let hp = &m.ps[h];
            let mut nd = 1e9;
            for q in &m.ps {
                if !(q.team == ht || q.noTouch || q.bekci()) {
                    nd = jmin(nd, hypot(q.x - hp.x, q.y - hp.y));
                }
            }
            if m.tick - sk.t < sk.T && hypot(sk.to.x - hp.x, sk.to.y - hp.y) > 2.5 && nd >= jmin(3.5, sk.nd0 - 2.0) {
                let c = r.opts.iter().position(|x| x.kind == "sür" && x.v.is_some() && x.to.as_ref().map_or(false, |t| hypot(t.x - sk.to.x, t.y - sk.to.y) < 4.0));
                dvC = c;
                if let Some(c) = c {
                    let pr = jmin(1.0, dis::pressAt(&m.ps, ht, hp.x, hp.y));
                    if c != oi && r.opts[oi].v.unwrap_or(f64::NAN) < r.opts[c].v.unwrap() + dis::secimW(m, h, pr, false) {
                        oi = c;
                        r.opts[oi].devam = Some(1.0);
                    }
                }
            }
        }
    }
    let pl = m.ps[h].hal_mut()._plan.take();
    if let Some(pl) = pl.filter(|pl| m.tick - pl.t < 30.0) {
        let same = r.opts.iter().position(|x| x.kind == pl.kind && x.q == Some(pl.q) && x.to.as_ref().map_or(false, |t| hypot(t.x - pl.to.x, t.y - pl.to.y) < 4.0));
        if let Some(s) = same {
            let x = &r.opts[s];
            if x.det.as_ref().map_or(false, |d| d.P.unwrap_or(f64::NAN) >= pl.P - 0.15) && x.v.map_or(false, |v| v > 0.0) {
                oi = s;
            }
        }
    }
    // ölçüm (alan-kos tutma): kafanın sürme seçeneklerinden biri zorla seçilir
    if let Some(sira) = m.mac.zorla.take() {
        let ix: Vec<usize> = (0..r.opts.len()).filter(|&i| r.opts[i].kind == "sür" && r.opts[i].v.is_some() && r.opts[i].to.is_some()).collect();
        if let Some(&i) = ix.get(sira) {
            let o = &r.opts[i];
            let to = o.to.as_ref().unwrap();
            m.mac.zorla_sonuc = Some(super::ZorlaSonuc { to: (to.x, to.y), P: o.det.as_ref().and_then(|d| d.P).unwrap_or(f64::NAN), v: o.v.unwrap(), secilen: i == oi, n: ix.len() });
            oi = i;
        }
    }
    // ölçüm (ALAN_SECP_IZ): kararın bütün seçenekleri ve tutma olasılıkları, stderr'e tek satır JSON
    if crate::iz_acik() && std::env::var("ALAN_SECP_IZ").is_ok() {
        let hp = &m.ps[h];
        let f = |v: Option<f64>| v.filter(|x| x.is_finite()).map_or("null".to_string(), |x| format!("{}", x));
        let o: Vec<String> = r.opts.iter().enumerate().map(|(i, x)| format!("[{:?},{},{},{},{},{},{}]", x.kind, x.to.as_ref().map_or("null".into(), |t| format!("[{},{}]", t.x, t.y)), f(x.v), f(x.det.as_ref().and_then(|d| d.P)), f(x.det.as_ref().and_then(|d| d.Ps)), f(x._own), if i == oi { 1 } else { 0 })).collect();
        eprintln!("SECP [{},{:?},{},{},{},[{}]]", m.tick, &*hp.name, hp.team, hp.x, hp.y, o.join(","));
    }
    // m._sec ve m.lastDec (gösterim)
    {
        let hp = &m.ps[h];
        let o: Vec<crate::model::JM_secOE> = r.opts.iter().enumerate().filter(|(_, x)| x.v.is_some() && x.kind != "tut" && x.kind != "fake" && (x.to.is_some() || x.q.is_some())).take(14).map(|(i, x)| {
            let at = match x.q {
                Some(q) if q != h && x.kind == "pas" => (m.ps[q].x, m.ps[q].y),
                _ => x.to.as_ref().map_or((f64::NAN, f64::NAN), |t| (t.x, t.y)),
            };
            crate::model::JM_secOE { k: x.kind.clone(), x: at.0, y: at.1, v: x.v.unwrap(), dv: if Some(i) == dvC { 1.0 } else { 0.0 } }
        }).collect();
        let tut = r.opts.iter().find(|x| x.kind == "tut" && x.v.is_some()).map_or(0.0, |x| x.v.unwrap());
        m._sec = Some(crate::model::JM_sec { h, t: m.tick, o, tut });
        let sira: Vec<usize> = if oi != 0 { std::iter::once(oi).chain((0..r.opts.len()).filter(|&i| i != oi)).collect() } else { (0..r.opts.len()).collect() };
        let opts = sira.iter().take(4).map(|&i| {
            let o = &r.opts[i];
            let it = if o.q == Some(h) && (o.kind == "pas" || o.kind == "önüne") { " · it" } else { "" };
            let spr = o.spr.map_or(false, crate::mat::dogru);
            let mut kind = format!("{}{}{}", o.kind, o.shot.as_ref().map_or(String::new(), |s| format!(" {s}")), it);
            if spr {
                kind.push_str(" · tam hız");
                let tw = r.opts.iter().enumerate().find(|(j, y)| *j != i && !y.spr.map_or(false, crate::mat::dogru) && y.kind == o.kind && y.q == o.q && matches!((&y.to, &o.to), (Some(a), Some(b)) if hypot(a.x - b.x, a.y - b.y) < 0.01));
                if let Some((_, tw)) = tw {
                    if let (Some(tv), Some(ov)) = (tw.v, o.v) {
                        kind.push_str(&format!(" ({}{:.3})", if ov >= tv { "+" } else { "" }, ov - tv));
                    }
                }
            }
            crate::model::JMLastDecOptsE {
                kind,
                to: o.to.as_ref().map(|t| crate::model::JMLastDecOptsETo { x: t.x, y: t.y }),
                q: o.q.map_or(String::new(), |q| m.ps[q].name.to_string()),
                P: o.det.as_ref().map_or(f64::NAN, |d| if o.kind == "gönder" { d.Pk.unwrap_or(f64::NAN) } else { d.P.unwrap_or(f64::NAN) - d.Ps.unwrap_or(f64::NAN) }),
                v: o.v.unwrap_or(f64::NAN),
                sjK: o.sjK,
                kh: o.kh.clone(),
                devam: o.devam,
                ..Default::default()
            }
        }).collect();
        m.lastDec = Some(crate::model::JMLastDec { name: hp.name.to_string(), team: ht, x: hp.x, y: hp.y, opts });
    }
    let z = m.r.sonraki();
    m.decT = jround(5.0 + 8.0 * (1.0 - m.tac[ht as usize].tempo.unwrap_or(f64::NAN)) + 3.0 * z);
    let mut o = r.opts[oi].clone();
    let hp = &m.ps[h];
    if o.kind == "sür" {
        let to = o.to.clone().unwrap_or_default();
        if hp.hal._surK.as_ref().map_or(true, |k| k.id != hp.id || hypot(k.to.x - to.x, k.to.y - to.y) >= 4.0) {
            let mut nd0 = 1e9;
            for q in &m.ps {
                if !(q.team == ht || q.noTouch || q.bekci()) {
                    nd0 = jmin(nd0, hypot(q.x - hp.x, q.y - hp.y));
                }
            }
            let T = jround(hypot(to.x - hp.x, to.y - hp.y) / 0.25) + 6.0;
            let id = hp.id;
            m.ps[h].hal_mut()._surK = Some(JP_surK { id, to: JP_surKTo { x: to.x, y: to.y }, t: m.tick, T, nd0 });
        }
    } else {
        m.ps[h].hal_mut()._surK = None;
    }
    m.ps[h].hal_mut()._sprIst = o.spr.map_or(false, crate::mat::dogru);
    kafaBasla(m, h, &mut o, false);
    {
        let s0 = m.ps[h].hal.sarj == Some(true);
        let sk = dis::sarjKarar(m, h, &gg, m.ch);
        let tutS = o.kind == "tut" && o.det.as_ref().map_or(false, |d| d.ch.unwrap_or(f64::NAN) > m.ch + 0.005) && m.ps[h].en.unwrap_or(1.0) >= 0.1;
        m.ps[h].hal_mut().sarj = Some(sk || tutS);
        if m.ps[h].hal.sarj == Some(true) && !s0 && !m.mac._lite {
            let n = m.ps[h].name.clone();
            let ch = js_num(jround(100.0 * m.ch));
            ev(m, "sarj", ht, format!("{n} şarj ediyor · %{ch}"));
        }
    }
    let why = o.why.clone().filter(|s| !s.is_empty()).unwrap_or_else(|| o.kind.clone());
    if o.kind == "sür" {
        let to = o.to.clone().unwrap_or_default();
        {
            let hp = &mut m.ps[h];
            hp.hal_mut().driveTo = Some(crate::model::JPDriveTo { x: to.x, y: to.y });
            hp.tx = to.x;
            hp.ty = to.y;
            hp.hal_mut().drive = true;
        }
        let spr = o.spr.map_or(false, crate::mat::dogru);
        let hp = &m.ps[h];
        if spr && SPR_FAZ.ayir && hp._sf.is_none() && !hp.bekci() && hp.en.unwrap_or(1.0) >= ALAN_EN3_SGIR(hp, SBED()) {
            sprintBasla(m, h, None);
        }
        m.ps[h].hal_mut().lastWhy = Some(why.into());
    } else if o.kind == "fake" {
        fakeBasla(m, h, &o);
        m.ps[h].hal_mut().lastWhy = Some(why.into());
    } else if o.kind == "tut" {
        {
            let hp = &mut m.ps[h];
            hp.hal_mut().driveTo = None;
            hp.tx = hp.x;
            hp.ty = hp.y;
        }
        if let Some(hd) = o.hold.filter(|&x| dogru(x)) {
            m.decT = hd;
            m.ps[h].hal_mut().holdD = true;
        }
        let lw = m.ps[h].hal.lastWhy.clone();
        let n = m.ps[h].name.clone();
        let look = o.det.as_ref().and_then(|_| dis_lookTo(m, &o));
        if o.why.as_deref() == Some("çek") && lw.as_deref() != Some("çek") {
            ev(m, "pas", ht, format!("{n} rakibi üstüne çekiyor{}", look.as_ref().map_or(String::new(), |l| format!(" · {l} açılacak"))));
        } else if o.why.as_deref() == Some("koşu") && lw.as_deref() != Some("koşu") {
            if let Some(l) = look {
                ev(m, "pas", ht, format!("{n} bekliyor · {l} koşusunu bitirince verecek"));
            }
        }
        m.ps[h].hal_mut().lastWhy = Some(why.into());
    } else {
        m.ps[h].hal_mut().lastWhy = Some(why.into());
        if o.launch.is_some() {
            let sy = o.sYon.unwrap_or(0.0);
            savurBasla(m, h, o, 0.0, sy, None);
        } else {
            launch(m, h, o);
        }
    }
    false
}

/// JS o.det.lookTo.name: kafanın bakılan oyuncusu (taslakta yok; kafa ekleyecek).
fn dis_lookTo(_m: &Mac, _o: &Secenek) -> Option<String> {
    None
}

/// Boştaki top: durduysa en yakın alır; yoksa alan seçimi, topun tiki ve temas, gövde, sayı.
fn bostaTop(m: &mut Mac) {
    let done = m.ball.as_ref().unwrap().done;
    if done.map(|d| d.ad()) == Some("durdu") {
        let b = m.ball.as_ref().unwrap();
        let (bx, by) = (b.x, b.y);
        let mut best: Option<Pid> = None;
        let mut bd = 1e9;
        for (pi, p) in m.ps.iter().enumerate() {
            if p.noTouch {
                continue;
            }
            let d = hypot(p.x - bx, p.y - by);
            if d < bd {
                bd = d;
                best = Some(pi);
            }
        }
        if bd < Q.reach {
            let best = best.unwrap();
            let z = m.r.sonraki();
            let kk = MacKural::of(m);
            let cp = {
                let b = m.ball.as_mut().unwrap();
                crate::temas::ctrl_p(&mut b.tt, best, &m.ps, None, &kk)
            };
            if z < cp {
                let n = m.ps[best].name.clone();
                take(m, best, Some(format!("{n} boştaki Çekirdeği aldı")));
            }
        }
    } else {
        alanSec(m);
        let kk = MacKural::of(m);
        let (bx0, by0) = (m.ball.as_ref().unwrap().x, m.ball.as_ref().unwrap().y);
        let mut b = m.ball.take().unwrap();
        let mut rr = std::mem::take(&mut m.r);
        let r = crate::temas::step_all(&mut b.tt, &mut m.ps, &FZ, &mut || rr.sonraki(), &kk);
        m.r = rr;
        m.ball = Some(b);
        for pi in 0..m.ps.len() {
            if let Some(d) = m.ps[pi].hal._dalis.filter(|&d| dogru(d)) {
                let f = jmin(1.0, d);
                enHarca(m, pi, "dalış", EN.dalis * f);
                let p = &mut m.ps[pi];
                p.hal_mut()._dolYok = Some(jmax(oo(p.hal._dolYok), m.tick + jround(EN.dalisDolYok * f)));
                p.hal_mut()._dalis = Some(0.0);
            }
        }
        {
            let b = m.ball.as_mut().unwrap();
            if b.direk != b.ek._dk0 {
                b.ek._dk0 = b.direk;
                let (bt, from) = (b.team as f64, b.tt.t.from);
                if m.st.direk.is_empty() {
                    m.st.direk = vec![0.0, 0.0];
                }
                m.st.direk[bt as usize] += 1.0;
                let t = format!("{}Direkten sekti", from.map_or(String::new(), |f| format!("{} · ", m.ps[f].name)));
                ev(m, "direk", bt, t);
            }
        }
        let took = r.as_ref().map_or(false, |r| r.took);
        if !took && m.ball.as_ref().unwrap().done.is_none() && r.is_none() {
            let b = m.ball.as_mut().unwrap();
            let from = b.tt.t.from;
            let bt = b.t;
            let skip = |pi: usize| Some(pi) == from && or0(bt) < 4.0;
            crate::temas::govde(&mut b.tt, bx0, by0, &m.ps, &skip, super::vurus::BODY_R, super::vurus::BODY_E);
        }
        if let Some(r) = r.as_ref().filter(|r| r.took) {
            let fl = m.fl.as_ref();
            let rp = r.p;
            let n = m.ps[rp].name.clone();
            let t = if fl.map_or(false, |f| f.kind == "gönder") && m.ps[rp].bekci() { format!("{n} göndermeyi tuttu") } else { format!("{n} aktarımı kesti") };
            take(m, rp, Some(t));
            return;
        }
        if r.is_some() {
            if let Some(fl) = m.fl.as_mut() {
                let b = m.ball.as_ref().unwrap();
                fl.end = crate::model::JMFlEnd { x: b.x, y: b.y };
            }
        }
        let b = m.ball.as_ref().unwrap();
        let dn = b.done.map(|d| d.ad());
        if let Some(d) = dn.filter(|d| d.starts_with("kuyu")) {
            let sc = if d == "kuyu-sag" { 0.0 } else { 1.0 };
            let (pk, from) = (b.ek.shotPk, b.tt.t.from);
            m.score[sc as usize] += 1.0;
            kafaBitir(m, Son { gol: Some(sc), ..Default::default() });
            if pk.is_some() {
                if let Some(c) = m.st.cal.as_mut() {
                    c.g += 1.0;
                }
            }
            let t = format!("SAYI · {} · {}–{}", from.map_or(String::new(), |f| m.ps[f].name.to_string()), js_num(m.score[0]), js_num(m.score[1]));
            ev(m, "sayi", sc, t);
            super::kurulum::kickoff(m, 1.0 - sc);
        } else if dn == Some("durdu") {
            if let Some(fl) = m.fl.as_mut() {
                let b = m.ball.as_ref().unwrap();
                fl.end = crate::model::JMFlEnd { x: b.x, y: b.y };
            }
        }
    }
}

#[allow(dead_code)]
fn _kullan() {
    let _ = (KR, tlv(&Oyuncu::default(), "huc", "x"));
}
