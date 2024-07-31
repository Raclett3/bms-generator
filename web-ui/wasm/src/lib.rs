use generator::{
    bms::chart_to_bms,
    chord::ChordDensity,
    generate::{generate_chart, ChartParams, Scatter},
    keysound::{ChordKeySound, ChordRoot, ChordType},
};
use wasm_bindgen::prelude::*;

static CHORD_PROGRESSION: [(ChordRoot, ChordType); 8] = [
    (ChordRoot::D, ChordType::Major),
    (ChordRoot::A, ChordType::Major),
    (ChordRoot::B, ChordType::Minor),
    (ChordRoot::Fs, ChordType::Minor),
    (ChordRoot::G, ChordType::Major),
    (ChordRoot::D, ChordType::Major),
    (ChordRoot::G, ChordType::Major),
    (ChordRoot::A, ChordType::Major),
];

#[wasm_bindgen]
pub struct BmsParams {
    bars: usize,
    bpm: f32,
    title: String,
    jack_tolerance: f32,
    chord_density: Vec<u64>,
    scatter_strength: f32,
    scatter_decay_rate: f32,
    seed: u64,
}

#[wasm_bindgen]
impl BmsParams {
    #[wasm_bindgen(constructor)]
    pub fn new(
        bars: usize,
        bpm: f32,
        title: String,
        jack_tolerance: f32,
        chord_density: Vec<u64>,
        scatter_strength: f32,
        scatter_decay_rate: f32,
        seed: u64,
    ) -> Self {
        BmsParams {
            bars,
            bpm,
            title,
            jack_tolerance,
            chord_density,
            scatter_strength,
            scatter_decay_rate,
            seed,
        }
    }
}

#[wasm_bindgen]
pub fn generate_bms(params: &BmsParams) -> Option<String> {
    let chord_density = ChordDensity::from_power_of_two(&params.chord_density);

    let scatter = Scatter::new(
        params.scatter_strength.abs(),
        params.scatter_decay_rate,
        params.scatter_strength < 0.0,
    );

    let chart_params = ChartParams::new(
        chord_density,
        params.bpm,
        params.bars,
        params.jack_tolerance,
        scatter,
        params.seed,
    );
    let chart = generate_chart(&chart_params);

    let notes: usize = chart
        .bars
        .iter()
        .flatten()
        .map(|chord| chord.lanes.len())
        .sum();
    let total = f32::max(1000.0 - 1000000.0 / (1000.0 + notes as f32), 250.0);
    let duration = 240.0 / chart.bpm * chart.bars.len() as f32;
    let density = notes as f32 / duration;
    let genre = format!("{density:.02} notes/s");
    let artist = format!(
        "jacks: {:.01}, scatter: {:.01}, seed: {:?}",
        params.jack_tolerance, params.scatter_strength, params.seed,
    );

    let mut keysounds = ChordKeySound::new(CHORD_PROGRESSION.to_vec());

    let mut bms: Vec<u8> = Vec::new();

    if chart_to_bms(
        &mut bms,
        &chart,
        &params.title,
        &genre,
        &artist,
        total,
        &mut keysounds,
    )
    .is_ok()
    {
        unsafe { Some(String::from_utf8_unchecked(bms)) }
    } else {
        None
    }
}
