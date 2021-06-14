// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locid_macros::langid;
use icu_plurals::provider::{self, PluralRuleStringsV1};
use icu_plurals::{PluralCategory, PluralRuleType, PluralRules};
use icu_provider::struct_provider::StructProvider;
use std::borrow::Cow;

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

    assert_eq!(categories.len(), 6);

    assert_eq!(categories[0], &PluralCategory::Few);
    assert_eq!(categories[1], &PluralCategory::Many);
    assert_eq!(categories[2], &PluralCategory::One);
    assert_eq!(categories[3], &PluralCategory::Other);
    assert_eq!(categories[4], &PluralCategory::Two);
    assert_eq!(categories[5], &PluralCategory::Zero);
}

#[test]
fn test_plural_rules_non_static_lifetime() {
    let local_string = "v = 0 and i % 10 = 1".to_string();
    let local_data = PluralRuleStringsV1 {
        zero: None,
        one: Some(Cow::Borrowed(&local_string)),
        two: None,
        few: None,
        many: None,
    };
    let provider = StructProvider {
        key: provider::key::CARDINAL_V1,
        data: &local_data,
    };

    let lid = langid!("und");
    let pr = PluralRules::try_new(lid, &provider, PluralRuleType::Cardinal).unwrap();

    assert_eq!(pr.select(1_usize), PluralCategory::One);
    assert_eq!(pr.select(5_usize), PluralCategory::Other);
    assert_eq!(pr.select(11_usize), PluralCategory::One);
}
