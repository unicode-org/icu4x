// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::ule::*;

// Safety (based on the safety checklist on the VarULE trait):
//  1. [T] does not include any uninitialized or padding bytes (achieved by being a slice of a ULE type)
//  2. [T] is aligned to 1 byte (achieved by being a slice of a ULE type)
//  3. The impl of `validate_byte_slice()` returns an error if any byte is not valid.
//  4. The impl of `validate_byte_slice()` returns an error if the slice cannot be used in its entirety
//  5. The impl of `from_byte_slice_unchecked()` returns a reference to the same data.
//  6. `as_byte_slice()` is equivalent to a regular transmute of the underlying data; `parse_byte_slice()`
//     is equivalent to `validate_byte_slice()` followed by `from_byte_slice_unchecked()` due to
//     the guarantee from `ULE`
//  7. `[T]` byte equality is semantic equality (relying on the guideline of the underlying `ULE` type)
unsafe impl<T> VarULE for [T]
where
    T: ULE,
{
    type Error = T::Error;

    #[inline]
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), Self::Error> {
        T::validate_byte_slice(bytes)
    }

    #[inline]
    fn parse_byte_slice(bytes: &[u8]) -> Result<&Self, Self::Error> {
        T::parse_byte_slice(bytes)
    }
    #[inline]
    unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &Self {
        T::from_byte_slice_unchecked(bytes)
    }
    #[inline]
    fn as_byte_slice(&self) -> &[u8] {
        T::as_byte_slice(self)
    }
}
