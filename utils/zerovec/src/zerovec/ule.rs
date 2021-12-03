// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;

/// A type similar to [`ZeroVec`] that can be used inside `VarZeroVec<T>`
#[repr(transparent)]
pub struct ZeroVecULE<T: AsULE + ?Sized>(pub [T::ULE]);

impl<T> ZeroVecULE<T>
where
    T: AsULE + ?Sized,
{
    #[inline]
    pub fn as_zerovec(&self) -> ZeroVec<'_, T> {
        ZeroVec::Borrowed(&self.0)
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
