//! Kafa günlüğü (match.js 2894–3048): her kararın kafadaki beklentisi ve maçta ne olduğu (sapma).
use super::*;
use crate::mat::{hypot, jmax, jround};
use crate::model::Secenek;

/// JS KS: sapma eşikleri
pub struct Ks {
    pub hiz: f64,
    pub hizMin: f64,
    pub gec: f64,
    pub kayip: f64,
    pub kayipSur: f64,
    pub sans: f64,
    pub yer: f64,
    pub zaman: f64,
    pub golAz: f64,
    pub golCok: f64,
}
pub const KS: Ks = Ks { hiz: 0.2, hizMin: 0.3, gec: 8.0, kayip: 0.8, kayipSur: 0.85, sans: 0.3, yer: 8.0, zaman: 30.0, golAz: 0.05, golCok: 0.6 };

/// Kararın sonu (JS son: {t, santra, devam, kim, kimId, kimT, x, y, gol}).
#[derive(Clone, Debug, Default)]
pub struct Son {
    pub t: f64,
    pub santra: bool,
    pub devam: bool,
    pub kim: Option<String>,
    pub kimId: Option<f64>,
    pub kimT: Option<f64>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub gol: Option<f64>,
}

/// Sapma (JS {k, a, y}).
#[derive(Clone, Debug, Default)]
pub struct Sapma {
    pub k: String,
    pub a: f64,
    pub y: String,
}

/// Kararın kaydı (JS K.P.r).
#[derive(Clone, Debug, Default)]
pub struct KafaKayit {
    pub t: f64,
    pub ad: String,
    pub id: f64,
    pub team: f64,
    pub kind: String,
    pub kendine: bool,
    pub q: Option<String>,
    pub qid: Option<f64>,
    pub shot: Option<String>,
    pub ot: bool,
    pub v: Option<f64>,
    pub P: Option<f64>,
    pub Ps: Option<f64>,
    pub Pk: Option<f64>,
    pub hiz: Option<f64>,
    pub savT: Option<f64>,
    pub tf: Option<f64>,
    pub alis: Option<(Option<f64>, Option<f64>)>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub cikis: Option<f64>,
    pub hizC: Option<f64>,
    pub hizG: Option<f64>,
    pub bozuldu: Option<String>,
    pub sek: Option<f64>,
    pub son: Option<Son>,
    pub sapma: Vec<Sapma>,
}

#[derive(Clone, Debug, Default)]
pub struct KafaP {
    /// uygulanan seçeneğin kimliği (JS K.P.o)
    pub oid: u64,
    pub r: KafaKayit,
}

/// JS m._kafa
#[derive(Clone, Debug, Default)]
pub struct KafaGunluk {
    pub L: Vec<KafaKayit>,
    pub P: Option<KafaP>,
}

/// JS k3(x): 3 basamağa yuvarla (sonlu değilse yok)
pub fn k3(x: Option<f64>) -> Option<f64> {
    match x {
        Some(v) if v.is_finite() => Some(jround(1e3 * v) / 1e3),
        _ => None,
    }
}
fn kHiz(v: Option<(f64, f64)>) -> Option<f64> {
    v.map(|v| hypot(v.0, v.1))
}

/// JS kafaAc(m)
pub fn kafaAc(m: &mut Mac) {
    m.mac._kafa = Some(KafaGunluk::default());
}

/// JS kafaBitir(m, son)
pub fn kafaBitir(m: &mut Mac, mut son: Son) {
    let tick = m.tick;
    let Some(k) = m.mac._kafa.as_mut() else { return };
    let Some(p) = k.P.take() else { return };
    let mut r = p.r;
    son.t = tick;
    r.son = Some(son);
    r.sapma = kafaSapma(&r);
    k.L.push(r);
}

