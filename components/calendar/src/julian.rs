// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains types and implementations for the Julian calendar

use crate::iso::{Iso, IsoDateInner, IsoDay, IsoMonth, IsoYear};
use crate::{types, Calendar, Date, DateDuration, DateDurationUnit, DateTime, DateTimeError};
use core::convert::TryInto;

#[derive(Copy, Clone, Debug, Default)]
// The Julian calendar
pub struct Julian;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
// The inner date type used for representing Date<Georgian>
pub struct JulianDateInner(IsoDateInner);

impl Calendar for Julian {
    type DateInner = JulianDateInner;
    fn date_from_iso(&self, iso: Date<Iso>) -> JulianDateInner {
        let added_days = Self::calculate_day_difference_between_calendars(*iso.inner());
        Self::offset_date(
            &mut JulianDateInner(*iso.inner()),
            DateDuration::new(0, 0, 0, -added_days),
        )
    }

    fn date_to_iso(&self, date: &Self::DateInner) -> Date<Iso> {
        let added_days = Self::calculate_day_difference_between_calendars(date.0);
        let mut date = Self::offset_date(
            &mut JulianDateInner(date.0),
            DateDuration::new(0, 0, 0, added_days),
        );

        // Edge case when the year is divisible by 100, and not by 400
        if date.0.year.0 % 400 == 0 && u8::from(date.0.month) == 2 && u8::from(date.0.day) == 29 {
            date.0.month = 3.try_into().unwrap();
            date.0.day = 1.try_into().unwrap();
        }

        Date::from_raw(date.0, Iso)
    }

    fn months_in_year(&self, date: &Self::DateInner) -> u8 {
        Iso.months_in_year(&date.0)
    }

    fn days_in_year(&self, date: &Self::DateInner) -> u32 {
        if Self::is_leap_year(date.0.year) {
            366
        } else {
            365
        }
    }

    fn days_in_month(&self, date: &Self::DateInner) -> u8 {
        Self::days_in_month(date.0.year, date.0.month)
    }

    fn day_of_week(&self, date: &Self::DateInner) -> types::IsoWeekday {
        Iso.day_of_week(Julian.date_to_iso(&date).inner())
    }

    fn offset_date(&self, date: &mut Self::DateInner, offset: DateDuration<Self>) {
        Self::offset_date(date, offset);
    }

    #[allow(clippy::field_reassign_with_default)]
    fn until(
        &self,
        date1: &Self::DateInner,
        date2: &Self::DateInner,
        largest_unit: DateDurationUnit,
        smallest_unit: DateDurationUnit,
    ) -> DateDuration<Self> {
        Iso.until(&date1.0, &date2.0, largest_unit, smallest_unit)
            .cast_unit()
    }

    /// The calendar-specific year represented by `date`
    fn year(&self, date: &Self::DateInner) -> types::Year {
        date.0.year.into()
    }

    /// The calendar-specific month represented by `date`
    fn month(&self, date: &Self::DateInner) -> types::Month {
        date.0.month.into()
    }

    /// The calendar-specific day-of-month represented by `date`
    fn day_of_month(&self, date: &Self::DateInner) -> types::DayOfMonth {
        date.0.day.into()
    }

    fn day_of_year_info(&self, date: &Self::DateInner) -> types::DayOfYearInfo {
        Iso.day_of_year_info(Julian.date_to_iso(&date).inner())
    }

    fn debug_name() -> &'static str {
        "Julian"
    }
}

impl Julian {
    /// Construct a new Julian Calendar
    pub fn new() -> Self {
        Self
    }

    fn is_leap_year(year: IsoYear) -> bool {
        year.0 % 4 == 0
    }

    fn days_in_month(year: IsoYear, month: IsoMonth) -> u8 {
        match u8::from(month) {
            4 | 6 | 9 | 11 => 30,
            2 if Self::is_leap_year(year) => 29,
            2 => 28,
            _ => 31,
        }
    }

    fn calculate_day_difference_between_calendars(date: IsoDateInner) -> i32 {
        // March 1st 200 is the first day when both julian and georgian calendar fall on same day.
        // Using that date as base to calculate slack
        let year = date.year.0 - 200;
        let slack = year / 400;

        if year % 400 == 0 && u8::from(date.month) <= 2 {
            slack + 1
        } else {
            slack
        }
    }

    fn offset_date(date: &mut JulianDateInner, mut offset: DateDuration<Self>) -> JulianDateInner {
        date.0.year.0 += offset.years;
        date.0.add_months(offset.months);
        offset.months = 0;

        offset.days += offset.weeks * 7;
        offset.days += u8::from(date.0.day) as i32 - 1;
        date.0.day = 1.try_into().unwrap();

        while offset.days != 0 {
            if offset.days < 0 {
                date.0.add_months(-1);
                let month_days = Julian::days_in_month(date.0.year, date.0.month);
                if (-offset.days) > month_days as i32 {
                    offset.days += month_days as i32;
                } else {
                    date.0.day = (1 + (month_days as i8 + offset.days as i8) as u8)
                        .try_into()
                        .unwrap();
                    offset.days = 0;
                }
            } else {
                let month_days = Julian::days_in_month(date.0.year, date.0.month);
                if offset.days >= month_days as i32 {
                    date.0.add_months(1);
                    offset.days -= month_days as i32;
                } else {
                    date.0.day = (u8::from(date.0.day) + offset.days as u8)
                        .try_into()
                        .unwrap();
                    offset.days = 0;
                }
            }
        }

        *date
    }
}

impl Date<Julian> {
    /// Construct new Julian Date
    pub fn new_julian_date(
        year: IsoYear,
        month: IsoMonth,
        day: IsoDay,
    ) -> Result<Date<Julian>, DateTimeError> {
        let day_int = u8::from(day);
        let month_int = u8::from(month);

        if day_int > 28 {
            let bound = Julian::days_in_month(year, month);
            if day_int > bound {
                return Err(DateTimeError::OutOfRange);
            }
        }

        Ok(Date::from_raw(
            JulianDateInner {
                0: IsoDateInner {
                    day: day_int.try_into()?,
                    month: month_int.try_into()?,
                    year,
                },
            },
            Julian,
        ))
    }

    pub fn new_julian_date_from_integers(
        year: i32,
        month: u8,
        day: u8,
    ) -> Result<Date<Julian>, DateTimeError> {
        Self::new_julian_date(year.into(), month.try_into()?, day.try_into()?)
    }
}

impl DateTime<Julian> {
    /// Constrict a new Julian datetime form integers
    pub fn new_julian_datetime_from_integers(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Result<DateTime<Julian>, DateTimeError> {
        Ok(DateTime {
            date: Date::new_julian_date(year.into(), month.try_into()?, day.try_into()?)?,
            time: types::Time::try_new(hour, minute, second)?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::IsoWeekday;

    #[test]
    fn test_day_iso_to_julian() {
        // March 1st 200 is same on both calendars
        let iso_date = Date::new_iso_date_from_integers(200, 3, 1).unwrap();
        let julian_date = Julian.date_from_iso(iso_date);
        assert_eq!(julian_date.0.year.0, 200);
        assert_eq!(u8::from(julian_date.0.month), 3);
        assert_eq!(u8::from(julian_date.0.day), 1);
    }
}
