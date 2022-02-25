// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::script::ScriptWithExt;
use crate::{
    CanonicalCombiningClass, EastAsianWidth, GraphemeClusterBreak, LineBreak, Script,
    SentenceBreak, WordBreak,
};

use zerovec::ule::{AsULE, RawBytesULE};

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
