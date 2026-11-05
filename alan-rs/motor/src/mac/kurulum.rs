//! Kurulum (match.js 237–316, 717–1240): oyuncular, diziliş, roller (talimat paketleri), markaj, santra.
use super::tac::{Tac, BLOK, GEN, TAC0};
use super::*;
use crate::core_sabit::Q;
use crate::mat::{hypot, jmax, jmin};
use crate::model::{JPSlot, JPTl};
use crate::rng::Rng;
use crate::saha::{CY, H, MAC_LEN};
use serde::Deserialize;
use std::collections::BTreeMap;

/// Diziliş yeri (JS {d, l}).
#[derive(Clone, Copy, Debug, Default, Deserialize)]
pub struct Yer {
    #[serde(default)]
    pub d: f64,
    #[serde(default)]
    pub l: f64,
}
/// Diziliş (JS "3-2-1" ya da [{d, l}…]).
#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum DizV {
    Dizi(Vec<Yer>),
    Metin(String),
}
/// JS `m.diz[t]` = {sav, huc}
#[derive(Clone, Debug, Default, Deserialize)]
pub struct Diz {
    #[serde(default)]
    pub sav: Option<DizV>,
    #[serde(default)]
    pub huc: Option<DizV>,
}

/// Rol talimatı (JS roles[t][i] = {h, s, tl: {huc, sav}, eg}).
#[derive(Clone, Debug, Default, Deserialize)]
pub struct Rol {
    #[serde(default)]
    pub h: Option<String>,
    #[serde(default)]
    pub s: Option<String>,
    #[serde(default)]
    pub tl: Option<RolTl>,
    #[serde(default)]
    pub eg: Option<Eg>,
}
#[derive(Clone, Debug, Default, Deserialize)]
pub struct RolTl {
    #[serde(default)]
    pub huc: Option<BTreeMap<String, Option<f64>>>,
    #[serde(default)]
    pub sav: Option<BTreeMap<String, Option<f64>>>,
}
/// Oyuncu eğilimi (JS r.eg = {ps, rk})
#[derive(Clone, Copy, Debug, Default, Deserialize)]
pub struct Eg {
    #[serde(default)]
    pub ps: f64,
    #[serde(default)]
    pub rk: f64,
}
pub type Roller = Vec<Option<Vec<Option<Rol>>>>;

/// createMatch seçenekleri (JS o).
#[derive(Clone, Debug, Default)]
pub struct KurSecenek {
    pub tac: Option<Vec<Option<Tac>>>,
    pub len: Option<f64>,
    pub diz: Option<Vec<Option<Diz>>>,
    pub devre: Option<bool>,
    pub attrs: Option<Vec<Option<Vec<(String, HamV)>>>>,
    pub roles: Option<Roller>,
}

/// Kurulumun yerleşime sorduğu (shape.js slotBase, bekci). Sınamalarda kayıttan verilir.
pub trait KurDis {
    fn slotBase(&mut self, m: &Mac, p: Pid, ex: f64, ey: f64, att: bool) -> (f64, f64);
    fn bekci(&mut self, m: &Mac, p: Pid, ex: f64, ey: f64) -> (f64, f64);
}
/// Gerçek yerleşim (gecici_dis).
pub struct GeciciKur;
impl KurDis for GeciciKur {
    fn slotBase(&mut self, m: &Mac, p: Pid, ex: f64, ey: f64, att: bool) -> (f64, f64) {
        gecici_dis::slotBase(m, p, ex, ey, att)
    }
    fn bekci(&mut self, m: &Mac, p: Pid, ex: f64, ey: f64) -> (f64, f64) {
        gecici_dis::bekci(m, p, ex, ey)
    }
}

