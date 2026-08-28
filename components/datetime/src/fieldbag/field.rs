// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Field-specific enums for [`DateTimeFieldBag`](super::DateTimeFieldBag).

#[doc(inline)]
pub use crate::options::SubsecondDigits as Subsecond;

/// Options for formatting the era.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    all(feature = "serde", feature = "unstable"),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(
    all(feature = "serde", feature = "unstable"),
    serde(rename_all = "kebab-case")
)]
#[non_exhaustive]
pub enum Era {
    /// Short era name, such as "AD" or "BC".
    Short,
    /// Long era name, such as "Anno Domini".
    Long,
    /// Narrow era name, such as "A" or "B".
    Narrow,
}

/// Options for formatting the year.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    all(feature = "serde", feature = "unstable"),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(
    all(feature = "serde", feature = "unstable"),
    serde(rename_all = "kebab-case")
)]
#[non_exhaustive]
pub enum Year {
    /// Numeric year, such as "2026".
    Numeric,
    /// Two-digit year, such as "26".
    TwoDigit,
}

/// Options for formatting the month.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    all(feature = "serde", feature = "unstable"),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(
    all(feature = "serde", feature = "unstable"),
    serde(rename_all = "kebab-case")
)]
#[non_exhaustive]
pub enum Month {
    /// Numeric month, such as "8".
    Numeric,
    /// Two-digit month, such as "08".
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
#[cfg_attr(
    all(feature = "serde", feature = "unstable"),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(
    all(feature = "serde", feature = "unstable"),
    serde(rename_all = "kebab-case")
)]
#[non_exhaustive]
pub enum Day {
    /// Numeric day, such as "1" or "28".
    Numeric,
    /// Two-digit day, such as "01" or "28".
    TwoDigit,
}

/// Options for formatting the day of week.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    all(feature = "serde", feature = "unstable"),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(
    all(feature = "serde", feature = "unstable"),
    serde(rename_all = "kebab-case")
)]
#[non_exhaustive]
pub enum Weekday {
    /// Short weekday name, such as "Fri".
    Short,
    /// Long weekday name, such as "Friday".
    Long,
    /// Narrow weekday name, such as "F".
    Narrow,
}

/// Options for formatting the day period.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    all(feature = "serde", feature = "unstable"),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(
    all(feature = "serde", feature = "unstable"),
    serde(rename_all = "kebab-case")
)]
#[non_exhaustive]
pub enum DayPeriod {
    /// Short day period, such as "AM" or "PM".
    Short,
    /// Long day period, such as "morning" or "afternoon".
    Long,
    /// Narrow day period, such as "a" or "p".
    Narrow,
}

/// Options for formatting the hour.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    all(feature = "serde", feature = "unstable"),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(
    all(feature = "serde", feature = "unstable"),
    serde(rename_all = "kebab-case")
)]
#[non_exhaustive]
pub enum Hour {
    /// Numeric hour, such as "9" or "14".
    Numeric,
    /// Two-digit hour, such as "09" or "14".
    TwoDigit,
}

/// Options for formatting the minute.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    all(feature = "serde", feature = "unstable"),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(
    all(feature = "serde", feature = "unstable"),
    serde(rename_all = "kebab-case")
)]
#[non_exhaustive]
pub enum Minute {
    /// Numeric minute, such as "5" or "36".
    Numeric,
    /// Two-digit minute, such as "05" or "36".
    TwoDigit,
}

/// Options for formatting the second.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    all(feature = "serde", feature = "unstable"),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(
    all(feature = "serde", feature = "unstable"),
    serde(rename_all = "kebab-case")
)]
#[non_exhaustive]
pub enum Second {
    /// Numeric second, such as "5" or "30".
    Numeric,
    /// Two-digit second, such as "05" or "30".
    TwoDigit,
}

/// Options for formatting the time zone name.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    all(feature = "serde", feature = "unstable"),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(
    all(feature = "serde", feature = "unstable"),
    serde(rename_all = "kebab-case")
)]
#[non_exhaustive]
pub enum TimeZoneName {
    /// Short specific non-location format (e.g. "PDT").
    ShortSpecific,
    /// Long specific non-location format (e.g. "Pacific Daylight Time").
    LongSpecific,
    /// Short localized offset format (e.g. "GMT-8").
    ShortOffset,
    /// Long localized offset format (e.g. "GMT-08:00").
    LongOffset,
    /// Short generic non-location format (e.g. "PT").
    ShortGeneric,
    /// Long generic non-location format (e.g. "Pacific Time").
    LongGeneric,
}
