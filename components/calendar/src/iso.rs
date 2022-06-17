// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains types and implementations for the ISO calendar.
//!
//! ```rust
//! use icu::calendar::{Date, DateTime};
//!
//! // `Date` type
//! let date_iso = Date::new_iso_date(1970, 1, 2)
//!     .expect("Failed to initialize ISO Date instance.");
//!
//! // `DateTime` type
//! let datetime_iso = DateTime::new_iso_datetime(1970, 1, 2, 13, 1, 0)
//!     .expect("Failed to initialize ISO DateTime instance.");
//!
//! // `Date` checks
//! assert_eq!(date_iso.year().number, 1970);
//! assert_eq!(date_iso.month().ordinal, 1);
//! assert_eq!(date_iso.day_of_month().0, 2);
//!
//! // `DateTime` type
//! assert_eq!(datetime_iso.date.year().number, 1970);
//! assert_eq!(datetime_iso.date.month().ordinal, 1);
//! assert_eq!(datetime_iso.date.day_of_month().0, 2);
//! assert_eq!(datetime_iso.time.hour.number(), 13);
//! assert_eq!(datetime_iso.time.minute.number(), 1);
//! assert_eq!(datetime_iso.time.second.number(), 0);
//! ```

use crate::any_calendar::AnyCalendarKind;
use crate::{types, Calendar, Date, DateDuration, DateDurationUnit, DateTime, DateTimeError};
use crate::{ArithmeticDate, CalendarArithmetic};
use core::convert::TryInto;
use tinystr::tinystr;

// The georgian epoch is equivalent to first day in fixed day measurement
const EPOCH: i32 = 1;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
#[allow(clippy::exhaustive_structs)] // this type is stable
/// The ISO Calendar
pub struct Iso;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
/// The inner date type used for representing Date<Iso>
pub struct IsoDateInner(pub(crate) ArithmeticDate<Iso>);

impl CalendarArithmetic for Iso {
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
        year % 4 == 0 && (year % 400 == 0 || year % 100 != 0)
    }
}

impl Calendar for Iso {
    type DateInner = IsoDateInner;
    fn date_from_iso(&self, iso: Date<Iso>) -> IsoDateInner {
        *iso.inner()
    }

    fn date_to_iso(&self, date: &Self::DateInner) -> Date<Iso> {
        Date::from_raw(*date, Iso)
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
        // For the purposes of the calculation here, Monday is 0, Sunday is 6
        // ISO has Monday=1, Sunday=7, which we transform in the last step

        // The days of the week are the same every 400 years
        // so we normalize to the nearest multiple of 400
        let years_since_400 = date.0.year % 400;
        let leap_years_since_400 = years_since_400 / 4 - years_since_400 / 100;
        // The number of days to the current year
        let days_to_current_year = 365 * years_since_400 + leap_years_since_400;
        // The weekday offset from January 1 this year and January 1 2000
        let year_offset = days_to_current_year % 7;

        // Corresponding months from
        // https://en.wikipedia.org/wiki/Determination_of_the_day_of_the_week#Corresponding_months
        let month_offset = if Self::is_leap_year(date.0.year) {
            match date.0.month {
                10 => 0,
                5 => 1,
                2 | 8 => 2,
                3 | 11 => 3,
                6 => 4,
                9 | 12 => 5,
                1 | 4 | 7 => 6,
                _ => unreachable!(),
            }
        } else {
            match date.0.month {
                1 | 10 => 0,
                5 => 1,
                8 => 2,
                2 | 3 | 11 => 3,
                6 => 4,
                9 | 12 => 5,
                4 | 7 => 6,
                _ => unreachable!(),
            }
        };
        let january_1_2000 = 5; // Saturday
        let day_offset = (january_1_2000 + year_offset + month_offset + date.0.day as i32) % 7;

        // We calculated in a zero-indexed fashion, but ISO specifies one-indexed
        types::IsoWeekday::from((day_offset + 1) as usize)
    }

