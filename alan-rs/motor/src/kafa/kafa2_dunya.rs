//! Hayali dünyalar (decide.js 4717–5064, 5796–5804, 6779–6810, 6884–6931): holdWorld, bekSav, bekKay,
//! pwKos, predictWorld, bekYerW, geriCekil, paylastir; ve `p.markRef` / `p.tutQ` gibi oyuncu
//! bağlarının dünyada çözülmesi.
#![allow(non_snake_case, clippy::too_many_arguments)]

use super::kafa2_bag as dis;
use super::kafa2_bag::{ALAN_HZ};
use super::kafa2_baglam::*;
use crate::mat::{hypot, jmax, jmin, jround};
use crate::model::Oyuncu;
use crate::saha::{CY as GCY, H as GH, W as GW};
use std::sync::Arc;

/// JS `p.markRef` / `p.tutQ` (gerçek oyuncu nesnesi): maçın oyuncu dizisindeki sıra. Maç görünümü
/// yoksa kararın dünyası.
pub fn ref_nesne(k: &Kafa, src: &[Op], pid: Pid) -> Op {
    match &k.mac {
        Some(m) if pid < m.ps.len() => m.ps[pid].clone(),
        _ => src[pid.min(src.len() - 1)].clone(),
    }
}
/// JS `src.find(z => z.id === r.id && z.team === r.team) || r`
pub fn ref_bul(k: &Kafa, src: &[Op], pid: Pid) -> Op {
    let r = ref_nesne(k, src, pid);
    match bul_i(src, r.id, r.team) {
        Some(i) => src[i].clone(),
        None => r,
    }
}
/// JS `src.find(z => z.id === r.id && z.team === r.team)` (bulunmazsa None)
pub fn ref_bul0(k: &Kafa, src: &[Op], pid: Pid) -> Option<Op> {
    let r = ref_nesne(k, src, pid);
    bul_i(src, r.id, r.team).map(|i| src[i].clone())
}

/// JS bekKay(p, B, T) (decide.js:4824): Bekçi'nin topa göre yeri, T + sutHazir tikte varabildiği kadar.
pub fn bekKay(k: &Kafa, p: &Op, B: Nokta, T: f64) -> Op {
    if p.noTouch {
        return p.clone();
    }
    let g = dis::AS_bekci(k, p, B, None);
    let dx = g.x - p.x;
    let dy = g.y - p.y;
    let L = hypot(dx, dy);
    if L < 0.05 {
        return p.clone();
    }
    let mv = jmin(L, (T + k.D.sutHazir) * (0.17 + 0.006 * ALAN_HZ(p)) * dis::ALAN_SPRE(k, p) * crate::temas::bek_kay(p));
    if mv > 0.0 {
        kxy(p, p.x + dx / L * mv, p.y + dy / L * mv)
    } else {
        p.clone()
    }
}

/// JS bekSav(p, h, T, src) (decide.js:4789): Bekçi T tik boyunca topu taşıyanın (h, hızıyla ilerler)
/// karşısına kayar (kale çevresinde yanal hız sınırıyla).
pub fn bekSav(k: &Kafa, p: &Op, h: &Oyuncu, T: f64, src: &[Op]) -> Op {
    if p.noTouch {
        return p.clone();
    }
    let mut q = kv(p);
    let gx = if t0(p.team) { 0.0 } else { GW };
    let kc0 = crate::temas::bek_kay(p);
    let v0 = dis::AM_spd(k, p);
    let sK = dis::AM_sprK(k, p);
    let mut hv = h.clone();
    let mut t = 0usize;
    while (t as f64) < T {
        let tf = t as f64;
        hv.x = h.x + 0.5 * veya(h.vx, 0.0) * tf;
        hv.y = h.y + 0.5 * veya(h.vy, 0.0) * tf;
        let g = {
            let fm = dis::Fm { tac: &[], ps: src, holder: Some(&hv) };
            dis::AS_bekci(k, p, nk(hv.x, hv.y), Some(&fm))
        };
        let dx = g.x - q.x;
        let dy = g.y - q.y;
        let L = hypot(dx, dy);
        let v = v0 * if L > 1.2 { sK } else { 1.0 };
        let want = if L < 0.3 { 0.0 } else { jmin(v, 0.25 * L) };
        let mut wx = if L != 0.0 { dx / L * want } else { 0.0 };
        let mut wy = if L != 0.0 { dy / L * want } else { 0.0 };
        {
            let rx = q.x - gx;
            let ry = q.y - GCY;
            let rl = hypot(rx, ry);
            if rl > 0.5 {
                let ux = rx / rl;
                let uy = ry / rl;
                let wr = wx * ux + wy * uy;
                let tx = wx - wr * ux;
                let ty = wy - wr * uy;
                let tl = hypot(tx, ty);
                let kc = kc0 * v;
                if tl > kc {
                    wx = wr * ux + tx / tl * kc;
                    wy = wr * uy + ty / tl * kc;
                }
            }
        }
        dis::AM_ivme(k, &mut q, wx, wy, v);
        q.x = jmax(0.8, jmin(GW - 0.8, q.x + q.vx));
        q.y = jmax(0.8, jmin(GH - 0.8, q.y + q.vy));
        t += 1;
    }
    Arc::new(q)
}

