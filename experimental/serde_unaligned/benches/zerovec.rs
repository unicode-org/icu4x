// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::SeedableRng;
use rand_distr::{Distribution, LogNormal};
use rand_pcg::Lcg64Xsh32;
use std::fmt;

use serde_unaligned::ule::*;
use serde_unaligned::ZeroVec;
use serde_unaligned::ule::samples::*;

#[repr(align(8))]
#[derive(Default)]
struct AlignedBuffer(Vec<u8>);

fn vec_to_unaligned_uvec<'a, T>(vec: &Vec<T>, buffer: &'a mut AlignedBuffer) -> ZeroVec<'a, T>
where
    T: AsULE + Copy + PartialEq + fmt::Debug,
    <<T as AsULE>::ULE as ULE>::Error: fmt::Debug,
{
    // Pad with zero to ensure it is not aligned
    buffer.0.push(0);
    buffer.0.extend(ZeroVec::from_aligned(vec.as_slice()).as_bytes());
    ZeroVec::<T>::try_from_bytes(&buffer.0[1..]).unwrap()
}

fn overview_bench(c: &mut Criterion) {
    c.bench_function("zerovec/overview", |b| {
        b.iter(|| {
            ZeroVec::<u32>::try_from_bytes(black_box(&TEST_BUFFER_LE))
                .unwrap()
                .iter()
                .sum::<u32>()
        });
    });

    #[cfg(feature = "bench")]
    {
        sum_benches(c);
        binary_search_benches(c);
    }
}

#[cfg(feature = "bench")]
fn sum_benches(c: &mut Criterion) {
    c.bench_function("zerovec/sum/sample/slice", |b| {
        b.iter(|| black_box(&TEST_SLICE).iter().sum::<u32>());
    });

    c.bench_function("zerovec/sum/sample/zerovec", |b| {
        b.iter(|| {
            ZeroVec::<u32>::try_from_bytes(black_box(&TEST_BUFFER_LE)).unwrap().iter().sum::<u32>()
        });
    });
}

#[cfg(feature = "bench")]
fn binary_search_benches(c: &mut Criterion) {
    c.bench_function("zerovec/binary_search/sample/slice", |b| {
        b.iter(|| black_box(&TEST_SLICE).binary_search(&0x0c0d0c));
    });

    c.bench_function("zerovec/binary_search/sample/zerovec", |b| {
        let uvec = ZeroVec::<u32>::try_from_bytes(black_box(&TEST_BUFFER_LE)).unwrap();
        b.iter(|| uvec.binary_search(&0x0c0d0c));
    });

    // Generate a large list of u32s for stress testing.
    // Lcg64Xsh32 is a PRNG with a fixed seed for reproducible benchmarks.
    // LogNormal(10, 1) generates numbers with mean 36315 and mode 8103, a distribution that, in
    // spirit, correlates with Unicode properties (many low values and a long tail of high values)
    let mut rng = Lcg64Xsh32::seed_from_u64(2021);
    let dist = LogNormal::new(10.0, 1.0).unwrap();
    let haystack = {
        let mut unsorted: Vec<u32> = (&dist)
            .sample_iter(&mut rng)
            .take(1000)
            .map(|f| f as u32)
            .collect();
        unsorted.sort();
        unsorted
    };
    let needles: Vec<u32> = (&dist)
        .sample_iter(&mut rng)
        .take(100)
        .map(|f| f as u32)
        .collect();

    c.bench_function("zerovec/binary_search/log_normal/slice", |b| {
        b.iter(|| {
            black_box(&needles)
                .iter()
                .map(|needle| black_box(&haystack).binary_search(&needle))
                .filter(|r| r.is_ok())
                .count()
        });
    });

    c.bench_function("zerovec/binary_search/log_normal/zerovec", |b| {
        let mut buffer = AlignedBuffer::default();
        let uvec = vec_to_unaligned_uvec(black_box(&haystack), &mut buffer);
        assert_eq!(uvec, haystack.as_slice());
        b.iter(|| {
            black_box(&needles)
                .iter()
                .map(|needle| black_box(&uvec).binary_search(&needle))
                .filter(|r| r.is_ok())
                .count()
        });
    });
}

criterion_group!(benches, overview_bench,);
criterion_main!(benches);
