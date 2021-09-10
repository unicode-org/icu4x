// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::ule::*;
use crate::ZeroVec;

impl<T> AsVarULE for Vec<T>
where
    T: ULE + Clone,
{
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

impl<T> AsVarULE for ZeroVec<'static, T>
where
    T: AsULE,
    T::ULE: Clone,
{
    type VarULE = [T::ULE];
    #[inline]
    fn as_unaligned(&self) -> &[T::ULE] {
        self.as_slice()
    }
    #[inline]
    fn from_unaligned(unaligned: &[T::ULE]) -> Self {
        ZeroVec::Owned(unaligned.into())
    }
}

// This is safe to implement because from_byte_slice_unchecked returns
// the same value as parse_byte_slice
unsafe impl<T> VarULE for [T]
where
    T: ULE,
{
    type Error = T::Error;

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
