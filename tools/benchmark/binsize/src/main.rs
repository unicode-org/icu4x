// variables1.rs
// Make me compile! Execute the command `rustlings hint variables1` if you want a hint :)

// About this `I AM NOT DONE` thing:
// We sometimes encourage you to keep trying things on a given exercise,
// even after you already figured it out. If you got everything working and
// feel ready for the next exercise, remove the `I AM NOT DONE` comment below.

// I AM NOT DONE

use std::fs;

fn wasm_filesize(dir: &str) -> Result<u64, std::io::Error> {
    let paths = fs::read_dir(dir).unwrap();
    for path in paths {
        let p = path.unwrap().path();
        if let Some(suffix) = p.extension() {
            if suffix == "wasm" {
                println!(
                    "WASM file: {:?}, size: {}",
                    p.file_stem().unwrap(),
                    p.metadata()?.len()
                );
            }
        }
    }
    Ok(1)
}

fn main() {
    wasm_filesize("wasmpkg");
}
