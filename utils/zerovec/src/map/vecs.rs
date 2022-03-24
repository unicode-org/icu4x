// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::ule::*;
use crate::varzerovec::owned::VarZeroVecOwned;
use crate::{VarZeroSlice, VarZeroVec};
use crate::{ZeroSlice, ZeroVec};
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::mem;
use core::ops::Range;

/// Trait abstracting over [`ZeroVec`] and [`VarZeroVec`], for use in [`ZeroMap`](super::ZeroMap). **You
/// should not be implementing or calling this trait directly.**
///
/// The T type is the type received by [`Self::zvl_binary_search()`], as well as the one used
/// for human-readable serialization.
///
/// Methods are prefixed with `zvl_*` to avoid clashes with methods on the types themselves
pub trait ZeroVecLike<'a, T: ?Sized> {
    /// The type returned by `Self::get()`
    type GetType: ?Sized + 'static;
    /// A fully borrowed version of this
    type BorrowedVariant: ZeroVecLike<'a, T, GetType = Self::GetType>
        + BorrowedZeroVecLike<'a, T>
        + Copy;

    /// Create a new, empty vector
    fn zvl_new() -> Self;
    /// Search for a key in a sorted vector, returns `Ok(index)` if found,
    /// returns `Err(insert_index)` if not found, where `insert_index` is the
    /// index where it should be inserted to maintain sort order.
    fn zvl_binary_search(&self, k: &T) -> Result<usize, usize>;
    /// Search for a key within a certain range in a sorted vector. Returns `None` if the
    /// range is out of bounds, and `Ok` or `Err` in the same way as `zvl_binary_search`.
    /// Indices are returned relative to the start of the range.
    fn zvl_binary_search_in_range(
        &self,
        k: &T,
        range: Range<usize>,
    ) -> Option<Result<usize, usize>>;

    /// Search for a key in a sorted vector by a predicate, returns `Ok(index)` if found,
    /// returns `Err(insert_index)` if not found, where `insert_index` is the
    /// index where it should be inserted to maintain sort order.
    fn zvl_binary_search_by(&self, predicate: impl FnMut(&T) -> Ordering) -> Result<usize, usize>;
    /// Get element at `index`
    fn zvl_get(&self, index: usize) -> Option<&Self::GetType>;
    /// The length of this vector
    fn zvl_len(&self) -> usize;
    /// Check if this vector is in ascending order according to `T`s `Ord` impl
    fn zvl_is_ascending(&self) -> bool;
    /// Check if this vector is empty
    fn zvl_is_empty(&self) -> bool {
        self.zvl_len() == 0
    }

    /// Construct a borrowed variant by borrowing from `&self`.
    ///
    /// This function behaves like `&'b self -> Self::BorrowedVariant<'b>`,
    /// where `'b` is the lifetime of the reference to this object.
    ///
    /// Note: We rely on the compiler recognizing `'a` and `'b` as covariant and
    /// casting `&'b Self<'a>` to `&'b Self<'b>` when this gets called, which works
    /// out for `ZeroVec` and `VarZeroVec` containers just fine.
    fn zvl_as_borrowed(&'a self) -> Self::BorrowedVariant;

    /// Extract the inner borrowed variant if possible. Returns `None` if the data is owned.
    ///
    /// This function behaves like `&'_ self -> Self::BorrowedVariant<'a>`,
    /// where `'a` is the lifetime of this object's borrowed data.
    ///
    /// This function is similar to matching the `Borrowed` variant of `ZeroVec`
    /// or `VarZeroVec`, returning the inner borrowed type.
    fn zvl_as_borrowed_inner(&self) -> Option<Self::BorrowedVariant>;

    /// Construct from the borrowed version of the type
    ///
    /// These are useful to ensure serialization parity between borrowed and owned versions
    fn zvl_from_borrowed(b: Self::BorrowedVariant) -> Self;

    /// Compare this type with a `Self::GetType`. This must produce the same result as
    /// if `g` were converted to `Self`
    fn t_cmp_get(t: &T, g: &Self::GetType) -> Ordering;

    /// Compare two values of `Self::GetType`. This must produce the same result as
    /// if both `a` and `b` were converted to `Self`
    fn get_cmp_get(a: &Self::GetType, b: &Self::GetType) -> Ordering;

    /// Obtain a version of T suitable for serialization
    ///
    /// This uses a callback because it's not possible to return owned-or-borrowed
    /// types without GATs
    fn t_with_ser<R>(g: &Self::GetType, f: impl FnOnce(&T) -> R) -> R;
}

