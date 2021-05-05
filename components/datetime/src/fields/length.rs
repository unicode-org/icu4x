// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::{
    cmp::{Ord, PartialOrd},
    convert::TryFrom,
};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum LengthError {
    #[error("Invalid length")]
    InvalidLength,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Ord, PartialOrd)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum FieldLength {
    One = 1,
    TwoDigit = 2,
    Abbreviated = 3,
    Wide = 4,
    Narrow = 5,
    Six = 6,
}

impl From<FieldLength> for u8 {
    fn from(length: FieldLength) -> Self {
        match length {
            FieldLength::One => 1,
            FieldLength::TwoDigit => 2,
            FieldLength::Abbreviated => 3,
            FieldLength::Wide => 4,
            FieldLength::Narrow => 5,
            FieldLength::Six => 6,
        }
    }
}

macro_rules! try_field_length {
    ($i:ty) => {
        impl TryFrom<$i> for FieldLength {
            type Error = LengthError;

            fn try_from(input: $i) -> Result<Self, Self::Error> {
                Ok(match input {
                    1 => Self::One,
                    2 => Self::TwoDigit,
                    3 => Self::Abbreviated,
                    4 => Self::Wide,
                    5 => Self::Narrow,
                    6 => Self::Six,
                    _ => return Err(LengthError::InvalidLength),
                })
            }
        }
    };
}

try_field_length!(u8);
try_field_length!(usize);
