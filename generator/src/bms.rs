use crate::generate::{Chart, LANES};
use std::fmt::Write;

static LANE_MAPPING: [usize; 7] = [11, 12, 13, 14, 15, 18, 19];

pub fn chart_to_bms(
    buf: &mut impl Write,
    chart: &Chart,
    title: &str,
    total: f32,
) -> std::fmt::Result {
    writeln!(buf, "#PLAYER 1")?;
    writeln!(buf, "#TITLE {title}")?;
    writeln!(buf, "#TOTAL {total:.0}")?;
    writeln!(buf, "#BPM {:.2}", chart.bpm)?;
    writeln!(buf, "#PLAYLEVEL 1")?;
    writeln!(buf, "#RANK 3")?;

    for (bar_idx, bar) in chart.bars.iter().enumerate().take(999) {
        let mut lanes = vec![vec![false; bar.len()]; LANES];
        for (i, chord) in bar.iter().enumerate() {
            for lane in chord.iter().copied() {
                lanes[lane as usize][i] = true;
            }
        }

        for (lane_idx, lane) in lanes.into_iter().enumerate() {
            write!(buf, "#{:03}{}:", bar_idx + 1, LANE_MAPPING[lane_idx])?;

            for note in lane.into_iter() {
                if note {
                    write!(buf, "01")?;
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
