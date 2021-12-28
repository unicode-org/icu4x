// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider always serving the same struct.

use crate::prelude::*;
use crate::yoke::trait_hack::YokeTraitHack;
use crate::yoke::*;

/// A data provider that returns clones of a constant data payload.
///
/// # Examples
///
/// ```
/// use icu_provider::prelude::*;
/// use icu_provider::hello_world::*;
/// use icu_provider::struct_provider::StructProvider;
/// use std::borrow::Cow;
///
/// let local_data = HelloWorldV1 {
///     message: Cow::Owned("hello world".to_string()),
/// };
///
/// // A placeholder key to use to serve the data struct
/// const SAMPLE_KEY: ResourceKey = icu_provider::resource_key!(x, "xyz", "example", 1);
///
/// let provider = StructProvider {
///     key: SAMPLE_KEY,
///     data: DataPayload::from_owned(local_data),
/// };
///
/// let payload: DataPayload<HelloWorldV1Marker> = provider.load_payload(&DataRequest::from(SAMPLE_KEY))
///     .expect("Load should succeed")
///     .take_payload()
///     .expect("Data should be present");
///
/// assert_eq!(payload.get().message, "hello world");
/// ```
pub struct StructProvider<M>
where
    M: DataMarker,
{
    pub key: ResourceKey,
    pub data: DataPayload<M>,
}

impl<M> DataProvider<M> for StructProvider<M>
where
    M: DataMarker,
    for<'a> YokeTraitHack<<M::Yokeable as Yokeable<'a>>::Output>: Clone,
{
    fn load_payload(&self, req: &DataRequest) -> Result<DataResponse<M>, DataError> {
        req.resource_path.key.match_key(self.key)?;
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(self.data.clone()),
        })
    }
}
