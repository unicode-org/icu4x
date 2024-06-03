// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Providers that combine multiple other providers.
//!
//! # Types of Forking Providers
//!
//! ## marker-Based
//!
//! To fork between providers that support different data markers, see:
//!
//! - [`ForkByMarkerProvider`]
//! - [`MultiForkByMarkerProvider`]
//!
//! ## Locale-Based
//!
//! To fork between providers that support different locales, see:
//!
//! - [`ForkByErrorProvider`]`<`[`MissingLocalePredicate`]`>`
//! - [`MultiForkByErrorProvider`]`<`[`MissingLocalePredicate`]`>`
//!
//! [`MissingLocalePredicate`]: predicates::MissingLocalePredicate
//!
//! # Examples
//!
//! See:
//!
//! - [`ForkByMarkerProvider`]
//! - [`MultiForkByMarkerProvider`]
//! - [`MissingLocalePredicate`]

use alloc::vec::Vec;

mod by_error;

pub mod predicates;

#[macro_use]
mod macros;

pub use by_error::ForkByErrorProvider;
pub use by_error::MultiForkByErrorProvider;

use predicates::ForkByErrorPredicate;
use predicates::MissingDataMarkerPredicate;

/// Create a provider that returns data from one of two child providers based on the marker.
///
/// The result of the first provider that supports a particular [`DataMarkerInfo`] will be returned,
/// even if the request failed for other reasons (such as an unsupported language). Therefore,
/// you should add child providers that support disjoint sets of markers.
///
/// [`ForkByMarkerProvider`] does not support forking between [`DataProvider`]s. However, it
/// supports forking between [`AnyProvider`], [`BufferProvider`], and [`DynamicDataProvider`].
///
/// # Examples
///
/// Normal usage:
///
/// ```
/// use icu_locale_core::langid;
/// use icu_provider::hello_world::*;
/// use icu_provider::prelude::*;
/// use icu_provider_adapters::fork::ForkByMarkerProvider;
///
/// struct DummyBufferProvider;
/// impl DynamicDataProvider<BufferMarker> for DummyBufferProvider {
///     fn load_data(
///         &self,
///         marker: DataMarkerInfo,
///         req: DataRequest,
///     ) -> Result<DataResponse<BufferMarker>, DataError> {
///         Err(DataErrorKind::MissingDataMarker.with_req(marker, req))
///     }
/// }
///
/// let forking_provider = ForkByMarkerProvider::new(
///     DummyBufferProvider,
///     HelloWorldProvider.into_json_provider(),
/// );
///
/// let provider = forking_provider.as_deserializing();
///
/// let german_hello_world: DataPayload<HelloWorldV1Marker> = provider
///     .load(DataRequest {
///         locale: &langid!("de").into(),
///         ..Default::default()
///     })
///     .expect("Loading should succeed")
///     .take_payload()
///     .expect("Data should be present");
///
/// assert_eq!("Hallo Welt", german_hello_world.get().message);
/// ```
///
/// Stops at the first provider supporting a marker, even if the locale is not supported:
///
/// ```
/// use icu_locale_core::{subtags::language, langid};
/// use icu_provider::hello_world::*;
/// use icu_provider::prelude::*;
/// use icu_provider_adapters::filter::Filterable;
/// use icu_provider_adapters::fork::ForkByMarkerProvider;
///
/// let forking_provider = ForkByMarkerProvider::new(
///     HelloWorldProvider
///         .into_json_provider()
///         .filterable("Chinese")
///         .filter_by_langid(|langid| langid.language == language!("zh")),
///     HelloWorldProvider
///         .into_json_provider()
///         .filterable("German")
///         .filter_by_langid(|langid| langid.language == language!("de")),
/// );
///
/// let provider: &dyn DataProvider<HelloWorldV1Marker> =
///     &forking_provider.as_deserializing();
///
/// // Chinese is the first provider, so this succeeds
/// let chinese_hello_world = provider
///     .load(DataRequest {
///         locale: &langid!("zh").into(),
///         ..Default::default()
///     })
///     .expect("Loading should succeed")
///     .take_payload()
///     .expect("Data should be present");
///
/// assert_eq!("你好世界", chinese_hello_world.get().message);
///
/// // German is shadowed by Chinese, so this fails
/// provider
///     .load(DataRequest {
///         locale: &langid!("de").into(),
///         ..Default::default()
///     })
///     .expect_err("Should stop at the first provider, even though the second has data");
/// ```
///
/// [`DataMarkerInfo`]: icu_provider::DataMarkerInfo
/// [`DataProvider`]: icu_provider::DataProvider
/// [`AnyProvider`]: icu_provider::AnyProvider
/// [`BufferProvider`]: icu_provider::BufferProvider
/// [`DynamicDataProvider`]: icu_provider::DynamicDataProvider
pub type ForkByMarkerProvider<P0, P1> = ForkByErrorProvider<P0, P1, MissingDataMarkerPredicate>;

