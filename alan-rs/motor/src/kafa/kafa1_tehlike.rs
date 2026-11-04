//! Tehlike ve değer (decide.js:131–147 threat/oppAt/holdVal, 3492–3770 build/space/front/Vend/runOn/
//! runSut/tercihEk/dagin/V/V0/tekF/laneOk/respond, 3945–3966 zoneR/pressAt/freeAt/recvCh).
#![allow(non_snake_case)]

use super::gecici_dis1::{bosluk, sutEngel};
use super::kafa1_baglam::Kafa;
use super::kafa1_js::*;
use crate::alan::Kaynak;
use crate::mat::{cos, exp, hypot, jmax, jmin, pow, sin};
use crate::model::Oyuncu;
use crate::oyuncu::Tepki;
use crate::saha::{CY as GCY, H as GH, KR, W as GW};
use crate::temas::kuyu_girer;

/// JS `GW8`, `GH8`
pub const GW8: f64 = if GW == 100.0 { 99.2 } else { GW - 0.8 };
pub const GH8: f64 = if GH == 50.0 { 49.2 } else { GH - 0.8 };

/// Kale (atak yönü) x'i: takım 0 sağa (GW) atar.
#[inline(always)]
pub fn gx_of(team: f64) -> f64 {
    if team == 0.0 {
        GW
    } else {
        0.0
    }
}
/// Kendi kalesi x'i.
#[inline(always)]
pub fn ox_of(team: f64) -> f64 {
    if team == 0.0 {
        0.0
    } else {
        GW
    }
}

/// JS kuyuD(x, y): Kuyu sınırının içeriye uzaklığı (dışarıda -1).
pub fn kuyuD(x: f64, y: f64) -> f64 {
    let r = KR + 0.6;
    let dy = y - GCY;
    if dy > r || dy < -r || (x > r && x < GW - r) {
        -1.0
    } else {
        r - jmin(hypot(x, dy), hypot(x - GW, dy))
    }
}

/// SPT (space'in örnek noktaları: [r, açı])
pub fn SPT() -> Vec<(f64, f64)> {
    spt()
}
fn spt() -> Vec<(f64, f64)> {
    let mut v = Vec::new();
    for r in [6.0, 10.0, 14.0, 18.0, 22.0, 26.0, 30.0] {
        for k in -3..=3 {
            v.push((r, 0.37 * k as f64));
        }
    }
    v
}
/// FRT (front'un örnek noktaları)
fn frt() -> Vec<(f64, f64)> {
    let gkx = GW / 100.0;
    let gky = GH / 50.0;
    let mut v = Vec::new();
    let mut x = 36.0 * gkx;
    while x <= 92.0 * gkx {
        let mut y = 6.0 * gky;
        while y <= 44.0 * gky {
            v.push((x, y));
            y += 7.6 * gky;
        }
        x += 8.0 * gkx;
    }
    v
}

/// JS bosOy dönüşü
pub struct BosOy {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub v: Vec<f64>,
    pub a: Vec<f64>,
    pub t: Vec<i8>,
}

impl Kafa {
    /// JS pasMax()
    pub fn pasMax(&self) -> f64 {
        self.D.passMaxF
    }

    /// Kale tehlikesi (decide.js:136): kaleye yakınlık × ortaya yakınlık.
    pub fn threat(&self, team: f64, x: f64, y: f64) -> f64 {
        threat_d(&self.D, team, x, y)
    }
    /// Rakip alanı (decide.js:140).
    pub fn oppAt(&self, src: &[Op], team: f64, x: f64, y: f64) -> f64 {
        let mut s = 0.0;
        for p in src {
            if p.team != team {
                s += p.etki().infl(x, y);
            }
        }
        s
    }
    /// decide.js:145
    pub fn holdVal(&self, src: &[Op], team: f64, x: f64, y: f64) -> f64 {
        self.threat(team, x, y) * (0.4 + 0.6 * exp(2.0 * -self.oppAt(src, team, x, y)))
    }

