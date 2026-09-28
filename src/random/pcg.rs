// Source: https://en.wikipedia.org/wiki/Permuted_congruential_generator
const MULTIPLIER: u64 = 6_364_136_223_846_793_005;
const INCREMENT: u64 = 1_442_695_040_888_963_407;

pub struct Pcg {
    state: u64
}

impl Pcg {
    pub fn new(seed: u64) -> Self {
        let mut rng = Self {
            state: seed.wrapping_add(INCREMENT), 
        };

        rng.next_u32();
        return rng;
    }

    pub fn next_u32(&mut self) -> u32 {
        
        let previous_state = self.state;        
        
        self.state = previous_state
                .wrapping_mul(MULTIPLIER)
                .wrapping_add(INCREMENT);

        let shifted = (((previous_state >> 18) ^ previous_state) >> 27) as u32;
        let rotation = (previous_state >> 59) as u32;
        return shifted.rotate_right(rotation);
    }
}

#[cfg(test)]
mod tests {
    use super::Pcg;

    #[test]
    fn generates_expected_sequence() {
        let mut rng = Pcg::new(42);
        let expected = [
            3_270_867_926,
            1_795_671_209,
            1_924_641_435,
            1_143_034_755,
            4_121_910_957,
        ];

        for value in expected {
            assert_eq!(rng.next_u32(), value);
        }
    }

    #[test]
    fn same_seed_produces_same_sequence() {
        let mut first = Pcg::new(42);
        let mut second = Pcg::new(42);

        for _ in 0..10 {
            assert_eq!(first.next_u32(), second.next_u32());
        }
    }

    #[test]
    fn different_seeds_produce_different_first_values() {
        let mut first = Pcg::new(42);
        let mut second = Pcg::new(43);

        assert_ne!(first.next_u32(), second.next_u32());
    }
}