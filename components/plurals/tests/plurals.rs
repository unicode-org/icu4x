// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use icu_locale_macros::langid;
use icu_plurals::{PluralCategory, PluralRuleType, PluralRules};

#[test]
fn test_plural_rules() {
    let provider = icu_testdata::get_provider();

    let lid = langid!("en");

    let pr = PluralRules::try_new(lid, &provider, PluralRuleType::Cardinal).unwrap();

    assert_eq!(pr.select(5_usize), PluralCategory::Other);
}

#[test]
fn test_plural_rules_missing() {
    let provider = icu_testdata::get_provider();

    let lid = langid!("xx");

    let pr = PluralRules::try_new(lid, &provider, PluralRuleType::Cardinal);

    assert!(pr.is_err());
}

#[test]
fn test_plural_category_all() {
    let categories: Vec<&PluralCategory> = PluralCategory::all().collect();

    assert_eq!(categories.get(0), Some(&&PluralCategory::Zero));
}
