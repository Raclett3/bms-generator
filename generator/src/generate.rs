use crate::chord::ChordDensity;
use crate::rng::RNG;

pub const CHORDS_PER_BAR: usize = 16;
pub const LANES: usize = 7;

pub struct ChartParams {
    chord_density: ChordDensity,
    bpm: f32,
    bars: usize,
    seed: u64,
}

impl ChartParams {
    pub fn new(chord_density: ChordDensity, bpm: f32, bars: usize, seed: u64) -> Self {
        ChartParams {
            chord_density,
            bpm,
            bars,
            seed,
        }
    }
}

type Chord = Vec<u8>;

pub struct Chart {
    pub bpm: f32,
    pub bars: Vec<Vec<Chord>>,
}

impl Chart {
    fn new(bpm: f32) -> Self {
        Chart {
            bpm,
            bars: Vec::new(),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
struct NoteRandomizer {
    // TODO: 軸補正などを行うために weight を実装
    priority: Vec<usize>,
}

impl NoteRandomizer {
    fn new(priority: Vec<usize>) -> Self {
        assert!(priority.len() == LANES);
        NoteRandomizer { priority }
    }

    fn from_context(context: &GenerateContext) -> Self {
        let Some(last_chord) = context.generated_chords.last() else {
            return NoteRandomizer::new(vec![0; LANES]);
        };

        NoteRandomizer {
            priority: (0..LANES as u8)
                .map(|i| last_chord.contains(&i).into())
                .collect(),
        }
    }

    fn generate(&self, mut count: usize, rng: &mut RNG) -> Vec<u8> {
        let mut lanes_by_priority = Vec::new();
        let mut selected_notes: Vec<u8> = Vec::new();

        for (i, priority) in self.priority.iter().copied().enumerate() {
            if lanes_by_priority.len() < priority + 1 {
                lanes_by_priority.resize(priority + 1, Vec::new());
            }

            lanes_by_priority[priority].push(i as u8);
        }

        for mut lanes in lanes_by_priority {
            if count == 0 {
                break;
            }

            if lanes.len() <= count {
                count -= lanes.len();
                selected_notes.append(&mut lanes);
            } else {
                for _ in 0..count {
                    let idx = rng.next() as usize % lanes.len();
                    selected_notes.push(lanes.swap_remove(idx));
                }

                break;
            }
        }

        selected_notes.sort_unstable();

        selected_notes
    }
}

struct GenerateContext {
    generated_chords: Vec<Chord>,
    rng: RNG,
}

impl GenerateContext {
    fn new(seed: u64) -> Self {
        GenerateContext {
            generated_chords: Vec::new(),
            rng: RNG::new_u64(seed),
        }
    }
}

fn generate_bar(chord_density: &ChordDensity, context: &mut GenerateContext) -> Vec<Chord> {
    (0..CHORDS_PER_BAR)
        .map(|i| {
            let count = chord_density.generate_chord_density(i, &mut context.rng);
            let randomizer = NoteRandomizer::from_context(context);
            let notes = randomizer.generate(count as usize, &mut context.rng);
            context.generated_chords.push(notes.clone());
            notes
        })
        .collect()
}

pub fn generate_chart(params: &ChartParams) -> Chart {
    let mut context = GenerateContext::new(params.seed);
    let mut chart = Chart::new(params.bpm);

    for _ in 0..params.bars {
        let bar = generate_bar(&params.chord_density, &mut context);
        chart.bars.push(bar);
    }

    chart
}

#[cfg(test)]
mod test {
    use super::{generate_chart, ChartParams, GenerateContext, NoteRandomizer};
    use crate::{chord::ChordDensity, generate::CHORDS_PER_BAR};
    use approx::assert_relative_eq;

    #[test]
    fn test_note_randomizer() {
        let mut context = GenerateContext::new(123);
        context.generated_chords.push(vec![0, 2, 4, 6]);
        let randomizer = NoteRandomizer::from_context(&context);

        assert_eq!(randomizer, NoteRandomizer::new(vec![1, 0, 1, 0, 1, 0, 1]));

        for _ in 0..1000 {
            assert_eq!(randomizer.generate(3, &mut context.rng), vec![1, 3, 5]);
        }

        for _ in 0..1000 {
            assert_eq!(
                randomizer.generate(7, &mut context.rng),
                vec![0, 1, 2, 3, 4, 5, 6],
            );
        }
    }

    #[test]
    fn test_generate_chart() {
        let params = ChartParams {
            chord_density: ChordDensity::new(vec![vec![300]]),
            bpm: 222.22,
            bars: 64,
            seed: 199024,
        };

        let chart = generate_chart(&params);

        assert_relative_eq!(params.bpm, chart.bpm);

        for bar in chart.bars.iter() {
            assert_eq!(bar.len(), CHORDS_PER_BAR);
        }

        let flatten_chart: Vec<_> = chart.bars.into_iter().flatten().collect();

        for window in flatten_chart.windows(2) {
            assert!(window[0].iter().all(|x| !window[1].contains(x)));
        }
    }
}
