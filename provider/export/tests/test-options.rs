// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[path = "testutil.rs"]
mod testutil;

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use icu_locale::{langid, LanguageIdentifier};
use icu_provider::export::*;
use icu_provider::hello_world::*;
use icu_provider::prelude::*;
use icu_provider_export::prelude::*;
use testutil::TestingExporter;

struct TestingProvider(BTreeMap<(&'static str, &'static str), &'static str>);

impl TestingProvider {
    fn new<const N: usize>(data: [((&'static str, &'static str), &'static str); N]) -> Self {
        Self(BTreeMap::from_iter(data))
    }

    fn with_decimal_symbol_like_data() -> Self {
        Self::new([
            (("ar", ""), "c3f15eb63fa35608"),
            (("ar-EG", ""), "c3f15eb63fa35608"),
            (("ar-EG", "latn"), "29e2dc764329c56"),
            (("ar", "latn"), "29e2dc764329c56"),
            (("bn", ""), "31828215dcef2fcb"),
            (("bn", "latn"), "1be94084ee7dcfbf"),
            (("ccp", ""), "c39715a84718596"),
            (("ccp", "latn"), "1be94084ee7dcfbf"),
            (("en", ""), "8df59f98704d3b0c"),
            (("en-001", ""), "8df59f98704d3b0c"),
            (("en-ZA", ""), "8df59f98704d3b0c"),
            (("es", ""), "2c22710b06ef69b6"),
            (("es-AR", ""), "3ec76252c7ed8d8c"),
            (("fil", ""), "8df59f98704d3b0c"),
            (("fr", ""), "bd076f44d0623175"),
            (("ja", ""), "8df59f98704d3b0c"),
            (("ru", ""), "8f773f51e85a65c1"),
            (("sr", ""), "3ec76252c7ed8d8c"),
            (("sr-Latn", ""), "3ec76252c7ed8d8c"),
            (("th", ""), "8df59f98704d3b0c"),
            (("th", "thai"), "db1d187d375ccfd2"),
            (("tr", ""), "3ec76252c7ed8d8c"),
            (("und", ""), "8df59f98704d3b0c"),
        ])
    }
}

impl DataProvider<HelloWorldV1Marker> for TestingProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<HelloWorldV1Marker>, DataError> {
        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(HelloWorldV1 {
                message: (*self
                    .0
                    .get(&(
                        req.id.locale.to_string().as_str(),
                        req.id.marker_attributes as &str,
                    ))
                    .ok_or(DataErrorKind::IdentifierNotFound.into_error())?)
                .into(),
            }),
        })
    }
}

impl IterableDataProvider<HelloWorldV1Marker> for TestingProvider {
    fn iter_ids(&self) -> Result<BTreeSet<DataIdentifierCow>, DataError> {
        Ok(self
            .0
            .keys()
            .map(|(l, a)| {
                DataIdentifierCow::from_owned(
                    DataMarkerAttributes::from_str_or_panic(a).to_owned(),
                    l.parse().unwrap(),
                )
            })
            .collect())
    }
}

make_exportable_provider!(TestingProvider, [HelloWorldV1Marker,]);

fn families(
    langids: impl IntoIterator<Item = LanguageIdentifier>,
) -> impl IntoIterator<Item = DataLocaleFamily> {
    langids
        .into_iter()
        .map(Into::into)
        .map(DataLocaleFamily::with_descendants)
}

fn export_to_map(driver: ExportDriver, provider: &TestingProvider) -> BTreeMap<String, Vec<u8>> {
    let _ = simple_logger::SimpleLogger::new()
        .env()
        .with_level(log::LevelFilter::Trace)
        .init();

    let mut exporter = TestingExporter::default();
    driver.export(provider, &mut exporter).unwrap();

    exporter.finish()
}

#[test]
fn all_preferred() {
    let exported = export_to_map(
        ExportDriver::new(
            [DataLocaleFamily::FULL],
            DeduplicationStrategy::None.into(),
            LocaleFallbacker::new().static_to_owned(),
        ),
        &TestingProvider::with_decimal_symbol_like_data(),
    );

    // These are all of the supported locales.
    let locales = [
        "ar",
        "ar-EG",
        "ar-EG/latn",
        "ar/latn",
        "bn",
        "bn/latn",
        "ccp",
        "ccp/latn",
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
        "th/thai",
        "tr",
        "und",
    ];

    // Should return exactly the supported locales set.
    assert_eq!(exported.keys().collect::<Vec<_>>(), locales);
}

