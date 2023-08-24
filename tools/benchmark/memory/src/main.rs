// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use clap::Parser;
use serde_json::json;
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

    if let Some(ref os) = processed.os {
        if !os
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            panic!("The OS had an unexpected character");
        }
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

fn parse_dhat_log(dhat_log: &[String]) -> (u64, u64, u64) {
    assert_eq!(
        dhat_log.len(),
        4,
        "Expected the dhat output to be 4 lines long."
    );

    (
        extract_bytes_from_log_line("dhat: Total:", dhat_log.get(0).unwrap()),
        extract_bytes_from_log_line("dhat: At t-gmax:", dhat_log.get(1).unwrap()),
        extract_bytes_from_log_line("dhat: At t-end:", dhat_log.get(2).unwrap()),
    )
}

fn extract_bytes_from_log_line(preamble: &str, text: &str) -> u64 {
    let start = preamble.len();
    let end = text
        .find("bytes")
        .expect("Unable to find the word \"bytes\" in the dhat output.");

    return text
        .get(start..end)
        .expect("Unable to get a substring.")
        .trim()
        .replace(',', "")
        .parse::<u64>()
        .expect("Unable to parse the byte amount");
}

/// This file is intended to be run from CI to gather heap information, but it can also
/// be run locally. The charts are only generated in CI.
///
/// The workflow for this program is as follows:
///
/// 1. Process the CLI arguments to get the os, and examples.
/// 2. Loop through each example and:
///   a. Create the directory for the benchmarks to go in.
///   b. Run `cargo run --example {example}` with the appropriate settings.
///   c. Extract the dhat stderr, and process out the interesting bytes.
///   d. Add the output to an `ndjson` file.
///   e. Move the dhat-heap.json file to the benchmark folder.
fn main() {
    let ProcessedArgs { os, examples } = process_cli_args();

    let root_dir = PathBuf::from(concat!(std::env!("CARGO_MANIFEST_DIR"), "/../../.."));

    let examples = if !examples.is_empty() {
        examples
    } else {
        Command::new("cargo")
            .arg("build")
            .arg("--examples")
            .arg("--features")
            .arg("icu_benchmark_macros/benchmark_memory")
            .arg("--features")
            .arg("bench")
            .status()
            .unwrap_or_else(|err| {
                eprintln!("Failed to collect examples {err:?}");
                process::exit(1);
            });
        fs::read_dir(root_dir.join("target/debug/examples"))
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
            .arg("bench")
            // The dhat-rs instrumentation is hidden behind the "benchmark_memory" feature in the
            // icu_benchmark_macros package.
            .arg("--features")
            .arg("icu_benchmark_macros/benchmark_memory")
            .arg("--features")
            .arg("bench")
            .stderr(Stdio::piped())
            .spawn()
            .unwrap_or_else(|err| {
                eprintln!("The example {example:?} failed to run. {err:?}");
                process::exit(1);
            });

        let stdout = run_example
            .stderr
            .take()
            .expect("No stderr in the example.");

        let dhat_log: Vec<_> = BufReader::new(stdout)
            .lines()
            .map(|s| s.expect("Unable to read from stderr."))
            .inspect(|s| println!("[memory] > {s}"))
            .filter(|s| s.starts_with("dhat: "))
            .collect();

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

        if dhat_log.is_empty() {
            eprintln!(
                "The {example:?} example needs to be instrumented with icu_benchmark_macros."
            );
            continue;
        }
        let (total, gmax, end) = parse_dhat_log(&dhat_log);

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

        let dhat_source = Path::new("dhat-heap.json");

        assert!(
            dhat_source.exists(),
            "The dhat-heap.json file did not exist."
        );

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
    fn test_byte_extraction() {
        let log = [
            "dhat: Total:     20,122 bytes in 129 blocks".to_owned(),
            "dhat: At t-gmax: 9,328 bytes in 90 blocks".to_owned(),
            "dhat: At t-end:  0 bytes in 0 blocks".to_owned(),
            "dhat: The data in dhat-heap.json is viewable with dhat/dh_view.html".to_owned(),
        ];
        let (total, gmax, end) = parse_dhat_log(&log);

        assert_eq!(total, 20122);
        assert_eq!(gmax, 9328);
        assert_eq!(end, 0);
    }
}
