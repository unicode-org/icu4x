use criterion::{criterion_group, criterion_main, Criterion};

use icu_datetime::pattern::Pattern;

fn pattern_benches(c: &mut Criterion) {
    let inputs = vec![
        "dd/MM/y",
        "dd/MM",
        "d MMM",
        "d MMM y",
        "MMMM y",
        "d MMMM",
        "HH:mm:ss",
        "HH:mm",
        "y",
        "mm:ss",
        "h:mm:ss B",
        "E, h:mm B",
        "E, h:mm:ss B",
        "E d",
        "E h:mm a",
        "y G",
        "MMM y G",
        "dd/MM",
        "E, dd/MM",
        "LLL",
        "E, d MMM y",
        "E, dd/MM/y",
    ];

    {
        let mut group = c.benchmark_group("pattern");

        group.bench_function("parse", |b| {
            b.iter(|| {
                for input in &inputs {
                    let _ = Pattern::from_bytes(input.as_bytes()).unwrap();
                }
            })
        });

        group.finish();
    }
}

criterion_group!(benches, pattern_benches,);
criterion_main!(benches);
