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
use std::ops::Deref;
use std::rc::Rc;
use yoke::trait_hack::YokeTraitHack;
use yoke::*;

/// An object that receives data from a Serde Deserializer.
///
/// Implemented by `Option<`[`DataPayload`]`>`.
pub trait SerdeDeDataReceiver {
    /// Receives a reference-counted byte buffer.
    ///
    /// Upon calling this function, the receiver sends byte buffer back to the caller as the first
    /// argument of `f1`. The caller should then map the byte buffer to an
    /// [`erased_serde::Deserializer`] and pass it back to the receiver via `f2`.
    ///
    /// # Examples
    ///
    /// Deserialize from a reference-counted buffer:
    ///
    /// ```
    /// use icu_provider::prelude::*;
    /// use icu_provider::hello_world::*;
    /// use icu_provider::serde::SerdeDeDataReceiver;
    /// use std::rc::Rc;
    ///
    /// let json_text = "{\"message\":\"Hello World\"}";
    /// let rc_buffer: Rc<[u8]> = json_text.as_bytes().into();
    /// let mut receiver: Option<DataPayload<HelloWorldV1Marker>> = None;
    /// receiver
    ///     .receive_rc_buffer(rc_buffer, |bytes, f2| {
    ///         let mut d = serde_json::Deserializer::from_slice(bytes);
    ///         f2(&mut erased_serde::Deserializer::erase(&mut d))
    ///     })
    ///     .expect("Well-formed data");
    /// let payload = receiver.expect("Data is present");
    ///
    /// assert_eq!(payload.get().message, "Hello World");
    /// ```
    fn receive_rc_buffer(
        &mut self,
        rc_buffer: Rc<[u8]>,
        f1: for<'de> fn(
            bytes: &'de [u8],
            f2: &mut dyn FnMut(&mut dyn erased_serde::Deserializer<'de>),
        ),
    ) -> Result<(), Error>;

    /// Receives a `&'static` byte buffer via an [`erased_serde::Deserializer`].
    ///
    /// Note: Since the purpose of this function is to handle zero-copy deserialization of static
    /// byte buffers, we want `Deserializer<'static>` as opposed to `DeserializeOwned`.
    ///
    /// # Examples
    ///
    /// Deserialize from a string to create static references:
    ///
    /// ```
    /// use icu_provider::prelude::*;
    /// use icu_provider::hello_world::*;
    /// use icu_provider::serde::SerdeDeDataReceiver;
    ///
    /// let json_text = "{\"message\":\"Hello World\"}";
    /// let deserializer = &mut serde_json::Deserializer::from_str(json_text);
    /// let mut receiver: Option<DataPayload<HelloWorldV1Marker>> = None;
    /// receiver
    ///     .receive_static(&mut erased_serde::Deserializer::erase(deserializer))
    ///     .expect("Well-formed data");
    /// let payload = receiver.expect("Data is present");
    ///
    /// assert_eq!(payload.get().message, "Hello World");
    /// ```
    fn receive_static(
        &mut self,
        deserializer: &mut dyn erased_serde::Deserializer<'static>,
    ) -> Result<(), Error>;
}

impl<'d, 's, M> SerdeDeDataReceiver for Option<DataPayload<'d, 's, M>>
where
    M: DataMarker<'s>,
    M::Yokeable: serde::de::Deserialize<'static>,
    // Actual bound:
    //     for<'de> <M::Yokeable as Yokeable<'de>>::Output: serde::de::Deserialize<'de>,
    // Necessary workaround bound (see `yoke::trait_hack` docs):
    for<'de> YokeTraitHack<<M::Yokeable as Yokeable<'de>>::Output>: serde::de::Deserialize<'de>,
{
    fn receive_rc_buffer(
        &mut self,
        rc_buffer: Rc<[u8]>,
        f1: for<'de> fn(
            bytes: &'de [u8],
            f2: &mut dyn FnMut(&mut dyn erased_serde::Deserializer<'de>),
        ),
    ) -> Result<(), Error> {
        self.replace(DataPayload::try_from_rc_buffer(rc_buffer, move |bytes| {
            let mut holder = None;
            f1(bytes, &mut |deserializer| {
                holder.replace(
                    erased_serde::deserialize::<YokeTraitHack<<M::Yokeable as Yokeable>::Output>>(
                        deserializer,
                    )
                    .map(|w| w.0),
                );
            });
            // The holder is guaranteed to be populated so long as the lambda function was invoked,
            // which is in the contract of `receive_rc_buffer`.
            holder.unwrap()
        })?);
        Ok(())
    }

    fn receive_static(
        &mut self,
        deserializer: &mut dyn erased_serde::Deserializer<'static>,
    ) -> Result<(), Error> {
        let obj: M::Yokeable = erased_serde::deserialize(deserializer)?;
        self.replace(DataPayload::from_owned(obj));
        Ok(())
    }
}

/// A type-erased data provider that loads payloads from a Serde Deserializer.
///
/// Uses [`erased_serde`] to allow the trait to be object-safe.
pub trait SerdeDeDataProvider {
    /// Query the provider for data, loading it into a [`SerdeDeDataReceiver`].
    ///
    /// Returns Ok if the request successfully loaded data. If data failed to load, returns an
    /// Error with more information.
    fn load_to_receiver(
        &self,
        req: &DataRequest,
        receiver: &mut dyn SerdeDeDataReceiver,
    ) -> Result<DataResponseMetadata, Error>;
}

