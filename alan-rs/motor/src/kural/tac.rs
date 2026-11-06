//! Taktik (match.js 1–236, 894–938): takım talimatı (Tac), geçiş penceresi, karşı pres, maç durumu (macEff),
//! yetişme uyumu (yzUyum), evre.
use crate::mat::{dogru, hypot, js_num};
use crate::model::*;
use crate::oyuncu::oo;
use crate::saha::{dirOf, ownX, KX, KY, SN, W};
use crate::mat::{jmax, jmin, jround};
use crate::saha::CY;

pub use crate::mac_veri::{Durus, DurusN, Kosul, MacW, MacZ, Tac};

/// JS TAC0
pub fn TAC0() -> Tac {
    Tac {
        blok: Some("Orta".into()),
        pres: Some(1.0),
        onde: Some(0.0),
        genislik: Some("Normal".into()),
        risk: Some(0.5),
        tempo: Some(0.5),
        kazaninca: Some("Dengeli".into()),
        ..Default::default()
    }
}

pub const KP_N: usize = 3;
pub const KP_R: f64 = 16.0;

pub fn GECIS_VARS(tq: &Tac) -> String {
    if let Some(g) = tq.gecisH.as_ref().filter(|s| !s.is_empty()) {
        return g.clone();
    }
    match tq.kazaninca.as_deref() {
        Some("Kontra") => "esit".into(),
        Some("Yerleş") => "hic".into(),
        _ => "esit".into(),
    }
}
pub fn KP_VARS(tq: &Tac) -> String {
    if let Some(g) = tq.kpYer.as_ref().filter(|s| !s.is_empty()) {
        return g.clone();
    }
    let k = tq.kayip.unwrap_or(0.5);
    if k < 0.34 {
        "hic".into()
    } else if k > 0.66 {
        "her".into()
    } else {
        "rakip".into()
    }
}
pub fn gecisEsik(tq: &Tac) -> f64 {
    tq.gecisEsik.unwrap_or(4.0)
}

/// JS geriSay: lz takımının (Bekçi dışı) E'den kendi kalesine yakın oyuncuları.
pub fn geriSay(m: &Mac, lz: f64, ex: f64) -> f64 {
    let ox = ownX(lz);
    let mut n = 0.0;
    for q in &m.ps {
        if q.team == lz && !q.bekci() && (q.x - ox).abs() < (ex - ox).abs() {
            n += 1.0;
        }
    }
    n
}
/// JS onRef: t takımının (Bekçi dışı) en öndeki oyuncusu ya da E (yalnız x'i okunur).
pub fn onRef(m: &Mac, t: f64, ex: f64) -> f64 {
    let d = dirOf(t);
    let mut u = ex;
    for q in &m.ps {
        if q.team == t && !q.bekci() && (q.x - u) * d > 0.0 {
            u = q.x;
        }
    }
    u
}

/// JS `m.tac0 && m.tac0[t] || m.tac[t] || {}`
pub fn tq0(m: &Mac, t: f64) -> Tac {
    if let Some(Some(x)) = m.tac0.as_ref().map(|v| v[t as usize].clone()) {
        return x;
    }
    m.tac.get(t as usize).cloned().unwrap_or_default()
}

/// Topun takımı (JS `h ? h.team : b ? b.team : null`).
pub fn topTakim(m: &Mac) -> Option<f64> {
    if let Some(h) = m.holder {
        Some(m.ps[h].team)
    } else {
        m.ball.as_ref().map(|b| b.team as f64)
    }
}

