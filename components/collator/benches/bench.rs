// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group, criterion_main, Throughput};

use icu::collator::*;
use icu::locid::Locale;

pub fn collator_with_locale(criterion: &mut Criterion) {
    // Load all file content in the executable.
    let content_latin: (&str, Vec<&str>) = ("TestNames_Latin", include_str!("data/TestNames_Latin.txt").split("\n").collect());
    let content_japanese_h: (&str, Vec<&str>) = ("TestNames_Japanese_h", include_str!("data/TestNames_Japanese_h.txt").split("\n").collect());
    let content_japanese_k: (&str, Vec<&str>) = ("TestNames_Japanese_k", include_str!("data/TestNames_Japanese_k.txt").split("\n").collect());
    let content_asian: (&str, Vec<&str>) = ("TestNames_Asian", include_str!("data/TestNames_Asian.txt").split("\n").collect());
    let content_chinese: (&str, Vec<&str>) = ("TestNames_Chinese", include_str!("data/TestNames_Chinese.txt").split("\n").collect());
    let content_russian: (&str, Vec<&str>) = ("TestNames_Russian", include_str!("data/TestNames_Russian.txt").split("\n").collect());
    let content_thai: (&str, Vec<&str>) = ("TestNames_Thai", include_str!("data/TestNames_Thai.txt").split("\n").collect());
    let content_korean: (&str, Vec<&str>) = ("TestNames_Korean", include_str!("data/TestNames_Korean.txt").split("\n").collect());

    let performance_parameters = [
        ("en_US", vec![&content_latin]),
        ("da_DK", vec![&content_latin]),
        ("de_DE", vec![&content_latin]),
        // ("de__PHONEBOOK", vec![&content_latin]),
        ("fr_FR", vec![&content_latin]),
        ("ja_JP", vec![&content_latin, &content_japanese_h, &content_japanese_k, &content_asian]),
        ("zh_CN", vec![&content_latin, &content_chinese]),
        ("zh_TW", vec![&content_latin, &content_chinese]),
        ("zh__PINYIN", vec![&content_latin, &content_chinese]),
        ("ru_RU", vec![&content_latin, &content_russian]),
        ("th", vec![&content_latin, &content_thai]),
        ("ko_KR", vec![&content_latin, &content_korean]),
    ];

    for perf_group in performance_parameters {
        let (locale_under_test, content_under_test) = perf_group;
        let current_locale: Locale = locale_under_test.parse()
            .expect("Failed to parse locale.");

        let mut options = CollatorOptions::new();
        options.strength = Some(Strength::Primary);
        let collator: Collator =
            Collator::try_new_unstable(&icu_testdata::unstable(), &current_locale.into(), options).unwrap();

        let mut group = criterion.benchmark_group(locale_under_test);

        for lines_under_test in content_under_test {
            let (file_name, elements) = lines_under_test;
            group.throughput(Throughput::Elements(elements.len() as u64));
            group.bench_function(
                BenchmarkId::from_parameter(file_name),
                |bencher| bencher.iter_batched_ref(
                    || -> Vec<&str> { elements.clone() },
                    |lines| lines.sort_by(|left, right| collator.compare(left, right)),
                    BatchSize::SmallInput,
                )
            );
        }
        group.finish();
    }
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(20);
    targets = collator_with_locale
);

criterion_main!(benches);