/// Bir koşunun bir tiki (pwKos ve predictWorld'ün ortak döngüsü).
fn kos_adim(k: &Kafa, q: &mut Oyuncu, t: usize, gx: &mut f64, gy: &mut f64, sw: &mut Option<Sw>, vmax: f64, rc: f64) {
    if let Some(s) = *sw {
        let gec = if s.yer { hypot(*gx - q.x, *gy - q.y) < 1.2 } else { (t as f64) >= s.t.unwrap_or(f64::NAN) };
        if gec {
            *gx = s.x;
            *gy = s.y;
            *sw = None;
        }
    }
    let mut wx = q.vx;
    let mut wy = q.vy;
    if (t as f64) >= rc {
        let ex = *gx - q.x;
        let ey = *gy - q.y;
        let e = hypot(ex, ey);
        if e < 0.05 {
            wx = 0.0;
            wy = 0.0;
        } else {
            let v = jmin(vmax, 0.5 * e);
            wx = ex / e * v;
            wy = ey / e * v;
        }
    }
    dis::AM_ivme(k, q, wx, wy, vmax);
    q.x += q.vx;
    q.y += q.vy;
}

/// JS pwKos(p, gx0, gy0, sw0, vmax, rc, n, IV) (decide.js:4838): oyuncunun hedefe koşusu, n tik
/// (tik başına önbellekli: aynı oyuncu nesnesi ve aynı koşu → kaldığı yerden sürer).
pub fn pwKos(k: &mut Kafa, p: &Op, gx0: f64, gy0: f64, sw0: Option<Sw>, vmax: f64, rc: f64, n: usize) -> Op {
    if k.k2.PWT != k.bag._tick {
        k.k2.PWM.clear();
        k.k2.PWT = k.bag._tick;
    }
    let anahtar = Arc::as_ptr(p) as usize;
    let mut L = match k.k2.PWM.remove(&anahtar) {
        Some((_, L)) => L,
        None => Vec::new(),
    };
    let ei = L.iter().position(|z| {
        z.x0 == p.x
            && z.y0 == p.y
            && z.vx0 == p.vx
            && z.vy0 == p.vy
            && z.den == p.den
            && z.gx0 == gx0
            && z.gy0 == gy0
            && z.vmax == vmax
            && z.rc == rc
            && match (&sw0, &z.sw0) {
                (Some(a), Some(b)) => b.yer == a.yer && b.t == a.t && b.x == a.x && b.y == a.y,
                (Some(_), None) => false,
                (None, b) => b.is_none(),
            }
    });
    let ei = match ei {
        Some(i) => i,
        None => {
            let q = kv(p);
            let S = vec![q.x, q.y, q.vx, q.vy];
            L.push(PwE { x0: p.x, y0: p.y, vx0: p.vx, vy0: p.vy, den: p.den, gx0, gy0, vmax, rc, sw0, q, t: 0, gx: gx0, gy: gy0, sw: sw0, S });
            L.len() - 1
        }
    };
    {
        let e = &mut L[ei];
        if e.t < n {
            let (mut gx, mut gy, mut sw) = (e.gx, e.gy, e.sw);
            for t in e.t..n {
                kos_adim(k, &mut e.q, t, &mut gx, &mut gy, &mut sw, vmax, rc);
                e.S.push(e.q.x);
                e.S.push(e.q.y);
                e.S.push(e.q.vx);
                e.S.push(e.q.vy);
            }
            e.t = n;
            e.gx = gx;
            e.gy = gy;
            e.sw = sw;
        }
    }
    let e = &L[ei];
    let mut r = kv(p);
    let i = 4 * n;
    r.x = e.S[i];
    r.y = e.S[i + 1];
    r.vx = e.S[i + 2];
    r.vy = e.S[i + 3];
    k.k2.PWM.insert(anahtar, (p.clone(), L));
    Arc::new(r)
}

