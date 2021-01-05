// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use criterion::{criterion_group, criterion_main, Criterion};
use icu_locale_canonicalizer::LocaleCanonicalizer;
use icu_locid::Locale;

fn maximize_bench(c: &mut Criterion) {
    let provider = icu_testdata::get_provider();
    let lc = LocaleCanonicalizer::new(&provider).unwrap();

    let mut group = c.benchmark_group("likelysubtags");

    let locales: Vec<Locale> = vec![
        "en-US",
        "en-GB",
        "es-AR",
        "it",
        "zh-Hans-CN",
        "de-AT",
        "pl",
        "fr-FR",
        "de-AT",
        "sr-Cyrl-SR",
        "nb-NO",
        "fr-FR",
        "mk",
        "uk",
        "und-PL",
        "und-Latn-AM",
        "ug-Cyrl",
        "sr-ME",
        "mn-Mong",
        "lif-Limb",
    ]
    .iter()
    .map(|s| s.parse().unwrap())
    .collect();

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
