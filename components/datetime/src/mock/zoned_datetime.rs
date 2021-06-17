// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use tinystr::TinyStr8;

use crate::date::*;
use std::str::FromStr;

use super::{datetime::MockDateTime, time_zone::MockTimeZone};

/// A temporary struct that implements [`ZonedDateTimeInput`]
/// and is used in tests, benchmarks and examples of this component.
///
/// The composition of [`MockDateTime`] and [`MockTimeZone`].
///
/// *Notice:* Rust at the moment does not have a canonical way to represent date and time. We are introducing
/// `MockZonedDateTime` as an example of the data necessary for ICU [`ZonedDateTimeFormat`] to work, and
/// [we hope to work with the community](https://github.com/unicode-org/icu4x/blob/main/docs/research/datetime.md)
/// to develop core date and time APIs that will work as an input for this component.
///
/// # Examples
///
/// ```
/// use icu::datetime::mock::datetime::MockDateTime;
/// use icu::datetime::mock::time_zone::MockTimeZone;
/// use icu::datetime::mock::zoned_datetime::MockZonedDateTime;
///
/// let dt: MockDateTime = "2020-10-14T13:21:00".parse()
///     .expect("Failed to parse a datetime.");
///
/// let tz: MockTimeZone = "+05:00".parse()
///     .expect("Failed to parse a time zone.");
///
/// let zdt1 = MockZonedDateTime::new(dt, tz);
/// let zdt2: MockZonedDateTime = "2020-10-14T13:21:00+05:00".parse()
///     .expect("Failed to parse a zoned datetime.");
/// ```
/// [`ZonedDateTimeFormat`]: crate::zoned_datetime::ZonedDateTimeFormat
#[derive(Debug, Default)]
pub struct MockZonedDateTime {
    /// The datetime component.
    pub datetime: MockDateTime,
    /// The time zone component.
    pub time_zone: MockTimeZone,
}

impl MockZonedDateTime {
    /// Creates a new [`MockZonedDateTime`] from an already validated [`MockDateTime`] and [`MockTimeZone`].
    pub const fn new(datetime: MockDateTime, time_zone: MockTimeZone) -> Self {
        Self {
            datetime,
            time_zone,
        }
    }
}

impl FromStr for MockZonedDateTime {
    type Err = DateTimeError;
    /// Parse a [`MockZonedDateTime`] from a string.
    ///
    /// This utility is for easily creating dates, not a complete robust solution. The
    /// string must take a specific form of the ISO 8601 format:
    /// `YYYY-MM-DDThh:mm:ssZ`,
    /// `YYYY-MM-DDThh:mm:ss±hh`,
    /// `YYYY-MM-DDThh:mm:ss±hhmm`,
    /// `YYYY-MM-DDThh:mm:ss±hh:mm`,
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::mock::zoned_datetime::MockZonedDateTime;
    ///
    /// let date: MockZonedDateTime = "2020-10-14T13:21:00+05:30".parse()
    ///     .expect("Failed to parse a zoned datetime.");
    /// ```
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let datetime = MockDateTime::from_str(input)?;
        let time_zone = match input
            .rfind(|c| c == '+' || /* ASCII */ c == '-' || /* U+2212 */ c == '−' || c == 'Z')
        {
            Some(index) => MockTimeZone::from_str(&input[index..])?,
            None => return Err(DateTimeError::InvalidTimeZoneOffset),
        };

        Ok(Self {
            datetime,
            time_zone,
        })
    }
}

impl DateInput for MockZonedDateTime {
    fn year(&self) -> Option<Year> {
        self.datetime.year()
    }

    fn month(&self) -> Option<Month> {
        self.datetime.month()
    }

    fn day_of_month(&self) -> Option<DayOfMonth> {
        self.datetime.day_of_month()
    }

    fn iso_weekday(&self) -> Option<IsoWeekday> {
        self.datetime.iso_weekday()
    }

    fn day_of_year_info(&self) -> Option<DayOfYearInfo> {
        self.datetime.day_of_year_info()
    }
}

impl IsoTimeInput for MockZonedDateTime {
    fn hour(&self) -> Option<IsoHour> {
        self.datetime.hour()
    }

    fn minute(&self) -> Option<IsoMinute> {
        self.datetime.minute()
    }

    fn second(&self) -> Option<IsoSecond> {
        self.datetime.second()
    }

    fn fraction(&self) -> Option<FractionalSecond> {
        self.datetime.fraction()
    }
}

impl TimeZoneInput for MockZonedDateTime {
    fn gmt_offset(&self) -> GmtOffset {
        self.time_zone.gmt_offset()
    }

    fn time_zone_id(&self) -> Option<&str> {
        self.time_zone.time_zone_id()
    }

    fn metazone_id(&self) -> Option<&str> {
        self.time_zone.metazone_id()
    }

    fn time_variant(&self) -> Option<&TinyStr8> {
        self.time_zone.time_variant()
    }
}
