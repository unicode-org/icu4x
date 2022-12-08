// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use criterion::{BenchmarkId, Criterion};

include!("bench_data.rs");

fn function_under_bench(normalizer: &ComposingNormalizer, text: &str) {
    normalizer.normalize(text);
}

pub fn criterion_benchmark(criterion: &mut Criterion) {
    let group_name = "composing_normalizer_nfkc";

    let normalizer_under_bench: ComposingNormalizer =
        ComposingNormalizer::try_new_nfkc_unstable(&icu_testdata::unstable()).unwrap();

    let mut group = criterion.benchmark_group(group_name);

    for bench_data_content in normalizer_bench_data() {
        group.bench_function(
            BenchmarkId::from_parameter(format!("from_nfc_{}", bench_data_content.file_name)),
            |bencher| {
                bencher
                    .iter(|| function_under_bench(&normalizer_under_bench, &bench_data_content.nfc))
            },
        );
        group.bench_function(
            BenchmarkId::from_parameter(format!("from_nfd_{}", bench_data_content.file_name)),
            |bencher| {
                bencher
                    .iter(|| function_under_bench(&normalizer_under_bench, &bench_data_content.nfd))
            },
        );
        group.bench_function(
            BenchmarkId::from_parameter(format!("from_nfkc_{}", bench_data_content.file_name)),
            |bencher| {
                bencher.iter(|| {
                    function_under_bench(&normalizer_under_bench, &bench_data_content.nfkc)
                })
            },
        );
        group.bench_function(
            BenchmarkId::from_parameter(format!("from_nfkd_{}", bench_data_content.file_name)),
            |bencher| {
                bencher.iter(|| {
                    function_under_bench(&normalizer_under_bench, &bench_data_content.nfkd)
                })
            },
        );
    }
    group.finish();
}
