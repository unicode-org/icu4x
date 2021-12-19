// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::ule::*;
use crate::varzerovec::owned::VarZeroVecOwned;
use crate::varzerovec::VarZeroVecBorrowed;
use crate::VarZeroVec;
use crate::{ZeroSlice, ZeroVec};
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::mem;

/// Trait abstracting over [`ZeroVec`] and [`VarZeroVec`], for use in [`ZeroMap`](super::ZeroMap). You
/// should not be implementing or calling this trait directly.
///
/// The T type is the type received by [`Self::binary_search()`], as well as the one used
/// for human-readable serialization.
pub trait ZeroVecLike<'a, T: ?Sized> {
    /// The type returned by `Self::get()`
    type GetType: ?Sized + 'static;

    /// Create a new, empty vector
    fn zvl_new() -> Self;
    /// Search for a key in a sorted vector, returns `Ok(index)` if found,
    /// returns `Err(insert_index)` if not found, where `insert_index` is the
    /// index where it should be inserted to maintain sort order.
    fn zvl_binary_search(&self, k: &T) -> Result<usize, usize>;
    /// Get element at `index`
    fn zvl_get(&self, index: usize) -> Option<&Self::GetType>;
    /// The length of this vector
    fn len(&self) -> usize;
    /// Check if this vector is in ascending order according to `T`s `Ord` impl
    fn is_ascending(&self) -> bool;
    /// Check if this vector is empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Compare this type with a `Self::GetType`. This must produce the same result as
    /// if `g` were converted to `Self`
    fn t_cmp_get(t: &T, g: &Self::GetType) -> Ordering;

    /// Obtain a version of T suitable for serialization
    ///
    /// This uses a callback because it's not possible to return owned-or-borrowed
    /// types without GATs
    fn t_with_ser<R>(g: &Self::GetType, f: impl FnOnce(&T) -> R) -> R;
}

/// Trait abstracting over [`ZeroVec`] and [`VarZeroVec`], for use in [`ZeroMap`](super::ZeroMap). You
/// should not be implementing or calling this trait directly.
///
/// This trait augments [`ZeroVecLike`] with methods allowing for taking
/// longer references to the underlying buffer, for borrowed-only vector types.
pub trait BorrowedZeroVecLike<'a, T: ?Sized>: ZeroVecLike<'a, T> {
    /// Get element at `index`, with a longer lifetime
    fn zvl_get_borrowed(&self, index: usize) -> Option<&'a Self::GetType>;
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
    type BorrowedVariant: ZeroVecLike<'a, T, GetType = Self::GetType>
        + BorrowedZeroVecLike<'a, T>
        + Copy;

    /// Insert an element at `index`
    fn insert(&mut self, index: usize, value: &T);
    /// Remove the element at `index` (panicking if nonexistant)
    fn remove(&mut self, index: usize) -> Self::OwnedType;
    /// Replace the element at `index` with another one, returning the old element
    fn replace(&mut self, index: usize, value: &T) -> Self::OwnedType;
    /// Push an element to the end of this vector
    fn push(&mut self, value: &T);
    /// Create a new, empty vector, with given capacity
    fn with_capacity(cap: usize) -> Self;
    /// Remove all elements from the vector
    fn clear(&mut self);
    /// Reserve space for `addl` additional elements
    fn reserve(&mut self, addl: usize);
    /// Construct a borrowed variant by borrowing from `&self`.
    ///
    /// This function behaves like `&'b self -> Self::BorrowedVariant<'b>`,
    /// where `'b` is the lifetime of the reference to this object.
    ///
    /// Note: We rely on the compiler recognizing `'a` and `'b` as covariant and
    /// casting `&'b Self<'a>` to `&'b Self<'b>` when this gets called, which works
    /// out for `ZeroVec` and `VarZeroVec` containers just fine.
    fn as_borrowed(&'a self) -> Self::BorrowedVariant;

    /// Extract the inner borrowed variant if possible. Returns `None` if the data is owned.
    ///
    /// This function behaves like `&'_ self -> Self::BorrowedVariant<'a>`,
    /// where `'a` is the lifetime of this object's borrowed data.
    ///
    /// This function is similar to matching the `Borrowed` variant of `ZeroVec`
    /// or `VarZeroVec`, returning the inner borrowed type.
    fn as_borrowed_inner(&self) -> Option<Self::BorrowedVariant>;

    /// Construct from the borrowed version of the type
    ///
    /// These are useful to ensure serialization parity between borrowed and owned versions
    fn from_borrowed(b: Self::BorrowedVariant) -> Self;

    /// Convert an owned value to a borrowed T
    fn owned_as_t(o: &Self::OwnedType) -> &T;
}

impl<'a, T> ZeroVecLike<'a, T> for ZeroVec<'a, T>
where
    T: AsULE + Ord + Copy,
{
    type GetType = T::ULE;
    fn zvl_new() -> Self {
        Self::new()
    }
    fn zvl_binary_search(&self, k: &T) -> Result<usize, usize> {
        ZeroSlice::binary_search(self, k)
    }
    fn zvl_get(&self, index: usize) -> Option<&T::ULE> {
        self.get_ule_ref(index)
    }
    fn len(&self) -> usize {
        ZeroSlice::len(self)
    }
    fn is_ascending(&self) -> bool {
        self.as_ule_slice()
            .windows(2)
            .all(|w| T::from_unaligned(w[1]).cmp(&T::from_unaligned(w[0])) == Ordering::Greater)
    }

    fn t_cmp_get(t: &T, g: &Self::GetType) -> Ordering {
        t.cmp(&T::from_unaligned(*g))
    }

    fn t_with_ser<R>(g: &Self::GetType, f: impl FnOnce(&T) -> R) -> R {
        f(&T::from_unaligned(*g))
    }
}

impl<'a, T> ZeroVecLike<'a, T> for &'a ZeroSlice<T>
where
    T: AsULE + Ord + Copy,
{
    type GetType = T::ULE;
    fn zvl_new() -> Self {
        ZeroSlice::from_ule_slice(&[])
    }
    fn zvl_binary_search(&self, k: &T) -> Result<usize, usize> {
        ZeroSlice::binary_search(*self, k)
    }
    fn zvl_get(&self, index: usize) -> Option<&T::ULE> {
        self.get_ule_ref(index)
    }
    fn len(&self) -> usize {
        ZeroSlice::len(*self)
    }
    fn is_ascending(&self) -> bool {
        self.as_ule_slice()
            .windows(2)
            .all(|w| T::from_unaligned(w[1]).cmp(&T::from_unaligned(w[0])) == Ordering::Greater)
    }

    fn t_cmp_get(t: &T, g: &Self::GetType) -> Ordering {
        t.cmp(&T::from_unaligned(*g))
    }

    fn t_with_ser<R>(g: &Self::GetType, f: impl FnOnce(&T) -> R) -> R {
        f(&T::from_unaligned(*g))
    }
}

impl<'a, T> BorrowedZeroVecLike<'a, T> for &'a ZeroSlice<T>
where
    T: AsULE + Ord + Copy,
{
    fn zvl_get_borrowed(&self, index: usize) -> Option<&'a T::ULE> {
        self.as_ule_slice().get(index)
    }
}

