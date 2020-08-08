mod fixtures;
mod helpers;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use icu_locale::Locale;

fn locale_benches(c: &mut Criterion) {
    let path = "./benches/fixtures/locale.json";
    let data: fixtures::LocaleList = helpers::read_fixture(path).expect("Failed to read a fixture");

    // Construct
    {
        let mut group = c.benchmark_group("locale/construct");

        construct!(group, Locale, "locale", &data.canonicalized);

        group.finish();
    }

    // Stringify
    {
        let mut group = c.benchmark_group("locale/to_string");

        let locales: Vec<Locale> = data
            .canonicalized
            .iter()
            .map(|s| s.parse().unwrap())
            .collect();

        to_string!(group, Locale, "locale", &locales);

        group.finish();
    }

    // Compare
    {
        let mut group = c.benchmark_group("locale/compare");

        let locales: Vec<Locale> = data
            .canonicalized
            .iter()
            .map(|s| s.parse().unwrap())
            .collect();
        let locales2: Vec<Locale> = data
            .canonicalized
            .iter()
            .map(|s| s.parse().unwrap())
            .collect();

        compare_struct!(group, Locale, "locale", &locales, &locales2);

        compare_str!(group, Locale, "locale", &locales, &data.canonicalized);

        group.finish();
    }

    // Canonicalize
    {
        let mut group = c.benchmark_group("locale/canonicalize");

        canonicalize!(group, Locale, "locale", &data.casing);

        group.finish();
    }
}

criterion_group!(benches, locale_benches,);
criterion_main!(benches);
