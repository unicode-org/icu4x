// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;
use std::borrow::Cow;

/// Marker type for [`Cow`]`<str>` where the backing cart is `str`.
#[allow(non_camel_case_types)]
pub struct CowStr_M;

impl<'s> DataMarker<'s> for CowStr_M {
    type Yokeable = Cow<'static, str>;
    type Cart = str;
}

/// Marker type for [`Cow`]`<str>` where the backing cart is `String`. This is required if
/// `ErasedDataStruct` is to be used.
#[allow(non_camel_case_types)]
pub struct CowString_M;

impl<'s> DataMarker<'s> for CowString_M {
    type Yokeable = Cow<'static, str>;
    type Cart = String;
}
