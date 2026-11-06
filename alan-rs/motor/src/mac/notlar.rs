//! Oyuncu notları (match.js 4080–4132): topun değerindeki değişim taşıyana/vurana yazılır.
use super::*;
use crate::mat::{jmax, jmin};
use crate::model::JMNot;

pub const NOT_K: f64 = 3.0;
pub const NOT_0: f64 = 0.13;

fn ekle(n: &mut JMNot, id: f64, x: f64) {
    *n.p.entry(id as usize).or_insert(0.0) += x;
}

/// JS notTik(m)
pub fn not_tik(m: &mut Mac) {
    if m.mac._ic || m.mac._lite {
        return;
    }
    if m.not.is_none() {
        m.not = Some(JMNot::default());
    }
    let h = m.holder;
    let cp = super::hareket::core_pos(m);
    let n = m.not.as_ref().unwrap();
    let mut s = n.s;
    let team = match h {
        Some(h) => Some(m.ps[h].team),
        None => m.ball.as_ref().map(|b| b.team as f64).or(n.team),
    };
    if let (Some(cp), Some(team)) = (cp, team) {
        let v = gecici_dis::V(m, &m.ps, team, cp.0, cp.1, or0(m.ch));
        s = Some(if team == 0.0 { v } else { -v });
    }
    let actor = h.or_else(|| m.ball.as_ref().and_then(|b| b.tt.t.from)).or(n.actor);
    let sg = |p: Pid| if m.ps[p].team == 0.0 { 1.0 } else { -1.0 };
    let mut ek: Vec<(f64, f64)> = vec![];
    if let (Some(ns), Some(s)) = (n.s, s) {
        let d = s - ns;
        if n.team.is_some() && team.is_some() && team != n.team && h.is_some() {
            if let Some(a) = n.actor {
                ek.push((m.ps[a].id, sg(a) * d * 0.5));
            }
            let h = h.unwrap();
            ek.push((m.ps[h].id, sg(h) * d * 0.5));
        } else if let Some(a) = actor {
            ek.push((m.ps[a].id, sg(a) * d));
        }
    }
    let n = m.not.as_mut().unwrap();
    for (id, x) in ek {
        ekle(n, id, x);
    }
    n.s = s;
    n.team = team;
    if actor.is_some() {
        n.actor = actor;
    }
}

/// JS notSayi(m, t, atan): sayı olunca atana ve yenen Bekçi'ye
pub fn not_sayi(m: &mut Mac, t: f64, atan: Option<Pid>) {
    let bk = m.ps.iter().position(|p| p.team != t && p.bekci());
    let atan_id = atan.map(|a| m.ps[a].id);
    let bk_id = bk.map(|b| m.ps[b].id);
    let Some(n) = m.not.as_mut() else { return };
    let pre = match n.s {
        None => 0.0,
        Some(s) => {
            if t == 0.0 {
                s
            } else {
                -s
            }
        }
    };
    if let Some(a) = atan_id {
        ekle(n, a, 1.0 - pre);
    }
    if let Some(b) = bk_id {
        ekle(n, b, -0.5 * (1.0 - pre));
    }
    n.s = None;
    n.actor = None;
}

/// Bir oyuncunun maç notu (JS notlar(m)[i]).
#[derive(Clone, Debug)]
pub struct Not {
    pub id: f64,
    pub ad: String,
    pub team: f64,
    pub rol: String,
    pub puan: f64,
    pub not: f64,
}
pub fn notlar(m: &Mac) -> Vec<Not> {
    let bos = JMNot::default();
    let n = m.not.as_ref().unwrap_or(&bos);
    m.ps.iter().map(|p| {
        let pu = n.p.get(&(p.id as usize)).copied().unwrap_or(0.0);
        Not { id: p.id, ad: p.govde.name.to_string(), team: p.team, rol: p.govde.role.to_string(), puan: to_fixed(pu, 3), not: to_fixed(jmax(3.0, jmin(10.0, 6.3 + (pu - NOT_0) * NOT_K)), 1) }
    }).collect()
}
