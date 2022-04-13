// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub enum GenericPatternItem {
    Placeholder(u8),
    Literal(char),
}

impl From<u8> for GenericPatternItem {
    fn from(input: u8) -> Self {
        Self::Placeholder(input)
    }
}

impl From<char> for GenericPatternItem {
    fn from(input: char) -> Self {
        Self::Literal(input)
    }
}
