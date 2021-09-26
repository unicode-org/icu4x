// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(clippy::upper_case_acronyms)]
//! ULE implementation for Plain Old Data types, including all sized integers.

use super::*;

/// A u8 array of little-endian data with infallible conversions to and from &[u8].
#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PlainOldULE<const N: usize>(pub [u8; N]);

macro_rules! impl_byte_slice_size {
    ($size:literal) => {
        impl From<[u8; $size]> for PlainOldULE<$size> {
            #[inline]
            fn from(le_bytes: [u8; $size]) -> Self {
                Self(le_bytes)
            }
        }
        impl PlainOldULE<$size> {
            #[inline]
            pub fn as_bytes(&self) -> &[u8] {
                &self.0
            }
        }
        // This is safe to implement because from_byte_slice_unchecked returns
        // the same value as parse_byte_slice
        unsafe impl ULE for PlainOldULE<$size> {
            type Error = core::convert::Infallible;
            #[inline]
            fn parse_byte_slice(bytes: &[u8]) -> Result<&[Self], Self::Error> {
                // Safe because Self is transparent over [u8; $size]
                Ok(unsafe { Self::from_byte_slice_unchecked(bytes) })
            }
        }

        impl PlainOldULE<$size> {
            #[inline]
            pub fn from_byte_slice_unchecked_mut(bytes: &mut [u8]) -> &mut [Self] {
                let data = bytes.as_mut_ptr();
                let len = bytes.len() / $size;
                // Safe because Self is transparent over [u8; $size]
                unsafe { core::slice::from_raw_parts_mut(data as *mut Self, len) }
            }
        }
    };
}

macro_rules! impl_byte_slice_type {
    ($type:ty, $size:literal) => {
        impl From<$type> for PlainOldULE<$size> {
            #[inline]
            fn from(value: $type) -> Self {
                Self(value.to_le_bytes())
            }
        }
        impl AsULE for $type {
            type ULE = PlainOldULE<$size>;
            #[inline]
            fn as_unaligned(&self) -> Self::ULE {
                PlainOldULE(self.to_le_bytes())
            }
            #[inline]
            fn from_unaligned(unaligned: &Self::ULE) -> Self {
                <$type>::from_le_bytes(unaligned.0)
            }
        }
        // EqULE is true because $type and PlainOldULE<$size>
        // have the same byte sequence on little-endian
        unsafe impl EqULE for $type {}
    };
}

impl_byte_slice_size!(2);
impl_byte_slice_size!(4);
impl_byte_slice_size!(8);
impl_byte_slice_size!(16);

impl_byte_slice_type!(u16, 2);
impl_byte_slice_type!(u32, 4);
impl_byte_slice_type!(u64, 8);
impl_byte_slice_type!(u128, 16);

impl_byte_slice_type!(i16, 2);
impl_byte_slice_type!(i32, 4);
impl_byte_slice_type!(i64, 8);
impl_byte_slice_type!(i128, 16);

// This is safe to implement because from_byte_slice_unchecked returns
// the same value as parse_byte_slice
unsafe impl ULE for u8 {
    type Error = core::convert::Infallible;
    #[inline]
    fn parse_byte_slice(bytes: &[u8]) -> Result<&[Self], Self::Error> {
        Ok(bytes)
    }
    #[inline]
    unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &[Self] {
        bytes
    }
    #[inline]
    fn as_byte_slice(slice: &[Self]) -> &[u8] {
        slice
    }
}

impl AsULE for u8 {
    type ULE = Self;
    #[inline]
    fn as_unaligned(&self) -> Self::ULE {
        *self
    }
    #[inline]
    fn from_unaligned(unaligned: &Self::ULE) -> Self {
        *unaligned
    }
}

// EqULE is true because u8 is its own ULE.
unsafe impl EqULE for u8 {}
