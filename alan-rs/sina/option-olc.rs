// Option alanlarının maçta dolu / boş görülme ölçümü (MIMARI.md 3.6, Aşama 5-6). Kullanım: motor/tests/ altına
// kopyala, OPT_KUR=<kurulum.json,...> OPT_N=<tik> OPT_CIK=<çıktı> cargo test --release -p alan-motor --test option-olc -- --nocapture
use alan_motor::oyun::MacMotoru;
use std::collections::BTreeMap;
// {:#?} çıktısında her "ad: None" / "ad: Some(" satırını yolu (girintiye göre üst adlar) ile say
fn yuru(d: &str, g: &mut BTreeMap<String, (u64, u64)>) {
    let mut yigin: Vec<String> = Vec::new();
    for l in d.lines() {
        let girinti = l.len() - l.trim_start().len();
        let derin = girinti / 4;
        yigin.truncate(derin);
        let r = l.trim_start();
        let (ad, deger) = match r.split_once(": ") { Some((a, v)) if a.chars().all(|c| c.is_alphanumeric() || c == '_') => (a.to_string(), v), _ => {
            // adsız öğe (dizi öğesi, "Oyuncu {")
            let t = r.trim_end_matches(['{', '(', '[', ' ']).to_string();
            while yigin.len() < derin { yigin.push(String::new()); }
            if r.ends_with('{') || r.ends_with('(') || r.ends_with('[') { yigin.push(if t.is_empty() { "*".into() } else { t }); }
            continue;
        } };
        while yigin.len() < derin { yigin.push(String::new()); }
        let yol: Vec<&str> = yigin.iter().map(|s| s.as_str()).filter(|s| !s.is_empty() && !s.chars().next().unwrap().is_lowercase() || false).collect();
        let tip = yigin.iter().rev().find(|s| s.chars().next().map_or(false, |c| c.is_uppercase())).cloned().unwrap_or("?".into());
        let _ = yol;
        let bos = deger.starts_with("None");
        if bos || deger.starts_with("Some(") {
            let e = g.entry(format!("{tip}.{ad}")).or_insert((0, 0));
            if bos { e.1 += 1 } else { e.0 += 1 }
        }
        if deger.ends_with('{') || deger.ends_with('(') || deger.ends_with('[') {
            let t = deger.trim_start_matches("Some(").trim_end_matches(['{', '(', '[', ' ']).to_string();
            yigin.push(if t.is_empty() || t == "Some" { ad.clone() } else { t });
        }
    }
}
#[test]
fn option_olc() {
    let dosyalar = std::env::var("OPT_KUR").unwrap();
    let n: usize = std::env::var("OPT_N").unwrap().parse().unwrap();
    let mut g: BTreeMap<String, (u64, u64)> = BTreeMap::new();
    for f in dosyalar.split(',') {
        let v: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(f).unwrap()).unwrap();
        let k = v.get("kurulum").cloned().unwrap_or(v);
        let mut mt = alan_motor::mac::motor::Motor::kayittan(&k).unwrap();
        for t in 0..n {
            if mt.bitti() { break; }
            mt.adim();
            if t % 9 != 0 { continue; }
            let d = format!("{:#?}", mt.m.m);
            yuru(&d, &mut g);
        }
    }
    let mut out = String::new();
    for (k, (d, b)) in &g { out.push_str(&format!("{k} {d} {b}\n")); }
    std::fs::write(std::env::var("OPT_CIK").unwrap(), out).unwrap();
}
