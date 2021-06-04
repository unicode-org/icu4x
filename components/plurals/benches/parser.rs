// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod fixtures;
mod helpers;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use icu_provider::prelude::*;

fn parser(c: &mut Criterion) {
    use icu_plurals::rules::parse_condition;

    let fixture_data = helpers::get_plurals_data();

    let provider = icu_testdata::get_provider();

    let mut rules = vec![];

    for langid in &fixture_data.langs {
        let data_payload: DataPayload<icu_plurals::provider::PluralRuleStringsV1Marker> = provider
            .load_payload(&DataRequest {
                resource_path: ResourcePath {
                    key: icu_plurals::provider::key::CARDINAL_V1,
                    options: ResourceOptions {
                        variant: None,
                        langid: Some(langid.clone()),
                    },
                },
            })
            .unwrap()
            .take_payload()
            .unwrap();
        let data = data_payload.get();

        let r = &[&data.zero, &data.one, &data.two, &data.few, &data.many];

        for i in r {
            if let Some(x) = i {
                rules.push(x.to_string());
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
