// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locale_core::langid;
use icu_provider::hello_world::{HelloWorldProvider, HelloWorldV1, HelloWorldV1Marker};
use icu_provider::prelude::*;
use icu_provider_fs::FsDataProvider;

const PATHS: &[&str] = &[
    "tests/data/json",
    "tests/data/bincode",
    "tests/data/postcard",
];

#[test]
fn test_provider() {
    for path in PATHS {
        let provider = FsDataProvider::try_new(path.into()).unwrap();
        for id in HelloWorldProvider.iter_ids().unwrap() {
            let req = DataRequest {
                id: id.as_borrowed(),
                ..Default::default()
            };

            let expected = HelloWorldProvider
                .load(req)
                .unwrap_or_else(|e| panic!("{e}: {req:?} ({path})"))
                .payload;

            let actual: DataPayload<HelloWorldV1Marker> = provider
                .as_deserializing()
                .load(req)
                .unwrap_or_else(|e| panic!("{e}: {req:?} ({path})"))
                .payload;
            assert_eq!(actual.get(), expected.get());

            let actual: DataPayload<HelloWorldV1Marker> = (&provider as &dyn BufferProvider)
                .as_deserializing()
                .load(req)
                .unwrap_or_else(|e| panic!("{e}: {req:?} ({path})"))
                .payload;
            assert_eq!(actual.get(), expected.get());
        }
    }
}

#[test]
fn test_errors() {
    for path in PATHS {
        let provider = FsDataProvider::try_new(path.into()).unwrap();

        let err: Result<DataResponse<HelloWorldV1Marker>, DataError> =
            provider.as_deserializing().load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("zh-DE").into()),
                ..Default::default()
            });

        assert!(
            matches!(
                err,
                Err(DataError {
                    kind: DataErrorKind::IdentifierNotFound,
                    ..
                })
            ),
            "{err:?}"
        );

        struct WrongV1Marker;
        impl DynamicDataMarker for WrongV1Marker {
            type DataStruct = HelloWorldV1<'static>;
        }
        impl DataMarker for WrongV1Marker {
            const INFO: DataMarkerInfo =
                DataMarkerInfo::from_path(icu_provider::marker::data_marker_path!("nope@1"));
        }

        let err: Result<DataResponse<WrongV1Marker>, DataError> =
            provider.as_deserializing().load(Default::default());

        assert!(
            matches!(
                err,
                Err(DataError {
                    kind: DataErrorKind::MarkerNotFound,
                    ..
                })
            ),
            "{err:?}"
        );
    }
}

#[test]
fn prefix_match() {
    for path in PATHS {
        let provider = FsDataProvider::try_new(path.into()).unwrap();

        let id = DataIdentifierCow::from_owned(
            DataMarkerAttributes::from_str_or_panic("reve").to_owned(),
            "ja".parse().unwrap(),
        );

        assert!(DataProvider::<HelloWorldV1Marker>::load(
            &provider.as_deserializing(),
            DataRequest {
                id: id.as_borrowed(),
                ..Default::default()
            }
        )
        .is_err());

        assert!(DataProvider::<HelloWorldV1Marker>::load(
            &provider.as_deserializing(),
            DataRequest {
                id: id.as_borrowed(),
                metadata: {
                    let mut metadata = DataRequestMetadata::default();
                    metadata.attributes_prefix_match = true;
                    metadata
                }
            }
        )
        .is_ok());

        let id = DataIdentifierCow::from_owned(
            DataMarkerAttributes::from_str_or_panic("non-existent").to_owned(),
            "ja".parse().unwrap(),
        );

        assert!(DataProvider::<HelloWorldV1Marker>::load(
            &provider.as_deserializing(),
            DataRequest {
                id: id.as_borrowed(),
                metadata: {
                    let mut metadata = DataRequestMetadata::default();
                    metadata.attributes_prefix_match = true;
                    metadata
                }
            }
        )
        .is_err());
    }
}
