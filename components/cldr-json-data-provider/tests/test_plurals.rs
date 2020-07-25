use std::borrow::Borrow;
use std::borrow::Cow;
use std::convert::TryFrom;
use std::fs;

use icu_cldr_json_data_provider::CldrPluralsDataProvider;
use icu_data_provider::prelude::*;
use icu_data_provider::structs::plurals::*;

#[test]
fn test_basic() {
    let json_str = fs::read_to_string("tests/testdata/plurals.json").unwrap();
    let provider = CldrPluralsDataProvider::try_from(json_str.as_str()).unwrap();

    // Spot-check locale 'cs' since it has some interesting entries
    let cs_rules: Cow<PluralRuleStringsV1> = provider
        .load(&data_provider::Request {
            data_key: DataKey {
                category: data_key::Category::Plurals,
                sub_category: "cardinal".parse().unwrap(),
                version: 1,
            },
            data_entry: DataEntry {
                variant: None,
                langid: "cs".parse().unwrap(),
            },
        })
        .unwrap()
        .take_payload()
        .unwrap();

    assert_eq!(None, cs_rules.zero);
    assert_eq!(
        Some("i = 1 and v = 0 @integer 1"),
        cs_rules.one.as_ref().map(|v| v.borrow())
    );
    assert_eq!(None, cs_rules.two);
    assert_eq!(
        Some("i = 2..4 and v = 0 @integer 2~4"),
        cs_rules.few.as_ref().map(|v| v.borrow())
    );
    assert_eq!(
        Some("v != 0   @decimal 0.0~1.5, 10.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0, …"),
        cs_rules.many.as_ref().map(|v| v.borrow())
    );
}
