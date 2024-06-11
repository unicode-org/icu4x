// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use zerovec::{maps::ZeroMapKV, VarZeroSlice, VarZeroVec};

use crate::{Pattern, SinglePlaceholder};

impl<'data> ZeroMapKV<'data> for Pattern<SinglePlaceholder, str> {
    type Container = VarZeroVec<'data, Pattern<SinglePlaceholder, str>>;
    type Slice = VarZeroSlice<Pattern<SinglePlaceholder, str>>;
    type GetType = Pattern<SinglePlaceholder, str>;
    type OwnedType = Pattern<SinglePlaceholder, String>;
}