    /// decide.js:3945
    pub fn zoneR(&self, team: f64, x: f64, y: f64) -> f64 {
        let gx = gx_of(team);
        let d = hypot(gx - x, GCY - y);
        self.D.zoneMin + (1.0 - self.D.zoneMin) * jmin(1.0, jmax(0.0, (d - 8.0) / self.D.zoneD))
    }
    /// Baskı (decide.js:3949).
    pub fn pressAt(&self, src: &[Op], team: f64, x: f64, y: f64) -> f64 {
        let z = self.zoneR(team, x, y);
        let mut s = 0.0;
        for p in src {
            if p.team == team || p.noTouch {
                continue;
            }
            let d = hypot(p.x - x, p.y - y) / z;
            let r = 0.55 * ya(p.R, 8.0);
            s += ya(p.D, 1.0) * exp(-d * d / (r * r));
        }
        s
    }
    /// decide.js:3959
    pub fn freeAt(&self, src: &[Op], team: f64, x: f64, y: f64) -> f64 {
        exp(1.6 * -self.pressAt(src, team, x, y))
    }
    /// decide.js:3962 (tek enerji açık: max(.05, ch))
    pub fn recvCh(&self, _src: &[Op], _team: f64, _x: f64, _y: f64, ch: f64, _q: &Oyuncu) -> f64 {
        jmax(0.05, ch)
    }

    /// decide.js:3492 (useCurve, useValTab kapalı)
    pub fn build(&self, _src: &[Op], team: f64, x: f64, y: f64) -> f64 {
        self.D.b0 + self.D.b1 * self.threat(team, x, y)
    }

