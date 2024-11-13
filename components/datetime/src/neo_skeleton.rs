// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Temporary module for neo datetime skeletons (Semantic Skeleta)

#[cfg(feature = "serde")]
use crate::neo_serde::*;
use icu_timezone::scaffold::IntoOption;

/// A specification for the length of a date or component of a date.
///
/// Contrary to [`crate::options::length::Time`] and
/// [`crate::options::length::Date`], this has only three levels, with no
/// `Full`; this is because `Full` corresponds to additional components,
/// rather than to making the components wider than in `Long`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[repr(u8)] // discriminants come from symbol count in UTS 35
#[non_exhaustive]
pub enum NeoSkeletonLength {
    /// A long date, typically spelled-out, as in “January 1, 2000”.
    Long = 4,
    /// A medium-sized date; typically abbreviated, as in “Jan. 1, 2000”.
    Medium = 3,
    /// A short date; typically numeric, as in “1/1/2000”.
    Short = 1,
}

impl IntoOption<NeoSkeletonLength> for NeoSkeletonLength {
    #[inline]
    fn into_option(self) -> Option<Self> {
        Some(self)
    }
}

/// The alignment context of the formatted string.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[non_exhaustive]
pub enum Alignment {
    /// Align fields as the locale specifies them to be aligned.
    ///
    /// This is the default option.
    Auto,
    /// Align fields as appropriate for a column layout. For example:
    ///
    /// | US Holiday   | Date       |
    /// |--------------|------------|
    /// | Memorial Day | 05/26/2025 |
    /// | Labor Day    | 09/01/2025 |
    /// | Veterans Day | 11/11/2025 |
    ///
    /// This option causes numeric fields to be padded when necessary. It does
    /// not impact whether a numeric or spelled-out field is chosen.
    Column,
}

impl IntoOption<Alignment> for Alignment {
    #[inline]
    fn into_option(self) -> Option<Self> {
        Some(self)
    }
}

/// A specification of how to render the year and the era.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[non_exhaustive]
pub enum YearStyle {
    /// Display the century and/or era when needed to disambiguate the year,
    /// based on locale preferences.
    ///
    /// This is the default option.
    ///
    /// Examples:
    ///
    /// - `1000 BC`
    /// - `77 AD`
    /// - `1900`
    /// - `'24`
    Auto,
    /// Always display the century, and display the era when needed to
    /// disambiguate the year, based on locale preferences.
    ///
    /// Examples:
    ///
    /// - `1000 BC`
    /// - `77 AD`
    /// - `1900`
    /// - `2024`
    Full,
    /// Always display the century and era.
    ///
    /// Examples:
    ///
    /// - `1000 BC`
    /// - `77 AD`
    /// - `1900 AD`
    /// - `2024 AD`
    Always,
    // TODO(#4478): add Hide and Never options once there is data to back them
}

impl IntoOption<YearStyle> for YearStyle {
    #[inline]
    fn into_option(self) -> Option<Self> {
        Some(self)
    }
}

/// A specification for how precisely to display the time of day.
///
/// The examples below are based on the following inputs and hour cycles:
///
/// 1. 11 o'clock with 12-hour time
/// 2. 16:20 (4:20 pm) with 24-hour time
/// 3. 7:15:01.85 with 24-hour time
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(from = "TimePrecisionSerde", into = "TimePrecisionSerde")
)]
#[non_exhaustive]
pub enum TimePrecision {
    /// Always display the hour. Display smaller fields if they are nonzero.
    ///
    /// Examples:
    ///
    /// 1. `11 am`
    /// 2. `16:20`
    /// 3. `07:15:01.85`
    HourPlus,
    /// Always display the hour. Hide all other time fields.
    ///
    /// Examples:
    ///
    /// 1. `11 am`
    /// 2. `16h`
    /// 3. `07h`
    HourExact,
    /// Always display the hour and minute. Display the second if nonzero.
    ///
    /// Examples:
    ///
    /// 1. `11:00 am`
    /// 2. `16:20`
    /// 3. `07:15:01.85`
    MinutePlus,
    /// Always display the hour and minute. Hide the second.
    ///
    /// Examples:
    ///
    /// 1. `11:00 am`
    /// 2. `16:20`
    /// 3. `07:15`
    MinuteExact,
    /// Display the hour, minute, and second. Display fractional seconds if nonzero.
    ///
    /// This is the default.
    ///
    /// Examples:
    ///
    /// 1. `11:00:00 am`
    /// 2. `16:20:00`
    /// 3. `07:15:01.85`
    SecondPlus,
    /// Display the hour, minute, and second with the given number of
    /// fractional second digits.
    ///
    /// Examples with [`FractionalSecondDigits::F1`]:
    ///
    /// 1. `11:00:00.0 am`
    /// 2. `16:20:00.0`
    /// 3. `07:15:01.8`
    SecondExact(FractionalSecondDigits),
}

impl IntoOption<TimePrecision> for TimePrecision {
    #[inline]
    fn into_option(self) -> Option<Self> {
        Some(self)
    }
}

/// A specification for how many fractional second digits to display.
///
/// For example, to display the time with millisecond precision, use
/// [`FractionalSecondDigits::F3`].
///
/// Lower-precision digits will be truncated.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u8", into = "u8"))]
#[non_exhaustive]
pub enum FractionalSecondDigits {
    /// Zero fractional digits. This is the default.
    F0,
    /// One fractional digit (tenths of a second).
    F1,
    /// Two fractional digits (hundredths of a second).
    F2,
    /// Three fractional digits (thousandths of a second).
    F3,
    /// Four fractional digits.
    F4,
    /// Five fractional digits.
    F5,
    /// Six fractional digits.
    F6,
    /// Seven fractional digits.
    F7,
    /// Eight fractional digits.
    F8,
    /// Nine fractional digits.
    F9,
}

/// An error from constructing [`FractionalSecondDigits`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, displaydoc::Display)]
#[non_exhaustive]
pub enum FractionalSecondError {
    /// The provided value is out of range (0-9).
    OutOfRange,
}

impl From<FractionalSecondDigits> for u8 {
    fn from(value: FractionalSecondDigits) -> u8 {
        use FractionalSecondDigits::*;
        match value {
            F0 => 0,
            F1 => 1,
            F2 => 2,
            F3 => 3,
            F4 => 4,
            F5 => 5,
            F6 => 6,
            F7 => 7,
            F8 => 8,
            F9 => 9,
        }
    }
}

impl TryFrom<u8> for FractionalSecondDigits {
    type Error = FractionalSecondError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use FractionalSecondDigits::*;
        match value {
            0 => Ok(F0),
            1 => Ok(F1),
            2 => Ok(F2),
            3 => Ok(F3),
            4 => Ok(F4),
            5 => Ok(F5),
            6 => Ok(F6),
            7 => Ok(F7),
            8 => Ok(F8),
            9 => Ok(F9),
            _ => Err(FractionalSecondError::OutOfRange),
        }
    }
}
