// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Collection of iteration APIs for data providers.

use std::collections::HashSet;

use crate::prelude::*;

/// A [`DynamicDataProvider`] that can iterate over all supported [`DataLocale`] for a certain key.
///
/// Implementing this trait means that a data provider knows all of the data it can successfully
/// return from a load request.
pub trait IterableDynamicDataProvider<M: DynamicDataMarker>: DynamicDataProvider<M> {
    /// Given a [`DataKey`], returns a list of [`DataLocale`].
    fn supported_requests_for_key(
        &self,
        key: DataKey,
    ) -> Result<HashSet<(DataLocale, DataKeyAttributes)>, DataError>;
}

/// A [`DataProvider`] that can iterate over all supported [`DataLocale`] for a certain key.
///
/// Implementing this trait means that a data provider knows all of the data it can successfully
/// return from a load request.
pub trait IterableDataProvider<M: DataMarker>: DataProvider<M> {
    /// Returns a list of [`DataLocale`].
    fn supported_requests(&self) -> Result<HashSet<(DataLocale, DataKeyAttributes)>, DataError>;
    /// Returns whether a [`DataLocale`] is in the supported locales list.
    fn supports_request(
        &self,
        locale: &DataLocale,
        key_attributes: &DataKeyAttributes,
    ) -> Result<bool, DataError> {
        self.supported_requests()
            .map(|v| v.contains(&(locale.clone(), key_attributes.clone())))
    }
}

impl<M, P> IterableDynamicDataProvider<M> for Box<P>
where
    M: DynamicDataMarker,
    P: IterableDynamicDataProvider<M> + ?Sized,
{
    fn supported_requests_for_key(
        &self,
        key: DataKey,
    ) -> Result<HashSet<(DataLocale, DataKeyAttributes)>, DataError> {
        (**self).supported_requests_for_key(key)
    }
}
