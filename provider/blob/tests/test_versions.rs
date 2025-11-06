// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locale::subtags::{region, script, Language};
use icu_locale_core::LanguageIdentifier;
use icu_provider::dynutil::UpcastDataPayload;
use icu_provider::export::*;
use icu_provider::hello_world::*;
use icu_provider::prelude::*;
use icu_provider_blob::export::*;
use icu_provider_blob::BlobDataProvider;
use std::collections::BTreeSet;

const BLOB_V3: &[u8] = include_bytes!("data/v3.postcard");

fn run_driver(mut exporter: BlobExporter, provider: &impl IterableDataProvider<HelloWorldV1>)
where
    ExportMarker: UpcastDataPayload<HelloWorldV1>,
{
    for id in &provider.iter_ids().unwrap() {
        let req = DataRequest {
            id: id.as_borrowed(),
            ..Default::default()
        };
        let res = DataProvider::<HelloWorldV1>::load(provider, req).unwrap();
        exporter
            .put_payload(
                HelloWorldV1::INFO,
                id.as_borrowed(),
                &ExportMarker::upcast(res.payload),
            )
            .unwrap();
    }
    exporter
        .flush(HelloWorldV1::INFO, {
            let mut metadata = FlushMetadata::default();
            metadata.checksum = Some(1234);
            metadata
        })
        .unwrap();
    exporter.close().unwrap();
}

fn check_hello_world(blob_provider: impl DataProvider<HelloWorldV1>, test_prefix_match: bool) {
    let hello_world_provider = HelloWorldProvider;
    for id in hello_world_provider.iter_ids().unwrap() {
        let blob_result = blob_provider
            .load(DataRequest {
                id: id.as_borrowed(),
                ..Default::default()
            })
            .unwrap();
        let expected_result = hello_world_provider
            .load(DataRequest {
                id: id.as_borrowed(),
                ..Default::default()
            })
            .unwrap();
        assert_eq!(blob_result.payload, expected_result.payload, "{id:?}");
        assert_eq!(
            blob_result.metadata.checksum, expected_result.metadata.checksum,
            "{id:?}"
        );
    }

    if test_prefix_match {
        let id = DataIdentifierCow::from_owned(
            DataMarkerAttributes::from_str_or_panic("reve").to_owned(),
            "ja".parse().unwrap(),
        );
        assert!(blob_provider
            .load(DataRequest {
                id: id.as_borrowed(),
                ..Default::default()
            })
            .is_err());

        assert!(blob_provider
            .load(DataRequest {
                id: id.as_borrowed(),
                metadata: {
                    let mut metadata = DataRequestMetadata::default();
                    metadata.attributes_prefix_match = true;
                    metadata
                }
            })
            .is_ok());

        let id = DataIdentifierCow::from_owned(
            DataMarkerAttributes::from_str_or_panic("non-existent").to_owned(),
            "ja".parse().unwrap(),
        );

        assert!(blob_provider
            .load(DataRequest {
                id: id.as_borrowed(),
                metadata: {
                    let mut metadata = DataRequestMetadata::default();
                    metadata.attributes_prefix_match = true;
                    metadata
                }
            })
            .is_err());
    }
}

#[test]
fn test_format() {
    let mut blob: Vec<u8> = Vec::new();
    let exporter = BlobExporter::new_with_sink(Box::new(&mut blob));
    run_driver(exporter, &HelloWorldProvider);
    assert_eq!(BLOB_V3, blob.as_slice());

    let blob_provider = BlobDataProvider::try_new_from_blob(blob.into_boxed_slice()).unwrap();
    assert!(
        !blob_provider.internal_is_using_bigger_format(),
        "Should have exported to smaller  format"
    );
    check_hello_world(blob_provider.as_deserializing(), true);
}

// This tests that the Bigger format works by attempting to export something with 26^4 = 456976 data entries
#[test]
fn test_format_bigger() {
    // We do print progress since this is a slower test and it's useful to get some feedback.
    println!("Exporting blob ....");
    let mut blob: Vec<u8> = Vec::new();
    let exporter = BlobExporter::new_with_sink(Box::new(&mut blob));
    run_driver(exporter, &ManyLocalesProvider);

    // Rather than check in a 10MB file, we just compute hashes
    println!("Computing hash ....");
    let hash = twox_hash::XxHash64::oneshot(1234, &blob);

    assert_eq!(
        hash, 9019763565456414394,
        "Bigger format appears to have changed!"
    );

    println!("Loading and testing locales .... ");
    let blob_provider = BlobDataProvider::try_new_from_blob(blob.into_boxed_slice()).unwrap();
    assert!(
        blob_provider.internal_is_using_bigger_format(),
        "Should have exported to Bigger format"
    );
    let blob_provider = blob_provider.as_deserializing();

    for loc in &[
        "abc-Latn-001",
        "pqj-Latn-001",
        "zlr-Latn-001",
        "qfr-Latn-001",
        "tyz-Latn-001",
        "uaf-Latn-001",
    ] {
        let blob_result = DataProvider::<HelloWorldV1>::load(
            &blob_provider,
            DataRequest {
                id: DataIdentifierBorrowed::for_locale(&loc.parse().expect("locale must parse")),
                ..Default::default()
            },
        )
        .unwrap()
        .payload;
        assert_eq!(blob_result.get().message, format!("Hello {loc}!"))
    }
}

struct ManyLocalesProvider;

impl DataProvider<HelloWorldV1> for ManyLocalesProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<HelloWorldV1>, DataError> {
        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(HelloWorld {
                message: format!("Hello {}!", req.id.locale).into(),
            }),
        })
    }
}

const LOWERCASE: core::ops::RangeInclusive<u8> = b'a'..=b'z';

impl IterableDataProvider<HelloWorldV1> for ManyLocalesProvider {
    fn iter_ids(&self) -> Result<BTreeSet<DataIdentifierCow<'_>>, DataError> {
        Ok(LOWERCASE
            .flat_map(|i0| {
                LOWERCASE.flat_map(move |i1| {
                    LOWERCASE.map(move |i2| {
                        DataIdentifierCow::from_locale(
                            LanguageIdentifier::from((
                                Language::try_from_raw([i0, i1, i2]).unwrap(),
                                Some(script!("Latn")),
                                Some(region!("001")),
                            ))
                            .into(),
                        )
                    })
                })
            })
            .collect())
    }
}

extern crate alloc;
icu_provider::export::make_exportable_provider!(ManyLocalesProvider, [HelloWorldV1,]);
