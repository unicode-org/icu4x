// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use criterion::{BenchmarkId, Criterion, Throughput};

use icu_normalizer::ComposingNormalizer;

fn function_under_bench(normalizer: &ComposingNormalizer, text: &str) {
    normalizer.normalize(text);
}

pub fn criterion_benchmark(criterion: &mut Criterion) {
    let group_name = "composing_normalizer_nfc";
    let normalizer: ComposingNormalizer =
        ComposingNormalizer::try_new_nfc_unstable(&icu_testdata::unstable()).unwrap();

    let content_latin: (&str, &str) = (
        "TestNames_Latin",
        include_str!("./data/TestNames_Latin.txt"),
    );
    let content_jp_h: (&str, &str) = (
        "TestNames_Japanese_h",
        include_str!("./data/TestNames_Japanese_h.txt"),
    );
    let content_jp_k: (&str, &str) = (
        "TestNames_Japanese_k",
        include_str!("./data/TestNames_Japanese_k.txt"),
    );
    let content_korean: (&str, &str) = (
        "TestNames_Korean",
        include_str!("./data/TestNames_Korean.txt"),
    );
    let content_random_words_ar: (&str, &str) = (
        "TestRandomWordsUDHR_ar",
        include_str!("./data/TestRandomWordsUDHR_ar.txt"),
    );
    let content_random_words_de: (&str, &str) = (
        "TestRandomWordsUDHR_de",
        include_str!("./data/TestRandomWordsUDHR_de.txt"),
    );
    let content_random_words_el: (&str, &str) = (
        "TestRandomWordsUDHR_el",
        include_str!("./data/TestRandomWordsUDHR_el.txt"),
    );
    let content_random_words_es: (&str, &str) = (
        "TestRandomWordsUDHR_es",
        include_str!("./data/TestRandomWordsUDHR_es.txt"),
    );
    let content_random_words_fr: (&str, &str) = (
        "TestRandomWordsUDHR_fr",
        include_str!("./data/TestRandomWordsUDHR_fr.txt"),
    );
    let content_random_words_he: (&str, &str) = (
        "TestRandomWordsUDHR_he",
        include_str!("./data/TestRandomWordsUDHR_he.txt"),
    );
    let content_random_words_pl: (&str, &str) = (
        "TestRandomWordsUDHR_pl",
        include_str!("./data/TestRandomWordsUDHR_pl.txt"),
    );
    let content_random_words_ru: (&str, &str) = (
        "TestRandomWordsUDHR_ru",
        include_str!("./data/TestRandomWordsUDHR_ru.txt"),
    );
    let content_random_words_th: (&str, &str) = (
        "TestRandomWordsUDHR_th",
        include_str!("./data/TestRandomWordsUDHR_th.txt"),
    );
    let content_random_words_tr: (&str, &str) = (
        "TestRandomWordsUDHR_tr",
        include_str!("./data/TestRandomWordsUDHR_tr.txt"),
    );
    let content_viet: (&str, &str) = ("udhr_vie", include_str!("./data/udhr_vie.txt"));

    let mut group = criterion.benchmark_group(group_name);
    for (file_name, content) in [
        content_latin,
        content_viet,
        content_jp_k,
        content_jp_h,
        content_korean,
        content_random_words_ru,
        content_random_words_ar,
        content_random_words_el,
        content_random_words_es,
        content_random_words_fr,
        content_random_words_tr,
        content_random_words_th,
        content_random_words_pl,
        content_random_words_he,
        content_random_words_de,
    ] {
        group.throughput(Throughput::Bytes(content.len() as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(file_name),
            content,
            |bencher, content| bencher.iter(|| function_under_bench(&normalizer, content)),
        );
    }
    group.finish();
}
