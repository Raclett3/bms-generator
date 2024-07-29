use keysound_gen::{drum_names, note_names};

use crate::generate::{Chart, LANES};
use crate::keysound::KeySound;
use std::io::Write;

static LANE_MAPPING: [usize; 7] = [11, 12, 13, 14, 15, 18, 19];

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
    genre: &str,
    artist: &str,
    total: f32,
    keysounds: &mut impl KeySound,
) -> std::io::Result<()> {
    writeln!(buf, "#PLAYER 1")?;
    writeln!(buf, "#TITLE {title}")?;
    writeln!(buf, "#GENRE {genre}")?;
    writeln!(buf, "#ARTIST {artist}")?;
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
    writeln!(buf, "#WAVXX s_x_silence.wav")?;

    writeln!(buf, "#00101:XX")?;
    for (bar_idx, bar) in chart.bars.iter().enumerate().take(999) {
        for bgm_lane in keysounds.bgm_sound_indices(bar_idx).into_iter() {
            write!(buf, "#{:03}01:", bar_idx + 1)?;
            for sound_idx in bgm_lane {
                if let Some(sound_idx) = sound_idx {
                    write!(buf, "{}", to_bms_index(sound_idx))?;
                } else {
                    write!(buf, "00")?;
                }
            }
            writeln!(buf)?;
        }

        let mut lanes = vec![vec![None; bar.len()]; LANES];
        for (i, chord) in bar.iter().enumerate() {
            for (j, lane) in chord.lanes.iter().copied().enumerate() {
                lanes[lane as usize][i] = Some(keysounds.key_sound_idx(bar_idx, i, j));
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

        write!(buf, "#{:03}16:", bar_idx + 1)?;
        for (i, chord) in bar.iter().enumerate() {
            if chord.scratch {
                write!(
                    buf,
                    "{}",
                    to_bms_index(keysounds.scratch_sound_idx(bar_idx, i))
                )?;
            } else {
                write!(buf, "00")?;
            }
        }

        writeln!(buf)?;
    }

    Ok(())
}
