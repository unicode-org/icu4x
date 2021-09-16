// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn overview_bench(c: &mut Criterion) {
    c.bench_function("fixed_decimal_format/overview", |b| {
        #[allow(clippy::suspicious_map)]
        b.iter(|| {
            // Code to time
        });
    });
}

criterion_group!(benches, overview_bench);
criterion_main!(benches);
