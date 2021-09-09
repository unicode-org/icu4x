// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::Calendar;
use core::fmt;
use core::marker::PhantomData;

#[derive(Copy, Clone, Eq, PartialEq)]
/// A duration between two dates
pub struct DateDuration<C: Calendar + ?Sized> {
    /// The number of years
    pub years: i32,
    /// The number of months
    pub months: i32,
    /// The number of weeks
    pub weeks: i32,
    /// The number of days
    pub days: i32,
    /// A marker for the calendar
    pub marker: PhantomData<C>,
}

/// A "duration unit" used to specify the minimum or maximum duration of time to
/// care about
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum DateDurationUnit {
    /// Duration in years
    Years,
    /// Duration in months
    Months,
    /// Duration in weeks
    Weeks,
    /// Duration in days
    Days,
}

impl<C: Calendar + ?Sized> Default for DateDuration<C> {
    fn default() -> Self {
        Self {
            years: 0,
            months: 0,
            weeks: 0,
            days: 0,
            marker: PhantomData,
        }
    }
}

impl<C: Calendar + ?Sized> DateDuration<C> {
    /// Construct a DateDuration
    pub fn new(years: i32, months: i32, weeks: i32, days: i32) -> Self {
        DateDuration {
            years,
            months,
            weeks,
            days,
            marker: PhantomData,
        }
    }
}

impl<C: Calendar> fmt::Debug for DateDuration<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_struct("DateDuration")
            .field("years", &self.years)
            .field("months", &self.months)
            .field("weeks", &self.weeks)
            .field("days", &self.days)
            .finish()
    }
}
