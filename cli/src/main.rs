use clap::{error::ErrorKind, CommandFactory, Parser};
use generator::{
    bms::{chart_dp_to_bms, chart_to_bms},
    chord::ChordDensity,
    generate::{generate_chart, generate_chart_dp, ChartParams, NotesParams, Scatter},
    keysound::{ChordKeySound, ChordRoot, ChordType},
};
use std::{
    fs::File,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// Filename of output BMS
    filename: String,

    #[arg(long, default_value_t = 150.0)]
    bpm: f32,

    #[arg(long, default_value_t = 16)]
    bars: usize,

    /// Comma-separated density of the chart (percentage of 1/1, 1/2, 1/4, 1/8, 1/16 notes, respectively)
    #[arg(long, default_value_t = format!("0,0,100,100,100"))]
    density: String,

    /// Tolerance for jacks
    /// (0 allows no jacks, 1 allows up to two consecutive notes, 0.4 allows them in 40% chance, and re-rolls otherwise.)
    #[arg(long, default_value_t = 0.0)]
    jack_tolerance: f32,

    /// Strength of scattering (With stronger scattering, jacks or basses will appear less)
    ///
    /// This can also be set to a negative value, which produces more jacks and bass rushes than usual.
    #[arg(long, default_value_t = 0.0)]
    scatter: f32,

    /// Decay rate of memory used for scattering (0.0 means super-short-term memory, and 1.0 means evarlasting memory)
    #[arg(long, default_value_t = 0.5)]
    scatter_decay_rate: f32,

    #[arg(long)]
    seed: Option<u64>,

    /// Enabling this option generates a DP chart
    #[arg(long)]
    dp: bool,

    /// Title of the BMS file
    #[arg(long, default_value_t = format!("test"))]
    title: String,
}

fn seed_from_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

fn parse_density(input: &str) -> Option<ChordDensity> {
    let values: Vec<u64> = input
        .split(',')
        .map(|x| x.parse().ok())
        .collect::<Option<_>>()?;
    if values.len() != 5 {
        return None;
    }
    Some(ChordDensity::from_power_of_two(&values))
}

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

fn main() {
    let args = Args::parse();

    let Some(chord_density) = parse_density(&args.density) else {
        let mut cmd = Args::command();
        cmd.error(
            ErrorKind::ValueValidation,
            "--density must be up to 5 comma-separated integers.",
        )
        .exit();
    };

    if !(0.0..=1.0).contains(&args.scatter_decay_rate) {
        let mut cmd = Args::command();
        cmd.error(
            ErrorKind::ValueValidation,
            "--scatter-decay-rate must be between 0 and 1.",
        )
        .exit();
    };

    let scatter = Scatter::new(
        args.scatter.abs(),
        args.scatter_decay_rate,
        args.scatter < 0.0,
    );

    let seed = args.seed.unwrap_or_else(seed_from_time);
    let chart_params = ChartParams::new(args.bpm, args.bars, seed);
    let notes_params = NotesParams::new(chord_density, args.jack_tolerance, scatter);

    let file = File::create(args.filename).expect("Failed to open file");

    let success = if args.dp {
        let chart = generate_chart_dp(&chart_params, &notes_params, &notes_params);
        let notes: usize = chart
            .bars
            .iter()
            .flatten()
            .flat_map(|chords| chords.iter().map(|chord| chord.lanes.len()))
            .sum();
        let total = f32::max(1000.0 - 1000000.0 / (1000.0 + notes as f32), 250.0);
        let duration = 240.0 / chart.bpm * chart.bars.len() as f32;
        let density = notes as f32 / duration;
        let genre = format!("密度: {density:.02} notes/s");
        let artist = format!(
            "jacks: {:.01}, scatter: {:.01}, seed: {seed:?}",
            args.jack_tolerance, args.scatter
        );

        let mut keysounds = ChordKeySound::new(CHORD_PROGRESSION.to_vec());

        chart_dp_to_bms(
            &file,
            &chart,
            &args.title,
            &genre,
            &artist,
            total,
            &mut keysounds,
        )
        .is_ok()
    } else {
        let chart = generate_chart(&chart_params, &notes_params);
        let notes: usize = chart
            .bars
            .iter()
            .flatten()
            .map(|chord| chord.lanes.len())
            .sum();
        let total = f32::max(1000.0 - 1000000.0 / (1000.0 + notes as f32), 250.0);
        let duration = 240.0 / chart.bpm * chart.bars.len() as f32;
        let density = notes as f32 / duration;
        let genre = format!("密度: {density:.02} notes/s");
        let artist = format!(
            "jacks: {:.01}, scatter: {:.01}, seed: {seed:?}",
            args.jack_tolerance, args.scatter
        );

        let mut keysounds = ChordKeySound::new(CHORD_PROGRESSION.to_vec());

        chart_to_bms(
            &file,
            &chart,
            &args.title,
            &genre,
            &artist,
            total,
            &mut keysounds,
        )
        .is_ok()
    };
    if success {
        println!("BMS の生成に成功しました。");
    } else {
        eprintln!("BMS の書き出しに失敗しました。");
    }
}
