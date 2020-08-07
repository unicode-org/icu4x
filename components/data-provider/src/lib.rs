//! `icu-data-provider` is one of the [`ICU4X`] components.
//!
//! It defines traits and structs for transmitting data through the ICU4X locale data pipeline.
//! The primary trait is [`DataProvider`]. It has one method, which transforms a [`Request`] into
//! a [`Response`]:
//!
//! ```ignore
//! fn load<'a>(&'a self, req: &Request) -> Result<Response<'d>, Error>;
//! ```
//!
//! A Request contains a [`DataKey`] (a composition of a [`Category`] and sub-category, e.g.,
//! "plurals/cardinal@1") and [`DataEntry`] (a language identifier and optional variant, e.g.,
//! "fr") being requested. The Response contains the data payload corresponding to the Request.
//!
//! The most common types required for ICU4X DataProvider are included via the prelude:
//!
//! ```
//! use icu_data_provider::prelude::*;
//! use std::any::TypeId;
//!
//! // Types included directly:
//! println!("{:?}", TypeId::of::<DataKey>());
//! println!("{:?}", TypeId::of::<DataEntry>());
//! println!("{:?}", TypeId::of::<DataProvider>());
//!
//! // Types included via module namespace:
//! println!("{:?}", TypeId::of::<data_key::Category>());
//! println!("{:?}", TypeId::of::<data_provider::Request>());
//! println!("{:?}", TypeId::of::<data_provider::Response>());
//!
//! // Macros included:
//! assert_eq!("plurals/cardinal@1", icu_data_key!(plurals: cardinal@1).to_string());
//! ```
//!
//! ## Types of Data Providers
//!
//! Any object implementing DataProvider can be used to supply ICU4X with locale data. ICU4X ships
//! with some pre-built data providers:
//!
//! - [`FsDataProvider`][icu_fs_data_provider::FsDataProvider] reads structured data from the
//!   filesystem. It can also write out that filesystem structure.
//! - [`CldrJsonDataProvider`][icu_cldr_json_data_provider::CldrJsonDataProvider] reads structured
//!   data directly from CLDR source files.
//!
//! ## Iterable Data Providers
//!
//! Data providers can implement [`DataEntryCollection`], allowing them to be used via the
//! auto-implemented trait [`IterableDataProvider`]. This allows iteration over all DataEntry
//! instances supported for a certain key in the data provider. This can be useful when
//! transforming data between storage formats. For more information, see the [`iter`] module.
//!
//! ## InvariantDataProvider
//!
//! For testing or development purposes, this crate also offers [`InvariantDataProvider`], which
//! returns fixed data that does not vary by locale. You must enable InvariantDataProvider via the
//! `"invariant"` feature in your Cargo.toml file.

pub mod data_entry;
pub mod data_key;
pub mod data_provider;
pub mod error;
pub mod iter;
pub mod structs;

#[cfg(feature = "invariant")]
pub(crate) mod invariant;
#[cfg(feature = "invariant")]
pub mod validator;
#[cfg(feature = "invariant")]
pub use invariant::InvariantDataProvider;

mod cloneable_any;

pub mod prelude {
    pub use crate::data_entry::DataEntry;
    pub use crate::data_key;
    pub use crate::data_key::DataKey;
    pub use crate::data_provider;
    pub use crate::data_provider::DataProvider;
    pub use crate::icu_data_key;
}
