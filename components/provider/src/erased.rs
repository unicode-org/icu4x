// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use crate::error::Error;
use crate::prelude::*;
use std::any::Any;
use std::any::TypeId;
use std::borrow::Cow;
use std::fmt::Debug;

#[cfg(feature = "invariant")]
use std::default::Default;

/// Re-export erased_serde for the impl_erased! macro
pub use erased_serde;

/// Auto-implemented trait allowing for type erasure of data provider structs. Requires the
/// static lifetime in order to be convertible to Any.
pub trait ErasedDataStruct: 'static {
    /// Clone this trait object reference, returning a boxed trait object.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_provider::erased::ErasedDataStruct;
    /// use icu_provider::structs::icu4x::HelloV1;
    ///
    /// // Create type-erased reference
    /// let data = HelloV1::default();
    /// let erased_reference: &dyn ErasedDataStruct = &data;
    ///
    /// // Create a new type-erased trait object
    /// let erased_boxed: Box<dyn ErasedDataStruct> = erased_reference.clone_into_box();
    /// ```
    fn clone_into_box(&self) -> Box<dyn ErasedDataStruct>;

    /// Return this boxed trait object as Box<dyn Any>.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_provider::erased::ErasedDataStruct;
    /// use icu_provider::structs::icu4x::HelloV1;
    ///
    /// // Create type-erased box
    /// let erased: Box<dyn ErasedDataStruct> = Box::new(HelloV1::default());
    ///
    /// // Convert to typed box
    /// let boxed: Box<HelloV1> = erased.into_any().downcast().expect("Types should match");
    /// ```
    fn into_any(self: Box<Self>) -> Box<dyn Any>;

    /// Return this trait object reference as &dyn Any.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_provider::erased::ErasedDataStruct;
    /// use icu_provider::structs::icu4x::HelloV1;
    ///
    /// // Create type-erased reference
    /// let data = HelloV1::default();
    /// let erased: &dyn ErasedDataStruct = &data;
    ///
    /// // Borrow as typed reference
    /// let borrowed: &HelloV1 = erased.as_any().downcast_ref().expect("Types should match");
    /// ```
    fn as_any(&self) -> &dyn Any;

    /// Return this trait object reference for Serde serialization.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_provider::erased::ErasedDataStruct;
    /// use icu_provider::structs::icu4x::HelloV1;
    ///
    /// // Create type-erased reference
    /// let data = HelloV1::default();
    /// let erased: &dyn ErasedDataStruct = &data;
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
    /// assert_eq!("{\"hello\":\"(und) Hello World\"}".as_bytes(), buffer);
    /// ```
    fn as_serialize(&self) -> &dyn erased_serde::Serialize;
}

impl ToOwned for dyn ErasedDataStruct {
    type Owned = Box<dyn ErasedDataStruct>;

    fn to_owned(&self) -> Self::Owned {
        self.clone_into_box()
    }
}

impl Clone for Box<dyn ErasedDataStruct> {
    fn clone(&self) -> Box<dyn ErasedDataStruct> {
        self.clone_into_box()
    }
}