/// JS kafaBasla(m, h, o, ot)
pub fn kafaBasla(m: &mut Mac, h: Pid, o: &mut Secenek, ot: bool) {
    if m.mac._kafa.is_none() {
        return;
    }
    kafaBitir(m, Son { devam: true, ..Default::default() });
    if o.kind == "fake" {
        return;
    }
    m.mac.oSay += 1;
    let oid = m.mac.oSay;
    o._kafaId = Some(oid);
    let hp = &m.ps[h];
    let d = o.det.as_ref();
    let q = o.q.filter(|&q| !m.ps[q].name.is_empty());
    let e: Option<(f64, f64)> = if let Some(d) = d.filter(|d| d.endM.x.is_finite()) {
        Some((d.endM.x, d.endM.y))
    } else if o.kind == "sür" || o.kind == "önüne" {
        o.to.as_ref().map(|t| (t.x, t.y))
    } else if let Some(q) = q.filter(|&q| q != h) {
        Some((m.ps[q].x, m.ps[q].y))
    } else {
        o.lead.as_ref().map(|l| (l.x, l.y))
    };
    let r = KafaKayit {
        t: m.tick,
        ad: hp.name.clone(),
        id: hp.id,
        team: hp.team,
        kind: o.kind.clone(),
        kendine: o.kendine,
        q: q.filter(|&q| q != h).map(|q| m.ps[q].name.clone()),
        qid: q.map(|q| m.ps[q].id),
        shot: o.shot.clone(),
        ot,
        v: k3(o.v),
        P: k3(d.map(|d| d.P)),
        Ps: d.map(|d| d.Ps).and_then(|x| k3(Some(x))),
        Pk: k3(d.map(|d| d.Pk)),
        hiz: k3(kHiz(o.launch.as_ref().map(|l| (l.vx, l.vy)))),
        savT: o.savT,
        tf: o.tf.map(jround),
        alis: e.map(|e| (k3(Some(e.0)), k3(Some(e.1)))),
        x: k3(Some(hp.x)),
        y: k3(Some(hp.y)),
        ..Default::default()
    };
    m.mac._kafa.as_mut().unwrap().P = Some(KafaP { oid, r });
}

fn ayni(m: &Mac, o: &Secenek) -> bool {
    matches!((&m.mac._kafa, o._kafaId), (Some(k), Some(id)) if k.P.as_ref().map_or(false, |p| p.oid == id))
}

/// JS kafaCikis(m, o, v)
pub fn kafaCikis(m: &mut Mac, o: &Secenek, v: (f64, f64)) {
    if !ayni(m, o) {
        return;
    }
    let tick = m.tick;
    let r = &mut m.mac._kafa.as_mut().unwrap().P.as_mut().unwrap().r;
    r.cikis = Some(tick);
    r.hizC = k3(kHiz(o.launch.as_ref().map(|l| (l.vx, l.vy))));
    r.hizG = k3(kHiz(Some(v)));
}

/// JS kafaNot(m, o, y)
pub fn kafaNot(m: &mut Mac, o: &Secenek, y: &str) {
    if ayni(m, o) {
        m.mac._kafa.as_mut().unwrap().P.as_mut().unwrap().r.bozuldu = Some(y.into());
    }
}

fn f2(x: f64) -> String {
    js_num(jround(100.0 * x) / 100.0).replace('.', ",")
}
fn yuz(x: f64) -> String {
    format!("%{}", js_num(jround(100.0 * x)))
}

