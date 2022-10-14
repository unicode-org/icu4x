use criterion::{black_box, BenchmarkId, Criterion, Throughput};

use icu_normalizer::DecomposingNormalizer;

fn function_under_bench(normalizer: &DecomposingNormalizer, text: &str) {
    normalizer.normalize(text);
}

pub fn criterion_benchmark(criterion: &mut Criterion) {
    let group_name = "decomposing_normalizer_nfkd";
    let normalizer: DecomposingNormalizer =
        DecomposingNormalizer::try_new_nfkd_unstable(&icu_testdata::unstable()).unwrap();
    let params = black_box([
        "ä", "Ä", "ệ", "Ệ", "𝅗𝅥", "\u{2126}", "ﾍﾞ", "ﾍﾟ", "ﬁ", "\u{FDFA}", "㈎", "\u{0345}",
    ]);

    let mut group = criterion.benchmark_group(group_name);
    for text in params {
        group.throughput(Throughput::Elements(1));
        group.bench_with_input(BenchmarkId::new(group_name, text), text, |bencher, text| {
            bencher.iter(|| function_under_bench(&normalizer, text))
        });
    }
    group.finish();
}
