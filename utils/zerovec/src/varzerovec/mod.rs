// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::ule::*;
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::fmt;
use core::ops::Index;

pub(crate) mod borrowed;
pub(crate) mod owned;
#[cfg(feature = "serde")]
mod serde;
mod ule;

pub use borrowed::VarZeroVecBorrowed;
pub use owned::VarZeroVecOwned;
pub use ule::VarZeroVecULE;

/// A zero-copy vector for variable-width types.
///
/// `VarZeroVec<T>` is designed as a drop-in replacement for `Vec<T>` in situations where it is
/// desirable to borrow data from an unaligned byte slice, such as zero-copy deserialization, and
/// where `T`'s data is variable-length (e.g. `String`)
///
/// `T` must implement [`VarULE`], which is already implemented for [`str`] and `[u8]`. For storing more
/// complicated series of elements, it is implemented on `ZeroVecULE<T>` as well as `VarZeroVecULE<T>`
/// for nesting.
///
/// For example, here are some owned types and their zero-copy equivalents:
///
/// - `Vec<String>`: `VarZeroVec<'a, str>`
/// - `Vec<Vec<u8>>>`: `VarZeroVec<'a, [u8]>`
/// - `Vec<Vec<u32>>`: `VarZeroVec<'a, ZeroVecULE<u32>>`
/// - `Vec<Vec<String>>`: `VarZeroVec<'a, VarZeroVecULE<str>>`
///
/// For creating zero-copy vectors of fixed-size types, see [`ZeroVec`](crate::ZeroVec).
///
/// `VarZeroVec<T>` behaves much like [`Cow`](alloc::borrow::Cow), where it can be constructed from
/// owned data (and then mutated!) but can also borrow from some buffer.
///
/// # How it Works
///
/// `VarZeroVec<T>`, when used with non-human-readable serializers (like `bincode`), will
/// serialize to a specially formatted list of bytes. The format is:
///
/// - 4 bytes for `length` (interpreted as a little-endian u32)
/// - `4 * length` bytes of `indices` (interpreted as little-endian u32)
/// - Remaining bytes for actual `data`
///
/// Each element in the `indices` array points to the starting index of its corresponding
/// data part in the `data` list. The ending index can be calculated from the starting index
/// of the next element (or the length of the slice if dealing with the last element).
///
/// # Safety
///
/// `VarZeroVec<T>` is implemented with a fair amount of unsafe code, but is externally
/// safe to use.
///
/// # Example
///
/// ```rust
/// # use std::str::Utf8Error;
/// # use zerovec::ule::ULEError;
/// use zerovec::VarZeroVec;
///
/// // The little-endian bytes correspond to the list of strings.
/// let strings = vec!["w".to_owned(), "ω".to_owned(), "文".to_owned(), "𑄃".to_owned()];
/// let bytes = &[
///     4, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0,  3, 0, 0, 0,
///     6, 0, 0, 0, 119, 207, 137, 230, 150, 135, 240, 145, 132, 131,
/// ];
///
/// let zerovec: VarZeroVec<str> = VarZeroVec::parse_byte_slice(bytes)?;
///
/// assert_eq!(zerovec.get(2), Some("文"));
/// assert_eq!(zerovec, &*strings);
/// # Ok::<(), ULEError>(())
/// ```
///
/// Here's another example with `ZeroVecULE<T>` (similar to `[T]`):
///
/// ```rust
/// # use std::str::Utf8Error;
/// # use zerovec::ule::ULEError;
/// use zerovec::VarZeroVec;
/// use zerovec::ZeroVec;
/// use zerovec::zerovec::ZeroVecULE;
/// use zerovec::ule::*;
///
/// // The structured list correspond to the list of integers.
/// let numbers: Vec<Vec<u32>> = vec![
///     vec![12, 25, 38],
///     vec![39179, 100],
///     vec![42, 55555],
///     vec![12345, 54321, 9],
/// ];
/// let bytes = &[4, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 20, 0, 0, 0, 28, 0, 0, 0,
///              12, 0, 0, 0, 25, 0, 0, 0, 38, 0, 0, 0, 11, 153, 0, 0, 100, 0,
///              0, 0, 42, 0, 0, 0, 3, 217, 0, 0, 57, 48, 0, 0, 49, 212, 0, 0,
///              9, 0, 0, 0];
///
/// let zerovec: VarZeroVec<ZeroVecULE<u32>> = VarZeroVec::parse_byte_slice(bytes)?;
///
/// assert_eq!(zerovec.get(2).and_then(|v| v.get(1)), Some(55555));
/// for (zv, v) in zerovec.iter().zip(numbers.iter()) {
///     assert_eq!(zv, &**v);   
/// }
/// # Ok::<(), ULEError>(())
/// ```
///
///
/// [`VarZeroVec`]s can be nested infinitely, see the docs of [`VarZeroVecULE`]
/// for more information.
///
/// [`ule`]: crate::ule
pub enum VarZeroVec<'a, T: ?Sized> {
    /// An allocated VarZeroVec, allowing for mutations.
    ///
    /// # Examples
    ///
    /// ```
    /// use zerovec::VarZeroVec;
    ///
    /// let mut vzv = VarZeroVec::<str>::default();
    /// vzv.make_mut().push("foo");
    /// vzv.make_mut().push("bar");
    /// assert!(matches!(vzv, VarZeroVec::Owned(_)));
    /// ```
    Owned(VarZeroVecOwned<T>),
    /// A borrowed VarZeroVec, requiring no allocations.
    ///
    /// If a mutating operation is invoked on VarZeroVec, the Borrowed is converted to Owned.
    ///
    /// # Examples
    ///
    /// ```
    /// use zerovec::VarZeroVec;
    ///
    /// let bytes = &[
    ///     4, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0,  3, 0, 0, 0,
    ///     6, 0, 0, 0, 119, 207, 137, 230, 150, 135, 240, 145, 132, 131,
    /// ];
    ///
    /// let vzv: VarZeroVec<str> = VarZeroVec::parse_byte_slice(bytes).unwrap();
    /// assert!(matches!(vzv, VarZeroVec::Borrowed(_)));
    /// ```
    Borrowed(VarZeroVecBorrowed<'a, T>),
}

