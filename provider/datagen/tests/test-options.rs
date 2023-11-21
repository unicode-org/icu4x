// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::{BTreeMap, HashSet};

use elsa::sync::FrozenMap;
use icu_datagen::prelude::*;
use icu_locid_transform::provider::*;
use icu_provider::datagen::ExportMarker;
use icu_provider::datagen::*;
use icu_provider::hello_world::*;
use icu_provider::make_exportable_provider;
use icu_provider::prelude::*;
use postcard::ser_flavors::{AllocVec, Flavor};
use writeable::Writeable;

struct TestingProvider;

impl DataProvider<HelloWorldV1Marker> for TestingProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<HelloWorldV1Marker>, DataError> {
        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(HelloWorldV1 {
                message: match req.locale.to_string().as_str() {
                    "ar" => "Hello in ar",
                    "ar-EG" => "Hello in ar",
                    "ar-EG-u-nu-latn" => "Hello in ar-u-nu-latn",
                    "ar-u-nu-latn" => "Hello in ar-u-nu-latn",
                    "en" => "Hello in en",
                    "en-ZA" => "Hello in en",
                    "sr" => "Hello in sr",
                    "sr-Latn" => "Hello in sr-Latn",
                    "und" => "Hello in en",
                    _ => return Err(DataErrorKind::MissingLocale.into_error()),
                }
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
        Ok([
            "ar",
            "ar-EG",
            "ar-EG-u-nu-latn",
            "ar-u-nu-latn",
            "en",
            "en-ZA",
            "sr",
            "sr-Latn",
            "und",
        ]
        .into_iter()
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

#[derive(Default)]
struct TestingExporter {
    data: FrozenMap<DataLocale, String>,
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
        self.data.insert(
            locale.clone(),
            postcard::from_bytes::<HelloWorldV1>(&output)
                .unwrap()
                .message
                .to_string(),
        );
        Ok(())
    }
}

impl TestingExporter {
    /// Returns a map of the data locales as BCP-47 strings to their hello world messages.
    pub fn take_map_and_reset(&mut self) -> BTreeMap<String, String> {
        core::mem::take(&mut self.data)
            .into_tuple_vec()
            .into_iter()
            .map(|(data_locale, buffer)| (data_locale.write_to_string().into_owned(), buffer))
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

    let provider = TestingProvider;

    let mut testing_exporter = TestingExporter::default();

    let driver = DatagenDriver::new().with_keys([HelloWorldV1Marker::KEY]);

    let explicit_locales: HashSet<LanguageIdentifier> = [
        langid!("arc"), // Not supported
        langid!("ar-EG"),
        langid!("en-GB"),
        langid!("sr-ME"),
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

    // These are all of the supported locales.
    let all_locales = [
        "ar",
        "ar-EG",
        "ar-EG-u-nu-latn",
        "ar-u-nu-latn",
        "en",
        "en-ZA",
        "sr",
        "sr-Latn",
        "und",
    ];

    // All+Hybrid should return exactly the supported locales set.
    assert_eq!(data_all_hybrid.keys().collect::<Vec<_>>(), all_locales);

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
    let all_locales_dedup = [
        "ar",
        // "ar-EG", (same as 'ar')
        "ar-EG-u-nu-latn", // (same as 'ar-u-nu-latn' but DIFFERENT than 'ar-EG')
        "ar-u-nu-latn",
        // "en", (same as 'und')
        // "en-ZA", (same as 'und')
        "sr", // Note: 'sr' and 'sr-Latn' are the same, but they don't inherit
        "sr-Latn",
        "und",
    ];

    // Assert that the stated equivalences are correct
    assert_eq!(
        data_all_hybrid.get("ar-EG").unwrap(),
        data_all_hybrid.get("ar").unwrap()
    );
    assert_eq!(
        data_all_hybrid.get("en-ZA").unwrap(),
        data_all_hybrid.get("und").unwrap()
    );
    assert_eq!(
        data_all_hybrid.get("en").unwrap(),
        data_all_hybrid.get("und").unwrap()
    );

    // All+Runtime should return the supported locales set with deduplication.
    assert_eq!(
        data_all_runtime.keys().collect::<Vec<_>>(),
        all_locales_dedup
    );

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

    // Explicit locales are "arc", "ar-EG", "en-GB", "sr-ME"
    let explicit_hybrid_locales = [
        "ar",              // ancestor of ar-EG
        "ar-EG",           // explicit locale
        "ar-EG-u-nu-latn", // descendant of ar-EG
        // "ar-u-nu-latn", // ??? should this be included?
        "arc",    // Aramaic, inheriting from und
        "en",     // ancestor of en-GB
        "en-GB",  // explicit locale not in supported locales
        // "en-ZA", // not reachable
        // "sr", // not reachable from sr-ME
        "sr-Latn", // ancestor of sr-ME
        "sr-ME",   // explicit locale not in supported locales
        "und",     // ancestor of everything
    ];

    // Explicit+Hybrid should return the expanded explicit locales set above.
    assert_eq!(
        data_explicit_hybrid.keys().collect::<Vec<_>>(),
        explicit_hybrid_locales
    );

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

    // Explicit locales are "arc", "ar-EG", "en-GB", "sr-ME"
    let explicit_hybrid_locales_dedup = [
        "ar",
        // "ar-Arab-EG", (same as 'ar')
        // "ar-EG", (same as 'ar')
        "ar-EG-u-nu-latn",
        // "arc", (same as 'und')
        // "en", (same as 'und')
        // "en-001", (same as 'und')
        // "en-GB", (same as 'und')
        "sr-Latn",
        // "sr-ME", (same as 'sr-Latn')
        "und",
    ];

    // Explicit+Runtime should return the expanded then deduplicated explicit locales set above.
    assert_eq!(
        data_explicit_runtime.keys().collect::<Vec<_>>(),
        explicit_hybrid_locales_dedup,
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

    // Explicit locales are "arc", "ar-EG", "en-GB", "sr-ME"
    let explicit_preresolved_locales = [
        "ar-EG",
        "ar-EG-u-nu-latn", // extensions included even in preresolved mode
        "arc",
        "en-GB",
        "sr-ME",
    ];

    // Explicit+Preresolved should return the exact explicit locales set.
    assert_eq!(
        data_explicit_preresolved.keys().collect::<Vec<_>>(),
        explicit_preresolved_locales,
    );
}
