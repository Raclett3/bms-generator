use bms_writer::BmsWriter;

use crate::generate::{Chart, LANES};
use crate::keysound::KeySound;
use std::io::Write;

static LANE_MAPPING: [u8; 8] = [11, 12, 13, 14, 15, 18, 19, 16];

pub fn chart_to_bms(
    mut buf: impl Write,
    chart: &Chart,
    title: &str,
    genre: &str,
    artist: &str,
    total: f32,
    keysounds: &mut impl KeySound,
) -> std::io::Result<()> {
    let mut bms = BmsWriter::new();

    bms.set_title(title);
    bms.set_genre(genre);
    bms.set_artist(artist);
    bms.set_bpm(chart.bpm);
    bms.set_total(total);

    for (i, source) in keysounds.sources().iter().enumerate() {
        bms.set_keysound(i, source.name());
    }

    for (bar_idx, bar) in chart.bars.iter().enumerate().take(999) {
        let mut lanes = vec![vec![None; bar.len()]; LANES];
        for (i, chord) in bar.iter().enumerate() {
            for (j, lane) in chord.lanes.iter().copied().enumerate() {
                lanes[lane as usize][i] = Some(keysounds.key_sound_idx(bar_idx, i, j));
            }
        }

        for (lane_idx, lane) in lanes.into_iter().enumerate() {
            bms.push_channel(bar_idx, LANE_MAPPING[lane_idx], lane);
        }

        let scratch: Vec<_> = bar
            .iter()
            .enumerate()
            .map(|(i, chord)| {
                chord
                    .scratch
                    .then(|| keysounds.scratch_sound_idx(bar_idx, i))
            })
            .collect();

        bms.push_channel(bar_idx, LANE_MAPPING[7], scratch);

        for bgm_lane in keysounds.bgm_sound_indices(bar_idx).into_iter() {
            bms.push_channel(bar_idx, 1, bgm_lane);
        }
    }

    bms.write(&mut buf)
}
