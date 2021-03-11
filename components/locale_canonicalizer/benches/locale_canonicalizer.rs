// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).
mod helpers;

use criterion::{criterion_group, criterion_main, Criterion};
use icu_locale_canonicalizer::LocaleCanonicalizer;
use icu_locid::Locale;

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

criterion_group!(benches, maximize_bench);
criterion_main!(benches);
