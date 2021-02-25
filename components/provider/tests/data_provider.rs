// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::Debug;

use icu_provider::erased::*;
use icu_provider::hello_world::{self, HelloWorldV1};
use icu_provider::prelude::*;

// This file tests DataProvider borrow semantics with a dummy data provider based on a
// JSON string. It also exercises most of the data provider code paths.

/// Key for HelloAlt, used for testing mismatched types
const HELLO_ALT_KEY: ResourceKey = icu_provider::resource_key!(icu4x, "helloalt", 1);

/// A data struct serialization-compatible with HelloWorldV1 used for testing mismatched types
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
struct HelloAlt {
    message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
struct HelloCombined<'s> {
    #[serde(borrow)]
    pub hello_v1: HelloWorldV1<'s>,
    pub hello_alt: HelloAlt,
}

/// A DataProvider that owns its data. DataProvider is implemented on `DataWarehouse`, returning
/// owned data, and on `&'d DataWarehouse`, returning borrowed data. Both support only
/// HELLO_WORLD_V1 and use `impl_erased!()`.
#[derive(Debug)]
struct DataWarehouse<'s> {
    data: HelloCombined<'s>,
}

impl<'d, 's: 'd> DataProvider<'d, HelloWorldV1<'s>> for DataWarehouse<'s> {
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<'d, HelloWorldV1<'s>>, DataError> {
        req.resource_path
            .key
            .match_key(hello_world::key::HELLO_WORLD_V1)?;
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(Cow::Owned(self.data.hello_v1.clone())),
        })
    }
}

icu_provider::impl_erased!(DataWarehouse<'static>, 'd);

impl<'d, 's: 'd> DataProvider<'d, HelloWorldV1<'s>> for &'d DataWarehouse<'s> {
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<'d, HelloWorldV1<'s>>, DataError> {
        req.resource_path
            .key
            .match_key(hello_world::key::HELLO_WORLD_V1)?;
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(Cow::Borrowed(&self.data.hello_v1)),
        })
    }
}

icu_provider::impl_erased!(&'d DataWarehouse<'static>, 'd);

/// A DataProvider that returns borrowed data. Supports both HELLO_WORLD_V1 and HELLO_ALT. Custom implementation of
/// ErasedDataProvider.
#[derive(Debug)]
struct DataProviderBorrowing<'d, 's> {
    borrowed_data: &'d HelloCombined<'s>,
}

impl<'d, 's> From<&'d DataWarehouse<'s>> for DataProviderBorrowing<'d, 's> {
    fn from(warehouse: &'d DataWarehouse<'s>) -> Self {
        DataProviderBorrowing {
            borrowed_data: &warehouse.data,
        }
    }
}

impl<'d, 's> DataProvider<'d, HelloWorldV1<'s>> for DataProviderBorrowing<'d, 's> {
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<'d, HelloWorldV1<'s>>, DataError> {
        req.resource_path
            .key
            .match_key(hello_world::key::HELLO_WORLD_V1)?;
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(Cow::Borrowed(&self.borrowed_data.hello_v1)),
        })
    }
}

impl<'d, 's> DataProvider<'d, HelloAlt> for DataProviderBorrowing<'d, 's> {
    fn load_payload(&self, req: &DataRequest) -> Result<DataResponse<'d, HelloAlt>, DataError> {
        req.resource_path.key.match_key(HELLO_ALT_KEY)?;
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(Cow::Borrowed(&self.borrowed_data.hello_alt)),
        })
    }
}

impl<'d> ErasedDataProvider<'d> for DataProviderBorrowing<'d, 'static> {
    /// Loads JSON data. Returns borrowed data.
    fn load_to_receiver<'a>(
        &self,
        req: &DataRequest,
        receiver: &'a mut dyn ErasedDataReceiver<'d, '_>,
    ) -> Result<DataResponseMetadata, DataError> {
        match req.resource_path.key {
            hello_world::key::HELLO_WORLD_V1 => {
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

#[allow(clippy::redundant_static_lifetimes)]
const DATA: &'static str = r#"{
    "hello_v1": {
        "message": "Hello V1"
    },
    "hello_alt": {
        "message": "Hello Alt"
    }
}"#;

#[allow(clippy::needless_lifetimes)]
fn get_warehouse<'s>(data: &'s str) -> DataWarehouse<'s> {
    let data: HelloCombined = serde_json::from_str(data).expect("Well-formed data");
    DataWarehouse { data }
}

