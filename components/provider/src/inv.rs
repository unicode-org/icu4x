// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use crate::erased::*;
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
/// use icu_provider::structs;
/// use icu_provider::InvariantDataProvider;
///
/// let provider = InvariantDataProvider;
/// let expected_entries = vec![ResourceOptions::default()];
/// let actual_entries: Vec<ResourceOptions> = provider
///     .supported_options_for_key(&structs::plurals::key::CARDINAL_V1)
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

impl<'d> ErasedDataProvider<'d> for InvariantDataProvider {
    fn load_to_receiver(
        &self,
        _req: &DataRequest,
        receiver: &mut dyn ErasedDataReceiver<'d, '_>,
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

#[test]
fn test_receiver() {
    use crate::structs;
    let provider = InvariantDataProvider;
    let mut receiver = DataReceiver::<structs::icu4x::HelloV1>::new();
    provider
        .load_to_receiver(
            &DataRequest::from(structs::icu4x::key::HELLO_V1),
            &mut receiver,
        )
        .unwrap();
    let plurals_data: &structs::icu4x::HelloV1 = &receiver.take_payload().unwrap();
    assert_eq!(
        plurals_data,
        &structs::icu4x::HelloV1 {
            hello: Cow::Borrowed("(und) Hello World")
        }
    );
}
