mod fixtures;
mod helpers;

use std::convert::TryInto;

use icu_locale::{subtags, LanguageIdentifier, ParserError};

type Result = std::result::Result<LanguageIdentifier, ParserError>;

fn test_langid_fixtures(tests: Vec<fixtures::LocaleTest>) {
    for test in tests {
        match test.output {
            fixtures::LocaleInfo::String(s) => {
                if let fixtures::LocaleInfo::Object(ref o) = &test.input {
                    if o.field_type == "Locale" {
                        continue;
                    }
                }
                let input: LanguageIdentifier = test.input.try_into().expect("Parsing failed.");
                assert_eq!(input.to_string(), s);
            }
            fixtures::LocaleInfo::Error(err) => {
                let err: ParserError = err.into();
                let input: Result = test.input.try_into();
                assert_eq!(input, Err(err));
            }
            fixtures::LocaleInfo::Identifier(ident) => {
                let input: LanguageIdentifier = test.input.try_into().expect("Parsing failed.");
                let output: LanguageIdentifier = ident.try_into().expect("Parsing failed.");
                assert_eq!(input, output);
            }
            fixtures::LocaleInfo::Object(o) => {
                let input: LanguageIdentifier = test.input.try_into().expect("Parsing failed.");
                let output: LanguageIdentifier = o.try_into().expect("Parsing failed.");
                assert_eq!(input, output);
            }
        }
    }
}

#[test]
fn test_langid_parsing() {
    let path = "./tests/fixtures/langid.json";
    let data = helpers::read_fixture(path).expect("Failed to read a fixture");

    test_langid_fixtures(data);
}

#[test]
fn test_langid_invalid() {
    let path = "./tests/fixtures/invalid.json";
    let data = helpers::read_fixture(path).expect("Failed to read a fixture");

    test_langid_fixtures(data);
}

#[test]
fn test_langid_canonicalize() {
    let path = "./tests/fixtures/canonicalize.json";
    let data = helpers::read_fixture(path).expect("Failed to read a fixture");

    test_langid_fixtures(data);
}

#[test]
fn test_langid_from_locale() {
    let path = "./tests/fixtures/locale.json";
    let data = helpers::read_fixture(path).expect("Failed to read a fixture");

    test_langid_fixtures(data);
}

#[test]
fn test_langid_subtag_language() {
    let mut lang: subtags::Language = "en".parse().expect("Failed to parse a language.");
    let s: &str = (&lang).into();
    assert_eq!(s, "en");
    lang.clear();
    assert_eq!(lang, "und");
    assert_eq!(lang.to_string(), "und".to_string());
    assert!(lang.is_empty());
}

#[test]
fn test_langid_subtag_region() {
    let region: subtags::Region = "en".parse().expect("Failed to parse a region.");
    let s: &str = (&region).into();
    assert_eq!(s, "EN");
    assert_eq!(region, "EN");
}

#[test]
fn test_langid_subtag_script() {
    let script: subtags::Script = "Latn".parse().expect("Failed to parse a script.");
    let s: &str = (&script).into();
    assert_eq!(s, "Latn");
    assert_eq!(script, "Latn");
}

#[test]
fn test_langid_subtag_variant() {
    let variant: subtags::Variant = "macos".parse().expect("Failed to parse a variant.");
    let s: &str = (&variant).into();
    assert_eq!(s, "macos");
    assert_eq!(variant, "macos");
}

#[test]
fn test_langid_subtag_variants() {
    let variant: subtags::Variant = "macos".parse().expect("Failed to parse a variant.");
    let mut variants = subtags::Variants::from_vec_unchecked(vec![variant]);
    assert_eq!(variants.get(0).unwrap(), "macos");
    variants.clear();
    assert_eq!(variants.len(), 0);
}
