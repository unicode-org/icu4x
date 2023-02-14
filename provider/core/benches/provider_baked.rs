// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

extern crate alloc;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use icu_locid::langid;
use icu_provider::hello_world::HelloWorldV1Marker;
use icu_provider::prelude::*;

struct BakedDataProvider;
struct BakedAnyProvider;

mod baked {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/data/baked/mod.rs"
    ));
    impl_data_provider!(super::BakedDataProvider);
    impl_any_provider!(super::BakedAnyProvider);
}

fn overview_bench(c: &mut Criterion) {
    c.bench_function("baked/overview", |b| {
        b.iter(|| {
            let _: DataPayload<HelloWorldV1Marker> = black_box(&BakedDataProvider)
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
        baked_bench(c);
        any_bench(c);
    }
}

#[cfg(feature = "bench")]
fn baked_bench(c: &mut Criterion) {
    c.bench_function("baked", |b| {
        b.iter(|| {
            let _: DataPayload<HelloWorldV1Marker> = black_box(&BakedDataProvider)
                .load(DataRequest {
                    locale: &langid!("ru").into(),
                    metadata: Default::default(),
                })
                .and_then(DataResponse::take_payload)
                .expect("Loading was successful");
        });
    });
}

#[cfg(all(feature = "bench"))]
fn any_bench(c: &mut Criterion) {
    c.bench_function("any/generic", |b| {
        b.iter(|| {
            let _: DataPayload<HelloWorldV1Marker> = black_box(&BakedAnyProvider)
                .as_downcasting()
                .load(DataRequest {
                    locale: &langid!("ru").into(),
                    metadata: Default::default(),
                })
                .and_then(DataResponse::take_payload)
                .expect("Loading was successful");
        });
    });

    c.bench_function("any/erased", |b| {
        b.iter(|| {
            let _: DataPayload<HelloWorldV1Marker> =
                black_box(&BakedAnyProvider as &dyn AnyProvider)
                    .as_downcasting()
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
