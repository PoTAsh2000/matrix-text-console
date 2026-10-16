use std::time::{SystemTime, UNIX_EPOCH};

/// Small xorshift64 random generator.
pub struct Rng(u64);

impl Rng {
    pub fn from_seed(seed: u64) -> Self {
        // State must never be zero
        Self(seed | 1)
    }

    /// Seeds from the system clock.
    pub fn from_time() -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |d| d.as_nanos());
        Self::from_seed(0x9E37_79B9_7F4A_7C15 ^ (nanos as u64))
    }

    fn next_u32(&mut self) -> u32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 32) as u32
    }

    /// Random value in `0..bound`.
    pub fn below(&mut self, bound: u32) -> u32 {
        ((u64::from(self.next_u32()) * u64::from(bound)) >> 32) as u32
    }

    /// Random value in `0.0..1.0`.
    pub fn unit(&mut self) -> f32 {
        (self.next_u32() >> 8) as f32 / 16_777_216.0
    }

    /// Random value in `start..end`.
    pub fn range_f32(&mut self, start: f32, end: f32) -> f32 {
        start + self.unit() * (end - start)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn below_stays_in_bound() {
        let mut rng = Rng::from_seed(42);
        assert!((0..1000).all(|_| rng.below(7) < 7));
    }

    #[test]
    fn unit_stays_in_unit_range() {
        let mut rng = Rng::from_seed(42);
        assert!((0..1000).all(|_| (0.0..1.0).contains(&rng.unit())));
    }

    #[test]
    fn range_f32_stays_in_range() {
        let mut rng = Rng::from_seed(42);
        assert!((0..1000).all(|_| (6.0..18.0).contains(&rng.range_f32(6.0, 18.0))));
    }
}
