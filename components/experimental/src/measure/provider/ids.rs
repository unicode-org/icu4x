// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module temporarily saves unit IDs until they can be read directly from databake.

#[cfg(feature = "compiled_data")]
impl crate::provider::Baked {
    pub const UNIT_IDS_V1_UND_METER: u16 = 85;
    pub const UNIT_IDS_V1_UND_GRAM: u16 = 59;
    pub const UNIT_IDS_V1_UND_LITER: u16 = 82;
}
