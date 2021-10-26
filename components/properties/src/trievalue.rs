// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{GeneralSubcategory, Script};
use core::convert::TryInto;
use core::num::TryFromIntError;
use icu_codepointtrie::codepointtrie::TrieValue;
use num_enum::TryFromPrimitiveError;

use core::convert::TryFrom;

impl TrieValue for GeneralSubcategory {
    const DATA_GET_ERROR_VALUE: GeneralSubcategory = GeneralSubcategory::Unassigned;
    type TryFromU32Error = TryFromPrimitiveError<Self>;
    fn try_from_u32(i: u32) -> Result<Self, Self::TryFromU32Error> {
        // If the u32 is out of range, fall back to u8::MAX, which is out of range of the GeneralSubcategory enum.
        GeneralSubcategory::try_from(i.try_into().unwrap_or(u8::MAX))
    }
}

impl TrieValue for Script {
    const DATA_GET_ERROR_VALUE: Script = Script::Unknown;
    type TryFromU32Error = TryFromIntError;

    fn try_from_u32(i: u32) -> Result<Self, Self::TryFromU32Error> {
        u16::try_from(i).map(Script)
    }
}