    fn offset_date(&self, date: &mut Self::DateInner, mut offset: DateDuration<Self>) {
        date.0.year += offset.years;
        date.add_months(offset.months);
        offset.months = 0;

        offset.days += offset.weeks * 7;

        // Normalize date to beginning of month
        offset.days += date.0.day as i32 - 1;
        date.0.day = 1;

        while offset.days != 0 {
            if offset.days < 0 {
                date.add_months(-1);
                let month_days = self.days_in_month(date);
                if (-offset.days) > month_days as i32 {
                    offset.days += month_days as i32;
                } else {
                    // Add 1 since we are subtracting from the first day of the
                    // *next* month
                    date.0.day = 1 + (month_days as i8 + offset.days as i8) as u8;
                    offset.days = 0;
                }
            } else {
                let month_days = self.days_in_month(date);
                // >= because we date.day is 1, so adding the number of days in the month
                // will still have the same effect
                if offset.days >= month_days as i32 {
                    date.add_months(1);
                    offset.days -= month_days as i32;
                } else {
                    date.0.day += offset.days as u8;
                    offset.days = 0;
                }
            }
        }
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
    fn year(&self, date: &Self::DateInner) -> types::Year {
        Self::year_as_iso(date.0.year)
    }

    /// The calendar-specific month represented by `date`
    fn month(&self, date: &Self::DateInner) -> types::Month {
        date.0.solar_month()
    }

    /// The calendar-specific day-of-month represented by `date`
    fn day_of_month(&self, date: &Self::DateInner) -> types::DayOfMonth {
        date.0.day_of_month()
    }

    fn day_of_year_info(&self, date: &Self::DateInner) -> types::DayOfYearInfo {
        let prev_year = date.0.year - 1;
        let next_year = date.0.year + 1;
        types::DayOfYearInfo {
            day_of_year: date.0.day_of_year(),
            days_in_year: date.0.days_in_year(),
            prev_year: Self::year_as_iso(prev_year),
            days_in_prev_year: Iso::days_in_year_direct(prev_year),
            next_year: Self::year_as_iso(next_year),
        }
    }

    fn debug_name(&self) -> &'static str {
        "ISO"
    }

    fn any_calendar_kind(&self) -> Option<AnyCalendarKind> {
        Some(AnyCalendarKind::Iso)
    }
}

impl Date<Iso> {
    /// Construct a new ISO date from integers.
    ///
    /// ```rust
    /// use icu::calendar::Date;
    ///
    /// let date_iso = Date::new_iso_date(1970, 1, 2)
    ///     .expect("Failed to initialize ISO Date instance.");
    ///
    /// assert_eq!(date_iso.year().number, 1970);
    /// assert_eq!(date_iso.month().ordinal, 1);
    /// assert_eq!(date_iso.day_of_month().0, 2);
    /// ```
    pub fn new_iso_date(year: i32, month: u8, day: u8) -> Result<Date<Iso>, DateTimeError> {
        if !(1..=12).contains(&month) {
            return Err(DateTimeError::OutOfRange);
        }
        if day == 0 || day > Iso::days_in_month(year, month) {
            return Err(DateTimeError::OutOfRange);
        }
        Ok(Date::from_raw(
            IsoDateInner(ArithmeticDate::new(year, month, day)),
            Iso,
        ))
    }
}

impl DateTime<Iso> {
    /// Construct a new ISO datetime from integers.
    ///
    /// ```rust
    /// use icu::calendar::DateTime;
    ///
    /// let datetime_iso = DateTime::new_iso_datetime(1970, 1, 2, 13, 1, 0)
    ///     .expect("Failed to initialize ISO DateTime instance.");
    ///
    /// assert_eq!(datetime_iso.date.year().number, 1970);
    /// assert_eq!(datetime_iso.date.month().ordinal, 1);
    /// assert_eq!(datetime_iso.date.day_of_month().0, 2);
    /// assert_eq!(datetime_iso.time.hour.number(), 13);
    /// assert_eq!(datetime_iso.time.minute.number(), 1);
    /// assert_eq!(datetime_iso.time.second.number(), 0);
    /// ```
    pub fn new_iso_datetime(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Result<DateTime<Iso>, DateTimeError> {
        Ok(DateTime {
            date: Date::new_iso_date(year, month, day)?,
            time: types::Time::try_new(hour, minute, second, 0)?,
        })
    }
}

impl Iso {
    /// Construct a new ISO Calendar
    pub fn new() -> Self {
        Self
    }

