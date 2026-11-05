//! Zemin değerleri (decide.js 5867–6072, 6545–6590, 6632–6776): kayıp haritası (kayipF), ileri tablosu
//! (ileriKur, ileriTab, ileriDeger, ilerleme), zemin, sahiplik haritası (sahiplikKur, zeminS), yerDeger.
#![allow(non_snake_case, clippy::too_many_arguments)]

use crate::kafa::kafa2_bag::ALAN_HZ;
use crate::mat::dogru;
use crate::mat::exp;
use crate::saha::KR;
use crate::kafa::gecici_dis1::KuralErisim as _;
use super::kafa2_bag as dis;
use super::kafa2_baglam::*;
use super::kafa2_dunya::{bekYerW, predictWorld};
use super::kafa2_imkan::{imkSg, imkTau, imkUzak, imkan, sutS, sutVakit, tasima, IMK_HUC};
use crate::mat::{hypot, jmax, jmin};
use crate::model::Oyuncu;
use crate::saha::{CY as GCY, H as GH, W as GW};

use std::sync::Arc;

/// JS gam() (decide.js:5939): sonraki adımın indirimi (prensip tartısında D._gPr)
#[inline(always)]
pub fn gam(k: &Kafa) -> f64 {
    k.bag._gPr.unwrap_or(k.D.combo)
}

/// JS kayipF(src, team, ch, risk) (decide.js:5867): top (x, y)'de kaybedilirse rakibin kazancı:
/// rakibin oradan imkânı, zemini ya da pası; 6 br'lik ızgarada (önbellekli), ikili ara değerli.
pub struct KayipF {
    src: Arc<Dunya>,
    team: f64,
    risk: f64,
    cc: f64,
    ot: f64,
    c: crate::HizliMap<u64, f64>,
    yq: Option<Vec<(Op, f64)>>,
    gB: Option<f64>,
    oB: f64,
    /// Önceden hesaplama (deney, ALAN_KF_ONDEN): kararın ızgarasıysa takımı; istenen hücreler ve
    /// önceden hesaplanıp henüz istenmemiş olanlar.
    kayit: Option<usize>,
    istenen: crate::HizliMap<u64, (f64, f64)>,
    onden: crate::HizliMap<u64, ()>,
}

pub fn kayipF(k: &Kafa, src: Arc<Dunya>, team: f64, ch: f64, risk: f64) -> KayipF {
    KayipF { src, team, risk, cc: dis::chKayip(k, ch), ot: 1.0 - team, c: Default::default(), yq: None, gB: k.bag._gPr, oB: k.k2.IMK_OFS, kayit: None, istenen: Default::default(), onden: Default::default() }
}

// ------------------------------------------------------------------ kararın ızgarasını önceden hesaplama
//
// Ölçüm (2 maç × 300 tik): kararın kayıp ızgarası ~90–110 hücre ister (tam ızgara 231); art arda iki
// kararda sonrakinin hücre süresinin ~%81–83'ü öncekinin kümesindedir, öncekinin ~%25'i boşa gider.
// Kararın başında önceki kümenin topluca hesaplanması, hücreleri paralel dağıtmanın ilk adımıdır.
// Hücreler sırasından bağımsızdır (ters sırayla ve önceden hesaplamayla 3 maç × 900 tik birebir aynı),
// maç davranışı değişmez; tek çekirdekte yalnız boşa giden hücreler kadar pahalıdır. Varsayılan kapalı.
// Sonuç (3 maç × 600 tik, duvar saati): bütün hücre hesabı sürenin %8–24'ü (callgrind'in %43'ü özyinelemeli
// çağrılarda kapsayıcı maliyeti şişiriyordu), toplu hesaba düşen pay %5–16. 4 çekirdekle en çok %4–12 kazanç:
// hücreleri paralel dağıtmak işçiler arası paketleme bedeline değmeyebilir.

/// ALAN_KF_ONDEN açık mı (bir kez okunur; wasm'da kapalı).
pub fn onden_acik() -> bool {
    if cfg!(target_family = "wasm") {
        return false;
    }
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *A.get_or_init(|| std::env::var("ALAN_KF_ONDEN").is_ok())
}

thread_local! {
    /// takım başına son kararın istediği hücreler
    static SON_KUME: std::cell::RefCell<[Vec<(f64, f64)>; 2]> = Default::default();
    /// [önceden hesaplanan, bunlardan istenen, kararda istenen bütün hücreler]
    static ONDEN_SAY: std::cell::Cell<[u64; 3]> = const { std::cell::Cell::new([0; 3]) };
    /// toplu hesaplamada geçen süre (sn)
    static ONDEN_SN: std::cell::Cell<f64> = const { std::cell::Cell::new(0.0) };
}

