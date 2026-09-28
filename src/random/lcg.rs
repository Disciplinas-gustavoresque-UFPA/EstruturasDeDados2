// A linear congruential generator (LCG) is an algorithm that yields a sequence of pseudo-randomized 
// numbers calculated with a discontinuous piecewise linear equation. 
// The method represents one of the oldest and best-known pseudorandom number generator algorithms. 
// The theory behind them is relatively easy to understand, and they are easily implemented and fast,
// especially on computer hardware which can provide modular arithmetic by storage-bit truncation.
// Source: https://en.wikipedia.org/wiki/Linear_congruential_generator
const MULTIPLIER: u32 = 1_664_525;
const INCREMENT: u32 = 1_013_904_223;

pub struct Lcg {
    state: u32,
}

impl Lcg {
    pub fn new(seed: u32) -> Self {
        Self { state: seed }
    }

    pub fn next_u32(&mut self) -> u32 {
        // The generator is defined by the recurrence relation:
        // T(n) = a * T(n - 1) + c mod m
        // where 'm' is the modulus
        // 'a' is the multiplier
        // 'c' is the increment
        // and 'n' is the previous value.
        // By using the wrapping function, we are using modulo u32::MAX or (2^32-1)
        self.state = self.state.wrapping_mul(MULTIPLIER).wrapping_add(INCREMENT);

        return self.state;
    }
}

#[cfg(test)]
mod tests {
    use super::Lcg;

    #[test]
    fn test_generates_expected_sequence() {
        let mut rng = Lcg::new(42);
        let next_numbers: Vec<u32> = vec![
            1_083_814_273,
            378_494_188,
            2_479_403_867,
            955_863_294,
            1_613_448_261,
        ];

        for number in next_numbers {
            assert_eq!(rng.next_u32(), number);
        }
    }

    #[test]
    fn test_same_seed_produces_same_sequence() {
        let mut first = Lcg::new(42);
        let mut second = Lcg::new(42);

        for _ in 0..10 {
            assert_eq!(first.next_u32(), second.next_u32());
        }
    }

    #[test]
    fn test_diffent_seeds_produce_different_sequences() {
        let mut first = Lcg::new(42);
        let mut second = Lcg::new(43);

        for _ in 0..10 {
            assert_ne!(first.next_u32(), second.next_u32());
        }
    }

    #[test]
    fn test_unsigned_integer_bounds() {
        let mut random = Lcg::new(u32::MAX);
        assert_eq!(random.next_u32(), 1012239698);

        random = Lcg::new(u32::MIN);

        assert_eq!(random.next_u32(), 1013904223);
    }
}
