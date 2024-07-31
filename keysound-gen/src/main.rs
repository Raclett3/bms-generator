use keysound_gen::keysounds;
use std::{fs::create_dir_all, path::PathBuf};

fn main() {
    let dirname: PathBuf = std::env::args().nth(1).expect("dirname").into();

    create_dir_all(&dirname).expect("Failed to create directory");

    let drums_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("drums");

    for keysound in keysounds() {
        keysound.write_to_dir(&drums_path, &dirname);
    }
}
