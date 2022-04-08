// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use clap::{App, Arg, ArgGroup};
use icu_datagen::{get_all_keys, DatagenOptions};
use icu_provider::datagen::IterableDynProvider;
use icu_provider::datagen::{DataConverter, HeapStatsMarker};
use icu_provider_adapters::filter::Filterable;

use icu_provider::prelude::*;

use icu_datagen::cldr::CldrPathsAllInOne;
use icu_provider_blob::BlobDataProvider;
use litemap::LiteMap;
use simple_logger::SimpleLogger;
use std::borrow::Cow;
use std::cmp;
use std::collections::HashSet;
use std::fs;
use std::mem::ManuallyDrop;
use std::path::PathBuf;
use std::rc::Rc;

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

// Types in this list cannot be zero-copy deserialized (and are unlikely to work with CrabBake).
//
// Such types contain some data that was allocated during deserializations
//
// Every entry in this list is a bug that needs to be addressed before ICU4X 1.0.
static EXPECTED_NET_VIOLATIONS: &[&str] = &[
    // https://github.com/unicode-org/icu4x/issues/1678
    "datetime/skeletons@1",
    // https://github.com/unicode-org/icu4x/issues/1034
    "locale_canonicalizer/aliases@1",
];

// Types in this list can be zero-copy deserialized (and do not contain allocated data),
// however there is some allocation that occurs during deserialization for validation. This is unlikely to affect
// CrabBake since CrabBake can bypass validation steps.
//
// Entries in this list represent a less-than-ideal state of things, however ICU4X is shippable with violations
// in this list since it does not affect CrabBake.
static EXPECTED_TOTAL_VIOLATIONS: &[&str] = &[
    // Regex DFAs need to be validated, which involved creating a BTreeMap
    "list/and@1",
    "list/or@1",
    "list/unit@1",
];

