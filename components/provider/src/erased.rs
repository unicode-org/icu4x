// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use crate::error::Error;
use crate::prelude::*;
use std::any::Any;
use std::any::TypeId;
use std::borrow::Cow;
use std::default::Default;
use std::fmt::Debug;

/// Re-export erased_serde for the impl_erased! macro
pub use erased_serde;

/// Auto-implemented trait allowing for type erasure of data provider structs. Requires the
/// static lifetime in order to be convertible to Any.
pub trait ErasedDataStruct: 'static + Debug {
    /// Clone this trait object reference, returning a boxed trait object.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_provider::erased::ErasedDataStruct;
    /// use icu_provider::structs::icu4x::HelloWorldV1;
    ///
    /// // Create type-erased reference
    /// let data = HelloWorldV1::default();
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
    /// use icu_provider::structs::icu4x::HelloWorldV1;
    ///
    /// // Create type-erased box
    /// let erased: Box<dyn ErasedDataStruct> = Box::new(HelloWorldV1::default());
    ///
    /// // Convert to typed box
    /// let boxed: Box<HelloWorldV1> = erased.into_any().downcast().expect("Types should match");
    /// ```
    fn into_any(self: Box<Self>) -> Box<dyn Any>;

    /// Return this trait object reference as &dyn Any.
    ///
    /// Also see associated method downcast_ref().
    ///
    /// # Example
    ///
    /// ```
    /// use icu_provider::erased::ErasedDataStruct;
    /// use icu_provider::structs::icu4x::HelloWorldV1;
    ///
    /// // Create type-erased reference
    /// let data = HelloWorldV1::default();
    /// let erased: &dyn ErasedDataStruct = &data;
    ///
    /// // Borrow as typed reference
    /// let borrowed: &HelloWorldV1 = erased.as_any().downcast_ref().expect("Types should match");
    /// ```
    fn as_any(&self) -> &dyn Any;

    /// Return this trait object reference for Serde serialization.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_provider::erased::ErasedDataStruct;
    /// use icu_provider::structs::icu4x::HelloWorldV1;
    ///
    /// // Create type-erased reference
    /// let data = HelloWorldV1::default();
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
    /// assert_eq!("{\"message\":\"(und) Hello World\"}".as_bytes(), buffer);
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

impl dyn ErasedDataStruct {
    /// Convenience function: Return a downcast reference, or an error if mismatched types.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_provider::erased::ErasedDataStruct;
    /// use icu_provider::structs::icu4x::HelloWorldV1;
    ///
    /// // Create type-erased reference
    /// let data = HelloWorldV1::default();
    /// let erased: &dyn ErasedDataStruct = &data;
    ///
    /// // Borrow as typed reference
    /// let borrowed: &HelloWorldV1 = erased.downcast_ref().expect("Types should match");
    /// ```
    pub fn downcast_ref<T: Any>(&self) -> Result<&T, Error> {
        self.as_any()
            .downcast_ref()
            .ok_or_else(|| Error::MismatchedType {
                actual: Some(self.as_any().type_id()),
                generic: Some(TypeId::of::<T>()),
            })
    }
}

impl<T> ErasedDataStruct for T
where
    T: erased_serde::Serialize + Clone + Debug + Any,
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

/// An receiver capable of accepting type-erased data.
///
/// Lifetimes:
///
/// - `'d` = lifetime of borrowed data (Cow::Borrowed)
/// - `'de` = deserializer lifetime; can usually be `'_`
pub trait ErasedDataReceiver<'d, 'de> {
    /// Consumes a Serde Deserializer into this ErasedDataReceiver as owned data.
    ///
    /// This method results in an owned payload, but the payload could have non-static references
    /// according to the deserializer lifetime.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_provider::prelude::*;
    /// use icu_provider::erased::DataReceiver;
    /// use icu_provider::erased::ErasedDataReceiver;
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

    /// Sets the payload to the default value. Assumes Default is implemented for the type.
    ///
    /// This method results in an owned payload.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_provider::prelude::*;
    /// use icu_provider::erased::DataReceiver;
    /// use icu_provider::erased::ErasedDataReceiver;
    /// use std::borrow::Cow;
    ///
    /// let mut receiver = DataReceiver::<String>::new();
    /// receiver.receive_default().expect("Default is implemented for String");
    ///
    /// assert!(matches!(receiver.payload, Some(Cow::Owned(_))));
    /// assert_eq!("", *receiver.borrow_payload().unwrap());
    /// ```
    fn receive_default(&mut self) -> Result<(), Error>;

