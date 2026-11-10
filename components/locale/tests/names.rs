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
fn test_language_identifier_display_name_en_dialect_medium() {
    let prefs_en = DisplayNamesPreferences::from(locale!("en"));
    let mut dialect_options = LanguageIdentifierDisplayNameOptions::default();
    dialect_options.language_display = Some(LanguageDisplay::Dialect);

    let display_name =
        LanguageIdentifierDisplayName::try_new_heavy(prefs_en, langid!("en-GB"), dialect_options)
            .unwrap();
    assert_try_writeable_parts_eq!(display_name.as_borrowed(), "British English", Ok(()), []);

    let display_name =
        LanguageIdentifierDisplayName::try_new_heavy(prefs_en, langid!("az"), dialect_options)
            .unwrap();
    assert_try_writeable_parts_eq!(display_name.as_borrowed(), "Azerbaijani", Ok(()), []);

    let display_name =
        LanguageIdentifierDisplayName::try_new_heavy(prefs_en, langid!("zh-Hans"), dialect_options)
            .unwrap();
    assert_try_writeable_parts_eq!(display_name.as_borrowed(), "Simplified Chinese", Ok(()), []);
}

#[test]
fn test_language_identifier_display_name_en_dialect_short() {
    let prefs_en = DisplayNamesPreferences::from(locale!("en"));
    let mut dialect_options = LanguageIdentifierDisplayNameOptions::default();
    dialect_options.language_display = Some(LanguageDisplay::Dialect);

    let display_name = LanguageIdentifierDisplayName::try_new_short_heavy(
        prefs_en,
        langid!("en-GB"),
        dialect_options,
    )
    .unwrap();
    assert_try_writeable_parts_eq!(display_name.as_borrowed(), "UK English", Ok(()), []);

    let display_name = LanguageIdentifierDisplayName::try_new_short_heavy(
        prefs_en,
        langid!("az"),
        dialect_options,
    )
    .unwrap();
    assert_try_writeable_parts_eq!(display_name.as_borrowed(), "Azeri", Ok(()), []);

    let display_name = LanguageIdentifierDisplayName::try_new_short_heavy(
        prefs_en,
        langid!("zh-Hans"),
        dialect_options,
    )
    .unwrap();
    assert_try_writeable_parts_eq!(display_name.as_borrowed(), "Simplified Chinese", Ok(()), []);
}

#[test]
fn test_language_identifier_display_name_en_dialect_long() {
    let prefs_en = DisplayNamesPreferences::from(locale!("en"));
    let mut dialect_options = LanguageIdentifierDisplayNameOptions::default();
    dialect_options.language_display = Some(LanguageDisplay::Dialect);

    let display_name = LanguageIdentifierDisplayName::try_new_long_heavy(
        prefs_en,
        langid!("en-GB"),
        dialect_options,
    )
    .unwrap();
    assert_try_writeable_parts_eq!(display_name.as_borrowed(), "British English", Ok(()), []);

    let display_name =
        LanguageIdentifierDisplayName::try_new_long_heavy(prefs_en, langid!("az"), dialect_options)
            .unwrap();
    assert_try_writeable_parts_eq!(display_name.as_borrowed(), "Azerbaijani", Ok(()), []);

    let display_name = LanguageIdentifierDisplayName::try_new_long_heavy(
        prefs_en,
        langid!("zh-Hans"),
        dialect_options,
    )
    .unwrap();
    assert_try_writeable_parts_eq!(
        display_name.as_borrowed(),
        "Simplified Mandarin Chinese",
        Ok(()),
        []
    );
}

#[test]
fn test_language_identifier_display_name_en_standard_medium() {
    let prefs_en = DisplayNamesPreferences::from(locale!("en"));
    let mut standard_options = LanguageIdentifierDisplayNameOptions::default();
    standard_options.language_display = Some(LanguageDisplay::Standard);

    let display_name =
        LanguageIdentifierDisplayName::try_new_heavy(prefs_en, langid!("en-GB"), standard_options)
            .unwrap();
    assert_try_writeable_parts_eq!(
        display_name.as_borrowed(),
        "English (United Kingdom)",
        Ok(()),
        []
    );

    let display_name =
        LanguageIdentifierDisplayName::try_new_heavy(prefs_en, langid!("az"), standard_options)
            .unwrap();
    assert_try_writeable_parts_eq!(display_name.as_borrowed(), "Azerbaijani", Ok(()), []);

    let display_name = LanguageIdentifierDisplayName::try_new_heavy(
        prefs_en,
        langid!("zh-Hans"),
        standard_options,
    )
    .unwrap();
    assert_try_writeable_parts_eq!(
        display_name.as_borrowed(),
        "Chinese (Simplified)",
        Ok(()),
        []
    );
}

