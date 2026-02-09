// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_calendar::Date;
use icu_calendar::Gregorian;

#[cfg(feature = "compiled_data")]
use crate::zone::models::AtTime;
use crate::zone::UtcOffset;
use crate::{DateTime, Time};
#[cfg(feature = "compiled_data")]
use crate::{TimeZone, TimeZoneInfo, ZonedDateTime};

impl From<jiff::civil::Time> for Time {
    fn from(jiff: jiff::civil::Time) -> Self {
        #[allow(clippy::unwrap_used)] // jiff returns valid fields
        Time::try_new(
            jiff.hour() as u8,
            jiff.minute() as u8,
            jiff.second() as u8,
            jiff.millisecond() as u32 * 1_000_000
                + jiff.microsecond() as u32 * 1_000
                + jiff.nanosecond() as u32,
        )
        .unwrap()
    }
}

impl From<jiff::civil::DateTime> for DateTime<Gregorian> {
    fn from(jiff: jiff::civil::DateTime) -> Self {
        #[allow(clippy::unwrap_used)] // jiff returns valid fields
        Self {
            date: Date::try_new_gregorian(jiff.year() as i32, jiff.month() as u8, jiff.day() as u8)
                .unwrap(),
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
        crate::zone::IanaParser::new().parse(jiff.iana_name().unwrap_or_default())
    }
}

#[cfg(feature = "compiled_data")]
impl From<&jiff::Zoned> for ZonedDateTime<Gregorian, TimeZoneInfo<AtTime>> {
    fn from(jiff: &jiff::Zoned) -> Self {
        let DateTime { date, time } = jiff.datetime().into();

        let zone = TimeZone::from(jiff.time_zone())
            .with_offset(Some(jiff.offset().into()))
            .at_date_time_iso(DateTime {
                date: date.to_calendar(icu_calendar::Iso),
                time,
            });

        ZonedDateTime { date, time, zone }
    }
}
