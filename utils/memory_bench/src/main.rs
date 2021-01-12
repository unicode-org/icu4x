// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use quote::quote;
use serde_json::json;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process;
use std::process::Command;
use std::{env, process::Stdio};
use std::{fs, io::BufReader};

fn inject_dhat_scoped_variable(syntax: &mut syn::File) {
    let dhat = syn::parse_str("let _dhat = Dhat::start_heap_profiling();")
        .expect("Unable to parse the dhat string");

    // Find the main function.
    for item in syntax.items.iter_mut() {
        if let syn::Item::Fn(ref mut fn_item) = item {
            if fn_item.sig.ident == "main" {
                // Create a new vector with the injected statement at the beginning.
                let mut new_stmts = vec![dhat];
                for stmt in fn_item.block.stmts.drain(..) {
                    new_stmts.push(stmt);
                }
                fn_item.block.stmts = new_stmts;
                return;
            }
        }
    }

    panic!("Unable to find the main function.");
}

fn inject_allocator_declaration(syntax: &mut syn::File) {
    let use_code = "use dhat::{Dhat, DhatAlloc};";
    let allocator_code = "
        #[global_allocator]
        static ALLOCATOR: DhatAlloc = DhatAlloc;
    ";
    let mut new_items = vec![
        syn::parse_str(use_code).expect("Unable to parse the dhat use string"),
        syn::parse_str(allocator_code).expect("Unable to parse the allocator string"),
    ];

    for item in syntax.items.drain(..) {
        new_items.push(item);
    }
    syntax.items = new_items;
}

