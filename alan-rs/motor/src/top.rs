//! Top (Çekirdek) fiziği (core.js: direk, chgMul, sahaIci, makeBall, stepBall, rollTo, predict, solveLaunch).

use crate::alan::{k, Kaynak};
use crate::mat::{hypot as hyp_std, jmax, jmin};
use crate::saha::*;

/// JS `hyp`: taşmaya dayanıklı uzunluk (√(1 + r²)·m). `Math.hypot`'tan farklı yuvarlar, top bunu kullanır.
#[inline(always)]
pub fn hyp(x: f64, y: f64) -> f64 {
    let x = if x < 0.0 { -x } else { x };
    let y = if y < 0.0 { -y } else { y };
    if x == f64::INFINITY || y == f64::INFINITY {
        return f64::INFINITY;
    }
    let m = if x > y { x } else { y };
    if m != m || x != x || y != y {
        return f64::NAN;
    }
    if m == 0.0 {
        return 0.0;
    }
    let r = (if m == x { y } else { x }) / m;
    (1.0 + r * r).sqrt() * m
}

#[allow(dead_code)]
#[inline(always)]
fn _hyp_std(x: f64, y: f64) -> f64 {
    hyp_std(x, y)
}

/// Topun neden durduğu (JS `b.done`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bitis {
    KuyuSol,
    KuyuSag,
    Durdu,
}
impl Bitis {
    pub fn ad(self) -> &'static str {
        match self {
            Bitis::KuyuSol => "kuyu-sol",
            Bitis::KuyuSag => "kuyu-sag",
            Bitis::Durdu => "durdu",
        }
    }
}

/// Top günlüğü (JS `b.log`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TopOlay {
    Son,
    Direk,
    Kuyu { sag: bool },
    KenarUc { sag: bool },
    KenarYan { ust: bool },
    Dur { sonuk: bool },
}

#[derive(Clone, Debug)]
pub struct Top {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    /// enerji (tek enerji: şarj = uçuş enerjisi)
    pub e: f64,
    pub ch: f64,
    pub team: u8,
    pub alive: bool,
    pub dead: f64,
    pub t: f64,
    pub dist: f64,
    pub walls: f64,
    pub done: Option<Bitis>,
    pub log: Vec<(f64, TopOlay)>,
    pub direk: f64,
    /// duvarsız, sapmasız geçen tik (JS tp, _w0, _d0)
    pub tp: Option<f64>,
    pub w0: Option<f64>,
    pub d0: Option<f64>,
    pub defl: f64,
}

impl Top {
    /// JS makeBall varsayılanları (tek enerji açık: e = ch = max(taban, ch)).
    pub fn yeni(x: f64, y: f64, vx: f64, vy: f64, team: u8, ch: Option<f64>, taban: f64) -> Top {
        let c0 = jmax(taban, ch.unwrap_or(0.5));
        let mut b = Top {
            x,
            y,
            vx,
            vy,
            e: c0,
            ch: c0,
            team,
            alive: true,
            dead: 0.0,
            t: 0.0,
            dist: 0.0,
            walls: 0.0,
            done: None,
            log: Vec::new(),
            direk: 0.0,
            tp: None,
            w0: None,
            d0: None,
            defl: 0.0,
        };
        saha_ici(&mut b);
        b
    }
    /// JS clone: günlük boş.
    pub fn kopya(&self) -> Top {
        let mut b = self.clone();
        b.log = Vec::new();
        b
    }
}

pub fn saha_ici(b: &mut Top) {
    let e = 0.001;
    if b.x < 0.5 + e {
        b.x = 0.5 + e;
    } else if b.x > W - 0.5 - e {
        b.x = W - 0.5 - e;
    }
    if b.y < 0.5 + e {
        b.y = 0.5 + e;
    } else if b.y > H - 0.5 - e {
        b.y = H - 0.5 - e;
    }
}

/// Şarjın vuruş hızına çarpanı (tek enerji: .75 + .7·ch).
#[inline(always)]
pub fn chg_mul(ch: f64) -> f64 {
    0.75 + 0.7 * ch
}

