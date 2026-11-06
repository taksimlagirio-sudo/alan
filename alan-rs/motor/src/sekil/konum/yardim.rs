//! position'un yardımcıları ve ara yapıları.
#![allow(non_snake_case, dead_code)]

use crate::mat::{hypot, jmax, jmin};
use crate::model::{Oyuncu, Pid};
use crate::temas::Kural;

use super::super::gecici_dis as dis;
use super::super::*;

/// JS iş adının başı (regex `^(a|b|…)`).
pub(super) fn bas(job: &Option<std::sync::Arc<str>>, on: &[&str]) -> bool {
    let j = job.as_deref().unwrap_or("");
    on.iter().any(|o| j.starts_with(o))
}
pub(super) fn icerir(job: &Option<std::sync::Arc<str>>, s: &str) -> bool {
    job.as_deref().unwrap_or("").contains(s)
}
/// JS tl(p, ev, k) sayı olarak (null → 0: kullanıldığı her yerde aynı sonuç).
#[inline]
pub(super) fn tl0(p: &Oyuncu, ev: &str, k: &str) -> f64 {
    dis::tl(p, ev, k).unwrap_or(0.0)
}
/// JS `x || 1`.
#[inline]
pub(super) fn nz1(l: f64) -> f64 {
    if l != 0.0 && l == l {
        l
    } else {
        1.0
    }
}
/// JS `p.lock > t`.
#[inline]
pub(super) fn kilitli(p: &Oyuncu, t: f64) -> bool {
    p.hal.lock.map(|l| l > t).unwrap_or(false)
}
/// JS `(u.lock > t ? u.lockAt : t)`.
#[inline]
pub(super) fn kilit_an(p: &Oyuncu, t: f64) -> f64 {
    if kilitli(p, t) {
        p.hal.lockAt.unwrap_or(f64::NAN)
    } else {
        t
    }
}
/// JS sıralama karşılaştırıcısı `a - b` (NaN → eşit).
#[inline]
pub(super) fn sira(d: f64) -> std::cmp::Ordering {
    d.partial_cmp(&0.0).unwrap_or(std::cmp::Ordering::Equal)
}
/// JS `a || b` iki sayıyla (0 ve NaN yanlış).
#[inline]
pub(super) fn yada(a: f64, b: f64) -> f64 {
    if a != 0.0 && a == a {
        a
    } else {
        b
    }
}

/// Hücumcunun aday noktası (JS `cands` öğesi).
#[derive(Clone, Debug, Default)]
pub(super) struct Aday {
    pub x: f64,
    pub y: f64,
    pub hat: bool,
    pub kv: Option<(f64, f64)>,
    /// kovalayan hayalet (görülen dünyadaki sırası)
    pub kd: Option<usize>,
    pub kor: bool,
    pub vac: bool,
    pub uc: Option<Pid>,
    pub vg: bool,
    pub bind: Option<&'static str>,
    pub bh: Option<BindH>,
    pub bind_t: bool,
    pub dogal: bool,
}
impl Aday {
    pub fn xy(x: f64, y: f64) -> Aday {
        Aday { x, y, ..Default::default() }
    }
}

/// bindCak'ın taşıyıcısı (JS `{x, y, tx, ty, vx, vy, a}`).
#[derive(Clone, Copy, Debug)]
pub(super) struct BindH {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub tx: Option<f64>,
    pub ty: Option<f64>,
    pub hz: f64,
    /// taşıyıcının sırası (iş adı için)
    pub pid: Pid,
}
impl BindH {
    pub fn of(p: &Oyuncu, pid: Pid) -> BindH {
        BindH { x: p.x, y: p.y, vx: p.vx, vy: p.vy, tx: Some(p.tx), ty: Some(p.ty), hz: p.hz(), pid }
    }
}

/// JS bindCak(p, c, b): p'nin c'ye koşusu ile taşıyıcının yolu arasındaki en yakın mesafe.
pub(super) fn bind_cak(p: &Oyuncu, cx: f64, cy: f64, b: &BindH, k: &dyn Kural) -> f64 {
    let L = {
        let l = hypot(cx - p.x, cy - p.y);
        if l != 0.0 && l == l {
            l
        } else {
            1e-9
        }
    };
    let v = v_sp(p, k);
    let T = L / v;
    let ux = (cx - p.x) / L * v;
    let uy = (cy - p.y) / L * v;
    let niyet = b.tx.is_some();
    let (btx, bty) = (b.tx.unwrap_or(0.0), b.ty.unwrap_or(0.0));
    let n_l = if niyet { hypot(btx - b.x, bty - b.y) } else { 0.0 };
    let nv = if niyet { 0.17 + 0.006 * b.hz } else { 0.0 };
    let TT = if niyet { jmax(T, n_l / nv) } else { T };
    let mut mn: f64 = 1e9;
    let mut t = 0.0;
    while t <= TT + 1e-9 {
        let kk = jmin(t, T);
        let (bx, by) = if niyet {
            if n_l > 1e-9 {
                (b.x + (btx - b.x) * jmin(1.0, nv * t / n_l), b.y + (bty - b.y) * jmin(1.0, nv * t / n_l))
            } else {
                (b.x, b.y)
            }
        } else {
            (b.x + b.vx * t, b.y + b.vy * t)
        };
        mn = jmin(mn, hypot(p.x + ux * kk - bx, p.y + uy * kk - by));
        t += jmax(1.0, TT / 12.0);
    }
    mn
}

