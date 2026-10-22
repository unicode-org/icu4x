// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locale::names::{
    DisplayNamesPreferences, LanguageDisplay, LanguageIdentifierDisplayName,
    LanguageIdentifierDisplayNameOptions, LanguageIdentifierNameFallbackError,
};
use icu_locale_core::{langid, locale};
use writeable::{Part, assert_try_writeable_parts_eq};

#[test]
fn test_language_identifier_display_name_styles_and_lengths_en() {
    let prefs_en = DisplayNamesPreferences::from(locale!("en"));

    let mut dialect_options = LanguageIdentifierDisplayNameOptions::default();
    dialect_options.language_display = Some(LanguageDisplay::Dialect);

    let mut standard_options = LanguageIdentifierDisplayNameOptions::default();
    standard_options.language_display = Some(LanguageDisplay::Standard);

    let menu_options = LanguageIdentifierDisplayNameOptions::default();

    macro_rules! check_row {
        ($constructor:ident, $options:expr, [$en_gb:literal, $az:literal, $zh_hans:literal]) => {{
            for (id, expected) in [
                (langid!("en-GB"), $en_gb),
                (langid!("az"), $az),
                (langid!("zh-Hans"), $zh_hans),
            ] {
                let display_name =
                    LanguageIdentifierDisplayName::$constructor(prefs_en, id, $options).unwrap();
                assert_try_writeable_parts_eq!(display_name.as_borrowed(), expected, Ok(()), []);
            }
        }};
    }

    // Dialect + Medium (default)
    check_row!(
        try_new_heavy,
        dialect_options,
        ["British English", "Azerbaijani", "Simplified Chinese"]
    );

    // Dialect + Short
    check_row!(
        try_new_short_heavy,
        dialect_options,
        ["UK English", "Azeri", "Simplified Chinese"]
    );

    // Dialect + Long
    check_row!(
        try_new_long_heavy,
        dialect_options,
        [
            "British English",
            "Azerbaijani",
            "Simplified Mandarin Chinese"
        ]
    );

    // Standard + Medium
    check_row!(
        try_new_heavy,
        standard_options,
        [
            "English (United Kingdom)",
            "Azerbaijani",
            "Chinese (Simplified)"
        ]
    );

    // Menu + Short
    check_row!(
        try_new_short_menu_heavy,
        menu_options,
        ["English (UK)", "Azeri", "Chinese, Mandarin (Simplified)"]
    );
}

#[test]
fn test_language_identifier_display_name_tiers_fr() {
    let prefs_fr = DisplayNamesPreferences::from(locale!("fr"));
    let options = LanguageIdentifierDisplayNameOptions::default();

    macro_rules! check_tier {
        ($constructor:ident, $langid:literal, $expected:literal, $res:expr, $parts:expr) => {{
            let display_name =
                LanguageIdentifierDisplayName::$constructor(prefs_fr, langid!($langid), options)
                    .unwrap();
            assert_try_writeable_parts_eq!(display_name.as_borrowed(), $expected, $res, $parts);
        }};
    }

    // Tiny tier:
    check_tier!(try_new_tiny, "fr-CA", "français (Canada)", Ok(()), []);
    check_tier!(
        try_new_tiny,
        "zh-Hans",
        "zh (Hans)",
        Err(LanguageIdentifierNameFallbackError),
        [(0, 2, Part::ERROR), (4, 8, Part::ERROR)]
    );
    check_tier!(
        try_new_tiny,
        "hi-Deva-IN",
        "hi (Deva, IN)",
        Err(LanguageIdentifierNameFallbackError),
        [
            (0, 2, Part::ERROR),
            (4, 8, Part::ERROR),
            (10, 12, Part::ERROR)
        ]
    );

    // Light tier:
    check_tier!(try_new_light, "fr-CA", "français canadien", Ok(()), []);
    check_tier!(try_new_light, "zh-Hans", "chinois simplifié", Ok(()), []);
    check_tier!(
        try_new_light,
        "hi-Deva-IN",
        "hindi (Deva, Inde)",
        Err(LanguageIdentifierNameFallbackError),
        [(7, 11, Part::ERROR)]
    );

    // Heavy tier:
    check_tier!(try_new_heavy, "fr-CA", "français canadien", Ok(()), []);
    check_tier!(try_new_heavy, "zh-Hans", "chinois simplifié", Ok(()), []);
    check_tier!(
        try_new_heavy,
        "hi-Deva-IN",
        "hindi (dévanagari, Inde)",
        Ok(()),
        []
    );
}
