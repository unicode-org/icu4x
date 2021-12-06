// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::ule::*;
use core::str;

// Safety (based on the safety checklist on the VarULE trait):
//  1. str does not include any uninitialized or padding bytes.
//  2. str is aligned to 1 byte.
//  3. The impl of `validate_byte_slice()` returns an error if any byte is not valid.
//  4. The impl of `validate_byte_slice()` returns an error if the slice cannot be used in its entirety
//  5. The impl of `from_byte_slice_unchecked()` returns a reference to the same data.
//  6. `parse_byte_slice()` is equivalent to `validate_byte_slice()` followed by `from_byte_slice_unchecked()`
//  7. str byte equality is semantic equality
unsafe impl VarULE for str {
    type Error = str::Utf8Error;

    #[inline]
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), Self::Error> {
        str::from_utf8(bytes)?;
        Ok(())
    }

    #[inline]
    fn parse_byte_slice(bytes: &[u8]) -> Result<&Self, Self::Error> {
        str::from_utf8(bytes)
    }
    /// Invariant: must be safe to call when called on a slice that previously
    /// succeeded with `parse_byte_slice`
    #[inline]
    unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &Self {
        str::from_utf8_unchecked(bytes)
    }
}

// Safety (based on the safety checklist on the VarULE trait):
//  1. [u8] does not include any uninitialized or padding bytes (achieved by being a slice of a ULE type)
//  2. [u8] is aligned to 1 byte (achieved by being a slice of a ULE type)
//  3. The impl of `validate_byte_slice()` returns an error if any byte is not valid.
//  4. The impl of `validate_byte_slice()` returns an error if the slice cannot be used in its entirety
//  5. The impl of `from_byte_slice_unchecked()` returns a reference to the same data.
//  6. All other methods are defaulted
//  7. `[u8]` byte equality is semantic equality
unsafe impl VarULE for [u8] {
    type Error = core::convert::Infallible;

    #[inline]
    fn validate_byte_slice(_: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &Self {
        bytes
    }
}