impl<P0, P1> ForkByMarkerProvider<P0, P1> {
    /// A provider that returns data from one of two child providers based on the marker.
    ///
    /// See [`ForkByMarkerProvider`].
    pub fn new(p0: P0, p1: P1) -> Self {
        ForkByErrorProvider::new_with_predicate(p0, p1, MissingDataMarkerPredicate)
    }
}

/// A provider that returns data from the first child provider supporting the marker.
///
/// The result of the first provider that supports a particular [`DataMarkerInfo`] will be returned,
/// even if the request failed for other reasons (such as an unsupported language). Therefore,
/// you should add child providers that support disjoint sets of markers.
///
/// [`MultiForkByMarkerProvider`] does not support forking between [`DataProvider`]s. However, it
/// supports forking between [`AnyProvider`], [`BufferProvider`], and [`DynamicDataProvider`].
///
/// # Examples
///
/// ```
/// use icu_locale_core::{subtags::language, langid};
/// use icu_provider::hello_world::*;
/// use icu_provider::prelude::*;
/// use icu_provider_adapters::filter::Filterable;
/// use icu_provider_adapters::fork::MultiForkByMarkerProvider;
///
/// let forking_provider = MultiForkByMarkerProvider::new(
///     vec![
///         HelloWorldProvider
///             .into_json_provider()
///             .filterable("Chinese")
///             .filter_by_langid(|langid| langid.language == language!("zh")),
///         HelloWorldProvider
///             .into_json_provider()
///             .filterable("German")
///             .filter_by_langid(|langid| langid.language == language!("de")),
///     ],
/// );
///
/// let provider: &dyn DataProvider<HelloWorldV1Marker> =
///     &forking_provider.as_deserializing();
///
/// // Chinese is the first provider, so this succeeds
/// let chinese_hello_world = provider
///     .load(DataRequest {
///         locale: &langid!("zh").into(),
///         ..Default::default()
///     })
///     .expect("Loading should succeed")
///     .take_payload()
///     .expect("Data should be present");
///
/// assert_eq!("你好世界", chinese_hello_world.get().message);
///
/// // German is shadowed by Chinese, so this fails
/// provider
///     .load(DataRequest {
///         locale: &langid!("de").into(),
///         ..Default::default()
///     })
///     .expect_err("Should stop at the first provider, even though the second has data");
/// ```
///
/// [`DataMarkerInfo`]: icu_provider::DataMarkerInfo
/// [`DataProvider`]: icu_provider::DataProvider
/// [`AnyProvider`]: icu_provider::AnyProvider
/// [`BufferProvider`]: icu_provider::BufferProvider
/// [`DynamicDataProvider`]: icu_provider::DynamicDataProvider
pub type MultiForkByMarkerProvider<P> = MultiForkByErrorProvider<P, MissingDataMarkerPredicate>;

impl<P> MultiForkByMarkerProvider<P> {
    /// Create a provider that returns data from the first child provider supporting the marker.
    ///
    /// See [`MultiForkByMarkerProvider`].
    pub fn new(providers: Vec<P>) -> Self {
        MultiForkByErrorProvider::new_with_predicate(providers, MissingDataMarkerPredicate)
    }
}
