// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Providers that filter resource requests.
//!
//! Requests that fail a filter test will return [`DataError::FilteredResource`] and will not
//! appear in [`IterableDataProvider`] iterators.
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
//! use icu_locid_macros::language;
//!
//! // Only return German data from a HelloWorldProvider:
//! HelloWorldProvider::new_with_placeholder_data()
//!     .filterable()
//!     .filter_by_langid(|langid| langid.language == language!("de"));
//! ```
//!
//! [`IterableDataProvider`]: crate::iter::IterableDataProvider

mod impls;

pub use impls::*;

use crate::iter::IterableDataProviderCore;
use crate::prelude::*;

/// A data provider that selectively filters out data requests.
///
/// Data requests that are rejected by the filter will return [`DataError::FilteredResource`], and
/// they will not be returned by [`IterableDataProviderCore::supported_options_for_key`].
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

    /// A description for this filter, used in error messages.
    pub description: String,
}

impl<'d, 's, D, F, M> DataProvider<'d, 's, M> for RequestFilterDataProvider<D, F>
where
    F: Fn(&DataRequest) -> bool,
    M: DataMarker<'s>,
    D: DataProvider<'d, 's, M>,
{
    fn load_payload(&self, req: &DataRequest) -> Result<DataResponse<'d, 's, M>, DataError> {
        if (self.predicate)(req) {
            self.inner.load_payload(req)
        } else {
            Err(DataError::FilteredResource(
                req.clone(),
                self.description.clone(),
            ))
        }
    }
}

impl<D, F> IterableDataProviderCore for RequestFilterDataProvider<D, F>
where
    F: Fn(&DataRequest) -> bool,
    D: IterableDataProviderCore,
{
    fn supported_options_for_key(
        &self,
        resc_key: &ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions> + '_>, DataError> {
        self.inner.supported_options_for_key(resc_key).map(|iter| {
            let resc_key = *resc_key;
            // Use filter_map instead of filter to avoid cloning the options
            let filtered_iter = iter.filter_map(move |options| {
                let request = DataRequest {
                    resource_path: ResourcePath {
                        key: resc_key,
                        options,
                    },
                };
                if (self.predicate)(&request) {
                    Some(request.resource_path.options)
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
    fn filterable(self) -> RequestFilterDataProvider<Self, fn(&DataRequest) -> bool>;
}

impl<T> Filterable for T
where
    T: Sized,
{
    fn filterable(self) -> RequestFilterDataProvider<Self, fn(&DataRequest) -> bool> {
        fn noop(_: &DataRequest) -> bool {
            true
        }
        RequestFilterDataProvider {
            inner: self,
            predicate: noop,
            description: "some description".into(),
        }
    }
}