    /// JS bosOy(src, team, bx, by) (bosluk'un oyuncu dizileri)
    pub fn bosOy(&self, src: &[Op], team: f64, bx: f64, by: f64) -> BosOy {
        let n = src.len();
        let mut o = BosOy { x: vec![0.0; n], y: vec![0.0; n], v: vec![0.0; n], a: vec![0.0; n], t: vec![0; n] };
        for (k, p) in src.iter().enumerate() {
            o.x[k] = p.x;
            o.y[k] = p.y;
            o.v[k] = self.spd(p);
            if p.team == team {
                o.t[k] = if !p.bekci() && hypot(p.x - bx, p.y - by) > 1.5 { 1 } else { 0 };
            } else {
                o.t[k] = -1;
                o.a[k] = 0.3 * p.yet.ozk("tepki");
            }
        }
        o
    }
    fn bos_tara(&self, o: &BosOy, x: f64, y: f64) -> (f64, f64) {
        let (mut ta, mut td) = (1e9, 1e9);
        for k in 0..o.x.len() {
            let tk = o.t[k];
            if tk == 0 {
                continue;
            }
            let t = hypot(o.x[k] - x, o.y[k] - y) / o.v[k];
            if tk == 1 {
                if t < ta {
                    ta = t;
                }
            } else {
                let tt = t + 6.0 - o.a[k];
                if tt < td {
                    td = tt;
                }
            }
        }
        (ta, td)
    }
    /// Kale önündeki boş şut yerleri (decide.js:3526).
    pub fn space(&self, src: &[Op], team: f64, bx: f64, by: f64) -> f64 {
        let gx = gx_of(team);
        let dir = if team == 0.0 { -1.0 } else { 1.0 };
        let o = self.bosOy(src, team, bx, by);
        let mut best: f64 = 0.0;
        for (r, an) in spt() {
            let x = gx + dir * r * cos(an);
            let y = GCY + r * sin(an);
            if y < 1.0 || y > GH - 1.0 {
                continue;
            }
            let (ta, td) = self.bos_tara(&o, x, y);
            let tb = self.ucusT(hypot(bx - x, by - y), self.D.arriveFirm, None);
            let own = 1.0 / (1.0 + exp((jmax(ta, tb) - td) / 6.0));
            if 1.0 * own <= best {
                continue;
            }
            let v = own * self.shotQ(src, team, x, y, 0.4);
            if v > best {
                best = v;
            }
        }
        best
    }
    /// Önde boş yer (decide.js:3567).
    pub fn front(&self, src: &[Op], team: f64, bx: f64, by: f64) -> f64 {
        let o = self.bosOy(src, team, bx, by);
        let mut best: f64 = 0.0;
        for (fx, y) in frt() {
            let x = if team == 0.0 { fx } else { GW - fx };
            let (ta, td) = self.bos_tara(&o, x, y);
            let tb = self.ucusT(hypot(bx - x, by - y), self.D.arriveFirm, None);
            let own = 1.0 / (1.0 + exp((jmax(ta, tb) - td) / 6.0));
            let v = own * (self.D.b0 + self.D.b1 * self.threat(team, x, y));
            if v > best {
                best = v;
            }
        }
        best
    }
    /// decide.js:3598
    pub fn Vend(&mut self, src: &[Op], team: f64, x: f64, y: f64, ch: f64) -> f64 {
        let vv = jmax(jmax(jmax(self.shotQ(src, team, x, y, ch), self.build(src, team, x, y)), bosluk(self, src, team, x, y)), self.front(src, team, x, y));
        self.dagin(src, team, x, vv, Some(y))
    }
    /// Topa koşunun değeri (decide.js:3602).
    pub fn runOn(&mut self, w: &[Op], team: f64, h: &Oyuncu, at: Nk) -> f64 {
        let dir = if team == 0.0 { 1.0 } else { -1.0 };
        let vC = 0.85 * self.spd(h);
        let mut best: f64 = 0.0;
        let bkR = w.iter().find(|p| p.team != team && p.bekci() && !p.noTouch);
        let kr = self.kural();
        for f in [6.0, 12.0, 18.0] {
            for s in [-8.0, 0.0, 8.0] {
                let x = at.x + dir * f;
                let y = at.y + s;
                if x < 2.0 || x > GW - 2.0 || y < 2.0 || y > GH - 2.0 {
                    continue;
                }
                if !h.bekci() && !kuyu_girer(h, x, y, hypot(x - h.x, y - h.y) / jmax(0.05, self.vS(h)), false, &kr) {
                    continue;
                }
                let ta = hypot(x - at.x, y - at.y) / vC;
                let mut td = 1e9;
                for p in w {
                    if p.team != team && !p.bekci() {
                        td = jmin(td, hypot(p.x - x, p.y - y) / self.spd(p) + self.TEPKI(p, Tepki::BosTop));
                    }
                }
                let own = 1.0 / (1.0 + exp((ta - td) / 6.0));
                let s0 = match bkR {
                    Some(bk) => self.runSut(team, bk, x, y) * sutEngel(self, w, team, x, y),
                    None => self.shotQ(w, team, x, y, 0.4),
                };
                let vv = jmax(s0, self.D.b0 + self.D.b1 * self.threat(team, x, y));
                let v = own * self.dagin(w, team, x, vv, Some(y));
                if v > best {
                    best = v;
                }
            }
        }
        best
    }
    /// Bekçi yerine geçtikten sonra şutun açıklığı (decide.js:3634; JS gibi tik başına önbellekli).
    pub fn runSut(&self, team: f64, bk: &Oyuncu, x: f64, y: f64) -> f64 {
        let t = self.D._tick;
        {
            let mut c = self.rnb.borrow_mut();
            if c.0 != t || (c.0.is_none() && t.is_none() && false) {
                c.1.clear();
                c.0 = t;
            }
        }
        let ix = crate::mat::jround(2.0 * x);
        let iy = crate::mat::jround(2.0 * y);
        // eskiden metin anahtarı (team, id, en'in bitleri, _yer, ix, iy); aynı eşitlikte sayı anahtarı
        let sk = crate::js_sayi_k;
        let k = [sk(team), sk(bk.id), bk.en.map_or(u64::MAX, f64::to_bits), bk._yer as u64, sk(ix), sk(iy)];
        if let Some(v) = self.rnb.borrow().1.get(&k) {
            return *v;
        }
        let X = ix / 2.0;
        let Y = iy / 2.0;
        let g = self.dis.bekci(bk, nk(X, Y)).unwrap_or(nk(bk.x, bk.y));
        let mut b2 = bk.clone();
        b2.x = g.x;
        b2.y = g.y;
        b2.vx = 0.0;
        b2.vy = 0.0;
        let v = self.sutKapali(&[Op::new(b2)], team, X, Y, 0.4);
        self.rnb.borrow_mut().1.insert(k, v);
        v
    }
    /// decide.js:3656
    pub fn tercihEk(&self, v: f64, d: f64) -> f64 {
        if d <= 0.0 {
            v + d
        } else {
            v + d * jmax(0.0, 1.0 - v)
        }
    }
    /// Savunma dağınıksa değer büyür (decide.js:3659).
    pub fn dagin(&self, src: &[Op], team: f64, x: f64, vv: f64, y: Option<f64>) -> f64 {
        let gx = gx_of(team);
        let dB = match y {
            Some(y) => hypot(x - gx, y - GCY),
            None => 0.0,
        };
        let mut n = 0;
        for o in src {
            if o.team != team && !o.bekci() {
                let ic = match y {
                    Some(_) => hypot(o.x - gx, o.y - GCY) < dB,
                    None => (o.x - gx).abs() < (x - gx).abs(),
                };
                if ic {
                    n += 1;
                }
            }
        }
        let kat = match n {
            0 => 1.6,
            1 => 1.3,
            2 => 1.1,
            _ => 1.0,
        };
        1.0 - pow(jmax(0.0, 1.0 - jmin(1.0, vv)), kat)
    }
    /// Yerin değeri (decide.js:3666).
    pub fn V(&self, src: &[Op], team: f64, x: f64, y: f64, ch: f64) -> f64 {
        let vv = self.V0(src, team, x, y, ch);
        self.dagin(src, team, x, vv, Some(y))
    }
    /// decide.js:3670
    pub fn V0(&self, src: &[Op], team: f64, x: f64, y: f64, ch: f64) -> f64 {
        jmax(self.shotQ(src, team, x, y, ch), self.build(src, team, x, y) * self.tekF(src, team, x, y, ch))
    }
    /// decide.js:3673 chKayip (tek enerji açık)
    pub fn chKayip(&self, ch: f64) -> f64 {
        if ch >= 0.0 {
            jmax(0.05, ch)
        } else {
            0.3
        }
    }
    /// Tek enerji: topun enerjisi yolu taşır mı (decide.js:3674).
    pub fn tekF(&self, src: &[Op], team: f64, x: f64, y: f64, ch: f64) -> f64 {
        if !(ch >= 0.0) {
            return 1.0;
        }
        let P = crate::saha::P;
        let et = etkiler(src);
        let mut kk = Kaynak::Sec(&et);
        let k = crate::alan::k(&mut kk, x, y, team as u8);
        let e = jmax(P.tek_taban, ch);
        let L = yao(self.D._tekL, 18.0);
        let need = L * P.tek_s * (P.attn + jmax(0.0, -k) * P.drain * P.tek_drain * (1.0 - P.pierce * e));
        jmax(0.4, jmin(1.0, 0.55 + (e - need) / 0.3))
    }
    /// Pas yolunun açıklığı (decide.js:3680).
    pub fn laneOk(&self, src: &[Op], team: f64, A: Nk, B: Nk, arr: Option<f64>, lag: Option<f64>) -> f64 {
        let arr = arr.unwrap_or(self.D.arrive);
        let lag = yao(lag, 0.0);
        let L = ya(hypot(B.x - A.x, B.y - A.y), 1.0);
        let ux = (B.x - A.x) / L;
        let uy = (B.y - A.y) / L;
        let mut open = 1.0;
        let faz = self.dis.ALAN_SPR_FAZ().is_some();
        for o in src {
            if o.team == team || o.bekci() || o.noTouch {
                continue;
            }
            let al = (o.x - A.x) * ux + (o.y - A.y) * uy;
            if al <= 0.0 || al >= L + 1.0 {
                continue;
            }
            let pe = (-(o.x - A.x) * uy + (o.y - A.y) * ux).abs();
            let sv = jmax(0.0, self.ucusT(L, arr, Some(al)) + lag - self.TEPKI(o, Tepki::Kovala));
            let R = 1.6 + if faz { self.dis.ALAN_KOSU_M(o, sv, self.vS(o)) } else { sv * self.vS(o) };
            open *= 1.0 - 1.0 / (1.0 + exp((pe - R) / 0.6));
        }
        open
    }
    /// En yakın iki savunucunun cevabı (decide.js:3714): src sırasıyla yeni yerler (None: yerinde).
    pub fn respond(&self, src: &[Op], team: f64, at: Nk, T: f64, kind: &str) -> Vec<Option<Nk>> {
        let gx = gx_of(team);
        let gd = ya(hypot(gx - at.x, GCY - at.y), 1.0);
        let g = nk(at.x + (gx - at.x) / gd * 2.2, at.y + (GCY - at.y) / gd * 2.2);
        let tAt = self.threat(team, at.x, at.y);
        struct Ds {
            i: usize,
            rc: f64,
            tr: f64,
            v: f64,
            ta: f64,
        }
        let mut ds: Vec<Ds> = Vec::new();
        for (i, p) in src.iter().enumerate() {
            if p.team == team || p.bekci() || p.noTouch {
                continue;
            }
            let pres = p.press || is_bas(&p.job, &["pres"]);
            let mr = p.markRef.and_then(|k| src.get(k));
            if let Some(mr) = mr {
                if hypot(mr.x - at.x, mr.y - at.y) > 4.0 && self.threat(team, mr.x, mr.y) > tAt && !pres {
                    continue;
                }
            }
            let rc = if kind == "drive" {
                if pres {
                    0.0
                } else {
                    jmax(0.0, self.TEPKI(p, Tepki::Algi))
                }
            } else {
                jmax(0.0, self.TEPKI(p, Tepki::Kovala))
            };
            let tr = self.dis.gec(p, at.x, at.y, Some(hypot(p.x - at.x, p.y - at.y))).unwrap_or(0.0);
            let v = self.vS(p);
            ds.push(Ds { i, rc, tr, v, ta: rc + tr + hypot(p.x - at.x, p.y - at.y) / v });
        }
        ds.sort_by(|a, b| a.ta.partial_cmp(&b.ta).unwrap_or(std::cmp::Ordering::Equal));
        ds.truncate(2);
        let faz = self.dis.ALAN_SPR_FAZ().is_some();
        let mut out = vec![None; src.len()];
        for d in &ds {
            let p = &src[d.i];
            let kt = (p.x - gx).abs() < (at.x - gx).abs() - 0.5 && hypot(p.x - gx, p.y - GCY) < gd;
            let run = if faz { self.dis.ALAN_KOSU_M(p, T - d.rc - d.tr, d.v) } else { jmax(0.0, T - d.rc - d.tr) * d.v };
            if !kt {
                let ux = (gx - at.x) / gd;
                let uy = (GCY - at.y) / gd;
                let sg = if (p.x - at.x) * -uy + (p.y - at.y) * ux >= 0.0 { 1.0 } else { -1.0 };
                let yn = nk(at.x - uy * sg * 1.8 + 0.6 * ux, at.y + ux * sg * 1.8 + 0.6 * uy);
                let L1 = hypot(yn.x - p.x, yn.y - p.y);
                if run <= L1 {
                    out[d.i] = Some(nk(p.x + (yn.x - p.x) / ya(L1, 1.0) * run, p.y + (yn.y - p.y) / ya(L1, 1.0) * run));
                    continue;
                }
                let L2 = hypot(g.x - yn.x, g.y - yn.y);
                let f = jmin(1.0, (run - L1) / ya(L2, 1.0));
                out[d.i] = Some(nk(yn.x + (g.x - yn.x) * f, yn.y + (g.y - yn.y) * f));
                continue;
            }
            let tg = g;
            let dx = tg.x - p.x;
            let dy = tg.y - p.y;
            let L = hypot(dx, dy);
            out[d.i] = Some(if L <= run { tg } else { nk(p.x + dx / ya(L, 1.0) * run, p.y + dy / ya(L, 1.0) * run) });
        }
        out
    }
}

/// threat (decide.js): yalnız sabitlere bağlı (D.threatK, D.sideK).
pub fn threat_d(D: &crate::model::KafaD, team: f64, x: f64, y: f64) -> f64 {
    let gx = gx_of(team);
    let d = hypot(x - gx, y - GCY);
    exp(-d / D.threatK) * (1.0 - D.sideK * (y - GCY).abs() / GCY)
}
