// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_datagen::fs_exporter::serializers::Json;
use icu_datagen::fs_exporter::*;
use icu_datagen::prelude::*;
use icu_provider::datagen::*;
use icu_provider::prelude::*;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;

#[global_allocator]
static A: icu_datagen::MeasuringAllocator = icu_datagen::MeasuringAllocator;

include!("data/locales.rs.data");

#[test]
fn generate_json_and_verify_postcard() {
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
            options
        })
        .unwrap(),
    );

    let postcard_out = Box::new(PostcardTestingExporter {
        size_hash: Default::default(),
        zero_copy_violations: Default::default(),
        zero_copy_net_violations: Default::default(),
        rountrip_errors: Default::default(),
        fingerprints: File::create(data_root.join("postcard/fingerprints.csv")).unwrap(),
    });

    let mut options = options::Options::default();
    options.locales = options::LocaleInclude::Explicit(LOCALES.iter().cloned().collect());

    DatagenProvider::try_new(options, source)
        .unwrap()
        .export(
            icu_datagen::all_keys().into_iter().collect(),
            MultiExporter::new(vec![json_out, postcard_out]),
        )
        .unwrap();
}

struct PostcardTestingExporter {
    size_hash: Mutex<BTreeMap<(DataKey, String), (usize, u64)>>,
    zero_copy_violations: Mutex<BTreeSet<DataKey>>,
    zero_copy_net_violations: Mutex<BTreeSet<DataKey>>,
    rountrip_errors: Mutex<BTreeSet<(DataKey, String)>>,
    fingerprints: File,
}

// Types in this list cannot be zero-copy deserialized.
//
// Such types contain some data that was allocated during deserializations
//
// Every entry in this list is a bug that needs to be addressed before ICU4X 1.0.
const EXPECTED_NET_VIOLATIONS: &[DataKey] = &[
    // https://github.com/unicode-org/icu4x/issues/1678
    icu_datetime::provider::calendar::DateSkeletonPatternsV1Marker::KEY,
];

// Types in this list can be zero-copy deserialized (and do not contain allocated data),
// however there is some allocation that occurs during deserialization for validation.
//
// Entries in this list represent a less-than-ideal state of things, however ICU4X is shippable with violations
// in this list since it does not affect databake.
const EXPECTED_TOTAL_VIOLATIONS: &[DataKey] = &[
    // Regex DFAs need to be validated, which involved creating a BTreeMap
    icu_list::provider::AndListV1Marker::KEY,
    icu_list::provider::OrListV1Marker::KEY,
    icu_list::provider::UnitListV1Marker::KEY,
];

impl DataExporter for PostcardTestingExporter {
    fn put_payload(
        &self,
        key: DataKey,
        locale: &DataLocale,
        payload_before: &DataPayload<ExportMarker>,
    ) -> Result<(), DataError> {
        use postcard::{
            ser_flavors::{AllocVec, Flavor},
            Serializer,
        };
        let mut serializer = Serializer {
            output: AllocVec::new(),
        };
        payload_before.serialize(&mut serializer).unwrap();
        let serialized = serializer.output.finalize().unwrap();

        let size = serialized.len();

        // We're using SipHash, which is deprecated, but we want a stable hasher
        // (we're fine with it not being cryptographically secure since we're just using it to track diffs)
        #[allow(deprecated)]
        use std::hash::{Hash, Hasher, SipHasher};
        #[allow(deprecated)]
        let mut hasher = SipHasher::new();
        serialized.iter().for_each(|b| b.hash(&mut hasher));
        let hash = hasher.finish();

        let buffer_payload = DataPayload::from_owned_buffer(serialized.into_boxed_slice());

        let (payload_after, (heap_diff, heap_max)) = icu_datagen::deserialize_and_measure(key, buffer_payload).unwrap();

        if payload_before != &payload_after {
            self.rountrip_errors
                .lock()
                .expect("poison")
                .insert((key, locale.to_string()));
        }

        if heap_max > 0 {
            if heap_diff != 0 {
                if !EXPECTED_NET_VIOLATIONS.contains(&key) {
                    println!("Net violation {key} {locale}");
                }
                self.zero_copy_net_violations
                    .lock()
                    .expect("poison")
                    .insert(key);
            } else {
                if !EXPECTED_TOTAL_VIOLATIONS.contains(&key) {
                    println!("Violation {key} {locale}");
                }
                self.zero_copy_violations
                    .lock()
                    .expect("poison")
                    .insert(key);
            }
        }

        self.size_hash
            .lock()
            .expect("poison")
            .insert((key, locale.to_string()), (size, hash));

        Ok(())
    }

    fn close(&mut self) -> Result<(), DataError> {
        for ((key, locale), (size, hash)) in self.size_hash.get_mut().expect("poison") {
            writeln!(&mut self.fingerprints, "{key}, {locale}, {size}B, {hash:x}")?;
        }

        assert_eq!(
            self.rountrip_errors.get_mut().expect("poison"),
            &mut BTreeSet::default()
        );

        let total_violations = self
            .zero_copy_violations
            .get_mut()
            .expect("poison")
            .iter()
            .copied()
            .collect::<Vec<_>>();
        let net_violations = self
            .zero_copy_net_violations
            .get_mut()
            .expect("poison")
            .iter()
            .copied()
            .collect::<Vec<_>>();

        assert!(total_violations == EXPECTED_TOTAL_VIOLATIONS && net_violations == EXPECTED_NET_VIOLATIONS,
            "Expected violations list does not match found violations!\n\
            If the new list is smaller, please update EXPECTED_VIOLATIONS in make-testdata.rs\n\
            If it is bigger and that was unexpected, please make sure the key remains zero-copy, or ask ICU4X team members if it is okay\
            to temporarily allow for this key to be allowlisted.\n\
            Expected (net):\n{EXPECTED_NET_VIOLATIONS:?}\nFound (net):\n{net_violations:?}\nExpected (total):\n{EXPECTED_TOTAL_VIOLATIONS:?}\nFound (total):\n{total_violations:?}"
        );

        Ok(())
    }
}
