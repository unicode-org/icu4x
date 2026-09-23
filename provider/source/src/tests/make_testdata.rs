// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::SourceDataProvider;
use icu::locale::{DataLocale, data_locale};
use icu_provider::export::*;
use icu_provider::prelude::*;
use icu_provider_export::prelude::*;

include!("../../tests/locales.rs.data");

#[test]
#[cfg(feature = "use_wasm")]
fn make_testdata() {
    // Only produce output if the variable is set. Test is hermetic otherwise.
    if std::option_env!("ICU4X_WRITE_TESTDATA").is_none() {
        return;
    }

    simple_logger::SimpleLogger::new()
        .env()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();

    let mut exporter = icu_provider_export::fs_exporter::FilesystemExporter::try_new(
        Box::new(icu_provider_export::fs_exporter::serializers::Json::pretty()),
        {
            let mut options = icu_provider_export::fs_exporter::Options::default();
            options.root = "data/debug".into();
            options.overwrite = icu_provider_export::fs_exporter::OverwriteOption::RemoveAndReplace;
            options
        },
    )
    .unwrap();

    let provider = SourceDataProvider::new_testing();

    ExportDriver::new(
        LOCALES.iter().copied().map(DataLocaleFamily::single),
        DeduplicationStrategy::Maximal.into(),
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
        matches!(
            attrs.as_str().split('/').next_back().unwrap(),
            "CAD" | "EGP" | "EUR" | "GBP" | "USD"
        )
    })
    .with_marker_attributes_filter("locale_names_region", |attrs| {
        matches!(
            attrs.as_str(),
            "419" // part of dialect name
            | "FR" // standard
            | "CG" | "MM" // nested parens
            | "HK" // has alt name
        )
    })
    .with_marker_attributes_filter("locale_names_language", |attrs| {
        matches!(
            attrs.as_str(),
            "fr" // standard
            | "zh" // has short menu name
            | "en-GB" // has short name
            | "zh-Hant" // has long menu name but not short menu name
            | "de-CH" // has dialect name
            | "ku" // has menu attributes
        )
    })
    .with_marker_attributes_filter("locale_names_script", |attrs| {
        matches!(
            attrs.as_str(),
            "Latn" // standard
            | "Hans" | "Hant" // for contrast
            | "Cans" // has short script name
        )
    })
    .with_marker_attributes_filter("locale_names_variant", |attrs| {
        matches!(attrs.as_str(), "posix")
    })
    .with_marker_attributes_filter("numbering_system", |attrs| {
        matches!(attrs.as_str(), "arab" | "beng" | "cakm" | "latn" | "thai")
    })
    .with_marker_attributes_filter("transliterator", |attrs| {
        matches!(
            attrs.as_str(),
            "de-t-de-d0-ascii"
                | "el-latn-t-s0-ascii"
                | "el-latn-t-el-m0-bgn"
                | "und-arab-t-und-beng"
                | "und-latn-t-s0-ascii"
                | "und-t-d0-publish"
                | "und-t-s0-publish"
                | "und-t-und-latn-d0-ascii"
                | "und-x-bengali-interind"
                | "und-x-interind-arabic"
        )
    })
    .export(&provider, BorrowedExporter(&exporter))
    .unwrap();

    // Locales that only exist in the test data for a single marker: exporting them for
    // every marker would multiply the generated test data for no added coverage.
    ExportDriver::new(
        EXTRA_NUMBERS_LOCALES
            .iter()
            .copied()
            .map(DataLocaleFamily::single),
        DeduplicationStrategy::Maximal.into(),
        LocaleFallbacker::try_new_unstable(&provider).unwrap(),
    )
    .with_markers([
        icu::decimal::provider::DecimalSymbolsV1::INFO,
        icu::experimental::dimension::provider::currency::symbols::CurrencyDecimalSymbolsV1::INFO,
    ])
    .export(&provider, BorrowedExporter(&exporter))
    .unwrap();

    let _ = std::fs::remove_file("data/debug/currency/decimal/symbols/v1/.empty");

    exporter.close().unwrap();
}

/// Allows several [`ExportDriver`]s to write into the same exporter, which would
/// otherwise be consumed by the first export. Closing is left to the caller, once all
/// drivers have run.
struct BorrowedExporter<'a>(&'a dyn DataExporter);

impl DataExporter for BorrowedExporter<'_> {
    fn put_payload(
        &self,
        marker: DataMarkerInfo,
        id: DataIdentifierBorrowed,
        payload: &DataPayload<ExportMarker>,
    ) -> Result<(), DataError> {
        self.0.put_payload(marker, id, payload)
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
}
