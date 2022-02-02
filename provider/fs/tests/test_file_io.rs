// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(unused_imports)]

use icu_locid::LanguageIdentifier;
use icu_locid_macros::langid;
use icu_plurals::{provider::*, rules::runtime::ast::Rule};
use icu_provider::iter::IterableProvider;
use icu_provider::prelude::*;
use icu_provider::serde::*;
use icu_provider_fs::FsDataProvider;
use std::borrow::Cow;

#[derive(Debug, PartialEq)]
struct PluralRulesTestData {
    zero: Option<&'static str>,
    one: Option<&'static str>,
    two: Option<&'static str>,
    few: Option<&'static str>,
    many: Option<&'static str>,
}

impl From<&PluralRulesTestData> for PluralRulesV1<'_> {
    fn from(i: &PluralRulesTestData) -> Self {
        fn parse(i: &'static str) -> Rule {
            i.parse().expect("Failed to parse rule")
        }
        Self {
            zero: i.zero.map(parse),
            one: i.one.map(parse),
            two: i.two.map(parse),
            few: i.few.map(parse),
            many: i.many.map(parse),
        }
    }
}

#[cfg(feature = "deserialize_json")]
const EXPECTED_RU_DATA: PluralRulesTestData = PluralRulesTestData {
    zero: None,
    one: Some("v = 0 and i % 10 = 1 and i % 100 != 11"),
    two: None,
    few: Some("v = 0 and i % 10 = 2..4 and i % 100 != 12..14"),
    many: Some("v = 0 and i % 10 = 0 or v = 0 and i % 10 = 5..9 or v = 0 and i % 100 = 11..14"),
};

#[cfg(feature = "deserialize_bincode_1")]
const EXPECTED_SR_DATA: PluralRulesTestData = PluralRulesTestData {
    zero: None,
    one: Some("v = 0 and i % 10 = 1 and i % 100 != 11 or f % 10 = 1 and f % 100 != 11"),
    two: None,
    few: Some(
        "v = 0 and i % 10 = 2..4 and i % 100 != 12..14 or f % 10 = 2..4 and f % 100 != 12..14",
    ),
    many: None,
};

#[allow(dead_code)]
fn get_request(langid: LanguageIdentifier) -> DataRequest {
    DataRequest {
        resource_path: ResourcePath {
            key: key::CARDINAL_V1,
            options: ResourceOptions {
                variant: None,
                langid: Some(langid),
            },
        },
    }
}

#[cfg(not(feature = "deserialize_json"))]
#[test]
fn test_json_feature() {
    FsDataProvider::try_new("./tests/testdata/json").expect_err("JSON is not enabled");
}

#[cfg(feature = "deserialize_json")]
#[test]
fn test_json() {
    let provider = FsDataProvider::try_new("./tests/testdata/json")
        .expect("Loading file from testdata directory");

    let plurals_data: DataPayload<PluralRulesV1Marker> = provider
        .load_payload(&get_request(langid!("ru")))
        .expect("The data should be valid")
        .take_payload()
        .expect("The data should be present");
    assert_eq!(plurals_data.get(), &PluralRulesV1::from(&EXPECTED_RU_DATA));
}

#[cfg(feature = "deserialize_json")]
#[test]
fn test_json_dyn_erased_serde() {
    let provider = FsDataProvider::try_new("./tests/testdata/json")
        .expect("Loading file from testdata directory");

    let plurals_data: DataPayload<PluralRulesV1Marker> = (&provider as &dyn BufferProvider)
        .as_deserializing()
        .load_payload(&get_request(langid!("ru")))
        .expect("The data should be valid")
        .take_payload()
        .expect("The data should be present");
    assert_eq!(plurals_data.get(), &PluralRulesV1::from(&EXPECTED_RU_DATA));
}

