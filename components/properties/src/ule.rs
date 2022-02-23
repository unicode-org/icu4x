// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::script::ScriptWithExt;
use crate::{
    CanonicalCombiningClass, EastAsianWidth, GeneralCategory, GraphemeClusterBreak, LineBreak,
    Script, SentenceBreak, WordBreak,
};

use core::convert::TryFrom;
use zerovec::ule::{AsULE, RawBytesULE, ZeroVecError, ULE};

impl AsULE for CanonicalCombiningClass {
    type ULE = u8;

    #[inline]
    fn to_unaligned(self) -> Self::ULE {
        self.0
    }

    #[inline]
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        Self(unaligned)
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct GeneralCategoryULE(u8);

impl AsULE for GeneralCategory {
    type ULE = GeneralCategoryULE;

    #[inline]
    fn to_unaligned(self) -> Self::ULE {
        let u = self as u8;
        GeneralCategoryULE(u)
    }

    #[inline]
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        // Safe because the contents of GeneralCategoryULE are required to be valid.
        unsafe { Self::from_unchecked(unaligned.0) }
    }
}

// Safety (based on the safety checklist on the ULE trait):
//  1. GeneralCategory does not include any uninitialized or padding bytes.
//     (achieved by `#[repr(transparent)]` on a type that satisfies this invariant)
//  2. GeneralCategory is aligned to 1 byte.
//     (achieved by `#[repr(transparent)]` on a type that satisfies this invariant)
//  3. The impl of validate_byte_slice() returns an error if any byte is not valid.
//     Because GeneralCategory is repr(u8), any length of byte slice is okay.
//  4. The impl of validate_byte_slice() returns an error if there are extra bytes (impossible)
//  5. The other ULE methods use the default impl.
//  6. The PartialEq implementation on GeneralCategory uses byte equality.
unsafe impl ULE for GeneralCategoryULE {
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), ZeroVecError> {
        // Validate the bytes
        for b in bytes {
            GeneralCategory::try_from(*b).map_err(|_| ZeroVecError::parse::<Self>())?;
        }
        Ok(())
    }
}

impl AsULE for Script {
    type ULE = RawBytesULE<2>;

    #[inline]
    fn to_unaligned(self) -> Self::ULE {
        RawBytesULE(self.0.to_le_bytes())
    }

    #[inline]
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        Script(u16::from_le_bytes(unaligned.0))
    }
}

impl AsULE for ScriptWithExt {
    type ULE = RawBytesULE<2>;

    #[inline]
    fn to_unaligned(self) -> Self::ULE {
        RawBytesULE(self.0.to_le_bytes())
    }

    #[inline]
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        ScriptWithExt(u16::from_le_bytes(unaligned.0))
    }
}

impl AsULE for EastAsianWidth {
    type ULE = u8;

    #[inline]
    fn to_unaligned(self) -> Self::ULE {
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
    fn to_unaligned(self) -> Self::ULE {
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
    fn to_unaligned(self) -> Self::ULE {
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
    fn to_unaligned(self) -> Self::ULE {
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
    fn to_unaligned(self) -> Self::ULE {
        self.0
    }

    #[inline]
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        Self(unaligned)
    }
}