impl<T> ErasedDataStruct for T
where
    T: erased_serde::Serialize + Clone + Any,
{
    fn clone_into_box(&self) -> Box<dyn ErasedDataStruct> {
        Box::new(self.clone())
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_serialize(&self) -> &dyn erased_serde::Serialize {
        self
    }
}

/// An object capable of accepting data from a variety of forms.
/// Lifetimes:
/// - 'd = lifetime of borrowed data (Cow::Borrowed)
/// - 'de = lifetime parameter of owned data (Cow::Owned)
pub trait ErasedDataReceiver<'d> {
    /// Consumes a Serde Deserializer into this ErasedDataReceiver as owned data.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_provider::prelude::*;
    /// use std::borrow::Cow;
    ///
    /// const JSON: &'static str = "\"hello world\"";
    ///
    /// let mut receiver = DataReceiver::<String>::new();
    /// let mut d = serde_json::Deserializer::from_str(JSON);
    /// receiver.receive_deserializer(&mut erased_serde::Deserializer::erase(&mut d))
    ///     .expect("Deserialization should be successful");
    ///
    /// assert_eq!(receiver.payload, Some(Cow::Owned("hello world".to_string())));
    /// ```
    fn receive_deserializer(
        &mut self,
        deserializer: &mut dyn erased_serde::Deserializer<'d>,
    ) -> Result<(), erased_serde::Error>;

    /// Sets the payload to the default value. Requires Default to be implemented for the type.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_provider::prelude::*;
    /// use std::borrow::Cow;
    ///
    /// let mut receiver = DataReceiver::<String>::new();
    /// receiver.receive_default().expect("Default is implemented for String");
    ///
    /// assert_eq!(receiver.payload, Some(Cow::Owned("".to_string())));
    /// ```
    fn receive_default(&mut self) -> Result<(), Error>;

    fn receive_erased(&mut self, erased: Cow<'d, dyn ErasedDataStruct>) -> Result<(), Error>;

    fn take_erased(&mut self) -> Option<Cow<'d, dyn ErasedDataStruct>>;
}

impl<'a, 'd> dyn ErasedDataReceiver<'d> + 'a {
    pub fn receive_payload<T>(&mut self, payload: Cow<'d, T>) -> Result<(), Error>
    where
        T: erased_serde::Serialize + Clone + Any,
    {
        self.receive_erased(match payload {
            Cow::Borrowed(borrowed) => Cow::Borrowed(borrowed),
            Cow::Owned(owned) => Cow::Owned(Box::new(owned) as Box<dyn ErasedDataStruct>),
        })
    }
}

/// Concrete implementation of ErasedDataReceiver parameterized for a certain type.
///
/// # Example
///
/// ```
/// use icu_provider::prelude::*;
///
/// let mut string_receiver = DataReceiver::<String>::new();
/// ```
#[derive(Debug)]
pub struct DataReceiver<'d, T>
where
    T: Clone + Debug,
{
    pub payload: Option<Cow<'d, T>>,
}

#[cfg(feature = "invariant")]
impl<'d, T> Default for DataReceiver<'d, T>
where
    T: Clone + Debug + Default,
{
    /// Creates a new ErasedDataReceiver with the Default data pre-loaded.
    fn default() -> Self {
        Self {
            payload: Default::default(),
        }
    }
}

impl<'d, T> DataReceiver<'d, T>
where
    T: Clone + Debug,
{
    /// Creates a new, empty ErasedDataReceiver.
    pub fn new() -> Self {
        Self { payload: None }
    }

    /// Convenience method: borrows the payload from the underlying Cow.
    pub fn borrow_payload(&self) -> Result<&T, Error> {
        use std::borrow::Borrow;
        self.payload
            .as_ref()
            .map(|cow| cow.borrow())
            .ok_or(Error::MissingPayload)
    }

    /// Convenience method: consumes self and returns the underlying Cow.
    pub fn take_payload(self) -> Result<Cow<'d, T>, Error> {
        self.payload.ok_or(Error::MissingPayload)
    }
}

impl<'d, T> DataReceiver<'d, T>
where
    T: serde::Deserialize<'d> + serde::Serialize + Clone + Debug + Any + Default,
{
    /// Creates a new, empty ErasedDataReceiver, returning it as a boxed trait object.
    pub fn new_boxed() -> Box<dyn ErasedDataReceiver<'d> + 'd> {
        let receiver: DataReceiver<'d, T> = Self::new();
        Box::new(receiver)
    }
}

impl<'d, T> ErasedDataReceiver<'d> for DataReceiver<'d, T>
where
    T: serde::Deserialize<'d> + serde::Serialize + Clone + Debug + Any + Default,
{
    fn receive_deserializer(
        &mut self,
        deserializer: &mut dyn erased_serde::Deserializer<'d>,
    ) -> Result<(), erased_serde::Error> {
        let obj: T = erased_serde::deserialize(deserializer)?;
        self.payload = Some(Cow::Owned(obj));
        Ok(())
    }

    fn receive_default(&mut self) -> Result<(), Error> {
        self.payload = Some(Cow::Owned(T::default()));
        Ok(())
    }

    fn receive_erased(&mut self, cow: Cow<'d, dyn ErasedDataStruct>) -> Result<(), Error> {
        match cow {
            Cow::Borrowed(erased) => {
                let borrowed: &'d T =
                    erased
                        .as_any()
                        .downcast_ref()
                        .ok_or_else(|| Error::MismatchedType {
                            actual: Some(erased.type_id()),
                            generic: Some(TypeId::of::<T>()),
                        })?;
                self.payload = Some(Cow::Borrowed(borrowed));
            }
            Cow::Owned(erased) => {
                let boxed: Box<T> =
                    erased
                        .into_any()
                        .downcast()
                        .map_err(|any| Error::MismatchedType {
                            actual: Some(any.type_id()),
                            generic: Some(TypeId::of::<T>()),
                        })?;
                self.payload = Some(Cow::Owned(*boxed));
            }
        };
        Ok(())
    }

    fn take_erased(&mut self) -> Option<Cow<'d, dyn ErasedDataStruct>> {
        match self.payload.take() {
            Some(cow) => match cow {
                Cow::Borrowed(borrowed) => Some(Cow::Borrowed(borrowed)),
                Cow::Owned(owned) => {
                    let boxed: Box<dyn ErasedDataStruct> = Box::new(owned);
                    Some(Cow::Owned(boxed))
                }
            },
            None => None,
        }
    }
}

