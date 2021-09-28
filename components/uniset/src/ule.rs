// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::enum_props::{GeneralSubcategory, Script};
use core::convert::TryFrom;
use num_enum::TryFromPrimitiveError;
use zerovec::ule::{AsULE, PlainOldULE, ULE};

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct GeneralSubcategoryULE(u8);

impl AsULE for GeneralSubcategory {
    type ULE = GeneralSubcategoryULE;

    #[inline]
    fn as_unaligned(&self) -> Self::ULE {
        let u = *self as u8;
        GeneralSubcategoryULE(u)
    }

    #[inline]
    fn from_unaligned(unaligned: &Self::ULE) -> Self {
        // Safe because the contents of GeneralSubcategoryULE are required to be valid.
        unsafe { Self::from_unchecked(unaligned.0) }
    }
}

unsafe impl ULE for GeneralSubcategoryULE {
    type Error = TryFromPrimitiveError<GeneralSubcategory>;

    fn validate_byte_slice(bytes: &[u8]) -> Result<(), Self::Error> {
        // Validate the bytes
        for b in bytes {
            GeneralSubcategory::try_from(*b)?;
        }
        Ok(())
    }

    /// Invariant: must be safe to call when called on a slice that previously
    /// succeeded with `parse_byte_slice`
    unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &[Self] {
        let data = bytes.as_ptr() as *const Self;
        let len = bytes.len();
        core::slice::from_raw_parts(data, len)
    }
    fn as_byte_slice(slice: &[Self]) -> &[u8] {
        let data = slice.as_ptr() as *const u8;
        let len = slice.len();
        // Safe because Self is transparent over u8
        unsafe { core::slice::from_raw_parts(data, len) }
    }
}

impl AsULE for Script {
    type ULE = PlainOldULE<2>;

    #[inline]
    fn as_unaligned(&self) -> Self::ULE {
        PlainOldULE(self.0.to_le_bytes())
    }

    #[inline]
    fn from_unaligned(unaligned: &Self::ULE) -> Self {
        Script(u16::from_le_bytes(unaligned.0))
    }
}
