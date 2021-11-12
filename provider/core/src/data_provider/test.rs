// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::borrow::Cow;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::string::ToString;
use core::fmt::Debug;
use serde::{Deserialize, Serialize};

use super::*;
use crate::erased::*;
use crate::hello_world::{key::HELLO_WORLD_V1, HelloWorldV1, HelloWorldV1Marker};
use crate::prelude::*;
use crate::yoke;

// This file tests DataProvider borrow semantics with a dummy data provider based on a
// JSON string. It also exercises most of the data provider code paths.

/// Key for HelloAlt, used for testing mismatched types
const HELLO_ALT_KEY: ResourceKey = crate::resource_key!(Core, "helloalt", 1);

/// A data struct serialization-compatible with HelloWorldV1 used for testing mismatched types
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Yokeable, ZeroCopyFrom)]
#[yoke(cloning_zcf)]
struct HelloAlt {
    message: String,
}

/// Marker type for [`HelloAlt`].
struct HelloAltMarker {}

impl DataMarker for HelloAltMarker {
    type Yokeable = HelloAlt;
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
struct HelloCombined<'data> {
    #[serde(borrow)]
    pub hello_v1: HelloWorldV1<'data>,
    pub hello_alt: HelloAlt,
}

/// A DataProvider that owns its data, returning an Rc-variant DataPayload.
/// Supports only HELLO_WORLD_V1. Uses `impl_dyn_provider!()`.
#[derive(Debug)]
struct DataWarehouse<'data> {
    hello_v1: Rc<HelloWorldV1<'data>>,
    hello_alt: Rc<HelloAlt>,
}

impl<'data> DataProvider<'data, HelloWorldV1Marker> for DataWarehouse<'data> {
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<'data, HelloWorldV1Marker>, DataError> {
        req.resource_path.key.match_key(HELLO_WORLD_V1)?;
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_partial_owned(self.hello_v1.clone())),
        })
    }
}

crate::impl_dyn_provider!(DataWarehouse<'static>, {
    HELLO_WORLD_V1 => HelloWorldV1Marker,
}, ERASED);

/// A DataProvider that supports both HELLO_WORLD_V1 and HELLO_ALT.
#[derive(Debug)]
struct DataProvider2<'data> {
    data: DataWarehouse<'data>,
}

impl<'data> From<DataWarehouse<'data>> for DataProvider2<'data> {
    fn from(warehouse: DataWarehouse<'data>) -> Self {
        DataProvider2 { data: warehouse }
    }
}

impl<'data> DataProvider<'data, HelloWorldV1Marker> for DataProvider2<'data> {
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<'data, HelloWorldV1Marker>, DataError> {
        req.resource_path.key.match_key(HELLO_WORLD_V1)?;
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_partial_owned(self.data.hello_v1.clone())),
        })
    }
}

impl<'data> DataProvider<'data, HelloAltMarker> for DataProvider2<'data> {
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<'data, HelloAltMarker>, DataError> {
        req.resource_path.key.match_key(HELLO_ALT_KEY)?;
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_partial_owned(self.data.hello_alt.clone())),
        })
    }
}

crate::impl_dyn_provider!(DataProvider2<'static>, {
    HELLO_WORLD_V1 => HelloWorldV1Marker,
    HELLO_ALT_KEY => HelloAltMarker,
}, ERASED);

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
fn get_warehouse<'data>(data: &'data str) -> DataWarehouse<'data> {
    let data: HelloCombined = serde_json::from_str(data).expect("Well-formed data");
    DataWarehouse {
        hello_v1: Rc::from(data.hello_v1),
        hello_alt: Rc::from(data.hello_alt),
    }
}

fn get_payload_v1<'data, P: DataProvider<'data, HelloWorldV1Marker> + ?Sized>(
    provider: &P,
) -> Result<DataPayload<'data, HelloWorldV1Marker>, DataError> {
    provider.load_payload(&get_request_v1())?.take_payload()
}

fn get_payload_alt<'data, P: DataProvider<'data, HelloAltMarker> + ?Sized>(
    d: &P,
) -> Result<DataPayload<'data, HelloAltMarker>, DataError> {
    d.load_payload(&get_request_alt())?.take_payload()
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
    let hello_data = get_payload_v1(&warehouse).unwrap();
    assert!(matches!(hello_data.inner, DataPayloadInner::RcStruct(_)));
    assert!(matches!(
        hello_data.get(),
        HelloWorldV1 {
            message: Cow::Borrowed(_),
        }
    ));
}

#[test]
fn test_warehouse_owned_dyn_erased() {
    let warehouse = get_warehouse(DATA);
    let hello_data = get_payload_v1(&warehouse as &dyn ErasedDataProvider).unwrap();
    assert!(matches!(hello_data.inner, DataPayloadInner::RcStruct(_)));
    assert!(matches!(
        hello_data.get(),
        HelloWorldV1 {
            message: Cow::Borrowed(_),
        }
    ));
}

