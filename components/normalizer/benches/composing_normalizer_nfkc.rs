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

    // Load file content in reverse order vector.
    let content_latin: (&str, &str) = (
        "TestNames_Latin",
        include_str!("data/TestNames_Latin.txt"),
    );
    let content_jp_h: (&str, &str) = (
        "TestNames_Japanese_h",
        include_str!("data/TestNames_Japanese_h.txt"),
    );
    let content_jp_k: (&str, &str) = (
        "TestNames_Japanese_k",
        include_str!("data/TestNames_Japanese_k.txt"),
    );
    let content_korean: (&str, &str) = (
        "TestNames_Korean",
        include_str!("data/TestNames_Korean.txt"),
    );
    let content_viet: (&str, &str) = (
        "udhr_vie",
        include_str!("data/udhr_vie.txt"),
    );

    let mut group = criterion.benchmark_group(group_name);
    for (file_name, content) in [content_latin, content_viet, content_jp_k, content_jp_h, content_korean] {
        group.throughput(Throughput::Bytes(content.len() as u64));
        group.bench_with_input(BenchmarkId::from_parameter(file_name), content, |bencher, content| {
            bencher.iter(|| function_under_bench(&normalizer, content))
        });
    }
    group.finish();
}
