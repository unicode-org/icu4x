// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod fixtures;
mod helpers;

use std::convert::TryInto;

use icu_locid::{LanguageIdentifier, Locale, ParserError};

type Result = std::result::Result<Locale, ParserError>;

fn test_langid_fixtures(tests: Vec<fixtures::LocaleTest>) {
    for test in tests {
        match test.output {
            fixtures::LocaleInfo::String(s) => {
                let input: Locale = test.input.try_into().expect("Parsing failed.");
                assert_eq!(input.to_string(), s);
            }
            fixtures::LocaleInfo::Error(err) => {
                let err: ParserError = err.into();
                let input: Result = test.input.try_into();
                assert_eq!(input, Err(err));
            }
            fixtures::LocaleInfo::Identifier(ident) => {
                let input: Locale = test.input.try_into().expect("Parsing failed.");
                let output: Locale = ident.clone().try_into().expect("Parsing failed.");
                assert_eq!(input, output);
                assert_eq!(input.to_string(), ident.identifier);
            }
            fixtures::LocaleInfo::Object(o) => {
                let input: Locale = test.input.try_into().expect("Parsing failed.");
                let output: Locale = o.try_into().expect("Parsing failed.");
                assert_eq!(input, output);
            }
        }
    }
}

#[test]
fn test_locale_parsing() {
    let path = "./tests/fixtures/locale.json";
    let data = helpers::read_fixture(path).expect("Failed to read a fixture");

    test_langid_fixtures(data);
}

#[test]
fn test_langid_invalid() {
    let path = "./tests/fixtures/invalid-extensions.json";
    let data = helpers::read_fixture(path).expect("Failed to read a fixture");

    test_langid_fixtures(data);
}

#[test]
fn test_locale_is_empty() {
    let locale: Locale = Locale::default();
    assert!(locale.extensions.is_empty());
    assert_eq!(locale.to_string(), "und".to_string());
}

#[test]
fn test_locale_conversions() {
    let locale: Locale = Locale::default();
    let langid: LanguageIdentifier = locale.clone().into();
    let locale2: Locale = langid.into();
    assert_eq!(locale, locale2);
    assert_eq!(locale, "und");
}

#[test]
fn test_locale_canonicalize() {
    let locale: Locale = "En-latn-US-MacOS"
        .parse()
        .expect("Failed to parse a locale.");
    assert_eq!(
        locale.to_string(),
        Locale::canonicalize("eN-latN-uS-macOS").unwrap()
    );
}

#[test]
fn test_locale_partialeq_str() {
    let path = "./tests/fixtures/locale.json";
    let tests: Vec<fixtures::LocaleTest> =
        helpers::read_fixture(path).expect("Failed to read a fixture");
    for test in tests {
        let parsed: Locale = test.input.try_into().expect("Parsing failed.");
        assert_eq!(parsed, parsed.to_string().as_str());
    }

    // Check that trailing characters are not ignored
    let locale: Locale = "en".parse().expect("Parsing failed.");
    assert_ne!(locale, "en-US");
}

#[test]
fn test_locale_cmp_bytes() {
    let path = "./tests/fixtures/locale.json";
    let tests: Vec<fixtures::LocaleTest> =
        helpers::read_fixture(path).expect("Failed to read a fixture");
    let bcp47_strings = tests
        .iter()
        .map(|t| match t.input {
            fixtures::LocaleInfo::Identifier(ref s) => s.identifier.as_str(),
            _ => match t.output {
                fixtures::LocaleInfo::Identifier(ref s) => s.identifier.as_str(),
                _ => panic!("No string in fixture input or output: {:?}", t),
            },
        })
        .collect::<Vec<&str>>();
    for a in bcp47_strings.iter() {
        for b in bcp47_strings.iter() {
            use std::str::FromStr;
            let a_langid = Locale::from_str(a).expect("Invalid BCP-47 in fixture");
            let a_normalized = a_langid.to_string();
            let string_cmp = a_normalized.as_bytes().cmp(b.as_bytes());
            let test_cmp = a_langid.cmp_bytes(b.as_bytes());
            assert_eq!(string_cmp, test_cmp, "{:?}/{:?}", a, b);
        }
    }
}
