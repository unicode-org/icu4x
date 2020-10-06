use rand::SeedableRng;
use rand_distr::{Distribution, Triangular};
use rand_pcg::Lcg64Xsh32;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use icu_utils_num::FixedDecimal;
use std::str::FromStr;

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
        b.iter(|| {
            #[allow(clippy::suspicious_map)]
            nums.iter()
                .map(|v| black_box(*v))
                .map(FixedDecimal::from)
                .count()
        });
    });
}

fn larger_isize_benches(c: &mut Criterion) {
    // Larger nums: -1e16 to 1e16
    let nums = triangular_nums(1e16);

    // Note: this could be bench_function_with_inputs, but there are 1000 random inputs.
    // Instead, consider all inputs together in the same benchmark.
    c.bench_function("isize/larger", |b| {
        b.iter(|| {
            #[allow(clippy::suspicious_map)]
            nums.iter()
                .map(|v| black_box(*v))
                .map(FixedDecimal::from)
                .count()
        });
    });
}

fn to_string_benches(c: &mut Criterion) {
    let objects = [
        FixedDecimal::from(2250).multiplied_pow10(-2).unwrap(),
        FixedDecimal::from(908070605040302010u128),
    ];

    {
        let mut group = c.benchmark_group("to_string");
        for object in objects.iter() {
            group.bench_with_input(
                BenchmarkId::from_parameter(object.to_string()),
                object,
                |b, object| b.iter(|| object.to_string()),
            );
        }
        group.finish();
    }

    {
        let mut group = c.benchmark_group("write_to");
        for object in objects.iter() {
            group.bench_with_input(
                BenchmarkId::from_parameter(object.to_string()),
                object,
                |b, object| {
                    b.iter(|| {
                        let mut result = String::with_capacity(object.write_len());
                        object.write_to(&mut result)
                    })
                },
            );
        }
        group.finish();
    }
}

fn from_string_benches(c: &mut Criterion) {
    let objects = [
        "0012.3400",
        "00.0012216734340",
        "00002342561123400.0",
        "-00123400",
        "922337203685477580898230948203840239384.9823094820384023938423424",
        "0.000000001",
        "1000000001",
        &{
            let mut x = format!("{:0fill$}", 0, fill = 32768);
            x.push_str(".");
            x.push_str(&format!("{:0fill$}", 0, fill = 32768));
            x
        },
    ];

    {
        let mut group = c.benchmark_group("from_string");
        for object in objects.iter() {
            group.bench_with_input(
                BenchmarkId::from_parameter(object.to_string()),
                object,
                |b, object| b.iter(|| FixedDecimal::from_str(object).unwrap()),
            );
        }
        group.finish();
    }
}

criterion_group!(
    benches,
    smaller_isize_benches,
    larger_isize_benches,
    to_string_benches,
    from_string_benches
);
criterion_main!(benches);
