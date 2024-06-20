// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Collection of iteration APIs for data providers.

use std::collections::HashSet;

use crate::prelude::*;

/// A [`DynamicDataProvider`] that can iterate over all supported [`DataLocale`] for a certain marker.
///
/// The provider is not allowed to return `Ok` for requests that were not returned by `iter_requests`,
/// and must not fail with a [`DataErrorKind::MissingLocale`] for requests that were returned.
pub trait IterableDynamicDataProvider<M: DynamicDataMarker>: DynamicDataProvider<M> {
    /// Given a [`DataMarkerInfo`], returns a list of [`DataLocale`].
    fn iter_requests_for_marker(
        &self,
        marker: DataMarkerInfo,
    ) -> Result<HashSet<(DataLocale, DataMarkerAttributes)>, DataError>;
}

/// A [`DataProvider`] that can iterate over all supported [`DataLocale`] for a certain marker.
///
/// The provider is not allowed to return `Ok` for requests that were not returned by `iter_requests`,
/// and must not fail with a [`DataErrorKind::MissingLocale`] for requests that were returned.
pub trait IterableDataProvider<M: DataMarker>: DataProvider<M> {
    /// Returns a list of [`DataLocale`].
    fn iter_requests(&self) -> Result<HashSet<(DataLocale, DataMarkerAttributes)>, DataError>;
}

impl<M, P> IterableDynamicDataProvider<M> for Box<P>
where
    M: DynamicDataMarker,
    P: IterableDynamicDataProvider<M> + ?Sized,
{
    fn iter_requests_for_marker(
        &self,
        marker: DataMarkerInfo,
    ) -> Result<HashSet<(DataLocale, DataMarkerAttributes)>, DataError> {
        (**self).iter_requests_for_marker(marker)
    }
}
