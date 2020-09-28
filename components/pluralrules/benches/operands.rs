mod fixtures;
mod helpers;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::convert::TryInto;

use icu_num_util::FixedDecimal;
use icu_pluralrules::PluralOperands;

const DATA_PATH: &str = "./benches/fixtures/numbers.json";

fn operands(c: &mut Criterion) {
    let data: fixtures::NumbersFixture =
        helpers::read_fixture(DATA_PATH).expect("Failed to read a fixture");

    c.bench_function("operands/create/usize", |b| {
        b.iter(|| {
            for s in &data.usize {
                let _: PluralOperands = black_box(*s).into();
            }
        })
    });

    c.bench_function("operands/create/isize", |b| {
        b.iter(|| {
            for s in &data.isize {
                let _: PluralOperands = black_box(*s)
                    .try_into()
                    .expect("Failed to parse a number into an operands.");
            }
        })
    });

    c.bench_function("operands/create/string", |b| {
        b.iter(|| {
            for s in &data.string {
                let _: PluralOperands = black_box(s)
                    .parse()
                    .expect("Failed to parse a number into an operands.");
            }
        })
    });

    c.bench_function("operands/eq/mostly_unequal", |b| {
        let p: PluralOperands = "1".parse().expect("Parse successful");
        for s in &data.isize {
            let q: PluralOperands = black_box(*s)
                .try_into()
                .expect("Failed to parse a number into an operands.");
            b.iter(|| {
                let _ = black_box(black_box(p) == black_box(q));
            })
        }
    });

    c.bench_function("operands/eq/mostly_equal", |b| {
        for s in &data.isize {
            let p: PluralOperands = black_box(*s)
                .try_into()
                .expect("Failed to parse a number into an operands.");
            let q: PluralOperands = black_box(*s)
                .try_into()
                .expect("Failed to parse a number into an operands.");
            b.iter(|| {
                let _ = black_box(black_box(p) == black_box(q));
            })
        }
    });

    c.bench_function("operands/create/from_fixed_decimal", |b| {
        for s in &data.fixed_decimals {
            let f: FixedDecimal = FixedDecimal::from(s.value)
                .multiplied_pow10(s.exponent)
                .unwrap();
            b.iter(|| {
                let _: PluralOperands = PluralOperands::from(black_box(&f));
            });
        }
    });
}

criterion_group!(benches, operands,);
criterion_main!(benches);
