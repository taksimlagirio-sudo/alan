//! Kafanın dış dünyası (kafa1 `Dis`): maçın ve yerleşimin kafaya açtığı işlevler.
//!
//! Kafa `&self` ile okur; maçın durumu kararın başında [`MacDis::kur`] ile alınır (JS'te kafa `D._mac`
//! üstünden maçı canlı okur; bir kararın içinde maç değişmediği için anlık görüntü aynıdır).
#![allow(non_snake_case)]

use crate::kafa::gecici_dis1::{self as d1, Dis};
use crate::kafa::kafa1_js::{nk, Nk};
use crate::mac::enerji as en;
use crate::mac::{ikili, MacKural};
use crate::model::{JMShGs, Mac, Oyuncu};
use crate::oyuncu::TepkiS;
use crate::sekil;
use crate::temas::Kural;
use std::sync::Arc;

/// Kararın başındaki maç görüntüsü.
pub struct MacDis {
    /// maçın sprint kuralı (son karar veren taşıyıcı, tik, taşıyıcı, alıcı): JS bunları canlı okur;
    /// kafa her çağrıldığında `kural_tazele` ile güncellenir
    pub kural: std::cell::Cell<MacKural>,
    pub ch: std::cell::Cell<Option<f64>>,
    /// (id, team, x, y)
    pub holder: std::cell::Cell<Option<(f64, f64, f64, f64)>>,
    /// takım başına: etkin taktik (yerleşimin tipi)
    pub tacE: [sekil::gecici_dis::TacE; 2],
    /// takım başına: (m.tac[k].siddet, yz, macEff(m,k,{}).macK ?? 1)
    pub siddet: [(Option<f64>, bool, f64); 2],
    pub enHarcama: [Option<f64>; 2],
    pub gs: std::cell::RefCell<Option<JMShGs>>,
    pub prT: std::cell::RefCell<Option<d1::PrT>>,
    /// yerleşimin tehlike'si için: son seçenekler, oyuncu kimlikleri, maçın tiki
    pub mac: Arc<Mac>,
}

fn hafif(m: &mut Mac) -> Mac {
    // olay günlüğü ve kafa günlüğü kopyalanmaz
    let ev = std::mem::take(&mut m.events);
    let kg = m.mac._kafa.take();
    let ps = std::mem::take(&mut m.ps);
    let mut c = m.clone();
    m.events = ev;
    m.mac._kafa = kg;
    c.ps = ps.clone();
    m.ps = ps;
    c
}

impl MacDis {
    pub fn kur(m: &mut Mac) -> MacDis {
        let kural = MacKural::of(m);
        let tacE = [0u8, 1].map(|t| {
            let tq = crate::mac::tac::tacEff(m, t as f64);
            let kp = tq._kp.is_some();
            sekil::gecici_dis::TacE { t: tq, kp }
        });
        let siddet = [0usize, 1].map(|t| {
            let tq = m.tac.get(t).cloned().unwrap_or_default();
            let yz = tq.yz == Some(true);
            let mk = crate::mac::tac::macEff(m, t as f64, Default::default()).macK.unwrap_or(1.0);
            (tq.siddet, yz, mk)
        });
        let enHarcama = [0usize, 1].map(|t| m.tac.get(t).and_then(|q| q.enHarcama));
        let holder = m.holder.map(|h| {
            let p = &m.ps[h];
            (p.id, p.team, p.x, p.y)
        });
        let gs = m.mac.sekil.sh.gs.as_ref().map(|g| JMShGs { t: g.t, dt: g.dt as f64, on: g.on, onP: g.onP, oy: g.oy, ..Default::default() });
        MacDis { kural: std::cell::Cell::new(kural), ch: std::cell::Cell::new(Some(m.ch)), holder: std::cell::Cell::new(holder), tacE, siddet, enHarcama, gs: std::cell::RefCell::new(gs), prT: std::cell::RefCell::new(m.mac.prT_k.clone()), mac: Arc::new(hafif(m)) }
    }
    fn takim_tac(&self, t: f64) -> &sekil::gecici_dis::TacE {
        &self.tacE[if t == 1.0 { 1 } else { 0 }]
    }
}

