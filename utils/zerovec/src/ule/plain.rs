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

/// A signed version of [`PlainOldULE`]. This primarily exists so that complex types
/// like `VarZeroVec<[SignedPlainOldULE]>` will correctly serialize in a human-readable
/// fashion.
#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SignedPlainOldULE<const N: usize>(pub PlainOldULE<N>);

// These impls exist to make the macros simpler
impl<const N: usize> PlainOldULE<N> {
    #[inline]
    fn into_bytes(self) -> [u8; N] {
        self.0
    }
}

impl<const N: usize> SignedPlainOldULE<N> {
    #[inline]
    fn into_bytes(self) -> [u8; N] {
        self.0 .0
    }
}

macro_rules! impl_byte_slice_size {
    ($size:literal) => {
        impl From<[u8; $size]> for PlainOldULE<$size> {
            #[inline]
            fn from(le_bytes: [u8; $size]) -> Self {
                Self(le_bytes)
            }
        }
        impl From<[u8; $size]> for SignedPlainOldULE<$size> {
            #[inline]
            fn from(le_bytes: [u8; $size]) -> Self {
                Self(le_bytes.into())
            }
        }
        impl PlainOldULE<$size> {
            #[inline]
            pub fn as_bytes(&self) -> &[u8] {
                &self.0
            }
        }
        // Safety (based on the safety checklist on the ULE trait):
        //  1. PlainOldULE does not include any uninitialized or padding bytes.
        //     (achieved by `#[repr(transparent)]` on a type that satisfies this invariant)
        //  2. PlainOldULE is aligned to 1 byte.
        //     (achieved by `#[repr(transparent)]` on a type that satisfies this invariant)
        //  3. The impl of validate_byte_slice() returns an error if any byte is not valid (never).
        //  4. The impl of validate_byte_slice() returns an error if there are leftover bytes.
        //  5. The other ULE methods use the default impl.
        //  6. PlainOldULE byte equality is semantic equality
        unsafe impl ULE for PlainOldULE<$size> {
            type Error = ULEError<core::convert::Infallible>;

            #[inline]
            fn validate_byte_slice(bytes: &[u8]) -> Result<(), Self::Error> {
                if bytes.len() % $size == 0 {
                    // Safe because Self is transparent over [u8; $size]
                    Ok(())
                } else {
                    Err(ULEError::InvalidLength {
                        ty: concat!("PlainOldULE<", stringify!($size), ">"),
                        len: bytes.len(),
                    })
                }
            }
        }

        /// Forwards to above impl and is safe for the same reason
        unsafe impl ULE for SignedPlainOldULE<$size> {
            type Error = ULEError<core::convert::Infallible>;

            #[inline]
            fn validate_byte_slice(bytes: &[u8]) -> Result<(), Self::Error> {
                PlainOldULE::<$size>::validate_byte_slice(bytes)
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
    ($type:ty, $ule_type:ty) => {
        impl From<$type> for $ule_type {
            #[inline]
            fn from(value: $type) -> Self {
                value.to_le_bytes().into()
            }
        }
        impl AsULE for $type {
            type ULE = $ule_type;
            #[inline]
            fn as_unaligned(self) -> Self::ULE {
                self.to_le_bytes().into()
            }
            #[inline]
            fn from_unaligned(unaligned: Self::ULE) -> Self {
                <$type>::from_le_bytes(unaligned.into_bytes())
            }
        }
        // EqULE is true because $type and PlainOldULE<$size>
        // have the same byte sequence on little-endian
        unsafe impl EqULE for $type {}

        #[cfg(feature = "serde")]
        impl serde::Serialize for $ule_type {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                <$type>::from_unaligned(*self).serialize(serializer)
            }
        }

        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for $ule_type {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                Ok(<$type>::as_unaligned(<$type>::deserialize(deserializer)?))
            }
        }
    };
}

impl_byte_slice_size!(2);
impl_byte_slice_size!(4);
impl_byte_slice_size!(8);
impl_byte_slice_size!(16);

impl_byte_slice_type!(u16, PlainOldULE<2>);
impl_byte_slice_type!(u32, PlainOldULE<4>);
impl_byte_slice_type!(u64, PlainOldULE<8>);
impl_byte_slice_type!(u128, PlainOldULE<16>);

impl_byte_slice_type!(i16, SignedPlainOldULE<2>);
impl_byte_slice_type!(i32, SignedPlainOldULE<4>);
impl_byte_slice_type!(i64, SignedPlainOldULE<8>);
impl_byte_slice_type!(i128, SignedPlainOldULE<16>);

// Safety (based on the safety checklist on the ULE trait):
//  1. u8 does not include any uninitialized or padding bytes.
//  2. u8 is aligned to 1 byte.
//  3. The impl of validate_byte_slice() returns an error if any byte is not valid (never).
//  4. The impl of validate_byte_slice() returns an error if there are leftover bytes (never).
//  5. The other ULE methods use the default impl.
//  6. u8 byte equality is semantic equality
unsafe impl ULE for u8 {
    type Error = core::convert::Infallible;
    #[inline]
    fn validate_byte_slice(_bytes: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }
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
    fn as_unaligned(self) -> Self::ULE {
        self
    }
    #[inline]
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        unaligned
    }
}

// EqULE is true because u8 is its own ULE.
unsafe impl EqULE for u8 {}