impl<'a, T> MutableZeroVecLike<'a, T> for ZeroVec<'a, T>
where
    T: AsULE + Ord + Copy + 'static,
{
    type OwnedType = T;
    type BorrowedVariant = &'a ZeroSlice<T>;
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
    fn with_capacity(cap: usize) -> Self {
        ZeroVec::Owned(Vec::with_capacity(cap))
    }
    fn clear(&mut self) {
        self.to_mut().clear()
    }
    fn reserve(&mut self, addl: usize) {
        self.to_mut().reserve(addl)
    }

    fn as_borrowed(&'a self) -> &'a ZeroSlice<T> {
        &*self
    }
    fn as_borrowed_inner(&self) -> Option<&'a ZeroSlice<T>> {
        if let ZeroVec::Borrowed(b) = *self {
            Some(ZeroSlice::from_ule_slice(b))
        } else {
            None
        }
    }
    fn from_borrowed(b: &'a ZeroSlice<T>) -> Self {
        b.as_zerovec()
    }

    fn owned_as_t(o: &Self::OwnedType) -> &T {
        o
    }
}

impl<'a, T> ZeroVecLike<'a, T> for VarZeroVec<'a, T>
where
    T: VarULE,
    T: Ord,
    T: ?Sized,
{
    type GetType = T;
    fn zvl_new() -> Self {
        Self::new()
    }
    fn zvl_binary_search(&self, k: &T) -> Result<usize, usize> {
        self.binary_search(k)
    }
    fn zvl_get(&self, index: usize) -> Option<&T> {
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

    fn t_cmp_get(t: &T, g: &Self::GetType) -> Ordering {
        t.cmp(g)
    }

    #[inline]
    fn t_with_ser<R>(g: &Self::GetType, f: impl FnOnce(&T) -> R) -> R {
        f(g)
    }
}

impl<'a, T> ZeroVecLike<'a, T> for VarZeroVecBorrowed<'a, T>
where
    T: VarULE,
    T: Ord,
    T: ?Sized,
{
    type GetType = T;
    fn zvl_new() -> Self {
        Self::new()
    }
    fn zvl_binary_search(&self, k: &T) -> Result<usize, usize> {
        Self::binary_search(self, k)
    }
    fn zvl_get(&self, index: usize) -> Option<&T> {
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

    fn t_cmp_get(t: &T, g: &Self::GetType) -> Ordering {
        t.cmp(g)
    }

    #[inline]
    fn t_with_ser<R>(g: &Self::GetType, f: impl FnOnce(&T) -> R) -> R {
        f(g)
    }
}

impl<'a, T> BorrowedZeroVecLike<'a, T> for VarZeroVecBorrowed<'a, T>
where
    T: VarULE,
    T: Ord,
    T: ?Sized,
{
    fn zvl_get_borrowed(&self, index: usize) -> Option<&'a T> {
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
    type BorrowedVariant = VarZeroVecBorrowed<'a, T>;
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
    fn as_borrowed_inner(&self) -> Option<VarZeroVecBorrowed<'a, T>> {
        if let VarZeroVec::Borrowed(b) = *self {
            Some(b)
        } else {
            None
        }
    }
    fn from_borrowed(b: VarZeroVecBorrowed<'a, T>) -> Self {
        VarZeroVec::Borrowed(b)
    }

    fn owned_as_t(o: &Self::OwnedType) -> &T {
        o
    }
}
