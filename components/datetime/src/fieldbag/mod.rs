// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Types for expressing field-by-field models for datetime formats.

pub mod field;
mod skeleton;

use core::{fmt, str::FromStr};

pub use skeleton::DateTimeFieldBagParseError;

use field::*;
use writeable::Writeable;

/// A bag of individual datetime formatting fields.
///
/// This struct is designed to mirror the ECMA-402 `Intl.DateTimeFormat` options bag, with minor
/// extensions to cover additional features implemented by ICU4X.
///
/// This struct is convertible back and forth to a UTS #35 skeleton string. However, it only
/// implements the subset of skeleton string functionality required by ECMA-402 and ICU4X.
///
/// # Examples
///
/// TODO: Add an example when more fully implemented
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct DateTimeFieldBag {
    /// The representation of the era.
    ///
    /// If `None`, do not show the era, unless required by other fields.
    pub era: Option<Era>,

    /// The representation of the year.
    ///
    /// If `None`, do not show the year, unless required by other fields.
    pub year: Option<Year>,

    /// The representation of the month.
    ///
    /// If `None`, do not show the month, unless required by other fields.
    pub month: Option<Month>,

    /// The representation of the day.
    ///
    /// If `None`, do not show the day, unless required by other fields.
    pub day: Option<Day>,

    /// The representation of the weekday.
    ///
    /// If `None`, do not show the weekday.
    pub weekday: Option<Weekday>,

    /// The formatting style used for day periods like "in the morning", "am", "noon", "n" etc.
    ///
    /// If `None`, use AM/PM.
    ///
    /// Ignored unless a 12-hour clock is used.
    ///
    /// Note: Many locales use the same string irrespective of the width specified.
    pub day_period: Option<DayPeriod>,

    /// Choice between 12-hour and 24-hour time.
    ///
    /// If `None`, use the locale preference.
    ///
    /// Ignored unless `hour` is set.
    ///
    /// This corresponds to the `hour12` field in ECMA-402.
    pub hour_kind: Option<HourKind>,

    /// The representation of the hour.
    ///
    /// If `None`, do not show the hour, unless required by other fields.
    pub hour: Option<Hour>,

    /// The representation of the minute.
    ///
    /// If `None`, do not show the minute, unless required by other fields.
    pub minute: Option<Minute>,

    /// The representation of the second.
    ///
    /// If `None`, do not show the second, unless required by other fields.
    pub second: Option<Second>,

    /// The number of digits used to represent fractions of a second.
    /// Any additional digits are truncated.
    ///
    /// If `None`, do not show fractional seconds.
    pub fractional_second_digits: Option<FractionalSecondDigits>,

    /// The localized representation of the time zone name.
    ///
    /// If `None`, do not show the time zone.
    pub time_zone_name: Option<TimeZoneName>,
}

impl DateTimeFieldBag {
    /// Creates a [`DateTimeFieldBag`] from a skeleton string.
    ///
    /// This function rejects strings with duplicate fields, unknown fields, non-field
    /// characters, and other conditions in the [`DateTimeFieldBagParseError`]. For a
    /// more lenient parse function, use [`DateTimeFieldBag::from_skeleton`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldbag::DateTimeFieldBag;
    /// use icu::datetime::fieldbag::field::Era;
    /// use icu::datetime::fieldbag::field::Year;
    /// use writeable::assert_writeable_eq;
    ///
    /// let bag = DateTimeFieldBag::try_from_skeleton("GGGGy").unwrap();
    ///
    /// let mut expected = DateTimeFieldBag::default();
    /// expected.era = Some(Era::Long);
    /// expected.year = Some(Year::Numeric);
    /// assert_eq!(bag, expected);
    ///
    /// let err_bag = DateTimeFieldBag::try_from_skeleton("...GGGGy...");
    /// assert!(err_bag.is_err());
    /// ```
    pub fn try_from_skeleton(skeleton: &str) -> Result<Self, DateTimeFieldBagParseError> {
        let parse_result = skeleton::uts35_to_fieldbag(skeleton);
        if let Some(err) = parse_result.1 {
            Err(err)
        } else {
            Ok(parse_result.0)
        }
    }

    /// Creates a [`DateTimeFieldBag`] from a skeleton string.
    ///
    /// This function gracefully ignores duplicate fields, unknown fields, non-field
    /// characters, and other conditions in the [`DateTimeFieldBagParseError`]. For a
    /// more strict parse function, use [`DateTimeFieldBag::try_from_skeleton`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldbag::DateTimeFieldBag;
    /// use icu::datetime::fieldbag::field::Era;
    /// use icu::datetime::fieldbag::field::Year;
    /// use writeable::assert_writeable_eq;
    ///
    /// let bag = DateTimeFieldBag::from_skeleton("GGGGy");
    ///
    /// let mut expected = DateTimeFieldBag::default();
    /// expected.era = Some(Era::Long);
    /// expected.year = Some(Year::Numeric);
    /// assert_eq!(bag, expected);
    ///
    /// let lenient_bag = DateTimeFieldBag::from_skeleton("...GGGGy...");
    /// assert_eq!(lenient_bag, expected);
    /// ```
    pub fn from_skeleton(skeleton: &str) -> Self {
        // drop the error
        skeleton::uts35_to_fieldbag(skeleton).0
    }
}

impl FromStr for DateTimeFieldBag {
    type Err = DateTimeFieldBagParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from_skeleton(s)
    }
}

impl Writeable for DateTimeFieldBag {
    fn write_to<W: ?Sized + fmt::Write>(&self, sink: &mut W) -> fmt::Result {
        skeleton::fieldbag_to_uts35(self, sink)
    }
}

writeable::impl_display_with_writeable!(DateTimeFieldBag);
