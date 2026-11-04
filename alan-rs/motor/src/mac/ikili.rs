//! İkili (match.js 1404–1430, 1835–2008): gövde gövdeye temas (itme, çarpma dürtüsü, sürtünme, denge kaybı),
//! gövdeleri ayırma, denge dolumu ve sendeleme, omuz/dürtme katsayıları.
use super::enerji::{enHarca_p, verim, EN};
use super::*;
use crate::core_sabit::{Q, TQ};
use crate::mat::{atan2, cos, hypot, jmax, jmin, jround, sin};
use crate::model::{JMSt, JM_temas, JM_temaslarLE};
use crate::saha::H;
use crate::yetenek::mek;

/// JS IK: ikili parametreleri.
pub struct Ik {
    pub yerles: f64,
    pub esnek: f64,
    pub surt: f64,
    pub dengeK: f64,
    pub dolum: f64,
    pub dolumS: f64,
    pub yavas: f64,
    pub sendele: f64,
    pub sendT: f64,
    pub sendCd: f64,
    pub kalkan: f64,
    pub kalkanR: f64,
    pub pokeT: f64,
    pub pokeK: f64,
    pub pokeAc: f64,
    pub pokeDn: f64,
    pub omuzV: f64,
}
pub const IK: Ik = Ik { yerles: 1.6, esnek: 0.15, surt: 0.12, dengeK: 3.2, dolum: 0.012, dolumS: 0.03, yavas: 0.35, sendele: 0.15, sendT: 12.0, sendCd: 40.0, kalkan: 0.7, kalkanR: 3.2, pokeT: 0.22, pokeK: 0.03, pokeAc: 0.5, pokeDn: 0.35, omuzV: 0.2 };

pub const fn ikOn() -> bool {
    true
}

/// Enerjinin ikiliye etkisi (Enerji 3.0: barın yarısı kadar).
pub fn ikEn(p: &Oyuncu) -> f64 {
    match p.en {
        None => 1.0,
        Some(e) => 1.15 * (0.5 + 0.5 * jmax(0.0, jmin(1.0, e))),
    }
}
pub fn omuzK(q: &Oyuncu) -> f64 {
    (1.0 + 0.03 * (q.yet.y(mek::OMUZ) - 10.0)) * ikEn(q)
}
pub fn durtFark(q: &Oyuncu, h: &Oyuncu) -> f64 {
    q.yet.y(mek::DURTME) - h.yet.y(mek::DURTME_KORU)
}
/// JS kutle(p, h): ph = (p === h); yerinde duran (taşıyıcı dışı) daha ağır.
pub fn kutle(p: &Oyuncu, ph: bool) -> f64 {
    (1.0 + 0.05 * (p.yet.y(mek::KUTLE) - 10.0)) * if !ph && hypot(or0(p.vx), or0(p.vy)) < 0.06 { IK.yerles } else { 1.0 }
}
pub fn direnc(p: &Oyuncu, ph: bool) -> f64 {
    (0.8 + 0.05 * (p.yet.y(if ph { mek::TOPLU_DENGE } else { mek::DENGE_DIRENC }) - 10.0)) * if !ph && hypot(or0(p.vx), or0(p.vy)) < 0.06 { 1.8 } else { 1.0 } * ikEn(p)
}

/// İkilinin enerji hedefi (JS IKM: enerji harcanacak maç).
pub struct IkHedef<'a> {
    pub st: &'a mut JMSt,
    pub lite: bool,
}

/// JS'in modül düzeyindeki IKT (en sert temas), IKM (enerji), IKL (temas listesi).
#[derive(Default)]
pub struct IkBag<'a> {
    pub ikt: Option<JM_temas>,
    pub ikm: Option<IkHedef<'a>>,
    pub ikl: Option<Vec<JM_temaslarLE>>,
}

/// Taşıyıcı bilgisi: None = JS `h` verilmedi (undefined), Some(None) = null, Some(Some(i)) = oyuncu.
pub type HArg = Option<Option<Pid>>;

