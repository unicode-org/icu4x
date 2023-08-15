use icu_displaynames::{DisplayNamesOptions, LocaleDisplayNamesFormatter};
use icu_locid::locale;
use icu_locid::Locale;
use std::str::FromStr;

#[test]
fn test_concatenate() {
    #[derive(Debug)]
    struct TestCase<'a> {
        pub input_1: &'a Locale,
        pub expected: &'a str,
    }
    let cases = [
        TestCase {
            input_1: &locale!("de-CH"),
            expected: "Swiss High German",
        },
        TestCase {
            input_1: &locale!("zh_Hans"),
            expected: "Simplified Chinese",
        },
        TestCase {
            input_1: &locale!("es-419"),
            expected: "Latin American Spanish",
        },
        TestCase {
            input_1: &locale!("es-Cyrl-MX"),
            expected: "Spanish (Cyrillic, Mexico)",
        },
        TestCase {
            input_1: &Locale::from_str("en-Latn-GB-fonipa-scouse").unwrap(),
            expected: "English (Latin, United Kingdom, IPA Phonetics, Scouse)",
        },
        // TODO: Uncomment this when the fallback is properly supported.
        // TestCase {
        //     input_1: &locale!("xx-YY"),
        //     expected: "xx (YY)",
        // },
    ];
    for cas in &cases {
        // TODO: Add tests for different data locales.
        let locale = locale!("en-001");
        let options: DisplayNamesOptions = Default::default();

        let display_name = LocaleDisplayNamesFormatter::try_new_unstable(
            &icu_testdata::unstable(),
            &locale.into(),
            options,
        )
        .expect("Data should load successfully");

        assert_eq!(display_name.of(cas.input_1), cas.expected);
    }
}
