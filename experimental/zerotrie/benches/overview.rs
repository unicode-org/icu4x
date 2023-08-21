// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use litemap::LiteMap;
use std::collections::HashMap;
use zerotrie::ZeroTrieExtendedCapacity;
use zerotrie::ZeroTriePerfectHash;
use zerotrie::ZeroTrieSimpleAscii;
use zerovec::ZeroHashMap;
use zerovec::ZeroMap;

mod testdata {
    include!("../tests/data.rs");
}

fn get_basic_bench(c: &mut Criterion) {
    let mut g = c.benchmark_group("get/basic");

    // NOTE: All the trie data are the same for basic data
    let trie = testdata::basic::TRIE_ASCII;
    let data = testdata::basic::DATA_ASCII;

    g.bench_function("SimpleAscii", |b| {
        let trie = ZeroTrieSimpleAscii::from_bytes(trie);
        b.iter(|| {
            for (key, expected) in black_box(data) {
                let actual = black_box(&trie).get(key);
                assert_eq!(Some(*expected), actual);
            }
        });
    });

    g.bench_function("PerfectHash", |b| {
        let trie = ZeroTriePerfectHash::from_bytes(trie);
        b.iter(|| {
            for (key, expected) in black_box(data) {
                let actual = black_box(&trie).get(key);
                assert_eq!(Some(*expected), actual);
            }
        });
    });

    g.bench_function("ExtendedCapacity", |b| {
        let trie = ZeroTrieExtendedCapacity::from_bytes(trie);
        b.iter(|| {
            for (key, expected) in black_box(data) {
                let actual = black_box(&trie).get(key);
                assert_eq!(Some(*expected), actual);
            }
        });
    });

    #[cfg(feature = "bench")]
    g.bench_function("ZeroMap/usize", |b| {
        let zm: ZeroMap<[u8], usize> = data.iter().copied().collect();
        b.iter(|| {
            for (key, expected) in black_box(data) {
                let actual = black_box(&zm).get_copied(key);
                assert_eq!(Some(*expected), actual);
            }
        });
    });

    #[cfg(feature = "bench")]
    g.bench_function("ZeroMap/u8", |b| {
        let zm: ZeroMap<[u8], u8> = data.iter().map(|(k, v)| (*k, *v as u8)).collect();
        b.iter(|| {
            for (key, expected) in black_box(data) {
                let actual = black_box(&zm).get_copied(key);
                assert_eq!(Some(*expected as u8), actual);
            }
        });
    });

    #[cfg(feature = "bench")]
    g.bench_function("HashMap", |b| {
        let hm: HashMap<&[u8], usize> = data.iter().copied().map(|(a, b)| (a, b)).collect();
        b.iter(|| {
            for (key, expected) in black_box(data) {
                let actual = black_box(&hm).get(key);
                assert_eq!(Some(expected), actual);
            }
        });
    });

    #[cfg(feature = "bench")]
    g.bench_function("ZeroHashMap/usize", |b| {
        let zhm: ZeroHashMap<[u8], usize> = data
            .iter()
            .copied()
            .collect();
        b.iter(|| {
            for (key, expected) in black_box(data) {
                let actual = black_box(&zhm).get(key);
                // No get_copied on ZHM so we need to do it manually
                let actual = actual.map(|x| <zerovec::vecs::FlexZeroSlice as zerovec::maps::ZeroVecLike<usize>>::zvl_get_as_t(x, |y| *y));
                assert_eq!(Some(*expected), actual);
            }
        });
    });

    #[cfg(feature = "bench")]
    g.bench_function("ZeroHashMap/u8", |b| {
        let zhm: ZeroHashMap<[u8], u8> = data.iter().map(|(k, v)| (*k, *v as u8)).collect();
        b.iter(|| {
            for (key, expected) in black_box(data) {
                let actual = black_box(&zhm).get(key).copied();
                assert_eq!(Some(*expected as u8), actual);
            }
        });
    });
}

fn get_subtags_bench_medium(c: &mut Criterion) {
    let g = c.benchmark_group("get/subtags_10pct");

    let strings = testdata::short_subtags_10pct::STRINGS;
    let litemap = testdata::strings_to_litemap(strings);

    get_subtags_bench_helper(g, strings, litemap);
}

