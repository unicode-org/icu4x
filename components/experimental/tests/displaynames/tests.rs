// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_experimental::displaynames::{
    DisplayNamesOptions, LanguageIdentifierDisplayNameOptions, multi::LocaleDisplayNamesFormatter,
};
use icu_locale_core::Locale;
use icu_locale_core::locale;
use std::borrow::Cow;
use writeable::assert_writeable_eq;

#[test]
fn test_concatenate() {
    #[derive(Debug)]
    struct TestCase<'a> {
        pub input_1: &'a Locale,
        pub expected: &'a str,
        pub should_borrow: bool,
        /// TODO(#8100): single should not error
        pub single_should_err: bool,
    }
    let cases = [
        TestCase {
            input_1: &locale!("de-CH"),
            expected: "Swiss High German",
            should_borrow: true,
            single_should_err: false,
        },
        TestCase {
            input_1: &locale!("zh-Hans"),
            expected: "Simplified Chinese",
            should_borrow: true,
            single_should_err: false,
        },
        TestCase {
            input_1: &locale!("es-419"),
            expected: "Latin American Spanish",
            should_borrow: true,
            single_should_err: false,
        },
        TestCase {
            input_1: &locale!("es-Cyrl-MX"),
            expected: "Mexican Spanish (Cyrillic)",
            should_borrow: false,
            single_should_err: false,
        },
        TestCase {
            input_1: &"en-Latn-GB-fonipa-scouse".parse().unwrap(),
            expected: "British English (Latin, IPA Phonetics, Scouse)",
            should_borrow: false,
            single_should_err: false,
        },
        TestCase {
            input_1: &locale!("de-Latn-CH"),
            expected: "Swiss High German (Latin)",
            should_borrow: false,
            single_should_err: false,
        },
        TestCase {
            input_1: &locale!("zh-Hans-CN"),
            expected: "Simplified Chinese (China)",
            should_borrow: false,
            single_should_err: false,
        },
        TestCase {
            input_1: &locale!("es-419-fonipa"),
            expected: "Latin American Spanish (IPA Phonetics)",
            should_borrow: false,
            single_should_err: false,
        },
        TestCase {
            input_1: &locale!("es-Latn-419"),
            expected: "Latin American Spanish (Latin)",
            should_borrow: false,
            single_should_err: false,
        },
        TestCase {
            // Language not found
            input_1: &locale!("xx"),
            expected: "xx",
            should_borrow: true,
            single_should_err: true,
        },
        TestCase {
            input_1: &locale!("xx-YY"),
            expected: "xx (YY)",
            should_borrow: false,
            single_should_err: true,
        },
        TestCase {
            // Script not found
            input_1: &locale!("en-Qzzz"),
            expected: "English (Qzzz)",
            should_borrow: false,
            single_should_err: true,
        },
        TestCase {
            // Region not found
            input_1: &locale!("en-QZ"),
            expected: "English (QZ)",
            should_borrow: false,
            single_should_err: true,
        },
        TestCase {
            // Variant not found
            input_1: &locale!("en-qzzzz"),
            expected: "English (qzzzz)",
            should_borrow: false,
            single_should_err: true,
        },
        TestCase {
            input_1: &"aa-Brai-CC-fonipa-posix".parse().unwrap(),
            expected: "Afar (Braille, Cocos (Keeling) Islands, IPA Phonetics, Computer)",
            should_borrow: false,
            single_should_err: false,
        },
        TestCase {
            input_1: &locale!("nl-BE"),
            expected: "Flemish",
            should_borrow: true,
            single_should_err: false,
        },
        TestCase {
            input_1: &locale!("nl-Latn-BE"),
            expected: "Flemish (Latin)",
            should_borrow: false,
            single_should_err: false,
        },
        TestCase {
            input_1: &"zh-Hans-fonipa".parse().unwrap(),
            expected: "Simplified Chinese (IPA Phonetics)",
            should_borrow: false,
            single_should_err: false,
        },
        TestCase {
            input_1: &locale!("hi-Latn"),
            expected: "Hindi (Latin)",
            should_borrow: true,
            single_should_err: false,
        },
        TestCase {
            input_1: &locale!("zh-Hant-HK"),
            expected: "Traditional Chinese (Hong Kong SAR China)",
            should_borrow: false,
            single_should_err: false,
        },
        TestCase {
            // Multiple variants
            input_1: &Locale::try_from_str("es-fonipa-posix-valencia").unwrap(),
            expected: "Spanish (IPA Phonetics, Computer, Valencian)",
            should_borrow: false,
            single_should_err: false,
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
        use icu_experimental::displaynames::single::LanguageIdentifierDisplayNameOwned;
        let lang_id = cas.input_1.id.clone();
        let single_options = LanguageIdentifierDisplayNameOptions::default();
        let result = LanguageIdentifierDisplayNameOwned::try_new(
            locale.clone().into(),
            lang_id,
            single_options,
        );
        match result {
            Ok(single_display_name) => {
                assert_eq!(cas.single_should_err, false, "{cas:?}");
                assert_writeable_eq!(single_display_name, cas.expected);
            }
            Err(_) => {
                assert_eq!(cas.single_should_err, true, "{cas:?}");
            }
        }
    }
}
