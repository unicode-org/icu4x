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
//! [`DataProvider`]: prelude::DataProvider
//! [`Request`]: prelude::DataRequest
//! [`Response`]: prelude::DataResponse
//! [`DataKey`]: prelude::DataKey
//! [`Category`]: prelude::DataCategory
//! [`DataEntry`]: prelude::DataEntry
//! [`DataEntryCollection`]: iter::DataEntryCollection
//! [`IterableDataProvider`]: iter::IterableDataProvider
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

    pub use crate::v2::DataProviderV2;
    pub use crate::v2::DataReceiver;
    pub use crate::v2::DataReceiverForType;
    pub use crate::v2::DataResponseV2;
}

// Also include the same symbols at the top level for selective inclusion
pub use prelude::*;

pub mod v2 {
    use crate::error::Error;
    use crate::prelude::*;
    use icu_locid::LanguageIdentifier;
    use std::any::Any;
    use std::any::TypeId;
    use std::borrow::Borrow;
    use std::borrow::Cow;
    use std::fmt::Debug;

    pub trait DataReceiver<'d, 'de> {
        fn receive_deserializer(
            &mut self,
            deserializer: &mut dyn erased_serde::Deserializer<'de>,
        ) -> Result<(), erased_serde::Error>;

        fn receive_borrow(&mut self, borrowed_any: &'d dyn Any) -> Result<(), Error>;

        fn receive_box(&mut self, boxed_any: Box<dyn Any>) -> Result<(), Error>;

        fn receive_option(&mut self, option_any: &mut dyn Any) -> Result<(), Error>;

        fn as_serialize(&self) -> Option<&dyn erased_serde::Serialize>;
    }

    #[derive(Debug)]
    pub struct DataReceiverForType<'d, T>
    where
        T: Clone + Debug,
    {
        pub payload: Option<Cow<'d, T>>,
    }

    impl<'d, T> Default for DataReceiverForType<'d, T>
    where
        T: Clone + Debug,
    {
        fn default() -> Self {
            Self { payload: None }
        }
    }

    impl<'d, T> DataReceiverForType<'d, T>
    where
        T: Clone + Debug,
    {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn borrow_payload(&self) -> Option<&T> {
            self.payload.as_ref().map(|cow| cow.borrow())
        }
    }

    impl<'d, T> DataReceiver<'d, 'static> for DataReceiverForType<'d, T>
    where
        T: serde::Deserialize<'static> + erased_serde::Serialize + Any + Clone + Debug,
    {
        fn receive_deserializer(
            &mut self,
            deserializer: &mut dyn erased_serde::Deserializer<'static>,
        ) -> Result<(), erased_serde::Error> {
            let obj: T = erased_serde::deserialize(deserializer)?;
            self.payload = Some(Cow::Owned(obj));
            Ok(())
        }

        fn receive_borrow(&mut self, borrowed_any: &'d dyn Any) -> Result<(), Error> {
            let borrowed: &T =
                borrowed_any
                    .downcast_ref()
                    .ok_or_else(|| Error::MismatchedType {
                        actual: Some(borrowed_any.type_id()),
                        generic: Some(TypeId::of::<T>()),
                    })?;
            self.payload = Some(Cow::Borrowed(borrowed));
            Ok(())
        }

        fn receive_box(&mut self, boxed_any: Box<dyn Any>) -> Result<(), Error> {
            let boxed: Box<T> = boxed_any.downcast().map_err(|any| Error::MismatchedType {
                actual: Some(any.type_id()),
                generic: Some(TypeId::of::<T>()),
            })?;
            self.payload = Some(Cow::Owned(*boxed));
            Ok(())
        }

        fn receive_option(&mut self, option_any: &mut dyn Any) -> Result<(), Error> {
            let option: &mut Option<T> =
                option_any
                    .downcast_mut()
                    .ok_or_else(|| Error::MismatchedType {
                        actual: None,
                        generic: Some(TypeId::of::<T>()),
                    })?;
            self.payload = option.take().map(|t| Cow::Owned(t));
            Ok(())
        }

        fn as_serialize(&self) -> Option<&dyn erased_serde::Serialize> {
            match &self.payload {
                Some(cow) => {
                    let borrowed: &T = cow.borrow();
                    Some(borrowed as &dyn erased_serde::Serialize)
                }
                None => None,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct DataResponseV2 {
        pub data_langid: LanguageIdentifier,
    }

    pub trait DataProviderV2<'d> {
        fn load_v2(
            &self,
            req: &DataRequest,
            receiver: &mut dyn DataReceiver<'d, 'static>,
        ) -> Result<DataResponseV2, Error>;
    }
}
