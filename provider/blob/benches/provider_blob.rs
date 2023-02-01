// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use icu_locid::langid;
use icu_provider::hello_world::HelloWorldV1Marker;
use icu_provider::prelude::*;
use icu_provider_blob::BlobDataProvider;

static BLOB: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/data/hello_world.postcard"
));

fn overview_bench(c: &mut Criterion) {
    c.bench_function("static/overview", |b| {
        b.iter(|| {
            let provider = BlobDataProvider::try_new_from_static_blob(BLOB).unwrap();
            let _: DataPayload<HelloWorldV1Marker> = black_box(&provider)
                .as_deserializing()
                .load(DataRequest {
                    locale: &langid!("ru").into(),
                    metadata: Default::default(),
                })
                .and_then(DataResponse::take_payload)
                .expect("Loading was successful");
        });
    });

    #[cfg(feature = "bench")]
    {
        static_bench(c);
        blob_bench(c);
    }
}

#[cfg(feature = "bench")]
fn static_bench(c: &mut Criterion) {
    fn create() -> BlobDataProvider {
        icu_provider_blob::BlobDataProvider::try_new_from_static_blob(BLOB).unwrap()
    }

    c.bench_function("static/create", |b| b.iter(create));

    let provider = create();

    c.bench_function("static/generic", |b| {
        b.iter(|| {
            let _: DataPayload<HelloWorldV1Marker> = black_box(&provider)
                .as_deserializing()
                .load(DataRequest {
                    locale: &langid!("ru").into(),
                    metadata: Default::default(),
                })
                .and_then(DataResponse::take_payload)
                .expect("Loading was successful");
        });
    });

    c.bench_function("static/erased_serde", |b| {
        b.iter(|| {
            let _: DataPayload<HelloWorldV1Marker> = black_box(&provider as &dyn BufferProvider)
                .as_deserializing()
                .load(DataRequest {
                    locale: &langid!("ru").into(),
                    metadata: Default::default(),
                })
                .and_then(DataResponse::take_payload)
                .expect("Loading was successful");
        });
    });
}

#[cfg(feature = "bench")]
fn blob_bench(c: &mut Criterion) {
    fn create() -> BlobDataProvider {
        icu_provider_blob::BlobDataProvider::try_new_from_blob(black_box(BLOB.into())).unwrap()
    }

    c.bench_function("blob/create", |b| b.iter(create));

    let provider = create();

    c.bench_function("blob/generic", |b| {
        b.iter(|| {
            let _: DataPayload<HelloWorldV1Marker> = black_box(&provider)
                .as_deserializing()
                .load(DataRequest {
                    locale: &langid!("ru").into(),
                    metadata: Default::default(),
                })
                .and_then(DataResponse::take_payload)
                .expect("Loading was successful");
        });
    });

    c.bench_function("blob/erased_serde", |b| {
        b.iter(|| {
            let _: DataPayload<HelloWorldV1Marker> = black_box(&provider as &dyn BufferProvider)
                .as_deserializing()
                .load(DataRequest {
                    locale: &langid!("ru").into(),
                    metadata: Default::default(),
                })
                .expect("The data should be valid")
                .take_payload()
                .expect("Loading was successful");
        });
    });
}

criterion_group!(benches, overview_bench,);
criterion_main!(benches);
