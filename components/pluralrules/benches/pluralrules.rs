mod fixtures;
mod helpers;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use icu_pluralrules::PluralCategory;

fn plurals_bench(c: &mut Criterion) {
    use icu_pluralrules::rules::{parse, Lexer};

    let path = "./benches/fixtures/plurals.json";
    let data: fixtures::PluralsFixture =
        helpers::read_fixture(path).expect("Failed to read a fixture");

    let pl_data = data
        .rules
        .get("pl")
        .expect("Polish data should be in the fixture.");
    let pl_data: Vec<&String> = PluralCategory::all()
        .filter_map(|cat| pl_data.get(cat))
        .collect();

    let mut group = c.benchmark_group("plurals/parser");

    group.bench_function("lex", |b| {
        b.iter(|| {
            for val in &pl_data {
                let lexer = Lexer::new(black_box(val.as_bytes()));
                let _ = lexer.count();
            }
        })
    });

    group.bench_function("parse", |b| {
        b.iter(|| {
            for val in &pl_data {
                let _ = parse(black_box(val.as_bytes()));
            }
        })
    });

    group.finish();

    #[cfg(feature = "io-json")]
    {
        use criterion::BenchmarkId;
        use icu_locale::LanguageIdentifier;
        use icu_pluralrules::data::io::json::DataProvider;
        use icu_pluralrules::PluralOperands;
        use icu_pluralrules::{PluralRuleType, PluralRules};

        let path = "./benches/fixtures/numbers.json";
        let num_data: fixtures::NumbersFixture =
            helpers::read_fixture(path).expect("Failed to read a fixture");

        c.bench_function("plurals/convert+select/json", |b| {
            let loc: LanguageIdentifier = "pl".parse().unwrap();
            let dtp = DataProvider {};
            let pr = PluralRules::try_new(loc, PluralRuleType::Cardinal, &dtp).unwrap();
            b.iter(|| {
                for s in &num_data.usize {
                    let _ = pr.select(*s);
                }
            })
        });

        c.bench_function("plurals/select/json", |b| {
            let loc: LanguageIdentifier = "pl".parse().unwrap();
            let dtp = DataProvider {};
            let pr = PluralRules::try_new(loc, PluralRuleType::Cardinal, &dtp).unwrap();
            let operands: Vec<PluralOperands> = num_data
                .usize
                .iter()
                .map(|d| (*d).into())
                .collect();
            b.iter(|| {
                for op in &operands {
                    let _ = pr.select((*op).clone());
                }
            })
        });

        c.bench_with_input(
            BenchmarkId::new("plurals/construct/json", data.langs.len()),
            &data.langs,
            |b, langs| {
                let dtp = DataProvider {};
                b.iter(|| {
                    for lang in langs {
                        PluralRules::try_new(lang.clone(), PluralRuleType::Ordinal, &dtp).unwrap();
                        PluralRules::try_new(lang.clone(), PluralRuleType::Cardinal, &dtp).unwrap();
                    }
                });
            },
        );
    }
}

criterion_group!(benches, plurals_bench,);
criterion_main!(benches);
