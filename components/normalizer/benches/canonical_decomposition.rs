// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use criterion::{black_box, BenchmarkId, Criterion, Throughput};

use icu_normalizer::properties::CanonicalDecomposition;

fn function_under_bench(canonical_decomposer: &CanonicalDecomposition, character: char) {
    canonical_decomposer.decompose(character);
}

pub fn criterion_benchmark(criterion: &mut Criterion) {
    let group_name = "canonical_decomposition";
    let decomposer = CanonicalDecomposition::try_new_unstable(&icu_testdata::unstable()).unwrap();
    let params = black_box([ 'a', 'ä', 'Ä' ]);

    let mut group = criterion.benchmark_group(group_name);
    for single_char in params {
        group.throughput(Throughput::Elements(1));
        group.bench_with_input(
            BenchmarkId::new(group_name, single_char),
            &single_char,
            |bencher, character| bencher.iter(|| function_under_bench(&decomposer, *character)),
        );
    }
    group.finish();
}
