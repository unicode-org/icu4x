// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::Calendar;
use std::fmt;
use std::marker::PhantomData;

#[derive(Copy, Clone, Eq, PartialEq)]
/// A duration between two dates
pub struct DateDuration<C: Calendar + ?Sized> {
    pub years: i32,
    pub months: i32,
    pub weeks: i32,
    pub days: i32,
    pub marker: PhantomData<C>,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum DurationUnit {
    Years,
    Months,
    Weeks,
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
