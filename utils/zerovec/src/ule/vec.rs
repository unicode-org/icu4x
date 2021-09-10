// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::ule::*;

impl<T> AsVarULE for Vec<T> where T: ULE + Clone {
    type VarULE = [T];
    #[inline]
    fn as_unaligned(&self) -> &[T] {
        self
    }
    #[inline]
    fn from_unaligned(unaligned: &[T]) -> Self {
        unaligned.into()
    }
}

impl<T> VarULE for [T]
where
    T: ULE,
{
    type Error = T::Error;

    #[inline]
    fn parse_byte_slice(bytes: &[u8]) -> Result<&Self, Self::Error> {
        T::parse_byte_slice(bytes)
    }
    /// Invariant: must be safe to call when called on a slice that previously
    /// succeeded with `parse_byte_slice`
    #[inline]
    unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &Self {
        T::from_byte_slice_unchecked(bytes)
    }
    #[inline]
    fn as_byte_slice(&self) -> &[u8] {
        T::as_byte_slice(self)
    }
}