pub fn SLOTS() -> [JPSlot; 7] {
    let s = |r: &str, d: f64, l: f64| JPSlot { role: r.into(), d, l };
    [s("Bekçi", 0.0, 0.0), s("Bek", 0.2, -0.5), s("Bek", 0.2, 0.5), s("Orta", 0.45, -0.35), s("Orta", 0.45, 0.35), s("Kanat", 0.75, -0.85), s("Uç", 0.8, 0.25)]
}
pub const NAMES: [[&str; 7]; 2] = [["Aras", "Deniz", "Ece", "Kaan", "Mert", "Selin", "Tuna"], ["Bora", "Cem", "Duru", "Ilgaz", "Nehir", "Rüzgâr", "Yaz"]];

/// JS `{hiz: 10, …, taraf: 'sag'}` (createMatch'in varsayılan özellikleri, bu sırayla).
pub const A_ADLAR: [&str; 18] = ["hiz", "ceviklik", "guc", "uzanma", "alan", "kontrol", "surme", "pas", "gonderme", "yukleme", "tepki", "gorus", "yeralma", "sogukkanlilik", "kesme", "refleks", "yerlesim", "hakimiyet"];

/// Ham özelliklerden oyuncunun `a`sı (JS p.a) ve yetenek satırı.
pub fn yetKur(p: &mut Oyuncu, a: &HamA) {
    let mut j = crate::model::JPA::default();
    for (k, v) in &a.0 {
        let x = match v {
            HamV::S(x) => *x,
            HamV::M(s) => {
                if k == "taraf" {
                    j.taraf = s.clone();
                }
                continue;
            }
        };
        match k.as_str() {
            "hiz" => j.hiz = x,
            "ceviklik" => j.ceviklik = x,
            "guc" => j.guc = x,
            "uzanma" => j.uzanma = x,
            "alan" => j.alan = x,
            "kontrol" => j.kontrol = x,
            "surme" => j.surme = x,
            "pas" => j.pas = x,
            "gonderme" => j.gonderme = x,
            "yukleme" => j.yukleme = x,
            "tepki" => j.tepki = x,
            "gorus" => j.gorus = x,
            "yeralma" => j.yeralma = x,
            "sogukkanlilik" => j.sogukkanlilik = x,
            "kesme" => j.kesme = x,
            "refleks" => j.refleks = x,
            "yerlesim" => j.yerlesim = x,
            "hakimiyet" => j.hakimiyet = x,
            "enerji" => j.enerji = x,
            "govde" => j.govde = x,
            "caliskanlik" => j.caliskanlik = x,
            _ => {}
        }
    }
    p.govde_mut().a = j;
    p.govde_mut().yet = std::sync::Arc::new(crate::yetenek::Yetenek::kur(&a.ham()));
}

/// JS createMatch(seed, o) (yerleşim gerçek: gecici_dis).
pub fn createMatch(seed: f64, o: &KurSecenek) -> Mac {
    createMatch_ile(seed, o, &mut GeciciKur)
}

pub fn createMatch_ile(seed: f64, o: &KurSecenek, dis: &mut dyn KurDis) -> Mac {
    let mut m = Mac::default();
    m.r = Rng::yeni(if dogru(seed) { seed as i64 as u32 } else { 1 });
    m.seed = seed;
    m.tick = 0.0;
    m.len = match o.len {
        Some(l) if dogru(l) => l,
        _ => MAC_LEN,
    };
    m.devre = o.devre.unwrap_or(true);
    m.devre2 = false;
    m.score = vec![0.0, 0.0];
    m.ch = 0.3;
    m.decT = 0.0;
    m.tac = (0..2).map(|t| {
        let x = o.tac.as_ref().and_then(|v| v.get(t).cloned().flatten()).unwrap_or_default();
        TAC0().ust(&x)
    }).collect();
    let z2 = || vec![0.0, 0.0];
    m.st = crate::model::JMSt { duel: z2(), duelW: z2(), pass: z2(), passOk: z2(), shot: z2(), steal: z2(), poss: z2(), bank: z2(), lead: z2(), loose: z2(), ..Default::default() };
    let sl = SLOTS();
    for t in 0..2 {
        for (i, s) in sl.iter().enumerate() {
            let mut a = HamA::default();
            for k in A_ADLAR {
                a.koy(k, HamV::S(10.0));
            }
            a.koy("taraf", HamV::M("sag".into()));
            if let Some(Some(at)) = o.attrs.as_ref().and_then(|v| v.get(t)) {
                for (k, v) in at {
                    a.koy(k, v.clone());
                }
            }
            let bek = &*s.role == "Bekçi";
            let mut p = Oyuncu { id: (10 * t + i) as f64, team: t as f64, govde: std::sync::Arc::new(crate::model::OyuncuGovde { i: i as f64, role: s.role.clone(), slot: s.clone(), name: NAMES[t][i].into(), ..Default::default() }), R: if bek { 7.0 } else { 8.0 }, D: if bek { 1.2 } else { 1.0 }, x: W / 2.0, y: CY, vx: 0.0, vy: 0.0, tx: W / 2.0, ty: CY, cd: 0.0, kim: crate::model::Kim::oyuncu((10 * t + i) as f64), ..Default::default() };
            yetKur(&mut p, &a);
            m.ps.push(p);
            m.mac.ham.push(a);
        }
    }
    if let Some(r) = &o.roles {
        assignRoles_ile(&mut m, r, dis);
    }
    if let Some(dz) = &o.diz {
        for t in 0..2 {
            if let Some(Some(d)) = dz.get(t) {
                dizUygula_ile(&mut m, t as f64, d, dis);
            }
        }
    }
    kickoff_ile(&mut m, 0.0, dis);
    m
}

