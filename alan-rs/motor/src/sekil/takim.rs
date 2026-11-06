//! Takım tartıları: pas yolu (laneOpen, yolP, yerV), tehlike yönleri (tehlike), boşluğun takıma değeri
//! (teamV, teamBest, coverSpots, yolT, kurO), savunmanın hat kapatması (kapatHat, coverBoth), metrics.
#![allow(non_snake_case)]

use crate::mat::{atan2, cos, exp, hypot, jmax, jmin, sin};
use crate::model::{KafaD, Oyuncu, Pid};
use crate::oyuncu::Tepki;
use crate::temas::Kural;

use super::gecici_dis as dis;
use super::*;

/// JS laneOpen(w, att, A, B, v): A→B pas yolunun açıklığı (rakiplerin uzanmasıyla).
pub fn lane_open(kafa: dis::KafaEl, w: &[Varlik], oy: &[Oyuncu], att: u8, A: (f64, f64), B: (f64, f64), v: Option<f64>, k: &dyn Kural, d: &KafaD) -> f64 {
    let uc = v.map(|v| v == 0.0 || v != v).unwrap_or(true);
    let v = match v {
        Some(v) if v != 0.0 && v == v => v,
        _ => 1.0,
    };
    let L = {
        let l = hypot(B.0 - A.0, B.1 - A.1);
        if l != 0.0 && l == l {
            l
        } else {
            1.0
        }
    };
    let ux = (B.0 - A.0) / L;
    let uy = (B.1 - A.1) / L;
    let mut open = 1.0;
    for o in w {
        if o.team == att || o.bekci {
            continue;
        }
        let al = (o.x - A.0) * ux + (o.y - A.1) * uy;
        if al <= 0.0 || al >= L + 1.0 {
            continue;
        }
        let g = o.govde(oy);
        let pe = (-(o.x - A.0) * uy + (o.y - A.1) * ux).abs();
        let tb = if uc { dis::ucus_t(kafa, L, d.arrive, Some(al)) } else { al / v };
        let R = 1.6 + jmax(0.0, tb - TEPKI(g, Tepki::Kovala, k)) * spd(g) * alan_spre(g, k);
        open *= 1.0 - 1.0 / (1.0 + exp((pe - R) / 0.5));
    }
    open
}

/// JS yolP(m, w, att, A, B) (ALAN_PRENSIP açık, bekYol var): kafanın pas yolu × Bekçi'nin yolu.
pub fn yol_p(kafa: dis::KafaEl, w: &[Varlik], oy: &[Oyuncu], att: u8, A: (f64, f64), B: (f64, f64), d: &KafaD) -> f64 {
    dis::lane_ok(kafa, w, oy, att, A, B, d.arriveFirm, Some(d.sut_hazir)) * dis::bek_yol(kafa, w, oy, att, A, B, d.sut_hazir)
}
/// JS yerV(m, w, att, x, y, ch): yerin değeri (kafanın yerDeger'i).
pub fn yer_v(kafa: dis::KafaEl, w: &[Varlik], oy: &[Oyuncu], att: u8, x: f64, y: f64, ch: f64) -> f64 {
    dis::yer_deger(kafa, w, oy, att, x, y, ch)
}


/// Taşıyıcının tehlikeli bir yönü (JS tehlike öğesi).
#[derive(Clone, Debug)]
pub struct Tehlike {
    pub an: f64,
    pub w: f64,
    pub w0: f64,
    pub x: f64,
    pub y: f64,
    pub k: String,
    pub L: f64,
}