/// JS kafaSapma(r): kafadaki beklentiyle olanın farkları.
pub fn kafaSapma(r: &KafaKayit) -> Vec<Sapma> {
    let mut s = vec![];
    let d0 = Son::default();
    let S = r.son.as_ref().unwrap_or(&d0);
    let vur = r.hiz.is_some();
    let bizde = S.kimT == Some(r.team) || S.gol == Some(r.team);
    let kayip = (S.kimT.is_some() && S.kimT != Some(r.team)) || (S.gol.is_some() && S.gol != Some(r.team));
    let sp = |k: &str, a: f64, y: String| Sapma { k: k.into(), a, y };
    if vur {
        match r.cikis {
            None => {
                if kayip {
                    s.push(sp("savur", 3.0, format!("vuruş çıkmadan {} aldı", S.kim.clone().filter(|x| !x.is_empty()).unwrap_or_else(|| "rakip".into()))));
                } else if r.bozuldu.is_some() || S.devam {
                    s.push(sp("savur", 3.0, format!("vuruş çıkmadı{}", r.bozuldu.as_ref().map_or(String::new(), |b| format!(": {b}")))));
                }
            }
            Some(c) => {
                let hiz = r.hiz.unwrap();
                if let Some(hc) = r.hizC {
                    if hiz > 0.05 && (hc - hiz).abs() > jmax(KS.hizMin, KS.hiz * hiz) {
                        s.push(sp("hız", 3.0, format!("hız kafada {}, çıkan {}", f2(hiz), f2(hc))));
                    }
                }
                if let Some(st) = r.savT {
                    if c - r.t - st > KS.gec {
                        s.push(sp("gecikme", 2.0, format!("vuruş kafada {} tikte çıkacaktı, {} tikte çıktı", js_num(st), js_num(c - r.t))));
                    }
                }
            }
        }
    }
    if r.kind == "gönder" {
        if let (Some(_), Some(pk)) = (r.cikis, r.Pk) {
            if S.gol == Some(r.team) && pk < KS.golAz {
                s.push(sp("şans", 1.0, format!("gol oldu, kafada gol ihtimali {}", yuz(pk))));
            } else if S.gol.is_none() && pk >= KS.golCok {
                s.push(sp("kayıp", 1.0, format!("kafada gol ihtimali {}, gol olmadı", yuz(pk))));
            }
        }
        return s;
    }
    let Some(P) = r.P else { return s };
    if vur && r.cikis.is_none() {
        return s;
    }
    let kim = S.kim.clone().filter(|x| !x.is_empty());
    if vur && r.q.as_ref().map_or(false, |q| !q.is_empty()) {
        let q = r.q.clone().unwrap();
        let aP = jmax(0.0, P - r.Ps.filter(|x| dogru(*x)).unwrap_or(0.0));
        if aP >= KS.kayip && (S.kim.as_deref() != Some(q.as_str()) || r.sek.map_or(false, dogru)) && (kim.is_some() || S.gol.is_some()) {
            let ne = if S.kim.as_deref() == Some(q.as_str()) { "top sekip ona döndü".to_string() } else if let Some(k) = &kim { format!("{k} aldı") } else { "gol yedik".into() };
            let ek = if kayip || S.kim.as_deref() == Some(q.as_str()) { "" } else { " (bizde kaldı)" };
            s.push(sp(if kayip { "kayıp" } else { "kaçtı" }, 2.0, format!("kafada {} {q} alacaktı, {ne}{ek}", yuz(aP))));
        }
        return s;
    }
    if kayip && P >= if vur { KS.kayip } else { KS.kayipSur } {
        let kk = kim.as_ref().map_or("gol yedik".to_string(), |k| format!("{k} aldı"));
        let y = if vur {
            format!("kafada {} bizde kalacaktı, {kk}", yuz(P))
        } else {
            let a = if r.kind == "sür" { "sürerken" } else { "beklerken" };
            let b = kim.as_ref().map_or("kaybetti".to_string(), |k| format!("{k} aldı"));
            format!("{a} {b}, kafada {} tutacaktı", yuz(P))
        };
        s.push(sp("kayıp", 2.0, y));
    }
    if vur && bizde {
        if let Some(kid) = S.kimId {
            let hedef = Some(kid) == r.qid;
            let d = match (&r.alis, S.x) {
                (Some((Some(ax), Some(ay))), Some(sx)) => hypot(sx - ax, S.y.unwrap_or(f64::NAN) - ay),
                _ => 0.0,
            };
            let gec = r.tf.map_or(0.0, |tf| S.t - r.t - tf);
            let kn = kim.clone().unwrap_or_default();
            if hedef && P <= KS.sans {
                s.push(sp("şans", 1.0, format!("kafada {}, {kn} aldı", yuz(P))));
            }
            if hedef && (d > KS.yer || gec.abs() > KS.zaman) {
                let a = if gec.abs() > KS.zaman { format!("kafadakinden {} sn {}", f2(gec.abs() / SN()), if gec > 0.0 { "geç" } else { "erken" }) } else { "kafadakinden".into() };
                let b = if d > KS.yer { format!(", {} br ötede", js_num(jround(d))) } else { String::new() };
                s.push(sp("yer", 1.0, format!("{kn} topu {a}{b} aldı")));
            }
            if !hedef && r.qid.is_some() && d > KS.yer {
                let q = r.q.clone().filter(|x| !x.is_empty()).unwrap_or_else(|| "kendine".into());
                s.push(sp("yer", 1.0, format!("{q} giden topu {kn} aldı, {} br ötede", js_num(jround(d)))));
            }
        }
    }
    s
}
