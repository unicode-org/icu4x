// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

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
    /// use icu_provider::hello_world::HelloWorldV1;
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
    /// use icu_provider::hello_world::HelloWorldV1;
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
    /// use icu_provider::hello_world::HelloWorldV1;
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
    /// use icu_provider::hello_world::HelloWorldV1;
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
    /// use icu_provider::hello_world::HelloWorldV1;
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

impl<'d, T> DataResponse<'d, T>
where
    T: erased_serde::Serialize + Clone + Debug + Any,
{
    /// Convert this DataResponse of a Sized type into a DataResponse of an ErasedDataStruct.
    ///
    /// Can be used to implement ErasedDataProvider on types implementing DataProvider.
    pub fn into_erased(self) -> DataResponse<'d, dyn ErasedDataStruct> {
        DataResponse {
            metadata: self.metadata,
            payload: self.payload.map(|p| match p {
                Cow::Borrowed(v) => Cow::Borrowed(v as &dyn ErasedDataStruct),
                Cow::Owned(v) => {
                    let boxed: Box<dyn ErasedDataStruct> = Box::new(v);
                    Cow::Owned(boxed)
                }
            }),
        }
    }
}

impl<'d> DataResponse<'d, dyn ErasedDataStruct> {
    /// Convert this DataResponse of an ErasedDataStruct into a DataResponse of a Sized type.
    ///
    /// Can be used to implement DataProvider on types implementing ErasedDataProvider.
    pub fn downcast<T>(self) -> Result<DataResponse<'d, T>, Error>
    where
        T: erased_serde::Serialize + Clone + Debug + Any,
    {
        let metadata = self.metadata;
        let cow = match self.payload {
            Some(cow) => cow,
            None => {
                return Ok(DataResponse {
                    metadata,
                    payload: None,
                })
            }
        };
        let payload = match cow {
            Cow::Borrowed(erased) => {
                let borrowed: &'d T =
                    erased
                        .as_any()
                        .downcast_ref()
                        .ok_or_else(|| Error::MismatchedType {
                            actual: Some(erased.type_id()),
                            generic: Some(TypeId::of::<T>()),
                        })?;
                Some(Cow::Borrowed(borrowed))
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
                Some(Cow::Owned(*boxed))
            }
        };
        Ok(DataResponse { metadata, payload })
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
    /// use icu_provider::erased::DataReceiver;
    /// use icu_provider::erased::SerdeDataReceiver;
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

/// A type-erased data provider that loads paylads from a Serde Deserializer.
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

// Note: Once trait aliases land, we could enable the following alias.
// https://github.com/rust-lang/rust/issues/41517
// pub trait ErasedDataProvider<'d> = DataProvider<'d, dyn ErasedDataStruct>;

/// A type-erased data provider that loads a payload of types implementing Any.
pub trait ErasedDataProvider<'d> {
    /// Query the provider for data, returning the result as an ErasedDataStruct trait object.
    ///
    /// Returns Ok if the request successfully loaded data. If data failed to load, returns an
    /// Error with more information.
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<'d, dyn ErasedDataStruct>, Error>;
}

/// Helper macro to implement ErasedDataProvider on an object implementing DataProvider for a
/// single type. Calls to `self.load_to_receiver` delegate to `self.load_payload`.
#[macro_export]
macro_rules! impl_erased {
    ($provider:ty, $struct:ty, $lifetime:tt) => {
        impl<$lifetime> $crate::erased::ErasedDataProvider<$lifetime> for $provider {
            fn load_payload(
                &self,
                req: &$crate::prelude::DataRequest,
            ) -> Result<
                $crate::prelude::DataResponse<'d, dyn $crate::erased::ErasedDataStruct>,
                $crate::prelude::DataError,
            > {
                let result: $crate::prelude::DataResponse<$struct> =
                    $crate::prelude::DataProvider::load_payload(self, req)?;
                Ok(result.into_erased())
            }
        }
    };
}

/// Convenience implementation of DataProvider<T> given an ErasedDataProvider trait object.
impl<'a, 'd, 'de, T> DataProvider<'d, T> for dyn ErasedDataProvider<'d> + 'a
where
    T: serde::Serialize + Clone + Debug + Any + Default,
{
    fn load_payload(&self, req: &DataRequest) -> Result<DataResponse<'d, T>, Error> {
        let result = ErasedDataProvider::load_payload(self, req)?;
        result.downcast()
    }
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
