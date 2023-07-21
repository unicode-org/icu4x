// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use icu_calendar::{Date, chinese::Chinese};

fn chinese_from_fixed_bench(c: &mut Criterion) {
    c.bench_function("chinese_based/chinese_from_fixed", |b| {
        b.iter(|| {
            for i in -1000..=1000 {
                let year = i;
                let month = i as u8 % 12 + 1;
                let day = i as u8 % 28 + 1;
                let iso = Date::try_new_iso_date(year, month, day).unwrap();
                black_box(iso.to_calendar(Chinese));
            }
        });
    });
}

criterion_group!(benches, chinese_from_fixed_bench,);
criterion_main!(benches);