#[test]
fn test_warehouse_owned_dyn_generic() {
    let warehouse = get_warehouse(DATA);
    let hello_data = get_payload_v1(&warehouse as &dyn DataProvider<HelloWorldV1Marker>).unwrap();
    assert!(matches!(hello_data.inner, DataPayloadInner::RcStruct(_)));
    assert!(matches!(
        hello_data.get(),
        HelloWorldV1 {
            message: Cow::Borrowed(_),
        }
    ));
}

#[test]
fn test_warehouse_owned_dyn_erased_alt() {
    let warehouse = get_warehouse(DATA);
    let response = get_payload_alt(&warehouse as &dyn ErasedDataProvider);
    assert!(matches!(
        response,
        Err(DataError::MissingResourceKey { .. })
    ));
}

#[test]
fn test_provider2() {
    let warehouse = get_warehouse(DATA);
    let provider = DataProvider2::from(warehouse);
    let hello_data = get_payload_v1(&provider).unwrap();
    assert!(matches!(hello_data.inner, DataPayloadInner::RcStruct(_)));
    assert!(matches!(
        hello_data.get(),
        HelloWorldV1 {
            message: Cow::Borrowed(_),
        }
    ));
}

#[test]
fn test_provider2_dyn_erased() {
    let warehouse = get_warehouse(DATA);
    let provider = DataProvider2::from(warehouse);
    let hello_data = get_payload_v1(&provider as &dyn ErasedDataProvider).unwrap();
    assert!(matches!(hello_data.inner, DataPayloadInner::RcStruct(_)));
    assert!(matches!(
        hello_data.get(),
        HelloWorldV1 {
            message: Cow::Borrowed(_),
        }
    ));
}

#[test]
fn test_provider2_dyn_erased_alt() {
    let warehouse = get_warehouse(DATA);
    let provider = DataProvider2::from(warehouse);
    let hello_data = get_payload_alt(&provider as &dyn ErasedDataProvider).unwrap();
    assert!(matches!(hello_data.inner, DataPayloadInner::RcStruct(_)));
    assert!(matches!(hello_data.get(), HelloAlt { .. }));
}

#[test]
fn test_provider2_dyn_generic() {
    let warehouse = get_warehouse(DATA);
    let provider = DataProvider2::from(warehouse);
    let hello_data = get_payload_v1(&provider as &dyn DataProvider<HelloWorldV1Marker>).unwrap();
    assert!(matches!(hello_data.inner, DataPayloadInner::RcStruct(_)));
    assert!(matches!(
        hello_data.get(),
        HelloWorldV1 {
            message: Cow::Borrowed(_),
        }
    ));
}

#[test]
fn test_provider2_dyn_generic_alt() {
    let warehouse = get_warehouse(DATA);
    let provider = DataProvider2::from(warehouse);
    let hello_data = get_payload_alt(&provider as &dyn DataProvider<HelloAltMarker>).unwrap();
    assert!(matches!(hello_data.inner, DataPayloadInner::RcStruct(_)));
    assert!(matches!(hello_data.get(), HelloAlt { .. }));
}

#[test]
fn test_mismatched_types() {
    let warehouse = get_warehouse(DATA);
    let provider = DataProvider2::from(warehouse);
    // Request is for v2, but type argument is for v1
    let response: Result<DataPayload<HelloWorldV1Marker>, DataError> =
        ErasedDataProvider::load_erased(&provider, &get_request_alt())
            .unwrap()
            .take_payload()
            .unwrap()
            .downcast();
    assert!(matches!(response, Err(DataError::MismatchedType { .. })));
}

fn check_v1_v2<'data, P>(d: &P)
where
    P: DataProvider<'data, HelloWorldV1Marker> + DataProvider<'data, HelloAltMarker> + ?Sized,
{
    let v1: DataPayload<'data, HelloWorldV1Marker> = d
        .load_payload(&get_request_v1())
        .unwrap()
        .take_payload()
        .unwrap();
    let v2: DataPayload<'data, HelloAltMarker> = d
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
    let provider = DataProvider2::from(warehouse);
    check_v1_v2(&provider);
}

#[test]
fn test_v1_v2_dyn_erased() {
    let warehouse = get_warehouse(DATA);
    let provider = DataProvider2::from(warehouse);
    check_v1_v2(&provider as &dyn ErasedDataProvider);
}

#[test]
fn test_local() {
    let local_data = DATA.to_string();
    let warehouse = get_warehouse(&local_data);
    let hello_data = get_payload_v1(&warehouse).unwrap();
    assert!(matches!(hello_data.inner, DataPayloadInner::RcStruct(_)));
    assert!(matches!(
        hello_data.get(),
        HelloWorldV1 {
            message: Cow::Borrowed(_),
        }
    ));
}

#[test]
fn test_local_ref() {
    let local_data = DATA.to_string();
    let warehouse = get_warehouse(&local_data);
    let hello_data = get_payload_v1(&warehouse).unwrap();
    assert!(matches!(hello_data.inner, DataPayloadInner::RcStruct(_)));
    assert!(matches!(
        hello_data.get(),
        HelloWorldV1 {
            message: Cow::Borrowed(_),
        }
    ));
}

// Note: Local data is not allowed in ErasedDataProvider. How do you test this?
