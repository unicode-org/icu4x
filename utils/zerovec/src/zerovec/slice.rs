// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;
use alloc::boxed::Box;

/// A zero-copy "slice", i.e. the zero-copy version of `[T]`. This behaves
/// similarly to [`ZeroVec<T>`], however [`ZeroVec<T>`] is allowed to contain
/// owned data and as such is ideal for deserialization since most human readable
/// serialization formats cannot unconditionally deserialize zero-copy.
///
/// This type can be used inside `VarZeroVec<T>`: This essentially allows for
/// the construction of zero-copy types isomorphic to `Vec<Vec<T>>` by instead
/// using `VarZeroVec<ZeroSlice<T>>`. See the [`VarZeroVec`](crate::VarZeroVec) docs for an example.
#[repr(transparent)]
pub struct ZeroSlice<T: AsULE>(pub [T::ULE]);

impl<T> ZeroSlice<T>
where
    T: AsULE,
{
    /// Get this [`ZeroSlice`] as a borrowed [`ZeroVec`]
    ///
    /// [`ZeroSlice`] does not have most of the methods that [`ZeroVec`] does,
    /// so it is recommended to convert it to a [`ZeroVec`] before doing anything.
    #[inline]
    pub fn as_zerovec(&self) -> ZeroVec<'_, T> {
        ZeroVec::Borrowed(&self.0)
    }

    /// Construct a `&ZeroSlice<T>` from a slice of ULEs
    #[inline]
    pub fn from_slice(slice: &[T::ULE]) -> &Self {
        // This is safe because ZeroSlice is transparent over [T::ULE]
        // so &ZeroSlice<T> can be safely cast from &[T::ULE]
        unsafe { &*(slice as *const _ as *const Self) }
    }

    /// Construct a `Box<ZeroSlice<T>>` from a boxed slice of ULEs
    #[inline]
    pub fn from_boxed_slice(slice: Box<[T::ULE]>) -> Box<Self> {
        // This is safe because ZeroSlice is transparent over [T::ULE]
        // so Box<ZeroSlice<T>> can be safely cast from Box<[T::ULE]>
        unsafe { Box::from_raw(Box::into_raw(slice) as *mut Self) }
    }
}

impl<T> ZeroSlice<T>
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
// (`ZeroSlice<T>` is a transparent wrapper around [T::ULE])
//  1. [T::ULE] does not include any uninitialized or padding bytes (achieved by being a slice of a ULE type)
//  2. [T::ULE] is aligned to 1 byte (achieved by being a slice of a ULE type)
//  3. The impl of `validate_byte_slice()` returns an error if any byte is not valid.
//  4. The impl of `validate_byte_slice()` returns an error if the slice cannot be used in its entirety
//  5. The impl of `from_byte_slice_unchecked()` returns a reference to the same data.
//  6. `as_byte_slice()` and `parse_byte_slice()` are defaulted
//  7. `[T::ULE]` byte equality is semantic equality (relying on the guideline of the underlying `ULE` type)
unsafe impl<T: AsULE + 'static> VarULE for ZeroSlice<T> {
    #[inline]
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), ZeroVecError> {
        T::ULE::validate_byte_slice(bytes)
    }

    #[inline]
    unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &Self {
        Self::from_slice(T::ULE::from_byte_slice_unchecked(bytes))
    }
}

impl<T> Eq for ZeroSlice<T> where T: AsULE + Eq {}

impl<T> PartialEq<ZeroSlice<T>> for ZeroSlice<T>
where
    T: AsULE + PartialEq,
{
    #[inline]
    fn eq(&self, other: &ZeroSlice<T>) -> bool {
        self.as_zerovec().eq(&other.as_zerovec())
    }
}

impl<T> PartialEq<[T]> for ZeroSlice<T>
where
    T: AsULE + PartialEq,
{
    #[inline]
    fn eq(&self, other: &[T]) -> bool {
        self.iter().eq(other.iter().copied())
    }
}

impl<'a, T> PartialEq<ZeroVec<'a, T>> for ZeroSlice<T>
where
    T: AsULE + PartialEq,
{
    #[inline]
    fn eq(&self, other: &ZeroVec<'a, T>) -> bool {
        self.as_zerovec().eq(other)
    }
}

impl<'a, T> PartialEq<ZeroSlice<T>> for ZeroVec<'a, T>
where
    T: AsULE + PartialEq,
{
    #[inline]
    fn eq(&self, other: &ZeroSlice<T>) -> bool {
        self.eq(&other.as_zerovec())
    }
}

impl<T> fmt::Debug for ZeroSlice<T>
where
    T: AsULE + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_zerovec().fmt(f)
    }
}

impl<'a, T: AsULE + PartialOrd> PartialOrd for ZeroSlice<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.iter().partial_cmp(other.iter())
    }
}

impl<T: AsULE + Ord> Ord for ZeroSlice<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.iter().cmp(other.iter())
    }
}

impl<T: AsULE> AsRef<ZeroSlice<T>> for Vec<T::ULE> {
    fn as_ref(&self) -> &ZeroSlice<T> {
        ZeroSlice::<T>::from_slice(&**self)
    }
}
