// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider_export::blob_exporter::BlobExporter;
use icu_provider_export::prelude::*;
use icu_provider_source::SourceDataProvider;

#[test]
fn test_export_language_identifier_display_names() {
    let provider = SourceDataProvider::new();
    let mut blob_bytes = Vec::new();
    let exporter = BlobExporter::new_with_sink(Box::new(&mut blob_bytes));

    let modern_locales = provider
        .locales_for_coverage_levels([icu_provider_source::CoverageLevel::Modern])
        .unwrap();

    ExportDriver::new(
        modern_locales
            .into_iter()
            .map(|loc| DataLocaleFamily::without_descendants(loc))
            // for the purposes of this test, use every 10th locale
            .step_by(10),
        DeduplicationStrategy::None.into(),
        LocaleFallbacker::try_new_unstable(&provider).unwrap(),
    )
    .with_markers(
        icu::experimental::provider::MARKERS
            .iter()
            .copied()
            .filter(|info| info.id.name().starts_with("LocaleNames")),
    )
    .export(&provider, exporter)
    .unwrap();

    assert!(
        blob_bytes.len() >= 100_000,
        "postcard blob size {} should be at least 100 kB",
        blob_bytes.len()
    );
}