fn gecY_k(y: &sekil::gec::GecY) -> Arc<d1::GecY> {
    Arc::new(d1::GecY { g: y.g, m: Some(y.m.to_vec()) })
}
thread_local! {
    /// Yerleşimin gecikme eğrilerinin (önbellekte, değişmez) kafa biçimi: bir kez dönüştürülür.
    /// Anahtar eğrinin Arc adresi; yerleşimin Arc'ı da tutulur ki adres başka eğriye geçmesin.
    static GECY_K: std::cell::RefCell<crate::HizliMap<usize, (Arc<sekil::gec::GecY>, Arc<d1::GecY>)>> = std::cell::RefCell::new(Default::default());
}
fn gecY_k_arc(y: &Arc<sekil::gec::GecY>) -> Arc<d1::GecY> {
    let k = Arc::as_ptr(y) as usize;
    GECY_K.with(|c| {
        if let Some((_, d)) = c.borrow().get(&k) {
            return d.clone();
        }
        let d = gecY_k(y);
        c.borrow_mut().insert(k, (y.clone(), d.clone()));
        d
    })
}
fn gecTab_k(g: &sekil::gec::GecTab) -> d1::GecTab {
    d1::GecTab { vx: g.vx, vy: g.vy, s0: g.s0, x: g.x, y: g.y, t: g.t.to_vec(), Y: g.Y.iter().map(gecY_k_arc).collect(), vm: g.vm }
}
fn gecTab_s(g: &d1::GecTab) -> sekil::gec::GecTab {
    let Y: Vec<Arc<sekil::gec::GecY>> = g
        .Y
        .iter()
        .map(|y| {
            let mut m = [0.0; 81];
            if let Some(v) = &y.m {
                for (i, x) in v.iter().take(81).enumerate() {
                    m[i] = *x;
                }
            }
            Arc::new(sekil::gec::GecY { g: y.g, m })
        })
        .collect();
    let mut t = [0.0; 9];
    t.copy_from_slice(&g.t[..9]);
    sekil::gec::GecTab { vx: g.vx, vy: g.vy, s0: g.s0, x: g.x, y: g.y, t, Y: Y.try_into().unwrap(), vm: g.vm }
}

