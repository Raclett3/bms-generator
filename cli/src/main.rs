use clap::{error::ErrorKind, CommandFactory, Parser};
use generator::{
    bms::chart_to_bms,
    chord::ChordDensity,
    generate::{generate_chart, ChartParams},
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

    #[arg(long)]
    seed: Option<u64>,
}

fn seed_from_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

fn parse_density(input: &str) -> Option<ChordDensity> {
    let values: Vec<u64> = input.split(',').map(|x| x.parse().ok()).collect::<Option<_>>()?;
    if values.len() != 5 {
        return None;
    }
    Some(ChordDensity::from_power_of_two(&values))
}

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

    let seed = args.seed.unwrap_or_else(seed_from_time);
    let params = ChartParams::new(chord_density, args.bpm, args.bars, seed);
    let chart = generate_chart(&params);

    let file = File::create(args.filename).expect("Failed to open file");
    let notes: usize = chart.bars.iter().flatten().map(|chord| chord.len()).sum();
    let total = f32::max(1000.0 - 1000000.0 / (1000.0 + notes as f32), 250.0);

    if chart_to_bms(&file, &chart, "test", total).is_ok() {
        println!("BMS の生成に成功しました。");
    } else {
        eprintln!("BMS の書き出しに失敗しました。");
    }
}
