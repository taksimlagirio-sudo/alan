//! Topu döndürme (core.js Turn): taşıyıcı Çekirdeği gövdesinin etrafında dolaştırır ya da fiskeyle aşırtır.

use crate::core_sabit::{Q, TQ};
use crate::mat::{cos, jmax, jmin, kis, sin, PI};
use crate::oyuncu::Oyuncu;
use crate::top::hyp;
use crate::yetenek::{denge_k, mek};

pub fn wrap_a(mut a: f64) -> f64 {
    while a > PI {
        a -= 2.0 * PI;
    }
    while a < -PI {
        a += 2.0 * PI;
    }
    a
}

/// Ayak tarafının etkisi (şimdilik herkes 1).
pub fn taraf(_p: &Oyuncu) -> f64 {
    1.0
}

/// Dönüş hızı (rad/tik): yakın rakipten saklarken Dolaş, açıkta Savurma; koşarken yavaş, dengesizken yavaş.
pub fn omega(p: &Oyuncu, yakin: bool, yavas: f64) -> f64 {
    let v = hyp(p.vx, p.vy);
    let d_k = match p.den {
        Some(d) if d < 1.0 && !p.bekci() => denge_k(Some(d), yavas),
        _ => 1.0,
    };
    jmax(0.03, (TQ.w0 + (p.govde.yet.y(if yakin { mek::DOLAS } else { mek::SAVURMA_HIZ }) - 10.0) * TQ.w_sk) * (1.0 - TQ.w_v * jmin(1.0, v / TQ.v_ref)) * taraf(p) * d_k)
}

pub fn short_dir(a0: f64, a1: f64) -> f64 {
    if wrap_a(a1 - a0) >= 0.0 {
        1.0
    } else {
        -1.0
    }
}

pub fn arc(a0: f64, a1: f64, dir: f64) -> f64 {
    let mut d = wrap_a(a1 - a0);
    if dir > 0.0 && d < 0.0 {
        d += 2.0 * PI;
    }
    if dir < 0.0 && d > 0.0 {
        d -= 2.0 * PI;
    }
    d.abs()
}

/// Rakip (o) ile Çekirdek arasında taşıyıcının gövdesi var mı.
pub fn shielded(px: f64, py: f64, ox: f64, oy: f64, cx: f64, cy: f64) -> bool {
    let sx = cx - ox;
    let sy = cy - oy;
    let l2 = {
        let v = sx * sx + sy * sy;
        if v == 0.0 {
            1.0
        } else {
            v
        }
    };
    let t = kis(((px - ox) * sx + (py - oy) * sy) / l2, 0.0, 1.0);
    hyp(ox + sx * t - px, oy + sy * t - py) < TQ.shield
}

pub fn flick_miss(p: &Oyuncu) -> f64 {
    jmax(0.02, 0.12 - 0.015 * (p.govde.yet.y(mek::FISKE_KACIR) - 10.0))
}

