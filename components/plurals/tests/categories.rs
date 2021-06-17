// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod fixtures;
mod helpers;

use icu_locid::LanguageIdentifier;
use icu_plurals::{PluralCategory, PluralRules};
use std::str::FromStr;

#[test]
fn test_categories() {
    let provider = icu_testdata::get_provider();

    let path = "./tests/fixtures/categories.json";
    let test_set: Vec<fixtures::CategoriesTest> =
        helpers::read_fixture(path).expect("Failed to read a fixture");

    for test in test_set {
        let pr = PluralRules::try_new(
            LanguageIdentifier::from_str(&test.langid).unwrap(),
            &provider,
            test.plural_type.into(),
        )
        .unwrap();

        assert_eq!(
            test.categories.len(),
            pr.categories().count(),
            "\n\
	    {}({:?})\n\
	    Expected: {:#?}\n\
	    Actual: {:#?}\n\
	    ",
            test.langid,
            test.plural_type,
            test.categories,
            pr.categories().collect::<Vec<&PluralCategory>>(),
        );

        for (expected, actual) in test.categories.iter().zip(pr.categories()) {
            assert_eq!(
                expected,
                actual,
                "\n\
		{}({:?})\n\
		Expected: {:#?}\n\
		Actual: {:#?}\n\
		",
                test.langid,
                test.plural_type,
                test.categories,
                pr.categories().collect::<Vec<&PluralCategory>>(),
            );
        }
    }
}
