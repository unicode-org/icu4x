// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains types and implementations for the Julian calendar.
//!
//! ```rust
//! use icu::calendar::{Date, DateTime,
//!                     types::IsoHour, types::IsoMinute, types::IsoSecond,
//!                     julian::Julian};
//!
//! // `Date` type
//! let date_iso = Date::new_iso_date_from_integers(1970, 1, 2).unwrap();
//! let date_julian = Date::new_from_iso(date_iso, Julian);
//!
//! // `DateTime` type
//! let datetime_iso = DateTime::new_iso_datetime_from_integers(1970, 1, 2, 13, 1, 0).unwrap();
//! let datetime_julian = DateTime::new_from_iso(datetime_iso, Julian);
//!
//! // `Date` checks
//! assert_eq!(date_julian.year().number, 1969);
//! assert_eq!(date_julian.month().number, 12);
//! assert_eq!(date_julian.day_of_month().0, 20);
//!
//! // `DateTime` type
//! assert_eq!(datetime_julian.date.year().number, 1969);
//! assert_eq!(datetime_julian.date.month().number, 12);
//! assert_eq!(datetime_julian.date.day_of_month().0, 20);
//! assert_eq!(datetime_julian.time.hour, IsoHour::new_unchecked(13));
//! assert_eq!(datetime_julian.time.minute, IsoMinute::new_unchecked(1));
//! assert_eq!(datetime_julian.time.second, IsoSecond::new_unchecked(0));
//! ```

use crate::iso::{Iso, IsoYear};
use crate::{
    types, ArithmeticDate, Calendar, CalendarArithmetic, Date, DateDuration, DateDurationUnit,
    DateTime, DateTimeError,
};
use core::convert::TryInto;
use core::marker::PhantomData;
use tinystr::tinystr;

// Julian epoch is equivalent to fixed_from_iso of December 30th of 0 year
// 1st Jan of 1st year Julian is equivalent to December 30th of 0th year of ISO year
const JULIAN_EPOCH: i32 = -1;

/// The Julian calendar
#[derive(Copy, Clone, Debug, Hash, Default, Eq, PartialEq)]
#[allow(clippy::exhaustive_structs)] // this type is stable
pub struct Julian;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
// The inner date type used for representing Date<Julian>
pub struct JulianDateInner(pub(crate) ArithmeticDate<Julian>);

impl CalendarArithmetic for Julian {
    fn month_days(year: i32, month: u8) -> u8 {
        match month {
            4 | 6 | 9 | 11 => 30,
            2 if Self::is_leap_year(year) => 29,
            2 => 28,
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            _ => 0,
        }
    }

    fn months_for_every_year() -> u8 {
        12
    }

    fn is_leap_year(year: i32) -> bool {
        year % 4 == 0
    }
}

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
        date.0.months_in_year()
    }

    fn days_in_year(&self, date: &Self::DateInner) -> u32 {
        date.0.days_in_year()
    }

    fn days_in_month(&self, date: &Self::DateInner) -> u8 {
        date.0.days_in_month()
    }

    fn day_of_week(&self, date: &Self::DateInner) -> types::IsoWeekday {
        Iso.day_of_week(Julian.date_to_iso(date).inner())
    }

    fn offset_date(&self, date: &mut Self::DateInner, offset: DateDuration<Self>) {
        date.0.offset_date(offset);
    }

    #[allow(clippy::field_reassign_with_default)]
    fn until(
        &self,
        date1: &Self::DateInner,
        date2: &Self::DateInner,
        _calendar2: &Self,
        _largest_unit: DateDurationUnit,
        _smallest_unit: DateDurationUnit,
    ) -> DateDuration<Self> {
        date1.0.until(date2.0, _largest_unit, _smallest_unit)
    }

    /// The calendar-specific year represented by `date`
    /// Julian has the same era scheme as Georgian
    fn year(&self, date: &Self::DateInner) -> types::Year {
        crate::gregorian::year_as_gregorian(date.0.year)
    }

    /// The calendar-specific month represented by `date`
    fn month(&self, date: &Self::DateInner) -> types::Month {
        types::Month {
            number: date.0.month.into(),
            code: types::MonthCode(tinystr!(8, "TODO")),
        }
    }

    /// The calendar-specific day-of-month represented by `date`
    fn day_of_month(&self, date: &Self::DateInner) -> types::DayOfMonth {
        date.0.day_of_month()
    }

    fn day_of_year_info(&self, date: &Self::DateInner) -> types::DayOfYearInfo {
        let prev_year = IsoYear(date.0.year - 1);
        let next_year = IsoYear(date.0.year + 1);
        types::DayOfYearInfo {
            day_of_year: date.0.day_of_year(),
            days_in_year: date.0.days_in_year(),
            prev_year: prev_year.into(),
            days_in_prev_year: Julian::days_in_year_direct(prev_year.0),
            next_year: next_year.into(),
        }
    }

    fn debug_name(&self) -> &'static str {
        "Julian"
    }
}

