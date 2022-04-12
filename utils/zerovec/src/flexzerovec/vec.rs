// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::FlexZeroSlice;
use super::FlexZeroVecOwned;
use core::ops::Deref;

pub enum FlexZeroVec<'a> {
    Owned(FlexZeroVecOwned),
    Borrowed(&'a FlexZeroSlice),
}

impl<'a> Deref for FlexZeroVec<'a> {
    type Target = FlexZeroSlice;
    fn deref(&self) -> &Self::Target {
        match self {
            FlexZeroVec::Owned(v) => v.deref(),
            FlexZeroVec::Borrowed(v) => v,
        }
    }
}
