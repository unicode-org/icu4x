// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

extern crate alloc;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use icu_locid::locale;
use icu_provider::hello_world::HelloWorldV1Marker;
use icu_provider::prelude::*;
use icu_provider::AsDeserializingBufferProvider;
use icu_provider_blob::BlobDataProvider;

static POSTCARD_BYTES: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/data/testdata.postcard"
));

struct BakedDataProvider;

mod baked {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/baked/mod.rs"));
    impl_data_provider!(super::BakedDataProvider);
    impl_any_provider!(super::BakedDataProvider);
}

#[inline(never)]
fn create_static_data_provider() -> BlobDataProvider {
    icu_provider_blob::BlobDataProvider::try_new_from_static_blob(POSTCARD_BYTES).unwrap()
}

#[inline(never)]
fn create_blob_data_provider() -> BlobDataProvider {
    icu_provider_blob::BlobDataProvider::try_new_from_blob(black_box(POSTCARD_BYTES.into()))
        .unwrap()
}

#[inline(never)]
fn create_baked_data_provider() -> BakedDataProvider {
    BakedDataProvider
}

#[inline(never)]
fn create_baked_any_provider() -> BakedDataProvider {
    BakedDataProvider
}

fn providers_bench(c: &mut Criterion) {
    c.bench_function("provider/construct/static", |b| {
        b.iter(create_static_data_provider);
    });
    c.bench_function("provider/construct/blob", |b| {
        b.iter(create_blob_data_provider);
    });
    c.bench_function("provider/construct/baked", |b| {
        b.iter(create_baked_data_provider);
    });
    c.bench_function("provider/construct/any", |b| {
        b.iter(create_baked_data_provider);
    });

    c.bench_function("provider/load/static", |b| {
        let provider = create_static_data_provider();
        b.iter(|| {
            let result: DataResponse<HelloWorldV1Marker> = provider
                .as_deserializing()
                .load(DataRequest {
                    locale: &locale!("ja").into(),
                    metadata: Default::default(),
                })
                .unwrap();
            result
        });
    });
    c.bench_function("provider/load/blob", |b| {
        let provider = create_blob_data_provider();
        b.iter(|| {
            let result: DataResponse<HelloWorldV1Marker> = provider
                .as_deserializing()
                .load(DataRequest {
                    locale: &locale!("ja").into(),
                    metadata: Default::default(),
                })
                .unwrap();
            result
        });
    });
    c.bench_function("provider/load/baked", |b| {
        let provider = create_baked_data_provider();
        b.iter(|| {
            let result: DataResponse<HelloWorldV1Marker> = provider
                .load(DataRequest {
                    locale: &locale!("ja").into(),
                    metadata: Default::default(),
                })
                .unwrap();
            result
        });
    });
    c.bench_function("provider/load/any", |b| {
        let provider = create_baked_any_provider();
        b.iter(|| {
            let result: DataResponse<HelloWorldV1Marker> = provider
                .as_downcasting()
                .load(DataRequest {
                    locale: &locale!("ja").into(),
                    metadata: Default::default(),
                })
                .unwrap();
            result
        });
    });
}

criterion_group!(benches, providers_bench,);
criterion_main!(benches);
