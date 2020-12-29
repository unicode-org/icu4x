// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::Debug;
use std::prelude::v1::*;
use std::str::FromStr;

use icu_provider::prelude::*;
use icu_provider::structs::{self, decimal::SymbolsV1, decimal::SymbolsV2};

// This file tests DataProvider borrow semantics with a dummy data provider based on a
// JSON string. It also exercises most of the data provider code paths.

#[derive(Serialize, Deserialize, Debug)]
struct DecimalJsonSchema {
    pub symbols_v1_a: SymbolsV1,
    pub symbols_v2_a: SymbolsV2,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonSchema {
    pub decimal: DecimalJsonSchema,
}

/// A DataProvider that returns owned data; uses impl_erased!() and supports only SymbolsV1
#[derive(Debug)]
struct DataWarehouse {
    data: JsonSchema,
}

/// A DataProvider that returns borrowed data; custom implementation of ErasedDataProvider
#[derive(Debug)]
struct DataProviderBorrowing<'d> {
    borrowed_data: &'d JsonSchema,
}

/// A DataProvider that uses borrowed data with impl_erased!() and supports only SymbolsV1
#[derive(Debug)]
struct DataProviderImplErased<'d> {
    borrowed_data: &'d JsonSchema,
}

impl DataWarehouse {
    pub fn provider_borrowing(&self) -> DataProviderBorrowing {
        self.into()
    }

    pub fn provider_impl_erased(&self) -> DataProviderImplErased {
        self.into()
    }
}

impl FromStr for DataWarehouse {
    type Err = DataError;

    /// Create a DataProviderBorrowing from a JSON string slice.
    fn from_str(data: &str) -> Result<Self, DataError> {
        let data: JsonSchema = match serde_json::from_str(data) {
            Ok(data) => data,
            Err(err) => return Err(DataError::new_resc_error(err)),
        };
        Ok(Self { data })
    }
}

impl<'d> From<&'d DataWarehouse> for DataProviderBorrowing<'d> {
    fn from(warehouse: &'d DataWarehouse) -> Self {
        DataProviderBorrowing {
            borrowed_data: &warehouse.data,
        }
    }
}

impl<'d> From<&'d DataWarehouse> for DataProviderImplErased<'d> {
    fn from(warehouse: &'d DataWarehouse) -> Self {
        DataProviderImplErased {
            borrowed_data: &warehouse.data,
        }
    }
}

impl DataProvider<'static, SymbolsV1> for DataWarehouse {
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<'static, SymbolsV1>, DataError> {
        if req.resource_path.key != structs::decimal::key::SYMBOLS_V1 {
            return Err(DataError::UnsupportedResourceKey(req.resource_path.key));
        }
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(Cow::Owned(self.data.decimal.symbols_v1_a.clone())),
        })
    }
}

icu_provider::impl_erased!(DataWarehouse, 'd);

impl<'d> DataProvider<'d, SymbolsV1> for DataProviderBorrowing<'d> {
    fn load_payload(&self, req: &DataRequest) -> Result<DataResponse<'d, SymbolsV1>, DataError> {
        if req.resource_path.key != structs::decimal::key::SYMBOLS_V1 {
            return Err(DataError::UnsupportedResourceKey(req.resource_path.key));
        }
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(Cow::Borrowed(&self.borrowed_data.decimal.symbols_v1_a)),
        })
    }
}

impl<'d> DataProvider<'d, SymbolsV2> for DataProviderBorrowing<'d> {
    fn load_payload(&self, req: &DataRequest) -> Result<DataResponse<'d, SymbolsV2>, DataError> {
        if req.resource_path.key != structs::decimal::key::SYMBOLS_V2 {
            return Err(DataError::UnsupportedResourceKey(req.resource_path.key));
        }
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(Cow::Borrowed(&self.borrowed_data.decimal.symbols_v2_a)),
        })
    }
}

impl<'d> ErasedDataProvider<'d> for DataProviderBorrowing<'d> {
    /// Loads JSON data. Returns borrowed data.
    fn load_to_receiver(
        &self,
        req: &DataRequest,
        receiver: &mut dyn DataReceiver<'d>,
    ) -> Result<DataResponseMetadata, DataError> {
        match req.resource_path.key {
            structs::decimal::key::SYMBOLS_V1 => {
                receiver.receive_borrow(&self.borrowed_data.decimal.symbols_v1_a)?;
                Ok(DataResponseMetadata::default())
            }
            structs::decimal::key::SYMBOLS_V2 => {
                receiver.receive_borrow(&self.borrowed_data.decimal.symbols_v2_a)?;
                Ok(DataResponseMetadata::default())
            }
            _ => Err(DataError::UnsupportedResourceKey(req.resource_path.key)),
        }
    }
}

impl<'d> DataProvider<'d, SymbolsV1> for DataProviderImplErased<'d> {
    fn load_payload(&self, req: &DataRequest) -> Result<DataResponse<'d, SymbolsV1>, DataError> {
        if req.resource_path.key != structs::decimal::key::SYMBOLS_V1 {
            return Err(DataError::UnsupportedResourceKey(req.resource_path.key));
        }
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(Cow::Borrowed(&self.borrowed_data.decimal.symbols_v1_a)),
        })
    }
}

