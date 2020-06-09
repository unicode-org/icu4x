extern crate icu_data_provider_json;

use std::borrow::Cow;
use std::prelude::v1::*;

use icu_data_provider as datap;
use icu_data_provider::DataProvider;
use icu_data_provider_json::JsonDataProvider;

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

fn get_response(provider: &JsonDataProvider) -> datap::Response {
    return provider
        .load(&datap::Request {
            langid: "en-US".parse().unwrap(),
            category: datap::Category::Decimal,
            key: datap::decimal::Key::SymbolsV1.into(),
            payload: None,
        })
        .unwrap();
}

fn check_data(decimal_data: &datap::decimal::SymbolsV1) {
    assert_eq!(
        decimal_data,
        &datap::decimal::SymbolsV1 {
            zero_digit: '0',
            decimal_separator: Cow::Borrowed("."),
            grouping_separator: Cow::Borrowed(","),
        }
    );
}

#[test]
fn test_read_string() {
    let provider = get_provider();
    let response = get_response(&provider);
    let decimal_data: &datap::decimal::SymbolsV1 = response.borrow_payload().unwrap();
    check_data(decimal_data);
}

#[test]
fn test_read_utf8() {
    let provider = JsonDataProvider::from_slice(DATA.as_bytes()).unwrap();
    let response = get_response(&provider);
    let decimal_data: &datap::decimal::SymbolsV1 = response.borrow_payload().unwrap();
    check_data(decimal_data);
}

#[test]
fn test_borrow_payload_mut() {
    let provider = get_provider();
    let mut response = get_response(&provider);
    let decimal_data: &mut datap::decimal::SymbolsV1 = response.borrow_payload_mut().unwrap();
    check_data(decimal_data);
}

#[test]
fn test_take_payload() {
    let provider = get_provider();
    let response = get_response(&provider);
    let decimal_data: Cow<datap::decimal::SymbolsV1> = response.take_payload().unwrap();
    check_data(&decimal_data);
}

#[test]
fn test_clone_payload() {
    let final_data = {
        let provider = get_provider();
        let response = get_response(&provider);
        let decimal_data: Cow<datap::decimal::SymbolsV1> = response.take_payload().unwrap();
        decimal_data.into_owned()
    };
    check_data(&final_data);
}
