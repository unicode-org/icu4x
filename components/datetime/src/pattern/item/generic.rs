// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum GenericPatternItem {
    Date,
    Time,
    Literal(char),
}

impl From<char> for GenericPatternItem {
    fn from(input: char) -> Self {
        Self::Literal(input)
    }
}
