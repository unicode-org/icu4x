// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::fields::FieldLength;
use core::{cmp::Ordering, convert::TryFrom};
use displaydoc::Display;
use icu_provider::{yoke, zerofrom};
use zerovec::ule::{AsULE, ZeroVecError, ULE};

#[derive(Display, Debug, PartialEq, Copy, Clone)]
#[non_exhaustive]
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
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake), databake(path = icu_datetime::fields))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[allow(clippy::exhaustive_enums)] // part of data struct
pub enum FieldSymbol {
    Era,
    Year(Year),
    Month(Month),
    Week(Week),
    Day(Day),
    Weekday(Weekday),
    DayPeriod(DayPeriod),
    Hour(Hour),
    Minute,
    Second(Second),
    TimeZone(TimeZone),
}

impl FieldSymbol {
    /// Symbols are necessary components of `Pattern` struct which
    /// uses efficient byte serialization and deserialization via `zerovec`.
    ///
    /// The `FieldSymbol` impl provides non-public methods that can be used to efficiently
    /// convert between `u8` and the symbol variant.
    ///
    /// The serialization model packages the variant in one byte.
    ///
    /// 1) The top four bits are used to determine the type of the field
    ///    using that type's `idx()/from_idx()` for the mapping.
    ///    (Examples: `Year`, `Month`, `Hour`)
    ///
    /// 2) The bottom four bits are used to determine the symbol of the type.
    ///    (Examples: `Year::Calendar`, `Hour::H11`)
    ///
    /// # Diagram
    ///
    /// ```text
    /// ┌─┬─┬─┬─┬─┬─┬─┬─┐
    /// ├─┴─┴─┴─┼─┴─┴─┴─┤
    /// │ Type  │Symbol │
    /// └───────┴───────┘
    /// ```
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

    #[inline]
    pub(crate) fn idx(&self) -> u8 {
        let (high, low) = match self {
            FieldSymbol::Era => (0, 0),
            FieldSymbol::Year(year) => (1, year.idx()),
            FieldSymbol::Month(month) => (2, month.idx()),
            FieldSymbol::Week(w) => (3, w.idx()),
            FieldSymbol::Day(day) => (4, day.idx()),
            FieldSymbol::Weekday(wd) => (5, wd.idx()),
            FieldSymbol::DayPeriod(dp) => (6, dp.idx()),
            FieldSymbol::Hour(hour) => (7, hour.idx()),
            FieldSymbol::Minute => (8, 0),
            FieldSymbol::Second(second) => (9, second.idx()),
            FieldSymbol::TimeZone(tz) => (10, tz.idx()),
        };
        let result = high << 4;
        result | low
    }

    #[inline]
    pub(crate) fn from_idx(idx: u8) -> Result<Self, SymbolError> {
        // extract the top four bits to determine the symbol.
        let low = idx & 0b0000_1111;
        // use the bottom four bits out of `u8` to disriminate the field type.
        let high = idx >> 4;

        Ok(match high {
            0 if low == 0 => Self::Era,
            1 => Self::Year(Year::from_idx(low)?),
            2 => Self::Month(Month::from_idx(low)?),
            3 => Self::Week(Week::from_idx(low)?),
            4 => Self::Day(Day::from_idx(low)?),
            5 => Self::Weekday(Weekday::from_idx(low)?),
            6 => Self::DayPeriod(DayPeriod::from_idx(low)?),
            7 => Self::Hour(Hour::from_idx(low)?),
            8 if low == 0 => Self::Minute,
            9 => Self::Second(Second::from_idx(low)?),
            10 => Self::TimeZone(TimeZone::from_idx(low)?),
            _ => return Err(SymbolError::InvalidIndex(idx)),
        })
    }

    /// Returns the index associated with this FieldSymbol.
    fn discriminant_idx(&self) -> u8 {
        match self {
            FieldSymbol::Era => 0,
            FieldSymbol::Year(_) => 1,
            FieldSymbol::Month(_) => 2,
            FieldSymbol::Week(_) => 3,
            FieldSymbol::Day(_) => 4,
            FieldSymbol::Weekday(_) => 5,
            FieldSymbol::DayPeriod(_) => 6,
            FieldSymbol::Hour(_) => 7,
            FieldSymbol::Minute => 8,
            FieldSymbol::Second(_) => 9,
            FieldSymbol::TimeZone(_) => 10,
        }
    }

    /// Compares this enum with other solely based on the enum variant,
    /// ignoring the enum's data.
    pub(crate) fn discriminant_cmp(&self, other: &Self) -> Ordering {
        self.discriminant_idx().cmp(&other.discriminant_idx())
    }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct FieldSymbolULE(u8);

impl AsULE for FieldSymbol {
    type ULE = FieldSymbolULE;
    fn to_unaligned(self) -> Self::ULE {
        FieldSymbolULE(self.idx())
    }
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        #[allow(clippy::unwrap_used)] // OK because the ULE is pre-validated
        Self::from_idx(unaligned.0).unwrap()
    }
}

