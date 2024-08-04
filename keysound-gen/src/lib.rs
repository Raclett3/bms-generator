use std::fs::File;
use std::io::{copy, BufWriter, Write};
use std::iter::once;
use std::path::Path;

use riff::write_riff;
use synth::{sample, triangle, Envelope};

pub mod riff;
pub mod synth;

pub const SAMPLE_RATE: u32 = 44100;

pub enum SoundSource {
    Oscillator {
        oscillator: Box<dyn Fn(f32) -> f32>,
        volume: f32,
        length: f32,
        envelope: Envelope,
    },
    Prerendered {
        filename: String,
    },
}

impl SoundSource {
    pub fn render(&self, mut buf: impl Write, file_dir: &Path) {
        match self {
            SoundSource::Oscillator {
                oscillator,
                volume,
                length,
                envelope,
            } => {
                let samples = sample(oscillator, *length, envelope, SAMPLE_RATE as f32, *volume);
                write_riff(buf, SAMPLE_RATE, &samples).expect("Failed to write wave");
            }
            SoundSource::Prerendered { filename } => {
                let file_path = file_dir.join(filename);
                let mut file = File::open(file_path).expect("Failed to open file");
                copy(&mut file, &mut buf).expect("Failed to write to file");
            }
        }
    }
}

pub struct KeySoundSource {
    name: String,
    source: SoundSource,
}

impl KeySoundSource {
    pub fn new(name: String, source: SoundSource) -> Self {
        KeySoundSource { name, source }
    }

    pub fn write_to_dir(&self, input_dir: &Path, output_dir: &Path) {
        let file_path = output_dir.join(format!("{}.wav", &self.name));
        let file = File::create(file_path).expect("Failed to open file");
        let buf = BufWriter::new(file);
        self.source.render(buf, input_dir);
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn source(&self) -> &SoundSource {
        &self.source
    }
}

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
    ["kick", "snare", "hihat", "cymbal"]
        .into_iter()
        .map(|x| x.to_owned())
        .collect()
}

fn synth_keysounds() -> impl Iterator<Item = KeySoundSource> {
    let envelope = Envelope::new(0.0, 0.02, 0.8, 0.01);
    note_names()
        .into_iter()
        .enumerate()
        .map(move |(i, note_name)| {
            KeySoundSource::new(
                format!("s_s_{note_name}"),
                SoundSource::Oscillator {
                    oscillator: Box::new(triangle(note_to_freq(i as i32 + 3 - 24))),
                    volume: 0.5,
                    length: 0.1,
                    envelope: envelope.clone(),
                },
            )
        })
}

fn drum_keysounds<'a>() -> impl Iterator<Item = KeySoundSource> + 'a {
    drum_names().into_iter().map(move |drum_name| {
        KeySoundSource::new(
            format!("s_dr_{drum_name}"),
            SoundSource::Prerendered {
                filename: format!("{drum_name}.wav"),
            },
        )
    })
}

fn silence_keysound() -> impl Iterator<Item = KeySoundSource> {
    once(KeySoundSource::new(
        "s_x_silence".to_owned(),
        SoundSource::Oscillator {
            oscillator: Box::new(|_| 0.0),
            volume: 0.0,
            length: 62.3,
            envelope: Envelope::new(0.0, 0.0, 0.0, 0.0),
        },
    ))
}

pub fn keysounds() -> Vec<KeySoundSource> {
    synth_keysounds()
        .chain(drum_keysounds())
        .chain(silence_keysound())
        .collect()
}
