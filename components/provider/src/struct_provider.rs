// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use crate::error::Error;
use crate::prelude::*;
use std::borrow::Cow;
use std::fmt::Debug;

/// A data provider that unconditionally returns references to borrowed data.
///
/// # Example
///
/// ```
/// use icu_provider::prelude::*;
/// use icu_provider::struct_provider::StructProvider;
/// use std::borrow::Cow;
/// use std::default::Default;
///
/// #[derive(Clone, Debug, PartialEq)]
/// struct SampleDataStruct<'s> {
///     value: &'s str,
/// }
///
/// let local_data = SampleDataStruct {
///     value: &"hello world".to_string(),
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
/// let payload: Cow<SampleDataStruct> = provider.load_payload(&DataRequest::from(SAMPLE_KEY))
///     .expect("Load should succeed")
///     .payload
///     .expect("Data should be present");
///
/// assert_eq!(*payload, local_data);
/// assert!(matches!(payload, Cow::Borrowed(_)))
/// ```
pub struct StructProvider<'d, T> {
    pub key: ResourceKey,
    pub data: &'d T,
}

impl<'d, T> DataProvider<'d, T> for StructProvider<'d, T>
where
    T: Clone + Debug + Sized + 'd,
{
    fn load_payload(&self, req: &DataRequest) -> Result<DataResponse<'d, T>, Error> {
        if req.resource_path.key == self.key {
            Ok(DataResponse {
                metadata: DataResponseMetadata::default(),
                payload: Some(Cow::Borrowed(self.data)),
            })
        } else {
            Err(Error::UnsupportedResourceKey(req.resource_path.key))
        }
    }
}
