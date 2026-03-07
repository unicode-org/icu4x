// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::borrow::Borrow;

#[cfg(any(feature = "serde", feature = "alloc"))]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// A struct transparent over `[u8]` with convenient helper functions.
#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct ByteStr([u8]);

impl ByteStr {
    #[allow(unsafe_code)] // transparent newtype casts are documented
    pub const fn from_bytes(input: &[u8]) -> &Self {
        // Safety: [u8] and ByteStr have the same layout and invariants
        unsafe { &*(input as *const [u8] as *const Self) }
    }

    #[cfg(feature = "serde")]
    #[allow(unsafe_code)] // transparent newtype casts are documented
    pub fn from_boxed_bytes(input: Box<[u8]>) -> Box<Self> {
        // Safety: [u8] and ByteStr have the same layout and invariants
        unsafe { core::mem::transmute::<Box<[u8]>, Box<Self>>(input) }
    }

    #[allow(dead_code)] // may want this in the future
    pub fn from_str(input: &str) -> &Self {
        Self::from_bytes(input.as_bytes())
    }

    #[allow(dead_code)] // may want this in the future
    pub fn empty() -> &'static Self {
        Self::from_bytes(&[])
    }

    #[allow(dead_code)] // not used in all features
    pub const fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub const fn len(&self) -> usize {
        self.0.len()
    }

    #[allow(dead_code)] // not used in all features
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
    #[allow(clippy::indexing_slicing)] // "panic" is in method name
    pub(crate) const fn byte_at_or_panic(&self, index: usize) -> u8 {
        self.0[index]
    }

    /// Const function to evaluate `self < other`.
    #[allow(clippy::indexing_slicing)] // in-range loop conditions
    pub(crate) const fn is_less_then(&self, other: &Self) -> bool {
        let mut i = 0;
        while i < self.len() && i < other.len() {
            if self.0[i] < other.0[i] {
                return true;
            }
            if self.0[i] > other.0[i] {
                return false;
            }
            i += 1;
        }
        self.len() < other.len()
    }

    /// Const function to evaluate `self[..prefix_len] == other[..prefix_len]`
    ///
    /// # Panics
    ///
    /// Panics if `prefix_len` is longer than either this string or the other string
    #[allow(clippy::indexing_slicing)] // in-range loop conditions
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

impl Borrow<[u8]> for ByteStr {
    fn borrow(&self) -> &[u8] {
        self.as_bytes()
    }
}

#[cfg(feature = "alloc")]
impl Borrow<[u8]> for Box<ByteStr> {
    fn borrow(&self) -> &[u8] {
        self.as_bytes()
    }
}

/// An abstraction over a slice of key-value pairs that can have either `&[u8]`
/// or `&str` keys. This is used in the builder to avoid unsound casts.
#[derive(Copy, Clone)]
pub(crate) enum SliceWithIndices<'a> {
    Bytes(&'a [(&'a [u8], usize)]),
    Str(&'a [(&'a str, usize)]),
}

impl<'a> SliceWithIndices<'a> {
    pub const fn from_byte_slice(s: &'a [(&'a [u8], usize)]) -> Self {
        Self::Bytes(s)
    }

    pub const fn from_str_slice(s: &'a [(&'a str, usize)]) -> Self {
        Self::Str(s)
    }

    pub const fn len(&self) -> usize {
        match self {
            Self::Bytes(s) => s.len(),
            Self::Str(s) => s.len(),
        }
    }

    #[allow(clippy::indexing_slicing)]
    pub const fn get(&self, index: usize) -> (&'a ByteStr, usize) {
        match self {
            Self::Bytes(s) => {
                let (key, value) = s[index];
                (ByteStr::from_bytes(key), value)
            }
            Self::Str(s) => {
                let (key, value) = s[index];
                (ByteStr::from_bytes(key.as_bytes()), value)
            }
        }
    }

    pub const fn last(&self) -> Option<(&'a ByteStr, usize)> {
        if self.len() == 0 {
            None
        } else {
            Some(self.get(self.len() - 1))
        }
    }

    #[cfg(feature = "alloc")]
    pub fn to_vec_u8(&self) -> Vec<(&'a [u8], usize)> {
        match self {
            Self::Bytes(s) => s.to_vec(),
            Self::Str(s) => s.iter().map(|(k, v)| (k.as_bytes(), *v)).collect(),
        }
    }
}
