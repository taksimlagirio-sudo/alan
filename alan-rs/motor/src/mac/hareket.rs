//! Hareket (match.js 1597–1834, 2109–2560, 2594–2750): koşu hızı, ivme, dolanma, ilişkiler (boşluk doldurma,
//! pres tuzağı), perde, fake, savurma, ön hareket ve tikin hareket adımı `move`.
pub use crate::kural::hareket::kuyuSafe;
use super::enerji::*;
use super::gecici_dis as dis;
use super::ikili::{dengeHiz, govdeAyir, ikOn, kutle, omuzK, IkBag, IkHedef};
use super::kurulum::tlv;
use super::*;
use crate::core_sabit::{IN, Q, TQ};
use crate::mat::{acos, atan2, cos, hypot, jmax, jmin, jround, jsign, sin, PI};
use crate::model::{JPSavur, Secenek, JP_onH};
use crate::saha::{CY, H, KR};
use crate::yetenek::{ag, kademe, mek, zar};
pub use crate::kural::hareket::ivme;

/// JS topHiz(m, p): topla koşunun katı (Toplu koşu; omuzdan sonra yavaşlama).
pub fn topHiz(m: &Mac, p: &Oyuncu) -> f64 {
    let s = p.govde.yet.y(mek::TOPLU_KOSU);
    let k = jmin(1.0, 0.68 + 0.02 * s);
    let sl = if oo(p.hal.slowT) > m.tick { jmax(0.4, jmin(0.85, dis::DU::slowK + 0.025 * (s - 10.0))) } else { 1.0 };
    k * sl
}


fn pres_fake_kovala(j: &str) -> bool {
    bas(j, &["pres", "fake", "kovala"])
}

/// JS dolan(m): taşıyıcıya gövdesinin arkasından gelen presçi yandan dolanır; yolundaki oyuncunun etrafından geçer.
pub fn dolan(m: &mut Mac) {
    let Some(h) = m.holder else { return };
    let r0 = 2.4;
    let (hx, hy, ht) = (m.ps[h].x, m.ps[h].y, m.ps[h].team);
    let ca = m.ps[h].ca.unwrap_or(0.0);
    let bx = cos(ca);
    let by = sin(ca);
    for pi in 0..m.ps.len() {
        let p = &m.ps[pi];
        if p.team == ht || p.bekci() || !pres_fake_kovala(job(p)) {
            continue;
        }
        let al = hypot(p.tx - p.x, p.ty - p.y);
        if al < 0.3 {
            continue;
        }
        let fx = hx - p.x;
        let fy = hy - p.y;
        let fd = hypot(fx, fy);
        if fd > 4.0 {
            continue;
        }
        if !crate::donus::shielded(hx, hy, p.x, p.y, p.tx, p.ty) {
            continue;
        }
        let fdd = if dogru(fd) { fd } else { 1.0 };
        let px = -fy / fdd;
        let py = fx / fdd;
        let sg = if px * bx + py * by >= 0.0 { 1.0 } else { -1.0 };
        let mut a = atan2(py * sg, px * sg);
        if p.hal._kAnT == Some(m.tick) {
            let ka = p.hal._kAn.unwrap_or(f64::NAN);
            let dv = atan2(sin(a - ka), cos(a - ka));
            let mx = acos(1.2 / r0);
            if dv.abs() > mx {
                a = ka + jsign(dv) * mx;
            }
        }
        let p = &mut m.ps[pi];
        p.tx = hx + cos(a) * r0;
        p.ty = hy + sin(a) * r0;
        p.sprint = true;
        p.job = Some(format!("{} · dolanıyor", job(p)).into());
    }
    for pi in 0..m.ps.len() {
        let p = &m.ps[pi];
        if p.team == ht || p.bekci() || !pres_fake_kovala(job(p)) {
            continue;
        }
        let ax = p.tx - p.x;
        let ay = p.ty - p.y;
        let l = hypot(ax, ay);
        if l < 0.5 {
            continue;
        }
        let mut ob: Option<Pid> = None;
        let mut ou = 1.0;
        for (ci, c) in m.ps.iter().enumerate() {
            if ci == pi || ci == h {
                continue;
            }
            let u = ((c.x - p.x) * ax + (c.y - p.y) * ay) / (l * l);
            if !(u <= 0.0 || u >= 1.0 || u >= ou) && hypot(p.x + ax * u - c.x, p.y + ay * u - c.y) < 1.8 {
                ob = Some(ci);
                ou = u;
            }
        }
        let Some(ob) = ob else { continue };
        let (obx, oby) = (m.ps[ob].x, m.ps[ob].y);
        if hypot(obx - p.x, oby - p.y) > 4.0 {
            continue;
        }
        let nx = -ay / l;
        let ny = ax / l;
        let sg = if (p.tx - obx) * nx + (p.ty - oby) * ny >= 0.0 { 1.0 } else { -1.0 };
        let p = &mut m.ps[pi];
        p.tx = obx + nx * sg * 2.4 + ax / l * 0.8;
        p.ty = oby + ny * sg * 2.4 + ay / l * 0.8;
        p.sprint = true;
        if !job(p).contains("dolanıyor") {
            p.job = Some(format!("{} · dolanıyor", job(p)).into());
        }
    }
}

pub const PERDE: f64 = 1.9;

