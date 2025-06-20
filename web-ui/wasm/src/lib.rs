use base64::prelude::*;
use generator::{
    bms::{chart_dp_to_bms, chart_to_bms},
    chord::ChordDensity,
    generate::{generate_chart, generate_chart_dp, ChartParams, NotesParams, Scatter},
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
pub struct JsChartParams {
    bars: usize,
    bpm: f32,
    title: String,
    seed: u64,
}

#[wasm_bindgen]
impl JsChartParams {
    #[wasm_bindgen(constructor)]
    pub fn new(bars: usize, bpm: f32, title: String, seed: u64) -> Self {
        JsChartParams {
            bars,
            bpm,
            title,
            seed,
        }
    }

    fn to_chart_params(&self) -> ChartParams {
        ChartParams::new(self.bpm, self.bars, self.seed)
    }
}

#[wasm_bindgen]
pub struct JsNotesParams {
    jack_tolerance: f32,
    chord_density: Vec<u64>,
    scatter_strength: f32,
    scatter_decay_rate: f32,
}

#[wasm_bindgen]
impl JsNotesParams {
    #[wasm_bindgen(constructor)]
    pub fn new(
        jack_tolerance: f32,
        chord_density: Vec<u64>,
        scatter_strength: f32,
        scatter_decay_rate: f32,
    ) -> Self {
        JsNotesParams {
            jack_tolerance,
            chord_density,
            scatter_strength,
            scatter_decay_rate,
        }
    }

    fn to_notes_params(&self) -> NotesParams {
        let chord_density = ChordDensity::from_power_of_two(&self.chord_density);

        let scatter = Scatter::new(
            self.scatter_strength.abs(),
            self.scatter_decay_rate,
            self.scatter_strength < 0.0,
        );

        NotesParams::new(chord_density, self.jack_tolerance, scatter)
    }
}

#[wasm_bindgen]
pub fn data_uri(content: &[u8], mime: &str) -> String {
    format!("data:{mime};base64,{}", BASE64_STANDARD.encode(content))
}

#[wasm_bindgen]
pub fn generate_bms(
    js_chart_params: JsChartParams,
    js_notes_params: Vec<JsNotesParams>,
) -> Option<Vec<u8>> {
    assert!((1..=2).contains(&js_notes_params.len()));

    let chart_params = js_chart_params.to_chart_params();
    if js_notes_params.len() == 1 {
        let notes_params = js_notes_params[0].to_notes_params();
        let chart = generate_chart(&chart_params, &notes_params);

        let notes: usize = chart
            .bars
            .iter()
            .flatten()
            .map(|chord| chord.lanes.len())
            .sum();
        let total = f32::max(1000.0 - 1000000.0 / (1000.0 + notes as f32), 250.0);
        let duration = 240.0 / chart.bpm * chart.bars.len() as f32;
        let density = notes as f32 / duration;
        let genre = format!("密度: {density:.02} notes/s");
        let artist = format!(
            "jacks: {:.01}, scatter: {:.01}, seed: {:?}",
            js_notes_params[0].jack_tolerance,
            js_notes_params[0].scatter_strength,
            js_chart_params.seed,
        );

        let mut keysounds = ChordKeySound::new(CHORD_PROGRESSION.to_vec());

        let mut bms: Vec<u8> = Vec::new();

        if chart_to_bms(
            &mut bms,
            &chart,
            &js_chart_params.title,
            &genre,
            &artist,
            total,
            &mut keysounds,
        )
        .is_ok()
        {
            Some(bms)
        } else {
            None
        }
    } else {
        let notes_params_left = js_notes_params[0].to_notes_params();
        let notes_params_right = js_notes_params[1].to_notes_params();
        let chart = generate_chart_dp(&chart_params, &notes_params_left, &notes_params_right);

        let notes: usize = chart
            .bars
            .iter()
            .flatten()
            .flat_map(|chords| chords.iter().map(|chord| chord.lanes.len()))
            .sum();
        let total = f32::max(1000.0 - 1000000.0 / (1000.0 + notes as f32), 250.0);
        let duration = 240.0 / chart.bpm * chart.bars.len() as f32;
        let density = notes as f32 / duration;
        let genre = format!("密度: {density:.02} notes/s");
        let artist = format!(
            "jacks: {:.01}, scatter: {:.01}, seed: {:?}",
            js_notes_params[0].jack_tolerance,
            js_notes_params[0].scatter_strength,
            js_chart_params.seed,
        );

        let mut keysounds = ChordKeySound::new(CHORD_PROGRESSION.to_vec());

        let mut bms: Vec<u8> = Vec::new();

        if chart_dp_to_bms(
            &mut bms,
            &chart,
            &js_chart_params.title,
            &genre,
            &artist,
            total,
            &mut keysounds,
        )
        .is_ok()
        {
            Some(bms)
        } else {
            None
        }
    }
}
