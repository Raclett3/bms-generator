use crate::rng::RNG;

#[derive(PartialEq, Debug, Clone)]
pub struct ChordDensity {
    density_seq: Vec<Vec<u64>>,
}

impl ChordDensity {
    pub fn new(density_seq: Vec<Vec<u64>>) -> Self {
        ChordDensity { density_seq }
    }

    pub fn from_power_of_two(freq: &[u64]) -> Self {
        assert!(freq.len() > 0);
        let density_seq = (0..1 << (freq.len() - 1))
            .map(|i| {
                freq.into_iter()
                    .rev()
                    .cloned()
                    .enumerate()
                    .filter(|&(j, _)| (1 << j) - 1 & i == 0)
                    .map(|(_, x)| x)
                    .collect()
            })
            .collect();

        Self::new(density_seq)
    }

    pub fn generate_chord_density(&self, index: usize, rng: &mut RNG) -> u64 {
        let densities = &self.density_seq[index % self.density_seq.len()];
        densities
            .into_iter()
            .map(|d| {
                if d % 100 > rng.next() % 100 {
                    d / 100 + 1
                } else {
                    d / 100
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::ChordDensity;
    use crate::rng::RNG;

    #[test]
    fn test_from_power_two() {
        let density = ChordDensity::from_power_of_two(&[1, 2, 4, 8, 16]);

        assert_eq!(
            density.density_seq,
            vec![
                vec![16, 8, 4, 2, 1],
                vec![16],
                vec![16, 8],
                vec![16],
                vec![16, 8, 4],
                vec![16],
                vec![16, 8],
                vec![16],
                vec![16, 8, 4, 2],
                vec![16],
                vec![16, 8],
                vec![16],
                vec![16, 8, 4],
                vec![16],
                vec![16, 8],
                vec![16],
            ]
        );
    }

    #[test]
    fn test_next_chord_count() {
        let density = ChordDensity::from_power_of_two(&[100, 100, 100, 100]);
        let mut rng = RNG::new_u64(123456);

        assert_eq!(
            (0..8)
                .map(|i| density.generate_chord_density(i, &mut rng))
                .collect::<Vec<_>>(),
            vec![4, 1, 2, 1, 3, 1, 2, 1],
        );

        let density = ChordDensity::new(vec![vec![200, 150]]);

        for _ in 0..10000 {
            assert!((3..=4).contains(&density.generate_chord_density(0, &mut rng)));
        }
    }
}
