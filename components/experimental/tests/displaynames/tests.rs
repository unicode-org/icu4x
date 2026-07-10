// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_experimental::displaynames::single::{
    LanguageIdentifierDisplayName, LanguageIdentifierDisplayNameOwned,
    LanguageIdentifierNameFallbackError,
};
use icu_experimental::displaynames::{
    DisplayNamesOptions, LanguageIdentifierDisplayNameOptions, multi::LocaleDisplayNamesFormatter,
};
use icu_locale_core::Locale;
use icu_locale_core::locale;
use std::borrow::Cow;
use writeable::{
    Part, Writeable, assert_try_writeable_eq, assert_try_writeable_parts_eq, assert_writeable_eq,
};

#[test]
fn test_concatenate() {
    #[derive(Debug, Clone)]
    enum DisplayType {
        /// The string is valid for both dialect and menu
        Any,
        /// The string is valid for dialect only
        Dialect,
        /// The string is valid for menu only
        Menu,
    }
    #[derive(Debug, Clone)]
    struct TestCase<'a> {
        pub input_1: &'a Locale,
        pub display_type: DisplayType,
        pub expected: &'a str,
        pub should_borrow: bool,
        pub single_result: Result<(), LanguageIdentifierNameFallbackError>,
    }
    let cases = [
        TestCase {
            input_1: &locale!("de-CH"),
            display_type: DisplayType::Dialect,
            expected: "Swiss High German",
            should_borrow: true,
            single_result: Ok(()),
        },
        TestCase {
            input_1: &locale!("de-CH"),
            display_type: DisplayType::Menu,
            expected: "German (Switzerland)",
            should_borrow: false,
            single_result: Ok(()),
        },
        TestCase {
            input_1: &locale!("zh"),
            display_type: DisplayType::Dialect,
            expected: "Chinese",
            should_borrow: true,
            single_result: Ok(()),
        },
        TestCase {
            input_1: &locale!("zh"),
            display_type: DisplayType::Menu,
            expected: "Chinese, Mandarin",
            should_borrow: true,
            single_result: Ok(()),
        },
        TestCase {
            input_1: &locale!("zh-Hans"),
            display_type: DisplayType::Dialect,
            expected: "Simplified Chinese",
            should_borrow: true,
            single_result: Ok(()),
        },
        TestCase {
            input_1: &locale!("zh-Hans"),
            display_type: DisplayType::Menu,
            // Note: this behavior might change in CLDR 49
            expected: "Chinese, Mandarin (Simplified)",
            should_borrow: false,
            single_result: Ok(()),
        },
        TestCase {
            input_1: &locale!("es-419"),
            display_type: DisplayType::Dialect,
            expected: "Latin American Spanish",
            should_borrow: true,
            single_result: Ok(()),
        },
        TestCase {
            input_1: &locale!("es-419"),
            display_type: DisplayType::Menu,
            expected: "Spanish (Latin America)",
            should_borrow: false,
            single_result: Ok(()),
        },
        TestCase {
            input_1: &locale!("es-Cyrl-MX"),
            display_type: DisplayType::Dialect,
            expected: "Mexican Spanish (Cyrillic)",
            should_borrow: false,
            single_result: Ok(()),
        },
        TestCase {
            input_1: &locale!("es-Cyrl-MX"),
            display_type: DisplayType::Menu,
            expected: "Spanish (Cyrillic, Mexico)",
            should_borrow: false,
            single_result: Ok(()),
        },
        TestCase {
            input_1: &"en-Latn-GB-fonipa-scouse".parse().unwrap(),
            display_type: DisplayType::Dialect,
            expected: "British English (Latin, IPA Phonetics, Scouse)",
            should_borrow: false,
            single_result: Ok(()),
        },
        TestCase {
            input_1: &"en-Latn-GB-fonipa-scouse".parse().unwrap(),
            display_type: DisplayType::Menu,
            expected: "English (Latin, United Kingdom, IPA Phonetics, Scouse)",
            should_borrow: false,
            single_result: Ok(()),
        },
        TestCase {
            input_1: &locale!("de-Latn-CH"),
            display_type: DisplayType::Dialect,
            expected: "Swiss High German (Latin)",
            should_borrow: false,
            single_result: Ok(()),
        },
        TestCase {
            input_1: &locale!("de-Latn-CH"),
            display_type: DisplayType::Menu,
            expected: "German (Latin, Switzerland)",
            should_borrow: false,
            single_result: Ok(()),
        },
        TestCase {
            input_1: &locale!("zh-Hans-CN"),
            display_type: DisplayType::Dialect,
            expected: "Simplified Chinese (China)",
            should_borrow: false,
            single_result: Ok(()),
        },
        TestCase {
            input_1: &locale!("zh-Hans-CN"),
            display_type: DisplayType::Menu,
            // Note: this behavior might change in CLDR 49
            expected: "Chinese, Mandarin (Simplified, China)",
            should_borrow: false,
            single_result: Ok(()),
        },
        TestCase {
            input_1: &locale!("es-419-fonipa"),
            display_type: DisplayType::Dialect,
            expected: "Latin American Spanish (IPA Phonetics)",
            should_borrow: false,
            single_result: Ok(()),
        },
        TestCase {
            input_1: &locale!("es-419-fonipa"),
            display_type: DisplayType::Menu,
            expected: "Spanish (Latin America, IPA Phonetics)",
            should_borrow: false,
            single_result: Ok(()),
        },
        TestCase {
            input_1: &locale!("es-Latn-419"),
            display_type: DisplayType::Dialect,
            expected: "Latin American Spanish (Latin)",
            should_borrow: false,
            single_result: Ok(()),
        },
        TestCase {
            input_1: &locale!("es-Latn-419"),
            display_type: DisplayType::Menu,
            expected: "Spanish (Latin, Latin America)",
            should_borrow: false,
            single_result: Ok(()),
        },
        TestCase {
            // Language not found
            input_1: &locale!("xx"),
            display_type: DisplayType::Any,
            expected: "xx",
            should_borrow: true,
            single_result: Err(LanguageIdentifierNameFallbackError),
        },
        TestCase {
            input_1: &locale!("xx-YY"),
            display_type: DisplayType::Any,
            expected: "xx (YY)",
            should_borrow: false,
            single_result: Err(LanguageIdentifierNameFallbackError),
        },
        TestCase {
            // Script not found
            input_1: &locale!("en-Qzzz"),
            display_type: DisplayType::Any,
            expected: "English (Qzzz)",
            should_borrow: false,
            single_result: Err(LanguageIdentifierNameFallbackError),
        },
        TestCase {
            // Region not found
            input_1: &locale!("en-QZ"),
            display_type: DisplayType::Any,
            expected: "English (QZ)",
            should_borrow: false,
            single_result: Err(LanguageIdentifierNameFallbackError),
        },
        TestCase {
            // Variant not found
            input_1: &locale!("en-qzzzz"),
            display_type: DisplayType::Any,
            expected: "English (qzzzz)",
            should_borrow: false,
            single_result: Err(LanguageIdentifierNameFallbackError),
        },
        TestCase {
            input_1: &"aa-Brai-CC-fonipa-posix".parse().unwrap(),
            display_type: DisplayType::Any,
            expected: "Afar (Braille, Cocos (Keeling) Islands, IPA Phonetics, Computer)",
            should_borrow: false,
            single_result: Ok(()),
        },
        TestCase {
            input_1: &locale!("nl-BE"),
            display_type: DisplayType::Dialect,
            expected: "Flemish",
            should_borrow: true,
            single_result: Ok(()),
        },
        TestCase {
            input_1: &locale!("nl-BE"),
            display_type: DisplayType::Menu,
            expected: "Dutch (Belgium)",
            should_borrow: false,
            single_result: Ok(()),
        },
        TestCase {
            input_1: &locale!("nl-Latn-BE"),
            display_type: DisplayType::Dialect,
            expected: "Flemish (Latin)",
            should_borrow: false,
            single_result: Ok(()),
        },
        TestCase {
            input_1: &locale!("nl-Latn-BE"),
            display_type: DisplayType::Menu,
            expected: "Dutch (Latin, Belgium)",
            should_borrow: false,
            single_result: Ok(()),
        },
        TestCase {
            input_1: &"zh-Hans-fonipa".parse().unwrap(),
            display_type: DisplayType::Dialect,
            expected: "Simplified Chinese (IPA Phonetics)",
            should_borrow: false,
            single_result: Ok(()),
        },
        TestCase {
            input_1: &locale!("hi-Latn"),
            display_type: DisplayType::Any,
            expected: "Hindi (Latin)",
            should_borrow: true,
            single_result: Ok(()),
        },
        TestCase {
            input_1: &locale!("zh-Hant-HK"),
            display_type: DisplayType::Dialect,
            expected: "Traditional Chinese (Hong Kong SAR China)",
            should_borrow: false,
            single_result: Ok(()),
        },
        TestCase {
            input_1: &locale!("zh-Hant-HK"),
            display_type: DisplayType::Menu,
            // Note: this behavior might change in CLDR 49
            expected: "Chinese, Mandarin (Traditional, Hong Kong SAR China)",
            should_borrow: false,
            single_result: Ok(()),
        },
        TestCase {
            // Multiple variants
            input_1: &Locale::try_from_str("es-fonipa-posix-valencia").unwrap(),
            display_type: DisplayType::Any,
            expected: "Spanish (IPA Phonetics, Computer, Valencian)",
            should_borrow: false,
            single_result: Ok(()),
        },
        TestCase {
            input_1: &locale!("ku"),
            display_type: DisplayType::Dialect,
            expected: "Kurdish",
            should_borrow: true,
            single_result: Ok(()),
        },
        TestCase {
            input_1: &locale!("ku"),
            display_type: DisplayType::Menu,
            expected: "Kurdish (Kurmanji)",
            should_borrow: false,
            single_result: Ok(()),
        },
        TestCase {
            input_1: &locale!("ku-IQ"),
            display_type: DisplayType::Dialect,
            expected: "Kurdish (Iraq)",
            should_borrow: false,
            single_result: Ok(()),
        },
        TestCase {
            input_1: &locale!("ku-IQ"),
            display_type: DisplayType::Menu,
            expected: "Kurdish (Kurmanji, Iraq)",
            should_borrow: false,
            single_result: Ok(()),
        },
    ];
    for cas in &cases {
        // TODO: Add tests for different data locales.
        let locale = locale!("en-001");

        // Test the older LocaleDisplayNamesFormatter
        fn check_locale_name_formatter(
            formatter: &LocaleDisplayNamesFormatter,
            cas: &TestCase<'_>,
        ) {
            let result = formatter.of(cas.input_1);
            assert_eq!(result, cas.expected, "{cas:?}");
            if cas.should_borrow {
                assert!(matches!(result, Cow::Borrowed(_)), "{cas:?}");
            } else {
                assert!(matches!(result, Cow::Owned(_)), "{cas:?}");
                let result = result.into_owned();
                assert_eq!(result.capacity(), result.len(), "{cas:?}");
            }
        }
        if matches!(cas.display_type, DisplayType::Any | DisplayType::Dialect) {
            let options: DisplayNamesOptions = Default::default();
            let formatter = LocaleDisplayNamesFormatter::try_new(locale.clone().into(), options)
                .expect("Data should load successfully");
            check_locale_name_formatter(&formatter, cas);
        }
        if matches!(cas.display_type, DisplayType::Any | DisplayType::Menu) {
            let mut options: DisplayNamesOptions = Default::default();
            options.language_display = icu_experimental::displaynames::LanguageDisplay::Standard;
            options.style = Some(icu_experimental::displaynames::Style::Menu);
            // "Hindi (Latin)" is a literal string in data,
            // but it gets reconstructed from patterns for Menu names
            let mut cas = cas.clone();
            if cas.expected == "Hindi (Latin)" {
                cas.should_borrow = false;
            }
            // "Kurmanji" Kurdish is not supported in the old code
            if cas.expected == "Kurdish (Kurmanji)" {
                cas.expected = "Kurdish";
                cas.should_borrow = true;
            }
            if cas.expected == "Kurdish (Kurmanji, Iraq)" {
                cas.expected = "Kurdish (Iraq)";
            }
            let formatter = LocaleDisplayNamesFormatter::try_new(locale.clone().into(), options)
                .expect("Data should load successfully");
            check_locale_name_formatter(&formatter, &cas);
        }

        // Test the newer LanguageIdentifierDisplayName
        let lang_id = cas.input_1.id.clone();
        let single_options = LanguageIdentifierDisplayNameOptions::default();

        fn check_language_name_borrowed(
            borrowed: LanguageIdentifierDisplayName<'_>,
            cas: &TestCase<'_>,
        ) {
            assert_writeable_eq!(borrowed, cas.expected, "{cas:?}");
            assert_try_writeable_eq!(borrowed, cas.expected, cas.single_result, "{cas:?}");
            let cow = borrowed.write_to_string();
            if cas.should_borrow {
                assert!(matches!(cow, Cow::Borrowed(_)), "{cas:?}");
            } else {
                assert!(matches!(cow, Cow::Owned(_)), "{cas:?}");
                let result = cow.into_owned();
                assert_eq!(result.capacity(), result.len(), "{cas:?}");
            }
        }
        if matches!(cas.display_type, DisplayType::Any | DisplayType::Dialect) {
            let dname_standard_owned = LanguageIdentifierDisplayNameOwned::try_new(
                locale.clone().into(),
                lang_id.clone(),
                single_options,
            )
            .unwrap();
            let borrowed = dname_standard_owned.as_borrowed();
            check_language_name_borrowed(borrowed, cas);
        }
        if matches!(cas.display_type, DisplayType::Any | DisplayType::Menu) {
            let dname_menu_owned = LanguageIdentifierDisplayNameOwned::try_new_menu(
                locale.clone().into(),
                lang_id,
                single_options,
            )
            .unwrap();
            let borrowed = dname_menu_owned.as_borrowed();
            // "Hindi (Latin)" is a literal string in data,
            // but it gets reconstructed from patterns for Menu names
            let mut cas = cas.clone();
            if cas.expected == "Hindi (Latin)" {
                cas.should_borrow = false;
            }
            check_language_name_borrowed(borrowed, &cas);
        }
    }
}