impl Dis for MacDis {
    // ---- enerji / sprint ----
    fn ALAN_SPRE(&self, p: &Oyuncu) -> f64 {
        sekil::ALAN_SPRE(p, &self.kural.get())
    }
    fn ALAN_SPR(&self, p: &Oyuncu) -> f64 {
        en::ALAN_SPR(p, &self.kural.get())
    }
    fn ALAN_SPR_HK(&self, p: &Oyuncu) -> f64 {
        en::ALAN_SPR_HK(p)
    }
    fn ALAN_SPR_FAZ(&self) -> Option<d1::SprFaz> {
        let F = &en::SPR_FAZ;
        let b = |v: bool| if v { 1.0 } else { 0.0 };
        Some(d1::SprFaz { ust: b(F.ust), hkPay: F.hkPay, tepeT: F.tepeT, after: F.after, dolT: F.dolT, bedel: F.bedel, patT: F.patT, frenT: F.frenT, patK: F.patK, ayir: b(F.ayir), itKos: b(F.itKos), topla: b(F.topla), toplaBos: F.toplaBos, itBos: None, peak: None })
    }
    fn ALAN_KOSU_T(&self, p: &Oyuncu, d: f64, v1: f64) -> f64 {
        en::ALAN_KOSU_T(p, d, v1, &self.kural.get())
    }
    fn ALAN_KOSU_M(&self, p: &Oyuncu, t: f64, v1: f64) -> f64 {
        en::ALAN_KOSU_M(p, t, v1, &self.kural.get())
    }
    fn ALAN_EN3_KUL(&self, p: &Oyuncu, e: f64, bizde: bool, bx: f64, by: f64) -> f64 {
        en::ALAN_EN3_KUL(p, e, bizde, Some((bx, by)), &self.kural.get())
    }
    fn ALAN_EN3_SGIR(&self, p: &Oyuncu, bed: f64) -> f64 {
        en::ALAN_EN3_SGIR(p, bed)
    }
    fn ALAN_EN2_TS(&self, p: &Oyuncu, e: Option<f64>) -> f64 {
        en::ALAN_EN2_TS(p, e)
    }
    fn ALAN_EN2_SB(&self, p: &Oyuncu) -> f64 {
        en::ALAN_EN2_SB(p)
    }
    fn ALAN_EN2_SD(&self, p: &Oyuncu) -> f64 {
        en::ALAN_EN2_SD(p)
    }
    fn ALAN_EN3_DOL(&self, p: &Oyuncu, sahip: bool, nd: f64, yerK: f64) -> f64 {
        en::ALAN_EN3_DOL(p, sahip, nd, yerK)
    }
    fn ALAN_EN3_TEPE(&self, e: Option<f64>, k: f64) -> f64 {
        en::ALAN_EN3_TEPE(e, k)
    }
    fn ALAN_EN_S(&self, p: &Oyuncu) -> f64 {
        en::ALAN_EN_S(p)
    }
    fn ALAN_EN2(&self) -> d1::En2 {
        let E = &en::EN2;
        d1::En2 { tepe: E.tepe, pat: E.pat, dinlen: E.dinlen, tempoDol: E.tempoDol, yogK: E.yogK, yogMin: E.yogMin }
    }
    fn ALAN_EN(&self) -> d1::En {
        let E = &en::EN;
        d1::En { dalis: E.dalis, dalisDolYok: E.dalisDolYok, sprEsik: E.sprEsik, ivme: E.ivme, surme: E.surme, dolas: E.dolas, fake: E.fake, kalkan: E.kalkan, fizK: E.fizK }
    }
    fn ALAN_DENGE_KAYIP(&self, p: &Oyuncu, t: f64) -> f64 {
        ikili::dengeKayip(p, t)
    }
    fn ALAN_DOKUNUS(&self) -> Option<d1::Dokunus> {
        mac_dokunus()
    }
    fn ALAN_YOL_VER_TEPKI(&self) -> bool {
        false
    }
    fn ALAN_TASIYICI_SONRA_YAVAS(&self) -> bool {
        false
    }
    // ---- yerleşim ----
    fn tepki_s(&self) -> TepkiS {
        self.kural.get().tepki_s()
    }
    fn S(&self) -> Option<d1::ShapeS> {
        let S = &sekil::sabit::S;
        Some(d1::ShapeS { react: Some(S.react), rcvBuf: S.rcvBuf, marginT: S.marginT, markOff: Some(S.markOff), bekOnce: Some(S.bekOnce), bekSerbest: S.bekSerbest })
    }
    fn GR(&self) -> Option<d1::ShapeGR> {
        Some(d1::ShapeGR { yasH: sekil::sabit::GR.yasH })
    }
    fn GR_yasHata(&self, yas: f64) -> f64 {
        sekil::sabit::GR.yasHata(yas)
    }
    fn gecTab(&self, o: &Oyuncu, _refsiz: bool) -> Option<d1::GecTab> {
        Some(gecTab_k(&sekil::gec::gecTab(o, &self.kural.get())))
    }
    fn gecOf(&self, G: &d1::GecTab, x: f64, y: f64, s: Option<f64>) -> f64 {
        // sekil::gec::gecOf ile aynı; yalnız gereken yönün tablosu okunur (9 yön kopyalanmaz)
        let dx = x - G.x;
        let dy = y - G.y;
        let L = crate::mat::hypot(dx, dy);
        if L < 0.5 {
            return 0.0;
        }
        let c = if G.s0 < 0.001 { 1.0 } else { (G.vx * dx + G.vy * dy) / (G.s0 * L) };
        let kk = (crate::mat::jround(4.0 * crate::mat::jmax(-1.0, crate::mat::jmin(1.0, c))) + 4.0) as usize;
        match s {
            Some(s) => {
                let Y = &G.Y[kk];
                if let Some(a) = Y.m.as_ref().and_then(|v| v.get(..81)).and_then(|v| <&[f64; 81]>::try_from(v).ok()) {
                    return sekil::gec::gecK_m(Y.g, a, s, G.vm);
                }
                let mut m = [0.0; 81];
                if let Some(v) = &Y.m {
                    for (i, x) in v.iter().take(81).enumerate() {
                        m[i] = *x;
                    }
                }
                sekil::gec::gecK_m(Y.g, &m, s, G.vm)
            }
            None => G.t[kk],
        }
    }
    fn gec(&self, o: &Oyuncu, x: f64, y: f64, s: Option<f64>) -> Option<f64> {
        Some(sekil::gec::gec(o, x, y, s, &self.kural.get()))
    }
    fn gside(&self, a: Nk, t: f64, off: f64) -> Nk {
        let (x, y) = sekil::yer::gside(a.x, a.y, if t == 1.0 { 1 } else { 0 }, off);
        nk(x, y)
    }
    fn bekci(&self, p: &Oyuncu, b: Nk) -> Option<Nk> {
        let (x, y) = sekil::yer::bekci(p, b.x, b.y, None, &self.kural.get());
        Some(nk(x, y))
    }
    fn aliciDirenc(&self, pts: &[Nk], p: &Oyuncu, rc: f64, iMin: f64, rP: f64) -> Option<Nk> {
        let yol: Vec<sekil::ucus::Nokta> = pts.iter().map(|q| sekil::ucus::Nokta { x: q.x, y: q.y, e: 0.0 }).collect();
        sekil::ucus::aliciDirenc(&yol, p, rc, iMin, Some(rP), &self.kural.get()).map(|a| nk(a.x, a.y))
    }
    fn tehlike(&self, h: &Oyuncu) -> Vec<d1::Tehlike> {
        let m = &*self.mac;
        let hi = m.ps.iter().position(|q| q.id == h.id && q.team == h.team);
        let d = crate::mac::gecici_dis::kafa_d();
        sekil::takim::tehlike(m, &m.ps, hi, d)
            .unwrap_or_default()
            .into_iter()
            .map(|t| d1::Tehlike { an: t.an, w: t.w, w0: t.w0, x: t.x, y: t.y, k: t.k, L: t.L })
            .collect()
    }
    fn ilkPres(&self, tick: f64, qv: &mut Oyuncu, hx: &Oyuncu, team: f64, TH: &[d1::Tehlike], _dtac: &d1::TacEff, gs: &Option<JMShGs>) {
        use sekil::varlik::{Gs, Mc};
        // JS ilkPres(vm, …): vm yalnız tik taşır; yerleşimin pres hesabı maçtan yalnız tiki okur.
        // Boş maç ve yerleşim durumu her çağrıda kurulmaz (kesişme döngüsünde çok sık çağrılır).
        ILK_TAMPON.with(|t| {
            let mut t = t.borrow_mut();
            let IlkTampon { m, ek, ps } = &mut *t;
            m.tick = tick;
            ek.sh.gs = gs.as_ref().map(|g| Gs { t: g.t, dt: g.dt as u8, on: g.on, onP: g.onP, oy: g.oy });
            let (pplan0, sprint0, onK0) = (qv.pplan.clone(), qv.sprint, qv._onK.clone());
            ps.clear();
            ps.push(std::mem::take(qv));
            let th: Vec<sekil::takim::Tehlike> = TH.iter().map(|t| sekil::takim::Tehlike { an: t.an, w: t.w, w0: t.w0, x: t.x, y: t.y, k: t.k.clone(), L: t.L }).collect();
            let dtac = self.takim_tac(team);
            let d = crate::mac::gecici_dis::kafa_d();
            let kural = self.kural.get();
            let mut c = Mc { m, ps, ball: None, ek, k: &kural, d, lite: true, rnd: None };
            sekil::pres::ilkPres_h(&mut c, 0, hx, if team == 1.0 { 1 } else { 0 }, Some(&th), dtac);
            let mut sonuc = std::mem::take(&mut ps[0]);
            let (tx, ty, job) = (sonuc.tx, sonuc.ty, sonuc.job.take());
            ps.clear();
            *qv = sonuc;
            qv.tx = tx;
            qv.ty = ty;
            qv.job = job;
            // JS: qv nesnesi kesişme simülasyonu boyunca yaşar; ilkPres'in yazdığı pres planı (fake/iç) ve
            // sprint bir sonraki adımda görülür. (_onK bu görünümde yazılmaz.)
            let _ = (pplan0, sprint0);
            qv._onK = onK0;
        });
    }
    fn AS_ilkPres_var(&self) -> bool {
        true
    }
    // ---- maç ----
    fn IK(&self) -> Option<d1::Ik> {
        Some(d1::IK_JS)
    }
    fn ivme(&self, p: &mut Oyuncu, wx: f64, wy: f64, vmax: f64) {
        crate::mac::hareket::ivme(p, wx, wy, vmax)
    }
    fn kutle(&self, p: &Oyuncu, ayni: bool) -> f64 {
        ikili::kutle(p, ayni)
    }
    fn direnc(&self, p: &Oyuncu, ayni: bool) -> f64 {
        ikili::direnc(p, ayni)
    }
    fn omuzK(&self, q: &Oyuncu) -> f64 {
        ikili::omuzK(q)
    }
    fn durtFark(&self, q: &Oyuncu, h: &Oyuncu) -> Option<f64> {
        Some(ikili::durtFark(q, h))
    }
    fn sendS(&self, p: &Oyuncu) -> Option<f64> {
        Some(ikili::sendS(p))
    }
    fn ikEn(&self, p: &Oyuncu) -> Option<f64> {
        Some(ikili::ikEn(p))
    }
    fn BEKLE_BOZ(&self) -> Option<f64> {
        Some(crate::mac::dongu::BEKLE_BOZ)
    }
    // ---- maçın durumu ----
    fn mac_var(&self) -> bool {
        true
    }
    fn mac_ch(&self) -> Option<f64> {
        self.ch.get()
    }
    fn mac_holder(&self) -> Option<(f64, f64, f64, f64)> {
        self.holder.get()
    }
    fn tac_enHarcama(&self, team: f64) -> Option<f64> {
        self.enHarcama[if team == 1.0 { 1 } else { 0 }]
    }
    fn tac_siddet(&self, team: f64) -> Option<(Option<f64>, bool, f64)> {
        Some(self.siddet[if team == 1.0 { 1 } else { 0 }])
    }
    fn tacEff(&self, team: f64) -> d1::TacEff {
        // kafa yalnız taşır (ilkPres'e geri verir); takımı işaretle
        d1::TacEff(vec![("team".into(), team)])
    }
    fn sh_gs(&self) -> Option<JMShGs> {
        self.gs.borrow().clone()
    }
    fn prT(&self) -> Option<d1::PrT> {
        self.prT.borrow().clone()
    }
    // ---- algı kopyaları ----
    fn algi_e(&self, p: &Oyuncu) -> Option<f64> {
        p.__e
    }
    fn algi_yas(&self, p: &Oyuncu) -> Option<f64> {
        p.__yas
    }
    fn bekciFm(&self, p: &Oyuncu, B: Nk, fm: &d1::Fm) -> Nk {
        dunya_kopya(fm.ps, |ps| {
            let ben = fm.ps.iter().position(|q| q.id == p.id && q.team == p.team);
            let bm = sekil::yer::BekM { ps, holder: fm.holder, ben };
            let (x, y) = sekil::yer::bekci(p, B.x, B.y, Some(&bm), &self.kural.get());
            nk(x, y)
        })
    }
    fn slotBase(&self, fm: &d1::Fm, p: &Oyuncu, B: Nk, hucum: bool) -> Nk {
        dunya_kopya(fm.ps, |ps| {
            let y = sekil::yer::slotBase(&self.mac, ps, p, B.x, B.y, hucum);
            nk(y.x, y.y)
        })
    }
    fn S_prHat(&self, blok: &str) -> Option<f64> {
        sekil::sabit::prHat(blok)
    }
    fn tl(&self, p: &Oyuncu, ev: &str, k: &str) -> Option<f64> {
        crate::mac::kurulum::tl(p, ev, k)
    }
    fn sprK(&self, p: &Oyuncu) -> f64 {
        en::sprK(p, &self.kural.get())
    }
}

