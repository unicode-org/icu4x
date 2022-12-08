// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use criterion::{BenchmarkId, Criterion};

use icu_normalizer::properties::{CanonicalComposition, CanonicalDecomposition, Decomposed};

include!("bench_data.rs");

fn function_under_bench(
    canonical_composer: &CanonicalComposition,
    composable_points: &Vec<Decomposed>,
) {
    for decomposed in composable_points.iter() {
        match decomposed {
            Decomposed::Expansion(a, b) => {
                canonical_composer.compose(*a, *b);
            }
            _ => {}
        }
    };
}

pub fn criterion_benchmark(criterion: &mut Criterion) {
    let group_name = "canonical_composition";

    let composer = CanonicalComposition::try_new_unstable(&icu_testdata::unstable()).unwrap();
    let decomposer = CanonicalDecomposition::try_new_unstable(&icu_testdata::unstable()).unwrap();

    let mut group = criterion.benchmark_group(group_name);

    for bench_data_content in normalizer_bench_data() {
        let decompose_data = bench_data_content.nfc.chars().map(|c| decomposer.decompose(c)).collect();
        group.bench_function(
            BenchmarkId::from_parameter(format!("from_nfc_{}", bench_data_content.file_name)),
            |bencher| {
                bencher
                    .iter(|| function_under_bench(&composer, &decompose_data))
            },
        );
    }

    group.finish();
}