/// JS iliski(m): boşluk doldurma (doldur talimatı) ve kenar pres tuzağı.
pub fn iliski(m: &mut Mac) {
    let Some(h) = m.holder else { return };
    let att = m.ps[h].team;
    let dt = 1.0 - att;
    let dir = if att == 0.0 { 1.0 } else { -1.0 };
    for pi in 0..m.ps.len() {
        let p = &m.ps[pi];
        if p.team != att || pi == h || p.bekci() {
            continue;
        }
        let j = super::kurulum::tl(p, "huc", "doldur");
        let Some(j) = j.filter(|&j| dogru(j)) else { continue };
        let Some(a) = (0..m.ps.len()).find(|&q| m.ps[q].team == att && m.ps[q].govde.i == j) else { continue };
        if a == h {
            continue;
        }
        let A = &m.ps[a];
        let s = A.govde.rslot.as_ref().map_or((A.govde.slot.d, A.govde.slot.l), |r| (r.d, r.l));
        let bx = (if att == 0.0 { 0.0 } else { W }) + dir * (8.0 + 84.0 * s.0) * KX;
        let by = CY + 20.0 * s.1 * KY;
        if (A.x - bx) * dir > 8.0 && !icer(job(p), &["kovala", "alıcı", "koşu"]) {
            let ad = if A.govde.name.is_empty() { js_num(j) } else { A.govde.name.to_string() };
            let p = &mut m.ps[pi];
            p.tx = bx;
            p.ty = by;
            p.sprint = hypot(p.x - bx, p.y - by) > 6.0;
            p.job = Some(format!("boşluk doldurma · {ad}").into());
        }
    }
    let tuzak = m.tac.get(dt as usize).and_then(|t| t.tuzak.clone());
    let (hx, hy) = (m.ps[h].x, m.ps[h].y);
    if tuzak.as_deref() == Some("kenar") && (hy - CY).abs() > 13.0 * KY && !m.ps[h].bekci() {
        let defs: Vec<Pid> = (0..m.ps.len()).filter(|&i| m.ps[i].team == dt && !m.ps[i].bekci() && !m.ps[i].noTouch).collect();
        let d = |i: Pid| hypot(m.ps[i].x - hx, m.ps[i].y - hy);
        let srt = |v: &mut Vec<Pid>| v.sort_by(|&a, &b| d(a).partial_cmp(&d(b)).unwrap_or(std::cmp::Ordering::Equal));
        let mut pr: Vec<Pid> = defs.iter().copied().filter(|&i| m.ps[i].press).collect();
        srt(&mut pr);
        let p2 = pr.get(1).copied().or_else(|| {
            let mut v: Vec<Pid> = defs.iter().copied().filter(|&i| !m.ps[i].press && m.ps[i].govde.slot.d > 0.25 && d(i) < 14.0).collect();
            srt(&mut v);
            v.first().copied()
        });
        let sg = if hy < CY { 1.0 } else { -1.0 };
        let own = if dt == 0.0 { 0.0 } else { W };
        if let Some(&p0) = pr.first() {
            m.ps[p0].ty = hy + 0.8 * sg;
        }
        if let Some(p2) = p2 {
            let p = &mut m.ps[p2];
            p.tx = hx + 0.1 * (own - hx);
            p.ty = hy + 3.6 * sg;
            p.sprint = true;
            p.press = true;
            p.job = Some("pres tuzağı · içi kapat".into());
        }
    }
}

/// JS perde(m): takım arkadaşı presçinin önüne perde olur (bugün çağrılmıyor; dışa açık).
pub fn perde(m: &mut Mac) {
    let Some(h) = m.holder.filter(|&h| !m.ps[h].bekci()) else {
        m._perde = None;
        return;
    };
    let (hx, hy, ht) = (m.ps[h].x, m.ps[h].y, m.ps[h].team);
    if let Some(p0) = m._perde.clone() {
        let c = idden(m, p0.cid);
        let q = idden(m, p0.qid);
        if let (Some(c), Some(q)) = (c, q) {
            let (qx, qy) = (m.ps[q].x, m.ps[q].y);
            if !(ht != m.ps[c].team || m.tick > p0.until || hypot(qx - hx, qy - hy) > 9.0) {
                let l = { let l = hypot(hx - qx, hy - qy); if dogru(l) { l } else { 1.0 } };
                let c = &mut m.ps[c];
                c.tx = qx + (hx - qx) / l * PERDE;
                c.ty = qy + (hy - qy) / l * PERDE;
                c.sprint = true;
                c.job = Some("perde".into());
                return;
            }
        }
        m._perde = None;
    }
    let mut q: Option<Pid> = None;
    let mut qd = 8.0;
    for (zi, z) in m.ps.iter().enumerate() {
        if z.team != ht && pres_fake_kovala(job(z)) {
            let dd = hypot(z.x - hx, z.y - hy);
            if dd < qd && dd > 2.6 {
                qd = dd;
                q = Some(zi);
            }
        }
    }
    let Some(q) = q else { return };
    let qid = m.ps[q].id;
    if m.mac._perdeQ.iter().any(|x| x.0 == qid && x.1 > m.tick) {
        return;
    }
    let tk = m.tick + 60.0;
    match m.mac._perdeQ.iter_mut().find(|x| x.0 == qid) {
        Some(x) => x.1 = tk,
        None => m.mac._perdeQ.push((qid, tk)),
    }
    let (qx, qy) = (m.ps[q].x, m.ps[q].y);
    let mut c: Option<Pid> = None;
    let mut cd = 7.0;
    for (zi, z) in m.ps.iter().enumerate() {
        if z.team == ht && zi != h && !z.bekci() && !icer(job(z), &["alıcı", "koşu", "kovala"]) {
            let dd = hypot(z.x - qx, z.y - qy);
            if dd < cd {
                cd = dd;
                c = Some(zi);
            }
        }
    }
    let Some(c) = c else { return };
    let ok = m.ps[c].govde.yet.ozk("yeralma");
    if m.r.sonraki() < jmax(0.0, jmin(0.9, 0.06 * (ok - 4.0))) {
        m._perde = Some(Perde { cid: m.ps[c].id, qid, until: m.tick + 90.0 });
    }
}