fn get_receiver_v1<'d, 's>() -> DataReceiver<'d, HelloWorldV1<'s>> {
    DataReceiver::new()
}

fn get_payload_v1<'d, 's, P: DataProvider<'d, HelloWorldV1<'s>> + ?Sized + 'd>(
    provider: &P,
) -> Result<Cow<'d, HelloWorldV1<'s>>, DataError>
where
    's: 'd,
{
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
            key: hello_world::key::HELLO_WORLD_V1,
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

#[test]
fn test_warehouse_owned() {
    let warehouse = get_warehouse(DATA);
    let hello_data = get_payload_v1(&warehouse);
    assert!(matches!(
        hello_data,
        Ok(Cow::Owned(HelloWorldV1 {
            message: Cow::Borrowed(_),
        }))
    ));
}

#[test]
fn test_warehouse_owned_dyn_erased() {
    let warehouse = get_warehouse(DATA);
    let hello_data = get_payload_v1(&warehouse as &dyn ErasedDataProvider);
    assert!(matches!(
        hello_data,
        Ok(Cow::Owned(HelloWorldV1 {
            message: Cow::Borrowed(_),
        }))
    ));
}

#[test]
fn test_warehouse_owned_dyn_generic() {
    let warehouse = get_warehouse(DATA);
    let hello_data = get_payload_v1(&warehouse as &dyn DataProvider<HelloWorldV1>);
    assert!(matches!(
        hello_data,
        Ok(Cow::Owned(HelloWorldV1 {
            message: Cow::Borrowed(_),
        }))
    ));
}

#[test]
fn test_warehouse_owned_dyn_erased_alt() {
    let warehouse = get_warehouse(DATA);
    let response = get_payload_alt(&warehouse as &dyn ErasedDataProvider);
    assert!(matches!(
        response,
        Err(DataError::UnsupportedResourceKey { .. })
    ));
}

#[test]
fn test_warehouse_owned_receiver() {
    let warehouse = get_warehouse(DATA);
    let mut receiver: DataReceiver<HelloWorldV1> = get_receiver_v1();
    warehouse
        .load_to_receiver(&get_request_v1(), &mut receiver)
        .unwrap();
    assert!(matches!(
        receiver.payload,
        Some(Cow::Owned(HelloWorldV1 {
            message: Cow::Borrowed(_),
        }))
    ));
}

#[test]
fn test_warehouse_ref() {
    let warehouse = get_warehouse(DATA);
    let hello_data = get_payload_v1(&&warehouse);
    assert!(matches!(
        hello_data,
        Ok(Cow::Borrowed(HelloWorldV1 {
            message: Cow::Borrowed(_),
        }))
    ));
}

#[test]
fn test_warehouse_ref_dyn_erased() {
    let warehouse = get_warehouse(DATA);
    let hello_data = get_payload_v1(&&warehouse as &dyn ErasedDataProvider);
    assert!(matches!(
        hello_data,
        Ok(Cow::Borrowed(HelloWorldV1 {
            message: Cow::Borrowed(_),
        }))
    ));
}

#[test]
fn test_warehouse_ref_dyn_generic() {
    let warehouse = get_warehouse(DATA);
    let hello_data = get_payload_v1(&&warehouse as &dyn DataProvider<HelloWorldV1>);
    assert!(matches!(
        hello_data,
        Ok(Cow::Borrowed(HelloWorldV1 {
            message: Cow::Borrowed(_),
        }))
    ));
}

#[test]
fn test_warehouse_ref_dyn_erased_alt() {
    let warehouse = get_warehouse(DATA);
    let response = get_payload_alt(&&warehouse as &dyn ErasedDataProvider);
    assert!(matches!(
        response,
        Err(DataError::UnsupportedResourceKey { .. })
    ));
}

#[test]
fn test_warehouse_ref_receiver() {
    let warehouse = get_warehouse(DATA);
    let mut receiver: DataReceiver<HelloWorldV1> = get_receiver_v1();
    (&&warehouse)
        .load_to_receiver(&get_request_v1(), &mut receiver)
        .unwrap();
    assert!(matches!(
        receiver.payload,
        Some(Cow::Borrowed(HelloWorldV1 {
            message: Cow::Borrowed(_),
        }))
    ));
}

#[test]
fn test_borrowing() {
    let warehouse = get_warehouse(DATA);
    let provider = DataProviderBorrowing::from(&warehouse);
    let hello_data = get_payload_v1(&provider);
    assert!(matches!(
        hello_data,
        Ok(Cow::Borrowed(HelloWorldV1 {
            message: Cow::Borrowed(_),
        }))
    ));
}

#[test]
fn test_borrowing_dyn_erased() {
    let warehouse = get_warehouse(DATA);
    let provider = DataProviderBorrowing::from(&warehouse);
    let hello_data = get_payload_v1(&provider as &dyn ErasedDataProvider);
    assert!(matches!(
        hello_data,
        Ok(Cow::Borrowed(HelloWorldV1 {
            message: Cow::Borrowed(_),
        }))
    ));
}

#[test]
fn test_borrowing_dyn_erased_alt() {
    let warehouse = get_warehouse(DATA);
    let provider = DataProviderBorrowing::from(&warehouse);
    let hello_data = get_payload_alt(&provider as &dyn ErasedDataProvider);
    assert!(matches!(hello_data, Ok(Cow::Borrowed(HelloAlt { .. }))));
}

#[test]
fn test_borrowing_dyn_generic() {
    let warehouse = get_warehouse(DATA);
    let provider = DataProviderBorrowing::from(&warehouse);
    let hello_data = get_payload_v1(&provider as &dyn DataProvider<HelloWorldV1>);
    assert!(matches!(
        hello_data,
        Ok(Cow::Borrowed(HelloWorldV1 {
            message: Cow::Borrowed(_),
        }))
    ));
}

#[test]
fn test_borrowing_dyn_generic_alt() {
    let warehouse = get_warehouse(DATA);
    let provider = DataProviderBorrowing::from(&warehouse);
    let hello_data = get_payload_alt(&provider as &dyn DataProvider<HelloAlt>);
    assert!(matches!(hello_data, Ok(Cow::Borrowed(HelloAlt { .. }))));
}

#[test]
fn test_mismatched_types() {
    let warehouse = get_warehouse(DATA);
    let provider = DataProviderBorrowing::from(&warehouse);
    // Request is for v2, but type argument is for v1
    let response: Result<DataResponse<HelloWorldV1>, DataError> =
        (&provider as &dyn ErasedDataProvider).load_payload(&get_request_alt());
    assert!(matches!(response, Err(DataError::MismatchedType { .. })));
}

fn check_v1_v2<'d, 's, P>(d: &P)
where
    's: 'd,
    P: DataProvider<'d, HelloWorldV1<'s>> + DataProvider<'d, HelloAlt> + ?Sized,
{
    let v1: Cow<'d, HelloWorldV1<'s>> = d
        .load_payload(&get_request_v1())
        .unwrap()
        .take_payload()
        .unwrap();
    let v2: Cow<'d, HelloAlt> = d
        .load_payload(&get_request_alt())
        .unwrap()
        .take_payload()
        .unwrap();
    if v1.message == v2.message {
        panic!()
    }
}

#[test]
fn test_v1_v2_generic() {
    let warehouse = get_warehouse(DATA);
    let provider = DataProviderBorrowing::from(&warehouse);
    check_v1_v2(&provider);
}

#[test]
fn test_v1_v2_dyn_erased() {
    let warehouse = get_warehouse(DATA);
    let provider = DataProviderBorrowing::from(&warehouse);
    check_v1_v2(&provider as &dyn ErasedDataProvider);
}

#[test]
fn test_local() {
    let local_data = DATA.to_string();
    let warehouse = get_warehouse(&local_data);
    let hello_data = get_payload_v1(&warehouse);
    assert!(matches!(
        hello_data,
        Ok(Cow::Owned(HelloWorldV1 {
            message: Cow::Borrowed(_),
        }))
    ));
}

#[test]
fn test_local_ref() {
    let local_data = DATA.to_string();
    let warehouse = get_warehouse(&local_data);
    let hello_data = get_payload_v1(&&warehouse);
    assert!(matches!(
        hello_data,
        Ok(Cow::Borrowed(HelloWorldV1 {
            message: Cow::Borrowed(_),
        }))
    ));
}

// Note: Local data is not allowed in ErasedDataProvider. How do you test this?