/// Toplu (önceden) hücre hesabında geçen toplam süre, saniye: paralel dağıtılabilecek pay.
pub fn onden_sn() -> f64 {
    ONDEN_SN.with(|c| c.get())
}

/// Önceden hesaplama sayaçları: (önceden hesaplanan, bunlardan istenen, kararda istenen bütün hücreler).
pub fn onden_say() -> (u64, u64, u64) {
    let s = ONDEN_SAY.with(|c| c.get());
    (s[0], s[1], s[2])
}
fn say_ekle(i: usize) {
    ONDEN_SAY.with(|c| {
        let mut s = c.get();
        s[i] += 1;
        c.set(s);
    });
}

impl Drop for KayipF {
    fn drop(&mut self) {
        if let Some(t) = self.kayit {
            let v: Vec<(f64, f64)> = self.istenen.values().copied().collect();
            SON_KUME.with(|s| s.borrow_mut()[t] = v);
        }
    }
}

impl KayipF {
    fn kazanImk(&self, k: &mut Kafa, X: f64, Y: f64) -> f64 {
        if imkUzak(self.ot, X, Y) {
            return 0.0;
        }
        let mut q: Option<&Op> = None;
        let mut dq = 1e9;
        for p in self.src.iter() {
            if p.team != self.ot || p.noTouch || bekci(p) {
                continue;
            }
            let d = hypot(p.x - X, p.y - Y);
            if d < dq {
                dq = d;
                q = Some(p);
            }
        }
        let r = q.map(|q| {
            let mut c = (**q).clone();
            c.x = X;
            c.y = Y;
            c.vx = 0.0;
            c.vy = 0.0;
            c
        });
        imkan(k, &self.src, self.ot, nk(X, Y), r.as_ref(), self.cc)
    }
    fn pasR(&mut self, k: &mut Kafa, X: f64, Y: f64) -> f64 {
        if self.yq.is_none() {
            let mut v = Vec::new();
            for q in self.src.iter() {
                if q.team == self.ot && !q.noTouch && !bekci(q) {
                    let d = yerDeger(k, &self.src, self.ot, q.x, q.y, self.cc);
                    v.push((q.clone(), d));
                }
            }
            self.yq = Some(v);
        }
        let A = nk(X, Y);
        let mut b: f64 = 0.0;
        let yq = self.yq.clone().unwrap();
        for (q, v) in &yq {
            let L = hypot(q.x - X, q.y - Y);
            if L < 5.0 || L > dis::pasMax(k) || *v <= b {
                continue;
            }
            let lo = dis::laneOk(k, &self.src, self.ot, A, nk(q.x, q.y), Some(k.D.arriveFirm), Some(k.D.sutHazir)) * bekYol(k, &self.src, self.ot, A, nk(q.x, q.y), Some(k.D.sutHazir));
            b = jmax(b, lo * v);
        }
        gam(k) * b
    }
    /// Kararın ızgarası olarak işaretle: istenen hücreler kaydedilir ve (ALAN_KF_ONDEN) aynı takımın
    /// önceki kararının hücreleri şimdi topluca hesaplanır.
    pub fn karar_icin(&mut self, k: &mut Kafa) {
        if !onden_acik() {
            return;
        }
        let t = if self.team == 0.0 { 0 } else { 1 };
        self.kayit = Some(t);
        let kume = SON_KUME.with(|s| std::mem::take(&mut s.borrow_mut()[t]));
        let t0 = std::time::Instant::now();
        self.hucreler(k, &kume);
        let dt = t0.elapsed().as_secs_f64();
        ONDEN_SN.with(|c| c.set(c.get() + dt));
    }
    /// Verilen hücrelerin henüz hesaplanmamış olanlarını hesapla (paralel dağıtılacak toplu iş).
    pub fn hucreler(&mut self, k: &mut Kafa, kume: &[(f64, f64)]) {
        for &(X, Y) in kume {
            let key = (1e3 * X + Y).to_bits();
            if self.c.contains_key(&key) {
                continue;
            }
            self.hucre_hesap(k, X, Y);
            self.onden.insert(key, ());
            say_ekle(0);
        }
    }
    fn hucre(&mut self, k: &mut Kafa, X: f64, Y: f64) -> f64 {
        if self.kayit.is_some() {
            let key = (1e3 * X + Y).to_bits();
            if self.istenen.insert(key, (X, Y)).is_none() {
                say_ekle(2);
                if self.onden.remove(&key).is_some() {
                    say_ekle(1);
                }
            }
        }
        self.hucre_hesap(k, X, Y)
    }
    fn hucre_hesap(&mut self, k: &mut Kafa, X: f64, Y: f64) -> f64 {
        let key = (1e3 * X + Y).to_bits();
        if let Some(v) = self.c.get(&key) {
            return *v;
        }
        let g1 = k.bag._gPr;
        let o1 = k.k2.IMK_OFS;
        let ba = g1 != self.gB || o1 != self.oB;
        if ba {
            k.bag._gPr = self.gB;
            k.k2.IMK_OFS = self.oB;
        }
        let a = self.kazanImk(k, X, Y);
        let b = zeminS(k, &self.src.clone(), 1.0 - self.team, X, Y, self.cc, None);
        let c = self.pasR(k, X, Y);
        let v = jmax(jmax(a, b), c);
        if ba {
            k.bag._gPr = g1;
            k.k2.IMK_OFS = o1;
        }
        self.c.insert(key, v);
        v
    }
    /// JS `lossAt(x, y)`
    pub fn deger(&mut self, k: &mut Kafa, x: f64, y: f64) -> f64 {
        let KG = IMK_HUC;
        let gx0 = (x / KG).floor() * KG;
        let gy0 = (y / KG).floor() * KG;
        let fx = (x - gx0) / KG;
        let fy = (y - gy0) / KG;
        let a = self.hucre(k, gx0, gy0);
        let b = self.hucre(k, gx0 + KG, gy0);
        let c = self.hucre(k, gx0, gy0 + KG);
        let d = self.hucre(k, gx0 + KG, gy0 + KG);
        self.risk * ((1.0 - fx) * (1.0 - fy) * a + fx * (1.0 - fy) * b + (1.0 - fx) * fy * c + fx * fy * d)
    }
}