/// JS dizSlots("3-2-1")
pub fn dizSlots(s: &str) -> Vec<Yer> {
    let rows: Vec<usize> = s.split('-').map(|x| x.trim().parse::<f64>().unwrap_or(f64::NAN)).filter(|&n| n > 0.0).map(|n| n as usize).collect();
    let r = rows.len();
    let ls = |n: usize| -> &'static [f64] {
        match n {
            1 => &[0.0],
            2 => &[-0.5, 0.5],
            3 => &[-0.8, 0.0, 0.8],
            4 => &[-0.9, -0.3, 0.3, 0.9],
            5 => &[-0.9, -0.45, 0.0, 0.45, 0.9],
            _ => &[-0.8, 0.0, 0.8],
        }
    };
    let mut out = vec![];
    for (k, &n) in rows.iter().enumerate() {
        let d = if r > 1 { 0.2 + 0.6 * k as f64 / (r - 1) as f64 } else { 0.45 };
        for &l in ls(n) {
            out.push(Yer { d, l });
        }
    }
    out
}

pub fn dizUygula(m: &mut Mac, t: f64, dz: &Diz) {
    dizUygula_ile(m, t, dz, &mut GeciciKur)
}

pub fn dizUygula_ile(m: &mut Mac, t: f64, dz: &Diz, dis: &mut dyn KurDis) {
    let sl = |v: &Option<DizV>| -> Option<Vec<Yer>> {
        match v {
            Some(DizV::Dizi(a)) => Some(a.clone()),
            Some(DizV::Metin(s)) if !s.is_empty() => Some(dizSlots(s)),
            _ => None,
        }
    };
    let sv = sl(&dz.sav);
    let hu = sl(&dz.huc);
    for p in m.ps.iter_mut() {
        if p.team != t || p.bekci() {
            continue;
        }
        let k = p.govde.i as i64 - 1;
        if let Some(s0) = &p.govde.slot0 {
            p.govde_mut().slot = JPSlot { role: s0.role.clone(), d: s0.d, l: s0.l };
            p.govde_mut().rslot = p.govde.rslot0.as_ref().map(|r| crate::model::JPRslot { d: r.d, l: r.l });
        }
        if let Some(y) = sv.as_ref().and_then(|v| v.get(k as usize)) {
            p.govde_mut().slot = JPSlot { role: p.govde.slot.role.clone(), d: y.d, l: y.l };
        }
        if let Some(y) = hu.as_ref().and_then(|v| v.get(k as usize)) {
            p.govde_mut().rslot = Some(crate::model::JPRslot { d: y.d, l: y.l });
        }
        p.govde_mut().slot0 = Some(crate::model::JPSlot0 { role: p.govde.slot.role.clone(), d: p.govde.slot.d, l: p.govde.slot.l });
        p.govde_mut().rslot0 = p.govde.rslot.as_ref().map(|r| crate::model::JPRslot0 { d: r.d, l: r.l });
    }
    let d = m.diz.get_or_insert_with(|| vec![None, None]);
    d[t as usize] = Some(dz.clone());
    if let Some(e) = m.evre.as_mut() {
        e[t as usize] = None;
    }
    markAta_ile(m, dis);
}

