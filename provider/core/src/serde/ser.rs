// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::prelude::*;
use crate::yoke::*;
use alloc::boxed::Box;
use core::ops::Deref;
use super::BufferFormat;
use super::Error;
use serde::ser::Serialize;
use alloc::rc::Rc;

/// Auto-implemented trait for all data structs that support [`serde::Serialize`]. This trait is
/// usually used as a trait object in [`DataProvider`]`<dyn `[`SerdeSeDataStruct`]`>`.
pub trait SerdeSeDataStruct: 'static {
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

impl<T> SerdeSeDataStruct for T
where
    T: 'static + serde::Serialize,
{
    fn as_serialize(&self) -> &dyn erased_serde::Serialize {
        self
    }
}

fn serialize_impl<M>(payload: DataPayload<M>, buffer_format: BufferFormat) -> Result<DataPayload<BufferMarker>, Error>
where
    M: DataMarker,
    // Actual bound:
    //     <M::Yokeable as Yokeable<'de>>::Output: Serialize,
    // Necessary workaround bound (see `yoke::trait_hack` docs):
    for<'de> &'de <M::Yokeable as Yokeable<'de>>::Output: Serialize,
{
    let mut serializer = postcard::Serializer {
        output: postcard::flavors::AllocVec(Vec::new()),
    };
    payload.get().serialize(&mut serializer)?;
    Ok(DataPayload::from_rc_buffer(serializer.output.0.into()))
}

pub struct SerializingDataProvider<'a, M: DataMarker> {
    pub provider: &'a DataProvider<M>,
    pub buffer_format: BufferFormat,
}

pub trait AsSerializingDataProvider<M: DataMarker> {
    fn as_serializing(&self) -> SerializingDataProvider<M>;
}

impl<M> DataResponse<M>
where
    M: DataMarker,
    // Actual bound:
    //     for<'de> <M::Yokeable as Yokeable<'de>>::Output: Serialize,
    // Necessary workaround bound (see `yoke::trait_hack` docs):
    for<'de> &'de <M::Yokeable as Yokeable<'de>>::Output: Serialize,
{
    pub fn into_serialized(self, buffer_format: BufferFormat) -> Result<DataResponse<BufferMarker>, DataError> {
        if let Some(old_payload) = self.payload {
            let new_payload = serialize_impl(old_payload, buffer_format)?;
            let mut new_metadata = self.metadata;
            new_metadata.buffer_format = Some(buffer_format);
            Ok(DataResponse {
                metadata: new_metadata,
                payload: Some(new_payload),
            })
        } else {
            Ok(DataResponse {
                metadata: self.metadata,
                payload: None,
            })
        }
    }
}

impl<M> BufferProvider for SerializingDataProvider<'_, M>
where
    M: DataMarker,
    // Actual bound:
    //     for<'de> <M::Yokeable as Yokeable<'de>>::Output: Serialize,
    // Necessary workaround bound (see `yoke::trait_hack` docs):
    for<'de> &'de <M::Yokeable as Yokeable<'de>>::Output: Serialize,
{
    /// Converts a data struct to a buffer by serializing the to a supported buffer format.
    fn load_buffer(&self, req: DataRequest) -> Result<DataResponse<BufferMarker>, DataError> {
        self.provider.load_payload(&req)?.into_serialized(self.buffer_format)
    }
}


/// A wrapper around `Box<`[`SerdeSeDataStruct`]`>` for integration with DataProvider.
#[derive(yoke::Yokeable)]
pub struct SerdeSeDataStructBox(Box<dyn SerdeSeDataStruct>);

impl Deref for SerdeSeDataStructBox {
    type Target = dyn SerdeSeDataStruct;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<M> crate::dynutil::UpcastDataPayload<M> for SerdeSeDataStructMarker
where
    M: DataMarker,
    for<'a> &'a <M::Yokeable as Yokeable<'a>>::Output: serde::Serialize,
{
    fn upcast(other: DataPayload<M>) -> DataPayload<SerdeSeDataStructMarker> {
        let owned: Box<dyn SerdeSeDataStruct> = Box::new(other.yoke);
        DataPayload::from_owned(SerdeSeDataStructBox(owned))
    }
}

/// Marker type for [`SerdeSeDataStruct`].
pub struct SerdeSeDataStructMarker {}

impl DataMarker for SerdeSeDataStructMarker {
    type Yokeable = SerdeSeDataStructBox;
}