/// Trait abstracting over [`ZeroVec`] and [`VarZeroVec`], for use in [`ZeroMap`](super::ZeroMap). **You
/// should not be implementing or calling this trait directly.**
///
/// This trait augments [`ZeroVecLike`] with methods allowing for taking
/// longer references to the underlying buffer, for borrowed-only vector types.
///
/// Methods are prefixed with `zvl_*` to avoid clashes with methods on the types themselves
pub trait BorrowedZeroVecLike<'a, T: ?Sized>: ZeroVecLike<'a, T> {
    /// Get element at `index`, with a longer lifetime
    fn zvl_get_borrowed(&self, index: usize) -> Option<&'a Self::GetType>;
}

/// Trait abstracting over [`ZeroVec`] and [`VarZeroVec`], for use in [`ZeroMap`](super::ZeroMap). **You
/// should not be implementing or calling this trait directly.**
///
/// This trait augments [`ZeroVecLike`] with methods allowing for mutation of the underlying
/// vector for owned vector types.
///
/// Methods are prefixed with `zvl_*` to avoid clashes with methods on the types themselves
pub trait MutableZeroVecLike<'a, T: ?Sized>: ZeroVecLike<'a, T> {
    /// The type returned by `Self::remove()` and `Self::replace()`
    type OwnedType;

    /// Insert an element at `index`
    fn zvl_insert(&mut self, index: usize, value: &T);
    /// Remove the element at `index` (panicking if nonexistant)
    fn zvl_remove(&mut self, index: usize) -> Self::OwnedType;
    /// Replace the element at `index` with another one, returning the old element
    fn zvl_replace(&mut self, index: usize, value: &T) -> Self::OwnedType;
    /// Push an element to the end of this vector
    fn zvl_push(&mut self, value: &T);
    /// Create a new, empty vector, with given capacity
    fn zvl_with_capacity(cap: usize) -> Self;
    /// Remove all elements from the vector
    fn zvl_clear(&mut self);
    /// Reserve space for `addl` additional elements
    fn zvl_reserve(&mut self, addl: usize);

    /// Convert an owned value to a borrowed T
    fn owned_as_t(o: &Self::OwnedType) -> &T;
}

