//! Oyuncu satırı (JS'te maçın `p` nesnesi; envanter: 132 alan). Fizik çekirdeğinin okuduğu alanlar burada;
//! maç ve kafa alanları ilgili modüller taşındıkça eklenir.

use crate::alan::Etki;
use crate::mat::{jmax, jmin};
use crate::yetenek::{mek, spr_h, Yetenek};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rol {
    Bekci,
    Diger,
}

#[derive(Clone, Debug)]
pub struct Oyuncu {
    /// takım içindeki numara (JS id) ve takım (0/1)
    pub id: u32,
    pub team: u8,
    pub rol: Rol,
    /// rolün adı (Bekçi, Forvet…)
    pub rol_ad: String,
    pub name: String,
    pub yet: Yetenek,
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub tx: Option<f64>,
    pub ty: Option<f64>,
    /// bar (0–1); JS `p.en` (null: enerji yok)
    pub en: Option<f64>,
    /// denge (0–1, ikili sonrası); yoksa tam
    pub den: Option<f64>,
    pub no_touch: bool,
    /// Kuyu süresi dolup sönmüş (Kuyu'ya giremez)
    pub sonuk: bool,
    /// Kuyu alanında geçen tik
    pub kuyu_t: f64,
    /// alan kaynağı: yarıçap, şiddet, koni, yön, seçim
    pub r: Option<f64>,
    pub d: Option<f64>,
    pub cone: bool,
    pub fx: f64,
    pub fy: f64,
    pub am_a: bool,
    pub am: f64,
    /// bu topun alıcısı (JS `_rcvB === b`): topun kimliği
    pub rcv_b: Option<u64>,
    /// Bekçi'nin bu tikteki dalış payı (JS `_dalis`)
    pub dalis: f64,
}

impl Oyuncu {
    pub fn bekci(&self) -> bool {
        self.rol == Rol::Bekci
    }
    /// Alan kaynağı olarak görünüşü.
    pub fn etki(&self) -> Etki {
        Etki {
            x: self.x,
            y: self.y,
            r: match self.r {
                Some(r) if r != 0.0 && r == r => r,
                _ => 8.0,
            },
            d: self.d.unwrap_or(1.0),
            koni: self.cone,
            fx: self.fx,
            fy: self.fy,
            team: self.team,
            am_a: self.am_a,
            am: self.am,
            en: self.en.unwrap_or(1.0),
        }
    }
    /// Etkin Hız (Enerji 3.0: bar hızı düşürmez, ham Hız).
    #[inline(always)]
    pub fn hz(&self) -> f64 {
        self.yet.hiz
    }
    /// Temel koşu hızı (br/tik).
    #[inline(always)]
    pub fn taban_hiz(&self) -> f64 {
        0.17 + 0.006 * self.hz()
    }
    /// Sprint katı Hız'dan (çekirdek; maç sprint evresiyle sarar).
    #[inline(always)]
    pub fn spr0(&self) -> f64 {
        spr_h(self.yet.hiz)
    }
}

/// Tepki süreleri (JS ALAN_TEPKI): işe göre tik; bar azaldıkça uzar (Enerji 3.0 tepkiK).
#[derive(Clone, Copy, Debug)]
pub enum Tepki {
    Alan,
    Algi,
    Karar,
    YeniTop,
    Kovala,
    BosTop,
    Uzanma,
}

/// Yerleşimin tepki parametreleri (JS AlanShape.S'den: lag0, lagOk, react, reactLoose, reactLooseOk).
#[derive(Clone, Copy, Debug)]
pub struct TepkiS {
    pub lag0: f64,
    pub lag_ok: f64,
    pub react: f64,
    pub react_loose: f64,
    pub react_loose_ok: f64,
}
impl Default for TepkiS {
    fn default() -> Self {
        TepkiS { lag0: 10.0, lag_ok: 0.4, react: 0.5, react_loose: 6.0, react_loose_ok: 0.25 }
    }
}

pub const EN3_TEPKI_K: f64 = 0.25;

pub fn tepki(p: &Oyuncu, k: Tepki, s: &TepkiS) -> f64 {
    use crate::core_sabit::Q;
    let y = &p.yet;
    let t = match k {
        Tepki::Alan => s.lag0 - y.y(mek::ALAN_ACMA) * s.lag_ok,
        Tepki::Algi => s.lag0 - y.y(mek::ALGI_GECIKME) * s.lag_ok,
        Tepki::Karar => 9.0 - 0.4 * y.y(mek::KARAR_SURE),
        Tepki::YeniTop => Q.tepki0 - y.y(mek::ALGI_GECIKME) * Q.tepki_ok,
        Tepki::Kovala => 14.0 - y.y(mek::PRES_BASLA) * s.react,
        Tepki::BosTop => s.react_loose - y.y(mek::PRES_BASLA) * s.react_loose_ok,
        Tepki::Uzanma => Q.reach_react - y.y(mek::UZANMA_BASLA) * Q.reach_react_ok,
    };
    match p.en {
        Some(e) => t * (1.0 + EN3_TEPKI_K * (1.0 - jmax(0.0, jmin(1.0, e)))),
        None => t,
    }
}