/// Direğe çarpma: bu tikte (x,y)+(vx,vy) yolunda dört direkten birine değiyorsa yeni yer ve hız.
pub fn direk(x: f64, y: f64, vx: f64, vy: f64, p: &Fizik) -> Option<[f64; 4]> {
    if DIREK <= 0.0 {
        return None;
    }
    let nx = x + vx;
    if x > DIREK + 4.0 && nx > DIREK + 4.0 && x < W - DIREK - 4.0 && nx < W - DIREK - 4.0 {
        return None;
    }
    let l2 = vx * vx + vy * vy;
    if l2 < 1e-12 {
        return None;
    }
    let mut tw = 2.0;
    if nx <= 0.5 && vx.abs() > 1e-9 {
        tw = (0.5 - x) / vx;
    } else if nx >= W - 0.5 && vx.abs() > 1e-9 {
        tw = (W - 0.5 - x) / vx;
    }
    let (mut bt, mut bx, mut by) = (2.0f64, 0.0f64, 0.0f64);
    for px in [0.5, W - 0.5] {
        for py in [H / 2.0 - MOUTH, H / 2.0 + MOUTH] {
            let fx = x - px;
            let fy = y - py;
            let c0 = fx * fx + fy * fy - DIREK * DIREK;
            if c0 <= 0.0 {
                continue;
            }
            let bb = fx * vx + fy * vy;
            if bb >= 0.0 {
                continue;
            }
            let disc = bb * bb - l2 * c0;
            if disc < 0.0 {
                continue;
            }
            let t = (-bb - disc.sqrt()) / l2;
            if !(t < 0.0 || t > 1.0 || t >= bt) {
                bt = t;
                bx = px;
                by = py;
            }
        }
    }
    if bt > 1.0 || bt > tw {
        return None;
    }
    let cx = x + vx * bt;
    let cy = y + vy * bt;
    let mut nnx = cx - bx;
    let mut nny = cy - by;
    let nl = {
        let h = hyp(nnx, nny);
        if h == 0.0 || h != h {
            1.0
        } else {
            h
        }
    };
    nnx /= nl;
    nny /= nl;
    let vn = vx * nnx + vy * nny;
    Some([bx + nnx * (DIREK + 0.001), by + nny * (DIREK + 0.001), (vx - 2.0 * vn * nnx) * p.rest, (vy - 2.0 * vn * nny) * p.rest])
}

/// Bir tikin sürtünme ve enerji hesabı (stepBall, rollTo ve predict'in ortak başı).
/// (drag, yeni e, alive, dead, ch, söndü mü)
#[inline(always)]
fn surtunme(kk: f64, sp0: f64, mut e: f64, mut alive: bool, mut dead: f64, ch: f64, p: &Fizik) -> (f64, f64, bool, f64, f64, bool) {
    let res = 1.0 - p.pierce * ch;
    let mut drag = p.fric;
    if kk < 0.0 {
        drag += -kk * p.opp_drag * res * p.tek_drag;
    } else {
        drag *= 1.0 - p.own_flow * kk;
    }
    let mut son = false;
    let mut ch2 = ch;
    if alive {
        e -= (sp0 * p.attn + if kk < 0.0 { -kk * sp0 * p.drain * res * p.tek_drain } else { 0.0 }) * p.tek_s;
        if kk > 0.0 {
            e = jmin(1.0, e + kk * sp0 * p.tek_gain);
        }
        if e <= 0.0 {
            e = 0.0;
            alive = false;
            dead = 0.0;
            son = true;
        }
        ch2 = if alive { e } else { 0.0 };
    } else {
        dead += 1.0;
        drag += jmin(p.dead_max, p.dead_ramp * dead);
    }
    (drag, e, alive, dead, ch2, son)
}

#[inline(always)]
fn yuvarlan(vx: &mut f64, vy: &mut f64, drag: f64, p: &Fizik) {
    let f = 1.0 - drag;
    *vx *= f;
    *vy *= f;
    let s1 = hyp(*vx, *vy);
    if s1 > 0.0 {
        let r = jmax(0.0, s1 - p.roll) / s1;
        *vx *= r;
        *vy *= r;
    }
}

