// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;
use crate::prelude::*;
use alloc::borrow::Cow;
use alloc::string::String;

/// Marker type for [`Cow`]`<str>` where the backing cart is `str`.
pub struct CowStrMarker;

impl<'data> DataMarker<'data> for CowStrMarker {
    type Yokeable = Cow<'static, str>;
    type Cart = str;
}

impl DataPayload<'static, CowStrMarker> {
    /// Make a [`DataPayload`]`<`[`CowStrMarker`]`>` from a static string slice.
    pub fn from_static_str(s: &'static str) -> DataPayload<'static, CowStrMarker> {
        DataPayload::from_owned(Cow::Borrowed(s))
    }
}

/// Marker type for [`Cow`]`<str>` where the backing cart is `String`. This is required if
/// `ErasedDataStruct` is to be used.
pub struct CowStringMarker;

impl<'data> DataMarker<'data> for CowStringMarker {
    type Yokeable = Cow<'static, str>;
    type Cart = String;
}