impl FieldSymbolULE {
    #[inline]
    pub(crate) fn validate_byte(byte: u8) -> Result<(), ZeroVecError> {
        FieldSymbol::from_idx(byte)
            .map(|_| ())
            .map_err(|_| ZeroVecError::parse::<FieldSymbol>())
    }
}

// Safety checklist for ULE:
//
// 1. Must not include any uninitialized or padding bytes (true since transparent over a ULE).
// 2. Must have an alignment of 1 byte (true since transparent over a ULE).
// 3. ULE::validate_byte_slice() checks that the given byte slice represents a valid slice.
// 4. ULE::validate_byte_slice() checks that the given byte slice has a valid length
//    (true since transparent over a type of size 1).
// 5. All other methods must be left with their default impl.
// 6. Byte equality is semantic equality.
unsafe impl ULE for FieldSymbolULE {
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), ZeroVecError> {
        for byte in bytes {
            Self::validate_byte(*byte)?;
        }
        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[allow(clippy::exhaustive_enums)] // used in data struct
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
            Self::Era => 0,
            Self::Year(Year::Calendar) => 1,
            Self::Year(Year::WeekOf) => 2,
            Self::Month(Month::Format) => 3,
            Self::Month(Month::StandAlone) => 4,
            Self::Week(Week::WeekOfYear) => 5,
            Self::Week(Week::WeekOfMonth) => 6,
            Self::Day(Day::DayOfMonth) => 7,
            Self::Day(Day::DayOfYear) => 8,
            Self::Day(Day::DayOfWeekInMonth) => 9,
            Self::Day(Day::ModifiedJulianDay) => 10,
            Self::Weekday(Weekday::Format) => 11,
            Self::Weekday(Weekday::Local) => 12,
            Self::Weekday(Weekday::StandAlone) => 13,
            Self::DayPeriod(DayPeriod::AmPm) => 14,
            Self::DayPeriod(DayPeriod::NoonMidnight) => 15,
            Self::Hour(Hour::H11) => 16,
            Self::Hour(Hour::H12) => 17,
            Self::Hour(Hour::H23) => 18,
            Self::Hour(Hour::H24) => 19,
            Self::Minute => 20,
            Self::Second(Second::Second) => 21,
            Self::Second(Second::FractionalSecond) => 22,
            Self::Second(Second::Millisecond) => 23,
            Self::TimeZone(TimeZone::LowerZ) => 24,
            Self::TimeZone(TimeZone::UpperZ) => 25,
            Self::TimeZone(TimeZone::UpperO) => 26,
            Self::TimeZone(TimeZone::LowerV) => 27,
            Self::TimeZone(TimeZone::UpperV) => 28,
            Self::TimeZone(TimeZone::LowerX) => 29,
            Self::TimeZone(TimeZone::UpperX) => 30,
        }
    }
}

