// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::{BTreeMap, HashSet};
use std::path::Path;

use elsa::sync::FrozenMap;
use icu_datagen::prelude::*;
use icu_decimal::provider::DecimalSymbolsV1Marker;
use icu_provider::datagen::ExportMarker;
use icu_provider::prelude::*;
use postcard::ser_flavors::{AllocVec, Flavor};
use writeable::Writeable;

#[derive(Default)]
struct TestingExporter {
    data: FrozenMap<(DataKey, DataLocale), Vec<u8>>,
}

impl<'a> DataExporter for &'a mut TestingExporter {
    fn put_payload(
        &self,
        key: DataKey,
        locale: &DataLocale,
        payload: &DataPayload<ExportMarker>,
    ) -> Result<(), DataError> {
        let mut serializer = postcard::Serializer {
            output: AllocVec::new(),
        };
        payload.serialize(&mut serializer)?;
        let output = serializer
            .output
            .finalize()
            .expect("Failed to finalize serializer output");
        println!("Putting: {key}/{locale}");
        self.data.insert((key, locale.clone()), output);
        Ok(())
    }
}

impl TestingExporter {
    /// Returns a map of the data locales as BCP-47 strings to their buffers.
    pub fn take_map_and_reset(&mut self) -> BTreeMap<String, Vec<u8>> {
        core::mem::take(&mut self.data)
            .into_tuple_vec()
            .into_iter()
            .map(|((_data_key, data_locale), buffer)| {
                (data_locale.write_to_string().into_owned(), buffer)
            })
            .collect()
    }
}

