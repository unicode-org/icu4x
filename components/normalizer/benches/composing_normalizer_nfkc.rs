// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use criterion::{black_box, BenchmarkId, Criterion, Throughput};

use icu_normalizer::ComposingNormalizer;

fn function_under_bench(normalizer: &ComposingNormalizer, text: &str) {
    normalizer.normalize(text);
}

pub fn criterion_benchmark(criterion: &mut Criterion) {
    let group_name = "composing_normalizer_nfkc";
    let normalizer: ComposingNormalizer =
        ComposingNormalizer::try_new_nfkc_unstable(&icu_testdata::unstable()).unwrap();
    let params = black_box([ "a\u{0308}", "A\u{0308}", "e\u{0323}\u{0302}", ]);

    let mut group = criterion.benchmark_group(group_name);
    for text in params {
        group.throughput(Throughput::Elements(1));
        group.bench_with_input(BenchmarkId::new(group_name, text), text, |bencher, text| {
            bencher.iter(|| function_under_bench(&normalizer, text))
        });
    }
    group.finish();
}
