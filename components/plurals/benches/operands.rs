// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod fixtures;
mod helpers;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fixed_decimal::FixedDecimal;
use icu_plurals::PluralOperands;
use std::convert::TryInto;

fn operands(c: &mut Criterion) {
    let data = helpers::get_numbers_data();

    c.bench_function("plurals/operands/overview", |b| {
        b.iter(|| {
            for s in &data.usize {
                let _: PluralOperands = black_box(*s).into();
            }
            for s in &data.isize {
                let _: PluralOperands = black_box(*s)
                    .try_into()
                    .expect("Failed to parse a number into an operands.");
            }
            for s in &data.string {
                let _: PluralOperands = black_box(s)
                    .parse()
                    .expect("Failed to parse a number into an operands.");
            }
            for s in &data.fixed_decimals {
                let f: FixedDecimal = FixedDecimal::from(s.value).multiplied_pow10(s.exponent);
                let _: PluralOperands = PluralOperands::from(black_box(&f));
            }
        })
    });

    #[cfg(feature = "bench")]
    {
        use criterion::BenchmarkId;

        c.bench_function("plurals/operands/create/usize", |b| {
            b.iter(|| {
                for s in &data.usize {
                    let _: PluralOperands = black_box(*s).into();
                }
            })
        });

        c.bench_function("plurals/operands/create/isize", |b| {
            b.iter(|| {
                for s in &data.isize {
                    let _: PluralOperands = black_box(*s)
                        .try_into()
                        .expect("Failed to parse a number into an operands.");
                }
            })
        });

        c.bench_function("plurals/operands/create/string", |b| {
            b.iter(|| {
                for s in &data.string {
                    let _: PluralOperands = black_box(s)
                        .parse()
                        .expect("Failed to parse a number into an operands.");
                }
            })
        });

        {
            let mut group = c.benchmark_group("plurals/operands/create/string/samples");
            for s in &data.string_samples {
                group.bench_with_input(BenchmarkId::from_parameter(s), s, |b, s| {
                    b.iter(|| {
                        let _: PluralOperands = black_box(s).parse().unwrap();
                    })
                });
            }
        }
        c.bench_function("plurals/operands/eq/mostly_unequal", |b| {
            let p: PluralOperands = "1".parse().expect("Parse successful");
            b.iter(|| {
                for s in &data.isize {
                    let q: PluralOperands = black_box(*s)
                        .try_into()
                        .expect("Failed to parse a number into operands.");
                    let _ = black_box(black_box(p) == black_box(q));
                }
            })
        });

        c.bench_function("plurals/operands/eq/mostly_equal", |b| {
            b.iter(|| {
                for s in &data.isize {
                    let p: PluralOperands = black_box(*s)
                        .try_into()
                        .expect("Failed to parse a number into operands.");
                    let q: PluralOperands = black_box(*s)
                        .try_into()
                        .expect("Failed to parse a number into operands.");
                    let _ = black_box(black_box(p) == black_box(q));
                }
            })
        });

        c.bench_function("plurals/operands/create/from_fixed_decimal", |b| {
            b.iter(|| {
                for s in &data.fixed_decimals {
                    let f: FixedDecimal = FixedDecimal::from(s.value).multiplied_pow10(s.exponent);
                    let _: PluralOperands = PluralOperands::from(black_box(&f));
                }
            });
        });

        {
            let samples = [
                FixedDecimal::from(1_i128).multiplied_pow10(0),
                FixedDecimal::from(123450_i128).multiplied_pow10(-4),
                FixedDecimal::from(2500_i128).multiplied_pow10(-2),
            ];
            let mut group = c.benchmark_group("plurals/operands/create/from_fixed_decimal/samples");
            for s in samples.iter() {
                group.bench_with_input(
                    BenchmarkId::from_parameter(format!("{:?}", &s)),
                    s,
                    |b, f| b.iter(|| PluralOperands::from(black_box(f))),
                );
            }
        }
    }
}

criterion_group!(benches, operands);
criterion_main!(benches);
