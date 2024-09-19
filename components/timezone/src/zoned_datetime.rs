// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_calendar::{AsCalendar, Date, Iso, Time};

use crate::{FormattableTimeZone, MetazoneCalculator, TimeZone, ZoneOffsetCalculator};

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
    #[cfg(feature = "compiled_data")]
    pub fn into_formattable(self) -> FormattableZonedDateTime<A> {
        use crate::{MetazoneCalculator, ZoneOffsetCalculator};
        FormattableZonedDateTime {
            zone: MetazoneCalculator::new().compute(
                self.zone,
                &ZoneOffsetCalculator::new(),
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

impl MetazoneCalculator {
    /// Converts a [`ZonedDateTime`] to a [`FormattableZonedDateTime`] at the given datetime.
    pub fn compute_dt<A: AsCalendar>(
        &self,
        zoned: ZonedDateTime<A>,
        zoc: &ZoneOffsetCalculator,
    ) -> FormattableZonedDateTime<A> {
        FormattableZonedDateTime {
            zone: self.compute(
                zoned.zone,
                zoc,
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