impl Julian {
    /// Construct a new Julian Calendar
    pub fn new() -> Self {
        Self
    }

    // "Fixed" is a day count representation of calendars staring from Jan 1st of year 1 of the Georgian Calendar.
    // The fixed date algorithms are from
    // Dershowitz, Nachum, and Edward M. Reingold. _Calendrical calculations_. Cambridge University Press, 2008.
    //
    // Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/1ee51ecfaae6f856b0d7de3e36e9042100b4f424/calendar.l#L1689-L1709
    pub(crate) fn fixed_from_julian(date: ArithmeticDate<Julian>) -> i32 {
        let year = if date.year < 0 {
            date.year + 1
        } else {
            date.year
        };
        let mut fixed: i32 = JULIAN_EPOCH - 1 + 365 * (year - 1) + (year - 1) / 4;
        fixed += (367 * (date.month as i32) - 362) / 12;
        fixed += if date.month <= 2 {
            0
        } else if Self::is_leap_year(date.year) {
            -1
        } else {
            -2
        };

        fixed + (date.day as i32)
    }

    pub(crate) fn fixed_from_julian_integers(year: i32, month: i32, day: i32) -> i32 {
        #[allow(clippy::unwrap_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
        Self::fixed_from_julian(ArithmeticDate {
            year,
            month: month.try_into().unwrap(),
            day: day.try_into().unwrap(),
            marker: PhantomData,
        })
    }

    /// Convenience function so we can call days_in_year without
    /// needing to construct a full ArithmeticDate
    fn days_in_year_direct(year: i32) -> u32 {
        if Julian::is_leap_year(year) {
            366
        } else {
            365
        }
    }

    // Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/1ee51ecfaae6f856b0d7de3e36e9042100b4f424/calendar.l#L1711-L1738
    fn julian_from_fixed(date: i32) -> JulianDateInner {
        let approx = ((4 * date) + 1464) / 1461;
        let year = if approx <= 0 { approx - 1 } else { approx };
        let prior_days = date - Self::fixed_from_julian_integers(year, 1, 1);
        let correction = if date < Self::fixed_from_julian_integers(year, 3, 1) {
            0
        } else if Julian::is_leap_year(year) {
            1
        } else {
            2
        };
        let month = (12 * (prior_days + correction) + 373) / 367;
        let day = date - Self::fixed_from_julian_integers(year, month, 1) + 1;

        #[allow(clippy::unwrap_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
        *Date::new_julian_date(year, month as u8, day as u8)
            .unwrap()
            .inner()
    }
}

impl Date<Julian> {
    /// Construct new Julian Date.
    ///
    /// ```rust
    /// use icu::calendar::Date;
    ///
    /// let date_julian = Date::new_julian_date(1969, 12, 20).unwrap();
    ///
    /// assert_eq!(date_julian.year().number, 1969);
    /// assert_eq!(date_julian.month().number, 12);
    /// assert_eq!(date_julian.day_of_month().0, 20);
    /// ```
    pub fn new_julian_date(year: i32, month: u8, day: u8) -> Result<Date<Julian>, DateTimeError> {
        let inner = ArithmeticDate {
            year,
            month,
            day,
            marker: PhantomData,
        };

        if day > 28 {
            let bound = inner.days_in_month();
            if day > bound {
                return Err(DateTimeError::OutOfRange);
            }
        }

        Ok(Date::from_raw(JulianDateInner(inner), Julian))
    }
}

