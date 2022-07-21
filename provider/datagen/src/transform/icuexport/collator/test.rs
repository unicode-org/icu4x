// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::cmp::Ordering;

use icu_collator::{Collator, CollatorOptions, Strength};
use crate::CldrLocaleSubset;
use crate::SourceData;
use icu_locid::{langid, Locale};
use icu_provider::AsDowncastingAnyProvider;
use icu_provider::AsDynamicDataProviderAnyMarkerWrap;
use icu_provider::{AnyMarker, DynamicDataProvider};
use lazy_static::lazy_static;

fn get_provider() -> impl DynamicDataProvider<AnyMarker> {
    lazy_static! {
        static ref SOURCE_DATA: SourceData = SourceData::default()
            .with_icuexport(icu_testdata::paths::icuexport_toml_root())
            .unwrap()
            // CLDR data is needed for locale fallback support
            .with_cldr(icu_testdata::paths::cldr_json_root(), CldrLocaleSubset::Full)
            .unwrap();
    }
    crate::create_datagen_provider!(*SOURCE_DATA)
}

#[derive(Debug)]
struct TestCase<'a> {
    left: &'a str,
    right: &'a str,
    expectation: Ordering,
}

fn check_expectations(collator: &Collator, cases: &[TestCase<'_>]) {
    for cas in cases
    {
        let TestCase {
            left,
            right,
            expectation,
        } = cas;
        assert_eq!(collator.compare(left, right), *expectation, "{:?}", cas);
    }
}

// TODO(#2226): Re-enable this test.
#[ignore]
#[test]
fn test_fi() {
    // Adapted from ficoll.cpp in ICU4C
    // Testing that w and v behave as in the root collation is for checking
    // that the sorting collation doesn't exhibit the behavior of the search
    // collation, which (somewhat questionably) treats w and v as primary-equal.
    let cases = [
        TestCase {
            left: "wat",
            right: "vat",
            expectation: Ordering::Greater,
        },
        TestCase {
            left: "vat",
            right: "way",
            expectation: Ordering::Less,
        },
        TestCase {
            left: "aübeck",
            right: "axbeck",
            expectation: Ordering::Greater,
        },
        TestCase {
            left: "Låvi",
            right: "Läwe",
            expectation: Ordering::Less,
        },
        // ICU4C has a duplicate of the case below.
        // The duplicate is omitted here.
        // Instead, the subsequent tests are added for ICU4X.
        TestCase {
            left: "ä",
            right: "o",
            expectation: Ordering::Greater,
        },
        TestCase {
            left: "a\u{0308}",
            right: "ä",
            expectation: Ordering::Equal,
        },
    ];
    let locale: Locale = langid!("fi").into();
    let data_provider = icu_testdata::get_provider();

    let mut options = CollatorOptions::new();
    options.set_strength(Some(Strength::Tertiary));

    {
        let collator: Collator =
            Collator::try_new(locale.clone(), &data_provider, options).unwrap();
        check_expectations(&collator, &cases);
    }

    options.set_strength(Some(Strength::Primary));

    {
        let collator: Collator = Collator::try_new(locale, &data_provider, options).unwrap();
        check_expectations(&collator, &cases);
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
    let cases = [
        TestCase {
            left: "wat",
            right: "vat",
            expectation: Ordering::Greater,
        },
        TestCase {
            left: "vat",
            right: "way",
            expectation: Ordering::Less,
        },
        TestCase {
            left: "aübeck",
            right: "axbeck",
            expectation: Ordering::Greater,
        },
        TestCase {
            left: "Låvi",
            right: "Läwe",
            expectation: Ordering::Less,
        },
        // ICU4C has a duplicate of the case below.
        // The duplicate is omitted here.
        // Instead, the subsequent tests are added for ICU4X.
        TestCase {
            left: "ä",
            right: "o",
            expectation: Ordering::Greater,
        },
        TestCase {
            left: "a\u{0308}",
            right: "ä",
            expectation: Ordering::Equal,
        },
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
        check_expectations(&collator, &cases);
    }

    options.set_strength(Some(Strength::Primary));

    {
        let collator: Collator = Collator::try_new(locale, &provider_no_fallback, options).unwrap();
        check_expectations(&collator, &cases);
    }
}
