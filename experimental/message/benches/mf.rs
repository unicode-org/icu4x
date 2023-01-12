// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use icu_message::parser::Parser;
use icu_message::types::VariableType;
use icu_message::MessageFormat;
use std::collections::HashMap;

fn overview_bench(c: &mut Criterion) {
    let source = "{Hello World}";
    c.bench_function("message/format/simple/format_from_source", |b| {
        let mf = MessageFormat::<&str>::new();
        b.iter(|| {
            let _ = mf.format_from_source::<&str, &str>(black_box(source), None);
        })
    });

    c.bench_function("message/format/simple/format_to_string", |b| {
        let mf = MessageFormat::<&str>::new();
        let parser = Parser::new(source);
        let msg = parser.parse().unwrap();
        b.iter(|| {
            let _ = mf.format_to_string::<&str, &str>(black_box(&msg), None);
        })
    });

    let source = "{Today is {$today}} a good day.";
    let mut vars = HashMap::new();
    vars.insert("today".to_string(), VariableType::String("January 25 2022"));

    c.bench_function("message/format/placeholder/format_from_source", |b| {
        let mf = MessageFormat::<&str>::new();
        b.iter(|| {
            let _ = mf.format_from_source::<&str, _>(black_box(source), Some(&vars));
        })
    });

    c.bench_function("message/format/placeholder/format_to_string", |b| {
        let mf = MessageFormat::<&str>::new();
        let parser = Parser::new(source);
        let msg = parser.parse().unwrap();
        b.iter(|| {
            let _ = mf.format_to_string::<&str, &str>(black_box(&msg), Some(&vars));
        })
    });
}

fn compare_bench(c: &mut Criterion) {
    let mut sources = vec![];
    for i in 0..99 {
        let source = format!("{{Value {i}}}");
        sources.push(source);
    }

    let messages: Vec<_> = sources
        .iter()
        .map(|s| {
            let parser = Parser::new(s.as_str());
            parser.parse().unwrap()
        })
        .collect();

    c.bench_function("message/format/compare/simple", |b| {
        let mf = MessageFormat::<&str>::new();
        b.iter(|| {
            for msg in &messages {
                let _ = mf.format_to_string::<_, &str>(black_box(msg), None);
            }
        })
    });
}

criterion_group!(benches, overview_bench, compare_bench);
criterion_main!(benches);
