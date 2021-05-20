// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::fields::FieldLength;
use std::{cmp::Ordering, convert::TryFrom};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum SymbolError {
    /// Unknown field symbol.
    #[error("Unknown field symbol: {0}")]
    Unknown(u8),
    /// Invalid character for a field symbol.
    #[error("Invalid character for a field symbol: {0}")]
    Invalid(char),
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum FieldSymbol {
    Year(Year),
    Month(Month),
    Day(Day),
    Weekday(Weekday),
    DayPeriod(DayPeriod),
    Hour(Hour),
    Minute,
    Second(Second),
    TimeZone(TimeZone),
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum TextOrNumeric {
    Text,
    Numeric,
}

/// [`FieldSymbols`](FieldSymbol) can be either text or numeric. This categorization is important
/// when matching skeletons with a components [`Bag`](crate::options::components::Bag).
pub trait LengthType {
    fn get_length_type(&self, length: FieldLength) -> TextOrNumeric;
}

impl FieldSymbol {
    /// Skeletons are a Vec<Field>, and represent the Fields that can be used to match to a
    /// specific pattern. The order of the Vec does not affect the Pattern that is output.
    /// However, it's more performant when matching these fields, and it's more deterministic
    /// when serializing them to present them in a consistent order.
    ///
    /// This ordering is taken by the order of the fields listed in the [UTS 35 Date Field Symbol Table]
    /// (https://unicode.org/reports/tr35/tr35-dates.html#Date_Field_Symbol_Table), and are generally
    /// ordered most significant to least significant.
    fn get_canonical_order(&self) -> u8 {
        match self {
            Self::Year(Year::Calendar) => 0,
            Self::Year(Year::WeekOf) => 1,
            Self::Month(Month::Format) => 2,
            Self::Month(Month::StandAlone) => 3,
            Self::Day(Day::DayOfMonth) => 4,
            Self::Day(Day::DayOfYear) => 5,
            Self::Day(Day::DayOfWeekInMonth) => 6,
            Self::Day(Day::ModifiedJulianDay) => 7,
            Self::Weekday(Weekday::Format) => 8,
            Self::Weekday(Weekday::Local) => 9,
            Self::Weekday(Weekday::StandAlone) => 10,
            Self::DayPeriod(DayPeriod::AmPm) => 11,
            Self::DayPeriod(DayPeriod::NoonMidnight) => 12,
            Self::Hour(Hour::H11) => 13,
            Self::Hour(Hour::H12) => 14,
            Self::Hour(Hour::H23) => 15,
            Self::Hour(Hour::H24) => 16,
            Self::Minute => 17,
            Self::Second(Second::Second) => 18,
            Self::Second(Second::FractionalSecond) => 19,
            Self::Second(Second::Millisecond) => 20,
            Self::TimeZone(TimeZone::LowerZ) => 21,
            Self::TimeZone(TimeZone::UpperZ) => 22,
            Self::TimeZone(TimeZone::UpperO) => 23,
            Self::TimeZone(TimeZone::LowerV) => 24,
            Self::TimeZone(TimeZone::UpperV) => 25,
            Self::TimeZone(TimeZone::LowerX) => 26,
            Self::TimeZone(TimeZone::UpperX) => 27,
        }
    }
}

impl TryFrom<u8> for FieldSymbol {
    type Error = SymbolError;
    fn try_from(b: u8) -> Result<Self, Self::Error> {
        match b {
            b'm' => Ok(Self::Minute),
            _ => Year::try_from(b)
                .map(Self::Year)
                .or_else(|_| Month::try_from(b).map(Self::Month))
                .or_else(|_| Day::try_from(b).map(Self::Day))
                .or_else(|_| Weekday::try_from(b).map(Self::Weekday))
                .or_else(|_| DayPeriod::try_from(b).map(Self::DayPeriod))
                .or_else(|_| Hour::try_from(b).map(Self::Hour))
                .or_else(|_| Second::try_from(b).map(Self::Second))
                .or_else(|_| TimeZone::try_from(b).map(Self::TimeZone)),
        }
    }
}

impl TryFrom<char> for FieldSymbol {
    type Error = SymbolError;

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        if ch.is_ascii() {
            Self::try_from(ch as u8)
        } else {
            Err(SymbolError::Invalid(ch))
        }
    }
}

impl From<FieldSymbol> for char {
    fn from(symbol: FieldSymbol) -> Self {
        match symbol {
            FieldSymbol::Year(year) => match year {
                Year::Calendar => 'y',
                Year::WeekOf => 'Y',
            },
            FieldSymbol::Month(month) => match month {
                Month::Format => 'M',
                Month::StandAlone => 'L',
            },
            FieldSymbol::Day(day) => match day {
                Day::DayOfMonth => 'd',
                Day::DayOfYear => 'D',
                Day::DayOfWeekInMonth => 'F',
                Day::ModifiedJulianDay => 'g',
            },
            FieldSymbol::Weekday(weekday) => match weekday {
                Weekday::Format => 'E',
                Weekday::Local => 'e',
                Weekday::StandAlone => 'c',
            },
            FieldSymbol::DayPeriod(dayperiod) => match dayperiod {
                DayPeriod::AmPm => 'a',
                DayPeriod::NoonMidnight => 'b',
            },
            FieldSymbol::Hour(hour) => match hour {
                Hour::H11 => 'K',
                Hour::H12 => 'h',
                Hour::H23 => 'H',
                Hour::H24 => 'k',
            },
            FieldSymbol::Minute => 'm',
            FieldSymbol::Second(second) => match second {
                Second::Second => 's',
                Second::FractionalSecond => 'S',
                Second::Millisecond => 'A',
            },
            FieldSymbol::TimeZone(time_zone) => match time_zone {
                TimeZone::LowerZ => 'z',
                TimeZone::UpperZ => 'Z',
                TimeZone::UpperO => 'O',
                TimeZone::LowerV => 'v',
                TimeZone::UpperV => 'V',
                TimeZone::LowerX => 'x',
                TimeZone::UpperX => 'X',
            },
        }
    }
}

impl PartialOrd for FieldSymbol {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FieldSymbol {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_canonical_order().cmp(&other.get_canonical_order())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum Year {
    Calendar,
    WeekOf,
}

impl LengthType for Year {
    fn get_length_type(&self, _length: FieldLength) -> TextOrNumeric {
        TextOrNumeric::Numeric
    }
}

impl TryFrom<u8> for Year {
    type Error = SymbolError;
    fn try_from(b: u8) -> Result<Self, Self::Error> {
        match b {
            b'y' => Ok(Self::Calendar),
            b'Y' => Ok(Self::WeekOf),
            b => Err(SymbolError::Unknown(b)),
        }
    }
}

impl From<Year> for FieldSymbol {
    fn from(input: Year) -> Self {
        Self::Year(input)
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum Month {
    Format,
    StandAlone,
}

impl LengthType for Month {
    fn get_length_type(&self, length: FieldLength) -> TextOrNumeric {
        match length {
            FieldLength::One => TextOrNumeric::Numeric,
            FieldLength::TwoDigit => TextOrNumeric::Numeric,
            FieldLength::Abbreviated => TextOrNumeric::Text,
            FieldLength::Wide => TextOrNumeric::Text,
            FieldLength::Narrow => TextOrNumeric::Text,
            FieldLength::Six => TextOrNumeric::Text,
        }
    }
}

impl TryFrom<u8> for Month {
    type Error = SymbolError;
    fn try_from(b: u8) -> Result<Self, Self::Error> {
        match b {
            b'M' => Ok(Self::Format),
            b'L' => Ok(Self::StandAlone),
            b => Err(SymbolError::Unknown(b)),
        }
    }
}

impl From<Month> for FieldSymbol {
    fn from(input: Month) -> Self {
        Self::Month(input)
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum Day {
    DayOfMonth,
    DayOfYear,
    DayOfWeekInMonth,
    ModifiedJulianDay,
}

impl LengthType for Day {
    fn get_length_type(&self, _length: FieldLength) -> TextOrNumeric {
        TextOrNumeric::Numeric
    }
}

impl TryFrom<u8> for Day {
    type Error = SymbolError;
    fn try_from(b: u8) -> Result<Self, Self::Error> {
        match b {
            b'd' => Ok(Self::DayOfMonth),
            b'D' => Ok(Self::DayOfYear),
            b'F' => Ok(Self::DayOfWeekInMonth),
            b'g' => Ok(Self::ModifiedJulianDay),
            b => Err(SymbolError::Unknown(b)),
        }
    }
}

impl From<Day> for FieldSymbol {
    fn from(input: Day) -> Self {
        Self::Day(input)
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum Hour {
    H11,
    H12,
    H23,
    H24,
}

impl LengthType for Hour {
    fn get_length_type(&self, _length: FieldLength) -> TextOrNumeric {
        TextOrNumeric::Numeric
    }
}

impl TryFrom<u8> for Hour {
    type Error = SymbolError;
    fn try_from(b: u8) -> Result<Self, Self::Error> {
        match b {
            b'K' => Ok(Self::H11),
            b'h' => Ok(Self::H12),
            b'H' => Ok(Self::H23),
            b'k' => Ok(Self::H24),
            b => Err(SymbolError::Unknown(b)),
        }
    }
}

impl From<Hour> for FieldSymbol {
    fn from(input: Hour) -> Self {
        Self::Hour(input)
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum Second {
    Second,
    FractionalSecond,
    Millisecond,
}

impl LengthType for Second {
    fn get_length_type(&self, _length: FieldLength) -> TextOrNumeric {
        TextOrNumeric::Numeric
    }
}

impl TryFrom<u8> for Second {
    type Error = SymbolError;
    fn try_from(b: u8) -> Result<Self, Self::Error> {
        match b {
            b's' => Ok(Self::Second),
            b'S' => Ok(Self::FractionalSecond),
            b'A' => Ok(Self::Millisecond),
            b => Err(SymbolError::Unknown(b)),
        }
    }
}

impl From<Second> for FieldSymbol {
    fn from(input: Second) -> Self {
        Self::Second(input)
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum Weekday {
    Format,
    Local,
    StandAlone,
}

impl LengthType for Weekday {
    fn get_length_type(&self, length: FieldLength) -> TextOrNumeric {
        match self {
            Self::Format => TextOrNumeric::Text,
            Self::Local | Self::StandAlone => match length {
                FieldLength::One | FieldLength::TwoDigit => TextOrNumeric::Text,
                _ => TextOrNumeric::Numeric,
            },
        }
    }
}

impl TryFrom<u8> for Weekday {
    type Error = SymbolError;
    fn try_from(b: u8) -> Result<Self, Self::Error> {
        match b {
            b'E' => Ok(Self::Format),
            b'e' => Ok(Self::Local),
            b'c' => Ok(Self::StandAlone),
            b => Err(SymbolError::Unknown(b)),
        }
    }
}

impl From<Weekday> for FieldSymbol {
    fn from(input: Weekday) -> Self {
        Self::Weekday(input)
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum DayPeriod {
    AmPm,
    NoonMidnight,
}

impl LengthType for DayPeriod {
    fn get_length_type(&self, _length: FieldLength) -> TextOrNumeric {
        TextOrNumeric::Text
    }
}

impl TryFrom<u8> for DayPeriod {
    type Error = SymbolError;
    fn try_from(b: u8) -> Result<Self, Self::Error> {
        match b {
            b'a' => Ok(Self::AmPm),
            b'b' => Ok(Self::NoonMidnight),
            b => Err(SymbolError::Unknown(b)),
        }
    }
}

impl From<DayPeriod> for FieldSymbol {
    fn from(input: DayPeriod) -> Self {
        Self::DayPeriod(input)
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum TimeZone {
    LowerZ,
    UpperZ,
    UpperO,
    LowerV,
    UpperV,
    LowerX,
    UpperX,
}

impl LengthType for TimeZone {
    fn get_length_type(&self, length: FieldLength) -> TextOrNumeric {
        use TextOrNumeric::*;
        match self {
            // It is reasonable to default to Text on release builds instead of panicking.
            //
            // Erroneous symbols are gracefully handled by returning error Results
            // in the formatting code.
            //
            // The default cases may want to be updated to return errors themselves
            // if the skeleton matching code ever becomes fallible.
            Self::UpperZ => match u8::from(length) {
                1..=3 => Numeric,
                4 => Text,
                5 => Numeric,
                _ => Text,
            },
            Self::UpperO => match u8::from(length) {
                1 => Text,
                4 => Numeric,
                _ => Text,
            },
            Self::LowerX | Self::UpperX => Numeric,
            Self::LowerZ | Self::LowerV | Self::UpperV => Text,
        }
    }
}

impl TryFrom<u8> for TimeZone {
    type Error = SymbolError;
    fn try_from(b: u8) -> Result<Self, Self::Error> {
        match b {
            b'z' => Ok(Self::LowerZ),
            b'Z' => Ok(Self::UpperZ),
            b'O' => Ok(Self::UpperO),
            b'v' => Ok(Self::LowerV),
            b'V' => Ok(Self::UpperV),
            b'x' => Ok(Self::LowerX),
            b'X' => Ok(Self::UpperX),
            b => Err(SymbolError::Unknown(b)),
        }
    }
}

impl From<TimeZone> for FieldSymbol {
    fn from(input: TimeZone) -> Self {
        Self::TimeZone(input)
    }
}
