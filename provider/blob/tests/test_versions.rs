// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_datagen::prelude::*;
use icu_provider::datagen::IterableDataProvider;
use icu_provider::hello_world::*;
use icu_provider::prelude::*;
use icu_provider_blob::export::*;
use icu_provider_blob::BlobDataProvider;

const BLOB_V1: &[u8] = include_bytes!("data/v1.postcard");
const BLOB_V2: &[u8] = include_bytes!("data/v2.postcard");

fn run_driver(exporter: BlobExporter) -> Result<(), DataError> {
    DatagenDriver::new()
        .with_keys([icu_provider::hello_world::HelloWorldV1Marker::KEY])
        .with_all_locales()
        .export(&DatagenProvider::new_custom(), exporter)
}

fn check_hello_world(blob_provider: impl DataProvider<HelloWorldV1Marker>) {
    let hello_world_provider = HelloWorldProvider;
    for locale in hello_world_provider.supported_locales().unwrap() {
        let blob_result = blob_provider
            .load(DataRequest {
                locale: &locale,
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();
        let expected_result = hello_world_provider
            .load(DataRequest {
                locale: &locale,
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();
        assert_eq!(blob_result, expected_result, "{locale:?}");
    }
}

#[test]
fn test_v1() {
    let mut blob: Vec<u8> = Vec::new();
    let exporter = BlobExporter::new_with_sink(Box::new(&mut blob));
    run_driver(exporter).unwrap();
    assert_eq!(BLOB_V1, blob.as_slice());

    let blob_privider = BlobDataProvider::try_new_from_blob(blob.into_boxed_slice()).unwrap();
    check_hello_world(blob_privider.as_deserializing());
}

#[test]
fn test_v2() {
    let mut blob: Vec<u8> = Vec::new();
    let exporter = BlobExporter::new_v2_with_sink(Box::new(&mut blob));
    run_driver(exporter).unwrap();
    assert_eq!(BLOB_V2, blob.as_slice());

    let blob_privider = BlobDataProvider::try_new_from_blob(blob.into_boxed_slice()).unwrap();
    check_hello_world(blob_privider.as_deserializing());
}
