// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{Calendar, DateDuration, DurationUnit, Iso};
use std::fmt;

pub trait AsCalendar {
    type Calendar: Calendar;
    fn as_calendar(&self) -> &Self::Calendar;
}

impl<C: Calendar> AsCalendar for C {
    type Calendar = C;
    #[inline]
    fn as_calendar(&self) -> &Self {
        self
    }
}

pub struct Date<A: AsCalendar> {
    inner: <A::Calendar as Calendar>::DateInner,
    calendar: A,
}

impl<A: AsCalendar> Date<A> {
    /// Construct a date from an ISO date and some calendar representation
    #[inline]
    pub fn new_from_iso(iso: Date<Iso>, calendar: A) -> Self {
        let inner = calendar.as_calendar().date_from_iso(iso);
        Date { inner, calendar }
    }

    #[inline]
    pub fn to_iso(&self) -> Date<Iso> {
        self.calendar.as_calendar().date_to_iso(self.inner())
    }

    /// The number of months in the year of this date
    #[inline]
    pub fn months_in_year(&self) -> u8 {
        self.calendar.as_calendar().months_in_year(self.inner())
    }

    /// The number of days in the year of this date
    #[inline]
    pub fn days_in_year(&self) -> u32 {
        self.calendar.as_calendar().days_in_year(self.inner())
    }

    /// The number of days in the month of this date
    #[inline]
    pub fn days_in_month(&self) -> u8 {
        self.calendar.as_calendar().days_in_month(self.inner())
    }

    /// The day of the week for this date
    ///
    /// Monday is 1, Sunday is 7, according to ISO
    #[inline]
    pub fn day_of_week(&self) -> u8 {
        self.calendar.as_calendar().day_of_week(self.inner())
    }

    /// Add a `duration` to this date, mutating it
    #[inline]
    pub fn add(&mut self, duration: DateDuration<A::Calendar>) {
        self.calendar
            .as_calendar()
            .offset_date(&mut self.inner, duration)
    }

    /// Add a `duration` to this date, returning the new one
    #[inline]
    pub fn added(mut self, duration: DateDuration<A::Calendar>) -> Self {
        self.add(duration);
        self
    }

    /// Calculating the duration between `other - self`
    #[inline]
    pub fn until<B: AsCalendar<Calendar = A::Calendar>>(
        &self,
        other: &Date<B>,
        largest_unit: DurationUnit,
        smallest_unit: DurationUnit,
    ) -> DateDuration<A::Calendar> {
        self.calendar
            .as_calendar()
            .until(self.inner(), other.inner(), largest_unit, smallest_unit)
    }

    /// Construct a date from raw values for a given calendar. This does not check any
    /// invariants for the date and calendar, and should only be called by calendar implementations.
    ///
    /// Calling this outside of calendar implementations is sound, but calendar implementations are not
    /// expected to do anything sensible with such invalid dates.
    #[inline]
    pub fn from_raw(inner: <A::Calendar as Calendar>::DateInner, calendar: A) -> Self {
        Self { inner, calendar }
    }

    /// Get the inner date implementation. Should not be called outside of calendar implementations
    #[inline]
    pub fn inner(&self) -> &<A::Calendar as Calendar>::DateInner {
        &self.inner
    }
}

impl<C, A, B> PartialEq<Date<B>> for Date<A>
where
    C: Calendar,
    A: AsCalendar<Calendar = C>,
    B: AsCalendar<Calendar = C>,
{
    fn eq(&self, other: &Date<B>) -> bool {
        self.inner.eq(&other.inner)
    }
}

impl<A: AsCalendar> Eq for Date<A> {}

impl<A: AsCalendar> fmt::Debug for Date<A>
where
    <A::Calendar as Calendar>::DateInner: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "Date({:?}, for calendar {})",
            self.inner,
            A::Calendar::debug_name()
        )
    }
}

impl<A: AsCalendar + Clone> Clone for Date<A> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            calendar: self.calendar.clone(),
        }
    }
}
