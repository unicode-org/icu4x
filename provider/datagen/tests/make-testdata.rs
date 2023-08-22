// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crlify::BufWriterWithLineEndingFix;
use icu_datagen::fs_exporter::serializers::Json;
use icu_datagen::fs_exporter::*;
use icu_datagen::prelude::*;
use icu_provider::datagen::*;
use icu_provider::prelude::*;
use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;

include!("locales.rs.data");

#[test]
fn generate_json_and_verify_postcard() {
    simple_logger::SimpleLogger::new()
        .env()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();

    let data_root = Path::new(concat!(core::env!("CARGO_MANIFEST_DIR"), "/tests/data/"));

    let source = DatagenProvider::default()
        .with_cldr(data_root.join("cldr"))
        .unwrap()
        .with_icuexport(data_root.join("icuexport"))
        .unwrap()
        .with_segmenter_lstm(data_root.join("lstm"))
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
        zero_copy_transient_violations: Default::default(),
        rountrip_errors: Default::default(),
        fingerprints: BufWriterWithLineEndingFix::new(
            File::create(data_root.join("postcard/fingerprints.csv")).unwrap(),
        ),
    });

    DatagenDriver::new()
        .with_keys(icu_datagen::all_keys())
        .with_locales(LOCALES.iter().cloned())
        .with_segmenter_models(vec![
            "thaidict".into(),
            "Thai_codepoints_exclusive_model4_heavy".into(),
        ])
        .export(&source, MultiExporter::new(vec![json_out, postcard_out]))
        .unwrap();
}

struct PostcardTestingExporter {
    size_hash: Mutex<BTreeMap<(DataKey, String), (usize, u64)>>,
    zero_copy_violations: Mutex<BTreeSet<DataKey>>,
    zero_copy_transient_violations: Mutex<BTreeSet<DataKey>>,
    rountrip_errors: Mutex<BTreeSet<(DataKey, String)>>,
    fingerprints: BufWriterWithLineEndingFix<File>,
}

// Types in this list cannot be zero-copy deserialized.
//
// Such types contain some data that was allocated during deserializations
//
// Every entry in this list is a bug that needs to be addressed before stabilization.
const EXPECTED_VIOLATIONS: &[DataKey] = &[
    // https://github.com/unicode-org/icu4x/issues/1678
    icu_datetime::provider::calendar::DateSkeletonPatternsV1Marker::KEY,
];

// Types in this list can be zero-copy deserialized (and do not contain allocated data),
// however there is some allocation that occurs during deserialization for validation.
//
// Entries in this list represent a less-than-ideal state of things, however ICU4X is shippable with violations
// in this list since it does not affect databake.
const EXPECTED_TRANSIENT_VIOLATIONS: &[DataKey] = &[
    // Regex DFAs need to be validated, which involved creating a BTreeMap.
    // If required we could avoid this using one of the approaches in
    // https://github.com/unicode-org/icu4x/pulls/3697.
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

        MeasuringAllocator::start_measure();

        let ((allocated, deallocated), payload_after) = icu_datagen::deserialize_and_measure(
            key,
            buffer_payload,
            MeasuringAllocator::end_measure,
        )
        .unwrap();

        if payload_before != &payload_after {
            self.rountrip_errors
                .lock()
                .expect("poison")
                .insert((key, locale.to_string()));
        }

        if deallocated != allocated {
            if !EXPECTED_VIOLATIONS.contains(&key) {
                log::warn!("Zerocopy violation {key} {locale}: {allocated}B allocated, {deallocated}B deallocated");
            }
            self.zero_copy_violations
                .lock()
                .expect("poison")
                .insert(key);
        } else if allocated > 0 {
            if !EXPECTED_TRANSIENT_VIOLATIONS.contains(&key) {
                log::warn!("Transient zerocopy violation {key} {locale}: {allocated}B allocated/deallocated");
            }
            self.zero_copy_transient_violations
                .lock()
                .expect("poison")
                .insert(key);
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

        let violations = self
            .zero_copy_violations
            .get_mut()
            .expect("poison")
            .iter()
            .copied()
            .collect::<Vec<_>>();

        let transient_violations = self
            .zero_copy_transient_violations
            .get_mut()
            .expect("poison")
            .iter()
            .copied()
            .collect::<Vec<_>>();

        assert!(transient_violations == EXPECTED_TRANSIENT_VIOLATIONS && violations == EXPECTED_VIOLATIONS,
            "Expected violations list does not match found violations!\n\
            If the new list is smaller, please update EXPECTED_VIOLATIONS in make-testdata.rs\n\
            If it is bigger and that was unexpected, please make sure the key remains zero-copy, or ask ICU4X team members if it is okay\
            to temporarily allow for this key to be allowlisted.\n\
            Expected:\n{EXPECTED_VIOLATIONS:?}\nFound:\n{violations:?}\nExpected (transient):\n{EXPECTED_TRANSIENT_VIOLATIONS:?}\nFound (transient):\n{transient_violations:?}"
        );

        Ok(())
    }
}

#[global_allocator]
static ALLOCATOR: MeasuringAllocator = MeasuringAllocator;

// Inspired by the assert_no_alloc crate
struct MeasuringAllocator;

impl MeasuringAllocator {
    // We need to track allocations on each thread independently
    thread_local! {
        static ACTIVE: Cell<bool> = Cell::new(false);
        static TOTAL_ALLOCATED: Cell<u64> = Cell::new(0);
        static TOTAL_DEALLOCATED: Cell<u64> = Cell::new(0);
    }

    pub fn start_measure() {
        Self::ACTIVE.with(|c| c.set(true));
    }

    pub fn end_measure() -> (u64, u64) {
        Self::ACTIVE.with(|c| c.set(false));
        (
            Self::TOTAL_ALLOCATED.with(|c| c.take()),
            Self::TOTAL_DEALLOCATED.with(|c| c.take()),
        )
    }
}

unsafe impl GlobalAlloc for MeasuringAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if Self::ACTIVE.with(|f| f.get()) {
            Self::TOTAL_ALLOCATED.with(|c| c.set(c.get() + layout.size() as u64));
        }
        System.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if Self::ACTIVE.with(|f| f.get()) {
            Self::TOTAL_DEALLOCATED.with(|c| c.set(c.get() + layout.size() as u64));
        }
        System.dealloc(ptr, layout)
    }
}
