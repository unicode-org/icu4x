use rand::SeedableRng;
use rand_distr::{Distribution, Triangular};
use rand_pcg::Lcg64Xsh32;

use criterion::{criterion_group, criterion_main, Criterion};

use icu_num_util::FixedDecimal;

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

fn smaller_isize_benches(c: &mut Criterion) {
    // Smaller nums: -1e4 to 1e4
    let nums = triangular_nums(1e4);

    // Note: this could be bench_function_with_inputs, but there are 1000 random inputs.
    // Instead, consider all inputs together in the same benchmark.
    c.bench_function("isize/smaller", |b| {
        b.iter(|| nums.iter().map(|v| *v).map(FixedDecimal::from).count());
    });
}

fn larger_isize_benches(c: &mut Criterion) {
    // Larger nums: -1e16 to 1e16
    let nums = triangular_nums(1e16);

    // Note: this could be bench_function_with_inputs, but there are 1000 random inputs.
    // Instead, consider all inputs together in the same benchmark.
    c.bench_function("isize/larger", |b| {
        b.iter(|| nums.iter().map(|v| *v).map(FixedDecimal::from).count());
    });
}

criterion_group!(benches, smaller_isize_benches, larger_isize_benches);
criterion_main!(benches);
