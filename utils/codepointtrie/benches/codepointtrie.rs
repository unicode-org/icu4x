// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use icu_codepointtrie::CodePointTrie;

#[path = "tries/mod.rs"]
mod tries;

#[cfg(feature = "bench")]
mod sample_str_lng {
    pub const ENG: &str = "Universal Declaration of Human Rights";
    pub const PCD: &str = "Dèclaråcion dès dreûts d' l'ome po tos lès payîs dè monde";
    pub const UKR: &str = "ЗАГАЛЬНА ДЕКЛАРАЦІЯ ПРАВ ЛЮДИНІ";
    pub const YUE: &str = "世界人权宣言";
    pub const CCP: &str = "𑄟𑄚𑄬𑄭𑄃𑄇𑄴𑄇𑄥𑄧𑄁𑄢𑄴 𑄝𑄬𑄇𑄴𑄅𑄚𑄮𑄢𑄴 𑄟𑄧𑄚𑄳𑄢𑄧𑄧𑄇𑄉𑄮𑄌𑄴";
}

const SAMPLE_STRING_MIXED: &str = "Dèclaråcion ЗАГАЛЬНА 世界人权宣言 𑄟𑄚𑄬𑄭𑄃𑄇𑄴𑄇𑄥𑄧𑄁𑄢𑄴";

/// A function that returns 100 code points in the desired language
fn one_hundred_code_points(sample_str: &str) -> String {
    sample_str.chars().cycle().take(100).collect()
}

fn get_trie_small() -> CodePointTrie<'static, u8> {
    CodePointTrie::try_new(
        tries::gc_small::HEADER,
        tries::gc_small::INDEX.as_zerovec(),
        tries::gc_small::DATA.as_zerovec(),
    )
    .unwrap()
}

fn get_trie_fast() -> CodePointTrie<'static, u8> {
    CodePointTrie::try_new(
        tries::gc_fast::HEADER,
        tries::gc_fast::INDEX.as_zerovec(),
        tries::gc_fast::DATA.as_zerovec(),
    )
    .unwrap()
}

fn overview_bench(c: &mut Criterion) {
    let s = one_hundred_code_points(SAMPLE_STRING_MIXED);
    let cpt_small = get_trie_small();
    let cpt_fast = get_trie_fast();

    c.bench_function("cpt/overview", |b| {
        b.iter(|| {
            black_box(&s)
                .chars()
                .map(|c| black_box(&cpt_small).get(c as u32))
                .reduce(|s, v| s.wrapping_add(v))
        });
    });

    #[cfg(feature = "bench")]
    {
        lang_bench(c, &cpt_small, "small/eng", sample_str_lng::ENG);
        lang_bench(c, &cpt_small, "small/pcd", sample_str_lng::PCD);
        lang_bench(c, &cpt_small, "small/ukr", sample_str_lng::UKR);
        lang_bench(c, &cpt_small, "small/yue", sample_str_lng::YUE);
        lang_bench(c, &cpt_small, "small/ccp", sample_str_lng::CCP);
        lang_bench(c, &cpt_fast, "fast/eng", sample_str_lng::ENG);
        lang_bench(c, &cpt_fast, "fast/pcd", sample_str_lng::PCD);
        lang_bench(c, &cpt_fast, "fast/ukr", sample_str_lng::UKR);
        lang_bench(c, &cpt_fast, "fast/yue", sample_str_lng::YUE);
        lang_bench(c, &cpt_fast, "fast/ccp", sample_str_lng::CCP);
    }
}

#[cfg(feature = "bench")]
fn lang_bench(c: &mut Criterion, cpt: &CodePointTrie<u8>, lid: &str, sample_str: &str) {
    let bench_name = format!("cpt/get/{}", lid);
    let s = one_hundred_code_points(sample_str);

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
