// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use zerovec::ule::samples::*;
use zerovec::ZeroVec;

fn overview_bench(c: &mut Criterion) {
    c.bench_function("zerovec/serde/overview", |b| {
        // Same as "zerovec/serde/deserialize_sum/u32/zerovec"
        let buffer = bincode::serialize(
            &ZeroVec::<u32>::try_from_bytes(black_box(&TEST_BUFFER_LE)).unwrap(),
        )
        .unwrap();
        b.iter(|| {
            bincode::deserialize::<ZeroVec<u32>>(&buffer)
                .unwrap()
                .iter()
                .sum::<u32>()
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
    c.bench_function("zerovec/serde/serialize/u32/slice", |b| {
        b.iter(|| bincode::serialize(&Vec::from(black_box(TEST_SLICE))));
    });

    c.bench_function("zerovec/serde/deserialize_sum/u32/slice", |b| {
        let buffer = bincode::serialize(&Vec::from(black_box(TEST_SLICE))).unwrap();
        b.iter(|| {
            bincode::deserialize::<Vec<u32>>(&buffer)
                .unwrap()
                .iter()
                .sum::<u32>()
        });
    });

    c.bench_function("zerovec/serde/serialize/u32/zerovec", |b| {
        b.iter(|| bincode::serialize(&ZeroVec::from_aligned(black_box(TEST_SLICE))));
    });

    c.bench_function("zerovec/serde/deserialize_sum/u32/zerovec", |b| {
        let buffer = bincode::serialize(
            &ZeroVec::<u32>::try_from_bytes(black_box(&TEST_BUFFER_LE)).unwrap(),
        )
        .unwrap();
        b.iter(|| {
            bincode::deserialize::<ZeroVec<u32>>(&buffer)
                .unwrap()
                .iter()
                .sum::<u32>()
        });
    });
}

#[cfg(feature = "bench")]
fn char_benches(c: &mut Criterion) {
    const ORIGINAL_CHARS: &[char] = &[
        'ⶢ', '⺇', 'Ⱜ', '◁', '◩', '⌂', '⼅', '⏻', '⢜', '◊', 'ⲫ', '⏷', '◢', '⟉', '℞',
    ];

    c.bench_function("zerovec/serde/serialize/char/slice", |b| {
        b.iter(|| bincode::serialize(&Vec::from(ORIGINAL_CHARS)));
    });

    c.bench_function("zerovec/serde/deserialize/char/slice", |b| {
        let buffer = bincode::serialize(&Vec::from(ORIGINAL_CHARS)).unwrap();
        b.iter(|| bincode::deserialize::<Vec<char>>(&buffer));
    });

    c.bench_function("zerovec/serde/serialize/char/zerovec", |b| {
        b.iter(|| bincode::serialize(&ZeroVec::from_aligned(black_box(ORIGINAL_CHARS))));
    });

    c.bench_function("zerovec/serde/deserialize/char/zerovec", |b| {
        let buffer = bincode::serialize(&ZeroVec::from_aligned(black_box(ORIGINAL_CHARS))).unwrap();
        b.iter(|| bincode::deserialize::<ZeroVec<char>>(&buffer));
    });
}

criterion_group!(benches, overview_bench,);
criterion_main!(benches);
