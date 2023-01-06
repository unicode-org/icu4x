// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use criterion::{BenchmarkId, Criterion};
use detone::IterDecomposeVietnamese;

use icu_normalizer::properties::{CanonicalComposition, CanonicalDecomposition, Decomposed};
use icu_normalizer::ComposingNormalizer;

struct BenchDataContent {
    pub file_name: String,
    pub pairs: Vec<(char, char)>,
}

fn normalizer_bench_data() -> [BenchDataContent; 16] {
    let nfc_normalizer: ComposingNormalizer =
        ComposingNormalizer::try_new_nfc_unstable(&icu_testdata::unstable()).unwrap();

    return [
        BenchDataContent {
            file_name: "TestNames_Latin".to_owned(),
            pairs: decompose_data(
                &nfc_normalizer.normalize(include_str!("./data/TestNames_Latin.txt")),
            ),
        },
        BenchDataContent {
            file_name: "TestNames_Japanese_h".to_owned(),
            pairs: decompose_data(
                &nfc_normalizer.normalize(include_str!("./data/TestNames_Japanese_h.txt")),
            ),
        },
        BenchDataContent {
            file_name: "TestNames_Japanese_k".to_owned(),
            pairs: decompose_data(
                &nfc_normalizer.normalize(include_str!("./data/TestNames_Japanese_k.txt")),
            ),
        },
        BenchDataContent {
            file_name: "TestNames_Korean".to_owned(),
            pairs: decompose_data(
                &nfc_normalizer.normalize(include_str!("./data/TestNames_Korean.txt")),
            ),
        },
        BenchDataContent {
            file_name: "TestRandomWordsUDHR_ar".to_owned(),
            #[cfg(debug_assertions)]
            pairs: Vec::new(),
            #[cfg(not(debug_assertions))]
            pairs: decompose_data(
                &nfc_normalizer.normalize(include_str!("./data/TestRandomWordsUDHR_ar.txt")),
            ),
        },
        BenchDataContent {
            file_name: "TestRandomWordsUDHR_de".to_owned(),
            pairs: decompose_data(
                &nfc_normalizer.normalize(include_str!("./data/TestRandomWordsUDHR_de.txt")),
            ),
        },
        BenchDataContent {
            file_name: "TestRandomWordsUDHR_el".to_owned(),
            pairs: decompose_data(
                &nfc_normalizer.normalize(include_str!("./data/TestRandomWordsUDHR_el.txt")),
            ),
        },
        BenchDataContent {
            file_name: "TestRandomWordsUDHR_es".to_owned(),
            pairs: decompose_data(
                &nfc_normalizer.normalize(include_str!("./data/TestRandomWordsUDHR_es.txt")),
            ),
        },
        BenchDataContent {
            file_name: "TestRandomWordsUDHR_fr".to_owned(),
            pairs: decompose_data(
                &nfc_normalizer.normalize(include_str!("./data/TestRandomWordsUDHR_fr.txt")),
            ),
        },
        BenchDataContent {
            file_name: "TestRandomWordsUDHR_he".to_owned(),
            pairs: decompose_data(
                &nfc_normalizer.normalize(include_str!("./data/TestRandomWordsUDHR_he.txt")),
            ),
        },
        BenchDataContent {
            file_name: "TestRandomWordsUDHR_pl".to_owned(),
            pairs: decompose_data(
                &nfc_normalizer.normalize(include_str!("./data/TestRandomWordsUDHR_pl.txt")),
            ),
        },
        BenchDataContent {
            file_name: "TestRandomWordsUDHR_ru".to_owned(),
            pairs: decompose_data(
                &nfc_normalizer.normalize(include_str!("./data/TestRandomWordsUDHR_ru.txt")),
            ),
        },
        BenchDataContent {
            file_name: "TestRandomWordsUDHR_th".to_owned(),
            #[cfg(debug_assertions)]
            pairs: Vec::new(),
            #[cfg(not(debug_assertions))]
            pairs: decompose_data(
                &nfc_normalizer.normalize(include_str!("./data/TestRandomWordsUDHR_th.txt")),
            ),
        },
        BenchDataContent {
            file_name: "TestRandomWordsUDHR_tr".to_owned(),
            pairs: decompose_data(
                &nfc_normalizer.normalize(include_str!("./data/TestRandomWordsUDHR_tr.txt")),
            ),
        },
        BenchDataContent {
            file_name: "udhr_vie".to_owned(),
            pairs: decompose_data(&nfc_normalizer.normalize(include_str!("./data/udhr_vie.txt"))),
        },
        BenchDataContent {
            file_name: "udhr_vie_detone".to_owned(),
            pairs: {
                let result: Vec<(char, char)> = nfc_normalizer
                    .normalize(include_str!("./data/udhr_vie.txt"))
                    .chars()
                    .map(|c| {
                        let mut iter = std::iter::once(c).decompose_vietnamese_tones(true);
                        if let Some(base) = iter.next() {
                            if let Some(tone) = iter.next() {
                                Some((base, tone))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .filter(Option::is_some)
                    .map(Option::unwrap)
                    .collect();
                assert!(result.len() > 0);
                result
            },
        },
    ];
}

fn function_under_bench(
    canonical_composer: &CanonicalComposition,
    composable_points: &[(char, char)],
) {
    for pair in composable_points.iter() {
        canonical_composer.compose(pair.0, pair.1);
    }
}

pub fn criterion_benchmark(criterion: &mut Criterion) {
    let group_name = "canonical_composition";
    let mut group = criterion.benchmark_group(group_name);

    let composer = CanonicalComposition::try_new_unstable(&icu_testdata::unstable()).unwrap();

    for bench_data_content in normalizer_bench_data() {
        group.bench_function(
            BenchmarkId::from_parameter(format!("from_nfc_{}", bench_data_content.file_name)),
            |bencher| bencher.iter(|| function_under_bench(&composer, &bench_data_content.pairs)),
        );
    }

    group.finish();
}

fn decompose_data(nfc: &str) -> Vec<(char, char)> {
    let decomposer = CanonicalDecomposition::try_new_unstable(&icu_testdata::unstable()).unwrap();
    nfc.chars()
        .map(|c| decomposer.decompose(c))
        .map(|decomposed| {
            if let Decomposed::Expansion(a, b) = decomposed {
                Some((a, b))
            } else {
                None
            }
        })
        .filter(Option::is_some)
        .map(Option::unwrap)
        .collect()
}
