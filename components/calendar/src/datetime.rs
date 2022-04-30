// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::types::Time;
use crate::{AsCalendar, Date, Iso};

/// A date+time for a given calendar
///
/// This can work with wrappers arount [`Calendar`](crate::Calendar) types,
/// e.g. `Rc<C>`, via the [`AsCalendar`] trait, much like
/// [`Date`]
#[derive(Debug)]
#[allow(clippy::exhaustive_structs)] // this type is stable
pub struct DateTime<A: AsCalendar> {
    pub date: Date<A>,
    pub time: Time,
}

impl<A: AsCalendar> DateTime<A> {
    pub fn new(date: Date<A>, time: Time) -> Self {
        DateTime { date, time }
    }

    /// Construct a DateTime from an ISO datetime and some calendar representation
    #[inline]
    pub fn new_from_iso(iso: DateTime<Iso>, calendar: A) -> Self {
        let date = Date::new_from_iso(iso.date, calendar);
        DateTime {
            date,
            time: iso.time,
        }
    }

    /// Convert the DateTime to an ISO DateTime
    #[inline]
    pub fn to_iso(&self) -> DateTime<Iso> {
        DateTime {
            date: self.date.to_iso(),
            time: self.time,
        }
    }

    /// Convert the DateTime to a DateTime in a different calendar
    #[inline]
    pub fn to_calendar<A2: AsCalendar>(&self, calendar: A2) -> DateTime<A2> {
        DateTime {
            date: self.date.to_calendar(calendar),
            time: self.time,
        }
    }
}
