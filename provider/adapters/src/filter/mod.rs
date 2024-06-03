// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Providers that filter resource requests.
//!
//! Requests that fail a filter test will return [`DataError`] of kind [`FilteredResource`](
//! DataErrorKind::FilteredResource) and will not appear in [`IterableDynamicDataProvider`] iterators.
//!
//! The main struct is [`RequestFilterDataProvider`]. Although that struct can be created
//! directly, the traits in this module provide helper functions for common filtering patterns.
//!
//! To create a `RequestFilterDataProvider`, you can use the [`Filterable`] blanket function:
//!
//! ```
//! use icu_provider_adapters::filter::Filterable;
//!
//! // now call .filterable() on any object to get a RequestFilterDataProvider
//! ```
//!
//! # Examples
//!
//! ```
//! use icu_locale_core::subtags::language;
//! use icu_provider::hello_world::*;
//! use icu_provider::prelude::*;
//! use icu_provider_adapters::filter::Filterable;
//!
//! // Only return German data from a HelloWorldProvider:
//! HelloWorldProvider
//!     .filterable("Demo German-only filter")
//!     .filter_by_langid(|langid| langid.language == language!("de"));
//! ```
//!
//! [`IterableDynamicDataProvider`]: icu_provider::datagen::IterableDynamicDataProvider

mod impls;

#[cfg(feature = "datagen")]
use icu_provider::datagen;
use icu_provider::prelude::*;

/// A data provider that selectively filters out data requests.
///
/// Data requests that are rejected by the filter will return a [`DataError`] with kind
/// [`FilteredResource`](DataErrorKind::FilteredResource), and they will not be returned
/// by [`datagen::IterableDynamicDataProvider::supported_requests_for_marker`].
///
/// Although this struct can be created directly, the traits in this module provide helper
/// functions for common filtering patterns.
#[allow(clippy::exhaustive_structs)] // this type is stable
#[derive(Debug)]
pub struct RequestFilterDataProvider<D, F>
where
    F: Fn(DataRequest) -> bool,
{
    /// The data provider to which we delegate requests.
    pub inner: D,

    /// The predicate function. A return value of `true` indicates that the request should
    /// proceed as normal; a return value of `false` will reject the request.
    pub predicate: F,

    /// A name for this filter, used in error messages.
    pub filter_name: &'static str,
}

impl<D, F, M> DynamicDataProvider<M> for RequestFilterDataProvider<D, F>
where
    F: Fn(DataRequest) -> bool,
    M: DynDataMarker,
    D: DynamicDataProvider<M>,
{
    fn load_data(
        &self,
        marker: DataMarkerInfo,
        req: DataRequest,
    ) -> Result<DataResponse<M>, DataError> {
        if (self.predicate)(req) {
            self.inner.load_data(marker, req)
        } else {
            Err(DataErrorKind::FilteredResource
                .with_str_context(self.filter_name)
                .with_req(marker, req))
        }
    }
}

impl<D, F, M> DataProvider<M> for RequestFilterDataProvider<D, F>
where
    F: Fn(DataRequest) -> bool,
    M: DataMarker,
    D: DataProvider<M>,
{
    fn load(&self, req: DataRequest) -> Result<DataResponse<M>, DataError> {
        if (self.predicate)(req) {
            self.inner.load(req)
        } else {
            Err(DataErrorKind::FilteredResource
                .with_str_context(self.filter_name)
                .with_req(M::INFO, req))
        }
    }
}

impl<D, F> AnyProvider for RequestFilterDataProvider<D, F>
where
    F: Fn(DataRequest) -> bool,
    D: AnyProvider,
{
    fn load_any(&self, marker: DataMarkerInfo, req: DataRequest) -> Result<AnyResponse, DataError> {
        if (self.predicate)(req) {
            self.inner.load_any(marker, req)
        } else {
            Err(DataErrorKind::FilteredResource
                .with_str_context(self.filter_name)
                .with_req(marker, req))
        }
    }
}

#[cfg(feature = "datagen")]
impl<M, D, F> datagen::IterableDynamicDataProvider<M> for RequestFilterDataProvider<D, F>
where
    M: DynDataMarker,
    F: Fn(DataRequest) -> bool,
    D: datagen::IterableDynamicDataProvider<M>,
{
    fn supported_requests_for_marker(
        &self,
        marker: DataMarkerInfo,
    ) -> Result<std::collections::HashSet<(DataLocale, DataMarkerAttributes)>, DataError> {
        self.inner.supported_requests_for_marker(marker).map(|set| {
            // Use filter_map instead of filter to avoid cloning the locale
            set.into_iter()
                .filter(|(locale, marker_attributes)| {
                    (self.predicate)(DataRequest {
                        locale,
                        marker_attributes,
                        ..Default::default()
                    })
                })
                .collect()
        })
    }
}

#[cfg(feature = "datagen")]
impl<M, D, F> datagen::IterableDataProvider<M> for RequestFilterDataProvider<D, F>
where
    M: DataMarker,
    F: Fn(DataRequest) -> bool,
    D: datagen::IterableDataProvider<M>,
{
    fn supported_requests(
        &self,
    ) -> Result<std::collections::HashSet<(DataLocale, DataMarkerAttributes)>, DataError> {
        self.inner.supported_requests().map(|vec| {
            // Use filter_map instead of filter to avoid cloning the locale
            vec.into_iter()
                .filter(|(locale, marker_attributes)| {
                    (self.predicate)(DataRequest {
                        locale,
                        marker_attributes,
                        ..Default::default()
                    })
                })
                .collect()
        })
    }
}

#[cfg(feature = "datagen")]
impl<D, F, MFrom, MTo> datagen::DataConverter<MFrom, MTo> for RequestFilterDataProvider<D, F>
where
    D: datagen::DataConverter<MFrom, MTo>,
    MFrom: DynDataMarker,
    MTo: DynDataMarker,
    F: Fn(DataRequest) -> bool,
{
    fn convert(
        &self,
        marker: DataMarkerInfo,
        from: DataPayload<MFrom>,
    ) -> Result<DataPayload<MTo>, (DataPayload<MFrom>, DataError)> {
        // Conversions are type-agnostic
        self.inner.convert(marker, from)
    }
}

/// A blanket-implemented trait exposing the [`Self::filterable()`] function.
///
/// For more details, see [`icu_provider_adapters::filter`](crate::filter).
pub trait Filterable: Sized {
    /// Creates a filterable data provider with the given name for debugging.
    ///
    /// For more details, see [`icu_provider_adapters::filter`](crate::filter).
    fn filterable(
        self,
        filter_name: &'static str,
    ) -> RequestFilterDataProvider<Self, fn(DataRequest) -> bool>;
}

impl<T> Filterable for T
where
    T: Sized,
{
    fn filterable(
        self,
        filter_name: &'static str,
    ) -> RequestFilterDataProvider<Self, fn(DataRequest) -> bool> {
        fn noop(_: DataRequest) -> bool {
            true
        }
        RequestFilterDataProvider {
            inner: self,
            predicate: noop,
            filter_name,
        }
    }
}
