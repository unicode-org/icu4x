// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{Pattern, SinglePlaceholder, SinglePlaceholderPattern};

use alloc::boxed::Box;
use zerovec::{maps::ZeroMapKV, ule::VarULE, VarZeroSlice, VarZeroVec, ZeroVecError};

impl<'a> ZeroMapKV<'a> for Pattern<SinglePlaceholder, str> {
    type Container = VarZeroVec<'a, Pattern<SinglePlaceholder, str>>;
    type Slice = VarZeroSlice<Pattern<SinglePlaceholder, str>>;
    type GetType = Pattern<SinglePlaceholder, str>;
    type OwnedType = Box<Pattern<SinglePlaceholder, str>>;
}

/// # Safety
///
/// Safety checklist for `ULE`:
///
/// 1. `str` does not include any uninitialized or padding bytes.
/// 2. `str` is aligned to 1 byte.
/// 3. The implementation of `validate_byte_slice()` returns an error
///    if any byte is not valid.
/// 4. The implementation of `validate_byte_slice()` returns an error
///    if the slice cannot be used to build a `Pattern<SinglePlaceholder, str>`
///    in its entirety.
/// 5. The implementation of `from_byte_slice_unchecked()` returns a reference to the same data.
/// 6. `parse_byte_slice()` is equivalent to `validate_byte_slice()` followed by `from_byte_slice_unchecked()`.
/// 7. `str` byte equality is semantic equality.
unsafe impl VarULE for Pattern<SinglePlaceholder, str> {
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), ZeroVecError> {
        SinglePlaceholderPattern::try_from_utf8_store(bytes)
            .map_err(|_| ZeroVecError::VarZeroVecFormatError)?;
        Ok(())
    }

    unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &Self {
        // SAFETY: As `validate_byte_slice` succeeded, `try_from_utf8_store` succeeded, which implies valid UTF-8
        let store = core::str::from_utf8_unchecked(bytes);

        // SAFETY: As `validate_byte_slice` succeeded, `try_from_utf8_store` also succeeded
        SinglePlaceholderPattern::from_borrowed_store_unchecked(store)
    }
}
