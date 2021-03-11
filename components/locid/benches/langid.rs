// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).
mod fixtures;
mod helpers;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use icu_locid::LanguageIdentifier;

fn langid_benches(c: &mut Criterion) {
    let path = "./benches/fixtures/langid.json";
    let data: fixtures::LocaleList = helpers::read_fixture(path).expect("Failed to read a fixture");

    // Overview
    {
        let mut group = c.benchmark_group("langid");

        overview!(group, LanguageIdentifier, &data.canonicalized, "en-US");

        group.finish();
    }

    #[cfg(feature = "bench")]
    {
        use criterion::BenchmarkId;
        use icu_locid::Locale;

        // Construct
        {
            let mut group = c.benchmark_group("langid/construct");

            construct!(group, LanguageIdentifier, "langid", &data.canonicalized);
            construct!(group, Locale, "locale", &data.canonicalized);

            group.finish();
        }

        // Stringify
        {
            let mut group = c.benchmark_group("langid/to_string");

            let langids: Vec<LanguageIdentifier> = data
                .canonicalized
                .iter()
                .map(|s| s.parse().unwrap())
                .collect();

            to_string!(group, LanguageIdentifier, "langid", &langids);
            to_string!(group, Locale, "locale", &langids);

            group.finish();
        }

        // Compare
        {
            let mut group = c.benchmark_group("langid/compare");

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

            compare_struct!(group, LanguageIdentifier, "langid", &langids, &langids2);
            compare_struct!(group, Locale, "locale", &langids, &langids2);

            compare_str!(
                group,
                LanguageIdentifier,
                "langid",
                &langids,
                &data.canonicalized
            );
            compare_str!(group, Locale, "locale", &langids, &data.canonicalized);

            group.finish();
        }

        // Canonicalize
        {
            let mut group = c.benchmark_group("langid/canonicalize");

            canonicalize!(group, LanguageIdentifier, "langid", &data.casing);
            canonicalize!(group, Locale, "locale", &data.casing);

            group.finish();
        }
    }
}

criterion_group!(benches, langid_benches,);
criterion_main!(benches);
