// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use crate::error::Error;
use crate::prelude::*;
use std::any::Any;
use std::any::TypeId;
use std::borrow::Borrow;
use std::borrow::Cow;
use std::fmt::Debug;

#[cfg(feature = "invariant")]
use std::default::Default;

/// Re-export erased_serde for the impl_erased! macro
pub use erased_serde;

/// An object capable of accepting data from a variety of forms.
/// Lifetimes:
/// - 'd = lifetime of borrowed data (Cow::Borrowed)
/// - 'de = lifetime parameter of owned data (Cow::Owned)
pub trait DataReceiver<'d> {
    /// Consumes a Serde Deserializer into this DataReceiver as owned data.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_provider::prelude::*;
    /// use std::borrow::Cow;
    ///
    /// const JSON: &'static str = "\"hello world\"";
    ///
    /// let mut receiver = DataReceiverForType::<String>::new();
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

    /// Sets this DataReceiver to a borrowed value.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_provider::prelude::*;
    /// use std::borrow::Cow;
    ///
    /// let borrowed_data: String = "hello world".to_string();
    ///
    /// let mut receiver = DataReceiverForType::<String>::new();
    /// receiver.receive_borrow(&borrowed_data).expect("Types should match");
    ///
    /// assert_eq!(receiver.payload, Some(Cow::Borrowed(&borrowed_data)));
    /// ```
    fn receive_borrow(&mut self, borrowed_any: &'d dyn Any) -> Result<(), Error>;

    /// Takes a Box into this DataReceiver as owned data.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_provider::prelude::*;
    /// use std::borrow::Cow;
    ///
    /// let mut receiver = DataReceiverForType::<String>::new();
    /// let boxed = Box::new("hello world".to_string());
    /// receiver.receive_box(boxed).expect("Types should match");
    ///
    /// assert_eq!(receiver.payload, Some(Cow::Owned("hello world".to_string())));
    /// ```
    fn receive_box(&mut self, boxed_any: Box<dyn Any>) -> Result<(), Error>;

    /// Takes the value out of an Option into this DataReceiver as owned data.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_provider::prelude::*;
    /// use std::borrow::Cow;
    ///
    /// let mut receiver = DataReceiverForType::<String>::new();
    /// let mut option = Some("hello world".to_string());
    /// receiver.receive_option(&mut option).expect("Types should match");
    ///
    /// assert!(option.is_none());
    /// assert_eq!(receiver.payload, Some(Cow::Owned("hello world".to_string())));
    /// ```
    fn receive_option(&mut self, option_any: &mut dyn Any) -> Result<(), Error>;

    /// Sets the payload to the default value. Requires Default to be implemented for the type.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_provider::prelude::*;
    /// use std::borrow::Cow;
    ///
    /// let mut receiver = DataReceiverForType::<String>::new();
    /// receiver.receive_default().expect("Default is implemented for String");
    ///
    /// assert_eq!(receiver.payload, Some(Cow::Owned("".to_string())));
    /// ```
    fn receive_default(&mut self) -> Result<(), Error>;

    /// Borrows the value in this DataReceiver as a Serialize trait object.
    fn as_serialize(&self) -> Option<&dyn erased_serde::Serialize>;
}

impl<'a, 'd> dyn DataReceiver<'d> + 'a {
    /// Sets this DataReceiver to the default value of the given type.
    #[cfg(feature = "invariant")]
    pub fn receive_invariant<T: Default + Any>(&mut self) -> Result<(), Error> {
        let mut option = Some(T::default());
        self.receive_option(&mut option)
    }

    /// Sets this DataReceiver to a Cow. Delegates to the main trait methods.
    pub fn receive_optional_cow<T: Clone + Any + Debug>(
        &mut self,
        optional_cow: Option<Cow<'d, T>>,
    ) -> Result<(), Error> {
        match optional_cow {
            Some(cow) => match cow {
                Cow::Borrowed(v) => self.receive_borrow(v),
                Cow::Owned(v) => {
                    let mut option = Some(v);
                    self.receive_option(&mut option)
                }
            },
            None => {
                let mut option: Option<T> = None;
                self.receive_option(&mut option)
            }
        }
    }
}

/// Concrete implementation of DataReceiver parameterized for a certain type.
///
/// # Example
///
/// ```
/// use icu_provider::prelude::*;
///
/// let mut string_receiver = DataReceiverForType::<String>::new();
/// ```
#[derive(Debug)]
pub struct DataReceiverForType<'d, T>
where
    T: Clone + Debug,
{
    pub payload: Option<Cow<'d, T>>,
}

#[cfg(feature = "invariant")]
impl<'d, T> Default for DataReceiverForType<'d, T>
where
    T: Clone + Debug + Default,
{
    /// Creates a new DataReceiver with the Default data pre-loaded.
    fn default() -> Self {
        Self {
            payload: Default::default(),
        }
    }
}