/// JS tehlike(m, h): kafanın son seçeneklerinden taşıyıcının tehlikeli yönleri (açıyla birleşmiş, payları).
pub fn tehlike(m: &crate::model::Mac, ps: &[Oyuncu], h: Option<Pid>, d: &KafaD) -> Option<Vec<Tehlike>> {
    let h = h?;
    let sec = m._sec.as_ref()?;
    if sec.h != h || m.tick - sec.t > 45.0 {
        return None;
    }
    let O = &sec.o;
    if O.is_empty() {
        return None;
    }
    let hp = &ps[h];
    let dk = |o: &crate::model::JmSecOe| if o.dv != 0.0 && o.dv == o.dv { d.surKarar } else { 0.0 };
    let mut vm = f64::NEG_INFINITY;
    for o in O {
        vm = jmax(vm, o.v + dk(o));
    }
    let mut out: Vec<Tehlike> = Vec::new();
    let mut top = 0.0;
    for o in O {
        let L = hypot(o.x - hp.x, o.y - hp.y);
        if L < 1.0 {
            continue;
        }
        let w = exp((o.v + dk(o) - vm) / 0.04) * jmax(0.0, o.v - jmax(0.0, sec.tut));
        if w <= 1e-4 {
            continue;
        }
        let an = atan2(o.y - hp.y, o.x - hp.x);
        match out.iter_mut().find(|u| atan2(sin(u.an - an), cos(u.an - an)).abs() < 0.45) {
            Some(f) => {
                if w > f.w0 {
                    f.an = an;
                    f.x = o.x;
                    f.y = o.y;
                    f.k = o.k.clone();
                    f.w0 = w;
                }
                f.w += w;
            }
            None => out.push(Tehlike { an, w, w0: w, x: o.x, y: o.y, k: o.k.clone(), L }),
        }
        top += w;
    }
    if out.is_empty() || top <= 0.0 {
        return None;
    }
    for u in out.iter_mut() {
        u.w /= top;
    }
    out.sort_by(|a, b| b.w.partial_cmp(&a.w).unwrap_or(std::cmp::Ordering::Equal));
    Some(out)
}

/// JS coverSpots(o, att, E, targets, R): savunucunun `R` içinde gidebileceği kapatma noktaları.
pub fn cover_spots(o: (f64, f64), att: u8, E: (f64, f64), targets: &[(f64, f64)], R: f64) -> Vec<(f64, f64)> {
    let mut out = vec![o];
    let A = gside(E.0, E.1, 1 - att, 2.2);
    for P in targets {
        let B = gside(P.0, P.1, 1 - att, 2.2);
        for t in [0.0, 0.5, 1.0] {
            let g = (A.0 + (B.0 - A.0) * t, A.1 + (B.1 - A.1) * t);
            let d = hypot(g.0 - o.0, g.1 - o.1);
            out.push(if d <= R { g } else { (o.0 + (g.0 - o.0) / d * R, o.1 + (g.1 - o.1) / d * R) });
        }
    }
    out
}

/// Ayrı kurulmuş koşu bedeni (JS `{a: p.a, x, y, vx, vy}`): yetenek p'nin, kimlik/bar yok.
pub fn govde_f(p: &Oyuncu, x: f64, y: f64, vx: f64, vy: f64) -> Oyuncu {
    let mut f = Oyuncu::bos();
    f.govde_mut().a = p.govde.a.clone();
    f.govde_mut().yet = p.govde.yet.clone();
    f.id = f64::NAN;
    f.x = x;
    f.y = y;
    f.vx = vx;
    f.vy = vy;
    f
}

/// JS yolT(p, c): gel-dön koşusu (önce c.kv'ye, sonra c'ye) süresi: (t1, t).
pub fn yol_t(p: &Oyuncu, c: (f64, f64), kv: (f64, f64), k: &dyn Kural) -> (f64, f64) {
    let v = v_sp(p, k);
    let d1 = hypot(kv.0 - p.x, kv.1 - p.y);
    let d2 = hypot(c.0 - kv.0, c.1 - kv.1);
    let ux = if d1 > 0.1 { (kv.0 - p.x) / d1 } else { 0.0 };
    let uy = if d1 > 0.1 { (kv.1 - p.y) / d1 } else { 0.0 };
    let t1 = gec(p, kv.0, kv.1, Some(d1), k) + d1 / v;
    let f = govde_f(p, kv.0, kv.1, ux * v * 0.8, uy * v * 0.8);
    (t1, t1 + gec(&f, c.0, c.1, Some(d2), k) + d2 / v)
}

/// JS kurO dönüşü: koşucuyu kovalayan markajcının yeri, varış bedeni ve başlangıcı.
pub struct KurO {
    pub x: f64,
    pub y: f64,
    /// None: bedeni hayaletin kendisi (JS `f: kd`)
    pub f: Option<Oyuncu>,
    pub t0: f64,
}

