// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Collection of iteration APIs for data providers.

use std::collections::HashSet;

use crate::prelude::*;

/// A [`DynamicDataProvider`] that can iterate over all supported [`LanguageIdentifier`] for a certain key.
///
/// Implementing this trait means that a data provider knows all of the data it can successfully
/// return from a load request.
pub trait IterableDynamicDataProvider<M: DataMarker>: DynamicDataProvider<M> {
    /// Given a [`DataKey`], returns a list of [`LanguageIdentifier`].
    fn supported_requests_for_key(
        &self,
        key: DataKey,
    ) -> Result<HashSet<(LanguageIdentifier, DataKeyAttributes)>, DataError>;
}

/// A [`DataProvider`] that can iterate over all supported [`LanguageIdentifier`] for a certain key.
///
/// Implementing this trait means that a data provider knows all of the data it can successfully
/// return from a load request.
pub trait IterableDataProvider<M: KeyedDataMarker>: DataProvider<M> {
    /// Returns a list of [`LanguageIdentifier`].
    fn supported_requests(
        &self,
    ) -> Result<HashSet<(LanguageIdentifier, DataKeyAttributes)>, DataError>;
    /// Returns whether a [`LanguageIdentifier`] is in the supported locales list.
    fn supports_request(
        &self,
        langid: &LanguageIdentifier,
        key_attributes: &DataKeyAttributes,
    ) -> Result<bool, DataError> {
        self.supported_requests()
            .map(|v| v.contains(&(langid.clone(), key_attributes.clone())))
    }
}

impl<M, P> IterableDynamicDataProvider<M> for Box<P>
where
    M: DataMarker,
    P: IterableDynamicDataProvider<M> + ?Sized,
{
    fn supported_requests_for_key(
        &self,
        key: DataKey,
    ) -> Result<HashSet<(LanguageIdentifier, DataKeyAttributes)>, DataError> {
        (**self).supported_requests_for_key(key)
    }
}
