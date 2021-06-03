// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;
use std::borrow::Cow;

/// Marker type for [`Cow`]`<str>`.
#[allow(non_camel_case_types)]
pub struct CowStr_M;

impl<'s> DataMarker<'s> for CowStr_M {
    type Yokeable = Cow<'static, str>;
    type Cart = str;
}
