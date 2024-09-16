// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::SourceDataProvider;
use icu::locale::{langid, LanguageIdentifier};
use icu_provider::dynutil::UpcastDataPayload;
use icu_provider::export::*;
use icu_provider::prelude::*;
use icu_provider_export::prelude::*;
use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use std::collections::BTreeSet;
use std::sync::Mutex;

include!("../../tests/locales.rs.data");

#[test]
#[cfg(feature = "use_wasm")]
fn make_testdata() {
    // Only produce output if the variable is set. Test is hermetic otherwise.
    let exporter: Box<dyn DataExporter> = if std::option_env!("ICU4X_WRITE_TESTDATA").is_none() {
        Box::new(ZeroCopyCheckExporter {
            zero_copy_violations: Default::default(),
            zero_copy_transient_violations: Default::default(),
            rountrip_errors: Default::default(),
        })
    } else {
        simple_logger::SimpleLogger::new()
            .env()
            .with_level(log::LevelFilter::Info)
            .init()
            .unwrap();

        Box::new(MultiExporter::new(vec![
            Box::new(
                icu_provider_export::fs_exporter::FilesystemExporter::try_new(
                    Box::new(icu_provider_export::fs_exporter::serializers::Json::pretty()),
                    {
                        let mut options = icu_provider_export::fs_exporter::Options::default();
                        options.root = "data/debug".into();
                        options.overwrite =
                            icu_provider_export::fs_exporter::OverwriteOption::RemoveAndReplace;
                        options
                    },
                )
                .unwrap(),
            ),
            Box::new(ZeroCopyCheckExporter {
                zero_copy_violations: Default::default(),
                zero_copy_transient_violations: Default::default(),
                rountrip_errors: Default::default(),
            }),
        ]))
    };

    let provider = SourceDataProvider::new_testing();

    ExportDriver::new(
        LOCALES
            .iter()
            .cloned()
            .map(Into::into)
            .map(DataLocaleFamily::with_descendants),
        DeduplicationStrategy::None.into(),
        LocaleFallbacker::try_new_unstable(&provider).unwrap(),
    )
    .with_segmenter_models([
        "thaidict".into(),
        "Thai_codepoints_exclusive_model4_heavy".into(),
    ])
    .with_marker_attributes_filter("units", |attrs| {
        let (_length, unit) = attrs.as_str().split_once('-').unwrap();
        matches!(
            unit,
            "meter" | "foot" | "kilogram" | "pound" | "hour" | "minute" | "second"
        )
    })
    .with_marker_attributes_filter("currency", |attrs| {
        matches!(attrs.as_str(), "CAD" | "EGP" | "EUR" | "GBP" | "USD")
    })
    .export(&provider, exporter)
    .unwrap()
}

struct ZeroCopyCheckExporter {
    zero_copy_violations: Mutex<BTreeSet<DataMarkerInfo>>,
    zero_copy_transient_violations: Mutex<BTreeSet<DataMarkerInfo>>,
    rountrip_errors: Mutex<BTreeSet<(DataMarkerInfo, String)>>,
}

// Types in this list cannot be zero-copy deserialized.
//
// Such types contain some data that was allocated during deserializations
//
// Every entry in this list is a bug that needs to be addressed before stabilization.
const EXPECTED_VIOLATIONS: &[DataMarkerInfo] = &[];

// Types in this list can be zero-copy deserialized (and do not contain allocated data),
// however there is some allocation that occurs during deserialization for validation.
//
// Entries in this list represent a less-than-ideal state of things, however ICU4X is shippable with violations
// in this list since it does not affect databake.
const EXPECTED_TRANSIENT_VIOLATIONS: &[DataMarkerInfo] = &[
    // Regex DFAs need to be validated, which involved creating a BTreeMap.
    // If required we could avoid this using one of the approaches in
    // https://github.com/unicode-org/icu4x/pulls/3697.
    icu::list::provider::AndListV2Marker::INFO,
    icu::list::provider::OrListV2Marker::INFO,
    icu::list::provider::UnitListV2Marker::INFO,
];

impl DataExporter for ZeroCopyCheckExporter {
    fn put_payload(
        &self,
        marker: DataMarkerInfo,
        id: DataIdentifierBorrowed,
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

        let buffer_payload = DataPayload::from_owned_buffer(serialized.into_boxed_slice());

        MeasuringAllocator::start_measure();

        let allocated;
        let deallocated;
        let payload_after;

        macro_rules! cb {
            ($($marker:path = $path:literal,)+ #[experimental] $($emarker:path = $epath:literal,)+) => {
                ((allocated, deallocated), payload_after) = match marker {
                    k if k == icu_provider::hello_world::HelloWorldV1Marker::INFO => {
                        let deserialized: DataPayload<icu_provider::hello_world::HelloWorldV1Marker> = buffer_payload.into_deserialized(icu_provider::buf::BufferFormat::Postcard1).unwrap();
                        (MeasuringAllocator::end_measure(), UpcastDataPayload::upcast(deserialized))
                    }
                    $(
                        k if k == <$marker>::INFO => {
                            let deserialized: DataPayload<$marker> = buffer_payload.into_deserialized(icu_provider::buf::BufferFormat::Postcard1).unwrap();
                            (MeasuringAllocator::end_measure(), UpcastDataPayload::upcast(deserialized))
                        }
                    )+
                    $(
                        k if k == <$emarker>::INFO => {
                            let deserialized: DataPayload<$emarker> = buffer_payload.into_deserialized(icu_provider::buf::BufferFormat::Postcard1).unwrap();
                            (MeasuringAllocator::end_measure(), UpcastDataPayload::upcast(deserialized))
                        }
                    )+
                    _ => unreachable!("unregistered marker {marker:?}")
                };
            }
        }
        icu_provider_registry::registry!(cb);

        if payload_before != &payload_after {
            self.rountrip_errors.lock().expect("poison").insert((
                marker,
                id.locale.to_string()
                    + if id.marker_attributes.is_empty() {
                        ""
                    } else {
                        "-x"
                    }
                    + id.marker_attributes.as_str(),
            ));
        }

        if deallocated != allocated {
            if !EXPECTED_VIOLATIONS.contains(&marker) {
                eprintln!("Zerocopy violation {marker:?} {id:?}: {allocated}B allocated, {deallocated}B deallocated");
            }
            self.zero_copy_violations
                .lock()
                .expect("poison")
                .insert(marker);
        } else if allocated > 0 {
            if !EXPECTED_TRANSIENT_VIOLATIONS.contains(&marker) {
                eprintln!("Transient zerocopy violation {marker:?} {id:?}: {allocated}B allocated/deallocated");
            }
            self.zero_copy_transient_violations
                .lock()
                .expect("poison")
                .insert(marker);
        }

        Ok(())
    }

    fn close(&mut self) -> Result<(), DataError> {
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
            If it is bigger and that was unexpected, please make sure the marker remains zero-copy, or ask ICU4X team members if it is okay \
            to temporarily allow for this marker to be allowlisted.\n\
            Common cause: did you forget to add `serde(borrow)` to all of the fields in your data struct?\n\
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
        static ACTIVE: Cell<bool> = const { Cell::new(false) };
        static TOTAL_ALLOCATED: Cell<u64> = const { Cell::new(0) };
        static TOTAL_DEALLOCATED: Cell<u64> = const { Cell::new(0) };
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
