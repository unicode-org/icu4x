// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::comparison;
use core::cmp::Ordering;
use core::fmt;

#[cfg(feature = "serde")]
use alloc::boxed::Box;

/// A string key in a ZeroTrie.
///
/// This type has a custom Ord impl, making it suitable for use in a sorted
/// map for ZeroTrie construction.
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct ByteStr([u8]);

impl ByteStr {
    #[inline]
    pub(crate) const fn from_byte_slice_with_value<'a, 'l>(
        input: &'l [(&'a [u8], usize)],
    ) -> &'l [(&'a ByteStr, usize)] {
        // Safety: [u8] and ByteStr have the same layout and invariants
        unsafe { core::mem::transmute(input) }
    }

    #[inline]
    pub(crate) const fn from_str_slice_with_value<'a, 'l>(
        input: &'l [(&'a str, usize)],
    ) -> &'l [(&'a ByteStr, usize)] {
        // Safety: str and ByteStr have the same layout, and ByteStr is less restrictive
        unsafe { core::mem::transmute(input) }
    }

    /// Casts a `&[u8]` to a `&ByteStr`
    #[inline]
    pub const fn from_bytes(input: &[u8]) -> &Self {
        // Safety: [u8] and ByteStr have the same layout and invariants
        unsafe { core::mem::transmute(input) }
    }

    /// Casts a `Box<[u8]>` to a `Box<ByteStr>`
    #[cfg(feature = "alloc")]
    pub const fn from_boxed_bytes(input: Box<[u8]>) -> Box<Self> {
        // Safety: [u8] and ByteStr have the same layout and invariants
        unsafe { core::mem::transmute(input) }
    }

    /// Casts a `&str` to a `&ByteStr`
    pub const fn from_str(input: &str) -> &Self {
        Self::from_bytes(input.as_bytes())
    }

    /// Creates an empty ByteStr
    pub const fn empty() -> &'static Self {
        Self::from_bytes(&[])
    }

    /// Returns this ByteStr as a byte slice
    pub const fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Whether the ByteStr is an empty slice
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// How many bytes are in the ByteStr
    pub const fn len(&self) -> usize {
        self.0.len()
    }

    /// Whether the ByteStr is all ASCII-range
    pub fn is_all_ascii(&self) -> bool {
        for byte in self.0.iter() {
            if !byte.is_ascii() {
                return false;
            }
        }
        true
    }

    #[allow(dead_code)] // may want this in the future
    pub(crate) fn byte_at(&self, index: usize) -> Option<u8> {
        self.0.get(index).copied()
    }

    /// Returns the byte at the given index, panicking if out of bounds.
    pub(crate) const fn byte_at_or_panic(&self, index: usize) -> u8 {
        self.0[index]
    }

    /// Const function to evaluate `self < other`.
    pub(crate) const fn is_less_than(&self, other: &Self) -> bool {
        let mut i = 0;
        while i < self.len() && i < other.len() {
            let a = comparison::shift(self.0[i]);
            let b = comparison::shift(other.0[i]);
            if a < b {
                return true;
            }
            if a > b {
                return false;
            }
            i += 1;
        }
        self.len() < other.len()
    }

    /// Const function to evaluate `self[..prefix_len] == other[..prefix_len]`
    pub(crate) const fn prefix_eq(&self, other: &ByteStr, prefix_len: usize) -> bool {
        assert!(prefix_len <= self.len());
        assert!(prefix_len <= other.len());
        let mut i = 0;
        while i < prefix_len {
            if self.0[i] != other.0[i] {
                return false;
            }
            i += 1;
        }
        true
    }
}

// Note: Does NOT impl Borrow<[u8]> because the Ord impls differ.
// AsRef is okay to implement.

impl AsRef<[u8]> for ByteStr {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl AsRef<ByteStr> for ByteStr {
    fn as_ref(&self) -> &ByteStr {
        self
    }
}

impl<'a> From<&'a str> for &'a ByteStr {
    fn from(other: &'a str) -> Self {
        ByteStr::from_str(other)
    }
}

impl fmt::Debug for ByteStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        if let Ok(s) = core::str::from_utf8(self.as_bytes()) {
            write!(f, "{s}")
        } else {
            write!(f, "{:?}", self.as_bytes())
        }
    }
}

impl Ord for ByteStr {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        crate::comparison::cmp_slices(&self.0, &other.0)
    }
}

impl PartialOrd for ByteStr {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
