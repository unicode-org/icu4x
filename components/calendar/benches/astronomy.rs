// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use icu_calendar::astronomy::Astronomical;
use icu_calendar::types::Moment;

fn solar_longitude_bench(c: &mut Criterion) {
    c.bench_function("astronomy/solar_longitude", |b| {
        b.iter(|| {
            for i in (-100000..=100000).step_by(573) {
                let moment = black_box(Moment::new(i as f64));
                black_box(Astronomical::solar_longitude(moment));
            }
        });
    });
}

criterion_group!(benches, solar_longitude_bench,);
criterion_main!(benches);
