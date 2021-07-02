// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider always serving the same struct.

use crate::error::Error;
use crate::prelude::*;
use yoke::*;

/// A data provider that unconditionally returns references to borrowed data.
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
///     data: &local_data,
/// };
///
/// let payload: DataPayload<HelloWorldV1Marker> = provider.load_payload(&DataRequest::from(SAMPLE_KEY))
///     .expect("Load should succeed")
///     .take_payload()
///     .expect("Data should be present");
///
/// assert_eq!(payload.get(), &local_data);
/// ```
pub struct StructProvider<'d, T: ?Sized> {
    pub key: ResourceKey,
    pub data: &'d T,
}

impl<'d, 's, M> DataProvider<'d, 's, M> for StructProvider<'d, M::Cart>
where
    M: DataMarker<'s>,
    M::Yokeable: ZeroCopyFrom<M::Cart>,
{
    fn load_payload(&self, req: &DataRequest) -> Result<DataResponse<'d, 's, M>, Error> {
        req.resource_path.key.match_key(self.key)?;
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_borrowed(self.data)),
        })
    }
}
