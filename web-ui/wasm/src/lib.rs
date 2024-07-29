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
pub fn generate_bms(
    bars: usize,
    bpm: f32,
    title: &str,
    jack_tolerance: f32,
    chord_density: &[u64],
    scatter_strength: f32,
    scatter_decay_rate: f32,
    seed: u64,
) -> Option<String> {
    let chord_density = ChordDensity::from_power_of_two(chord_density);

    let scatter = Scatter::new(
        scatter_strength.abs(),
        scatter_decay_rate,
        scatter_strength < 0.0,
    );

    let params = ChartParams::new(chord_density, bpm, bars, jack_tolerance, scatter, seed);
    let chart = generate_chart(&params);

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
    let artist =
        format!("jacks: {jack_tolerance:.01}, scatter: {scatter_strength:.01}, seed: {seed:?}");

    let mut keysounds = ChordKeySound::new(CHORD_PROGRESSION.to_vec());

    let mut bms: Vec<u8> = Vec::new();

    if chart_to_bms(
        &mut bms,
        &chart,
        title,
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
