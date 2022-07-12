// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locid::langid;
use icu_provider::datagen::IterableResourceProvider;
use icu_provider::hello_world::{HelloWorldProvider, HelloWorldV1, HelloWorldV1Marker};
use icu_provider::prelude::*;
use icu_provider_fs::FsDataProvider;

const JSON_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/json");
const BINCODE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/bincode");
const POSTCARD_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/postcard");

const PATHS: &[&str] = &[JSON_PATH, BINCODE_PATH, POSTCARD_PATH];

#[test]
fn test_provider() {
    for path in PATHS {
        let provider = FsDataProvider::try_new(path).unwrap();
        for locale in HelloWorldProvider.supported_locales().unwrap() {
            let req = DataRequest {
                options: locale.into(),
                metadata: Default::default(),
            };

            let expected = HelloWorldProvider
                .load_resource(&req)
                .unwrap()
                .take_payload()
                .unwrap();

            let actual: DataPayload<HelloWorldV1Marker> = provider
                .load_resource(&req)
                .unwrap()
                .take_payload()
                .unwrap();
            assert_eq!(actual.get(), expected.get());

            let actual: DataPayload<HelloWorldV1Marker> = (&provider as &dyn BufferProvider)
                .as_deserializing()
                .load_resource(&req)
                .unwrap()
                .take_payload()
                .unwrap();
            assert_eq!(actual.get(), expected.get());
        }
    }
}

#[test]
fn test_errors() {
    for path in PATHS {
        let provider = FsDataProvider::try_new(path).unwrap();

        let err: Result<DataResponse<HelloWorldV1Marker>, DataError> =
            provider.load_resource(&DataRequest {
                options: langid!("zh-DE").into(),
                metadata: Default::default(),
            });

        assert!(
            matches!(
                err,
                Err(DataError {
                    kind: DataErrorKind::MissingResourceOptions,
                    ..
                })
            ),
            "{:?}",
            err
        );

        struct WrongV1Marker;
        impl DataMarker for WrongV1Marker {
            type Yokeable = HelloWorldV1<'static>;
        }
        impl ResourceMarker for WrongV1Marker {
            const KEY: ResourceKey = resource_key!("nope@1");
        }

        let err: Result<DataResponse<WrongV1Marker>, DataError> =
            provider.load_resource(&Default::default());

        assert!(
            matches!(
                err,
                Err(DataError {
                    kind: DataErrorKind::MissingResourceKey,
                    ..
                })
            ),
            "{:?}",
            err
        );
    }
}
