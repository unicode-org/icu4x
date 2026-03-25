// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_experimental::displaynames::{DisplayNamesOptions, LocaleDisplayNamesFormatter};
use icu_locale_core::locale;
use icu_locale_core::Locale;
use std::borrow::Cow;

#[test]
fn test_concatenate() {
    #[derive(Debug)]
    struct TestCase<'a> {
        pub input_1: &'a Locale,
        pub expected: &'a str,
        pub should_borrow: bool,
    }
    let cases = [
        TestCase {
            input_1: &locale!("de-CH"),
            expected: "Swiss High German",
            should_borrow: true,
        },
        TestCase {
            input_1: &locale!("zh-Hans"),
            expected: "Simplified Chinese",
            should_borrow: true,
        },
        TestCase {
            input_1: &locale!("es-419"),
            expected: "Latin American Spanish",
            should_borrow: true,
        },
        TestCase {
            input_1: &locale!("es-Cyrl-MX"),
            expected: "Mexican Spanish (Cyrillic)",
            should_borrow: false,
        },
        TestCase {
            input_1: &"en-Latn-GB-fonipa-scouse".parse().unwrap(),
            expected: "British English (Latin, IPA Phonetics, Scouse)",
            should_borrow: false,
        },
        TestCase {
            input_1: &locale!("de-Latn-CH"),
            expected: "Swiss High German (Latin)",
            should_borrow: false,
        },
        TestCase {
            input_1: &locale!("zh-Hans-CN"),
            expected: "Simplified Chinese (China)",
            should_borrow: false,
        },
        TestCase {
            input_1: &locale!("es-419-fonipa"),
            expected: "Latin American Spanish (IPA Phonetics)",
            should_borrow: false,
        },
        TestCase {
            input_1: &locale!("es-Latn-419"),
            expected: "Latin American Spanish (Latin)",
            should_borrow: false,
        },
        TestCase {
            input_1: &locale!("xx"),
            expected: "xx",
            should_borrow: true,
        },
        TestCase {
            input_1: &locale!("xx-YY"),
            expected: "xx (YY)",
            should_borrow: false,
        },
        TestCase {
            input_1: &"aa-Brai-CC-fonipa-posix".parse().unwrap(),
            expected: "Afar (Braille, Cocos (Keeling) Islands, IPA Phonetics, Computer)",
            should_borrow: false,
        },
    ];
    for cas in &cases {
        // TODO: Add tests for different data locales.
        let locale = locale!("en-001");
        let options: DisplayNamesOptions = Default::default();

        let display_name = LocaleDisplayNamesFormatter::try_new(locale.into(), options)
            .expect("Data should load successfully");

        let result = display_name.of(cas.input_1);
        assert_eq!(result, cas.expected);

        if cas.should_borrow {
            assert!(matches!(result, Cow::Borrowed(_)));
        } else {
            assert!(matches!(result, Cow::Owned(_)));
            let result = result.into_owned();
            assert_eq!(result.capacity(), result.len());
        }
    }
}

#[test]
fn test_region_display_name_short() {
    use icu_experimental::displaynames::{DisplayNamesPreferences, RegionDisplayName};
    use icu_locale_core::subtags::region;
    use writeable::assert_writeable_eq;

    let mut prefs = DisplayNamesPreferences::default();
    prefs.locale_preferences = (&locale!("en-US")).into();

    let display_name = RegionDisplayName::try_new_short(prefs, region!("US"))
        .expect("Data should load successfully");

    // In en-US, US short is "US".
    assert_writeable_eq!(display_name, "US");

    let display_name_long =
        RegionDisplayName::try_new(prefs, region!("US")).expect("Data should load successfully");
    assert_writeable_eq!(display_name_long, "United States");

    let display_name_ae = RegionDisplayName::try_new_short(prefs, region!("AE"))
        .expect("Data should load successfully");
    // AE does not have a short name in English, should fall back to long.
    assert_writeable_eq!(display_name_ae, "United Arab Emirates");
}
