// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

use icu::collator::*;
use icu::locid::{locale, Locale};

pub fn collator_simple(criterion: &mut Criterion) {
    let mut options = CollatorOptions::new();
    options.strength = Some(Strength::Primary);
    let collator: Collator =
        Collator::try_new_unstable(&icu_testdata::unstable(), &Default::default(), options)
            .unwrap();

    let params = black_box([
        (
            "Pneumonoultramicroscopicsilicovolcanoconiosis",
            "Hippopotomonstrosesquippedaliophobia",
        ),
        (
            "Pneumonoultramicroscopicsilicovolcanoconiosis",
            "Pneumonoultramicroscopicsilicovolcanoconiosis--",
        ),
    ]);

    let mut group = criterion.benchmark_group("collator_simple");
    for tuple in params {
        group.throughput(Throughput::Elements(1));
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{:?}", tuple)),
            &tuple,
            |bencher, tuple| bencher.iter(|| collator.compare(tuple.0, tuple.1)),
        );
    }
    group.finish();
}

pub fn collator_with_handling(criterion: &mut Criterion) {
    let mut options = CollatorOptions::new();

    options.strength = Some(Strength::Tertiary);
    options.alternate_handling = Some(AlternateHandling::NonIgnorable);

    let collator: Collator =
        Collator::try_new_unstable(&icu_testdata::unstable(), &Default::default(), options)
            .unwrap();

    let params = black_box([
        ("di Silva", "Di Silva"),
        ("Di Silva", "diSilva"),
        ("diSilva", "U.S.A."),
        ("U.S.A.", "USA"),
    ]);

    let mut group = criterion.benchmark_group("default_collator_with_handling");
    for tuple in params {
        group.throughput(Throughput::Elements(1));
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{:?}", tuple)),
            &tuple,
            |bencher, tuple| bencher.iter(|| collator.compare(tuple.0, tuple.1)),
        );
    }
    group.finish();
}

pub fn collator_with_locale_old_spanish(criterion: &mut Criterion) {
    let locale_es: Locale = locale!("es-u-co-trad");
    let mut options = CollatorOptions::new();
    options.strength = Some(Strength::Primary);
    let collator: Collator =
        Collator::try_new_unstable(&icu_testdata::unstable(), &locale_es.into(), options).unwrap();

    let params = black_box([("pollo", "polvo")]);

    let mut group = criterion.benchmark_group("collator_with_locale_old_spanish");
    for tuple in params {
        group.throughput(Throughput::Elements(1));
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{:?}", tuple)),
            &tuple,
            |bencher, tuple| bencher.iter(|| collator.compare(tuple.0, tuple.1)),
        );
    }
    group.finish();
}

criterion_group!(benches, collator_simple, collator_with_handling, collator_with_locale_old_spanish);

criterion_main!(benches);
