// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(deprecated)] // remove in 2.0

use std::collections::BTreeMap;

use elsa::sync::FrozenMap;
use icu_datagen::prelude::*;
use icu_datagen::DeduplicationStrategy;
use icu_datagen::FallbackOptions;
use icu_locid_transform::provider::*;
use icu_provider::datagen::ExportMarker;
use icu_provider::datagen::*;
use icu_provider::hello_world::*;
use icu_provider::make_exportable_provider;
use icu_provider::prelude::*;
use postcard::ser_flavors::{AllocVec, Flavor};
use writeable::Writeable;

struct TestingProvider(BTreeMap<&'static str, &'static str>);

impl TestingProvider {
    fn new<const N: usize>(data: [(&'static str, &'static str); N]) -> Self {
        Self(BTreeMap::from_iter(data))
    }

    fn with_decimal_symbol_like_data() -> Self {
        Self::new([
            ("ar", "c3f15eb63fa35608"),
            ("ar-EG", "c3f15eb63fa35608"),
            ("ar-EG-u-nu-latn", "29e2dc764329c56"),
            ("ar-u-nu-latn", "29e2dc764329c56"),
            ("bn", "31828215dcef2fcb"),
            ("bn-u-nu-latn", "1be94084ee7dcfbf"),
            ("ccp", "c39715a84718596"),
            ("ccp-u-nu-latn", "1be94084ee7dcfbf"),
            ("en", "8df59f98704d3b0c"),
            ("en-001", "8df59f98704d3b0c"),
            ("en-ZA", "8df59f98704d3b0c"),
            ("es", "2c22710b06ef69b6"),
            ("es-AR", "3ec76252c7ed8d8c"),
            ("fil", "8df59f98704d3b0c"),
            ("fr", "bd076f44d0623175"),
            ("ja", "8df59f98704d3b0c"),
            ("ru", "8f773f51e85a65c1"),
            ("sr", "3ec76252c7ed8d8c"),
            ("sr-Latn", "3ec76252c7ed8d8c"),
            ("th", "8df59f98704d3b0c"),
            ("th-u-nu-thai", "db1d187d375ccfd2"),
            ("tr", "3ec76252c7ed8d8c"),
            ("und", "8df59f98704d3b0c"),
        ])
    }
}

impl DataProvider<HelloWorldV1Marker> for TestingProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<HelloWorldV1Marker>, DataError> {
        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(HelloWorldV1 {
                message: (*self
                    .0
                    .get(req.locale.to_string().as_str())
                    .ok_or(DataErrorKind::MissingLocale.into_error())?)
                .into(),
            })),
        })
    }
}

impl DataProvider<LocaleFallbackLikelySubtagsV1Marker> for TestingProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<LocaleFallbackLikelySubtagsV1Marker>, DataError> {
        icu_locid_transform::provider::Baked.load(req)
    }
}

impl DataProvider<LocaleFallbackParentsV1Marker> for TestingProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<LocaleFallbackParentsV1Marker>, DataError> {
        icu_locid_transform::provider::Baked.load(req)
    }
}

impl DataProvider<CollationFallbackSupplementV1Marker> for TestingProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<CollationFallbackSupplementV1Marker>, DataError> {
        icu_locid_transform::provider::Baked.load(req)
    }
}

impl IterableDataProvider<HelloWorldV1Marker> for TestingProvider {
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        Ok(self
            .0
            .keys()
            .map(|s| s.parse::<DataLocale>().unwrap())
            .collect())
    }
}

impl IterableDataProvider<LocaleFallbackLikelySubtagsV1Marker> for TestingProvider {
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        Ok(vec![Default::default()])
    }
}

impl IterableDataProvider<LocaleFallbackParentsV1Marker> for TestingProvider {
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        Ok(vec![Default::default()])
    }
}

impl IterableDataProvider<CollationFallbackSupplementV1Marker> for TestingProvider {
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        Ok(vec![Default::default()])
    }
}

make_exportable_provider!(
    TestingProvider,
    [
        HelloWorldV1Marker,
        LocaleFallbackLikelySubtagsV1Marker,
        LocaleFallbackParentsV1Marker,
        CollationFallbackSupplementV1Marker,
    ]
);

