// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider always serving the same struct.

use crate::prelude::*;

/// A data provider that returns clones of a constant type-erased payload.
///
/// # Examples
///
/// ```
/// use icu_provider::prelude::*;
/// use icu_provider::hello_world::*;
/// use icu_provider::struct_provider::AnyPayloadProvider;
/// use std::borrow::Cow;
///
/// const CONST_DATA: HelloWorldV1<'static> = HelloWorldV1 {
///     message: Cow::Borrowed("hello world"),
/// };
///
/// // A placeholder key to use to serve the data struct
/// const SAMPLE_KEY: ResourceKey = icu_provider::resource_key!("xyz/example@1");
///
/// let provider = AnyPayloadProvider {
///     key: SAMPLE_KEY,
///     data: AnyPayload::from_static_ref(&CONST_DATA),
/// };
///
/// let payload: DataPayload<HelloWorldV1Marker> = provider.load_any(&DataRequest::from(SAMPLE_KEY))
///     .expect("Load should succeed")
///     .downcast()
///     .expect("Types should match")
///     .take_payload()
///     .expect("Data should be present");
///
/// assert_eq!(payload.get().message, "hello world");
/// ```
pub struct AnyPayloadProvider {
    pub key: ResourceKey,
    pub data: AnyPayload,
}

impl AnyProvider for AnyPayloadProvider {
    fn load_any(&self, key: ResourceKey, _: &DataRequest) -> Result<AnyResponse, DataError> {
        key.match_key(self.key)?;
        Ok(AnyResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(self.data.clone()),
        })
    }
}
