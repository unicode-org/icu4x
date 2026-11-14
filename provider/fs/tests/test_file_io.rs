// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locale_core::data_locale;
use icu_provider::hello_world::{HelloWorld, HelloWorldProvider, HelloWorldV1};
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
                .unwrap_or_else(|e| panic!("{e}: {req:?} ({path})"));

            let actual: DataResponse<HelloWorldV1> = provider
                .as_deserializing()
                .load(req)
                .unwrap_or_else(|e| panic!("{e}: {req:?} ({path})"));
            assert_eq!(actual.payload.get(), expected.payload.get());
            assert_eq!(actual.metadata.checksum, expected.metadata.checksum);

            let actual: DataResponse<HelloWorldV1> = (&provider as &dyn BufferProvider)
                .as_deserializing()
                .load(req)
                .unwrap_or_else(|e| panic!("{e}: {req:?} ({path})"));
            assert_eq!(actual.payload.get(), expected.payload.get());
            assert_eq!(actual.metadata.checksum, expected.metadata.checksum);
        }
    }
}

#[test]
fn test_errors() {
    for path in PATHS {
        let provider = FsDataProvider::try_new(path.into()).unwrap();

        let err: Result<DataResponse<HelloWorldV1>, DataError> =
            provider.as_deserializing().load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&data_locale!("zh-DE")),
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

        icu_provider::data_marker!(WrongV1, HelloWorld<'static>);

        let err: Result<DataResponse<WrongV1>, DataError> =
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

        assert!(
            DataProvider::<HelloWorldV1>::load(
                &provider.as_deserializing(),
                DataRequest {
                    id: id.as_borrowed(),
                    ..Default::default()
                }
            )
            .is_err()
        );

        assert!(
            DataProvider::<HelloWorldV1>::load(
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
            .is_ok()
        );

        let id = DataIdentifierCow::from_owned(
            DataMarkerAttributes::from_str_or_panic("non-existent").to_owned(),
            "ja".parse().unwrap(),
        );

        assert!(
            DataProvider::<HelloWorldV1>::load(
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
            .is_err()
        );
    }
}

icu_provider::data_marker!(HelloSingletonV1, HelloWorld<'static>, is_singleton = true);
icu_provider::data_marker!(HelloMissingV1, HelloWorld<'static>, is_singleton = true);

#[test]
fn test_singleton() {
    let tmp_dir = std::env::temp_dir().join("icu4x_fs_test_singleton");
    let _ = std::fs::remove_dir_all(&tmp_dir);

    #[cfg(feature = "export")]
    {
        use icu_provider::dynutil::UpcastDataPayload;
        use icu_provider::export::{DataExporter, ExportMarker, FlushMetadata};
        use icu_provider_fs::export::{
            FilesystemExporter, Options, OverwriteOption, serializers::Json,
        };

        let mut options = Options::default();
        options.root = tmp_dir.clone();
        options.overwrite = OverwriteOption::RemoveAndReplace;
        let exporter = FilesystemExporter::try_new(Box::new(Json::default()), options).unwrap();

        let payload: DataPayload<HelloSingletonV1> = DataPayload::from_owned(HelloWorld {
            message: "Hello, Singleton!".into(),
        });
        let export_payload: DataPayload<ExportMarker> = UpcastDataPayload::upcast(payload);
        let mut flush_metadata = FlushMetadata::default();
        flush_metadata.checksum = Some(1234);
        exporter
            .flush_singleton(HelloSingletonV1::INFO, &export_payload, flush_metadata)
            .unwrap();
    }
    #[cfg(not(feature = "export"))]
    {
        std::fs::create_dir_all(&tmp_dir).unwrap();
        std::fs::write(tmp_dir.join("manifest.json"), r#"{"syntax":"Json"}"#).unwrap();
        let file_path = tmp_dir.join("hello/singleton/v1.json");
        std::fs::create_dir_all(file_path.parent().unwrap()).unwrap();
        std::fs::write(&file_path, r#"{"message":"Hello, Singleton!"}"#).unwrap();
        std::fs::write(tmp_dir.join("hello/singleton/v1_checksum"), "1234").unwrap();
    }

    let provider = FsDataProvider::try_new(tmp_dir.clone()).unwrap();
    let req = DataRequest {
        id: DataIdentifierBorrowed::for_locale(&data_locale!("und")),
        ..Default::default()
    };

    let res: DataResponse<HelloSingletonV1> = provider.as_deserializing().load(req).unwrap();
    assert_eq!(res.payload.get().message, "Hello, Singleton!");
    assert_eq!(res.metadata.checksum, Some(1234));

    // Non-und locale InvalidRequest
    let req_invalid = DataRequest {
        id: DataIdentifierBorrowed::for_locale(&data_locale!("en")),
        ..Default::default()
    };
    let err = DataProvider::<HelloSingletonV1>::load(&provider.as_deserializing(), req_invalid)
        .unwrap_err();
    assert!(matches!(err.kind, DataErrorKind::InvalidRequest));

    // Missing singleton MarkerNotFound
    let err = DataProvider::<HelloMissingV1>::load(&provider.as_deserializing(), req).unwrap_err();
    assert!(matches!(err.kind, DataErrorKind::MarkerNotFound));

    std::fs::remove_dir_all(&tmp_dir).unwrap();
}
