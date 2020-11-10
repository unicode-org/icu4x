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
//! A Request contains a [`DataKey`] (a composition of a [`Category`] and sub-category, e.g.,
//! "plurals/cardinal@1") and [`DataEntry`] (a language identifier and optional variant, e.g.,
//! "fr") being requested. The Response contains the data payload corresponding to the Request.
//!
//! The most common types required for ICU4X [`DataProvider`] are included via the prelude:
//!
//! ```
//! use icu_provider::prelude::*;
//! use std::any::TypeId;
//!
//! // Types included:
//! println!("{:?}", TypeId::of::<dyn DataProvider>());
//! println!("{:?}", TypeId::of::<DataError>());
//! println!("{:?}", TypeId::of::<DataKey>());
//! println!("{:?}", TypeId::of::<DataEntry>());
//! println!("{:?}", TypeId::of::<DataCategory>());
//! println!("{:?}", TypeId::of::<DataRequest>());
//! println!("{:?}", TypeId::of::<DataResponse>());
//! println!("{:?}", TypeId::of::<DataResponseBuilder>());
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
//! Data providers can implement [`DataEntryCollection`], allowing them to be used via the
//! auto-implemented trait [`IterableDataProvider`]. This allows iteration over all [`DataEntry`]
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
//! [`DataProvider`]: ./prelude/trait.DataProvider.html
//! [`Request`]: ./prelude/struct.DataRequest.html
//! [`Response`]: ./prelude/struct.DataResponse.html
//! [`DataKey`]: ./prelude/struct.DataKey.html
//! [`Category`]: ./prelude/enum.DataCategory.html
//! [`DataEntry`]: ./prelude/struct.DataEntry.html
//! [`DataEntryCollection`]: ./iter/trait.DataEntryCollection.html
//! [`IterableDataProvider`]: ./iter/trait.IterableDataProvider.html
//! [`iter`]: ./iter/index.html
mod cloneable_any;
mod data_entry;
#[macro_use]
mod data_key;
mod data_provider;
mod error;
pub mod iter;
pub mod structs;

#[cfg(feature = "invariant")]
mod invariant;

#[cfg(feature = "invariant")]
pub use invariant::InvariantDataProvider;

pub mod prelude {
    //! Core selection of APIs and structures for `DataProvider`.
    pub use crate::data_entry::DataEntry;
    pub use crate::data_key::DataCategory;
    pub use crate::data_key::DataKey;
    pub use crate::data_provider::DataProvider;
    pub use crate::data_provider::DataRequest;
    pub use crate::data_provider::DataResponse;
    pub use crate::data_provider::DataResponseBuilder;
    pub use crate::error::Error as DataError;
}

// Also include the same symbols at the top level for selective inclusion
pub use prelude::*;
