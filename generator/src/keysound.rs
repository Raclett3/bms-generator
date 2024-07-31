use std::collections::HashMap;

use keysound_gen::{keysounds, KeySoundSource};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ChordRoot {
    C,
    Cs,
    D,
    Ds,
    E,
    F,
    Fs,
    G,
    Gs,
    A,
    As,
    B,
}

impl ChordRoot {
    pub fn to_index(&self) -> usize {
        match self {
            ChordRoot::C => 0,
            ChordRoot::Cs => 1,
            ChordRoot::D => 2,
            ChordRoot::Ds => 3,
            ChordRoot::E => 4,
            ChordRoot::F => 5,
            ChordRoot::Fs => 6,
            ChordRoot::G => 7,
            ChordRoot::Gs => 8,
            ChordRoot::A => 9,
            ChordRoot::As => 10,
            ChordRoot::B => 11,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ChordType {
    Major,
    Minor,
    Major7,
    Minor7,
    MajorMajor7,
    MinorMajor7,
}

impl ChordType {
    pub fn to_indices(&self, root: ChordRoot) -> Vec<usize> {
        let root_index = root.to_index();
        let mut indices = match self {
            ChordType::Major => vec![0, 4, 7, 12, 16, 19, 24],
            ChordType::Minor => vec![0, 3, 7, 12, 15, 19, 24],
            ChordType::Major7 => vec![0, 4, 7, 10, 12, 16, 19, 22, 24],
            ChordType::Minor7 => vec![0, 3, 7, 10, 12, 15, 19, 22, 24],
            ChordType::MajorMajor7 => vec![0, 4, 7, 11, 12, 16, 19, 23, 24],
            ChordType::MinorMajor7 => vec![0, 3, 7, 11, 12, 15, 19, 23, 24],
        };
        indices.iter_mut().for_each(|x| *x += root_index);
        indices
    }
}

pub trait KeySound {
    fn sources(&self) -> &[KeySoundSource];
    fn key_sound_idx(&mut self, bar_idx: usize, chord_pos: usize, chord_idx: usize) -> usize;
    fn scratch_sound_idx(&mut self, bar_idx: usize, chord_pos: usize) -> usize;
    fn bgm_sound_indices(&mut self, bar_idx: usize) -> Vec<Vec<Option<usize>>>;
}

pub struct ChordKeySound {
    chords: Vec<(ChordRoot, ChordType)>,
    sources: Vec<KeySoundSource>,
    source_indices: HashMap<String, usize>,
    notes: usize,
}

impl ChordKeySound {
    pub fn new(chords: Vec<(ChordRoot, ChordType)>) -> Self {
        let sources = keysounds();
        let source_indices = sources
            .iter()
            .map(|keysound| keysound.name().to_owned())
            .zip(0..)
            .collect();

        ChordKeySound {
            chords,
            sources,
            source_indices,
            notes: 0,
        }
    }
}

impl KeySound for ChordKeySound {
    fn sources(&self) -> &[KeySoundSource] {
        &self.sources
    }

    fn key_sound_idx(&mut self, bar_idx: usize, _chord_pos: usize, _chord_idx: usize) -> usize {
        let (chord_root, chord_type) = self.chords[bar_idx % self.chords.len()];
        let chord_indices = chord_type.to_indices(chord_root);
        let idx = chord_indices[self.notes % chord_indices.len()];
        self.notes += 1;

        idx
    }

    fn scratch_sound_idx(&mut self, _bar_idx: usize, _chord_pos: usize) -> usize {
        self.source_indices["s_dr_cymbal"]
    }

    fn bgm_sound_indices(&mut self, bar_idx: usize) -> Vec<Vec<Option<usize>>> {
        let kick = self.source_indices["s_dr_kick"];
        let snare = self.source_indices["s_dr_snare"];
        let hihat = self.source_indices["s_dr_hihat"];
        let silence = self.source_indices["s_x_silence"];

        let mut drum_patterns = vec![
            vec![Some(kick), Some(kick), Some(kick), Some(kick)],
            vec![None, Some(snare), None, Some(snare)],
            vec![
                None,
                Some(hihat),
                None,
                Some(hihat),
                None,
                Some(hihat),
                None,
                Some(hihat),
            ],
        ];

        if bar_idx == 0 {
            drum_patterns.push(vec![Some(silence)])
        }

        drum_patterns
    }
}

#[cfg(test)]
mod test {
    use crate::keysound::{ChordRoot, ChordType};

    #[test]
    fn test_chord() {
        for index in ChordType::Major.to_indices(ChordRoot::C) {
            assert!([0, 4, 7].contains(&(index % 12)));
        }

        for index in ChordType::Minor.to_indices(ChordRoot::D) {
            assert!([2, 5, 9].contains(&(index % 12)));
        }

        for index in ChordType::Major7.to_indices(ChordRoot::E) {
            assert!([4, 8, 11, 2].contains(&(index % 12)));
        }

        for index in ChordType::Minor7.to_indices(ChordRoot::F) {
            assert!([5, 8, 0, 3].contains(&(index % 12)));
        }

        for index in ChordType::MajorMajor7.to_indices(ChordRoot::G) {
            assert!([7, 11, 2, 6].contains(&(index % 12)));
        }

        for index in ChordType::MinorMajor7.to_indices(ChordRoot::A) {
            assert!([9, 0, 4, 8].contains(&(index % 12)));
        }
    }
}