impl DateTime<Julian> {
    /// Construct a new Julian datetime from integers.
    ///
    /// ```rust
    /// use icu::calendar::{DateTime,
    ///                     types::IsoHour,
    ///                     types::IsoMinute,
    ///                     types::IsoSecond};
    ///
    /// let datetime_julian = DateTime::new_julian_datetime(1969, 12, 20, 13, 1, 0).unwrap();
    ///
    /// assert_eq!(datetime_julian.date.year().number, 1969);
    /// assert_eq!(datetime_julian.date.month().number, 12);
    /// assert_eq!(datetime_julian.date.day_of_month().0, 20);
    /// assert_eq!(datetime_julian.time.hour, IsoHour::new_unchecked(13));
    /// assert_eq!(datetime_julian.time.minute, IsoMinute::new_unchecked(1));
    /// assert_eq!(datetime_julian.time.second, IsoSecond::new_unchecked(0));
    /// ```
    pub fn new_julian_datetime(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Result<DateTime<Julian>, DateTimeError> {
        Ok(DateTime {
            date: Date::new_julian_date(year, month, day)?,
            time: types::Time::try_new(hour, minute, second, 0)?,
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
        assert_eq!(julian_date.0.year, 200);
        assert_eq!(julian_date.0.month, 3);
        assert_eq!(julian_date.0.day, 1);

        // Feb 28th, 200 (iso) = Feb 29th, 200 (julian)
        let iso_date = Date::new_iso_date_from_integers(200, 2, 28).unwrap();
        let julian_date = Julian.date_from_iso(iso_date);
        assert_eq!(julian_date.0.year, 200);
        assert_eq!(julian_date.0.month, 2);
        assert_eq!(julian_date.0.day, 29);

        // March 1st 400 (iso) = Feb 29th, 400 (julian)
        let iso_date = Date::new_iso_date_from_integers(400, 3, 1).unwrap();
        let julian_date = Julian.date_from_iso(iso_date);
        assert_eq!(julian_date.0.year, 400);
        assert_eq!(julian_date.0.month, 2);
        assert_eq!(julian_date.0.day, 29);

        // Jan 1st, 2022 (iso) = Dec 19, 2021 (julian)
        let iso_date = Date::new_iso_date_from_integers(2022, 1, 1).unwrap();
        let julian_date = Julian.date_from_iso(iso_date);
        assert_eq!(julian_date.0.year, 2021);
        assert_eq!(julian_date.0.month, 12);
        assert_eq!(julian_date.0.day, 19);
    }

    #[test]
    fn test_day_julian_to_iso() {
        // March 1st 200 is same on both calendars
        let julian_date = Date::new_julian_date(200, 3, 1).unwrap();
        let iso_date = Julian.date_to_iso(julian_date.inner());
        let iso_expected_date = Date::new_iso_date_from_integers(200, 3, 1).unwrap();
        assert_eq!(iso_date, iso_expected_date);

        // Feb 28th, 200 (iso) = Feb 29th, 200 (julian)
        let julian_date = Date::new_julian_date(200, 2, 29).unwrap();
        let iso_date = Julian.date_to_iso(julian_date.inner());
        let iso_expected_date = Date::new_iso_date_from_integers(200, 2, 28).unwrap();
        assert_eq!(iso_date, iso_expected_date);

        // March 1st 400 (iso) = Feb 29th, 400 (julian)
        let julian_date = Date::new_julian_date(400, 2, 29).unwrap();
        let iso_date = Julian.date_to_iso(julian_date.inner());
        let iso_expected_date = Date::new_iso_date_from_integers(400, 3, 1).unwrap();
        assert_eq!(iso_date, iso_expected_date);

        // Jan 1st, 2022 (iso) = Dec 19, 2021 (julian)
        let julian_date = Date::new_julian_date(2021, 12, 19).unwrap();
        let iso_date = Julian.date_to_iso(julian_date.inner());
        let iso_expected_date = Date::new_iso_date_from_integers(2022, 1, 1).unwrap();
        assert_eq!(iso_date, iso_expected_date);

        // March 1st, 2022 (iso) = Feb 16, 2022 (julian)
        let julian_date = Date::new_julian_date(2022, 2, 16).unwrap();
        let iso_date = Julian.date_to_iso(julian_date.inner());
        let iso_expected_date = Date::new_iso_date_from_integers(2022, 3, 1).unwrap();
        assert_eq!(iso_date, iso_expected_date);
    }
}
