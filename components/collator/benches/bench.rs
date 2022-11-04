// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::time::Duration;

use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};

use icu::collator::*;
use icu::locid::Locale;


pub fn collator_with_locale(criterion: &mut Criterion) {

    // Load file content in reverse order vector.
    let content_latin: (&str, Vec<&str>) = (
        "TestNames_Latin",
        include_str!("data/TestNames_Latin.txt")
            .split('\n')
            .rev()
            .collect::<Vec<&str>>(),
    );
    let content_asian: (&str, Vec<&str>) = (
        "TestNames_Asian",
        include_str!("data/TestNames_Asian.txt")
            .split('\n')
            .rev()
            .collect(),
    );
    let content_russian: (&str, Vec<&str>) = (
        "TestNames_Russian",
        include_str!("data/TestNames_Russian.txt")
            .split('\n')
            .rev()
            .collect(),
    );

    let performance_parameters = [
        ("en_US", vec![&content_latin]),
        ("fr_FR", vec![&content_latin]),
        ("ja_JP", vec![&content_latin, &content_asian]),
        ("zh_CN", vec![&content_latin]),
        ("ru_RU", vec![&content_latin, &content_russian]),
    ];

    for perf_group in performance_parameters {
        let (locale_under_test, files_under_test) = perf_group;
        let current_locale: Locale = locale_under_test.parse().expect("Failed to parse locale.");

        let mut options = CollatorOptions::new();
        options.strength = Some(Strength::Primary);
        let collator: Collator =
            Collator::try_new_unstable(&icu_testdata::unstable(), &current_locale.into(), options)
                .unwrap();

        let mut group = criterion.benchmark_group(locale_under_test);

        for content_under_test in files_under_test {
            let (file_name, elements) = content_under_test;

            // baseline performance, normal sort done by rust libraries.
            group.bench_function(BenchmarkId::new("rust_sort", file_name), |bencher| {
                bencher.iter_batched(
                    || elements.clone(),
                    |mut lines| lines.sort(),
                    BatchSize::SmallInput,
                )
            });

            // ICU collator performance
            group.bench_function(BenchmarkId::new("icu_sort",file_name), |bencher| {
                bencher.iter_batched(
                    || elements.clone(),
                    |mut lines| lines.sort_by(|left, right| collator.compare(left, right)),
                    BatchSize::SmallInput,
                )
            });
        }
        group.finish();
    }
}

criterion_group!(
    name = benches;
    config = Criterion::default()
        .measurement_time(Duration::new(60, 0))
        .sample_size(100);
    targets = collator_with_locale
);

criterion_main!(benches);
