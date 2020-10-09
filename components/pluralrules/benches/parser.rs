mod fixtures;
mod helpers;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use icu_data_provider::{icu_data_key, structs, DataEntry, DataProvider, DataRequest};
use icu_fs_data_provider::FsDataProvider;
use std::borrow::Cow;

fn parser(c: &mut Criterion) {
    use icu_pluralrules::rules::parse_condition;

    let plurals_data = helpers::get_plurals_data();

    let provider = FsDataProvider::try_new("../../resources/testdata/data/json")
        .expect("Loading file from testdata directory");

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

    c.bench_function("parser/overview", |b| {
        b.iter(|| {
            for rule in &rules {
                let _ = parse_condition(black_box(rule.as_bytes()));
            }
        })
    });

    #[cfg(feature = "bench")]
    c.bench_function("parser/lex", |b| {
        use icu_pluralrules::rules::Lexer;
        b.iter(|| {
            for rule in &rules {
                let _ = Lexer::new(black_box(rule.as_bytes())).count();
            }
        })
    });
}

criterion_group!(benches, parser,);
criterion_main!(benches);