pub fn gecisBasla(m: &mut Mac, pi: Pid) {
    let (pt, px, py) = (m.ps[pi].team, m.ps[pi].x, m.ps[pi].y);
    let lz = 1.0 - pt;
    let geri = geriSay(m, lz, onRef(m, pt, px));
    m.hhKul = false;
    m.gc = None;
    m.kp = None;
    {
        let tl = tq0(m, lz);
        let yer = KP_VARS(&tl);
        let rakYar = (px - ownX(lz)).abs() > W / 2.0;
        let kpOn = yer == "her" || (yer == "rakip" && rakYar);
        if kpOn {
            let mut c: Vec<(f64, f64)> = m.ps.iter().filter(|q| q.team == lz && !q.bekci() && hypot(q.x - px, q.y - py) < KP_R).map(|q| (hypot(q.x - px, q.y - py), q.id)).collect();
            c.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
            let ids: Vec<f64> = c.iter().take(KP_N).map(|x| x.1).collect();
            if !ids.is_empty() {
                m.kp = Some(crate::model::JMKp { team: lz, ids, bitis: m.tick + jround(tl.kpSure.unwrap_or(2.0) * SN()), sahip: None });
            }
        }
    }
    let tw = tq0(m, pt);
    if geri < gecisEsik(&tw) {
        m.gc = Some(crate::model::JMGc { team: pt, t0: m.tick, kendi: (px - ownX(pt)).abs() < 35.0 * KX, git: false, on: vec![], ..Default::default() });
        gecisGuncelle(m);
    }
    m.gecis = m.gc.is_some();
}

pub fn gecisGuncelle(m: &mut Mac) {
    if m.gc.is_none() && !m.hhKul {
        if let Some(h0) = m.holder.filter(|&h| !m.ps[h].bekci()) {
            let (t, hx) = (m.ps[h0].team, m.ps[h0].x);
            let dG0 = (hx - ownX(t)).abs();
            if dG0 < 65.0 * KX && geriSay(m, 1.0 - t, hx) < gecisEsik(&tq0(m, t)) {
                m.hhKul = true;
                m.gc = Some(crate::model::JMGc { team: t, t0: m.tick, kendi: dG0 < 35.0 * KX, git: false, on: vec![], hizli: true, ..Default::default() });
            }
        }
    }
    let Some(gteam) = m.gc.as_ref().map(|g| g.team) else {
        m.gecis = false;
        return;
    };
    let h = m.holder;
    if topTakim(m) != Some(gteam) {
        m.gc = None;
        m.gecis = false;
        return;
    }
    let ex = match h {
        Some(h) => m.ps[h].x,
        None => m.ball.as_ref().unwrap().x,
    };
    let lz = 1.0 - gteam;
    let geri = geriSay(m, lz, onRef(m, gteam, ex));
    if geri >= gecisEsik(&tq0(m, gteam)) || m.tick - m.gc.as_ref().unwrap().t0 > 4.0 * SN() {
        m.gc = None;
        m.gecis = false;
        return;
    }
    let tq = tq0(m, gteam);
    let kural = GECIS_VARS(&tq);
    let dir = dirOf(gteam);
    let mut on: Vec<Pid> = (0..m.ps.len()).filter(|&i| {
        let q = &m.ps[i];
        q.team == gteam && !q.bekci() && Some(i) != h && (q.x - ex) * dir > 0.0
    }).collect();
    let n = on.len() as f64;
    let nT = n + if h.is_some() { 1.0 } else { 0.0 };
    let git = kural == "her" || (kural == "esit" && nT >= geri) || (kural == "ustun" && nT > geri);
    let kendi = m.gc.as_ref().unwrap().kendi;
    if git && kendi {
        if let Some(h) = h {
            let (hx, hy) = (m.ps[h].x, m.ps[h].y);
            let mut c: Vec<(f64, Pid)> = (0..m.ps.len()).filter(|&i| {
                let q = &m.ps[i];
                q.team == gteam && !q.bekci() && i != h && !on.contains(&i)
            }).map(|i| (hypot(m.ps[i].x - hx, m.ps[i].y - hy), i)).collect();
            c.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
            if let Some(d) = c.first() {
                on.push(d.1);
            }
        }
    }
    let hid = h.map(|h| m.ps[h].id);
    let onid: Vec<f64> = on.iter().map(|&i| m.ps[i].id).collect();
    let g = m.gc.as_mut().unwrap();
    g.biz = nT;
    g.rakip = geri;
    g.git = git;
    if git {
        let mut v: Vec<f64> = Vec::new();
        for id in g.on.iter().copied().filter(|&id| hid.map_or(true, |hh| id != hh)).chain(onid) {
            if !v.contains(&id) {
                v.push(id);
            }
        }
        g.on = v;
    } else {
        g.on = vec![];
    }
    m.gecis = true;
}

