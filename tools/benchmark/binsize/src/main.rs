// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// This module has two modi operandi.
// 1.: It looks for wasm binaries in a directory (i.e., extension .wasm),
// notes the size in bytes of each, and writes the result in ndjson format
// to stdout. Likewise for benchmarking gnuzip'd wasm binaries.
// 2.: It writes the size of file in ndjson format to stdout.
// Note that the ICU4X wasm executables have to be build prior to
// execution of this module. Use 'cargo make wasm-example'.
use std::env;
use std::fs;
use std::process;

fn wasm_filesize(dir: &str, filesuffix: &str) -> Result<u64, std::io::Error> {
    let paths = fs::read_dir(dir).expect("Directory with wasm binaries not found!");
    let mut count: u64 = 0;
    for path in paths {
        let p = path.unwrap().path();
        if let Some(suffix) = p.extension() {
            if suffix == filesuffix {
                count += 1;
                println!(
                    // Write the file name and size in bytes to stdout in ndjson format.
                    "{{\"biggerIsBetter\":false,\"name\":{:?},\"unit\":\"bytes\",\"value\":{}}}",
                    p.file_name().unwrap(),
                    p.metadata()?.len()
                );
            }
        }
    }
    Ok(count)
}

fn any_file_size(filepath: &str) -> Result<bool, std::io::Error> {
    let exists: bool = std::path::Path::new(filepath).exists();
    if exists {
        let fsize = fs::metadata(filepath)?.len();
        println!(
            "{{\"biggerIsBetter\":false,\"name\":{filepath:?},\"unit\":\"bytes\",\"value\":{fsize}}}"
        );
    }
    Ok(exists)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: cargo run --package icu_benchmark_binsize -- <PATH> <wasm | gz | file>");
        process::exit(1);
    }
    let path = &args[1];
    let path_type = &args[2];

    if path_type != "wasm" && path_type != "gz" && path_type != "file" {
        eprintln!("Invalid path type {path_type}, use wasm or gz");
        process::exit(1);
    }

    if path_type == "file" {
        let rc = any_file_size(path);
        if !rc.unwrap() {
            eprintln!("File {path} not found");
        }
    } else {
        let count = wasm_filesize(path, path_type);
        if count.unwrap() == 0 {
            eprintln!("No wasm binaries found in directory {path}");
            process::exit(1);
        }
    }
}