#[test]
fn test_fallback_options() {
    simple_logger::SimpleLogger::new()
        .env()
        .with_level(log::LevelFilter::Trace)
        .init()
        .unwrap();

    let data_root = Path::new(concat!(core::env!("CARGO_MANIFEST_DIR"), "/tests/data/"));

    let provider = DatagenProvider::default()
        .with_cldr(data_root.join("cldr"))
        .unwrap()
        .with_icuexport(data_root.join("icuexport"))
        .unwrap();

    let mut testing_exporter = TestingExporter::default();

    let driver = DatagenDriver::new().with_keys([DecimalSymbolsV1Marker::KEY]);

    let explicit_locales: HashSet<LanguageIdentifier> = [
        langid!("arc"), // Aramaic, not in CLDR
        langid!("ar-EG"),
        langid!("en-GB"),
        langid!("es"),
        langid!("sr-ME"),
        langid!("ru-Cyrl-RU"),
    ]
    .into_iter()
    .collect();

    //
    // All+Hybrid
    //
    driver
        .clone()
        .with_all_locales()
        .with_fallback_mode(FallbackMode::Hybrid)
        .export(&provider, &mut testing_exporter)
        .unwrap();
    let data_all_hybrid = testing_exporter.take_map_and_reset();

    // These are all of the supported locales for DecimalSymbolsV1 in tests/data.
    let all_locales: Vec<&str> = vec![
        "ar",
        "ar-EG",
        "ar-EG-u-nu-latn",
        "ar-u-nu-latn",
        "bn",
        "bn-u-nu-latn",
        "ccp",
        "ccp-u-nu-latn",
        "en",
        "en-001",
        "en-ZA",
        "es",
        "es-AR",
        "fil",
        "fr",
        "ja",
        "ru",
        "sr",
        // "sr-Cyrl", (normalizes to 'sr')
        "sr-Latn",
        "th",
        "th-u-nu-thai",
        "tr",
        "und",
    ];

    // All+Hybrid should return exactly the supported locales set.
    itertools::assert_equal(data_all_hybrid.keys(), all_locales.iter());

    //
    // All+Runtime
    //

    driver
        .clone()
        .with_all_locales()
        .with_fallback_mode(FallbackMode::RuntimeManual)
        .export(&provider, &mut testing_exporter)
        .unwrap();
    let data_all_runtime = testing_exporter.take_map_and_reset();

    // These are all of the supported locales with deduplication applied.
    let all_locales_dedup: Vec<&str> = vec![
        "ar",
        // "ar-EG", (same as 'ar')
        // "ar-EG-u-nu-latn", (same as 'ar-u-nu-latn')
        "ar-u-nu-latn",
        "bn",
        "bn-u-nu-latn",
        "ccp",
        "ccp-u-nu-latn",
        // "en", (same as 'und')
        // "en-001", (same as 'und')
        // "en-ZA", (same as 'und')
        "es",
        "es-AR",
        // "fil", (same as 'und')
        "fr",
        // "ja", (same as 'und')
        "ru",
        "sr", // Note: 'sr' and 'sr-Latn' are the same, but they don't inherit
        "sr-Latn",
        // "th", (same as 'und')
        "th-u-nu-thai",
        "tr",
        "und",
    ];

    // Assert that the stated equivalences are correct
    assert_eq!(
        data_all_hybrid.get("ar-EG").unwrap(),
        data_all_hybrid.get("ar").unwrap()
    );
    assert_eq!(
        data_all_hybrid.get("ar-EG-u-nu-latn").unwrap(),
        data_all_hybrid.get("ar-u-nu-latn").unwrap()
    );
    assert_eq!(
        data_all_hybrid.get("en").unwrap(),
        data_all_hybrid.get("und").unwrap()
    );
    assert_eq!(
        data_all_hybrid.get("en-001").unwrap(),
        data_all_hybrid.get("und").unwrap()
    );
    assert_eq!(
        data_all_hybrid.get("en-ZA").unwrap(),
        data_all_hybrid.get("und").unwrap()
    );
    assert_eq!(
        data_all_hybrid.get("fil").unwrap(),
        data_all_hybrid.get("und").unwrap()
    );
    assert_eq!(
        data_all_hybrid.get("ja").unwrap(),
        data_all_hybrid.get("und").unwrap()
    );
    assert_eq!(
        data_all_hybrid.get("th").unwrap(),
        data_all_hybrid.get("und").unwrap()
    );

    // All+Runtime should return the supported locales set with deduplication.
    itertools::assert_equal(data_all_runtime.keys(), all_locales_dedup.iter());

    //
    // Explicit+Hybrid
    //

    driver
        .clone()
        .with_locales(explicit_locales.clone())
        .with_fallback_mode(FallbackMode::Hybrid)
        .export(&provider, &mut testing_exporter)
        .unwrap();
    let data_explicit_hybrid = testing_exporter.take_map_and_reset();

    // Explicit locales are "arc", "ar-EG", "en-GB", "es", "sr-ME", "ru-Cyrl-RU"
    let explicit_hybrid_locales: Vec<&str> = vec![
        "ar",              // ancestor of ar-EG
        "ar-EG",           // explicit locale
        "ar-EG-u-nu-latn", // descendant of ar-EG
        // "ar-u-nu-latn", // ??? should this be included?
        "arc",    // Aramaic, inheriting from und
        "en",     // ancestor of en-GB
        "en-001", // ancestor of en-GB
        "en-GB",  // explicit locale not in supported locales
        // "en-ZA", // not reachable
        "es",         // explicit and supported
        "es-AR",      // descendant of es
        "ru",         // ancestor of ru-Cyrl-RU
        "ru-Cyrl-RU", // explicit locale, even though it is not normalized
        // "sr", // not reachable from sr-ME
        "sr-Latn", // ancestor of sr-ME
        "sr-ME",   // explicit locale not in supported locales
        "und",     // ancestor of everything
    ];

    // Explicit+Hybrid should return the expanded explicit locales set above.
    itertools::assert_equal(data_explicit_hybrid.keys(), explicit_hybrid_locales.iter());

    //
    // Explicit+Runtime
    //

    driver
        .clone()
        .with_locales(explicit_locales.clone())
        .with_fallback_mode(FallbackMode::RuntimeManual)
        .export(&provider, &mut testing_exporter)
        .unwrap();
    let data_explicit_runtime = testing_exporter.take_map_and_reset();

    // Explicit locales are "arc", "ar-EG", "en-GB", "es", "sr-ME", "ru-Cyrl-RU"
    let explicit_hybrid_locales_dedup: Vec<&str> = vec![
        "ar",
        // "ar-Arab-EG", (same as 'ar')
        // "ar-EG", (same as 'ar')
        "ar-EG-u-nu-latn",
        // "arc", (same as 'und')
        // "en", (same as 'und')
        // "en-001", (same as 'und')
        // "en-GB", (same as 'und')
        "es",
        "es-AR",
        "ru",
        // "ru-Cyrl-RU", (same as 'ru')
        "sr-Latn",
        // "sr-ME", (same as 'sr-Latn')
        "und",
    ];

    // Explicit+Runtime should return the expanded then deduplicated explicit locales set above.
    itertools::assert_equal(
        data_explicit_runtime.keys(),
        explicit_hybrid_locales_dedup.iter(),
    );

    //
    // Explicit+Preresolved
    //

    driver
        .clone()
        .with_locales(explicit_locales.clone())
        .with_fallback_mode(FallbackMode::Preresolved)
        .export(&provider, &mut testing_exporter)
        .unwrap();
    let data_explicit_preresolved = testing_exporter.take_map_and_reset();

    // Explicit locales are "arc", "ar-EG", "en-GB", "es", "sr-ME", "ru-Cyrl-RU"
    let explicit_preresolved_locales: Vec<&str> = vec![
        "ar-EG",
        "ar-EG-u-nu-latn", // extensions included even in preresolved mode
        "arc",
        "en-GB",
        "es",
        "ru-Cyrl-RU",
        "sr-ME",
    ];

    // Explicit+Preresolved should return the exact explicit locales set.
    itertools::assert_equal(
        data_explicit_preresolved.keys(),
        explicit_preresolved_locales.iter(),
    );
}
