mod fixtures;
mod helpers;

use criterion::{criterion_group, criterion_main, Criterion};

use icu_fs_data_provider::FsDataProvider;
use icu_pluralrules::{PluralRuleType, PluralRules};

fn pluralrules(c: &mut Criterion) {
    let plurals_data = helpers::get_plurals_data();
    let numbers_data = helpers::get_numbers_data();

    let provider = FsDataProvider::try_new("./tests/data/json_plurals_37")
        .expect("Loading file from testdata directory");

    c.bench_function("pluralrules/overview", |b| {
        b.iter(|| {
            for lang in &plurals_data.langs {
                let pr = PluralRules::try_new(lang.clone(), PluralRuleType::Cardinal, &provider)
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
                    PluralRules::try_new(lang.clone(), PluralRuleType::Ordinal, &provider).unwrap();
                    PluralRules::try_new(lang.clone(), PluralRuleType::Cardinal, &provider)
                        .unwrap();
                }
            });
        });

        let loc: LanguageIdentifier = "pl".parse().unwrap();
        let pr = PluralRules::try_new(loc, PluralRuleType::Cardinal, &provider).unwrap();
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
