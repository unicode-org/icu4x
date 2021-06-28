// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locid::LanguageIdentifier;
use icu_locid_macros::langid;
use icu_plurals::provider::*;
use icu_provider::prelude::*;
use icu_provider::serde::*;
use icu_provider_fs::FsDataProvider;
use std::borrow::Cow;

const EXPECTED_RU_DATA: PluralRuleStringsV1 = PluralRuleStringsV1 {
    zero: None,
    one: Some(Cow::Borrowed("v = 0 and i % 10 = 1 and i % 100 != 11")),
    two: None,
    few: Some(Cow::Borrowed(
        "v = 0 and i % 10 = 2..4 and i % 100 != 12..14",
    )),
    many: Some(Cow::Borrowed(
        "v = 0 and i % 10 = 0 or v = 0 and i % 10 = 5..9 or v = 0 and i % 100 = 11..14",
    )),
};

#[cfg(feature = "bincode")]
const EXPECTED_SR_DATA: PluralRuleStringsV1 = PluralRuleStringsV1 {
    zero: None,
    one: Some(Cow::Borrowed(
        "v = 0 and i % 10 = 1 and i % 100 != 11 or f % 10 = 1 and f % 100 != 11",
    )),
    two: None,
    few: Some(Cow::Borrowed(
        "v = 0 and i % 10 = 2..4 and i % 100 != 12..14 or f % 10 = 2..4 and f % 100 != 12..14",
    )),
    many: None,
};

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

#[test]
fn test_json() {
    let provider = FsDataProvider::try_new("./tests/testdata/json")
        .expect("Loading file from testdata directory");

    let plurals_data: DataPayload<PluralRuleStringsV1Marker> = provider
        .load_payload(&get_request(langid!("ru")))
        .expect("The data should be valid")
        .take_payload()
        .expect("The data should be present");
    assert_eq!(plurals_data.get(), &EXPECTED_RU_DATA);
}

#[test]
fn test_json_dyn_erased_serde() {
    let provider = FsDataProvider::try_new("./tests/testdata/json")
        .expect("Loading file from testdata directory");

    let plurals_data: DataPayload<PluralRuleStringsV1Marker> = (&provider
        as &dyn SerdeDeDataProvider)
        .load_payload(&get_request(langid!("ru")))
        .expect("The data should be valid")
        .take_payload()
        .expect("The data should be present");
    assert_eq!(plurals_data.get(), &EXPECTED_RU_DATA);
}

#[test]
fn test_json_errors() {
    let provider = FsDataProvider::try_new("./tests/testdata/json")
        .expect("Loading file from testdata directory");

    type Provider<'d, 's> = dyn DataProvider<'d, 's, PluralRuleStringsV1Marker>;

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
        Err(DataError::UnavailableResourceOptions(_))
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
        Err(DataError::UnsupportedResourceKey(_))
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
        Err(DataError::UnsupportedResourceKey(_))
    ));
}

#[test]
#[cfg(feature = "export")]
fn test_json_export() {
    //
}

#[test]
#[cfg(feature = "bincode")]
fn test_bincode() {
    let provider = FsDataProvider::try_new("./tests/testdata/bincode")
        .expect("Loading file from testdata directory");

    let plurals_data: DataPayload<PluralRuleStringsV1Marker> = provider
        .load_payload(&get_request(langid!("sr")))
        .expect("The data should be valid")
        .take_payload()
        .expect("The data should be present");
    assert_eq!(plurals_data.get(), &EXPECTED_SR_DATA);
}

#[test]
#[cfg(feature = "bincode")]
fn test_bincode_dyn_erased_serde() {
    let provider = FsDataProvider::try_new("./tests/testdata/bincode")
        .expect("Loading file from testdata directory");

    let plurals_data: DataPayload<PluralRuleStringsV1Marker> = (&provider
        as &dyn SerdeDeDataProvider)
        .load_payload(&get_request(langid!("sr")))
        .expect("The data should be valid")
        .take_payload()
        .expect("The data should be present");
    assert_eq!(plurals_data.get(), &EXPECTED_SR_DATA);
}
