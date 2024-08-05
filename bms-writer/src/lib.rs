use std::{collections::BTreeMap, io::Write};

use encoding_rs::SHIFT_JIS;

pub const KEYBOARD_CHANNELS: [u8; 7] = [11, 12, 13, 14, 15, 18, 19];
pub const SCRATCH_CHANNEL: u8 = 16;
pub const BGM_CHANNEL: u8 = 1;

type Bar = Vec<Vec<Option<usize>>>;

fn to_bms_index(idx: usize) -> String {
    static CHARS: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    format!(
        "{}{}",
        CHARS[(idx + 1) / 36] as char,
        CHARS[(idx + 1) % 36] as char
    )
}

fn to_shift_jis(s: &str) -> Vec<u8> {
    SHIFT_JIS.encode(s).0.to_vec()
}

#[derive(Clone, Debug)]
struct Channel {
    bars: Vec<Bar>,
}

impl Channel {
    fn new() -> Self {
        Channel { bars: Vec::new() }
    }

    fn push_to_bar(&mut self, bar_idx: usize, bar: Vec<Option<usize>>) {
        if self.bars.len() < bar_idx + 1 {
            self.bars.resize_with(bar_idx + 1, Vec::new);
        }

        self.bars[bar_idx].push(bar);
    }

    fn write(&self, channel_idx: u8, w: &mut impl Write) -> std::io::Result<()> {
        for (bar_idx, bar) in self.bars.iter().enumerate() {
            for single_bar in bar.iter() {
                write!(w, "#{bar_idx:03}{channel_idx:02}:", bar_idx = bar_idx + 1)?;
                for &sound_idx in single_bar.iter() {
                    if let Some(sound_idx) = sound_idx {
                        write!(w, "{}", to_bms_index(sound_idx))?;
                    } else {
                        write!(w, "00")?;
                    }
                }
                writeln!(w)?;
            }
        }

        Ok(())
    }
}

#[derive(Clone, Default, Debug)]
pub struct BmsWriter {
    channels: BTreeMap<u8, Channel>,
    keysounds: BTreeMap<usize, String>,
    bpm: f32,
    total: f32,
    title: String,
    genre: String,
    artist: String,
}

impl BmsWriter {
    pub fn new() -> Self {
        BmsWriter {
            bpm: 120.0,
            total: 300.0,
            ..Default::default()
        }
    }

    pub fn set_artist(&mut self, artist: &str) {
        self.artist = artist.to_owned();
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_owned();
    }

    pub fn set_genre(&mut self, genre: &str) {
        self.genre = genre.to_owned();
    }

    pub fn set_bpm(&mut self, bpm: f32) {
        self.bpm = bpm;
    }

    pub fn set_total(&mut self, total: f32) {
        self.total = total;
    }

    pub fn push_channel(&mut self, bar_idx: usize, channel_id: u8, bar: Vec<Option<usize>>) {
        let channel = self.channels.entry(channel_id).or_insert_with(Channel::new);
        channel.push_to_bar(bar_idx, bar);
    }

    pub fn set_keysound(&mut self, idx: usize, name: &str) {
        self.keysounds.insert(idx, name.to_owned());
    }

    pub fn write(&self, w: &mut impl Write) -> std::io::Result<()> {
        writeln!(w, "#PLAYER 1")?;
        w.write(&to_shift_jis(&format!("#TITLE {}\n", self.title)))?;
        w.write(&to_shift_jis(&format!("#GENRE {}\n", self.genre)))?;
        w.write(&to_shift_jis(&format!("#ARTIST {}\n", self.artist)))?;
        writeln!(w, "#TOTAL {:.0}", self.total)?;
        writeln!(w, "#BPM {:.2}", self.bpm)?;
        writeln!(w, "#PLAYLEVEL 1")?;
        writeln!(w, "#RANK 3")?;

        for (&idx, name) in self.keysounds.iter() {
            writeln!(w, "#WAV{} {}.wav", to_bms_index(idx), name)?;
        }

        for (&channel_idx, channel) in self.channels.iter() {
            channel.write(channel_idx, w)?;
        }

        Ok(())
    }
}
