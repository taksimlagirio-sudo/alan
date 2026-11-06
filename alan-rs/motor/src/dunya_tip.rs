//! Kafanın dünyasının tipi (temel katman: `model` ve üstü kullanır; dünya katmanı yeniden dışa açar).
use std::sync::Arc;

use crate::model::Oyuncu;

/// Kafanın dünyasının öğesi: paylaşımlı oyuncu.
pub type Op = Arc<Oyuncu>;

/// Kafanın dünyası (MIMARI.md 2.3, 4c-2c): oyuncuların paylaşımlı gövdeleri. Bir varsayımsal dünya çoğu
/// oyuncuyu taban dünyayla paylaşır, yalnız değişenleri yenidir (tik başına ~2100 dünya, ~1450 yeni oyuncu).
/// Bugün iç yapı düz bir dizi; dilim gibi okunur (`Deref<Target = [Op]>`). Aşama 4c-2c'nin hedefi: taban +
/// değişenler.
#[derive(Clone, Debug, Default)]
pub struct Dunya(Vec<Op>);
impl Dunya {
    #[inline]
    pub fn yeni(v: Vec<Op>) -> Dunya {
        Dunya(v)
    }
    #[inline]
    pub fn with_capacity(n: usize) -> Dunya {
        Dunya(Vec::with_capacity(n))
    }
    #[inline]
    pub fn push(&mut self, o: Op) {
        self.0.push(o)
    }
    /// İç dizinin kopyası (paylaşımlı gövdeler; oyuncular kopyalanmaz).
    #[inline]
    pub fn to_vec(&self) -> Vec<Op> {
        self.0.clone()
    }
    #[inline]
    pub fn into_vec(self) -> Vec<Op> {
        self.0
    }
    /// `i`'nci oyuncuyu değiştir (yeni gövde).
    #[inline]
    pub fn koy(&mut self, i: usize, o: Op) {
        self.0[i] = o
    }
    /// Sona oyuncular ekle (iki takımın dünyalarını birleştirmek için).
    #[inline]
    pub fn extend(&mut self, it: impl IntoIterator<Item = Op>) {
        self.0.extend(it)
    }
    #[inline]
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Op> {
        self.0.iter_mut()
    }
}
impl std::ops::Deref for Dunya {
    type Target = [Op];
    #[inline(always)]
    fn deref(&self) -> &[Op] {
        &self.0
    }
}
impl FromIterator<Op> for Dunya {
    fn from_iter<I: IntoIterator<Item = Op>>(i: I) -> Dunya {
        Dunya(i.into_iter().collect())
    }
}
impl From<Vec<Op>> for Dunya {
    #[inline]
    fn from(v: Vec<Op>) -> Dunya {
        Dunya(v)
    }
}
impl<'a> IntoIterator for &'a Dunya {
    type Item = &'a Op;
    type IntoIter = std::slice::Iter<'a, Op>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