// ======================================================================
// Maçın kafası: tek `Kafa` nesnesi maçta yaşar (JS'te AlanDecide modülü); her tikte dış dünyası
// (MacDis) ve D._tick tazelenir. Yerleşim ve maç, çalışırken kafaya bu kapsamlı bağlamdan ulaşır.
// ======================================================================
use crate::kafa::Kafa;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Default)]
pub struct KafaKutu {
    pub k: Option<Rc<RefCell<Kafa>>>,
    /// bu tikin dış dünyası (kuralı tazelemek için)
    pub dis: Option<Arc<MacDis>>,
    /// dış dünyanın tazelendiği tik
    pub t: Option<f64>,
}
impl std::fmt::Debug for KafaKutu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KafaKutu").field("t", &self.t).finish_non_exhaustive()
    }
}

thread_local! {
    static KAFA: RefCell<Option<Rc<RefCell<Kafa>>>> = const { RefCell::new(None) };
}

/// Maçın kafasını bu tik için hazırla (yoksa kur) ve kapsamlı bağlama koy.
pub fn kafa_hazirla(m: &mut Mac) -> Rc<RefCell<Kafa>> {
    if m.mac.kafa.k.is_none() {
        let d = Arc::new(MacDis::kur(m));
        m.mac.kafa.dis = Some(d.clone());
        m.mac.kafa.k = Some(Rc::new(RefCell::new(Kafa::yeni(d))));
        m.mac.kafa.t = Some(m.tick);
    } else if m.mac.kafa.t != Some(m.tick) {
        // JS m._prT: kafanın yazdığı pres tartısını maç tutar, kafa1 (girEsik) dış dünyadan okur
        if let Some(k) = m.mac.kafa.k.as_ref() {
            if let Ok(k) = k.try_borrow() {
                m.mac.prT_k = k.prT.as_ref().map(|p| p.d1());
            }
        }
        let d = Arc::new(MacDis::kur(m));
        m.mac.kafa.dis = Some(d.clone());
        let k = m.mac.kafa.k.as_ref().unwrap();
        if let Ok(mut k) = k.try_borrow_mut() {
            k.dis = d;
        }
        m.mac.kafa.t = Some(m.tick);
    }
    let k = m.mac.kafa.k.clone().unwrap();
    if let Ok(mut kk) = k.try_borrow_mut() {
        kk.D._tick = Some(m.tick);
        // JS D._mac = m (kafaAc): kafa maçı her an görür (ileri tablosu ilk okunduğunda kurulur)
        if kk.mac.as_ref().map_or(true, |g| g.tick != m.tick) {
            kk.mac = Some(crate::mac::gecici_dis::mac_gor(m));
        }
    }
    if let Ok(kk) = k.try_borrow() {
        let v = (kk.dis.clone(), kk.D._tick);
        DIS.with(|d| *d.borrow_mut() = Some(v));
    }
    KAFA.with(|c| *c.borrow_mut() = Some(k.clone()));
    k
}

