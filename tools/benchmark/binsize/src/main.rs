// variables1.rs
// Make me compile! Execute the command `rustlings hint variables1` if you want a hint :)

use std::fs;
use std::string::String;

fn wasm_filesize(dir: &str) -> Result<u64, std::io::Error> {
    let paths = fs::read_dir(dir).unwrap();
    for path in paths {
        let p = path.unwrap().path();
        if let Some(suffix) = p.extension() {
            if suffix == "wasm" {
                println!(
                    "test {} ... bench:      {} ns/iter",
                    String::from(p.file_stem().unwrap().to_str().unwrap()),
                    p.metadata()?.len()
                );
            }
        }
    }
    Ok(1)
}

fn main() {
    let _rc = wasm_filesize("wasmpkg");
}
