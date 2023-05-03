// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[cfg(not(target_os = "windows"))]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

// Something is broken wrt Windows and this test on CI. Disable for now.
#[cfg(not(target_os = "windows"))]
pub mod test {
    use icu_datagen::{all_keys_with_experimental, DatagenProvider, SourceData};
    use icu_provider::datagen::IterableDynamicDataProvider;
    use icu_provider::prelude::*;
    use std::cmp;
    use std::collections::BTreeSet;

    // Types in this list cannot be zero-copy deserialized.
    //
    // Such types contain some data that was allocated during deserializations
    //
    // Every entry in this list is a bug that needs to be addressed before ICU4X 1.0.
    static EXPECTED_NET_VIOLATIONS: &[&str] = &[
        // https://github.com/unicode-org/icu4x/issues/1678
        "datetime/skeletons@1",
    ];

    // Types in this list can be zero-copy deserialized (and do not contain allocated data),
    // however there is some allocation that occurs during deserialization for validation.
    //
    // Entries in this list represent a less-than-ideal state of things, however ICU4X is shippable with violations
    // in this list since it does not affect databake.
    static EXPECTED_TOTAL_VIOLATIONS: &[&str] = &[
        // Regex DFAs need to be validated, which involved creating a BTreeMap
        "list/and@1",
        "list/or@1",
        "list/unit@1",
    ];

    #[test]
    fn test_zero_copy() {
        // don't drop to avoid dhat from printing stats at the end
        core::mem::forget(dhat::Profiler::new_heap());

        // Actual data is only needed to determine included locales.
        let locale_provider = DatagenProvider::try_new(
            {
                use icu_datagen::options::*;
                let mut options = Options::default();
                options.locales =
                    LocaleInclude::Explicit(icu_testdata::locales().into_iter().collect());
                options
            },
            SourceData::offline()
                .with_cldr(repodata::paths::cldr(), Default::default())
                .unwrap()
                .with_icuexport(repodata::paths::icuexport())
                .unwrap()
                .with_segmenter_lstm(repodata::paths::lstm())
                .unwrap(),
        )
        .unwrap();

        let postcard_provider = icu_testdata::buffer_no_fallback();

        // violations for net_bytes_allocated
        let mut net_violations = BTreeSet::new();
        // violations for total_bytes_allocated (but not net_bytes_allocated)
        let mut total_violations = BTreeSet::new();

        for key in all_keys_with_experimental().into_iter() {
            let mut max_total_violation = 0;
            let mut max_net_violation = 0;

            for locale in locale_provider.supported_locales_for_key(key).unwrap() {
                let payload = postcard_provider
                    .load_buffer(
                        key,
                        DataRequest {
                            locale: &locale,
                            metadata: Default::default(),
                        },
                    )
                    .unwrap()
                    .take_payload()
                    .unwrap();

                let stats_before = dhat::HeapStats::get();

                // We need to generate the stats before the deserialized struct gets dropped, in order
                // to distinguish between a temporary and permanent allocation.
                let stats_after =
                    icu_datagen::deserialize_and_discard(key, payload, dhat::HeapStats::get)
                        .unwrap();

                let vio_total = stats_after.total_bytes - stats_before.total_bytes;
                let vio_net = stats_after.curr_bytes - stats_before.curr_bytes;
                max_total_violation = cmp::max(vio_total, max_total_violation);
                max_net_violation = cmp::max(vio_net, max_net_violation);
            }
            if max_total_violation != 0 {
                if max_net_violation != 0 {
                    net_violations.insert(key.path().get());
                } else {
                    total_violations.insert(key.path().get());
                }
            }
        }

        assert!(total_violations.iter().eq(EXPECTED_TOTAL_VIOLATIONS.iter()) && net_violations.iter().eq(EXPECTED_NET_VIOLATIONS.iter()),
            "Expected violations list does not match found violations!\n\
            If the new list is smaller, please update EXPECTED_VIOLATIONS in verify-zero-copy.rs\n\
            If it is bigger and that was unexpected, please make sure the key remains zero-copy, or ask ICU4X team members if it is okay\
            to temporarily allow for this key to be allowlisted.\n\
            Expected (net):\n{EXPECTED_NET_VIOLATIONS:?}\nFound (net):\n{net_violations:?}\nExpected (total):\n{EXPECTED_TOTAL_VIOLATIONS:?}\nFound (total):\n{total_violations:?}")
    }
}

#[cfg(target_os = "windows")]
#[test]
fn test_zero_copy() {
    // do nothing
}
