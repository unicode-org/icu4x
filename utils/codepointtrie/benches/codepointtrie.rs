// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use icu_codepointtrie::toml::CodePointTrieToml;
use icu_codepointtrie::CodePointTrie;
use std::convert::TryInto;
use std::fs;

const SAMPLE_STRING_ENG: &str = "Universal Declaration of Human Rights";
const SAMPLE_STRING_PCD: &str = "Dèclaråcion dès dreûts d' l'ome po tos lès payîs dè monde";
const SAMPLE_STRING_UKR: &str = "ЗАГАЛЬНА ДЕКЛАРАЦІЯ ПРАВ ЛЮДИНІ";
const SAMPLE_STRING_YUE: &str = "世界人权宣言";
const SAMPLE_STRING_CCP: &str = "𑄟𑄚𑄬𑄭𑄃𑄇𑄴𑄇𑄥𑄧𑄁𑄢𑄴 𑄝𑄬𑄇𑄴𑄅𑄚𑄮𑄢𑄴 𑄟𑄧𑄚𑄳𑄢𑄧𑄧𑄇𑄉𑄮𑄌𑄴";

const SAMPLE_STRING_MIXED: &str = "Dèclaråcion ЗАГАЛЬНА 世界人权宣言 𑄟𑄚𑄬𑄭𑄃𑄇𑄴𑄇𑄥𑄧𑄁𑄢𑄴";

/// A function that returns 100 code points in the desired language
fn one_hundred_code_points(sample_str: &str) -> String {
    sample_str.chars().cycle().take(100).collect()
}

fn load_code_point_trie(buffer: &mut Vec<u8>) -> CodePointTrie<u8> {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/benches/testdata/gc_for_test.toml"
    );
    *buffer = fs::read(path).unwrap();
    let toml: CodePointTrieToml = toml::from_slice(buffer.as_slice()).unwrap();
    (&toml).try_into().unwrap()
}

fn overview_bench(c: &mut Criterion) {
    let s = one_hundred_code_points(SAMPLE_STRING_MIXED);
    let mut buffer = Vec::<u8>::new();
    let cpt = load_code_point_trie(&mut buffer);

    c.bench_function("cpt/overview", |b| {
        b.iter(|| {
            black_box(&s)
                .chars()
                .map(|c| black_box(&cpt).get(c as u32))
                .reduce(|s, v| s.wrapping_add(v))
        });
    });

    #[cfg(feature = "bench")]
    {
        lang_bench(c, "eng", SAMPLE_STRING_ENG);
        lang_bench(c, "pcd", SAMPLE_STRING_PCD);
        lang_bench(c, "ukr", SAMPLE_STRING_UKR);
        lang_bench(c, "yue", SAMPLE_STRING_YUE);
        lang_bench(c, "ccp", SAMPLE_STRING_CCP);
    }
}

#[cfg(feature = "bench")]
fn lang_bench(c: &mut Criterion, lid: &str, sample_str: &str) {
    let bench_name = format!("cpt/get/{}", lid);
    let s = one_hundred_code_points(sample_str);
    let mut buffer = Vec::<u8>::new();
    let cpt = load_code_point_trie(&mut buffer);

    c.bench_function(&bench_name, |b| {
        b.iter(|| {
            black_box(&s)
                .chars()
                .map(|c| black_box(&cpt).get(c as u32))
                .reduce(|s, v| s.wrapping_add(v))
        });
    });
}

criterion_group!(benches, overview_bench,);
criterion_main!(benches);
