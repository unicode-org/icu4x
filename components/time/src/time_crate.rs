// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_calendar::Gregorian;

use crate::zone::UtcOffset;
use crate::ZonedDateTime;
use crate::{DateTime, Hour, Minute, Nanosecond, Second, Time};

impl From<time::Time> for Time {
    fn from(time: time::Time) -> Self {
        Time {
            hour: Hour(time.hour()),
            minute: Minute(time.minute()),
            second: Second(time.second()),
            subsecond: Nanosecond(time.nanosecond()),
        }
    }
}

impl From<time::PrimitiveDateTime> for DateTime<Gregorian> {
    fn from(chrono: time::PrimitiveDateTime) -> Self {
        Self {
            date: chrono.date().into(),
            time: chrono.time().into(),
        }
    }
}

impl From<time::UtcOffset> for UtcOffset {
    fn from(other: time::UtcOffset) -> Self {
        UtcOffset::from_seconds_unchecked(other.whole_seconds())
    }
}

impl From<&time::OffsetDateTime> for ZonedDateTime<Gregorian, UtcOffset> {
    fn from(other: &time::OffsetDateTime) -> Self {
        let date = other.date().into();
        let time = other.time().into();
        let zone = other.offset().into();

        ZonedDateTime { date, time, zone }
    }
}
