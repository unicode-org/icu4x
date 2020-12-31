// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
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
//!
//! ## Additional Traits
//!
//! ### `IterableDataProvider`
//!
//! Data providers can implement [`IterableDataProvider`], allowing iteration over all [`ResourceOptions`]
//! instances supported for a certain key in the data provider.
//!
//! For more information, see the `iter` module.
//!
//! ### `ErasedDataProvider`
//!
//! The [`DataProvider`] trait has a type argument corresponding to the type being loaded. A peer
//! trait [`ErasedDataProvider`] removes the type argument, using runtime type checking instead.
//!
//! Since [`ErasedDataProvider`] is not specific to a single type, it can be useful for:
//!
//! - Caches
//! - Bulk data operations
//! - Transforming from one format to another
//!
//! ## Types and Lifetimes
//!
//! All types `T` implementing standard Clone and Debug can be passed through the data provider.
//!
//! Most DataProvider traits take a lifetime argument `'d`. This represents the lifetime of data
//! returned by the DataProvider.
//!
//! DataProvider returns data in the form of a `Cow<'d, T>`, where `'d` is the lifetime of the data
//! if it is borrowed, and `T` is constrained by `T: 'd` such that if the data is owned, it may not
//! have any fields whose lifetime subceeds `'d`.
//!
//! When using `ErasedDataProvider`, the following additional requirements are placed on `T`:
//!
//! - `T: 'static`, since `T` must be compatible with `Any`
//! - `serde::Deserialize` and `serde::Serialize`, allowing for type-agnostic (de)serialization
//! - `Default`, allowing `InvariantDataProvider` to work
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
//! [`ErasedDataProvider`]: erased::ErasedDataProvider

pub mod data_provider;
#[macro_use]
pub mod resource;
pub mod erased;
pub mod inv;
pub mod iter;
pub mod struct_provider;
pub mod structs;

mod error;

pub use error::Error as DataError;

pub mod prelude {
    //! Core selection of APIs and structures for `DataProvider`.
    pub use crate::data_provider::DataProvider;
    pub use crate::data_provider::DataRequest;
    pub use crate::data_provider::DataResponse;
    pub use crate::data_provider::DataResponseMetadata;
    pub use crate::error::Error as DataError;
    pub use crate::iter::IterableDataProvider;
    pub use crate::iter::KeyedDataProvider;
    pub use crate::resource::ResourceCategory;
    pub use crate::resource::ResourceKey;
    pub use crate::resource::ResourceOptions;
    pub use crate::resource::ResourcePath;
}

// Also include the same symbols at the top level for selective inclusion
pub use prelude::*;
