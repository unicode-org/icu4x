// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locid::locale;
use icu_plurals::{provider::CardinalV1Marker, PluralCategory, PluralRuleType, PluralRules};
use icu_provider::prelude::*;

#[test]
fn test_plural_rules() {
    let provider = icu_testdata::get_provider();

    let pr =
        PluralRules::try_new(&provider, &locale!("en").into(), PluralRuleType::Cardinal).unwrap();

    assert_eq!(pr.select(5_usize), PluralCategory::Other);
}

#[test]
fn test_static_load_works() {
    let provider = icu_testdata::get_provider();

    let _rules: DataPayload<CardinalV1Marker> = provider
        .load(DataRequest {
            locale: &locale!("en").into(),
            metadata: Default::default(),
        })
        .expect("Failed to load payload")
        .take_payload()
        .expect("Failed to retrieve payload");
}

#[test]
fn test_plural_rules_missing() {
    // Use get_postcard_provider to skip vertical fallback
    let provider = icu_testdata::get_postcard_provider();

    let pr = PluralRules::try_new(&provider, &locale!("xx").into(), PluralRuleType::Cardinal);

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