fn esit(h: HArg, i: Pid) -> bool {
    matches!(h, Some(Some(x)) if x == i)
}

/// JS ikiliTemas(a, b, ux, uy, o, ilk, h): iki rakip gövde (ia, ib) birbirine girdi; o kadar ayır, ilk turda çarpma.
pub fn ikiliTemas(ps: &mut [Oyuncu], ia: Pid, ib: Pid, ux: f64, uy: f64, o: f64, ilk: bool, h: HArg, bag: &mut IkBag) -> f64 {
    let (ah, bh) = (esit(h, ia), esit(h, ib));
    let (a, b) = iki(ps, ia, ib);
    let mA = kutle(a, ah);
    let mB = kutle(b, bh);
    let iA = 1.0 / mA;
    let iB = 1.0 / mB;
    let sA = iA / (iA + iB);
    let sB = 1.0 - sA;
    a.x -= ux * o * sA;
    a.y -= uy * o * sA;
    b.x += ux * o * sB;
    b.y += uy * o * sB;
    if !ilk {
        return f64::NAN; // JS undefined
    }
    let (avx, avy, bvx, bvy) = (or0(a.vx), or0(a.vy), or0(b.vx), or0(b.vy));
    let vr = (avx - bvx) * ux + (avy - bvy) * uy;
    let mut J = 0.0;
    if vr > 0.0 {
        J = (1.0 + IK.esnek) * vr / (iA + iB);
        a.vx = avx - J * iA * ux;
        a.vy = avy - J * iA * uy;
        b.vx = bvx + J * iB * ux;
        b.vy = bvy + J * iB * uy;
    }
    let tx = -uy;
    let ty = ux;
    let ta = or0(a.vx) * tx + or0(a.vy) * ty;
    let tb = or0(b.vx) * tx + or0(b.vy) * ty;
    a.vx -= ta * IK.surt * tx;
    a.vy -= ta * IK.surt * ty;
    b.vx -= tb * IK.surt * tx;
    b.vy -= tb * IK.surt * ty;
    if h.is_some() && (J > 0.1 || (h.unwrap().is_some() && (ah || bh) && J > 0.03)) && bag.ikt.as_ref().map_or(true, |t| J > t.J) {
        bag.ikt = Some(JM_temas { x: (a.x + b.x) / 2.0, y: (a.y + b.y) / 2.0, a: atan2(uy, ux), J, t: 0.0 });
    }
    if !(h.map_or(false, |x| x.is_some()) && (ah || bh)) {
        return J;
    }
    if let Some(hd) = bag.ikm.as_mut() {
        for (p, pi) in [(&mut *a, ia), (&mut *b, ib)] {
            let v = EN.kalkan * EN.fizK * (1.0 + 10.0 * J) * p.yet.verim(mek::TEMAS_BEDEL);
            enHarca_p(hd.st, hd.lite, p, pi, "ikili", v);
        }
    }
    let yuk = J + if vr > 0.0 { 0.0015 } else { 0.0 } + 0.006 * (ta - tb).abs();
    let gA = avx * ux + avy * uy;
    let gB = -(bvx * ux + bvy * uy);
    let fA = if gA > 0.02 && gA > gB { 0.5 } else { 1.0 };
    let fB = if gB > 0.02 && gB > gA { 0.5 } else { 1.0 };
    a.den = Some(jmax(0.0, a.den.unwrap_or(1.0) - yuk * fA * IK.dengeK / (mA * direnc(a, ah))));
    b.den = Some(jmax(0.0, b.den.unwrap_or(1.0) - yuk * fB * IK.dengeK / (mB * direnc(b, bh))));
    J
}

