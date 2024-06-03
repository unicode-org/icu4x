// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Helpers for switching between multiple providers.

use icu_provider::prelude::*;

#[cfg(feature = "datagen")]
use icu_provider::datagen;

/// A provider that is one of two types determined at runtime.
///
/// Data provider traits implemented by both `P0` and `P1` are implemented on
/// `EitherProvider<P0, P1>`.
#[allow(clippy::exhaustive_enums)] // this is stable
#[derive(Debug)]
pub enum EitherProvider<P0, P1> {
    /// A value of type `P0`.
    A(P0),
    /// A value of type `P1`.
    B(P1),
}

impl<P0: AnyProvider, P1: AnyProvider> AnyProvider for EitherProvider<P0, P1> {
    #[inline]
    fn load_any(&self, marker: DataMarkerInfo, req: DataRequest) -> Result<AnyResponse, DataError> {
        use EitherProvider::*;
        match self {
            A(p) => p.load_any(marker, req),
            B(p) => p.load_any(marker, req),
        }
    }
}

impl<M: DynDataMarker, P0: DynamicDataProvider<M>, P1: DynamicDataProvider<M>>
    DynamicDataProvider<M> for EitherProvider<P0, P1>
{
    #[inline]
    fn load_data(
        &self,
        marker: DataMarkerInfo,
        req: DataRequest,
    ) -> Result<DataResponse<M>, DataError> {
        use EitherProvider::*;
        match self {
            A(p) => p.load_data(marker, req),
            B(p) => p.load_data(marker, req),
        }
    }
}

impl<M: DataMarker, P0: DataProvider<M>, P1: DataProvider<M>> DataProvider<M>
    for EitherProvider<P0, P1>
{
    #[inline]
    fn load(&self, req: DataRequest) -> Result<DataResponse<M>, DataError> {
        use EitherProvider::*;
        match self {
            A(p) => p.load(req),
            B(p) => p.load(req),
        }
    }
}

#[cfg(feature = "datagen")]
impl<
        M: DynDataMarker,
        P0: datagen::IterableDynamicDataProvider<M>,
        P1: datagen::IterableDynamicDataProvider<M>,
    > datagen::IterableDynamicDataProvider<M> for EitherProvider<P0, P1>
{
    #[inline]
    fn supported_requests_for_marker(
        &self,
        marker: DataMarkerInfo,
    ) -> Result<std::collections::HashSet<(DataLocale, DataMarkerAttributes)>, DataError> {
        use EitherProvider::*;
        match self {
            A(p) => p.supported_requests_for_marker(marker),
            B(p) => p.supported_requests_for_marker(marker),
        }
    }
}

#[cfg(feature = "datagen")]
impl<M: DataMarker, P0: datagen::IterableDataProvider<M>, P1: datagen::IterableDataProvider<M>>
    datagen::IterableDataProvider<M> for EitherProvider<P0, P1>
{
    #[inline]
    fn supported_requests(
        &self,
    ) -> Result<std::collections::HashSet<(DataLocale, DataMarkerAttributes)>, DataError> {
        use EitherProvider::*;
        match self {
            A(p) => p.supported_requests(),
            B(p) => p.supported_requests(),
        }
    }
}