/// dhat requires that the files are manually instrumented. This function automates
/// that process, by processing the AST using the syn package, modifying the AST,
/// and then finally writing it back out to a file with a slightly different name.
/// The instrumented version of the file can then be run.
fn run_dhat_injection(filename: &PathBuf) -> String {
    let mut file = File::open(&filename).expect("Unable to open file");

    let mut src = String::new();
    file.read_to_string(&mut src).expect("Unable to read file");

    let mut syntax = syn::parse_file(&src).expect("Unable to parse file");

    inject_allocator_declaration(&mut syntax);
    inject_dhat_scoped_variable(&mut syntax);

    quote!(#syntax).to_string()
}

fn print_usage() {
    eprintln!("Usage: cargo run --component memory -- [os] [component] [...example]");
    eprintln!("e.g. : cargo run --component memory -- macos-latest datetime work_log skeletons");
}

fn process_cli_args() -> (String, String, Vec<String>) {
    let mut args = env::args();

    let (os, component) = match (args.next(), args.next(), args.next()) {
        (Some(_executable), Some(os), Some(component)) => {
            if !os
                .chars()
                .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
            {
                eprintln!("The OS had an unexpected character \"{:?}\"", component);
                print_usage();
                process::exit(1);
            }

            if !component
                .chars()
                .all(|c| c.is_alphanumeric() || c == '_' || c == '/')
            {
                eprintln!(
                    "The component had an unexpected character \"{:?}\"",
                    component
                );
                print_usage();
                process::exit(1);
            }

            (os, component)
        }
        _ => {
            print_usage();
            process::exit(1);
        }
    };

    let mut examples = Vec::new();
    for example in args {
        if !example.chars().all(|c| c.is_alphanumeric() || c == '_') {
            eprintln!("An example had an unexpected character \"{:?}\"", example);
            print_usage();
            process::exit(1);
        }
        examples.push(example);
    }

    (os, component, examples)
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
        .replace(",", "")
        .parse::<u64>()
        .expect("Unable to parse the byte amount");
}

/// This file is intended to be run from CI to gather heap information, but it can also
/// be run locally. The charts are only generated in CI.
///
/// The workflow for this program is as follows:
///
/// 1. Process the CLI arguments to get the os, component, and examples.
/// 2. Create the directory for the benchmarks to go in.
/// 3. Loop through each example and:
///   a. Inject dhat memory instrumentation into the main function.
///   b. Write out that file to path/to/component/examples/foo___memory.rs
///   c. Run `cargo run --example foo` from the component directory
///   d. Extract the dhat stderr, and process out the interesting bytes.
///   e. Add the output to an `ndjson` file.
///   f. Move the dhat-heap.json file to the benchmark folder.
fn main() {
    let (os, component, examples) = process_cli_args();

    println!(
        "[memory_bench] Running memory instrumentation on examples in {:?}",
        component
    );

    let root_dir = {
        let mut path = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").expect("$CARGO_MANIFEST_DIR"));
        path.pop();
        path.pop();
        path
    };

    let component_dir = {
        let mut path = root_dir.clone();
        path.push(&component);
        if !path.exists() {
            panic!("Could not find the component at the path: {:?}", path);
        }
        path
    };

    let example_dir = {
        let mut path = component_dir.clone();
        path.push("examples");
        if !path.exists() {
            panic!(
                "Could not find the examples directory at the path: {:?}",
                path
            );
        }
        path
    };

    let benchmark_dir = {
        let mut path = root_dir;
        path.push("benchmarks/memory");
        path.push(os);
        path.push(&component);
        path
    };

    let benchmark_output_path = {
        let mut path = benchmark_dir.clone();
        path.push("output.ndjson");
        path
    };

    fs::create_dir_all(&benchmark_dir).unwrap_or_else(|err| {
        panic!(
            "Unable to create the benchmark directory {:?} {:?}",
            benchmark_dir, err
        );
    });

    if benchmark_output_path.exists() {
        fs::remove_file(&benchmark_output_path).unwrap_or_else(|err| {
            panic!(
                "Could not remove the file: {:?} {:?}",
                benchmark_output_path, err
            );
        });
    }

    let mut benchmark_output = fs::OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(&benchmark_output_path)
        .expect("Unable to open the benchmark output file for write.");

    for example in examples {
        println!("[memory_bench] Starting example {:?}", example);
        let example_path = {
            let mut path = example_dir.clone();
            path.push(format!("{}.rs", example));
            if !path.exists() {
                eprintln!("Could not find the example at the path: {:?}", path);
                process::exit(1);
            }
            path
        };
        let example_copy_path = {
            let mut path = example_dir.clone();
            path.push(format!("{}___memory.rs", example));
            path
        };

        println!(
            "[memory_bench] Injecting the memory instrumentation into {:?}",
            example
        );
        let instrumented_src = run_dhat_injection(&example_path);

        let mut file = File::create(&example_copy_path).unwrap_or_else(|err| {
            panic!(
                "Could not create the file {:?} {:?}",
                example_copy_path, err
            );
        });

        file.write_all(instrumented_src.as_bytes())
            .unwrap_or_else(|err| {
                eprintln!("Could not write the file {:?} {:?}", example_copy_path, err);
                process::exit(1);
            });

        println!(
            "[memory_bench] Running the instrumented example {:?}",
            example
        );

        let mut run_example = Command::new("cargo")
            .arg("+nightly")
            .arg("run")
            .arg("--example")
            .arg(format!("{}___memory", example))
            .arg("--profile")
            .arg("bench")
            .arg("-Z")
            .arg("unstable-options")
            .current_dir(&component_dir)
            .stderr(Stdio::piped())
            .spawn()
            .unwrap_or_else(|err| {
                eprintln!("The example {:?} failed to run. {:?}", example, err);
                process::exit(1);
            });

        let stdout = run_example
            .stderr
            .take()
            .expect("No stderr in the example.");

        let dhat_log: Vec<_> = BufReader::new(stdout)
            .lines()
            .map(|s| s.expect("Unable to read from stderr."))
            .inspect(|s| println!("[memory_bench] > {}", s))
            .filter(|s| s.starts_with("dhat: "))
            .collect();

        fs::remove_file(&example_copy_path).unwrap_or_else(|err| {
            panic!(
                "Could not remove the file {:?} {:?}",
                example_copy_path, err
            );
        });

        let (total, gmax, end) = parse_dhat_log(&dhat_log);

        let write_json = |bytes, label| {
            json!({
                 "name": label,
                 "unit": "bytes",
                 "value": bytes,
                 "biggerIsBetter": false
            })
            .to_string()
        };

        let output = format!(
            "{}\n{}\n{}\n",
            write_json(total, format!("{} – Total Heap Allocations", example)),
            write_json(gmax, format!("{} – Heap at Global Memory Max", example)),
            write_json(
                end,
                format!("{} – Heap at End of Program Execution", example)
            ),
        );

        benchmark_output
            .write_all(output.as_bytes())
            .expect("Unable to write out the results.");

        let dhat_destination = {
            let mut path = benchmark_dir.clone();
            path.push(format!("{}-dhat-heap.json", example));
            path
        };

        let dhat_source = {
            let mut path = component_dir.clone();
            path.push("dhat-heap.json");
            assert!(path.exists(), "The dhat-heap.json file did not exist.");
            path
        };

        fs::rename(&dhat_source, &dhat_destination).expect("Unable to move the dhat-heap.json");

        println!("[memory_bench] Memory log:  {:?}", benchmark_output_path);
        println!("[memory_bench] dhat file:   {:?}", dhat_destination);
        println!(
            "[memory_bench] Viewable in: https://gregtatum.github.io/dhat-viewer/dh_view.html"
        );
        println!("\n{}", output);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dhat_injection() {
        let path = {
            let mut path =
                PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("$CARGO_MANIFEST_DIR"));
            path.push("tests/fixtures/code.rs.txt");
            path
        };

        // The result ends up on one line of text which is a bit hard to read.
        // Split it up on the semi-colons to improve readability.
        let result: Vec<String> = run_dhat_injection(&path)
            .split(';')
            .map(String::from)
            .collect();

        assert_eq!(
            result,
            [
                // Injected code:
                "use dhat :: { Dhat , DhatAlloc } ",
                " # [global_allocator] static ALLOCATOR : DhatAlloc = DhatAlloc ",
                // Original code:
                " use std :: env ",
                " fn do_something () { println ! (\"It has another function in it.\") ",
                " println ! (\"It uses an import. {}\" , env ! (\"CARGO_PKG_VERSION\") ",
                ") ",
                // Injected code:
                " } fn main () { let _dhat = Dhat :: start_heap_profiling () ",
                //               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
                // Original:
                " println ! (\"This is a test fixture\") ",
                " }",
            ],
        );
    }

    #[test]
    fn test_byte_extraction() {
        let log = vec![
            String::from("dhat: Total:     20,122 bytes in 129 blocks"),
            String::from("dhat: At t-gmax: 9,328 bytes in 90 blocks"),
            String::from("dhat: At t-end:  0 bytes in 0 blocks"),
            String::from("dhat: The data in dhat-heap.json is viewable with dhat/dh_view.html"),
        ];
        let (total, gmax, end) = parse_dhat_log(&log);

        assert_eq!(total, 20122);
        assert_eq!(gmax, 9328);
        assert_eq!(end, 0);
    }
}
