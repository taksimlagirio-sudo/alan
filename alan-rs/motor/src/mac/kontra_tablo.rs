//! Kontra değer tablosu (analiz/KONTRA-TABLO.md): kaleye uzaklık × topun önündeki rakip savunmacı × top kazanılalı geçen
//! süre → "N sn içinde gol". Hız kararı (enerji::sprint_karar3, hiz2 + ALAN_KTAB_IZ) kontra koşusunun değerini buradan okur.
//! Tablo gömülü (kontra_tablo.tsv, wasm'da da); yerelde ALAN_KONTRA_TABLO=<yol> ile başka dosya okunur.
use crate::mat::{jmax, jmin};

/// Az örnekli hücre, yedek değerle (enerji::KONTRA_GOL) n / (n + KTAB_N0) ağırlıkla harmanlanır.
pub const KTAB_N0: f64 = 3.0;

pub struct KontraTablo {
    /// eksenler (artan): uzak, savunmacı (tamsayı), süre
    uz: Vec<f64>,
    sv: Vec<f64>,
    su: Vec<f64>,
    /// hücre: (gol, n), [uz][sv][su]; yoksa None
    h: Vec<Option<(f64, f64)>>,
}

fn sirala(mut v: Vec<f64>) -> Vec<f64> {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v.dedup();
    v
}

impl KontraTablo {
    pub fn coz(metin: &str) -> Option<KontraTablo> {
        let mut sat: Vec<[f64; 5]> = Vec::new();
        for l in metin.lines() {
            let l = l.trim();
            if l.is_empty() || l.starts_with('#') || l.starts_with("uzak") {
                continue;
            }
            let v: Vec<f64> = l.split('\t').filter_map(|x| x.trim().parse::<f64>().ok()).collect();
            if v.len() >= 4 {
                sat.push([v[0], v[1], v[2], v[3], *v.get(4).unwrap_or(&KTAB_N0)]);
            }
        }
        if sat.is_empty() {
            return None;
        }
        let uz = sirala(sat.iter().map(|s| s[0]).collect());
        let sv = sirala(sat.iter().map(|s| s[1]).collect());
        let su = sirala(sat.iter().map(|s| s[2]).collect());
        let mut h = vec![None; uz.len() * sv.len() * su.len()];
        let ix = |v: &Vec<f64>, x: f64| v.iter().position(|&y| y == x).unwrap();
        for s in &sat {
            let k = (ix(&uz, s[0]) * sv.len() + ix(&sv, s[1])) * su.len() + ix(&su, s[2]);
            h[k] = Some((s[3], s[4]));
        }
        Some(KontraTablo { uz, sv, su, h })
    }

    /// Bir hücre: tablonun değeri az örnekte yedekle harmanlanır; hücre yoksa yedek.
    fn hucre(&self, i: usize, j: usize, k: usize, yedek: f64) -> f64 {
        match self.h[(i * self.sv.len() + j) * self.su.len() + k] {
            Some((g, n)) => {
                let w = n / (n + KTAB_N0);
                w * g + (1.0 - w) * yedek
            }
            None => yedek,
        }
    }

    /// (uzaklık br, savunmacı sayısı (yumuşak), geçen süre sn) → gol olasılığı; eksenler boyunca doğrusal ara değer, uçlarda
    /// sınırda. `yedek(n)`: tamsayı savunmacı için yedek değer.
    pub fn oku(&self, uzak: f64, sav: f64, sure: f64, yedek: &dyn Fn(f64) -> f64) -> f64 {
        let ara = |v: &Vec<f64>, x: f64| -> (usize, usize, f64) {
            if x <= v[0] {
                return (0, 0, 0.0);
            }
            let n = v.len();
            if x >= v[n - 1] {
                return (n - 1, n - 1, 0.0);
            }
            let i = v.iter().rposition(|&y| y <= x).unwrap();
            (i, i + 1, (x - v[i]) / (v[i + 1] - v[i]))
        };
        let (i0, i1, fi) = ara(&self.uz, uzak);
        let (j0, j1, fj) = ara(&self.sv, jmax(0.0, sav));
        let (k0, k1, fk) = ara(&self.su, jmax(0.0, sure));
        let mut t = 0.0;
        for (i, wi) in [(i0, 1.0 - fi), (i1, fi)] {
            for (j, wj) in [(j0, 1.0 - fj), (j1, fj)] {
                for (k, wk) in [(k0, 1.0 - fk), (k1, fk)] {
                    let w = wi * wj * wk;
                    if w > 0.0 {
                        t += w * self.hucre(i, j, k, yedek(self.sv[j]));
                    }
                }
            }
        }
        jmin(1.0, jmax(0.0, t))
    }
}

/// Gömülü tablo (yerelde ALAN_KONTRA_TABLO=<yol> ile başka dosya).
pub fn tablo() -> Option<&'static KontraTablo> {
    static T: std::sync::OnceLock<Option<KontraTablo>> = std::sync::OnceLock::new();
    T.get_or_init(|| {
        if crate::iz_acik() {
            if let Ok(yol) = std::env::var("ALAN_KONTRA_TABLO") {
                if let Ok(m) = std::fs::read_to_string(&yol) {
                    return KontraTablo::coz(&m);
                }
            }
        }
        KontraTablo::coz(include_str!("kontra_tablo.tsv"))
    })
    .as_ref()
}

/// Deney (ALAN_KTAB_IZ; ölçümde/wasm'da KTAB_ZORLA): hiz2'de kontra koşusunun değeri bu tablodan. Varsayılan kapalı.
pub static KTAB_ZORLA: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub fn ktab() -> bool {
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    KTAB_ZORLA.load(std::sync::atomic::Ordering::Relaxed) || *A.get_or_init(|| crate::iz_acik() && std::env::var("ALAN_KTAB_IZ").is_ok())
}

#[cfg(test)]
mod sina {
    use super::*;
    #[test]
    fn ara_deger() {
        let t = KontraTablo::coz("uzak\tsavunmaci\tsure\tgol\tn\n25\t0\t0\t0.8\t1000\n25\t1\t0\t0.4\t1000\n45\t0\t0\t0.4\t1000\n45\t1\t0\t0.2\t1000\n").unwrap();
        let y = |_: f64| 0.0;
        assert!((t.oku(25.0, 0.0, 0.0, &y) - 0.8).abs() < 0.01);
        assert!((t.oku(35.0, 0.5, 5.0, &y) - 0.45).abs() < 0.01);
        assert!((t.oku(100.0, 9.0, 0.0, &y) - 0.2).abs() < 0.01);
    }
}
