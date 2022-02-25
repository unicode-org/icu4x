// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::script::ScriptWithExt;
use crate::{
    CanonicalCombiningClass, EastAsianWidth, GeneralCategory, GraphemeClusterBreak, LineBreak,
    Script, SentenceBreak, WordBreak,
};
use core::convert::TryInto;
use core::num::TryFromIntError;
use icu_codepointtrie::TrieValue;

use core::convert::TryFrom;

impl TrieValue for CanonicalCombiningClass {
    const DATA_GET_ERROR_VALUE: CanonicalCombiningClass = CanonicalCombiningClass::NotReordered;
    type TryFromU32Error = TryFromIntError;

    fn try_from_u32(i: u32) -> Result<Self, Self::TryFromU32Error> {
        u8::try_from(i).map(Self)
    }
}

impl TrieValue for GeneralCategory {
    const DATA_GET_ERROR_VALUE: GeneralCategory = GeneralCategory::Unassigned;
    type TryFromU32Error = &'static str;

    fn try_from_u32(i: u32) -> Result<Self, Self::TryFromU32Error> {
        // If the u32 is out of range, fall back to u8::MAX, which is out of range of the GeneralCategory enum.
        GeneralCategory::try_from(i.try_into().unwrap_or(u8::MAX))
            .map_err(|_| "Cannot parse GeneralCategory from integer")
    }
}

impl TrieValue for Script {
    const DATA_GET_ERROR_VALUE: Script = Script::Unknown;
    type TryFromU32Error = TryFromIntError;

    fn try_from_u32(i: u32) -> Result<Self, Self::TryFromU32Error> {
        u16::try_from(i).map(Script)
    }
}

impl TrieValue for ScriptWithExt {
    const DATA_GET_ERROR_VALUE: ScriptWithExt = ScriptWithExt::Unknown;
    type TryFromU32Error = TryFromIntError;

    fn try_from_u32(i: u32) -> Result<Self, Self::TryFromU32Error> {
        u16::try_from(i).map(Self)
    }
}

impl TrieValue for EastAsianWidth {
    const DATA_GET_ERROR_VALUE: EastAsianWidth = EastAsianWidth::Neutral;
    type TryFromU32Error = TryFromIntError;

    fn try_from_u32(i: u32) -> Result<Self, Self::TryFromU32Error> {
        u8::try_from(i).map(Self)
    }
}

impl TrieValue for LineBreak {
    const DATA_GET_ERROR_VALUE: LineBreak = LineBreak::Unknown;
    type TryFromU32Error = TryFromIntError;

    fn try_from_u32(i: u32) -> Result<Self, Self::TryFromU32Error> {
        u8::try_from(i).map(Self)
    }
}

impl TrieValue for GraphemeClusterBreak {
    const DATA_GET_ERROR_VALUE: GraphemeClusterBreak = GraphemeClusterBreak::Other;
    type TryFromU32Error = TryFromIntError;

    fn try_from_u32(i: u32) -> Result<Self, Self::TryFromU32Error> {
        u8::try_from(i).map(Self)
    }
}

impl TrieValue for WordBreak {
    const DATA_GET_ERROR_VALUE: WordBreak = WordBreak::Other;
    type TryFromU32Error = TryFromIntError;

    fn try_from_u32(i: u32) -> Result<Self, Self::TryFromU32Error> {
        u8::try_from(i).map(Self)
    }
}

impl TrieValue for SentenceBreak {
    const DATA_GET_ERROR_VALUE: SentenceBreak = SentenceBreak::Other;
    type TryFromU32Error = TryFromIntError;

    fn try_from_u32(i: u32) -> Result<Self, Self::TryFromU32Error> {
        u8::try_from(i).map(Self)
    }
}
