// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

extern crate icu_provider_export;

use icu_provider::dynutil::UpcastDataPayload;
use icu_provider::export::*;
use icu_provider::prelude::*;
use icu_provider_export::baked_exporter;
use icu_provider_export::prelude::*;
use icu_provider_source::{CoverageLevel, SourceDataProvider};
use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;

const REPO_VERSION: &str = "version.workspace = true";

const COMPONENTS: &[(&str, &[DataMarkerInfo], &str)] = &[
    ("calendar", icu::calendar::provider::MARKERS, REPO_VERSION),
    ("casemap", icu::casemap::provider::MARKERS, REPO_VERSION),
    ("collator", icu::collator::provider::MARKERS, REPO_VERSION),
    ("datetime", icu::datetime::provider::MARKERS, REPO_VERSION),
    ("decimal", icu::decimal::provider::MARKERS, REPO_VERSION),
    ("list", icu::list::provider::MARKERS, REPO_VERSION),
    ("locale", icu::locale::provider::MARKERS, REPO_VERSION),
    (
        "locale_fallback",
        icu::locale::fallback::provider::MARKERS,
        REPO_VERSION,
    ),
    (
        "normalizer",
        icu::normalizer::provider::MARKERS,
        REPO_VERSION,
    ),
    ("plurals", icu::plurals::provider::MARKERS, REPO_VERSION),
    (
        "properties",
        icu::properties::provider::MARKERS,
        REPO_VERSION,
    ),
    ("segmenter", icu::segmenter::provider::MARKERS, REPO_VERSION),
    ("time", icu::time::provider::MARKERS, REPO_VERSION),
    (
        "experimental",
        icu::experimental::provider::MARKERS,
        r#"version = "0.6.0""#,
    ),
];