#[cfg(feature = "deserialize_json")]
#[test]
fn test_json_errors() {
    let provider = FsDataProvider::try_new("./tests/testdata/json")
        .expect("Loading file from testdata directory");

    type Provider = dyn DataProvider<PluralRulesV1Marker>;

    assert!(matches!(
        Provider::load_payload(
            &provider,
            &DataRequest {
                resource_path: ResourcePath {
                    key: key::CARDINAL_V1,
                    options: ResourceOptions {
                        variant: None,
                        langid: Some(langid!("ru"))
                    }
                }
            },
        ),
        Ok(_)
    ));

    assert!(matches!(
        Provider::load_payload(
            &provider,
            &DataRequest {
                resource_path: ResourcePath {
                    key: key::CARDINAL_V1,
                    options: ResourceOptions {
                        variant: None,
                        langid: Some(langid!("zh"))
                    }
                }
            },
        ),
        Err(DataError {
            kind: DataErrorKind::MissingResourceOptions,
            ..
        })
    ));

    assert!(matches!(
        Provider::load_payload(
            &provider,
            &DataRequest {
                resource_path: ResourcePath {
                    key: key::ORDINAL_V1,
                    options: ResourceOptions {
                        variant: None,
                        langid: Some(langid!("ru"))
                    }
                }
            },
        ),
        Err(DataError {
            kind: DataErrorKind::MissingResourceKey,
            ..
        })
    ));

    assert!(matches!(
        Provider::load_payload(
            &provider,
            &DataRequest {
                resource_path: ResourcePath {
                    key: icu_provider::hello_world::key::HELLO_WORLD_V1,
                    options: ResourceOptions {
                        variant: None,
                        langid: Some(langid!("ru"))
                    }
                }
            },
        ),
        Err(DataError {
            kind: DataErrorKind::MissingResourceKey,
            ..
        })
    ));
}

#[test]
#[cfg(feature = "deserialize_json")]
fn test_json_iterator() {
    let provider = FsDataProvider::try_new("./tests/testdata/json")
        .expect("Loading file from testdata directory");

    let mut options: Vec<ResourceOptions> = provider
        .supported_options_for_key(&key::CARDINAL_V1)
        .unwrap()
        .collect();

    options.sort_by_cached_key(|options| (options.variant.to_owned(), options.langid.to_owned()));

    assert_eq!(
        options,
        vec![
            ResourceOptions {
                variant: None,
                langid: Some(langid!("ru"))
            },
            ResourceOptions {
                variant: Some(Cow::Borrowed("variant")),
                langid: Some(langid!("de"))
            }
        ]
    );
}

#[test]
#[cfg(feature = "deserialize_bincode_1")]
fn test_bincode() {
    let provider = FsDataProvider::try_new("./tests/testdata/bincode")
        .expect("Loading file from testdata directory");

    let plurals_data: DataPayload<PluralRulesV1Marker> = provider
        .load_payload(&get_request(langid!("sr")))
        .expect("The data should be valid")
        .take_payload()
        .expect("The data should be present");
    assert_eq!(plurals_data.get(), &PluralRulesV1::from(&EXPECTED_SR_DATA));
}

#[test]
#[cfg(feature = "deserialize_bincode_1")]
fn test_bincode_dyn_erased_serde() {
    let provider = FsDataProvider::try_new("./tests/testdata/bincode")
        .expect("Loading file from testdata directory");

    let plurals_data: DataPayload<PluralRulesV1Marker> = (&provider as &dyn BufferProvider)
        .as_deserializing()
        .load_payload(&get_request(langid!("sr")))
        .expect("The data should be valid")
        .take_payload()
        .expect("The data should be present");
    assert_eq!(plurals_data.get(), &PluralRulesV1::from(&EXPECTED_SR_DATA));
}

#[test]
#[cfg(feature = "deserialize_bincode_1")]
fn test_bincode_iterator() {
    let provider = FsDataProvider::try_new("./tests/testdata/bincode")
        .expect("Loading file from testdata directory");

    let mut options: Vec<ResourceOptions> = provider
        .supported_options_for_key(&key::CARDINAL_V1)
        .unwrap()
        .collect();

    options.sort_by_cached_key(|options| (options.variant.to_owned(), options.langid.to_owned()));

    assert_eq!(
        options,
        vec![
            ResourceOptions {
                variant: None,
                langid: Some(langid!("sr"))
            },
            ResourceOptions {
                variant: Some(Cow::Borrowed("variant")),
                langid: Some(langid!("de"))
            }
        ]
    );
}
