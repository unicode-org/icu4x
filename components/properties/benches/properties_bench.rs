// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use icu_properties::props::{Alphabetic, GeneralCategory, Script};
use icu_properties::{CodePointMapData, CodePointSetData};

const SAMPLE_STRING_MIXED: &str = "Hello, 世界! 🎃 ЗАГАЛЬНА";

fn one_hundred_code_points(sample_str: &str) -> String {
    sample_str.chars().cycle().take(100).collect()
}

fn set_benchmarks(c: &mut Criterion) {
    let s = one_hundred_code_points(SAMPLE_STRING_MIXED);
    let alpha = CodePointSetData::new::<Alphabetic>();

    c.bench_function("icu_properties/set/contains", |b| {
        b.iter(|| {
            black_box(&s)
                .chars()
                .filter(|ch| black_box(&alpha).contains(*ch))
                .count()
        })
    });

    c.bench_function("icu_properties/set/iter_ranges", |b| {
        b.iter(|| black_box(&alpha).iter_ranges().count())
    });
}

fn map_benchmarks(c: &mut Criterion) {
    let s = one_hundred_code_points(SAMPLE_STRING_MIXED);

    let gc = CodePointMapData::<GeneralCategory>::new();
    c.bench_function("icu_properties/map/get/general_category", |b| {
        b.iter(|| {
            black_box(&s).chars().for_each(|ch| {
                black_box(black_box(&gc).get(ch));
            })
        })
    });

    let script = CodePointMapData::<Script>::new();
    c.bench_function("icu_properties/map/get/script", |b| {
        b.iter(|| {
            black_box(&s).chars().for_each(|ch| {
                black_box(black_box(&script).get(ch));
            })
        })
    });

    c.bench_function("icu_properties/map/get_set_for_value", |b| {
        b.iter(|| {
            let set = black_box(&gc).get_set_for_value(GeneralCategory::UppercaseLetter);
            set.as_borrowed().contains('A')
        })
    });
}

criterion_group!(benches, set_benchmarks, map_benchmarks);
criterion_main!(benches);