/// JS TLV: talimat varsayılanları (null = yok).
pub fn TLV(ev: &str, k: &str) -> Option<f64> {
    match (ev, k) {
        ("huc", "kosu" | "istek" | "destek" | "risk" | "ritim" | "tempo" | "gonder" | "serbest") => Some(0.5),
        ("sav", "adam") => Some(0.0),
        ("sav", "pres") => Some(0.5),
        _ => None,
    }
}
pub const TLV_HUC: [&str; 10] = ["kosu", "istek", "destek", "risk", "ritim", "tempo", "gonder", "serbest", "doldur", "bindir"];
pub const TLV_SAV: [&str; 3] = ["adam", "kimi", "pres"];

/// JS PAKET: rol paketleri.
pub fn PAKET(ev: &str, ad: &str) -> Vec<(&'static str, f64)> {
    match (ev, ad) {
        ("h", "Kurucu") => vec![("destek", 0.2), ("serbest", 0.0), ("kosu", 0.3)],
        ("h", "Kanat") => vec![("serbest", 0.21)],
        ("h", "Koşucu") => vec![("kosu", 1.0), ("serbest", 0.76)],
        ("h", "Pivot") => vec![("serbest", 0.21)],
        ("s", "Markajcı") => vec![("adam", 1.0)],
        ("s", "Presçi") => vec![("pres", 1.0)],
        _ => vec![],
    }
}

/// JS tl(p, ev, k): oyuncunun talimatı, yoksa varsayılan.
pub fn tl(p: &Oyuncu, ev: &str, k: &str) -> Option<f64> {
    if let Some(t) = &p.govde.tl {
        let o = if ev == "huc" { &t.huc } else { &t.sav };
        if let Some(v) = o.get(k) {
            return Some(*v);
        }
    }
    TLV(ev, k)
}
/// tl(...) sayı olarak (JS'te null sayı işleminde 0).
pub fn tlv(p: &Oyuncu, ev: &str, k: &str) -> f64 {
    tl(p, ev, k).unwrap_or(0.0)
}

/// JS tlKur(r)
pub fn tlKur(r: &Rol) -> JPTl {
    let mut t = JPTl::default();
    for (k, v) in PAKET("h", r.h.as_deref().unwrap_or("")) {
        t.huc.insert(k.into(), v);
    }
    for (k, v) in PAKET("s", r.s.as_deref().unwrap_or("")) {
        t.sav.insert(k.into(), v);
    }
    if let Some(rt) = &r.tl {
        // JS {...paket, ...h}: null değer de yazılır (null → anahtar yok sayılır: tl() null'ı varsayılana bırakır)
        for (src, dst) in [(&rt.huc, &mut t.huc), (&rt.sav, &mut t.sav)] {
            if let Some(h) = src {
                for (k, v) in h {
                    match v {
                        Some(x) => {
                            dst.insert(k.clone(), *x);
                        }
                        None => {
                            dst.remove(k);
                        }
                    }
                }
            }
        }
    }
    t
}

/// JS RSLOT (rol yerleri; bugün kullanılmıyor, dışa açık)
pub fn RSLOT(ad: &str, s: &JPSlot) -> Option<(f64, f64)> {
    match ad {
        "Kurucu" => Some((0.1, 0.4 * s.l)),
        "Kanat" => Some((jmax(s.d, 0.55), if s.l < 0.0 { -1.0 } else { 1.0 })),
        "Koşucu" => Some((0.8, s.l)),
        "Pivot" => Some((0.85, 0.0)),
        _ => None,
    }
}

