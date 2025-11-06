// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use icu_locale_core::langid;
use icu_provider::hello_world::HelloWorldV1;
use icu_provider::prelude::*;
use icu_provider_fs::FsDataProvider;

fn overview_bench(c: &mut Criterion) {
    // End-to-end JSON test
    c.bench_function("json/overview", |b| {
        b.iter(|| {
            let provider = FsDataProvider::try_new("./tests/data/json".into())
                .expect("Loading file from testdata directory");
            let _: DataResponse<HelloWorldV1> = black_box(&provider)
                .as_deserializing()
                .load(DataRequest {
                    id: DataIdentifierBorrowed::for_locale(&langid!("ru").into()),
                    ..Default::default()
                })
                .expect("Loading was successful");
        });
    });

    {
        json_bench(c);
        bincode_bench(c);
        postcard_bench(c);
    }
}

fn json_bench(c: &mut Criterion) {
    let provider = FsDataProvider::try_new("./tests/data/json".into())
        .expect("Loading file from testdata directory");

    c.bench_function("json/generic", |b| {
        b.iter(|| {
            let _: DataResponse<HelloWorldV1> = black_box(&provider)
                .as_deserializing()
                .load(DataRequest {
                    id: DataIdentifierBorrowed::for_locale(&langid!("ru").into()),
                    ..Default::default()
                })
                .expect("Loading was successful");
        });
    });

    c.bench_function("json/erased_serde", |b| {
        b.iter(|| {
            let _: DataResponse<HelloWorldV1> = black_box(&provider as &dyn BufferProvider)
                .as_deserializing()
                .load(DataRequest {
                    id: DataIdentifierBorrowed::for_locale(&langid!("ru").into()),
                    ..Default::default()
                })
                .expect("Loading was successful");
        });
    });
}

fn bincode_bench(c: &mut Criterion) {
    let provider = FsDataProvider::try_new("./tests/data/bincode".into())
        .expect("Loading file from testdata directory");

    c.bench_function("bincode/generic", |b| {
        b.iter(|| {
            let _: DataResponse<HelloWorldV1> = black_box(&provider)
                .as_deserializing()
                .load(DataRequest {
                    id: DataIdentifierBorrowed::for_locale(&langid!("ru").into()),
                    ..Default::default()
                })
                .expect("Loading was successful");
        });
    });

    c.bench_function("bincode/erased_serde", |b| {
        b.iter(|| {
            let _: DataResponse<HelloWorldV1> = black_box(&provider as &dyn BufferProvider)
                .as_deserializing()
                .load(DataRequest {
                    id: DataIdentifierBorrowed::for_locale(&langid!("ru").into()),
                    ..Default::default()
                })
                .expect("Loading was successful");
        });
    });
}

fn postcard_bench(c: &mut Criterion) {
    let provider = FsDataProvider::try_new("./tests/data/postcard".into())
        .expect("Loading file from testdata directory");

    c.bench_function("postcard/generic", |b| {
        b.iter(|| {
            let _: DataResponse<HelloWorldV1> = black_box(&provider)
                .as_deserializing()
                .load(DataRequest {
                    id: DataIdentifierBorrowed::for_locale(&langid!("ru").into()),
                    ..Default::default()
                })
                .expect("Loading was successful");
        });
    });

    c.bench_function("postcard/erased_serde", |b| {
        b.iter(|| {
            let _: DataResponse<HelloWorldV1> = black_box(&provider as &dyn BufferProvider)
                .as_deserializing()
                .load(DataRequest {
                    id: DataIdentifierBorrowed::for_locale(&langid!("ru").into()),
                    ..Default::default()
                })
                .expect("Loading was successful");
        });
    });
}

criterion_group!(benches, overview_bench,);
criterion_main!(benches);
