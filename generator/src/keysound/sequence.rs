use keysound_gen::{synth::Envelope, KeySoundSource, SoundSource};
use std::collections::HashMap;
use std::f32::{consts::PI, NAN};
use std::hash::Hash;

use super::KeySound;
use crate::generate::CHORDS_PER_BAR;

#[derive(Clone, Copy, PartialEq, Debug, Hash)]
pub enum Oscillator {
    Sine,
    Triangle,
    Square,
    Saw,
}

fn sinusoid(freq: f32) -> impl Fn(f32) -> f32 {
    move |x: f32| f32::sin(x * freq * 2.0 * PI)
}

fn triangle(freq: f32) -> impl Fn(f32) -> f32 {
    move |x: f32| f32::asin(f32::sin(x * freq * 2.0 * PI))
}

fn square(freq: f32) -> impl Fn(f32) -> f32 {
    move |x: f32| if x * freq % 1.0 < 0.5 { 1.0 } else { -1.0 }
}

fn saw(freq: f32) -> impl Fn(f32) -> f32 {
    move |x: f32| (-x * freq + 0.5) % 1.0 * 2.0
}

impl Oscillator {
    fn into_fn(self, freq: f32) -> Box<dyn Fn(f32) -> f32> {
        match self {
            Oscillator::Sine => Box::new(sinusoid(freq)),
            Oscillator::Triangle => Box::new(triangle(freq)),
            Oscillator::Square => Box::new(square(freq)),
            Oscillator::Saw => Box::new(saw(freq)),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Note {
    osc: Oscillator,
    note: i32,     // 0 = A4 = 440Hz
    length: usize, // by sixteenth notes
    volume: f32,
}

impl Eq for Note {}

impl Hash for Note {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.osc.hash(state);
        state.write_i32(self.note);
        state.write_usize(self.length);
        state.write_u32(self.volume.to_bits());
    }
}

impl Note {
    pub fn new(osc: Oscillator, note: i32, length: usize, volume: f32) -> Note {
        assert!(volume != NAN);
        Note {
            osc,
            note,
            length,
            volume,
        }
    }

    fn to_source(&self, volume: f32, bpm: f32, envelope: Envelope) -> SoundSource {
        let freq = self.freq();
        let length = 15.0 / bpm * self.length as f32;

        SoundSource::Oscillator {
            oscillator: self.osc.into_fn(freq),
            volume,
            length,
            envelope,
        }
    }

    fn freq(&self) -> f32 {
        440.0 * 2f32.powf(self.note as f32 / 12.0)
    }
}

pub struct SequenceKeySound {
    keysound_indices: Vec<Vec<usize>>,
    keysounds: Vec<KeySoundSource>,
    used_keysounds: Vec<Vec<usize>>,
}

fn silence_keysound() -> KeySoundSource {
    KeySoundSource::new(
        "key_silence".to_owned(),
        SoundSource::Oscillator {
            oscillator: Box::new(|_| 0.0),
            volume: 0.0,
            length: 62.3,
            envelope: Envelope::new(0.0, 0.0, 0.0, 0.0),
        },
    )
}

impl SequenceKeySound {
    pub fn new(notes: &[impl AsRef<[Note]>], bpm: f32, envelope: &Envelope) -> Self {
        let mut keysound_idx_map: HashMap<&Note, usize> = HashMap::new();
        let mut keysounds = vec![silence_keysound()];
        let keysound_indices = notes
            .iter()
            .map(|chord| {
                chord
                    .as_ref()
                    .iter()
                    .map(|note| {
                        *keysound_idx_map.entry(note).or_insert_with(|| {
                            let new_keysound_idx = keysounds.len();
                            let keysound = KeySoundSource::new(
                                format!("key_{new_keysound_idx:03}"),
                                note.to_source(0.5, bpm, envelope.clone()),
                            );
                            keysounds.push(keysound);
                            new_keysound_idx
                        })
                    })
                    .collect()
            })
            .collect();

        SequenceKeySound {
            keysound_indices,
            keysounds,
            used_keysounds: Vec::new(),
        }
    }

    pub fn keysounds(&self) -> &[KeySoundSource] {
        &self.keysounds
    }
}

impl KeySound for SequenceKeySound {
    fn sources(&self) -> &[KeySoundSource] {
        &self.keysounds
    }

    fn key_sound_idx(&mut self, bar_idx: usize, chord_pos: usize, chord_idx: usize) -> usize {
        if self.used_keysounds.len() < bar_idx + 1 {
            self.used_keysounds
                .resize(bar_idx + 1, vec![0; CHORDS_PER_BAR]);
        }
        self.used_keysounds[bar_idx][chord_pos] += 1;
        self.keysound_indices[(bar_idx * CHORDS_PER_BAR + chord_pos) % self.keysound_indices.len()]
            .get(chord_idx)
            .copied()
            .unwrap_or(self.keysounds.len()) // silence
    }

    fn scratch_sound_idx(&mut self, _bar_idx: usize, _chord_pos: usize) -> usize {
        self.keysounds.len()
    }

    fn bgm_sound_indices(&mut self, bar_idx: usize) -> Vec<Vec<Option<usize>>> {
        let silence = 0;

        let unused_keysounds: Vec<_> = self
            .keysound_indices
            .iter()
            .skip((bar_idx * CHORDS_PER_BAR) % self.keysound_indices.len())
            .take(CHORDS_PER_BAR)
            .zip(self.used_keysounds[bar_idx].iter().copied())
            .map(|(keysounds, used_keysounds)| {
                &keysounds[usize::min(used_keysounds, keysounds.len())..]
            })
            .collect();
        let bgm_channels = unused_keysounds.iter().map(|x| x.len()).max().unwrap();
        let mut bgm_channels: Vec<_> = (0..bgm_channels)
            .map(|i| {
                (0..CHORDS_PER_BAR)
                    .map(|j| unused_keysounds[j].get(i).copied())
                    .collect()
            })
            .collect();

        if bar_idx == 0 {
            bgm_channels.push(vec![Some(silence)])
        }

        bgm_channels
    }
}
