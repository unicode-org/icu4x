// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::dynutil::UpcastDataPayload;
use crate::prelude::*;
use crate::yoke::*;
use alloc::boxed::Box;
use core::ops::Deref;
use crabbake::{Bakeable, TokenStream};

#[derive(yoke::Yokeable)]
pub struct CrabbakeBox(Box<dyn Bakeable>);

impl Deref for CrabbakeBox {
    type Target = dyn Bakeable;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<M> UpcastDataPayload<M> for CrabbakeMarker
where
    M: DataMarker,
    for<'a> <M::Yokeable as Yokeable<'a>>::Output: Bakeable,
{
    fn upcast(other: DataPayload<M>) -> DataPayload<CrabbakeMarker> {
        let owned: Box<dyn Bakeable> = Box::new(other.yoke);
        DataPayload::from_owned(CrabbakeBox(owned))
    }
}

impl<M> DataPayload<M>
where
    M: DataMarker,
    for<'a> <M::Yokeable as Yokeable<'a>>::Output: Bakeable,
{
    /// Converts a [`DataPayload`] into something that can be baked.
    ///
    /// See [`DataPayload::tokenize()`] for an example.
    pub fn into_bakeable(self) -> DataPayload<CrabbakeMarker> {
        CrabbakeMarker::upcast(self)
    }
}

impl DataPayload<CrabbakeMarker> {
    /// Serializes this [`DataPayload`] into a [`TokenStream`] using its
    /// [`Bakeable`] implementation.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_provider::prelude::*;
    /// use icu_provider::hello_world::HelloWorldV1Marker;
    ///
    /// // Create an example DataPayload
    /// let payload: DataPayload<HelloWorldV1Marker> = Default::default();
    ///
    /// // Serialize the payload to a JSON string
    /// let tokens = payload.into_bakeable().tokenize().expect("Tokenization should succeed");
    /// assert_eq!("HelloWorldV1 { message: \"(und) Hello World\" }", tokens.to_string());
    /// ```
    pub fn tokenize(&self) -> TokenStream {
        self.get().bake()
    }
}

/// Marker type for [`CrabbakeBox`].
pub struct CrabbakeMarker {}

impl DataMarker for CrabbakeMarker {
    type Yokeable = CrabbakeBox;
}
