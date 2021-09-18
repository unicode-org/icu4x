// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;
use core::marker::PhantomData;

pub struct VarZeroVecOwned<T> {
    marker: PhantomData<[T]>,
    // safety invariant: must parse into a valid SliceComponents
    entire_slice: Vec<u8>,
}

impl<T: AsVarULE> VarZeroVecOwned<T> {
    /// Construct a VarZeroVecOwned from a list of elements
    pub fn new(elements: &[T]) -> Option<Self> {
        Some(Self {
            marker: PhantomData,
            entire_slice: components::get_serializable_bytes(elements)?,
        })
    }

    fn get_components<'a>(&'a self) -> SliceComponents<'a, T> {
        unsafe {
            // safety: VarZeroVecOwned is guaranteed to parse here
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

    /// Obtain an iterator over VarZeroVecOwned's elements
    pub fn iter<'b>(&'b self) -> impl Iterator<Item = &'b T::VarULE> {
        self.get_components().iter()
    }

    /// Get one of VarZeroVecOwned's elements, returning None if the index is out of bounds
    pub fn get(&self, idx: usize) -> Option<&T::VarULE> {
        self.get_components().get(idx)
    }

    /// Get this [`VarZeroVecOwned`] as a borrowed [`VarZeroVec`]
    ///
    /// If you wish to repeatedly call methods on this [`VarZeroVecOwned`],
    /// it is more efficient to perform this conversion first
    pub fn as_varzerovec<'a>(&'a self) -> VarZeroVec<'a, T> {
        VarZeroVec(VarZeroVecInner::Borrowed(self.get_components()))
    }

    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.get_components().to_vec()
    }
}

impl<T> VarZeroVecOwned<T>
where
    T: AsVarULE,
    T::VarULE: Ord,
{
    /// Binary searches a sorted `VarZeroVecOwned<T>` for the given element. For more information, see
    /// the primitive function [`binary_search`].
    ///
    /// [`binary_search`]: https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search
    #[inline]
    pub fn binary_search(&self, x: &T::VarULE) -> Result<usize, usize> {
        self.get_components().binary_search(x)
    }
}
