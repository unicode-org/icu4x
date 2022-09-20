// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::convert::TryFrom;

use crate::fields::{Field, FieldLength, FieldSymbol};
use crate::pattern::PatternError;

use super::{GenericPatternItem, PatternItem};

#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[allow(clippy::exhaustive_enums)]
pub enum MixedPatternItem {
    Field(Field),
    // A single digit, 0..=9
    Placeholder(u8),
    Literal(char),
}

impl TryFrom<MixedPatternItem> for PatternItem {
    type Error = PatternError;

    fn try_from(value: MixedPatternItem) -> Result<Self, Self::Error> {
        match value {
            MixedPatternItem::Field(f) => Ok(Self::Field(f)),
            MixedPatternItem::Literal(l) => Ok(Self::Literal(l)),
            MixedPatternItem::Placeholder(_) => Err(PatternError::UnsupportedPlaceholder),
        }
    }
}

impl TryFrom<MixedPatternItem> for GenericPatternItem {
    type Error = PatternError;

    fn try_from(value: MixedPatternItem) -> Result<Self, Self::Error> {
        match value {
            MixedPatternItem::Placeholder(p) => Ok(Self::Placeholder(p)),
            MixedPatternItem::Literal(l) => Ok(Self::Literal(l)),
            MixedPatternItem::Field(_) => Err(PatternError::UnsupportedFields),
        }
    }
}

impl From<PatternItem> for MixedPatternItem {
    fn from(item: PatternItem) -> Self {
        match item {
            PatternItem::Field(f) => Self::Field(f),
            PatternItem::Literal(l) => Self::Literal(l),
        }
    }
}

impl From<GenericPatternItem> for MixedPatternItem {
    fn from(item: GenericPatternItem) -> Self {
        match item {
            GenericPatternItem::Placeholder(p) => Self::Placeholder(p),
            GenericPatternItem::Literal(l) => Self::Literal(l),
        }
    }
}

impl From<(FieldSymbol, FieldLength)> for MixedPatternItem {
    fn from(input: (FieldSymbol, FieldLength)) -> Self {
        Self::from(PatternItem::from(input))
    }
}

impl TryFrom<(FieldSymbol, u8)> for MixedPatternItem {
    type Error = PatternError;
    fn try_from(input: (FieldSymbol, u8)) -> Result<Self, Self::Error> {
        PatternItem::try_from(input).map(Self::from)
    }
}

impl TryFrom<(char, u8)> for MixedPatternItem {
    type Error = PatternError;
    fn try_from(input: (char, u8)) -> Result<Self, Self::Error> {
        PatternItem::try_from(input).map(Self::from)
    }
}

impl From<char> for MixedPatternItem {
    fn from(input: char) -> Self {
        Self::Literal(input)
    }
}

impl From<u8> for MixedPatternItem {
    fn from(input: u8) -> Self {
        Self::Placeholder(input)
    }
}
