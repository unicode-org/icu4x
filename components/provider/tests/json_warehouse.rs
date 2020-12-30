// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::Debug;
use std::str::FromStr;

use icu_provider::prelude::*;
use icu_provider::structs::{self, icu4x::HelloV1};

// This file tests DataProvider borrow semantics with a dummy data provider based on a
// JSON string. It also exercises most of the data provider code paths.

/// Key for HelloAlt, used for testing mismatched types
const HELLO_ALT_KEY: ResourceKey = icu_provider::resource_key!(icu4x, "helloalt", 1);

/// A data struct serialization-compatible with HelloV1 used for testing mismatched types
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
struct HelloAlt {
    hello: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
struct JsonSchema<'s> {
    #[serde(borrow)]
    pub hello_v1: HelloV1<'s>,
    pub hello_alt: HelloAlt,
}

/// A DataProvider that returns owned data; uses impl_erased!() and supports only HelloV1
#[derive(Debug)]
struct DataWarehouse<'s> {
    data: JsonSchema<'s>,
}

/// A DataProvider that returns borrowed data; custom implementation of ErasedDataProvider
#[derive(Debug)]
struct DataProviderBorrowing<'d, 's> {
    borrowed_data: &'d JsonSchema<'s>,
}

/// A DataProvider that uses borrowed data with impl_erased!() and supports only HelloV1
#[derive(Debug)]
struct DataProviderImplErased<'d, 's> {
    borrowed_data: &'d JsonSchema<'s>,
}

impl<'s> DataWarehouse<'s> {
    pub fn provider_borrowing<'d>(&'d self) -> DataProviderBorrowing<'d, 's> {
        self.into()
    }

    pub fn provider_impl_erased<'d>(&'d self) -> DataProviderImplErased<'d, 's> {
        self.into()
    }
}

impl<'d, 's> From<&'d DataWarehouse<'s>> for DataProviderBorrowing<'d, 's> {
    fn from(warehouse: &'d DataWarehouse<'s>) -> Self {
        DataProviderBorrowing {
            borrowed_data: &warehouse.data,
        }
    }
}

impl<'d, 's> From<&'d DataWarehouse<'s>> for DataProviderImplErased<'d, 's> {
    fn from(warehouse: &'d DataWarehouse<'s>) -> Self {
        DataProviderImplErased {
            borrowed_data: &warehouse.data,
        }
    }
}

impl<'d> DataProvider<'d, HelloV1<'d>> for DataWarehouse<'d> {
    fn load_payload(&self, req: &DataRequest) -> Result<DataResponse<'d, HelloV1<'d>>, DataError> {
        if req.resource_path.key != structs::icu4x::key::HELLO_V1 {
            return Err(DataError::UnsupportedResourceKey(req.resource_path.key));
        }
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(Cow::Owned(self.data.hello_v1.clone())),
        })
    }
}

icu_provider::impl_erased!(DataWarehouse<'static>, 'd);

impl<'d, 's> DataProvider<'d, HelloV1<'s>> for DataProviderBorrowing<'d, 's> {
    fn load_payload(&self, req: &DataRequest) -> Result<DataResponse<'d, HelloV1<'s>>, DataError> {
        if req.resource_path.key != structs::icu4x::key::HELLO_V1 {
            return Err(DataError::UnsupportedResourceKey(req.resource_path.key));
        }
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(Cow::Borrowed(&self.borrowed_data.hello_v1)),
        })
    }
}

impl<'d, 's> DataProvider<'d, HelloAlt> for DataProviderBorrowing<'d, 's> {
    fn load_payload(&self, req: &DataRequest) -> Result<DataResponse<'d, HelloAlt>, DataError> {
        if req.resource_path.key != HELLO_ALT_KEY {
            return Err(DataError::UnsupportedResourceKey(req.resource_path.key));
        }
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(Cow::Borrowed(&self.borrowed_data.hello_alt)),
        })
    }
}

impl<'d> ErasedDataProvider<'d> for DataProviderBorrowing<'d, 'static> {
    /// Loads JSON data. Returns borrowed data.
    fn load_to_receiver(
        &self,
        req: &DataRequest,
        receiver: &mut dyn ErasedDataReceiver<'d>,
    ) -> Result<DataResponseMetadata, DataError> {
        match req.resource_path.key {
            structs::icu4x::key::HELLO_V1 => {
                receiver.receive_erased(Cow::Borrowed(&self.borrowed_data.hello_v1))?;
                Ok(DataResponseMetadata::default())
            }
            HELLO_ALT_KEY => {
                receiver.receive_erased(Cow::Borrowed(&self.borrowed_data.hello_alt))?;
                Ok(DataResponseMetadata::default())
            }
            _ => Err(DataError::UnsupportedResourceKey(req.resource_path.key)),
        }
    }
}

impl<'d, 's> DataProvider<'d, HelloV1<'s>> for DataProviderImplErased<'d, 's> {
    fn load_payload(&self, req: &DataRequest) -> Result<DataResponse<'d, HelloV1<'s>>, DataError> {
        if req.resource_path.key != structs::icu4x::key::HELLO_V1 {
            return Err(DataError::UnsupportedResourceKey(req.resource_path.key));
        }
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(Cow::Borrowed(&self.borrowed_data.hello_v1)),
        })
    }
}

