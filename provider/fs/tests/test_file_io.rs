// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(unused_imports)]

use icu_locid::langid;
use icu_locid::LanguageIdentifier;
use icu_provider::hello_world::*;
use icu_provider::prelude::*;
use icu_provider::serde::*;
use icu_provider_fs::FsDataProvider;

#[test]
fn test_json() {
    let provider =
        FsDataProvider::try_new("./tests/data/json").expect("Loading file from testdata directory");

    let privet: DataPayload<HelloWorldV1Marker> = provider
        .load_resource(&DataRequest {
            options: langid!("ru").into(),
            metadata: Default::default(),
        })
        .and_then(DataResponse::take_payload)
        .expect("The data should be present");
    assert_eq!(privet.get().message, "Привет, мир");
}

#[test]
fn test_json_dyn_erased_serde() {
    let provider =
        FsDataProvider::try_new("./tests/data/json").expect("Loading file from testdata directory");

    let privet: DataPayload<HelloWorldV1Marker> = (&provider as &dyn BufferProvider)
        .as_deserializing()
        .load_resource(&DataRequest {
            options: langid!("ru").into(),
            metadata: Default::default(),
        })
        .and_then(DataResponse::take_payload)
        .expect("The data should be present");
    assert_eq!(privet.get().message, "Привет, мир");
}

#[test]
fn test_json_errors() {
    let provider =
        FsDataProvider::try_new("./tests/data/json").expect("Loading file from testdata directory");

    struct WrongV1Marker;
    impl DataMarker for WrongV1Marker {
        type Yokeable = HelloWorldV1<'static>;
    }
    impl ResourceMarker for WrongV1Marker {
        const KEY: ResourceKey = resource_key!("nope@1");
    }

    assert!(matches!(
        (&provider as &dyn ResourceProvider<HelloWorldV1Marker>).load_resource(&DataRequest {
            options: langid!("ru").into(),
            metadata: Default::default(),
        },),
        Ok(_)
    ));

    assert!(matches!(
        (&provider as &dyn ResourceProvider<HelloWorldV1Marker>).load_resource(&DataRequest {
            options: langid!("sr").into(),
            metadata: Default::default(),
        },),
        Err(DataError {
            kind: DataErrorKind::MissingResourceOptions,
            ..
        })
    ));

    assert!(matches!(
        (&provider as &dyn ResourceProvider<WrongV1Marker>).load_resource(&DataRequest {
            options: langid!("ru").into(),
            metadata: Default::default(),
        },),
        Err(DataError {
            kind: DataErrorKind::MissingResourceKey,
            ..
        })
    ));

    assert!(matches!(
        (&provider as &dyn ResourceProvider<WrongV1Marker>).load_resource(&DataRequest {
            options: langid!("ru").into(),
            metadata: Default::default(),
        },),
        Err(DataError {
            kind: DataErrorKind::MissingResourceKey,
            ..
        })
    ));
}

#[test]
fn test_bincode() {
    let provider = FsDataProvider::try_new("./tests/data/bincode")
        .expect("Loading file from testdata directory");

    let privet: DataPayload<HelloWorldV1Marker> = provider
        .load_resource(&DataRequest {
            options: langid!("ru").into(),
            metadata: Default::default(),
        })
        .and_then(DataResponse::take_payload)
        .expect("The data should be present");
    assert_eq!(privet.get().message, "Привет, мир");
}

#[test]
fn test_bincode_dyn_erased_serde() {
    let provider = FsDataProvider::try_new("./tests/data/bincode")
        .expect("Loading file from testdata directory");

    let privet: DataPayload<HelloWorldV1Marker> = (&provider as &dyn BufferProvider)
        .as_deserializing()
        .load_resource(&DataRequest {
            options: langid!("ru").into(),
            metadata: Default::default(),
        })
        .and_then(DataResponse::take_payload)
        .expect("The data should be present");
    assert_eq!(privet.get().message, "Привет, мир");
}