pub fn inWin(m: &Mac, t: f64) -> bool {
    matches!(&m.gc, Some(g) if g.team == t && g.git)
}

/// JS kpAkt: karşı presin oyuncuları (t takımı karşı preste ise).
pub fn kpAkt(m: &mut Mac, t: f64) -> Option<Vec<f64>> {
    let k = m.kp.as_ref()?;
    let bt = topTakim(m);
    if m.tick >= k.bitis || bt == Some(k.team) {
        m.kp = None;
        return None;
    }
    if let Some(h) = m.holder {
        match k.sahip {
            Some(s) => {
                if h != s {
                    m.kp = None;
                    return None;
                }
            }
            None => m.kp.as_mut().unwrap().sahip = Some(h),
        }
    }
    let k = m.kp.as_ref().unwrap();
    if k.team == t {
        Some(k.ids.clone())
    } else {
        None
    }
}

pub fn kpEff(m: &mut Mac, t: f64, r: Tac) -> Tac {
    if kpAkt(m, t).is_some() {
        Tac { _kp: Some("pres".into()), ..r }
    } else {
        r
    }
}

pub fn yzUyum(m: &mut Mac, t: f64, r: Tac) -> Tac {
    if !m.tac.get(t as usize).map_or(false, |x| x.yz == Some(true)) {
        return r;
    }
    let ti = t as usize;
    let d = m.score[ti] - m.score[1 - ti];
    if d > 0.0 {
        return r;
    }
    if let Some(h) = m.holder.filter(|&h| m.ps[h].team == t) {
        let og = if t == 0.0 { W } else { 0.0 };
        let geri = m.ps.iter().filter(|p| p.team != t && !p.bekci() && (p.x - og).abs() < W / 2.0).count();
        let hp = &m.ps[h];
        let yakin = m.ps.iter().any(|p| p.team != t && !p.bekci() && hypot(p.x - hp.x, p.y - hp.y) < 5.0);
        if yakin {
            m._yzBas = Some(m.tick);
        }
        m._yzBekle = geri >= 5 && m.tick - oo(m._yzBas) > 4.0 * SN();
    }
    if !m._yzBekle {
        return r;
    }
    let len = if dogru(m.len) { m.len } else { crate::saha::MAC_LEN };
    let kal = len - m.tick;
    let tempo = jmin(0.9, r.tempo.unwrap_or(0.5) + 0.4);
    let sabir = jmax(0.1, r.sabir.unwrap_or(0.5) - if d < 0.0 { if kal < 30.0 * SN() { 0.5 } else { 0.35 } } else { 0.2 });
    Tac { tempo: Some(tempo), sabir: Some(sabir), ..r }
}

/// JS tacEff: talimat + geçiş + karşı pres + maç durumu + yetişme uyumu.
pub fn tacEff(m: &mut Mac, t: f64) -> Tac {
    let a = tacEff0(m, t);
    let b = kpEff(m, t, a);
    let c = macEff(m, t, b);
    yzUyum(m, t, c)
}

pub use crate::mac_veri::{fak, macW, MAC_ORAN, MAC_UC};

/// JS macEff: skor ve kalan süreye göre atılan/yenen golün oranı (macK) ve zamanın değeri (macZ).
pub fn macEff(m: &mut Mac, t: f64, tq: Tac) -> Tac {
    if !dogru(m.len) {
        return tq;
    }
    let ti = t as usize;
    let kal = m.len - m.tick;
    let d = m.score[ti] - m.score[1 - ti];
    let key = format!("{}:{}", js_num(d), js_num((kal / 120.0).floor()));
    let c = m._macC.get_or_insert_with(Default::default);
    let w = *c.entry(key).or_insert_with(|| macW(d, kal));
    let n = macW(0.0, m.len);
    let oran = jmax(MAC_UC[0], jmin(MAC_UC[1], w.c / jmax(0.001, w.g) / (n.c / jmax(0.001, n.g))));
    let birim = jmax(0.05, (w.g + w.c) / 2.0);
    Tac { macK: Some(oran), macZ: Some(MacZ { d, kal, birim }), ..tq }
}