/// JS `oppIn(sn, x, y)`: görülenlerin alan etkisi toplamı.
pub(super) fn opp_in(sn: &[Varlik], ps: &[Oyuncu], x: f64, y: f64) -> f64 {
    let mut s = 0.0;
    for q in sn {
        let g = q.govde(ps);
        let e = crate::alan::Etki { x: q.x, y: q.y, r: if g.R != 0.0 && g.R == g.R { g.R } else { 8.0 }, d: g.D, koni: false, fx: 0.0, fy: 0.0, team: q.team, am_a: false, am: 0.0, en: 1.0 };
        s += e.infl(x, y);
    }
    s
}
/// JS `laneIn(sn, a, b, n=4)`.
pub(super) fn lane_in(sn: &[Varlik], ps: &[Oyuncu], a: (f64, f64), b: (f64, f64)) -> f64 {
    let n = 4;
    let mut s = 0.0;
    for i in 1..=n {
        let t = i as f64 / (n as f64 + 1.0);
        s += opp_in(sn, ps, a.0 + (b.0 - a.0) * t, a.1 + (b.1 - a.1) * t);
    }
    s / n as f64
}

/// JS `okuT(p)` (savunucunun okuma ufku).
#[inline]
pub(super) fn oku_tp(p: &Oyuncu) -> f64 {
    cl(niyet(p) - 6.0 + 4.0, 2.0, 18.0)
}

/// JS `ileri(q, T, r)` hayalet için: görülen hızıyla (gerçek maçta) ya da gerçek hızla T tik ileri.
pub(super) fn ileri_v(o: &Varlik, T: f64, r: Option<&Oyuncu>, ps: &[Oyuncu], sg: bool) -> (f64, f64) {
    let (vx, vy) = match (sg, o.gvx) {
        (true, Some(gx)) => (gx, o.gvy.unwrap_or(0.0)),
        _ => {
            let v = r.unwrap_or_else(|| o.r0(ps));
            (v.vx, v.vy)
        }
    };
    if T != 0.0 && T == T {
        (cl(o.x + vx * T, 1.0, GW - 1.0), cl(o.y + vy * T, 1.0, GH - 1.0))
    } else {
        (o.x, o.y)
    }
}
/// JS `ileri(q, T)` gerçek oyuncu için.
pub(super) fn ileri_p(q: &Oyuncu, T: f64) -> (f64, f64) {
    if T != 0.0 && T == T {
        (cl(q.x + q.vx * T, 1.0, GW - 1.0), cl(q.y + q.vy * T, 1.0, GH - 1.0))
    } else {
        (q.x, q.y)
    }
}

/// Eşleşme işinin hedefi (JS isler / Js öğesinin `q`su).
#[derive(Clone, Debug)]
pub(super) enum IsQ {
    P(Pid),
    Bolge { id: f64, x: f64, y: f64 },
}
/// Eşleşme işi (JS isler / Js öğesi).
#[derive(Clone, Debug)]
pub(super) struct Is {
    pub q: IsQ,
    pub w: f64,
    pub g: (f64, f64),
    pub job: String,
    pub derin: bool,
    pub bul: bool,
    pub oku_f: bool,
    pub grp: Option<f64>,
    pub car: Option<usize>,
    pub zone: Option<Pid>,
    pub rq: Vec<(Pid, f64)>,
    pub thr: f64,
}
impl Is {
    pub fn qp(&self) -> Option<Pid> {
        match self.q {
            IsQ::P(p) => Some(p),
            _ => None,
        }
    }
}
/// UA çıktısı (JS `out.get(p)`).
#[derive(Clone, Debug)]
pub(super) struct Ua {
    pub q: Pid,
    pub g: (f64, f64),
    pub job: String,
    pub derin: bool,
    pub bul: bool,
    pub sprint: bool,
}

/// JS PRS: pres adaylarının tartısı.
pub(super) struct Prs {
    pub cand: Vec<bool>,
    pub pv: Vec<Option<PrV>>,
    pub p_lim: usize,
    pub G: f64,
    pub K: f64,
    pub P0: f64,
    pub fixed: Vec<Pid>,
}
#[derive(Clone, Copy, Debug)]
pub(super) struct PrV {
    pub P: f64,
    pub v: f64,
    pub c: f64,
    pub t_r: f64,
}

/// JS `tP` (kafanın pres tartısı, `m._prT`).
pub(super) struct TP {
    pub m: Vec<(f64, f64)>,
    pub mm: Vec<(String, f64)>,
    pub hm: Vec<(f64, f64)>,
    pub h: Option<f64>,
}
impl TP {
    pub fn m_get(&self, id: f64) -> Option<f64> {
        self.m.iter().find(|x| x.0 == id).map(|x| x.1)
    }
    pub fn hm_get(&self, id: f64) -> Option<f64> {
        self.hm.iter().find(|x| x.0 == id).map(|x| x.1)
    }
    pub fn mm_get(&self, k: &str) -> Option<f64> {
        self.mm.iter().find(|x| x.0 == k).map(|x| x.1)
    }
}

/// JS sayının metni (tam sayı kimlikler için).
pub(super) fn id_str(x: f64) -> String {
    if x == x.trunc() && x.abs() < 1e15 {
        format!("{}", x as i64)
    } else {
        format!("{}", x)
    }
}

/// JS `perms(a)`: bütün sıralamalar (JS'teki sırayla).
pub(super) fn perms<T: Clone>(a: &[T]) -> Vec<Vec<T>> {
    if a.len() <= 1 {
        return vec![a.to_vec()];
    }
    let mut out = Vec::new();
    for i in 0..a.len() {
        let mut kal = a[..i].to_vec();
        kal.extend_from_slice(&a[i + 1..]);
        for r in perms(&kal) {
            let mut v = vec![a[i].clone()];
            v.extend(r);
            out.push(v);
        }
    }
    out
}
