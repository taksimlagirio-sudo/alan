//! Topun uçuşunu okuma (JS flightRead, alici, aliciDirenc, kendiTopu, menzilP, menzilO).
#![allow(non_snake_case)]

use std::sync::Arc;

use crate::core_sabit::{IN, Q};
use crate::mat::{exp, hypot, jmax, jmin};
use crate::model::{Oyuncu, Pid, TopRef};
use crate::oyuncu::Tepki;
use crate::temas::{bek_re, kuyu_girer, menzil, Kural, TopT};
use crate::top::{tahmin, YolNokta};
use crate::yetenek::{ag, mek, zar as _zar};

use super::gecici_dis as dis;
use super::*;

/// Topun yolundaki bir nokta (JS `pts[i]`).
pub type Nokta = YolNokta;

/// Bir oyuncunun topa varışı (JS flightRead `arr` değeri; alici/aliciDirenc dönüşü).
#[derive(Clone, Copy, Debug, Default)]
pub struct Ar {
    pub t: f64,
    pub x: f64,
    pub y: f64,
    pub est: f64,
    pub fren: bool,
    pub direnc: bool,
}

/// JS flightRead dönüşü: herkesin varışı (oyuncu sırasıyla), ilk varan, topun yolu.
#[derive(Clone, Debug)]
pub struct Fr {
    pub arr: Vec<Ar>,
    pub first: Option<Pid>,
    pub pts: Arc<Vec<Nokta>>,
}

/// JS vurusOku/okuT herhangi bir beden için (hayalet de olabilir): rakibin vuruşu okuyup tepkisini
/// kısaltması. `ps`: topu vuranın (b.from) bulunduğu oyuncular.
pub fn okuT(b: Option<&mut TopT>, p: &Oyuncu, ps: &[Oyuncu], d: f64, bek0: bool) -> f64 {
    let b = match b {
        Some(b) => b,
        None => return d,
    };
    if b.t.kt.is_none() || b.b.defl != 0.0 || b.b.walls != 0.0 || p.takim() == b.b.team || p.bekci() {
        return d;
    }
    let bek = bek0 || b.t.k_bek;
    let a = 100.0 * p.team + p.id;
    // JS: anahtar String(ck); NaN (kimliksiz hayalet) tek anahtarda toplanır.
    let ck = if a != a { i64::MIN } else if bek { (-1.0 - a) as i64 } else { a as i64 };
    if let Some(r) = b.t.oku.get(&ck) {
        return d * r.0 - r.1;
    }
    let kk = b.t.kt.unwrap() + 1.618 * b.t.kz;
    let z = if bek { 0.0 } else { _zar(a, kk, 21.0) + _zar(a, kk, 23.0) - 1.0 };
    let mut on = 0.0;
    if let Some(fi) = b.t.from {
        if b.t.cue > 0.0 {
            let f = jmax(0.05, jmin(0.95, 0.5 + IN.sez * (p.yet.y(mek::VURUS_SEZ) - 10.0) - IN.giz * (ps[fi].yet.y(mek::VURUS_GIZLE) - 10.0)));
            on = b.t.cue * f * if bek { (IN.u_a + IN.u_b) / 2.0 } else { IN.u_a + (IN.u_b - IN.u_a) * _zar(a, kk, 29.0) };
        }
    }
    let r = (1.0 + IN.tepki_z * z, on);
    b.t.oku.insert(ck, r);
    d * r.0 - r.1
}

/// JS menzilP(p) (AlanCore.menzil).
#[inline]
pub fn menzilP(p: &Oyuncu, k: &dyn Kural) -> f64 {
    menzil(p, k.ik_yavas())
}

/// JS menzilO(p, tp, b, bek): rakibin uzanması topun yaşıyla büyür.
pub fn menzilO(p: &Oyuncu, tp: f64, b: Option<&mut TopT>, ps: &[Oyuncu], bek: bool, k: &dyn Kural) -> f64 {
    let o = okuT(b, p, ps, TEPKI(p, Tepki::Uzanma, k), bek);
    jmax(1.15, jmin(menzilP(p, k), Q.reach_body + jmax(0.0, tp - o) * Q.reach_grow))
}

/// JS inK: (Kuyu açık) bu tikte oraya giremez.
#[inline]
fn inK(p: &Oyuncu, q: &Nokta, i: f64, k: &dyn Kural) -> bool {
    !kuyu_girer(p, q.x, q.y, i, false, k)
}

