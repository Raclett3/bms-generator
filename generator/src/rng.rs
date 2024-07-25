pub struct RNG([u64; 2]);

// Xoroshiro128+ seedable RNG
impl RNG {
    pub fn new(seed: [u64; 2]) -> Self {
        RNG(seed)
    }

    pub fn new_u64(seed: u64) -> Self {
        RNG([seed, 0])
    }

    pub fn next(&mut self) -> u64 {
        let [s0, mut s1] = self.0;
        let result = s0.wrapping_add(s1);

        s1 ^= s0;
        self.0[0] = s0.rotate_left(55) ^ s1 ^ (s1 << 14);
        self.0[1] = s1.rotate_left(36);

        result
    }
}

#[cfg(test)]
mod test {
    use super::RNG;
    use std::{
        collections::BTreeSet,
        time::{SystemTime, UNIX_EPOCH},
    };

    #[test]
    fn test_rng() {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        let mut rng = RNG::new_u64(seed);
        let mut set = BTreeSet::new();

        for _ in 0..1000000 {
            let n = rng.next();
            assert!(set.insert(n));
        }
    }
}