/// JS kurO(kd, att, p, c, yl).
pub fn kur_o(kd: &Varlik, oy: &[Oyuncu], att: u8, p: &Oyuncu, kv: Option<(f64, f64)>, yl: Option<(f64, f64)>, k: &dyn Kural) -> KurO {
    let (yl, kv) = match (yl, kv) {
        (Some(y), Some(kv)) => (y, kv),
        _ => return KurO { x: kd.x, y: kd.y, f: None, t0: 0.0 },
    };
    let g = gside(kv.0, kv.1, 1 - att, S.mark_off);
    let bx = kv.0 - p.x;
    let by = kv.1 - p.y;
    let bl = {
        let l = hypot(bx, by);
        if l != 0.0 && l == l {
            l
        } else {
            1.0
        }
    };
    let r = &oy[kd.r.unwrap()];
    let v = 0.8 * v_sp(r, k);
    KurO { x: g.0, y: g.1, f: Some(govde_f(r, g.0, g.1, bx / bl * v, by / bl * v)), t0: yl.0 }
}

/// teamV'nin aday noktası (JS `c`): konum, gel-dön ara noktası (kv), kovalayan hayalet (kd: dünya sırası).
#[derive(Clone, Copy, Debug)]
pub struct TvAday {
    pub x: f64,
    pub y: f64,
    pub kv: Option<(f64, f64)>,
    pub kd: Option<usize>,
    pub taban: bool,
}

/// JS iş adı `/^(markaj|arkada)/`.
fn markajci(job: &Option<std::sync::Arc<str>>) -> bool {
    match job {
        Some(j) => j.starts_with("markaj") || j.starts_with("arkada"),
        None => false,
    }
}