/// JS aliciDirenc(pts, p, rc, iMin, rP): alıcı topa yetişemiyorsa en az geç kaldığı nokta.
pub fn aliciDirenc(pts: &[Nokta], p: &Oyuncu, rc: f64, iMin: f64, rP: Option<f64>, k: &dyn Kural) -> Option<Ar> {
    let nb = !p.bekci();
    let v = vSp(p, k);
    let G = gecTab(p, k);
    let rP = rP.unwrap_or(1.4);
    let mut bb: Option<(f64, f64, f64, f64, f64)> = None; // g, sk, t, x, y
    let mut i = jmax(0.0, iMin) as usize;
    let lim = jmin(pts.len() as f64, 60.0) as usize;
    // JS: i = max(0, iMin || 0) (kesirli olabilir); i < min(len, 60); i += 2
    let mut fi = jmax(0.0, if iMin == iMin { iMin } else { 0.0 });
    let _ = &mut i;
    while fi < lim as f64 {
        let ix = fi as usize;
        if fi.fract() != 0.0 {
            // JS pts[kesirli] → undefined: q.x hatası; gerçekte iMin tam sayıdır.
            break;
        }
        let q = &pts[ix];
        if nb && inK(p, q, fi, k) {
            fi += 2.0;
            continue;
        }
        let dr = hypot(q.x - p.x, q.y - p.y) - rP;
        let tr = jmax(rc + jmax(0.0, dr) / v + gecOf(&G, q.x, q.y, Some(dr)), dis::frenT(p, q.x, q.y, rP, fi - rc, v));
        let g = jmax(0.0, tr - fi);
        let sk = g + 0.25 * fi;
        if bb.is_none() || sk < bb.unwrap().1 - 1e-9 {
            bb = Some((g, sk, jmax(fi, tr.ceil()), q.x, q.y));
        }
        fi += 2.0;
    }
    bb.map(|b| Ar { t: b.2, x: b.3, y: b.4, est: 0.0, fren: false, direnc: true })
}

/// JS alici(pts, p, ps, rc, iMax, tp0, iMin, buf, b): alıcının topu en rahat alacağı nokta (rakipten önce
/// varır, en boşta). `w`: dünya (JS `ps`), `oy`: bedenlerin oyuncuları, `p`: alıcının bedeni.
pub fn alici(pts: &[Nokta], p: &Oyuncu, w: &[Varlik], oy: &[Oyuncu], rc: f64, iMax: f64, tp0: Option<f64>, iMin: f64, buf: Option<f64>, mut b: Option<&mut TopT>, k: &dyn Kural, d: &crate::model::KafaD) -> Option<Ar> {
    let _ = d;
    let nb = !p.bekci();
    let v = vSp(p, k);
    let opp: Vec<&Varlik> = w.iter().filter(|o| o.team != p.takim() && !o.bekci).collect();
    let oT: Vec<f64> = opp.iter().map(|o| { let g = o.govde(oy); jmax(0.0, okuT(b.as_deref_mut(), g, oy, TEPKI(g, Tepki::Kovala, k), true)) }).collect();
    let oV: Vec<f64> = opp.iter().map(|o| vSp(o.govde(oy), k)).collect();
    let (px, py) = (p.x, p.y);
    let gP = gecTab(p, k);
    let gO: Vec<GecTab> = opp.iter().map(|o| gecTab(o.r0(oy), k)).collect();
    let MP = tp0.is_some();
    let rP = menzilP(p, k);
    let mut yol = 1e9;
    let aB = 0.018 * ag(&p.yet, p.den, p.bekci(), k.ik_yavas());
    let vS = IN.v_set;
    let setT = |q: &Nokta, q2: &Nokta, dr: f64| -> f64 {
        if dr <= 0.0 {
            return 0.0;
        }
        let bx = q2.x - q.x;
        let by = q2.y - q.y;
        let bl = hypot(bx, by);
        let rx = q.x - px;
        let ry = q.y - py;
        let rl = hypot(rx, ry);
        if bl < 1e-6 || rl < 1e-6 {
            return 0.0;
        }
        let vt = -v * (bx * rx + by * ry) / (bl * rl);
        if vt > vS {
            (vt - vS) * (vt - vS) / (2.0 * aB * v)
        } else {
            0.0
        }
    };
    let mut bs = -1e9;
    let mut best: Option<Ar> = None;
    let mut i = 0usize;
    while i < pts.len() && !(i as f64 > iMax) {
        let fi = i as f64;
        if iMin != 0.0 && iMin == iMin && fi < iMin {
            i += 2;
            continue;
        }
        let q = &pts[i];
        if nb && inK(p, q, fi, k) {
            i += 2;
            continue;
        }
        let mut td = 1e9;
        for kk in 0..opp.len() {
            let o = opp[kk];
            let dk = hypot(q.x - o.x, q.y - o.y) - if MP { menzilO(o.govde(oy), tp0.unwrap() + fi, b.as_deref_mut(), oy, true, k) } else { 1.4 };
            td = jmin(td, oT[kk] + jmax(0.0, dk) / oV[kk] + gecOf(&gO[kk], q.x, q.y, Some(dk)));
        }
        yol = jmin(yol, td - fi);
        if yol < 0.0 {
            break;
        }
        let dr = hypot(q.x - px, q.y - py) - if MP { rP } else { 1.4 };
        let q2 = pts.get(i + 1).unwrap_or(q);
        let tr0 = rc + jmax(0.0, dr) / v + gecOf(&gP, q.x, q.y, Some(dr)) + setT(q, q2, dr);
        let tF = dis::frenT(p, q.x, q.y, if MP { rP } else { 1.4 }, fi - rc, v);
        let tr = jmax(tr0, tF);
        if tr + buf.unwrap_or(0.0) > fi {
            i += 2;
            continue;
        }
        if td < fi {
            i += 2;
            continue;
        }
        let margin = yol;
        if best.is_some() && margin < S.rcvMarginT {
            i += 2;
            continue;
        }
        if best.is_some() && fi > dis::PNC_GORUS {
            break;
        }
        let sc = dis::freeAt(w, oy, p.team, q.x, q.y) - S.rcvRisk * exp(-margin / S.rcvMarginT);
        if sc > bs {
            bs = sc;
            best = Some(Ar { t: fi, x: q.x, y: q.y, est: 0.0, fren: tF > tr0, direnc: false });
        }
        if best.is_some() && margin >= S.rcvMarginT {
            break;
        }
        i += 2;
    }
    best
}