/// JS otAkil(m, q, b): tek dokunuşu düşünür mü (topa bir kez zar).
pub fn otAkil(m: &mut Mac, q: Pid, b: crate::model::TopRef) -> bool {
    let p = &m.ps[q];
    if p.hal._otA.as_ref().map_or(true, |a| a.b != b) {
        let ok = zar(p.id, m.tick, 11.0) < kademe(p.govde.yet.y(mek::TEK_DOKUNUS_DUSUN), dis::D::otOk);
        m.ps[q].hal_mut()._otA = Some(crate::model::JP_otA { b, ok });
    }
    m.ps[q].hal._otA.as_ref().unwrap().ok
}

/// JS yanil(m, p, hedef, neden): oyuncu yanlış yöne adanır.
pub fn yanil(m: &mut Mac, pi: Pid, hx: f64, hy: f64, neden: &str) {
    let tick = m.tick;
    let p = &mut m.ps[pi];
    let ok = p.govde.yet.y(mek::FAKE_KAN);
    let cv = p.govde.yet.ceviklik - 10.0;
    p.hal_mut().adandi = Some(jmax(oo(p.hal.adandi), tick + jmax(3.0, jround(3.0 + 0.45 * jmax(0.0, 20.0 - ok) - 0.2 * cv))));
    p.hal_mut().adT = Some(crate::model::JPAdT { x: hx, y: hy });
    p.hal_mut().adNeden = Some(neden.into());
    p.cd = jmax(or0(p.cd), p.hal.adandi.unwrap());
}

/// JS fakeBasla(m, h, o)
pub fn fakeBasla(m: &mut Mac, h: Pid, o: &Secenek) {
    let sh = o.show.clone().unwrap_or_default();
    let T = dis::fakeT(m, h);
    let mut fooled: Vec<f64> = vec![];
    let mut ad: Vec<String> = vec![];
    for pi in 0..m.ps.len() {
        if dis::fakeIlgili(m, pi, h, &sh) {
            let pr = dis::fakeP(m, h, pi, &sh.k, sh.c);
            if m.r.sonraki() < pr {
                fooled.push(m.ps[pi].id);
                ad.push(if m.ps[pi].bekci() { "Bekçi yattı".into() } else { format!("{} aldandı", m.ps[pi].govde.name) });
            }
        }
    }
    let mut tg = vec![];
    let mut ra = vec![];
    for pi in 0..m.ps.len() {
        let id = m.ps[pi].id;
        if fooled.contains(&id) {
            let t = dis::fakeHedef(m, pi, h, &sh);
            tg.push((id, t));
            yanil(m, pi, t.0, t.1, "aldandı");
            ra.push((id, m.ps[pi].hal.adandi.unwrap_or(0.0)));
        }
    }
    let v = EN.fake * m.ps[h].govde.yet.verim(mek::FAKE_BEDEL);
    enHarca(m, h, "fake", v);
    let ht = m.ps[h].team;
    m.mac.fake = Some(Fake { team: ht, hid: m.ps[h].id, k: sh.k.clone(), to: sh.to, t0: m.tick, until: m.tick + T, react: m.tick + T + 8.0, fooled: fooled.clone(), tg, ra, ax: None, ay: None });
    if m.st.fake.is_empty() {
        m.st.fake = vec![0.0, 0.0];
    }
    m.st.fake[ht as usize] += 1.0;
    if !fooled.is_empty() {
        if m.st.fakeOk.is_empty() {
            m.st.fakeOk = vec![0.0, 0.0];
        }
        m.st.fakeOk[ht as usize] += 1.0;
    }
    let tick = m.tick;
    {
        let hp = &mut m.ps[h];
        hp.hal_mut().fkBlock = true;
        hp.hal_mut().fkCool = Some(tick + T + 45.0);
        hp.hal_mut().driveTo = None;
        hp.tx = hp.x;
        hp.ty = hp.y;
    }
    m.decT = T + 1.0;
    let ne = if sh.k == "gönder" { "göndermeyi" } else if sh.k == "pas" { "pası" } else { "yana gidişi" };
    let ek = if ad.is_empty() { " · kimse aldanmadı".to_string() } else { format!(" · {}", ad.join(", ")) };
    let n = m.ps[h].govde.name.clone();
    ev(m, "fake", ht, format!("{n} {ne} gösterdi{ek}"));
}

