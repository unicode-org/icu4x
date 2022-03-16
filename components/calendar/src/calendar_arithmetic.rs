// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{types, Calendar, DateDuration, DateDurationUnit};
use core::marker::PhantomData;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct ArithmeticDate<C: CalendarArithmetic> {
    pub year: i32,
    /// 1-based month of year
    pub month: u8,
    /// 1-based day of month
    pub day: u8,
    pub marker: PhantomData<C>,
}

pub trait CalendarArithmetic: Calendar {
    fn month_days(month: u8, year: i32) -> u8;
    fn months_for_every_year() -> u8;
    fn is_leap_year(year: i32) -> bool;
}

impl<C: CalendarArithmetic> ArithmeticDate<C> {
    #[inline]
    pub fn offset_date(&mut self, mut offset: DateDuration<C>) {
        self.year += offset.years;
        self.month += offset.months as u8;
        offset.months = 0;

        offset.days += offset.weeks * 7;

        offset.days += self.day as i32 - 1;
        self.day = 1;

        while offset.days != 0 {
            if offset.days < 0 {
                self.month -= 1;
                let month_days = C::month_days(self.month, self.year);
                if (-offset.days) > month_days as i32 {
                    offset.days += month_days as i32;
                } else {
                    self.day = 1 + (month_days as i8 + offset.days as i8) as u8;
                    offset.days = 0;
                }
            } else {
                let month_days = C::month_days(self.month, self.year);

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
        let months_in_year = C::months_for_every_year();
        let mut days: u32 = 0;
        for _month in 1..=months_in_year {
            days += C::month_days(self.month, self.year) as u32;
        }
        days
    }

    #[inline]
    pub fn months_in_year(&self) -> u8 {
        C::months_for_every_year() as u8
    }

    #[inline]
    pub fn days_in_month(&self) -> u8 {
        C::month_days(self.month, self.year)
    }

    #[inline]
    pub fn day_of_year(&self) -> u32 {
        let mut day_of_year = 0;
        for _month in 1..self.month {
            day_of_year += C::month_days(self.month, self.year) as u32;
        }
        day_of_year + (self.day as u32)
    }

    #[inline]
    pub fn day_of_month(&self) -> types::DayOfMonth {
        types::DayOfMonth(self.day.into())
    }
}
