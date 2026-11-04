//! Maçın zar dizisi (match.js rng, mulberry32): JS ile bit bit aynı.

#[derive(Clone, Debug)]
pub struct Rng {
    pub s: i32,
}

impl Rng {
    pub fn yeni(seed: u32) -> Rng {
        Rng { s: seed as i32 }
    }
    /// JS `f()`: [0, 1)
    #[inline]
    pub fn sonraki(&mut self) -> f64 {
        let a = self.s.wrapping_add(1831565813);
        self.s = a;
        let a_u = a as u32;
        let mut t = ((a_u ^ (a_u >> 15)) as i32).wrapping_mul((1 | a_u) as i32);
        let tu = t as u32;
        t = (t.wrapping_add(((tu ^ (tu >> 7)) as i32).wrapping_mul((61 | tu) as i32))) ^ t;
        let tu = t as u32;
        ((tu ^ (tu >> 14)) as f64) / 4294967296.0
    }
}
