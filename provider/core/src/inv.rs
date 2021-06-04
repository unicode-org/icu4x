// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Locale-invariant data provider that requires no I/O.

use crate::error::Error;
use crate::iter::IterableDataProviderCore;
use crate::prelude::*;
use std::rc::Rc;
use yoke::*;

/// A locale-invariant data provider. Sometimes useful for testing. Not intended to be used in
/// production environments.
///
/// The objects returned by [`InvariantDataProvider`] are guaranteed to conform to the correct struct
/// definition, so [`InvariantDataProvider`] can also be used to validate unknown data providers.
///
/// # Examples
///
/// ```
/// use icu_provider::prelude::*;
/// use icu_provider::inv::InvariantDataProvider;
/// use icu_provider::hello_world::{key, HelloWorldV1Marker};
/// use std::borrow::Cow;
///
/// let provider = InvariantDataProvider;
/// let result: DataPayload<HelloWorldV1Marker> = provider
///     .load_payload(&DataRequest::from(key::HELLO_WORLD_V1))
///     .unwrap()
///     .take_payload()
///     .unwrap();
///
/// assert_eq!("(und) Hello World", result.get().message);
/// ```
pub struct InvariantDataProvider;

impl<'d, 's, M> DataProvider<'d, 's, M> for InvariantDataProvider
where
    M: DataMarker<'s>,
    M::Cart: Default,
    M::Yokeable: ZeroCopyFrom<M::Cart>,
{
    fn load_payload(&self, _req: &DataRequest) -> Result<DataResponse<'d, 's, M>, Error> {
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_partial_owned(
                Rc::from(M::Cart::default()),
            )),
        })
    }
}

impl IterableDataProviderCore for InvariantDataProvider {
    fn supported_options_for_key(
        &self,
        _resc_key: &ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, Error> {
        let list: Vec<ResourceOptions> = vec![ResourceOptions::default()];
        Ok(Box::new(list.into_iter()))
    }
}