icu_provider::impl_erased!(DataProviderImplErased<'d, 'static>, 'd);

#[allow(clippy::redundant_static_lifetimes)]
const DATA: &'static str = r#"{
    "hello_v1": {
        "hello": "Hello V1"
    },
    "hello_alt": {
        "hello": "Hello Alt"
    }
}"#;

fn get_warehouse<'s>(data: &'s str) -> DataWarehouse<'s> {
    let data: JsonSchema = serde_json::from_str(data).expect("Well-formed data");
    DataWarehouse { data }
}

fn get_receiver_v1<'d, 's>() -> DataReceiver<'d, HelloV1<'s>> {
    DataReceiver::new()
}

fn get_payload_v1<'d, P: DataProvider<'d, HelloV1<'d>> + ?Sized + 'd>(
    provider: &P,
) -> Result<Cow<'d, HelloV1<'d>>, DataError> {
    provider.load_payload(&get_request_v1())?.take_payload()
}

fn get_payload_alt<'d, P: DataProvider<'d, HelloAlt> + ?Sized>(
    d: &P,
) -> Result<Cow<'d, HelloAlt>, DataError> {
    d.load_payload(&get_request_alt())?.take_payload()
}

fn get_request_v1() -> DataRequest {
    DataRequest {
        resource_path: ResourcePath {
            key: structs::icu4x::key::HELLO_V1,
            options: Default::default(),
        },
    }
}

fn get_request_alt() -> DataRequest {
    DataRequest {
        resource_path: ResourcePath {
            key: HELLO_ALT_KEY,
            options: Default::default(),
        },
    }
}

fn check_data_v1(hello_data: &HelloV1) {
    assert_eq!(
        hello_data,
        &HelloV1 {
            hello: Cow::Borrowed("Hello V1"),
        }
    );
}

fn check_data_alt(hello_data: &HelloAlt) {
    assert_eq!(
        hello_data,
        &HelloAlt {
            hello: "Hello Alt".to_string(),
        }
    );
}

#[test]
fn test_warehouse() {
    let warehouse = get_warehouse(DATA);
    let hello_data = get_payload_v1(&warehouse);
    check_data_v1(&hello_data.unwrap());
}

#[test]
fn test_warehouse_dyn_erased() {
    let warehouse = get_warehouse(DATA);
    let hello_data = get_payload_v1(&warehouse as &dyn ErasedDataProvider);
    check_data_v1(&hello_data.unwrap());
}

#[test]
fn test_warehouse_dyn_erased_alt() {
    let warehouse = get_warehouse(DATA);
    let response = get_payload_alt(&warehouse as &dyn ErasedDataProvider);
    assert!(matches!(
        response,
        Err(DataError::UnsupportedResourceKey { .. })
    ));
}

/*

#[test]
fn test_data_receiver() {
    let warehouse = get_warehouse(DATA);
    let mut receiver: DataReceiver<HelloV1> = get_receiver_v1();
    warehouse
        .provider_borrowing()
        .load_to_receiver(&get_request_v1(), &mut receiver)
        .unwrap();
    let hello_data: &HelloV1 = &receiver.take_payload().unwrap();
    check_data_v1(hello_data);
}

#[test]
fn test_generic() {
    let warehouse = get_warehouse(DATA);
    let hello_data = get_payload_v1(&warehouse.provider_borrowing());
    check_data_v1(&hello_data.unwrap());
}

#[test]
fn test_dyn_erased() {
    let warehouse = get_warehouse(DATA);
    let hello_data = get_payload_v1(&warehouse.provider_borrowing() as &dyn ErasedDataProvider);
    check_data_v1(&hello_data.unwrap());
}

#[test]
fn test_dyn_generic() {
    let warehouse = get_warehouse(DATA);
    let hello_data = get_payload_v1(&warehouse.provider_borrowing() as &dyn DataProvider<HelloV1>);
    check_data_v1(&hello_data.unwrap());
}

#[test]
fn test_mismatched_types() {
    let warehouse = get_warehouse(DATA);
    // Request is for v2, but type argument is for v1
    let response: Result<DataResponse<HelloV1>, DataError> = (&warehouse.provider_borrowing()
        as &dyn ErasedDataProvider)
        .load_payload(&get_request_alt());
    assert!(matches!(response, Err(DataError::MismatchedType { .. })));
}

#[test]
fn test_impl_erased() {
    let warehouse = get_warehouse(DATA);
    let hello_data = get_payload_v1(&warehouse.provider_impl_erased() as &dyn ErasedDataProvider);
    check_data_v1(&hello_data.unwrap());
}

fn check_v1_v2<'d, P: DataProvider<'d, HelloV1<'d>> + DataProvider<'d, HelloAlt> + ?Sized>(d: &P) {
    let v1: Cow<'d, HelloV1> = d
        .load_payload(&get_request_v1())
        .unwrap()
        .take_payload()
        .unwrap();
    let v2: Cow<'d, HelloAlt> = d
        .load_payload(&get_request_alt())
        .unwrap()
        .take_payload()
        .unwrap();
    if v1.hello == v2.hello {
        panic!()
    }
}

#[test]
fn test_v1_v2_generic() {
    let warehouse = get_warehouse(DATA);
    check_v1_v2(&warehouse.provider_borrowing());
}

#[test]
fn test_v1_v2_dyn_erased() {
    let warehouse = get_warehouse(DATA);
    check_v1_v2(&warehouse.provider_borrowing() as &dyn ErasedDataProvider);
}

*/
