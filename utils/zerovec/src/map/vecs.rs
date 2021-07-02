// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::ule::*;
use crate::VarZeroVec;
use crate::ZeroVec;
use std::cmp::Ordering;
use std::mem;

/// Trait abstracting over [`ZeroVec`] and [`VarZeroVec`], for use in [`ZeroMap`](super::ZeroMap). You
/// should not be implementing or calling this trait directly.
pub trait ZeroVecLike<'a, T> {
    /// The type received by `Self::binary_search()`
    type NeedleType: ?Sized;
    /// The type returned by `Self::get()`
    type GetType: ?Sized;
    /// Search for a key in a sorted vector, returns `Ok(index)` if found,
    /// returns `Err(insert_index)` if not found, where `insert_index` is the
    /// index where it should be inserted to maintain sort order.
    fn binary_search(&self, k: &Self::NeedleType) -> Result<usize, usize>;
    /// Get element at `index`
    fn get(&self, index: usize) -> Option<&Self::GetType>;
    /// Insert an element at `index`
    fn insert(&mut self, index: usize, value: T);
    /// Remove the element at `index` (panicking if nonexistant)
    fn remove(&mut self, index: usize) -> T;
    /// Replace the element at `index` with another one, returning the old element
    fn replace(&mut self, index: usize, value: T) -> T;
    /// Push an element to the end of this vector
    fn push(&mut self, value: T);
    /// The length of this vector
    fn len(&self) -> usize;
    /// Create a new, empty vector
    fn new() -> Self;
    /// Create a new, empty vector, with given capacity
    fn with_capacity(cap: usize) -> Self;
    /// Remove all elements from the vector
    fn clear(&mut self);
    /// Reserve space for `addl` additional elements
    fn reserve(&mut self, addl: usize);
    /// Check if this vector is in ascending order according to `T`s `Ord` impl
    fn is_ascending(&self) -> bool;
    /// Check if this vector is empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'a, T> ZeroVecLike<'a, T> for ZeroVec<'a, T>
where
    T: AsULE + Ord + Copy,
{
    type NeedleType = T;
    type GetType = T::ULE;
    fn binary_search(&self, k: &T) -> Result<usize, usize> {
        self.binary_search(k)
    }
    fn get(&self, index: usize) -> Option<&T::ULE> {
        self.get_ule_ref(index)
    }
    fn insert(&mut self, index: usize, value: T) {
        self.make_mut().insert(index, value.as_unaligned())
    }
    fn remove(&mut self, index: usize) -> T {
        T::from_unaligned(&self.make_mut().remove(index))
    }
    fn replace(&mut self, index: usize, value: T) -> T {
        let vec = self.make_mut();
        T::from_unaligned(&mem::replace(&mut vec[index], value.as_unaligned()))
    }
    fn push(&mut self, value: T) {
        self.make_mut().push(value.as_unaligned())
    }
    fn len(&self) -> usize {
        self.len()
    }
    fn new() -> Self {
        ZeroVec::Owned(Vec::new())
    }
    fn with_capacity(cap: usize) -> Self {
        ZeroVec::Owned(Vec::with_capacity(cap))
    }
    fn clear(&mut self) {
        self.make_mut().clear()
    }
    fn reserve(&mut self, addl: usize) {
        self.make_mut().reserve(addl)
    }
    fn is_ascending(&self) -> bool {
        self.as_slice()
            .windows(2)
            .all(|w| T::from_unaligned(&w[1]).cmp(&T::from_unaligned(&w[0])) == Ordering::Greater)
    }
}

impl<'a, T> ZeroVecLike<'a, T> for VarZeroVec<'a, T>
where
    T: AsVarULE + Clone,
    T::VarULE: Ord,
{
    type NeedleType = T::VarULE;
    type GetType = T::VarULE;
    fn binary_search(&self, k: &T::VarULE) -> Result<usize, usize> {
        self.binary_search(k)
    }
    fn get(&self, index: usize) -> Option<&T::VarULE> {
        self.get(index)
    }
    fn insert(&mut self, index: usize, value: T) {
        self.make_mut().insert(index, value)
    }
    fn remove(&mut self, index: usize) -> T {
        self.make_mut().remove(index)
    }
    fn replace(&mut self, index: usize, value: T) -> T {
        let vec = self.make_mut();
        mem::replace(&mut vec[index], value)
    }
    fn push(&mut self, value: T) {
        self.make_mut().push(value)
    }
    fn len(&self) -> usize {
        self.len()
    }
    fn new() -> Self {
        Vec::new().into()
    }
    fn with_capacity(cap: usize) -> Self {
        Vec::with_capacity(cap).into()
    }
    fn clear(&mut self) {
        self.make_mut().clear()
    }
    fn reserve(&mut self, addl: usize) {
        self.make_mut().reserve(addl)
    }
    fn is_ascending(&self) -> bool {
        if !self.is_empty() {
            let mut prev = self.get(0).unwrap();
            for element in self.iter().skip(1) {
                if element.cmp(prev) != Ordering::Greater {
                    return false;
                }
                prev = element;
            }
        }
        true
    }
}