fn export_to_map(
    driver_1_4: DatagenDriver,
    driver_1_5: DatagenDriver,
    provider: &TestingProvider,
) -> BTreeMap<String, String> {
    #[derive(Default)]
    struct TestingExporter(FrozenMap<DataLocale, String>);

    impl DataExporter for &mut TestingExporter {
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
            self.0.insert(
                locale.clone(),
                postcard::from_bytes::<HelloWorldV1>(&output)
                    .unwrap()
                    .message
                    .to_string(),
            );
            Ok(())
        }
    }

    let _ = simple_logger::SimpleLogger::new()
        .env()
        .with_level(log::LevelFilter::Trace)
        .init();

    let mut exporter_1_4 = TestingExporter::default();
    driver_1_4.export(provider, &mut exporter_1_4).unwrap();

    let results_1_4 = exporter_1_4
        .0
        .into_tuple_vec()
        .into_iter()
        .map(|(data_locale, buffer)| (data_locale.write_to_string().into_owned(), buffer))
        .collect();

    let mut exporter_1_5 = TestingExporter::default();
    driver_1_5.export(provider, &mut exporter_1_5).unwrap();

    let results_1_5 = exporter_1_5
        .0
        .into_tuple_vec()
        .into_iter()
        .map(|(data_locale, buffer)| (data_locale.write_to_string().into_owned(), buffer))
        .collect();

    assert_eq!(results_1_4, results_1_5);

    results_1_4
}

