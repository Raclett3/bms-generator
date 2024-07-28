use crate::chord::ChordDensity;
use crate::rng::RNG;

pub const CHORDS_PER_BAR: usize = 16;
pub const LANES: usize = 7;

pub struct Scatter {
    strength: f32,
    decay: f32,
    inverted: bool,
}

impl Scatter {
    pub fn new(strength: f32, decay: f32, inverted: bool) -> Self {
        Scatter {
            strength,
            decay,
            inverted,
        }
    }
}

pub struct ChartParams {
    chord_density: ChordDensity,
    bpm: f32,
    bars: usize,
    jack_tolerance: f32,
    scatter: Scatter,
    seed: u64,
}

impl ChartParams {
    pub fn new(
        chord_density: ChordDensity,
        bpm: f32,
        bars: usize,
        jack_tolerance: f32,
        scatter: Scatter,
        seed: u64,
    ) -> Self {
        ChartParams {
            chord_density,
            bpm,
            bars,
            jack_tolerance,
            scatter,
            seed,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Chord {
    pub lanes: Vec<u8>,
    pub scratch: bool,
}

impl Chord {
    fn new(lanes: Vec<u8>, scratch: bool) -> Self {
        Chord { lanes, scratch }
    }

    fn contains(&self, lane: u8) -> bool {
        self.lanes.contains(&lane)
    }
}

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
    weight: Vec<f32>,
    priority: Vec<usize>,
    reroll_chance: Vec<f32>,
}

impl NoteRandomizer {
    fn new(priority: Vec<usize>, reroll_chance: Vec<f32>, weight: Vec<f32>) -> Self {
        assert!(priority.len() == LANES);
        assert!(reroll_chance.len() == LANES);
        NoteRandomizer {
            weight,
            priority,
            reroll_chance,
        }
    }

    fn from_context(context: &GenerateContext) -> Self {
        let jack_tolerance = context.params.jack_tolerance;
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
            context.bias_to_weight(),
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

                let weight_sum: f32 = lanes.iter().map(|&lane| self.weight[lane as usize]).sum();
                let rng_next = rng.next_f32() * weight_sum;

                let (idx, _) = lanes
                    .iter()
                    .enumerate()
                    .scan(0.0, |weight_acc, (i, &lane)| {
                        *weight_acc += self.weight[lane as usize];
                        Some((i, *weight_acc))
                    })
                    .find(|&(_, weight_acc)| rng_next < weight_acc)
                    .unwrap_or((lanes.len() - 1, 0.0));
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

struct GenerateContext<'a> {
    generated_chords: Vec<Chord>,
    rng: RNG,
    ongoing_jacks: Vec<usize>,
    bias: Vec<f32>,
    params: &'a ChartParams,
}

impl<'a> GenerateContext<'a> {
    fn new(seed: u64, params: &'a ChartParams) -> Self {
        GenerateContext {
            generated_chords: Vec::new(),
            rng: RNG::new_u64(seed),
            ongoing_jacks: vec![0; LANES],
            bias: vec![0.0; LANES],
            params,
        }
    }

    fn push_chord(&mut self, chord: Chord) {
        for (i, bias) in self.bias.iter_mut().enumerate() {
            *bias = *bias * self.params.scatter.decay;

            if chord.contains(i as u8) {
                *bias += self.params.scatter.strength;
            }
        }

        for (i, jacks) in self.ongoing_jacks.iter_mut().enumerate() {
            if chord.contains(i as u8) {
                *jacks += 1;
            } else {
                *jacks = 0;
            }
        }
        self.generated_chords.push(chord);
    }

    fn bias_to_weight(&self) -> Vec<f32> {
        let min_bias = self.bias.iter().copied().fold(f32::MAX, f32::min);
        self.bias
            .iter()
            .map(|&bias| {
                if self.params.scatter.inverted {
                    1.0 + bias
                } else {
                    2.0f32.powf(-(bias - min_bias))
                }
            })
            .collect()
    }
}

fn generate_bar(
    bar_idx: usize,
    chord_density: &ChordDensity,
    context: &mut GenerateContext,
) -> Vec<Chord> {
    (0..CHORDS_PER_BAR)
        .map(|i| {
            let count = chord_density.generate_chord_density(i, &mut context.rng);
            let mut randomizer = NoteRandomizer::from_context(context);
            let notes = randomizer.generate(count as usize, &mut context.rng);
            let chord = Chord::new(notes, bar_idx % 8 == 0 && i == 0);
            context.push_chord(chord.clone());
            chord
        })
        .collect()
}

pub fn generate_chart(params: &ChartParams) -> Chart {
    let mut context = GenerateContext::new(params.seed, params);
    let mut chart = Chart::new(params.bpm);

    for bar_idx in 0..params.bars {
        let bar = generate_bar(bar_idx, &params.chord_density, &mut context);
        chart.bars.push(bar);
    }

    chart
}

#[cfg(test)]
mod test {
    use super::{generate_chart, ChartParams, GenerateContext, NoteRandomizer};
    use crate::{
        chord::ChordDensity,
        generate::{Chord, Scatter, CHORDS_PER_BAR, LANES},
    };
    use approx::assert_relative_eq;

    #[test]
    fn test_bias() {
        let params = ChartParams {
            chord_density: ChordDensity::new(vec![vec![300]]),
            bpm: 222.22,
            bars: 64,
            jack_tolerance: 0.0,
            scatter: Scatter::new(1.0, 0.5, false),
            seed: 199024,
        };

        let mut context = GenerateContext::new(123, &params);
        context.push_chord(Chord::new(vec![0, 2, 4, 6], false));
        assert_relative_eq!(
            context.bias.as_slice(),
            [1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0].as_slice()
        );
        context.push_chord(Chord::new(vec![1, 2, 3, 4, 5], false));
        assert_relative_eq!(
            context.bias.as_slice(),
            [0.5, 1.0, 1.5, 1.0, 1.5, 1.0, 0.5].as_slice()
        );
        context.push_chord(Chord::new(vec![0, 6], false));
        assert_relative_eq!(
            context.bias.as_slice(),
            [1.25, 0.5, 0.75, 0.5, 0.75, 0.5, 1.25].as_slice()
        );
        context.push_chord(Chord::new(vec![3], false));
        assert_relative_eq!(
            context.bias.as_slice(),
            [0.625, 0.25, 0.375, 1.25, 0.375, 0.25, 0.625].as_slice()
        );
    }

    #[test]
    fn test_bias_to_weight() {
        let mut params = ChartParams {
            chord_density: ChordDensity::new(vec![vec![300]]),
            bpm: 255.0,
            bars: 128,
            jack_tolerance: 0.0,
            scatter: Scatter::new(10.0, 1.0, false),
            seed: 199024,
        };

        {
            let mut context = GenerateContext::new(123, &params);
            context.bias = vec![0.0, 1.0, 2.0, 3.0, 2.0, 1.0, 0.0];
            assert_relative_eq!(
                context.bias_to_weight().as_slice(),
                [1.0, 0.5, 0.25, 0.125, 0.25, 0.5, 1.0].as_slice(),
            );
        }

        params.scatter = Scatter::new(10.0, 1.0, true);

        {
            let mut context = GenerateContext::new(123, &params);
            context.bias = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
            assert_eq!(
                context.bias_to_weight().as_slice(),
                [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0].as_slice()
            );
        }
    }

    #[test]
    fn test_note_randomizer() {
        let params = ChartParams {
            chord_density: ChordDensity::new(vec![vec![300]]),
            bpm: 222.22,
            bars: 64,
            jack_tolerance: 0.0,
            scatter: Scatter::new(0.0, 0.0, false),
            seed: 199024,
        };

        let mut context = GenerateContext::new(123, &params);
        context.push_chord(Chord::new(vec![0, 2, 4, 6], false));
        let mut randomizer = NoteRandomizer::from_context(&context);

        assert_eq!(
            randomizer,
            NoteRandomizer::new(
                vec![1, 0, 1, 0, 1, 0, 1],
                vec![0.0; LANES],
                vec![1.0; LANES]
            )
        );

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
            scatter: Scatter::new(0.0, 0.0, false),
            seed: 199024,
        };

        let chart = generate_chart(&params);

        assert_relative_eq!(params.bpm, chart.bpm);

        for bar in chart.bars.iter() {
            assert_eq!(bar.len(), CHORDS_PER_BAR);
        }

        let flatten_chart: Vec<_> = chart.bars.into_iter().flatten().collect();

        for window in flatten_chart.windows(2) {
            assert!(window[0].lanes.iter().all(|&x| !window[1].contains(x)));
        }

        let params = ChartParams {
            chord_density: ChordDensity::new(vec![vec![300]]),
            bpm: 222.22,
            bars: 64,
            jack_tolerance: 1.0,
            scatter: Scatter::new(0.0, 0.0, false),
            seed: 199024,
        };

        let chart = generate_chart(&params);

        let flatten_chart: Vec<_> = chart.bars.into_iter().flatten().collect();

        for window in flatten_chart.windows(3) {
            for lane in 0..7 {
                assert!(!window.iter().all(|chord| chord.contains(lane)));
            }
        }
    }
}