/// JS fakeYonet(m)
pub fn fakeYonet(m: &mut Mac) {
    for p in m.m.ps.iter_mut() {
        if p.hal.fkBlock && m.m.tick > oo(p.hal.fkCool) {
            p.hal_mut().fkBlock = false;
        }
    }
    let Some(f) = m.mac.fake.as_ref() else { return };
    let h = m.holder;
    let hid = h.map(|h| m.ps[h].id);
    if m.tick > f.react || h.is_none() || (hid != Some(f.hid) && m.tick <= f.until) {
        m.mac.fake = None;
        return;
    }
    let h = h.unwrap();
    if hid == Some(f.hid) && m.tick <= f.until {
        let mut f = m.mac.fake.take().unwrap();
        let hp = &mut m.m.ps[h];
        hp.hal_mut().driveTo = None;
        let fz = (m.m.tick - f.t0) / jmax(1.0, f.until - f.t0);
        let su = atan2(f.to.1 - hp.y, f.to.0 - hp.x);
        if f.k == "gövde" {
            if f.ax.is_none() {
                f.ax = Some(hp.x + 0.8 * cos(su));
                f.ay = Some(hp.y + 0.8 * sin(su));
            }
            hp.tx = f.ax.unwrap();
            hp.ty = f.ay.unwrap();
            hp.sprint = false;
        } else {
            hp.tx = hp.x;
            hp.ty = hp.y;
        }
        let ta = if f.k == "gövde" { su } else if fz < 0.4 { su + PI } else { su };
        let c0 = hp.ca.unwrap_or(ta);
        let dd = atan2(sin(ta - c0), cos(ta - c0));
        let w = 1.6 * crate::donus::omega(hp, false, super::ikili::IK.yavas);
        hp.ca = Some(c0 + jsign(dd) * jmin(dd.abs(), w));
        m.mac.fake = Some(f);
    }
    let tick = m.tick;
    let f = m.mac.fake.as_mut().unwrap();
    let ra = f.ra.clone();
    f.fooled.retain(|id| !(tick > ra.iter().find(|x| x.0 == *id).map_or(f64::NAN, |x| x.1)));
}

/// JS topaYigilma(m): pas giderken alıcının çevresindeki takım arkadaşları açılır.
pub const AYR: f64 = 7.0;
pub fn topaYigilma(m: &mut Mac) {
    if m.holder.is_some() {
        return;
    }
    let (Some(b), Some(fl)) = (m.ball.as_ref(), m.fl.as_ref()) else { return };
    let Some(P) = fl.q else { return };
    if dogru(b.defl) || b.defl_b || b.done.is_some() || fl.kind == "gönder" {
        return;
    }
    let t = b.team as f64;
    let (ax, ay) = (m.ps[P].tx, m.ps[P].ty);
    for pi in 0..m.ps.len() {
        let p = &m.ps[pi];
        if p.team != t || pi == P || p.bekci() || icer(job(p), &["alıcı", "kovala"]) {
            continue;
        }
        let dx = p.tx - ax;
        let dy = p.ty - ay;
        let d = hypot(dx, dy);
        if d >= AYR {
            continue;
        }
        let l0 = { let l = hypot(p.x - ax, p.y - ay); if dogru(l) { l } else { 1.0 } };
        let ux = if d > 0.1 { dx / d } else { (p.x - ax) / l0 };
        let uy = if d > 0.1 { dy / d } else { (p.y - ay) / l0 };
        let p = &mut m.ps[pi];
        p.tx = cl(ax + ux * AYR, 2.0, W - 2.0);
        p.ty = cl(ay + uy * AYR, 2.0, H - 2.0);
    }
}

/// JS korumaBaski(m): koruma süresindeki Bekçi'nin çevresine rakip giremez.
pub fn korumaBaski(m: &mut Mac) {
    let Some(h) = m.holder else { return };
    if !(m.ps[h].bekci() && gt(m.ps[h].hal._korT, m.tick)) {
        return;
    }
    let r = super::ikili::IK.kalkanR;
    let (hx, hy, ht) = (m.ps[h].x, m.ps[h].y, m.ps[h].team);
    for p in m.ps.iter_mut() {
        if p.team == ht {
            continue;
        }
        let dt = hypot(p.tx - hx, p.ty - hy);
        let dp = hypot(p.x - hx, p.y - hy);
        if dt >= r && dp >= r {
            continue;
        }
        let bx = if dt > 0.3 { p.tx } else { p.x };
        let by = if dt > 0.3 { p.ty } else { p.y };
        let dx = bx - hx;
        let dy = by - hy;
        let l = { let l = hypot(dx, dy); if dogru(l) { l } else { 1.0 } };
        p.tx = cl(hx + dx / l * r, 1.0, W - 1.0);
        p.ty = cl(hy + dy / l * r, 1.0, H - 1.0);
        p.hal_mut().hamle = None;
    }
}

/// JS outOfKuyu: Kuyu açıkken bir şey yapmaz (eski kale çemberi).
pub fn outOfKuyu(_p: &mut Oyuncu, _vel: bool) {}

/// JS corePos(m): Çekirdeğin yeri (taşıyıcının önünde ya da boştaki top).
pub fn corePos(m: &Mac) -> Option<(f64, f64)> {
    match m.holder {
        None => m.ball.as_ref().map(|b| (b.x, b.y)),
        Some(h) => {
            let hp = &m.ps[h];
            let cs = dis::coreSide(m, h);
            let r = Q.body + oo(hp._dke);
            Some((hp.x + cs.0 * r, hp.y + cs.1 * r))
        }
    }
}