/// JS teamV(m, sn, att, E, c, mates, okP, p, pK): `p` c'ye koşarsa takımın kazancı; savunmanın en kötü
/// karşılığına göre (okuma `okP` ile karışır). `sn`: p'nin gördüğü dünya (geçici olarak oynatılır, geri konur).
pub fn team_v(c: &mut Mc, sn: &mut Vec<Varlik>, svi: Pid, att: u8, E: (f64, f64), cc: TvAday, mates: &[(f64, f64)], ok_p: f64, p: Option<Pid>, p_k: Option<Pid>) -> f64 {
    let p_k = p_k.or(p);
    let ch = if c.m.ch != 0.0 && c.m.ch == c.m.ch { c.m.ch } else { 0.3 };
    let k = c.k;
    let d = c.d;
    let w = cl((ok_p - 6.0) / 10.0, 0.0, 1.0);
    let mut crowd: f64 = 1.0;
    for q in mates {
        crowd = jmin(crowd, hypot(q.0 - cc.x, q.1 - cc.y) / S.ayr);
    }
    let sut = c.m.tac.get(att as usize).and_then(|t| t.sut).unwrap_or(0.5);
    // p'nin c'ye koşan kopyası teamV boyunca aynı (ps teamV içinde yazılmaz): bir kez kurulur
    let r_k = p.map(|pi| {
        let pp = &c.ps[pi];
        let k_l = hypot(cc.x - pp.x, cc.y - pp.y);
        let kos = k_l > S.kos_min;
        let kux = if kos { (cc.x - pp.x) / k_l } else { 0.0 };
        let kuy = if kos { (cc.y - pp.y) / k_l } else { 0.0 };
        let kva = if kos { spd(pp) * jmin(1.0, k_l / S.kos_hiz) } else { 0.0 };
        let mut r = pp.clone();
        r.x = cc.x;
        r.y = cc.y;
        r.vx = kux * kva;
        r.vy = kuy * kva;
        (r, kos, kux, kuy)
    });
    let my_opt = |c: &mut Mc, w2: &[Varlik]| -> f64 {
        // kpOf'tan önceki çağrılar aynı dünyayı okur: kafa çevirisi bir kez (ow_paylas); sıra JS'teki gibi
        let (vc, y_p, k_a, ro) = {
            let ps = &*c.ps;
            dis::ow_paylas(w2, ps, || {
                let vc = yer_v(c.kafa, w2, ps, att, cc.x, cc.y, dis::recv_ch(w2, ps, att, cc.x, cc.y, ch, p.map(|i| &ps[i])));
                match p {
                    None => (vc, yol_p(c.kafa, w2, ps, att, E, (cc.x, cc.y), d), 1.0, vc),
                    Some(_) => {
                        let (r, kos, kux, kuy) = r_k.as_ref().unwrap();
                        let (r, kos, kux, kuy) = (r, *kos, *kux, *kuy);
                        let sut_m = hypot(cc.x - opp_x(att), cc.y - GCY) < d.shot_range * (0.75 + 0.5 * sut);
                        let k_a = dis::keep_p(c.kafa, r, w2, ps, if kos && kux * dir_of(att) > -0.3 { Some((kux, kuy)) } else { None }, if sut_m { d.sut_hazir } else { d.recv_keep_t });
                        let y_p = yol_p(c.kafa, w2, ps, att, E, (cc.x, cc.y), d);
                        let ro = if d.use_space { jmax(vc, dis::run_on(c.kafa, w2, ps, att, r, (cc.x, cc.y))) } else { vc };
                        (vc, y_p, k_a, ro)
                    }
                }
            })
        };
        match p {
            None => crowd * (y_p * vc + kp_of(c, w2, svi, (cc.x, cc.y), p_k, y_p, 1.0)) * S.W.tv_pass,
            Some(pi) => {
                let kp = kp_of(c, w2, svi, (cc.x, cc.y), Some(pi), y_p, k_a);
                crowd * (y_p * k_a * ro + kp) * S.W.tv_pass
            }
        }
    };
    if w <= 0.0 {
        return my_opt(c, sn);
    }
    let ps0 = &*c.ps;
    let opp: Vec<usize> = (0..sn.len()).filter(|&i| sn[i].team != att && !sn[i].bekci).collect();
    let mut by_e = opp.clone();
    by_e.sort_by(|&u, &v| (hypot(sn[u].x - E.0, sn[u].y - E.1) - hypot(sn[v].x - E.0, sn[v].y - E.1)).partial_cmp(&0.0).unwrap_or(std::cmp::Ordering::Equal));
    let mut by_c = opp.clone();
    by_c.sort_by(|&u, &v| (hypot(sn[u].x - cc.x, sn[u].y - cc.y) - hypot(sn[v].x - cc.x, sn[v].y - cc.y)).partial_cmp(&0.0).unwrap_or(std::cmp::Ordering::Equal));
    let kd = if p.is_some() { cc.kd.filter(|&i| i < sn.len()) } else { None };
    let ref_is = |o: &Varlik, pi: Pid| -> bool { o.r.map(|r| ps0[r].hal.tutQ == Some(pi) && markajci(&ps0[r].job)).unwrap_or(false) };
    let mk = match (p, kd) {
        (Some(pi), None) => opp.iter().copied().find(|&i| ref_is(&sn[i], pi)),
        _ => None,
    };
    let mut rs: Vec<usize> = Vec::new();
    if let Some(&e0) = by_e.first() {
        rs.push(e0);
    }
    match kd {
        Some(kdi) => {
            if Some(&kdi) != by_e.first() {
                rs.push(kdi);
            } else if let Some(&o2) = by_c.iter().find(|&&o| o != kdi) {
                rs.push(o2);
            }
        }
        None => {
            if mk.is_some() && mk != by_e.first().copied() {
                rs.push(mk.unwrap());
            } else if let Some(&c0) = by_c.first() {
                if Some(&c0) != by_e.first() {
                    rs.push(c0);
                }
            }
        }
    }
    let yl = match (p, cc.kv) {
        (Some(pi), Some(kv)) => Some(yol_t(&ps0[pi], (cc.x, cc.y), kv, k)),
        _ => None,
    };
    let ta = match p {
        Some(pi) => {
            let pp = &ps0[pi];
            match yl {
                Some(y) => y.1,
                None => {
                    let dd = hypot(cc.x - pp.x, cc.y - pp.y);
                    gec(pp, cc.x, cc.y, Some(dd - 0.3), k) + jmax(0.0, dd - 0.3) / v_sp(pp, k)
                }
            }
        }
        None => 1e9,
    };
    let k_o = kd.map(|kdi| kur_o(&sn[kdi], ps0, att, &ps0[p.unwrap()], cc.kv, yl, k));
    let rof = |sn: &[Varlik], oi: usize| -> f64 {
        let o = &sn[oi];
        match p {
            Some(_) => {
                if Some(oi) == kd {
                    let k_o = k_o.as_ref().unwrap();
                    let r = &ps0[o.r.unwrap()];
                    let gf = match &k_o.f {
                        Some(f) => gec(f, cc.x, cc.y, None, k),
                        None => gec(o.r0(ps0), cc.x, cc.y, None, k),
                    };
                    jmax(0.0, ta - k_o.t0 - 0.5 * lag_of(r, k) - gf) * v_sp(r, k)
                } else {
                    let r0 = o.r0(ps0);
                    jmax(0.0, ta - tepki(r0, k) - gec(r0, cc.x, cc.y, None, k)) * v_sp(r0, k)
                }
            }
            None => S.W.dl_reach,
        }
    };
    let m_resp = |sn: &[Varlik], oi: usize| -> Option<Vec<(f64, f64)>> {
        let pi = p?;
        let o = &sn[oi];
        if !ref_is(o, pi) {
            return None;
        }
        let fr0 = if Some(oi) == kd { let k = k_o.as_ref().unwrap(); (k.x, k.y) } else { (o.x, o.y) };
        let g = gside(cc.x, cc.y, 1 - att, S.mark_off);
        let R0 = rof(sn, oi);
        let d0 = hypot(g.0 - fr0.0, g.1 - fr0.1);
        Some(vec![if d0 <= R0 { g } else { (fr0.0 + (g.0 - fr0.0) / d0 * R0, fr0.1 + (g.1 - fr0.1) / d0 * R0) }])
    };
    let choices: Vec<Vec<(usize, (f64, f64))>> = rs
        .iter()
        .map(|&oi| {
            let v = match m_resp(sn, oi) {
                Some(v) => v,
                None => {
                    let o0 = if Some(oi) == kd { let k = k_o.as_ref().unwrap(); (k.x, k.y) } else { (sn[oi].x, sn[oi].y) };
                    cover_spots(o0, att, E, &[(cc.x, cc.y)], rof(sn, oi))
                }
            };
            v.into_iter().map(|g| (oi, g)).collect()
        })
        .collect();
    let combos: Vec<Vec<(usize, (f64, f64))>> = if choices.len() == 1 {
        choices[0].iter().map(|k| vec![*k]).collect()
    } else if !choices.is_empty() {
        choices[0].iter().flat_map(|k| choices[1].iter().map(move |j| vec![*k, *j])).collect()
    } else {
        vec![vec![]]
    };
    let keep: Vec<(f64, f64)> = rs.iter().map(|&o| (sn[o].x, sn[o].y)).collect();
    let mut worst = 1e9;
    let H = c.m.holder.filter(|&h| c.ps[h].takim() == att);
    let h_r = H.map(|h| {
        let mut r = c.ps[h].clone();
        r.x = E.0;
        r.y = E.1;
        r
    });
    let h_opt = |c: &mut Mc, w2: &[Varlik]| -> f64 {
        let ps = &*c.ps;
        let h_r = h_r.as_ref().unwrap();
        dis::ow_paylas(w2, ps, || {
            let Vh = yer_v(c.kafa, w2, ps, att, E.0, E.1, ch);
            if d.recv_keep_t != 0.0 {
                dis::keep_p(c.kafa, h_r, w2, ps, None, d.recv_keep_t) * if d.use_space { jmax(Vh, dis::run_on(c.kafa, w2, ps, att, h_r, E)) } else { Vh }
            } else {
                d.recv_keep_t
            }
        })
    };
    let (mut h_base, mut my_w) = (1e9, 1e9);
    if let Some(kp) = c.ek.kp_c.as_mut() {
        kp.mut_ += 1;
    }
    for cmb in &combos {
        for (oi, g) in cmb {
            sn[*oi].x = g.0;
            sn[*oi].y = g.1;
        }
        if H.is_some() {
            let hv = h_opt(c, sn);
            let mv = my_opt(c, sn);
            h_base = jmin(h_base, hv);
            my_w = jmin(my_w, mv);
            worst = jmin(worst, jmax(mv, hv));
        } else {
            worst = jmin(worst, my_opt(c, sn));
        }
    }
    for (i, &o) in rs.iter().enumerate() {
        sn[o].x = keep[i].0;
        sn[o].y = keep[i].1;
    }
    if let Some(kp) = c.ek.kp_c.as_mut() {
        kp.mut_ -= 1;
    }
    if H.is_some() && cc.taban {
        return worst;
    }
    if let (Some(h), Some(pi), false) = (H, p, cc.taban) {
        let key = (c.m.tick.to_bits(), c.ps[pi].id.to_bits(), c.ps[h].id.to_bits());
        let v = match c.ek.tb_c {
            Some((kk, v)) if kk == key => v,
            _ => {
                let px = c.ps[pi].x;
                let py = c.ps[pi].y;
                let v = team_v(c, sn, svi, att, E, TvAday { x: px, y: py, kv: None, kd: None, taban: true }, mates, ok_p, p, p_k);
                c.ek.tb_c = Some((key, v));
                v
            }
        };
        worst = worst - v + if my_w < 1e9 { S.tv_egim * my_w } else { 0.0 };
    } else if H.is_some() {
        worst = jmax(0.0, worst - h_base) + if my_w < 1e9 { S.tv_egim * my_w } else { 0.0 };
    }
    let opened = if d.use_space && ok_p > 0.0 {
        let ps = &*c.ps;
        let vv = jmax(dis::space(c.kafa, sn, ps, att, cc.x, cc.y), dis::front(c.kafa, sn, ps, att, cc.x, cc.y));
        yol_p(c.kafa, sn, ps, att, E, (cc.x, cc.y), d) * dis::dagin(sn, ps, att, cc.x, vv, cc.y) * S.W.tv_pass
    } else {
        0.0
    };
    jmax((1.0 - w) * my_opt(c, sn) + w * worst, crowd * opened)
}

