// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Collection of iteration APIs for data providers.

use crate::prelude::*;
use icu_locid::Locale;

/// A [`DynProvider`] that can iterate over all supported [`Locale`]s for a certain key.
///
/// Implementing this trait means that a data provider knows all of the data it can successfully
/// return from a load request.
pub trait IterableDynProvider<M: DataMarker>: DynProvider<M> {
    /// Given a [`ResourceKey`], returns a list of [`Locale`]s.
    fn supported_locales_for_key(&self, key: ResourceKey) -> Result<Vec<Locale>, DataError>;
}

/// A [`ResourceProvider`] that can iterate over all supported [`Locale`]s for a certain key.
///
/// Implementing this trait means that a data provider knows all of the data it can successfully
/// return from a load request.
pub trait IterableResourceProvider<M: ResourceMarker>: ResourceProvider<M> {
    /// Returns a list of [`Locale`]s.
    fn supported_locales(&self) -> Result<Vec<Locale>, DataError>;
}

impl<M, P> IterableDynProvider<M> for Box<P>
where
    M: DataMarker,
    P: IterableDynProvider<M> + ?Sized,
{
    fn supported_locales_for_key(&self, key: ResourceKey) -> Result<Vec<Locale>, DataError> {
        (**self).supported_locales_for_key(key)
    }
}
