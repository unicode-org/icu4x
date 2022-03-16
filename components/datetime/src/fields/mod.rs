// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[macro_use]
mod macros;
mod length;
pub(crate) mod symbols;
mod ule;

use displaydoc::Display;
pub use length::{FieldLength, LengthError};
pub use symbols::*;

use core::{
    cmp::{Ord, PartialOrd},
    convert::TryFrom,
};

#[derive(Display, Debug, Copy, Clone)]
pub enum Error {
    #[displaydoc("Field {0:?} is not a valid length")]
    InvalidLength(FieldSymbol),
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Ord, PartialOrd)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
#[cfg_attr(feature = "serialize", derive(serde::Deserialize))]
pub struct Field {
    pub symbol: FieldSymbol,
    pub length: FieldLength,
}

impl Field {
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

    #[inline]
    pub(crate) fn bytes_in_range(symbol: &u8, length: &u8) -> bool {
        FieldSymbol::idx_in_range(symbol) && FieldLength::idx_in_range(length)
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
        let (symbol, length) = (
            input.0,
            FieldLength::from_idx(input.1 as u8)
                .map_err(|_| Self::Error::InvalidLength(input.0))?,
        );
        Ok(Self { symbol, length })
    }
}
