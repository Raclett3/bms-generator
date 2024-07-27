mod riff;
mod synth;

use riff::write_riff;
use std::fs::File;
use synth::{sample, sinusoid, Envelope};

fn main() {
    let filename = std::env::args().nth(1).expect("filename");

    let file = File::create(filename).expect("Failed to open the file");
    let sample_rate = 44100;
    let samples = sample(
        sinusoid(440.0),
        1.0,
        &Envelope::new(0.1, 0.1, 0.5, 0.1),
        sample_rate as f32,
        0.5,
    );
    write_riff(&file, sample_rate as u32, &samples).expect("Failed to write wave");
}
