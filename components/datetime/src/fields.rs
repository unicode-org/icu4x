use std::convert::TryFrom;

#[derive(Debug)]
pub enum Error {
    Unknown,
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

impl FieldLength {
    pub fn try_from(idx: usize) -> Result<Self, Error> {
        Ok(match idx {
            1 => Self::One,
            2 => Self::TwoDigit,
            3 => Self::Abbreviated,
            4 => Self::Wide,
            5 => Self::Narrow,
            6 => Self::Six,
            _ => return Err(Error::TooLong),
        })
    }
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
    type Error = Error;
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
    type Error = Error;
    fn try_from(b: u8) -> Result<Self, Self::Error> {
        match b {
            b'y' => Ok(Self::Calendar),
            b'Y' => Ok(Self::WeekOf),
            _ => Err(Error::Unknown),
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
    type Error = Error;
    fn try_from(b: u8) -> Result<Self, Self::Error> {
        match b {
            b'M' => Ok(Self::Format),
            b'L' => Ok(Self::StandAlone),
            _ => Err(Error::Unknown),
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
    type Error = Error;
    fn try_from(b: u8) -> Result<Self, Self::Error> {
        match b {
            b'd' => Ok(Self::DayOfMonth),
            b'D' => Ok(Self::DayOfYear),
            b'F' => Ok(Self::DayOfWeekInMonth),
            b'g' => Ok(Self::ModifiedJulianDay),
            _ => Err(Error::Unknown),
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
    type Error = Error;
    fn try_from(b: u8) -> Result<Self, Self::Error> {
        match b {
            b'K' => Ok(Self::H11),
            b'h' => Ok(Self::H12),
            b'H' => Ok(Self::H23),
            b'k' => Ok(Self::H24),
            _ => Err(Error::Unknown),
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
    type Error = Error;
    fn try_from(b: u8) -> Result<Self, Self::Error> {
        match b {
            b's' => Ok(Self::Second),
            b'S' => Ok(Self::FractionalSecond),
            b'A' => Ok(Self::Millisecond),
            _ => Err(Error::Unknown),
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
    type Error = Error;
    fn try_from(b: u8) -> Result<Self, Self::Error> {
        match b {
            b'E' => Ok(Self::Format),
            b'e' => Ok(Self::Local),
            b'c' => Ok(Self::StandAlone),
            _ => Err(Error::Unknown),
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
    type Error = Error;
    fn try_from(b: u8) -> Result<Self, Self::Error> {
        match b {
            b'a' => Ok(Self::AmPm),
            _ => Err(Error::Unknown),
        }
    }
}

impl From<DayPeriod> for FieldSymbol {
    fn from(input: DayPeriod) -> Self {
        FieldSymbol::DayPeriod(input)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Field {
    pub symbol: FieldSymbol,
    pub length: FieldLength,
}

impl Field {}

impl From<(FieldSymbol, FieldLength)> for Field {
    fn from(input: (FieldSymbol, FieldLength)) -> Self {
        Field {
            symbol: input.0,
            length: input.1,
        }
    }
}
