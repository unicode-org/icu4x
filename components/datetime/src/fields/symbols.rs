// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).
use std::convert::TryFrom;

#[derive(Debug, PartialEq)]
pub enum SymbolError {
    /// Unknown field symbol
    Unknown(u8),
    /// Invalid character for a field symbol
    Invalid(char),
}

#[derive(Debug, PartialEq, Clone, Copy)]
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
    ///
    pub fn get_canonical_order(&self) -> u8 {
        match self {
            FieldSymbol::Year(Year::Calendar) => 0,
            FieldSymbol::Year(Year::WeekOf) => 1,
            FieldSymbol::Month(Month::Format) => 2,
            FieldSymbol::Month(Month::StandAlone) => 3,
            FieldSymbol::Day(Day::DayOfMonth) => 4,
            FieldSymbol::Day(Day::DayOfYear) => 5,
            FieldSymbol::Day(Day::DayOfWeekInMonth) => 6,
            FieldSymbol::Day(Day::ModifiedJulianDay) => 7,
            FieldSymbol::Weekday(Weekday::Format) => 8,
            FieldSymbol::Weekday(Weekday::Local) => 9,
            FieldSymbol::Weekday(Weekday::StandAlone) => 10,
            FieldSymbol::DayPeriod(DayPeriod::AmPm) => 11,
            FieldSymbol::DayPeriod(DayPeriod::NoonMidnight) => 12,
            FieldSymbol::Hour(Hour::H11) => 13,
            FieldSymbol::Hour(Hour::H12) => 14,
            FieldSymbol::Hour(Hour::H23) => 15,
            FieldSymbol::Hour(Hour::H24) => 16,
            FieldSymbol::Minute => 17,
            FieldSymbol::Second(Second::Second) => 18,
            FieldSymbol::Second(Second::FractionalSecond) => 19,
            FieldSymbol::Second(Second::Millisecond) => 20,
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
                .or_else(|_| Second::try_from(b).map(Self::Second)),
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
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
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
        Self::Year(input)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
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
        Self::Month(input)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
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

#[derive(Debug, PartialEq, Clone, Copy)]
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

#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
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
        Self::Second(input)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
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
        Self::Weekday(input)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum DayPeriod {
    AmPm,
    NoonMidnight,
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