fn main() -> eyre::Result<()> {
    // manually drop to avoid dhat from printing stats at the end
    let _profiler = ManuallyDrop::new(dhat::Profiler::new_heap());

    let matches = App::new("ICU4X Zero-Copy verifier")
        .version("0.0.1")
        .author("The ICU4X Project Developers")
        .about("Verify that a Postcard blob deserializes in a zero-copy fashion")
        .arg(
            Arg::with_name("VERBOSE")
                .short("v")
                .long("verbose")
                .help("Requests verbose output"),
        )
        .arg(
            Arg::with_name("POSTCARD_FILE")
                .long("postcard-file")
                .takes_value(true)
                .help("Path to .postcard file with all of the data."),
        )
        .arg(
            Arg::with_name("CLDR_ROOT")
                .long("cldr-root")
                .value_name("PATH")
                .help("Path to the CLDR JSON root directory.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("UPROPS_PATH")
                .long("uprops-path")
                .value_name("PATH")
                .help("Path to the uprops directory.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("INPUT_FROM_TESTDATA")
                .long("input-from-testdata")
                .help("Load input data from the icu_testdata project."),
        )
        .arg(
            Arg::with_name("CHECK")
                .long("check")
                .help("Check violations against list of expected violations (recommended to be used with --test-keys and features = experimental)."),
        )
        .arg(
            Arg::with_name("KEYS")
                .short("k")
                .long("keys")
                .multiple(true)
                .takes_value(true)
                .help(
                    "Include this resource key in the output. Accepts multiple arguments. \
                    Also see --test-keys.",
                ),
        )
        .group(
            ArgGroup::with_name("KEY_MODE")
                .arg("KEYS")
                .arg("TEST_KEYS")
                .required(true),
        )
        .arg(
            Arg::with_name("TEST_KEYS")
                .long("test-keys")
                .help("Include all keys supported by testdata."),
        )
        .get_matches();

    if matches.is_present("VERBOSE") {
        SimpleLogger::new()
            .with_level(log::LevelFilter::Trace)
            .init()
            .unwrap()
    } else {
        SimpleLogger::new()
            .env()
            .with_level(log::LevelFilter::Info)
            .init()
            .unwrap()
    }

    let all_keys = get_all_keys();
    let selected_keys = if matches.is_present("TEST_KEYS") {
        // We go with all keys now and later ignore key errors.
        all_keys
    } else {
        let mut keys = HashSet::new();

        if let Some(paths) = matches.values_of("KEYS") {
            keys.extend(paths.map(Cow::Borrowed));
        }

        let filtered: Vec<_> = all_keys
            .into_iter()
            .filter(|k| keys.contains(k.get_path()))
            .collect();

        if filtered.is_empty() {
            eyre::bail!("No keys selected (or keys passed in do not exist), pass in --keys KEYNAME or --test-keys");
        }

        filtered
    };

    let cldr_json_root = if let Some(cldr) = matches.value_of("CLDR_ROOT") {
        PathBuf::from(cldr)
    } else if matches.is_present("INPUT_FROM_TESTDATA") {
        icu_testdata::paths::cldr_json_root()
    } else {
        eyre::bail!("Value for --cldr--root must be specified (or --input-from-testdata)",)
    };
    let uprops_root = if let Some(uprops) = matches.value_of("UPROPS_PATH") {
        PathBuf::from(uprops)
    } else if matches.is_present("INPUT_FROM_TESTDATA") {
        icu_testdata::paths::uprops_toml_root()
    } else {
        eyre::bail!("Value for --uprops-path must be specified (or --input-from-testdata)",)
    };
    let cldr_paths = Box::new(CldrPathsAllInOne {
        cldr_json_root,
        locale_subset: matches
            .value_of("CLDR_LOCALE_SUBSET")
            .unwrap_or("full")
            .to_string(),
    });

    let segmenter_data_root = icu_datagen::segmenter::segmenter_data_root();

    let datagen_options = DatagenOptions {
        cldr_paths: &*cldr_paths,
        uprops_root: &uprops_root,
        segmenter_data_root: &segmenter_data_root,
    };

    let converter = icu_datagen::create_omnibus_provider!(datagen_options, &*cldr_paths, uprops_root, segmenter_data_root);

    let selected_locales = icu_testdata::metadata::load()?.package_metadata.locales;

    let converter = Box::new(
        converter
            .filterable("icu4x-datagen locales")
            .filter_by_langid_allowlist_strict(&selected_locales),
    );

    let postcard_file = if let Some(file) = matches.value_of("POSTCARD_FILE") {
        PathBuf::from(file)
    } else if matches.is_present("INPUT_FROM_TESTDATA") {
        icu_testdata::paths::data_root().join("testdata.postcard")
    } else {
        eyre::bail!("Value for --postcard-file must be specified (or --input-from-testdata)",)
    };
    let blob: Vec<u8> = fs::read(postcard_file).expect("Reading pre-computed postcard buffer");
    // Create a DataProvider from it:
    let provider =
        BlobDataProvider::new_from_rc_blob(Rc::from(blob)).expect("Deserialization should succeed");

    // Litemap keeps it sorted, convenient

    // violations for net_bytes_allocated
    let mut net_violations: LiteMap<&'static str, usize> = LiteMap::new();
    // violations for total_bytes_allocated (but not net_bytes_allocated)
    let mut total_violations: LiteMap<&'static str, u64> = LiteMap::new();

    for key in selected_keys.into_iter() {
        let props_key = key.get_path().starts_with("props/");

        let mut max_total_violation = 0;
        let mut max_net_violation = 0;

        for options in converter.supported_options_for_key(key)? {
            let result = provider.load_buffer(
                key,
                &DataRequest {
                    options: options.clone(),
                    metadata: Default::default(),
                },
            );
            let result = match result {
                Err(e) => {
                    if props_key {
                        // uprops keys currently don't all get loaded into the testdata
                        continue;
                    } else {
                        return Err(e.into());
                    }
                }
                Ok(r) => r,
            };
            let payload = result.take_payload()?;

            let stats: DataPayload<HeapStatsMarker> =
                converter.convert(key, payload).map_err(|e| e.1)?;
            let vio_total = stats.get().total_bytes_allocated;
            let vio_net = stats.get().net_bytes_allocated;
            log::trace!(
                "Key {} with options [{}] takes {} bytes ({} net)",
                key,
                options,
                vio_total,
                vio_net
            );
            max_total_violation = cmp::max(vio_total, max_total_violation);
            max_net_violation = cmp::max(vio_net, max_net_violation);
        }
        log::info!(
            "Key {} takes max {} bytes ({} net)",
            key,
            max_total_violation,
            max_net_violation
        );
        if max_total_violation != 0 {
            if max_net_violation != 0 {
                net_violations.insert(key.get_path(), max_net_violation);
            } else {
                total_violations.insert(key.get_path(), max_total_violation);
            }
        }
    }

    let total_vio_vec: Vec<_> = total_violations.iter_keys().copied().collect();
    let net_vio_vec: Vec<_> = net_violations.iter_keys().copied().collect();

    if total_vio_vec.is_empty() && net_vio_vec.is_empty() {
        log::info!("Congratulations! All keys are zero-copy");
    } else {
        log::info!("Found the following keys that are not yet zero-copy:");
        for (name, vio) in net_violations.iter() {
            log::info!("\t{}: max heap size {} bytes", name, vio);
        }

        if !total_vio_vec.is_empty() {
            log::info!("Also found the following keys that are zero-copy but temporarily allocate during deserialization:");

            for (name, vio) in total_violations.iter() {
                log::info!("\t{}: allocates a maximum of {} bytes", name, vio);
            }
        }
    }
    if matches.is_present("CHECK")
        && (total_vio_vec != EXPECTED_TOTAL_VIOLATIONS || net_vio_vec != EXPECTED_NET_VIOLATIONS)
    {
        eyre::bail!("Expected violations list does not match found violations!\n\
                    If the new list is smaller, please update EXPECTED_VIOLATIONS in verify-zero-copy.rs\n\
                    If it is bigger and that was unexpected, please make sure the key remains zero-copy, or ask ICU4X team members if it is okay\
                    to temporarily allow for this key to be allowlisted.\n\
                    Expected (net):\n{:?}\nFound (net):\n{:?}\nExpected (total):\n{:?}\nFound (total):\n{:?}", EXPECTED_NET_VIOLATIONS, net_vio_vec, EXPECTED_TOTAL_VIOLATIONS, total_vio_vec)
    }

    Ok(())
}
