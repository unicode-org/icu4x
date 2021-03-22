// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Collection of traits for providers that support *deserializing* data via Serde.
//!
//! See the `crate::export::serde` mod for APIs involving *serializing* data via Serde.

use crate::error::Error;
use crate::prelude::*;
use std::borrow::Cow;
use std::fmt::Debug;

/// A receiver capable of accepting data from a Serde Deserializer.
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
    /// # Example
    ///
    /// ```
    /// use icu_provider::prelude::*;
    /// use icu_provider::serde::SerdeDeDataReceiver;
    /// use std::borrow::Cow;
    ///
    /// const JSON: &'static str = "\"hello world\"";
    ///
    /// let mut receiver = DataPayload::<String>::new();
    /// let mut d = serde_json::Deserializer::from_str(JSON);
    /// receiver.receive_deserializer(&mut erased_serde::Deserializer::erase(&mut d))
    ///     .expect("Deserialization should be successful");
    ///
    /// assert!(matches!(receiver.cow, Some(Cow::Owned(_))));
    /// assert_eq!("hello world", *receiver.borrow().unwrap());
    /// ```
    fn receive_deserializer(
        &mut self,
        deserializer: &mut dyn erased_serde::Deserializer<'de>,
    ) -> Result<(), Error>;
}

impl<'d, 'de, T> SerdeDeDataReceiver<'de> for DataPayload<'d, T>
where
    T: serde::Deserialize<'de> + Clone + Debug,
{
    fn receive_deserializer(
        &mut self,
        deserializer: &mut dyn erased_serde::Deserializer<'de>,
    ) -> Result<(), Error> {
        let obj: T = erased_serde::deserialize(deserializer)?;
        self.cow = Some(Cow::Owned(obj));
        Ok(())
    }
}

/// A type-erased data provider that loads payloads from a Serde Deserializer.
///
/// Uses erased_serde to allow the trait to be object-safe.
pub trait SerdeDeDataProvider<'de> {
    /// Query the provider for data, loading it into a SerdeDeDataReceiver.
    ///
    /// Returns Ok if the request successfully loaded data. If data failed to load, returns an
    /// Error with more information.
    fn load_to_receiver(
        &self,
        req: &DataRequest,
        receiver: &mut dyn SerdeDeDataReceiver<'de>,
    ) -> Result<DataResponseMetadata, Error>;
}

/// Implementation of DataProvider<T> given a SerdeDeDataProvider trait object.
impl<'a, 'd, 'de, T> DataProvider<'d, T> for dyn SerdeDeDataProvider<'de> + 'a
where
    T: serde::Deserialize<'de> + Clone + Debug,
{
    fn load_payload(&self, req: &DataRequest) -> Result<DataResponse<'d, T>, Error> {
        let mut payload = DataPayload::<T>::new();
        let metadata = self.load_to_receiver(req, &mut payload)?;
        Ok(DataResponse { metadata, payload })
    }
}

