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
use std::hash::Hasher;

const BLOB_V1: &[u8] = include_bytes!("data/v1.postcard");
const BLOB_V2: &[u8] = include_bytes!("data/v2.postcard");

fn run_driver(mut exporter: BlobExporter, provider: &impl IterableDataProvider<HelloWorldV1Marker>)
where
    ExportMarker: UpcastDataPayload<HelloWorldV1Marker>,
{
    for id in &provider.iter_ids().unwrap() {
        let req = DataRequest {
            id: id.as_borrowed(),
            ..Default::default()
        };
        let res = DataProvider::<HelloWorldV1Marker>::load(provider, req).unwrap();
        exporter
            .put_payload(
                HelloWorldV1Marker::INFO,
                id.as_borrowed(),
                &ExportMarker::upcast(res.payload),
            )
            .unwrap();
    }
    exporter
        .flush(HelloWorldV1Marker::INFO, DeduplicationStrategy::None)
        .unwrap();
    exporter.close().unwrap();
}

fn check_hello_world(
    blob_provider: impl DataProvider<HelloWorldV1Marker>,
    test_prefix_match: bool,
) {
    let hello_world_provider = HelloWorldProvider;
    for id in hello_world_provider.iter_ids().unwrap() {
        let blob_result = blob_provider
            .load(DataRequest {
                id: id.as_borrowed(),
                ..Default::default()
            })
            .unwrap()
            .payload;
        let expected_result = hello_world_provider
            .load(DataRequest {
                id: id.as_borrowed(),
                ..Default::default()
            })
            .unwrap()
            .payload;
        assert_eq!(blob_result, expected_result, "{:?}", id);
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
fn test_v1() {
    let mut blob: Vec<u8> = Vec::new();
    let exporter = BlobExporter::new_with_sink(Box::new(&mut blob));
    run_driver(exporter, &HelloWorldProvider);
    assert_eq!(BLOB_V1, blob.as_slice());

    let blob_provider = BlobDataProvider::try_new_from_blob(blob.into_boxed_slice()).unwrap();
    check_hello_world(blob_provider.as_deserializing(), false);
}

#[test]
fn test_v2() {
    let mut blob: Vec<u8> = Vec::new();
    let exporter = BlobExporter::new_v2_with_sink(Box::new(&mut blob));
    run_driver(exporter, &HelloWorldProvider);
    assert_eq!(BLOB_V2, blob.as_slice());

    let blob_provider = BlobDataProvider::try_new_from_blob(blob.into_boxed_slice()).unwrap();
    assert!(
        !blob_provider.internal_is_using_v2_bigger_format(),
        "Should have exported to smaller V2 format"
    );
    check_hello_world(blob_provider.as_deserializing(), true);
}

// This tests that the V2Bigger format works by attempting to export something with 26^4 = 456976 data entries
#[test]
fn test_v2_bigger() {
    // We do print progress since this is a slower test and it's useful to get some feedback.
    println!("Exporting blob ....");
    let mut blob: Vec<u8> = Vec::new();
    let exporter = BlobExporter::new_v2_with_sink(Box::new(&mut blob));
    run_driver(exporter, &ManyLocalesProvider);
    // Rather than check in a 10MB file, we just compute hashes
    println!("Computing hash ....");
    // Construct a hasher with a random, stable seed
    let mut hasher = twox_hash::XxHash64::with_seed(1234);
    hasher.write(&blob);
    let hash = hasher.finish();

    assert_eq!(
        hash, 11911648880836574343,
        "V2Bigger format appears to have changed!"
    );

    println!("Loading and testing locales .... ");
    let blob_provider = BlobDataProvider::try_new_from_blob(blob.into_boxed_slice()).unwrap();
    assert!(
        blob_provider.internal_is_using_v2_bigger_format(),
        "Should have exported to V2Bigger format"
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
        let blob_result = DataProvider::<HelloWorldV1Marker>::load(
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

impl DataProvider<HelloWorldV1Marker> for ManyLocalesProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<HelloWorldV1Marker>, DataError> {
        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(HelloWorldV1 {
                message: format!("Hello {}!", req.id.locale).into(),
            }),
        })
    }
}

const LOWERCASE: core::ops::RangeInclusive<u8> = b'a'..=b'z';

impl IterableDataProvider<HelloWorldV1Marker> for ManyLocalesProvider {
    fn iter_ids(&self) -> Result<BTreeSet<DataIdentifierCow>, DataError> {
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

icu_provider::export::make_exportable_provider!(ManyLocalesProvider, [HelloWorldV1Marker,]);
