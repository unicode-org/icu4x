// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use icu_locale::subtags;
use icu_locale::LanguageIdentifier;
use icu_locale_macros::*;

const LANG_PL: subtags::Language = language!("pL");
const SCRIPT_LATN: subtags::Script = script!("lAtN");
const REGION_US: subtags::Region = region!("us");
const VARIANT_MACOS: subtags::Variant = variant!("MACOS");
const LANGID: LanguageIdentifier = langid!("de-Arab-AT");

#[test]
fn language() {
    let lang = language!("Pl");

    assert_eq!(lang, "pl");
    assert_eq!(LANG_PL, "pl");
    assert_eq!(lang, LANG_PL);
}

#[test]
fn script() {
    let script = script!("latn");

    assert_eq!(script, "Latn");
    assert_eq!(SCRIPT_LATN, "Latn");
    assert_eq!(script, SCRIPT_LATN);
}

#[test]
fn region() {
    let region = region!("us");

    assert_eq!(region, "US");
    assert_eq!(REGION_US, "US");
    assert_eq!(region, REGION_US);
}

#[test]
fn variant() {
    let variant = variant!("macOS");

    assert_eq!(variant, "macos");
    assert_eq!(VARIANT_MACOS, "macos");
    assert_eq!(variant, VARIANT_MACOS);
}

#[test]
fn langid() {
    let langid = langid!("de_Arab_aT");

    assert_eq!(langid.to_string(), "de-Arab-AT");
    assert_eq!(LANGID.to_string(), "de-Arab-AT");
    assert_eq!(langid, LANGID);

    let langid_with_variant = langid!("de_Arab_aT-macOS");
    assert_eq!(langid_with_variant.to_string(), "de-Arab-AT-macos");
}
