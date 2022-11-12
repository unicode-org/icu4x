// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use criterion::{BenchmarkId, Criterion, Throughput};

use icu_normalizer::properties::CanonicalDecomposition;

fn chunkify(data: &String) -> Vec<&str> {
    let chunk_size = 1000;
    let nb_chunk = 10;
    assert!(data.len() > nb_chunk * chunk_size);
    let mut result = vec![];

    for i in 1..=nb_chunk {
        let max_length = data.char_indices().nth(i * chunk_size).unwrap().0;
        result.push(&data[..max_length])
    }
    result
}

fn function_under_bench(canonical_decomposer: &CanonicalDecomposition, characters: &str) {
    characters.chars().for_each(|c| {
        canonical_decomposer.decompose(c);
    });
}

pub fn criterion_benchmark(criterion: &mut Criterion) {
    // Load file content in reverse order vector.
    let content_latin: (&str, String) = (
        "TestNames_Latin",
        include_str!("data/TestNames_Latin.txt").replace(char::is_whitespace, ""),
    );
    let content_jp_h: (&str, String) = (
        "TestNames_Japanese_h",
        include_str!("data/TestNames_Japanese_h.txt").replace(char::is_whitespace, ""),
    );
    let content_jp_k: (&str, String) = (
        "TestNames_Japanese_k",
        include_str!("data/TestNames_Japanese_k.txt").replace(char::is_whitespace, ""),
    );
    let content_korean: (&str, String) = (
        "TestNames_Korean",
        include_str!("data/TestNames_Korean.txt").replace(char::is_whitespace, ""),
    );
    let content_viet: (&str, String) = (
        "udhr_vie",
        include_str!("data/udhr_vie.txt").replace(char::is_whitespace, ""),
    );

    let group_name = "canonical_decomposition";
    let decomposer = CanonicalDecomposition::try_new_unstable(&icu_testdata::unstable()).unwrap();

    let mut group = criterion.benchmark_group(format!("{}", group_name));
    for (file_name, content) in [content_latin, content_korean, content_jp_k, content_jp_h, content_viet] {
        for (chunk, &data) in chunkify(&content).iter().enumerate() {
            group.throughput(Throughput::Elements(data.chars().count() as u64));
            group.bench_with_input(
                BenchmarkId::from_parameter(format!("{}/chunk_{}", file_name, chunk)),
                data,
                |bencher, characters| bencher.iter(|| function_under_bench(&decomposer, characters)),
            );
        }
    }
    group.finish();
}