// ------------------------------------------------------------------ ileri tablosu

const ITX: f64 = 6.0;
const ITY: f64 = 6.0;

/// JS ileriKur(m)'un sonucu: iki takım için (x, y)'de topun ileri değeri (taşıma, pas, sürme zinciri).
#[derive(Debug)]
pub struct IleriTab {
    pub V: [Vec<f64>; 2],
    pub nx: usize,
    pub ny: usize,
}
impl IleriTab {
    fn oku_v(&self, v: &[f64], x: f64, y: f64) -> f64 {
        let (nx, ny) = (self.nx as f64, self.ny);
        let fx = jmax(0.0, jmin(nx - 1.001, (x - 2.0) / ITX));
        let fy = jmax(0.0, jmin(ny as f64 - 1.001, (y - 2.0) / ITY));
        let i = fx.floor() as usize;
        let j = fy.floor() as usize;
        let u = fx - i as f64;
        let w = fy - j as f64;
        let kk = i * ny + j;
        v[kk] * (1.0 - u) * (1.0 - w) + v[kk + ny] * u * (1.0 - w) + v[kk + 1] * (1.0 - u) * w + v[kk + ny + 1] * u * w
    }
    pub fn oku(&self, t: f64, x: f64, y: f64) -> f64 {
        self.oku_v(&self.V[t as usize], x, y)
    }
}

struct Yol {
    x: f64,
    y: f64,
    lo: f64,
    mx: f64,
    my: f64,
}