    /// Sets the payload to the value contained in the given Cow of `ErasedDataStruct`. May be
    /// owned or borrowed.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_provider::prelude::*;
    /// use icu_provider::erased::DataReceiver;
    /// use icu_provider::erased::ErasedDataReceiver;
    /// use std::borrow::Cow;
    ///
    /// let local_data = "hello world".to_string();
    ///
    /// let mut receiver = DataReceiver::<String>::new();
    /// receiver.receive_erased(Cow::Borrowed(&local_data)).expect("Types should match");
    ///
    /// assert!(matches!(receiver.payload, Some(Cow::Borrowed(_))));
    /// assert_eq!("hello world", *receiver.borrow_payload().unwrap());
    /// ```
    fn receive_erased(&mut self, erased: Cow<'d, dyn ErasedDataStruct>) -> Result<(), Error>;

    /// Takes the payload as a Cow of `ErasedDataStruct`. Error if not present.
    ///
    /// This method is useful if you have an ErasedDataReceiver trait object. If you have a fully
    /// typed DataReceiver, you should use take_payload() instead.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_provider::prelude::*;
    /// use icu_provider::erased::DataReceiver;
    /// use icu_provider::erased::ErasedDataReceiver;
    /// use std::borrow::Cow;
    ///
    /// let mut receiver = DataReceiver::<String>::new();
    /// receiver.receive_default().expect("Default is implemented for String");
    ///
    /// assert!(receiver.has_payload());
    /// let erased_payload = receiver.take_erased().unwrap();
    /// assert!(!receiver.has_payload());
    ///
    /// assert_eq!("", erased_payload.as_any().downcast_ref::<String>().unwrap());
    /// ```
    fn take_erased(&mut self) -> Result<Cow<'d, dyn ErasedDataStruct>, Error>;

    /// Returns whether the receiver has a payload.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_provider::prelude::*;
    /// use icu_provider::erased::DataReceiver;
    /// use icu_provider::erased::ErasedDataReceiver;
    /// use std::borrow::Cow;
    ///
    /// let mut receiver = DataReceiver::<String>::new();
    /// assert!(!receiver.has_payload());
    ///
    /// receiver.receive_default().expect("Default is implemented for String");
    /// assert!(receiver.has_payload());
    /// ```
    fn has_payload(&self) -> bool;

    /// Discards the payload if present.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_provider::prelude::*;
    /// use icu_provider::erased::DataReceiver;
    /// use icu_provider::erased::ErasedDataReceiver;
    /// use std::borrow::Cow;
    ///
    /// let mut receiver = DataReceiver::<String>::new();
    /// receiver.receive_default().expect("Default is implemented for String");
    ///
    /// assert!(receiver.has_payload());
    /// receiver.reset();
    /// assert!(!receiver.has_payload());
    /// ```
    fn reset(&mut self);
}

impl<'a, 'd> dyn ErasedDataReceiver<'d, '_> + 'a {
    /// Convenience method: sets the payload to the value contained in the given Cow of a concrete
    /// type. May be owned or borrowed.
    ///
    /// This method is useful for transferring payloads between receivers.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_provider::prelude::*;
    /// use icu_provider::erased::DataReceiver;
    /// use icu_provider::erased::ErasedDataReceiver;
    /// use std::borrow::Cow;
    ///
    /// let mut receiver1 = DataReceiver::<f64>::new();
    /// let mut receiver2 = DataReceiver::<f64>::new();
    ///
    /// receiver1.receive_default();
    /// assert!(receiver1.payload.is_some());
    /// assert_eq!(0.0, *receiver1.borrow_payload().unwrap());
    ///
    /// (&mut receiver2 as &mut dyn ErasedDataReceiver)
    ///     .receive_payload(receiver1.take_payload().unwrap());
    ///
    /// assert!(receiver1.payload.is_none());
    /// assert_eq!(0.0, *receiver2.borrow_payload().unwrap());
    /// ```
    pub fn receive_payload<T>(&mut self, payload: Cow<'d, T>) -> Result<(), Error>
    where
        T: erased_serde::Serialize + Clone + Debug + Any,
    {
        self.receive_erased(match payload {
            Cow::Borrowed(borrowed) => Cow::Borrowed(borrowed),
            Cow::Owned(owned) => Cow::Owned(Box::new(owned) as Box<dyn ErasedDataStruct>),
        })
    }
}

/// Concrete struct backing ErasedDataReceiver. Implemented for:
///
/// - Sized `T` implementing Serde. All methods supported.
/// - `dyn ErasedDataStruct`. Deserialization not supported.
///
/// # Example
///
/// ```
/// use icu_provider::prelude::*;
/// use icu_provider::erased::DataReceiver;
///
/// let mut string_receiver = DataReceiver::<String>::new();
/// // Now pass string_receiver as an argument to ErasedDataProvider
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

/// Implementation of DataReceiver for sized types.
impl<'d, 'de, T> ErasedDataReceiver<'d, 'de> for DataReceiver<'d, T>
where
    T: serde::Deserialize<'de> + serde::Serialize + Clone + Debug + Any + Default,
{
    fn receive_deserializer(
        &mut self,
        deserializer: &mut dyn erased_serde::Deserializer<'de>,
    ) -> Result<(), Error> {
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

    fn take_erased(&mut self) -> Result<Cow<'d, dyn ErasedDataStruct>, Error> {
        match self.payload.take() {
            Some(cow) => match cow {
                Cow::Borrowed(borrowed) => Ok(Cow::Borrowed(borrowed)),
                Cow::Owned(owned) => {
                    let boxed: Box<dyn ErasedDataStruct> = Box::new(owned);
                    Ok(Cow::Owned(boxed))
                }
            },
            None => Err(Error::MissingPayload),
        }
    }

    fn has_payload(&self) -> bool {
        self.payload.is_some()
    }

    fn reset(&mut self) {
        self.payload.take();
    }
}

/// Implementation of DataReceiver for ErasedDataStruct trait object.
impl<'d, 'de> ErasedDataReceiver<'d, 'de> for DataReceiver<'d, dyn ErasedDataStruct> {
    fn receive_deserializer(
        &mut self,
        _deserializer: &mut dyn erased_serde::Deserializer<'de>,
    ) -> Result<(), Error> {
        Err(Error::NeedsTypeInfo)
    }