/// JS markAta: adam adama oynayanlara en ucuz eşleme (bütün eşlemeler denenir).
pub fn markAta(m: &mut Mac) {
    markAta_ile(m, &mut GeciciKur)
}
pub fn markAta_ile(m: &mut Mac, dis: &mut dyn KurDis) {
    for t in [0.0, 1.0] {
        let d: Vec<Pid> = (0..m.ps.len()).filter(|&i| m.ps[i].team == t && !m.ps[i].bekci() && tlv(&m.ps[i], "sav", "adam") >= 0.5).collect();
        let o: Vec<Pid> = (0..m.ps.len()).filter(|&i| m.ps[i].team != t && !m.ps[i].bekci()).collect();
        if d.is_empty() || o.is_empty() {
            continue;
        }
        let dir = if t == 0.0 { 1.0 } else { -1.0 };
        let ex = (if t == 0.0 { 0.0 } else { W }) + 30.0 * dir * KX;
        let ey = CY;
        let dp: Vec<(f64, f64)> = d.iter().map(|&p| dis.slotBase(m, p, ex, ey, false)).collect();
        let op: Vec<(f64, f64)> = o.iter().map(|&q| dis.slotBase(m, q, ex, ey, true)).collect();
        let cost: Vec<Vec<f64>> = dp.iter().map(|a| op.iter().map(|b| hypot(a.0 - b.0, a.1 - b.1)).collect()).collect();
        let mut best: Option<Vec<usize>> = None;
        let mut bc = 1e9;
        let mut used = vec![false; o.len()];
        let mut cur = vec![];
        fn ara(i: usize, c: f64, n: usize, cost: &[Vec<f64>], used: &mut [bool], cur: &mut Vec<usize>, bc: &mut f64, best: &mut Option<Vec<usize>>) {
            if c >= *bc {
                return;
            }
            if i == n {
                *bc = c;
                *best = Some(cur.clone());
                return;
            }
            for j in 0..used.len() {
                if !used[j] {
                    used[j] = true;
                    cur.push(j);
                    ara(i + 1, c + cost[i][j], n, cost, used, cur, bc, best);
                    cur.pop();
                    used[j] = false;
                }
            }
        }
        ara(0, 0.0, d.len(), &cost, &mut used, &mut cur, &mut bc, &mut best);
        if let Some(b) = best {
            for (i, &p) in d.iter().enumerate() {
                m.ps[p].hal_mut().markRef = Some(o[b[i]]);
            }
        }
    }
}

pub fn assignRoles(m: &mut Mac, roles: &Roller) {
    assignRoles_ile(m, roles, &mut GeciciKur)
}

