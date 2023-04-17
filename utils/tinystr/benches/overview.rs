// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod common;
use common::*;

use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

use tinystr::TinyAsciiStr;

fn overview(c: &mut Criterion) {
    let mut g = c.benchmark_group("overview");

    g.bench_function("construct/TinyAsciiStr", |b| {
        b.iter(|| {
            for s in STRINGS_4 {
                let _: TinyAsciiStr<4> = black_box(s).parse().unwrap();
                let _: TinyAsciiStr<8> = black_box(s).parse().unwrap();
                let _: TinyAsciiStr<16> = black_box(s).parse().unwrap();
            }
            for s in STRINGS_8 {
                let _: TinyAsciiStr<8> = black_box(s).parse().unwrap();
                let _: TinyAsciiStr<16> = black_box(s).parse().unwrap();
            }
            for s in STRINGS_16 {
                let _: TinyAsciiStr<16> = black_box(s).parse().unwrap();
            }
        });
    });

    let parsed_ascii_4: Vec<TinyAsciiStr<4>> = STRINGS_4
        .iter()
        .map(|s| s.parse::<TinyAsciiStr<4>>().unwrap())
        .collect();
    let parsed_ascii_8: Vec<TinyAsciiStr<8>> = STRINGS_4
        .iter()
        .chain(STRINGS_8)
        .map(|s| s.parse::<TinyAsciiStr<8>>().unwrap())
        .collect();
    let parsed_ascii_16: Vec<TinyAsciiStr<16>> = STRINGS_4
        .iter()
        .chain(STRINGS_8)
        .chain(STRINGS_16)
        .map(|s| s.parse::<TinyAsciiStr<16>>().unwrap())
        .collect();

    g.bench_function("read/TinyAsciiStr", |b| {
        b.iter(|| {
            let mut collector: usize = 0;
            for t in black_box(&parsed_ascii_4) {
                let s: &str = t;
                collector += s.bytes().map(usize::from).sum::<usize>();
            }
            for t in black_box(&parsed_ascii_8) {
                let s: &str = t;
                collector += s.bytes().map(usize::from).sum::<usize>();
            }
            for t in black_box(&parsed_ascii_16) {
                let s: &str = t;
                collector += s.bytes().map(usize::from).sum::<usize>();
            }
            collector
        });
    });

    g.bench_function("compare/TinyAsciiStr", |b| {
        b.iter(|| {
            let mut collector: usize = 0;
            for ts in black_box(&parsed_ascii_4).windows(2) {
                let o = ts[0].cmp(&ts[1]);
                collector ^= o as usize;
            }
            for ts in black_box(&parsed_ascii_8).windows(2) {
                let o = ts[0].cmp(&ts[1]);
                collector ^= o as usize;
            }
            for ts in black_box(&parsed_ascii_16).windows(2) {
                let o = ts[0].cmp(&ts[1]);
                collector ^= o as usize;
            }
            collector
        });
    });
}

criterion_group!(benches, overview,);
criterion_main!(benches);
