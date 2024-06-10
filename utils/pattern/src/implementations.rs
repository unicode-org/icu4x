// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::borrow::Cow;
use zerovec::{maps::ZeroMapKV, ZeroVec};

use crate::{Pattern, SinglePlaceholder};

impl<'data> ZeroMapKV<'data> for Pattern<SinglePlaceholder, Cow<'data, str>> {
    type Container = ZeroVec<Cow<'data, str>>;
    type Slice = ZeroVec<Cow<'data, str>>;
    type GetType = Cow<'data, str>;
    type OwnedType = Cow<'data, str>;
}
