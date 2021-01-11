// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
mod fixtures;
mod helpers;

use icu_locale_canonicalizer::LocaleCanonicalizer;
use icu_locid::Locale;
use icu_locid_macros::langid;

#[test]
fn test_maximize() {
    let provider = icu_testdata::get_provider();
    let lc = LocaleCanonicalizer::new(&provider).unwrap();

    let path = "./tests/fixtures/maximize.json";
    let testcases: Vec<fixtures::LikelySubtagsTest> =
        helpers::read_fixture(path).expect("Failed to read a fixture");

    for case in testcases {
        let mut locale: Locale = case.input.into();
        let unmodified = locale.clone();
        let was_modified = lc.maximize(&mut locale).unwrap();
        assert_eq!(locale.to_string(), case.output);
        if was_modified {
            assert_ne!(locale, unmodified);
        } else {
            assert_eq!(locale, unmodified);
        }
    }

    let mut locale = langid!("zz").into();
    assert!(lc.maximize(&mut locale).is_err());
}

#[test]
fn test_minimize() {
    let provider = icu_testdata::get_provider();
    let lc = LocaleCanonicalizer::new(&provider).unwrap();

    let path = "./tests/fixtures/minimize.json";
    let testcases: Vec<fixtures::LikelySubtagsTest> =
        helpers::read_fixture(path).expect("Failed to read a fixture");

    for case in testcases {
        let mut locale: Locale = case.input.into();
        let unmodified = locale.clone();
        let was_modified = lc.minimize(&mut locale).unwrap();
        assert_eq!(locale.to_string(), case.output);
        if was_modified {
            assert_ne!(locale, unmodified);
        } else {
            assert_eq!(locale, unmodified);
        }
    }
}
