use generator::{
    bms::chart_to_bms,
    chord::ChordDensity,
    generate::{generate_chart, ChartParams, LANES},
};
use std::time::{SystemTime, UNIX_EPOCH};

fn print_bar(bar: &[Vec<u8>]) {
    for chord in bar.iter().rev() {
        for i in 0..LANES as u8 {
            if !chord.contains(&i) {
                print!("  ");
                continue;
            }

            if i % 2 == 0 {
                print!("\x1b[37m");
            } else {
                print!("\x1b[34m");
            }

            print!("--");
        }

        println!("\x1b[0m");
    }
}

fn main() {
    let chord_density = ChordDensity::from_power_of_two(&[0, 0, 100, 100, 100]);
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    let params = ChartParams::new(chord_density, 222.22, 16, seed);
    let chart = generate_chart(&params);

    let is_bms_format = std::env::args().any(|arg| arg == "--bms");

    if is_bms_format {
        let mut bms = String::new();
        let notes: usize = chart.bars.iter().flatten().map(|chord| chord.len()).sum();
        let total = 200.0 + (notes as f32) * 0.3;
        if chart_to_bms(&mut bms, &chart, "test", total).is_ok() {
            println!("{bms}");
        } else {
            eprintln!("An error has occured generating BMS.");
        }
    } else {
        for bar in chart.bars.iter().rev() {
            print_bar(bar);
        }
    }
}
