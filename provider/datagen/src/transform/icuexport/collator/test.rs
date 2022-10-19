// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::cmp::Ordering;

use crate::testutil::ResolvedLocaleAdapter;
use crate::*;
use icu_collator::{provider::CollationDataV1Marker, Collator, CollatorOptions, Strength};
use icu_locid::{langid, locale};
use icu_provider_adapters::fallback::LocaleFallbackProvider;

#[derive(Debug)]
struct TestCase<'a> {
    left: &'a str,
    right: &'a str,
    expectation: Ordering,
}

fn check_expectations(locale: &DataLocale, options: CollatorOptions, cases: &[TestCase<'_>]) {
    let collator =
        Collator::try_new_unstable(&DatagenProvider::for_test(), locale, options).unwrap();
    for cas in cases {
        let TestCase {
            left,
            right,
            expectation,
        } = cas;
        assert_eq!(collator.compare(left, right), *expectation, "{:?}", cas);
    }
}

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
    let locale: DataLocale = langid!("fi").into();
    let mut options = CollatorOptions::new();

    options.strength = Some(Strength::Tertiary);
    check_expectations(&locale, options, &cases);

    options.strength = Some(Strength::Primary);
    check_expectations(&locale, options, &cases);
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
    let locale: DataLocale = langid!("sv").into();
    let mut options = CollatorOptions::new();

    options.strength = Some(Strength::Tertiary);
    check_expectations(&locale, options, &cases);

    options.strength = Some(Strength::Primary);
    check_expectations(&locale, options, &cases);
}

#[test]
fn test_nb_nn_no() {
    let provider = crate::DatagenProvider::for_test();

    let input = vec!["ü", "y", "å", "ø"];
    let expected = &["y", "ü", "ø", "å"];

    // Test "no" macro language without fallback (should equal expected)
    let collator =
        Collator::try_new_unstable(&provider, &locale!("no").into(), CollatorOptions::new())
            .unwrap();
    let mut strs = input.clone();
    strs.sort_by(|a, b| collator.compare(a, b));
    assert_eq!(strs, expected);

    // Test "und" without fallback (should NOT equal expected)
    let collator =
        Collator::try_new_unstable(&provider, &locale!("und").into(), CollatorOptions::new())
            .unwrap();
    let mut strs = input.clone();
    strs.sort_by(|a, b| collator.compare(a, b));
    assert_ne!(strs, expected);

    // Test "nb" without fallback (should fail to load)
    if Collator::try_new_unstable(&provider, &locale!("nb").into(), CollatorOptions::new()).is_ok()
    {
        panic!("Should fail to create 'nb' without fallback enabled")
    }

    // Enable locale fallback on the provider now
    let provider = LocaleFallbackProvider::try_new_unstable(provider).unwrap();
    let provider = ResolvedLocaleAdapter::new(provider);

    // Test "no" macro language WITH fallback (should equal expected)
    let input_locale = locale!("no").into();
    let collator =
        Collator::try_new_unstable(&provider, &input_locale, CollatorOptions::new()).unwrap();
    let mut strs = input.clone();
    strs.sort_by(|a, b| collator.compare(a, b));
    assert_eq!(strs, expected);
    assert_eq!(
        provider.resolved_locale_for(CollationDataV1Marker::KEY, input_locale),
        Some(&locale!("no").into())
    );

    // Now "nb" should work
    let input_locale = locale!("nb").into();
    let collator =
        Collator::try_new_unstable(&provider, &input_locale, CollatorOptions::new()).unwrap();
    let mut strs = input.clone();
    strs.sort_by(|a, b| collator.compare(a, b));
    assert_eq!(strs, expected);
    assert_eq!(
        provider.resolved_locale_for(CollationDataV1Marker::KEY, input_locale),
        Some(&locale!("no").into())
    );

    // And "nn" should work, too
    let input_locale = locale!("nn").into();
    let collator =
        Collator::try_new_unstable(&provider, &input_locale, CollatorOptions::new()).unwrap();
    let mut strs = input.clone();
    strs.sort_by(|a, b| collator.compare(a, b));
    assert_eq!(strs, expected);
    assert_eq!(
        provider.resolved_locale_for(CollationDataV1Marker::KEY, input_locale),
        Some(&locale!("no").into())
    );
}
