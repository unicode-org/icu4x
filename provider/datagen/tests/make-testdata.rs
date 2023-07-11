// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_datagen::fs_exporter::serializers::{Json, Postcard};
use icu_datagen::fs_exporter::*;
use icu_datagen::prelude::*;
use icu_provider::datagen::*;
use icu_provider::prelude::*;
use std::collections::BTreeSet;
use std::path::Path;

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

include!("../../../tools/testdata-scripts/locales.rs.data"); // TODO where should this live?

#[test]
fn generate_fs_and_verify_zero_copy() {
    simple_logger::SimpleLogger::new()
        .env()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();

    let data_root = Path::new(concat!(core::env!("CARGO_MANIFEST_DIR"), "/tests/data/"));

    let source = SourceData::offline()
        .with_cldr(data_root.join("cldr"), Default::default())
        .unwrap()
        .with_icuexport(data_root.join("icuexport"))
        .unwrap();

    let json_out = Box::new(
        FilesystemExporter::try_new(Box::new(Json::pretty()), {
            let mut options = ExporterOptions::default();
            options.root = data_root.join("json");
            options.overwrite = OverwriteOption::RemoveAndReplace;
            options.fingerprint = true;
            options
        })
        .unwrap(),
    );

    let postcard_out = Box::new(
        FilesystemExporter::try_new(Box::<Postcard>::default(), {
            let mut options = ExporterOptions::default();
            options.root = data_root.join("postcard");
            options.overwrite = OverwriteOption::RemoveAndReplace;
            options.fingerprint = true;
            options
        })
        .unwrap(),
    );

    let mut options = options::Options::default();
    options.locales = options::LocaleInclude::Explicit(LOCALES.iter().cloned().collect());

    DatagenProvider::try_new(options, source)
        .unwrap()
        .export(
            icu_datagen::all_keys().into_iter().collect(),
            MultiExporter::new(vec![json_out, postcard_out]),
        )
        .unwrap();

    // don't drop to avoid dhat from printing stats at the end
    core::mem::forget(dhat::Profiler::new_heap());

    // violations for net_bytes_allocated
    let mut net_violations = BTreeSet::new();
    // violations for total_bytes_allocated (but not net_bytes_allocated)
    let mut total_violations = BTreeSet::new();

    for entry in glob::glob(
        &data_root
            .join("postcard/**/*.postcard")
            .display()
            .to_string(),
    )
    .unwrap()
    {
        let entry = entry.unwrap();

        let payload =
            DataPayload::from_owned_buffer(std::fs::read(&entry).unwrap().into_boxed_slice());

        let key = icu_datagen::key(
            &entry
                .strip_prefix(data_root.join("postcard"))
                .unwrap()
                .parent()
                .unwrap()
                .display()
                .to_string(),
        )
        .unwrap();

        let stats_before = dhat::HeapStats::get();

        // We need to generate the stats before the deserialized struct gets dropped, in order
        // to distinguish between a temporary and permanent allocation.
        let stats_after =
            icu_datagen::deserialize_and_discard(key, payload, dhat::HeapStats::get).unwrap();

        if stats_after.total_bytes != stats_before.total_bytes {
            if stats_after.curr_bytes != stats_before.curr_bytes {
                net_violations.insert(key.path().get());
            } else {
                total_violations.insert(key.path().get());
            }
        }
    }

    // Types in this list cannot be zero-copy deserialized.
    //
    // Such types contain some data that was allocated during deserializations
    //
    // Every entry in this list is a bug that needs to be addressed before ICU4X 1.0.
    const EXPECTED_NET_VIOLATIONS: &[&str] = &[
        // https://github.com/unicode-org/icu4x/issues/1678
        "datetime/skeletons@1",
    ];

    // Types in this list can be zero-copy deserialized (and do not contain allocated data),
    // however there is some allocation that occurs during deserialization for validation.
    //
    // Entries in this list represent a less-than-ideal state of things, however ICU4X is shippable with violations
    // in this list since it does not affect databake.
    const EXPECTED_TOTAL_VIOLATIONS: &[&str] = &[
        // Regex DFAs need to be validated, which involved creating a BTreeMap
        "list/and@1",
        "list/or@1",
        "list/unit@1",
    ];

    assert!(total_violations.iter().eq(EXPECTED_TOTAL_VIOLATIONS.iter()) && net_violations.iter().eq(EXPECTED_NET_VIOLATIONS.iter()),
        "Expected violations list does not match found violations!\n\
        If the new list is smaller, please update EXPECTED_VIOLATIONS in make-testdata.rs\n\
        If it is bigger and that was unexpected, please make sure the key remains zero-copy, or ask ICU4X team members if it is okay\
        to temporarily allow for this key to be allowlisted.\n\
        Expected (net):\n{EXPECTED_NET_VIOLATIONS:?}\nFound (net):\n{net_violations:?}\nExpected (total):\n{EXPECTED_TOTAL_VIOLATIONS:?}\nFound (total):\n{total_violations:?}");
}