icu_provider::impl_erased!(DataProviderImplErased<'d>, 'd);

#[allow(clippy::redundant_static_lifetimes)]
const DATA: &'static str = r#"{
    "decimal": {
        "symbols_v1_a": {
            "zero_digit": "0",
            "decimal_separator": ".",
            "grouping_separator": ","
        },
        "symbols_v2_a": {
            "zero_digit": "0",
            "decimal_separator": ",",
            "grouping_separator": "."
        }
    }
}"#;

fn get_warehouse() -> DataWarehouse {
    DataWarehouse::from_str(DATA).unwrap()
}

fn get_receiver_v1<'d>() -> DataReceiverForType<'d, SymbolsV1> {
    DataReceiverForType::default()
}

fn get_payload_v1<'d, P: DataProvider<'d, SymbolsV1> + ?Sized>(
    d: &P,
) -> Result<Cow<'d, SymbolsV1>, DataError> {
    let response = d.load_payload(&get_request_v1())?;
    Ok(response.payload.unwrap())
}

fn get_payload_v2<'d, P: DataProvider<'d, SymbolsV2> + ?Sized>(
    d: &P,
) -> Result<Cow<'d, SymbolsV2>, DataError> {
    let response = d.load_payload(&get_request_v2())?;
    Ok(response.payload.unwrap())
}

fn get_request_v1() -> DataRequest {
    DataRequest {
        resource_path: ResourcePath {
            key: structs::decimal::key::SYMBOLS_V1,
            options: Default::default(),
        },
    }
}

fn get_request_v2() -> DataRequest {
    DataRequest {
        resource_path: ResourcePath {
            key: structs::decimal::key::SYMBOLS_V2,
            options: Default::default(),
        },
    }
}

fn check_data(decimal_data: &SymbolsV1) {
    assert_eq!(
        decimal_data,
        &SymbolsV1 {
            zero_digit: '0',
            decimal_separator: ".".into(),
            grouping_separator: ",".into(),
        }
    );
}

#[test]
fn test_warehouse() {
    let warehouse = get_warehouse();
    let decimal_data = get_payload_v1(&warehouse);
    check_data(&decimal_data.unwrap());
}

#[test]
fn test_warehouse_dyn_erased() {
    let warehouse = get_warehouse();
    let decimal_data = get_payload_v1(&warehouse as &dyn ErasedDataProvider);
    check_data(&decimal_data.unwrap());
}

#[test]
fn test_warehouse_dyn_erased_v2() {
    let warehouse = get_warehouse();
    let response = get_payload_v2(&warehouse as &dyn ErasedDataProvider);
    assert!(matches!(
        response,
        Err(DataError::UnsupportedResourceKey { .. })
    ));
}

#[test]
fn test_data_receiver() {
    let warehouse = get_warehouse();
    let mut receiver = get_receiver_v1();
    warehouse
        .provider_borrowing()
        .load_to_receiver(&get_request_v1(), &mut receiver)
        .unwrap();
    let decimal_data: &SymbolsV1 = &receiver.payload.unwrap();
    check_data(decimal_data);
}

#[test]
fn test_generic() {
    let warehouse = get_warehouse();
    let decimal_data = get_payload_v1(&warehouse.provider_borrowing());
    check_data(&decimal_data.unwrap());
}

#[test]
fn test_dyn_erased() {
    let warehouse = get_warehouse();
    let decimal_data = get_payload_v1(&warehouse.provider_borrowing() as &dyn ErasedDataProvider);
    check_data(&decimal_data.unwrap());
}

#[test]
fn test_dyn_generic() {
    let warehouse = get_warehouse();
    let decimal_data =
        get_payload_v1(&warehouse.provider_borrowing() as &dyn DataProvider<SymbolsV1>);
    check_data(&decimal_data.unwrap());
}

#[test]
fn test_mismatched_types() {
    let warehouse = get_warehouse();
    // Request is for v2, but type argument is for v1
    let response: Result<DataResponse<SymbolsV1>, DataError> = (&warehouse.provider_borrowing()
        as &dyn ErasedDataProvider)
        .load_payload(&get_request_v2());
    assert!(matches!(response, Err(DataError::MismatchedType { .. })));
}

#[test]
fn test_impl_erased() {
    let warehouse = get_warehouse();
    let decimal_data = get_payload_v1(&warehouse.provider_impl_erased() as &dyn ErasedDataProvider);
    check_data(&decimal_data.unwrap());
}

fn check_v1_v2<'d, P: DataProvider<'d, SymbolsV1> + DataProvider<'d, SymbolsV2> + ?Sized>(d: &P) {
    let v1: Cow<'d, SymbolsV1> = d.load_payload(&get_request_v1()).unwrap().payload.unwrap();
    let v2: Cow<'d, SymbolsV2> = d.load_payload(&get_request_v2()).unwrap().payload.unwrap();
    if v1.zero_digit != v2.zero_digit {
        panic!()
    }
}

#[test]
fn test_v1_v2_generic() {
    let warehouse = get_warehouse();
    check_v1_v2(&warehouse.provider_borrowing());
}

#[test]
fn test_v1_v2_dyn_erased() {
    let warehouse = get_warehouse();
    check_v1_v2(&warehouse.provider_borrowing() as &dyn ErasedDataProvider);
}
