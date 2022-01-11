// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains types and implementations for the Julian calendar

use crate::iso::{Iso, IsoDateInner, IsoDay, IsoMonth, IsoYear};
use crate::{types, Calendar, Date, DateDuration, DateDurationUnit, DateTime, DateTimeError};
use core::convert::TryInto;

// Julian epoch is equivalent to fixed_from_iso of December 30th of 0 year
// 1st Jan of 1st year Julian is equivalent to December 30th of 0th year of ISO year
const JULIAN_EPOCH: i32 = -1;

#[derive(Copy, Clone, Debug, Default)]
// The Julian calendar
pub struct Julian;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
// The inner date type used for representing Date<Julian>
// The inner date uses IsoDateInner, but represent Julian Dates
pub struct JulianDateInner(IsoDateInner);

impl Calendar for Julian {
    type DateInner = JulianDateInner;
    fn date_from_iso(&self, iso: Date<Iso>) -> JulianDateInner {
        let fixed_iso = Iso::fixed_from_iso(*iso.inner());
        Self::julian_from_fixed(fixed_iso)
    }

    fn date_to_iso(&self, date: &Self::DateInner) -> Date<Iso> {
        let fixed_julian = Julian::fixed_from_julian(date.0);
        Iso::iso_from_fixed(fixed_julian)
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
        Iso.day_of_week(Julian.date_to_iso(date).inner())
    }

    fn offset_date(&self, date: &mut Self::DateInner, offset: DateDuration<Self>) {
        Self::offset_date(date, offset);
    }

