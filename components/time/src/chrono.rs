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

impl From<chrono::NaiveTime> for Time {
    fn from(chrono: chrono::NaiveTime) -> Self {
        use chrono::Timelike;
        Time {
            hour: Hour(chrono.hour() as u8),
            minute: Minute(chrono.minute() as u8),
            second: Second(chrono.second() as u8),
            subsecond: Nanosecond(chrono.nanosecond()),
        }
    }
}

impl From<chrono::NaiveDateTime> for DateTime<Gregorian> {
    fn from(chrono: chrono::NaiveDateTime) -> Self {
        Self {
            date: chrono.date().into(),
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

#[cfg(feature = "compiled_data")]
impl From<&chrono::DateTime<chrono_tz::Tz>> for ZonedDateTime<Gregorian, TimeZoneInfo<Base>> {
    fn from(chrono: &chrono::DateTime<chrono_tz::Tz>) -> Self {
        use chrono::Offset;

        let date = chrono.date_naive().into();
        let time = chrono.time().into();
        let zone =
            TimeZone::from(chrono.timezone()).with_offset(Some(chrono.offset().fix().into()));

        ZonedDateTime { date, time, zone }
    }
}

#[cfg(feature = "compiled_data")]
impl From<&chrono::DateTime<chrono_tz::Tz>> for ZonedDateTime<Gregorian, TimeZoneInfo<AtTime>> {
    fn from(chrono: &chrono::DateTime<chrono_tz::Tz>) -> Self {
        let ZonedDateTime::<_, TimeZoneInfo<Base>> { date, time, zone } = chrono.into();

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