thread_local! {
    /// bu tikin dış dünyası ve tiki (yedek kafalar için)
    static DIS: RefCell<Option<(Arc<dyn crate::kafa::gecici_dis1::Dis>, Option<f64>)>> = const { RefCell::new(None) };
    /// iç içe çağrılar için yedek kafalar (kafa → dış dünya → yerleşim → kafa)
    static YAN: RefCell<Vec<Rc<RefCell<Kafa>>>> = const { RefCell::new(Vec::new()) };
}

/// Kapsamdaki kafayla çalış (yerleşimin ve maçın kafa çağrıları). Kafa o an başka bir çağrının
/// içindeyse (iç içe) aynı dış dünyalı yedek bir kafa kullanılır.
/// ilkPres köprüsünün yeniden kullanılan tamponu (maç yalnız tik taşır)
#[derive(Default)]
struct IlkTampon {
    m: Mac,
    ek: sekil::varlik::SekilDurum,
    ps: Vec<Oyuncu>,
}
thread_local! {
    static ILK_TAMPON: std::cell::RefCell<IlkTampon> = std::cell::RefCell::new(IlkTampon::default());
}
thread_local! {
    /// Kafa dünyasının (Arc'lı oyuncular, değişmez) düz kopyası: aynı dünya için yeniden kopyalanmaz.
    /// Anahtar oyuncu nesnelerinin kendisi (Arc'lar tutulduğu için başka dünyayla karışmaz).
    static DUNYA_KOPYA: std::cell::RefCell<(Vec<crate::kafa::kafa1_js::Op>, Vec<Oyuncu>)> = const { std::cell::RefCell::new((Vec::new(), Vec::new())) };
}
/// `src`'nin düz kopyasıyla `f`'i çalıştır (dünya aynıysa önceki kopya).
fn dunya_kopya<R>(src: &[crate::kafa::kafa1_js::Op], f: impl FnOnce(&[Oyuncu]) -> R) -> R {
    DUNYA_KOPYA.with(|c| {
        let ayni = {
            let c = c.borrow();
            c.0.len() == src.len() && c.0.iter().zip(src).all(|(a, b)| Arc::ptr_eq(a, b))
        };
        if !ayni {
            match c.try_borrow_mut() {
                Ok(mut c) => {
                    c.0 = src.to_vec();
                    c.1 = src.iter().map(|q| (**q).clone()).collect();
                }
                // iç içe çağrıda (kopya ödünçteyken) başka dünya: geçici kopya
                Err(_) => {
                    let ps: Vec<Oyuncu> = src.iter().map(|q| (**q).clone()).collect();
                    return f(&ps);
                }
            }
        }
        let c = c.borrow();
        f(&c.1)
    })
}
/// Maçın dokunuş ayarı (JS ALAN_DOKUNUS): bugün kapalı (maç taşımadı)
pub fn mac_dokunus() -> Option<d1::Dokunus> {
    None
}
/// Yedek kafaya düşülen çağrı sayısı (sınama)
pub static YAN_SAY: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
pub fn kafa_ile<R>(f: impl FnOnce(&mut Kafa) -> R) -> R {
    let k = KAFA.with(|c| c.borrow().clone()).expect("kafa bağlamı kurulmadı (kafa_hazirla)");
    if let Ok(mut b) = k.try_borrow_mut() {
        let t = b.D._tick;
        return crate::sekil::d_tick_ile(t, || f(&mut b));
    }
    YAN_SAY.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    if crate::iz_acik() && std::env::var("ALAN_YAN_IZ").is_ok() { eprintln!("YAN {}", std::backtrace::Backtrace::force_capture().to_string().lines().filter(|l| l.contains("alan_motor::")).take(8).map(|l| l.trim().replace("alan_motor::", "")).collect::<Vec<_>>().join(" < ")); }
    let (dis, tick) = DIS.with(|d| d.borrow().clone()).expect("kafa dış dünyası kurulmadı");
    let yan = YAN.with(|y| {
        let mut y = y.borrow_mut();
        if let Some(q) = y.iter().find(|q| q.try_borrow_mut().is_ok()) {
            return q.clone();
        }
        let q = Rc::new(RefCell::new(Kafa::yeni(dis.clone())));
        y.push(q.clone());
        q
    });
    let mut b = yan.borrow_mut();
    if !Arc::ptr_eq(&b.dis, &dis) {
        b.dis = dis;
    }
    b.D._tick = tick;
    crate::sekil::d_tick_ile(tick, || f(&mut b))
}

