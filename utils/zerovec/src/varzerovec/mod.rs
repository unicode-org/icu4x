// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::ule::*;
use components::SliceComponents;
use either::Either;
use std::fmt::{self, Display};
use std::ops::Index;

mod components;
#[cfg(feature = "serde")]
mod serde;

/// A zero-copy vector for variable-width types.
///
/// `VarZeroVec<T>` is designed as a drop-in replacement for `Vec<T>` in situations where it is
/// desirable to borrow data from an unaligned byte slice, such as zero-copy deserialization, and
/// where `T`'s data is variable-length (e.g. `String`)
///
/// `T` must implement [`AsVarULE`], which is already implemented for [`String`]. References are obtained
/// as its [`AsVarULE::VarULE`] type, which in the case of [`String`] is [`str`].
///
/// `VarZeroVec<T>` behaves much like [`std::borrow::Cow`], where it can be constructed from owned data
/// but can also borrow from some buffer.
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
/// ```
/// # use std::str::Utf8Error;
/// # use zerovec::VarZeroVecError;
/// use zerovec::VarZeroVec;
///
/// // The little-endian bytes correspond to the list of strings.
/// let strings = vec!["w".to_owned(), "ω".to_owned(), "文".to_owned(), "𑄃".to_owned()];
/// let bytes = &[
///     4, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0,  3, 0, 0, 0,
///     6, 0, 0, 0, 119, 207, 137, 230, 150, 135, 240, 145, 132, 131,
/// ];
///
/// let zerovec: VarZeroVec<String> = VarZeroVec::try_from_bytes(bytes)?;
///
/// assert_eq!(zerovec.get(2), Some("文"));
/// assert_eq!(zerovec, &*strings);
/// # Ok::<(), VarZeroVecError<Utf8Error>>(())
/// ```
///
/// [`ule`]: crate::ule
#[derive(Clone)]
pub struct VarZeroVec<'a, T>(VarZeroVecInner<'a, T>);

/// Implementation details of VarZeroVec
///
/// Should not be made public.
///
/// Safety invariant: Borrowed() must have the appropriate
/// format:
///
/// The format of the slice is conceptually:
/// ```rust,ignore
/// struct SliceFormat {
///     len: u32::ULE,
///     lens: [u32::ULE],
///     things: [u8]
/// }
/// ```
///
/// `len` is the length of `lens`, and each element of `lens` is the starting
/// index of the thing in `things`.
///
/// The actual implementation details of this can be found in the `components` module
#[derive(Clone)]
enum VarZeroVecInner<'a, T> {
    Owned(Vec<T>),
    /// This is *basically* an `&'a [u8]` to a zero copy buffer, but split out into
    /// the buffer components. Logically this is capable of behaving as
    /// a `&'a [T::VarULE]`, but since `T::VarULE` is unsized that type does not actually
    /// exist
    Borrowed(SliceComponents<'a, T>),
}

#[derive(Clone, Debug)]
pub enum VarZeroVecError<E> {
    FormatError,
    ParseError(E),
}

impl<E: Display> Display for VarZeroVecError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::FormatError => write!(f, "Incorrect slice format"),
            Self::ParseError(ref e) => e.fmt(f),
        }
    }
}

impl<T: AsVarULE> fmt::Debug for VarZeroVec<'_, T>
where
    T::VarULE: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

type ParseErrorFor<T> = VarZeroVecError<<<T as AsVarULE>::VarULE as VarULE>::Error>;

impl<E> From<E> for VarZeroVecError<E> {
    fn from(e: E) -> Self {
        Self::ParseError(e)
    }
}

impl<'a, T> From<Vec<T>> for VarZeroVec<'a, T> {
    #[inline]
    fn from(other: Vec<T>) -> Self {
        Self(VarZeroVecInner::Owned(other))
    }
}

impl<'a, T> From<VarZeroVecInner<'a, T>> for VarZeroVec<'a, T> {
    #[inline]
    fn from(other: VarZeroVecInner<'a, T>) -> Self {
        Self(other)
    }
}

