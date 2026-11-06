//! Vuruş (match.js 2751–2893): Çekirdeği bırakma (pas, önüne, kenardan, gönder, çalım) ve top üretimi.
use super::enerji::*;
use super::gecici_dis as dis;
use super::hareket::corePos;
use super::*;
use crate::alan::{Etki, Kaynak};
use crate::core_sabit::IN;
use crate::mat::{hypot, jmax, jmin, jround};
use crate::model::{JMFl, JMFlEnd, JMFlTo, MacTop, Secenek, TopEk};
use crate::saha::P;
use crate::temas::{TopT, TopTemas};
use crate::top::{tahmin, Top};
use crate::yetenek::mek;

pub const BODY_R: f64 = 1.0;
pub const BODY_E: f64 = 0.35;

/// JS makeBall({…}) alanları.
#[derive(Clone, Debug, Default)]
pub struct TopIstek {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub team: f64,
    pub ch: Option<f64>,
    pub from: Option<Pid>,
    pub recv: Option<Pid>,
    pub vur: Option<Pid>,
    pub defl: f64,
}

/// JS vurTaban(p): vuruşun tabanı (topun en az enerjisi)
pub fn vurTaban(p: &Oyuncu) -> f64 {
    P.tek_taban * jmax(0.5, jmin(1.5, 1.0 + 0.05 * (p.govde.yet.y(mek::VURUS_TABAN) - 10.0)))
}

/// JS K.makeBall(o): yeni top (yeni kimlikle).
pub fn makeBall(m: &mut Mac, o: TopIstek) -> MacTop {
    let taban = match o.vur {
        Some(v) => vurTaban(&m.ps[v]),
        None => P.tek_taban,
    };
    let mut b = Top::yeni(o.x, o.y, o.vx, o.vy, o.team as u8, o.ch, taban);
    b.defl = o.defl;
    m.mac.topSay += 1;
    let t = TopTemas { kimlik: m.mac.topSay, from: o.from, recv: o.recv, ..Default::default() };
    MacTop { tt: TopT { b, t }, ek: TopEk { ..Default::default() } }
}

fn etkiler(m: &Mac) -> Vec<Etki> {
    m.ps.iter().map(|p| p.etki()).collect()
}

