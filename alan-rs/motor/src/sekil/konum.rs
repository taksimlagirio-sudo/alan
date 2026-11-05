//! position(m): her tik topsuz herkesin işi (`job`) ve hedefi (`tx`, `ty`).
//!
//! Sıra (JS ile aynı): geçmiş, hücum tarafı ve E (topun/alıcının yeri), kovalayanlar, savunma (pres, süpürücüler,
//! markaj, eşleşme işi), yedek işler, takım düzeltmeleri, hücumcuların boşluk araması, Bekçi, yol ver, sınırlar.
#![allow(non_snake_case, clippy::needless_range_loop)]

use std::sync::Arc;

use crate::mat::{hypot, jmax, jmin, jround};
use crate::model::{JPHistE, JPPastE, Oyuncu, Pid, TopRef};
use crate::oyuncu::Tepki;
use crate::saha::KR;
use crate::temas::bek_re;
use crate::yetenek::mek;

use super::gecici_dis as dis;
use super::ucus::okuT as okuTb;
use super::*;

mod esles;
mod hucum;
pub use hucum::SEYREK_KAPALI;
mod savunma;
mod yardim;

use yardim::*;

/// JS position(m).
pub fn position(c: &mut Mc) {
    let k = c.k;
    let n = c.ps.len();
    let tick = c.m.tick;
    if c.ek.bkCik.len() != n {
        c.ek.bkCik = vec![None; n];
    }
    let KT = if c.m.holder.is_some() { None } else { kendiTopu(c) };
    let h: Option<Pid> = c.m.holder.or(KT.map(|x| x.0));
    for p in c.ps.iter_mut() {
        let e = JPHistE { x: p.x, y: p.y, vx: p.vx, vy: p.vy };
        let hs = std::sync::Arc::make_mut(p.hal_mut().hist.get_or_insert_with(Default::default));
        hs.push(e);
        if hs.len() > 14 {
            hs.remove(0);
        }
    }
    if tick % 20.0 == 0.0 {
        for p in c.ps.iter_mut() {
            let e = JPPastE { x: p.x, y: p.y };
            let pa = std::sync::Arc::make_mut(p.hal_mut().past.get_or_insert_with(Default::default));
            pa.push(e);
            if pa.len() > 4 {
                pa.remove(0);
            }
        }
    }
    let top_id: Option<TopRef> = c.top_id();
    let mut att: u8 = match h {
        Some(h) => c.ps[h].takim(),
        None => c.ball.as_ref().expect("position: top yok").b.team,
    };
    let mut E: (f64, f64) = match h {
        Some(h) => (c.ps[h].x, c.ps[h].y),
        None => (0.0, 0.0),
    };
    let mut fr: Option<Arc<Fr>> = None;
    let mut belirgin: Option<bool> = None;
    if h.is_none() {
        let taze = matches!(&c.ek.sh.fr, Some(_) if c.ek.sh.frT > tick - 3.0 && c.ek.sh.frB == top_id);
        if !taze {
            let f = flightRead(c).map(Arc::new);
            c.ek.sh.fr = f;
            c.ek.sh.frT = tick;
            c.ek.sh.frB = top_id;
        }
        let f0 = c.ek.sh.fr.clone().expect("position: uçuş okunamadı");
        let first = f0.first.unwrap();
        let f = f0.arr[first];
        E = (f.x, f.y);
        let ps = &*c.ps;
        let mut other = 1e9;
        for (q, r) in f0.arr.iter().enumerate() {
            if ps[q].team != ps[first].team {
                other = jmin(other, r.t);
            }
        }
        if ps[first].bekci() && ps[first].hal.chB != top_id {
            let mut fp: Option<Pid> = None;
            let mut fr0: Option<Ar> = None;
            for (q, rq) in f0.arr.iter().enumerate() {
                if !ps[q].bekci() && !ps[q].noTouch && (fr0.is_none() || rq.t < fr0.unwrap().t) {
                    fp = Some(q);
                    fr0 = Some(*rq);
                }
            }
            if let Some(fp) = fp {
                let fr0 = fr0.unwrap();
                let mut oth = 1e9;
                for (q, r) in f0.arr.iter().enumerate() {
                    if ps[q].team != ps[fp].team && !(ps[q].bekci() && ps[q].hal.chB != top_id) {
                        oth = jmin(oth, r.t);
                    }
                }
                E = (fr0.x, fr0.y);
                if oth > 1.25 * fr0.t + 4.0 {
                    att = ps[fp].takim();
                }
                belirgin = Some(oth - fr0.t >= 12.0);
            }
        } else {
            if other > 1.25 * f.t + 4.0 {
                att = ps[first].takim();
            }
            belirgin = Some(other - f.t >= 12.0);
        }
        fr = Some(f0);
    }
    if h.is_none() {
        if let (Some(b), Some(sa)) = (c.ball.as_ref(), c.ek.sh.att) {
            if att != sa && hypot(b.b.x - ownX(att), b.b.y - GCY) < 25.0 && !belirgin.unwrap_or(false) {
                att = sa;
            }
        }
    }
    {
        let sh = &mut c.ek.sh;
        if h.is_some() || sh.att.is_none() || Some(att) == sh.att {
            sh.aday = None;
        } else {
            if sh.aday != Some(att) {
                sh.aday = Some(att);
                sh.adayT = tick;
            }
            if tick - sh.adayT < 10.0 {
                att = sh.att.unwrap();
            } else {
                sh.aday = None;
            }
        }
    }
    if c.ek.sh.att != Some(att) {
        let fresh = c.ek.sh.att.is_some();
        c.ek.sh.att = Some(att);
        c.ek.sh.gain = tick;
        for p in c.ps.iter_mut() {
            p.hal_mut().lock = Some(0.0);
            p.hal_mut().kapPrev = None;
            p.hal_mut().zonePrev = None;
            p.hal_mut().swPrev = None;
            p.hal_mut().uaPrev = None;
            p.hal_mut().rec = Some(if fresh && p.takim() != att && !p.bekci() && (p.x - E.0) * dirOf(p.takim()) > 2.0 {
                tick + jmax(2.0, jround(26.0 - p.govde.yet.y(mek::PRES_BASLA) * S.recOk))
            } else {
                0.0
            });
        }
    }
    {
        let bdefl = c.ball.as_ref().map(|b| b.b.defl).unwrap_or(0.0);
        let sh = &mut c.ek.sh;
        if sh.ball != Some(top_id) || (top_id.is_some() && bdefl != sh.defl) {
            sh.ball = Some(top_id);
            sh.ballT = tick;
            sh.defl = if top_id.is_some() { bdefl } else { 0.0 };
        }
    }
    let since = tick - c.ek.sh.gain;
    if let Some(h) = h {
        for p in c.ps.iter_mut() {
            if kilitli(p, tick) && p.hal._lockH != Some(h) {
                p.hal_mut().lock = Some(0.0);
            }
        }
    }
    for p in c.ps.iter_mut() {
        if p.takim() == att {
            p.hal_mut().tutQ = None;
        }
        p.sprint = false;
        p.press = kilitli(p, tick) && !p.noTouch;
        p.job = Some(if p.noTouch { "geçildi".into() } else { String::new().into() });
    }
    if let Some(hh) = h {
        if c.m.holder.is_none() && c.ball.is_some() && c.m.fl.as_ref().and_then(|f| f.q) == Some(hh) {
            if let Some(f0) = flightRead(c) {
                let r0 = f0.arr[hh];
                let p = &mut c.ps[hh];
                p.tx = r0.x;
                p.ty = r0.y;
                p.sprint = true;
                p.job = Some("alıcı".into());
                p.hal_mut().chB = top_id;
            }
        }
    }
    let mut chase = vec![false; n];
    kovala(c, fr.as_deref(), att, top_id, &mut chase);
    // gönderme (şut) sonrası sekmeye koşanlar
    let gon = c.m.fl.as_ref().map(|f| f.kind == "gönder").unwrap_or(false);
    if gon {
        if let Some(b) = c.ball.as_ref() {
            if b.b.done.is_none() && b.b.defl == 0.0 {
                let bt = b.b.team;
                let bfrom = b.t.from;
                let gx = oppX(bt);
                let dr = dirOf(bt);
                let ballT = c.ek.sh.ballT;
                for i in 0..n {
                    let p = &c.ps[i];
                    if p.takim() == bt && !p.bekci() && Some(i) != bfrom && !chase[i] && hypot(p.x - gx, p.y - GCY) < 28.0 && tick - ballT >= TEPKI(p, Tepki::Kovala, k) {
                        chase[i] = true;
                        let p = &mut c.ps[i];
                        p.tx = gx - dr * (5.0 + p.id % 3.0 * 2.0);
                        p.ty = GCY + 4.0 * (p.id % 4.0 - 1.5);
                        p.sprint = true;
                        p.job = Some("sekme".into());
                    }
                }
            }
        }
    }
    let dt = 1 - att;
    let dtac = dis::tacEff(c, dt);
    let mut sv = savunma::Sv::yeni(c, h, KT, att, dt, dtac, E, fr, chase, since);
    sv.savunma(c);
    sv.hucum(c);
    sv.son(c);
    c.m.E = Some(crate::model::JME { x: E.0, y: E.1 });
    c.m.att = Some(att as f64);
}

