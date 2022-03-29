// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use icu_locid::langid;
use icu_plurals::provider::*;
use icu_provider::prelude::*;
use icu_provider_fs::FsDataProvider;

fn overview_bench(c: &mut Criterion) {
    // End-to-end JSON test
    c.bench_function("json/overview", |b| {
        b.iter(|| {
            let provider = FsDataProvider::try_new("./tests/testdata/json")
                .expect("Loading file from testdata directory");
            let _: DataPayload<CardinalV1Marker> = black_box(&provider)
                .load_resource(&DataRequest {
                    options: locale!("ru").into(),
                    metadata: Default::default(),
                })
                .expect("The data should be valid")
                .take_payload()
                .expect("Loading was successful");
        });
    });

    #[cfg(feature = "bench")]
    {
        json_bench(c);
        #[cfg(feature = "deserialize_bincode_1")]
        {
            bincode_bench(c);
        }
    }
}

#[cfg(feature = "bench")]
fn json_bench(c: &mut Criterion) {
    let provider = FsDataProvider::try_new("./tests/testdata/json")
        .expect("Loading file from testdata directory");

    c.bench_function("json/generic", |b| {
        b.iter(|| {
            let _: DataPayload<CardinalV1Marker> = black_box(&provider)
                .load_resource(&DataRequest {
                    options: locale!("ru").into(),
                    metadata: Default::default(),
                })
                .expect("The data should be valid")
                .take_payload()
                .expect("Loading was successful");
        });
    });

    c.bench_function("json/erased_serde", |b| {
        b.iter(|| {
            let _: DataPayload<CardinalV1Marker> = black_box(&provider as &dyn BufferProvider)
                .as_deserializing()
                .load_resource(&DataRequest {
                    options: locale!("ru").into(),
                    metadata: Default::default(),
                })
                .expect("The data should be valid")
                .take_payload()
                .expect("Loading was successful");
        });
    });
}

#[cfg(all(feature = "bench", feature = "deserialize_bincode_1"))]
fn bincode_bench(c: &mut Criterion) {
    let provider = FsDataProvider::try_new("./tests/testdata/bincode")
        .expect("Loading file from testdata directory");

    c.bench_function("bincode/generic", |b| {
        b.iter(|| {
            let _: DataPayload<CardinalV1Marker> = black_box(&provider)
                .load_resource(&DataRequest {
                    options: locale!("sr").into(),
                    metadata: Default::default(),
                })
                .expect("The data should be valid")
                .take_payload()
                .expect("Loading was successful");
        });
    });

    c.bench_function("bincode/erased_serde", |b| {
        b.iter(|| {
            let _: DataPayload<CardinalV1Marker> = black_box(&provider as &dyn BufferProvider)
                .as_deserializing()
                .load_resource(&DataRequest {
                    options: locale!("sr").into(),
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
