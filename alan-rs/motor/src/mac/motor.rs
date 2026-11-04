//! Maç motoru (oyun arayüzü `MacMotoru`): kayıttan kurulan maçı tik tik oynatır.
#![allow(non_snake_case)]

use crate::model::Mac;
use crate::oyun::{bar_ort, IsSozluk, IzSatiri, Kare, MacMotoru};

pub struct Motor {
    pub m: Mac,
    ev_say: usize,
    son_fl: Option<(f64, String)>,
    h0: Option<usize>,
}

impl Motor {
    pub fn yeni(m: Mac) -> Motor {
        let ev_say = m.events.len();
        Motor { m, ev_say, son_fl: None, h0: None }
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
