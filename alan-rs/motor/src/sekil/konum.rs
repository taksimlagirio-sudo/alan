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
pub use savunma::{yerlesik_adam, YERLESIK_ADAM_ZORLA};
mod yardim;

use yardim::*;

/// JS position(m).
pub fn position(c: &mut Mc) {
    let k = c.k;
    let n = c.ps.len();
    let tick = c.m.tick;
    if c.ek.bk_cik.len() != n {
        c.ek.bk_cik = vec![None; n];
    }
    let KT = if c.m.holder.is_some() { None } else { kendi_topu(c) };
    let h: Option<Pid> = c.m.holder.or(KT.map(|x| x.0));
    for p in c.ps.iter_mut() {
        let e = JPHistE { x: p.x, y: p.y, vx: p.vx, vy: p.vy };
        let hs = std::sync::Arc::make_mut(&mut p.hal_mut().hist);
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
        let taze = matches!(&c.ek.sh.fr, Some(_) if c.ek.sh.fr_t > tick - 3.0 && c.ek.sh.fr_b == top_id);
        if !taze {
            let f = flight_read(c).map(Arc::new);
            c.ek.sh.fr = f;
            c.ek.sh.fr_t = tick;
            c.ek.sh.fr_b = top_id;
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
            if att != sa && hypot(b.b.x - own_x(att), b.b.y - GCY) < 25.0 && !belirgin.unwrap_or(false) {
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
                sh.aday_t = tick;
            }
            if tick - sh.aday_t < 10.0 {
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
            p.hal_mut().lock = 0.0;
            p.hal_mut().kapPrev = None;
            p.hal_mut().zonePrev = None;
            p.hal_mut().swPrev = None;
            p.hal_mut().uaPrev = None;
            p.hal_mut().rec = if fresh && p.takim() != att && !p.bekci() && (p.x - E.0) * dir_of(p.takim()) > 2.0 {
                tick + jmax(2.0, jround(26.0 - p.govde.yet.y(mek::PRES_BASLA) * S.rec_ok))
            } else {
                0.0
            };
        }
    }
    {
        let bdefl = c.ball.as_ref().map(|b| b.b.defl).unwrap_or(0.0);
        let sh = &mut c.ek.sh;
        if sh.ball != Some(top_id) || (top_id.is_some() && bdefl != sh.defl) {
            sh.ball = Some(top_id);
            sh.ball_t = tick;
            sh.defl = if top_id.is_some() { bdefl } else { 0.0 };
        }
    }
    let since = tick - c.ek.sh.gain;
    if let Some(h) = h {
        for p in c.ps.iter_mut() {
            if kilitli(p, tick) && p.hal._lockH != Some(h) {
                p.hal_mut().lock = 0.0;
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
            if let Some(f0) = flight_read(c) {
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
                let gx = opp_x(bt);
                let dr = dir_of(bt);
                let ball_t = c.ek.sh.ball_t;
                for i in 0..n {
                    let p = &c.ps[i];
                    if p.takim() == bt && !p.bekci() && Some(i) != bfrom && !chase[i] && hypot(p.x - gx, p.y - GCY) < 28.0 && tick - ball_t >= TEPKI(p, Tepki::Kovala, k) {
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
    let dtac = dis::tac_eff(c, dt);
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
        let best_e = mine.iter().map(|&i| fr.arr[i].est).fold(f64::INFINITY, jmin);
        let best_s = mine.iter().filter(|&&i| !c.ps[i].bekci()).map(|&i| fr.arr[i].est).fold(f64::INFINITY, jmin);
        if let Some(fq) = flq {
            if c.ps[fq].takim() == t && brecv == Some(fq) && !bdefl && !bdone && !bgon && !bsap0 {
                let rq0 = fr.arr[fq];
                let (mut o_e0, mut m_e0) = (1e9, 1e9);
                for (q, r0) in fr.arr.iter().enumerate() {
                    let qp = &c.ps[q];
                    if qp.noTouch {
                        continue;
                    }
                    if qp.takim() != t {
                        o_e0 = jmin(o_e0, r0.est);
                    } else if q != fq && !qp.bekci() && Some(q) != bfrom {
                        m_e0 = jmin(m_e0, r0.est);
                    }
                }
                if rq0.est >= o_e0 && m_e0 + 4.0 < jmin(rq0.est, o_e0) {
                    c.ball.as_mut().unwrap().t.sap = true;
                }
            }
        }
        let bsap = c.ball.as_ref().unwrap().t.sap;
        let loose = !fl || flq.is_none() || bdefl || bdone;
        let rq = flq.map(|q| fr.arr[q]);
        let recv_first = match rq {
            Some(rq) => !bsap && (rq.est <= best_e + 10.0 || (brecv == flq && !bdefl && !bsap)),
            None => false,
        };
        let own_pass = !loose && bteam == t;
        let first_mate = if own_pass && !recv_first {
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
        let to_mate = !loose && bteam == t && recv_first;
        let mut o_ek = 1e9;
        let mut o_p: Option<Ar> = None;
        for (q, rq) in fr.arr.iter().enumerate() {
            if c.ps[q].takim() != t && !c.ps[q].noTouch && rq.est < o_ek {
                o_ek = rq.est;
                o_p = Some(*rq);
            }
        }
        for &pi in &mine {
            let r = fr.arr[pi];
            let recv = fl && flq == Some(pi) && !loose;
            if to_mate && !recv {
                continue;
            }
            let pb = c.ps[pi].bekci();
            let pteam = c.ps[pi].takim();
            if !loose && bteam != t && !pb && t != att {
                continue;
            }
            if own_pass && !to_mate && Some(pi) != first_mate {
                continue;
            }
            let mut rt = if pb { jmax(0.0, bek_re(&c.ps[pi])) } else if loose { TEPKI(&c.ps[pi], Tepki::BosTop, k) } else { TEPKI(&c.ps[pi], Tepki::Kovala, k) };
            if !loose {
                let ps = &*c.ps;
                rt = okuTb(c.ball.as_deref_mut(), &ps[pi], ps, rt, false);
            }
            if !recv && c.ps[pi].hal.chB != top_id && tick - c.ek.sh.ball_t < rt {
                continue;
            }
            let basla = loose && !pb && hypot(r.x - opp_x(pteam), r.y - GCY) < 35.0 && r.est <= best_s;
            if !basla && !recv && !own_pass && !pb && r.est > o_ek + 6.0 && !(loose && hypot(r.x - own_x(pteam), r.y - GCY) < 25.0) {
                continue;
            }
            let bk_a = pb && !recv;
            let bk_ic = bk_a && hypot(r.x - own_x(pteam), r.y - GCY) <= KR;
            let bk_cik = bk_a && !bk_ic && !loose && bteam != t && r.est <= o_ek + S.bek_cik_pay + (if top_id.is_some() && c.ek.bk_cik[pi] == top_id { S.bek_cik_sur } else { 0.0 }) && r.est + S.bek_cik_saha < best_s;
            if !bk_a || bk_ic || loose || bk_cik {
                if pb && !recv && !bk_cik && hypot(r.x - own_x(pteam), r.y - GCY) > KR {
                    let mut o_e = 1e9;
                    for (q, rq2) in fr.arr.iter().enumerate() {
                        if c.ps[q].takim() != t && !c.ps[q].noTouch && rq2.est < o_e {
                            o_e = rq2.est;
                        }
                    }
                    if r.est + S.bek_once + (20.0 - c.ps[pi].govde.yet.y(mek::CIKIS)) * S.margin_t >= o_e {
                        continue;
                    }
                }
                let esik = (if pb { best_e } else { best_s }) + if pb { if bk_cik { S.bek_cik_pay } else { 0.0 } } else { (20.0 - c.ps[pi].govde.yet.y(mek::ALGI_HATA)) * S.margin_t + 1.0 };
                if recv || bk_ic || (loose && !pb && hypot(r.x - own_x(pteam), r.y - GCY) < 25.0 && r.est <= o_ek + 4.0) || r.est <= esik {
                    chase[pi] = true;
                    let p = &mut c.ps[pi];
                    p.tx = r.x;
                    p.ty = r.y;
                    p.sprint = true;
                    p.job = Some(if recv { "alıcı" } else if bk_cik { "kovala · çık" } else { "kovala" }.into());
                    p.hal_mut().chB = top_id;
                    if bk_cik {
                        c.ek.bk_cik[pi] = top_id;
                    }
                    if !recv && !pb && o_p.is_some() && r.est > o_ek + 1.0 && bteam != t {
                        let o = o_p.unwrap();
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