pub fn assignRoles_ile(m: &mut Mac, roles: &Roller, dis: &mut dyn KurDis) {
    let rol = |roles: &Roller, t: usize, i: f64| -> Option<Rol> { roles.get(t).cloned().flatten().and_then(|r| r.get((i as i64 - 1) as usize).cloned().flatten()) };
    for t in 0..2 {
        if roles.get(t).map_or(true, |r| r.is_none()) {
            continue;
        }
        for p in m.ps.iter_mut() {
            if p.team != t as f64 || p.bekci() {
                continue;
            }
            if let Some(r) = rol(roles, t, p.govde.i) {
                p.govde_mut().rh = Some(r.h.clone().filter(|s| !s.is_empty()).unwrap_or_else(|| "Serbest".into()).into());
                p.govde_mut().rs = Some(r.s.clone().filter(|s| !s.is_empty()).unwrap_or_else(|| "Bölge".into()).into());
                p.govde_mut().tl = Some(std::sync::Arc::new(tlKur(&r)));
            }
        }
    }
    for t in 0..2 {
        if roles.get(t).map_or(true, |r| r.is_none()) {
            continue;
        }
        let ps: Vec<Pid> = (0..m.ps.len()).filter(|&i| m.ps[i].team == t as f64 && !m.ps[i].bekci()).collect();
        let mut ort = JPTl::default();
        for (ev, ks) in [("huc", &TLV_HUC[..]), ("sav", &TLV_SAV[..])] {
            for &k in ks {
                if k == "kimi" {
                    continue;
                }
                let (mut s0, mut n0, mut var0) = (0.0, 0.0, false);
                for &pi in &ps {
                    let r = rol(roles, t, m.ps[pi].govde.i);
                    let v = r.as_ref().and_then(|r| r.tl.as_ref()).and_then(|tl| if ev == "huc" { tl.huc.as_ref() } else { tl.sav.as_ref() }).and_then(|o| o.get(k).cloned().flatten());
                    match v {
                        Some(v) => {
                            s0 += v;
                            var0 = true;
                        }
                        None => match TLV(ev, k) {
                            None => continue,
                            Some(d) => s0 += d,
                        },
                    }
                    n0 += 1.0;
                }
                if var0 && n0 != 0.0 {
                    let o = if ev == "huc" { &mut ort.huc } else { &mut ort.sav };
                    o.insert(k.into(), s0 / n0);
                }
            }
        }
        for &pi in &ps {
            let p = &mut m.ps[pi];
            p.govde_mut().rh = Some("Serbest".into());
            p.govde_mut().rs = Some("Bölge".into());
            p.govde_mut().tl = Some(std::sync::Arc::new(ort.clone()));
        }
    }
    for t in [0.0, 1.0] {
        let mut taken: Vec<Pid> = vec![];
        let ds: Vec<Pid> = (0..m.ps.len()).filter(|&i| m.ps[i].team == t && !m.ps[i].bekci() && tlv(&m.ps[i], "sav", "adam") >= 0.5).collect();
        for p in ds {
            let ki = tl(&m.ps[p], "sav", "kimi");
            let sec = ki.and_then(|ki| (0..m.ps.len()).find(|&q| m.ps[q].team != t && m.ps[q].govde.i == ki && !m.ps[q].bekci() && !taken.contains(&q)));
            let opp: Vec<Pid> = match sec {
                Some(s) => vec![s],
                None => {
                    let mut v: Vec<Pid> = (0..m.ps.len()).filter(|&q| m.ps[q].team != t && !m.ps[q].bekci() && !taken.contains(&q)).collect();
                    let rs = |q: &Oyuncu| q.govde.rslot.as_ref().map_or(q.govde.slot.d, |r| r.d);
                    v.sort_by(|&u, &w| {
                        let a = tlv(&m.ps[w], "huc", "kosu") - tlv(&m.ps[u], "huc", "kosu");
                        let a = if a != 0.0 && a == a { a } else { rs(&m.ps[w]) - rs(&m.ps[u]) };
                        a.partial_cmp(&0.0).unwrap_or(std::cmp::Ordering::Equal)
                    });
                    v
                }
            };
            if let Some(&o0) = opp.first() {
                taken.push(o0);
                m.ps[p].hal_mut().markRef = Some(o0);
            }
        }
    }
    markAta_ile(m, dis);
    m.ra = Some(vec![Default::default(), Default::default()]);
}

