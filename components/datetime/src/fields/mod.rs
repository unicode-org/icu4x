// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Enums representing the fields in a date pattern, including the field's type, length and symbol.

#![allow(clippy::exhaustive_structs)] // Field and FieldULE part of data struct

mod length;
pub(crate) mod symbols;

use displaydoc::Display;
pub use length::{FieldLength, LengthError};
pub use symbols::*;

use core::{
    cmp::{Ord, PartialOrd},
    convert::TryFrom,
};

/// An error relating to the field for a date pattern field as a whole.
///
/// Separate error types exist for parts of a field, like the
/// [`LengthError`](error for the field length) and the
/// [`SymbolError`](error for the field symbol).
#[derive(Display, Debug, Copy, Clone, PartialEq)]
#[non_exhaustive]
pub enum Error {
    /// An error originating inside of the [data provider](icu_provider).
    #[displaydoc("Field {0:?} is not a valid length")]
    InvalidLength(FieldSymbol),
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

/// A field within a date pattern string, also referred to as a date field. A date field is the
/// repetition of a specific pattern character one or more times within the pattern string.
/// The pattern character is known as the field symbol, which indicates the particular meaning for the field.
#[derive(Debug, Eq, PartialEq, Clone, Copy, Ord, PartialOrd)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_datetime::fields),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[zerovec::make_ule(FieldULE)]
pub struct Field {
    /// The field symbol for the `Field`, which corresponds to the field's meaning with the
    /// date pattern.
    pub symbol: FieldSymbol,
    /// The length of the `Field`, which in conjunction with the `FieldSymbol` informs the width or
    /// style of the formatting output corresponding to this field.
    pub length: FieldLength,
}

impl Field {
    #[cfg(any(feature = "datagen", feature = "experimental"))] // only referenced in experimental code
    pub(crate) fn get_length_type(&self) -> TextOrNumeric {
        match self.symbol {
            FieldSymbol::Era => TextOrNumeric::Text,
            FieldSymbol::Year(year) => year.get_length_type(self.length),
            FieldSymbol::Month(month) => month.get_length_type(self.length),
            FieldSymbol::Week(week) => week.get_length_type(self.length),
            FieldSymbol::Day(day) => day.get_length_type(self.length),
            FieldSymbol::Weekday(weekday) => weekday.get_length_type(self.length),
            FieldSymbol::DayPeriod(day_period) => day_period.get_length_type(self.length),
            FieldSymbol::Hour(hour) => hour.get_length_type(self.length),
            FieldSymbol::Minute => TextOrNumeric::Numeric,
            FieldSymbol::Second(second) => second.get_length_type(self.length),
            FieldSymbol::TimeZone(zone) => zone.get_length_type(self.length),
        }
    }
}

impl FieldULE {
    #[inline]
    pub(crate) fn validate_bytes(bytes: (u8, u8)) -> Result<(), zerovec::ZeroVecError> {
        symbols::FieldSymbolULE::validate_byte(bytes.0)?;
        length::FieldLengthULE::validate_byte(bytes.1)?;
        Ok(())
    }
}

impl From<(FieldSymbol, FieldLength)> for Field {
    fn from(input: (FieldSymbol, FieldLength)) -> Self {
        Self {
            symbol: input.0,
            length: input.1,
        }
    }
}

impl TryFrom<(FieldSymbol, usize)> for Field {
    type Error = Error;
    fn try_from(input: (FieldSymbol, usize)) -> Result<Self, Self::Error> {
        let length = if input.0 != FieldSymbol::Second(crate::fields::Second::FractionalSecond) {
            FieldLength::from_idx(input.1 as u8).map_err(|_| Self::Error::InvalidLength(input.0))?
        } else if input.1 <= 127 {
            FieldLength::from_idx(128 + input.1 as u8)
                .map_err(|_| Self::Error::InvalidLength(input.0))?
        } else {
            return Err(Self::Error::InvalidLength(input.0));
        };
        Ok(Self {
            symbol: input.0,
            length,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::fields::{Field, FieldLength, FieldSymbol, Second, Year};
    use zerovec::ule::{AsULE, ULE};

    #[test]
    fn test_field_as_ule() {
        let samples = [
            (
                Field::from((FieldSymbol::Minute, FieldLength::TwoDigit)),
                [FieldSymbol::Minute.idx(), FieldLength::TwoDigit.idx()],
            ),
            (
                Field::from((FieldSymbol::Year(Year::Calendar), FieldLength::Wide)),
                [
                    FieldSymbol::Year(Year::Calendar).idx(),
                    FieldLength::Wide.idx(),
                ],
            ),
            (
                Field::from((FieldSymbol::Year(Year::WeekOf), FieldLength::Wide)),
                [
                    FieldSymbol::Year(Year::WeekOf).idx(),
                    FieldLength::Wide.idx(),
                ],
            ),
            (
                Field::from((FieldSymbol::Second(Second::Millisecond), FieldLength::One)),
                [
                    FieldSymbol::Second(Second::Millisecond).idx(),
                    FieldLength::One.idx(),
                ],
            ),
        ];

        for (ref_field, ref_bytes) in samples {
            let ule = ref_field.to_unaligned();
            assert_eq!(ULE::as_byte_slice(&[ule]), ref_bytes);
            let field = Field::from_unaligned(ule);
            assert_eq!(field, ref_field);
        }
    }

    #[test]
    fn test_field_ule() {
        let samples = [(
            [
                Field::from((FieldSymbol::Year(Year::Calendar), FieldLength::Wide)),
                Field::from((FieldSymbol::Second(Second::Millisecond), FieldLength::One)),
            ],
            [
                [
                    FieldSymbol::Year(Year::Calendar).idx(),
                    FieldLength::Wide.idx(),
                ],
                [
                    FieldSymbol::Second(Second::Millisecond).idx(),
                    FieldLength::One.idx(),
                ],
            ],
        )];

        for (ref_field, ref_bytes) in samples {
            let mut bytes: Vec<u8> = vec![];
            for item in ref_field.iter() {
                let ule = item.to_unaligned();
                bytes.extend(ULE::as_byte_slice(&[ule]));
            }

            let mut bytes2: Vec<u8> = vec![];
            for seq in ref_bytes.iter() {
                bytes2.extend_from_slice(seq);
            }

            assert!(FieldULE::validate_byte_slice(&bytes).is_ok());
            assert_eq!(bytes, bytes2);
        }
    }
}
