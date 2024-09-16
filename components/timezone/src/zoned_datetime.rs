// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_calendar::{AsCalendar, Date, Iso, Time};

use crate::{FormattableTimeZone, TimeZone};
use crate::{MetazoneCalculator, ZoneOffsetCalculator};

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

    /// Convert the [`ZonedDateTime`] to a [`FormattableZonedDateTime`].
    pub fn to_formattable(
        self,
        metazone_calculator: &MetazoneCalculator,
        zone_offset_calculator: &ZoneOffsetCalculator,
    ) -> FormattableZonedDateTime<A> {
        FormattableZonedDateTime {
            zone: self.zone.to_formattable_at(
                metazone_calculator,
                zone_offset_calculator,
                &icu_calendar::DateTime::new(self.date.to_iso(), self.time),
            ),
            date: self.date,
            time: self.time,
        }
    }
}

/// A date and time local to a specified custom time zone.
#[derive(Debug)]
#[allow(clippy::exhaustive_structs)] // this type is stable
pub struct FormattableZonedDateTime<A: AsCalendar> {
    date: Date<A>,
    time: Time,
    zone: FormattableTimeZone,
}

impl<A: AsCalendar> FormattableZonedDateTime<A> {
    /// Creates a new [`FormattableZonedDateTime`] in the UTC time zone.
    pub fn new_in_utc(date: Date<A>, time: Time) -> Self {
        Self {
            date,
            time,
            zone: FormattableTimeZone::utc(),
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
    pub fn zone(&self) -> FormattableTimeZone {
        self.zone
    }

    /// Convert the FormattableZonedDateTime to an ISO FormattableZonedDateTime
    #[inline]
    pub fn to_iso(&self) -> FormattableZonedDateTime<Iso> {
        FormattableZonedDateTime {
            date: self.date.to_iso(),
            time: self.time,
            zone: self.zone,
        }
    }

    /// Convert the FormattableZonedDateTime to a FormattableZonedDateTime in a different calendar
    #[inline]
    pub fn to_calendar<A2: AsCalendar>(&self, calendar: A2) -> FormattableZonedDateTime<A2> {
        FormattableZonedDateTime {
            date: self.date.to_calendar(calendar),
            time: self.time,
            zone: self.zone,
        }
    }
}
