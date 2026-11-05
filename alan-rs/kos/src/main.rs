//! alan-kos: kayıttan maç oynat. Kullanım: alan-kos <kayit.json> <tik> [iz|ozet]
//! iz: her tik bir satır (sina/istatistik.js okur); ozet: süre ve skor;
//! kare <cikti.json>: izleme ekranı için bütün kareler (oyuncu yerleri, top, işler, olaylar).
//! parmak: her tik bir satır, maçın gözlemlenebilir durumunun bit bit özeti (motor::parmak; sina/ayni.sh).
//! Kayıt yerine anlar/ dosyası da verilebilir (içindeki `kurulum` kullanılır).
use alan_motor::mac::motor::Motor;
use alan_motor::oyun::MacMotoru;
use std::io::Write;

fn main() {
    let a: Vec<String> = std::env::args().collect();
    if a.len() < 3 {
        eprintln!("kullanım: alan-kos <kayit.json> <tik> [iz|ozet]");
        std::process::exit(2);
    }
    let metin = std::fs::read_to_string(&a[1]).expect("kayıt okunamadı");
    let v: serde_json::Value = serde_json::from_str(&metin).expect("kayıt JSON değil");
    // anlar/ dosyası: kurulum içinde
    let v = match v.get("kurulum") { Some(k) if v.get("yeni").is_none() => k.clone(), _ => v };
    let tik: u32 = a[2].split(',').next().and_then(|x| x.parse().ok()).expect("tik");
    let kip = a.get(3).map(|s| s.as_str()).unwrap_or("ozet");
    // paralel kafa (ALAN_ISCI=n): n-1 yardımcı iş parçacığı aynı maçı oynar, dalgaları bölüşür
    let n_isci: usize = std::env::var("ALAN_ISCI").ok().and_then(|s| s.parse().ok()).unwrap_or(1);
    let mut yardimci = Vec::new();
    let mut ana_isci: Option<std::rc::Rc<alan_motor::paralel::yerel::Isci>> = None;
    if n_isci > 1 {
        let pano = std::sync::Arc::new(alan_motor::paralel::yerel::Pano::default());
        for _ in 1..n_isci {
            let v2 = v.clone();
            let p2 = pano.clone();
            yardimci.push(std::thread::Builder::new().stack_size(256 << 20).spawn(move || {
                let mut mt = Motor::kayittan(&v2).expect("maç kurulamadı");
                alan_motor::paralel::kur(Some(std::rc::Rc::new(alan_motor::paralel::yerel::Isci::yeni(p2, n_isci))));
                while mt.tick() < tik && !mt.bitti() {
                    mt.adim();
                }
            }).unwrap());
        }
        let i = std::rc::Rc::new(alan_motor::paralel::yerel::Isci::yeni(pano, n_isci));
        alan_motor::paralel::kur(Some(i.clone()));
        ana_isci = Some(i);
    }
    let mut mt = Motor::kayittan(&v).expect("maç kurulamadı");
    let mut isler = alan_motor::oyun::IsSozluk::default();
    let mut kareler: Vec<String> = Vec::new();
    let mut olaylar: Vec<String> = Vec::new();
    let r2 = |v: f32| if v.is_finite() { format!("{}", (v * 100.0).round() / 100.0) } else { "null".into() };
    let t0 = std::time::Instant::now();
    let out = std::io::stdout();
    let mut out = std::io::BufWriter::new(out.lock());
    let mut olay_say = mt.m.ev_n;
    let mut son_karar: Option<f64> = None;
    if kip == "durum" {
        writeln!(out, "{}", durum(&mt.m)).unwrap();
    }
    // an: <kayıt> <t1,t2,…> an <önek> [süre]: maç bugünkü kafayla oynar; her ti'de kopya maç iki kez oynatılır
    // (bugünkü kafa / tutmayı gerçeğe yakın kestiren kafa), kareler <önek>-<ti>-{eski,yeni}.json (kare kipinin biçimi)
    if kip == "an" {
        let tikler: Vec<u32> = a[2].split(',').filter_map(|x| x.parse().ok()).collect();
        let onek = a.get(4).expect("an <önek>").clone();
        let sure: u32 = a.get(5).and_then(|x| x.parse().ok()).unwrap_or(120);
        let son = *tikler.iter().max().unwrap_or(&0);
        let yaz = |m: &alan_motor::model::Mac, kareler: &Vec<String>, olaylar: &Vec<String>, isler: &alan_motor::oyun::IsSozluk, yol: &str| {
            let ad: Vec<String> = m.ps.iter().map(|p| serde_json::to_string(&*p.name).unwrap()).collect();
            let tk: Vec<String> = m.ps.iter().map(|p| format!("{}", p.team)).collect();
            let rl: Vec<String> = m.ps.iter().map(|p| serde_json::to_string(&*p.role).unwrap()).collect();
            let is: Vec<String> = isler.yazilar.iter().map(|j| serde_json::to_string(j).unwrap()).collect();
            let j = format!("{{\"motor\":\"Rust motoru\",\"ad\":[{}],\"takim\":[{}],\"rol\":[{}],\"isler\":[{}],\"olay\":[{}],\"k\":[{}]}}", ad.join(","), tk.join(","), rl.join(","), is.join(","), olaylar.join(","), kareler.join(","));
            std::fs::write(yol, j).expect("yazılamadı");
        };
        while mt.tick() < son && !mt.bitti() {
            let t = mt.tick() + 1;
            if tikler.contains(&t) {
                let mut once = mt.m.clone();
                once.mac.kafa = Default::default();
                for (kip2, acik) in [("eski", false), ("yeni", true)] {
                    alan_motor::kafa::kafa1_ikili::KP_KAL_ZORLA.store(acik, std::sync::atomic::Ordering::Relaxed);
                    let mut m2 = Motor::yeni(once.clone());
                    let mut isler2 = alan_motor::oyun::IsSozluk::default();
                    let (mut kr, mut ol) = (Vec::new(), Vec::new());
                    let mut osay = m2.m.ev_n;
                    for _ in 0..sure {
                        if m2.bitti() { break; }
                        m2.adim();
                        let k = m2.kare(&mut isler2);
                        let mut s = format!("[{},{},{},{},{},{}", k.tick, k.sahip, r2(k.top_x), r2(k.top_y), k.skor[0], k.skor[1]);
                        for o in &k.oyuncular {
                            s.push_str(&format!(",{},{},{},{}", r2(o.x), r2(o.y), o.is, r2(o.en)));
                        }
                        s.push(']');
                        kr.push(s);
                        let m = &m2.m;
                        let yeni = ((m.ev_n - osay) as usize).min(m.events.len());
                        for e in m.events.iter().take(yeni).rev() {
                            ol.push(format!("[{},{},{}]", m.tick, e.team, serde_json::to_string(&e.text).unwrap()));
                        }
                        osay = m.ev_n;
                    }
                    yaz(&m2.m, &kr, &ol, &isler2, &format!("{}-{}-{}.json", onek, t, kip2));
                }
                alan_motor::kafa::kafa1_ikili::KP_KAL_ZORLA.store(false, std::sync::atomic::Ordering::Relaxed);
            }
            mt.adim();
        }
        return;
    }
    while mt.tick() < tik && !mt.bitti() {
        // ölçüm (tutma): kaleye 35 br'den yakın her yeni taşıyıcı kararında, kararın sürme seçenekleri kopya maçta
        // zorlanıp oynatılır; kafanın tutma kestirimi gerçekleşenle bir satırda (tik, ad, mesafe, sıra, n, P, v,
        // kafa seçer miydi, hedef, sonuç, süre). Sonuç: "kayip" (top rakipte), "birakti" (pas/şut), "tuttu" (süre bitti).
        let once = if kip == "tutma" { let mut c = mt.m.clone(); c.mac.kafa = Default::default(); Some(c) } else { None };
        let t_once = mt.m._sec.as_ref().map(|s| s.t);
        mt.adim();
        if once.is_some() && std::env::var("ALAN_TUTMA_PK").is_ok() { eprintln!("PK {}", alan_motor::parmak::parmak_izi(&mt.m)); }
        if let Some(once) = once {
            let m = &mt.m;
            let t_sec = m._sec.as_ref().map(|s| s.t);
            if t_sec.is_some() && t_sec != t_once {
                let h = m._sec.as_ref().unwrap().h;
                let hp = &m.ps[h];
                let gx = if hp.team == 0.0 { 120.0 } else { 0.0 };
                let dk = ((gx - hp.x) * (gx - hp.x) + (30.0 - hp.y) * (30.0 - hp.y)).sqrt();
                if dk < 35.0 && !hp.bekci() {
                    for sira in 0..4 {
                        let mut c = once.clone();
                        c.mac.zorla = Some(sira);
                        alan_motor::mac::step(&mut c);
                        let z = match c.mac.zorla_sonuc.take() { Some(z) => z, None => break };
                        let (hx, hy) = (c.ps[h].x, c.ps[h].y);
                        let T = (((z.to.0 - hx).powi(2) + (z.to.1 - hy).powi(2)).sqrt() / 0.25).round() + 6.0;
                        c.decT = T;
                        let t0 = c.tick;
                        let mut sonuc = "tuttu";
                        let mut j = 0.0;
                        let mut den_min: f64 = c.ps[h].den.unwrap_or(1.0);
                        // ALAN_TUTMA_KILIT: taşıyıcı sürme boyunca yeniden karar vermez (kafanın keepP tanımına en yakın ölçü)
                        let kilit = std::env::var("ALAN_TUTMA_KILIT").is_ok();
                        while j < T && !c.over {
                            if kilit && c.holder == Some(h) {
                                c.decT = T - j + 1.0;
                            }
                            alan_motor::mac::step(&mut c);
                            j += 1.0;
                            den_min = den_min.min(c.ps[h].den.unwrap_or(1.0));
                            match c.holder {
                                Some(q) if c.ps[q].team != hp.team => { sonuc = "kayip"; break; }
                                Some(q) if q != h => { sonuc = "birakti"; break; }
                                None if c.fl.as_ref().map_or(false, |f| f.t0 >= t0 && !f.kind.is_empty()) => { sonuc = "birakti"; break; }
                                _ => {}
                            }
                        }
                        writeln!(out, "[{},{},{:.2},{},{},{},{},{},[{},{}],{:?},{},{:.3}]", m.tick, serde_json::to_string(&*hp.name).unwrap(), dk, sira, z.n, if z.P.is_finite() { format!("{}", z.P) } else { "null".into() }, z.v, z.secilen, z.to.0, z.to.1, sonuc, j, den_min).unwrap();
                    }
                }
            }
        }
        let s = mt.iz();
        if kip == "iz" {
            writeln!(out, "{}", s.json()).unwrap();
        }
        if kip == "parmak" {
            writeln!(out, "{} {:016x}", mt.m.tick, alan_motor::parmak::parmak_izi(&mt.m)).unwrap();
        }
        if kip == "kararlar" {
            // her yeni taşıyıcı kararı bir satır: tik, ad, takım, yer, seçilen başta ilk 4 seçenek (tür, alıcı, değer, hedef)
            let m = &mt.m;
            let t = m._sec.as_ref().map(|s| s.t);
            if t.is_some() && t != son_karar {
                son_karar = t;
                if let Some(d) = &m.lastDec {
                    let o: Vec<String> = d.opts.iter().map(|o| format!("[{},{},{},{}]", serde_json::to_string(&o.kind).unwrap(), serde_json::to_string(&o.q).unwrap(), if o.v.is_finite() { format!("{}", o.v) } else { "null".into() }, o.to.as_ref().map_or("null".to_string(), |t| format!("[{},{}]", t.x, t.y)))).collect();
                    // bütün seçenekler (m._sec.o): tür, hedef, değer
                    let hep: Vec<String> = m._sec.as_ref().map_or(vec![], |s| s.o.iter().map(|e| format!("[{},{},{},{}]", serde_json::to_string(&e.k).unwrap(), e.x, e.y, if e.v.is_finite() { format!("{}", e.v) } else { "null".into() })).collect());
                    writeln!(out, "[{},{},{},{},{},[{}],[{}]]", m.tick, serde_json::to_string(&d.name).unwrap(), d.team, d.x, d.y, o.join(","), hep.join(",")).unwrap();
                }
            }
        }
        if kip == "durum" {
            let m = &mt.m;
            writeln!(out, "{}", durum(m)).unwrap();
        }
        if kip == "kare" {
            let k = mt.kare(&mut isler);
            let mut t = format!("[{},{},{},{},{},{}", k.tick, k.sahip, r2(k.top_x), r2(k.top_y), k.skor[0], k.skor[1]);
            for o in &k.oyuncular {
                t.push_str(&format!(",{},{},{},{}", r2(o.x), r2(o.y), o.is, r2(o.en)));
            }
            t.push(']');
            kareler.push(t);
            let m = &mt.m;
            let yeni = ((m.ev_n - olay_say) as usize).min(m.events.len());
            for e in m.events.iter().take(yeni).rev() {
                olaylar.push(format!("[{},{},{}]", m.tick, e.team, serde_json::to_string(&e.text).unwrap()));
            }
            olay_say = m.ev_n;
        }
    }
    let dt = t0.elapsed().as_secs_f64();
    if kip == "boyut" {
        for p in mt.m.ps.iter().take(3) {
            eprintln!("{} hist {:?} past {:?} _bk {:?} job {:?} _lc {} sizeof {}", p.name, p.hist.as_ref().map(|v| v.len()), p.past.as_ref().map(|v| v.len()), p._bk.as_ref().map(|v| v.len()), p.job.as_ref().map(|j| j.len()), p._lc.is_some(), std::mem::size_of::<alan_motor::model::Oyuncu>());
        }
        let t = std::time::Instant::now();
        let mut n = 0usize;
        for _ in 0..100000 { let c = mt.m.ps[3].clone(); n += c.name.len(); }
        eprintln!("klon 100k: {:?} ({})", t.elapsed(), n);
        let mut q = mt.m.ps[3].clone(); q.hist = None; q.past = None; q._bk = None;
        let t = std::time::Instant::now();
        for _ in 0..100000 { let c = q.clone(); n += c.name.len(); }
        eprintln!("klon hist/past/_bk yok 100k: {:?} ({})", t.elapsed(), n);
    }
    if kip == "karar" {
        if let Some(d) = &mt.m.lastDec {
            println!("karar {} ({},{})", d.name, d.x, d.y);
            for o in &d.opts {
                println!("  {:<40} q={:<8} v={:.6} P={:.4} to={:?}", o.kind, o.q, o.v, o.P, o.to.as_ref().map(|t| (t.x, t.y)));
            }
        }
    }
    if kip == "oyuncu" {
        let k: usize = a[4].parse().unwrap();
        let mut p = mt.m.ps[k].clone();
        p.yet = Default::default();
        println!("{:#?}", p);
    }
    if kip == "kare" {
        let m = &mt.m;
        let yol = a.get(4).expect("kare <cikti.json>");
        let ad: Vec<String> = m.ps.iter().map(|p| serde_json::to_string(&*p.name).unwrap()).collect();
        let tk: Vec<String> = m.ps.iter().map(|p| format!("{}", p.team)).collect();
        let rl: Vec<String> = m.ps.iter().map(|p| serde_json::to_string(&*p.role).unwrap()).collect();
        let is: Vec<String> = isler.yazilar.iter().map(|j| serde_json::to_string(j).unwrap()).collect();
        let j = format!("{{\"motor\":\"Rust motoru\",\"ad\":[{}],\"takim\":[{}],\"rol\":[{}],\"isler\":[{}],\"olay\":[{}],\"k\":[{}]}}", ad.join(","), tk.join(","), rl.join(","), is.join(","), olaylar.join(","), kareler.join(","));
        std::fs::write(yol, j).expect("yazılamadı");
    }
    let m = &mt.m;
    eprintln!("tik {} · skor {}-{} · {:.1} sn ({:.2} ms/tik)", m.tick, m.score[0], m.score[1], dt, 1e3 * dt / (m.tick.max(1.0)));
    if alan_motor::kafa::kafa2_zemin::onden_acik() {
        let (o, k, t) = alan_motor::kafa::kafa2_zemin::onden_say();
        let s = alan_motor::kafa::kafa2_zemin::onden_sn();
        eprintln!("önceden hesaplama: {o} hücre, {k} tanesi sonra istendi (oran %{:.0}) · kararın istediği {t} hücreden önceden hazır olan: %{:.0} · toplu hesap {s:.2} sn (toplam sürenin %{:.0})", 100.0 * k as f64 / o.max(1) as f64, 100.0 * k as f64 / t.max(1) as f64, 100.0 * s / dt);
    }
    alan_motor::kafa::kafa2_karar::dalga_olc_yaz();
    if std::env::var("ALAN_BEK_IZ").is_ok() {
        let (h, k) = alan_motor::sekil::yer::BEK_DER_SAY.with(|c| c.get());
        eprintln!("bek_derinlik: {} isabet, {} hesap", h, k);
    }
    if let Some(i) = ana_isci {
        let o = i.olc.borrow();
        eprintln!("paralel {} işçi: ana işçi {} iş hesapladı (ort {} bayt), {} paket açtı ({:.2} sn), {:.2} sn bekledi; dalgalarda {:.2} sn", n_isci, o.0, o.4 / o.0.max(1), o.1, o.3, o.2, i.olc_d.get());
    }
    for y in yardimci {
        y.join().unwrap();
    }
}


/// Bir tikin tam durumu (sina karşılaştırması için; durum-js.js ile aynı biçim)
fn durum(m: &alan_motor::model::Mac) -> String {
    use serde_json::{json, Value};
    let o = |v: Option<f64>| v.map_or(Value::Null, |x| json!(x));
    let f = |x: f64| if x.is_finite() { json!(x) } else { Value::Null };
    let ps: Vec<Value> = m
        .ps
        .iter()
        .map(|p| json!([f(p.x), f(p.y), f(p.vx), f(p.vy), o(p.en), o(p.den), f(p.tx), f(p.ty), p.job.clone().unwrap_or_default(), p._sf.as_ref().map_or(String::new(), |s| s.f.clone()), o(p.ca)]))
        .collect();
    let b = m.ball.as_ref().map_or(Value::Null, |b| json!([b.x, b.y, b.vx, b.vy]));
    json!({"t": m.tick, "h": m.holder.map_or(-1, |h| h as i64), "ch": m.ch, "r": m.r.s, "b": b, "p": ps}).to_string()
}
