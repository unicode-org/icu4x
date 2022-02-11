// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;
use alloc::boxed::Box;
use core::cmp::Ordering;
use core::ops::Range;

/// A zero-copy "slice", i.e. the zero-copy version of `[T]`. This behaves
/// similarly to [`ZeroVec<T>`], however [`ZeroVec<T>`] is allowed to contain
/// owned data and as such is ideal for deserialization since most human readable
/// serialization formats cannot unconditionally deserialize zero-copy.
///
/// This type can be used inside `VarZeroVec<T>`: This essentially allows for
/// the construction of zero-copy types isomorphic to `Vec<Vec<T>>` by instead
/// using `VarZeroVec<ZeroSlice<T>>`. See the [`VarZeroVec`](crate::VarZeroVec) docs for an example.
#[repr(transparent)]
pub struct ZeroSlice<T: AsULE>([T::ULE]);

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
    pub fn from_ule_slice(slice: &[T::ULE]) -> &Self {
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

    /// Returns this slice as its underlying `&[u8]` byte buffer representation.
    ///
    /// Useful for serialization.
    ///
    /// # Example
    ///
    /// ```
    /// use zerovec::ZeroVec;
    ///
    /// // The little-endian bytes correspond to the numbers on the following line.
    /// let bytes: &[u8] = &[0xD3, 0x00, 0x19, 0x01, 0xA5, 0x01, 0xCD, 0x01];
    /// let nums: &[u16] = &[211, 281, 421, 461];
    ///
    /// let zerovec = ZeroVec::alloc_from_slice(nums);
    ///
    /// assert_eq!(bytes, zerovec.as_bytes());
    /// ```
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        T::ULE::as_byte_slice(self.as_ule_slice())
    }

    /// Dereferences this slice as `&[T::ULE]`.
    #[inline]
    pub fn as_ule_slice(&self) -> &[T::ULE] {
        &self.0
    }

    /// Returns the number of elements in this slice.
    ///
    /// # Example
    ///
    /// ```
    /// use zerovec::ZeroVec;
    /// use zerovec::ule::AsULE;
    ///
    /// let bytes: &[u8] = &[0xD3, 0x00, 0x19, 0x01, 0xA5, 0x01, 0xCD, 0x01];
    /// let zerovec: ZeroVec<u16> = ZeroVec::parse_byte_slice(bytes).expect("infallible");
    ///
    /// assert_eq!(4, zerovec.len());
    /// assert_eq!(
    ///     bytes.len(),
    ///     zerovec.len() * std::mem::size_of::<<u16 as AsULE>::ULE>()
    /// );
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.as_ule_slice().len()
    }

    /// Returns whether this slice is empty.
    ///
    /// # Example
    ///
    /// ```
    /// use zerovec::ZeroVec;
    ///
    /// let bytes: &[u8] = &[0xD3, 0x00, 0x19, 0x01, 0xA5, 0x01, 0xCD, 0x01];
    /// let zerovec: ZeroVec<u16> = ZeroVec::parse_byte_slice(bytes).expect("infallible");
    /// assert!(!zerovec.is_empty());
    ///
    /// let emptyvec: ZeroVec<u16> = ZeroVec::parse_byte_slice(&[]).expect("infallible");
    /// assert!(emptyvec.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.as_ule_slice().is_empty()
    }
}

