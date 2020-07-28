mod fixtures;
mod helpers;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::convert::TryInto;

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

    c.bench_function("operands/create/float", |b| {
        b.iter(|| {
            for s in &data.float {
                let _: PluralOperands = black_box(*s)
                    .try_into()
                    .expect("Failed to parse a number into an operands.");
            }
        })
    });
}

criterion_group!(benches, operands,);
criterion_main!(benches);