    /// Count the number of days in a given month/year combo
    fn days_in_month(year: i32, month: u8) -> u8 {
        match month {
            4 | 6 | 9 | 11 => 30,
            2 if Self::is_leap_year(year) => 29,
            2 => 28,
            _ => 31,
        }
    }

    pub(crate) fn days_in_year_direct(year: i32) -> u32 {
        if Self::is_leap_year(year) {
            366
        } else {
            365
        }
    }

    // Minute count representation of calendars starting from  00:00:00 UTC on Jan 1st, 1970.
    pub fn unix_epoch_minute_from_iso(date: IsoDateInner) -> i32 {
        let minutes_a_day = 60 * 24;
        (Self::fixed_from_iso(date)
            - Self::fixed_from_iso(IsoDateInner(ArithmeticDate::new(1969, 12, 31))))
            * minutes_a_day
    }

    // Convert minute count since 00:00:00 UTC on Jan 1st, 1970 to ISO Date.
    pub fn iso_from_unix_epoch_minute(minute: i32) -> Date<Iso> {
        let minutes_a_day = 60 * 24;
        let total = Self::fixed_from_iso(IsoDateInner(ArithmeticDate::new(1969, 12, 31)))
            * minutes_a_day
            + minute;
        Self::iso_from_fixed(total / minutes_a_day)
    }

    // Fixed is day count representation of calendars starting from Jan 1st of year 1.
    // The fixed calculations algorithms are from the Calendrical Calculations book.
    //
    // Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/1ee51ecfaae6f856b0d7de3e36e9042100b4f424/calendar.l#L1167-L1189
    pub(crate) fn fixed_from_iso(date: IsoDateInner) -> i32 {
        // Calculate days per year
        let mut fixed: i32 = EPOCH - 1 + 365 * (date.0.year - 1);
        // Adjust for leap year logic
        fixed += ((date.0.year - 1) / 4) - ((date.0.year - 1) / 100) + ((date.0.year - 1) / 400);
        // Days of current year
        fixed += (367 * (date.0.month as i32) - 362) / 12;
        // Leap year adjustment for the current year
        fixed += if date.0.month <= 2 {
            0
        } else if Self::is_leap_year(date.0.year) {
            -1
        } else {
            -2
        };
        // Days passed in current month
        fixed + (date.0.day as i32)
    }

    fn fixed_from_iso_integers(year: i32, month: i32, day: i32) -> i32 {
        #[allow(clippy::unwrap_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
        Self::fixed_from_iso(
            *Date::new_iso_date(year, month as u8, day as u8)
                .unwrap()
                .inner(),
        )
    }
    pub(crate) fn iso_from_year_day(year: i32, year_day: u32) -> Date<Iso> {
        let mut month = 1;
        let mut day = year_day as i32;
        while month <= 12 {
            let month_days = Self::days_in_month(year, month) as i32;
            if day <= month_days {
                break;
            } else {
                day -= month_days;
                month += 1;
            }
        }

        #[allow(clippy::unwrap_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
        Date::new_iso_date(year, month, day.try_into().unwrap()).unwrap()
    }

    // Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/1ee51ecfaae6f856b0d7de3e36e9042100b4f424/calendar.l#L1191-L1217
    fn iso_year_from_fixed(date: i32) -> i32 {
        // 400 year cycles have 146097 days
        let n_400 = date / 146097;
        let date = date % 146097;

        // 100 year cycles have 36524 days
        let n_100 = date / 36524;
        let date = date % 36524;

        // 4 year cycles have 1461 days
        let n_4 = date / 1461;
        let date = date % 1461;

        let n_1 = date / 365;

        let year = 400 * n_400 + 100 * n_100 + 4 * n_4 + n_1;

        if n_100 == 4 || n_1 == 4 {
            year
        } else {
            year + 1
        }
    }