impl<'d, 's, M> DataProvider<'d, 's, M> for dyn SerdeDeDataProvider + 'd
where
    M: DataMarker<'s>,
    M::Yokeable: serde::de::Deserialize<'static>,
    // Actual bound:
    //     for<'de> <M::Yokeable as Yokeable<'de>>::Output: serde::de::Deserialize<'de>,
    // Necessary workaround bound (see `yoke::trait_hack` docs):
    for<'de> YokeTraitHack<<M::Yokeable as Yokeable<'de>>::Output>: serde::de::Deserialize<'de>,
{
    /// Serve objects implementing [`serde::Deserialize<'s>`] from a [`SerdeDeDataProvider`].
    fn load_payload(&self, req: &DataRequest) -> Result<DataResponse<'d, 's, M>, Error> {
        let mut payload = None;
        let metadata = self.load_to_receiver(req, &mut payload)?;
        Ok(DataResponse { metadata, payload })
    }
}

/// Auto-implemented trait for all data structs that support [`serde::Serialize`]. This trait is
/// usually used as a trait object in [`DataProvider`]`<dyn `[`SerdeSeDataStruct`]`>`.
pub trait SerdeSeDataStruct<'s>: 's {
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

impl<'s, T> SerdeSeDataStruct<'s> for T
where
    T: 's + serde::Serialize,
    for<'a> &'a T: Clone,
{
    fn clone_into_box(&self) -> Box<dyn SerdeSeDataStruct<'s> + 's> {
        todo!("#753")
        // Box::new(self.clone())
    }
    fn as_serialize(&self) -> &dyn erased_serde::Serialize {
        self
    }
}

/// A wrapper around `&dyn `[`SerdeSeDataStruct`]`<'s>` for integration with DataProvider.
pub struct SerdeSeDataStructWrap<'d, 's> {
    inner: &'d (dyn SerdeSeDataStruct<'s> + 's),
}

impl<'d, 's> Deref for SerdeSeDataStructWrap<'d, 's> {
    type Target = dyn SerdeSeDataStruct<'s> + 's;
    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl<'d, 's: 'd> SerdeSeDataStructWrap<'d, 's> {
    fn shorten(self) -> SerdeSeDataStructWrap<'d, 'd> {
        // This is safe because 's exceeds 'd
        // TODO(#760): The types must be covariant for this to actually be safe.
        unsafe { std::mem::transmute(self) }
    }
}

impl<'s> ZeroCopyFrom<dyn SerdeSeDataStruct<'s> + 's> for SerdeSeDataStructWrap<'static, 'static> {
    fn zero_copy_from<'b>(
        this: &'b (dyn SerdeSeDataStruct<'s> + 's),
    ) -> SerdeSeDataStructWrap<'b, 'b> {
        SerdeSeDataStructWrap { inner: this }.shorten()
    }
}

impl<'d, 's, M> crate::dynutil::UpcastDataPayload<'d, 's, M> for SerdeSeDataStructMarker
where
    M: DataMarker<'s>,
    for<'a> &'a <M::Yokeable as Yokeable<'a>>::Output: serde::Serialize,
    's: 'd,
{
    fn upcast(other: DataPayload<'d, 's, M>) -> DataPayload<'d, 's, SerdeSeDataStructMarker> {
        use crate::data_provider::DataPayloadInner::*;
        let cart: Rc<dyn SerdeSeDataStruct<'s> + 's> = match other.inner {
            Borrowed(_) => todo!("#752"),
            RcStruct(yoke) => Rc::from(yoke),
            Owned(yoke) => Rc::from(yoke),
            RcBuf(yoke) => Rc::from(yoke),
        };
        DataPayload::from_partial_owned(cart)
    }
}

unsafe impl<'a> Yokeable<'a> for SerdeSeDataStructWrap<'static, 'static> {
    type Output = SerdeSeDataStructWrap<'a, 'a>;
    fn transform(&'a self) -> &'a Self::Output {
        // The compiler isn't able to guess the variance of the trait object,
        // so we must transmute
        // Note (Manishearth): this is technically unsound since SerdeDeDataStruct
        // has no variance requirements. This will become a non-issue
        // once Borrowed is removed (https://github.com/unicode-org/icu4x/issues/752)
        unsafe { std::mem::transmute(self) }
    }
    fn transform_owned(self) -> Self::Output {
        // (needs a transmute for the same reason as above)
        unsafe { std::mem::transmute(self) }
    }
    unsafe fn make(from: Self::Output) -> Self {
        std::mem::transmute(from)
    }
    fn transform_mut<F>(&'a mut self, f: F)
    where
        F: 'static + for<'b> FnOnce(&'b mut Self::Output),
    {
        unsafe {
            f(std::mem::transmute::<&'a mut Self, &'a mut Self::Output>(
                self,
            ))
        }
    }
}

/// Marker type for [`SerdeSeDataStruct`].
pub struct SerdeSeDataStructMarker {}

impl<'s> DataMarker<'s> for SerdeSeDataStructMarker {
    type Yokeable = SerdeSeDataStructWrap<'static, 'static>;
    type Cart = dyn SerdeSeDataStruct<'s>;
}
