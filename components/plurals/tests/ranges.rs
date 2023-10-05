// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locid::locale;
use icu_plurals::{PluralCategory, PluralRanges};

#[test]
fn test_plural_ranges() {
    assert_eq!(
        PluralRanges::try_new(&locale!("he").into())
            .unwrap()
            .category_for_range(PluralCategory::One, PluralCategory::Two),
        PluralCategory::Other
    );
}

#[test]
fn test_plural_ranges_optimized_data() {
    assert_eq!(
        PluralRanges::try_new(&locale!("en").into())
            .unwrap()
            .category_for_range(PluralCategory::One, PluralCategory::Other),
        PluralCategory::Other
    );
}

#[test]
fn test_plural_ranges_missing_data_fallback() {
    assert_eq!(
        PluralRanges::try_new(&locale!("nl").into())
            .unwrap()
            .category_for_range(PluralCategory::Two, PluralCategory::Many),
        PluralCategory::Many
    );
}
