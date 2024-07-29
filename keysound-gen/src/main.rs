use keysound_gen::riff::write_riff;
use keysound_gen::synth::{sample, triangle, Envelope};
use keysound_gen::{drum_names, keysounds};
use std::io::BufWriter;
use std::{
    fs::{copy, create_dir_all, File},
    path::PathBuf,
};

fn main() {
    let dirname: PathBuf = std::env::args().nth(1).expect("dirname").into();

    create_dir_all(&dirname).expect("Failed to create directory");

    let sample_rate = 44100;

    for (freq, name) in keysounds() {
        let filepath = dirname.join(format!("s_s_{name}.wav"));

        let file = File::create(&filepath).expect("Failed to open the file");
        let mut bufwriter = BufWriter::new(&file);
        let samples = sample(
            triangle(freq),
            0.1,
            &Envelope::new(0.0, 0.02, 0.8, 0.01),
            sample_rate as f32,
            0.5,
        );
        write_riff(&mut bufwriter, sample_rate as u32, &samples).expect("Failed to write wave");
    }

    let drums_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("drums");

    for drum_name in drum_names() {
        let path_from = drums_path.join(format!("{drum_name}.wav"));
        let path_to = dirname.join(format!("s_dr_{drum_name}.wav"));

        copy(path_from, path_to).expect("Failed to copy the drum file");
    }

    let filepath = dirname.join(format!("s_x_silence.wav"));
    let file = File::create(&filepath).expect("Failed to open the file");
    let mut bufwriter = BufWriter::new(&file);
    let samples = sample(
        |_| 0.0,
        62.3,
        &Envelope::new(0.0, 0.0, 0.0, 0.0),
        sample_rate as f32,
        0.0,
    );
    write_riff(&mut bufwriter, sample_rate as u32, &samples).expect("Failed to write wave");
}