/// JS adamMi (predictWorld): adam adama savunan
fn adamMi(k: &Kafa, p: &Oyuncu) -> bool {
    p.markRef.is_some() && dis::AM_tl(k, p, "sav", "adam") >= 0.5
}

/// JS predictWorld(src, team, B, T, cevapAyri, resp, pasta, tas) (decide.js:4920): top B'deyken T tik
/// sonra herkes nerede. `resp`: cevap veren rakipler (sıra; verilmezse topa en yakın iki rakip),
/// `pasta`: pres işindekiler topa koşar, `tas`: topu taşıyan (pres işindekiler onu izler).
pub fn predictWorld(k: &mut Kafa, src: &[Op], team: f64, B: Nokta, T: f64, cevapAyri: bool, resp: Option<&[usize]>, pasta: bool, tas: Option<Nokta>) -> Dunya {
    let n_src = src.len();
    // near: topa en yakın iki rakip (Bekçi değil), kararlı sıralama
    let near: Vec<usize> = if cevapAyri {
        Vec::new()
    } else {
        let mut v: Vec<(usize, f64)> = (0..n_src).filter(|&i| src[i].team != team && !bekci(&src[i])).map(|i| (i, uz(src[i].x, src[i].y, B.x, B.y))).collect();
        v.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        v.into_iter().take(2).map(|x| x.0).collect()
    };
    let near_has = |i: usize| near.contains(&i);
    // devir: adam adama savunanların bırakılan adamı devralması
    let mut devir: Vec<Option<Op>> = vec![None; n_src];
    {
        let basan: Vec<usize> = match resp {
            Some(r) => r.to_vec(),
            None => near.clone(),
        };
        let mut tq: Option<usize> = None;
        let mut td = f64::INFINITY;
        for i in 0..n_src {
            let q = &src[i];
            if q.team == team && !bekci(q) {
                let d = uz(q.x, q.y, B.x, B.y);
                if tq.is_none() || d < td {
                    td = d;
                    tq = Some(i);
                }
            }
        }
        let tid = match tq {
            Some(i) if uz(src[i].x, src[i].y, B.x, B.y) < 2.5 => Some(src[i].id),
            _ => None,
        };
        let mr_id = |k: &Kafa, p: &Oyuncu| ref_nesne(k, src, p.markRef.unwrap()).id;
        let acik: Vec<Op> = basan.iter().filter(|&&i| adamMi(k, &src[i]) && Some(mr_id(k, &src[i])) != tid).map(|&i| ref_bul(k, src, src[i].markRef.unwrap())).collect();
        let mut alinan: Vec<Op> = Vec::new();
        if !acik.is_empty() && tid.is_some() {
            for fi in 0..n_src {
                let f = &src[fi];
                if f.team == team || bekci(f) || basan.contains(&fi) || !adamMi(k, f) || Some(mr_id(k, f)) != tid {
                    continue;
                }
                let mut bq: Option<&Op> = None;
                let mut bd = 1e9;
                for q in &acik {
                    if alinan.iter().any(|a| ayni_nesne(a, q)) {
                        continue;
                    }
                    let d = uz(q.x, q.y, f.x, f.y);
                    if d < bd {
                        bd = d;
                        bq = Some(q);
                    }
                }
                if let Some(bq) = bq {
                    alinan.push(bq.clone());
                    devir[fi] = Some(bq.clone());
                }
            }
        }
    }
    let mac_var = k.mac.is_some();
    let mut out: Dunya = Vec::with_capacity(n_src);
    for i in 0..n_src {
        let p = &src[i];
        if bekci(p) {
            out.push(if p.noTouch || (p.x == B.x && p.y == B.y) { p.clone() } else { bekKay(k, p, B, T) });
            continue;
        }
        let sp = 0.17 + 0.006 * ALAN_HZ(p);
        let (mut gx, mut gy);
        let mut sw: Option<Sw> = None;
        let pm;
        if p.team != team {
            let iq: Option<Op> = match p.tutQ {
                Some(tq) if !near_has(i) && devir[i].is_none() => {
                    let tqr = ref_nesne(k, src, tq);
                    let job = p.job.as_deref().unwrap_or("");
                    let job_pres = p.job.as_deref().unwrap_or("undefined").starts_with("pres");
                    if job.contains(&*tqr.name) && !job_pres {
                        ref_bul0(k, src, tq)
                    } else {
                        None
                    }
                }
                _ => None,
            };
            if iq.is_some() || near_has(i) || devir[i].is_some() || (pasta && pres_mi(&p.job)) {
                if let Some(iq) = iq {
                    let bu = iq.x == B.x && iq.y == B.y;
                    let qx = if bu { B.x } else { iq.tx };
                    let qy = if bu { B.y } else { iq.ty };
                    gx = qx + (p.tx - iq.x);
                    gy = qy + (p.ty - iq.y);
                    pm = false;
                } else if near_has(i) {
                    let gxk = if t0(p.team) { 0.0 } else { GW };
                    let dx = gxk - B.x;
                    let dy = GCY - B.y;
                    let L = veya(hypot(dx, dy), 1.0);
                    gx = B.x + dx / L * 2.0;
                    gy = B.y + dy / L * 2.0;
                    pm = false;
                } else if mac_var {
                    let gxk = if t0(p.team) { 0.0 } else { GW };
                    let mr: Option<Op> = match &devir[i] {
                        Some(d) => Some(d.clone()),
                        None => {
                            if p.markRef.is_some() && dis::AM_tl(k, p, "sav", "adam") >= 0.5 {
                                Some(ref_bul(k, src, p.markRef.unwrap()))
                            } else {
                                None
                            }
                        }
                    };
                    if let Some(mr) = mr {
                        pm = devir[i].is_none();
                        let mx = mr.tx;
                        let my = mr.ty;
                        let L = veya(hypot(gxk - mx, GCY - my), 1.0);
                        gx = mx + (gxk - mx) / L * 1.6;
                        gy = my + (GCY - my) / L * 1.6;
                        if let (Some(kv_), Some(otx)) = (&mr._kv, mr.otx) {
                            let oty = mr.oty.unwrap_or(f64::NAN);
                            let L2 = veya(hypot(gxk - otx, GCY - oty), 1.0);
                            let t1 = hypot(kv_.x - mr.x, kv_.y - mr.y) / ((0.17 + 0.006 * ALAN_HZ(&mr)) * dis::ALAN_SPRE(k, &mr));
                            let lag = jmax(1.0, jround(dis::ALAN_TEPKI(k, p, "algi"))) * 0.5;
                            sw = Some(Sw { yer: false, t: Some(t1 + lag), x: otx + (gxk - otx) / L2 * 1.6, y: oty + (GCY - oty) / L2 * 1.6 });
                        }
                    } else {
                        let m = k.mac.as_ref().unwrap();
                        let hold = m.holder.map(|h| m.ps[h].clone());
                        let fm = dis::Fm { tac: &m.tac, ps: &m.ps, holder: hold.as_deref() };
                        let sb = dis::AS_slotBase(k, &fm, p, B, false);
                        gx = sb.x;
                        gy = sb.y;
                        pm = false;
                    }
                } else {
                    let gxk = if t0(p.team) { 0.0 } else { GW };
                    let geride = (p.x - gxk).abs() > (B.x - gxk).abs() - 2.0;
                    if geride {
                        gx = B.x + 0.45 * (gxk - B.x);
                        gy = B.y + 0.45 * (GCY - B.y);
                    } else {
                        gx = p.x + 0.25 * (B.x - p.x);
                        gy = p.y + 0.5 * (B.y - p.y);
                    }
                    pm = false;
                }
            } else {
                gx = p.tx;
                gy = p.ty;
                pm = false;
                if let Some(tas) = tas {
                    if pres_mi(&p.job) {
                        gx = B.x + (p.tx - tas.x);
                        gy = B.y + (p.ty - tas.y);
                    }
                }
            }
        } else {
            if p.x == B.x && p.y == B.y {
                out.push(p.clone());
                continue;
            }
            gx = p.tx;
            gy = p.ty;
            pm = true;
            if let (Some(_), Some(otx)) = (&p._kv, p.otx) {
                sw = Some(Sw { yer: true, t: None, x: otx, y: p.oty.unwrap_or(f64::NAN) });
            }
        }
        let rk = p.team != team;
        let vmax = sp * if rk || p.sprint { dis::ALAN_SPR(k, p) } else { 1.0 };
        let rc = if rk { jmax(0.0, dis::ALAN_TEPKI(k, p, "kovala")) } else { 0.0 };
        let n = jmin(jround(T), 60.0).max(0.0) as usize;
        if pm {
            out.push(pwKos(k, p, gx, gy, sw, vmax, rc, n));
            continue;
        }
        let mut q = kv(p);
        for t in 0..n {
            kos_adim(k, &mut q, t, &mut gx, &mut gy, &mut sw, vmax, rc);
        }
        out.push(Arc::new(q));
    }
    out
}

