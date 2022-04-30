// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::any_calendar::{AnyCalendar, IncludedInAnyCalendar};
use crate::{types, Calendar, DateDuration, DateDurationUnit, Iso};
use alloc::rc::Rc;
use core::fmt;

/// Types that contain a calendar
///
/// This allows one to use [`Date`] with wrappers around calendars,
/// e.g. reference counted calendars.
pub trait AsCalendar {
    /// The calendar being wrapped
    type Calendar: Calendar;
    /// Obtain the inner calendar
    fn as_calendar(&self) -> &Self::Calendar;
}

impl<C: Calendar> AsCalendar for C {
    type Calendar = C;
    #[inline]
    fn as_calendar(&self) -> &Self {
        self
    }
}

impl<C: Calendar> AsCalendar for Rc<C> {
    type Calendar = C;
    #[inline]
    fn as_calendar(&self) -> &C {
        &*self
    }
}

/// A date for a given calendar
///
/// This can work with wrappers around [`Calendar`] types,
/// e.g. `Rc<C>`, via the [`AsCalendar`] trait
pub struct Date<A: AsCalendar> {
    pub(crate) inner: <A::Calendar as Calendar>::DateInner,
    calendar: A,
}

impl<A: AsCalendar> Date<A> {
    /// Construct a date from an ISO date and some calendar representation
    #[inline]
    pub fn new_from_iso(iso: Date<Iso>, calendar: A) -> Self {
        let inner = calendar.as_calendar().date_from_iso(iso);
        Date { inner, calendar }
    }

    /// Convert the Date to an ISO Date
    #[inline]
    pub fn to_iso(&self) -> Date<Iso> {
        self.calendar.as_calendar().date_to_iso(self.inner())
    }

    /// Convert the Date to a date in a different calendar
    #[inline]
    pub fn to_calendar<A2: AsCalendar>(&self, calendar: A2) -> Date<A2> {
        Date::new_from_iso(self.to_iso(), calendar)
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
    pub fn day_of_week(&self) -> types::IsoWeekday {
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
        largest_unit: DateDurationUnit,
        smallest_unit: DateDurationUnit,
    ) -> DateDuration<A::Calendar> {
        self.calendar.as_calendar().until(
            self.inner(),
            other.inner(),
            other.calendar.as_calendar(),
            largest_unit,
            smallest_unit,
        )
    }

    /// The calendar-specific year represented by `self`
    #[inline]
    pub fn year(&self) -> types::Year {
        self.calendar.as_calendar().year(&self.inner)
    }

    /// The calendar-specific month represented by `self`
    #[inline]
    pub fn month(&self) -> types::Month {
        self.calendar.as_calendar().month(&self.inner)
    }

    /// The calendar-specific day-of-month represented by `self`
    #[inline]
    pub fn day_of_month(&self) -> types::DayOfMonth {
        self.calendar.as_calendar().day_of_month(&self.inner)
    }

    /// The calendar-specific day-of-month represented by `self`
    #[inline]
    pub fn day_of_year_info(&self) -> types::DayOfYearInfo {
        self.calendar.as_calendar().day_of_year_info(&self.inner)
    }

    /// Construct a date from raw values for a given calendar. This does not check any
    /// invariants for the date and calendar, and should only be called by calendar implementations.
    ///
    /// Calling this outside of calendar implementations is sound, but calendar implementations are not
    /// expected to do anything sensible with such invalid dates.
    ///
    /// AnyCalendar *will* panic if AnyCalendar [`Date`] objects with mismatching
    /// date and calendar types are constructed
    #[inline]
    pub fn from_raw(inner: <A::Calendar as Calendar>::DateInner, calendar: A) -> Self {
        Self { inner, calendar }
    }

    /// Get the inner date implementation. Should not be called outside of calendar implementations
    #[inline]
    pub fn inner(&self) -> &<A::Calendar as Calendar>::DateInner {
        &self.inner
    }

    /// Get a reference to the contained calendar
    #[inline]
    pub fn calendar(&self) -> &A::Calendar {
        self.calendar.as_calendar()
    }
}

impl<C: IncludedInAnyCalendar, A: AsCalendar<Calendar = C>> Date<A> {
    /// Type-erase the date, converting it to a date for [`AnyCalendar`]
    pub fn to_any(&self) -> Date<AnyCalendar> {
        Date::from_raw(
            C::date_to_any(self.inner()),
            self.calendar().to_any_cloned(),
        )
    }
}

impl<C: Calendar> Date<C> {
    /// Wrap the calendar type in `Rc<T>`
    ///
    /// Useful when paired with [`Self::to_any()`] to obtain a `Date<Rc<AnyCalendar>>`
    pub fn wrap_calendar_in_rc(self) -> Date<Rc<C>> {
        Date::from_raw(self.inner, Rc::new(self.calendar))
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

impl<A: AsCalendar> fmt::Debug for Date<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "Date({:?}, for calendar {})",
            self.inner,
            self.calendar.as_calendar().debug_name()
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