#[test]
fn test_language_identifier_display_name_en_menu_short() {
    let prefs_en = DisplayNamesPreferences::from(locale!("en"));
    let menu_options = LanguageIdentifierDisplayNameOptions::default();

    let display_name = LanguageIdentifierDisplayName::try_new_short_menu_heavy(
        prefs_en,
        langid!("en-GB"),
        menu_options,
    )
    .unwrap();
    assert_try_writeable_parts_eq!(display_name.as_borrowed(), "English (UK)", Ok(()), []);

    let display_name = LanguageIdentifierDisplayName::try_new_short_menu_heavy(
        prefs_en,
        langid!("az"),
        menu_options,
    )
    .unwrap();
    assert_try_writeable_parts_eq!(display_name.as_borrowed(), "Azeri", Ok(()), []);

    let display_name = LanguageIdentifierDisplayName::try_new_short_menu_heavy(
        prefs_en,
        langid!("zh-Hans"),
        menu_options,
    )
    .unwrap();
    assert_try_writeable_parts_eq!(
        display_name.as_borrowed(),
        "Chinese, Mandarin (Simplified)",
        Ok(()),
        []
    );
}

#[test]
fn test_language_identifier_display_name_tiers_fr_tiny() {
    let prefs_fr = DisplayNamesPreferences::from(locale!("fr"));
    let options = LanguageIdentifierDisplayNameOptions::default();

    let display_name =
        LanguageIdentifierDisplayName::try_new_tiny(prefs_fr, langid!("fr-CA"), options).unwrap();
    assert_try_writeable_parts_eq!(display_name.as_borrowed(), "français (Canada)", Ok(()), []);

    let display_name =
        LanguageIdentifierDisplayName::try_new_tiny(prefs_fr, langid!("zh-Hans"), options).unwrap();
    assert_try_writeable_parts_eq!(
        display_name.as_borrowed(),
        "zh (Hans)",
        Err(LanguageIdentifierNameFallbackError),
        [(0, 2, Part::ERROR), (4, 8, Part::ERROR)]
    );

    let display_name =
        LanguageIdentifierDisplayName::try_new_tiny(prefs_fr, langid!("hi-Deva-IN"), options)
            .unwrap();
    assert_try_writeable_parts_eq!(
        display_name.as_borrowed(),
        "hi (Deva, IN)",
        Err(LanguageIdentifierNameFallbackError),
        [
            (0, 2, Part::ERROR),
            (4, 8, Part::ERROR),
            (10, 12, Part::ERROR)
        ]
    );
}

#[test]
fn test_language_identifier_display_name_tiers_fr_light() {
    let prefs_fr = DisplayNamesPreferences::from(locale!("fr"));
    let options = LanguageIdentifierDisplayNameOptions::default();

    let display_name =
        LanguageIdentifierDisplayName::try_new_light(prefs_fr, langid!("fr-CA"), options).unwrap();
    assert_try_writeable_parts_eq!(display_name.as_borrowed(), "français canadien", Ok(()), []);

    let display_name =
        LanguageIdentifierDisplayName::try_new_light(prefs_fr, langid!("zh-Hans"), options)
            .unwrap();
    assert_try_writeable_parts_eq!(display_name.as_borrowed(), "chinois simplifié", Ok(()), []);

    let display_name =
        LanguageIdentifierDisplayName::try_new_light(prefs_fr, langid!("hi-Deva-IN"), options)
            .unwrap();
    assert_try_writeable_parts_eq!(
        display_name.as_borrowed(),
        "hindi (Deva, Inde)",
        Err(LanguageIdentifierNameFallbackError),
        [(7, 11, Part::ERROR)]
    );
}

#[test]
fn test_language_identifier_display_name_tiers_fr_heavy() {
    let prefs_fr = DisplayNamesPreferences::from(locale!("fr"));
    let options = LanguageIdentifierDisplayNameOptions::default();

    let display_name =
        LanguageIdentifierDisplayName::try_new_heavy(prefs_fr, langid!("fr-CA"), options).unwrap();
    assert_try_writeable_parts_eq!(display_name.as_borrowed(), "français canadien", Ok(()), []);

    let display_name =
        LanguageIdentifierDisplayName::try_new_heavy(prefs_fr, langid!("zh-Hans"), options)
            .unwrap();
    assert_try_writeable_parts_eq!(display_name.as_borrowed(), "chinois simplifié", Ok(()), []);

    let display_name =
        LanguageIdentifierDisplayName::try_new_heavy(prefs_fr, langid!("hi-Deva-IN"), options)
            .unwrap();
    assert_try_writeable_parts_eq!(
        display_name.as_borrowed(),
        "hindi (dévanagari, Inde)",
        Ok(()),
        []
    );
}
