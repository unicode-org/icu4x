// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod fixtures;
mod helpers;

use icu_locale_canonicalizer::{CanonicalizationResult, LocaleCanonicalizer};
use icu_locid::Locale;

#[test]
fn test_maximize() {
    let provider = icu_testdata::get_provider();
    let lc = LocaleCanonicalizer::new(&provider).unwrap();

    let path = "./tests/fixtures/maximize.json";
    let testcases: Vec<fixtures::CanonicalizationTest> =
        helpers::read_fixture(path).expect("Failed to read a fixture");

    for case in testcases {
        if let Some(true) = case.disabled {
            continue;
        }
        let mut locale: Locale = case.input.parse().unwrap();
        let unmodified = locale.clone();
        let result = lc.maximize(&mut locale);
        assert_eq!(locale.to_string(), case.output);
        if result == CanonicalizationResult::Modified {
            assert_ne!(locale, unmodified);
        } else {
            assert_eq!(locale, unmodified);
        }
    }
}

#[test]
fn test_minimize() {
    let provider = icu_testdata::get_provider();
    let lc = LocaleCanonicalizer::new(&provider).unwrap();

    let path = "./tests/fixtures/minimize.json";
    let testcases: Vec<fixtures::CanonicalizationTest> =
        helpers::read_fixture(path).expect("Failed to read a fixture");

    for case in testcases {
        if let Some(true) = case.disabled {
            continue;
        }
        let mut locale: Locale = case.input.parse().unwrap();
        let unmodified = locale.clone();
        let result = lc.minimize(&mut locale);
        assert_eq!(locale.to_string(), case.output);
        if result == CanonicalizationResult::Modified {
            assert_ne!(locale, unmodified);
        } else {
            assert_eq!(locale, unmodified);
        }
    }
}

#[test]
fn test_canonicalize() {
    let provider = icu_testdata::get_provider();
    let lc = LocaleCanonicalizer::new(&provider).unwrap();

    let path = "./tests/fixtures/canonicalize.json";
    let testcases: Vec<fixtures::CanonicalizationTest> =
        helpers::read_fixture(path).expect("Failed to read a fixture");

    for case in testcases {
        if let Some(true) = case.disabled {
            continue;
        }
        let mut locale: Locale = case.input.parse().expect("Unable to parse input");
        let unmodified = locale.clone();
        let result = lc.canonicalize(&mut locale);
        assert_eq!(locale.to_string(), case.output);
        if result == CanonicalizationResult::Modified {
            assert_ne!(locale, unmodified);
        } else {
            assert_eq!(locale, unmodified);
        }
    }
}