/// JS bekYerW(w, team, B, T) (decide.js:5796): rakip Bekçi topa göre yerleşmiş dünya.
pub fn bekYerW(k: &Kafa, w: &[Op], team: f64, B: Nokta, T: Option<f64>) -> Dunya {
    let i = w.iter().position(|z| z.team != team && bekci(z) && !z.noTouch);
    let mut w2 = w.to_vec();
    if let Some(i) = i {
        let b2 = bekKay(k, &w[i], B, T.unwrap_or(30.0));
        if !ayni_nesne(&b2, &w[i]) {
            w2[i] = b2;
        }
    }
    w2
}

/// JS geriCekil(w, team, A, E, T, ilk) (decide.js:6779): taşıyıcı A'dan E'ye sürerken yakındaki
/// savunmacıların T tikteki cevabı (pres: önüne, arkadakiler: izler, ötekiler: E çevresine).
pub fn geriCekil(k: &Kafa, w: &[Op], team: f64, A: Nokta, E: Nokta, T: f64, ilk: bool) -> Dunya {
    let gx = if t0(team) { GW } else { 0.0 };
    let dA = hypot(gx - A.x, GCY - A.y);
    let mx = E.x - A.x;
    let my = E.y - A.y;
    w.iter()
        .map(|o| {
            if o.team == team || bekci(o) || o.noTouch {
                return o.clone();
            }
            let d = hypot(o.x - A.x, o.y - A.y);
            if d > 16.0 {
                return o.clone();
            }
            let run = jmax(0.0, T - if ilk { jmax(0.0, dis::ALAN_TEPKI(k, o, "kovala")) } else { 0.0 }) * (0.17 + 0.006 * ALAN_HZ(o)) * dis::ALAN_SPRE(k, o);
            let (tx, ty);
            if o.press {
                let gl = veya(hypot(gx - E.x, GCY - E.y), 1.0);
                tx = E.x + (gx - E.x) / gl * 1.6;
                ty = E.y + (GCY - E.y) / gl * 1.6;
            } else if hypot(gx - o.x, GCY - o.y) < dA && d < 10.0 && d > 2.4 + 1.5 {
                tx = o.x + mx;
                ty = o.y + my;
            } else {
                let ex = o.x - E.x;
                let ey = o.y - E.y;
                let el = veya(hypot(ex, ey), 1.0);
                tx = E.x + ex / el * 1.6;
                ty = E.y + ey / el * 1.6;
            }
            let dx = tx - o.x;
            let dy = ty - o.y;
            let L = hypot(dx, dy);
            if L < 0.05 {
                return o.clone();
            }
            let f = jmin(1.0, run / L);
            kxyv(o, o.x + dx * f, o.y + dy * f, 0.0, 0.0)
        })
        .collect()
}

