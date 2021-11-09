// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::ule::*;
use crate::varzerovec::owned::VarZeroVecOwned;
use crate::varzerovec::VarZeroVecBorrowed;
use crate::VarZeroVec;
use crate::ZeroVec;
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::mem;

/// Trait abstracting over [`ZeroVec`] and [`VarZeroVec`], for use in [`ZeroMap`](super::ZeroMap). You
/// should not be implementing or calling this trait directly.
pub trait ZeroVecLike<'a, T: ?Sized> {
    /// The type received by `Self::binary_search()`
    type NeedleType: ?Sized;
    /// The type returned by `Self::get()`
    type GetType: ?Sized + 'static;
    /// Search for a key in a sorted vector, returns `Ok(index)` if found,
    /// returns `Err(insert_index)` if not found, where `insert_index` is the
    /// index where it should be inserted to maintain sort order.
    fn binary_search(&self, k: &Self::NeedleType) -> Result<usize, usize>;
    /// Get element at `index`
    fn get(&self, index: usize) -> Option<&Self::GetType>;
    /// The length of this vector
    fn len(&self) -> usize;
    /// Check if this vector is in ascending order according to `T`s `Ord` impl
    fn is_ascending(&self) -> bool;
    /// Check if this vector is empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Trait abstracting over [`ZeroVec`] and [`VarZeroVec`], for use in [`ZeroMap`](super::ZeroMap). You
/// should not be implementing or calling this trait directly.
///
/// This trait augments [`ZeroVecLike`] with methods allowing for taking
/// longer references to the underlying buffer, for borrowed-only vector types.
pub trait BorrowedOnlyZeroVecLike<'a, T: ?Sized>: ZeroVecLike<'a, T> {
    /// Get element at `index`, with a longer lifetime
    fn get_lengthened(&self, index: usize) -> Option<&'a Self::GetType>;
}

/// Trait abstracting over [`ZeroVec`] and [`VarZeroVec`], for use in [`ZeroMap`](super::ZeroMap). You
/// should not be implementing or calling this trait directly.
///
/// This trait augments [`ZeroVecLike`] with methods allowing for mutation of the underlying
/// vector for owned vector types.
pub trait MutableZeroVecLike<'a, T: ?Sized>: ZeroVecLike<'a, T> {
    /// The type returned by `Self::remove()` and `Self::replace()`
    type OwnedType;
    /// A fully borrowed version of this
    type BorrowedVersion: ZeroVecLike<'a, T, NeedleType = Self::NeedleType, GetType = Self::GetType>
        + BorrowedOnlyZeroVecLike<'a, T>
        + Copy;
    /// Insert an element at `index`
    fn insert(&mut self, index: usize, value: &T);
    /// Remove the element at `index` (panicking if nonexistant)
    fn remove(&mut self, index: usize) -> Self::OwnedType;
    /// Replace the element at `index` with another one, returning the old element
    fn replace(&mut self, index: usize, value: &T) -> Self::OwnedType;
    /// Push an element to the end of this vector
    fn push(&mut self, value: &T);
    /// Create a new, empty vector
    fn new() -> Self;
    /// Create a new, empty vector, with given capacity
    fn with_capacity(cap: usize) -> Self;
    /// Remove all elements from the vector
    fn clear(&mut self);
    /// Reserve space for `addl` additional elements
    fn reserve(&mut self, addl: usize);
    /// Construct a borrowed version from this
    ///
    /// Note: This really should be `&'b self -> Self::BorrowedVersion<'b>`
    /// but doing that requires complicated `for<'b>` code that will likely trigger
    /// compiler bugs. Instead, we hope that `self` is covariant so this cast will
    /// just work in the implementation. Basically, we rely on the compiler
    /// casting `&'b Self<'a>` to `&'b Self<'b>` when this gets called, which works
    /// out for `ZeroVec` and `VarZeroVec` containers just fine.
    fn as_borrowed(&'a self) -> Self::BorrowedVersion;

    /// If this type *contains* its borrowed version, return that. Returns `None`
    /// when this contains owned data.
    ///
    /// This is subtly different from the [`Self::as_borrowed()`] since the returned type does
    /// is not tied to the lifetime of the reference `&self`. The tradeoff here is that this
    /// returns `None` when owned data is encountered, while we gain the ability to refer to the
    /// wider lifetime when we *know* for a fact that it is borrowed data.
    ///
    /// These are useful to ensure serialization parity between borrowed and owned versions
    fn maybe_as_borrowed(&self) -> Option<Self::BorrowedVersion>;

    /// Construct from the borrowed version of the type
    ///
    /// These are useful to ensure serialization parity between borrowed and owned versions
    fn from_borrowed(b: Self::BorrowedVersion) -> Self;
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
    fn len(&self) -> usize {
        self.len()
    }
    fn is_ascending(&self) -> bool {
        self.as_slice()
            .windows(2)
            .all(|w| T::from_unaligned(w[1]).cmp(&T::from_unaligned(w[0])) == Ordering::Greater)
    }
}

impl<'a, T> ZeroVecLike<'a, T> for &'a [T::ULE]
where
    T: AsULE + Ord + Copy,
{
    type NeedleType = T;
    type GetType = T::ULE;
    fn binary_search(&self, k: &T) -> Result<usize, usize> {
        ZeroVec::<T>::Borrowed(self).binary_search(k)
    }
    fn get(&self, index: usize) -> Option<&T::ULE> {
        <[T::ULE]>::get(self, index)
    }
    fn len(&self) -> usize {
        <[T::ULE]>::len(self)
    }
    fn is_ascending(&self) -> bool {
        ZeroVec::<T>::Borrowed(self)
            .as_slice()
            .windows(2)
            .all(|w| T::from_unaligned(w[1]).cmp(&T::from_unaligned(w[0])) == Ordering::Greater)
    }
}

impl<'a, T> BorrowedOnlyZeroVecLike<'a, T> for &'a [T::ULE]
where
    T: AsULE + Ord + Copy,
{
    fn get_lengthened(&self, index: usize) -> Option<&'a T::ULE> {
        <[T::ULE]>::get(self, index)
    }
}

