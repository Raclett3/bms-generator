use crate::rng::RNG;

pub struct Rule {
    freq_seq: Vec<Vec<u64>>,
}

impl Rule {
    pub fn new(freq_seq: Vec<Vec<u64>>) -> Self {
        Rule { freq_seq }
    }

    pub fn from_power_of_two(freq: &[u64]) -> Self {
        assert!(freq.len() > 0);
        let freq_seq = (0..1 << (freq.len() - 1))
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

        Self::new(freq_seq)
    }

    pub fn next_chord_count(&self, index: usize, rng: &mut RNG) -> u64 {
        let freqs = &self.freq_seq[index % self.freq_seq.len()];
        freqs
            .into_iter()
            .map(|freq| {
                if freq % 100 > rng.next() % 100 {
                    freq / 100 + 1
                } else {
                    freq / 100
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::Rule;
    use crate::rng::RNG;

    #[test]
    fn test_from_power_two() {
        let rule = Rule::from_power_of_two(&[1, 2, 4, 8, 16]);

        assert_eq!(
            rule.freq_seq,
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
        let rule = Rule::from_power_of_two(&[100, 100, 100, 100]);
        let mut rng = RNG::new_u64(123456);

        assert_eq!(
            (0..8)
                .map(|i| rule.next_chord_count(i, &mut rng))
                .collect::<Vec<_>>(),
            vec![4, 1, 2, 1, 3, 1, 2, 1],
        );

        let rule = Rule::new(vec![vec![200, 150]]);

        for _ in 0..10000 {
            assert!((3..=4).contains(&rule.next_chord_count(0, &mut rng)));
        }
    }
}
