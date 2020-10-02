mod fixtures;
mod helpers;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn main_bench(c: &mut Criterion) {
    {
        let data = helpers::get_numbers_data();

        c.bench_function("operands/total", |b| {
            use icu_num_util::FixedDecimal;
            use icu_pluralrules::PluralOperands;
            use std::convert::TryInto;

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
                    let f: FixedDecimal = FixedDecimal::from(s.value)
                        .multiplied_pow10(s.exponent)
                        .unwrap();
                    let _: PluralOperands = PluralOperands::from(black_box(&f));
                }
            })
        });
    }

    {
        use icu_fs_data_provider::FsDataProvider;
        use icu_pluralrules::rules::parse_condition;
        use icu_pluralrules::{PluralCategory, PluralRuleType, PluralRules};

        let plurals_data = helpers::get_plurals_data();
        let numbers_data = helpers::get_numbers_data();

        let pl_data = plurals_data
            .rules
            .get("pl")
            .expect("Polish data should be in the fixture.");
        let pl_data: Vec<&String> = PluralCategory::all()
            .filter_map(|cat| pl_data.get(cat))
            .collect();

        c.bench_function("parser/total", |b| {
            b.iter(|| {
                for val in &pl_data {
                    let _ = parse_condition(black_box(val.as_bytes()));
                }
            })
        });

        let provider = FsDataProvider::try_new("./tests/data/json_plurals_37")
            .expect("Loading file from testdata directory");

        c.bench_function("pluralrules/total", |b| {
            b.iter(|| {
                for lang in &plurals_data.langs {
                    let pr =
                        PluralRules::try_new(lang.clone(), PluralRuleType::Cardinal, &provider)
                            .unwrap();
                    for s in &numbers_data.usize {
                        let _ = pr.select(*s);
                    }
                }
            })
        });
    }
}

criterion_group!(benches, main_bench,);
criterion_main!(benches);