impl<'a, T: ?Sized> Clone for VarZeroVec<'a, T> {
    fn clone(&self) -> Self {
        match *self {
            VarZeroVec::Owned(ref o) => o.clone().into(),
            VarZeroVec::Borrowed(b) => b.into(),
        }
    }
}

impl<T: VarULE + ?Sized> fmt::Debug for VarZeroVec<'_, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<'a, T: ?Sized> From<VarZeroVecOwned<T>> for VarZeroVec<'a, T> {
    #[inline]
    fn from(other: VarZeroVecOwned<T>) -> Self {
        VarZeroVec::Owned(other)
    }
}

impl<'a, T: ?Sized> From<VarZeroVecBorrowed<'a, T>> for VarZeroVec<'a, T> {
    fn from(other: VarZeroVecBorrowed<'a, T>) -> Self {
        VarZeroVec::Borrowed(other)
    }
}

impl<'a, T: ?Sized + VarULE> From<VarZeroVec<'a, T>> for VarZeroVecOwned<T> {
    #[inline]
    fn from(other: VarZeroVec<'a, T>) -> Self {
        match other {
            VarZeroVec::Owned(o) => o,
            VarZeroVec::Borrowed(b) => b.into(),
        }
    }
}

impl<T: VarULE + ?Sized> Default for VarZeroVec<'_, T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T: VarULE + ?Sized> VarZeroVec<'a, T> {
    /// Creates a new, empty `VarZeroVec<T>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use zerovec::VarZeroVec;
    ///
    /// let vzv: VarZeroVec<str> = VarZeroVec::new();
    /// assert!(vzv.is_empty());
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self::Borrowed(VarZeroVecBorrowed::default())
    }

    /// Obtain a [`VarZeroVecBorrowed`] borrowing from the internal buffer
    pub fn as_borrowed<'b>(&'b self) -> VarZeroVecBorrowed<'b, T> {
        match self {
            VarZeroVec::Owned(ref owned) => owned.as_borrowed(),
            VarZeroVec::Borrowed(ref borrowed) => *borrowed,
        }
    }

    /// Get the number of elements in this vector
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::str::Utf8Error;
    /// # use zerovec::ule::ULEError;
    /// # use zerovec::VarZeroVec;
    ///
    /// let strings = vec!["foo".to_owned(), "bar".to_owned(),
    ///                    "baz".to_owned(), "quux".to_owned()];
    /// let bytes = VarZeroVec::<str>::get_serializable_bytes(&strings).unwrap();
    ///
    /// let mut vec: VarZeroVec<str> = VarZeroVec::parse_byte_slice(&bytes)?;
    /// assert_eq!(vec.len(), 4);
    /// # Ok::<(), ULEError>(())
    /// ```
    pub fn len(&self) -> usize {
        self.as_borrowed().len()
    }

    /// Returns `true` if the vector contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::str::Utf8Error;
    /// # use zerovec::ule::ULEError;
    /// # use zerovec::VarZeroVec;
    ///
    /// let strings: Vec<String> = vec![];
    /// let bytes = VarZeroVec::<str>::get_serializable_bytes(&strings).unwrap();
    ///
    /// let mut vec: VarZeroVec<str> = VarZeroVec::parse_byte_slice(&bytes)?;
    /// assert!(vec.is_empty());
    /// # Ok::<(), ULEError>(())
    /// ```
    pub fn is_empty(&self) -> bool {
        self.as_borrowed().is_empty()
    }

    /// Parse a VarZeroVec from a slice of the appropriate format
    ///
    /// Slices of the right format can be obtained via [`VarZeroVec::<str>::get_serializable_bytes()`]
    /// or [`VarZeroVec::get_encoded_slice()`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::str::Utf8Error;
    /// # use zerovec::ule::ULEError;
    /// # use zerovec::VarZeroVec;
    ///
    /// let strings = vec!["foo".to_owned(), "bar".to_owned(),
    ///                    "baz".to_owned(), "quux".to_owned()];
    /// let bytes = VarZeroVec::<str>::get_serializable_bytes(&strings).unwrap();
    ///
    /// let mut vec: VarZeroVec<str> = VarZeroVec::parse_byte_slice(&bytes)?;
    /// assert_eq!(&vec[0], "foo");
    /// assert_eq!(&vec[1], "bar");
    /// assert_eq!(&vec[2], "baz");
    /// assert_eq!(&vec[3], "quux");
    /// # Ok::<(), ULEError>(())
    /// ```
    pub fn parse_byte_slice(slice: &'a [u8]) -> Result<Self, ULEError> {
        if slice.is_empty() {
            // does not allocate
            return Ok(VarZeroVec::Owned(VarZeroVecOwned::new()));
        }

        let borrowed = VarZeroVecBorrowed::<T>::parse_byte_slice(slice)?;

        Ok(VarZeroVec::Borrowed(borrowed))
    }

    /// Obtain an iterator over VarZeroVec's elements
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::str::Utf8Error;
    /// # use zerovec::ule::ULEError;
    /// # use zerovec::VarZeroVec;
    ///
    /// let strings = vec!["foo".to_owned(), "bar".to_owned(),
    ///                    "baz".to_owned(), "quux".to_owned()];
    /// let bytes = VarZeroVec::<str>::get_serializable_bytes(&strings).unwrap();
    /// let mut vec: VarZeroVec<str> = VarZeroVec::parse_byte_slice(&bytes)?;
    ///
    /// let mut iter_results: Vec<&str> = vec.iter().collect();
    /// assert_eq!(iter_results[0], "foo");
    /// assert_eq!(iter_results[1], "bar");
    /// assert_eq!(iter_results[2], "baz");
    /// assert_eq!(iter_results[3], "quux");
    /// # Ok::<(), ULEError>(())
    /// ```
    pub fn iter<'b: 'a>(&'b self) -> impl Iterator<Item = &'b T> {
        self.as_borrowed().iter()
    }

    /// Get one of VarZeroVec's elements, returning None if the index is out of bounds
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::str::Utf8Error;
    /// # use zerovec::ule::ULEError;
    /// # use zerovec::VarZeroVec;
    ///
    /// let strings = vec!["foo".to_owned(), "bar".to_owned(),
    ///                    "baz".to_owned(), "quux".to_owned()];
    /// let bytes = VarZeroVec::<str>::get_serializable_bytes(&strings).unwrap();
    /// let mut vec: VarZeroVec<str> = VarZeroVec::parse_byte_slice(&bytes)?;
    ///
    /// let mut iter_results: Vec<&str> = vec.iter().collect();
    /// assert_eq!(vec.get(0), Some("foo"));
    /// assert_eq!(vec.get(1), Some("bar"));
    /// assert_eq!(vec.get(2), Some("baz"));
    /// assert_eq!(vec.get(3), Some("quux"));
    /// assert_eq!(vec.get(4), None);
    /// # Ok::<(), ULEError>(())
    /// ```
    pub fn get(&self, idx: usize) -> Option<&T> {
        self.as_borrowed().get(idx)
    }

    /// Convert this into a mutable vector of the owned `T` type, cloning if necessary.
    ///
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// # use std::str::Utf8Error;
    /// # use zerovec::ule::ULEError;
    /// # use zerovec::VarZeroVec;
    ///
    /// let strings = vec!["foo".to_owned(), "bar".to_owned(),
    ///                    "baz".to_owned(), "quux".to_owned()];
    /// let bytes = VarZeroVec::<str>::get_serializable_bytes(&strings).unwrap();
    /// let mut vec: VarZeroVec<str> = VarZeroVec::parse_byte_slice(&bytes)?;
    ///
    /// assert_eq!(vec.len(), 4);
    /// let mutvec = vec.make_mut();
    /// mutvec.push("lorem ipsum".into());
    /// mutvec[2] = "dolor sit".into();
    /// assert_eq!(&vec[0], "foo");
    /// assert_eq!(&vec[1], "bar");
    /// assert_eq!(&vec[2], "dolor sit");
    /// assert_eq!(&vec[3], "quux");
    /// assert_eq!(&vec[4], "lorem ipsum");
    /// # Ok::<(), ULEError>(())
    /// ```
    //
    // This function is crate-public for now since we don't yet want to stabilize
    // the internal implementation details
    pub fn make_mut(&mut self) -> &mut VarZeroVecOwned<T> {
        match self {
            VarZeroVec::Owned(ref mut vec) => vec,
            VarZeroVec::Borrowed(ref borrowed) => {
                let new_self = VarZeroVecOwned::from_borrowed(*borrowed);
                *self = new_self.into();
                // recursion is limited since we are guaranteed to hit the Owned branch
                self.make_mut()
            }
        }
    }

    /// Converts a borrowed ZeroVec to an owned ZeroVec. No-op if already owned.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::str::Utf8Error;
    /// # use zerovec::ule::ULEError;
    /// # use zerovec::VarZeroVec;
    ///
    /// let strings = vec!["foo".to_owned(), "bar".to_owned(),
    ///                    "baz".to_owned(), "quux".to_owned()];
    /// let bytes = VarZeroVec::<str>::get_serializable_bytes(&strings).unwrap();
    /// let mut vec: VarZeroVec<str> = VarZeroVec::parse_byte_slice(&bytes)?;
    ///
    /// assert_eq!(vec.len(), 4);
    /// // has 'static lifetime
    /// let owned = vec.into_owned();
    /// # Ok::<(), ULEError>(())
    /// ```
    pub fn into_owned(mut self) -> VarZeroVec<'static, T> {
        self.make_mut();
        match self {
            VarZeroVec::Owned(vec) => vec.into(),
            _ => unreachable!(),
        }
    }

    /// Obtain this `VarZeroVec` as a [`VarZeroVecULE`]
    pub fn as_ule(&self) -> &VarZeroVecULE<T> {
        let slice = self.get_encoded_slice();
        unsafe {
            // safety: the slice is known to come from a valid parsed VZV
            VarZeroVecULE::from_byte_slice_unchecked(slice)
        }
    }

    /// Obtain an owned `Vec<Box<T>>` out of this
    pub fn to_vec(&self) -> Vec<Box<T>> {
        self.as_borrowed().to_vec()
    }

    /// Obtain the internal encoded slice
    ///
    /// This can be passed back to [`Self::parse_byte_slice()`]
    pub fn get_encoded_slice(&self) -> &[u8] {
        match self {
            VarZeroVec::Owned(ref vec) => vec.entire_slice(),
            VarZeroVec::Borrowed(vec) => vec.entire_slice(),
        }
    }

    /// For a slice of `T`, get a list of bytes that can be passed to
    /// `parse_byte_slice` to recoup the same data.
    ///
    /// Returns `None` if the slice is too large to be represented in a list of
    /// bytes whose length fits in a `u32`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::str::Utf8Error;
    /// # use zerovec::ule::ULEError;
    /// # use zerovec::VarZeroVec;
    ///
    /// let strings = vec!["foo".to_owned(), "bar".to_owned(), "baz".to_owned()];
    /// let bytes = VarZeroVec::<str>::get_serializable_bytes(&strings).unwrap();
    ///
    /// let mut borrowed: VarZeroVec<str> = VarZeroVec::parse_byte_slice(&bytes)?;
    /// assert_eq!(borrowed, &*strings);
    ///
    /// # Ok::<(), ULEError>(())
    /// ```
    ///
    pub fn get_serializable_bytes<A: custom::EncodeAsVarULE<T>>(elements: &[A]) -> Option<Vec<u8>> {
        borrowed::get_serializable_bytes(elements)
    }

    /// Return whether the [`VarZeroVec`] is operating on owned or borrowed
    /// data. [`VarZeroVec::into_owned()`] and [`VarZeroVec::make_mut()`] can
    /// be used to force it into an owned type
    pub fn is_owned(&self) -> bool {
        match self {
            VarZeroVec::Owned(..) => true,
            VarZeroVec::Borrowed(..) => false,
        }
    }
}

