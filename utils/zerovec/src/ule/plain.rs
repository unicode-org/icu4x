// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(clippy::upper_case_acronyms)]
//! ULE implementation for Plain Old Data types, including all sized integers.

use super::*;
use crate::ZeroSlice;

/// A u8 array of little-endian data with infallible conversions to and from &[u8].
#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub struct RawBytesULE<const N: usize>(pub [u8; N]);

macro_rules! impl_byte_slice_size {
    ($unsigned:ty, $size:literal) => {
        impl From<[u8; $size]> for RawBytesULE<$size> {
            #[inline]
            fn from(le_bytes: [u8; $size]) -> Self {
                Self(le_bytes)
            }
        }
        impl RawBytesULE<$size> {
            #[inline]
            pub fn as_bytes(&self) -> &[u8] {
                &self.0
            }
        }
        // Safety (based on the safety checklist on the ULE trait):
        //  1. RawBytesULE does not include any uninitialized or padding bytes.
        //     (achieved by `#[repr(transparent)]` on a type that satisfies this invariant)
        //  2. RawBytesULE is aligned to 1 byte.
        //     (achieved by `#[repr(transparent)]` on a type that satisfies this invariant)
        //  3. The impl of validate_byte_slice() returns an error if any byte is not valid (never).
        //  4. The impl of validate_byte_slice() returns an error if there are leftover bytes.
        //  5. The other ULE methods use the default impl.
        //  6. RawBytesULE byte equality is semantic equality
        unsafe impl ULE for RawBytesULE<$size> {
            #[inline]
            fn validate_byte_slice(bytes: &[u8]) -> Result<(), ZeroVecError> {
                if bytes.len() % $size == 0 {
                    // Safe because Self is transparent over [u8; $size]
                    Ok(())
                } else {
                    Err(ZeroVecError::length::<Self>(bytes.len()))
                }
            }
        }

        impl RawBytesULE<$size> {
            #[inline]
            pub fn from_byte_slice_unchecked_mut(bytes: &mut [u8]) -> &mut [Self] {
                let data = bytes.as_mut_ptr();
                let len = bytes.len() / $size;
                // Safe because Self is transparent over [u8; $size]
                unsafe { core::slice::from_raw_parts_mut(data as *mut Self, len) }
            }

            /// Gets this RawBytesULE as an unsigned int. This is equivalent to calling
            /// [AsULE::from_unaligned()] on the appropriately sized type.
            #[inline]
            pub fn as_unsigned_int(&self) -> $unsigned {
                <$unsigned as $crate::ule::AsULE>::from_unaligned(*self)
            }
        }

        impl ZeroSlice<$unsigned> {
            /// This function can be used for constructing ZeroVecs in a const context, avoiding
            /// parsing checks.
            ///
            /// This cannot be generic over T because of current limitations in `const`, but if
            /// this method is needed in a non-const context, check out [`ZeroSlice::parse_byte_slice()`]
            /// instead.
            pub const fn try_from_bytes(bytes: &[u8]) -> Result<&Self, ZeroVecError> {
                let len = bytes.len();
                if len % $size == 0 {
                    unsafe { Ok(mem::transmute(bytes)) }
                } else {
                    Err(ZeroVecError::InvalidLength {
                        ty: concat!("RawBytesULE< ", $size, ">"),
                        len,
                    })
                }
            }
        }
    };
}

macro_rules! impl_byte_slice_type {
    ($type:ty, $size:literal) => {
        impl From<$type> for RawBytesULE<$size> {
            #[inline]
            fn from(value: $type) -> Self {
                Self(value.to_le_bytes())
            }
        }
        impl AsULE for $type {
            type ULE = RawBytesULE<$size>;
            #[inline]
            fn to_unaligned(self) -> Self::ULE {
                RawBytesULE(self.to_le_bytes())
            }
            #[inline]
            fn from_unaligned(unaligned: Self::ULE) -> Self {
                <$type>::from_le_bytes(unaligned.0)
            }
        }
        // EqULE is true because $type and RawBytesULE<$size>
        // have the same byte sequence on little-endian
        unsafe impl EqULE for $type {}
    };
}

impl_byte_slice_size!(u16, 2);
impl_byte_slice_size!(u32, 4);
impl_byte_slice_size!(u64, 8);
impl_byte_slice_size!(u128, 16);

impl_byte_slice_type!(u16, 2);
impl_byte_slice_type!(u32, 4);
impl_byte_slice_type!(u64, 8);
impl_byte_slice_type!(u128, 16);

impl_byte_slice_type!(i16, 2);
impl_byte_slice_type!(i32, 4);
impl_byte_slice_type!(i64, 8);
impl_byte_slice_type!(i128, 16);

// Safety (based on the safety checklist on the ULE trait):
//  1. u8 does not include any uninitialized or padding bytes.
//  2. u8 is aligned to 1 byte.
//  3. The impl of validate_byte_slice() returns an error if any byte is not valid (never).
//  4. The impl of validate_byte_slice() returns an error if there are leftover bytes (never).
//  5. The other ULE methods use the default impl.
//  6. u8 byte equality is semantic equality
unsafe impl ULE for u8 {
    #[inline]
    fn validate_byte_slice(_bytes: &[u8]) -> Result<(), ZeroVecError> {
        Ok(())
    }
}

impl AsULE for u8 {
    type ULE = Self;
    #[inline]
    fn to_unaligned(self) -> Self::ULE {
        self
    }
    #[inline]
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        unaligned
    }
}

// EqULE is true because u8 is its own ULE.
unsafe impl EqULE for u8 {}

// Safety (based on the safety checklist on the ULE trait):
//  1. i8 does not include any uninitialized or padding bytes.
//  2. i8 is aligned to 1 byte.
//  3. The impl of validate_byte_slice() returns an error if any byte is not valid (never).
//  4. The impl of validate_byte_slice() returns an error if there are leftover bytes (never).
//  5. The other ULE methods use the default impl.
//  6. i8 byte equality is semantic equality
unsafe impl ULE for i8 {
    #[inline]
    fn validate_byte_slice(_bytes: &[u8]) -> Result<(), ZeroVecError> {
        Ok(())
    }
}

impl AsULE for i8 {
    type ULE = Self;
    #[inline]
    fn to_unaligned(self) -> Self::ULE {
        self
    }
    #[inline]
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        unaligned
    }
}

// EqULE is true because i8 is its own ULE.
unsafe impl EqULE for i8 {}
