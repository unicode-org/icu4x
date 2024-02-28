// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

extern crate alloc;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use icu_provider::datagen::IterableDataProvider;
use icu_provider::hello_world::*;
use icu_provider::prelude::*;
use icu_provider_blob::BlobDataProvider;

const BLOB_V1: &[u8] = include_bytes!("../tests/data/v1.postcard");
const BLOB_V2: &[u8] = include_bytes!("../tests/data/v2.postcard");

fn blob_version_bench(c: &mut Criterion) {
    c.bench_function("provider/construct/v1", |b| {
        b.iter(|| BlobDataProvider::try_new_from_static_blob(black_box(BLOB_V1)).unwrap());
    });
    c.bench_function("provider/construct/v2", |b| {
        b.iter(|| BlobDataProvider::try_new_from_static_blob(black_box(BLOB_V1)).unwrap());
    });

    let hello_world_provider = HelloWorldProvider;
    let locales = hello_world_provider.supported_locales().unwrap();

    c.bench_function("provider/read/v1", |b| {
        let provider = BlobDataProvider::try_new_from_static_blob(black_box(BLOB_V1)).unwrap();
        b.iter(|| {
            for locale in black_box(&locales).iter() {
                black_box(&provider)
                    .load_buffer(
                        HelloWorldV1Marker::KEY,
                        DataRequest {
                            locale,
                            metadata: Default::default(),
                        },
                    )
                    .unwrap();
            }
        });
    });
    c.bench_function("provider/read/v2", |b| {
        let provider = BlobDataProvider::try_new_from_static_blob(black_box(BLOB_V2)).unwrap();
        b.iter(|| {
            for locale in black_box(&locales).iter() {
                black_box(&provider)
                    .load_buffer(
                        HelloWorldV1Marker::KEY,
                        DataRequest {
                            locale,
                            metadata: Default::default(),
                        },
                    )
                    .unwrap();
            }
        });
    });
}

criterion_group!(benches, blob_version_bench,);
criterion_main!(benches);
