// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Little-endian unaligned 24-bit unsigned integer.

use core::convert::TryFrom;
use zerovec::{
    ule::{AsULE, ULE},
    ZeroSlice, ZeroVecError,
};

/// Little-endian unaligned 24-bit unsigned integer.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_normalizer::u24),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct U24(pub [u8; 3]);

impl U24 {
    fn to_u32(self) -> u32 {
        u32::from(self.0[0]) | (u32::from(self.0[1]) << 8) | (u32::from(self.0[2]) << 16)
    }
}

impl AsULE for U24 {
    type ULE = U24;

    fn to_unaligned(self) -> Self::ULE {
        self
    }

    fn from_unaligned(unaligned: Self::ULE) -> Self {
        unaligned
    }
}

unsafe impl ULE for U24 {
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), ZeroVecError> {
        if bytes.len() % 3 == 0 {
            Ok(())
        } else {
            Err(ZeroVecError::length::<Self>(bytes.len()))
        }
    }
}

impl From<U24> for u32 {
    fn from(u: U24) -> Self {
        u.to_u32()
    }
}

/// Conversion input out of `U24` range
pub struct U24Error;

impl TryFrom<u32> for U24 {
    type Error = U24Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if (value & 0xFF000000) != 0 {
            Err(U24Error)
        } else {
            Ok(Self([
                (value as u8),
                ((value >> 8) as u8),
                ((value >> 16) as u8),
            ]))
        }
    }
}

impl PartialOrd for U24 {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        let self_u32 = self.to_u32();
        let other_u32 = other.to_u32();
        self_u32.partial_cmp(&other_u32)
    }
}

impl Ord for U24 {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let self_u32 = self.to_u32();
        let other_u32 = other.to_u32();
        self_u32.cmp(&other_u32)
    }
}

/// Slice that we need to transmute
const EMPTY_U24_SLICE: &[U24] = &[];
/// An empty `&ZeroSlice<U24>`
pub const EMPTY_U24: &ZeroSlice<U24> = unsafe { core::mem::transmute(EMPTY_U24_SLICE) };