/// Kovalayanlar (JS `chase`): topa ilk varanlar, alıcı, Bekçi'nin çıkışı.
fn kovala(c: &mut Mc, fr: Option<&Fr>, att: u8, top_id: Option<TopRef>, chase: &mut [bool]) {
    let fr = match fr {
        Some(f) => f,
        None => return,
    };
    let k = c.k;
    let tick = c.m.tick;
    let n = c.ps.len();
    let flq = c.m.fl.as_ref().and_then(|f| f.q);
    let fl = c.m.fl.is_some();
    for t in [0u8, 1u8] {
        let (bdefl, bdone, bteam, brecv, bgon, bsap0, bfrom) = {
            let b = c.ball.as_ref().unwrap();
            (b.b.defl != 0.0, b.b.done.is_some(), b.b.team, b.t.recv, b.t.gon, b.t.sap, b.t.from)
        };
        let own0 = flq.is_some() && !bdefl && !bdone && bteam == t;
        let mine: Vec<Pid> = (0..n).filter(|&i| { let p = &c.ps[i]; !(p.takim() != t || p.noTouch || (p.bekci() && own0 && flq != Some(i))) }).collect();
        if mine.is_empty() {
            continue;
        }
        let bestE = mine.iter().map(|&i| fr.arr[i].est).fold(f64::INFINITY, jmin);
        let bestS = mine.iter().filter(|&&i| !c.ps[i].bekci()).map(|&i| fr.arr[i].est).fold(f64::INFINITY, jmin);
        if let Some(fq) = flq {
            if c.ps[fq].takim() == t && brecv == Some(fq) && !bdefl && !bdone && !bgon && !bsap0 {
                let rq0 = fr.arr[fq];
                let (mut oE0, mut mE0) = (1e9, 1e9);
                for (q, r0) in fr.arr.iter().enumerate() {
                    let qp = &c.ps[q];
                    if qp.noTouch {
                        continue;
                    }
                    if qp.takim() != t {
                        oE0 = jmin(oE0, r0.est);
                    } else if q != fq && !qp.bekci() && Some(q) != bfrom {
                        mE0 = jmin(mE0, r0.est);
                    }
                }
                if rq0.est >= oE0 && mE0 + 4.0 < jmin(rq0.est, oE0) {
                    c.ball.as_mut().unwrap().t.sap = true;
                }
            }
        }
        let bsap = c.ball.as_ref().unwrap().t.sap;
        let loose = !fl || flq.is_none() || bdefl || bdone;
        let rq = flq.map(|q| fr.arr[q]);
        let recvFirst = match rq {
            Some(rq) => !bsap && (rq.est <= bestE + 10.0 || (brecv == flq && !bdefl && !bsap)),
            None => false,
        };
        let ownPass = !loose && bteam == t;
        let firstMate = if ownPass && !recvFirst {
            let mut a = mine[0];
            for &b in &mine[1..] {
                if fr.arr[b].est < fr.arr[a].est {
                    a = b;
                }
            }
            Some(a)
        } else {
            None
        };
        let toMate = !loose && bteam == t && recvFirst;
        let mut oEk = 1e9;
        let mut oP: Option<Ar> = None;
        for (q, rq) in fr.arr.iter().enumerate() {
            if c.ps[q].takim() != t && !c.ps[q].noTouch && rq.est < oEk {
                oEk = rq.est;
                oP = Some(*rq);
            }
        }
        for &pi in &mine {
            let r = fr.arr[pi];
            let recv = fl && flq == Some(pi) && !loose;
            if toMate && !recv {
                continue;
            }
            let pb = c.ps[pi].bekci();
            let pteam = c.ps[pi].takim();
            if !loose && bteam != t && !pb && t != att {
                continue;
            }
            if ownPass && !toMate && Some(pi) != firstMate {
                continue;
            }
            let mut rt = if pb { jmax(0.0, bek_re(&c.ps[pi])) } else if loose { TEPKI(&c.ps[pi], Tepki::BosTop, k) } else { TEPKI(&c.ps[pi], Tepki::Kovala, k) };
            if !loose {
                let ps = &*c.ps;
                rt = okuTb(c.ball.as_deref_mut(), &ps[pi], ps, rt, false);
            }
            if !recv && c.ps[pi].hal.chB != top_id && tick - c.ek.sh.ballT < rt {
                continue;
            }
            let basla = loose && !pb && hypot(r.x - oppX(pteam), r.y - GCY) < 35.0 && r.est <= bestS;
            if !basla && !recv && !ownPass && !pb && r.est > oEk + 6.0 && !(loose && hypot(r.x - ownX(pteam), r.y - GCY) < 25.0) {
                continue;
            }
            let bkA = pb && !recv;
            let bkIc = bkA && hypot(r.x - ownX(pteam), r.y - GCY) <= KR;
            let bkCik = bkA && !bkIc && !loose && bteam != t && r.est <= oEk + S.bekCikPay + (if top_id.is_some() && c.ek.bkCik[pi] == top_id { S.bekCikSur } else { 0.0 }) && r.est + S.bekCikSaha < bestS;
            if !bkA || bkIc || loose || bkCik {
                if pb && !recv && !bkCik && hypot(r.x - ownX(pteam), r.y - GCY) > KR {
                    let mut oE = 1e9;
                    for (q, rq2) in fr.arr.iter().enumerate() {
                        if c.ps[q].takim() != t && !c.ps[q].noTouch && rq2.est < oE {
                            oE = rq2.est;
                        }
                    }
                    if r.est + S.bekOnce + (20.0 - c.ps[pi].govde.yet.y(mek::CIKIS)) * S.marginT >= oE {
                        continue;
                    }
                }
                let esik = (if pb { bestE } else { bestS }) + if pb { if bkCik { S.bekCikPay } else { 0.0 } } else { (20.0 - c.ps[pi].govde.yet.y(mek::ALGI_HATA)) * S.marginT + 1.0 };
                if recv || bkIc || (loose && !pb && hypot(r.x - ownX(pteam), r.y - GCY) < 25.0 && r.est <= oEk + 4.0) || r.est <= esik {
                    chase[pi] = true;
                    let p = &mut c.ps[pi];
                    p.tx = r.x;
                    p.ty = r.y;
                    p.sprint = true;
                    p.job = Some(if recv { "alıcı" } else if bkCik { "kovala · çık" } else { "kovala" }.into());
                    p.hal_mut().chB = top_id;
                    if bkCik {
                        c.ek.bkCik[pi] = top_id;
                    }
                    if !recv && !pb && oP.is_some() && r.est > oEk + 1.0 && bteam != t {
                        let o = oP.unwrap();
                        let g = gside(o.x, o.y, t, 2.2);
                        let p = &mut c.ps[pi];
                        p.tx = g.0;
                        p.ty = g.1;
                        p.job = Some("kovala · kale tarafına".into());
                    }
                } else if basla {
                    chase[pi] = true;
                    let p = &mut c.ps[pi];
                    p.tx = r.x;
                    p.ty = r.y;
                    p.sprint = true;
                    p.job = Some("kovala · bas".into());
                    p.hal_mut().chB = top_id;
                }
            }
        }
    }
}

#[allow(unused_imports)]
use Oyuncu as _O;
