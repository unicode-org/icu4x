// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Field-specific enums for [`DateTimeFieldBag`](super::DateTimeFieldBag).

#[doc(inline)]
pub use crate::options::SubsecondDigits as Subsecond;

/// Options for formatting the era.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Era {
    /// Short era name, such as "AD" or "BC".
    Short,
    /// Long era name, such as "Anno Domini" or "Before Christ".
    Long,
    /// Narrow era name, such as "A" or "B".
    Narrow,
}

/// Options for formatting the year.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Year {
    /// Numeric calendar year, such as "2026".
    Numeric,
    /// Two-digit calendar year, such as "26".
    TwoDigit,
    // TODO: Add 3-digit, 4-digit, ... ?
    /// Abbreviated cyclic year name, such as "甲子".
    CyclicAbbreviated,
    /// Long cyclic year name.
    CyclicLong,
    /// Narrow cyclic year name.
    CyclicNarrow,
}

/// Options for formatting the month.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Month {
    /// Numeric month, such as "8" or "12".
    Numeric,
    /// Two-digit month, such as "08" or "12".
    TwoDigit,
    /// Short month name, such as "Aug".
    Short,
    /// Long month name, such as "August".
    Long,
    /// Narrow month name, such as "A".
    Narrow,
}

/// Options for formatting the day of month.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Day {
    /// Numeric day, such as "1" or "28".
    Numeric,
    /// Two-digit day, such as "01" or "28".
    TwoDigit,
}

/// Options for formatting the day of week.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Weekday {
    /// Abbreviated weekday name, such as "Fri".
    Abbreviated,
    /// Short weekday name, such as "Fr".
    Short,
    /// Long weekday name, such as "Friday".
    Long,
    /// Narrow weekday name, such as "F".
    Narrow,
}

/// Options for the length of the day period.
/// 
/// Set the hour length with [`Hour`]. Set the hour symbol with [`HourType`].
/// 
/// If either [`Hour`] or [`HourType`] are set, and they resolve to a locale
/// that needs a day period, the default is [`DayPeriod::Short`].
/// 
/// For an example, see [`HourType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DayPeriod {
    /// Short day period, such as "AM" or "PM".
    Short,
    /// Long day period, such as "morning" or "afternoon".
    Long,
    /// Narrow day period, such as "a" or "p".
    Narrow,
}

/// May not be paired with [`HourType::Auto`] variants.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DayPeriodType {
    AmPm,
    AmPmNoonMidnight,
    Flexible,
}

pub enum ManishDayPeriodKind {
    AmPm,
    AmPmNoonMidnight,
    Flexible,
    Auto,
    AutoNumeric,
    Hidden,
}

/// Options for the length of the hour field.
/// 
/// Set the hour symbol with [`HourType`]. Set the day period style with [`DayPeriod`].
/// 
/// If [`HourType`] is set but not [`Hour`], the default is [`Hour::Numeric`].
/// 
/// For an example, see [`HourType`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Hour {
    /// Numeric with the minimum number of digits.
    Numeric,
    /// Numeric, always padded to 2 digits.
    TwoDigit,
}

/// Options for the hour field symbol.
/// 
/// Set the length with [`Hour`]. Set the day period style with [`DayPeriod`].
/// 
/// If [`Hour`] is set but not [`HourType`], the default is [`HourType::Auto`].
/// 
/// # Examples
/// 
/// ```
/// use icu::datetime::fieldbag::*;
/// 
/// let mut bag = FieldBag::default();
/// bag.hour_type = HourType::AutoNumeric;
/// assert_writeable_eq!(bag, "j");
/// 
/// bag.hour = Hour::TwoDigit;
/// assert_writeable_eq!(bag, "jj");
/// 
/// bag.day_period = DayPeriod::Wide;
/// assert_writeable_eq!(bag, "jjjj");
/// 
/// bag.hour_type = HourType::H12;
/// assert_writeable_eq!(bag, "hhaaaa");
/// 
/// bag.day_period_type = DayPeriodType::Flexible;
/// assert_writeable_eq!(bag, "hhBBBB");
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum HourType {
    /// An hour with a possibly spelled-out day period: `'C'`
    Auto,
    /// A numeric hour with 12-hour or 24-hour time chosen by the locale: `'j'`
    AutoNumeric,
    /// Like [`Self::AutoNumeric`] but with the day period hidden: `'J'`
    AutoNumericNoDayPeriod,
    /// An hour that is always in the 11-hour cycle: `'K'`
    H11,
    /// An hour that is always in the 12-hour cycle: `'h'`
    H12,
    /// An hour that is always in the 23-hour cycle: `'H'`
    H23,
    /// An hour that is always in the 24-hour cycle: `'k'`
    H24,
}


pub enum HourAndDayPeriodKind {
    // Tier 1
    Auto,
    AutoNumeric,
    AutoNumericNoDayPeriod,
    Hour11AmPm,
    Hour11Noon,
    Hour11Flex,
    Hour12AmPm,
    Hour12Noon,
    Hour12Flex,
    Hour23,
    // Tier 2
    Hour23Flex,
    // Tier 2-
    Hour23AmPm,
    Hour23Noon,
}


pub enum AutoHourKind {
    Auto,
    AutoNumeric,
    AutoNumericNoDayPeriod,
}


pub enum DayPeriodPrecision {
    Hidden,
    Default,
    AmPmNoonMidnight,
    Flexible,
}

pub enum YetAnotherHourKind {
    Auto,
    H11,
    H12,
    H23,
}

/// 


/// Options for formatting the minute.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Minute {
    /// Numeric minute, such as "5" or "36".
    Numeric,
    /// Two-digit minute, such as "05" or "36".
    TwoDigit,
}

/// Options for formatting the second.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Second {
    /// Numeric second, such as "5" or "30".
    Numeric,
    /// Two-digit second, such as "05" or "30".
    TwoDigit,
}

/// Options for formatting the time zone name.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TimeZoneName {
    /// Short specific non-location format, such as "PDT".
    ShortSpecific,
    /// Long specific non-location format, such as "Pacific Daylight Time".
    LongSpecific,
    /// Short localized offset format, such as "GMT-8".
    ShortOffset,
    /// Long localized offset format, such as "GMT-08:00".
    LongOffset,
    /// Short generic non-location format, such as "PT".
    ShortGeneric,
    /// Long generic non-location format, such as "Pacific Time".
    LongGeneric,
}
