// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{GeneralSubcategory, Script};
use icu_codepointtrie::codepointtrie::TrieValue;
use num_enum::TryFromPrimitiveError;

use core::convert::TryFrom;

impl TrieValue for GeneralSubcategory {
    const DATA_GET_ERROR_VALUE: GeneralSubcategory = GeneralSubcategory::Unassigned;
    type Error = TryFromPrimitiveError<Self>;
    // Values from CodePointTrie data from ICU will use `u8` values, for
    // which we use the `GeneralSubcategory` enum. The values will be widened
    // to `u32` at the call site, so it is okay to truncate to convert to `u8`.
    fn parse_from_u32(i: u32) -> Result<Self, Self::Error> {
        GeneralSubcategory::try_from(i as u8)
    }
}