impl<'a, T> MutableZeroVecLike<'a, T> for ZeroVec<'a, T>
where
    T: AsULE + Ord + Copy,
{
    type OwnedType = T;
    type BorrowedVersion = &'a [T::ULE];
    fn insert(&mut self, index: usize, value: &T) {
        self.to_mut().insert(index, value.as_unaligned())
    }
    fn remove(&mut self, index: usize) -> T {
        T::from_unaligned(self.to_mut().remove(index))
    }
    fn replace(&mut self, index: usize, value: &T) -> T {
        let vec = self.to_mut();
        T::from_unaligned(mem::replace(&mut vec[index], value.as_unaligned()))
    }
    fn push(&mut self, value: &T) {
        self.to_mut().push(value.as_unaligned())
    }
    fn new() -> Self {
        ZeroVec::Owned(Vec::new())
    }
    fn with_capacity(cap: usize) -> Self {
        ZeroVec::Owned(Vec::with_capacity(cap))
    }
    fn clear(&mut self) {
        self.to_mut().clear()
    }
    fn reserve(&mut self, addl: usize) {
        self.to_mut().reserve(addl)
    }

    fn as_borrowed(&'a self) -> &'a [T::ULE] {
        self.as_slice()
    }
    fn maybe_as_borrowed(&self) -> Option<&'a [T::ULE]> {
        if let ZeroVec::Borrowed(b) = *self {
            Some(b)
        } else {
            None
        }
    }
    fn from_borrowed(b: &'a [T::ULE]) -> Self {
        ZeroVec::Borrowed(b)
    }
}

impl<'a, T> ZeroVecLike<'a, T> for VarZeroVec<'a, T>
where
    T: VarULE,
    T: Ord,
    T: ?Sized,
{
    type NeedleType = T;
    type GetType = T;
    fn binary_search(&self, k: &T) -> Result<usize, usize> {
        self.binary_search(k)
    }
    fn get(&self, index: usize) -> Option<&T> {
        self.get(index)
    }
    fn len(&self) -> usize {
        self.len()
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

impl<'a, T> ZeroVecLike<'a, T> for VarZeroVecBorrowed<'a, T>
where
    T: VarULE,
    T: Ord,
    T: ?Sized,
{
    type NeedleType = T;
    type GetType = T;
    fn binary_search(&self, k: &T) -> Result<usize, usize> {
        Self::binary_search(self, k)
    }
    fn get(&self, index: usize) -> Option<&T> {
        // using UFCS to avoid accidental recursion
        Self::get(*self, index)
    }
    fn len(&self) -> usize {
        Self::len(*self)
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

impl<'a, T> BorrowedOnlyZeroVecLike<'a, T> for VarZeroVecBorrowed<'a, T>
where
    T: VarULE,
    T: Ord,
    T: ?Sized,
{
    fn get_lengthened(&self, index: usize) -> Option<&'a T> {
        // using UFCS to avoid accidental recursion
        Self::get(*self, index)
    }
}

impl<'a, T> MutableZeroVecLike<'a, T> for VarZeroVec<'a, T>
where
    T: VarULE,
    T: Ord,
    T: ?Sized,
{
    type OwnedType = Box<T>;
    type BorrowedVersion = VarZeroVecBorrowed<'a, T>;
    fn insert(&mut self, index: usize, value: &T) {
        self.make_mut().insert(index, value)
    }
    fn remove(&mut self, index: usize) -> Box<T> {
        let vec = self.make_mut();
        let old = vec.get(index).expect("invalid index").to_boxed();
        vec.remove(index);
        old
    }
    fn replace(&mut self, index: usize, value: &T) -> Box<T> {
        let vec = self.make_mut();
        let old = vec.get(index).expect("invalid index").to_boxed();
        vec.replace(index, value);
        old
    }
    fn push(&mut self, value: &T) {
        let len = self.len();
        self.make_mut().insert(len, value)
    }
    fn new() -> Self {
        VarZeroVecOwned::new().into()
    }
    fn with_capacity(cap: usize) -> Self {
        VarZeroVecOwned::with_capacity(cap).into()
    }
    fn clear(&mut self) {
        self.make_mut().clear()
    }
    fn reserve(&mut self, addl: usize) {
        self.make_mut().reserve(addl)
    }
    fn as_borrowed(&'a self) -> VarZeroVecBorrowed<'a, T> {
        self.as_borrowed()
    }
    fn maybe_as_borrowed(&self) -> Option<VarZeroVecBorrowed<'a, T>> {
        if let VarZeroVec::Borrowed(b) = *self {
            Some(b)
        } else {
            None
        }
    }
    fn from_borrowed(b: VarZeroVecBorrowed<'a, T>) -> Self {
        VarZeroVec::Borrowed(b)
    }
}
