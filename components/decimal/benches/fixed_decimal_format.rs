// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use rand::SeedableRng;
use rand_distr::{Distribution, Triangular};
use rand_pcg::Lcg64Xsh32;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use fixed_decimal::FixedDecimal;
use icu_decimal::FixedDecimalFormat;
use icu_locid::Locale;
use writeable::Writeable;

fn triangular_nums(range: f64) -> Vec<isize> {
    // Use Lcg64Xsh32, a small, fast PRNG.
    // Generate 1000 numbers between -range and +range, weighted around 0.
    let rng = Lcg64Xsh32::seed_from_u64(2020);
    let dist = Triangular::new(-range, range, 0.0).unwrap();
    dist.sample_iter(rng)
        .take(1000)
        .map(|v| v as isize)
        .collect()
}

fn overview_bench(c: &mut Criterion) {
    let nums = triangular_nums(1e9);
    c.bench_function("icu_decimal/overview", |b| {
        #[allow(clippy::suspicious_map)]
        b.iter(|| {
            // This benchmark demonstrates the performance of the format function on 1000 numbers
            // ranging from -1e9 to 1e9.
            let provider = icu_provider::inv::InvariantDataProvider;
            let fdf =
                FixedDecimalFormat::try_new(Locale::und(), &provider, Default::default()).unwrap();
            nums.iter()
                .map(|v| black_box(*v))
                .map(FixedDecimal::from)
                .map(|n| fdf.format(&n).writeable_to_string())
                .count();
        });
    });
}

criterion_group!(benches, overview_bench);
criterion_main!(benches);
