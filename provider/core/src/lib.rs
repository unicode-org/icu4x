// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `icu_provider` is one of the [`ICU4X`] components.
//!
//! `icu_provider` defines traits and structs for transmitting data through the ICU4X locale
//! data pipeline. The primary trait is [`ResourceProvider`]. It is parameterized by a
//! [`ResourceMarker`], which contains the data type and a [`ResourceKey`]. It has one method,
//! [`ResourceProvider::load_resource`], which transforms a [`DataRequest`]
//! into a [`DataResponse`].
//!
//! - [`ResourceKey`] is a fixed identifier for the data type, such as `"plurals/cardinal@1"`.
//! - [`DataRequest`] contains additional annotations to choose a specific variant of the key,
//!   such as a locale.
//! - [`DataResponse`] contains the data if the request was successful.
//!
//! In addition, there are three other traits which are widely implemented:
//!
//! - [`AnyProvider`] returns data as `dyn Any` trait objects.
//! - [`BufferProvider`] returns data as `[u8]` buffers.
//! - [`DynProvider`] returns structured data but is not specific to a key.
//!
//! The most common types required for this crate are included via the prelude:
//!
//! ```
//! use icu_provider::prelude::*;
//! ```
//!
//! ## Types of Data Providers
//!
//! All nontrivial data providers can fit into one of two classes.
//!
//! 1. Type 1: Those whose data originates as structured Rust objects
//! 2. Type 2: Those whose data originates as unstructured `[u8]` buffers
//!
//! ### Type 1 Providers
//!
//! Type 1 providers generally implement [`AnyProvider`], which returns structured data cast into
//! `dyn Any` trait objects. Users can call [`as_downcasting()`] to get an object implementing
//! [`ResourceProvider`] by downcasting the trait objects.
//!
//! Examples of Type 1 providers:
//!
//! - [`CldrJsonDataProvider`] reads structured data from CLDR JSON source files and returns
//!   structured Rust objects.
//! - [`AnyPayloadProvider`] wraps a specific data struct and returns it.
//! - The upcoming `crabbake` provider which reads structured data from Rust source files
//!
//! ### Type 2 Providers
//!
//! Type 2 providers generally implement [`BufferProvider`], which returns unstructured data
//! typically represented as [`serde`]-serialized buffers. Users can call [`as_deserializing()`]
//! to get an object implementing [`ResourceProvider`] by invoking Serde Deserialize.
//!
//! Examples of Type 2 providers:
//!
//! - [`FsDataProvider`] reads individual buffers from the filesystem.
//! - [`BlobDataProvider`] reads buffers from a large in-memory blob.
//!
//! ### Special-Purpose Providers
//!
//! This crate also contains some concrete implementations for testing purposes:
//!
//! - [`InvariantDataProvider`] returns fixed data that does not vary by locale.
//! - [`HelloWorldProvider`] returns "hello world" strings in several languages.
//!
//! ## Combinatorial Providers
//!
//! ICU4X offers several built-in modules to combine providers in interesting ways:
//!
//! - Use the [`fork`] module to marshall data requests between multiple possible providers.
//! - Use the [`either`] module to choose between multiple provider types at runtime.
//! - Use the [`filter`] module to programmatically reject certain data requests.
//!
//! ## Types and Lifetimes
//!
//! Types compatible with [`Yokeable`] can be passed through the data provider, so long as they are
//! associated with a marker type implementing [`DataMarker`].
//!
//! Data structs should generally have one lifetime argument: `'data`. This lifetime allows data
//! structs to borrow zero-copy data.
//!
//! ## Additional Traits
//!
//! ### `IterableDataProvider`
//!
//! Data providers can implement [`IterableDynProvider`]/[`IterableResourceProvider`], allowing
//! iteration over all [`ResourceOptions`] instances supported for a certain key in the data provider.
//!
//! For more information, see the [`iter`] module.
//!
//! ### `DataProvider<SerializeMarker>`
//!
//! *Enabled with the "serialize" feature*
//!
//! Data providers capable of returning opaque `erased_serde::Serialize` trait objects can be use
//! as input to a data exporter, such as when writing data to the filesystem.
//!
//! This trait is normally implemented using the [`impl_dyn_provider!`] macro.
//!
//! [`ICU4X`]: ../icu/index.html
//! [`DataProvider`]: data_provider::DataProvider
//! [`ResourceKey`]: resource::ResourceKey
//! [`ResourceOptions`]: resource::ResourceOptions
//! [`IterableDynProvider`]: iter::IterableDynProvider
//! [`IterableResourceProvider`]: iter::IterableResourceProvider
//! [`InvariantDataProvider`]: inv::InvariantDataProvider
//! [`AnyPayloadProvider`]: struct_provider::AnyPayloadProvider
//! [`HelloWorldProvider`]: hello_world::HelloWorldProvider
//! [`AnyProvider`]: any::AnyProvider
//! [`Yokeable`]: yoke::Yokeable
//! [`impl_dyn_provider!`]: impl_dyn_provider
//! [`as_downcasting()`]: AsDowncastingAnyProvider::as_downcasting
//! [`as_deserializing()`]: AsDeserializingBufferProvider::as_deserializing
//! [`CldrJsonDataProvider`]: ../icu_provider_cldr/struct.CldrJsonDataProvider.html
//! [`FsDataProvider`]: ../icu_provider_fs/struct.FsDataProvider.html
//! [`BlobDataProvider`]: ../icu_provider_blob/struct.BlobDataProvider.html

#![cfg_attr(
    not(any(test, feature = "std")),
    no_std,
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic
    )
)]

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
    //! Core selection of APIs and structures for the ICU4X data provider.
    pub use crate::any::AnyMarker;
    pub use crate::any::AnyPayload;
    pub use crate::any::AnyProvider;
    pub use crate::any::AnyResponse;
    pub use crate::buf::BufferMarker;
    pub use crate::buf::BufferProvider;
    pub use crate::data_provider::DataPayload;
    pub use crate::data_provider::DataRequest;
    pub use crate::data_provider::DataResponse;
    pub use crate::data_provider::DataResponseMetadata;
    pub use crate::data_provider::DynProvider;
    pub use crate::data_provider::ResourceProvider;
    pub use crate::error::DataError;
    pub use crate::error::DataErrorKind;
    pub use crate::marker::DataMarker;
    pub use crate::marker::ResourceMarker;
    pub use crate::resource::ResourceKey;
    pub use crate::resource::ResourceKeyHash;
    pub use crate::resource::ResourceOptions;
    pub use crate::resource_key;

    pub use crate::any::AsDowncastingAnyProvider;
    pub use crate::any::AsDynProviderAnyMarkerWrap;
    #[cfg(feature = "serde")]
    pub use crate::serde::AsDeserializingBufferProvider;

    pub use yoke;
    pub use zerofrom;
}

/// Re-export of the yoke and zerofrom crates for convenience of downstream implementors.
pub use yoke;
pub use zerofrom;

// Also include the same symbols at the top level for selective inclusion
pub use prelude::*;
