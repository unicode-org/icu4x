// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use serde_unaligned::uvec::*;

fn overview_bench(c: &mut Criterion) {
    c.bench_function("serde/uvec/overview", |b| {
        b.iter(|| {
            // TODO
        });
    });

    #[cfg(feature = "bench")]
    {
        u32_benches(c);
    }
}

#[cfg(feature = "bench")]
fn u32_benches(c: &mut Criterion) {
    c.bench_function("serde/vec/serialize_u32", |b| {
        b.iter(|| bincode::serialize(&Vec::from(black_box(TEST_SLICE))));
    });

    c.bench_function("serde/vec/deserialize_u32_sum", |b| {
        let buffer = bincode::serialize(&Vec::from(TEST_SLICE)).unwrap();
        b.iter(|| {
            bincode::deserialize::<Vec<u32>>(&buffer)
                .unwrap()
                .iter()
                .sum::<u32>()
        });
    });

    c.bench_function("serde/uvec/serialize_u32", |b| {
        b.iter(|| bincode::serialize(&UVec::from(black_box(TEST_SLICE))));
    });

    c.bench_function("serde/uvec/deserialize_u32_sum", |b| {
        let buffer = bincode::serialize(&UVec::from(TEST_SLICE)).unwrap();
        b.iter(|| bincode::deserialize::<UVec<u32>>(&buffer).unwrap().sum());
    });
}

criterion_group!(benches, overview_bench,);
criterion_main!(benches);
