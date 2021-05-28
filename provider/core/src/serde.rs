// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Collection of traits for providers that support serializing or deserializing data.
//!
//! ## Deserializing
//!
//! Providers that involve a [`serde::Deserializer`] to produce data from an opaque source should
//! implement [`SerdeDeDataProvider`]. For example, `FsDataProvider` implements [`SerdeDeDataProvider`].
//!
//! [`SerdeDeDataProvider`] can be made into a trait object. It is used over FFI.
//!
//! ## Serializing
//!
//! Providers that have full type information should implement [`DataProvider`]`<dyn `[`SerdeSeDataStruct`]`>`.
//! Note that a provider like `FsDataProvider` cannot implement that trait, because type information
//! on the data structs is required in order to deserialize and then serialize them.
//!
//! [`DataProvider`]`<dyn `[`SerdeSeDataStruct`]`>` is used by data exporters such as `FilesystemExporter`.

use crate::error::Error;
use crate::prelude::*;
use std::borrow::Cow;
use std::fmt::Debug;

/// An object that receives data from a Serde Deserializer. Implemented by [`DataPayload`].
///
/// Lifetimes:
///
/// - `'de` = deserializer lifetime; can usually be `'_`
pub trait SerdeDeDataReceiver<'de> {
    /// Consumes a Serde Deserializer into this SerdeDeDataReceiver as owned data.
    ///
    /// This method results in an owned payload, but the payload could have non-static references
    /// according to the deserializer lifetime.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_provider::prelude::*;
    /// use icu_provider::serde::SerdeDeDataReceiver;
    ///
    /// const JSON: &'static str = "\"hello world\"";
    ///
    /// let mut receiver: Option<&str> = None;
    /// let mut d = serde_json::Deserializer::from_str(JSON);
    /// receiver.receive_deserializer(&mut erased_serde::Deserializer::erase(&mut d))
    ///     .expect("Deserialization should be successful");
    ///
    /// assert!(matches!(receiver, Some(_)));
    /// assert_eq!(receiver, Some("hello world"));
    /// ```
    fn receive_deserializer(
        &mut self,
        deserializer: &mut dyn erased_serde::Deserializer<'de>,
    ) -> Result<(), Error>;
}

impl<'de, T> SerdeDeDataReceiver<'de> for Option<T>
where
    T: serde::Deserialize<'de> + Clone + Debug,
{
    fn receive_deserializer(
        &mut self,
        deserializer: &mut dyn erased_serde::Deserializer<'de>,
    ) -> Result<(), Error> {
        let obj: T = erased_serde::deserialize(deserializer)?;
        self.replace(obj);
        Ok(())
    }
}

/// A type-erased data provider that loads payloads from a Serde Deserializer.
///
/// Uses [`erased_serde`] to allow the trait to be object-safe.
pub trait SerdeDeDataProvider<'de> {
    /// Query the provider for data, loading it into a [`SerdeDeDataReceiver`].
    ///
    /// Returns Ok if the request successfully loaded data. If data failed to load, returns an
    /// Error with more information.
    fn load_to_receiver(
        &self,
        req: &DataRequest,
        receiver: &mut dyn SerdeDeDataReceiver<'de>,
    ) -> Result<DataResponseMetadata, Error>;
}

impl<'d, 'de, T> DataProvider<'d, T> for dyn SerdeDeDataProvider<'de> + 'd
where
    T: serde::Deserialize<'de> + Clone + Debug,
{
    /// Serve objects implementing [`serde::Deserialize<'de>`] from a [`SerdeDeDataProvider`].
    fn load_payload(&self, req: &DataRequest) -> Result<DataResponse<'d, T>, Error> {
        let mut payload = None;
        let metadata = self.load_to_receiver(req, &mut payload)?;
        Ok(DataResponse {
            metadata,
            payload: payload.map(|obj| DataPayload::from_owned(obj)),
        })
    }
}

/// Auto-implemented trait for all data structs that support [`serde::Serialize`]. This trait is
/// usually used as a trait object in [`DataProvider`]`<dyn `[`SerdeSeDataStruct`]`>`.
pub trait SerdeSeDataStruct<'s>: 's + Debug {
    /// Clone this trait object reference, returning a boxed trait object.
    fn clone_into_box(&self) -> Box<dyn SerdeSeDataStruct<'s> + 's>;

    /// Return this trait object reference for Serde serialization.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_provider::serde::SerdeSeDataStruct;
    /// use icu_provider::hello_world::HelloWorldV1;
    ///
    /// // Create type-erased reference
    /// let data = HelloWorldV1::default();
    /// let erased: &dyn SerdeSeDataStruct = &data;
    ///
    /// // Borrow as serialize trait object
    /// let serialize: &dyn erased_serde::Serialize = erased.as_serialize();
    ///
    /// // Serialize the object to a JSON string
    /// let mut buffer: Vec<u8> = vec![];
    /// serialize.erased_serialize(
    ///     &mut erased_serde::Serializer::erase(
    ///         &mut serde_json::Serializer::new(&mut buffer)
    ///     )
    /// ).expect("Serialization should succeed");
    /// assert_eq!("{\"message\":\"(und) Hello World\"}".as_bytes(), buffer);
    /// ```
    fn as_serialize(&self) -> &dyn erased_serde::Serialize;
}

impl_dyn_clone!(SerdeSeDataStruct<'s>, 's);

impl_dyn_from_payload!(SerdeSeDataStruct<'s>, 'd, 's);

impl<'s, T> SerdeSeDataStruct<'s> for T
where
    T: 's + serde::Serialize + Clone + Debug,
{
    fn clone_into_box(&self) -> Box<dyn SerdeSeDataStruct<'s> + 's> {
        Box::new(self.clone())
    }
    fn as_serialize(&self) -> &dyn erased_serde::Serialize {
        self
    }
}
