// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

extern crate icu_datagen;

use icu_datagen::baked_exporter;
use icu_datagen::fs_exporter;
use icu_datagen::fs_exporter::serializers::AbstractSerializer;
use icu_datagen::prelude::*;
use icu_datagen_bikeshed::{CoverageLevel, DatagenProvider};
use icu_provider::export::*;
use icu_provider::prelude::*;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;

const REPO_VERSION: &str = env!("CARGO_PKG_VERSION");
const EXPERIMENTAL_VERSION: &str = "0.1.0";
const COMPONENTS: &[(&str, &[DataMarkerInfo], &str)] = &[
    ("calendar", icu::calendar::provider::MARKERS, REPO_VERSION),
    ("casemap", icu::casemap::provider::MARKERS, REPO_VERSION),
    ("collator", icu::collator::provider::MARKERS, REPO_VERSION),
    ("datetime", icu::datetime::provider::MARKERS, REPO_VERSION),
    ("decimal", icu::decimal::provider::MARKERS, REPO_VERSION),
    ("list", icu::list::provider::MARKERS, REPO_VERSION),
    ("locale", icu::locale::provider::MARKERS, REPO_VERSION),
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
    ("timezone", icu::timezone::provider::MARKERS, REPO_VERSION),
    (
        "experimental",
        icu::experimental::provider::MARKERS,
        EXPERIMENTAL_VERSION,
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

    let source = DatagenProvider::new_latest_tested();

    let driver = DatagenDriver::new(
        source
            .locales_for_coverage_levels([
                CoverageLevel::Modern,
                CoverageLevel::Moderate,
                CoverageLevel::Basic,
            ])
            .unwrap()
            .into_iter()
            .map(LocaleFamily::with_descendants),
        FallbackOptions::maximal_deduplication(),
        LocaleFallbacker::try_new_unstable(&source).unwrap(),
    )
    .with_recommended_segmenter_models();

    let mut options = baked_exporter::Options::default();
    options.overwrite = true;
    options.pretty = true;

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
            ("LICENSE", include_str!("../LICENSE")),
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
                    .replace("_cldr_tag_", DatagenProvider::LATEST_TESTED_CLDR_TAG)
                    .replace(
                        "_icuexport_tag_",
                        DatagenProvider::LATEST_TESTED_ICUEXPORT_TAG,
                    )
                    .replace(
                        "_segmenter_lstm_tag_",
                        DatagenProvider::LATEST_TESTED_SEGMENTER_LSTM_TAG,
                    ),
            )
            .unwrap();
        }

        // Crates with non-singleton markers need fallback
        if markers.iter().any(|m| !m.is_singleton) {
            writeln!(
                &mut crlify::BufWriterWithLineEndingFix::new(
                    std::fs::OpenOptions::new()
                        .append(true)
                        .open(path.join("Cargo.toml"))
                        .unwrap()
                ),
                r#"icu_locale = {{ workspace = true, features = ["compiled_data"] }}"#
            )
            .unwrap();
            writeln!(
                &mut crlify::BufWriterWithLineEndingFix::new(
                    std::fs::OpenOptions::new()
                        .append(true)
                        .open(path.join("src/lib.rs"))
                        .unwrap()
                ),
                "pub use icu_locale;"
            )
            .unwrap();
        }

        let baked_exporter =
            baked_exporter::BakedExporter::new(path.join("data"), options).unwrap();
        let stub_exporter = StubExporter(
            baked_exporter::BakedExporter::new(path.join("stubdata"), options).unwrap(),
        );
        let fingerprinter = StatisticsExporter {
            size_hash: Default::default(),
            struct_sizes: Default::default(),
            identifiers: Default::default(),
            fingerprints: crlify::BufWriterWithLineEndingFix::new(
                File::create(path.join("fingerprints.csv")).unwrap(),
            ),
        };

        driver
            .clone()
            .with_markers(markers.iter().copied())
            .export(
                &source,
                MultiExporter::new(vec![
                    Box::new(baked_exporter),
                    Box::new(stub_exporter),
                    Box::new(fingerprinter),
                ]),
            )
            .unwrap();
    }
}

