// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use icu_message::parser::Parser;

fn overview_bench(c: &mut Criterion) {
    let source = "{Hello World}";
    c.bench_function("message/parse/simple", |b| {
        b.iter(|| {
            let parser = Parser::new(black_box(source));
            let _ = parser.parse();
        })
    });

    let source = "{Today is {$today}} a good day.";
    c.bench_function("message/parse/placeholder", |b| {
        b.iter(|| {
            let parser = Parser::new(black_box(source));
            let _ = parser.parse();
        })
    });
}

fn compare_bench(c: &mut Criterion) {
    let mut messages = vec![];

    for i in 0..99 {
        messages.push(format!("{{Value {i}}}"));
    }

    c.bench_function("message/parse/compare/simple", |b| {
        b.iter(|| {
            for msg in &messages {
                let parser = Parser::new(black_box(msg.as_str()));
                let _ = parser.parse();
            }
        })
    });
}

criterion_group!(benches, overview_bench, compare_bench);
criterion_main!(benches);
