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
//! fn load(&self, req: &DataRequest) -> Result<DataResponse<'d>, DataError>
//! ```
//!
//! A Request contains a [`ResourceKey`] (a composition of a [`Category`] and sub-category, e.g.,
//! "plurals/cardinal@1") and [`ResourceOptions`] (a language identifier and optional variant, e.g.,
//! "fr") being requested. The Response contains the data payload corresponding to the Request.
//!
//! The most common types required for ICU4X [`DataProvider`] are included via the prelude:
//!
//! ```
//! use icu_provider::prelude::*;
//! ```
//!
//! ## Types of Data Providers
//!
//! Any object implementing [`DataProvider`] can be used to supply ICU4X with locale data. ICU4X ships
//! with some pre-built data providers:
//!
//! - [`FsDataProvider`](../icu_provider_fs/struct.FsDataProvider.html) reads structured data from the
//!   filesystem. It can also write out that filesystem structure.
//! - [`CldrJsonDataProvider`](../icu_provider_cldr/transform/struct.CldrJsonDataProvider.html) reads structured
//!   data directly from CLDR source files.
//!
//! ## Iterable Data Providers
//!
//! Data providers can implement [`ResourceOptionsCollection`], allowing them to be used via the
//! auto-implemented trait [`IterableDataProvider`]. This allows iteration over all [`ResourceOptions`]
//! instances supported for a certain key in the data provider. This can be useful when
//! transforming data between storage formats. For more information, see the [`iter`] module.
//!
//! ## `InvariantDataProvider`
//!
//! For testing or development purposes, this crate also offers `InvariantDataProvider`, which
//! returns fixed data that does not vary by locale. You must enable `InvariantDataProvider` via the
//! `"invariant"` feature in your Cargo.toml file.
//!
//! [`ICU4X`]: ../icu/index.html
//! [`DataProvider`]: prelude::DataProvider
//! [`Request`]: prelude::DataRequest
//! [`Response`]: prelude::DataResponse
//! [`ResourceKey`]: prelude::ResourceKey
//! [`Category`]: prelude::ResourceCategory
//! [`ResourceOptions`]: prelude::ResourceOptions
//! [`ResourceOptionsCollection`]: iter::ResourceOptionsCollection
//! [`IterableDataProvider`]: iter::IterableDataProvider

mod data_provider;
mod data_receiver;
mod error;
#[macro_use]
mod resource;
pub mod iter;
pub mod structs;

#[cfg(feature = "invariant")]
mod invariant;

#[cfg(feature = "invariant")]
pub use invariant::InvariantDataProvider;

pub mod prelude {
    //! Core selection of APIs and structures for `DataProvider`.
    pub use crate::data_provider::DataProvider;
    pub use crate::data_provider::DataRequest;
    pub use crate::data_provider::DataResponse;
    pub use crate::data_provider::DataResponseWithPayload;
    pub use crate::data_receiver::DataReceiver;
    pub use crate::data_receiver::DataReceiverForType;
    pub use crate::data_receiver::DataReceiverThrowAway;
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
