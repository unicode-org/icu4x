// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{Pattern, PatternBackend};

use alloc::boxed::Box;
use zerovec::{
    maps::ZeroMapKV,
    ule::{UleError, VarULE},
    VarZeroSlice, VarZeroVec,
};

impl<'a, B: PatternBackend> ZeroMapKV<'a> for Pattern<B>
where
    Pattern<B>: VarULE,
{
    type Container = VarZeroVec<'a, Pattern<B>>;
    type Slice = VarZeroSlice<Pattern<B>>;
    type GetType = Pattern<B>;
    type OwnedType = Box<Pattern<B>>;
}

/// Implement `VarULE` for `Pattern<SinglePlaceholder, str>`.
///
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
unsafe impl<B> VarULE for Pattern<B>
where
    B: PatternBackend<Store = str>,
{
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), UleError> {
        B::try_store_from_bytes(bytes).map_err(|_| UleError::parse::<Self>())?;
        Ok(())
    }

    unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &Self {
        // SAFETY:
        //      As `validate_byte_slice` succeeded, this means `store` is valid.
        //      And the layout of `Pattern` is the same as `B::Store`
        //      because `_backend` is zero-sized and `store` is the only other field.
        core::mem::transmute_copy(&bytes)
    }
}
