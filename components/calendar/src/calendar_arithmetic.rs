// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{Calendar, DateDuration, DateDurationUnit};
use core::marker::PhantomData;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct ArithmeticDate<C: CalendarArithmetic> {
    pub year: i32,
    pub month: u8,
    pub day: u8,
    marker: PhantomData<C>,
}

pub trait CalendarArithmetic: Calendar {
    fn month_lengths(year: i32) -> [u8; 12];
    fn months_for_every_year() -> u8;
    fn is_leap_year(year: i32) -> bool;
}

impl<C: CalendarArithmetic> ArithmeticDate<C> {
    #[inline]
    pub fn offset_date(&self, mut offset: DateDuration<C>) {
        let month_lengths = C::month_lengths(self.year);
        self.year += offset.years;
        self.month += offset.months as u8;
        offset.months = 0;

        offset.days += offset.weeks * 7;

        offset.days += self.day as i32 - 1;
        self.day = 1;

        while offset.days != 0 {
            if offset.days < 0 {
                self.month -= 1;
                let month_days = month_lengths[self.month as usize];
                if (-offset.days) > month_days as i32 {
                    offset.days += month_days as i32;
                } else {
                    self.day = 1 + (month_days as i8 + offset.days as i8) as u8;
                    offset.days = 0;
                }
            } else {
                let month_days = month_lengths[self.month as usize];

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
    pub fn until(
        &self,
        date2: ArithmeticDate<C>,
        _largest_unit: DateDurationUnit,
        _smaller_unti: DateDurationUnit,
    ) -> DateDuration<C> {
        DateDuration::new(
            self.year - date2.year,
            self.month as i32 - date2.month as i32,
            0,
            self.day as i32 - date2.day as i32,
        )
    }

    #[inline]
    pub fn days_in_year(&self) -> u32 {
        let months = C::month_lengths(self.year);
        let mut days: u32 = 0;
        for month in months {
            days += month as u32;
        }
        days
    }

    #[inline]
    pub fn days_in_month(&self) -> u8 {
        let months = C::month_lengths(self.year);
        months[self.month as usize]
    }

    #[inline]
    pub fn months_in_year(&self) -> u8 {
        C::month_lengths(self.year).len() as u8
    }

    #[inline]
    pub fn day_of_year(&self) -> u32 {
        let months = C::month_lengths(self.year);
        let mut day_of_year = 0;
        for month in 1..self.month {
            day_of_year += months[month as usize] as u32;
        }
        day_of_year + (self.day as u32)
    }
}
