// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
mod fixtures;
mod helpers;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use icu_data_provider::{icu_data_key, structs, DataEntry, DataProvider, DataRequest};
use std::borrow::Cow;

fn parser(c: &mut Criterion) {
    use icu_plurals::rules::parse_condition;

    let plurals_data = helpers::get_plurals_data();

    let provider = icu_testdata::get_provider();

    let mut rules = vec![];

    for langid in &plurals_data.langs {
        let response = provider
            .load(&DataRequest {
                data_key: icu_data_key!(plurals: cardinal@1),
                data_entry: DataEntry {
                    variant: None,
                    langid: langid.clone(),
                },
            })
            .unwrap();
        let plurals_data: Cow<structs::plurals::PluralRuleStringsV1> =
            response.take_payload().unwrap();

        let r = &[
            &plurals_data.zero,
            &plurals_data.one,
            &plurals_data.two,
            &plurals_data.few,
            &plurals_data.many,
        ];

        for i in r {
            if let Some(x) = i {
                rules.push(x.clone());
            }
        }
    }

    c.bench_function("plurals/parser/overview", |b| {
        b.iter(|| {
            for rule in &rules {
                let _ = parse_condition(black_box(rule.as_bytes()));
            }
        })
    });

    #[cfg(feature = "bench")]
    c.bench_function("plurals/parser/lex", |b| {
        use icu_plurals::rules::Lexer;
        b.iter(|| {
            for rule in &rules {
                let _ = Lexer::new(black_box(rule.as_bytes())).count();
            }
        })
    });
}

criterion_group!(benches, parser,);
criterion_main!(benches);