/// A type-erased data provider that loads a payload of any type.
pub trait ErasedDataProvider<'d> {
    /// Query the provider for data, loading it into a ErasedDataReceiver.
    ///
    /// Returns Ok if the request successfully loaded data. If data failed to load, returns an
    /// Error with more information.
    fn load_to_receiver(
        &self,
        req: &DataRequest,
        receiver: &mut dyn ErasedDataReceiver<'d>,
    ) -> Result<DataResponseMetadata, Error>;

    /// Query the provider for data, returning it as a boxed Serialize trait object.
    ///
    /// Returns Ok if the request successfully loaded data. If data failed to load, returns an
    /// Error with more information.
    fn load_as_serialize(
        &self,
        req: &DataRequest,
    ) -> Result<Box<dyn erased_serde::Serialize>, Error>;
}

/// Helper macro to implement ErasedDataProvider on an object implementing DataProvider for a
/// single type. Calls to `self.load_to_receiver` delegate to `self.load_payload`.
#[macro_export]
macro_rules! impl_erased {
    ($type:ty, $lifetime:tt) => {
        impl<$lifetime> $crate::prelude::ErasedDataProvider<$lifetime> for $type {
            fn load_to_receiver(
                &self,
                req: &$crate::prelude::DataRequest,
                receiver: &mut dyn $crate::prelude::ErasedDataReceiver<$lifetime>,
            ) -> Result<$crate::prelude::DataResponseMetadata, $crate::prelude::DataError> {
                let mut result = self.load_payload(req)?;
                receiver.receive_payload(result.take_payload()?)?;
                Ok(result.metadata)
            }

            fn load_as_serialize(
                &self,
                req: &$crate::prelude::DataRequest,
            ) -> Result<Box<dyn $crate::erased::erased_serde::Serialize>, $crate::prelude::DataError> {
                let mut result = self.load_payload(req)?;
                Ok(Box::new(result.take_payload()?.into_owned()))
            }
        }
    };
}

/// Convenience implementation of DataProvider<T> given an ErasedDataProvider trait object.
impl<'a, 'd, T> DataProvider<'d, T> for dyn ErasedDataProvider<'d> + 'a
where
    T: serde::Deserialize<'d> + serde::Serialize + Clone + Debug + Any + Default,
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
