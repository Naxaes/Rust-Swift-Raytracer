use std::num::{Wrapping, NonZeroU32};

pub struct Random {
    state: Wrapping<u32>,
}

impl Random {
    pub fn new() -> Random {
        Self::new_with_seed(NonZeroU32::new(2547549).unwrap())
    }
    pub fn new_with_seed(seed: NonZeroU32) -> Random {
        Self { state: Wrapping(seed.get()) }
    }
    /// Random number between [0, 1].
    pub fn random_f32(&mut self) -> f32 {
        self.xor_shift_32() as f32 / u32::MAX as f32
    }
    /// Random number between [-1, 1].
    pub fn random_bilateral_f32(&mut self) -> f32 {
        self.random_f32() * 2.0 - 1.0
    }
    fn xor_shift_32(&mut self) -> u32
    {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        x.0
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_is_between_0_and_1() {
        let x = u32::MAX as f32 / u32::MAX as f32;
        assert!(0.0 <= x && x <= 1.0);

        let y = 0.0 / u32::MAX as f32;
        assert!(0.0 <= y && y <= 1.0);
    }
    #[test]
    fn test_is_between_minus_1_and_1() {
        let x = (u32::MAX as f32 / u32::MAX as f32) * 2.0 - 1.0;
        assert!(-1.0 <= x && x <= 1.0);

        let y = (0.0 / u32::MAX as f32) * 2.0 - 1.0;
        assert!(-1.0 <= y && y <= 1.0);
    }
}