fn main() {
    simple_logger::SimpleLogger::new()
        .env()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();

    let args = std::env::args().skip(1).collect::<Vec<_>>();

    let components = if args.is_empty() {
        COMPONENTS
            .iter()
            .map(|(krate, markers, version)| (krate.to_string(), *markers, *version))
            .collect::<Vec<_>>()
    } else {
        let map =
            std::collections::HashMap::<&str, (&'static [DataMarkerInfo], &'static str)>::from_iter(
                COMPONENTS
                    .iter()
                    .map(|(krate, markers, version)| (*krate, (*markers, *version))),
            );
        args.into_iter()
            .filter_map(|krate| {
                map.get(krate.as_str())
                    .map(|(markers, version)| (krate, *markers, *version))
            })
            .collect()
    };

    let source = SourceDataProvider::new()
        .with_tzdb(Path::new("provider/source/tests/data/tzdb"))
        .unwrap();

    let driver = ExportDriver::new(
        source
            .locales_for_coverage_levels([
                CoverageLevel::Modern,
                CoverageLevel::Moderate,
                CoverageLevel::Basic,
            ])
            .unwrap()
            .into_iter()
            .map(DataLocaleFamily::with_descendants),
        DeduplicationStrategy::Maximal.into(),
        LocaleFallbacker::try_new_unstable(&source).unwrap(),
    )
    .with_recommended_segmenter_models();

    let mut options = baked_exporter::Options::default();
    options.overwrite = true;
    options.pretty = true;

    let zero_copy_check_exporter = Box::leak(Box::new(ZeroCopyCheckExporter {
        zero_copy_violations: Default::default(),
        zero_copy_transient_violations: Default::default(),
        rountrip_errors: Default::default(),
    }));

    for (component, markers, version) in &components {
        let path = Path::new("provider/data").join(component);

        let _ = std::fs::remove_dir_all(&path);
        for dir in ["", "src", "data"] {
            std::fs::create_dir(path.join(dir)).unwrap();
        }
        for (file, template) in [
            ("build.rs", include_str!("../template/build.rs.template")),
            (
                "Cargo.toml",
                include_str!("../template/Cargo.toml.template"),
            ),
            ("LICENSE", include_str!("../template/LICENSE.template")),
            ("README.md", include_str!("../template/README.md.template")),
            (
                "src/lib.rs",
                include_str!("../template/src/lib.rs.template"),
            ),
        ] {
            std::fs::write(
                path.join(file),
                template
                    .replace("_component_", component)
                    .replace("_version_", version)
                    .replace("_unicode_tag_", SourceDataProvider::TESTED_UNICODE_TAG)
                    .replace("_cldr_tag_", SourceDataProvider::TESTED_CLDR_TAG)
                    .replace(
                        "_segmenter_lstm_tag_",
                        SourceDataProvider::TESTED_SEGMENTER_LSTM_TAG,
                    ),
            )
            .unwrap();
        }

        let baked_exporter =
            baked_exporter::BakedExporter::new(path.join("data"), options).unwrap();
        let stub_exporter = StubExporter(
            baked_exporter::BakedExporter::new(path.join("stubdata"), options).unwrap(),
        );

        let fingerprinter = StatisticsExporter::default();

        let export_metdata = driver
            .clone()
            .with_markers(markers.iter().copied())
            .export(
                &source,
                MultiExporter::new(vec![
                    Box::new(baked_exporter),
                    Box::new(stub_exporter),
                    Box::new(fingerprinter),
                    Box::new(&*zero_copy_check_exporter),
                ]),
            )
            .unwrap();

        let mut export_metadatas = export_metdata
            .exporter
            .0
            .unwrap()
            .downcast::<Vec<Option<Box<dyn core::any::Any>>>>()
            .unwrap();

        let baked_metadata = export_metadatas[0]
            .take()
            .unwrap()
            .downcast::<icu_provider_export::baked_exporter::BakedExporterCloseMetadata>()
            .unwrap();

        let fingerprint_metadata = export_metadatas[2]
            .take()
            .unwrap()
            .downcast::<HashMap<DataMarkerInfo, Statistics>>()
            .unwrap();

        let mut lines = Vec::new();

        for (marker, data) in fingerprint_metadata.into_iter() {
            let mut marker_debug_path = String::new();
            for (index, c) in format!("{marker:?}").char_indices() {
                if c.is_ascii_uppercase() && index > 0 {
                    marker_debug_path.push('/');
                }
                marker_debug_path.push(c.to_ascii_lowercase());
            }

            if marker.is_singleton {
                let ((baked_struct_size, postcard_struct_size), hash) =
                    data.size_hash[&Default::default()];
                lines.push(format!(
                    "{marker_debug_path}, <singleton>, {baked_struct_size}B, {postcard_struct_size}B, {hash:x}"
                ));
            } else {
                let postcard_structs_size = data
                    .struct_sizes
                    .values()
                    .map(|(_, postcard)| postcard)
                    .sum::<usize>();
                let baked_structs_size = data
                    .struct_sizes
                    .values()
                    .map(|(baked, _)| baked)
                    .sum::<usize>();

                let baked_exporter::Statistics {
                    structs_count,
                    lookup_struct_size,
                    identifiers_count,
                    ..
                } = &baked_metadata.statistics[&marker];

                lines.push(format!(
                    "{marker_debug_path}, <total>, {baked_structs_size}B, {postcard_structs_size}B, {structs_count} unique payloads",
                ));
                lines.push(format!(
                    "{marker_debug_path}, <lookup>, {lookup_struct_size}B, {identifiers_count} identifiers",
                ));

                let mut seen = HashMap::new();
                for (id, ((baked_size, postcard_size), hash)) in data
                    .size_hash
                    .into_iter()
                    .map(|(id, v)| {
                        (
                            if !id.marker_attributes.is_empty() {
                                format!(
                                    "{locale}/{marker_attributes}",
                                    locale = id.locale,
                                    marker_attributes = id.marker_attributes.as_str(),
                                )
                            } else {
                                id.locale.to_string()
                            },
                            v,
                        )
                    })
                    .collect::<BTreeMap<_, _>>()
                {
                    if let Some(deduped_req) = seen.get(&hash) {
                        lines.push(format!("{marker_debug_path}, {id}, -> {deduped_req}",));
                    } else {
                        lines.push(format!(
                            "{marker_debug_path}, {id}, {baked_size}B, {postcard_size}B, {hash:x}",
                        ));
                        seen.insert(hash, id.clone());
                    }
                }
            }
        }

        lines.sort();
        let mut out = crlify::BufWriterWithLineEndingFix::new(
            File::create(path.join("fingerprints.csv")).unwrap(),
        );
        for line in lines {
            writeln!(&mut out, "{line}").unwrap();
        }
    }

    if components.len() == COMPONENTS.len() {
        // On full datagen runs (as in CI) validate that `--markers all --locales full` works
        ExportDriver::new(
            [DataLocaleFamily::FULL],
            DeduplicationStrategy::Maximal.into(),
            LocaleFallbacker::try_new_unstable(&source).unwrap(),
        )
        // These are all supported models
        .with_recommended_segmenter_models()
        .export(&source, &*zero_copy_check_exporter)
        .unwrap();
    }

    zero_copy_check_exporter.check();
}

