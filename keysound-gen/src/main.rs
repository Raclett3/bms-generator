use std::{f32::consts::PI, fs::File};

use riff::write_riff;

mod riff;

fn f32_sample_to_i16(sample: f32) -> i16 {
    unsafe {
        ((sample) * 32767.0)
            .clamp(-32767.0, 32767.0)
            .to_int_unchecked()
    }
}

fn sinusoid(frequency: f32, sample_rate: f32, samples: usize) -> Vec<[i16; 2]> {
    (0..samples)
        .map(|i| {
            let sample = f32::sin(i as f32 / sample_rate * frequency * 2.0 * PI);
            let sample_i16 = f32_sample_to_i16(sample);
            [sample_i16, sample_i16]
        })
        .collect()
}

fn main() {
    let filename = std::env::args().nth(1).expect("filename");

    let file = File::create(filename).expect("Failed to open the file");
    let sample_rate = 44100;
    let samples = sinusoid(440.0, sample_rate as f32, sample_rate);
    write_riff(&file, sample_rate as u32, &samples).expect("Failed to write wave");
}
