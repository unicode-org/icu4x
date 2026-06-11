// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use clap::Parser;
use serde_json::{Value, json};
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process;
use std::process::Command;
use std::process::Stdio;
use std::{fs, io::BufReader};

#[derive(Parser)]
#[command(about = "Collect a memory report for examples using dhat-rs.")]
struct ProcessedArgs {
    #[arg(
        long,
        value_name = "OS",
        help = "Nests the results of the benchmark in a folder per-OS, primarily needed by CI."
    )]
    os: Option<String>,
    #[arg(value_name = "EXAMPLES", num_args = 0.., index=1)]
    #[arg(help = "The space separated list of examples to run. Leave empty for all examples.")]
    examples: Vec<String>,
}

fn process_cli_args() -> ProcessedArgs {
    let processed = ProcessedArgs::parse();

    if let Some(ref os) = processed.os
        && !os
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
    {
        panic!("The OS had an unexpected character");
    }
    for example in &processed.examples {
        if !example
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '/')
        {
            panic!("An example had an unexpected character \"{example:?}\"");
        }
    }
    processed
}

fn parse_dhat_json_file(path: &Path) -> (u64, u64, u64) {
    let json = fs::read_to_string(path).unwrap_or_else(|err| {
        panic!("Unable to read dhat JSON file {path:?}: {err:?}");
    });
    parse_dhat_json_str(&json)
}

fn parse_dhat_json_str(json: &str) -> (u64, u64, u64) {
    let json: Value = serde_json::from_str(json).expect("Unable to parse dhat JSON.");
    assert_eq!(
        json.get("dhatFileVersion").and_then(Value::as_u64),
        Some(2),
        "Unsupported dhat JSON file version."
    );
    assert_eq!(
        json.get("mode").and_then(Value::as_str),
        Some("rust-heap"),
        "Expected dhat heap profiling JSON."
    );

    let pps = json
        .get("pps")
        .and_then(Value::as_array)
        .expect("Unable to read dhat program points.");

    let sum_field = |field: &str| -> u64 {
        pps.iter()
            .map(|pp| {
                pp.get(field)
                    .and_then(Value::as_u64)
                    .unwrap_or_else(|| panic!("Unable to read dhat field {field:?}."))
            })
            .sum()
    };

    (sum_field("tb"), sum_field("gb"), sum_field("eb"))
}

