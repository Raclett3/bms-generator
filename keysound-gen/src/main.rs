use keysound_gen::keysounds;
use keysound_gen::riff::write_riff;
use keysound_gen::synth::{sample, sinusoid, Envelope};
use std::{
    fs::{create_dir_all, File},
    path::PathBuf,
};

fn main() {
    let dirname: PathBuf = std::env::args().nth(1).expect("dirname").into();

    create_dir_all(&dirname).expect("Failed to create directory");

    let sample_rate = 44100;

    for (freq, name) in keysounds() {
        let filename = format!("s_{name}.wav");
        let mut filepath = dirname.clone();
        filepath.push(filename);

        let file = File::create(&filepath).expect("Failed to open the file");
        let samples = sample(
            sinusoid(freq),
            0.1,
            &Envelope::new(0.0, 0.02, 0.8, 0.01),
            sample_rate as f32,
            0.5,
        );
        write_riff(&file, sample_rate as u32, &samples).expect("Failed to write wave");
    }
}