/// JS savurBasla(m, h, o, w0, sYon, uIn): vuruştan önce Çekirdeği gövdenin etrafında savur.
pub fn savurBasla(m: &mut Mac, h: Pid, mut o: Secenek, w0: f64, sYon: f64, uIn: Option<f64>) {
    let ca0 = m.ps[h].ca.unwrap_or_else(|| dis::caOf(m, h));
    let gon = o.kind == "gönder";
    let plase = gon && o.shot.as_deref() == Some("plase");
    let vE = dis::vurE(m, h, m.ch, gon);
    let la = o.launch.clone().unwrap_or_default();
    let Some(pl) = dis::savurPlan(m, h, (la.vx, la.vy), ca0, w0, sYon, plase, vE) else {
        super::kafa_bag::kafaNot(m, &o, "savurmanın yolu kapalı");
        m.decT = 1.0;
        return;
    };
    if pl.T < 1.0 {
        o.savW = Some(pl.wRel);
        o.savU = Some(pl.uSh);
        return super::vurus::launch(m, h, o);
    }
    let mut k = pl.k;
    if let Some(u) = uIn {
        let vO = hypot(la.vx, la.vy) * k;
        let mx = u * (1.0 - 0.35 * jmin(1.0, pl.A / PI)) + 0.4;
        if vO > mx {
            k *= mx / vO;
        }
    }
    if uIn.is_some() && k < pl.k - 1e-9 {
        let l2 = (la.vx * k, la.vy * k);
        if let Some(pl2) = dis::savurPlan(m, h, l2, ca0, w0, sYon, plase, vE) {
            let l = o.launch.get_or_insert_with(Default::default);
            if pl2.k < 1.0 {
                l.vx = l2.0 * pl2.k;
                l.vy = l2.1 * pl2.k;
            } else {
                l.vx = l2.0;
                l.vy = l2.1;
            }
            if pl2.T < 1.0 {
                o.savW = Some(pl2.wRel);
                o.savU = Some(pl2.uSh);
                return super::vurus::launch(m, h, o);
            }
            let tick = m.tick;
            let hp = &mut m.ps[h];
            hp.ca = Some(ca0);
            hp.hal_mut().savur = Some(JPSavur { t0: tick, o: Box::new(o), s: pl2.s, a: pl2.a, wMax: pl2.wMax, wR: pl2.wR, rem: pl2.A, w: pl2.w0, g: pl2.P, u: pl2.uSh, });
            hp.hal_mut().holdD = false;
            m.decT = 99.0;
            return;
        }
    }
    if k < 1.0 {
        let l = o.launch.get_or_insert_with(Default::default);
        l.vx = la.vx * k;
        l.vy = la.vy * k;
    }
    let tick = m.tick;
    let hp = &mut m.ps[h];
    hp.ca = Some(ca0);
    let wR = pl.wR * jmin(1.0, k / if dogru(pl.k) { pl.k } else { 1.0 });
    hp.hal_mut().savur = Some(JPSavur { t0: tick, o: Box::new(o), s: pl.s, a: pl.a, wMax: pl.wMax, wR, rem: pl.A, w: pl.w0, g: pl.P, u: pl.uSh, });
    hp.hal_mut().holdD = false;
    m.decT = 99.0;
}

/// JS savurAdim(m, h, ps): savurmanın bir tiki; bitince vurur (true).
pub fn savurAdim(m: &mut Mac, h: Pid) -> bool {
    let b = Q.body;
    let cr = TQ.min_d / 2.0 + 0.3;
    let (hx, hy, ht, hca) = (m.ps[h].x, m.ps[h].y, m.ps[h].team, m.ps[h].ca.unwrap_or(f64::NAN));
    let s = m.ps[h].hal_mut().savur.as_mut().unwrap();
    let geri = s.g > 1e-9;
    s.w = if geri { jmin(jmin(s.w + s.a, s.wMax), (2.0 * s.a * s.g).sqrt()) } else { jmin(jmin(s.w + s.a, s.wMax), (s.wR * s.wR + 2.0 * s.a * s.rem).sqrt()) };
    let st = jmin(s.w, if geri { s.g } else { s.rem });
    let ca1 = hca + (if geri { -s.s } else { s.s }) * st;
    let cx = hx + cos(ca1) * b;
    let cy = hy + sin(ca1) * b;
    let carp = m.ps.iter().find(|q| q.team != ht && !q.noTouch && hypot(cx - q.x, cy - q.y) < cr).map(|q| q.govde.name.clone());
    if let Some(ad) = carp {
        let o = m.ps[h].hal_mut().savur.take().unwrap().o;
        super::kafa_bag::kafaNot(m, &o, &format!("savururken {ad}'e çarptı"));
        m.decT = if m.mac._lite { 99.0 } else { 1.0 };
        return false;
    }
    m.ps[h].ca = Some(ca1);
    let s = m.ps[h].hal_mut().savur.as_mut().unwrap();
    if geri {
        s.g -= st;
        if s.g <= 1e-9 {
            s.g = 0.0;
            s.w = 0.0;
        }
        return false;
    }
    s.rem -= st;
    if s.rem > 1e-9 {
        return false;
    }
    let s = m.ps[h].hal_mut().savur.take().unwrap();
    let mut o = *s.o;
    o.savW = Some(s.w);
    o.savU = Some(s.u);
    super::vurus::launch(m, h, o);
    true
}