#[test]
fn all_hybrid() {
    let exported = export_to_map(
        ExportDriver::new(
            [DataLocaleFamily::FULL],
            DeduplicationStrategy::None.into(),
            LocaleFallbacker::new().static_to_owned(),
        ),
        &TestingProvider::with_decimal_symbol_like_data(),
    );

    // These are all of the supported locales.
    let locales = [
        "ar",
        "ar-EG",
        "ar-EG/latn",
        "ar/latn",
        "bn",
        "bn/latn",
        "ccp",
        "ccp/latn",
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
        "th/thai",
        "tr",
        "und",
    ];

    // Should return exactly the supported locales set.
    assert_eq!(exported.keys().collect::<Vec<_>>(), locales);
}

#[test]
fn all_runtime() {
    let exported = export_to_map(
        ExportDriver::new(
            [DataLocaleFamily::FULL],
            DeduplicationStrategy::Maximal.into(),
            LocaleFallbacker::new().static_to_owned(),
        ),
        &TestingProvider::with_decimal_symbol_like_data(),
    );

    // These are all of the supported locales with deduplication applied.
    #[rustfmt::skip]
    let locales = [
        "ar",
        // "ar-EG", (same as 'ar')
        // "ar-EG/latn", (same as 'ar/latn')
        "ar/latn",
        "bn",
        "bn/latn",
        "ccp",
        "ccp/latn",
        // "en", (same as 'und')
        // "en-001", (same as 'und')
        // "en-ZA", (same as 'und')
        "es",
        "es-AR",
        // "fil", (same as 'und')
        "fr",
        // "ja", (same as 'und')
        "ru",
        "sr",
        // Note: 'sr' and 'sr-Latn' are the same, but they don't inherit
        "sr-Latn",
        // "th", (same as 'und')
        "th/thai",
        "tr",
        "und",
    ];

    // Should return the supported locales set with deduplication.
    assert_eq!(exported.keys().collect::<Vec<_>>(), locales);
}

#[test]
fn all_runtime_retain_base() {
    let exported = export_to_map(
        ExportDriver::new(
            [DataLocaleFamily::FULL],
            DeduplicationStrategy::RetainBaseLanguages.into(),
            LocaleFallbacker::new().static_to_owned(),
        ),
        &TestingProvider::with_decimal_symbol_like_data(),
    );

    // These are all of the supported locales with deduplication applied.
    #[rustfmt::skip]
    let locales = [
        "ar",
        // "ar-EG", (same as 'ar')
        // "ar-EG/latn", (same as 'ar/latn')
        "ar/latn",
        "bn",
        "bn/latn",
        "ccp",
        "ccp/latn",
        "en", // (same as 'und', but retained)
        // "en-001", (same as 'en')
        // "en-ZA", (same as 'en-001')
        "es",
        "es-AR",
        "fil", // (same as 'und', but retained)
        "fr",
        "ja", // (same as 'und', but retained)
        "ru",
        "sr", // Note: 'sr' and 'sr-Latn' are the same, but they don't inherit
        "sr-Latn",
        "th", // (same as 'und', but retained)
        "th/thai",
        "tr",
        "und",
    ];

    // Should return the supported locales set with deduplication.
    assert_eq!(exported.keys().collect::<Vec<_>>(), locales);
}