struct StubExporter<E>(E);

impl<E: DataExporter> DataExporter for StubExporter<E> {
    fn put_payload(
        &self,
        marker: DataMarkerInfo,
        id: DataIdentifierBorrowed,
        payload: &DataPayload<ExportMarker>,
    ) -> Result<(), DataError> {
        if (id.locale.is_unknown()
            || marker == icu::segmenter::provider::SegmenterBreakLineOverrideV2::INFO)
            && marker.expose_baked_consts
        {
            self.0.put_payload(marker, id, payload)
        } else {
            Ok(())
        }
    }

    fn flush_singleton(
        &self,
        marker: DataMarkerInfo,
        payload: &DataPayload<ExportMarker>,
        metadata: FlushMetadata,
    ) -> Result<(), DataError> {
        self.0.flush_singleton(marker, payload, metadata)
    }

    fn flush(&self, marker: DataMarkerInfo, metadata: FlushMetadata) -> Result<(), DataError> {
        self.0.flush(marker, metadata)
    }

    fn close(&mut self) -> Result<ExporterCloseMetadata, DataError> {
        self.0.close()
    }
}

#[derive(Default)]
struct StatisticsExporter {
    data: Mutex<HashMap<DataMarkerInfo, Statistics>>,
}

#[derive(Default)]
struct Statistics {
    size_hash: HashMap<DataIdentifierCow<'static>, ((usize, usize), u64)>,
    struct_sizes: HashMap<u64, (usize, usize)>,
    identifiers: HashSet<DataIdentifierCow<'static>>,
}

impl DataExporter for StatisticsExporter {
    fn put_payload(
        &self,
        marker: DataMarkerInfo,
        id: DataIdentifierBorrowed,
        payload: &DataPayload<ExportMarker>,
    ) -> Result<(), DataError> {
        let baked_size = payload.baked_size();

        // We're using SipHash, which is deprecated, but we want a stable hasher
        // (we're fine with it not being cryptographically secure since we're just using it to track diffs)
        #[allow(deprecated)]
        use std::hash::{Hasher, SipHasher};
        #[allow(deprecated)]
        let mut hasher = SipHasher::new();
        let postcard_size = payload.hash_and_postcard_size(&mut hasher);
        let hash = hasher.finish();

        let mut data = self.data.lock().expect("poison");
        let data = data.entry(marker).or_default();
        data.size_hash
            .insert(id.into_owned(), ((baked_size, postcard_size), hash));
        data.struct_sizes.insert(hash, (baked_size, postcard_size));
        data.identifiers.insert(id.into_owned());

        Ok(())
    }

    fn close(&mut self) -> Result<ExporterCloseMetadata, DataError> {
        Ok(ExporterCloseMetadata(Some(Box::new(core::mem::take(
            self.data.get_mut().expect("poison"),
        )))))
    }
}

struct ZeroCopyCheckExporter {
    zero_copy_violations: Mutex<BTreeSet<DataMarkerInfo>>,
    zero_copy_transient_violations: Mutex<BTreeSet<DataMarkerInfo>>,
    rountrip_errors: Mutex<BTreeMap<DataMarkerInfo, BTreeSet<String>>>,
}

