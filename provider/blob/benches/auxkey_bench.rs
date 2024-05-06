// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

extern crate alloc;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use icu_provider::prelude::*;
use icu_provider_blob::BlobDataProvider;
use icu_datagen::prelude::*;
use icu_provider_blob::export::BlobExporter;
use icu_datetime::provider::neo::GregorianDateNeoSkeletonPatternsV1Marker;

struct Baked;

const _: () = {
    pub mod icu {
        pub use icu_datetime as datetime;
        pub use icu_locid_transform as locid_transform;
    }
    icu_datetime_data::make_provider!(Baked);
    icu_datetime_data::impl_datetime_patterns_gregory_skeleton_v1!(Baked);
    icu_datetime_data::impliterable_datetime_patterns_gregory_skeleton_v1!(Baked);
};

icu_provider::make_exportable_provider!(Baked, [GregorianDateNeoSkeletonPatternsV1Marker,]);

fn make_blob_v1() -> Vec<u8> {
    let mut blob: Vec<u8> = Vec::new();
    let exporter = BlobExporter::new_with_sink(Box::new(&mut blob));
    DatagenDriver::new()
        .with_keys([GregorianDateNeoSkeletonPatternsV1Marker::KEY])
        .with_locales_and_fallback([LocaleFamily::FULL], Default::default())
        .export(&Baked, exporter)
        .unwrap();
    assert!(blob.len() > 100);
    blob
}

fn make_blob_v2() -> Vec<u8> {
    let mut blob: Vec<u8> = Vec::new();
    let exporter = BlobExporter::new_v2_with_sink(Box::new(&mut blob));
    DatagenDriver::new()
        .with_keys([GregorianDateNeoSkeletonPatternsV1Marker::KEY])
        .with_locales_and_fallback([LocaleFamily::FULL], Default::default())
        .export(&Baked, exporter)
        .unwrap();
    assert!(blob.len() > 100);
    blob
}


fn auxkey_bench(c: &mut Criterion) {
    let blob_v1 = make_blob_v1();
    let blob_v1 = blob_v1.as_slice();
    let blob_v2 = make_blob_v2();
    let blob_v2 = blob_v2.as_slice();

    c.bench_function("provider/auxkey/construct/v1", |b| {
        b.iter(|| BlobDataProvider::try_new_from_blob(black_box(blob_v1).into()).unwrap());
    });
    c.bench_function("provider/auxkey/construct/v2", |b| {
        b.iter(|| BlobDataProvider::try_new_from_blob(black_box(blob_v2).into()).unwrap());
    });
}

criterion_group!(benches, auxkey_bench,);
criterion_main!(benches);
