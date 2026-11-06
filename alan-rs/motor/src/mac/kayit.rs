//! Kayıttan maç kurma (alan-v5/mac-isci.js: kurUygula, egUy, rastgele; `yeni` iletisinin kurulumu).
//! Kayıt: kayit-*.json → `yeni` = {tohum, tac, diz, len, devre, oz, kadro, oyuncular, roller}.
use super::kurulum::{assign_roles_ile, create_match_ile, diz_uygula_ile, yet_kur, Diz, GeciciKur, KurDis, KurSecenek, Roller};
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

fn ham_v(v: &serde_json::Value) -> Option<HamV> {
    match v {
        serde_json::Value::Number(n) => n.as_f64().map(HamV::S),
        serde_json::Value::String(s) => Some(HamV::M(s.clone())),
        _ => None,
    }
}

/// JS egUy(m, R): oyuncu eğilimleri
pub fn eg_uy(m: &mut Mac, r: &Roller) {
    for p in m.ps.iter_mut() {
        let e = r.get(p.team as usize).cloned().flatten().and_then(|v| v.get((p.govde.i as i64 - 1) as usize).cloned().flatten()).and_then(|x| x.eg);
        if let Some(e) = e {
            p.govde_mut().eg = Some(crate::model::JPEg { ps: e.ps, rk: e.rk });
        }
    }
}

/// JS kurUygula(m, d): kadro (ad, özellikler), roller, diziliş.
pub fn kur_uygula(m: &mut Mac, d: &Kayit, dis: &mut dyn KurDis) {
    if let Some(oy) = &d.oyuncular {
        for t in 0..2 {
            let Some(Some(a)) = oy.get(t) else { continue };
            for pi in 0..m.ps.len() {
                if m.ps[pi].team != t as f64 {
                    continue;
                }
                let Some(Some(o)) = a.get(m.ps[pi].govde.i as usize) else { continue };
                if let Some(ad) = &o.ad {
                    m.ps[pi].govde_mut().name = ad.as_str().into();
                }
                let mut h = m.mac.ham[pi].clone();
                for (k, v) in &o.a {
                    if let Some(v) = ham_v(v) {
                        h.koy(k, v);
                    }
                }
                yet_kur(&mut m.ps[pi], &h);
                m.mac.ham[pi] = h;
            }
        }
    }
    if let Some(r) = &d.roller {
        assign_roles_ile(m, r, dis);
        eg_uy(m, r);
    }
    if let Some(dz) = &d.diz {
        for t in 0..2 {
            if let Some(Some(z)) = dz.get(t) {
                diz_uygula_ile(m, t as f64, z, dis);
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
        yet_kur(&mut m.ps[pi], &h);
        m.mac.ham[pi] = h;
    }
}

/// Kayıttan maç (mac-isci.js `yeni`): createMatch → (rastgele) → kurUygula → kafaAc.
pub fn mac_kur(d: &Kayit) -> Mac {
    mac_kur_ile(d, &mut GeciciKur)
}
pub fn mac_kur_ile(d: &Kayit, dis: &mut dyn KurDis) -> Mac {
    let len = d.len.filter(|&l| dogru(l)).unwrap_or(5400.0);
    let o = KurSecenek { tac: d.tac.clone(), len: Some(len), diz: d.diz.clone(), devre: Some(d.devre.unwrap_or(false)), ..Default::default() };
    let mut m = create_match_ile(d.tohum, &o, dis);
    if d.oz.as_deref() == Some("rastgele") {
        rastgele(&mut m, d.kadro.unwrap_or(0.0));
    }
    kur_uygula(&mut m, d, dis);
    kafa_bag::kafa_ac(&mut m);
    m
}

/// Maç içi taktik değişikliği (mac-isci.js `degistir` iletisi / `gecmis` öğesi): snapT tikinde t takımına.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct Degisim {
    #[serde(default, rename = "snapT")]
    pub tick: f64,
    #[serde(default)]
    pub t: f64,
    #[serde(default)]
    pub tac: Option<Tac>,
    #[serde(default)]
    pub diz: Option<Diz>,
    #[serde(default)]
    pub roller: Option<Vec<Option<super::kurulum::Rol>>>,
}

/// JS uygula(m, c) (mac-isci.js): taktik o anki taktiğe yazılır; koşul kuralı etkinken (`_ev`) temel taktiğe de
/// (maç her tik taktiği temel taktik + etkin koşullardan yeniden kurar); roller ve diziliş.
pub fn uygula(m: &mut Mac, c: &Degisim) {
    uygula_ile(m, c, &mut GeciciKur)
}
pub fn uygula_ile(m: &mut Mac, c: &Degisim, dis: &mut dyn KurDis) {
    let t = c.t as usize;
    if let Some(tc) = &c.tac {
        if t < m.tac.len() {
            m.tac[t] = m.tac[t].ust(tc);
            if m.tac[t]._ev.as_deref().map_or(false, |s| !s.is_empty()) {
                if let Some(Some(b)) = m.tac0.get_mut(t) {
                    *b = b.ust(tc);
                }
            }
        }
    }
    if let Some(r) = &c.roller {
        let mut R: Roller = vec![None, None];
        R[t] = Some(r.clone());
        assign_roles_ile(m, &R, dis);
        eg_uy(m, &R);
    }
    if let Some(z) = &c.diz {
        diz_uygula_ile(m, c.t, z, dis);
    }
}
