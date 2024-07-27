use crate::chord::ChordDensity;
use crate::rng::RNG;

pub const CHORDS_PER_BAR: usize = 16;
pub const LANES: usize = 7;

pub struct ChartParams {
    chord_density: ChordDensity,
    bpm: f32,
    bars: usize,
    jack_tolerance: f32,
    seed: u64,
}

impl ChartParams {
    pub fn new(
        chord_density: ChordDensity,
        bpm: f32,
        bars: usize,
        jack_tolerance: f32,
        seed: u64,
    ) -> Self {
        ChartParams {
            chord_density,
            bpm,
            bars,
            jack_tolerance,
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
    reroll_chance: Vec<f32>,
}

impl NoteRandomizer {
    fn new(priority: Vec<usize>, reroll_chance: Vec<f32>) -> Self {
        assert!(priority.len() == LANES);
        assert!(reroll_chance.len() == LANES);
        NoteRandomizer {
            priority,
            reroll_chance,
        }
    }

    fn from_context(context: &GenerateContext, jack_tolerance: f32) -> Self {
        let max_jacks = jack_tolerance.ceil() as usize;
        let reroll_chance = (1.0 - jack_tolerance.fract()) % 1.0;

        NoteRandomizer::new(
            context
                .ongoing_jacks
                .iter()
                .map(|&jacks| jacks.saturating_sub(max_jacks))
                .collect(),
            context
                .ongoing_jacks
                .iter()
                .map(|&jacks| {
                    if jacks == max_jacks {
                        reroll_chance
                    } else {
                        0.0
                    }
                })
                .collect(),
        )
    }

    fn generate(&mut self, mut count: usize, rng: &mut RNG) -> Vec<u8> {
        let mut lanes_by_priority = Vec::new();
        let mut selected_notes: Vec<u8> = Vec::new();

        for (i, priority) in self.priority.iter().copied().enumerate() {
            if lanes_by_priority.len() < priority + 1 {
                lanes_by_priority.resize(priority + 1, Vec::new());
            }

            lanes_by_priority[priority].push(i as u8);
        }

        let mut deferred_lanes = Vec::new();

        'outer: for mut lanes in lanes_by_priority {
            lanes.append(&mut deferred_lanes);

            while !lanes.is_empty() {
                if count == 0 {
                    break 'outer;
                }

                let idx = rng.next() as usize % lanes.len();
                let lane = lanes.swap_remove(idx);

                let reroll_chance = self.reroll_chance[lane as usize];
                self.reroll_chance[lane as usize] = 0.0;

                if reroll_chance > rng.next_f32() {
                    deferred_lanes.push(lane);
                    continue;
                }

                count -= 1;
                selected_notes.push(lane);
            }
        }

        selected_notes.sort_unstable();

        selected_notes
    }
}

struct GenerateContext {
    generated_chords: Vec<Chord>,
    rng: RNG,
    ongoing_jacks: Vec<usize>,
}

impl GenerateContext {
    fn new(seed: u64) -> Self {
        GenerateContext {
            generated_chords: Vec::new(),
            rng: RNG::new_u64(seed),
            ongoing_jacks: vec![0; LANES],
        }
    }

    fn push_chord(&mut self, chord: Chord) {
        for (i, jacks) in self.ongoing_jacks.iter_mut().enumerate() {
            if chord.contains(&(i as u8)) {
                *jacks += 1;
            } else {
                *jacks = 0;
            }
        }
        self.generated_chords.push(chord);
    }
}

fn generate_bar(chord_density: &ChordDensity, context: &mut GenerateContext, params: &ChartParams) -> Vec<Chord> {
    (0..CHORDS_PER_BAR)
        .map(|i| {
            let count = chord_density.generate_chord_density(i, &mut context.rng);
            let mut randomizer = NoteRandomizer::from_context(context, params.jack_tolerance);
            let notes = randomizer.generate(count as usize, &mut context.rng);
            context.push_chord(notes.clone());
            notes
        })
        .collect()
}

pub fn generate_chart(params: &ChartParams) -> Chart {
    let mut context = GenerateContext::new(params.seed);
    let mut chart = Chart::new(params.bpm);

    for _ in 0..params.bars {
        let bar = generate_bar(&params.chord_density, &mut context, &params);
        chart.bars.push(bar);
    }

    chart
}

#[cfg(test)]
mod test {
    use super::{generate_chart, ChartParams, GenerateContext, NoteRandomizer};
    use crate::{chord::ChordDensity, generate::{CHORDS_PER_BAR, LANES}};
    use approx::assert_relative_eq;

    #[test]
    fn test_note_randomizer() {
        let mut context = GenerateContext::new(123);
        context.push_chord(vec![0, 2, 4, 6]);
        let mut randomizer = NoteRandomizer::from_context(&context, 0.0);

        assert_eq!(randomizer, NoteRandomizer::new(vec![1, 0, 1, 0, 1, 0, 1], vec![0.0; LANES]));

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
            jack_tolerance: 0.0,
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

        let params = ChartParams {
            chord_density: ChordDensity::new(vec![vec![300]]),
            bpm: 222.22,
            bars: 64,
            jack_tolerance: 1.0,
            seed: 199024,
        };

        let chart = generate_chart(&params);

        let flatten_chart: Vec<_> = chart.bars.into_iter().flatten().collect();

        for window in flatten_chart.windows(3) {
            for lane in 0..7 {
                assert!(!window.iter().all(|chord| chord.contains(&lane)));
            }
        }
    }
}
