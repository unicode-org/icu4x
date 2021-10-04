// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::ule::*;

// This is safe to implement because from_byte_slice_unchecked returns
// the same value as parse_byte_slice
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