/// JS launch(m, h, o): seçeneği uygula, hatayı ekle, topu bırak.
pub fn launch(m: &mut Mac, h: Pid, mut o: Secenek) {
    let F = &SPR_FAZ;
    let ht = m.ps[h].team;
    let hname = m.ps[h].govde.name.clone();
    if F.itKos && o.q == Some(h) && o.kind != "gönder" {
        if m.ps[h]._sf.is_some() {
            if !m.mac._lite {
                ev(m, "pas", ht, format!("{hname} topu yine önüne itti"));
            }
        } else {
            if o.itK.is_some() && SPK() {
                m.ps[h].hal_mut()._itK = o.itK;
            }
            let spr = o.spr.map_or(false, crate::mat::dogru);
            if spr && F.ayir && !m.ps[h].bekci() && m.ps[h].en.unwrap_or(1.0) >= ALAN_EN3_SGIR(&m.ps[h], SBED()) {
                sprintBasla(m, h, Some(format!("{hname} topu önüne itti")));
            } else if !m.mac._lite {
                ev(m, "pas", ht, format!("{hname} topu önüne itti"));
            }
        }
    }
    if o.kind != "gönder" {
        let (tx, ty) = match (&o.to, o.q) {
            (Some(t), _) => (t.x, t.y),
            (None, Some(q)) => (m.ps[q].x, m.ps[q].y),
            _ => (m.ps[h].x, m.ps[h].y),
        };
        let hp = &m.ps[h];
        m._pas = Some(crate::model::JM_pas { team: ht, fx: hp.x, tx, len: hypot(tx - hp.x, ty - hp.y), t: m.tick });
    }
    if o.to.is_some() && o.launch.is_some() && matches!(o.kind.as_str(), "pas" | "önüne" | "kenardan") {
        if let Some(c0) = corePos(m) {
            let l = o.launch.as_mut().unwrap();
            let to = o.to.as_ref().unwrap();
            let sp = hypot(l.vx, l.vy);
            let dx = to.x - c0.0;
            let dy = to.y - c0.1;
            let ll = hypot(dx, dy);
            if ll > 1.0 && sp > 0.0 {
                l.vx = dx / ll * sp;
                l.vy = dy / ll * sp;
            }
        }
    }
    let gon = o.kind == "gönder";
    let kenar = o.kind == "kenardan";
    let mut ek = o.ot.filter(|&x| dogru(x)).unwrap_or(1.0);
    ek *= if gon { dis::gonEk(m, h, o.shot.as_deref()) } else { dis::pasEk(m, h) };
    let la = o.launch.clone().unwrap_or_default();
    let v: (f64, f64) = if o.noErr {
        (la.vx, la.vy)
    } else {
        let mut r = std::mem::take(&mut m.r);
        let v = match o.savW {
            Some(sw) => dis::applyErrorSav(m, h, (la.vx, la.vy), &mut r, ek, gon, sw, o.savU.unwrap_or(f64::NAN), o.shot.as_deref(), kenar),
            None => dis::applyError(m, h, (la.vx, la.vy), &mut r, ek, gon, o.shot.as_deref(), kenar),
        };
        m.r = r;
        v
    };
    let t = ht;
    let tk = if o.ot.map_or(false, dogru) { " · tek dokunuş" } else { "" };
    if o.ot.map_or(false, dogru) {
        if m.st.ot.is_empty() {
            m.st.ot = vec![0.0, 0.0];
        }
        m.st.ot[t as usize] += 1.0;
        if !gon && !kenar {
            let qn = o.q.map_or(String::new(), |q| m.ps[q].govde.name.to_string());
            let ch = js_num(jround(100.0 * m.ch));
            ev(m, "pas", t, format!("{hname} tek dokunuşla → {qn} · şarj %{ch}"));
        }
    }
    super::kafa_bag::kafaCikis(m, &o, v);
    let cp = corePos(m).unwrap_or((m.ps[h].x, m.ps[h].y));
    let ch = m.ch;
    let mut b = makeBall(m, TopIstek { x: cp.0, y: cp.1, vx: v.0, vy: v.1, team: t, ch: Some(ch), from: Some(h), recv: o.q, vur: Some(h), defl: 0.0 });
    b.tt.t.kt = Some(m.tick);
    b.tt.t.kz = (((if dogru(m.seed) { m.seed } else { 1.0 }) as i64 as u32) % 9973) as f64;
    b.tt.t.cue = (if o.ot.map_or(false, dogru) { IN.cue_ot } else { IN.cue0 }) + jmax(0.0, oo(o.savT));
    if m.mac._lite {
        b.tt.t.k_bek = true;
    }
    m.holder = None;
    if o.q.is_some() && !gon && !o.noErr && o.launch.is_some() {
        let e1 = tahmin(&b.tt.b, &mut Kaynak::Sec(&[]), &P, Some(600)).1;
        let b0 = makeBall(m, TopIstek { x: cp.0, y: cp.1, vx: la.vx, vy: la.vy, team: t, ch: Some(ch), from: Some(h), recv: o.q, vur: Some(h), defl: 0.0 });
        m.mac.topSay -= 1;
        let e0 = tahmin(&b0.tt.b, &mut Kaynak::Sec(&[]), &P, Some(600)).1;
        if hypot(e1.x - e0.x, e1.y - e0.y) > 6.0 {
            b.tt.t.sap = true;
        }
    }
    if let Some(bk) = m.ps.iter().position(|p| p.team != t && p.bekci()) {
        let kk = MacKural::of(m);
        b.tt.t.bek_id = Some(m.ps[bk].id as u32);
        b.tt.t.bek_m = crate::temas::bek_har(&m.ps[bk], b.x, b.y, b.vx, b.vy, &kk);
    }
    let src = etkiler(m);
    let pr = tahmin(&b.tt.b, &mut Kaynak::Sec(&src), &P, Some(600)).1;
    m.fl = Some(JMFl { t0: m.tick, kind: o.kind.clone(), q: o.q, to: o.to.as_ref().map(|t| JMFlTo { x: t.x, y: t.y }), end: JMFlEnd { x: pr.x, y: pr.y }, shot: o.shot.clone() });
    m.ball = Some(b);
    let ti = t as usize;
    if gon {
        let b = m.m.ball.as_mut().unwrap();
        b.tt.t.gon = true;
        m.m.st.shot[ti] += 1.0;
        b.ek.shotPk = o.det.as_ref().and_then(|d| d.Pk);
        // JS `!m.lite` (yazım: m.lite hiç yok) → her zaman
        let sh = o.shot.clone().unwrap_or_default();
        let ch = js_num(jround(100.0 * m.ch));
        ev(m, "gonder", t, format!("{hname} {sh} gönderdi{tk} · şarj %{ch}"));
    } else if o.kind == "çalım" {
        if m.st.calim.is_empty() {
            m.st.calim = vec![0.0, 0.0];
        }
        m.st.calim[ti] += 1.0;
        let wp = o.wp.clone().unwrap_or_default();
        let (x, y) = o.wp1.as_ref().map_or((wp.x, wp.y), |w| (w.x, w.y));
        let sonra = o.wp1.as_ref().map(|_| (wp.x, wp.y));
        m.ps[h].hal_mut().kenarW = Some(KenarW { x, y, sonra, until: m.tick + 60.0 });
        ev(m, "pas", t, format!("{hname} kenardan çalım"));
    } else {
        m.st.pass[ti] += 1.0;
        if kenar {
            m.st.bank[ti] += 1.0;
            if o.kendine {
                if m.st.calim.is_empty() {
                    m.st.calim = vec![0.0, 0.0];
                }
                m.st.calim[ti] += 1.0;
                ev(m, "pas", t, format!("{hname} kenardan çalım · kendine sektirdi"));
            } else {
                let qn = o.q.map_or(String::new(), |q| m.ps[q].govde.name.to_string());
                ev(m, "pas", t, format!("{hname} kenardan sektirdi → {qn}{tk}"));
            }
        }
        if o.kind == "önüne" {
            m.st.lead[ti] += 1.0;
        }
    }
}

