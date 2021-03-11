// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).
use criterion::{criterion_group, criterion_main, Criterion};
use icu_uniset::UnicodeSet;
use std::char;

fn uniset_bench(c: &mut Criterion) {
    let best_ex = vec![65, 70];
    let best_sample = UnicodeSet::from_inversion_list(best_ex).unwrap();
    let worst_ex: Vec<u32> = (0..((char::MAX as u32) + 1)).collect();
    let worst_sample = UnicodeSet::from_inversion_list(worst_ex).unwrap();

    c.bench_function("uniset/overview", |b| {
        #[allow(clippy::suspicious_map)]
        b.iter(|| {
            best_sample
                .iter()
                .map(|ch| best_sample.contains(ch))
                .count();
            worst_sample
                .iter()
                .map(|ch| worst_sample.contains(ch))
                .count();
            best_sample
                .iter()
                .map(|ch| best_sample.contains_range(&('A'..ch)))
                .count();
            worst_sample
                .iter()
                .take(100)
                .map(|ch| worst_sample.contains_range(&(char::from_u32(0).unwrap()..ch)))
                .count();
        })
    });

    #[cfg(feature = "bench")]
    {
        let mut group = c.benchmark_group("uniset/contains");
        group.bench_with_input("best", &best_sample, |b, sample| {
            b.iter(|| sample.iter().map(|ch| sample.contains(ch)))
        });
        group.bench_with_input("worst", &worst_sample, |b, sample| {
            b.iter(|| sample.iter().take(100).map(|ch| sample.contains(ch)))
        });
        group.finish();

        let mut group = c.benchmark_group("uniset/contains_range");
        group.bench_with_input("best", &best_sample, |b, sample| {
            b.iter(|| sample.iter().map(|ch| sample.contains_range(&('A'..ch))))
        });
        group.bench_with_input("worst", &worst_sample, |b, sample| {
            b.iter(|| {
                sample
                    .iter()
                    .take(100)
                    .map(|ch| sample.contains_range(&(char::from_u32(0).unwrap()..ch)))
            })
        });
        group.finish();
    }
}

criterion_group!(benches, uniset_bench);
criterion_main!(benches);
