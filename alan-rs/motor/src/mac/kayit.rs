//! Kayıttan maç kurma (alan-v5/mac-isci.js: kurUygula, egUy, rastgele; `yeni` iletisinin kurulumu).
//! Kayıt: kayit-*.json → `yeni` = {tohum, tac, diz, len, devre, oz, kadro, oyuncular, roller}.
use super::kurulum::{assignRoles_ile, createMatch_ile, dizUygula_ile, yetKur, Diz, GeciciKur, KurDis, KurSecenek, Roller};
use super::*;
use serde::Deserialize;

/// Kaydın bir oyuncusu (JS {ad, a}).
#[derive(Clone, Debug, Default, Deserialize)]
pub struct KOyuncu {
    #[serde(default)]
    pub ad: Option<String>,
    #[serde(default)]
    pub a: serde_json::Map<String, serde_json::Value>,
}

/// JS `yeni` iletisi (kayit-*.json'un `yeni`si).
#[derive(Clone, Debug, Default, Deserialize)]
pub struct Kayit {
    #[serde(default)]
    pub tohum: f64,
    #[serde(default)]
    pub tac: Option<Vec<Option<Tac>>>,
    #[serde(default)]
    pub diz: Option<Vec<Option<Diz>>>,
    #[serde(default)]
    pub len: Option<f64>,
    #[serde(default)]
    pub devre: Option<bool>,
    /// "10" (kadronun kendi değerleri) ya da "rastgele"
    #[serde(default)]
    pub oz: Option<String>,
    #[serde(default)]
    pub kadro: Option<f64>,
    #[serde(default)]
    pub oyuncular: Option<Vec<Option<Vec<Option<KOyuncu>>>>>,
    #[serde(default)]
    pub roller: Option<Roller>,
}

impl Kayit {
    /// kayit-*.json metninden (`yeni` alanı ya da doğrudan ileti)
    pub fn json(s: &str) -> Result<Kayit, String> {
        let v: serde_json::Value = serde_json::from_str(s).map_err(|e| e.to_string())?;
        let y = v.get("yeni").cloned().unwrap_or(v);
        serde_json::from_value(y).map_err(|e| e.to_string())
    }
}

fn hamV(v: &serde_json::Value) -> Option<HamV> {
    match v {
        serde_json::Value::Number(n) => n.as_f64().map(HamV::S),
        serde_json::Value::String(s) => Some(HamV::M(s.clone())),
        _ => None,
    }
}

/// JS egUy(m, R): oyuncu eğilimleri
pub fn egUy(m: &mut Mac, r: &Roller) {
    for p in m.ps.iter_mut() {
        let e = r.get(p.team as usize).cloned().flatten().and_then(|v| v.get((p.i as i64 - 1) as usize).cloned().flatten()).and_then(|x| x.eg);
        if let Some(e) = e {
            p.eg = Some(crate::model::JPEg { ps: e.ps, rk: e.rk });
        }
    }
}

/// JS kurUygula(m, d): kadro (ad, özellikler), roller, diziliş.
pub fn kurUygula(m: &mut Mac, d: &Kayit, dis: &mut dyn KurDis) {
    if let Some(oy) = &d.oyuncular {
        for t in 0..2 {
            let Some(Some(a)) = oy.get(t) else { continue };
            for pi in 0..m.ps.len() {
                if m.ps[pi].team != t as f64 {
                    continue;
                }
                let Some(Some(o)) = a.get(m.ps[pi].i as usize) else { continue };
                if let Some(ad) = &o.ad {
                    m.ps[pi].name = ad.clone();
                }
                let mut h = m.mac.ham[pi].clone();
                for (k, v) in &o.a {
                    if let Some(v) = hamV(v) {
                        h.koy(k, v);
                    }
                }
                yetKur(&mut m.ps[pi], &h);
                m.mac.ham[pi] = h;
            }
        }
    }
    if let Some(r) = &d.roller {
        assignRoles_ile(m, r, dis);
        egUy(m, r);
    }
    if let Some(dz) = &d.diz {
        for t in 0..2 {
            if let Some(Some(z)) = dz.get(t) {
                dizUygula_ile(m, t as f64, z, dis);
            }
        }
    }
}

/// JS rastgele(m): kadronun bütün özellikleri 6–16 arası (kadro numarasıyla belirlenimli).
pub fn rastgele(m: &mut Mac, kadro: f64) {
    let mut r0: u32 = (kadro * 2654435761.0) as i64 as u32;
    let mut r = || {
        r0 = r0.wrapping_mul(1664525).wrapping_add(1013904223);
        r0 as f64 / 4294967296.0
    };
    for pi in 0..m.ps.len() {
        let mut h = m.mac.ham[pi].clone();
        for e in h.0.iter_mut() {
            e.1 = HamV::S(6.0 + (r() * 11.0).floor());
        }
        yetKur(&mut m.ps[pi], &h);
        m.mac.ham[pi] = h;
    }
}

/// Kayıttan maç (mac-isci.js `yeni`): createMatch → (rastgele) → kurUygula → kafaAc.
pub fn macKur(d: &Kayit) -> Mac {
    macKur_ile(d, &mut GeciciKur)
}
pub fn macKur_ile(d: &Kayit, dis: &mut dyn KurDis) -> Mac {
    let len = d.len.filter(|&l| dogru(l)).unwrap_or(5400.0);
    let o = KurSecenek { tac: d.tac.clone(), len: Some(len), diz: d.diz.clone(), devre: Some(d.devre.unwrap_or(false)), ..Default::default() };
    let mut m = createMatch_ile(d.tohum, &o, dis);
    if d.oz.as_deref() == Some("rastgele") {
        rastgele(&mut m, d.kadro.unwrap_or(0.0));
    }
    kurUygula(&mut m, d, dis);
    kafa_bag::kafaAc(&mut m);
    m
}
