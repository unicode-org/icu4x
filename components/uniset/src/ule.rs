// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::convert::TryFrom;
use crate::enum_props::GeneralSubcategory;
use num_enum::TryFromPrimitiveError;
use zerovec::ule::{AsULE, ULE};

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct GeneralSubcategoryULE([u8; 4]);

impl AsULE for GeneralSubcategory {
    type ULE = GeneralSubcategoryULE;

    #[inline]
    fn as_unaligned(&self) -> Self::ULE {
        let u = *self as u32;
        GeneralSubcategoryULE(u.to_le_bytes())
    }

    #[inline]
    fn from_unaligned(unaligned: &Self::ULE) -> Self {
        let u = u32::from_le_bytes(unaligned.0);
        // Safe because the bytes of GeneralSubcategoryULE are required to be valid.
        unsafe { Self::from_unchecked(u) }
    }
}

unsafe impl ULE for GeneralSubcategoryULE {
    type Error = TryFromPrimitiveError<GeneralSubcategory>;

    fn parse_byte_slice(bytes: &[u8]) -> Result<&[Self], Self::Error> {
        // Validate the bytes
        for chunk in bytes.chunks_exact(4) {
            // TODO: Use slice::as_chunks() when stabilized
            let u = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
            GeneralSubcategory::try_from(u)?;
        }
        // Safe because Self is transparent over [u8; 4] and has been validated
        Ok(unsafe { Self::from_byte_slice_unchecked(bytes) })
    }

    /// Invariant: must be safe to call when called on a slice that previously
    /// succeeded with `parse_byte_slice`
    unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &[Self] {
        let data = bytes.as_ptr() as *const Self;
        let len = bytes.len() / 4;
        core::slice::from_raw_parts(data, len)
    }
    fn as_byte_slice(slice: &[Self]) -> &[u8] {
        let data = slice.as_ptr() as *const u8;
        let len = slice.len() * 4;
        // Safe because Self is transparent over [u8; 4]
        unsafe { core::slice::from_raw_parts(data, len) }
    }
}