/// Topun bir tiki (JS stepBall).
pub fn adim(b: &mut Top, src: &mut Kaynak, p: &Fizik) {
    if b.done.is_some() {
        return;
    }
    let kk = k(src, b.x, b.y, b.team);
    let sp0 = hyp(b.vx, b.vy);
    let (drag, e, alive, dead, ch, son) = surtunme(kk, sp0, b.e, b.alive, b.dead, b.ch, p);
    b.e = e;
    b.alive = alive;
    b.dead = dead;
    b.ch = ch;
    if son {
        b.log.push((b.t, TopOlay::Son));
    }
    yuvarlan(&mut b.vx, &mut b.vy, drag, p);
    let mut nx = b.x + b.vx;
    let mut ny = b.y + b.vy;
    if let Some(dk) = direk(b.x, b.y, b.vx, b.vy, p) {
        nx = dk[0];
        ny = dk[1];
        b.vx = dk[2];
        b.vy = dk[3];
        b.walls += 1.0;
        b.direk += 1.0;
        if b.alive {
            b.e = jmax(0.0, b.e - p.wall_loss * p.tek_s);
        }
        b.log.push((b.t, TopOlay::Direk));
    } else {
        for ex in [0.0, W] {
            let hit = if ex == 0.0 { nx <= 0.5 } else { nx >= W - 0.5 };
            if !hit {
                continue;
            }
            let wx = if ex == 0.0 { 0.5 } else { W - 0.5 };
            let tt = if b.vx.abs() > 1e-9 { (wx - b.x) / b.vx } else { 0.0 };
            let yy = b.y + b.vy * jmax(0.0, jmin(1.0, tt));
            if (yy - H / 2.0).abs() <= MOUTH {
                b.x = wx;
                b.y = yy;
                b.done = Some(if ex == 0.0 { Bitis::KuyuSol } else { Bitis::KuyuSag });
                b.log.push((b.t, TopOlay::Kuyu { sag: ex != 0.0 }));
                return;
            }
            nx = if ex == 0.0 { 1.0 - nx } else { 2.0 * W - 1.0 - nx };
            b.vx = -b.vx * p.rest;
            b.vy *= p.rest;
            b.walls += 1.0;
            if b.alive {
                b.e = jmax(0.0, b.e - p.wall_loss * p.tek_s);
            }
            b.log.push((b.t, TopOlay::KenarUc { sag: ex != 0.0 }));
        }
    }
    if ny < 0.5 || ny > H - 0.5 {
        ny = if ny < 0.5 { 1.0 - ny } else { 2.0 * H - 1.0 - ny };
        b.vy = -b.vy * p.rest;
        b.vx *= p.rest;
        b.walls += 1.0;
        if b.alive {
            b.e = jmax(0.0, b.e - p.wall_loss * p.tek_s);
        }
        b.log.push((b.t, TopOlay::KenarYan { ust: ny < H / 2.0 }));
    }
    b.dist += hyp(nx - b.x, ny - b.y);
    b.x = nx;
    b.y = ny;
    b.t += 1.0;
    if Some(b.walls) != b.w0 || b.defl != b.d0.unwrap_or(f64::NAN) {
        b.tp = Some(0.0);
        b.w0 = Some(b.walls);
        b.d0 = Some(b.defl);
    } else {
        b.tp = Some(b.tp.unwrap_or(b.t - 1.0) + 1.0);
    }
    if hyp(b.vx, b.vy) < 0.015 {
        b.vx = 0.0;
        b.vy = 0.0;
        b.done = Some(Bitis::Durdu);
        b.log.push((b.t, TopOlay::Dur { sonuk: !b.alive }));
    }
}