impl<'a, T: AsVarULE> VarZeroVec<'a, T> {
    /// Get the number of elements in this vector
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::str::Utf8Error;
    /// # use zerovec::VarZeroVecError;
    /// # use zerovec::VarZeroVec;
    ///
    /// let strings = vec!["foo".to_owned(), "bar".to_owned(),
    ///                    "baz".to_owned(), "quux".to_owned()];
    /// let bytes = VarZeroVec::get_serializable_bytes(&strings).unwrap();
    ///
    /// let mut vec: VarZeroVec<String> = VarZeroVec::try_from_bytes(&bytes)?;
    /// assert_eq!(vec.len(), 4);
    /// # Ok::<(), VarZeroVecError<Utf8Error>>(())
    /// ```
    pub fn len(&self) -> usize {
        match self.0 {
            VarZeroVecInner::Owned(ref vec) => vec.len(),
            VarZeroVecInner::Borrowed(components) => components.len(),
        }
    }

    /// Returns `true` if the vector contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::str::Utf8Error;
    /// # use zerovec::VarZeroVecError;
    /// # use zerovec::VarZeroVec;
    ///
    /// let strings: Vec<String> = vec![];
    /// let bytes = VarZeroVec::get_serializable_bytes(&strings).unwrap();
    ///
    /// let mut vec: VarZeroVec<String> = VarZeroVec::try_from_bytes(&bytes)?;
    /// assert!(vec.is_empty());
    /// # Ok::<(), VarZeroVecError<Utf8Error>>(())
    /// ```
    pub fn is_empty(&self) -> bool {
        match self.0 {
            VarZeroVecInner::Owned(ref vec) => vec.is_empty(),
            VarZeroVecInner::Borrowed(components) => components.is_empty(),
        }
    }

    /// Parse a VarZeroVec from a slice of the appropriate format
    ///
    /// Slices of the right format can be obtained via VarZeroVec::get_serializable_bytes()
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::str::Utf8Error;
    /// # use zerovec::VarZeroVecError;
    /// # use zerovec::VarZeroVec;
    ///
    /// let strings = vec!["foo".to_owned(), "bar".to_owned(),
    ///                    "baz".to_owned(), "quux".to_owned()];
    /// let bytes = VarZeroVec::get_serializable_bytes(&strings).unwrap();
    ///
    /// let mut vec: VarZeroVec<String> = VarZeroVec::try_from_bytes(&bytes)?;
    /// assert_eq!(&vec[0], "foo");
    /// assert_eq!(&vec[1], "bar");
    /// assert_eq!(&vec[2], "baz");
    /// assert_eq!(&vec[3], "quux");
    /// # Ok::<(), VarZeroVecError<Utf8Error>>(())
    /// ```
    pub fn try_from_bytes(slice: &'a [u8]) -> Result<Self, ParseErrorFor<T>> {
        if slice.is_empty() {
            // does not allocate
            return Ok(VarZeroVecInner::Owned(Vec::new()).into());
        }

        let components = SliceComponents::<T>::try_from_bytes(slice)?;

        Ok(VarZeroVecInner::Borrowed(components).into())
    }

