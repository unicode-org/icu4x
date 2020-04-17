mod fixtures;
mod helpers;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use icu_locale::LanguageIdentifier;

fn langid_benches(c: &mut Criterion) {
    let path = "./benches/fixtures/langid.json";
    let data: fixtures::LangId = helpers::read_fixture(path).expect("Failed to read a fixture");

    // Construct
    {
        let mut group = c.benchmark_group("langid_construct");

        let data_bytes: Vec<&[u8]> = data.canonicalized.iter().map(|s| s.as_bytes()).collect();

        group.bench_function("from_bytes", |b| {
            b.iter(|| {
                for s in &data_bytes {
                    let _: Result<LanguageIdentifier, _> =
                        LanguageIdentifier::from_bytes(black_box(s));
                }
            })
        });

        group.bench_function("from_str", |b| {
            b.iter(|| {
                for s in &data.canonicalized {
                    let _: Result<LanguageIdentifier, _> = black_box(s).parse();
                }
            })
        });

        group.finish();
    }

    // Stringify
    {
        let langids: Vec<LanguageIdentifier> = data
            .canonicalized
            .iter()
            .map(|s| s.parse().unwrap())
            .collect();

        c.bench_function("langid_to_string", |b| {
            b.iter(|| {
                for s in &langids {
                    let _ = black_box(s).to_string();
                }
            })
        });
    }

    // Compare
    {
        let langids: Vec<LanguageIdentifier> = data
            .canonicalized
            .iter()
            .map(|s| s.parse().unwrap())
            .collect();
        let langids2: Vec<LanguageIdentifier> = data
            .canonicalized
            .iter()
            .map(|s| s.parse().unwrap())
            .collect();

        let mut group = c.benchmark_group("langid_compare");

        group.bench_function("langid", |b| {
            b.iter(|| {
                for (lid1, lid2) in langids.iter().zip(langids2.iter()) {
                    let _ = black_box(lid1) == black_box(lid2);
                }
            })
        });

        group.bench_function("str", |b| {
            b.iter(|| {
                for (lid, s) in langids.iter().zip(data.canonicalized.iter()) {
                    let _ = black_box(lid) == &black_box(s).as_str();
                }
            })
        });

        group.finish();
    }

    // Canonicalize
    {
        c.bench_function("langid_canonicalize", |b| {
            b.iter(|| {
                for s in &data.casing {
                    let _ = LanguageIdentifier::canonicalize(black_box(s));
                }
            })
        });
    }
}

criterion_group!(benches, langid_benches,);
criterion_main!(benches);