#[test]
fn all_preferred() {
    let exported = export_to_map(
        DatagenDriver::new()
            .with_keys([HelloWorldV1Marker::KEY])
            .with_all_locales()
            .with_fallback_mode(FallbackMode::PreferredForExporter),
        &TestingProvider::with_decimal_symbol_like_data(),
    );

    // These are all of the supported locales.
    let locales = [
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

    // Should return exactly the supported locales set.
    assert_eq!(exported.keys().collect::<Vec<_>>(), locales);
}

#[test]
fn all_hybrid() {
    let exported = export_to_map(
        DatagenDriver::new()
            .with_keys([HelloWorldV1Marker::KEY])
            .with_all_locales()
            .with_fallback_mode(FallbackMode::Hybrid),
        DatagenDriver::new()
            .with_keys([HelloWorldV1Marker::KEY])
            .with_locales_and_fallback([LocaleFamily::full()], {
                let mut options = FallbackOptions::default();
                options.deduplication_strategy = Some(DeduplicationStrategy::NoDeduplication);
                options
            }),
        &TestingProvider::with_decimal_symbol_like_data(),
    );

    // These are all of the supported locales.
    let locales = [
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

    // Should return exactly the supported locales set.
    assert_eq!(exported.keys().collect::<Vec<_>>(), locales);
}

#[test]
fn all_runtime() {
    let exported = export_to_map(
        DatagenDriver::new()
            .with_keys([HelloWorldV1Marker::KEY])
            .with_all_locales()
            .with_fallback_mode(FallbackMode::RuntimeManual),
        DatagenDriver::new()
            .with_keys([HelloWorldV1Marker::KEY])
            .with_locales_and_fallback([LocaleFamily::full()], {
                let mut options = FallbackOptions::default();
                options.deduplication_strategy = Some(DeduplicationStrategy::Maximal);
                options
            }),
        &TestingProvider::with_decimal_symbol_like_data(),
    );

    // These are all of the supported locales with deduplication applied.
    let locales = [
        "ar",
        // "ar-EG", (same as 'ar')
        "ar-EG-u-nu-latn", // (same as 'ar-u-nu-latn' but DIFFERENT than 'ar-EG')
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

    // Should return the supported locales set with deduplication.
    assert_eq!(exported.keys().collect::<Vec<_>>(), locales);
}

#[test]
fn explicit_preferred() {
    let exported = export_to_map(
        DatagenDriver::new()
            .with_keys([HelloWorldV1Marker::KEY])
            .with_locales([
                langid!("arc"), // Aramaic, not in supported list
                langid!("ar-EG"),
                langid!("ar-SA"),
                langid!("en-GB"),
                langid!("es"),
                langid!("sr-ME"),
                langid!("ru-Cyrl-RU"),
            ])
            .with_fallback_mode(FallbackMode::PreferredForExporter),
        &TestingProvider::with_decimal_symbol_like_data(),
    );

    // Explicit locales are "arc", "ar-EG", "ar-SA", "en-GB", "es", "sr-ME", "ru-Cyrl-RU"
    let locales = [
        "ar",              // ancestor of ar-EG
        "ar-EG",           // explicit locale
        "ar-EG-u-nu-latn", // descendant of ar-EG
        "ar-SA",           // explicit locale, inheriting from ar
        "ar-SA-u-nu-latn", // extensions should be included (#4533)
        "ar-u-nu-latn",    // extensions should be included (#4533)
        "arc",             // Aramaic, inheriting from und
        "en",              // ancestor of en-GB
        "en-001",          // ancestor of en-GB
        "en-GB",           // explicit locale not in supported locales
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

    // Should return the expanded explicit locales set above.
    assert_eq!(exported.keys().collect::<Vec<_>>(), locales);
}

#[test]
fn explicit_hybrid() {
    const SELECTED_LOCALES: [LanguageIdentifier; 7] = [
        langid!("arc"), // Aramaic, not in supported list
        langid!("ar-EG"),
        langid!("ar-SA"),
        langid!("en-GB"),
        langid!("es"),
        langid!("sr-ME"),
        langid!("ru-Cyrl-RU"),
    ];
    let exported = export_to_map(
        DatagenDriver::new()
            .with_keys([HelloWorldV1Marker::KEY])
            .with_locales(SELECTED_LOCALES)
            .with_fallback_mode(FallbackMode::Hybrid),
        DatagenDriver::new()
            .with_keys([HelloWorldV1Marker::KEY])
            .with_locales_and_fallback(
                SELECTED_LOCALES
                    .into_iter()
                    .map(LocaleFamily::with_descendants),
                {
                    let mut options = FallbackOptions::default();
                    options.deduplication_strategy = Some(DeduplicationStrategy::NoDeduplication);
                    options
                },
            ),
        &TestingProvider::with_decimal_symbol_like_data(),
    );

    // Explicit locales are "arc", "ar-EG", "ar-SA", "en-GB", "es", "sr-ME", "ru-Cyrl-RU"
    let locales = [
        "ar",              // ancestor of ar-EG
        "ar-EG",           // explicit locale
        "ar-EG-u-nu-latn", // descendant of ar-EG
        "ar-SA",           // explicit locale, inheriting from ar
        "ar-SA-u-nu-latn", // extensions should be included (#4533)
        "ar-u-nu-latn",    // extensions should be included (#4533)
        "arc",             // Aramaic, inheriting from und
        "en",              // ancestor of en-GB
        "en-001",          // ancestor of en-GB
        "en-GB",           // explicit locale not in supported locales
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

    // Should return the expanded explicit locales set above.
    assert_eq!(exported.keys().collect::<Vec<_>>(), locales);
}

#[test]
fn explicit_runtime() {
    const SELECTED_LOCALES: [LanguageIdentifier; 7] = [
        langid!("arc"), // Aramaic, not in supported list
        langid!("ar-EG"),
        langid!("ar-SA"),
        langid!("en-GB"),
        langid!("es"),
        langid!("sr-ME"),
        langid!("ru-Cyrl-RU"),
    ];
    let exported = export_to_map(
        DatagenDriver::new()
            .with_keys([HelloWorldV1Marker::KEY])
            .with_locales(SELECTED_LOCALES)
            .with_fallback_mode(FallbackMode::RuntimeManual),
        DatagenDriver::new()
            .with_keys([HelloWorldV1Marker::KEY])
            .with_locales_and_fallback(
                SELECTED_LOCALES
                    .into_iter()
                    .map(LocaleFamily::with_descendants),
                {
                    let mut options = FallbackOptions::default();
                    options.deduplication_strategy = Some(DeduplicationStrategy::Maximal);
                    options
                },
            ),
        &TestingProvider::with_decimal_symbol_like_data(),
    );

    // Explicit locales are "arc", "ar-EG", "ar-SA", "en-GB", "es", "sr-ME", "ru-Cyrl-RU"
    let locales = [
        "ar",
        // "ar-Arab-EG", (same as 'ar')
        // "ar-EG", (same as 'ar')
        "ar-EG-u-nu-latn",
        // "ar-SA", (same as 'ar')
        // "ar-SA-u-nu-latn", (same as 'ar-u-nu-latn')
        "ar-u-nu-latn",
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

    // Should return the expanded then deduplicated explicit locales set above.
    assert_eq!(exported.keys().collect::<Vec<_>>(), locales);
}

#[test]
fn explicit_preresolved() {
    const SELECTED_LOCALES: [LanguageIdentifier; 7] = [
        langid!("arc"), // Aramaic, not in supported list
        langid!("ar-EG"),
        langid!("ar-SA"),
        langid!("en-GB"),
        langid!("es"),
        langid!("sr-ME"),
        langid!("ru-Cyrl-RU"),
    ];
    let exported = export_to_map(
        DatagenDriver::new()
            .with_keys([HelloWorldV1Marker::KEY])
            .with_locales(SELECTED_LOCALES)
            .with_fallback_mode(FallbackMode::Preresolved),
        DatagenDriver::new()
            .with_keys([HelloWorldV1Marker::KEY])
            .with_locales_no_fallback(SELECTED_LOCALES, Default::default()),
        &TestingProvider::with_decimal_symbol_like_data(),
    );

    // Explicit locales are "arc", "ar-EG", "ar-SA", "en-GB", "es", "sr-ME", "ru-Cyrl-RU"
    let locales = [
        "ar-EG",
        "ar-EG-u-nu-latn", // extensions included even in preresolved mode
        "ar-SA",
        "ar-SA-u-nu-latn", // extensions included even in preresolved mode
        "arc",
        "en-GB",
        "es",
        "ru-Cyrl-RU",
        "sr-ME",
    ];

    // Should return the exact explicit locales set.
    assert_eq!(exported.keys().collect::<Vec<_>>(), locales);
}

#[test]
fn explicit_runtime_und() {
    let exported = export_to_map(
        DatagenDriver::new()
            .with_keys([HelloWorldV1Marker::KEY])
            .with_locales([langid!("und")])
            .with_fallback_mode(FallbackMode::RuntimeManual),
        DatagenDriver::new()
            .with_keys([HelloWorldV1Marker::KEY])
            .with_locales_and_fallback([LocaleFamily::with_descendants(langid!("und"))], {
                let mut options = FallbackOptions::default();
                options.deduplication_strategy = Some(DeduplicationStrategy::Maximal);
                options
            }),
        &TestingProvider::with_decimal_symbol_like_data(),
    );

    // Explicit locales are "und"
    let locales = ["und"];

    // Should return the exact explicit locales set.
    assert_eq!(exported.keys().collect::<Vec<_>>(), locales);
}

#[test]
fn explicit_hybrid_und() {
    let exported = export_to_map(
        DatagenDriver::new()
            .with_keys([HelloWorldV1Marker::KEY])
            .with_locales([langid!("und")])
            .with_fallback_mode(FallbackMode::Hybrid),
        DatagenDriver::new()
            .with_keys([HelloWorldV1Marker::KEY])
            .with_locales_and_fallback([LocaleFamily::with_descendants(langid!("und"))], {
                let mut options = FallbackOptions::default();
                options.deduplication_strategy = Some(DeduplicationStrategy::NoDeduplication);
                options
            }),
        &TestingProvider::with_decimal_symbol_like_data(),
    );

    // Explicit locales are "und"
    let locales = ["und"];

    // Should return the exact explicit locales set.
    assert_eq!(exported.keys().collect::<Vec<_>>(), locales);
}

#[test]
fn explicit_preresolved_und() {
    let exported = export_to_map(
        DatagenDriver::new()
            .with_keys([HelloWorldV1Marker::KEY])
            .with_locales([langid!("und")])
            .with_fallback_mode(FallbackMode::Preresolved),
        DatagenDriver::new()
            .with_keys([HelloWorldV1Marker::KEY])
            .with_locales_no_fallback([langid!("und")], Default::default()),
        &TestingProvider::with_decimal_symbol_like_data(),
    );

    // Explicit locales are "und"
    let locales = ["und"];

    // Should return the exact explicit locales set.
    assert_eq!(exported.keys().collect::<Vec<_>>(), locales);
}

#[test]
fn explicit_runtime_empty() {
    let exported = export_to_map(
        DatagenDriver::new()
            .with_keys([HelloWorldV1Marker::KEY])
            .with_locales([])
            .with_fallback_mode(FallbackMode::RuntimeManual),
        DatagenDriver::new()
            .with_keys([HelloWorldV1Marker::KEY])
            .with_locales_and_fallback([], {
                let mut options = FallbackOptions::default();
                options.deduplication_strategy = Some(DeduplicationStrategy::Maximal);
                options
            }),
        &TestingProvider::with_decimal_symbol_like_data(),
    );

    // Explicit locales are empty
    let locales = ["und"];

    // Should return the exact explicit locales set.
    assert_eq!(exported.keys().collect::<Vec<_>>(), locales);
}

#[test]
fn explicit_hybrid_empty() {
    let exported = export_to_map(
        DatagenDriver::new()
            .with_keys([HelloWorldV1Marker::KEY])
            .with_locales([])
            .with_fallback_mode(FallbackMode::Hybrid),
        DatagenDriver::new()
            .with_keys([HelloWorldV1Marker::KEY])
            .with_locales_and_fallback([], {
                let mut options = FallbackOptions::default();
                options.deduplication_strategy = Some(DeduplicationStrategy::NoDeduplication);
                options
            }),
        &TestingProvider::with_decimal_symbol_like_data(),
    );

    // Explicit locales are empty
    let locales = ["und"];

    // Should return the exact explicit locales set.
    assert_eq!(exported.keys().collect::<Vec<_>>(), locales);
}

#[test]
fn explicit_preresolved_empty() {
    let exported = export_to_map(
        DatagenDriver::new()
            .with_keys([HelloWorldV1Marker::KEY])
            .with_locales([])
            .with_fallback_mode(FallbackMode::Preresolved),
        DatagenDriver::new()
            .with_keys([HelloWorldV1Marker::KEY])
            .with_locales_no_fallback([], Default::default()),
        &TestingProvider::with_decimal_symbol_like_data(),
    );

    // Explicit locales are empty
    let locales: [&str; 0] = [];

    // Should return the exact explicit locales set.
    assert_eq!(exported.keys().collect::<Vec<_>>(), locales);
}
