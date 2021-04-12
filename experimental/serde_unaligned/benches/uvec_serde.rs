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
        char_benches(c);
    }
}

#[cfg(feature = "bench")]
fn u32_benches(c: &mut Criterion) {
    c.bench_function("serde/u32/vec/serialize", |b| {
        b.iter(|| bincode::serialize(&Vec::from(black_box(TEST_SLICE))));
    });

    c.bench_function("serde/u32/vec/deserialize_sum", |b| {
        let buffer = bincode::serialize(&Vec::from(TEST_SLICE)).unwrap();
        b.iter(|| {
            bincode::deserialize::<Vec<u32>>(&buffer)
                .unwrap()
                .iter()
                .sum::<u32>()
        });
    });

    c.bench_function("serde/u32/uvec/serialize", |b| {
        b.iter(|| bincode::serialize(&UVec::from(black_box(TEST_SLICE))));
    });

    c.bench_function("serde/u32/uvec/deserialize_sum", |b| {
        let buffer = bincode::serialize(&UVec::from(TEST_SLICE)).unwrap();
        b.iter(|| bincode::deserialize::<UVec<u32>>(&buffer).unwrap().sum());
    });
}

#[cfg(feature = "bench")]
fn char_benches(c: &mut Criterion) {
    const ORIGINAL_CHARS: &[char] = &[
        'ⶢ', '⺇', 'Ⱜ', '◁', '◩', '⌂', '⼅', '⏻', '⢜', '◊', 'ⲫ', '⏷', '◢', '⟉', '℞',
    ];

    c.bench_function("serde/char/vec/serialize", |b| {
        b.iter(|| bincode::serialize(&Vec::from(black_box(ORIGINAL_CHARS))));
    });

    c.bench_function("serde/char/vec/deserialize", |b| {
        let buffer = bincode::serialize(&Vec::from(ORIGINAL_CHARS)).unwrap();
        b.iter(|| bincode::deserialize::<Vec<char>>(&buffer));
    });

    c.bench_function("serde/char/uvec/serialize", |b| {
        b.iter(|| bincode::serialize(&UVec::from(black_box(ORIGINAL_CHARS))));
    });

    c.bench_function("serde/char/uvec/deserialize", |b| {
        let buffer = bincode::serialize(&UVec::from(ORIGINAL_CHARS)).unwrap();
        b.iter(|| bincode::deserialize::<UVec<char>>(&buffer));
    });
}

criterion_group!(benches, overview_bench,);
criterion_main!(benches);
