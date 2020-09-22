use std::fmt;

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
    Era,
    Year(Year),
    Month(Month),
    Day(Day),
    Weekday(Weekday),
    Period(Period),
    Hour(Hour),
    Minute,
    Second(Second),
}

impl FieldSymbol {
    pub fn write(&self, w: &mut impl fmt::Write) -> fmt::Result {
        match self {
            Self::Era => w.write_char('G'),
            Self::Year(year) => year.write(w),
            Self::Month(month) => month.write(w),
            Self::Day(day) => day.write(w),
            Self::Weekday(weekday) => weekday.write(w),
            Self::Period(period) => period.write(w),
            Self::Hour(hour) => hour.write(w),
            Self::Minute => w.write_char('m'),
            Self::Second(second) => second.write(w),
        }
    }

    pub fn from_byte(b: u8) -> Result<Self, Error> {
        match b {
            b'G' => Ok(Self::Era),
            b'm' => Ok(Self::Minute),
            _ => Year::from_byte(b)
                .map(Self::Year)
                .or_else(|_| Month::from_byte(b).map(Self::Month))
                .or_else(|_| Day::from_byte(b).map(Self::Day))
                .or_else(|_| Weekday::from_byte(b).map(Self::Weekday))
                .or_else(|_| Period::from_byte(b).map(Self::Period))
                .or_else(|_| Hour::from_byte(b).map(Self::Hour))
                .or_else(|_| Second::from_byte(b).map(Self::Second)),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FieldType {
    Era,
    Year,
    Month,
    Day,
    Hour,
}

impl FieldType {
    pub fn iter() -> impl Iterator<Item = &'static Self> {
        [Self::Era, Self::Year, Self::Month, Self::Day].iter()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Year {
    Calendar,
    WeekOf,
    Extended,
    Cyclic,
    RelatedGregorian,
}

impl Year {
    pub fn write(&self, w: &mut impl fmt::Write) -> fmt::Result {
        w.write_char(match self {
            Self::Calendar => 'y',
            Self::WeekOf => 'Y',
            Self::Extended => 'u',
            Self::Cyclic => 'U',
            Self::RelatedGregorian => 'r',
        })
    }

    pub fn from_byte(b: u8) -> Result<Self, Error> {
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

impl Month {
    pub fn write(&self, w: &mut impl fmt::Write) -> fmt::Result {
        w.write_char(match self {
            Self::Format => 'M',
            Self::StandAlone => 'L',
        })
    }

    pub fn from_byte(b: u8) -> Result<Self, Error> {
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

impl Day {
    pub fn write(&self, w: &mut impl fmt::Write) -> fmt::Result {
        w.write_char(match self {
            Self::DayOfMonth => 'd',
            Self::DayOfYear => 'D',
            Self::DayOfWeekInMonth => 'F',
            Self::ModifiedJulianDay => 'g',
        })
    }

    pub fn from_byte(b: u8) -> Result<Self, Error> {
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
    Preferred,
    PreferredNoDayPeriod,
    PreferredFlexibleDayPeriod,
}

impl Hour {
    pub fn write(&self, w: &mut impl fmt::Write) -> fmt::Result {
        w.write_char(match self {
            Self::H11 => 'K',
            Self::H12 => 'h',
            Self::H23 => 'H',
            Self::H24 => 'k',
            Self::Preferred => 'j',
            Self::PreferredNoDayPeriod => 'J',
            Self::PreferredFlexibleDayPeriod => 'C',
        })
    }

    pub fn from_byte(b: u8) -> Result<Self, Error> {
        match b {
            b'K' => Ok(Self::H11),
            b'h' => Ok(Self::H12),
            b'H' => Ok(Self::H23),
            b'k' => Ok(Self::H24),
            b'j' => Ok(Self::Preferred),
            b'J' => Ok(Self::PreferredNoDayPeriod),
            b'C' => Ok(Self::PreferredFlexibleDayPeriod),
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

impl Second {
    pub fn write(&self, w: &mut impl fmt::Write) -> fmt::Result {
        w.write_char(match self {
            Self::Second => 's',
            Self::FractionalSecond => 'S',
            Self::Millisecond => 'A',
        })
    }

    pub fn from_byte(b: u8) -> Result<Self, Error> {
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

impl Weekday {
    pub fn write(&self, w: &mut impl fmt::Write) -> fmt::Result {
        w.write_char(match self {
            Self::Format => 'E',
            Self::Local => 'e',
            Self::StandAlone => 'c',
        })
    }

    pub fn from_byte(b: u8) -> Result<Self, Error> {
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
pub enum Period {
    AmPm,
    NoonMidnight,
    Flexible,
}

impl Period {
    pub fn write(&self, w: &mut impl fmt::Write) -> fmt::Result {
        w.write_char(match self {
            Self::AmPm => 'a',
            Self::NoonMidnight => 'b',
            Self::Flexible => 'B',
        })
    }

    pub fn from_byte(b: u8) -> Result<Self, Error> {
        match b {
            b'a' => Ok(Self::AmPm),
            b'b' => Ok(Self::NoonMidnight),
            b'B' => Ok(Self::Flexible),
            _ => Err(Error::Unknown),
        }
    }
}

impl From<Period> for FieldSymbol {
    fn from(input: Period) -> Self {
        FieldSymbol::Period(input)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Field {
    pub symbol: FieldSymbol,
    pub length: FieldLength,
}

impl Field {
    pub fn write_pattern(&self, w: &mut impl fmt::Write) -> fmt::Result {
        for _ in 0..(self.length as u8) {
            self.symbol.write(w)?;
        }
        Ok(())
    }
}

impl From<(FieldSymbol, FieldLength)> for Field {
    fn from(input: (FieldSymbol, FieldLength)) -> Self {
        Field {
            symbol: input.0,
            length: input.1,
        }
    }
}
