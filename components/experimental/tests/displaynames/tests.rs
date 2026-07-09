// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_experimental::displaynames::single::{
    LanguageIdentifierDisplayNameOwned, LanguageIdentifierNameFallbackError,
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
    #[derive(Debug)]
    struct TestCase<'a> {
        pub input_1: &'a Locale,
        pub expected: &'a str,
        pub should_borrow: bool,
        pub single_result: Result<(), LanguageIdentifierNameFallbackError>,
    }
    let cases = [
        TestCase {
            input_1: &locale!("de-CH"),
            expected: "Swiss High German",
            should_borrow: true,
            single_result: Ok(()),
        },
        TestCase {
            input_1: &locale!("zh-Hans"),
            expected: "Simplified Chinese",
            should_borrow: true,
            single_result: Ok(()),
        },
        TestCase {
            input_1: &locale!("es-419"),
            expected: "Latin American Spanish",
            should_borrow: true,
            single_result: Ok(()),
        },
        TestCase {
            input_1: &locale!("es-Cyrl-MX"),
            expected: "Mexican Spanish (Cyrillic)",
            should_borrow: false,
            single_result: Ok(()),
        },
        TestCase {
            input_1: &"en-Latn-GB-fonipa-scouse".parse().unwrap(),
            expected: "British English (Latin, IPA Phonetics, Scouse)",
            should_borrow: false,
            single_result: Ok(()),
        },
        TestCase {
            input_1: &locale!("de-Latn-CH"),
            expected: "Swiss High German (Latin)",
            should_borrow: false,
            single_result: Ok(()),
        },
        TestCase {
            input_1: &locale!("zh-Hans-CN"),
            expected: "Simplified Chinese (China)",
            should_borrow: false,
            single_result: Ok(()),
        },
        TestCase {
            input_1: &locale!("es-419-fonipa"),
            expected: "Latin American Spanish (IPA Phonetics)",
            should_borrow: false,
            single_result: Ok(()),
        },
        TestCase {
            input_1: &locale!("es-Latn-419"),
            expected: "Latin American Spanish (Latin)",
            should_borrow: false,
            single_result: Ok(()),
        },
        TestCase {
            // Language not found
            input_1: &locale!("xx"),
            expected: "xx",
            should_borrow: true,
            single_result: Err(LanguageIdentifierNameFallbackError),
        },
        TestCase {
            input_1: &locale!("xx-YY"),
            expected: "xx (YY)",
            should_borrow: false,
            single_result: Err(LanguageIdentifierNameFallbackError),
        },
        TestCase {
            // Script not found
            input_1: &locale!("en-Qzzz"),
            expected: "English (Qzzz)",
            should_borrow: false,
            single_result: Err(LanguageIdentifierNameFallbackError),
        },
        TestCase {
            // Region not found
            input_1: &locale!("en-QZ"),
            expected: "English (QZ)",
            should_borrow: false,
            single_result: Err(LanguageIdentifierNameFallbackError),
        },
        TestCase {
            // Variant not found
            input_1: &locale!("en-qzzzz"),
            expected: "English (qzzzz)",
            should_borrow: false,
            single_result: Err(LanguageIdentifierNameFallbackError),
        },
        TestCase {
            input_1: &"aa-Brai-CC-fonipa-posix".parse().unwrap(),
            expected: "Afar (Braille, Cocos (Keeling) Islands, IPA Phonetics, Computer)",
            should_borrow: false,
            single_result: Ok(()),
        },
        TestCase {
            input_1: &locale!("nl-BE"),
            expected: "Flemish",
            should_borrow: true,
            single_result: Ok(()),
        },
        TestCase {
            input_1: &locale!("nl-Latn-BE"),
            expected: "Flemish (Latin)",
            should_borrow: false,
            single_result: Ok(()),
        },
        TestCase {
            input_1: &"zh-Hans-fonipa".parse().unwrap(),
            expected: "Simplified Chinese (IPA Phonetics)",
            should_borrow: false,
            single_result: Ok(()),
        },
        TestCase {
            input_1: &locale!("hi-Latn"),
            expected: "Hindi (Latin)",
            should_borrow: true,
            single_result: Ok(()),
        },
        TestCase {
            input_1: &locale!("zh-Hant-HK"),
            expected: "Traditional Chinese (Hong Kong SAR China)",
            should_borrow: false,
            single_result: Ok(()),
        },
        TestCase {
            // Multiple variants
            input_1: &Locale::try_from_str("es-fonipa-posix-valencia").unwrap(),
            expected: "Spanish (IPA Phonetics, Computer, Valencian)",
            should_borrow: false,
            single_result: Ok(()),
        },
    ];
    for cas in &cases {
        // TODO: Add tests for different data locales.
        let locale = locale!("en-001");
        let options: DisplayNamesOptions = Default::default();

        // Test the older LocaleDisplayNamesFormatter
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

        // Test the newer LanguageIdentifierDisplayName
        let lang_id = cas.input_1.id.clone();
        let single_options = LanguageIdentifierDisplayNameOptions::default();
        let single_display_name = LanguageIdentifierDisplayNameOwned::try_new(
            locale.clone().into(),
            lang_id,
            single_options,
        )
        .unwrap();
        let borrowed = single_display_name.as_borrowed();
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