    /// Obtain an iterator over VarZeroVec's elements
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::str::Utf8Error;
    /// # use zerovec::VarZeroVecError;
    /// # use zerovec::VarZeroVec;
    ///
    /// let strings = vec!["foo".to_owned(), "bar".to_owned(),
    ///                    "baz".to_owned(), "quux".to_owned()];
    /// let bytes = VarZeroVec::get_serializable_bytes(&strings).unwrap();
    /// let mut vec: VarZeroVec<String> = VarZeroVec::try_from_bytes(&bytes)?;
    ///
    /// let mut iter_results: Vec<&str> = vec.iter().collect();
    /// assert_eq!(iter_results[0], "foo");
    /// assert_eq!(iter_results[1], "bar");
    /// assert_eq!(iter_results[2], "baz");
    /// assert_eq!(iter_results[3], "quux");
    /// # Ok::<(), VarZeroVecError<Utf8Error>>(())
    /// ```
    pub fn iter<'b: 'a>(&'b self) -> impl Iterator<Item = &'b T::VarULE> {
        // We use Either here so that we can use `impl Trait` with heterogeneous types
        //
        // An alternate design is to write explicit iterators. Once Rust has generators this will
        // be unnecessary.
        match self.0 {
            VarZeroVecInner::Owned(ref vec) => Either::Left(vec.iter().map(|t| t.as_unaligned())),
            VarZeroVecInner::Borrowed(components) => Either::Right(components.iter()),
        }
    }

    /// Get one of VarZeroVec's elements, returning None if the index is out of bounds
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::str::Utf8Error;
    /// # use zerovec::VarZeroVecError;
    /// # use zerovec::VarZeroVec;
    ///
    /// let strings = vec!["foo".to_owned(), "bar".to_owned(),
    ///                    "baz".to_owned(), "quux".to_owned()];
    /// let bytes = VarZeroVec::get_serializable_bytes(&strings).unwrap();
    /// let mut vec: VarZeroVec<String> = VarZeroVec::try_from_bytes(&bytes)?;
    ///
    /// let mut iter_results: Vec<&str> = vec.iter().collect();
    /// assert_eq!(vec.get(0), Some("foo"));
    /// assert_eq!(vec.get(1), Some("bar"));
    /// assert_eq!(vec.get(2), Some("baz"));
    /// assert_eq!(vec.get(3), Some("quux"));
    /// assert_eq!(vec.get(4), None);
    /// # Ok::<(), VarZeroVecError<Utf8Error>>(())
    /// ```
    pub fn get(&self, idx: usize) -> Option<&T::VarULE> {
        match self.0 {
            VarZeroVecInner::Owned(ref vec) => vec.get(idx).map(|t| t.as_unaligned()),
            VarZeroVecInner::Borrowed(components) => components.get(idx),
        }
    }

    /// Convert this into a mutable vector of the owned `T` type, cloning if necessary.
    ///
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// # use std::str::Utf8Error;
    /// # use zerovec::VarZeroVecError;
    /// # use zerovec::VarZeroVec;
    ///
    /// let strings = vec!["foo".to_owned(), "bar".to_owned(),
    ///                    "baz".to_owned(), "quux".to_owned()];
    /// let bytes = VarZeroVec::get_serializable_bytes(&strings).unwrap();
    /// let mut vec: VarZeroVec<String> = VarZeroVec::try_from_bytes(&bytes)?;
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
    /// # Ok::<(), VarZeroVecError<Utf8Error>>(())
    /// ```
    //
    // This function is crate-public for now since we don't yet want to stabilize
    // the internal implementation details
    pub(crate) fn make_mut(&mut self) -> &mut Vec<T>
    where
        T: Clone,
    {
        match self.0 {
            VarZeroVecInner::Owned(ref mut vec) => vec,
            VarZeroVecInner::Borrowed(components) => {
                let vec = components.iter().map(T::from_unaligned).collect();
                let new_self = VarZeroVecInner::Owned(vec).into();
                *self = new_self;
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
    /// # use zerovec::VarZeroVecError;
    /// # use zerovec::VarZeroVec;
    ///
    /// let strings = vec!["foo".to_owned(), "bar".to_owned(),
    ///                    "baz".to_owned(), "quux".to_owned()];
    /// let bytes = VarZeroVec::get_serializable_bytes(&strings).unwrap();
    /// let mut vec: VarZeroVec<String> = VarZeroVec::try_from_bytes(&bytes)?;
    ///
    /// assert_eq!(vec.len(), 4);
    /// // has 'static lifetime
    /// let owned = vec.into_owned();
    /// # Ok::<(), VarZeroVecError<Utf8Error>>(())
    /// ```
    pub fn into_owned(mut self) -> VarZeroVec<'static, T>
    where
        T: Clone,
    {
        self.make_mut();
        match self.0 {
            VarZeroVecInner::Owned(vec) => vec.into(),
            _ => unreachable!(),
        }
    }

    /// Obtain an owned `Vec<T>` out of this
    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        let mut new = self.clone();
        new.make_mut();
        match new.0 {
            VarZeroVecInner::Owned(vec) => vec,
            _ => unreachable!(),
        }
    }

    /// If this is borrowed, get the borrowed slice
    #[cfg(feature = "serde")]
    pub(crate) fn get_slice_for_borrowed(&self) -> Option<&'a [u8]> {
        match self.0 {
            VarZeroVecInner::Owned(..) => None,
            VarZeroVecInner::Borrowed(b) => Some(b.entire_slice()),
        }
    }

    /// For a slice of `T`, get a list of bytes that can be passed to
    /// `try_from_bytes` to recoup the same data.
    ///
    /// Returns `None` if the slice is too large to be represented in a list of
    /// bytes whose length fits in a `u32`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::str::Utf8Error;
    /// # use zerovec::VarZeroVecError;
    /// # use zerovec::VarZeroVec;
    ///
    /// let strings = vec!["foo".to_owned(), "bar".to_owned(), "baz".to_owned()];
    /// let bytes = VarZeroVec::get_serializable_bytes(&strings).unwrap();
    ///
    /// let mut borrowed: VarZeroVec<String> = VarZeroVec::try_from_bytes(&bytes)?;
    /// assert_eq!(borrowed, &*strings);
    ///
    /// # Ok::<(), VarZeroVecError<Utf8Error>>(())
    /// ```
    ///
    pub fn get_serializable_bytes(elements: &[T]) -> Option<Vec<u8>> {
        components::get_serializable_bytes(elements)
    }
}

impl<'a, T> VarZeroVec<'a, T>
where
    T: AsVarULE,
    T::VarULE: Ord,
{
    /// Binary searches a sorted `VarZeroVec<T>` for the given element. For more information, see
    /// the primitive function [`binary_search`].
    ///
    /// # Example
    ///
    /// ```
    /// # use std::str::Utf8Error;
    /// # use zerovec::VarZeroVecError;
    /// # use zerovec::VarZeroVec;
    ///
    /// let strings = vec!["a".to_owned(), "b".to_owned(),
    ///                    "f".to_owned(), "g".to_owned()];
    /// let bytes = VarZeroVec::get_serializable_bytes(&strings).unwrap();
    /// let mut vec: VarZeroVec<String> = VarZeroVec::try_from_bytes(&bytes)?;
    ///
    /// assert_eq!(vec.binary_search("f"), Ok(2));
    /// assert_eq!(vec.binary_search("e"), Err(2));
    /// # Ok::<(), VarZeroVecError<Utf8Error>>(())
    /// ```
    ///
    /// [`binary_search`]: https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search
    #[inline]
    pub fn binary_search(&self, x: &T::VarULE) -> Result<usize, usize> {
        match self.0 {
            VarZeroVecInner::Owned(ref vec) => {
                vec.binary_search_by(|probe| probe.as_unaligned().cmp(x))
            }
            VarZeroVecInner::Borrowed(components) => components.binary_search(x),
        }
    }
}

impl<'a, T: AsVarULE> Index<usize> for VarZeroVec<'a, T> {
    type Output = T::VarULE;
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("Indexing VarZeroVec out of bounds")
    }
}

impl<'a, 'b, T> PartialEq<VarZeroVec<'b, T>> for VarZeroVec<'a, T>
where
    T: AsVarULE,
    T::VarULE: PartialEq,
{
    #[inline]
    fn eq(&self, other: &VarZeroVec<'b, T>) -> bool {
        // Note: T implements PartialEq but not T::ULE
        self.iter().eq(other.iter())
    }
}

impl<T> PartialEq<&[T]> for VarZeroVec<'_, T>
where
    T: AsVarULE,
    T::VarULE: PartialEq,
{
    #[inline]
    fn eq(&self, other: &&[T]) -> bool {
        self.iter().eq(other.iter().map(|t| t.as_unaligned()))
    }
}
