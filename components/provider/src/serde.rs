// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::error::Error;
use crate::prelude::*;
use std::borrow::Cow;
use std::fmt::Debug;

/// A receiver capable of accepting data from a Serde Deserializer.
///
/// Lifetimes:
///
/// - `'de` = deserializer lifetime; can usually be `'_`
pub trait SerdeDataReceiver<'de> {
    /// Consumes a Serde Deserializer into this SerdeDataReceiver as owned data.
    ///
    /// This method results in an owned payload, but the payload could have non-static references
    /// according to the deserializer lifetime.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_provider::prelude::*;
    /// use icu_provider::serde::DataReceiver;
    /// use icu_provider::serde::SerdeDataReceiver;
    /// use std::borrow::Cow;
    ///
    /// const JSON: &'static str = "\"hello world\"";
    ///
    /// let mut receiver = DataReceiver::<String>::new();
    /// let mut d = serde_json::Deserializer::from_str(JSON);
    /// receiver.receive_deserializer(&mut erased_serde::Deserializer::erase(&mut d))
    ///     .expect("Deserialization should be successful");
    ///
    /// assert!(matches!(receiver.payload, Some(Cow::Owned(_))));
    /// assert_eq!("hello world", *receiver.borrow_payload().unwrap());
    /// ```
    fn receive_deserializer(
        &mut self,
        deserializer: &mut dyn erased_serde::Deserializer<'de>,
    ) -> Result<(), Error>;
}

/// Concrete struct backing SerdeDataReceiver.
///
/// # Example
///
/// ```
/// use icu_provider::prelude::*;
/// use icu_provider::serde::DataReceiver;
///
/// let mut string_receiver = DataReceiver::<String>::new();
/// // Now pass string_receiver as an argument to SerdeDataProvider
/// ```
#[derive(Debug)]
pub struct DataReceiver<'d, T>
where
    T: ToOwned + ?Sized,
    <T as ToOwned>::Owned: Debug,
{
    pub payload: Option<Cow<'d, T>>,
}

impl<'d, T> DataReceiver<'d, T>
where
    T: ToOwned + ?Sized,
    <T as ToOwned>::Owned: Debug,
{
    /// Creates a new, empty DataReceiver.
    ///
    /// Default is not implemented because it would be misleading: does the DataReceiver start
    /// empty, or does it start with the Default value of T?
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { payload: None }
    }

    /// Convenience method: borrows the payload from the underlying Cow. Error if not present.
    pub fn borrow_payload(&self) -> Result<&T, Error> {
        use std::borrow::Borrow;
        self.payload
            .as_ref()
            .map(|cow| cow.borrow())
            .ok_or(Error::MissingPayload)
    }

    /// Convenience method: takes ownership of the underlying payload. Error if not present.
    pub fn take_payload(&mut self) -> Result<Cow<'d, T>, Error> {
        self.payload.take().ok_or(Error::MissingPayload)
    }
}

impl<'d, 'de, T> SerdeDataReceiver<'de> for DataReceiver<'d, T>
where
    T: serde::Deserialize<'de> + Clone + Debug,
{
    fn receive_deserializer(
        &mut self,
        deserializer: &mut dyn erased_serde::Deserializer<'de>,
    ) -> Result<(), Error> {
        let obj: T = erased_serde::deserialize(deserializer)?;
        self.payload = Some(Cow::Owned(obj));
        Ok(())
    }
}

/// A type-erased data provider that loads payloads from a Serde Deserializer.
///
/// Uses erased_serde to allow the trait to be object-safe.
pub trait SerdeDataProvider<'de> {
    /// Query the provider for data, loading it into a SerdeDataReceiver.
    ///
    /// Returns Ok if the request successfully loaded data. If data failed to load, returns an
    /// Error with more information.
    fn load_to_receiver(
        &self,
        req: &DataRequest,
        receiver: &mut dyn SerdeDataReceiver<'de>,
    ) -> Result<DataResponseMetadata, Error>;
}

/// Convenience implementation of DataProvider<T> given a SerdeDataProvider trait object.
impl<'a, 'd, 'de, T> DataProvider<'d, T> for dyn SerdeDataProvider<'de> + 'a
where
    T: serde::Deserialize<'de> + Clone + Debug,
{
    fn load_payload(&self, req: &DataRequest) -> Result<DataResponse<'d, T>, Error> {
        let mut receiver = DataReceiver::<T>::new();
        let metadata = self.load_to_receiver(req, &mut receiver)?;
        Ok(DataResponse {
            metadata,
            payload: receiver.payload,
        })
    }
}