/// JS onHareket(m): alıcı top gelmeden tek dokunuşa hazırlanır (savurmayı önceden başlatır).
pub fn onHareket(m: &mut Mac) {
    if m.mac._lite || m.holder.is_some() {
        return;
    }
    let (Some(b), Some(fl)) = (m.ball.as_ref(), m.fl.as_ref()) else { return };
    let Some(q) = fl.q else { return };
    if b.done.is_some() || fl.kind == "gönder" || b.team as f64 != m.ps[q].team {
        return;
    }
    let bk = b.kimlik();
    if m.ps[q].hal._onH.as_ref().map_or(false, |h| h.b != bk) {
        m.ps[q].hal_mut()._onH = None;
    }
    let pre_ok = m.pre.as_ref().map_or(false, |p| p.b == bk && p.q == q) && m.ps[q].hal._rcvB == Some(bk);
    if m.ps[q].hal._onH.is_none() && pre_ok && otAkil(m, q, bk) {
        let pre = m.pre.as_ref().unwrap();
        let vB = pre.r.best.map_or(-1e9, |i| pre.r.opts[i].v.unwrap_or(f64::NAN));
        let o = pre.r.opts.iter().find(|x| x.v.is_some() && x.launch.is_some() && matches!(x.kind.as_str(), "pas" | "önüne" | "kenardan" | "gönder")).cloned();
        if let Some(o) = o {
            if o.v.unwrap() >= vB - dis::D::otMargin {
                let gon = o.kind == "gönder";
                let ch = m.ball.as_ref().unwrap().ch;
                let vE = dis::vurE(m, q, ch, gon);
                let l = o.launch.as_ref().unwrap();
                if let Some(pl) = dis::onPlan(m, q, (l.vx, l.vy), gon && o.shot.as_deref() == Some("plase"), vE) {
                    let tArr = m.ps[q].hal._rcvAbs.unwrap_or(f64::NAN);
                    m.ps[q].hal_mut()._onH = Some(JP_onH { b: bk, tArr, w: 0.0, ..pl });
                }
            }
        }
    }
    let tick = m.tick;
    if let Some(hh) = m.ps[q].hal_mut()._onH.as_mut() {
        if tick >= hh.tArr - hh.Tsw {
            hh.w = jmin(hh.wS, hh.w + hh.a);
        }
    }
}

/// JS kancaOf(m, p): gelen topun hızıyla dönüşün başlangıcı (tek dokunuş savurması).
#[derive(Clone, Copy, Debug)]
pub struct Kanca {
    pub w0: f64,
    pub s: f64,
    pub ca: f64,
    pub u: f64,
    pub on: bool,
}
pub fn kancaOf(m: &Mac, p: Pid) -> Option<Kanca> {
    let gb0 = m._gelenB.as_ref()?;
    let (vx, vy) = if super::gv_var(gb0) { (gb0.tt.t.gvx, gb0.tt.t.gvy) } else { (gb0.vx, gb0.vy) };
    let b = Q.body;
    let u = hypot(vx, vy);
    let ca = m.ps[p].ca.unwrap_or(0.0);
    let cr = cos(ca) * vy - sin(ca) * vx;
    if let Some(hh) = &m._onHk {
        let sIn = if cr >= 0.0 { 1.0 } else { -1.0 };
        return Some(Kanca { w0: jmax(hh.w, if sIn == hh.s { u / b } else { 0.0 }), s: hh.s, ca, u, on: true });
    }
    Some(Kanca { w0: u / b, s: if cr >= 0.0 { 1.0 } else { -1.0 }, ca, u, on: false })
}