/// JS roleSample: rol uyumu sayacı (her 10 tikte).
pub fn roleSample(m: &mut Mac) {
    let Some(h) = m.holder else { return };
    if m.tick % 10.0 != 0.0 {
        return;
    }
    let (ex, hteam) = (m.ps[h].x, m.ps[h].team);
    for pi in 0..m.ps.len() {
        let p = &m.ps[pi];
        if p.govde.tl.is_none() || pi == h {
            continue;
        }
        let t = p.team;
        let dir = dirOf(t);
        let atk = hteam == t;
        let rsl = p.govde.rslot.as_ref().map_or((p.govde.slot.d, p.govde.slot.l), |r| (r.d, r.l));
        let mut k: Option<&str>;
        let mut ok = false;
        if atk {
            k = Some("h");
            if rsl.1.abs() >= 0.75 {
                ok = (p.y - CY).abs() >= 13.0 * KY;
            } else if tlv(p, "huc", "destek") < 0.35 {
                ok = (p.x - ex) * dir < -2.0;
            } else if tlv(p, "huc", "kosu") >= 0.75 {
                let ox = W - ownX(t);
                let mut last: Option<f64> = None;
                for q in &m.ps {
                    if q.team != t && !q.bekci() && last.map_or(true, |l| (q.x - ox).abs() < (l - ox).abs()) {
                        last = Some(q.x);
                    }
                }
                ok = last.map_or(false, |l| (p.x - l) * dir >= -3.0);
            } else if rsl.0 >= 0.8 && rsl.1.abs() < 0.3 {
                if (ex - W / 2.0) * dir <= 0.0 {
                    k = None;
                } else {
                    ok = hypot(p.x - (W - ownX(t)), p.y - CY) < 28.0 && (p.y - CY).abs() < 12.0;
                }
            } else {
                k = None;
            }
        } else {
            k = Some("s");
            if p.govde.slot.d <= 0.25 {
                let mut deep = 1e9;
                for q in &m.ps {
                    if q.team == t && !q.bekci() {
                        deep = jmin(deep, (q.x - ownX(t)).abs());
                    }
                }
                ok = (p.x - ownX(t)).abs() <= deep + 3.0;
            } else if tlv(p, "sav", "pres") >= 0.75 {
                ok = p.press;
            } else if tlv(p, "sav", "adam") >= 0.5 {
                ok = p.hal.markRef.map_or(false, |r| hypot(p.x - m.ps[r].x, p.y - m.ps[r].y) < 6.0);
            } else if p.govde.slot.d >= 0.7 {
                ok = (p.x - W / 2.0) * dir > 0.0;
            } else {
                k = None;
            }
        }
        if let Some(k) = k {
            let (ti, i) = (t as usize, p.govde.i as usize);
            let ra = m.ra.as_mut().unwrap();
            let a = ra[ti].entry(i).or_insert_with(|| crate::model::JMRaEE { h: vec![0.0, 0.0], s: vec![0.0, 0.0] });
            let v = if k == "h" { &mut a.h } else { &mut a.s };
            v[0] += 1.0;
            if ok {
                v[1] += 1.0;
            }
        }
    }
}

/// JS kickoff(m, t): t takımı santra yapar.
pub fn kickoff(m: &mut Mac, t: f64) {
    kickoff_ile(m, t, &mut GeciciKur)
}

