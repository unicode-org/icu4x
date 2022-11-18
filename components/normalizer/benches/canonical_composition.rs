// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use criterion::{BenchmarkId, Criterion, Throughput};
use icu_normalizer::properties::CanonicalComposition;

fn function_under_bench(
    canonical_composer: &CanonicalComposition,
    composable_points: &Vec<Vec<char>>,
) {
    composable_points.iter().for_each(|points| {
        canonical_composer.compose(points[0], points[1]);
    });
}

// transform the source part as a vector of characters.
fn as_vec_char(points: &str) -> Vec<char> {
    points
        .split_whitespace()
        .map(|point| u32::from_str_radix(point, 16).unwrap())
        .map(char::from_u32)
        .map(|x| x.unwrap())
        .collect::<Vec<char>>()
}

pub fn criterion_benchmark(criterion: &mut Criterion) {
    let group_name = "canonical_composition";
    let composer = CanonicalComposition::try_new_unstable(&icu_testdata::unstable()).unwrap();

    let data: Vec<Vec<char>> = include_str!("../tests/data/NormalizationTest.txt")
        .split('\n')
        .filter(|&s| !s.starts_with('#') && !s.starts_with('@') && !s.is_empty()) // remove comments
        .map(|line| &line[..line.find(';').unwrap()]) // split at delimiter.
        .map(as_vec_char)
        .filter(|x| x.len() > 1) // there's no point in composing one char.
        .collect();

    let mut group = criterion.benchmark_group(group_name);

    group.throughput(Throughput::Elements(data.len() as u64));
    group.bench_function(BenchmarkId::from_parameter("icu4x"), |bencher| {
        bencher.iter(|| function_under_bench(&composer, &data))
    });

    group.finish();
}
