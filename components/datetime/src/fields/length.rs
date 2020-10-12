// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use std::convert::TryFrom;

#[derive(Debug)]
pub enum LengthError {
    TooLong,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FieldLength {
    One = 1,
    TwoDigit = 2,
    Abbreviated = 3,
    Wide = 4,
    Narrow = 5,
    Six = 6,
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
                    _ => return Err(LengthError::TooLong),
                })
            }
        }
    };
}

try_field_length!(u8);
try_field_length!(usize);
