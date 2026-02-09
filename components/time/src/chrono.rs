// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use chrono::Offset;
use icu_calendar::Date;
use icu_calendar::Gregorian;

#[cfg(feature = "compiled_data")]
use crate::zone::models::AtTime;
use crate::zone::models::Base;
use crate::zone::UtcOffset;
use crate::{DateTime, Time, TimeZone, TimeZoneInfo, ZonedDateTime};

impl From<chrono::NaiveTime> for Time {
    fn from(chrono: chrono::NaiveTime) -> Self {
        use chrono::Timelike;
        #[allow(clippy::unwrap_used)] // chrono returns valid fields
        Time::try_new(
            chrono.hour() as u8,
            chrono.minute() as u8,
            chrono.second() as u8,
            chrono.nanosecond(),
        )
        .unwrap()
    }
}

impl From<chrono::NaiveDateTime> for DateTime<Gregorian> {
    fn from(chrono: chrono::NaiveDateTime) -> Self {
        use chrono::Datelike;
        #[allow(clippy::unwrap_used)] // chrono returns valid fields
        Self {
            date: Date::try_new_gregorian(chrono.year(), chrono.month() as u8, chrono.day() as u8)
                .unwrap(),
            time: chrono.time().into(),
        }
    }
}

impl From<chrono::FixedOffset> for UtcOffset {
    fn from(chrono: chrono::FixedOffset) -> Self {
        UtcOffset::from_seconds_unchecked(chrono.local_minus_utc())
    }
}

#[cfg(feature = "compiled_data")]
impl From<chrono_tz::Tz> for TimeZone {
    fn from(chrono: chrono_tz::Tz) -> Self {
        crate::zone::IanaParser::new().parse(chrono.name())
    }
}

impl<Tz: chrono::TimeZone> From<chrono::DateTime<Tz>>
    for ZonedDateTime<Gregorian, TimeZoneInfo<Base>>
{
    fn from(chrono: chrono::DateTime<Tz>) -> Self {
        let DateTime { date, time } = chrono.naive_local().into();

        let zone = TimeZone::UNKNOWN.with_offset(Some(chrono.offset().fix().into()));

        ZonedDateTime { date, time, zone }
    }
}

#[cfg(feature = "compiled_data")]
impl From<&chrono::DateTime<chrono_tz::Tz>> for ZonedDateTime<Gregorian, TimeZoneInfo<AtTime>> {
    fn from(chrono: &chrono::DateTime<chrono_tz::Tz>) -> Self {
        let DateTime { date, time } = chrono.naive_local().into();

        let zone = TimeZone::from(chrono.timezone())
            .with_offset(Some(chrono.offset().fix().into()))
            .at_date_time_iso(DateTime {
                date: date.to_calendar(icu_calendar::Iso),
                time,
            });

        ZonedDateTime { date, time, zone }
    }
}
