// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use criterion::{BenchmarkId, black_box, Criterion, Throughput};

use icu_normalizer::DecomposingNormalizer;

fn function_under_bench(normalizer: &DecomposingNormalizer, text: &str) {
    normalizer.normalize(text);
}

pub fn criterion_benchmark(criterion: &mut Criterion) {
    let group_name = "decomposing_normalizer_nfd";
    let normalizer: DecomposingNormalizer =
        DecomposingNormalizer::try_new_nfd_unstable(&icu_testdata::unstable()).unwrap();
    let params = black_box([ "a", "ä", "Ä", ]);

    let mut group = criterion.benchmark_group(group_name);
    for text in params {
        group.throughput(Throughput::Elements(1));
        group.bench_with_input(BenchmarkId::new(group_name, text), text, |bencher, text| {
            bencher.iter(|| function_under_bench(&normalizer, text))
        });
    }
    group.finish();
}
