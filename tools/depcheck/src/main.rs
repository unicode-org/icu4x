// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use self::allowlist::*;
use std::collections::BTreeSet;
use std::process::{self, Command};
use std::str;

mod allowlist;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct DepSpec {
    crate_name: String,
    crate_version: String,
}

/// Get the dependency list produced by `cargo tree -p {package} -e {edge_kind}`
fn get_dep_list(package: &str, edge_kind: &str, extra_args: &str) -> Vec<DepSpec> {
    let mut cmd = Command::new("cargo");
    cmd.arg("tree")
        .arg("-p")
        .arg(package)
        .arg("-e")
        .arg(edge_kind)
        .arg("--no-default-features");
    for arg in extra_args.split(' ') {
        if !arg.is_empty() {
            cmd.arg(arg);
        }
    }
    let output = cmd.output().expect("Failed to run `cargo tree`");

    if !output.status.success() {
        eprintln!("Failed to run `cargo tree -p {package} -e {edge_kind} --no-default-features {extra_args}`:");
        if let Ok(s) = str::from_utf8(&output.stderr) {
            eprintln!("{s}");
        }
        process::exit(1);
    }
    let mut spec: Vec<_> = output
        .stdout
        .split(|b| *b == b'\n')
        .filter_map(|slice| {
            if slice.is_empty() {
                return None;
            }
            if slice[0] == b'[' {
                // cargo tree output has sections like `[dev-dependencies]`
                return None;
            }

            let mut iter = slice.split(|b| *b == b' ');
            let mut found_crate_name = None;
            for section in &mut iter {
                if section.is_empty() {
                    continue;
                }
                // The format is {line drawing characters} {crate name} {crate version}
                if char::from(section[0]).is_ascii_alphabetic() {
                    found_crate_name =
                        Some(str::from_utf8(section).expect("Must be utf-8").to_owned());
                    break;
                }
            }
            if let Some(crate_name) = found_crate_name {
                let crate_version = iter
                    .next()
                    .expect("There must be a version after the crate name!");
                let crate_version = str::from_utf8(crate_version)
                    .expect("Must be utf-8")
                    .to_owned();
                Some(DepSpec {
                    crate_name,
                    crate_version,
                })
            } else {
                None
            }
        })
        .collect();
    spec.sort();
    spec.dedup();

    spec
}

/// Given a `cargo tree` invocation and the dependency sets to check, checks for any unlisted or duplicated deps
///
/// `dep_list_name_for_error` is the name of the const above to show in the error suggestion
fn test_dep_list(
    package: &str,
    edge_kind: &str,
    extra_args: &str,
    sets: &[&BTreeSet<&str>],
    dep_list_name_for_error: &str,
) {
    println!("Testing `cargo tree -p {package} -e {edge_kind} --no-default-features {extra_args}`");
    let mut errors = Vec::new();
    let dep_list = get_dep_list(package, edge_kind, extra_args);
    for i in dep_list.windows(2) {
        if i[0].crate_name == i[1].crate_name {
            errors.push(format!(
                "Found two versions for `{0}` ({1} & {2})",
                i[0].crate_name, i[0].crate_version, i[1].crate_version
            ));
        }
    }

    'dep_loop: for i in dep_list {
        let name = &i.crate_name;
        for s in sets {
            if s.contains(&**name) {
                continue 'dep_loop;
            }
        }
        errors.push(format!(
            "Found non-allowlisted crate `{name}`, consider adding to \
                             {dep_list_name_for_error} in tools/depcheck/src/allowlist.rs if intentional"
        ));
    }

    if !errors.is_empty() {
        eprintln!("Found invalid dependencies:");
        for e in errors {
            eprintln!("\t{e}");
        }
        process::exit(1);
    }
}

