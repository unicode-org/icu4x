// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

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
        TestCase {
            input_1: &Locale::from_str("de-Latn-CH").unwrap(),
            expected: "German (Latin, Switzerland)",
        },
        TestCase {
            input_1: &Locale::from_str("zh-Hans-CN").unwrap(),
            expected: "Simplified Chinese (China)",
        },
        TestCase {
            input_1: &Locale::from_str("es-419-fonipa").unwrap(),
            expected: "Latin American Spanish (IPA Phonetics)",
        },
        TestCase {
            input_1: &Locale::from_str("es-Latn-419").unwrap(),
            expected: "Spanish (Latin, Latin America)",
        },
        TestCase {
            input_1: &locale!("xx-YY"),
            expected: "xx (YY)",
        },
    ];
    for cas in &cases {
        // TODO: Add tests for different data locales.
        let locale = locale!("en-001");
        let options: DisplayNamesOptions = Default::default();

        let display_name = LocaleDisplayNamesFormatter::try_new(&locale.into(), options)
            .expect("Data should load successfully");

        assert_eq!(display_name.of(cas.input_1), cas.expected);
    }
}
