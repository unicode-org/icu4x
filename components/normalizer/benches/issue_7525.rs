// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

fn generate_benchmark_data() -> Vec<(&'static str, String)> {
    vec![
        ("long_ascii", "a".repeat(10000)),
        ("frequent_exits", "a\u{0300}".repeat(1000)),
        ("mixed_text", {
            let mut s = String::new();
            for _ in 0..100 {
                s.push_str("hello ");
                s.push_str("café");
                s.push_str(" world ");
            }
            s
        }),
        ("english_accents", {
            let mut s = String::new();
            for _ in 0..100 {
                s.push_str("The quick brown fox jumps over the lazy dog. ");
                s.push_str("Café résumé naïve. ");
            }
            s
        }),
        ("pathological_alternating", {
            let mut s = String::new();
            for _ in 0..1000 {
                s.push('a');
                s.push('\u{0300}');
            }
            s
        }),
        (
            "normalized_ascii",
            "The quick brown fox jumps over the lazy dog. ".repeat(200),
        ),
        ("short_ascii", "hello".to_string()),
        ("short_with_exit", "café".to_string()),
    ]
}

fn bench_normalize_to(c: &mut Criterion) {
    let normalizer = icu_normalizer::ComposingNormalizerBorrowed::new_nfc();
    let data = generate_benchmark_data();

    let mut group = c.benchmark_group("issue_7525_normalize_to");

    for (name, text) in data {
        group.bench_with_input(BenchmarkId::new("normalize_to", name), &text, |b, text| {
            b.iter(|| {
                let mut sink = String::new();
                normalizer.normalize_to(black_box(text), &mut sink).unwrap();
                black_box(sink)
            })
        });
    }

    group.finish();
}

criterion_group!(benches, bench_normalize_to);
criterion_main!(benches);
