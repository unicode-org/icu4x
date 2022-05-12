// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod helpers;

use criterion::{criterion_group, criterion_main, Criterion};
use icu_locale_canonicalizer::LocaleCanonicalizer;
use icu_locid::Locale;

fn canonicalize_bench(c: &mut Criterion) {
    let provider = icu_testdata::get_provider();
    let lc = LocaleCanonicalizer::new(&provider).unwrap();

    let mut group = c.benchmark_group("canonicalize");

    let path = "./benches/fixtures/uncanonicalized-locales.json";
    let data: Vec<String> = helpers::read_fixture(path).expect("Failed to read a fixture");

    group.bench_function("create", |b| {
        b.iter(|| {
            let _: Vec<Locale> = data.iter().map(|s| s.parse().unwrap()).collect();
        })
    });

    group.bench_function("create+canonicalize", |b| {
        b.iter(|| {
            let locales: Vec<Locale> = data.iter().map(|s| s.parse().unwrap()).collect();
            for locale in locales.iter() {
                lc.canonicalize(&mut locale.clone());
            }
        })
    });

    group.finish();
}

fn canonicalize_noop_bench(c: &mut Criterion) {
    let provider = icu_testdata::get_provider();
    let lc = LocaleCanonicalizer::new(&provider).unwrap();

    let mut group = c.benchmark_group("canonicalize-noop");

    // None of these locales require canonicalization, so this measures the cost of calling
    // the canonicalizer on locales that will not be modified.
    let path = "./benches/fixtures/locales.json";
    let data: Vec<String> = helpers::read_fixture(path).expect("Failed to read a fixture");

    group.bench_function("create", |b| {
        b.iter(|| {
            let _: Vec<Locale> = data.iter().map(|s| s.parse().unwrap()).collect();
        })
    });

    group.bench_function("create+canonicalize", |b| {
        b.iter(|| {
            let locales: Vec<Locale> = data.iter().map(|s| s.parse().unwrap()).collect();
            for locale in locales.iter() {
                lc.canonicalize(&mut locale.clone());
            }
        })
    });

    group.finish();
}

fn maximize_bench(c: &mut Criterion) {
    let provider = icu_testdata::get_provider();
    let lc = LocaleCanonicalizer::new(&provider).unwrap();

    let mut group = c.benchmark_group("likelysubtags");

    let path = "./benches/fixtures/locales.json";
    let data: Vec<String> = helpers::read_fixture(path).expect("Failed to read a fixture");

    let locales: Vec<Locale> = data.iter().map(|s| s.parse().unwrap()).collect();

    group.bench_function("maximize", |b| {
        b.iter(|| {
            for locale in locales.iter() {
                lc.maximize(&mut locale.clone());
            }
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    canonicalize_bench,
    canonicalize_noop_bench,
    maximize_bench
);
criterion_main!(benches);
