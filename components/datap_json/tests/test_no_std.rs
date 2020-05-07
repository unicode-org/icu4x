extern crate icu_datap_json;

use std::borrow::Cow;
use std::prelude::v1::*;

use icu_datap_json::JsonDataProvider;
use icu_util::datap;
use icu_util::datap::DataProvider;

const DATA: &'static str = r#"{
    "decimal": {
        "symbols_v1_a": {
            "zero_digit": "0",
            "decimal_separator": ".",
            "grouping_separator": ","
        }
    }
}"#;

fn get_response() -> datap::Response {
    let json_data_provider = JsonDataProvider::from_str(DATA).unwrap();
    return json_data_provider
        .load(datap::Request {
            locale: "root".to_string(),
            category: datap::Category::Decimal,
            key: datap::Key::Decimal(datap::decimal::Key::SymbolsV1),
            payload: None,
        })
        .unwrap();
}

fn check_data(decimal_data: &datap::decimal::SymbolsV1) {
    assert_eq!(
        decimal_data,
        &datap::decimal::SymbolsV1 {
            zero_digit: '0',
            decimal_separator: ".".to_string(),
            grouping_separator: ",".to_string(),
        }
    );
}

#[test]
fn test_read_string() {
    let response = get_response();
    let decimal_data: &datap::decimal::SymbolsV1 = response.borrow_payload().unwrap();
    check_data(decimal_data);
}

#[test]
fn test_borrow_payload_mut() {
    let mut response = get_response();
    let decimal_data: &mut datap::decimal::SymbolsV1 = response.borrow_payload_mut().unwrap();
    check_data(decimal_data);
}

#[test]
fn test_take_payload() {
    let response = get_response();
    let decimal_data: Cow<'static, datap::decimal::SymbolsV1> = response.take_payload().unwrap();
    check_data(&decimal_data);
}