fn main() {
    let basic: BTreeSet<_> = BASIC_DEPS.iter().copied().collect();
    let serde: BTreeSet<_> = EXTRA_SERDE_DEPS.iter().copied().collect();
    let experimental: BTreeSet<_> = EXTRA_EXPERIMENTAL_DEPS.iter().copied().collect();
    let lstm: BTreeSet<_> = EXTRA_LSTM_DEPS.iter().copied().collect();
    let ryu: BTreeSet<_> = EXTRA_RYU_DEPS.iter().copied().collect();
    let capi: BTreeSet<_> = EXTRA_CAPI_DEPS.iter().copied().collect();
    let capi_build: BTreeSet<_> = EXTRA_CAPI_BUILD_DEPS.iter().copied().collect();
    let capi_logging: BTreeSet<_> = EXTRA_CAPI_LOGGING_DEPS.iter().copied().collect();
    let blob: BTreeSet<_> = EXTRA_BLOB_DEPS.iter().copied().collect();
    let fs: BTreeSet<_> = EXTRA_FS_DEPS.iter().copied().collect();
    let test: BTreeSet<_> = EXTRA_TEST_DEPS.iter().copied().collect();
    test_dep_list("icu", "normal", "", &[&basic], "`BASIC_DEPS`");
    test_dep_list(
        "icu",
        "normal",
        "--features serde",
        &[&basic, &serde],
        "`EXTRA_SERDE_DEPS`",
    );
    test_dep_list(
        "icu",
        "normal",
        "--features serde,experimental",
        &[&basic, &serde, &experimental],
        "`EXTRA_EXPERIMENTAL_DEPS`",
    );
    test_dep_list(
        "icu",
        "normal",
        "--features serde,experimental,icu_segmenter/lstm",
        &[&basic, &serde, &experimental, &lstm],
        "`EXTRA_LSTM_DEPS`",
    );
    test_dep_list(
        "icu_segmenter",
        "normal",
        "--features lstm",
        &[&basic, &experimental, &lstm],
        "`EXTRA_LSTM_DEPS`",
    );
    test_dep_list(
        "fixed_decimal",
        "normal",
        "--features fixed_decimal/ryu",
        &[&basic, &ryu],
        "`EXTRA_RYU_DEPS`",
    );

    test_dep_list(
        "icu_capi",
        "normal,no-proc-macro",
        "",
        // capi should NOT pull in serde or capi-build when the proc macro is disabled
        &[&basic, &experimental, &lstm, &ryu, &capi],
        "`EXTRA_CAPI_DEPS`",
    );

    test_dep_list(
        "icu_capi",
        "normal",
        "",
        &[
            &basic,
            &serde,
            &experimental,
            &lstm,
            &ryu,
            &capi,
            &capi_build,
        ],
        "`EXTRA_CAPI_BUILD_DEPS`",
    );

    test_dep_list(
        "icu_capi",
        "normal",
        "--features buffer_provider",
        &[
            &basic,
            &serde,
            &experimental,
            &lstm,
            &ryu,
            &capi,
            &capi_build,
            &blob,
        ],
        "`EXTRA_BLOB_DEPS`",
    );
    test_dep_list(
        "icu_capi",
        "normal",
        "--features provider_fs",
        &[
            &basic,
            &serde,
            &experimental,
            &lstm,
            &ryu,
            &capi,
            &capi_build,
            &blob,
            &fs,
        ],
        "`EXTRA_FS_DEPS`",
    );
    test_dep_list(
        "icu_capi",
        "normal",
        "--features provider_test",
        &[
            &basic,
            &serde,
            &experimental,
            &lstm,
            &ryu,
            &capi,
            &capi_build,
            &test,
        ],
        "`EXTRA_TEST_DEPS`",
    );
    test_dep_list(
        "icu_capi",
        "normal",
        "--features logging",
        &[
            &basic,
            &serde,
            &experimental,
            &lstm,
            &ryu,
            &capi,
            &capi_build,
            &capi_logging,
        ],
        "`EXTRA_CAPI_LOGGING_DEPS`",
    );
    // we aren't testing simple-logger, it's mostly for debugging purposes
}