/// Dünyayı kafanın biçimine çevir.
pub fn op_dunya(ps: &[Oyuncu]) -> Vec<crate::kafa::kafa1_js::Op> {
    ps.iter().map(|p| Arc::new(p.clone())).collect()
}

/// JS `m._prT`: kafa kararda yazar (D._mac === m). Rust'ta kafa kendinde tutar; karardan hemen sonra
/// maça (yerleşim okur) ve kafa1'in dış dünyasına (girEsik okur) aktarılır.
pub fn prT_esitle(m: &mut Mac) {
    let p = KAFA.with(|c| c.borrow().clone()).and_then(|k| k.try_borrow().ok().and_then(|k| k.prT.clone()));
    if let Some(p) = p {
        let (hm, _) = p.hm.clone().unwrap_or_default();
        m._prT = Some(crate::model::JM_prT { team: p.team, hId: p.hId, t: p.t.unwrap_or(f64::NAN), m: p.m.clone(), mm: p.mm.clone(), hm, h: p.h });
        m.mac.prT_k = Some(p.d1());
    }
}

/// Maçsız kullanım (sınamalar): dış dünyası verili bir kafayı kapsamlı bağlama koy.
pub fn kafa_bagla(dis: Arc<dyn crate::kafa::gecici_dis1::Dis>, tick: Option<f64>) {
    let mut k = Kafa::yeni(dis.clone());
    k.D._tick = tick;
    KAFA.with(|c| *c.borrow_mut() = Some(Rc::new(RefCell::new(k))));
    DIS.with(|d| *d.borrow_mut() = Some((dis, tick)));
}

/// JS'in canlı okuduğu sprint bağlamı (self.__mH, taşıyıcı, alıcı, tik): kafa çağrılmadan hemen önce.
pub fn kural_tazele(m: &Mac) {
    if let Some(d) = &m.mac.kafa.dis {
        d.kural.set(MacKural::of(m));
        d.holder.set(m.holder.map(|h| { let p = &m.ps[h]; (p.id, p.team, p.x, p.y) }));
        d.ch.set(Some(m.ch));
        // JS m._prT ve m.sh.gs canlı okunur: aynı tikte önceki kararın yazdığı görülür
        *d.prT.borrow_mut() = m.mac.prT_k.clone();
        *d.gs.borrow_mut() = m.mac.sekil.sh.gs.as_ref().map(|g| JMShGs { t: g.t, dt: g.dt as f64, on: g.on, onP: g.onP, oy: g.oy, ..Default::default() });
    }
}
