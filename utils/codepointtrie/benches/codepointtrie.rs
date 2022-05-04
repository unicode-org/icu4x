// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use icu_codepointtrie::toml::CodePointTrieToml;
use icu_codepointtrie::CodePointTrie;
use icu_locid::{langid, LanguageIdentifier};
use icu_provider::hello_world::{HelloWorldProvider, HelloWorldV1Marker};
use icu_provider::prelude::*;
use std::convert::TryInto;
use std::fs;

/// A function that returns 50 code points in the desired language
fn fifty_code_points(langid: LanguageIdentifier) -> String {
    let mut payload: Option<DataPayload<HelloWorldV1Marker>> = None;
    let sample_str = if langid == langid!("ccp") {
        // Special case for Chakma so that we can cover BMP code points
        "𑄟𑄚𑄬𑄭𑄃𑄇𑄴𑄇𑄥𑄧𑄁𑄢𑄴 𑄝𑄬𑄇𑄴𑄅𑄚𑄮𑄢𑄴 𑄟𑄧𑄚𑄳𑄢𑄧𑄧𑄇𑄉𑄮𑄌𑄴"
    } else {
        // For all other languages, get the Hello World
        let provider = HelloWorldProvider::new_with_placeholder_data();
        let payload_local = provider
            .load_resource(&DataRequest {
                options: langid.into(),
                metadata: Default::default(),
            })
            .expect("expected language to be present")
            .take_payload()
            .expect("expected payload to be present");
        payload.replace(payload_local);
        &payload.as_ref().unwrap().get().message
    };
    sample_str.chars().cycle().take(50).collect()
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
    let s = fifty_code_points(langid!("en"));
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
        lang_bench(c, langid!("en"));
        lang_bench(c, langid!("de"));
        lang_bench(c, langid!("el"));
        lang_bench(c, langid!("zh"));
        lang_bench(c, langid!("en"));
    }
}

#[cfg(feature = "bench")]
fn lang_bench(c: &mut Criterion, lid: LanguageIdentifier) {
    let bench_name = format!("cpt/get/{}", lid);
    let s = fifty_code_points(lid);
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