fn get_subtags_bench_large(c: &mut Criterion) {
    let g = c.benchmark_group("get/subtags_full");

    let strings = testdata::short_subtags::STRINGS;
    let litemap = testdata::strings_to_litemap(strings);

    get_subtags_bench_helper(g, strings, litemap);
}

fn get_subtags_bench_helper<M: criterion::measurement::Measurement>(
    mut g: criterion::BenchmarkGroup<M>,
    strings: &[&str],
    litemap: LiteMap<&[u8], usize>,
) {
    g.bench_function("SimpleAscii", |b| {
        let trie = ZeroTrieSimpleAscii::try_from(&litemap).unwrap();
        b.iter(|| {
            for (i, key) in black_box(strings).iter().enumerate() {
                let actual = black_box(&trie).get(key);
                assert_eq!(Some(i), actual);
            }
        });
    });

    g.bench_function("PerfectHash", |b| {
        let trie = ZeroTriePerfectHash::try_from(&litemap).unwrap();
        b.iter(|| {
            for (i, key) in black_box(strings).iter().enumerate() {
                let actual = black_box(&trie).get(key);
                assert_eq!(Some(i), actual);
            }
        });
    });

    g.bench_function("ExtendedCapacity", |b| {
        let trie = ZeroTrieExtendedCapacity::try_from(&litemap).unwrap();
        b.iter(|| {
            for (i, key) in black_box(strings).iter().enumerate() {
                let actual = black_box(&trie).get(key);
                assert_eq!(Some(i), actual);
            }
        });
    });

    #[cfg(feature = "bench")]
    g.bench_function("ZeroMap/usize", |b| {
        let zm: ZeroMap<[u8], usize> = litemap.iter().map(|(a, b)| (*a, b)).collect();
        b.iter(|| {
            for (i, key) in black_box(strings).iter().enumerate() {
                let actual = black_box(&zm).get_copied(key.as_bytes());
                assert_eq!(Some(i), actual);
            }
        });
    });

    #[cfg(feature = "bench")]
    g.bench_function("ZeroMap/u8", |b| {
        let zm: ZeroMap<[u8], u8> = litemap.iter().map(|(k, v)| (*k, *v as u8)).collect();
        b.iter(|| {
            for (i, key) in black_box(strings).iter().enumerate() {
                let actual = black_box(&zm).get_copied(key.as_bytes());
                assert_eq!(Some(i as u8), actual);
            }
        });
    });

    #[cfg(feature = "bench")]
    g.bench_function("HashMap", |b| {
        let hm: HashMap<&[u8], usize> = litemap.iter().map(|(a, b)| (*a, *b)).collect();
        b.iter(|| {
            for (i, key) in black_box(strings).iter().enumerate() {
                let actual = black_box(&hm).get(key.as_bytes());
                assert_eq!(Some(&i), actual);
            }
        });
    });

    #[cfg(feature = "bench")]
    g.bench_function("ZeroHashMap/usize", |b| {
        let zhm: ZeroHashMap<[u8], usize> = litemap
            .iter()
            .map(|(a, b)| (*a, b))
            .collect();
        b.iter(|| {
            for (i, key) in black_box(strings).iter().enumerate() {
                let actual = black_box(&zhm).get(key.as_bytes());
                // No get_copied on ZHM so we need to do it manually
                let actual = actual.map(|x| <zerovec::vecs::FlexZeroSlice as zerovec::maps::ZeroVecLike<usize>>::zvl_get_as_t(x, |y| *y));
                assert_eq!(Some(i), actual);
            }
        });
    });

    #[cfg(feature = "bench")]
    g.bench_function("ZeroHashMap/u8", |b| {
        let zhm: ZeroHashMap<[u8], u8> = litemap.iter().map(|(k, v)| (*k, *v as u8)).collect();
        b.iter(|| {
            for (i, key) in black_box(strings).iter().enumerate() {
                let actual = black_box(&zhm).get(key.as_bytes()).copied();
                assert_eq!(Some(i as u8), actual);
            }
        });
    });

    g.finish();
}

criterion_group!(
    benches,
    get_basic_bench,
    get_subtags_bench_medium,
    get_subtags_bench_large
);
criterion_main!(benches);
