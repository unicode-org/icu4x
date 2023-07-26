// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::{HashMap, HashSet};
use std::path::Path;

use elsa::sync::FrozenMap;
use icu_datagen::options::{FallbackMode, LocaleInclude, Options};
use icu_datagen::{DatagenProvider, SourceData};
use icu_decimal::provider::DecimalSymbolsV1Marker;
use icu_locid::langid;
use icu_provider::datagen::{DataExporter, ExportMarker};
use icu_provider::prelude::*;
use postcard::ser_flavors::{AllocVec, Flavor};

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
    pub fn take_map_and_reset(&mut self) -> HashMap<(DataKey, DataLocale), Vec<u8>> {
        core::mem::take(&mut self.data).into_iter().collect()
    }
}

#[test]
fn test_fallback_options() {
    simple_logger::SimpleLogger::new()
        .env()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();

    let data_root = Path::new(concat!(core::env!("CARGO_MANIFEST_DIR"), "/tests/data/"));

    let source = SourceData::offline()
        .with_cldr(data_root.join("cldr"), Default::default())
        .unwrap()
        .with_icuexport(data_root.join("icuexport"))
        .unwrap();

    let decimal_symbols_key: HashSet<DataKey> = [DecimalSymbolsV1Marker::KEY].into_iter().collect();

    let mut testing_exporter = TestingExporter::default();

    let mut options = Options::default();

    options.locales = LocaleInclude::All;
    options.fallback = FallbackMode::Hybrid;
    DatagenProvider::try_new(options.clone(), source.clone())
        .unwrap()
        .export(decimal_symbols_key.clone(), &mut testing_exporter)
        .unwrap();
    let data_all_hybrid = testing_exporter.take_map_and_reset();

    options.fallback = FallbackMode::RuntimeManual;
    DatagenProvider::try_new(options.clone(), source.clone())
        .unwrap()
        .export(decimal_symbols_key.clone(), &mut testing_exporter)
        .unwrap();
    let data_all_runtime = testing_exporter.take_map_and_reset();

    options.locales = LocaleInclude::Explicit(
        [langid!("en-GB"), langid!("sr-ME"), langid!("ar")]
            .into_iter()
            .collect(),
    );
    options.fallback = FallbackMode::Hybrid;
    DatagenProvider::try_new(options.clone(), source.clone())
        .unwrap()
        .export(decimal_symbols_key.clone(), &mut testing_exporter)
        .unwrap();
    let data_explicit_hybrid = testing_exporter.take_map_and_reset();

    options.fallback = FallbackMode::RuntimeManual;
    DatagenProvider::try_new(options.clone(), source.clone())
        .unwrap()
        .export(decimal_symbols_key.clone(), &mut testing_exporter)
        .unwrap();
    let data_explicit_runtime = testing_exporter.take_map_and_reset();

    // options.fallback = FallbackMode::Preresolved;
    // DatagenProvider::try_new(options, source).unwrap()
    //     .export(decimal_symbols_key, &mut testing_exporter)
    //     .unwrap();
    // let data_explicit_preresolved = testing_exporter.take_map_and_reset();
}
