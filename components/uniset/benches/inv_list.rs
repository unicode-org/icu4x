use criterion::{criterion_group, criterion_main, Criterion};
use icu_unicodeset::UnicodeSet;
use std::convert::TryFrom;

/// Best Case Contains
///
/// Create a single small range and check contains on every value in range
fn best_case_contains(c: &mut Criterion) {
    let check = vec![65, 70];
    let uset = UnicodeSet::try_from(check).unwrap();
    c.bench_function("inv_list/contains_best", |b| {
        b.iter(|| uset.iter().map(|c| uset.contains(c)))
    });
}

/// Worst Case Contains
///
/// Create the maximum number of ranges ([0, 1, 2, 3], etc.) and check contains on 100 first values
fn worst_case_contains(c: &mut Criterion) {
    let check: Vec<u32> = (0..((std::char::MAX as u32) + 1)).collect();
    let uset = UnicodeSet::try_from(check).unwrap();
    c.bench_function("inv_list/contains_worst", |b| {
        b.iter(|| uset.iter().take(100).map(|c| uset.contains(c)))
    });
}
/// Best Case Contains Range
///
/// Create a single small range and check contains on every value in range
fn best_case_contains_range(c: &mut Criterion) {
    let check = vec![65, 70];
    let uset = UnicodeSet::try_from(check).unwrap();
    c.bench_function("inv_list/contains_range_best", |b| {
        b.iter(|| uset.iter().map(|c| uset.contains_range(&('A'..c))))
    });
}

/// Worst Case Contains Range
///
/// Create the maximum number of ranges ([0, 1, 2, 3], etc.) and check contains on 100 first values
fn worst_case_contains_range(c: &mut Criterion) {
    let check: Vec<u32> = (0..((std::char::MAX as u32) + 1)).collect();
    let start = std::char::from_u32(0).unwrap();
    let uset = UnicodeSet::try_from(check).unwrap();
    c.bench_function("inv_list/contains_range_worst", |b| {
        b.iter(|| {
            uset.iter()
                .take(100)
                .map(|c| uset.contains_range(&(start..c)))
        })
    });
}
criterion_group!(
    benches,
    best_case_contains,
    worst_case_contains,
    best_case_contains_range,
    worst_case_contains_range
);
criterion_main!(benches);
