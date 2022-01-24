// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `icu_provider` is one of the [`ICU4X`] components.
//!
//! It defines traits and structs for transmitting data through the ICU4X locale data pipeline.
//! The primary trait is [`DataProvider`]. It has one method, which transforms a [`Request`] into
//! a [`Response`]:
//!
//! ```ignore
//! fn load_payload(&self, req: &DataRequest) -> Result<DataResponse<'data>, DataError>
//! ```
//!
//! A [`Request`] contains a [`ResourceKey`] (a fixed identifier such as "plurals/cardinal@1") and
//! [`ResourceOptions`] (a language identifier and optional variant, e.g. "fr") being requested.
//! The Response contains the data payload corresponding to the Request.
//!
//! A [`Response`] contains a [`DataPayload`] along with other metadata.
//!
//! The most common types required for ICU4X [`DataProvider`] are included via the prelude:
//!
//! ```
//! use icu_provider::prelude::*;
//! ```
//!
//! ## Concrete Implementations of Data Providers
//!
//! Any object implementing [`DataProvider`] can be used to supply ICU4X with locale data. ICU4X ships
//! with some pre-built data providers:
//!
//! - [`CldrJsonDataProvider`](../icu_provider_cldr/transform/struct.CldrJsonDataProvider.html) reads structured
//!   data directly from CLDR source files.
//! - [`FsDataProvider`](../icu_provider_fs/struct.FsDataProvider.html) reads structured data from the
//!   filesystem. It can also write out that filesystem structure. More efficient than CldrJsonDataProvider.
//!
//! This crate also contains some concrete implementations for testing purposes:
//!
//! - [`InvariantDataProvider`] returns fixed data that does not vary by locale.
//! - [`AnyPayloadProvider`] wraps a particular instance of a struct and returns it.
//! - [`HelloWorldProvider`] returns "hello world" strings in several languages.
//!
//! ## Types and Lifetimes
//!
//! Types compatible with [`Yokeable`] can be passed through the data provider, so long as they are
//! associated with a marker type implementing [`DataMarker`].
//!
//! Most [`DataProvider`] traits take one lifetime argument: `'data`. This lifetime allows data
//! structs to borrow zero-copy data. In practice, it also represents the lifetime of data that
//! the Cart of the Yoke of the DataPayload borrows; for more information on carts and yokes,
//! see [`yoke`].
//!
//! ## Additional Traits
//!
//! ### `IterableDataProvider`
//!
//! Data providers can implement [`IterableProvider`], allowing iteration over all [`ResourceOptions`]
//! instances supported for a certain key in the data provider.
//!
//! For more information, see the [`iter`] module.
//!
//! ### `BufferProvider`
//!
//! The trait [`BufferProvider`] represents a data provider that produces buffers (`[u8]`), which
//! are typically deserialized later via Serde. This allows for a Serde-enabled provider
//! to be saved as a trait object without being specific to a data struct type.
//!
//! ### `AnyProvider`
//!
//! The trait [`AnyProvider`] removes the type argument from [`DataProvider`] and requires
//! that all data structs be convertible to the [`Any`](core::any::Any) type. This enables the
//! processing of data without the caller knowing the underlying data struct.
//!
//! Since [`AnyProvider`] is not specific to a single type, it can be useful for caches or
//! other bulk data operations.
//!
//! ### `DataProvider<SerializeMarker>`
//!
//! *Enabled with the "serialize" feature*
//!
//! Data providers capable of returning opaque `erased_serde::Serialize` trait objects can be used as
//! input to a data exporter, such as when writing data to the filesystem.
//!
//! This trait is normally implemented using the [`impl_dyn_provider!`] macro.
//!
//! This trait is normally implemented using the [`impl_dyn_provider!`] macro.
//!
//! [`ICU4X`]: ../icu/index.html
//! [`DataProvider`]: data_provider::DataProvider
//! [`Request`]: data_provider::DataRequest
//! [`Response`]: data_provider::DataResponse
//! [`ResourceKey`]: resource::ResourceKey
//! [`ResourceOptions`]: resource::ResourceOptions
//! [`IterableProvider`]: iter::IterableProvider
//! [`InvariantDataProvider`]: inv::InvariantDataProvider
//! [`AnyPayloadProvider`]: struct_provider::AnyPayloadProvider
//! [`HelloWorldProvider`]: hello_world::HelloWorldProvider
//! [`AnyProvider`]: any::AnyProvider
//! [`Yokeable`]: yoke::Yokeable
//! [`impl_dyn_provider!`]: impl_dyn_provider

#![cfg_attr(not(any(test, feature = "std")), no_std)]

extern crate alloc;

#[macro_use]
pub mod dynutil;

pub mod any;
pub mod buf;
mod data_provider;
pub mod either;
mod error;
pub mod export;
pub mod filter;
pub mod fork;
pub mod hello_world;
mod helpers;
pub mod inv;
pub mod iter;
#[macro_use]
pub mod marker;
#[macro_use]
mod resource;
#[cfg(feature = "serde")]
pub mod serde;
pub mod struct_provider;

#[cfg(feature = "macros")]
pub use icu_provider_macros::data_struct;

pub mod prelude {
    //! Core selection of APIs and structures for [`DataProvider`].
    pub use crate::any::AnyMarker;
    pub use crate::any::AnyPayload;
    pub use crate::any::AnyProvider;
    pub use crate::any::AnyResponse;
    pub use crate::buf::BufferMarker;
    pub use crate::buf::BufferProvider;
    pub use crate::data_provider::DataPayload;
    pub use crate::data_provider::DataProvider;
    pub use crate::data_provider::DataRequest;
    pub use crate::data_provider::DataResponse;
    pub use crate::data_provider::DataResponseMetadata;
    pub use crate::error::DataError;
    pub use crate::error::DataErrorKind;
    pub use crate::marker::DataMarker;
    pub use crate::resource::ResourceKey;
    pub use crate::resource::ResourceOptions;
    pub use crate::resource::ResourcePath;
    pub use crate::resource_key;

    pub use crate::any::AsDataProviderAnyMarkerWrap;
    pub use crate::any::AsDowncastingAnyProvider;
    #[cfg(feature = "serde")]
    pub use crate::serde::AsDeserializingBufferProvider;
}

/// Re-export of the yoke crate for convenience of downstream implementors.
pub use yoke;

// Also include the same symbols at the top level for selective inclusion
pub use prelude::*;