    fn iso_new_year(year: i32) -> i32 {
        Self::fixed_from_iso_integers(year, 1, 1)
    }

    // Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/1ee51ecfaae6f856b0d7de3e36e9042100b4f424/calendar.l#L1237-L1258
    pub(crate) fn iso_from_fixed(date: i32) -> Date<Iso> {
        let year = Self::iso_year_from_fixed(date);
        let prior_days = date - Self::iso_new_year(year);
        let correction = if date < Self::fixed_from_iso_integers(year, 3, 1) {
            0
        } else if Self::is_leap_year(year) {
            1
        } else {
            2
        };
        let month = (12 * (prior_days + correction) + 373) / 367;
        let day = date - Self::fixed_from_iso_integers(year, month, 1) + 1;
        #[allow(clippy::unwrap_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
        Date::new_iso_date(year, month as u8, day as u8).unwrap()
    }

    pub(crate) fn day_of_year(date: IsoDateInner) -> u32 {
        // Cumulatively how much are dates in each month
        // offset from "30 days in each month" (in non leap years)
        let month_offset = [0, 1, -1, 0, 0, 1, 1, 2, 3, 3, 4, 4];
        #[allow(clippy::indexing_slicing)] // TODO(#1668) Clippy exceptions need docs or fixing.
        let mut offset = month_offset[date.0.month as usize - 1];
        if Self::is_leap_year(date.0.year) && date.0.month > 2 {
            // Months after February in a leap year are offset by one less
            offset += 1;
        }
        let prev_month_days = (30 * (date.0.month as i32 - 1) + offset) as u32;

        prev_month_days + date.0.day as u32
    }

    /// Wrap the year in the appropriate era code
    fn year_as_iso(year: i32) -> types::Year {
        types::Year {
            era: types::Era(tinystr!(16, "default")),
            number: year,
            related_iso: year,
        }
    }
}

impl IsoDateInner {
    pub(crate) fn jan_1(year: i32) -> Self {
        Self(ArithmeticDate::new(year, 1, 1))
    }
    pub(crate) fn dec_31(year: i32) -> Self {
        Self(ArithmeticDate::new(year, 12, 1))
    }