#[test]
fn explicit_preferred() {
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
        ExportDriver::new(
            families(SELECTED_LOCALES),
            DeduplicationStrategy::None.into(),
            LocaleFallbacker::new().static_to_owned(),
        ),
        &TestingProvider::with_decimal_symbol_like_data(),
    );

    // Explicit locales are "arc", "ar-EG", "ar-SA", "en-GB", "es", "sr-ME", "ru-Cyrl-RU"
    let locales = [
        "ar",         // ancestor of ar-EG
        "ar-EG",      // explicit locale
        "ar-EG/latn", // descendant of ar-EG
        "ar-SA",      // explicit locale, inheriting from ar
        "ar-SA/latn", // extensions should be included (#4533)
        "ar/latn",    // extensions should be included (#4533)
        "arc",        // Aramaic, inheriting from und
        "en",         // ancestor of en-GB
        "en-001",     // ancestor of en-GB
        "en-GB",      // explicit locale not in supported locales
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
        ExportDriver::new(
            families(SELECTED_LOCALES),
            DeduplicationStrategy::None.into(),
            LocaleFallbacker::new().static_to_owned(),
        ),
        &TestingProvider::with_decimal_symbol_like_data(),
    );

    // Explicit locales are "arc", "ar-EG", "ar-SA", "en-GB", "es", "sr-ME", "ru-Cyrl-RU"
    let locales = [
        "ar",         // ancestor of ar-EG
        "ar-EG",      // explicit locale
        "ar-EG/latn", // descendant of ar-EG
        "ar-SA",      // explicit locale, inheriting from ar
        "ar-SA/latn", // extensions should be included (#4533)
        "ar/latn",    // extensions should be included (#4533)
        "arc",        // Aramaic, inheriting from und
        "en",         // ancestor of en-GB
        "en-001",     // ancestor of en-GB
        "en-GB",      // explicit locale not in supported locales
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
        ExportDriver::new(
            families(SELECTED_LOCALES),
            DeduplicationStrategy::Maximal.into(),
            LocaleFallbacker::new().static_to_owned(),
        ),
        &TestingProvider::with_decimal_symbol_like_data(),
    );

    // Explicit locales are "arc", "ar-EG", "ar-SA", "en-GB", "es", "sr-ME", "ru-Cyrl-RU"
    #[rustfmt::skip]
    let locales = [
        "ar",
        // "ar-Arab-EG", (same as 'ar')
        // "ar-EG", (same as 'ar')
        // "ar-EG/latn", (same as 'ar/latn')
        // "ar-SA", (same as 'ar')
        // "ar-SA/latn", (same as 'ar/latn')
        "ar/latn",
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
fn explicit_runtime_retain_base() {
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
        ExportDriver::new(
            families(SELECTED_LOCALES),
            DeduplicationStrategy::RetainBaseLanguages.into(),
            LocaleFallbacker::new().static_to_owned(),
        ),
        &TestingProvider::with_decimal_symbol_like_data(),
    );

    // Explicit locales are "arc", "ar-EG", "ar-SA", "en-GB", "es", "sr-ME", "ru-Cyrl-RU"
    #[rustfmt::skip]
    let locales = [
        "ar",
        // "ar-Arab-EG", (same as 'ar')
        // "ar-EG", (same as 'ar')
        // "ar-EG/latn", (same as 'ar/latn')
        // "ar-SA", (same as 'ar')
        // "ar-SA/latn", (same as 'ar/latn')
        "ar/latn",
        "arc", // (same as 'und', but retained)
        "en",  // (same as 'und', but retained)
        // "en-001", (same as 'en')
        // "en-GB", (same as 'en-001')
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
        ExportDriver::new(
            SELECTED_LOCALES
                .into_iter()
                .map(Into::into)
                .map(DataLocaleFamily::single),
            DeduplicationStrategy::None.into(),
            LocaleFallbacker::new().static_to_owned(),
        ),
        &TestingProvider::with_decimal_symbol_like_data(),
    );

    // Explicit locales are "arc", "ar-EG", "ar-SA", "en-GB", "es", "sr-ME", "ru-Cyrl-RU"
    let locales = [
        "ar-EG",
        "ar-EG/latn", // extensions included even in preresolved mode
        "ar-SA",
        "ar-SA/latn", // extensions included even in preresolved mode
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
fn explicit_hybrid_without_descendants() {
    let selected_locales = [
        DataLocaleFamily::without_descendants(locale!("arc").into()), // Aramaic, not in supported list
        DataLocaleFamily::without_descendants(locale!("ar-EG").into()),
        DataLocaleFamily::without_descendants(locale!("ar-SA").into()),
        DataLocaleFamily::without_descendants(locale!("en-GB").into()),
        DataLocaleFamily::without_descendants(locale!("es").into()),
        DataLocaleFamily::without_descendants(locale!("sr-ME").into()),
        DataLocaleFamily::without_descendants(locale!("ru-Cyrl-RU").into()),
    ];
    let exported = export_to_map(
        ExportDriver::new(
            selected_locales,
            DeduplicationStrategy::None.into(),
            LocaleFallbacker::new().static_to_owned(),
        ),
        &TestingProvider::with_decimal_symbol_like_data(),
    );

    // Explicit locales are "arc", "ar-EG", "ar-SA", "en-GB", "es", "sr-ME", "ru-Cyrl-RU"
    let locales = [
        "ar",         // ancestor of ar-EG
        "ar-EG",      // explicit locale
        "ar-EG/latn", // explicit with extensions
        "ar-SA",      // explicit locale, inheriting from ar
        "ar-SA/latn", // extensions should be included (#4533)
        "ar/latn",    // extensions should be included (#4533)
        "arc",        // Aramaic, inheriting from und
        "en",         // ancestor of en-GB
        "en-001",     // ancestor of en-GB
        "en-GB",      // explicit locale not in supported locales
        // "en-ZA",        // not reachable
        "es", // explicit and supported
        // "es-AR",        // excluded: descendant of es
        "ru",         // ancestor of ru-Cyrl-RU
        "ru-Cyrl-RU", // explicit locale, even though it is not normalized
        // "sr",           // not reachable from sr-ME
        "sr-Latn", // ancestor of sr-ME
        "sr-ME",   // explicit locale not in supported locales
        "und",     // ancestor of everything
    ];

    // Should return the exact explicit locales set.
    assert_eq!(exported.keys().collect::<Vec<_>>(), locales);
}

#[test]
fn explicit_hybrid_without_ancestors() {
    let selected_locales = [
        DataLocaleFamily::without_ancestors(locale!("arc").into()), // Aramaic, not in supported list
        DataLocaleFamily::without_ancestors(locale!("ar-EG").into()),
        DataLocaleFamily::without_ancestors(locale!("ar-SA").into()),
        DataLocaleFamily::without_ancestors(locale!("en-GB").into()),
        DataLocaleFamily::without_ancestors(locale!("es").into()),
        DataLocaleFamily::without_ancestors(locale!("sr-ME").into()),
        DataLocaleFamily::without_ancestors(locale!("ru-Cyrl-RU").into()),
    ];
    let exported = export_to_map(
        ExportDriver::new(
            selected_locales,
            DeduplicationStrategy::None.into(),
            LocaleFallbacker::new().static_to_owned(),
        ),
        &TestingProvider::with_decimal_symbol_like_data(),
    );

    // Explicit locales are "arc", "ar-EG", "ar-SA", "en-GB", "es", "sr-ME", "ru-Cyrl-RU"
    let locales = [
        // "ar",           // excluded: ancestor of ar-EG
        "ar-EG",      // explicit locale
        "ar-EG/latn", // explicit with extensions
        "ar-SA",      // explicit locale, inheriting from ar
        "ar-SA/latn", // extensions should be included (#4533)
        // "ar/latn",    // excluded: ancestor of ar-EG
        "arc", // Aramaic, inheriting from und
        // "en",              // excluded: ancestor of en-GB
        // "en-001",          // excluded: ancestor of en-GB
        "en-GB", // explicit locale not in supported locales
        // "en-ZA",        // not reachable
        "es",    // explicit and supported
        "es-AR", // descendant of es
        // "ru",         // excluded: ancestor of ru-Cyrl-RU
        "ru-Cyrl-RU", // explicit locale, even though it is not normalized
        // "sr",           // not reachable from sr-ME
        // "sr-Latn", // excluded: ancestor of sr-ME
        "sr-ME", // explicit locale not in supported locales
                 // "und", // excluded: ancestor of everything
    ];

    // Should return the exact explicit locales set.
    assert_eq!(exported.keys().collect::<Vec<_>>(), locales);
}

#[test]
fn explicit_hybrid_mixed_families() {
    let selected_locales = [
        DataLocaleFamily::without_ancestors(locale!("arc").into()), // Aramaic, not in supported list
        DataLocaleFamily::with_descendants(locale!("ar-EG").into()),
        DataLocaleFamily::without_ancestors(locale!("ar-EG").into()), // duplicate entry for ar-EG
        DataLocaleFamily::with_descendants(locale!("en").into()),
        DataLocaleFamily::single(locale!("en").into()), // duplicate entry for en
        DataLocaleFamily::without_ancestors(locale!("en-GB").into()),
        DataLocaleFamily::without_descendants(locale!("es").into()),
        DataLocaleFamily::with_descendants(locale!("es").into()), // duplicate entry for es
    ];
    let exported = export_to_map(
        ExportDriver::new(
            selected_locales,
            DeduplicationStrategy::None.into(),
            LocaleFallbacker::new().static_to_owned(),
        ),
        &TestingProvider::with_decimal_symbol_like_data(),
    );

    let locales = [
        // "ar",              // excluded: ancestor of ar-EG
        "ar-EG",      // explicit locale
        "ar-EG/latn", // explicit with extensions
        // "ar-SA",           // explicit locale, inheriting from ar
        // "ar-SA/latn", // not reachable
        // "ar/latn",    // not reachable
        "arc", // Aramaic, inheriting from und
        "en",  // included as a singleton
        // "en-001",          // excluded: ancestor of en-GB
        "en-GB", // included without ancestors
        // "en-ZA",           // not reachable
        "es",    // explicit and supported
        "es-AR", // descendant of es
        // "ru",              // not requested
        // "ru-Cyrl-RU",      // not requested
        // "sr",              // not requested
        // "sr-Latn",         // not requested
        // "sr-ME",           // not requested
        "und",
    ];

    // Should return the exact explicit locales set.
    assert_eq!(exported.keys().collect::<Vec<_>>(), locales);
}

#[test]
fn explicit_runtime_und() {
    let exported = export_to_map(
        ExportDriver::new(
            [DataLocaleFamily::with_descendants(Default::default())],
            DeduplicationStrategy::Maximal.into(),
            LocaleFallbacker::new().static_to_owned(),
        ),
        &TestingProvider::with_decimal_symbol_like_data(),
    );

    // Explicit locales are "und"
    let locales = ["und"];

    // Should return the exact explicit locales set.
    assert_eq!(exported.keys().collect::<Vec<_>>(), locales);
}

#[test]
fn explicit_runtime_und_retain_base() {
    let exported = export_to_map(
        ExportDriver::new(
            [DataLocaleFamily::with_descendants(Default::default())],
            DeduplicationStrategy::RetainBaseLanguages.into(),
            LocaleFallbacker::new().static_to_owned(),
        ),
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
        ExportDriver::new(
            [DataLocaleFamily::with_descendants(Default::default())],
            DeduplicationStrategy::None.into(),
            LocaleFallbacker::new().static_to_owned(),
        ),
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
        ExportDriver::new(
            [DataLocaleFamily::single(Default::default())],
            DeduplicationStrategy::None.into(),
            LocaleFallbacker::new().static_to_owned(),
        ),
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
        ExportDriver::new(
            [],
            DeduplicationStrategy::Maximal.into(),
            LocaleFallbacker::new().static_to_owned(),
        ),
        &TestingProvider::with_decimal_symbol_like_data(),
    );

    // Should return the empty explicit locales set.
    assert!(exported.keys().collect::<Vec<_>>().is_empty());
}

#[test]
fn explicit_runtime_empty_retain_base() {
    let exported = export_to_map(
        ExportDriver::new(
            [],
            DeduplicationStrategy::RetainBaseLanguages.into(),
            LocaleFallbacker::new().static_to_owned(),
        ),
        &TestingProvider::with_decimal_symbol_like_data(),
    );

    // Should return the empty explicit locales set.
    assert!(exported.keys().collect::<Vec<_>>().is_empty());
}

#[test]
fn explicit_hybrid_empty() {
    let exported = export_to_map(
        ExportDriver::new(
            [],
            DeduplicationStrategy::None.into(),
            LocaleFallbacker::new().static_to_owned(),
        ),
        &TestingProvider::with_decimal_symbol_like_data(),
    );

    // Should return the empty explicit locales set.
    assert!(exported.keys().collect::<Vec<_>>().is_empty());
}

#[test]
fn explicit_preresolved_empty() {
    let exported = export_to_map(
        ExportDriver::new(
            [],
            DeduplicationStrategy::None.into(),
            LocaleFallbacker::new().static_to_owned(),
        ),
        &TestingProvider::with_decimal_symbol_like_data(),
    );

    // Explicit locales are empty
    let locales: [&str; 0] = [];

    // Should return the exact explicit locales set.
    assert_eq!(exported.keys().collect::<Vec<_>>(), locales);
}
