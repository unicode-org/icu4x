// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_calendar::{AsCalendar, Date, Iso, Time};

use crate::{ResolvedTimeZone, TimeZone, TimeZoneCalculator};

/// A date and time local to a specified custom time zone.
#[derive(Debug)]
#[allow(clippy::exhaustive_structs)] // this type is stable
pub struct ZonedDateTime<A: AsCalendar> {
    /// The date, local to the time zone
    pub date: Date<A>,
    /// The time, local to the time zone
    pub time: Time,
    /// The time zone
    pub zone: TimeZone,
}

impl<A: AsCalendar> ZonedDateTime<A> {
    /// Convert the ZonedDateTime to an ISO ZonedDateTime
    #[inline]
    pub fn to_iso(&self) -> ZonedDateTime<Iso> {
        ZonedDateTime {
            date: self.date.to_iso(),
            time: self.time,
            zone: self.zone,
        }
    }

    /// Convert the ZonedDateTime to a ZonedDateTime in a different calendar
    #[inline]
    pub fn to_calendar<A2: AsCalendar>(&self, calendar: A2) -> ZonedDateTime<A2> {
        ZonedDateTime {
            date: self.date.to_calendar(calendar),
            time: self.time,
            zone: self.zone,
        }
    }
}

/// A date and time local to a specified custom time zone.
#[derive(Debug)]
#[allow(clippy::exhaustive_structs)] // this type is stable
pub struct ResolvedZonedDateTime<A: AsCalendar> {
    date: Date<A>,
    time: Time,
    zone: ResolvedTimeZone,
}

impl<A: AsCalendar> ResolvedZonedDateTime<A> {
    /// Creates a new [`ResolvedZonedDateTime`] in the UTC time zone.
    pub fn new_in_utc(date: Date<A>, time: Time) -> Self {
        Self {
            date,
            time,
            zone: ResolvedTimeZone::utc(),
        }
    }

    /// The date, local to the time zone
    pub fn date(&self) -> &Date<A> {
        &self.date
    }

    /// The time, local to the time zone
    pub fn time(&self) -> &Time {
        &self.time
    }

    /// The time zone
    pub fn zone(&self) -> ResolvedTimeZone {
        self.zone
    }

    /// Convert the ResolvedZonedDateTime to an ISO ResolvedZonedDateTime
    #[inline]
    pub fn to_iso(&self) -> ResolvedZonedDateTime<Iso> {
        ResolvedZonedDateTime {
            date: self.date.to_iso(),
            time: self.time,
            zone: self.zone,
        }
    }

    /// Convert the ResolvedZonedDateTime to a ResolvedZonedDateTime in a different calendar
    #[inline]
    pub fn to_calendar<A2: AsCalendar>(&self, calendar: A2) -> ResolvedZonedDateTime<A2> {
        ResolvedZonedDateTime {
            date: self.date.to_calendar(calendar),
            time: self.time,
            zone: self.zone,
        }
    }
}

impl TimeZoneCalculator {
    /// Converts a [`ZonedDateTime`] to a [`ResolvedZonedDateTime`] at the given datetime.
    pub fn resolve<A: AsCalendar>(&self, zoned: ZonedDateTime<A>) -> ResolvedZonedDateTime<A> {
        ResolvedZonedDateTime {
            zone: self.resolve_at(
                zoned.zone,
                &icu_calendar::DateTime {
                    date: zoned.date.to_iso(),
                    time: zoned.time,
                },
            ),
            date: zoned.date,
            time: zoned.time,
        }
    }
}
