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

    /// Attempt to construct a `&ZeroSlice<T>` from a byte slice, returning an error
    /// if it's not a valid byte sequence
    pub fn parse_byte_slice(bytes: &[u8]) -> Result<&Self, ZeroVecError> {
        T::ULE::parse_byte_slice(bytes).map(Self::from_ule_slice)
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
    /// let bytes: &[u8] = &[0xD3, 0x00, 0x19, 0x01, 0xA5, 0x01, 0xCD, 0x80];
    /// let nums: &[u16] = &[211, 281, 421, 32973];
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
    /// let bytes: &[u8] = &[0xD3, 0x00, 0x19, 0x01, 0xA5, 0x01, 0xCD, 0x80];
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
    /// let bytes: &[u8] = &[0xD3, 0x00, 0x19, 0x01, 0xA5, 0x01, 0xCD, 0x80];
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
    /// let bytes: &[u8] = &[0xD3, 0x00, 0x19, 0x01, 0xA5, 0x01, 0xCD, 0x80];
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
    /// let bytes: &[u8] = &[0xD3, 0x00, 0x19, 0x01, 0xA5, 0x01, 0xCD, 0x80];
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

    /// Casts a `ZeroSlice<T>` to a compatible `ZeroSlice<P>`.
    ///
    /// `T` and `P` are compatible if they have the same `ULE` representation.
    ///
    /// If the `ULE`s of `T` and `P` are different types but have the same size,
    /// use [`Self::try_as_converted()`].
    ///
    /// # Examples
    ///
    /// ```
    /// use zerovec::ZeroSlice;
    ///
    /// const bytes: &[u8] = &[0xD3, 0x00, 0x19, 0x01, 0xA5, 0x01, 0xCD, 0x80];
    /// const zs_u16: &ZeroSlice<u16> = {
    ///     match ZeroSlice::<u16>::try_from_bytes(bytes) {
    ///         Ok(s) => s,
    ///         Err(_) => unreachable!()
    ///     }
    /// };
    ///
    /// let zs_i16: &ZeroSlice<i16> = zs_u16.cast();
    ///
    /// assert_eq!(zs_u16.get(3), Some(32973));
    /// assert_eq!(zs_i16.get(3), Some(-32563));
    /// ```
    #[inline]
    pub fn cast<P>(&self) -> &ZeroSlice<P>
    where
        P: AsULE<ULE = T::ULE>,
    {
        ZeroSlice::<P>::from_ule_slice(self.as_ule_slice())
    }

    /// Converts a `&ZeroSlice<T>` into a `&ZeroSlice<P>`.
    ///
    /// If `T` and `P` have the exact same `ULE`, use [`Self::cast()`].
    ///
    /// # Panics
    ///
    /// Panics if `T::ULE` and `P::ULE` are not the same size.
    ///
    /// # Examples
    ///
    /// ```
    /// use zerovec::ZeroSlice;
    ///
    /// const bytes: &[u8] = &[0x7F, 0xF3, 0x01, 0x00, 0x49, 0xF6, 0x01, 0x00];
    /// const zs_u32: &ZeroSlice<u32> = {
    ///     match ZeroSlice::<u32>::try_from_bytes(bytes) {
    ///         Ok(s) => s,
    ///         Err(_) => unreachable!()
    ///     }
    /// };
    ///
    /// let zs_char: &ZeroSlice<char> = zs_u32.try_as_converted()
    ///     .expect("valid code points");
    ///
    /// assert_eq!(zs_u32.get(0), Some(u32::from('🍿')));
    /// assert_eq!(zs_char.get(0), Some('🍿'));
    /// ```
    #[inline]
    pub fn try_as_converted<P: AsULE>(&self) -> Result<&ZeroSlice<P>, ZeroVecError> {
        assert_eq!(
            core::mem::size_of::<<T as AsULE>::ULE>(),
            core::mem::size_of::<<P as AsULE>::ULE>()
        );
        let new_slice = P::ULE::parse_byte_slice(self.as_bytes())?;
        Ok(ZeroSlice::from_ule_slice(new_slice))
    }

    /// Gets the first element. Returns None if empty.
    ///
    /// # Example
    ///
    /// ```
    /// use zerovec::ZeroVec;
    ///
    /// let bytes: &[u8] = &[0xD3, 0x00, 0x19, 0x01, 0xA5, 0x01, 0xCD, 0x80];
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
    /// let bytes: &[u8] = &[0xD3, 0x00, 0x19, 0x01, 0xA5, 0x01, 0xCD, 0x80];
    /// let zerovec: ZeroVec<u16> = ZeroVec::parse_byte_slice(bytes).expect("infallible");
    ///
    /// assert_eq!(zerovec.last(), Some(32973));
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
    /// let bytes: &[u8] = &[0xD3, 0x00, 0x19, 0x01, 0xA5, 0x01, 0xCD, 0x80];
    /// let zerovec: ZeroVec<u16> = ZeroVec::parse_byte_slice(bytes).expect("infallible");
    /// let mut it = zerovec.iter();
    ///
    /// assert_eq!(it.next(), Some(211));
    /// assert_eq!(it.next(), Some(281));
    /// assert_eq!(it.next(), Some(421));
    /// assert_eq!(it.next(), Some(32973));
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
    /// let bytes: &[u8] = &[0xD3, 0x00, 0x19, 0x01, 0xA5, 0x01, 0xCD, 0x80];
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
    /// let bytes: &[u8] = &[0xD3, 0x00, 0x19, 0x01, 0xA5, 0x01, 0xCD, 0x80];
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
