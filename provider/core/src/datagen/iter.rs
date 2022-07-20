// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Collection of iteration APIs for data providers.

use crate::prelude::*;

/// A [`DynamicDataProvider`] that can iterate over all supported [`DataOptions`] for a certain key.
///
/// Implementing this trait means that a data provider knows all of the data it can successfully
/// return from a load request.
pub trait IterableDynamicDataProvider<M: DataMarker>: DynamicDataProvider<M> {
    /// Given a [`DataKey`], returns a list of [`DataOptions`].
    fn supported_options_for_key(&self, key: DataKey) -> Result<Vec<DataOptions>, DataError>;
}

/// A [`DataProvider`] that can iterate over all supported [`DataOptions`] for a certain key.
///
/// Implementing this trait means that a data provider knows all of the data it can successfully
/// return from a load request.
pub trait IterableDataProvider<M: KeyedDataMarker>: DataProvider<M> {
    /// Returns a list of [`DataOptions`].
    fn supported_options(&self) -> Result<Vec<DataOptions>, DataError>;
}

impl<M, P> IterableDynamicDataProvider<M> for Box<P>
where
    M: DataMarker,
    P: IterableDynamicDataProvider<M> + ?Sized,
{
    fn supported_options_for_key(&self, key: DataKey) -> Result<Vec<DataOptions>, DataError> {
        (**self).supported_options_for_key(key)
    }
}