    fn add_months(&mut self, months: i32) {
        // Get a zero-indexed new month
        let new_month = (self.0.month as i32 - 1) + months;
        if new_month >= 0 {
            self.0.year += new_month / 12;
            self.0.month = ((new_month % 12) + 1) as u8;
        } else {
            // subtract full years
            self.0.year -= (-new_month) / 12;
            // subtract a partial year
            self.0.year -= 1;
            // adding 13 since months are 1-indexed
            self.0.month = (13 + (new_month % 12)) as u8
        }
    }
}

impl From<&'_ IsoDateInner> for crate::provider::EraStartDate {
    fn from(other: &'_ IsoDateInner) -> Self {
        Self {
            year: other.0.year,
            month: other.0.month,
            day: other.0.day,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::IsoWeekday;

    #[test]
    fn test_day_of_week() {
        // June 23, 2021 is a Wednesday
        assert_eq!(
            Date::new_iso_date(2021, 6, 23).unwrap().day_of_week(),
            IsoWeekday::Wednesday,
        );
        // Feb 2, 1983 was a Wednesday
        assert_eq!(
            Date::new_iso_date(1983, 2, 2).unwrap().day_of_week(),
            IsoWeekday::Wednesday,
        );
        // Jan 21, 2021 was a Tuesday
        assert_eq!(
            Date::new_iso_date(2020, 1, 21).unwrap().day_of_week(),
            IsoWeekday::Tuesday,
        );
    }

    #[test]
    fn test_day_of_year() {
        // June 23, 2021 was day 174
        assert_eq!(
            Date::new_iso_date(2021, 6, 23)
                .unwrap()
                .day_of_year_info()
                .day_of_year,
            174,
        );
        // June 23, 2020 was day 175
        assert_eq!(
            Date::new_iso_date(2020, 6, 23)
                .unwrap()
                .day_of_year_info()
                .day_of_year,
            175,
        );
        // Feb 2, 1983 was a Wednesday
        assert_eq!(
            Date::new_iso_date(1983, 2, 2)
                .unwrap()
                .day_of_year_info()
                .day_of_year,
            33,
        );
    }

    fn simple_subtract(a: &Date<Iso>, b: &Date<Iso>) -> DateDuration<Iso> {
        let a = a.inner();
        let b = b.inner();
        DateDuration::new(
            a.0.year - b.0.year,
            a.0.month as i32 - b.0.month as i32,
            0,
            a.0.day as i32 - b.0.day as i32,
        )
    }

    #[test]
    fn test_offset() {
        let today = Date::new_iso_date(2021, 6, 23).unwrap();
        let today_plus_5000 = Date::new_iso_date(2035, 3, 2).unwrap();
        let offset = today.clone().added(DateDuration::new(0, 0, 0, 5000));
        assert_eq!(offset, today_plus_5000);
        let offset = today
            .clone()
            .added(simple_subtract(&today_plus_5000, &today));
        assert_eq!(offset, today_plus_5000);

        let today = Date::new_iso_date(2021, 6, 23).unwrap();
        let today_minus_5000 = Date::new_iso_date(2007, 10, 15).unwrap();
        let offset = today.clone().added(DateDuration::new(0, 0, 0, -5000));
        assert_eq!(offset, today_minus_5000);
        let offset = today
            .clone()
            .added(simple_subtract(&today_minus_5000, &today));
        assert_eq!(offset, today_minus_5000);
    }

    #[test]
    fn test_offset_at_month_boundary() {
        let today = Date::new_iso_date(2020, 2, 28).unwrap();
        let today_plus_2 = Date::new_iso_date(2020, 3, 1).unwrap();
        let offset = today.added(DateDuration::new(0, 0, 0, 2));
        assert_eq!(offset, today_plus_2);

        let today = Date::new_iso_date(2020, 2, 28).unwrap();
        let today_plus_3 = Date::new_iso_date(2020, 3, 2).unwrap();
        let offset = today.added(DateDuration::new(0, 0, 0, 3));
        assert_eq!(offset, today_plus_3);

        let today = Date::new_iso_date(2020, 2, 28).unwrap();
        let today_plus_1 = Date::new_iso_date(2020, 2, 29).unwrap();
        let offset = today.added(DateDuration::new(0, 0, 0, 1));
        assert_eq!(offset, today_plus_1);

        let today = Date::new_iso_date(2019, 2, 28).unwrap();
        let today_plus_2 = Date::new_iso_date(2019, 3, 2).unwrap();
        let offset = today.added(DateDuration::new(0, 0, 0, 2));
        assert_eq!(offset, today_plus_2);

        let today = Date::new_iso_date(2019, 2, 28).unwrap();
        let today_plus_1 = Date::new_iso_date(2019, 3, 1).unwrap();
        let offset = today.added(DateDuration::new(0, 0, 0, 1));
        assert_eq!(offset, today_plus_1);

        let today = Date::new_iso_date(2020, 3, 1).unwrap();
        let today_minus_1 = Date::new_iso_date(2020, 2, 29).unwrap();
        let offset = today.added(DateDuration::new(0, 0, 0, -1));
        assert_eq!(offset, today_minus_1);
    }

    #[test]
    fn test_offset_minute_since_unix_epoch() {
        let today = Date::new_iso_date(2020, 2, 28).unwrap();
        assert_eq!(Iso::unix_epoch_minute_from_iso(*today.inner()), 26382240);
        assert_eq!(Iso::iso_from_unix_epoch_minute(26382240), today);
    }
}
