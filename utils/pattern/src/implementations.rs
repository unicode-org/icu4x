// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{Pattern, SinglePlaceholder, SinglePlaceholderPattern};

use zerovec::__zerovec_internal_reexport::boxed::Box;
use zerovec::{maps::ZeroMapKV, ule::VarULE, VarZeroSlice, VarZeroVec, ZeroVecError};

impl<'a> ZeroMapKV<'a> for Pattern<SinglePlaceholder, String> {
    type Container = VarZeroVec<'a, Pattern<SinglePlaceholder, String>>;
    type Slice = VarZeroSlice<Pattern<SinglePlaceholder, String>>;
    type GetType = Pattern<SinglePlaceholder, String>;
    type OwnedType = Box<Pattern<SinglePlaceholder, String>>;
}

unsafe impl VarULE for Pattern<SinglePlaceholder, String> {
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), ZeroVecError> {
        SinglePlaceholderPattern::try_from_bytes_store(bytes)
            .map_err(|_| ZeroVecError::VarZeroVecFormatError)?;
        Ok(())
    }

    unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &Self {
        // SAFETY: The `bytes` slice must be validated by `Self::validate_byte_slice`.
        let store = core::str::from_utf8_unchecked(bytes).to_owned();
        Box::leak(Box::new(SinglePlaceholderPattern::from_store_unchecked(
            store,
        )))
    }
}
