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

// Safety (based on the safety checklist on the ULE trait):
//  1. GeneralSubcategory does not include any uninitialized or padding bytes.
//  2. The impl of validate_byte_slice() returns an error if any byte is not valid.
//     Because GeneralSubcategory is repr(u8), any length of byte slice is okay.
//  3. The other ULE methods use the default impl.
//  4. The PartialEq implementation on GeneralSubcategory uses byte equality.
unsafe impl ULE for GeneralSubcategoryULE {
    type Error = TryFromPrimitiveError<GeneralSubcategory>;

    fn validate_byte_slice(bytes: &[u8]) -> Result<(), Self::Error> {
        // Validate the bytes
        for b in bytes {
            GeneralSubcategory::try_from(*b)?;
        }
        Ok(())
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
