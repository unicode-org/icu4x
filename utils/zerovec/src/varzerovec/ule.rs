// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::components::SliceComponents;
use super::*;
use core::marker::PhantomData;
use core::mem;

// safety invariant: The slice MUST be one which parses to
// a valid SliceComponents<T>
#[repr(transparent)]
pub struct VarZeroVecULE<T> {
    marker: PhantomData<[T]>,
    /// The original slice this was constructed from
    entire_slice: [u8],
}

impl<T: AsVarULE> VarZeroVecULE<T> {
    fn get_components<'a>(&'a self) -> SliceComponents<'a, T> {
        unsafe {
            // safety: VarZeroVecULE is guaranteed to parse here
            SliceComponents::from_bytes_unchecked(&self.entire_slice)
        }
    }

    /// Get the number of elements in this vector
    pub fn len(&self) -> usize {
        self.get_components().len()
    }

    /// Returns `true` if the vector contains no elements.
    pub fn is_empty(&self) -> bool {
        self.get_components().is_empty()
    }

    /// Obtain an iterator over VarZeroVecULE's elements
    pub fn iter<'b>(&'b self) -> impl Iterator<Item = &'b T::VarULE> {
        self.get_components().iter()
    }

    /// Get one of VarZeroVecULE's elements, returning None if the index is out of bounds
    pub fn get(&self, idx: usize) -> Option<&T::VarULE> {
        self.get_components().get(idx)
    }

    /// Get this [`VarZeroVecULE`] as a borrowed [`VarZeroVec`]
    ///
    /// If you wish to repeatedly call methods on this [`VarZeroVecULE`],
    /// it is more efficient to perform this conversion first
    pub fn as_varzerovec<'a>(&'a self) -> VarZeroVec<'a, T> {
        VarZeroVec(VarZeroVecInner::Borrowed(self.get_components()))
    }
}

impl<T> VarZeroVecULE<T>
where
    T: AsVarULE,
    T::VarULE: Ord,
{
    /// Binary searches a sorted `VarZeroVecULE<T>` for the given element. For more information, see
    /// the primitive function [`binary_search`].
    ///
    /// [`binary_search`]: https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search
    #[inline]
    pub fn binary_search(&self, x: &T::VarULE) -> Result<usize, usize> {
        self.get_components().binary_search(x)
    }
}
unsafe impl<T: AsVarULE + 'static> VarULE for VarZeroVecULE<T> {
    type Error = ParseErrorFor<T>;

    fn parse_byte_slice(bytes: &[u8]) -> Result<&Self, Self::Error> {
        let _: SliceComponents<T> = SliceComponents::try_from_bytes(bytes)?;
        unsafe { Ok(Self::from_byte_slice_unchecked(bytes)) }
    }

    unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &Self {
        // self is really just a wrapper around a byte slice
        mem::transmute(bytes)
    }

    fn as_byte_slice(&self) -> &[u8] {
        &self.entire_slice
    }
}