/// JS govdeAyir(ps, h): iç içe giren gövdeleri ayır (3 tur); rakipler ikiliyle, takım arkadaşları itilme payıyla.
pub fn govdeAyir(ps: &mut [Oyuncu], h: HArg, bag: &mut IkBag) {
    let bd = TQ.min_d;
    let n = ps.len();
    let mut gx: Vec<f64> = ps.iter().map(|p| p.x).collect();
    let mut gy: Vec<f64> = ps.iter().map(|p| p.y).collect();
    for it in 0..3 {
        let mut degdi = false;
        for i in 0..n {
            for j in i + 1..n {
                let dx = gx[j] - gx[i];
                let dy = gy[j] - gy[i];
                if dx >= bd || dx <= -bd || dy >= bd || dy <= -bd {
                    continue;
                }
                let d = hypot(dx, dy);
                if d >= bd {
                    continue;
                }
                degdi = true;
                let ux = if d > 1e-4 { dx / d } else { 1.0 };
                let uy = if d > 1e-4 { dy / d } else { 0.0 };
                if ikOn() && ps[i].team != ps[j].team {
                    let jt = ikiliTemas(ps, i, j, ux, uy, bd - d, it == 0, h, bag);
                    if it == 0 {
                        if let Some(l) = bag.ikl.as_mut() {
                            let iA = 1.0 / kutle(&ps[i], esit(h, i));
                            let iB = 1.0 / kutle(&ps[j], esit(h, j));
                            l.push(JM_temaslarLE { i: i as f64, j: j as f64, J: if dogru(jt) { jt } else { 0.0 }, o: bd - d, s: 2.0 * iA / (iA + iB) - 1.0 });
                        }
                    }
                    for k in [i, j] {
                        let p = &mut ps[k];
                        p.x = cl(p.x, 0.8, W - 0.8);
                        p.y = cl(p.y, 0.8, H - 0.8);
                    }
                    gx[i] = ps[i].x;
                    gy[i] = ps[i].y;
                    gx[j] = ps[j].x;
                    gy[j] = ps[j].y;
                    continue;
                }
                let (a, b) = iki(ps, i, j);
                let ga = 1.0 + 0.05 * (a.yet.y(mek::ITILME_PAYI) - 10.0);
                let gb = 1.0 + 0.05 * (b.yet.y(mek::ITILME_PAYI) - 10.0);
                let ka = gb / (ga + gb);
                let kb = 1.0 - ka;
                let o = bd - d;
                a.x -= ux * o * ka;
                a.y -= uy * o * ka;
                b.x += ux * o * kb;
                b.y += uy * o * kb;
                let rv = (b.vx - a.vx) * ux + (b.vy - a.vy) * uy;
                if rv < 0.0 {
                    a.vx += ux * rv * ka;
                    a.vy += uy * rv * ka;
                    b.vx -= ux * rv * kb;
                    b.vy -= uy * rv * kb;
                }
                for p in [a, b] {
                    p.x = cl(p.x, 0.8, W - 0.8);
                    p.y = cl(p.y, 0.8, H - 0.8);
                }
                gx[i] = ps[i].x;
                gy[i] = ps[i].y;
                gx[j] = ps[j].x;
                gy[j] = ps[j].y;
            }
        }
        if !degdi {
            break;
        }
    }
}

/// Sendeleme süresi (tik).
pub fn sendS(p: &Oyuncu) -> f64 {
    jmax(2.0, jround(IK.sendT - 0.5 * (p.yet.y(mek::SENDELEME) - 10.0)))
}
fn dengeDolumK(p: &Oyuncu) -> f64 {
    1.0 + 0.0625 * (p.yet.y(mek::DENGE_DOLUM) - 10.0)
}

