// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_datagen::get_all_keys;
use icu_provider::datagen::IterableDynProvider;
use icu_provider::datagen::{DataConverter, HeapStatsMarker};
use icu_provider_adapters::filter::Filterable;

use icu_provider::prelude::*;

use icu_datagen::SourceData;
use litemap::LiteMap;
use std::cmp;
use std::mem::ManuallyDrop;

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

#[test]
fn main() {
    // manually drop to avoid dhat from printing stats at the end
    let _profiler = ManuallyDrop::new(dhat::Profiler::new_heap());

    let selected_locales = icu_testdata::metadata::load()
        .unwrap()
        .package_metadata
        .locales;

    let converter = icu_datagen::create_datagen_provider!(SourceData::default()
        .with_cldr(icu_testdata::paths::cldr_json_root(), "full".to_string())
        .with_uprops(icu_testdata::paths::uprops_toml_root()))
    .filterable("icu4x-datagen locales")
    .filter_by_langid_allowlist_strict(&selected_locales);

    let provider = icu_testdata::get_postcard_provider();

    // Litemap keeps it sorted, convenient

    // violations for net_bytes_allocated
    let mut net_violations: LiteMap<&'static str, usize> = LiteMap::new();
    // violations for total_bytes_allocated (but not net_bytes_allocated)
    let mut total_violations: LiteMap<&'static str, u64> = LiteMap::new();

    for key in get_all_keys().into_iter() {
        let mut max_total_violation = 0;
        let mut max_net_violation = 0;

        for options in converter.supported_options_for_key(key).unwrap() {
            let result = provider.load_buffer(
                key,
                &DataRequest {
                    options: options.clone(),
                    metadata: Default::default(),
                },
            );
            let payload = match result {
                Err(_) if key.get_path().starts_with("props/") => {
                    // uprops keys currently don't all get loaded into the testdata
                    continue;
                }
                r => r.unwrap().take_payload().unwrap(),
            };

            let stats: DataPayload<HeapStatsMarker> =
                converter.convert(key, payload).map_err(|e| e.1).unwrap();
            let vio_total = stats.get().total_bytes_allocated;
            let vio_net = stats.get().net_bytes_allocated;
            max_total_violation = cmp::max(vio_total, max_total_violation);
            max_net_violation = cmp::max(vio_net, max_net_violation);
        }
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

    assert!(total_vio_vec == EXPECTED_TOTAL_VIOLATIONS && net_vio_vec == EXPECTED_NET_VIOLATIONS,
        "Expected violations list does not match found violations!\n\
        If the new list is smaller, please update EXPECTED_VIOLATIONS in verify-zero-copy.rs\n\
        If it is bigger and that was unexpected, please make sure the key remains zero-copy, or ask ICU4X team members if it is okay\
        to temporarily allow for this key to be allowlisted.\n\
        Expected (net):\n{:?}\nFound (net):\n{:?}\nExpected (total):\n{:?}\nFound (total):\n{:?}", EXPECTED_NET_VIOLATIONS, net_vio_vec, EXPECTED_TOTAL_VIOLATIONS, total_vio_vec)
}
