// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu::datetime::{NoCalendarFormatter, fieldsets};
use icu_provider::prelude::AsDeserializingBufferProvider;
use icu_provider_adapters::difference::DifferenceDataProvider;
use icu_provider_adapters::fallback::LocaleFallbackProvider;
use icu_provider_adapters::fork::ForkByErrorProvider;
use icu_provider_adapters::fork::predicates::IdentifierNotFoundPredicate;
use icu_provider_blob::BlobDataProvider;
use icu_provider_export::blob_exporter::BlobExporter;
use icu_provider_export::prelude::*;
use icu_provider_source::{AltVariantKind, SourceDataProvider};

fn main() {
    let source = SourceDataProvider::new();

    // Export the difference caused by `AltVariantKind::DatetimeAscii` into an in-memory blob.
    let alt_overlay_blob = {
        let mut blob = Vec::new();
        ExportDriver::new(
            [DataLocaleFamily::FULL],
            // `source` contains all locales explicitly, so the overlay must not rely
            // on fallback either, otherwise a deduplicated locale would be served
            // from `source` instead of falling back within the overlay.
            DeduplicationStrategy::None.into(),
            LocaleFallbacker::try_new_unstable(&source).unwrap(),
        )
        .export(
            &DifferenceDataProvider::new(
                source
                    .clone()
                    .with_alt_variants([AltVariantKind::DatetimeAscii]),
                source.clone(),
            ),
            BlobExporter::new_with_sink(Box::new(&mut blob)),
        )
        .unwrap();
        blob
    };

    assert!(alt_overlay_blob.len() < 15000, "{}", alt_overlay_blob.len());

    let alt_overlay =
        BlobDataProvider::try_new_from_blob(alt_overlay_blob.into_boxed_slice()).unwrap();

    // Reassemble `alt` by layering the difference on top of `source`.
    let alt = ForkByErrorProvider::new_with_predicate(
        alt_overlay.as_deserializing(),
        source.clone(),
        IdentifierNotFoundPredicate,
    );

    let fallbacker = LocaleFallbacker::try_new_unstable(&alt).unwrap();
    let alt = LocaleFallbackProvider::new(alt, fallbacker);

    let fallbacker = LocaleFallbacker::try_new_unstable(&source).unwrap();
    let source = LocaleFallbackProvider::new(source, fallbacker);

    let time = icu::time::Time::try_new(15, 30, 0, 0).unwrap();

    // CLDR uses U+202F NARROW NO-BREAK SPACE before the day period in `en`...
    assert_eq!(
        NoCalendarFormatter::try_new_unstable(
            &source,
            icu::locale::locale!("en").into(),
            fieldsets::T::short()
        )
        .unwrap()
        .format(&time)
        .to_string(),
        "3:30:00\u{202f}PM",
    );

    // ...while the ASCII alt variant uses a regular space.
    assert_eq!(
        NoCalendarFormatter::try_new_unstable(
            &alt,
            icu::locale::locale!("en").into(),
            fieldsets::T::short()
        )
        .unwrap()
        .format(&time)
        .to_string(),
        "3:30:00 PM",
    );
}
