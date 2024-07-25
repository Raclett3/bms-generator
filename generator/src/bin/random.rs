use generator::rng::RNG;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    let mut rng = RNG::new_u64(seed);

    for _ in 0..100 {
        println!("{}", rng.next());
    }
}