impl<T> ZeroSlice<T>
where
    T: AsULE,
{
    /// Gets the element at the specified index. Returns None if out of range.
    ///
    /// # Example
    ///
    /// ```
    /// use zerovec::ZeroVec;
    ///
    /// let bytes: &[u8] = &[0xD3, 0x00, 0x19, 0x01, 0xA5, 0x01, 0xCD, 0x01];
    /// let zerovec: ZeroVec<u16> = ZeroVec::parse_byte_slice(bytes).expect("infallible");
    ///
    /// assert_eq!(zerovec.get(2), Some(421));
    /// assert_eq!(zerovec.get(4), None);
    /// ```
    #[inline]
    pub fn get(&self, index: usize) -> Option<T> {
        self.as_ule_slice()
            .get(index)
            .copied()
            .map(T::from_unaligned)
    }

    /// Gets a subslice of elements within a certain range. Returns None if the range
    /// is out of bounds of this `ZeroSlice`.
    ///
    /// # Example
    ///
    /// ```
    /// use zerovec::ZeroVec;
    ///
    /// let bytes: &[u8] = &[0xD3, 0x00, 0x19, 0x01, 0xA5, 0x01, 0xCD, 0x01];
    /// let zerovec: ZeroVec<u16> = ZeroVec::parse_byte_slice(bytes).expect("infallible");
    ///
    /// assert_eq!(
    ///     zerovec.get_subslice(1..3),
    ///     Some(&*ZeroVec::from_slice(&[0x0119, 0x01A5]))
    /// );
    /// assert_eq!(zerovec.get_subslice(3..5), None);
    /// ```
    #[inline]
    pub fn get_subslice(&self, range: Range<usize>) -> Option<&ZeroSlice<T>> {
        self.0.get(range).map(ZeroSlice::from_ule_slice)
    }

    /// Get a borrowed reference to the underlying ULE type at a specified index.
    ///
    /// Prefer [`Self::get()`] over this method where possible since working
    /// directly with `ULE` types is less ergonomic
    pub fn get_ule_ref(&self, index: usize) -> Option<&T::ULE> {
        self.as_ule_slice().get(index)
    }

    /// Gets the first element. Returns None if empty.
    ///
    /// # Example
    ///
    /// ```
    /// use zerovec::ZeroVec;
    ///
    /// let bytes: &[u8] = &[0xD3, 0x00, 0x19, 0x01, 0xA5, 0x01, 0xCD, 0x01];
    /// let zerovec: ZeroVec<u16> = ZeroVec::parse_byte_slice(bytes).expect("infallible");
    ///
    /// assert_eq!(zerovec.first(), Some(211));
    /// ```
    #[inline]
    pub fn first(&self) -> Option<T> {
        self.as_ule_slice().first().copied().map(T::from_unaligned)
    }

    /// Gets the last element. Returns None if empty.
    ///
    /// # Example
    ///
    /// ```
    /// use zerovec::ZeroVec;
    ///
    /// let bytes: &[u8] = &[0xD3, 0x00, 0x19, 0x01, 0xA5, 0x01, 0xCD, 0x01];
    /// let zerovec: ZeroVec<u16> = ZeroVec::parse_byte_slice(bytes).expect("infallible");
    ///
    /// assert_eq!(zerovec.last(), Some(461));
    /// ```
    #[inline]
    pub fn last(&self) -> Option<T> {
        self.as_ule_slice().last().copied().map(T::from_unaligned)
    }

    /// Gets an iterator over the elements.
    ///
    /// # Example
    ///
    /// ```
    /// use zerovec::ZeroVec;
    ///
    /// let bytes: &[u8] = &[0xD3, 0x00, 0x19, 0x01, 0xA5, 0x01, 0xCD, 0x01];
    /// let zerovec: ZeroVec<u16> = ZeroVec::parse_byte_slice(bytes).expect("infallible");
    /// let mut it = zerovec.iter();
    ///
    /// assert_eq!(it.next(), Some(211));
    /// assert_eq!(it.next(), Some(281));
    /// assert_eq!(it.next(), Some(421));
    /// assert_eq!(it.next(), Some(461));
    /// assert_eq!(it.next(), None);
    /// ```
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = T> + '_ {
        self.as_ule_slice().iter().copied().map(T::from_unaligned)
    }
}

impl<T> ZeroSlice<T>
where
    T: AsULE + Ord,
{
    /// Binary searches a sorted `ZeroVec<T>` for the given element. For more information, see
    /// the primitive function [`binary_search`].
    ///
    /// # Example
    ///
    /// ```
    /// use zerovec::ZeroVec;
    ///
    /// let bytes: &[u8] = &[0xD3, 0x00, 0x19, 0x01, 0xA5, 0x01, 0xCD, 0x01];
    /// let zerovec: ZeroVec<u16> = ZeroVec::parse_byte_slice(bytes).expect("infallible");
    ///
    /// assert_eq!(zerovec.binary_search(&281), Ok(1));
    /// assert_eq!(zerovec.binary_search(&282), Err(2));
    /// ```
    ///
    /// [`binary_search`]: https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search
    #[inline]
    pub fn binary_search(&self, x: &T) -> Result<usize, usize> {
        self.as_ule_slice()
            .binary_search_by(|probe| T::from_unaligned(*probe).cmp(x))
    }
}

impl<T> ZeroSlice<T>
where
    T: AsULE,
{
    /// Binary searches a sorted `ZeroVec<T>` based on a given predicate. For more information, see
    /// the primitive function [`binary_search_by`].
    ///
    /// # Example
    ///
    /// ```
    /// use zerovec::ZeroVec;
    ///
    /// let bytes: &[u8] = &[0xD3, 0x00, 0x19, 0x01, 0xA5, 0x01, 0xCD, 0x01];
    /// let zerovec: ZeroVec<u16> = ZeroVec::parse_byte_slice(bytes).expect("infallible");
    ///
    /// assert_eq!(zerovec.binary_search_by(|x| x.cmp(&281)), Ok(1));
    /// assert_eq!(zerovec.binary_search_by(|x| x.cmp(&282)), Err(2));
    /// ```
    ///
    /// [`binary_search_by`]: https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search_by
    #[inline]
    pub fn binary_search_by(
        &self,
        mut predicate: impl FnMut(T) -> Ordering,
    ) -> Result<usize, usize> {
        self.as_ule_slice()
            .binary_search_by(|probe| predicate(T::from_unaligned(*probe)))
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
        Self::from_ule_slice(T::ULE::from_byte_slice_unchecked(bytes))
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
        ZeroSlice::<T>::from_ule_slice(&**self)
    }
}

impl<T: AsULE> AsRef<ZeroSlice<T>> for &[T::ULE] {
    fn as_ref(&self) -> &ZeroSlice<T> {
        ZeroSlice::<T>::from_ule_slice(&**self)
    }
}

impl<T> Default for &ZeroSlice<T>
where
    T: AsULE,
{
    fn default() -> Self {
        ZeroSlice::from_ule_slice(&[])
    }
}
