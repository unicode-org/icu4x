// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locid_macros::langid;
use icu_plurals::{
    provider::{self, PluralRulesV1Marker},
    rules::runtime::ast::Rule,
    PluralCategory, PluralRuleType, PluralRules,
};
use icu_provider::prelude::*;
use zerovec::VarZeroVec;

#[test]
fn test_plural_rules() {
    let provider = icu_testdata::get_provider();

    let lid = langid!("en");

    let pr = PluralRules::try_new(lid, &provider, PluralRuleType::Cardinal).unwrap();

    assert_eq!(pr.select(5_usize), PluralCategory::Other);
}

#[test]
fn test_static_provider_borrowed_rules() {
    let provider = icu_testdata::get_static_provider();

    let lid = langid!("en");

    let rules: DataPayload<PluralRulesV1Marker> = provider
        .load_payload(&DataRequest {
            resource_path: ResourcePath {
                key: provider::key::CARDINAL_V1,
                options: ResourceOptions {
                    variant: None,
                    langid: Some(lid),
                },
            },
        })
        .expect("Failed to load payload")
        .take_payload()
        .expect("Failed to retrieve payload");
    let rules = rules.get();
    assert!(matches!(rules.one, Some(Rule(VarZeroVec::Borrowed(_)))));
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
    let categories: Vec<PluralCategory> = PluralCategory::all().collect();

    assert_eq!(categories.len(), 6);

    assert_eq!(categories[0], PluralCategory::Few);
    assert_eq!(categories[1], PluralCategory::Many);
    assert_eq!(categories[2], PluralCategory::One);
    assert_eq!(categories[3], PluralCategory::Other);
    assert_eq!(categories[4], PluralCategory::Two);
    assert_eq!(categories[5], PluralCategory::Zero);
}
