use std::borrow::Cow;
use std::prelude::v1::*;
use std::str::FromStr;

use icu_data_provider::icu_data_key;
use icu_data_provider::prelude::*;
use icu_data_provider::structs;
use icu_data_provider_json::JsonDataWarehouse;

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

fn get_warehouse() -> JsonDataWarehouse {
    JsonDataWarehouse::from_str(DATA).unwrap()
}

fn get_response(warehouse: &JsonDataWarehouse) -> data_provider::Response {
    warehouse
        .provider()
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
    let warehouse = get_warehouse();
    let response = get_response(&warehouse);
    let decimal_data: &structs::decimal::SymbolsV1 = response.borrow_payload().unwrap();
    check_data(decimal_data);
}

#[test]
fn test_read_utf8() {
    let warehouse = JsonDataWarehouse::from_slice(DATA.as_bytes()).unwrap();
    let response = get_response(&warehouse);
    let decimal_data: &structs::decimal::SymbolsV1 = response.borrow_payload().unwrap();
    check_data(decimal_data);
}

#[test]
fn test_borrow_payload_mut() {
    let warehouse = get_warehouse();
    let mut response = get_response(&warehouse);
    let decimal_data: &mut structs::decimal::SymbolsV1 = response.borrow_payload_mut().unwrap();
    check_data(decimal_data);
}

#[test]
fn test_take_payload() {
    let warehouse = get_warehouse();
    let response = get_response(&warehouse);
    let decimal_data: Cow<structs::decimal::SymbolsV1> = response.take_payload().unwrap();
    check_data(&decimal_data);
}

#[test]
fn test_clone_payload() {
    let final_data = {
        let warehouse = get_warehouse();
        let response = get_response(&warehouse);
        let decimal_data: Cow<structs::decimal::SymbolsV1> = response.take_payload().unwrap();
        decimal_data.into_owned()
    };
    check_data(&final_data);
}