/// JS ileriKur(m) (decide.js:5941): diziliş yerleşmiş varsayılarak her hücrenin değeri (40 yineleme).
pub fn ileriKur(k: &mut Kafa, m: &MacGor) -> Option<IleriTab> {
    let nx = ((GW - 4.0) / ITX).floor() as usize + 1;
    let ny = ((GH - 4.0) / ITY).floor() as usize + 1;
    let ch = 0.3;
    let G = k.D.combo;
    let mut hucre = Vec::new();
    for i in 0..nx {
        for j in 0..ny {
            hucre.push(nk(jmin(GW - 2.0, 2.0 + i as f64 * ITX), jmin(GH - 2.0, 2.0 + j as f64 * ITY)));
        }
    }
    let dinc: Dunya = m
        .ps
        .iter()
        .map(|p| {
            let mut d = Oyuncu::bos();
            d.id = p.id;
            d.team = p.team;
            d.i = p.i;
            d.role = p.role.clone();
            d.name = p.name.clone();
            d.slot = p.slot.clone();
            d.rslot = p.rslot.clone();
            d.slot0 = p.slot0.clone();
            d.rslot0 = p.rslot0.clone();
            d.a = p.a.clone();
            d.yet = p.yet.clone();
            d.R = p.R;
            d.D = p.D;
            d.rh = p.rh.clone();
            d.rs = p.rs.clone();
            d.tl = p.tl.clone();
            d.eg = p.eg.clone();
            d.cd = 0.0;
            d.en = Some(1.0);
            d.x = p.x;
            d.y = p.y;
            d.vx = 0.0;
            d.vy = 0.0;
            // JS: tx/ty tanımsız (aritmetikte NaN gibi davranır)
            d.tx = f64::NAN;
            d.ty = f64::NAN;
            Arc::new(d)
        })
        .collect();
    let kd = k.dis.clone();
    let hat = |t: usize| -> f64 {
        let blok = m.tac.get(1 - t).map(|x| x.blok.clone()).unwrap_or_default();
        kd.S_prHat(&blok).unwrap_or(62.0 * (GW / 100.0))
    };
    let tasi = |p: &Op, g: Nokta| -> Op {
        let mut q = (**p).clone();
        q.x = g.x;
        q.y = g.y;
        q.vx = 0.0;
        q.vy = 0.0;
        Arc::new(q)
    };
    let yap = |k: &mut Kafa, t: usize| -> Vec<(f64, Vec<Yol>)> {
        let tf = t as f64;
        hucre
            .iter()
            .map(|&E| {
                let mut sav: Dunya = {
                    let fm = dis::Fm { tac: &m.tac, ps: &dinc, holder: None };
                    dinc.iter().filter(|p| p.team != tf).map(|p| tasi(p, if bekci(p) { dis::AS_bekci(k, p, E, Some(&fm)) } else { dis::AS_slotBase(k, &fm, p, E, false) })).collect()
                };
                if (if t == 0 { GW - E.x } else { E.x }) < hat(t) {
                    let mut bi: Option<usize> = None;
                    let mut bd = 1e9;
                    for (i, o) in sav.iter().enumerate() {
                        if bekci(o) {
                            continue;
                        }
                        let d = hypot(o.x - E.x, o.y - E.y);
                        if d < bd {
                            bd = d;
                            bi = Some(i);
                        }
                    }
                    if let Some(bi) = bi {
                        let g = dis::AS_gside(k, E, 1.0 - tf, 2.2);
                        sav[bi] = kxy(&sav[bi], g.x, g.y);
                    }
                }
                let huc: Dunya = {
                    let fm2 = dis::Fm { tac: &m.tac, ps: &sav, holder: None };
                    dinc.iter().filter(|p| p.team == tf).map(|p| tasi(p, if bekci(p) { dis::AS_bekci(k, p, E, Some(&fm2)) } else { dis::AS_slotBase(k, &fm2, p, E, true) })).collect()
                };
                let mut W: Dunya = sav.clone();
                W.extend(huc.iter().cloned());
                let mut tsi: Option<&Op> = None;
                let mut td = f64::INFINITY;
                for a in huc.iter() {
                    if bekci(a) || a.noTouch {
                        continue;
                    }
                    let d = hypot(a.x - E.x, a.y - E.y);
                    if tsi.is_none() || d < td {
                        td = d;
                        tsi = Some(a);
                    }
                }
                let r: Option<Oyuncu> = tsi.map(|t| {
                    let mut c = (**t).clone();
                    c.x = E.x;
                    c.y = E.y;
                    c.vx = 0.0;
                    c.vy = 0.0;
                    c
                });
                let base = tasima(k, &W, tf, r.as_ref(), E.x, E.y, ch);
                let mut yol: Vec<Yol> = Vec::new();
                let mut sur: Vec<Yol> = Vec::new();
                let dr = if t == 0 { 1.0 } else { -1.0 };
                if let Some(r) = &r {
                    for dy in [-ITY, 0.0, ITY] {
                        let B = nk(E.x + dr * ITX, E.y + dy);
                        if B.x < 2.0 || B.x > GW - 2.0 || B.y < 2.0 || B.y > GH - 2.0 {
                            continue;
                        }
                        let L = hypot(B.x - E.x, B.y - E.y);
                        let u = nk((B.x - E.x) / L, (B.y - E.y) / L);
                        let Ts = jmax(4.0, dis::surSure(k, r, L, Some(u), None));
                        let p = jmax(0.0, jmin(1.0, dis::keepP(k, r, &W, Some(u), Ts)));
                        if p > 0.05 {
                            sur.push(Yol { x: B.x, y: B.y, lo: p, mx: (E.x + B.x) / 2.0, my: (E.y + B.y) / 2.0 });
                        }
                    }
                }
                for a in huc.iter() {
                    if bekci(a) || a.noTouch || tsi.map(|t| ayni_varlik(t, a)).unwrap_or(false) {
                        continue;
                    }
                    let L = hypot(a.x - E.x, a.y - E.y);
                    if L < 5.0 || L > dis::pasMax(k) {
                        continue;
                    }
                    let lo = dis::laneOk(k, &W, tf, E, nk(a.x, a.y), Some(k.D.arriveFirm), Some(k.D.sutHazir)) * bekYol(k, &W, tf, E, nk(a.x, a.y), Some(k.D.sutHazir));
                    if lo > 0.05 {
                        yol.push(Yol { x: a.x, y: a.y, lo, mx: (E.x + a.x) / 2.0, my: (E.y + a.y) / 2.0 });
                    }
                }
                yol.extend(sur);
                (base, yol)
            })
            .collect()
    };
    let H = [yap(k, 0), yap(k, 1)];
    let mut tab = IleriTab { V: [H[0].iter().map(|c| c.0).collect(), H[1].iter().map(|c| c.0).collect()], nx, ny };
    for _ in 0..40 {
        let mut fark: f64 = 0.0;
        let mut Y: [Vec<f64>; 2] = [Vec::new(), Vec::new()];
        for t in 0..2 {
            Y[t] = H[t]
                .iter()
                .enumerate()
                .map(|(kk, c)| {
                    let mut v = c.0;
                    for a in &c.1 {
                        v = jmax(v, G * (a.lo * tab.oku_v(&tab.V[t], a.x, a.y) - (1.0 - a.lo) * tab.oku_v(&tab.V[1 - t], a.mx, a.my)));
                    }
                    fark = jmax(fark, (v - tab.V[t][kk]).abs());
                    v
                })
                .collect();
        }
        let [y0, y1] = Y;
        tab.V = [y0, y1];
        if fark < 1e-4 {
            break;
        }
    }
    Some(tab)
}

