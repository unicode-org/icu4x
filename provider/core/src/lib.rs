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
//! fn load_payload(&self, req: &DataRequest) -> Result<DataResponse<'d>, DataError>
//! ```
//!
//! A [`Request`] contains a [`ResourceKey`] (a composition of a [`Category`] and sub-category, e.g.,
//! "plurals/cardinal@1") and [`ResourceOptions`] (a language identifier and optional variant, e.g.,
//! "fr") being requested. The Response contains the data payload corresponding to the Request.
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
//! - [`StructProvider`] wraps a particular instance of a struct and returns it.
//! - [`HelloWorldProvider`] returns "hello world" strings in several languages.
//!
//! ## Types and Lifetimes
//!
//! Types compatible with [`Yokeable`] can be passed through the data provider, so long as they are
//! associated with a marker type implementing [`DataMarker`].
//!
//! Most [`DataProvider`] traits take two lifetime arguments: `'d` and `'s`. These represent data
//! lifetimes for different ownership models of the data.
//!
//! - `'d` is the lifetime of a reference to the whole data struct being borrowed.
//! - `'s` is the lifetime of fields within a data struct.
//!
//! Data structs and [`DataMarker`] use the `'s` lifetime, and this is also the lifetime that the
//! [`Yokeable`] implementation should handle.
//!
//! `'s` should exceed `'d` (i.e., `'s: 'd`). If a data struct is fully owned, `'s: 'static`.
//!
//! ## Additional Traits
//!
//! ### `IterableDataProvider`
//!
//! Data providers can implement [`IterableDataProvider`], allowing iteration over all [`ResourceOptions`]
//! instances supported for a certain key in the data provider.
//!
//! For more information, see the [`iter`] module.
//!
//! ### `SerdeDeDataProvider`
//!
//! *Enabled with the "provider_serde" feature*
//!
//! The trait [`SerdeDeDataProvider`] removes the type argument from `DataProvider` and requires
//! that all data structs be deserializable via Serde. This allows for a Serde-enabled provider
//! to be saved as a trait object without being specific to a data struct type.
//!
//! ### `DataProvider<dyn SerdeSeDataStruct>`
//!
//! *Enabled with the "provider_serde" feature*
//!
//! Data providers capable of returning opaque [`SerdeSeDataStruct`] trait objects can be used as
//! input to a data exporter, such as when writing data to the filesystem.
//!
//! This trait is normally implemented using the [`impl_dyn_provider!`] macro.
//!
//! ### `DataProvider<dyn ErasedDataStruct>`
//!
//! The trait [`ErasedDataProvider`] removes the type argument from [`DataProvider`] and requires
//! that all data structs be convertible to the [`Any`](std::any::Any) type. This enables the processing of data
//! without the caller knowing the underlying data struct.
//!
//! Since [`ErasedDataProvider`] is not specific to a single type, it can be useful for caches or
//! other bulk data operations.
//!
//! This trait is normally implemented using the [`impl_dyn_provider!`] macro.
//!
//! [`ICU4X`]: ../icu/index.html
//! [`DataProvider`]: data_provider::DataProvider
//! [`Request`]: data_provider::DataRequest
//! [`Response`]: data_provider::DataResponse
//! [`ResourceKey`]: resource::ResourceKey
//! [`Category`]: resource::ResourceCategory
//! [`ResourceOptions`]: resource::ResourceOptions
//! [`IterableDataProvider`]: iter::IterableDataProvider
//! [`InvariantDataProvider`]: inv::InvariantDataProvider
//! [`StructProvider`]: struct_provider::StructProvider
//! [`HelloWorldProvider`]: hello_world::HelloWorldProvider
//! [`ErasedDataProvider`]: erased::ErasedDataProvider
//! [`SerdeDeDataProvider`]: serde::SerdeDeDataProvider
//! [`SerdeSeDataStruct`]: serde::SerdeSeDataStruct
//! [`Yokeable`]: yoke::Yokeable
//! [`impl_dyn_provider!`]: impl_dyn_provider

#[macro_use]
pub mod dynutil;

pub(crate) mod data_provider;
#[macro_use]
mod resource;
#[macro_use]
pub mod erased;
pub mod export;
pub mod filter;
pub mod hello_world;
pub mod inv;
pub mod iter;
#[macro_use]
pub mod marker;
#[cfg(feature = "provider_serde")]
pub mod serde;
pub mod struct_provider;

mod error;

pub use error::Error as DataError;

pub mod prelude {
    //! Core selection of APIs and structures for [`DataProvider`].
    pub use crate::data_provider::DataPayload;
    pub use crate::data_provider::DataProvider;
    pub use crate::data_provider::DataRequest;
    pub use crate::data_provider::DataResponse;
    pub use crate::data_provider::DataResponseMetadata;
    pub use crate::error::Error as DataError;
    pub use crate::marker::DataMarker;
    pub use crate::resource::ResourceCategory;
    pub use crate::resource::ResourceKey;
    pub use crate::resource::ResourceOptions;
    pub use crate::resource::ResourcePath;
}

/// Re-export of the yoke crate for convenience of downstream implementors.
pub use yoke;

// Also include the same symbols at the top level for selective inclusion
pub use prelude::*;

pub mod internal {
    //! Macro dependencies; not intended to be used directly.
    /// Re-export tinystr16 for macro resource_key!()
    pub use tinystr::tinystr16;
    /// Re-export tinystr4 for macro resource_key!()
    pub use tinystr::tinystr4;
}
