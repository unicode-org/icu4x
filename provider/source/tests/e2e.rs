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

    ExportDriver::new(
        [DataLocaleFamily::FULL],
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
        blob_bytes.len() >= 5_000_000,
        "postcard blob size {} should be at least 5 MB",
        blob_bytes.len()
    );
}
