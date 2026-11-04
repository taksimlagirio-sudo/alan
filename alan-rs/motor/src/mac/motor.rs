//! Maç motoru (oyun arayüzü `MacMotoru`): kayıttan kurulan maçı tik tik oynatır.
#![allow(non_snake_case)]

use crate::model::Mac;
use crate::oyun::{bar_ort, IsSozluk, IzSatiri, Kare, MacMotoru};

pub struct Motor {
    pub m: Mac,
    /// son görülen olay (yeni olayları bulmak için; JS m._sonOlay)
    son_olay: Option<(String, String, String)>,
    ev_say: usize,
    son_fl: Option<(f64, String)>,
    h0: Option<usize>,
}

impl Motor {
    pub fn yeni(m: Mac) -> Motor {
        let ev_say = m.events.len();
        Motor { m, ev_say, son_fl: None, h0: None, son_olay: None }
    }
}

impl MacMotoru for Motor {
    fn kayittan(yeni: &serde_json::Value) -> Result<Self, String> {
        let k: super::kayit::Kayit = serde_json::from_value(yeni.get("yeni").cloned().unwrap_or_else(|| yeni.clone())).map_err(|e| e.to_string())?;
        Ok(Motor::yeni(super::kayit::macKur(&k)))
    }
    fn adim(&mut self) {
        self.h0 = self.m.holder;
        super::step(&mut self.m);
    }
    fn tick(&self) -> u32 {
        self.m.tick as u32
    }
    fn bitti(&self) -> bool {
        self.m.over
    }
    fn kare(&self, isler: &mut IsSozluk) -> Kare {
        let m = &self.m;
        let top = m.ball.as_ref().map(|b| (b.x, b.y));
        let hxy = m.holder.map(|h| (m.ps[h].x, m.ps[h].y));
        let ht = m.holder.map(|h| m.ps[h].team);
        Kare::kur(m.tick as u32, &m.ps, isler, top, m.holder, m.ch, [m.score[0], m.score[1]], [-1, -1], false, ht, hxy)
    }
    fn iz(&mut self) -> IzSatiri {
        let m = &self.m;
        let n = m.events.len();
        let yeni = n.saturating_sub(self.ev_say);
        let sut = m.events.iter().take(yeni).any(|e| e.r#type == "gonder");
        self.ev_say = n;
        let fl = m.fl.as_ref().map(|f| (f.t0, f.kind.clone()));
        let mut yeni_ucus = None;
        if let (Some(f), Some(fo)) = (&fl, m.fl.as_ref()) {
            if Some(f) != self.son_fl.as_ref() && !fo.kind.is_empty() {
                yeni_ucus = Some((fo.kind.clone(), self.h0.map_or(-1, |h| h as i32), fo.q.map_or(-1, |q| q as i32)));
            }
        }
        self.son_fl = fl;
        IzSatiri { tick: m.tick as u32, sahip: m.holder.map_or(-1, |h| h as i32), skor: [m.score[0] as u16, m.score[1] as u16], yeni_ucus, sut, bar: [bar_ort(&m.ps, 0.0), bar_ort(&m.ps, 1.0)] }
    }
}

fn olay_k(e: &super::Olay) -> (String, String, String) {
    (e.t.clone(), e.r#type.clone(), e.text.clone())
}

impl Motor {
    /// Son çağrıdan bu yana çıkan olaylar (eskiden yeniye). JS kaydet'in olay kısmı.
    pub fn yeni_olaylar(&mut self) -> Vec<super::Olay> {
        let ev = &self.m.events;
        let mut k = 0;
        while k < ev.len() && Some(olay_k(&ev[k])) != self.son_olay {
            k += 1;
        }
        let r: Vec<super::Olay> = ev[..k].iter().rev().cloned().collect();
        if let Some(e) = ev.first() {
            self.son_olay = Some(olay_k(e));
        }
        r
    }

    /// JS işçisinin karesi (mac-isci.js kaydet, 174 sayı): maç ekranı bununla çizer.
    pub fn kare174(&mut self, isler: &mut IsSozluk) -> [f32; 174] {
        // Çekirdeğin yeri kafaya sorulur: maçın ilk adımından önce de kafa hazır olsun
        crate::mac::kafa_dis::kafa_hazirla(&mut self.m);
        let nan = f32::NAN;
        let mut f = [0f32; 174];
        let m = &self.m;
        let ev = |e: &Option<crate::model::JMEvreE>| e.as_ref().and_then(|e| super::tac::EVKOD.iter().position(|k| *k == e.key)).map_or(-1.0, |i| i as f32);
        if let Some(E) = &m.evre {
            f[40] = E.first().map_or(-1.0, ev);
            f[41] = E.get(1).map_or(-1.0, ev);
        }
        match &m.mac.fake {
            Some(F) => {
                f[34] = m.ps.iter().position(|p| p.id == F.hid).map_or(-1.0, |i| i as f32);
                f[35] = if F.k == "gönder" { 1.0 } else if F.k == "pas" { 2.0 } else { 3.0 };
                f[36] = F.to.0 as f32;
                f[37] = F.to.1 as f32;
                let mut b = 0u32;
                for (k, p) in m.ps.iter().enumerate() {
                    if F.fooled.contains(&p.id) {
                        b |= 1 << k;
                    }
                }
                f[38] = b as f32;
                f[39] = ((m.tick - F.t0) / (F.until - F.t0).max(1.0)) as f32;
            }
            None => f[34] = -1.0,
        }
        let hold = m.holder.map(|h| &m.ps[h]);
        for (k, p) in m.ps.iter().enumerate() {
            if k >= 14 {
                break;
            }
            f[2 * k] = p.x as f32;
            f[2 * k + 1] = p.y as f32;
            f[42 + 3 * k] = isler.no(p.job.as_deref().unwrap_or("")) as f32;
            f[43 + 3 * k] = if p.tx.is_finite() { p.tx as f32 } else { nan };
            f[44 + 3 * k] = if p.ty.is_finite() { p.ty as f32 } else { nan };
            f[84 + k] = p.en.map_or(nan, |e| e as f32);
            f[98 + k] = if p.amA { p.am.unwrap_or(0.0) as f32 } else { 0.0 };
            f[113 + k] = p.den.map_or(nan, |e| e as f32);
            let mut ik = 0u8;
            if p.sendT.unwrap_or(0.0) > m.tick {
                ik |= 1;
            }
            if let Some(h) = hold {
                if h.team != p.team && crate::mat::hypot(p.vx, p.vy) < 0.06 && crate::mat::hypot(h.x - p.x, h.y - p.y) < 4.0 {
                    ik |= 2;
                }
            }
            f[127 + k] = ik as f32;
            f[157 + k] = match p._sf.as_ref().map(|s| s.f.as_str()) {
                Some("patlama") => 1.0,
                Some("tepe") => 2.0,
                Some("fren") => 3.0,
                Some("dolum") => 4.0,
                _ => 0.0,
            };
        }
        f[141] = nan;
        for q in 0..4 {
            f[145 + 3 * q] = nan;
            f[146 + 3 * q] = nan;
            f[147 + 3 * q] = nan;
        }
        f[112] = if hold.map_or(false, |h| h.sarj == Some(true)) { 1.0 } else { 0.0 };
        // itilmiş açık top: iten ve duracağı yer
        let it = match (&m.ball, &m.fl) {
            (Some(b), Some(fl)) if m.holder.is_none() && b.done.is_none() => match b.tt.t.from {
                Some(fr) if fl.q == Some(fr) && b.team as f64 == m.ps[fr].team => Some((fr, fl.end.x, fl.end.y)),
                _ => None,
            },
            _ => None,
        };
        f[171] = it.map_or(-1.0, |i| i.0 as f32);
        f[172] = it.map_or(nan, |i| i.1 as f32);
        f[173] = it.map_or(nan, |i| i.2 as f32);
        let c = super::hareket::corePos(m);
        f[28] = c.map_or(nan, |c| c.0 as f32);
        f[29] = c.map_or(nan, |c| c.1 as f32);
        f[30] = m.holder.map_or(-1.0, |h| h as f32);
        f[31] = match m.holder {
            Some(_) => m.ch as f32,
            None => m.ball.as_ref().map_or(0.0, |b| if crate::mat::dogru(b.tt.b.ch) { b.tt.b.ch as f32 } else { 0.0 }),
        };
        f[32] = m.score[0] as f32;
        f[33] = m.score[1] as f32;
        f
    }

    /// JS statik(m): oyuncuların değişmeyen bilgisi (JSON)
    pub fn statik_json(&self) -> String {
        let m = &self.m;
        let ps: Vec<String> = m
            .ps
            .iter()
            .enumerate()
            .map(|(k, p)| {
                let a = m.mac.ham.get(k).map(|h| {
                    let alan: Vec<String> = h
                        .0
                        .iter()
                        .map(|(ad, v)| {
                            let v = match v {
                                super::HamV::S(x) => if x.is_finite() { format!("{x}") } else { "null".into() },
                                super::HamV::M(s) => serde_json::to_string(s).unwrap(),
                            };
                            format!("{}:{}", serde_json::to_string(ad).unwrap(), v)
                        })
                        .collect();
                    format!("{{{}}}", alan.join(","))
                });
                format!(
                    "{{\"id\":{},\"name\":{},\"role\":{},\"team\":{},\"a\":{},\"R\":{},\"D\":{},\"i\":{}}}",
                    p.id,
                    serde_json::to_string(&*p.name).unwrap(),
                    serde_json::to_string(&*p.role).unwrap(),
                    p.team,
                    a.unwrap_or_else(|| "{}".into()),
                    p.R,
                    p.D,
                    p.i
                )
            })
            .collect();
        format!("[{}]", ps.join(","))
    }
}
