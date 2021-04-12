// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use serde_unaligned::uvec::*;

fn overview_bench(c: &mut Criterion) {
    c.bench_function("uvec/overview", |b| {
        b.iter(|| {
            // TODO
        });
    });

    #[cfg(feature = "bench")]
    {
        sum_benches(c);
    }
}

#[cfg(feature = "bench")]
fn sum_benches(c: &mut Criterion) {
    c.bench_function("uvec/sum/u32_slice", |b| {
        b.iter(|| UVec::from(black_box(TEST_SLICE)).sum());
    });

    c.bench_function("uvec/sum/u8_buffer", |b| {
        b.iter(|| {
            UVec::<u32>::from_unaligned_le_bytes(black_box(&TEST_BUFFER_LE))
                .unwrap()
                .sum()
        });
    });

    c.bench_function("uvec/sum_u32/u32_slice", |b| {
        b.iter(|| UVec::from(black_box(TEST_SLICE)).sum_u32());
    });

    c.bench_function("uvec/sum_u32/u8_buffer", |b| {
        b.iter(|| {
            UVec::<u32>::from_unaligned_le_bytes(black_box(&TEST_BUFFER_LE))
                .unwrap()
                .sum_u32()
        });
    });
}

criterion_group!(benches, overview_bench,);
criterion_main!(benches);
