pub struct XorShift {
    state: u32
}

impl XorShift {
    pub fn new(seed: u32) -> Self {
        assert_ne!(seed, 0, "XorShift requires a nonzero seed");
        Self { state: seed }
    }

    pub fn next_u32(&mut self) -> u32 {
        let mut value = self.state;
        value ^= value << 13;
        value ^= value >> 17;
        value ^= value << 5;
        self.state = value;
        return value
    }
}

#[cfg(test)]
mod tests {
    use super::XorShift;

    #[test]
    fn test_generates_expected_sequence() {
        let mut rng = XorShift::new(42);
        let expected = [
            11_355_432,
            2_836_018_348,
            476_557_059,
            3_648_046_016,
            3_759_983_556,
        ];

        for value in expected {
            assert_eq!(rng.next_u32(), value);
        }
    }

    #[test]
    fn test_same_seed_produces_same_sequence() {
        let mut first = XorShift::new(42);
        let mut second = XorShift::new(42);

        for _ in 0..10 {
            assert_eq!(first.next_u32(), second.next_u32());
        }
    }

    #[test]
    fn test_different_seeds_produce_different_sequences() {
        let mut first = XorShift::new(42);
        let mut second = XorShift::new(99);

        for _ in 0..10 {
            assert_ne!(first.next_u32(), second.next_u32());
        }
    }

    #[test]
    #[should_panic]
    fn test_seed_zero() {
        XorShift::new(0);
    }
}