impl<'d, T> DataReceiverForType<'d, T>
where
    T: Clone + Debug,
{
    /// Creates a new, empty DataReceiver.
    pub fn new() -> Self {
        Self { payload: None }
    }

    /// Borrows the payload from the underlying Cow.
    pub fn borrow_payload(&self) -> Option<&T> {
        self.payload.as_ref().map(|cow| cow.borrow())
    }
}

impl<'d, T> DataReceiverForType<'d, T>
where
    T: serde::de::DeserializeOwned + serde::Serialize + Clone + Debug + Any + Default,
{
    /// Creates a new, empty DataReceiver, returning it as a boxed trait object.
    pub fn new_boxed() -> Box<dyn DataReceiver<'d> + 'd> {
        let receiver: DataReceiverForType<'d, T> = Self::new();
        Box::new(receiver)
    }
}

impl<'d, T> DataReceiver<'d> for DataReceiverForType<'d, T>
where
    T: serde::de::DeserializeOwned + serde::Serialize + Clone + Debug + Any + Default,
{
    fn receive_deserializer(
        &mut self,
        deserializer: &mut dyn erased_serde::Deserializer<'d>,
    ) -> Result<(), erased_serde::Error> {
        let obj: T = erased_serde::deserialize(deserializer)?;
        self.payload = Some(Cow::Owned(obj));
        Ok(())
    }

    fn receive_borrow(&mut self, borrowed_any: &'d dyn Any) -> Result<(), Error> {
        let borrowed: &T = borrowed_any
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
        self.payload = option.take().map(Cow::Owned);
        Ok(())
    }

    fn receive_default(&mut self) -> Result<(), Error> {
        self.payload = Some(Cow::Owned(T::default()));
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

/// Concrete implementation of DataReceiver that records whether or not it received data, but
/// throws away any data it receives.
///
/// Can be used for checking whether a DataProvider supports a particular resource key.
pub struct DataReceiverThrowAway {
    flag: bool,
}

impl Default for DataReceiverThrowAway {
    fn default() -> Self {
        Self { flag: false }
    }
}

impl DataReceiverThrowAway {
    /// Whether any receive function has been called on this DataReceiver.
    pub fn has_received_data(&self) -> bool {
        self.flag
    }

    /// Resets this DataReceiver to its initial state.
    pub fn reset(&mut self) {
        self.flag = false;
    }
}

impl<'d> DataReceiver<'d> for DataReceiverThrowAway {
    fn receive_deserializer(
        &mut self,
        _: &mut dyn erased_serde::Deserializer<'d>,
    ) -> Result<(), erased_serde::Error> {
        self.flag = true;
        Ok(())
    }

    fn receive_borrow(&mut self, _: &'d dyn Any) -> Result<(), Error> {
        self.flag = true;
        Ok(())
    }

    fn receive_box(&mut self, _: Box<dyn Any>) -> Result<(), Error> {
        self.flag = true;
        Ok(())
    }

    fn receive_option(&mut self, _: &mut dyn Any) -> Result<(), Error> {
        self.flag = true;
        Ok(())
    }

    fn receive_default(&mut self) -> Result<(), Error> {
        self.flag = true;
        Ok(())
    }

    fn as_serialize(&self) -> Option<&dyn erased_serde::Serialize> {
        None
    }
}

/// A type-erased data provider that loads a payload of any type.
pub trait ErasedDataProvider<'d> {
    /// Query the provider for data, loading it into a DataReceiver.
    ///
    /// Returns Ok if the request successfully loaded data. If data failed to load, returns an
    /// Error with more information.
    fn load_to_receiver(
        &self,
        req: &DataRequest,
        receiver: &mut dyn DataReceiver<'d>,
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
                receiver: &mut dyn $crate::prelude::DataReceiver<$lifetime>,
            ) -> Result<$crate::prelude::DataResponseMetadata, $crate::prelude::DataError> {
                let result = self.load_payload(req)?;
                receiver.receive_optional_cow(result.payload)?;
                Ok(result.metadata)
            }

            fn load_as_serialize(
                &self,
                req: &$crate::prelude::DataRequest,
            ) -> Result<Box<dyn $crate::erased::erased_serde::Serialize>, $crate::prelude::DataError> {
                let result = self.load_payload(req)?;
                Ok(Box::new(result.payload.expect("Load was successful").into_owned()))
            }
        }
    };
}

/// Convenience implementation of DataProvider<T> given an ErasedDataProvider trait object.
impl<'a, 'd, T> DataProvider<'d, T> for dyn ErasedDataProvider<'d> + 'a
where
    T: serde::de::DeserializeOwned + serde::Serialize + Clone + Debug + Any + Default,
{
    fn load_payload(&self, req: &DataRequest) -> Result<DataResponse<'d, T>, Error> {
        let mut receiver = DataReceiverForType::<T>::new();
        let metadata = self.load_to_receiver(req, &mut receiver)?;
        Ok(DataResponse {
            metadata,
            payload: receiver.payload,
        })
    }
}