/// Yuvarlanan top (hx, hy)'den L uzaklığa vardığında hızı; varamazsa −1 (JS rollTo).
#[allow(clippy::too_many_arguments)]
pub fn roll_to(mut x: f64, mut y: f64, mut vx: f64, mut vy: f64, team: u8, ch0: f64, src: &mut Kaynak, hx: f64, hy: f64, l: f64, max_t: f64) -> f64 {
    let p = &P;
    let mut e = jmax(p.tek_taban, ch0);
    let mut alive = true;
    let mut dead = 0.0;
    let mut ch = e;
    let mut t = 0.0;
    while t < max_t {
        let kk = k(src, x, y, team);
        let sp0 = hyp(vx, vy);
        let (drag, e2, a2, d2, c2, _) = surtunme(kk, sp0, e, alive, dead, ch, p);
        e = e2;
        alive = a2;
        dead = d2;
        ch = c2;
        yuvarlan(&mut vx, &mut vy, drag, p);
        let mut nx = x + vx;
        let mut ny = y + vy;
        let mut done = false;
        if let Some(dk) = direk(x, y, vx, vy, p) {
            nx = dk[0];
            ny = dk[1];
            vx = dk[2];
            vy = dk[3];
            if alive {
                e = jmax(0.0, e - p.wall_loss * p.tek_s);
            }
        } else {
            if nx <= 0.5 {
                let wx = 0.5;
                let tt = if vx.abs() > 1e-9 { (wx - x) / vx } else { 0.0 };
                let yy = y + vy * jmax(0.0, jmin(1.0, tt));
                if (yy - H / 2.0).abs() <= MOUTH {
                    x = wx;
                    y = yy;
                    done = true;
                } else {
                    nx = 1.0 - nx;
                    vx = -vx * p.rest;
                    vy *= p.rest;
                    if alive {
                        e = jmax(0.0, e - p.wall_loss * p.tek_s);
                    }
                }
            }
            if !done && nx >= W - 0.5 {
                let wx = W - 0.5;
                let tt = if vx.abs() > 1e-9 { (wx - x) / vx } else { 0.0 };
                let yy = y + vy * jmax(0.0, jmin(1.0, tt));
                if (yy - H / 2.0).abs() <= MOUTH {
                    x = wx;
                    y = yy;
                    done = true;
                } else {
                    nx = 2.0 * W - 1.0 - nx;
                    vx = -vx * p.rest;
                    vy *= p.rest;
                    if alive {
                        e = jmax(0.0, e - p.wall_loss * p.tek_s);
                    }
                }
            }
        }
        if !done {
            if ny < 0.5 || ny > H - 0.5 {
                ny = if ny < 0.5 { 1.0 - ny } else { 2.0 * H - 1.0 - ny };
                vy = -vy * p.rest;
                vx *= p.rest;
                if alive {
                    e = jmax(0.0, e - p.wall_loss * p.tek_s);
                }
            }
            x = nx;
            y = ny;
            if hyp(vx, vy) < 0.015 {
                vx = 0.0;
                vy = 0.0;
                done = true;
            }
        }
        if hyp(x - hx, y - hy) >= l {
            return hyp(vx, vy);
        }
        if done {
            return -1.0;
        }
        t += 1.0;
    }
    -1.0
}

/// predict'in yol noktası.
#[derive(Clone, Copy, Debug)]
pub struct YolNokta {
    pub x: f64,
    pub y: f64,
    pub e: f64,
}

