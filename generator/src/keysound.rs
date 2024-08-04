mod chord;
mod sequence;

use keysound_gen::KeySoundSource;

pub use chord::{ChordKeySound, ChordRoot, ChordType};
pub use sequence::{Note, Oscillator, SequenceKeySound};

pub trait KeySound {
    fn sources(&self) -> &[KeySoundSource];
    fn key_sound_idx(&mut self, bar_idx: usize, chord_pos: usize, chord_idx: usize) -> usize;
    fn scratch_sound_idx(&mut self, bar_idx: usize, chord_pos: usize) -> usize;
    fn bgm_sound_indices(&mut self, bar_idx: usize) -> Vec<Vec<Option<usize>>>;
}
