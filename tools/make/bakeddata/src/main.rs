// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

extern crate icu_provider_export;

use icu_provider::export::*;
use icu_provider::prelude::*;
use icu_provider_export::baked_exporter;
use icu_provider_export::prelude::*;
use icu_provider_source::{CoverageLevel, SourceDataProvider};
use std::collections::BTreeMap;
use std::collections::{HashMap, HashSet};
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
        "normalizer",
        icu::normalizer::provider::MARKERS,
        REPO_VERSION,
    ),
    ("plurals", icu::plurals::provider::MARKERS, REPO_VERSION),
    (
        "properties",
        icu::properties::provider::MARKERS,
        "version = \"2.1.2\"",
    ),
    ("segmenter", icu::segmenter::provider::MARKERS, REPO_VERSION),
    ("time", icu::time::provider::MARKERS, REPO_VERSION),
    (
        "experimental",
        icu::experimental::provider::MARKERS,
        r#"version = "0.4.0""#,
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
                    .replace("_cldr_tag_", SourceDataProvider::TESTED_CLDR_TAG)
                    .replace("_icuexport_tag_", SourceDataProvider::TESTED_ICUEXPORT_TAG)
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
        struct SinkExporter;
        impl DataExporter for SinkExporter {
            fn put_payload(
                &self,
                _marker: DataMarkerInfo,
                _id: DataIdentifierBorrowed,
                _payload: &DataPayload<ExportMarker>,
            ) -> Result<(), DataError> {
                Ok(())
            }
        }
        ExportDriver::new(
            [DataLocaleFamily::FULL],
            DeduplicationStrategy::Maximal.into(),
            LocaleFallbacker::try_new_unstable(&source).unwrap(),
        )
        // These are all supported models
        .with_recommended_segmenter_models()
        .export(&source, SinkExporter)
        .unwrap();
    }
}

struct StubExporter<E>(E);

impl<E: DataExporter> DataExporter for StubExporter<E> {
    fn put_payload(
        &self,
        marker: DataMarkerInfo,
        id: DataIdentifierBorrowed,
        payload: &DataPayload<ExportMarker>,
    ) -> Result<(), DataError> {
        if id.locale.is_unknown() && marker.expose_baked_consts {
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
