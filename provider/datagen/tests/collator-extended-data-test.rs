// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::cmp::Ordering;

use icu_collator::{Collator, CollatorOptions, Strength};
use icu_datagen::CldrLocaleSubset;
use icu_datagen::SourceData;
use icu_locid::{langid, Locale};
use icu_provider::AsDowncastingAnyProvider;
use icu_provider::AsDynamicDataProviderAnyMarkerWrap;
use icu_provider::{AnyMarker, DynamicDataProvider};
use lazy_static::lazy_static;

lazy_static! {
    static ref SOURCE_DATA: SourceData = SourceData::default()
        .with_icuexport(icu_testdata::paths::icuexport_toml_root())
        .unwrap()
        // CLDR data is needed for locale fallback support
        .with_cldr(icu_testdata::paths::cldr_json_root(), CldrLocaleSubset::Full)
        .unwrap();
}

fn get_provider() -> impl DynamicDataProvider<AnyMarker> {
    icu_datagen::create_datagen_provider!(*SOURCE_DATA)
}

fn check_expectations(
    collator: &Collator,
    left: &[&str],
    right: &[&str],
    expectations: &[Ordering],
) {
    let mut left_iter = left.iter();
    let mut right_iter = right.iter();
    let mut expect_iter = expectations.iter();
    while let (Some(left_str), Some(right_str), Some(expectation)) =
        (left_iter.next(), right_iter.next(), expect_iter.next())
    {
        assert_eq!(collator.compare(left_str, right_str), *expectation);
    }
}

#[test]
fn test_sv() {
    // This is the same as test_fi. The purpose of this copy is to test that
    // Swedish defaults to "reformed", which behaves like Finnish "standard",
    // and not to "standard", which behaves like Finnish "traditional".

    // Adapted from ficoll.cpp in ICU4C
    // Testing that w and v behave as in the root collation is for checking
    // that the sorting collation doesn't exhibit the behavior of the search
    // collation, which (somewhat questionably) treats w and v as primary-equal.
    let left = [
        "wat",
        "vat",
        "aübeck",
        "Låvi",
        // ICU4C has a duplicate of the first case below.
        // The duplicate is omitted here.
        // Instead, the subsequent tests are added for ICU4X.
        "ä",
        "a\u{0308}",
    ];
    let right = [
        "vat", "way", "axbeck", "Läwe",
        // ICU4C has a duplicate of the first case below.
        // The duplicate is omitted here.
        // Instead, the subsequent tests are added for ICU4X.
        "o", "ä",
    ];
    let expectations = [
        Ordering::Greater,
        Ordering::Less,
        Ordering::Greater,
        Ordering::Less,
        Ordering::Greater,
        Ordering::Equal,
    ];
    let locale: Locale = langid!("sv").into();

    let any_dyn_provider = get_provider();
    let any_provider = any_dyn_provider.as_any_provider();
    let provider_no_fallback = any_provider.as_downcasting();

    let mut options = CollatorOptions::new();
    options.set_strength(Some(Strength::Tertiary));

    {
        let collator: Collator =
            Collator::try_new(locale.clone(), &provider_no_fallback, options).unwrap();
        check_expectations(&collator, &left, &right, &expectations);
    }

    options.set_strength(Some(Strength::Primary));

    {
        let collator: Collator = Collator::try_new(locale, &provider_no_fallback, options).unwrap();
        check_expectations(&collator, &left, &right, &expectations);
    }
}
