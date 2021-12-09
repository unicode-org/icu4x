// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::prelude::*;
use crate::yoke::*;
use alloc::boxed::Box;
use core::ops::Deref;

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