struct StubExporter<E>(E);

impl<E: DataExporter> DataExporter for StubExporter<E> {
    fn put_payload(
        &self,
        _marker: DataMarkerInfo,
        _id: DataIdentifierBorrowed,
        _payload: &DataPayload<ExportMarker>,
    ) -> Result<(), DataError> {
        Ok(())
    }

    fn flush_singleton(
        &self,
        marker: DataMarkerInfo,
        payload: &DataPayload<ExportMarker>,
    ) -> Result<(), DataError> {
        self.0.flush_singleton(marker, payload)
    }

    fn flush(&self, marker: DataMarkerInfo) -> Result<(), DataError> {
        self.0.flush(marker)
    }

    fn close(&mut self) -> Result<(), DataError> {
        self.0.close()
    }
}

struct StatisticsExporter<F> {
    size_hash: Mutex<BTreeMap<(DataMarkerInfo, String), (usize, u64)>>,
    struct_sizes: Mutex<BTreeMap<u64, usize>>,
    identifiers: Mutex<BTreeSet<String>>,
    fingerprints: F,
}

impl<F: Write + Send + Sync> DataExporter for StatisticsExporter<F> {
    fn put_payload(
        &self,
        marker: DataMarkerInfo,
        id: DataIdentifierBorrowed,
        payload_before: &DataPayload<ExportMarker>,
    ) -> Result<(), DataError> {
        let mut serialized = vec![];

        fs_exporter::serializers::Postcard::new(Default::default())
            .serialize(payload_before, &mut serialized)?;

        let size = serialized.len();

        // We're using SipHash, which is deprecated, but we want a stable hasher
        // (we're fine with it not being cryptographically secure since we're just using it to track diffs)
        #[allow(deprecated)]
        use std::hash::{Hash, Hasher, SipHasher};
        #[allow(deprecated)]
        let mut hasher = SipHasher::new();
        serialized.iter().for_each(|b| b.hash(&mut hasher));
        let hash = hasher.finish();

        self.size_hash.lock().expect("poison").insert(
            (
                marker,
                if marker.is_singleton && id.locale.is_und() {
                    "<singleton>".to_string()
                } else if !id.marker_attributes.is_empty() {
                    format!(
                        "{locale}/{marker_attributes}",
                        locale = id.locale,
                        marker_attributes = id.marker_attributes.as_str(),
                    )
                } else {
                    id.locale.to_string()
                },
            ),
            (size, hash),
        );

        self.struct_sizes.lock().expect("poison").insert(hash, size);
        self.identifiers
            .lock()
            .expect("poison")
            .insert(locale.to_string() + marker_attributes);

        Ok(())
    }

    fn flush(&self, _marker: DataMarkerInfo) -> Result<(), DataError> {
        Ok(())
    }

    fn close(&mut self) -> Result<(), DataError> {
        let identifiers = self.identifiers.lock().expect("poison");
        let size_hash = self.size_hash.lock().expect("poison");
        let struct_sizes = self.struct_sizes.lock().expect("poison");

        let identifiers_count = identifiers.len();
        let identifiers_size = identifiers.iter().map(String::len).sum::<usize>();
        writeln!(
            &mut self.fingerprints,
            "{identifiers_count} identifiers, {identifiers_size}B",
        )?;

        let structs_count = struct_sizes.len();
        let structs_size = struct_sizes.values().sum::<usize>();
        writeln!(
            &mut self.fingerprints,
            "{structs_count} structs, {structs_size}B",
        )?;

        writeln!(&mut self.fingerprints)?;

        let mut seen = std::collections::HashMap::new();
        for ((marker, req), (size, hash)) in size_hash.iter() {
            if let Some(deduped_req) = seen.get(hash) {
                writeln!(
                    &mut self.fingerprints,
                    "{marker:?}, {req}, {size}B, -> {deduped_req}",
                )?;
            } else {
                writeln!(
                    &mut self.fingerprints,
                    "{marker:?}, {req}, {size}B, {hash:x}",
                )?;
                seen.insert(hash, req);
            }
        }
        Ok(())
    }
}