/// JS ileriTab() (decide.js:6546): maçın ileri tablosu (taktik ya da kadro değişince yeniden).
pub fn ileriTab(k: &mut Kafa) -> Option<Arc<IleriTab>> {
    let m = k.mac.clone()?;
    let ayni_tik = matches!(&k.k2.IT, Some((t, _, _)) if *t == m.tick);
    if !ayni_tik {
        let key = format!(
            "{:?}|{:?}",
            m.tac,
            m.ps.iter().map(|p| (p.id, p.team, p.role.clone(), format!("{:?}", p.slot), format!("{:?}", p.rslot))).collect::<Vec<_>>()
        );
        let eski = k.k2.IT.take();
        let degisti = match &eski {
            Some((_, k0, _)) => *k0 != key,
            None => true,
        };
        if degisti {
            // D'nin `_` alanları (_tick dışında) geçici olarak boşaltılır; D._mac sade maç görünümü.
            let sakla_d = k.bag.clone();
            super::kafa2_baglam::d_alt_bosalt(&mut k.bag);
            let sakla_k2 = (k.k2._noHold, k.k2._noFake, k.k2._dalgada);
            k.k2._noHold = false;
            k.k2._noFake = false;
            k.k2._dalgada = false;
            let sakla_mac = k.mac.replace(MacGor { tick: 0.0, len: m.len, tac: m.tac.clone(), ps: m.ps.clone(), holder: None });
            let tab = ileriKur(k, &m).map(Arc::new);
            k.mac = sakla_mac;
            k.bag = sakla_d;
            k.k2._noHold = sakla_k2.0;
            k.k2._noFake = sakla_k2.1;
            k.k2._dalgada = sakla_k2.2;
            k.k2.IT = Some((m.tick, key, tab));
        } else {
            let (_, k0, tb) = eski.unwrap();
            k.k2.IT = Some((m.tick, k0, tb));
        }
    }
    k.k2.IT.as_ref().and_then(|x| x.2.clone())
}

