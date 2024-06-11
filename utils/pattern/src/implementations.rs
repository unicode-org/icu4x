// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use zerovec::{maps::ZeroMapKV, ule::VarULE, VarZeroSlice, VarZeroVec};

use crate::{Pattern, SinglePlaceholder};

impl<'a> ZeroMapKV<'a> for Pattern<SinglePlaceholder, str> {
    type Container = VarZeroVec<'a, Pattern<SinglePlaceholder, str>>;
    type Slice = VarZeroSlice<Pattern<SinglePlaceholder, str>>;
    type GetType = Pattern<SinglePlaceholder, str>;
    type OwnedType = Box<Pattern<SinglePlaceholder, str>>;
}

unsafe impl VarULE for Pattern<SinglePlaceholder, str> {
    fn validate_byte_slice(_bytes: &[u8]) -> Result<(), zerovec::ZeroVecError> {
        todo!()
    }

    unsafe fn from_byte_slice_unchecked(_bytes: &[u8]) -> &Self {
        todo!()
    }
}