/// JS kpOf(w2, c, q, yP, kA) (teamV içi): koşunun topu kaybetme riskine katkısı (`_kpC` kuruluysa).
fn kp_of(c: &mut Mc, w2: &[Varlik], svi: Pid, cc: (f64, f64), q: Option<Pid>, y_p: f64, k_a: f64) -> f64 {
    let q = match q {
        Some(q) => q,
        None => return 0.0,
    };
    match &c.ek.kp_c {
        Some(kp) if kp.le > 0.0 => {}
        _ => return 0.0,
    }
    let k = c.k;
    let d = c.d;
    let qp = &c.ps[q];
    let k_l = hypot(cc.0 - qp.x, cc.1 - qp.y);
    let t_k = gec(qp, cc.0, cc.1, Some(k_l), k) + k_l / v_sp(qp, k);
    let R = d.recv_keep_t;
    let ps = &*c.ps;
    let kp = c.ek.kp_c.as_mut().unwrap();
    let mut k_t = |T: f64| -> f64 {
        if kp.mut_ != 0 {
            return cl(dis::keep_p(c.kafa, &kp.Hr, w2, ps, None, T), 0.0, 1.0);
        }
        let key = (svi, T.to_bits());
        if let Some(v) = kp.kc.get(&key) {
            return *v;
        }
        let v = cl(dis::keep_p(c.kafa, &kp.Hr, w2, ps, None, T), 0.0, 1.0);
        kp.kc.insert(key, v);
        v
    };
    let k1 = k_t(jmax(R, t_k));
    if k1 < 0.001 || k_t(100.0) > 0.98 {
        return 0.0;
    }
    let k2 = k_t(jmax(R, t_k) + R);
    kp.k_h = k1;
    let le = kp.le;
    let ust = (1.0 - k2) * le - ((1.0 - k1) * le + k1 * ((1.0 - y_p) * le));
    let kat = k1 * y_p * (1.0 - cl(k_a, 0.0, 1.0));
    if ust <= 0.0 {
        0.0
    } else if kat <= 0.0 {
        ust
    } else {
        jmax(0.0, (1.0 - k2) * le - ((1.0 - k1) * le + k1 * ((1.0 - y_p) * le + y_p * (1.0 - cl(k_a, 0.0, 1.0)) * (kp.L)(c.kafa, cc.0, cc.1))))
    }
}