/// JS kendiTopu(m): kendi pasının alıcısı ve topa varışı (uçuş okunuşu taze ise ondan).
pub fn kendiTopu(c: &mut Mc) -> Option<(Pid, f64)> {
    let fq = c.m.fl.as_ref().and_then(|f| f.q)?;
    let b = c.ball.as_ref()?;
    if b.b.done.is_some() || b.t.from != Some(fq) {
        return None;
    }
    let bid = b.t.kimlik;
    let taze = match &c.ek.sh.fr {
        Some(fr) if c.ek.sh.frB == Some(bid) && c.ek.sh.frT > c.m.tick - 3.0 => Some(fr.clone()),
        _ => None,
    };
    let fr = match taze {
        Some(f) => f,
        None => Arc::new(flightRead(c)?),
    };
    if fr.first != Some(fq) {
        return None;
    }
    Some((fq, fr.arr.get(fq).map(|r| r.t).unwrap_or(0.0)))
}

/// JS flightRead(m): topun yolu ve her oyuncunun ona en erken nerede, ne zaman vardığı; alıcının alış yeri
/// (`p._rcv*`). Bekçi kendi tepkisiyle (bekRe) çıkar.
pub fn flightRead(c: &mut Mc) -> Option<Fr> {
    let k = c.k;
    let tick = c.m.tick;
    let fl_q = c.m.fl.as_ref().and_then(|f| f.q);
    let fl_t0 = c.m.fl.as_ref().map(|f| f.t0);
    let pts: Arc<Vec<Nokta>> = {
        let b = c.ball.as_deref()?;
        if b.b.done.is_some() {
            Arc::new(vec![Nokta { x: b.b.x, y: b.b.y, e: b.b.e }])
        } else {
            let pc = if c.lite { c.ek_pc(b.t.kimlik, b.b.t) } else { None };
            match pc {
                Some(p) => p,
                None => {
                    let src: Vec<crate::alan::Etki> = c.ps.iter().map(|p| p.etki()).collect();
                    let (yol, _) = tahmin(&b.b, &mut crate::alan::Kaynak::Sec(&src), &crate::saha::P, Some(240));
                    let a = Arc::new(yol);
                    if c.lite {
                        c.ek.pc = Some((b.t.kimlik, b.b.t, a.clone()));
                    }
                    a
                }
            }
        }
    };
    let n = c.ps.len();
    let w = c.dunya();
    let mut out: Vec<Ar> = Vec::with_capacity(n);
    for pi in 0..n {
        let (bteam, bdefl, bt, btp, bfrom, bid) = {
            let b = c.ball.as_deref().unwrap();
            (b.b.team, b.b.defl, b.b.t, b.b.tp, b.t.from, b.t.kimlik)
        };
        let ps = &*c.ps;
        let p = &ps[pi];
        let nb = !p.bekci();
        let rc = if p.takim() == bteam && fl_q == Some(pi) {
            0.0
        } else {
            let r = if p.bekci() { jmax(0.0, bek_re(p)) } else { okuT(c.ball.as_deref_mut(), p, ps, TEPKI(p, Tepki::Kovala, k), false) };
            let t0 = match fl_t0 {
                Some(t) if t != 0.0 && t == t => t,
                _ => tick,
            };
            jmax(0.0, r - (tick - t0))
        };
        let v = vSp(p, k);
        let mut best: Option<Ar> = None;
        let isRecv = p.takim() == bteam && fl_q == Some(pi) && bdefl == 0.0;
        let cdsv = c.ball.as_deref().unwrap().t.cds.get(&pi).copied().unwrap_or(0.0);
        let iMin = jmax(if bfrom == Some(pi) { jmax(0.0, 8.0 - bt) } else { 0.0 }, jmax(0.0, cdsv - bt));
        let tp0 = btp.unwrap_or(bt);
        let mut rcv: Option<Option<(TopRef, f64, f64, f64, Option<f64>)>> = None;
        if isRecv {
            let iMx = if p._rcvB == Some(bid) { p._rcvAbs.unwrap_or(f64::NAN) + 2.0 - tick } else { 1e9 };
            best = alici(&pts, p, &w, ps, rc, iMx, Some(tp0), iMin, Some(S.rcvBuf), c.ball.as_deref_mut(), k, c.d);
            if best.is_none() {
                best = alici(&pts, p, &w, ps, rc, iMx, Some(tp0), iMin, None, c.ball.as_deref_mut(), k, c.d);
            }
            rcv = Some(best.map(|b| {
                let T = b.t - rc - S.rcvBuf;
                (bid, tick + b.t, b.x, b.y, if T > 1.0 && !b.fren { Some(hypot(b.x - p.x, b.y - p.y) / T) } else { None })
            }));
            if best.is_none() {
                best = aliciDirenc(&pts, p, rc, iMin, Some(menzilP(p, k)), k);
            }
        }
        if best.is_none() {
            let G = gecTab(p, k);
            let mut i = 0usize;
            while i < pts.len() {
                let fi = i as f64;
                if fi < iMin {
                    i += 2;
                    continue;
                }
                let q = &pts[i];
                if nb && inK(p, q, fi, k) {
                    i += 2;
                    continue;
                }
                let r = if p.takim() == bteam { menzilP(p, k) } else { menzilO(p, tp0 + fi, c.ball.as_deref_mut(), ps, false, k) };
                let dr = hypot(q.x - p.x, q.y - p.y) - r;
                let tr = rc + jmax(0.0, dr) / v + gecOf(&G, q.x, q.y, Some(dr));
                if tr <= fi {
                    best = Some(Ar { t: fi, x: q.x, y: q.y, ..Default::default() });
                    break;
                }
                i += 2;
            }
        }
        let mut best = match best {
            Some(b) => b,
            None => {
                let b = c.ball.as_deref().unwrap();
                let mut q = pts.last().copied().unwrap_or(Nokta { x: b.b.x, y: b.b.y, e: 0.0 });
                if nb && inK(p, &q, pts.len() as f64 - 1.0, k) {
                    let mut i = pts.len() as i64 - 1;
                    while i >= 0 {
                        if !inK(p, &pts[i as usize], i as f64, k) {
                            q = pts[i as usize];
                            break;
                        }
                        i -= 1;
                    }
                }
                Ar { t: jmax(pts.len() as f64, rc + hypot(q.x - p.x, q.y - p.y) / v), x: q.x, y: q.y, ..Default::default() }
            }
        };
        let ft0 = match fl_t0 {
            Some(t) if t != 0.0 && t == t => t,
            _ => 0.0,
        };
        best.est = best.t * (1.0 + hsh(p.id, ft0, 3.0) * (20.0 - p.yet.y(mek::ALGI_HATA)) * 0.04);
        if let Some(r) = rcv {
            let p = &mut c.ps[pi];
            p._rcvV = None;
            if let Some((bid, abs, x, y, vv)) = r {
                p._rcvB = Some(bid);
                p._rcvAbs = Some(abs);
                p._rcvX = Some(x);
                p._rcvY = Some(y);
                p._rcvV = vv;
            }
        }
        out.push(best);
    }
    let mut first: Option<Pid> = None;
    for (pi, r) in out.iter().enumerate() {
        if first.is_none() || r.t < out[first.unwrap()].t {
            first = Some(pi);
        }
    }
    Some(Fr { arr: out, first, pts })
}

impl<'a> Mc<'a> {
    /// JS `b._pc` (hayali maçta topun yolu 6 tik saklanır).
    fn ek_pc(&self, id: TopRef, t: f64) -> Option<Arc<Vec<Nokta>>> {
        match &self.ek.pc {
            Some((i, t0, p)) if *i == id && t - t0 < 6.0 => {
                let s = (t - t0) as usize;
                Some(Arc::new(p[s.min(p.len())..].to_vec()))
            }
            _ => None,
        }
    }
}
