// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use std::convert::TryFrom;

#[derive(Debug)]
pub enum SymbolError {
    Unknown(u8),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FieldSymbol {
    Year(Year),
    Month(Month),
    Day(Day),
    Weekday(Weekday),
    DayPeriod(DayPeriod),
    Hour(Hour),
    Minute,
    Second(Second),
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
                .or_else(|_| Second::try_from(b).map(Self::Second)),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Year {
    Calendar,
    WeekOf,
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
        FieldSymbol::Year(input)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Month {
    Format,
    StandAlone,
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
        FieldSymbol::Month(input)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Day {
    DayOfMonth,
    DayOfYear,
    DayOfWeekInMonth,
    ModifiedJulianDay,
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
        FieldSymbol::Day(input)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Hour {
    H11,
    H12,
    H23,
    H24,
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
        FieldSymbol::Hour(input)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Second {
    Second,
    FractionalSecond,
    Millisecond,
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
        FieldSymbol::Second(input)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Weekday {
    Format,
    Local,
    StandAlone,
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
        FieldSymbol::Weekday(input)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DayPeriod {
    AmPm,
}

impl TryFrom<u8> for DayPeriod {
    type Error = SymbolError;
    fn try_from(b: u8) -> Result<Self, Self::Error> {
        match b {
            b'a' => Ok(Self::AmPm),
            b => Err(SymbolError::Unknown(b)),
        }
    }
}

impl From<DayPeriod> for FieldSymbol {
    fn from(input: DayPeriod) -> Self {
        FieldSymbol::DayPeriod(input)
    }
}
