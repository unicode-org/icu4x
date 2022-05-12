// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;
use crate::prelude::*;
use alloc::borrow::Cow;

/// Marker type for [`Cow`]`<str>` where the backing cart is `str`.
#[allow(clippy::exhaustive_structs)] // marker type
pub struct CowStrMarker;

impl DataMarker for CowStrMarker {
    type Yokeable = Cow<'static, str>;
}

impl DataPayload<CowStrMarker> {
    /// Make a [`DataPayload`]`<`[`CowStrMarker`]`>` from a static string slice.
    pub fn from_static_str(s: &'static str) -> DataPayload<CowStrMarker> {
        DataPayload::from_owned(Cow::Borrowed(s))
    }
}