/// Topun yolunu ileri oynat (JS predict): noktalar ve son hâl.
pub fn tahmin(b0: &Top, src: &mut Kaynak, p: &Fizik, max_t: Option<usize>) -> (Vec<YolNokta>, Top) {
    let mut b = b0.kopya();
    let mut pts = Vec::with_capacity(64);
    pts.push(YolNokta { x: b.x, y: b.y, e: b.e });
    let n = match max_t {
        Some(m) if m > 0 => m,
        _ => 900,
    };
    let team = b.team;
    let mut ch = b.ch;
    let (mut x, mut y, mut vx, mut vy, mut e, mut alive, mut dead, mut t, mut dist, mut walls, mut done) = (b.x, b.y, b.vx, b.vy, b.e, b.alive, b.dead, b.t, b.dist, b.walls, b.done);
    let mut i = 0;
    while i < n && done.is_none() {
        let kk = k(src, x, y, team);
        let sp0 = hyp(vx, vy);
        let (drag, e2, a2, d2, c2, son) = surtunme(kk, sp0, e, alive, dead, ch, p);
        e = e2;
        alive = a2;
        dead = d2;
        ch = c2;
        if son {
            b.log.push((t, TopOlay::Son));
        }
        yuvarlan(&mut vx, &mut vy, drag, p);
        let mut nx = x + vx;
        let mut ny = y + vy;
        if let Some(dk) = direk(x, y, vx, vy, p) {
            nx = dk[0];
            ny = dk[1];
            vx = dk[2];
            vy = dk[3];
            walls += 1.0;
            b.direk += 1.0;
            if alive {
                e = jmax(0.0, e - p.wall_loss * p.tek_s);
            }
            b.log.push((t, TopOlay::Direk));
        } else {
            if nx <= 0.5 {
                let wx = 0.5;
                let tt = if vx.abs() > 1e-9 { (wx - x) / vx } else { 0.0 };
                let yy = y + vy * jmax(0.0, jmin(1.0, tt));
                if (yy - H / 2.0).abs() <= MOUTH {
                    x = wx;
                    y = yy;
                    done = Some(Bitis::KuyuSol);
                    b.log.push((t, TopOlay::Kuyu { sag: false }));
                } else {
                    nx = 1.0 - nx;
                    vx = -vx * p.rest;
                    vy *= p.rest;
                    walls += 1.0;
                    if alive {
                        e = jmax(0.0, e - p.wall_loss * p.tek_s);
                    }
                    b.log.push((t, TopOlay::KenarUc { sag: false }));
                }
            }
            if done.is_none() && nx >= W - 0.5 {
                let wx = W - 0.5;
                let tt = if vx.abs() > 1e-9 { (wx - x) / vx } else { 0.0 };
                let yy = y + vy * jmax(0.0, jmin(1.0, tt));
                if (yy - H / 2.0).abs() <= MOUTH {
                    x = wx;
                    y = yy;
                    done = Some(Bitis::KuyuSag);
                    b.log.push((t, TopOlay::Kuyu { sag: true }));
                } else {
                    nx = 2.0 * W - 1.0 - nx;
                    vx = -vx * p.rest;
                    vy *= p.rest;
                    walls += 1.0;
                    if alive {
                        e = jmax(0.0, e - p.wall_loss * p.tek_s);
                    }
                    b.log.push((t, TopOlay::KenarUc { sag: true }));
                }
            }
        }
        if done.is_none() {
            if ny < 0.5 || ny > H - 0.5 {
                ny = if ny < 0.5 { 1.0 - ny } else { 2.0 * H - 1.0 - ny };
                vy = -vy * p.rest;
                vx *= p.rest;
                walls += 1.0;
                if alive {
                    e = jmax(0.0, e - p.wall_loss * p.tek_s);
                }
                b.log.push((t, TopOlay::KenarYan { ust: ny < H / 2.0 }));
            }
            dist += hyp(nx - x, ny - y);
            x = nx;
            y = ny;
            t += 1.0;
            if hyp(vx, vy) < 0.015 {
                vx = 0.0;
                vy = 0.0;
                done = Some(Bitis::Durdu);
                b.log.push((t, TopOlay::Dur { sonuk: !alive }));
            }
        }
        pts.push(YolNokta { x, y, e });
        i += 1;
    }
    b.ch = ch;
    b.x = x;
    b.y = y;
    b.vx = vx;
    b.vy = vy;
    b.e = e;
    b.alive = alive;
    b.dead = dead;
    b.t = t;
    b.dist = dist;
    b.walls = walls;
    b.done = done;
    (pts, b)
}

/// `to` noktasına `arrive` hızıyla varacak çıkış hızı (JS solveLaunch, 18 adım ikiye bölme).
pub fn cikis_coz(b0: &Top, to: (f64, f64), arrive: f64, src: &mut Kaynak, p: &Fizik, vmax: f64) -> (f64, bool) {
    let dx = to.0 - b0.x;
    let dy = to.1 - b0.y;
    let l = {
        let h = hyp(dx, dy);
        if h == 0.0 || h != h {
            1.0
        } else {
            h
        }
    };
    let ux = dx / l;
    let uy = dy / l;
    let vmax = if vmax == 0.0 || vmax != vmax { 3.0 } else { vmax };
    let hiz = |v0: f64, src: &mut Kaynak| -> f64 {
        let mut b = b0.kopya();
        b.vx = ux * v0;
        b.vy = uy * v0;
        b.done = None;
        let mut i = 0;
        while i < 900 && b.done.is_none() {
            adim(&mut b, src, p);
            let d = (b.x - b0.x) * ux + (b.y - b0.y) * uy;
            let last = hyp(b.vx, b.vy);
            if d >= l {
                return last;
            }
            if b.walls != 0.0 {
                return -1.0;
            }
            i += 1;
        }
        -1.0
    };
    let mut lo = 0.05;
    let mut hi = vmax;
    if hiz(hi, src) < arrive {
        return (hi, false);
    }
    for _ in 0..18 {
        let mid = (lo + hi) / 2.0;
        if hiz(mid, src) < arrive {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    (hi, true)
}
