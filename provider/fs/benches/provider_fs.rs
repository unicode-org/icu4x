// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use icu_locid::langid;
use icu_provider::hello_world::HelloWorldV1Marker;
use icu_provider::prelude::*;
use icu_provider_fs::FsDataProvider;

fn overview_bench(c: &mut Criterion) {
    // End-to-end JSON test
    c.bench_function("json/overview", |b| {
        b.iter(|| {
            let provider = FsDataProvider::try_new("./tests/data/json")
                .expect("Loading file from testdata directory");
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
        json_bench(c);
        bincode_bench(c);
        postcard_bench(c);
    }
}

#[cfg(feature = "bench")]
fn json_bench(c: &mut Criterion) {
    let provider =
        FsDataProvider::try_new("./tests/data/json").expect("Loading file from testdata directory");

    c.bench_function("json/generic", |b| {
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

    c.bench_function("json/erased_serde", |b| {
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
fn bincode_bench(c: &mut Criterion) {
    let provider = FsDataProvider::try_new("./tests/data/bincode")
        .expect("Loading file from testdata directory");

    c.bench_function("bincode/generic", |b| {
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

    c.bench_function("bincode/erased_serde", |b| {
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

#[cfg(feature = "bench")]
fn postcard_bench(c: &mut Criterion) {
    let provider = FsDataProvider::try_new("./tests/data/postcard")
        .expect("Loading file from testdata directory");

    c.bench_function("postcard/generic", |b| {
        b.iter(|| {
            let _: DataPayload<HelloWorldV1Marker> = black_box(&provider)
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

    c.bench_function("postcard/erased_serde", |b| {
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

criterion_group!(benches, overview_bench,);
criterion_main!(benches);