/// JS denge(m): denge dolar; çok düşerse sendeler (taşıyıcıysa Çekirdek ayağından kaçar).
pub fn denge(m: &mut Mac) {
    for pi in 0..m.ps.len() {
        let Some(den) = m.ps[pi].den else { continue };
        let sd = gt(m.ps[pi].sendT, m.tick);
        let yeni = jmin(1.0, den + (if sd { IK.dolumS } else { IK.dolum }) * dengeDolumK(&m.ps[pi]));
        m.ps[pi].den = Some(yeni);
        if !sd && yeni < IK.sendele && m.tick > oo(m.ps[pi].sendCd) {
            let s = sendS(&m.ps[pi]);
            let tick = m.tick;
            {
                let p = &mut m.ps[pi];
                p.sendT = Some(tick + s);
                p.sendCd = Some(tick + IK.sendCd);
                p.stun = Some(jmax(oo(p.stun), tick + s));
            }
            if m.holder == Some(pi) {
                let p = &m.ps[pi];
                let ca = p.ca.unwrap_or_else(|| atan2(or0(p.vy), or0(p.vx)));
                let b = Q.body;
                let mut o: Option<Pid> = None;
                let mut od = 1e9;
                for (qi, q) in m.ps.iter().enumerate() {
                    if q.team != p.team {
                        let dq = hypot(q.x - p.x, q.y - p.y);
                        if dq < od {
                            od = dq;
                            o = Some(qi);
                        }
                    }
                }
                let (px, py, pvx, pvy, pt) = (p.x, p.y, or0(p.vx), or0(p.vy), p.team);
                let top = vurus::makeBall(m, vurus::TopIstek { x: px + cos(ca) * b, y: py + sin(ca) * b, vx: 0.5 * pvx + 0.14 * cos(ca), vy: 0.5 * pvy + 0.14 * sin(ca), team: pt, ch: Some(m.ch), from: Some(o.unwrap_or(pi)), defl: 1.0, ..Default::default() });
                m.ball = Some(top);
                m.holder = None;
                m.fl = None;
                m.ps[pi].savur = None;
                m.st.loose[pt as usize] += 1.0;
                if let Some(o) = o {
                    let ot = m.ps[o].team as usize;
                    if m.ps[pi]._omuz.as_ref().map_or(false, |x| x.t >= m.tick - 3.0) {
                        m.st.duelW[ot] += 1.0;
                        if let Some(s) = m.ps[o].son.as_mut() {
                            s.ok = true;
                        }
                    } else if od < 3.0 {
                        m.st.duel[ot] += 1.0;
                        m.st.duelW[ot] += 1.0;
                    }
                }
                let kim = o.map_or(1.0 - pt, |o| m.ps[o].team);
                let ad = m.ps[pi].name.clone();
                ev(m, "kesme", kim, format!("{ad} sendeledi, Çekirdek ayağından kaçtı"));
            } else if !m.mac._lite {
                let (t, ad) = (m.ps[pi].team, m.ps[pi].name.clone());
                ev(m, "pas", t, format!("{ad} sendeledi"));
            }
        }
    }
}

/// JS dengeKayip(p, t): dengesizliğin önümüzdeki tiklerde götüreceği koşu (tik cinsinden).
pub fn dengeKayip(p: &Oyuncu, t: f64) -> f64 {
    let Some(mut dn) = p.den else { return 0.0 };
    let mut l = 0.0;
    let dm = dengeDolumK(p);
    for k in 0..150 {
        let sd = oo(p.sendT) > t + k as f64;
        if !sd && dn >= IK.yavas {
            break;
        }
        let mut f = if sd { 0.3 } else { 1.0 };
        if dn < IK.yavas {
            f *= 0.45 + 0.55 * dn / IK.yavas;
        }
        l += 1.0 - f;
        dn = jmin(1.0, dn + (if sd { IK.dolumS } else { IK.dolum }) * dm);
    }
    l
}

/// JS dengeHiz(m, p): denge, sendeleme ve (taşıyıcıysa) yakın rakibin kalkanı hızı keser.
pub fn dengeHiz(m: &Mac, pi: Pid) -> f64 {
    let p = &m.ps[pi];
    if p.den.is_none() && m.holder != Some(pi) {
        return 1.0;
    }
    let dn = p.den.unwrap_or(1.0);
    let mut f = if oo(p.sendT) > m.tick { 0.3 } else { 1.0 };
    if dn < IK.yavas {
        f *= 0.45 + 0.55 * dn / IK.yavas;
    }
    if m.holder == Some(pi) {
        for q in &m.ps {
            if q.team != p.team && !q.noTouch && hypot(q.x - p.x, q.y - p.y) < IK.kalkanR {
                f *= IK.kalkan;
                break;
            }
        }
    }
    f
}

#[allow(dead_code)]
fn _kullan() {
    let _ = verim;
}