/// JS ileriDeger(team, x, y) (decide.js:6585)
pub fn ileriDeger(k: &mut Kafa, team: f64, x: f64, y: f64) -> f64 {
    match ileriTab(k) {
        Some(T) => T.oku(team, x, y),
        None => k.D.b1 * dis::threat(k, team, x, y),
    }
}
/// JS ilerleme(w, team, x, y) (decide.js:6589)
pub fn ilerleme(k: &mut Kafa, w: &[Op], team: f64, x: f64, y: f64) -> f64 {
    let d = ileriDeger(k, team, x, y);
    let f = dis::freeAt(k, w, team, x, y);
    if crate::iz_acik() && std::env::var("ALAN_YD_IZ").is_ok() {
        eprintln!("  iler {} {} ileriDeger {:.9} freeAt {:.9}", x, y, d, f);
    }
    d * f
}
/// JS zemin(w, team, x, y, ch, k) (decide.js:6590)
pub fn zemin(k: &mut Kafa, w: &[Op], team: f64, x: f64, y: f64, ch: f64, kq: Option<&Oyuncu>) -> f64 {
    let a = tasima(k, w, team, kq, x, y, ch);
    let b = ilerleme(k, w, team, x, y);
    jmax(a, b)
}

// ------------------------------------------------------------------ sahiplik haritası

pub const SAH_ADIM: usize = 3;
const SAH_YON: [(i64, i64); 8] = [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)];

/// JS sahiplikKur'un sonucu: iki takımın 6 br'lik ızgarada top sahipliğinin değeri.
#[derive(Debug)]
pub struct Sahiplik {
    pub N: [Vec<f64>; 2],
    pub NX: usize,
    pub NY: usize,
}
impl Sahiplik {
    fn oku_n(N: &[f64], NX: usize, NY: usize, x: f64, y: f64) -> f64 {
        let KG = IMK_HUC;
        let fx0 = jmax(0.0, jmin(NX as f64 - 1.000001, x / KG));
        let fy0 = jmax(0.0, jmin(NY as f64 - 1.000001, y / KG));
        let i = fx0.floor() as usize;
        let j = fy0.floor() as usize;
        let fx = fx0 - i as f64;
        let fy = fy0 - j as f64;
        (1.0 - fx) * (1.0 - fy) * N[i * NY + j] + fx * (1.0 - fy) * N[(i + 1) * NY + j] + (1.0 - fx) * fy * N[i * NY + j + 1] + fx * fy * N[(i + 1) * NY + j + 1]
    }
    /// JS `SAH_F(t, x, y)`
    pub fn oku(&self, t: f64, x: f64, y: f64) -> f64 {
        Self::oku_n(&self.N[t as usize], self.NX, self.NY, x, y)
    }
}

struct SahYol {
    x: f64,
    y: f64,
    k: f64,
    lx: f64,
    ly: f64,
}

