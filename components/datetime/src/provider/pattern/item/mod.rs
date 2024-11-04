// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod generic;
mod ule;

use super::PatternError;
use crate::fields::{Field, FieldLength, FieldSymbol};
use core::convert::TryFrom;
pub use generic::GenericPatternItem;

/// An element of a [`Pattern`](super::runtime::Pattern).
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_datetime::provider::pattern::item))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[allow(clippy::exhaustive_enums)] // this type is stable
pub enum PatternItem {
    /// A field, like "abbreviated months". Mostly follows UTS 35.
    Field(Field),
    /// A literal code point.
    Literal(char),
}

impl From<(FieldSymbol, FieldLength)> for PatternItem {
    fn from(input: (FieldSymbol, FieldLength)) -> Self {
        Self::Field(Field {
            symbol: input.0,
            length: input.1,
        })
    }
}

impl TryFrom<(FieldSymbol, u8)> for PatternItem {
    type Error = PatternError;
    fn try_from(input: (FieldSymbol, u8)) -> Result<Self, Self::Error> {
        let length = FieldLength::from_idx(input.1)
            .map_err(|_| PatternError::FieldLengthInvalid(input.0))?;
        Ok(Self::Field(Field {
            symbol: input.0,
            length,
        }))
    }
}

impl TryFrom<(char, u8)> for PatternItem {
    type Error = PatternError;
    fn try_from(input: (char, u8)) -> Result<Self, Self::Error> {
        let symbol =
            FieldSymbol::try_from(input.0).map_err(|_| PatternError::InvalidSymbol(input.0))?;

        let length =
            FieldLength::from_idx(input.1).map_err(|_| PatternError::FieldLengthInvalid(symbol))?;
        Ok(Self::Field(Field { symbol, length }))
    }
}

impl From<char> for PatternItem {
    fn from(input: char) -> Self {
        Self::Literal(input)
    }
}
