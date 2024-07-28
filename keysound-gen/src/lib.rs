pub mod riff;
pub mod synth;

fn note_to_freq(note: i32) -> f32 {
    440.0 * 2f32.powf(note as f32 / 12.0)
}

pub fn note_names() -> Vec<String> {
    (3..=6)
        .flat_map(|octave| {
            [
                "C", "Cs", "D", "Ds", "E", "F", "Fs", "G", "Gs", "A", "As", "B",
            ]
            .iter()
            .cloned()
            .map(move |name| format!("{name}{octave}"))
        })
        .collect()
}

pub fn drum_names() -> Vec<String> {
    ["kick", "snare", "hihat"]
        .into_iter()
        .map(|x| x.to_owned())
        .collect()
}

pub fn keysounds() -> Vec<(f32, String)> {
    note_names()
        .into_iter()
        .enumerate()
        .map(|(i, note_name)| (note_to_freq(i as i32 + 3 - 24), note_name))
        .collect()
}