pub fn tacEff0(m: &Mac, t: f64) -> Tac {
    let tq = m.tac[t as usize].clone();
    if inWin(m, t) {
        let risk = jmin(1.0, tq.risk.unwrap_or(0.5) + 0.25);
        let tempo = jmax(tq.tempo.unwrap_or(0.5), 0.8);
        Tac { risk: Some(risk), tempo: Some(tempo), kontra: Some(true), ..tq }
    } else {
        tq
    }
}

/// JS BLOK (savunma bloğunun derinliği)
pub fn BLOK(b: &str) -> Option<f64> {
    match b {
        "Düşük" => Some(22.0 * KX),
        "Orta" => Some(34.0 * KX),
        "Yüksek" => Some(50.0 * KX),
        _ => None,
    }
}
/// JS GEN (hücum genişliği)
pub fn GEN(g: &str) -> Option<f64> {
    match g {
        "Dar" => Some(28.0 * KY),
        "Normal" => Some(38.0 * KY),
        "Geniş" => Some(46.0 * KY),
        _ => None,
    }
}

pub const EVKOD: [&str; 9] = ["huc.gecis", "huc.kurma", "huc.ilerleme", "huc.son", "sav.gecis", "sav.kurma", "sav.orta", "sav.son", "huc.hizli"];

pub fn evreBul(m: &mut Mac, t: f64) -> &'static str {
    let bt = topTakim(m);
    let ex = match m.holder {
        Some(h) => m.ps[h].x,
        None => m.ball.as_ref().map_or(W / 2.0, |b| b.x),
    };
    let dG = (ex - if t == 0.0 { 0.0 } else { W }).abs();
    if bt == Some(t) {
        if inWin(m, t) {
            if m.gc.as_ref().unwrap().hizli {
                "huc.hizli"
            } else {
                "huc.gecis"
            }
        } else if dG < 35.0 * KX {
            "huc.kurma"
        } else if dG < 65.0 * KX {
            "huc.ilerleme"
        } else {
            "huc.son"
        }
    } else if kpAkt(m, t).is_some() {
        "sav.gecis"
    } else if dG > 65.0 * KX {
        "sav.kurma"
    } else if dG > 35.0 * KX {
        "sav.orta"
    } else {
        "sav.son"
    }
}

pub fn evreUygula(m: &mut Mac) {
    if m.tac0.is_none() {
        m.tac0 = Some(vec![None, None]);
    }
    if m.evre.is_none() {
        m.evre = Some(vec![None, None]);
    }
    for t in [0.0, 1.0] {
        let ti = t as usize;
        let Some(cur) = m.tac.get(ti).cloned() else { continue };
        if cur._ev.as_deref().map_or(true, |s| s.is_empty()) {
            m.tac0.as_mut().unwrap()[ti] = Some(cur.clone());
        }
        let b = m.tac0.as_ref().unwrap()[ti].clone().unwrap_or(cur);
        let key = evreBul(m, t);
        let tick = m.tick;
        let ev = m.evre.as_mut().unwrap();
        if ev[ti].as_ref().map_or(true, |e| e.key != key) {
            ev[ti] = Some(crate::model::JMEvreE { key: key.into(), ev: key[..3].into(), ad: key[4..].into(), t0: tick });
        }
        let mut mg = b.clone();
        if let Some(ks) = b.kosul.as_ref().filter(|k| !k.is_empty()) {
            let fark = m.score[ti] - m.score[1 - ti];
            let kal = (m.len - m.tick) / SN();
            let mut akt = vec![];
            for k in ks {
                let ne = k.ne.as_deref().unwrap_or("");
                let okS = if ne == "onde" { fark > 0.0 } else if ne == "geride" { fark < 0.0 } else { ne != "esit" || fark == 0.0 };
                let okT = k.son.map_or(true, |s| kal <= s);
                if okS && okT {
                    if let Some(a) = &k.ayar {
                        mg = mg.ust(a);
                        mg._ev = Some("kosul".into());
                        akt.push(ne.to_string());
                    }
                }
            }
        }
        m.tac[ti] = mg;
    }
}

#[allow(dead_code)]
fn _kullan() {
    let _ = CY;
}

// Oyuncu talimatı (maç kurulumundan taşındı, Aşama 7b)
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
