// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use icu_locid::LanguageIdentifier;
use icu_locid_macros::langid;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::prelude::v1::*;
use std::str::FromStr;

use icu_provider::prelude::*;
use icu_provider::structs;
use icu_provider::v2;

// This file tests DataProvider borrow semantics with a dummy data provider based on a JSON string.

#[derive(Serialize, Deserialize, Debug)]
struct DecimalJsonSchema {
    pub symbols_v1_a: structs::decimal::SymbolsV1,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonSchema {
    pub decimal: DecimalJsonSchema,
}

#[derive(Debug)]
struct JsonDataWarehouse {
    data: JsonSchema,
}

/// A data provider reading from a JSON file.
#[derive(Debug)]
struct JsonDataProvider<'d> {
    borrowed_data: &'d JsonSchema,
}

impl JsonDataWarehouse {
    pub fn provider(&self) -> JsonDataProvider {
        self.into()
    }
}

impl FromStr for JsonDataWarehouse {
    type Err = DataError;

    /// Create a JsonDataProvider from a JSON string slice.
    fn from_str(data: &str) -> Result<Self, DataError> {
        let data: JsonSchema = match serde_json::from_str(data) {
            Ok(data) => data,
            Err(err) => return Err(DataError::new_resc_error(err)),
        };
        Ok(Self { data })
    }
}

impl<'d> From<&'d JsonDataWarehouse> for JsonDataProvider<'d> {
    fn from(warehouse: &'d JsonDataWarehouse) -> JsonDataProvider<'d> {
        JsonDataProvider {
            borrowed_data: &warehouse.data,
        }
    }
}

impl<'d> DataProvider<'d> for JsonDataProvider<'d> {
    /// Loads JSON data. Returns borrowed data.
    fn load(&self, _request: &DataRequest) -> Result<DataResponse<'d>, DataError> {
        let response = DataResponseBuilder {
            data_langid: LanguageIdentifier::default(),
        }
        .with_borrowed_payload(&self.borrowed_data.decimal.symbols_v1_a);
        Ok(response)
    }
}

impl<'d> DataProviderV2<'d> for JsonDataProvider<'d> {
    /// Loads JSON data. Returns borrowed data.
    fn load_v2(
        &self,
        _request: &DataRequest,
        receiver: &mut dyn DataReceiver<'d, 'static>,
    ) -> Result<DataResponseV2, DataError> {
        receiver.set_to_any(&self.borrowed_data.decimal.symbols_v1_a)?;
        Ok(DataResponseV2 {
            data_langid: LanguageIdentifier::default(),
        })
    }
}

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

fn get_receiver<'d>() -> DataReceiverImpl<'d, structs::decimal::SymbolsV1> {
    DataReceiverImpl { payload: None }
}

fn get_request() -> DataRequest {
    DataRequest {
        data_key: structs::decimal::key::SYMBOLS_V1,
        data_entry: DataEntry {
            variant: None,
            langid: langid!("en-US"),
        },
    }
}

fn get_response(warehouse: &JsonDataWarehouse) -> DataResponse {
    warehouse.provider().load(&get_request()).unwrap()
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

#[test]
fn test_data_receiver() {
    let warehouse = get_warehouse();
    let mut receiver = get_receiver();
    warehouse
        .provider()
        .load_v2(&get_request(), &mut receiver)
        .unwrap();
    let decimal_data: &structs::decimal::SymbolsV1 = &receiver.payload.unwrap();
    check_data(decimal_data);
}

#[test]
fn test_data_receiver_borrow() {
    let warehouse = get_warehouse();
    let mut receiver = get_receiver();
    warehouse
        .provider()
        .load_v2(&get_request(), &mut receiver)
        .unwrap();
    let decimal_data: &structs::decimal::SymbolsV1 =
        v2::borrow_payload(&receiver).unwrap().unwrap();
    check_data(decimal_data);
}

#[test]
fn test_data_receiver_borrow_mut() {
    let warehouse = get_warehouse();
    let mut receiver = get_receiver();
    warehouse
        .provider()
        .load_v2(&get_request(), &mut receiver)
        .unwrap();
    let decimal_data: &mut structs::decimal::SymbolsV1 =
        v2::borrow_payload_mut(&mut receiver).unwrap().unwrap();
    check_data(decimal_data);
}