/// JS paylastir(W, team, T, haric) (decide.js:6884; ALAN_PAYLASTIR kapalı): tehlikeli hücumcuları
/// boştaki savunmacılara paylaştırır.
pub fn paylastir(k: &Kafa, W: &[Op], team: f64, T: f64, haric: &[usize]) -> Dunya {
    let gxk = if t0(team) { GW } else { 0.0 };
    let mut at: Vec<(Op, f64)> = W.iter().filter(|p| p.team == team && !bekci(p) && !p.noTouch).map(|p| (p.clone(), dis::threat(k, team, p.x, p.y))).collect();
    at.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    if at.is_empty() {
        return W.to_vec();
    }
    let wMax = at[0].1;
    let dfs: Vec<usize> = (0..W.len()).filter(|&i| W[i].team != team && !bekci(&W[i]) && !W[i].noTouch && !haric.contains(&i)).collect();
    let mut kul: Vec<usize> = Vec::new();
    let mut W2 = W.to_vec();
    for (a, w) in &at {
        if *w < 0.3 * wMax {
            break;
        }
        let gl = veya(hypot(gxk - a.x, GCY - a.y), 1.0);
        let g = nk(a.x + (gxk - a.x) / gl * 1.6, a.y + (GCY - a.y) / gl * 1.6);
        let tutan = dfs.iter().find(|&&i| !kul.contains(&i) && hypot(W[i].x - g.x, W[i].y - g.y) < 3.0 && hypot(W[i].x - gxk, W[i].y - GCY) < gl);
        if let Some(&z) = tutan {
            kul.push(z);
            continue;
        }
        let mut bz: Option<usize> = None;
        let mut bt = 1e9;
        for &z in &dfs {
            if kul.contains(&z) {
                continue;
            }
            let p = &W[z];
            let t = jmax(0.0, dis::ALAN_TEPKI(k, p, "kovala")) + hypot(p.x - g.x, p.y - g.y) / ((0.17 + 0.006 * ALAN_HZ(p)) * dis::ALAN_SPRE(k, p));
            if t < bt {
                bt = t;
                bz = Some(z);
            }
        }
        let bz = match bz {
            Some(z) => z,
            None => break,
        };
        kul.push(bz);
        let d = &W[bz];
        let run = jmax(0.0, T - jmax(0.0, dis::ALAN_TEPKI(k, d, "kovala"))) * (0.17 + 0.006 * ALAN_HZ(d)) * dis::ALAN_SPRE(k, d);
        let dx = g.x - d.x;
        let dy = g.y - d.y;
        let L = hypot(dx, dy);
        W2[bz] = if L <= run { kxyv(d, g.x, g.y, 0.0, 0.0) } else { kxy(d, d.x + dx / L * run, d.y + dy / L * run) };
    }
    W2
}

