// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Providers that filter resource requests.
//!
//! Requests that fail a filter test will return [`DataError`] of kind [`FilteredResource`](
//! DataErrorKind::FilteredResource) and will not appear in [`IterableDynProvider`] iterators.
//!
//! The main struct is [`RequestFilterDataProvider`]. Although that struct can be created
//! directly, the traits in this module provide helper functions for common filtering patterns.
//!
//! To create a `RequestFilterDataProvider`, you can use the [`Filterable`] blanket function:
//!
//! ```
//! use icu_provider::filter::Filterable;
//!
//! // now call .filterable() on any object to get a RequestFilterDataProvider
//! ```
//!
//! # Examples
//!
//! ```
//! use icu_provider::prelude::*;
//! use icu_provider::hello_world::*;
//! use icu_provider::filter::Filterable;
//! use icu_locid::language;
//!
//! // Only return German data from a HelloWorldProvider:
//! HelloWorldProvider::new_with_placeholder_data()
//!     .filterable("Demo German-only filter")
//!     .filter_by_langid(|langid| langid.language == language!("de"));
//! ```
//!
//! [`IterableDynProvider`]: crate::iter::IterableDynProvider

mod impls;

pub use impls::*;

use crate::iter::*;
use crate::prelude::*;
use alloc::boxed::Box;

/// A data provider that selectively filters out data requests.
///
/// Data requests that are rejected by the filter will return a [`DataError`] with kind
/// [`FilteredResource`](DataErrorKind::FilteredResource), and they will not be returned
/// by [`IterableDynProvider::supported_options_for_key`].
///
/// Although this struct can be created directly, the traits in this module provide helper
/// functions for common filtering patterns.
pub struct RequestFilterDataProvider<D, F>
where
    F: Fn(&DataRequest) -> bool,
{
    /// The data provider to which we delegate requests.
    pub inner: D,

    /// The predicate function. A return value of `true` indicates that the request should
    /// proceed as normal; a return value of `false` will reject the request.
    pub predicate: F,

    /// A name for this filter, used in error messages.
    pub filter_name: &'static str,
}

impl<D, F, M> DynProvider<M> for RequestFilterDataProvider<D, F>
where
    F: Fn(&DataRequest) -> bool,
    M: DataMarker,
    D: DynProvider<M>,
{
    fn load_payload(
        &self,
        key: ResourceKey,
        req: &DataRequest,
    ) -> Result<DataResponse<M>, DataError> {
        if (self.predicate)(req) {
            self.inner.load_payload(key, req)
        } else {
            Err(DataErrorKind::FilteredResource
                .with_str_context(self.filter_name)
                .with_req(key, req))
        }
    }
}

impl<D, F, M> ResourceProvider<M> for RequestFilterDataProvider<D, F>
where
    F: Fn(&DataRequest) -> bool,
    M: ResourceMarker,
    D: ResourceProvider<M>,
{
    fn load_resource(&self, req: &DataRequest) -> Result<DataResponse<M>, DataError> {
        if (self.predicate)(req) {
            self.inner.load_resource(req)
        } else {
            Err(DataErrorKind::FilteredResource
                .with_str_context(self.filter_name)
                .with_req(M::KEY, req))
        }
    }
}

impl<D, F> BufferProvider for RequestFilterDataProvider<D, F>
where
    F: Fn(&DataRequest) -> bool,
    D: BufferProvider,
{
    fn load_buffer(
        &self,
        key: ResourceKey,
        req: &DataRequest,
    ) -> Result<DataResponse<BufferMarker>, DataError> {
        if (self.predicate)(req) {
            self.inner.load_buffer(key, req)
        } else {
            Err(DataErrorKind::FilteredResource
                .with_str_context(self.filter_name)
                .with_req(key, req))
        }
    }
}

impl<D, F> AnyProvider for RequestFilterDataProvider<D, F>
where
    F: Fn(&DataRequest) -> bool,
    D: AnyProvider,
{
    fn load_any(&self, key: ResourceKey, req: &DataRequest) -> Result<AnyResponse, DataError> {
        if (self.predicate)(req) {
            self.inner.load_any(key, req)
        } else {
            Err(DataErrorKind::FilteredResource
                .with_str_context(self.filter_name)
                .with_req(key, req))
        }
    }
}

impl<M, D, F> IterableDynProvider<M> for RequestFilterDataProvider<D, F>
where
    M: DataMarker,
    F: Fn(&DataRequest) -> bool,
    D: IterableDynProvider<M>,
{
    fn supported_options_for_key(
        &self,
        key: ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions> + '_>, DataError> {
        self.inner.supported_options_for_key(key).map(|iter| {
            // Use filter_map instead of filter to avoid cloning the options
            let filtered_iter = iter.filter_map(move |options| {
                let request = DataRequest {
                    options,
                    metadata: Default::default(),
                };
                if (self.predicate)(&request) {
                    Some(request.options)
                } else {
                    None
                }
            });
            let boxed_filtered_iter: Box<dyn Iterator<Item = ResourceOptions>> =
                Box::new(filtered_iter);
            boxed_filtered_iter
        })
    }
}

impl<M, D, F> IterableResourceProvider<M> for RequestFilterDataProvider<D, F>
where
    M: ResourceMarker,
    F: Fn(&DataRequest) -> bool,
    D: IterableResourceProvider<M>,
{
    fn supported_options(
        &self,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions> + '_>, DataError> {
        self.inner.supported_options().map(|iter| {
            // Use filter_map instead of filter to avoid cloning the options
            let filtered_iter = iter.filter_map(move |options| {
                let request = DataRequest {
                    options,
                    metadata: Default::default(),
                };
                if (self.predicate)(&request) {
                    Some(request.options)
                } else {
                    None
                }
            });
            let boxed_filtered_iter: Box<dyn Iterator<Item = ResourceOptions>> =
                Box::new(filtered_iter);
            boxed_filtered_iter
        })
    }
}

pub trait Filterable: Sized {
    /// Creates a filterable data provider with the given name for debugging.
    ///
    /// For more details, see [`icu_provider::filter`](crate::filter).
    fn filterable(
        self,
        filter_name: &'static str,
    ) -> RequestFilterDataProvider<Self, fn(&DataRequest) -> bool>;
}

impl<T> Filterable for T
where
    T: Sized,
{
    fn filterable(
        self,
        filter_name: &'static str,
    ) -> RequestFilterDataProvider<Self, fn(&DataRequest) -> bool> {
        fn noop(_: &DataRequest) -> bool {
            true
        }
        RequestFilterDataProvider {
            inner: self,
            predicate: noop,
            filter_name,
        }
    }
}
