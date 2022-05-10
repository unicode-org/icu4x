// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::FlexZeroSlice;
use super::FlexZeroVecOwned;
use crate::ZeroVecError;
use core::cmp::Ordering;
use core::iter::FromIterator;
use core::ops::Deref;

/// A zero-copy data structure that efficiently stores integer values.
///
/// `FlexZeroVec` automatically increases or decreases its storage capacity based on the largest
/// integer stored in the vector. It therefore results in lower memory usage when smaller numbers
/// are usually stored, but larger values must sometimes also be stored.
///
/// The maximum value that can be stored in `FlexZeroVec` is `usize::MAX` on the current platform.
///
/// `FlexZeroVec` is the data structure for storing `usize` in a `ZeroMap`.
#[derive(Debug)]
pub enum FlexZeroVec<'a> {
    Owned(FlexZeroVecOwned),
    Borrowed(&'a FlexZeroSlice),
}

impl<'a> Deref for FlexZeroVec<'a> {
    type Target = FlexZeroSlice;
    fn deref(&self) -> &Self::Target {
        match self {
            FlexZeroVec::Owned(v) => v.deref(),
            FlexZeroVec::Borrowed(v) => v,
        }
    }
}

impl<'a> AsRef<FlexZeroSlice> for FlexZeroVec<'a> {
    fn as_ref(&self) -> &FlexZeroSlice {
        self.deref()
    }
}

impl Eq for FlexZeroVec<'_> {}

impl<'a, 'b> PartialEq<FlexZeroVec<'b>> for FlexZeroVec<'a> {
    #[inline]
    fn eq(&self, other: &FlexZeroVec<'b>) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<'a> Default for FlexZeroVec<'a> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> PartialOrd for FlexZeroVec<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.iter().partial_cmp(other.iter())
    }
}

impl<'a> Ord for FlexZeroVec<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.iter().cmp(other.iter())
    }
}

impl<'a> FlexZeroVec<'a> {
    #[inline]
    /// Creates a new, borrowed, empty `FlexZeroVec`.
    ///
    /// # Examples
    ///
    /// ```
    /// use zerovec::vecs::FlexZeroVec;
    ///
    /// let zv: FlexZeroVec = FlexZeroVec::new();
    /// assert!(zv.is_empty());
    /// ```
    pub fn new() -> Self {
        Self::Borrowed(FlexZeroSlice::new_empty())
    }

    /// Parses a `&[u8]` buffer into a `FlexZeroVec`.
    ///
    /// The bytes within the byte buffer must remain constant for the life of the FlexZeroVec.
    ///
    /// # Endianness
    ///
    /// The byte buffer must be encoded in little-endian, even if running in a big-endian
    /// environment. This ensures a consistent representation of data across platforms.
    ///
    /// # Example
    ///
    /// ```
    /// use zerovec::vecs::FlexZeroVec;
    ///
    /// let bytes: &[u8] = &[2, 0xD3, 0x00, 0x19, 0x01, 0xA5, 0x01, 0xCD, 0x01];
    /// let fzv = FlexZeroVec::parse_byte_slice(bytes).expect("valid slice");
    ///
    /// assert!(matches!(fzv, FlexZeroVec::Borrowed(_)));
    /// assert_eq!(fzv.get(2), Some(421));
    /// ```
    pub fn parse_byte_slice(bytes: &'a [u8]) -> Result<Self, ZeroVecError> {
        let slice: &'a FlexZeroSlice = FlexZeroSlice::parse_byte_slice(bytes)?;
        Ok(Self::Borrowed(slice))
    }

    /// Converts a borrowed FlexZeroVec to an owned FlexZeroVec. No-op if already owned.
    ///
    /// # Example
    ///
    /// ```
    /// use zerovec::vecs::FlexZeroVec;
    ///
    /// let bytes: &[u8] = &[2, 0xD3, 0x00, 0x19, 0x01, 0xA5, 0x01, 0xCD, 0x01];
    /// let fzv = FlexZeroVec::parse_byte_slice(bytes).expect("valid bytes");
    /// assert!(matches!(fzv, FlexZeroVec::Borrowed(_)));
    ///
    /// let owned = fzv.into_owned();
    /// assert!(matches!(owned, FlexZeroVec::Owned(_)));
    /// ```
    pub fn into_owned(self) -> FlexZeroVec<'static> {
        match self {
            Self::Owned(owned) => FlexZeroVec::Owned(owned),
            Self::Borrowed(slice) => FlexZeroVec::Owned(FlexZeroVecOwned::from_slice(slice)),
        }
    }

    /// Allows the FlexZeroVec to be mutated by converting it to an owned variant, and producing
    /// a mutable [`FlexZeroVecOwned`].
    ///
    /// # Example
    ///
    /// ```
    /// use zerovec::vecs::FlexZeroVec;
    ///
    /// let bytes: &[u8] = &[2, 0xD3, 0x00, 0x19, 0x01, 0xA5, 0x01, 0xCD, 0x01];
    /// let mut fzv = FlexZeroVec::parse_byte_slice(bytes).expect("valid bytes");
    /// assert!(matches!(fzv, FlexZeroVec::Borrowed(_)));
    ///
    /// fzv.to_mut().push(12);
    /// assert!(matches!(fzv, FlexZeroVec::Owned(_)));
    /// assert_eq!(fzv.get(4), Some(12));
    /// ```
    pub fn to_mut(&mut self) -> &mut FlexZeroVecOwned {
        match self {
            Self::Owned(ref mut owned) => owned,
            Self::Borrowed(slice) => {
                *self = FlexZeroVec::Owned(FlexZeroVecOwned::from_slice(slice));
                // recursion is limited since we are guaranteed to hit the Owned branch
                self.to_mut()
            }
        }
    }

    /// Remove all elements from this FlexZeroVec and reset it to an empty borrowed state.
    pub fn clear(&mut self) {
        *self = Self::Borrowed(FlexZeroSlice::new_empty())
    }
}

impl FromIterator<usize> for FlexZeroVec<'_> {
    /// Creates a [`FlexZeroVec::Owned`] from an iterator of `usize`.
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        FlexZeroVec::Owned(FlexZeroVecOwned::from_iter(iter))
    }
}
