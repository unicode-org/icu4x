// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use clap::{App, Arg, ArgGroup};
use icu_datagen::get_all_keys;
use icu_provider::datagen::{DataConverter, HeapStatsMarker, OmnibusDatagenProvider};
use icu_provider::filter::Filterable;
use icu_provider::fork::by_key::ForkByKeyProvider;
use icu_provider::iter::IterableDynProvider;
use icu_provider::prelude::*;
use icu_provider::serde::SerializeMarker;
use icu_provider_blob::BlobDataProvider;
use icu_provider_cldr::CldrPathsAllInOne;
use litemap::LiteMap;
use simple_logger::SimpleLogger;
use std::borrow::Cow;
use std::cmp;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::mem::ManuallyDrop;
use std::path::PathBuf;
use std::rc::Rc;

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

static EXPECTED_VIOLATIONS: &[&str] = &[
    "datetime/skeletons@1",
    "list/and@1",
    "list/or@1",
    "list/unit@1",
    "locale_canonicalizer/aliases@1",
    "locale_canonicalizer/likelysubtags@1",
    "time_zone/formats@1",
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

    let converter: Box<dyn OmnibusDatagenProvider<SerializeMarker> + Sync> =
        Box::new(ForkByKeyProvider(
            icu_provider_cldr::create_exportable_provider(
                cldr_paths.as_ref(),
                uprops_root.clone(),
            )?,
            icu_provider_uprops::create_exportable_provider(&uprops_root)?,
        ));

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
    let mut blob: Vec<u8> = Vec::new();
    File::open(postcard_file)
        .expect("File should exist")
        .read_to_end(&mut blob)
        .expect("Reading pre-computed postcard buffer");
    // Create a DataProvider from it:
    let provider =
        BlobDataProvider::new_from_rc_blob(Rc::from(blob)).expect("Deserialization should succeed");

    // Litemap keeps it sorted, convenient
    let mut violations: LiteMap<&'static str, u64> = LiteMap::new();

    for key in selected_keys.into_iter() {
        let props_key = key.get_path().starts_with("props/");

        let mut max_violation = 0;

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
            let vio = stats.get().bytes_needed;
            log::trace!("Key {} with options [{}] takes {} bytes", key, options, vio);
            max_violation = cmp::max(vio, max_violation);
        }
        log::info!("Key {} takes max {} bytes", key, max_violation);
        if max_violation != 0 {
            violations.insert(key.get_path(), max_violation);
        }
    }

    let vio_vec: Vec<_> = violations.iter_keys().copied().collect();

    if vio_vec.is_empty() {
        log::info!("Congratulations! All keys are zero-copy");
    } else {
        log::info!("Found the following keys that are not yet zero-copy:");
        for (name, vio) in violations.iter() {
            log::info!("\t{}: max heap size {} bytes", name, vio);
        }
    }
    if matches.is_present("CHECK") && vio_vec != EXPECTED_VIOLATIONS {
        eyre::bail!("Expected violations list does not match found violations!\n\
                    If the new list is smaller, please update EXPECTED_VIOLATIONS in verify-zero-copy.rs\n\
                    If it is bigger and that was unexpected, please make sure the key remains zero-copy, or ask ICU4X team members if it is okay\
                    to temporarily allow for this key to be allowlisted.\n\
                    Expected:\n{:?}\nFound:\n{:?}", EXPECTED_VIOLATIONS, vio_vec)
    }

    Ok(())
}