impl DataExporter for &'_ ZeroCopyCheckExporter {
    fn put_payload(
        &self,
        marker: DataMarkerInfo,
        id: DataIdentifierBorrowed,
        payload_before: &DataPayload<ExportMarker>,
    ) -> Result<(), DataError> {
        use postcard::{
            Serializer,
            ser_flavors::{AllocVec, Flavor},
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
            ($($marker_ty:ty:$marker:ident,)+ #[unstable] $($emarker_ty:ty:$emarker:ident,)+) => {
                ((allocated, deallocated), payload_after) = match marker {
                    k if k == icu_provider::hello_world::HelloWorldV1::INFO => {
                        let deserialized: DataPayload<icu_provider::hello_world::HelloWorldV1> = buffer_payload.into_deserialized(icu_provider::buf::BufferFormat::Postcard1).unwrap();
                        (MeasuringAllocator::end_measure(), UpcastDataPayload::upcast(deserialized))
                    }
                    $(
                        k if k == <$marker_ty>::INFO => {
                            let deserialized: DataPayload<$marker_ty> = buffer_payload.into_deserialized(icu_provider::buf::BufferFormat::Postcard1).unwrap();
                            (MeasuringAllocator::end_measure(), UpcastDataPayload::upcast(deserialized))
                        }
                    )+
                    $(
                        k if k == <$emarker_ty>::INFO => {
                            let deserialized: DataPayload<$emarker_ty> = buffer_payload.into_deserialized(icu_provider::buf::BufferFormat::Postcard1).unwrap();
                            (MeasuringAllocator::end_measure(), UpcastDataPayload::upcast(deserialized))
                        }
                    )+
                    _ => unreachable!("unregistered marker {marker:?}")
                };
            }
        }
        icu_provider_registry::registry!(cb);

        if payload_before != &payload_after {
            self.rountrip_errors
                .lock()
                .expect("poison")
                .entry(marker)
                .or_default()
                .insert(
                    id.locale.to_string()
                        + if id.marker_attributes.is_empty() {
                            ""
                        } else {
                            "-x"
                        }
                        + id.marker_attributes.as_str(),
                );
        }

        if deallocated != allocated {
            if !ZeroCopyCheckExporter::EXPECTED_VIOLATIONS.contains(&marker) {
                eprintln!(
                    "Zerocopy violation {marker:?} {id:?}: {allocated}B allocated, {deallocated}B deallocated"
                );
            }
            self.zero_copy_violations
                .lock()
                .expect("poison")
                .insert(marker);
        } else if allocated > 0 {
            if !ZeroCopyCheckExporter::EXPECTED_TRANSIENT_VIOLATIONS.contains(&marker) {
                eprintln!(
                    "Transient zerocopy violation {marker:?} {id:?}: {allocated}B allocated/deallocated"
                );
            }
            self.zero_copy_transient_violations
                .lock()
                .expect("poison")
                .insert(marker);
        }

        Ok(())
    }
}

impl ZeroCopyCheckExporter {
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
        icu::list::provider::ListOrV1::INFO,
        icu::list::provider::ListAndV1::INFO,
        icu::list::provider::ListUnitV1::INFO,
    ];

    const EXPECTED_ROUNDTRIP_VIOLATIONS: &[DataMarkerInfo] = &[
        // These serialize to a different variant for stability
        icu::datetime::provider::names::DatetimeNamesMonthDangiV1::INFO,
        icu::datetime::provider::names::DatetimeNamesMonthHebrewV1::INFO,
        icu::datetime::provider::names::DatetimeNamesMonthChineseV1::INFO,
        icu::datetime::provider::names::DatetimeNamesYearJapaneseV1::INFO,
    ];

    fn check(&self) {
        let rountrip_errors = self
            .rountrip_errors
            .lock()
            .expect("poison")
            .keys()
            .copied()
            .collect::<Vec<_>>();

        assert_eq!(rountrip_errors, Self::EXPECTED_ROUNDTRIP_VIOLATIONS);

        let violations = self
            .zero_copy_violations
            .lock()
            .expect("poison")
            .iter()
            .copied()
            .collect::<Vec<_>>();

        let transient_violations = self
            .zero_copy_transient_violations
            .lock()
            .expect("poison")
            .iter()
            .copied()
            .collect::<Vec<_>>();

        assert!(
            transient_violations == Self::EXPECTED_TRANSIENT_VIOLATIONS
                && violations == Self::EXPECTED_VIOLATIONS,
            "Expected violations list does not match found violations!\n\
            If the new list is smaller, please update EXPECTED_VIOLATIONS in make-testdata.rs\n\
            If it is bigger and that was unexpected, please make sure the marker remains zero-copy, or ask ICU4X team members if it is okay \
            to temporarily allow for this marker to be allowlisted.\n\
            Common cause: did you forget to add `serde(borrow)` to all of the fields in your data struct?\n\
            Expected:\n{:?}\nFound:\n{violations:?}\nExpected (transient):\n{:?}\nFound (transient):\n{transient_violations:?}",
            Self::EXPECTED_VIOLATIONS,
            Self::EXPECTED_TRANSIENT_VIOLATIONS,
        );
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

        // Safety: By our safety invariant.
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if Self::ACTIVE.with(|f| f.get()) {
            Self::TOTAL_DEALLOCATED.with(|c| c.set(c.get() + layout.size() as u64));
        }

        // Safety: By our safety invariant.
        unsafe { System.dealloc(ptr, layout) }
    }
}
