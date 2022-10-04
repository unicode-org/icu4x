// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod generic;
mod ule;

use crate::fields::{Field, FieldLength, FieldSymbol};
use crate::pattern::PatternError;
use core::convert::TryFrom;
pub use generic::GenericPatternItem;

#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_datetime::pattern::item),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[allow(clippy::exhaustive_enums)] // this type is stable
pub enum PatternItem {
    Field(Field),
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
        let length = if input.0 != FieldSymbol::Second(crate::fields::Second::FractionalSecond) {
            FieldLength::from_idx(input.1).map_err(|_| PatternError::FieldLengthInvalid(input.0))?
        } else if input.1 <= 127 {
            FieldLength::from_idx(128 + input.1)
                .map_err(|_| PatternError::FieldLengthInvalid(input.0))?
        } else {
            return Err(PatternError::FieldLengthInvalid(input.0));
        };
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

        let length = if symbol != FieldSymbol::Second(crate::fields::Second::FractionalSecond) {
            FieldLength::from_idx(input.1).map_err(|_| PatternError::FieldLengthInvalid(symbol))?
        } else if input.1 <= 127 {
            FieldLength::from_idx(128 + input.1)
                .map_err(|_| PatternError::FieldLengthInvalid(symbol))?
        } else {
            return Err(PatternError::FieldLengthInvalid(symbol));
        };
        Ok(Self::Field(Field { symbol, length }))
    }
}

impl From<char> for PatternItem {
    fn from(input: char) -> Self {
        Self::Literal(input)
    }
}
