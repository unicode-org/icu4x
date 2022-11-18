// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use criterion::{BenchmarkId, Criterion, Throughput};

use icu_normalizer::properties::CanonicalDecomposition;

fn function_under_bench(
    canonical_decomposer: &CanonicalDecomposition,
    decomposable_points: &Vec<char>,
) {
    decomposable_points.iter().for_each(|&point| {
        canonical_decomposer.decompose(point);
    });
}

// transform the source part as a vector of characters.
fn as_char(points: &str) -> char {
    points
        .split_whitespace()
        .next() // just pick the first one.
        .map(|point| u32::from_str_radix(point, 16).unwrap())
        .map(|x| char::from_u32(x).unwrap())
        .unwrap()
}

pub fn criterion_benchmark(criterion: &mut Criterion) {
    let group_name = "canonical_decomposition";
    let decomposer = CanonicalDecomposition::try_new_unstable(&icu_testdata::unstable()).unwrap();

    let data: Vec<char> = include_str!("../tests/data/NormalizationTest.txt")
        .split('\n')
        .filter(|&s| !s.starts_with("#") && !s.starts_with("@") && !s.is_empty()) // remove comments
        .map(|line| &line[..line.find(';').unwrap()]) // split at delimiter.
        .map(|points| as_char(points))
        .collect();

    let mut group = criterion.benchmark_group(group_name);

    group.throughput(Throughput::Elements(data.len() as u64));
    group.bench_function(BenchmarkId::from_parameter("icu4x"), |bencher| {
        bencher.iter(|| function_under_bench(&decomposer, &data))
    });

    group.finish();
}
