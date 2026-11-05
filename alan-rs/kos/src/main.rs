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
    // dene: <kayıt> <tik> dene [N] [süre]: tik'teki taşıyıcı kararının her seçeneği zorla seçilir, kopya maç N kez
    // (karardan sonra zar dizisi değiştirilerek) süre tik doğal oynar; seçenek başına kafanın değeri/tutma kestirimi
    // ve gerçekleşen: gol atma, gol yeme, top kaybı (süre içinde rakibe geçti mi), sonda top kimde.
    // ALAN_DENE_KAFA: "kal" | "mek" | "mekg" (mek + sendeleme gecikmesi) (kopya maçların kafası; yoksa bugünkü kafa)
    if kip == "dene" {
        let n: u32 = a.get(4).and_then(|x| x.parse().ok()).unwrap_or(40);
        let sure: u32 = a.get(5).and_then(|x| x.parse().ok()).unwrap_or(90);
        let kafa = std::env::var("ALAN_DENE_KAFA").unwrap_or_default();
        while mt.tick() + 1 < tik && !mt.bitti() {
            mt.adim();
        }
        // ALAN_DENE_ILK: tik karar tiki değilse ilk taşıyıcı kararına dek ilerle (en çok 90 tik)
        if std::env::var("ALAN_DENE_ILK").is_ok() {
            for _ in 0..90 {
                let mut c = mt.m.clone();
                c.mac.kafa = Default::default();
                c.mac.zorla_i = Some(0);
                alan_motor::mac::step(&mut c);
                if c.mac.zorla_sonuc.is_some() { break; }
                mt.adim();
            }
            let h = { let mut c = mt.m.clone(); c.mac.kafa = Default::default(); alan_motor::mac::step(&mut c); c._sec.as_ref().map(|s| s.h) };
            eprintln!("KARAR_TIK {} {}", mt.tick() + 1, h.map_or("-".to_string(), |h| mt.m.ps[h].govde.name.to_string()));
        }
        // maç karara kadar bugünkü kafayla oynar; kopyalar seçilen kafayla
        alan_motor::kafa::kafa1_ikili::KP_KAL_ZORLA.store(kafa == "kal", std::sync::atomic::Ordering::Relaxed);
        alan_motor::kafa::kafa1_ikili::MEKANIZMA_ZORLA.store(kafa.starts_with("mek"), std::sync::atomic::Ordering::Relaxed);
        alan_motor::kafa::kafa1_ikili::SEND_GEC_ZORLA.store(kafa.starts_with("mekg"), std::sync::atomic::Ordering::Relaxed);
        alan_motor::kafa::kafa2_tarti::GORUS_ZORLA.store(kafa == "mekgu", std::sync::atomic::Ordering::Relaxed);
        // ALAN_EK=donus[,tek]: kopyaların kafasına ek deneyler
        let ek = std::env::var("ALAN_EK").unwrap_or_default();
        alan_motor::kural::gec::GEC_DONUS_ZORLA.store(ek.split(',').any(|x| x == "donus"), std::sync::atomic::Ordering::Relaxed);
        alan_motor::sekil::gor::ARKADAS_NIYET_ZORLA.store(ek.split(',').any(|x| x == "niyet"), std::sync::atomic::Ordering::Relaxed);
        alan_motor::kafa::kafa1_ucus::ALIS_KUME_ZORLA.store(ek.split(',').any(|x| x == "kume"), std::sync::atomic::Ordering::Relaxed);
        alan_motor::kafa::kafa2_tarti::TEK_SUT_ZORLA.store(ek.split(',').any(|x| x == "tek"), std::sync::atomic::Ordering::Relaxed);
        alan_motor::kafa::kafa1_ucus::BEK_DALIS_ZORLA.store(ek.split(',').any(|x| x == "dalis"), std::sync::atomic::Ordering::Relaxed);
        alan_motor::kafa::kafa2_imkan::SUT_GERCEK_ZORLA.store(ek.split(',').any(|x| x == "sutgercek"), std::sync::atomic::Ordering::Relaxed);
        let mut once = mt.m.clone();
        once.mac.kafa = Default::default();
        // seçenekler: önce hepsi bir kez zorlanır; ALAN_DENE_SEC=0,3,7 yalnız bunlar denenir, ALAN_DENE_LISTE seçenek sayısını yazar
        let mut sec: Option<Vec<usize>> = std::env::var("ALAN_DENE_SEC").ok().map(|x| x.split(',').filter_map(|y| y.parse().ok()).collect());
        // kopyalar teker teker kurulur (hepsini birden tutmak belleği doldurur)
        let kur = |i: usize| -> Option<(alan_motor::model::Mac, alan_motor::mac::ZorlaSonuc)> {
            let mut c = once.clone();
            c.mac.zorla_i = Some(i);
            alan_motor::mac::step(&mut c);
            let z = c.mac.zorla_sonuc.take()?;
            Some((c, z))
        };
        // ALAN_DENE_UST=K: kafanın değerine göre ilk K seçenek (seçilen her zaman içinde)
        if let Some(K) = std::env::var("ALAN_DENE_UST").ok().and_then(|x| x.parse::<usize>().ok()) {
            let mut v: Vec<(usize, f64, bool)> = Vec::new();
            for i in 0..64usize {
                let Some((_, z)) = kur(i) else { break };
                v.push((i, if z.v.is_finite() { z.v } else { -1e9 }, z.secilen));
            }
            v.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
            let mut ust: Vec<usize> = v.iter().take(K).map(|x| x.0).collect();
            if let Some(x) = v.iter().find(|x| x.2) { if !ust.contains(&x.0) { ust.push(x.0); } }
            sec = Some(ust);
        }
        if std::env::var("ALAN_DENE_LISTE").is_ok() {
            let c = { let mut c = once.clone(); c.mac.zorla_i = Some(0); alan_motor::mac::step(&mut c); c.mac.zorla_sonuc.take().map_or(0, |z| z.n) };
            println!("{c}");
            return;
        }
        {
            {
                for i in 0..64usize {
                    if !sec.as_ref().map_or(true, |s| s.contains(&i)) { continue; }
                    let Some((c, z)) = kur(i) else { break };
                    let (c, z) = (&c, &z);
                    let h = c._sec.as_ref().map_or(0, |s| s.h);
                    let tk = c.ps[h].team;
                    let (mut gol, mut yedi, mut kayip, mut son_biz, mut son_rak) = (0u32, 0u32, 0u32, 0u32, 0u32);
                    let mut kayip_t = 0.0;
                    let hh = &h;
                    let erken: u32 = std::env::var("ALAN_DENE_ERKEN").ok().and_then(|x| x.parse().ok()).unwrap_or(15);
                    let (mut erk, mut send) = (0u32, 0u32);
                    // ALAN_DENE_YAZ=<önek>: her seçeneğin ilk denemesinin kareleri <önek>-<i>.json (kare kipinin biçimi)
                    if let Ok(onek) = std::env::var("ALAN_DENE_YAZ") {
                        let mut m2 = Motor::yeni(c.clone());
                        let mut isler2 = alan_motor::oyun::IsSozluk::default();
                        let (mut kr, mut ol) = (Vec::<String>::new(), Vec::<String>::new());
                        let mut osay = m2.m.ev_n;
                        for _ in 0..sure {
                            if m2.bitti() { break; }
                            m2.adim();
                            let kk = m2.kare(&mut isler2);
                            let mut t = format!("[{},{},{},{},{},{}", kk.tick, kk.sahip, r2(kk.top_x), r2(kk.top_y), kk.skor[0], kk.skor[1]);
                            for o in &kk.oyuncular { t.push_str(&format!(",{},{},{},{}", r2(o.x), r2(o.y), o.is, r2(o.en))); }
                            t.push(']');
                            kr.push(t);
                            let m = &m2.m;
                            let yeni = ((m.ev_n - osay) as usize).min(m.events.len());
                            for e in m.events.iter().take(yeni).rev() { ol.push(format!("[{},{},{}]", m.tick, e.team, serde_json::to_string(&e.text).unwrap())); }
                            osay = m.ev_n;
                        }
                        let m = &m2.m;
                        let adl: Vec<String> = m.ps.iter().map(|p| serde_json::to_string(&*p.govde.name).unwrap()).collect();
                        let tkl: Vec<String> = m.ps.iter().map(|p| format!("{}", p.team)).collect();
                        let rl: Vec<String> = m.ps.iter().map(|p| serde_json::to_string(&*p.govde.role).unwrap()).collect();
                        let is: Vec<String> = isler2.yazilar.iter().map(|j| serde_json::to_string(j).unwrap()).collect();
                        let j = format!("{{\"motor\":\"Rust motoru\",\"ad\":[{}],\"takim\":[{}],\"rol\":[{}],\"isler\":[{}],\"olay\":[{}],\"k\":[{}]}}", adl.join(","), tkl.join(","), rl.join(","), is.join(","), ol.join(","), kr.join(","));
                        std::fs::write(format!("{}-{}.json", onek, i), j).expect("yazılamadı");
                    }
                    for d in 0..n {
                        let mut k = c.clone();
                        if d > 0 { k.r.s ^= (d as i32).wrapping_mul(-1640531527); }
                        let s0 = k.score.clone();
                        let mut kaybetti = false;
                        let mut sendeledi = false;
                        let t0 = k.tick;
                        for j in 0..sure {
                            if k.over { break; }
                            alan_motor::mac::step(&mut k);
                            // gol olunca santra topu rakibe verir: kayıp sayılmaz
                            if k.score != s0 { break; }
                            if !kaybetti && k.holder.map_or(false, |q| k.ps[q].team != tk) { kaybetti = true; kayip += 1; kayip_t += j as f64; if j < erken { erk += 1; } }
                            if !sendeledi && j < erken && k.ps[*hh].hal.sendT.map_or(false, |s| s > t0 as f64 + 0.5) { sendeledi = true; send += 1; }
                            if k.score != s0 { break; }
                        }
                        let ti = tk as usize;
                        if k.score[ti] > s0[ti] { gol += 1; }
                        if k.score[1 - ti] > s0[1 - ti] { yedi += 1; }
                        match k.holder { Some(q) if k.ps[q].team == tk => son_biz += 1, Some(_) => son_rak += 1, None => {} }
                    }
                    let f = |x: u32| x as f64 / n as f64;
                    let l = format!("{:2} {:<28} ({:5.1},{:5.1}) v {:6.3} P {:5} {} | gol {:.2} yedi {:.2} kayıp {:.2} (ort {:4.0} tik) son biz {:.2} rakip {:.2}",
                        i, z.tur, z.to.0, z.to.1, z.v, if z.P.is_finite() { format!("{:.2}", z.P) } else { "  -  ".into() }, if z.secilen { "*" } else { " " },
                        f(gol), f(yedi), f(kayip), if kayip > 0 { kayip_t / kayip as f64 } else { f64::NAN }, f(son_biz), f(son_rak));
                    let l = format!("{l} | ilk {erken} tikte kayıp {:.2} sendeleme {:.2} | T {:.0}", f(erk), f(send), z.T);
                    println!("{l}");
                }
            }
        }
        return;
    }
    // an: <kayıt> <t1,t2,…> an <önek> [süre]: maç bugünkü kafayla oynar; her ti'de kopya maç iki kez oynatılır
    // (bugünkü kafa / tutmayı gerçeğe yakın kestiren kafa), kareler <önek>-<ti>-{eski,yeni}.json (kare kipinin biçimi)
    if kip == "an" {
        let tikler: Vec<u32> = a[2].split(',').filter_map(|x| x.parse().ok()).collect();
        let onek = a.get(4).expect("an <önek>").clone();
        let sure: u32 = a.get(5).and_then(|x| x.parse().ok()).unwrap_or(120);
        let son = *tikler.iter().max().unwrap_or(&0);
        let yaz = |m: &alan_motor::model::Mac, kareler: &Vec<String>, olaylar: &Vec<String>, isler: &alan_motor::oyun::IsSozluk, yol: &str| {
            let ad: Vec<String> = m.ps.iter().map(|p| serde_json::to_string(&*p.govde.name).unwrap()).collect();
            let tk: Vec<String> = m.ps.iter().map(|p| format!("{}", p.team)).collect();
            let rl: Vec<String> = m.ps.iter().map(|p| serde_json::to_string(&*p.govde.role).unwrap()).collect();
            let is: Vec<String> = isler.yazilar.iter().map(|j| serde_json::to_string(j).unwrap()).collect();
            let j = format!("{{\"motor\":\"Rust motoru\",\"ad\":[{}],\"takim\":[{}],\"rol\":[{}],\"isler\":[{}],\"olay\":[{}],\"k\":[{}]}}", ad.join(","), tk.join(","), rl.join(","), is.join(","), olaylar.join(","), kareler.join(","));
            std::fs::write(yol, j).expect("yazılamadı");
        };
        while mt.tick() < son && !mt.bitti() {
            let t = mt.tick() + 1;
            if tikler.contains(&t) {
                let mut once = mt.m.clone();
                once.mac.kafa = Default::default();
                // ALAN_AN_YENI: "kal" (varsayılan: tutma kestirimi ölçülen gerçeğe eşlenir) | "mek" (sendeleme yarışı + dolanma) | "mekg" (mek + sendeleme gecikmesi)
                let an_yeni = std::env::var("ALAN_AN_YENI").unwrap_or_default();
                let mek = an_yeni.starts_with("mek");
                let mekg = an_yeni.starts_with("mekg");
                let mekgu = an_yeni == "mekgu";
                for (kip2, acik) in [("eski", false), ("yeni", true)] {
                    alan_motor::kafa::kafa1_ikili::KP_KAL_ZORLA.store(acik && !mek, std::sync::atomic::Ordering::Relaxed);
                    alan_motor::kafa::kafa1_ikili::MEKANIZMA_ZORLA.store(acik && mek, std::sync::atomic::Ordering::Relaxed);
                    alan_motor::kafa::kafa1_ikili::SEND_GEC_ZORLA.store(acik && mekg, std::sync::atomic::Ordering::Relaxed);
                    alan_motor::kafa::kafa2_tarti::GORUS_ZORLA.store(acik && mekgu, std::sync::atomic::Ordering::Relaxed);
                    let ek = std::env::var("ALAN_EK").unwrap_or_default();
                    alan_motor::kural::gec::GEC_DONUS_ZORLA.store(acik && ek.split(',').any(|x| x == "donus"), std::sync::atomic::Ordering::Relaxed);
                    alan_motor::sekil::gor::ARKADAS_NIYET_ZORLA.store(acik && ek.split(',').any(|x| x == "niyet"), std::sync::atomic::Ordering::Relaxed);
                    alan_motor::kafa::kafa1_ucus::ALIS_KUME_ZORLA.store(acik && ek.split(',').any(|x| x == "kume"), std::sync::atomic::Ordering::Relaxed);
                    alan_motor::kafa::kafa2_tarti::TEK_SUT_ZORLA.store(acik && ek.split(',').any(|x| x == "tek"), std::sync::atomic::Ordering::Relaxed);
                    alan_motor::kafa::kafa1_ucus::BEK_DALIS_ZORLA.store(acik && ek.split(',').any(|x| x == "dalis"), std::sync::atomic::Ordering::Relaxed);
                    alan_motor::kafa::kafa2_imkan::SUT_GERCEK_ZORLA.store(acik && ek.split(',').any(|x| x == "sutgercek"), std::sync::atomic::Ordering::Relaxed);
                    let mut m2 = Motor::yeni(once.clone());
                    let mut isler2 = alan_motor::oyun::IsSozluk::default();
                    let (mut kr, mut ol) = (Vec::new(), Vec::new());
                    let mut osay = m2.m.ev_n;
                    let karar_yaz = std::env::var("ALAN_AN_KARAR").is_ok();
                    let mut son_k = None;
                    for _ in 0..sure {
                        if m2.bitti() { break; }
                        m2.adim();
                        if karar_yaz {
                            // ALAN_AN_KARAR: kopya maçtaki her yeni taşıyıcı kararı (kararlar kipinin biçimi, başta değişke) stderr'e
                            let m = &m2.m;
                            let t = m._sec.as_ref().map(|s| s.t);
                            if t.is_some() && t != son_k {
                                son_k = t;
                                if let Some(d) = &m.lastDec {
                                    let o: Vec<String> = d.opts.iter().map(|o| format!("[{},{},{:.3},{}]", serde_json::to_string(&o.kind).unwrap(), serde_json::to_string(&o.q).unwrap(), o.v, o.to.as_ref().map_or("-".to_string(), |t| format!("({:.1},{:.1})", t.x, t.y)))).collect();
                                    // her türün en iyisi (bütün seçenekler)
                                    let mut en: Vec<(String, f64, f64, f64)> = Vec::new();
                                    if let Some(sc) = &m._sec { for e in &sc.o { match en.iter_mut().find(|x| x.0 == e.k) { Some(x) => if e.v > x.1 { *x = (e.k.clone(), e.v, e.x, e.y) }, None => en.push((e.k.clone(), e.v, e.x, e.y)) } } }
                                    let en: Vec<String> = en.iter().map(|x| format!("{}={:.3}@({:.0},{:.0})", x.0, x.1, x.2, x.3)).collect();
                                    let pp: Vec<String> = d.opts.iter().map(|o| format!("{:.2}", o.P)).collect();
                                    eprintln!("KARAR {} {} {} {} ({:.1},{:.1}) {} | P {} | en {}", kip2, m.tick, d.name, d.team, d.x, d.y, o.join(" "), pp.join(","), en.join(" "));
                                }
                            }
                        }
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
                alan_motor::kafa::kafa1_ikili::MEKANIZMA_ZORLA.store(false, std::sync::atomic::Ordering::Relaxed);
                alan_motor::kafa::kafa1_ikili::SEND_GEC_ZORLA.store(false, std::sync::atomic::Ordering::Relaxed);
                alan_motor::kafa::kafa2_tarti::GORUS_ZORLA.store(false, std::sync::atomic::Ordering::Relaxed);
                alan_motor::kural::gec::GEC_DONUS_ZORLA.store(false, std::sync::atomic::Ordering::Relaxed);
                alan_motor::sekil::gor::ARKADAS_NIYET_ZORLA.store(false, std::sync::atomic::Ordering::Relaxed);
                alan_motor::kafa::kafa1_ucus::ALIS_KUME_ZORLA.store(false, std::sync::atomic::Ordering::Relaxed);
                alan_motor::kafa::kafa2_tarti::TEK_SUT_ZORLA.store(false, std::sync::atomic::Ordering::Relaxed);
                alan_motor::kafa::kafa1_ucus::BEK_DALIS_ZORLA.store(false, std::sync::atomic::Ordering::Relaxed);
                alan_motor::kafa::kafa2_imkan::SUT_GERCEK_ZORLA.store(false, std::sync::atomic::Ordering::Relaxed);
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
                        writeln!(out, "[{},{},{:.2},{},{},{},{},{},[{},{}],{:?},{},{:.3}]", m.tick, serde_json::to_string(&*hp.govde.name).unwrap(), dk, sira, z.n, if z.P.is_finite() { format!("{}", z.P) } else { "null".into() }, z.v, z.secilen, z.to.0, z.to.1, sonuc, j, den_min).unwrap();
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
            eprintln!("{} hist {:?} past {:?} _bk {:?} job {:?} _lc {} sizeof {}", p.govde.name, p.hal.hist.as_ref().map(|v| v.len()), p.hal.past.as_ref().map(|v| v.len()), p.hal._bk.as_ref().map(|v| v.len()), p.job.as_ref().map(|j| j.len()), p.hal._lc.is_some(), std::mem::size_of::<alan_motor::model::Oyuncu>());
        }
        let t = std::time::Instant::now();
        let mut n = 0usize;
        for _ in 0..100000 { let c = mt.m.ps[3].clone(); n += c.govde.name.len(); }
        eprintln!("klon 100k: {:?} ({})", t.elapsed(), n);
        let mut q = mt.m.ps[3].clone(); q.hal_mut().hist = None; q.hal_mut().past = None; q.hal_mut()._bk = None;
        let t = std::time::Instant::now();
        for _ in 0..100000 { let c = q.clone(); n += c.govde.name.len(); }
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
        p.govde_mut().yet = Default::default();
        println!("{:#?}", p);
    }
    if kip == "kare" {
        let m = &mt.m;
        let yol = a.get(4).expect("kare <cikti.json>");
        let ad: Vec<String> = m.ps.iter().map(|p| serde_json::to_string(&*p.govde.name).unwrap()).collect();
        let tk: Vec<String> = m.ps.iter().map(|p| format!("{}", p.team)).collect();
        let rl: Vec<String> = m.ps.iter().map(|p| serde_json::to_string(&*p.govde.role).unwrap()).collect();
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