#[test]
fn test_fallback_parts() {
    let locale = locale!("en-001");
    let options = LanguageIdentifierDisplayNameOptions::default();

    // xx-YY has both language and region missing in CLDR en data.
    // It should fall back to "xx (YY)" and annotate "xx" and "YY" with Part::ERROR.
    let display_name = LanguageIdentifierDisplayNameOwned::try_new(
        locale.into(),
        "xx-Latn-YY".parse().unwrap(),
        options,
    )
    .unwrap();

    assert_try_writeable_parts_eq!(
        display_name.as_borrowed(),
        "xx (Latin, YY)",
        Err(LanguageIdentifierNameFallbackError),
        [(0, 2, Part::ERROR), (11, 13, Part::ERROR)]
    );
}

#[test]
fn test_single_language_display_name_standard() {
    use icu_experimental::displaynames::{LanguageDisplay, LanguageIdentifierDisplayNameOptions};
    use icu_locale_core::{langid, locale};

    let locale = locale!("en-001");
    let mut options = LanguageIdentifierDisplayNameOptions::default();
    options.language_display = Some(LanguageDisplay::Standard);

    // This should format "zh-Hant-HK" to "Chinese (Traditional, Hong Kong SAR China)"
    // in "en-001" using LanguageDisplay::Standard
    let lang_id = langid!("zh-Hant-HK");
    let lang_name = LanguageIdentifierDisplayNameOwned::try_new(locale.into(), lang_id, options)
        .expect("Data should load successfully");

    assert_try_writeable_eq!(
        lang_name.as_borrowed(),
        "Chinese (Traditional, Hong Kong SAR China)"
    );
}
