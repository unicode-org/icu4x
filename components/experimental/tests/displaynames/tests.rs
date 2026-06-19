// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_experimental::displaynames::{DisplayNamesOptions, multi::LocaleDisplayNamesFormatter};
use icu_locale_core::Locale;
use icu_locale_core::locale;
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
        TestCase {
            input_1: &locale!("nl-BE"),
            expected: "Flemish",
            should_borrow: true,
        },
        TestCase {
            input_1: &locale!("nl-Latn-BE"),
            expected: "Flemish (Latin)",
            should_borrow: false,
        },
        TestCase {
            input_1: &"zh-Hans-fonipa".parse().unwrap(),
            expected: "Simplified Chinese (IPA Phonetics)",
            should_borrow: false,
        },
        TestCase {
            input_1: &locale!("hi-Latn"),
            expected: "Hindi (Latin)",
            should_borrow: true,
        },
    ];
    for cas in &cases {
        // TODO: Add tests for different data locales.
        let locale = locale!("en-001");
        let options: DisplayNamesOptions = Default::default();

        let display_name = LocaleDisplayNamesFormatter::try_new(locale.clone().into(), options)
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

        // Test our new single formatter implementation (only for cases that are in the data, i.e. not "xx")
        if cas.input_1.id.language != icu_locale_core::subtags::language!("xx") {
            use icu_experimental::displaynames::single::LanguageIdentifierDisplayNameOwned;
            let lang_id = cas.input_1.id.clone();
            let single_display_name = LanguageIdentifierDisplayNameOwned::try_new(
                locale.clone().into(),
                lang_id,
                options,
            )
            .expect("Data should load successfully");

            use writeable::assert_writeable_eq;
            assert_writeable_eq!(single_display_name, cas.expected);
        }
    }
}

#[test]
fn test_single_language_display_name() {
    use icu_experimental::displaynames::DisplayNamesOptions;
    use icu_experimental::displaynames::single::LanguageIdentifierDisplayNameOwned;
    use icu_locale_core::{langid, locale};
    use writeable::assert_writeable_eq;

    let locale = locale!("en-001");
    let options: DisplayNamesOptions = Default::default();

    // This should format "zh-Hant-HK" to "Traditional Chinese (Hong Kong SAR China)" in "en-001"
    let lang_id = langid!("zh-Hant-HK");
    let lang_name = LanguageIdentifierDisplayNameOwned::try_new(locale.into(), lang_id, options)
        .expect("Data should load successfully");

    assert_writeable_eq!(lang_name, "Traditional Chinese (Hong Kong SAR China)");
}

#[cfg(any())]
// TODO(#7825): Enable this test once LanguageIdentifierDisplayNameOwned supports Style::Menu.
#[test]
fn test_single_language_display_name_menu() {
    use icu_experimental::displaynames::single::LanguageIdentifierDisplayNameOwned;
    use icu_experimental::displaynames::{DisplayNamesOptions, Style};
    use icu_locale_core::{langid, locale};
    use writeable::assert_writeable_eq;

    let locale = locale!("en-001");
    let mut options: DisplayNamesOptions = Default::default();
    options.style = Some(Style::Menu);

    // This should format "zh-Hant-HK" to "Chinese (Traditional, Hong Kong)" in "en-001" using Style::Menu
    let lang_id = langid!("zh-Hant-HK");
    let lang_name = LanguageIdentifierDisplayNameOwned::try_new(locale.into(), lang_id, options)
        .expect("Data should load successfully");

    assert_writeable_eq!(lang_name, "Chinese (Traditional, Hong Kong)");
}
