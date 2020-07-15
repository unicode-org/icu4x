use criterion::{criterion_group, criterion_main, Criterion};
use icu_unicodeset::UnicodeSet;
use std::{
    char::{from_u32, MAX},
    convert::TryFrom,
};

fn contains_bench(c: &mut Criterion) {
    let best_ex = vec![65, 70];
    let best_sample = UnicodeSet::try_from(best_ex).unwrap();
    let worst_ex: Vec<u32> = (0..((MAX as u32) + 1)).collect();
    let worst_sample = UnicodeSet::try_from(worst_ex).unwrap();

    let mut group = c.benchmark_group("uniset/contains");
    group.bench_with_input("best", &best_sample, |b, sample| {
        b.iter(|| sample.iter().map(|ch| sample.contains(ch)))
    });
    group.bench_with_input("worst", &worst_sample, |b, sample| {
        b.iter(|| sample.iter().take(100).map(|ch| sample.contains(ch)))
    });
    group.finish();
}

fn contains_range_bench(c: &mut Criterion) {
    let best_ex = vec![65, 70];
    let best_sample = UnicodeSet::try_from(best_ex).unwrap();
    let worst_ex: Vec<u32> = (0..((MAX as u32) + 1)).collect();
    let worst_sample = UnicodeSet::try_from(worst_ex).unwrap();

    let mut group = c.benchmark_group("uniset/contains_range");
    group.bench_with_input("best", &best_sample, |b, sample| {
        b.iter(|| sample.iter().map(|ch| sample.contains_range(&('A'..ch))))
    });
    group.bench_with_input("worst", &worst_sample, |b, sample| {
        b.iter(|| {
            sample
                .iter()
                .take(100)
                .map(|ch| sample.contains_range(&(from_u32(0).unwrap()..ch)))
        })
    });
    group.finish();
}

criterion_group!(benches, contains_bench, contains_range_bench);
criterion_main!(benches);
