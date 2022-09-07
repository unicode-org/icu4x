// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod fixtures;
mod helpers;

use criterion::{criterion_group, criterion_main, Criterion};

use icu_plurals::{PluralRuleType, PluralRules};

fn pluralrules(c: &mut Criterion) {
    let plurals_data = helpers::get_plurals_data();
    let numbers_data = helpers::get_numbers_data();

    let provider = icu_testdata::unstable();

    c.bench_function("plurals/pluralrules/overview", |b| {
        b.iter(|| {
            for lang in &plurals_data.langs {
                let pr = PluralRules::try_new_unstable(
                    &provider,
                    &lang.into(),
                    PluralRuleType::Cardinal,
                )
                .unwrap();
                for s in &numbers_data.usize {
                    let _ = pr.category_for(*s);
                }
            }
        })
    });

    #[cfg(feature = "bench")]
    {
        use criterion::black_box;
        use icu_locid::locale;

        c.bench_function("plurals/pluralrules/construct/fs", |b| {
            b.iter(|| {
                for lang in &plurals_data.langs {
                    PluralRules::try_new_unstable(&provider, &lang.into(), PluralRuleType::Ordinal)
                        .unwrap();
                    PluralRules::try_new_unstable(
                        &provider,
                        &lang.into(),
                        PluralRuleType::Cardinal,
                    )
                    .unwrap();
                }
            });
        });

        let pr = PluralRules::try_new_unstable(
            &provider,
            &locale!("ru").into(),
            PluralRuleType::Cardinal,
        )
        .unwrap();
        c.bench_function("plurals/pluralrules/select/fs", |b| {
            b.iter(|| {
                for s in &numbers_data.usize {
                    let _ = pr.category_for(black_box(*s));
                }
            })
        });
    }
}

criterion_group!(benches, pluralrules,);
criterion_main!(benches);
