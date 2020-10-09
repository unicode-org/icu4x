mod fixtures;
mod helpers;

use criterion::{criterion_group, criterion_main, Criterion};

use icu_pluralrules::{PluralRuleType, PluralRules};

fn pluralrules(c: &mut Criterion) {
    let plurals_data = helpers::get_plurals_data();
    let numbers_data = helpers::get_numbers_data();

    let provider = icu_testdata::get_provider();

    c.bench_function("pluralrules/overview", |b| {
        b.iter(|| {
            for lang in &plurals_data.langs {
                let pr = PluralRules::try_new(lang.clone(), &provider, PluralRuleType::Cardinal)
                    .unwrap();
                for s in &numbers_data.usize {
                    let _ = pr.select(*s);
                }
            }
        })
    });

    #[cfg(feature = "bench")]
    {
        use criterion::black_box;
        use icu_locale::LanguageIdentifier;

        c.bench_function("pluralrules/construct/fs", |b| {
            b.iter(|| {
                for lang in &plurals_data.langs {
                    PluralRules::try_new(lang.clone(), &provider, PluralRuleType::Ordinal).unwrap();
                    PluralRules::try_new(lang.clone(), &provider, PluralRuleType::Cardinal)
                        .unwrap();
                }
            });
        });

        let loc: LanguageIdentifier = "pl".parse().unwrap();
        let pr = PluralRules::try_new(loc, &provider, PluralRuleType::Cardinal).unwrap();
        c.bench_function("plurals/select/fs", |b| {
            b.iter(|| {
                for s in &numbers_data.usize {
                    let _ = pr.select(black_box(*s));
                }
            })
        });
    }
}

criterion_group!(benches, pluralrules,);
criterion_main!(benches);