/// JS move(m): tikin hareketi (sprint kararı, evreler, Kuyu, istenen hız, kaçınma, ivme, gövde, omuz, enerji).
pub fn r#move(m: &mut Mac) {
    let mt = m.tick;
    let mH = m.holder;
    sprintKarar3(m);
    sprFaz(m);
    super::kuyu::kuyuSaat(m, mt, mH);
    korumaBaski(m);
    if m.ball.as_ref().map_or(false, |b| dogru(b.defl) || b.defl_b) {
        if let Some(k) = m.mac._kafa.as_mut() {
            if let Some(p) = k.P.as_mut() {
                p.r.sek = Some(1.0);
            }
        }
    }
    let kk = mac_kurali(m);
    let hizi = |m: &Mac, qi: Pid| -> f64 {
        let q = &m.ps[qi];
        spd(q) * vK(q, mH == Some(qi), mt, &kk) * (if m.holder == Some(qi) { topHiz(m, q) * if q.hal.savur.is_some() { 0.5 } else { 1.0 } } else { 1.0 }) * dengeHiz(m, qi)
    };
    let ist: Vec<(f64, f64, f64, f64)> = (0..m.ps.len()).map(|qi| {
        let q = &m.ps[qi];
        let dx = q.tx - q.x;
        let dy = q.ty - q.y;
        let l = hypot(dx, dy);
        let v = hizi(m, qi);
        let w = if l < 0.3 { 0.0 } else { jmin(v, 0.25 * l) };
        (q.x, q.y, if dogru(l) { dx / l * w } else { 0.0 }, if dogru(l) { dy / l * w } else { 0.0 })
    }).collect();
    for i in 0..m.ps.len() {
        let p = &m.ps[i];
        let dx = p.tx - p.x;
        let dy = p.ty - p.y;
        let l = hypot(dx, dy);
        let v = hizi(m, i);
        let want = if l < 0.3 { 0.0 } else { jmin(v, 0.25 * l) };
        let mut wx = if dogru(l) { dx / l * want } else { 0.0 };
        let mut wy = if dogru(l) { dy / l * want } else { 0.0 };
        if p.bekci() {
            let gx = if p.team == 0.0 { 0.0 } else { W };
            let rx = p.x - gx;
            let ry = p.y - H / 2.0;
            let rl = hypot(rx, ry);
            if rl > 0.5 {
                let ux = rx / rl;
                let uy = ry / rl;
                let wr = wx * ux + wy * uy;
                let tx = wx - wr * ux;
                let ty = wy - wr * uy;
                let tl = hypot(tx, ty);
                let kc = crate::temas::bek_kay(p) * v;
                if tl > kc {
                    wx = wr * ux + tx / tl * kc;
                    wy = wr * uy + ty / tl * kc;
                }
            }
        }
        if !p.bekci() && m.holder != Some(i) && want > 0.05 {
            let ux = wx / want;
            let uy = wy / want;
            let bd = TQ.min_d + 0.3;
            let al = m.fl.as_ref().and_then(|f| f.q);
            let T = 12.0;
            let tg = bd / want;
            let p0 = ist[i];
            let (mut kx, mut ky) = (0.0, 0.0);
            for (j, q) in m.ps.iter().enumerate() {
                if j == i || q.team != p.team || q.bekci() {
                    continue;
                }
                let q0 = ist[j];
                let rx = q0.0 - p0.0;
                let ry = q0.1 - p0.1;
                if rx > 6.0 || rx < -6.0 || ry > 6.0 || ry < -6.0 {
                    continue;
                }
                let pay = if m.holder == Some(j) || al == Some(j) { 1.0 } else if al == Some(i) { 0.0 } else { 0.5 };
                if pay == 0.0 {
                    continue;
                }
                let vx = q0.2 - wx;
                let vy = q0.3 - wy;
                let vv = vx * vx + vy * vy;
                let rv = rx * vx + ry * vy;
                let c2 = rx * rx + ry * ry - bd * bd;
                let tc;
                if c2 <= 0.0 {
                    if rv >= 0.0 && rx * ux + ry * uy <= 0.0 {
                        continue;
                    }
                    tc = 0.0;
                } else {
                    if rv >= 0.0 || vv < 1e-9 {
                        continue;
                    }
                    let disc = rv * rv - vv * c2;
                    if disc <= 0.0 {
                        continue;
                    }
                    tc = (-rv - disc.sqrt()) / vv;
                    if tc > T {
                        continue;
                    }
                }
                let yl = vy * ux - vx * uy;
                let cL = ry * ux - rx * uy + yl * (tc + tg);
                let yan = if cL >= 0.0 { -1.0 } else { 1.0 };
                let k = (1.0 - tc / (T + 1.0)) * pay * 1.6;
                kx -= uy * yan * k;
                ky += ux * yan * k;
            }
            if kx != 0.0 || ky != 0.0 {
                let nx = ux + kx;
                let ny = uy + ky;
                let nl = { let l = hypot(nx, ny); if dogru(l) { l } else { 1.0 } };
                wx = nx / nl * want;
                wy = ny / nl * want;
            }
        }
        if let (Some(b), Some(fl)) = (m.ball.as_ref(), m.fl.as_ref()) {
            if fl.q == Some(i) && m.holder.is_none() && !(dogru(b.defl) || b.defl_b) && b.done.is_none() && p.hal._rcvB == Some(b.kimlik()) && p.hal._rcvAbs.is_some() {
                let bs = hypot(b.vx, b.vy);
                if bs > 0.05 {
                    let ux = b.vx / bs;
                    let uy = b.vy / bs;
                    let vt = -(wx * ux + wy * uy);
                    let v0 = jmax(0.0, -(or0(p.vx) * ux + or0(p.vy) * uy));
                    let tc = jmax(0.0, hypot(b.x - p.x, b.y - p.y) - crate::temas::menzil(p, super::ikili::IK.yavas)) / jmax(0.05, bs + v0);
                    let cap = IN.v_set + 0.018 * ag(&p.govde.yet, p.den, p.bekci(), super::ikili::IK.yavas) * jmax(0.0, jmin(tc, p.hal._rcvAbs.unwrap() - m.tick) - 1.0);
                    if vt > cap {
                        wx += (vt - cap) * ux;
                        wy += (vt - cap) * uy;
                    }
                }
            }
        }
        let p = &mut m.ps[i];
        ivme(p, wx, wy, v);
        p.x = cl(p.x + p.vx, 0.8, W - 0.8);
        p.y = cl(p.y + p.vy, 0.8, H - 0.8);
    }
    topGovdeIt(m);
    if let Some(h) = m.holder {
        if let Some(ca) = m.ps[h].ca {
            let b = Q.body;
            let cr = TQ.min_d / 2.0 + 0.3;
            let (hx, hy, ht) = (m.ps[h].x, m.ps[h].y, m.ps[h].team);
            let cx = hx + cos(ca) * b;
            let cy = hy + sin(ca) * b;
            for q in m.ps.iter_mut() {
                if q.team == ht {
                    continue;
                }
                let dx = q.x - cx;
                let dy = q.y - cy;
                let dd = hypot(dx, dy);
                if dd >= cr {
                    continue;
                }
                let ux = if dd > 0.001 { dx / dd } else { cos(ca) };
                let uy = if dd > 0.001 { dy / dd } else { sin(ca) };
                q.x = cx + ux * cr;
                q.y = cy + uy * cr;
                let vn = or0(q.vx) * ux + or0(q.vy) * uy;
                if vn < 0.0 {
                    q.vx -= vn * ux;
                    q.vy -= vn * uy;
                }
            }
        }
    }
    if let Some(h) = m.holder {
        if m.ps[h].ca.is_some() && m.ps[h].hal.savur.is_none() {
            let b = Q.body;
            let mr = 0.46;
            let hp = &mut m.ps[h];
            let (hx, hy) = (hp.x, hp.y);
            let ic = |ca: f64, r: f64| {
                let x = hx + cos(ca) * r;
                let y = hy + sin(ca) * r;
                x >= mr && x <= W - mr && y >= mr && y <= H - mr
            };
            let mut e = oo(hp._dke);
            while e > 0.0 && !ic(hp.ca.unwrap(), b + e) {
                e = jmax(0.0, e - 0.05);
            }
            hp._dke = Some(e);
            let ca = hp.ca.unwrap();
            if !ic(ca, b + e) {
                for k in 1..=63 {
                    let a1 = ca + 0.05 * k as f64;
                    let a2 = ca - 0.05 * k as f64;
                    if ic(a1, b + e) {
                        hp.ca = Some(a1);
                        break;
                    }
                    if ic(a2, b + e) {
                        hp.ca = Some(a2);
                        break;
                    }
                }
            }
        }
    }
    if ikOn() {
        let bd = TQ.min_d;
        let n = m.ps.len();
        for i in 0..n {
            for j in i + 1..n {
                let (a, b) = (&m.ps[i], &m.ps[j]);
                if a.team == b.team || a.bekci() || b.bekci() || a.noTouch || b.noTouch {
                    continue;
                }
                let va = hypot(or0(a.vx), or0(a.vy));
                let vb = hypot(or0(b.vx), or0(b.vy));
                if va < 0.12 || vb < 0.12 {
                    continue;
                }
                let (uax, uay, ubx, uby) = (a.vx / va, a.vy / va, b.vx / vb, b.vy / vb);
                if uax * ubx + uay * uby < 0.75 {
                    continue;
                }
                let mut ux = uax + ubx;
                let mut uy = uay + uby;
                let ul = { let l = hypot(ux, uy); if dogru(l) { l } else { 1.0 } };
                ux /= ul;
                uy /= ul;
                let dx = b.x - a.x;
                let dy = b.y - a.y;
                let lon = dx * ux + dy * uy;
                let lat = -dx * uy + dy * ux;
                if lon.abs() > 1.0 || lat.abs() > bd + 0.7 || lat.abs() < 0.6 * bd {
                    continue;
                }
                let nx = -uy * jsign(lat);
                let ny = ux * jsign(lat);
                let gA = kutle(a, m.holder == Some(i)) * omuzK(a);
                let gB = kutle(b, m.holder == Some(j)) * omuzK(b);
                let tick = m.tick;
                let (a, b) = iki(&mut m.m.ps, i, j);
                if gA >= 0.95 * gB {
                    a.vx += 0.03 * nx;
                    a.vy += 0.03 * ny;
                    a.hal_mut()._omuzT = Some(tick);
                }
                if gB >= 0.95 * gA {
                    b.vx -= 0.03 * nx;
                    b.vy -= 0.03 * ny;
                    b.hal_mut()._omuzT = Some(tick);
                }
                if !m.mac._lite && lat.abs() < bd + 0.05 && (gA - gB).abs() > 0.05 * jmax(gA, gB) && oo(a.hal._omEv) < tick && oo(b.hal._omEv) < tick {
                    a.hal_mut()._omEv = Some(tick + 80.0);
                    b.hal_mut()._omEv = Some(tick + 80.0);
                    let (g, z) = if gA > gB { (i, j) } else { (j, i) };
                    let (gt_, gn, zn) = (m.ps[g].team, m.ps[g].govde.name.clone(), m.ps[z].govde.name.clone());
                    ev(m, "pas", gt_, format!("{gn} omuz omuza {zn}'i yolundan itti"));
                }
            }
        }
    }
    {
        let lite = m.mac._lite;
        let h = m.holder;
        let mut bag = IkBag { ikt: None, ikm: Some(IkHedef { st: &mut m.m.st, lite }), ikl: None };
        govdeAyir(&mut m.m.ps, Some(h), &mut bag);
    }
    super::ikili::denge(m);
    topGovdeIt(m);
    enerji(m);
}

/// Boştaki topun oyunculara çarpıp itilmesi (JS K.govdeIt bloğu).
fn topGovdeIt(m: &mut Mac) {
    if m.holder.is_some() {
        return;
    }
    let Some(b) = m.m.ball.as_mut() else { return };
    if b.done.is_some() {
        return;
    }
    let from = b.tt.t.from;
    let bt = b.t;
    let ps = &m.m.ps;
    let skip = |pi: usize| {
        let p = &ps[pi];
        (Some(pi) == from && or0(bt) < 4.0) || (p.bekci() && hypot(p.x - if p.team == 0.0 { 0.0 } else { W }, p.y - CY) < KR)
    };
    crate::temas::govde_it(&mut b.tt.b, ps, &skip, super::vurus::BODY_R);
}

#[allow(dead_code)]
fn _kullan() {
    let _ = tlv;
}
