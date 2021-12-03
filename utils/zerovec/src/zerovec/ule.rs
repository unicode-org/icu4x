// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;
use alloc::boxed::Box;

/// A type similar to [`ZeroVec`] that can be used inside `VarZeroVec<T>`
#[repr(transparent)]
pub struct ZeroVecULE<T: AsULE>(pub [T::ULE]);

impl<T> ZeroVecULE<T>
where
    T: AsULE,
{
    /// Get this [`ZeroVecULE`] as a borrowed [`ZeroVec`]
    #[inline]
    pub fn as_zerovec(&self) -> ZeroVec<'_, T> {
        ZeroVec::Borrowed(&self.0)
    }

    /// Construct a `&ZeroVecULE<T>` from a slice of ULEs
    #[inline]
    pub fn from_slice(slice: &[T::ULE]) -> &Self {
        unsafe { &*(slice as *const _ as *const Self) }
    }

    /// Construct a `Box<ZeroVecULE<T>>` from a boxed slice of ULEs
    #[inline]
    pub fn from_boxed_slice(slice: Box<[T::ULE]>) -> Box<Self> {
        unsafe { Box::from_raw(Box::into_raw(slice) as *mut Self) }
    }
}

impl<T> ZeroVecULE<T>
where
    T: AsULE,
{
    /// Gets the element at the specified index. Returns None if out of range.
    #[inline]
    pub fn get(&self, index: usize) -> Option<T> {
        self.as_zerovec().get(index)
    }

    /// Gets an iterator over the elements.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = T> + '_ {
        self.0.iter().copied().map(T::from_unaligned)
    }
}

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
unsafe impl<T: AsULE + 'static> VarULE for ZeroVecULE<T> {
    type Error = <T::ULE as ULE>::Error;

    #[inline]
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), Self::Error> {
        T::ULE::validate_byte_slice(bytes)
    }

    #[inline]
    fn parse_byte_slice(bytes: &[u8]) -> Result<&Self, Self::Error> {
        Ok(Self::from_slice(T::ULE::parse_byte_slice(bytes)?))
    }
    #[inline]
    unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &Self {
        Self::from_slice(T::ULE::from_byte_slice_unchecked(bytes))
    }
    #[inline]
    fn as_byte_slice(&self) -> &[u8] {
        T::ULE::as_byte_slice(&self.0)
    }
}
