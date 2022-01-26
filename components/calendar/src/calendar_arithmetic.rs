// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{Calendar, DateDuration, DateDurationUnit};
use core::marker::PhantomData;

pub struct ArithmeticDate<C: CalendarArithmetic> {
    year: i32,
    month: u8,
    day: u8,
    marker: PhantomData<C>,
}

pub trait CalendarArithmetic: Calendar {
    fn month_lengths(year: i32) -> [u8; 12];
    fn months_for_every_year() -> Option<u8>;
    fn is_leap_year(year: i32) -> bool;
}

impl<C: CalendarArithmetic> ArithmeticDate<C> {
    #[inline]
    pub fn offset_date(&self, mut offset: DateDuration<C::Calendar>) {
        let month_lengths = Self::month_lengths(self.year);
        self.year += offset.years;
        self.month += offset.months;
        offset.months = 0;

        offset.days += offset.weeks * 7;

        offset.days += self.day - 1;
        self.day = 1;

        while offset.days != 0 {
            if offset.days < 0 {
                self.month -= 1;
                let month_days = month_lengths[self.month];
                if (-offset.days) > month_days as i32 {
                    offset.days += month_days as i32;
                } else {
                    self.day = 1 + (month_days as i8 + offset.days as i8) as u8;
                    offset.days = 0;
                }
            } else {
                let month_days = month_lengths[self.month];

                if offset.days >= month_days as i32 {
                    self.month += 1;
                    offset.days -= month_days as i32;
                } else {
                    self.day += offset.days as u8;
                    offset.days = 0;
                }
            }
        }
    }

    #[inline]
    pub fn days_in_year(&self) {
        let months = self.month_lengths(self.year);
        months.iter().sum()
    }

    #[inline]
    pub fn days_in_month(&self) -> u8 {
        let months = self.month_lengths(self.year);
        months[self.month]
    }
}
