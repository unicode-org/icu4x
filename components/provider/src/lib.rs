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
}

// Also include the same symbols at the top level for selective inclusion
pub use prelude::*;

pub mod v2 {
    use crate::error::Error;
    use crate::prelude::*;
    use downcast_rs::Downcast;
    use std::any::Any;
    use std::any::TypeId;
    use std::borrow::Borrow;
    use std::borrow::BorrowMut;
    use std::borrow::Cow;
    use std::fmt::Debug;

    pub trait DataReceiver<'d, 'de> {
        fn set_to(
            &mut self,
            deserializer: &mut dyn erased_serde::Deserializer<'de>,
        ) -> Result<(), Error>;

        fn set_to_any(&mut self, any: &'d dyn Any) -> Result<(), Error>;

        fn borrow_payload_as_any(&self) -> Option<&dyn Any>;

        fn borrow_payload_as_serialize(&self) -> Option<&dyn erased_serde::Serialize>;

        fn borrow_payload_as_any_mut(&mut self) -> Option<&mut dyn Any>;
    }

    pub struct DataReceiverDecoder<'a, 'd, 'de>(
        pub &'a dyn DataReceiver<'d, 'de>
    );

    impl<'a, 'd, 'de> DataReceiverDecoder<'a, 'd, 'de> {
        pub fn borrow_payload<T: Any>(&self) -> Option<Result<&T, Error>> {
            let borrowed: Option<&dyn Any> = self.0.borrow_payload_as_any();
            borrowed.map(|any| {
                let downcasted: Option<&T> = any.downcast_ref();
                downcasted.ok_or_else(|| Error::MismatchedType {
                    actual: any.type_id(),
                    generic: Some(TypeId::of::<T>()),
                })
            })
        }
    }

    pub struct DataReceiverDecoderMut<'a, 'd, 'de>(
        pub &'a mut dyn DataReceiver<'d, 'de>
    );

    impl<'a, 'd, 'de> DataReceiverDecoderMut<'a, 'd, 'de> {
        pub fn borrow_payload_mut<T: Any>(&mut self) -> Option<Result<&mut T, Error>> {
            let borrowed: Option<&mut dyn Any> = self.0.borrow_payload_as_any_mut();
            borrowed.map(|any| {
                let actual_type_id = (any as &dyn Any).type_id();
                any.downcast_mut().ok_or_else(|| Error::MismatchedType {
                    actual: actual_type_id,
                    generic: Some(TypeId::of::<T>()),
                })
            })
        }
    }

    #[derive(Debug)]
    pub struct DataReceiverImpl<'d, T>
    where
        T: serde::Deserialize<'static> + erased_serde::Serialize + Any + Clone + Debug,
    {
        pub payload: Option<Cow<'d, T>>,
    }

    impl<'d, T> DataReceiver<'d, 'static> for DataReceiverImpl<'d, T>
    where
        T: serde::Deserialize<'static> + erased_serde::Serialize + Any + Clone + Debug,
    {
        fn set_to(
            &mut self,
            deserializer: &mut dyn erased_serde::Deserializer<'static>,
        ) -> Result<(), Error> {
            let obj: T = erased_serde::deserialize(deserializer)?;
            self.payload = Some(Cow::Owned(obj));
            Ok(())
        }

        fn set_to_any(&mut self, any: &'d dyn Any) -> Result<(), Error> {
            self.payload = Some(Cow::Borrowed(any.downcast_ref().unwrap()));
            Ok(())
        }

        fn borrow_payload_as_any(&self) -> Option<&dyn Any> {
            match &self.payload {
                Some(cow) => {
                    let borrowed: &T = cow.borrow();
                    Some(borrowed.as_any())
                }
                None => None,
            }
        }

        fn borrow_payload_as_serialize(&self) -> Option<&dyn erased_serde::Serialize> {
            match &self.payload {
                Some(cow) => {
                    let borrowed: &T = cow.borrow();
                    Some(borrowed as &dyn erased_serde::Serialize)
                }
                None => None,
            }
        }

        fn borrow_payload_as_any_mut(&mut self) -> Option<&mut dyn Any> {
            match &mut self.payload {
                Some(cow) => {
                    let borrowed: &mut T = cow.to_mut().borrow_mut();
                    Some(borrowed.as_any_mut())
                }
                None => None,
            }
        }
    }

    pub trait DataProviderV2<'d> {
        fn load_v2(
            &self,
            req: &DataRequest,
            receiver: &mut dyn DataReceiver<'d, 'static>,
        ) -> Result<(), Error>;
    }
}
