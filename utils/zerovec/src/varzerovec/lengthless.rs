// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::components::VarZeroVecComponents;
use super::*;
use crate::ule::*;
use core::marker::PhantomData;
use core::mem;

/// A slice representing the index and data tables of a VarZeroVec,
/// *without* any length fields. The length field is expected to be stored elsewhere.
///
/// Without knowing the length this is of course unsafe to use directly.
#[repr(transparent)]
pub(crate) struct VarZeroLengthlessSlice<T: ?Sized, F = Index16> {
    marker: PhantomData<(F, T)>,
    /// The original slice this was constructed from
    // Safety invariant: This field must have successfully passed through
    // VarZeroVecComponents::parse_byte_slice_with_length() with the length
    // associated with this value.
    entire_slice: [u8],
}

impl<T: VarULE + ?Sized, F: VarZeroVecFormat> VarZeroLengthlessSlice<T, F> {
    /// Construct a new empty VarZeroLengthlessSlice
    pub(crate) const fn new_empty() -> &'static Self {
        // The empty VZV is special-cased to the empty slice
        unsafe { mem::transmute(&[] as &[u8]) }
    }

    /// Obtain a [`VarZeroVecComponents`] borrowing from the internal buffer
    ///
    /// Safety: `len` must be the length associated with this value
    #[inline]
    pub(crate) unsafe fn as_components<'a>(&'a self, len: u32) -> VarZeroVecComponents<'a, T, F> {
        unsafe {
            // safety: VarZeroSlice is guaranteed to parse here
            VarZeroVecComponents::from_bytes_unchecked_with_length(len, &self.entire_slice)
        }
    }

    /// Uses a `&[u8]` buffer as a `VarZeroSlice<T>` without any verification.
    ///
    /// # Safety
    ///
    /// `bytes` need to be an output from [`VarZeroLengthlessSlice::as_bytes()`], or alternatively
    /// be valid to be passed to `from_bytes_unchecked_with_length`
    ///
    /// The length associated with this value will be the length associated with the original slice.
    pub(crate) const unsafe fn from_bytes_unchecked(bytes: &[u8]) -> &Self {
        // self is really just a wrapper around a byte slice
        mem::transmute(bytes)
    }

    /// Get one of this slice's elements
    ///
    /// # Safety
    ///
    /// `index` must be in range, and `len` must be the length associated with this
    /// instance of VarZeroLengthlessSlice.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use zerovec::VarZeroVec;
    ///
    /// let strings = vec!["foo", "bar", "baz", "quux"];
    /// let vec = VarZeroVec::<str>::from(&strings);
    ///
    /// let mut iter_results: Vec<&str> = vec.iter().collect();
    /// unsafe {
    ///     assert_eq!(vec.get_unchecked(0), "foo");
    ///     assert_eq!(vec.get_unchecked(1), "bar");
    ///     assert_eq!(vec.get_unchecked(2), "baz");
    ///     assert_eq!(vec.get_unchecked(3), "quux");
    /// }
    /// ```
    pub(crate) unsafe fn get_unchecked(&self, len: u32, idx: usize) -> &T {
        self.as_components(len).get_unchecked(idx)
    }

    /// Get a reference to the entire encoded backing buffer of this slice
    ///
    /// The bytes can be passed back to [`Self::parse_byte_slice()`].
    ///
    /// To take the bytes as a vector, see [`VarZeroVec::into_bytes()`].
    ///
    /// # Example
    ///
    /// ```rust
    /// # use zerovec::VarZeroVec;
    ///
    /// let strings = vec!["foo", "bar", "baz"];
    /// let vzv = VarZeroVec::<str>::from(&strings);
    ///
    /// assert_eq!(vzv, VarZeroVec::parse_byte_slice(vzv.as_bytes()).unwrap());
    /// ```
    #[inline]
    pub(crate) const fn as_bytes(&self) -> &[u8] {
        &self.entire_slice
    }
}
