// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub mod fields;
mod skeleton;
mod tokenizer;

use core::fmt;

pub use skeleton::DateTimeFieldBagParseError;

use fields::*;
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
/// ```
/// use icu::datetime::fieldbag::DateTimeFieldBag;
/// ```
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
    pub fn try_from_skeleton(skeleton: &str) -> Result<Self, DateTimeFieldBagParseError> {
        let parse_result = skeleton::uts35_to_fieldbag(skeleton);
        if let Some(err) = parse_result.1 {
            Err(err)
        } else {
            Ok(parse_result.0)
        }
    }

    pub fn from_skeleton(skeleton: &str) -> Self {
        // drop the error
        skeleton::uts35_to_fieldbag(skeleton).0
    }
}

impl Writeable for DateTimeFieldBag {
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        
    }
}
