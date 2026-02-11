// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_calendar::Gregorian;

#[cfg(feature = "compiled_data")]
use crate::zone::models::{AtTime, Base};
use crate::zone::UtcOffset;
use crate::{DateTime, Hour, Minute, Nanosecond, Second, Time};
#[cfg(feature = "compiled_data")]
use crate::{TimeZone, TimeZoneInfo, ZonedDateTime};

impl From<jiff::civil::Time> for Time {
    fn from(jiff: jiff::civil::Time) -> Self {
        Time {
            hour: Hour(jiff.hour() as u8),
            minute: Minute(jiff.minute() as u8),
            second: Second(jiff.second() as u8),
            subsecond: Nanosecond(
                jiff.millisecond() as u32 * 1_000_000
                    + jiff.microsecond() as u32 * 1_000
                    + jiff.nanosecond() as u32,
            ),
        }
    }
}

impl From<jiff::civil::DateTime> for DateTime<Gregorian> {
    fn from(jiff: jiff::civil::DateTime) -> Self {
        Self {
            date: jiff.date().into(),
            time: jiff.time().into(),
        }
    }
}

impl From<jiff::tz::Offset> for UtcOffset {
    fn from(jiff: jiff::tz::Offset) -> Self {
        UtcOffset::from_seconds_unchecked(jiff.seconds())
    }
}

#[cfg(feature = "compiled_data")]
impl From<&jiff::tz::TimeZone> for TimeZone {
    fn from(jiff: &jiff::tz::TimeZone) -> Self {
        jiff.iana_name()
            .map(|i| crate::zone::IanaParser::new().parse(i))
            .unwrap_or(TimeZone::UNKNOWN)
    }
}

#[cfg(feature = "compiled_data")]
impl From<&jiff::Zoned> for ZonedDateTime<Gregorian, TimeZoneInfo<Base>> {
    fn from(jiff: &jiff::Zoned) -> Self {
        let date = jiff.date().into();
        let time = jiff.time().into();
        let zone = TimeZone::from(jiff.time_zone()).with_offset(Some(jiff.offset().into()));

        ZonedDateTime { date, time, zone }
    }
}

#[cfg(feature = "compiled_data")]
impl From<&jiff::Zoned> for ZonedDateTime<Gregorian, TimeZoneInfo<AtTime>> {
    fn from(jiff: &jiff::Zoned) -> Self {
        let ZonedDateTime::<_, TimeZoneInfo<Base>> { date, time, zone } = jiff.into();

        ZonedDateTime {
            date,
            time,
            zone: zone.at_date_time_iso(DateTime {
                date: date.to_calendar(icu_calendar::Iso),
                time,
            }),
        }
    }
}
