// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::Error;
use crate::dynutil::UpcastDataPayload;
use crate::prelude::*;
use crate::yoke::*;
use alloc::boxed::Box;
use core::ops::Deref;

/// A wrapper around `Box<erased_serde::Serialize>` for integration with DataProvider.
#[derive(yoke::Yokeable)]
pub struct SerializeBox(Box<dyn erased_serde::Serialize>);

impl Deref for SerializeBox {
    type Target = dyn erased_serde::Serialize;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<M> UpcastDataPayload<M> for SerializeMarker
where
    M: DataMarker,
    for<'a> &'a <M::Yokeable as Yokeable<'a>>::Output: serde::Serialize,
{
    fn upcast(other: DataPayload<M>) -> DataPayload<SerializeMarker> {
        let owned: Box<dyn erased_serde::Serialize> = Box::new(other.yoke);
        DataPayload::from_owned(SerializeBox(owned))
    }
}

impl<M> DataPayload<M>
where
    M: DataMarker,
    for<'a> &'a <M::Yokeable as Yokeable<'a>>::Output: serde::Serialize,
{
    /// Converts a [`DataPayload`] into something that can be serialized.
    ///
    /// See [`DataPayload::serialize()`] for an example.
    pub fn into_serializable(self) -> DataPayload<SerializeMarker> {
        SerializeMarker::upcast(self)
    }
}

impl DataPayload<SerializeMarker> {
    /// Serializes this [`DataPayload`] into a serializer using Serde.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_provider::hello_world::HelloWorldV1Marker;
    /// use icu_provider::prelude::*;
    ///
    /// // Create an example DataPayload
    /// let payload: DataPayload<HelloWorldV1Marker> = Default::default();
    ///
    /// // Serialize the payload to a JSON string
    /// let mut buffer: Vec<u8> = vec![];
    /// payload
    ///     .into_serializable()
    ///     .serialize(&mut <dyn erased_serde::Serializer>::erase(
    ///         &mut serde_json::Serializer::new(&mut buffer),
    ///     ))
    ///     .expect("Serialization should succeed");
    /// assert_eq!("{\"message\":\"(und) Hello World\"}".as_bytes(), buffer);
    /// ```
    pub fn serialize(
        &self,
        mut serializer: &mut dyn erased_serde::Serializer,
    ) -> Result<(), Error> {
        self.get().erased_serialize(&mut serializer)?;
        Ok(())
    }
}

/// Marker type for [`SerializeBox`].
#[allow(clippy::exhaustive_structs)] // marker type
pub struct SerializeMarker {}

impl DataMarker for SerializeMarker {
    type Yokeable = SerializeBox;
}
