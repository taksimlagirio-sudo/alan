//! Bilgi tabloları (analiz/TASARIM-BILGI.md): motorun kendi maçlarından ölçülen, kafanın kendi modeliyle harmanlanan tablolar.
//! Dosya: motor/veri/bilgi/<ad>.json (derlemeye gömülü, maç içinde değişmez). Üretim ve eskime denetimi: sina/bilgi.sh.
//! Biçim (bicim 1): ad, surum, uretim, kural_izi, davranis_izi, bayraklar, mac, tohumlar, k, eksen [{ad, dugum}],
//! deger ve n (düz dizi, ilk eksen en dışta). Okuma: çoklu doğrusal ara değer (değer ve n birlikte); harmanlama
//! (n·tablo + k·model) / (n + k): az örnekli hücrede model, çok örnekli hücrede ölçüm baskın.
#![allow(non_snake_case)]

pub struct Bilgi {
    pub ad: String,
    pub eksen: Vec<(String, Vec<f64>)>,
    pub deger: Vec<f64>,
    pub n: Vec<f64>,
    pub k: f64,
}

fn coz(ad: &str, metin: &str) -> Option<Bilgi> {
    let j: serde_json::Value = serde_json::from_str(metin).ok()?;
    if j.get("bicim").and_then(|b| b.as_i64()) != Some(1) {
        eprintln!("bilgi/{ad}.json: bilinmeyen biçim");
        return None;
    }
    let sayi = |v: &serde_json::Value| -> Vec<f64> { v.as_array().map(|a| a.iter().filter_map(|x| x.as_f64()).collect()).unwrap_or_default() };
    let eksen: Vec<(String, Vec<f64>)> = j["eksen"].as_array()?.iter().filter_map(|e| Some((e["ad"].as_str()?.to_string(), sayi(&e["dugum"])))).collect();
    let deger = sayi(&j["deger"]);
    let n = sayi(&j["n"]);
    let boy: usize = eksen.iter().map(|e| e.1.len()).product();
    if boy == 0 || boy != deger.len() || boy != n.len() {
        eprintln!("bilgi/{ad}.json: eksen ile değer/n boyu uyuşmuyor");
        return None;
    }
    Some(Bilgi { ad: ad.to_string(), eksen, deger, n, k: j["k"].as_f64().unwrap_or(30.0) })
}

/// Gömülü tablo (adıyla); yoksa ya da bozuksa None.
pub fn bilgi(ad: &str) -> Option<&'static Bilgi> {
    static TUTMA: std::sync::OnceLock<Option<Bilgi>> = std::sync::OnceLock::new();
    match ad {
        "tutma" => TUTMA.get_or_init(|| coz("tutma", include_str!("../../veri/bilgi/tutma.json"))).as_ref(),
        _ => None,
    }
}

impl Bilgi {
    /// (değer, n) ara değerli; `x`: eksen adı → değer (verilmeyen eksen ilk düğümde).
    pub fn oku(&self, x: &[(&str, f64)]) -> (f64, f64) {
        let mut parca: Vec<(usize, f64, usize)> = Vec::with_capacity(self.eksen.len());
        for (ad, d) in &self.eksen {
            let v = x.iter().find(|(a, _)| a == ad).map(|p| p.1).unwrap_or(d[0]);
            let m = d.len();
            let (i, f) = if m < 2 || v <= d[0] {
                (0, 0.0)
            } else if v >= d[m - 1] {
                (m - 2, 1.0)
            } else {
                let i = (0..m - 1).find(|&i| v < d[i + 1]).unwrap_or(m - 2);
                (i, (v - d[i]) / (d[i + 1] - d[i]))
            };
            parca.push((i, f, m));
        }
        let (mut dv, mut dn) = (0.0, 0.0);
        for b in 0..(1usize << parca.len()) {
            let (mut idx, mut w) = (0usize, 1.0);
            for (e, &(i, f, m)) in parca.iter().enumerate() {
                let ust = (b >> e) & 1 == 1;
                if m < 2 {
                    if ust {
                        w = 0.0;
                    }
                    idx *= m;
                    continue;
                }
                w *= if ust { f } else { 1.0 - f };
                idx = idx * m + i + ust as usize;
            }
            if w > 0.0 {
                dv += w * self.deger[idx];
                dn += w * self.n[idx];
            }
        }
        (dv, dn)
    }
    /// Kafanın kendi modeliyle harmanlanmış değer: (n·tablo + k·model) / (n + k).
    pub fn harmanla(&self, x: &[(&str, f64)], model: f64) -> f64 {
        let (v, n) = self.oku(x);
        (n * v + self.k * model) / (n + self.k)
    }
}
