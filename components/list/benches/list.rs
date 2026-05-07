// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use icu_list::{
    options::{ListFormatterOptions, ListLength},
    ListFormatter,
};
use icu_locale::locale;
use writeable::Writeable;

fn list_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("list");

    let locales = [locale!("en"), locale!("es"), locale!("th")];

    // Benchmark construction
    group.bench_function("construct/and/wide", |b| {
        b.iter(|| {
            for locale in black_box(&locales) {
                let _ = ListFormatter::try_new_and(
                    locale.into(),
                    ListFormatterOptions::default().with_length(black_box(ListLength::Wide)),
                );
            }
        })
    });

    group.bench_function("construct/or/short", |b| {
        b.iter(|| {
            for locale in black_box(&locales) {
                let _ = ListFormatter::try_new_or(
                    locale.into(),
                    ListFormatterOptions::default().with_length(black_box(ListLength::Short)),
                );
            }
        })
    });

    group.bench_function("construct/unit/narrow", |b| {
        b.iter(|| {
            for locale in black_box(&locales) {
                let _ = ListFormatter::try_new_unit(
                    locale.into(),
                    ListFormatterOptions::default().with_length(black_box(ListLength::Narrow)),
                );
            }
        })
    });

    // Benchmark formatting
    let list_2 = generate_list(2);
    let list_5 = generate_list(5);
    let list_1000 = generate_list(1000);

    let locales = [
        (locale!("en"), "en"),
        (locale!("es"), "es"),
        (locale!("ar"), "ar"),
    ];

    let mut result = String::with_capacity(10000);

    for (locale, locale_str) in locales {
        let formatter = ListFormatter::try_new_and(
            locale.clone().into(),
            ListFormatterOptions::default().with_length(ListLength::Wide),
        )
        .unwrap();

        group.bench_function(format!("format/and/wide/{locale_str}/2_items"), |b| {
            b.iter(|| {
                result.clear();
                let _ = black_box(&formatter)
                    .format(black_box(&list_2).iter())
                    .write_to(&mut result);
            })
        });

        group.bench_function(format!("format/and/wide/{locale_str}/5_items"), |b| {
            b.iter(|| {
                result.clear();
                let _ = black_box(&formatter)
                    .format(black_box(&list_5).iter())
                    .write_to(&mut result);
            })
        });

        group.bench_function(format!("format/and/wide/{locale_str}/1000_items"), |b| {
            b.iter(|| {
                result.clear();
                let _ = black_box(&formatter)
                    .format(black_box(&list_1000).iter())
                    .write_to(&mut result);
            })
        });
    }

    // Adversarial Spanish Benchmarks
    let es_locale = locale!("es");

    // Test 'and' (y -> e)
    let formatter_and = ListFormatter::try_new_and(
        es_locale.clone().into(),
        ListFormatterOptions::default().with_length(ListLength::Wide),
    )
    .unwrap();

    let list_higo = vec!["manzanas".to_string(), "higos".to_string()];
    let list_hielo = vec!["leche".to_string(), "hielo".to_string()];

    group.bench_function("format/and/wide/es/higo", |b| {
        b.iter(|| {
            result.clear();
            let _ = black_box(&formatter_and)
                .format(black_box(&list_higo).iter())
                .write_to(&mut result);
        })
    });

    group.bench_function("format/and/wide/es/hielo", |b| {
        b.iter(|| {
            result.clear();
            let _ = black_box(&formatter_and)
                .format(black_box(&list_hielo).iter())
                .write_to(&mut result);
        })
    });

    // Test 'or' (o -> u)
    let formatter_or = ListFormatter::try_new_or(
        es_locale.clone().into(),
        ListFormatterOptions::default().with_length(ListLength::Wide),
    )
    .unwrap();

    let list_8 = vec!["7".to_string(), "8".to_string()];
    let list_11 = vec!["10".to_string(), "11".to_string()];
    let list_110 = vec!["10".to_string(), "110".to_string()];

    group.bench_function("format/or/wide/es/8", |b| {
        b.iter(|| {
            result.clear();
            let _ = black_box(&formatter_or)
                .format(black_box(&list_8).iter())
                .write_to(&mut result);
        })
    });

    group.bench_function("format/or/wide/es/11", |b| {
        b.iter(|| {
            result.clear();
            let _ = black_box(&formatter_or)
                .format(black_box(&list_11).iter())
                .write_to(&mut result);
        })
    });

    group.bench_function("format/or/wide/es/110", |b| {
        b.iter(|| {
            result.clear();
            let _ = black_box(&formatter_or)
                .format(black_box(&list_110).iter())
                .write_to(&mut result);
        })
    });

    group.finish();
}

fn generate_list(n: usize) -> Vec<String> {
    (0..n).map(|i| format!("Item {}", i)).collect()
}

criterion_group!(benches, list_benches);
criterion_main!(benches);