pub fn kickoff_ile(m: &mut Mac, t: f64, dis: &mut dyn KurDis) {
    kafa_bag::kafaBitir(m, kafa_bag::Son { santra: true, ..Default::default() });
    for p in m.ps.iter_mut() {
        p.en = Some(jmin(1.0, p.en.unwrap_or(1.0) + enerji::EN.santra));
    }
    for pi in 0..m.ps.len() {
        let tq = tac::tq0(m, m.ps[pi].team);
        let p = &mut m.ps[pi];
        let huc = p.team == t;
        let s = if huc { p.govde.rslot0.as_ref().map(|r| (r.d, r.l)).or(p.govde.rslot.as_ref().map(|r| (r.d, r.l))) } else { None };
        let s = s.or(p.govde.slot0.as_ref().map(|r| (r.d, r.l))).unwrap_or((p.govde.slot.d, p.govde.slot.l));
        let x0 = ownX(p.team);
        let dir = dirOf(p.team);
        let bl = match tq.blok.as_deref() {
            Some("Düşük") => [8.0, 24.0],
            Some("Yüksek") => [13.0, 33.0],
            _ => [10.0, 30.0],
        };
        let d = if p.bekci() { 3.0 } else { (if huc { jmin(46.0, 12.0 + 34.0 * s.0) } else { jmin(46.0, bl[0] + s.0 * bl[1]) }) * KX };
        p.x = x0 + dir * d;
        p.y = CY + s.1 * (if huc { 20.0 } else { 16.0 }) * KY;
        p.vx = 0.0;
        p.vy = 0.0;
        p.tx = p.x;
        p.ty = p.y;
        p.hal_mut().driveTo = None;
        p.hal_mut()._dur = None;
    }
    m.winT = None;
    m.gecis = false;
    m.gc = None;
    let k = (0..m.ps.len()).find(|&i| m.ps[i].team == t && m.ps[i].bekci()).or_else(|| (0..m.ps.len()).find(|&i| m.ps[i].team == t && m.ps[i].govde.i == 3.0)).unwrap();
    {
        let kp = &mut m.ps[k];
        kp.x = ownX(t) + 4.0 * dirOf(t);
        kp.y = CY;
        kp.tx = kp.x;
        kp.ty = kp.y;
    }
    m.holder = Some(k);
    if m.ps[k].bekci() {
        m.ps[k].hal_mut()._korT = Some(m.tick + Q.bek_koru);
    }
    m.ball = None;
    m.ch = 0.3;
    m.decT = 30.0;
    m.fl = None;
    m.sh = Some(Default::default());
    let (ex, ey) = (m.ps[k].x, m.ps[k].y);
    for pi in 0..m.ps.len() {
        if pi == k {
            continue;
        }
        let b = if m.ps[pi].bekci() { dis.bekci(m, pi, ex, ey) } else { dis.slotBase(m, pi, ex, ey, m.ps[pi].team == t) };
        if b.0.is_finite() && b.1.is_finite() {
            let p = &mut m.ps[pi];
            p.x = jmax(0.8, jmin(W - 0.8, b.0));
            p.y = jmax(0.8, jmin(H - 0.8, b.1));
            // outOfKuyu: KUYU açıkken bir şey yapmaz
            if p.team != t {
                let kg = ownX(t);
                let dx = p.x - kg;
                let dy = p.y - CY;
                let d = hypot(dx, dy);
                let r = Q.kuyu_a + 1.0;
                if d < r {
                    let ux = if d > 1e-6 { dx / d } else { dirOf(t) };
                    let uy = if d > 1e-6 { dy / d } else { 0.0 };
                    p.x = jmax(0.8, jmin(W - 0.8, kg + ux * r));
                    p.y = jmax(0.8, jmin(H - 0.8, CY + uy * r));
                }
            }
            p.tx = p.x;
            p.ty = p.y;
        }
    }
    ikili::govdeAyir(&mut m.ps, None, &mut ikili::IkBag::default());
}

/// JS slotPos (eski yerleşimin yeri; dışa açık).
pub fn slotPos(m: &Mac, p: &Oyuncu, bx: f64, by: f64, att: bool) -> (f64, f64) {
    let t = p.team;
    let tac = &m.tac[t as usize];
    let s = &p.govde.slot;
    let x0 = ownX(t);
    let dir = dirOf(t);
    let bd = (bx - x0) * dir;
    if &*s.role == "Bekçi" {
        let (gx, gy) = (x0, CY);
        let (dx, dy) = (bx - gx, by - gy);
        let l = { let l = hypot(dx, dy); if dogru(l) { l } else { 1.0 } };
        let r = jmin(6.0, 0.22 * l);
        return (gx + dx / l * r, gy + dy / l * r);
    }
    let (back, mut len, wid, sh);
    if att {
        back = cl(bd - 18.0 * KX, 12.0 * KX, 62.0 * KX);
        len = 48.0 * KX;
        wid = GEN(tac.genislik.as_deref().unwrap_or("")).unwrap_or(38.0 * KY);
        sh = 0.15;
    } else {
        back = cl(jmin(BLOK(tac.blok.as_deref().unwrap_or("")).unwrap_or(34.0 * KX), bd - 4.0), 9.0 * KX, 62.0 * KX);
        len = 24.0 * KX;
        wid = 26.0 * KY;
        sh = 0.4;
        let front = bd - 7.0;
        if back + len > front {
            len = jmax(6.0, front - back);
        }
    }
    let mut depth = back + (s.d - 0.2) / 0.6 * len;
    if !att {
        depth = jmin(depth, jmax(back, bd - 7.0));
    }
    (cl(x0 + dir * depth, 2.0, W - 2.0), cl(CY + s.l * wid / 2.0 + (by - CY) * sh, 2.0, H - 2.0))
}