impl<'a, T> ZeroVecLike<'a, T> for ZeroVec<'a, T>
where
    T: 'a + AsULE + Ord + Copy,
{
    type GetType = T::ULE;
    type BorrowedVariant = &'a ZeroSlice<T>;

    fn zvl_new() -> Self {
        Self::new()
    }
    fn zvl_binary_search(&self, k: &T) -> Result<usize, usize> {
        ZeroSlice::binary_search(self, k)
    }
    fn zvl_binary_search_in_range(
        &self,
        k: &T,
        range: Range<usize>,
    ) -> Option<Result<usize, usize>> {
        let zs: &ZeroSlice<T> = &*self;
        zs.zvl_binary_search_in_range(k, range)
    }
    fn zvl_binary_search_by(
        &self,
        mut predicate: impl FnMut(&T) -> Ordering,
    ) -> Result<usize, usize> {
        ZeroSlice::binary_search_by(self, |probe| predicate(&probe))
    }
    fn zvl_get(&self, index: usize) -> Option<&T::ULE> {
        self.get_ule_ref(index)
    }
    fn zvl_len(&self) -> usize {
        ZeroSlice::len(self)
    }
    fn zvl_is_ascending(&self) -> bool {
        self.as_ule_slice()
            .windows(2)
            .all(|w| T::from_unaligned(w[1]).cmp(&T::from_unaligned(w[0])) == Ordering::Greater)
    }

    fn zvl_as_borrowed(&'a self) -> &'a ZeroSlice<T> {
        &*self
    }
    fn zvl_as_borrowed_inner(&self) -> Option<&'a ZeroSlice<T>> {
        if let ZeroVec::Borrowed(b) = *self {
            Some(ZeroSlice::from_ule_slice(b))
        } else {
            None
        }
    }
    fn zvl_from_borrowed(b: &'a ZeroSlice<T>) -> Self {
        b.as_zerovec()
    }

    fn t_cmp_get(t: &T, g: &Self::GetType) -> Ordering {
        t.cmp(&T::from_unaligned(*g))
    }

    fn get_cmp_get(a: &Self::GetType, b: &Self::GetType) -> Ordering {
        T::from_unaligned(*a).cmp(&T::from_unaligned(*b))
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
    type BorrowedVariant = &'a ZeroSlice<T>;

    fn zvl_new() -> Self {
        ZeroSlice::from_ule_slice(&[])
    }
    fn zvl_binary_search(&self, k: &T) -> Result<usize, usize> {
        ZeroSlice::binary_search(*self, k)
    }
    fn zvl_binary_search_in_range(
        &self,
        k: &T,
        range: Range<usize>,
    ) -> Option<Result<usize, usize>> {
        let subslice = self.get_subslice(range)?;
        Some(ZeroSlice::binary_search(subslice, k))
    }
    fn zvl_binary_search_by(
        &self,
        mut predicate: impl FnMut(&T) -> Ordering,
    ) -> Result<usize, usize> {
        ZeroSlice::binary_search_by(self, |probe| predicate(&probe))
    }
    fn zvl_get(&self, index: usize) -> Option<&T::ULE> {
        self.get_ule_ref(index)
    }
    fn zvl_len(&self) -> usize {
        ZeroSlice::len(*self)
    }
    fn zvl_is_ascending(&self) -> bool {
        self.as_ule_slice()
            .windows(2)
            .all(|w| T::from_unaligned(w[1]).cmp(&T::from_unaligned(w[0])) == Ordering::Greater)
    }

    fn zvl_as_borrowed(&'a self) -> &'a ZeroSlice<T> {
        self
    }
    fn zvl_as_borrowed_inner(&self) -> Option<&'a ZeroSlice<T>> {
        Some(self)
    }
    fn zvl_from_borrowed(b: &'a ZeroSlice<T>) -> Self {
        b
    }

    fn t_cmp_get(t: &T, g: &Self::GetType) -> Ordering {
        t.cmp(&T::from_unaligned(*g))
    }

    fn get_cmp_get(a: &Self::GetType, b: &Self::GetType) -> Ordering {
        T::from_unaligned(*a).cmp(&T::from_unaligned(*b))
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
    fn zvl_insert(&mut self, index: usize, value: &T) {
        self.to_mut().insert(index, value.to_unaligned())
    }
    fn zvl_remove(&mut self, index: usize) -> T {
        T::from_unaligned(self.to_mut().remove(index))
    }
    fn zvl_replace(&mut self, index: usize, value: &T) -> T {
        let vec = self.to_mut();
        T::from_unaligned(mem::replace(&mut vec[index], value.to_unaligned()))
    }
    fn zvl_push(&mut self, value: &T) {
        self.to_mut().push(value.to_unaligned())
    }
    fn zvl_with_capacity(cap: usize) -> Self {
        ZeroVec::Owned(Vec::with_capacity(cap))
    }
    fn zvl_clear(&mut self) {
        self.to_mut().clear()
    }
    fn zvl_reserve(&mut self, addl: usize) {
        self.to_mut().reserve(addl)
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
    type BorrowedVariant = &'a VarZeroSlice<T>;

    fn zvl_new() -> Self {
        Self::new()
    }
    fn zvl_binary_search(&self, k: &T) -> Result<usize, usize> {
        self.binary_search(k)
    }
    fn zvl_binary_search_in_range(
        &self,
        k: &T,
        range: Range<usize>,
    ) -> Option<Result<usize, usize>> {
        self.binary_search_in_range(k, range)
    }
    fn zvl_binary_search_by(&self, predicate: impl FnMut(&T) -> Ordering) -> Result<usize, usize> {
        self.binary_search_by(predicate)
    }
    fn zvl_get(&self, index: usize) -> Option<&T> {
        self.get(index)
    }
    fn zvl_len(&self) -> usize {
        self.len()
    }
    fn zvl_is_ascending(&self) -> bool {
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

    fn zvl_as_borrowed(&'a self) -> &'a VarZeroSlice<T> {
        self.as_slice()
    }
    fn zvl_as_borrowed_inner(&self) -> Option<&'a VarZeroSlice<T>> {
        if let VarZeroVec::Borrowed(b) = *self {
            Some(b)
        } else {
            None
        }
    }
    fn zvl_from_borrowed(b: &'a VarZeroSlice<T>) -> Self {
        b.as_varzerovec()
    }

    fn t_cmp_get(t: &T, g: &Self::GetType) -> Ordering {
        t.cmp(g)
    }

    fn get_cmp_get(a: &Self::GetType, b: &Self::GetType) -> Ordering {
        a.cmp(b)
    }

    #[inline]
    fn t_with_ser<R>(g: &Self::GetType, f: impl FnOnce(&T) -> R) -> R {
        f(g)
    }
}

impl<'a, T> ZeroVecLike<'a, T> for &'a VarZeroSlice<T>
where
    T: VarULE,
    T: Ord,
    T: ?Sized,
{
    type GetType = T;
    type BorrowedVariant = &'a VarZeroSlice<T>;

    fn zvl_new() -> Self {
        VarZeroSlice::new_empty()
    }
    fn zvl_binary_search(&self, k: &T) -> Result<usize, usize> {
        self.binary_search(k)
    }
    fn zvl_binary_search_in_range(
        &self,
        k: &T,
        range: Range<usize>,
    ) -> Option<Result<usize, usize>> {
        self.binary_search_in_range(k, range)
    }
    fn zvl_binary_search_by(&self, predicate: impl FnMut(&T) -> Ordering) -> Result<usize, usize> {
        self.binary_search_by(predicate)
    }
    fn zvl_get(&self, index: usize) -> Option<&T> {
        self.get(index)
    }
    fn zvl_len(&self) -> usize {
        self.len()
    }
    fn zvl_is_ascending(&self) -> bool {
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

    fn zvl_as_borrowed(&'a self) -> &'a VarZeroSlice<T> {
        self
    }
    fn zvl_as_borrowed_inner(&self) -> Option<&'a VarZeroSlice<T>> {
        Some(self)
    }
    fn zvl_from_borrowed(b: &'a VarZeroSlice<T>) -> Self {
        b
    }

    fn t_cmp_get(t: &T, g: &Self::GetType) -> Ordering {
        t.cmp(g)
    }

    fn get_cmp_get(a: &Self::GetType, b: &Self::GetType) -> Ordering {
        a.cmp(b)
    }

    #[inline]
    fn t_with_ser<R>(g: &Self::GetType, f: impl FnOnce(&T) -> R) -> R {
        f(g)
    }
}

impl<'a, T> BorrowedZeroVecLike<'a, T> for &'a VarZeroSlice<T>
where
    T: VarULE,
    T: Ord,
    T: ?Sized,
{
    fn zvl_get_borrowed(&self, index: usize) -> Option<&'a T> {
        self.get(index)
    }
}

impl<'a, T> MutableZeroVecLike<'a, T> for VarZeroVec<'a, T>
where
    T: VarULE,
    T: Ord,
    T: ?Sized,
{
    type OwnedType = Box<T>;
    fn zvl_insert(&mut self, index: usize, value: &T) {
        self.make_mut().insert(index, value)
    }
    fn zvl_remove(&mut self, index: usize) -> Box<T> {
        let vec = self.make_mut();
        let old = vec.get(index).expect("invalid index").to_boxed();
        vec.remove(index);
        old
    }
    fn zvl_replace(&mut self, index: usize, value: &T) -> Box<T> {
        let vec = self.make_mut();
        let old = vec.get(index).expect("invalid index").to_boxed();
        vec.replace(index, value);
        old
    }
    fn zvl_push(&mut self, value: &T) {
        let len = self.len();
        self.make_mut().insert(len, value)
    }
    fn zvl_with_capacity(cap: usize) -> Self {
        VarZeroVecOwned::with_capacity(cap).into()
    }
    fn zvl_clear(&mut self) {
        self.make_mut().clear()
    }
    fn zvl_reserve(&mut self, addl: usize) {
        self.make_mut().reserve(addl)
    }

    fn owned_as_t(o: &Self::OwnedType) -> &T {
        o
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_zerovec_binary_search_in_range() {
        let zv: ZeroVec<u16> = ZeroVec::from_slice_or_alloc(&[11, 22, 33, 44, 55, 66, 77]);

        // Full range search
        assert_eq!(zv.zvl_binary_search_in_range(&11, 0..7), Some(Ok(0)));
        assert_eq!(zv.zvl_binary_search_in_range(&12, 0..7), Some(Err(1)));
        assert_eq!(zv.zvl_binary_search_in_range(&44, 0..7), Some(Ok(3)));
        assert_eq!(zv.zvl_binary_search_in_range(&45, 0..7), Some(Err(4)));
        assert_eq!(zv.zvl_binary_search_in_range(&77, 0..7), Some(Ok(6)));
        assert_eq!(zv.zvl_binary_search_in_range(&78, 0..7), Some(Err(7)));

        // Out-of-range search
        assert_eq!(zv.zvl_binary_search_in_range(&44, 0..2), Some(Err(2)));
        assert_eq!(zv.zvl_binary_search_in_range(&44, 5..7), Some(Err(0)));

        // Offset search
        assert_eq!(zv.zvl_binary_search_in_range(&44, 2..5), Some(Ok(1)));
        assert_eq!(zv.zvl_binary_search_in_range(&45, 2..5), Some(Err(2)));

        // Out-of-bounds
        assert_eq!(zv.zvl_binary_search_in_range(&44, 0..100), None);
        assert_eq!(zv.zvl_binary_search_in_range(&44, 100..200), None);
    }
}