    #[allow(clippy::field_reassign_with_default)]
    fn until(
        &self,
        date1: &Self::DateInner,
        date2: &Self::DateInner,
        _largest_unit: DateDurationUnit,
        _smallest_unit: DateDurationUnit,
    ) -> DateDuration<Self> {
        DateDuration::new(
            date1.0.year.0 - date2.0.year.0,
            date1.0.year.0 - date2.0.year.0,
            0,
            u8::from(date1.0.day) as i32 - u8::from(date2.0.day) as i32,
        )
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
        let prev_year = IsoYear(date.0.year.0 - 1);
        let next_year = IsoYear(date.0.year.0 + 1);
        types::DayOfYearInfo {
            day_of_year: Julian::day_of_year(date.0),
            days_in_year: Julian::days_in_year(date.0.year),
            prev_year: prev_year.into(),
            days_in_prev_year: Julian::days_in_year(prev_year),
            next_year: next_year.into(),
        }
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

    fn days_in_year(year: IsoYear) -> u32 {
        if Self::is_leap_year(year) {
            366
        } else {
            365
        }
    }

    fn day_of_year(date: IsoDateInner) -> u32 {
        let month_offset = [0, 1, -1, 0, 0, 1, 1, 2, 3, 3, 4, 4];
        let mut offset = month_offset[u8::from(date.month) as usize - 1];
        if Self::is_leap_year(date.year) && u8::from(date.month) > 2 {
            offset += 1;
        }
        let prev_month_days = (30 * (u8::from(date.month) as i32 - 1) + offset) as u32;
        prev_month_days + u8::from(date.day) as u32
    }

    fn days_in_month(year: IsoYear, month: IsoMonth) -> u8 {
        match u8::from(month) {
            4 | 6 | 9 | 11 => 30,
            2 if Self::is_leap_year(year) => 29,
            2 => 28,
            _ => 31,
        }
    }

    // Fixed is day count representation of calendars starting from Jan 1st of year 1 of Georgian Calendar.
    // The fixed calculations algorithms are from the Calendrical Calculations book
    //
    // Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/1ee51ecfaae6f856b0d7de3e36e9042100b4f424/calendar.l#L1689-L1709
    fn fixed_from_julian(date: IsoDateInner) -> i32 {
        let year = if date.year.0 < 0 {
            date.year.0 + 1
        } else {
            date.year.0
        };
        let mut fixed: i32 = JULIAN_EPOCH - 1 + 365 * (year - 1) + (year - 1) / 4;
        fixed += (367 * (u8::from(date.month) as i32) - 362) / 12;
        fixed += if u8::from(date.month) <= 2 {
            0
        } else if Self::is_leap_year(date.year) {
            -1
        } else {
            -2
        };

        fixed + u8::from(date.day) as i32
    }

    fn fixed_from_julian_integers(year: i32, month: i32, day: i32) -> i32 {
        Self::fixed_from_julian(
            *Date::new_iso_date_from_integers(year, month as u8, day as u8)
                .unwrap()
                .inner(),
        )
    }

    // Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/1ee51ecfaae6f856b0d7de3e36e9042100b4f424/calendar.l#L1711-L1738
    fn julian_from_fixed(date: i32) -> JulianDateInner {
        let approx = ((4 * date) + 1464) / 1461;
        let year = if approx <= 0 { approx - 1 } else { approx };
        let prior_days = date - Self::fixed_from_julian_integers(year, 1, 1);
        let correction = if date < Self::fixed_from_julian_integers(year, 3, 1) {
            0
        } else if Self::is_leap_year(IsoYear::from(year)) {
            1
        } else {
            2
        };
        let month = (12 * (prior_days + correction) + 373) / 367;
        let day = date - Self::fixed_from_julian_integers(year, month, 1) + 1;

        *Date::new_julian_date_from_integers(year, month as u8, day as u8)
            .unwrap()
            .inner()
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

    #[test]
    fn test_day_iso_to_julian() {
        // March 1st 200 is same on both calendars
        let iso_date = Date::new_iso_date_from_integers(200, 3, 1).unwrap();
        let julian_date = Julian.date_from_iso(iso_date);
        assert_eq!(julian_date.0.year.0, 200);
        assert_eq!(u8::from(julian_date.0.month), 3);
        assert_eq!(u8::from(julian_date.0.day), 1);

        // Feb 28th, 200 (iso) = Feb 29th, 200 (julian)
        let iso_date = Date::new_iso_date_from_integers(200, 2, 28).unwrap();
        let julian_date = Julian.date_from_iso(iso_date);
        assert_eq!(julian_date.0.year.0, 200);
        assert_eq!(u8::from(julian_date.0.month), 2);
        assert_eq!(u8::from(julian_date.0.day), 29);

        // March 1st 400 (iso) = Feb 29th, 400 (julian)
        let iso_date = Date::new_iso_date_from_integers(400, 3, 1).unwrap();
        let julian_date = Julian.date_from_iso(iso_date);
        assert_eq!(julian_date.0.year.0, 400);
        assert_eq!(u8::from(julian_date.0.month), 2);
        assert_eq!(u8::from(julian_date.0.day), 29);

        // Jan 1st, 2022 (iso) = Dec 19, 2021 (julian)
        let iso_date = Date::new_iso_date_from_integers(2022, 1, 1).unwrap();
        let julian_date = Julian.date_from_iso(iso_date);
        assert_eq!(julian_date.0.year.0, 2021);
        assert_eq!(u8::from(julian_date.0.month), 12);
        assert_eq!(u8::from(julian_date.0.day), 19);
    }

    #[test]
    fn test_day_julian_to_iso() {
        // March 1st 200 is same on both calendars
        let julian_date = Date::new_julian_date_from_integers(200, 3, 1).unwrap();
        let iso_date = Julian.date_to_iso(julian_date.inner());
        let iso_expected_date = Date::new_iso_date_from_integers(200, 3, 1).unwrap();
        assert_eq!(iso_date, iso_expected_date);

        // Feb 28th, 200 (iso) = Feb 29th, 200 (julian)
        let julian_date = Date::new_julian_date_from_integers(200, 2, 29).unwrap();
        let iso_date = Julian.date_to_iso(julian_date.inner());
        let iso_expected_date = Date::new_iso_date_from_integers(200, 2, 28).unwrap();
        assert_eq!(iso_date, iso_expected_date);

        // March 1st 400 (iso) = Feb 29th, 400 (julian)
        let julian_date = Date::new_julian_date_from_integers(400, 2, 29).unwrap();
        let iso_date = Julian.date_to_iso(julian_date.inner());
        let iso_expected_date = Date::new_iso_date_from_integers(400, 3, 1).unwrap();
        assert_eq!(iso_date, iso_expected_date);

        // Jan 1st, 2022 (iso) = Dec 19, 2021 (julian)
        let julian_date = Date::new_julian_date_from_integers(2021, 12, 19).unwrap();
        let iso_date = Julian.date_to_iso(julian_date.inner());
        let iso_expected_date = Date::new_iso_date_from_integers(2022, 1, 1).unwrap();
        assert_eq!(iso_date, iso_expected_date);

        // March 1st, 2022 (iso) = Feb 16, 2022 (julian)
        let julian_date = Date::new_julian_date_from_integers(2022, 2, 16).unwrap();
        let iso_date = Julian.date_to_iso(julian_date.inner());
        let iso_expected_date = Date::new_iso_date_from_integers(2022, 3, 1).unwrap();
        assert_eq!(iso_date, iso_expected_date);
    }
}