impl<'a, T> VarZeroVec<'a, T>
where
    T: VarULE,
    T: ?Sized,
    T: Ord,
{
    /// Binary searches a sorted `VarZeroVec<T>` for the given element. For more information, see
    /// the primitive function [`binary_search`].
    ///
    /// # Example
    ///
    /// ```
    /// # use std::str::Utf8Error;
    /// # use zerovec::ule::ULEError;
    /// # use zerovec::VarZeroVec;
    ///
    /// let strings = vec!["a".to_owned(), "b".to_owned(),
    ///                    "f".to_owned(), "g".to_owned()];
    /// let bytes = VarZeroVec::<str>::get_serializable_bytes(&strings).unwrap();
    /// let mut vec: VarZeroVec<str> = VarZeroVec::parse_byte_slice(&bytes)?;
    ///
    /// assert_eq!(vec.binary_search("f"), Ok(2));
    /// assert_eq!(vec.binary_search("e"), Err(2));
    /// # Ok::<(), ULEError>(())
    /// ```
    ///
    /// [`binary_search`]: https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search
    #[inline]
    pub fn binary_search(&self, x: &T) -> Result<usize, usize> {
        self.as_borrowed().binary_search(x)
    }
}

impl<'a, T: VarULE + ?Sized> Index<usize> for VarZeroVec<'a, T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("Indexing VarZeroVec out of bounds")
    }
}

