// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

extern crate alloc;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use icu_provider::hello_world::*;
use icu_provider::prelude::*;
use icu_provider_blob::BlobDataProvider;

const BLOB_V3: &[u8] = include_bytes!("../tests/data/v3.postcard");

fn blob_version_bench(c: &mut Criterion) {
    c.bench_function("provider/construct/v3", |b| {
        b.iter(|| BlobDataProvider::try_new_from_static_blob(black_box(BLOB_V3)).unwrap());
    });

    let hello_world_provider = HelloWorldProvider;
    let locales = hello_world_provider.iter_ids().unwrap();

    c.bench_function("provider/read/v3", |b| {
        let provider = BlobDataProvider::try_new_from_static_blob(black_box(BLOB_V3)).unwrap();
        b.iter(|| {
            for id in black_box(&locales).iter() {
                black_box(&provider)
                    .load_data(
                        HelloWorldV1::INFO,
                        DataRequest {
                            id: id.as_borrowed(),
                            ..Default::default()
                        },
                    )
                    .unwrap();
            }
        });
    });
}

criterion_group!(benches, blob_version_bench,);
criterion_main!(benches);