impl TryFrom<char> for FieldSymbol {
    type Error = SymbolError;
    fn try_from(ch: char) -> Result<Self, Self::Error> {
        if !ch.is_ascii_alphanumeric() {
            return Err(SymbolError::Invalid(ch as u8));
        }

        (if ch == 'G' {
            Ok(Self::Era)
        } else {
            Err(SymbolError::Unknown(ch))
        })
        .or_else(|_| Year::try_from(ch).map(Self::Year))
        .or_else(|_| Month::try_from(ch).map(Self::Month))
        .or_else(|_| Week::try_from(ch).map(Self::Week))
        .or_else(|_| Day::try_from(ch).map(Self::Day))
        .or_else(|_| Weekday::try_from(ch).map(Self::Weekday))
        .or_else(|_| DayPeriod::try_from(ch).map(Self::DayPeriod))
        .or_else(|_| Hour::try_from(ch).map(Self::Hour))
        .or({
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
            FieldSymbol::Era => 'G',
            FieldSymbol::Year(year) => year.into(),
            FieldSymbol::Month(month) => month.into(),
            FieldSymbol::Week(week) => week.into(),
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

macro_rules! field_type {
    ($i:ident; { $($key:expr => $val:ident = $idx:expr,)* }; $length_type:ident; $ule_name:ident) => (
        field_type!($i; {$($key => $val = $idx,)*}; $ule_name);

        impl LengthType for $i {
            fn get_length_type(&self, _length: FieldLength) -> TextOrNumeric {
                TextOrNumeric::$length_type
            }
        }
    );
    ($i:ident; { $($key:expr => $val:ident = $idx:expr,)* }; $ule_name:ident) => (
        #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, yoke::Yokeable, zerofrom::ZeroFrom)]
        // FIXME: This should be replaced with a custom derive.
        // See: https://github.com/unicode-org/icu4x/issues/1044
        #[cfg_attr(
            feature = "datagen",
            derive(serde::Serialize, databake::Bake),
            databake(path = icu_datetime::fields),
        )]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
        #[allow(clippy::enum_variant_names)]
        #[repr(u8)]
        #[zerovec::make_ule($ule_name)]
        #[allow(clippy::exhaustive_enums)] // used in data struct
        pub enum $i {
            $($val = $idx, )*
        }

        impl $i {
            /// Retrieves an index of the field variant.
            ///
            /// # Examples
            ///
            /// ```ignore
            /// use icu::datetime::fields::Month;
            ///
            /// assert_eq!(Month::StandAlone::idx(), 1);
            /// ```
            ///
            /// # Stability
            ///
            /// This is mostly useful for serialization,
            /// and does not guarantee index stability between ICU4X
            /// versions.
            #[inline]
            pub(crate) fn idx(self) -> u8 {
                self as u8
            }

            /// Retrieves a field variant from an index.
            ///
            /// # Examples
            ///
            /// ```ignore
            /// use icu::datetime::fields::Month;
            ///
            /// assert_eq!(Month::from_idx(0), Month::Format);
            /// ```
            ///
            /// # Stability
            ///
            /// This is mostly useful for serialization,
            /// and does not guarantee index stability between ICU4X
            /// versions.
            #[inline]
            pub(crate) fn from_idx(idx: u8) -> Result<Self, SymbolError> {
                Self::new_from_u8(idx)
                    .ok_or(SymbolError::InvalidIndex(idx))
            }
        }

        impl TryFrom<char> for $i {
            type Error = SymbolError;

            fn try_from(ch: char) -> Result<Self, Self::Error> {
                match ch {
                    $(
                        $key => Ok(Self::$val),
                    )*
                    _ => Err(SymbolError::Unknown(ch)),
                }
            }
        }

        impl From<$i> for FieldSymbol {
            fn from(input: $i) -> Self {
                Self::$i(input)
            }
        }

        impl From<$i> for char {
            fn from(input: $i) -> char {
                match input {
                    $(
                        $i::$val => $key,
                    )*
                }
            }
        }
    );
}

field_type!(Year; {
    'y' => Calendar = 0,
    'Y' => WeekOf = 1,
}; Numeric; YearULE);

field_type!(Month; {
    'M' => Format = 0,
    'L' => StandAlone = 1,
}; MonthULE);

impl LengthType for Month {
    fn get_length_type(&self, length: FieldLength) -> TextOrNumeric {
        match length {
            FieldLength::One => TextOrNumeric::Numeric,
            FieldLength::TwoDigit => TextOrNumeric::Numeric,
            FieldLength::Abbreviated => TextOrNumeric::Text,
            FieldLength::Wide => TextOrNumeric::Text,
            FieldLength::Narrow => TextOrNumeric::Text,
            FieldLength::Six => TextOrNumeric::Text,
            FieldLength::Fixed(_) => {
                debug_assert!(false, "Fixed field length is only supported for seconds");
                TextOrNumeric::Text
            }
        }
    }
}

field_type!(Day; {
    'd' => DayOfMonth = 0,
    'D' => DayOfYear = 1,
    'F' => DayOfWeekInMonth = 2,
    'g' => ModifiedJulianDay = 3,
}; Numeric; DayULE);

field_type!(Hour; {
    'K' => H11 = 0,
    'h' => H12 = 1,
    'H' => H23 = 2,
    'k' => H24 = 3,
}; Numeric; HourULE);

field_type!(Second; {
    's' => Second = 0,
    'S' => FractionalSecond = 1,
    'A' => Millisecond = 2,
}; Numeric; SecondULE);

field_type!(Week; {
    'w' => WeekOfYear = 0,
    'W' => WeekOfMonth = 1,
}; Numeric; WeekULE);

field_type!(Weekday; {
    'E' => Format = 0,
    'e' => Local = 1,
    'c' => StandAlone = 2,
}; WeekdayULE);

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
    'a' => AmPm = 0,
    'b' => NoonMidnight = 1,
}; Text; DayPeriodULE);

field_type!(TimeZone; {
    'z' => LowerZ = 0,
    'Z' => UpperZ = 1,
    'O' => UpperO = 2,
    'v' => LowerV = 3,
    'V' => UpperV = 4,
    'x' => LowerX = 5,
    'X' => UpperX = 6,
}; TimeZoneULE);

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
