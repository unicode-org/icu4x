// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::fields::FieldLength;
use core::{cmp::Ordering, convert::TryFrom};
use displaydoc::Display;

#[derive(Display, Debug, PartialEq)]
pub enum SymbolError {
    /// Invalid field symbol index.
    #[displaydoc("Invalid field symbol index: {0}")]
    InvalidIndex(u8),
    #[displaydoc("Unknown field symbol: {0}")]
    Unknown(char),
    /// Invalid character for a field symbol.
    #[displaydoc("Invalid character for a field symbol: {0}")]
    Invalid(u8),
}

#[cfg(feature = "std")]
impl std::error::Error for SymbolError {}

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

impl FieldSymbol {
    /// Symbols are necessary components of `Pattern` struct, which
    /// uses efficient byte serialization and deserialization via `zerovec`.
    ///
    /// The `FieldSymbol` impl provides non-public methods that can be used to efficiently
    /// convert between `u8` and the symbol variant.
    ///
    /// The serialization model packages the variant in one byte.
    ///
    /// 1) The top four bits are used to determine the type of the field.
    ///    (Examples: `Year`, `Month`, `Hour`)
    ///
    /// 2) The bottom four bites are used to determine the symbol of the type.
    ///    (Examples: `Year::Calendar`, `Hour::H11`)
    ///
    /// # Diagram
    ///
    /// ┌─┬─┬─┬─┬─┬─┬─┬─┐
    /// ├─┴─┴─┴─┼─┴─┴─┴─┤
    /// │ Type  │Symbol │
    /// └───────┴───────┘
    ///
    /// # Optimization
    ///
    /// This model is optimized to package data efficiently when `FieldSymbol`
    /// is used as a variant of `PatternItem`. See the documentation of `PatternItemULE`
    /// for details on how it is composed.
    ///
    /// # Constraints
    ///
    /// This model limits the available number of possible types and symbols to 16 each.

    pub(crate) fn idx_in_range(kv: &u8) -> bool {
        let k = kv & 0b0000_1111;
        let v = kv >> 4;
        match k {
            0 => Year::idx_in_range(&v),
            1 => Month::idx_in_range(&v),
            2 => Day::idx_in_range(&v),
            3 => Weekday::idx_in_range(&v),
            4 => DayPeriod::idx_in_range(&v),
            5 => Hour::idx_in_range(&v),
            6 => true,
            7 => Second::idx_in_range(&v),
            8 => TimeZone::idx_in_range(&v),
            _ => false,
        }
    }

    pub(crate) fn idx(&self) -> u8 {
        let (lower, upper) = match self {
            FieldSymbol::Year(year) => (0, year.idx()),
            FieldSymbol::Month(month) => (1, month.idx()),
            FieldSymbol::Day(day) => (2, day.idx()),
            FieldSymbol::Weekday(wd) => (3, wd.idx()),
            FieldSymbol::DayPeriod(dp) => (4, dp.idx()),
            FieldSymbol::Hour(hour) => (5, hour.idx()),
            FieldSymbol::Minute => (6, 0),
            FieldSymbol::Second(second) => (7, second.idx()),
            FieldSymbol::TimeZone(tz) => (8, tz.idx()),
        };
        let result = upper << 4;
        result | lower
    }

    pub(crate) fn from_idx(idx: u8) -> Result<Self, SymbolError> {
        // extract the top four bits out of `u8` to disriminate the field type.
        let lower = idx & 0b0000_1111;
        // use the bottom four bits to determine the symbol.
        let upper = idx >> 4;

        Ok(match lower {
            0 => Self::Year(Year::from_idx(upper)?),
            1 => Self::Month(Month::from_idx(upper)?),
            2 => Self::Day(Day::from_idx(upper)?),
            3 => Self::Weekday(Weekday::from_idx(upper)?),
            4 => Self::DayPeriod(DayPeriod::from_idx(upper)?),
            5 => Self::Hour(Hour::from_idx(upper)?),
            6 => Self::Minute,
            7 => Self::Second(Second::from_idx(upper)?),
            8 => Self::TimeZone(TimeZone::from_idx(upper)?),
            _ => return Err(SymbolError::InvalidIndex(idx)),
        })
    }
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

impl TryFrom<char> for FieldSymbol {
    type Error = SymbolError;
    fn try_from(ch: char) -> Result<Self, Self::Error> {
        if !ch.is_ascii_alphanumeric() {
            return Err(SymbolError::Invalid(ch as u8));
        }
        Year::try_from(ch)
            .map(Self::Year)
            .or_else(|_| Month::try_from(ch).map(Self::Month))
            .or_else(|_| Day::try_from(ch).map(Self::Day))
            .or_else(|_| Weekday::try_from(ch).map(Self::Weekday))
            .or_else(|_| DayPeriod::try_from(ch).map(Self::DayPeriod))
            .or_else(|_| Hour::try_from(ch).map(Self::Hour))
            .or_else(|_| {
                if ch == 'm' {
                    Ok(Self::Minute)
                } else {
                    Err(SymbolError::Unknown(ch))
                }
            })
            .or_else(|_| Second::try_from(ch).map(Self::Second))
            .or_else(|_| TimeZone::try_from(ch).map(Self::TimeZone))
    }
}

impl From<FieldSymbol> for char {
    fn from(symbol: FieldSymbol) -> Self {
        match symbol {
            FieldSymbol::Year(year) => year.into(),
            FieldSymbol::Month(month) => month.into(),
            FieldSymbol::Day(day) => day.into(),
            FieldSymbol::Weekday(weekday) => weekday.into(),
            FieldSymbol::DayPeriod(dayperiod) => dayperiod.into(),
            FieldSymbol::Hour(hour) => hour.into(),
            FieldSymbol::Minute => 'm',
            FieldSymbol::Second(second) => second.into(),
            FieldSymbol::TimeZone(time_zone) => time_zone.into(),
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

field_type!(Year; {
    0; 'y' => Calendar,
    1; 'Y' => WeekOf
}; Numeric);

field_type!(Month; {
    0; 'M' => Format,
    1; 'L' => StandAlone
});

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

field_type!(Day; {
    0; 'd' => DayOfMonth,
    1; 'D' => DayOfYear,
    2; 'F' => DayOfWeekInMonth,
    3; 'g' => ModifiedJulianDay
}; Numeric);

field_type!(Hour; {
    0; 'K' => H11,
    1; 'h' => H12,
    2; 'H' => H23,
    3; 'k' => H24
}; Numeric);

field_type!(Second; {
    0; 's' => Second,
    1; 'S' => FractionalSecond,
    2; 'A' => Millisecond
}; Numeric);

field_type!(Weekday; {
    0; 'E' => Format,
    1; 'e' => Local,
    2; 'c' => StandAlone
});

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

field_type!(DayPeriod; {
    0; 'a' => AmPm,
    1; 'b' => NoonMidnight
}; Text);

field_type!(TimeZone; {
    0; 'z' => LowerZ,
    1; 'Z' => UpperZ,
    2; 'O' => UpperO,
    3; 'v' => LowerV,
    4; 'V' => UpperV,
    5; 'x' => LowerX,
    6; 'X' => UpperX
});

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
            Self::UpperZ => match length.idx() {
                1..=3 => Numeric,
                4 => Text,
                5 => Numeric,
                _ => Text,
            },
            Self::UpperO => match length.idx() {
                1 => Text,
                4 => Numeric,
                _ => Text,
            },
            Self::LowerX | Self::UpperX => Numeric,
            Self::LowerZ | Self::LowerV | Self::UpperV => Text,
        }
    }
}