/// JS holdWorld(h, src, ch, tac, T, rnd, to) (decide.js:4717; yalnız eski kafa): hayali maçta topu
/// T tik tut (ya da `to`'ya sür); kaybedildi mi.
pub fn holdWorld(k: &mut Kafa, hi: Pid, src: &[Op], ch: f64, tac: Option<&Tac>, T: f64, rnd: &mut Zar, to: Option<Nokta>) -> HoldW {
    if to.is_none() {
        if let Some((h, sp, cT, ct, cch, r)) = &k.k2.hold_c {
            if ayni_nesne(h, &src[hi]) && *sp == src.as_ptr() as usize && *cT == T && *ct == k.bag._tick && *cch == ch {
                return r.clone();
            }
        }
    }
    let tohum = (1e9 * rnd()).floor();
    let mut m = dis::AM_hayali(k, tohum, src, hi, ch, tac);
    let x0 = src[hi].x;
    let y0 = src[hi].y;
    let (i0, n0, t0_) = (k.k2._inLook, k.k2._noHold, k.bag._tick);
    k.k2._inLook = true;
    k.k2._noHold = true;
    let mut lost = false;
    let mut lostT = None;
    let mut t = 0usize;
    while (t as f64) < T {
        {
            let hc = Arc::make_mut(&mut m.ps[hi]);
            match to {
                Some(to) => {
                    hc.driveTo = Some(crate::model::JPDriveTo { x: to.x, y: to.y });
                    hc.tx = to.x;
                    hc.ty = to.y;
                    hc.drive = true;
                }
                None => {
                    hc.tx = x0;
                    hc.ty = y0;
                }
            }
        }
        dis::AM_step(k, &mut m);
        if m.holder != Some(hi) {
            lost = true;
            lostT = Some((t + 1) as f64);
            break;
        }
        t += 1;
    }
    k.k2._inLook = i0;
    k.k2._noHold = n0;
    k.bag._tick = t0_;
    let r = HoldW { lost, lostT, ps: Arc::new(m.ps), ch: m.ch };
    if to.is_none() {
        k.k2.hold_c = Some((src[hi].clone(), src.as_ptr() as usize, T, k.bag._tick, ch, r.clone()));
    }
    r
}