/// This file is intended to be run from CI to gather heap information, but it can also
/// be run locally. The charts are only generated in CI.
///
/// The workflow for this program is as follows:
///
/// 1. Process the CLI arguments to get the os, and examples.
/// 2. Loop through each example and:
///    a. Create the directory for the benchmarks to go in.
///    b. Run `cargo run --example {example}` with the appropriate settings.
///    c. Parse the dhat JSON, and process out the interesting bytes.
///    d. Add the output to an `ndjson` file.
///    e. Move the dhat-heap.json file to the benchmark folder.
fn main() {
    let ProcessedArgs { os, examples } = process_cli_args();

    let root_dir = PathBuf::from(concat!(std::env!("CARGO_MANIFEST_DIR"), "/../../.."));

    let examples = if !examples.is_empty() {
        examples
    } else {
        Command::new("cargo")
            .arg("build")
            .arg("--examples")
            .arg("--profile")
            .arg("bench-memory")
            .arg("--features")
            .arg("icu_benchmark_macros/benchmark_memory")
            .status()
            .unwrap_or_else(|err| {
                eprintln!("Failed to collect examples {err:?}");
                process::exit(1);
            });
        fs::read_dir(root_dir.join("target/bench-memory/examples"))
            .unwrap()
            .flat_map(|entry| {
                entry.ok()?.file_name().into_string().ok().and_then(|s| {
                    if cfg!(windows) {
                        s.strip_suffix(".exe").map(ToString::to_string)
                    } else {
                        (!s.contains(['-', '.'])).then_some(s)
                    }
                })
            })
            .collect()
    };

    println!("[memory] Examples to benchmark:  {examples:?}");

    // benchmarks/memory/{os}
    let benchmark_dir = root_dir
        .join("benchmarks/memory")
        .join(os.as_deref().unwrap_or("."));

    // Make the directory: benchmarks/memory/{os}
    fs::create_dir_all(&benchmark_dir).unwrap_or_else(|err| {
        panic!("Unable to create the benchmark directory {benchmark_dir:?} {err:?}");
    });

    // benchmarks/memory/{os}/output.ndjson
    let benchmark_output_path = benchmark_dir.join("output.ndjson");

    if benchmark_output_path.exists() {
        fs::remove_file(&benchmark_output_path).unwrap_or_else(|err| {
            panic!("Could not remove the file: {benchmark_output_path:?} {err:?}");
        });
    }

    for ref example in examples {
        let dhat_source = Path::new("dhat-heap.json");
        if dhat_source.exists() {
            fs::remove_file(dhat_source).unwrap_or_else(|err| {
                panic!("Unable to remove stale dhat JSON file {dhat_source:?}: {err:?}");
            });
        }

        let mut benchmark_output = fs::OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(&benchmark_output_path)
            .expect("Unable to open the benchmark output file for write.");

        println!("[memory] Starting example {example:?}");

        let mut run_example = Command::new("cargo")
            .arg("run")
            .arg("--example")
            .arg(example)
            .arg("--profile")
            .arg("bench-memory")
            // The dhat-rs instrumentation is hidden behind the "benchmark_memory" feature in the
            // icu_benchmark_macros package.
            .arg("--features")
            .arg("icu_benchmark_macros/benchmark_memory")
            .stderr(Stdio::piped())
            .spawn()
            .unwrap_or_else(|err| {
                eprintln!("The example {example:?} failed to run. {err:?}");
                process::exit(1);
            });

        let stderr = run_example
            .stderr
            .take()
            .expect("No stderr in the example.");

        for line in BufReader::new(stderr)
            .lines()
            .map(|s| s.expect("Unable to read from stderr."))
        {
            println!("[memory] > {line}");
        }

        let status = run_example
            .wait()
            .expect("Unable to get the status of the example child process.");

        if !status.success() {
            eprintln!(
                "The example \"{}\" had a non-zero exit code: {example:?}",
                status.code().expect("An example could not be run.")
            );
            process::exit(1);
        }

        if !dhat_source.exists() {
            eprintln!(
                "The {example:?} example needs to be instrumented with icu_benchmark_macros."
            );
            continue;
        }
        let (total, gmax, end) = parse_dhat_json_file(dhat_source);

        let write_json = |bytes, label| {
            json!({
                 "name": label,
                 "unit": "bytes",
                 "value": bytes,
                 "biggerIsBetter": false
            })
        };

        write!(
            benchmark_output,
            "{}\n{}\n{}\n",
            write_json(total, format!("{example} – Total Heap Allocations")),
            write_json(gmax, format!("{example} – Heap at Global Memory Max")),
            write_json(end, format!("{example} – Heap at End of Program Execution")),
        )
        .expect("Unable to write out the results.");

        let dhat_destination = benchmark_dir.join(format!("{example}-dhat-heap.json"));

        fs::rename(dhat_source, &dhat_destination).expect("Unable to move the dhat-heap.json");

        println!("[memory] Memory log:  {benchmark_output_path:?}");
        println!("[memory] dhat file:   {dhat_destination:?}");
        println!("[memory] Viewable in: https://nnethercote.github.io/dh_view/dh_view.html");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_json_extraction() {
        let json = r#"{
"dhatFileVersion":2,
"mode":"rust-heap",
"pps":[
{"tb":1000,"gb":1000,"eb":0},
{"tb":600,"gb":400,"eb":0},
{"tb":32,"gb":32,"eb":32},
{"tb":10,"gb":0,"eb":0}
]
}"#;

        let (total, gmax, end) = parse_dhat_json_str(json);

        assert_eq!(total, 1642);
        assert_eq!(gmax, 1432);
        assert_eq!(end, 32);
    }
}
