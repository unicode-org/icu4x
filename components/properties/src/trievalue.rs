// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::script::ScriptWithExt;
use crate::{
    BidiClass, BidiPairedBracketType, CanonicalCombiningClass, EastAsianWidth, GeneralCategory, GraphemeClusterBreak,
    LineBreak, Script, SentenceBreak, WordBreak,
};
use core::convert::TryInto;
use core::num::TryFromIntError;
use icu_collections::codepointtrie::TrieValue;

use core::convert::TryFrom;

impl TrieValue for CanonicalCombiningClass {
    type TryFromU32Error = TryFromIntError;

    fn try_from_u32(i: u32) -> Result<Self, Self::TryFromU32Error> {
        u8::try_from(i).map(Self)
    }
}

impl TrieValue for BidiClass {
    type TryFromU32Error = TryFromIntError;

    fn try_from_u32(i: u32) -> Result<Self, Self::TryFromU32Error> {
        u8::try_from(i).map(Self)
    }
}

impl TrieValue for GeneralCategory {
    type TryFromU32Error = &'static str;

    fn try_from_u32(i: u32) -> Result<Self, Self::TryFromU32Error> {
        // If the u32 is out of range, fall back to u8::MAX, which is out of range of the GeneralCategory enum.
        GeneralCategory::new_from_u8(i.try_into().unwrap_or(u8::MAX))
            .ok_or("Cannot parse GeneralCategory from integer")
    }
}

impl TrieValue for Script {
    type TryFromU32Error = TryFromIntError;

    fn try_from_u32(i: u32) -> Result<Self, Self::TryFromU32Error> {
        u16::try_from(i).map(Script)
    }
}

impl TrieValue for ScriptWithExt {
    type TryFromU32Error = TryFromIntError;

    fn try_from_u32(i: u32) -> Result<Self, Self::TryFromU32Error> {
        u16::try_from(i).map(Self)
    }
}

impl TrieValue for EastAsianWidth {
    type TryFromU32Error = TryFromIntError;

    fn try_from_u32(i: u32) -> Result<Self, Self::TryFromU32Error> {
        u8::try_from(i).map(Self)
    }
}

impl TrieValue for LineBreak {
    type TryFromU32Error = TryFromIntError;

    fn try_from_u32(i: u32) -> Result<Self, Self::TryFromU32Error> {
        u8::try_from(i).map(Self)
    }
}

impl TrieValue for GraphemeClusterBreak {
    type TryFromU32Error = TryFromIntError;

    fn try_from_u32(i: u32) -> Result<Self, Self::TryFromU32Error> {
        u8::try_from(i).map(Self)
    }
}

impl TrieValue for WordBreak {
    type TryFromU32Error = TryFromIntError;

    fn try_from_u32(i: u32) -> Result<Self, Self::TryFromU32Error> {
        u8::try_from(i).map(Self)
    }
}

impl TrieValue for SentenceBreak {
    type TryFromU32Error = TryFromIntError;

    fn try_from_u32(i: u32) -> Result<Self, Self::TryFromU32Error> {
        u8::try_from(i).map(Self)
    }
}

impl TrieValue for BidiPairedBracketType {
    type TryFromU32Error = TryFromIntError;

    fn try_from_u32(i: u32) -> Result<Self, Self::TryFromU32Error> {
        u8::try_from(i).map(Self)
    }
}
