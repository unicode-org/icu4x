// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{Pattern, SinglePlaceholder, SinglePlaceholderPattern};

#[cfg(feature = "alloc")]
use alloc::boxed::Box;
use zerovec::{maps::ZeroMapKV, ule::VarULE, VarZeroSlice, VarZeroVec, ZeroVecError};

impl<'a> ZeroMapKV<'a> for Pattern<SinglePlaceholder, str> {
    type Container = VarZeroVec<'a, Pattern<SinglePlaceholder, str>>;
    type Slice = VarZeroSlice<Pattern<SinglePlaceholder, str>>;
    type GetType = Pattern<SinglePlaceholder, str>;
    type OwnedType = Box<Pattern<SinglePlaceholder, str>>;
}

#[cfg(feature = "alloc")]
unsafe impl VarULE for Pattern<SinglePlaceholder, str> {
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), ZeroVecError> {
        SinglePlaceholderPattern::try_from_bytes_store(bytes)
            .map_err(|_| ZeroVecError::VarZeroVecFormatError)?;
        Ok(())
    }

    unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &Self {
        // SAFETY: The `bytes` slice must be validated by `Self::validate_byte_slice`.
        let store = core::str::from_utf8_unchecked(bytes);
        SinglePlaceholderPattern::from_borrowed_store_unchecked(store)
    }
}
