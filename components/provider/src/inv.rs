// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::error::Error;
use crate::iter::IterableDataProvider;
use crate::prelude::*;
use std::borrow::Cow;
use std::fmt::Debug;

/// A locale-invariant data provider. Sometimes useful for testing. Not intended to be used in
/// production environments.
///
/// The objects returned by `InvariantDataProvider` are guaranteed to conform to the correct struct
/// definition, so `InvariantDataProvider` can also be used to validate unknown data providers.
///
/// # Example
///
/// ```
/// use icu_provider::prelude::*;
/// use icu_provider::inv::InvariantDataProvider;
///
/// let provider = InvariantDataProvider;
/// let expected_entries = vec![ResourceOptions::default()];
/// let actual_entries: Vec<ResourceOptions> = provider
///     .supported_options_for_key(&icu_plurals::provider::key::CARDINAL_V1)
///     .unwrap()
///     .collect();
/// assert_eq!(&expected_entries, &actual_entries);
/// ```
pub struct InvariantDataProvider;

impl<'d, T> DataProvider<'d, T> for InvariantDataProvider
where
    T: Clone + Debug + Default + 'd,
{
    fn load_payload(&self, _req: &DataRequest) -> Result<DataResponse<'d, T>, Error> {
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(Cow::Owned(T::default())),
        })
    }
}

#[cfg(feature = "erased")]
impl<'d> crate::erased::ErasedDataProvider<'d> for InvariantDataProvider {
    fn load_to_receiver(
        &self,
        _req: &DataRequest,
        receiver: &mut dyn crate::erased::ErasedDataReceiver<'d>,
    ) -> Result<DataResponseMetadata, Error> {
        receiver.receive_default()?;
        Ok(DataResponseMetadata::default())
    }
}

impl IterableDataProvider<'_> for InvariantDataProvider {
    fn supported_options_for_key(
        &self,
        _resc_key: &ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, Error> {
        let list: Vec<ResourceOptions> = vec![ResourceOptions::default()];
        Ok(Box::new(list.into_iter()))
    }
}

#[cfg(all(feature = "hello_world", feature = "erased"))]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::erased::ErasedDataProvider;

    #[test]
    fn test_invariant() {
        use crate::hello_world;
        let provider = InvariantDataProvider;

        let data1: Cow<hello_world::HelloWorldV1> = provider
            .load_payload(&DataRequest::from(hello_world::key::HELLO_WORLD_V1))
            .unwrap()
            .take_payload()
            .unwrap();

        let data2: Cow<hello_world::HelloWorldV1> = (&provider as &dyn ErasedDataProvider)
            .load_payload(&DataRequest::from(hello_world::key::HELLO_WORLD_V1))
            .unwrap()
            .take_payload()
            .unwrap();

        assert_eq!(
            &*data1,
            &hello_world::HelloWorldV1 {
                message: Cow::Borrowed("(und) Hello World")
            }
        );

        assert_eq!(data1, data2);
    }
}
