// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `icu_provider` is one of the [`ICU4X`] components.
//!
//! Unicode's experience with ICU4X's parent projects, ICU4C and ICU4J, led the team to realize
//! that data management is the most critical aspect of deploying internationalization, and that it requires
//! a high level of customization for the needs of the platform it is embedded in. As a result
//! ICU4X comes with a selection of providers that should allow for ICU4X to naturally fit into
//! different business and technological needs of customers.
//!
//! `icu_provider` defines traits and structs for transmitting data through the ICU4X locale
//! data pipeline. The primary trait is [`DataProvider`]. It is parameterized by a
//! [`DataMarker`], which is the type-system-level data identifier. [`DataProvider`] has a single method,
//! [`DataProvider::load`], which transforms a [`DataRequest`] into a [`DataResponse`].
//!
//! - [`DataRequest`] contains selectors to choose a specific variant of the marker, such as a locale.
//! - [`DataResponse`] contains the data if the request was successful.
//!
//! The most common types required for this crate are included via the prelude:
//!
//! ```
//! use icu_provider::prelude::*;
//! ```
//!
//! ## Dynamic Data Providers
//!
//! If the type system cannot be leveraged to load data (such as when dynamically loading from I/O),
//! there's another form of the [`DataProvider`]: [`DynamicDataProvider`]. While [`DataProvider`] is parametrized
//! on the type-system level by a [`DataMarker`] (which are distinct types implementing this trait),
//! [`DynamicDataProvider`]s are parametrized at runtime by a [`DataMarkerInfo`] struct, which essentially is the runtime
//! representation of the [`DataMarker`] type.
//!
//! The [`DynamicDataProvider`] is still type-level parametrized by the type that it loads, and there are two
//! implementations that should be called out
//!
//! - [`DynamicDataProvider<AnyMarker>`], and [`AnyProvider`] (a slightly optimized alternative) return data as `dyn Any` trait objects.
//! - [`DynamicDataProvider<BufferMarker>`], a.k.a. [`BufferProvider`](buf::BufferProvider) returns data as `[u8]` buffers.
//!
//! ### AnyProvider
//!
//! These providers are able to return structured data cast into `dyn Any` trait objects. Users
//! can call [`as_downcasting()`] to get an object implementing [`DataProvider`] by downcasting
//! the trait objects.
//!
//! Examples of AnyProviders:
//!
//! - [`AnyPayloadProvider`] wraps a specific data struct and returns it.
//!
//! ### BufferProvider
//!
//! These providers are able to return unstructured data typically represented as
//! [`serde`]-serialized buffers. Users can call [`as_deserializing()`] to get an object
//! implementing [`DataProvider`] by invoking Serde Deserialize.
//!
//! Examples of BufferProviders:
//!
//! - [`FsDataProvider`] reads individual buffers from the filesystem.
//! - [`BlobDataProvider`] reads buffers from a large in-memory blob.
//!
//! ## Provider Adapters
//!
//! ICU4X offers several built-in modules to combine providers in interesting ways.
//! These can be found in the [`icu_provider_adapters`] crate.
//!
//! ## Testing Provider
//!
//! This crate also contains a concrete provider for demonstration purposes:
//!
//! - [`HelloWorldProvider`] returns "hello world" strings in several languages.
//!
//! ## Types and Lifetimes
//!
//! Types compatible with [`Yokeable`] can be passed through the data provider, so long as they are
//! associated with a marker type implementing [`DynamicDataMarker`].
//!
//! Data structs should generally have one lifetime argument: `'data`. This lifetime allows data
//! structs to borrow zero-copy data.
//!
//! ## Data generation API
//!
//! *This functionality is enabled with the "datagen" Cargo feature*
//!
//! The [`export`] module contains several APIs for data generation. See [`icu_datagen`] for the reference
//! data generation implementation.
//!
//! [`ICU4X`]: ../icu/index.html
//! [`AnyPayloadProvider`]: ../icu_provider_adapters/any_payload/struct.AnyPayloadProvider.html
//! [`HelloWorldProvider`]: hello_world::HelloWorldProvider
//! [`AnyProvider`]: any::AnyProvider
//! [`Yokeable`]: yoke::Yokeable
//! [`impl_dynamic_data_provider!`]: dynutil::impl_dynamic_data_provider
//! [`icu_provider_adapters`]: ../icu_provider_adapters/index.html
//! [`DatagenProvider`]: ../icu_datagen/struct.DatagenProvider.html
//! [`as_downcasting()`]: any::AsDowncastingAnyProvider::as_downcasting
//! [`as_deserializing()`]: buf::AsDeserializingBufferProvider::as_deserializing
//! [`FsDataProvider`]: ../icu_provider_fs/struct.FsDataProvider.html
//! [`BlobDataProvider`]: ../icu_provider_blob/struct.BlobDataProvider.html
//! [`icu_datagen`]: ../icu_datagen/index.html

