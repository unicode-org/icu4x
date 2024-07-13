// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;
use ::zerovec::{ule::VarULE, ZeroVecError};

/// Implement `VarULE` for `Pattern`.
///
/// # Safety
///
/// Note: `Pattern<B, B::Store>` is transparent over `B::Store`.
///
/// Safety checklist for `ULE`:
///
/// 1. Satisfied by `B::Store` being `VarULE` by the impl bound.
/// 2. Satisfied by `B::Store` being `VarULE` by the impl bound.
/// 3. The implementation of `validate_byte_slice()` returns an error
///    if any byte is not valid.
/// 4. The implementation of `validate_byte_slice()` returns an error
///    if the slice cannot be used to build a `Pattern<B, B::Store>`
///    in its entirety.
/// 5. The implementation of `from_byte_slice_unchecked()` returns a reference to the same data.
/// 6. All other methods *must* be left with their default impl.
/// 7. Satisfied by `B::Store` being `VarULE` by the impl bound.
unsafe impl<B> VarULE for Pattern<B, B::Store>
where
    B: PatternBackend,
    B::Store: VarULE,
{
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), ZeroVecError> {
        match Self::try_from_bytes_store(bytes) {
            Ok(_) => Ok(()),
            Err(_) => Err(ZeroVecError::parse::<Self>()),
        }
    }
    unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &Self {
        Self::from_bytes_store_unchecked(bytes)
    }
}

// TODO:
// impl<'a> ZeroMapKV<'a> for Pattern<SinglePlaceholder, str> {
//     type Container = VarZeroVec<'a, Pattern<SinglePlaceholder, str>>;
//     type Slice = VarZeroSlice<Pattern<SinglePlaceholder, str>>;
//     type GetType = Pattern<SinglePlaceholder, str>;
//     type OwnedType = Box<Pattern<SinglePlaceholder, str>>;
// }
