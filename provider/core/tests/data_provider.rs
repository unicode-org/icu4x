// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::Debug;

use icu_provider::erased::*;
use icu_provider::hello_world::{key::HELLO_WORLD_V1, HelloWorldV1, HelloWorldV1Helper};
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

struct HelloAltHelper {}
impl DataStructHelperTrait for HelloAltHelper {
    type Yokeable = HelloAlt;
}
unsafe impl<'a> icu_provider::yoke::Yokeable<'a> for HelloAlt {
    type Output = HelloAlt;
    fn transform(&'a self) -> &'a Self::Output {
        self
    }
    unsafe fn make(from: Self::Output) -> Self {
        from
    }
    fn with_mut<F>(&'a mut self, f: F)
    where
        F: 'static + for<'b> FnOnce(&'b mut Self::Output),
    {
        f(self)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
struct HelloCombined<'s> {
    #[serde(borrow)]
    pub hello_v1: HelloWorldV1<'s>,
    pub hello_alt: HelloAlt,
}

/// A DataProvider that owns its data. DataProvider is implemented on `DataWarehouse`, returning
/// owned data, and on `&'d DataWarehouse`, returning borrowed data. Both support only
/// HELLO_WORLD_V1 and use `impl_dyn_provider!()`.
#[derive(Debug)]
struct DataWarehouse<'s> {
    data: HelloCombined<'s>,
}

impl<'d, 's> DataProvider<'d, 's, HelloWorldV1Helper> for DataWarehouse<'s> {
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<'d, 's, HelloWorldV1Helper>, DataError> {
        req.resource_path.key.match_key(HELLO_WORLD_V1)?;
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_partial_owned(self.data.hello_v1.clone())),
        })
    }
}

icu_provider::impl_dyn_provider!(DataWarehouse<'static>, {
    HELLO_WORLD_V1 => HelloWorldV1Helper,
}, ERASED, 'd, 's);

impl<'d, 's> DataProvider<'d, 's, HelloWorldV1Helper> for &'d DataWarehouse<'s> {
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<'d, 's, HelloWorldV1Helper>, DataError> {
        req.resource_path.key.match_key(HELLO_WORLD_V1)?;
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_borrowed(&self.data.hello_v1)),
        })
    }
}

icu_provider::impl_dyn_provider!(&'d DataWarehouse<'static>, {
    HELLO_WORLD_V1 => HelloWorldV1Helper,
}, ERASED, 'd, 's);

/// A DataProvider that returns borrowed data. Supports both HELLO_WORLD_V1 and HELLO_ALT.
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

impl<'d, 's> DataProvider<'d, 's, HelloWorldV1Helper> for DataProviderBorrowing<'d, 's> {
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<'d, 's, HelloWorldV1Helper>, DataError> {
        req.resource_path.key.match_key(HELLO_WORLD_V1)?;
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_borrowed(&self.borrowed_data.hello_v1)),
        })
    }
}

impl<'d, 's> DataProvider<'d, 's, HelloAltHelper> for DataProviderBorrowing<'d, 's> {
    fn load_payload(&self, req: &DataRequest) -> Result<DataResponse<'d, 's, HelloAltHelper>, DataError> {
        req.resource_path.key.match_key(HELLO_ALT_KEY)?;
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_borrowed(&self.borrowed_data.hello_alt)),
        })
    }
}

icu_provider::impl_dyn_provider!(DataProviderBorrowing<'d, 'static>, {
    HELLO_WORLD_V1 => HelloWorldV1Helper,
    HELLO_ALT_KEY => HelloAltHelper,
}, ERASED, 'd, 's);

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

fn get_payload_v1<'d, 's, P: DataProvider<'d, 's, HelloWorldV1Helper> + ?Sized + 'd>(
    provider: &P,
) -> Result<Cow<'d, HelloWorldV1<'s>>, DataError>
{
    provider
        .load_payload(&get_request_v1())?
        .take_payload()
        .map(|p| p.into_cow())
}

fn get_payload_alt<'d, P: DataProvider<'d, 'static, HelloAltHelper> + ?Sized>(
    d: &P,
) -> Result<Cow<'d, HelloAlt>, DataError> {
    d.load_payload(&get_request_alt())?
        .take_payload()
        .map(|p| p.into_cow())
}

fn get_request_v1() -> DataRequest {
    DataRequest {
        resource_path: ResourcePath {
            key: HELLO_WORLD_V1,
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
    let hello_data = get_payload_v1(&warehouse as &dyn DataProvider<HelloWorldV1Helper>);
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
    let hello_data = get_payload_v1(&&warehouse as &dyn DataProvider<HelloWorldV1Helper>);
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
    let hello_data = get_payload_v1(&provider as &dyn DataProvider<HelloWorldV1Helper>);
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
    let hello_data = get_payload_alt(&provider as &dyn DataProvider<HelloAltHelper>);
    assert!(matches!(hello_data, Ok(Cow::Borrowed(HelloAlt { .. }))));
}

#[test]
fn test_mismatched_types() {
    let warehouse = get_warehouse(DATA);
    let provider = DataProviderBorrowing::from(&warehouse);
    // Request is for v2, but type argument is for v1
    let response: Result<DataPayload<HelloWorldV1Helper>, DataError> =
        ErasedDataProvider::load_erased(&provider, &get_request_alt())
            .unwrap()
            .take_payload()
            .unwrap()
            .downcast();
    assert!(matches!(response, Err(DataError::MismatchedType { .. })));
}

fn check_v1_v2<'d, 's, P>(d: &P)
where
    's: 'd,
    P: DataProvider<'d, 's, HelloWorldV1Helper> + DataProvider<'d, 's, HelloAltHelper> + ?Sized,
{
    let v1: DataPayload<'d, 's, HelloWorldV1Helper> = d
        .load_payload(&get_request_v1())
        .unwrap()
        .take_payload()
        .unwrap();
    let v2: DataPayload<'d, 's, HelloAltHelper> = d
        .load_payload(&get_request_alt())
        .unwrap()
        .take_payload()
        .unwrap();
    if v1.get().message == v2.get().message {
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