/// JS kapatHat(m, d, att, E, qo): markajcının hattı: adamının kale tarafı, adamına ve arkasındaki koşuya pas
/// yolunu en çok kapatan noktaya (yerleşim yeteneğiyle) kayar. 4 tik önbellekli (`d._kh`).
pub fn kapat_hat(c: &mut Mc, di: Pid, att: u8, E: (f64, f64), qo: &Varlik) -> (f64, f64) {
    let ok_d = yer_s(&c.ps[di]);
    let w = cl((ok_d - 6.0) / 10.0, 0.0, 1.0);
    let g0 = gside(qo.x, qo.y, 1 - att, S.mark_off);
    if w <= 0.0 || c.m.holder.is_none() {
        return g0;
    }
    {
        let dp = &c.ps[di];
        if let (Some(kh), Some(t)) = (&dp.hal._kh, dp.hal._khT) {
            if t > c.m.tick - 4.0 && dp.hal._khQ == qo.r {
                return (qo.x + kh.dx, qo.y + kh.dy);
            }
        }
    }
    let sn0 = seen(c, di);
    let mut sn: Vec<Varlik> = (*sn0).clone();
    let ch = if c.m.ch != 0.0 && c.m.ch == c.m.ch { c.m.ch } else { 0.3 };
    let gx = own_x(1 - att);
    let gl = {
        let l = hypot(gx - qo.x, GCY - qo.y);
        if l != 0.0 && l == l {
            l
        } else {
            1.0
        }
    };
    let run = (qo.x + (gx - qo.x) / gl * 7.0, qo.y + (GCY - qo.y) / gl * 7.0);
    let T = [(qo.x, qo.y), run];
    let mut spots = vec![g0];
    let d_mx = 4.0 + 0.15 * gl;
    for t in [0.55, 0.72, 0.86] {
        let lx = E.0 + (qo.x - E.0) * t;
        let ly = E.1 + (qo.y - E.1) * t;
        for g in [(lx, ly), ((lx + g0.0) / 2.0, (ly + g0.1) / 2.0)] {
            if hypot(g.0 - qo.x, g.1 - qo.y) <= d_mx {
                spots.push(g);
            }
        }
    }
    let d = c.d;
    let (dx0, dy0) = (c.ps[di].x, c.ps[di].y);
    // JS hayalet {x, y, a, team, role, ref: d}: R/D/press yok (8, 1, false), id yok.
    let mut gb = hayal(&c.ps[di]);
    gb.R = 0.0;
    gb.D = 1.0;
    gb.press = false;
    let gb = std::sync::Arc::new(gb);
    sn.push(Varlik { x: dx0, y: dy0, vx: 0.0, vy: 0.0, gvx: None, gvy: None, team: c.ps[di].takim(), bekci: c.ps[di].bekci(), press: false, id: f64::NAN, r: Some(di), kim: None, beden: Beden::Ayri(gb) });
    let gi = sn.len() - 1;
    let mut best = g0;
    let mut bv = 1e9;
    let ps = &*c.ps;
    for g in &spots {
        sn[gi].x = g.0;
        sn[gi].y = g.1;
        let mut v: f64 = 0.0;
        for P in T {
            v = jmax(v, yol_p(c.kafa, &sn, ps, att, E, P, d) * yer_v(c.kafa, &sn, ps, att, P.0, P.1, dis::recv_ch(&sn, ps, att, P.0, P.1, ch, None)));
        }
        v += 0.004 * hypot(g.0 - dx0, g.1 - dy0);
        if v < bv {
            bv = v;
            best = *g;
        }
    }
    let r = (g0.0 + (best.0 - g0.0) * w, g0.1 + (best.1 - g0.1) * w);
    let dp = &mut c.ps[di];
    dp.hal_mut()._kh = Some(crate::model::JpKh { dx: r.0 - qo.x, dy: r.1 - qo.y });
    dp.hal_mut()._khT = Some(c.m.tick);
    dp.hal_mut()._khQ = qo.r;
    r
}

/// JS metrics(m) dönüşü.
#[derive(Clone, Copy, Debug)]
pub struct Metrik {
    pub swarm: u32,
}

/// JS metrics(m): topun 8 br yakınındaki (pres dışı) oyuncu sayısı.
pub fn metrics(c: &Mc) -> Metrik {
    let (bx, by) = match c.m.holder {
        Some(h) => (c.ps[h].x, c.ps[h].y),
        None => {
            let b = c.ball.as_ref().expect("metrics: top yok");
            (b.b.x, b.b.y)
        }
    };
    let mut n = 0;
    for (i, p) in c.ps.iter().enumerate() {
        if Some(i) != c.m.holder && !p.bekci() && !p.press && hypot(p.x - bx, p.y - by) < 8.0 {
            n += 1;
        }
    }
    Metrik { swarm: n }
}

#[allow(dead_code)]
fn _kullan(_: &Oyuncu) {}