// https://github.com/unicode-org/icu4x/blob/main/documents/process/boilerplate.md#library-annotations
#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::exhaustive_structs,
        clippy::exhaustive_enums,
        missing_debug_implementations,
    )
)]
#![warn(missing_docs)]

extern crate alloc;

pub mod any;
pub mod buf;
pub mod constructors;
pub mod dynutil;
#[cfg(feature = "export")]
pub mod export;
pub mod hello_world;

// TODO: put this in a separate crate
#[cfg(feature = "serde")]
#[doc(hidden)]
pub mod serde_borrow_de_utils;

mod data_provider;
pub use data_provider::{
    BoundDataProvider, DataProvider, DataProviderWithMarker, DynamicDataProvider,
};
#[cfg(feature = "std")]
pub use data_provider::{IterableDataProvider, IterableDynamicDataProvider};

mod error;
pub use error::{DataError, DataErrorKind};

#[cfg(feature = "macros")]
pub use icu_provider_macros::data_struct;

mod request;
pub use request::{DataLocale, DataMarkerAttributes, DataRequest, DataRequestMetadata, *};

mod response;
#[doc(hidden)] // TODO(#4467): establish this as an internal API
pub use response::DataPayloadOr;
pub use response::{Cart, DataPayload, DataResponse, DataResponseMetadata};

#[path = "marker.rs"]
mod marker_full;

pub use marker_full::{DataMarker, DataMarkerInfo, DynamicDataMarker};
pub mod marker {
    //! Additional [`DataMarker`](super::DataMarker) helpers.

    pub use super::marker_full::{
        data_marker_path, impl_data_provider_never_marker, DataMarkerPath, DataMarkerPathHash,
        NeverMarker,
    };
}

/// Core selection of APIs and structures for the ICU4X data provider.
pub mod prelude {
    #[doc(no_inline)]
    pub use crate::any::{
        AnyMarker, AnyPayload, AnyProvider, AnyResponse, AsDowncastingAnyProvider,
        AsDynamicDataProviderAnyMarkerWrap,
    };
    #[doc(no_inline)]
    #[cfg(feature = "serde")]
    pub use crate::buf::AsDeserializingBufferProvider;
    #[doc(no_inline)]
    pub use crate::buf::{BufferMarker, BufferProvider};
    pub use crate::request::*;
    #[doc(no_inline)]
    pub use crate::{
        BoundDataProvider, DataError, DataErrorKind, DataLocale, DataMarker, DataMarkerAttributes,
        DataMarkerInfo, DataPayload, DataProvider, DataRequest, DataRequestMetadata, DataResponse,
        DataResponseMetadata, DynamicDataMarker, DynamicDataProvider,
    };
    #[cfg(feature = "std")]
    pub use crate::{IterableDataProvider, IterableDynamicDataProvider};

    #[doc(no_inline)]
    pub use yoke;
    #[doc(no_inline)]
    pub use zerofrom;
}

mod fallback;

#[doc(hidden)] // macro use
pub mod _internal {
    pub use super::fallback::{
        LocaleFallbackConfig, LocaleFallbackPriority, LocaleFallbackSupplement,
    };
    pub use icu_locale_core as locale_core;

    #[cfg(feature = "logging")]
    pub use log;

    #[cfg(all(not(feature = "logging"), debug_assertions, feature = "std"))]
    pub mod log {
        pub use std::eprintln as error;
        pub use std::eprintln as warn;
        pub use std::eprintln as info;
        pub use std::eprintln as debug;
        pub use std::eprintln as trace;
    }

    #[cfg(all(
        not(feature = "logging"),
        any(not(debug_assertions), not(feature = "std"))
    ))]
    pub mod log {
        #[macro_export]
        macro_rules! _internal_noop_log {
            ($($t:expr),*) => {};
        }
        pub use crate::_internal_noop_log as error;
        pub use crate::_internal_noop_log as warn;
        pub use crate::_internal_noop_log as info;
        pub use crate::_internal_noop_log as debug;
        pub use crate::_internal_noop_log as trace;
    }
}

#[test]
fn test_logging() {
    // This should compile on all combinations of features
    crate::_internal::log::info!("Hello World");
}