/// JS sahiplikKur(w, team, ch, B0) (decide.js:6634): her hücrede şut değeri (S) ve komşu hücreye
/// sürme / arkadaşa pas yolları; üç adımlık değer yinelemesi (rakibin karşı değeriyle).
pub fn sahiplikKur(k: &mut Kafa, w: &[Op], team: f64, ch: f64, B0: Option<Nokta>) -> Sahiplik {
    let KG = IMK_HUC;
    let NX = (GW / KG).round() as usize + 1;
    let NY = (GH / KG).round() as usize + 1;
    let g = gam(k);
    let vC = 0.1955;
    let PM = dis::pasMax(k);
    struct F {
        S: Vec<f64>,
        H: Vec<Vec<SahYol>>,
        N: Vec<f64>,
        gxo: f64,
    }
    let mut FF: Vec<F> = Vec::new();
    let kd = k.dis.clone();
    for t in [0.0, 1.0] {
        let cT = if t == team { ch } else { dis::chKayip(k, ch) };
        struct R {
            p: Op,
            G: Option<dis::GecTab>,
            v: f64,
            R: f64,
            rc: f64,
        }
        let rak: Vec<R> = w
            .iter()
            .filter(|p| p.team != t && !p.noTouch && !bekci(p))
            .map(|p| R { p: p.clone(), G: dis::AS_gecTab(k, p), v: dis::AS_vSp(k, p), R: jmin(2.0, crate::temas::menzil(p, 0.35)), rc: jmax(0.0, dis::ALAN_TEPKI(k, p, "kovala")) })
            .collect();
        let rv = |x: f64, y: f64| -> f64 {
            let mut m = 1e9;
            for o in &rak {
                let d = jmax(0.0, hypot(o.p.x - x, o.p.y - y) - o.R);
                if !(o.rc + d / o.v >= m) {
                    m = jmin(m, o.rc + o.G.as_ref().map_or(0.0, |g| kd.gecOf(g, x, y, Some(d))) + d / o.v);
                }
            }
            m
        };
        let biz: Vec<Op> = w.iter().filter(|p| p.team == t && !p.noTouch).cloned().collect();
        let mut S = vec![0.0; NX * NY];
        let mut H: Vec<Vec<SahYol>> = (0..NX * NY).map(|_| Vec::new()).collect();
        let gxo = if t == 0.0 { GW - KG } else { KG };
        for i in 0..NX {
            for j in 0..NY {
                let x = i as f64 * KG;
                let y = j as f64 * KG;
                let c = i * NY + j;
                if !imkUzak(t, x, y) {
                    let Tc = match B0 {
                        Some(b) => jmax(30.0, dis::ucusT(k, hypot(x - b.x, y - b.y), k.D.arriveFirm, None)),
                        None => 30.0,
                    };
                    let P = nk(x, y);
                    let wb0 = bekYerW(k, w, t, P, Some(Tc));
                    let kOf = |wb: &[Op]| -> Option<Oyuncu> {
                        let mut bd = 1e9;
                        let mut k0: Option<&Op> = None;
                        for o in wb {
                            if o.team != t || o.noTouch || bekci(o) {
                                continue;
                            }
                            let dd = hypot(o.x - x, o.y - y);
                            if dd < bd {
                                bd = dd;
                                k0 = Some(o);
                            }
                        }
                        k0.map(|o| (**o).clone())
                    };
                    let sOf = |k: &mut Kafa, wb: &[Op]| -> f64 {
                        let k0 = kOf(wb);
                        sutS(k, wb, t, x, y, cT, k0.as_ref(), None) * sutVakit(k, wb, t, x, y, k0.as_ref())
                    };
                    let mut sv = sOf(k, &wb0);
                    if sv > 0.005 {
                        let pw = predictWorld(k, w, t, P, Tc, false, None, false, None);
                        let wb = bekYerW(k, &pw, t, P, Some(Tc));
                        sv = sOf(k, &wb);
                    }
                    S[c] = sv;
                }
                let mut yol: Vec<SahYol> = Vec::new();
                for (di, dj) in SAH_YON {
                    let a = i as i64 + di;
                    let b = j as i64 + dj;
                    if a < 0 || b < 0 || a >= NX as i64 || b >= NY as i64 {
                        continue;
                    }
                    let x2 = a as f64 * KG;
                    let y2 = b as f64 * KG;
                    let tk = hypot(x2 - x, y2 - y) / vC;
                    let kk = jmin(imkSg((rv((x + x2) / 2.0, (y + y2) / 2.0) - tk / 2.0) / imkTau(tk / 2.0)), imkSg((rv(x2, y2) - tk) / imkTau(tk)));
                    yol.push(SahYol { x: x2, y: y2, k: kk, lx: x2, ly: y2 });
                }
                for q in &biz {
                    let L = hypot(q.x - x, q.y - y);
                    if L < 5.0 || L > PM {
                        continue;
                    }
                    let A = nk(x, y);
                    let kk = dis::laneOk(k, w, t, A, nk(q.x, q.y), Some(k.D.arriveFirm), Some(k.D.sutHazir)) * bekYol(k, w, t, A, nk(q.x, q.y), Some(k.D.sutHazir));
                    yol.push(SahYol { x: q.x, y: q.y, k: kk, lx: (x + q.x) / 2.0, ly: (y + q.y) / 2.0 });
                }
                H[c] = yol;
            }
        }
        let N = S.clone();
        FF.push(F { S, H, N, gxo });
    }
    for _ in 0..SAH_ADIM {
        let mut N2 = [vec![0.0; NX * NY], vec![0.0; NX * NY]];
        for t in 0..2 {
            let f = &FF[t];
            let o = &FF[1 - t];
            let kac = Sahiplik::oku_n(&o.N, NX, NY, f.gxo, GCY);
            for c in 0..f.S.len() {
                let mut b = if f.S[c] > 0.0 { f.S[c] - (1.0 - f.S[c]) * kac } else { f64::NEG_INFINITY };
                for e in &f.H[c] {
                    let v = e.k * g * Sahiplik::oku_n(&f.N, NX, NY, e.x, e.y) - (1.0 - e.k) * Sahiplik::oku_n(&o.N, NX, NY, e.lx, e.ly);
                    if v > b {
                        b = v;
                    }
                }
                N2[t][c] = b;
            }
        }
        let [a, b] = N2;
        FF[0].N = a;
        FF[1].N = b;
    }
    let n1 = FF.pop().unwrap().N;
    let n0 = FF.pop().unwrap().N;
    Sahiplik { N: [n0, n1], NX, NY }
}