impl<'a, A, T> From<&'_ [A]> for VarZeroVec<'a, T>
where
    T: VarULE,
    T: ?Sized,
    A: custom::EncodeAsVarULE<T>,
{
    fn from(other: &'_ [A]) -> Self {
        VarZeroVecOwned::from_elements(other).into()
    }
}

impl<'a, 'b, T> PartialEq<VarZeroVec<'b, T>> for VarZeroVec<'a, T>
where
    T: VarULE,
    T: ?Sized,
    T: PartialEq,
{
    #[inline]
    fn eq(&self, other: &VarZeroVec<'b, T>) -> bool {
        // VarULE has an API guarantee that this is equivalent
        // to `T::VarULE::eq()`
        self.get_encoded_slice().eq(other.get_encoded_slice())
    }
}

impl<'a, T> Eq for VarZeroVec<'a, T>
where
    T: VarULE,
    T: ?Sized,
    T: Eq,
{
}

impl<T, A> PartialEq<&'_ [A]> for VarZeroVec<'_, T>
where
    T: VarULE + ?Sized,
    T: PartialEq,
    A: AsRef<T>,
{
    #[inline]
    fn eq(&self, other: &&[A]) -> bool {
        self.iter().eq(other.iter().map(|t| t.as_ref()))
    }
}
