// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// This module takes as argument a directory path, looks for wasm
// executables in the directory (i.e., extension .wasm), takes the size in
// bytes of each, and writes the result in ndjson format to stdout.
// Note that the ICU4X wasm executables have to be build prior to
// execution of this module. Use 'cargo make wasm-example'.
use std::env;
use std::fs;
use std::process;

fn wasm_filesize(dir: &str) -> Result<u64, std::io::Error> {
    let paths = fs::read_dir(dir).expect("Directory with wasm binaries not found!");
    let mut count: u64 = 0;
    for path in paths {
        let p = path.unwrap().path();
        if let Some(suffix) = p.extension() {
            if suffix == "wasm" {
                count += 1;
                println!(
                    // Write the file name and size in bytes to stdout in ndjson format.
                    "{{\"biggerIsBetter\":false,\"name\":{:?},\"unit\":\"bytes\",\"value\":{}}}",
                    p.file_stem().unwrap(),
                    p.metadata()?.len()
                );
            }
        }
    }
    Ok(count)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run --package icu_benchmark_binsize -- <WASM BINARY DIRECTORY>");
        process::exit(1);
    }

    let wasmdir = &args[1];
    let count = wasm_filesize(wasmdir);
    if count.unwrap() == 0 {
        eprintln!("No wasm binaries found in directory {}", wasmdir);
        process::exit(1);
    }
}
