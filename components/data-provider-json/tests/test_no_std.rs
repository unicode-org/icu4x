use std::borrow::Cow;
use std::prelude::v1::*;
use std::str::FromStr;

use icu_data_provider::icu_data_key;
use icu_data_provider::prelude::*;
use icu_data_provider::structs;
use icu_data_provider_json::JsonDataProvider;

#[allow(clippy::redundant_static_lifetimes)]
const DATA: &'static str = r#"{
    "decimal": {
        "symbols_v1_a": {
            "zero_digit": "0",
            "decimal_separator": ".",
            "grouping_separator": ","
        }
    }
}"#;

fn get_provider() -> JsonDataProvider {
    JsonDataProvider::from_str(DATA).unwrap()
}

fn get_response(provider: &JsonDataProvider) -> data_provider::Response {
    provider
        .load(&data_provider::Request {
            data_key: icu_data_key!(decimal: symbols@1),
            data_entry: DataEntry {
                variant: None,
                langid: "en-US".parse().unwrap(),
            },
        })
        .unwrap()
}

fn check_data(decimal_data: &structs::decimal::SymbolsV1) {
    assert_eq!(
        decimal_data,
        &structs::decimal::SymbolsV1 {
            zero_digit: '0',
            decimal_separator: ".".into(),
            grouping_separator: ",".into(),
        }
    );
}

#[test]
fn test_read_string() {
    let provider = get_provider();
    let response = get_response(&provider);
    let decimal_data: &structs::decimal::SymbolsV1 = response.borrow_payload().unwrap();
    check_data(decimal_data);
}

#[test]
fn test_read_utf8() {
    let provider = JsonDataProvider::from_slice(DATA.as_bytes()).unwrap();
    let response = get_response(&provider);
    let decimal_data: &structs::decimal::SymbolsV1 = response.borrow_payload().unwrap();
    check_data(decimal_data);
}

#[test]
fn test_borrow_payload_mut() {
    let provider = get_provider();
    let mut response = get_response(&provider);
    let decimal_data: &mut structs::decimal::SymbolsV1 = response.borrow_payload_mut().unwrap();
    check_data(decimal_data);
}

#[test]
fn test_take_payload() {
    let provider = get_provider();
    let response = get_response(&provider);
    let decimal_data: Cow<structs::decimal::SymbolsV1> = response.take_payload().unwrap();
    check_data(&decimal_data);
}

#[test]
fn test_clone_payload() {
    let final_data = {
        let provider = get_provider();
        let response = get_response(&provider);
        let decimal_data: Cow<structs::decimal::SymbolsV1> = response.take_payload().unwrap();
        decimal_data.into_owned()
    };
    check_data(&final_data);
}
