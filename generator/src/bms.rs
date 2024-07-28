use keysound_gen::{drum_names, note_names};

use crate::generate::{Chart, LANES};
use crate::keysound::{ChordRoot, ChordType};
use std::io::Write;

static LANE_MAPPING: [usize; 7] = [11, 12, 13, 14, 15, 18, 19];

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

pub fn to_bms_index(idx: usize) -> String {
    static CHARS: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    format!(
        "{}{}",
        CHARS[(idx + 1) / 36] as char,
        CHARS[(idx + 1) % 36] as char
    )
}

pub fn chart_to_bms(
    mut buf: impl Write,
    chart: &Chart,
    title: &str,
    total: f32,
) -> std::io::Result<()> {
    writeln!(buf, "#PLAYER 1")?;
    writeln!(buf, "#TITLE {title}")?;
    writeln!(buf, "#TOTAL {total:.0}")?;
    writeln!(buf, "#BPM {:.2}", chart.bpm)?;
    writeln!(buf, "#PLAYLEVEL 1")?;
    writeln!(buf, "#RANK 3")?;

    let note_names = note_names();
    let drum_names = drum_names();

    for (i, note_name) in note_names.iter().enumerate() {
        writeln!(buf, "#WAV{} s_s_{note_name}.wav", to_bms_index(i))?;
    }

    for (i, drum_name) in (note_names.len()..).zip(drum_names.iter()) {
        writeln!(buf, "#WAV{} s_dr_{drum_name}.wav", to_bms_index(i))?;
    }

    let kick = drum_names.iter().position(|x| x == "kick").unwrap();
    let snare = drum_names.iter().position(|x| x == "snare").unwrap();
    let hihat = drum_names.iter().position(|x| x == "hihat").unwrap();

    let drum_patterns: [&[_]; 3] = [
        &[Some(kick), Some(kick), Some(kick), Some(kick)],
        &[None, Some(snare), None, Some(snare)],
        &[None, Some(hihat), None, Some(hihat), None, Some(hihat), None, Some(hihat)],
    ];

    for (bar_idx, bar) in chart.bars.iter().enumerate().take(999) {
        let (chord_root, chord_type) = CHORD_PROGRESSION[bar_idx % 8];
        let keysound_chord = chord_type.to_indices(chord_root);
        let mut keysound_idx = 0;

        for drum_pattern in drum_patterns.into_iter() {
            write!(buf, "#{:03}01:", bar_idx + 1)?;
            for drum in drum_pattern {
                if let Some(drum) = drum {
                    write!(buf, "{}", to_bms_index(drum + note_names.len()))?;
                } else {
                    write!(buf, "00")?;
                }
            }
            writeln!(buf)?;
        }

        let mut lanes = vec![vec![None; bar.len()]; LANES];
        for (i, chord) in bar.iter().enumerate() {
            for lane in chord.iter().copied() {
                lanes[lane as usize][i] = Some(keysound_chord[keysound_idx]);
                keysound_idx = (keysound_idx + 1) % keysound_chord.len();
            }
        }

        for (lane_idx, lane) in lanes.into_iter().enumerate() {
            write!(buf, "#{:03}{}:", bar_idx + 1, LANE_MAPPING[lane_idx])?;

            for keysound in lane.into_iter() {
                if let Some(idx) = keysound {
                    write!(buf, "{}", to_bms_index(idx))?;
                } else {
                    write!(buf, "00")?;
                }
            }
            writeln!(buf)?;
        }
        writeln!(buf)?;
    }

    Ok(())
}
