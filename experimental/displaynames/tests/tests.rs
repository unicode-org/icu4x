// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[allow(unused_imports)]
use icu_displaynames::{DisplayNamesOptions, Fallback, LanguageDisplay, Style, Capitalization, LocaleDisplayNamesFormatter};
use writeable::assert_writeable_eq;
use icu_locid::locale;

macro_rules! generate_locale_displaynames_test {
    ($test_name: ident, $constructor: ident, $options: expr,
     [$(($locale:literal, $locale_to_test:literal, $expected:literal)),+ $(,)?]) => {
        #[test]
        fn $test_name(){
            let display_name_obj = LocaleDisplayNamesFormatter::$constructor(
                &icu_testdata::unstable(),
                &locale!($locale).into(),
                $options
            )
            .expect("Data should load successfully.");

            $(
                assert_writeable_eq!(
                    display_name_obj.of(locale!($locale_to_test)).unwrap_or_default(),
                    $expected
                );
            )+
        }
    };
}
generate_locale_displaynames_test!(
    test_locale_dn_with_middle_capitalization,
    try_new_unstable,
    DisplayNamesOptions {
        capitalization: Capitalization::MiddleOfSentence
    },
    [
        ("da", "en", "engelsk"),
        ("da", "en@calendar=buddhist", "engelsk (buddhistisk kalender)"),
        ("da", "en_GB", "engelsk (Storbritannien)"),
        ("es", "en", "ingles"), // With acute on the e
        ("es", "en_GB", "ingles (Reino Unido)"),
        ("es", "en", "engelsk"),
    ]
);

generate_locale_displaynames_test!(
    test_locale_dn_with_short_style,
    try_new_unstable,
    DisplayNamesOptions {
        capitalization: Capitalization::MiddleOfSentence,
        style: Style::Short
    },
    [
        ("da", "en_GB", "engelsk (Storbritannien)"),
        ("es", "en_GB", "ingles (RU)"),
    ]
);

generate_locale_displaynames_test!(
    test_locale_dn_with_dialect,
    try_new_unstable,
    DisplayNamesOptions {
        capitalization: Capitalization::MiddleOfSentence,
        language_display: LanguageDisplay::Dialect
    },
    [
        ("da", "en_GB", "britisk engelsk"),
        ("es", "en_GB", "ingles britanico"), // with acute on the e, a
    ]
);

generate_locale_displaynames_test!(
    test_locale_dn_with_capitalization_at_beginning,
    try_new_unstable,
    DisplayNamesOptions {
        capitalization: Capitalization::BeginningOfSentence,
    },
    [
        ("da", "en", "Engelsk"),
        ("da", "en@calendar=buddhist", "Engelsk (buddhistisk kalender)"),
        ("da", "en_GB", "Engelsk (Storbritannien)"),
        ("es", "en", "Ingles"), // With acute on the e
        ("es", "en_GB", "Ingles (Reino Unido)"),
        ("es", "en", "engelsk"),
    ]
);

generate_locale_displaynames_test!(
    test_locale_dn_with_capitalization_at_beginning_and_short_style,
    try_new_unstable,
    DisplayNamesOptions {
        capitalization: Capitalization::BeginningOfSentence,
        style: Style::Short
    },
    [
        ("da", "en_GB", "Engelsk (Storbritannien)"),
        ("es", "en_GB", "Ingles (RU)"),
    ]
);

generate_locale_displaynames_test!(
    test_locale_dn_with_capitalization_at_beginning_and_dialect,
    try_new_unstable,
    DisplayNamesOptions {
        capitalization: Capitalization::BeginningOfSentence,
        language_display: LanguageDisplay::Dialect
    },
    [
        ("da", "en_GB", "Britisk engelsk"),
        ("es", "en_GB", "Ingles britanico"), // with acute on the e, a
    ]
);

generate_locale_displaynames_test!(
    test_locale_dn_with_capitalization_for_ui_list_menu,
    try_new_unstable,
    DisplayNamesOptions {
        capitalization: Capitalization::UiListOrMenu,
    },
    [
        ("da", "en", "Engelsk"),
        ("da", "en@calendar=buddhist", "Engelsk (buddhistisk kalender)"),
        ("da", "en_GB", "Engelsk (Storbritannien)"),
        ("es", "en", "Ingles"), // With acute on the e
        ("es", "en_GB", "Ingles (Reino Unido)"),
        ("es", "en", "engelsk"),
    ]
);

generate_locale_displaynames_test!(
    test_locale_dn_with_capitalization_for_ui_list_menu_and_short_style,
    try_new_unstable,
    DisplayNamesOptions {
        capitalization: Capitalization::UiListOrMenu,
        style: Style::Short
    },
    [
        ("da", "en_GB", "Engelsk (Storbritannien)"),
        ("es", "en_GB", "Ingles (RU)"),
    ]
);

generate_locale_displaynames_test!(
    test_locale_dn_with_capitalization_for_ui_list_menu_and_dialect,
    try_new_unstable,
    DisplayNamesOptions {
        capitalization: Capitalization::UiListOrMenu,
        language_display: LanguageDisplay::Dialect
    },
    [
        ("da", "en_GB", "Britisk engelsk"),
        ("es", "en_GB", "Ingles britanico"), // with acute on the e, a
    ]
);

generate_locale_displaynames_test!(
    test_locale_dn_with_capitalization_for_standalone,
    try_new_unstable,
    DisplayNamesOptions {
        capitalization: Capitalization::Standalone,
    },
    [
        ("da", "en", "engelsk"),
        ("da", "en@calendar=buddhist", "engelsk (buddhistisk kalender)"),
        ("da", "en_GB", "engelsk (Storbritannien)"),
        ("es", "en", "Ingles"), // With acute on the e
        ("es", "en_GB", "Ingles (Reino Unido)"),
        ("es", "en", "engelsk"),
    ]
);

generate_locale_displaynames_test!(
    test_locale_dn_with_capitalization_for_standalone_and_short_style,
    try_new_unstable,
    DisplayNamesOptions {
        capitalization: Capitalization::UiListOrMenu,
        style: Style::Short
    },
    [
        ("da", "en_GB", "engelsk (Storbritannien)"),
        ("es", "en_GB", "Ingles (RU)"),
    ]
);

generate_locale_displaynames_test!(
    test_locale_dn_with_capitalization_for_standalone_and_dialect,
    try_new_unstable,
    DisplayNamesOptions {
        capitalization: Capitalization::UiListOrMenu,
        language_display: LanguageDisplay::Dialect
    },
    [
        ("da", "en_GB", "britisk engelsk"),
        ("es", "en_GB", "Ingles britanico"), // with acute on the e, a
    ]
);