pub fn flick_w(p: &Oyuncu) -> f64 {
    TQ.flick_w * (1.0 + 0.05 * (p.govde.yet.y(mek::FISKE_HIZ) - 10.0))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mod {
    Dolas,
    Fiske,
}

#[derive(Clone, Copy, Debug)]
pub struct Plan {
    pub mode: Mod,
    pub dir: f64,
    pub keep: f64,
    pub cost: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DonusSonuc {
    Dondu,
    FiskeKacti,
    RakipAldi,
}

/// Dönüşü tik tik oynat (JS Turn.simulate). rnd yoksa beklenen tutma (keep) çarpılır.
/// Dönüş: sonuç, bitiş tiki, tutma.
pub fn simulate(p0: &Oyuncu, a0: f64, a1: f64, mode: Mod, dir: f64, opps0: &[Oyuncu], mut rnd: Option<&mut dyn FnMut() -> f64>, yavas: f64) -> (DonusSonuc, Option<f64>, f64) {
    let mut px = p0.x;
    let mut py = p0.y;
    let mut ox: Vec<f64> = opps0.iter().map(|o| o.x).collect();
    let mut oy: Vec<f64> = opps0.iter().map(|o| o.y).collect();
    let mut ocd: Vec<f64> = vec![0.0; opps0.len()];
    let body = Q.body;
    let r = Q.reach;
    let a = arc(a0, a1, dir);
    let sur = p0.govde.yet.ozk("surme");
    let mut done = 0.0f64;
    let mut keep = 1.0;
    let mut res: Option<DonusSonuc> = None;
    let mut t_end: Option<f64> = None;
    let mut landed = false;
    // omega taşıyıcının kopyasıyla (yeri değişir, hızı aynı)
    let w_d = if mode == Mod::Fiske { TQ.flick_w * taraf(p0) } else { omega(p0, false, yavas) };
    let mut t = 0.0f64;
    while t < 200.0 && res.is_none() {
        px += p0.vx;
        py += p0.vy;
        done += jmin(w_d, a - done);
        let an = a0 + dir * done;
        let cx = px + cos(an) * body;
        let cy = py + sin(an) * body;
        let fly = mode == Mod::Fiske && done < a;
        if mode == Mod::Fiske && done >= a && !landed {
            landed = true;
            let m = flick_miss(p0);
            match rnd.as_mut() {
                Some(r) => {
                    if r() < m {
                        res = Some(DonusSonuc::FiskeKacti);
                    }
                }
                None => keep *= 1.0 - m,
            }
        }
        for (i, o) in opps0.iter().enumerate() {
            if res.is_some() {
                break;
            }
            let ok = o.govde.yet.ozk("tepki");
            if t > TQ.react - ok * TQ.react_ok {
                let dx = cx - ox[i];
                let dy = cy - oy[i];
                let l = hyp(dx, dy);
                if l > 0.8 * r {
                    let v = jmin(TQ.riv_v, l - 0.8 * r);
                    ox[i] += dx / l * v;
                    oy[i] += dy / l * v;
                }
                let pd = {
                    let h = hyp(ox[i] - px, oy[i] - py);
                    if h == 0.0 || h != h {
                        1.0
                    } else {
                        h
                    }
                };
                if pd < TQ.min_d {
                    ox[i] = px + (ox[i] - px) / pd * TQ.min_d;
                    oy[i] = py + (oy[i] - py) / pd * TQ.min_d;
                }
            }
            let d = hyp(cx - ox[i], cy - oy[i]);
            if t >= TQ.grab && d < r && (fly || !shielded(px, py, ox[i], oy[i], cx, cy)) && t >= ocd[i] {
                ocd[i] = t + if fly { TQ.cd_loose } else { TQ.cd_hold };
                let pp = if fly { TQ.p_loose } else { kis(TQ.p_hold + (o.govde.yet.ozk("kesme") - sur) * TQ.p_hold_sk, 0.05, 0.8) };
                match rnd.as_mut() {
                    Some(r) => {
                        if r() < pp {
                            res = Some(DonusSonuc::RakipAldi);
                        }
                    }
                    None => keep *= 1.0 - pp,
                }
            }
        }
        if res.is_none() && done >= a && t_end.is_none() {
            t_end = Some(t);
        }
        if res.is_none() {
            if let Some(te) = t_end {
                if t - te >= TQ.tail {
                    res = Some(DonusSonuc::Dondu);
                }
            }
        }
        t += 1.0;
    }
    (res.unwrap_or(DonusSonuc::Dondu), t_end, keep)
}

/// Dört yoldan (iki yön × dolaş/fiske) en ucuzunu seç (JS Turn.choose): rakipleri Görüş'üyle hatalı görür.
pub fn choose(p: &Oyuncu, a0: f64, a1: f64, opps: &[Oyuncu], rnd: &mut dyn FnMut() -> f64, yavas: f64) -> Plan {
    let ok = p.govde.yet.ozk("gorus");
    let err = 0.2 * jmax(0.0, 16.0 - ok);
    let lam = kis(0.3 + 0.08 * (ok - 5.0), 0.3, 1.1);
    let sd = short_dir(a0, a1);
    let seen: Vec<Oyuncu> = opps
        .iter()
        .map(|o| {
            let mut c = o.clone();
            c.x = o.x + (rnd() - 0.5) * err;
            c.y = o.y + (rnd() - 0.5) * err;
            c
        })
        .collect();
    let mut cs = [(Mod::Dolas, sd), (Mod::Dolas, -sd), (Mod::Fiske, sd), (Mod::Fiske, -sd)].map(|(m, d)| Plan { mode: m, dir: d, keep: 0.0, cost: 0.0 });
    for c in cs.iter_mut() {
        let (_, te, keep) = simulate(p, a0, a1, c.mode, c.dir, &seen, None, yavas);
        c.keep = keep;
        c.cost = te.unwrap_or(200.0) * TQ.time_k + lam * (1.0 - keep) + (rnd() - 0.5) * jmax(0.0, 16.0 - ok) * 0.02;
    }
    // JS sort kararlı: eşit maliyette ilk sıradaki
    let mut en = 0;
    for i in 1..4 {
        if cs[i].cost < cs[en].cost {
            en = i;
        }
    }
    cs[en]
}