/// JS zeminS(w, team, x, y, ch, k) (decide.js:6747): sahiplik haritası varsa ondan
pub fn zeminS(k: &mut Kafa, w: &[Op], team: f64, x: f64, y: f64, ch: f64, kq: Option<&Oyuncu>) -> f64 {
    match &k.SAH_F {
        Some(f) => f.oku(team, x, y),
        None => zemin(k, w, team, x, y, ch, kq),
    }
}

/// JS yerDeger(w, team, x, y, ch) (decide.js:6773): oyuncunun yerinin değeri (şut ya da zemin)
pub fn yerDeger(k: &mut Kafa, w: &[Op], team: f64, x: f64, y: f64, ch: f64) -> f64 {
    let wb = bekYerW(k, w, team, nk(x, y), None);
    let a = dis::shotQ(k, &wb, team, x, y, ch) * sutVakit(k, w, team, x, y, None);
    let b = k.D.combo * zemin(k, w, team, x, y, ch, None);
    jmax(a, b)
}

// ------------------------------------------------------------------ yerleşimin de kullandıkları
// (MIMARI.md Aşama 2: yerleşim kararın dosyalarına bağlanmaz; bunlar değerlendirme katmanında)

/// JS bekYol(w, team, A, B, lag) (decide.js:6876): A→B pasını rakip Bekçi'nin kesmeme olasılığı.
pub fn bekYol(k: &mut Kafa, w: &[Op], team: f64, A: Nokta, B: Nokta, lag: Option<f64>) -> f64 {
    let kb = match w.iter().find(|o| o.team != team && bekci(o) && !o.noTouch) {
        Some(b) => b.clone(),
        None => return 1.0,
    };
    let L = veya(hypot(B.x - A.x, B.y - A.y), 1.0);
    let ux = (B.x - A.x) / L;
    let uy = (B.y - A.y) / L;
    let s = jmax(0.0, jmin(L, (kb.x - A.x) * ux + (kb.y - A.y) * uy));
    let cx0 = A.x + ux * s;
    let cy0 = A.y + uy * s;
    if hypot(cx0 - if t0(kb.team) { 0.0 } else { GW }, cy0 - GCY) > KR + 6.0 {
        return 1.0;
    }
    let Q = &crate::core_sabit::Q;
    let t = dis::ucusT(k, s, k.D.arriveFirm, None) + veyo(lag, 0.0);
    let rc = crate::temas::bek_re(&kb);
    let vk = (0.17 + 0.006 * ALAN_HZ(&kb)) * dis::ALAN_SPRE(k, &kb);
    let R = Q.bek_body + jmin(crate::temas::bek_dl(&kb), jmax(0.0, t - rc) * crate::temas::bek_dv(&kb)) + jmax(0.0, t - rc - 2.0) * vk * 0.6;
    1.0 / (1.0 + exp(-(hypot(kb.x - cx0, kb.y - cy0) - R) / 0.3))
}

/// JS riskOf(h, tac) (decide.js:5423)
pub fn riskOf(k: &Kafa, h: &Oyuncu, tac: Option<&Tac>) -> f64 {
    let tr = match tac.and_then(|t| t.risk) {
        Some(r) => 1.4 - 0.8 * r,
        None => 1.0,
    };
    let pr = h.tl.as_ref().and_then(|t| t.huc.get("risk").copied()).or(h.eg.as_ref().map(|e| e.rk)).unwrap_or(0.5);
    let mk = match tac.and_then(|t| t.macK) {
        Some(m) if dogru(m) => m,
        _ => 1.0,
    };
    k.D.riskBase * tr * (1.4 - 0.8 * pr) * mk
}

/// JS kayipYer(src, team, ch, h, tac) (decide.js:8801): kayıp haritası (dışarıdan)
pub fn kayipYer(k: &mut Kafa, src: &Arc<Dunya>, team: f64, ch: f64, h: &Oyuncu, tac: Option<&Tac>) -> KayipF {
    let r = riskOf(k, h, tac);
    kayipF(k, src.clone(), team, ch, r)
}
