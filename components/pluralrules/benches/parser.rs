mod fixtures;
mod helpers;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use icu_pluralrules::PluralCategory;

fn plurals_bench(c: &mut Criterion) {
    use icu_pluralrules::rules::Lexer;

    let plurals_data = helpers::get_plurals_data();

    let pl_data = plurals_data
        .rules
        .get("pl")
        .expect("Polish data should be in the fixture.");
    let pl_data: Vec<&String> = PluralCategory::all()
        .filter_map(|cat| pl_data.get(cat))
        .collect();

    c.bench_function("parser/lex", |b| {
        b.iter(|| {
            for val in &pl_data {
                let lexer = Lexer::new(black_box(val.as_bytes()));
                let _ = lexer.count();
            }
        })
    });
}

criterion_group!(benches, plurals_bench,);
criterion_main!(benches);
