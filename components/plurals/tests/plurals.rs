// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu::locid::locale;
use icu::plurals::{PluralCategory, PluralRuleType, PluralRules};

#[test]
fn test_plural_rules() {
    assert_eq!(
        PluralRules::try_new(&locale!("en").into(), PluralRuleType::Cardinal)
            .unwrap()
            .category_for(5_usize),
        PluralCategory::Other
    );
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
