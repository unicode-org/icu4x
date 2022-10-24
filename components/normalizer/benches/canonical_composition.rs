// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use criterion::{black_box, BenchmarkId, Criterion, Throughput};

use icu_normalizer::properties::CanonicalComposition;

fn function_under_bench(
    canonical_composer: &CanonicalComposition,
    starter_character: char,
    second_character: char,
) {
    canonical_composer.compose(starter_character, second_character);
}

pub fn criterion_benchmark(criterion: &mut Criterion) {
    let group_name = "canonical_composition";
    let composer = CanonicalComposition::try_new_unstable(&icu_testdata::unstable()).unwrap();
    let params = black_box([
        ('a', 'b'),
        ('a', '\u{0308}'),
        ('A', '\u{0308}'),
        ('ẹ', '\u{0302}'),
        ('Ẹ', '\u{0302}'),
        ('𑄱', '𑄧'),
        ('𝅗', '𝅥'),
        ('ে', 'া'),
        ('ᄀ', 'ᅡ'),
        ('가', 'ᆨ'),
    ]);

    let mut group = criterion.benchmark_group(group_name);
    for tuple in params {
        group.throughput(Throughput::Elements(1));
        group.bench_with_input(
            BenchmarkId::new(group_name, format!("{:?}", tuple)),
            &tuple,
            |bencher, tuple| bencher.iter(|| function_under_bench(&composer, tuple.0, tuple.1)),
        );
    }
    group.finish();
}
