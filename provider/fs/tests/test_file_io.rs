// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(unused_imports)]

use icu_locid::LanguageIdentifier;
use icu_locid_macros::langid;
use icu_plurals::provider::*;
use icu_provider::prelude::*;
use icu_provider::serde::*;
use icu_provider_fs::FsDataProvider;
use std::borrow::Cow;

#[cfg(feature = "provider_json")]
const EXPECTED_RU_DATA: PluralRulesV1 = PluralRulesV1 {
    zero: None,
    one: "v = 0 and i % 10 = 1 and i % 100 != 11".parse().ok(),
    two: None,
    few: "v = 0 and i % 10 = 2..4 and i % 100 != 12..14".parse().ok(),
    many: "v = 0 and i % 10 = 0 or v = 0 and i % 10 = 5..9 or v = 0 and i % 100 = 11..14"
        .parse()
        .ok(),
};

#[cfg(feature = "provider_bincode")]
const EXPECTED_SR_DATA: PluralRulesV1 = PluralRulesV1 {
    zero: None,
    one: "v = 0 and i % 10 = 1 and i % 100 != 11 or f % 10 = 1 and f % 100 != 11"
        .parse()
        .ok(),
    two: None,
    few: "v = 0 and i % 10 = 2..4 and i % 100 != 12..14 or f % 10 = 2..4 and f % 100 != 12..14"
        .parse()
        .ok(),
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

#[cfg(not(feature = "provider_json"))]
#[test]
fn test_json_feature() {
    FsDataProvider::try_new("./tests/testdata/json").expect_err("JSON is not enabled");
}

#[cfg(feature = "provider_json")]
#[test]
fn test_json() {
    let provider = FsDataProvider::try_new("./tests/testdata/json")
        .expect("Loading file from testdata directory");

    let plurals_data: DataPayload<PluralRulesV1Marker> = provider
        .load_payload(&get_request(langid!("ru")))
        .expect("The data should be valid")
        .take_payload()
        .expect("The data should be present");
    assert_eq!(plurals_data.get(), &EXPECTED_RU_DATA);
}

#[cfg(feature = "provider_json")]
#[test]
fn test_json_dyn_erased_serde() {
    let provider = FsDataProvider::try_new("./tests/testdata/json")
        .expect("Loading file from testdata directory");

    let plurals_data: DataPayload<PluralRulesV1Marker> = (&provider as &dyn SerdeDeDataProvider)
        .load_payload(&get_request(langid!("ru")))
        .expect("The data should be valid")
        .take_payload()
        .expect("The data should be present");
    assert_eq!(plurals_data.get(), &EXPECTED_RU_DATA);
}

#[cfg(feature = "provider_json")]
#[test]
fn test_json_errors() {
    let provider = FsDataProvider::try_new("./tests/testdata/json")
        .expect("Loading file from testdata directory");

    type Provider<'data> = dyn DataProvider<'data, PluralRulesV1Marker>;

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
        Err(DataError::MissingResourceOptions(_))
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
        Err(DataError::MissingResourceKey(_))
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
        Err(DataError::MissingResourceKey(_))
    ));
}

#[test]
#[cfg(feature = "provider_bincode")]
fn test_bincode() {
    let provider = FsDataProvider::try_new("./tests/testdata/bincode")
        .expect("Loading file from testdata directory");

    let plurals_data: DataPayload<PluralRulesV1Marker> = provider
        .load_payload(&get_request(langid!("sr")))
        .expect("The data should be valid")
        .take_payload()
        .expect("The data should be present");
    assert_eq!(plurals_data.get(), &EXPECTED_SR_DATA);
}

#[test]
#[cfg(feature = "provider_bincode")]
fn test_bincode_dyn_erased_serde() {
    let provider = FsDataProvider::try_new("./tests/testdata/bincode")
        .expect("Loading file from testdata directory");

    let plurals_data: DataPayload<PluralRulesV1Marker> = (&provider as &dyn SerdeDeDataProvider)
        .load_payload(&get_request(langid!("sr")))
        .expect("The data should be valid")
        .take_payload()
        .expect("The data should be present");
    assert_eq!(plurals_data.get(), &EXPECTED_SR_DATA);
}