    fn receive_default(&mut self) -> Result<(), Error> {
        Err(Error::NeedsTypeInfo)
    }

    fn receive_erased(&mut self, cow: Cow<'d, dyn ErasedDataStruct>) -> Result<(), Error> {
        self.payload = Some(cow);
        Ok(())
    }

    fn take_erased(&mut self) -> Result<Cow<'d, dyn ErasedDataStruct>, Error> {
        self.payload.take().ok_or(Error::MissingPayload)
    }

    fn has_payload(&self) -> bool {
        self.payload.is_some()
    }

    fn reset(&mut self) {
        self.payload.take();
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
        receiver: &mut dyn ErasedDataReceiver<'d, '_>,
    ) -> Result<DataResponseMetadata, Error>;
}

/// Helper macro to implement ErasedDataProvider on an object implementing DataProvider for a
/// single type. Calls to `self.load_to_receiver` delegate to `self.load_payload`.
#[macro_export]
macro_rules! impl_erased {
    ($type:ty, $lifetime:tt) => {
        impl<$lifetime> $crate::erased::ErasedDataProvider<$lifetime> for $type {
            fn load_to_receiver(
                &self,
                req: &$crate::prelude::DataRequest,
                receiver: &mut dyn $crate::erased::ErasedDataReceiver<$lifetime, '_>,
            ) -> Result<$crate::prelude::DataResponseMetadata, $crate::prelude::DataError> {
                let mut result = self.load_payload(req)?;
                receiver.receive_payload(result.take_payload()?)?;
                Ok(result.metadata)
            }
        }
    };
}

/// Convenience implementation of DataProvider<T> given an ErasedDataProvider trait object.
impl<'a, 'd, 'de, T> DataProvider<'d, T> for dyn ErasedDataProvider<'d> + 'a
where
    T: serde::Deserialize<'de> + serde::Serialize + Clone + Debug + Any + Default,
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
