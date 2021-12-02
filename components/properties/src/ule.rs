// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{
    EastAsianWidth, GeneralSubcategory, GraphemeClusterBreak, LineBreak, Script, ScriptWithExt,
    SentenceBreak, WordBreak,
};

use core::convert::TryFrom;
use num_enum::TryFromPrimitiveError;
use zerovec::ule::{AsULE, PlainOldULE, ULE};

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct GeneralSubcategoryULE(u8);

impl AsULE for GeneralSubcategory {
    type ULE = GeneralSubcategoryULE;

    #[inline]
    fn as_unaligned(self) -> Self::ULE {
        let u = self as u8;
        GeneralSubcategoryULE(u)
    }

    #[inline]
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        // Safe because the contents of GeneralSubcategoryULE are required to be valid.
        unsafe { Self::from_unchecked(unaligned.0) }
    }
}

// Safety (based on the safety checklist on the ULE trait):
//  1. GeneralSubcategory does not include any uninitialized or padding bytes.
//     (achieved by `#[repr(transparent)]` on a type that satisfies this invariant)
//  2. GeneralSubcategory is aligned to 1 byte.
//     (achieved by `#[repr(transparent)]` on a type that satisfies this invariant)
//  3. The impl of validate_byte_slice() returns an error if any byte is not valid.
//     Because GeneralSubcategory is repr(u8), any length of byte slice is okay.
//  4. The impl of validate_byte_slice() returns an error if there are extra bytes (impossible)
//  5. The other ULE methods use the default impl.
//  6. The PartialEq implementation on GeneralSubcategory uses byte equality.
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


// TODO: if useful, add a method to VarZeroVec<'a, [<T as AsULE>::ULE]> that returns a T
// for an index, perhaps like
// ```
// fn get_as_ule(&self, idx: usize) -> Option<T> {
//     self.get(idx).map(|ule| T::from_unaligned(ule))
// }
// ```


impl AsULE for Script {
    type ULE = PlainOldULE<2>;

    #[inline]
    fn as_unaligned(self) -> Self::ULE {
        PlainOldULE(self.0.to_le_bytes())
    }

    #[inline]
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        Script(u16::from_le_bytes(unaligned.0))
    }
}



impl AsULE for ScriptWithExt {
    type ULE = PlainOldULE<2>;

    #[inline]
    fn as_unaligned(self) -> Self::ULE {
        PlainOldULE(self.0.to_le_bytes())
    }

    #[inline]
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        ScriptWithExt(u16::from_le_bytes(unaligned.0))
    }
}

impl AsULE for EastAsianWidth {
    type ULE = u8;

    #[inline]
    fn as_unaligned(self) -> Self::ULE {
        self.0
    }

    #[inline]
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        Self(unaligned)
    }
}

impl AsULE for LineBreak {
    type ULE = u8;

    #[inline]
    fn as_unaligned(self) -> Self::ULE {
        self.0
    }

    #[inline]
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        Self(unaligned)
    }
}

impl AsULE for GraphemeClusterBreak {
    type ULE = u8;

    #[inline]
    fn as_unaligned(self) -> Self::ULE {
        self.0
    }

    #[inline]
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        Self(unaligned)
    }
}

impl AsULE for WordBreak {
    type ULE = u8;

    #[inline]
    fn as_unaligned(self) -> Self::ULE {
        self.0
    }

    #[inline]
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        Self(unaligned)
    }
}

impl AsULE for SentenceBreak {
    type ULE = u8;

    #[inline]
    fn as_unaligned(self) -> Self::ULE {
        self.0
    }

    #[inline]
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        Self(unaligned)
    }
}
