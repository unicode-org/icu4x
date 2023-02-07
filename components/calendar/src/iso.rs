// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains types and implementations for the ISO calendar.
//!
//! ```rust
//! use icu::calendar::{Date, DateTime};
//!
//! // `Date` type
//! let date_iso = Date::try_new_iso_date(1970, 1, 2)
//!     .expect("Failed to initialize ISO Date instance.");
//!
//! // `DateTime` type
//! let datetime_iso = DateTime::try_new_iso_datetime(1970, 1, 2, 13, 1, 0)
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
use crate::calendar_arithmetic::{ArithmeticDate, CalendarArithmetic};
use crate::helpers::{div_rem_euclid, quotient};
use crate::{types, Calendar, CalendarError, Date, DateDuration, DateDurationUnit, DateTime};
use tinystr::tinystr;

// The georgian epoch is equivalent to first day in fixed day measurement
const EPOCH: i32 = 1;

/// The [ISO Calendar]
///
/// The [ISO Calendar] is a standardized solar calendar with twelve months.
/// It is identical to the Gregorian calendar, except it uses negative years for years before 1 CE,
/// and may have differing formatting data for a given locale.
///
/// This type can be used with [`Date`] or [`DateTime`] to represent dates in this calendar.
///
/// [ISO Calendar]: https://en.wikipedia.org/wiki/ISO_calendar
///
/// # Era codes
///
/// This calendar supports one era, `"default"`

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
#[allow(clippy::exhaustive_structs)] // this type is stable
pub struct Iso;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
/// The inner date type used for representing [`Date`]s of [`Iso`]. See [`Date`] and [`Iso`] for more details.
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

    fn months_for_every_year(_: i32) -> u8 {
        12
    }

    fn is_leap_year(year: i32) -> bool {
        year % 4 == 0 && (year % 400 == 0 || year % 100 != 0)
    }

    fn days_in_provided_year(year: i32) -> u32 {
        if Self::is_leap_year(year) {
            366
        } else {
            365
        }
    }
}

impl Calendar for Iso {
    type DateInner = IsoDateInner;
    /// Construct a date from era/month codes and fields
    fn date_from_codes(
        &self,
        era: types::Era,
        year: i32,
        month_code: types::MonthCode,
        day: u8,
    ) -> Result<Self::DateInner, CalendarError> {
        if era.0 != tinystr!(16, "default") {
            return Err(CalendarError::UnknownEra(era.0, self.debug_name()));
        }

        ArithmeticDate::new_from_solar(self, year, month_code, day).map(IsoDateInner)
    }

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
    fn year(&self, date: &Self::DateInner) -> types::FormattableYear {
        Self::year_as_iso(date.0.year)
    }

    /// The calendar-specific month represented by `date`
    fn month(&self, date: &Self::DateInner) -> types::FormattableMonth {
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
    /// let date_iso = Date::try_new_iso_date(1970, 1, 2)
    ///     .expect("Failed to initialize ISO Date instance.");
    ///
    /// assert_eq!(date_iso.year().number, 1970);
    /// assert_eq!(date_iso.month().ordinal, 1);
    /// assert_eq!(date_iso.day_of_month().0, 2);
    /// ```
    pub fn try_new_iso_date(year: i32, month: u8, day: u8) -> Result<Date<Iso>, CalendarError> {
        if !(1..=12).contains(&month) {
            return Err(CalendarError::OutOfRange);
        }
        if day == 0 || day > Iso::days_in_month(year, month) {
            return Err(CalendarError::OutOfRange);
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
    /// let datetime_iso = DateTime::try_new_iso_datetime(1970, 1, 2, 13, 1, 0)
    ///     .expect("Failed to initialize ISO DateTime instance.");
    ///
    /// assert_eq!(datetime_iso.date.year().number, 1970);
    /// assert_eq!(datetime_iso.date.month().ordinal, 1);
    /// assert_eq!(datetime_iso.date.day_of_month().0, 2);
    /// assert_eq!(datetime_iso.time.hour.number(), 13);
    /// assert_eq!(datetime_iso.time.minute.number(), 1);
    /// assert_eq!(datetime_iso.time.second.number(), 0);
    /// ```
    pub fn try_new_iso_datetime(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Result<DateTime<Iso>, CalendarError> {
        Ok(DateTime {
            date: Date::try_new_iso_date(year, month, day)?,
            time: types::Time::try_new(hour, minute, second, 0)?,
        })
    }

    /// Minute count representation of calendars starting from 00:00:00 on Jan 1st, 1970.
    ///
    /// ```rust
    /// use icu::calendar::DateTime;
    ///
    /// let today = DateTime::try_new_iso_datetime(2020, 2, 29, 0, 0, 0).unwrap();
    ///
    /// assert_eq!(today.minutes_since_local_unix_epoch(), 26382240);
    /// assert_eq!(
    ///     DateTime::from_minutes_since_local_unix_epoch(26382240),
    ///     today
    /// );
    ///
    /// let today = DateTime::try_new_iso_datetime(1970, 1, 1, 0, 0, 0).unwrap();
    ///
    /// assert_eq!(today.minutes_since_local_unix_epoch(), 0);
    /// assert_eq!(DateTime::from_minutes_since_local_unix_epoch(0), today);
    /// ```
    pub fn minutes_since_local_unix_epoch(&self) -> i32 {
        let minutes_a_hour = 60;
        let hours_a_day = 24;
        let minutes_a_day = minutes_a_hour * hours_a_day;
        if let Ok(unix_epoch) = DateTime::try_new_iso_datetime(1970, 1, 1, 0, 0, 0) {
            (Iso::fixed_from_iso(*self.date.inner())
                - Iso::fixed_from_iso(*unix_epoch.date.inner()))
                * minutes_a_day
                + i32::from(self.time.hour.number()) * minutes_a_hour
                + i32::from(self.time.minute.number())
        } else {
            unreachable!("DateTime should be created successfully")
        }
    }

    /// Convert minute count since 00:00:00 on Jan 1st, 1970 to ISO Date.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use icu::calendar::DateTime;
    ///
    /// // After Unix Epoch
    /// let today = DateTime::try_new_iso_datetime(2020, 2, 29, 0, 0, 0).unwrap();
    ///
    /// assert_eq!(today.minutes_since_local_unix_epoch(), 26382240);
    /// assert_eq!(
    ///     DateTime::from_minutes_since_local_unix_epoch(26382240),
    ///     today
    /// );
    ///
    /// // Unix Epoch
    /// let today = DateTime::try_new_iso_datetime(1970, 1, 1, 0, 0, 0).unwrap();
    ///
    /// assert_eq!(today.minutes_since_local_unix_epoch(), 0);
    /// assert_eq!(DateTime::from_minutes_since_local_unix_epoch(0), today);
    ///
    /// // Before Unix Epoch
    /// let today = DateTime::try_new_iso_datetime(1967, 4, 6, 20, 40, 0).unwrap();
    ///
    /// assert_eq!(today.minutes_since_local_unix_epoch(), -1440200);
    /// assert_eq!(
    ///     DateTime::from_minutes_since_local_unix_epoch(-1440200),
    ///     today
    /// );
    /// ```
    pub fn from_minutes_since_local_unix_epoch(minute: i32) -> DateTime<Iso> {
        let (time, extra_days) = types::Time::from_minute_with_remainder_days(minute);
        #[allow(clippy::unwrap_used)] // constant date
        let unix_epoch = DateTime::try_new_iso_datetime(1970, 1, 1, 0, 0, 0).unwrap();
        let unix_epoch_days = Iso::fixed_from_iso(*unix_epoch.date.inner());
        let date = Iso::iso_from_fixed(unix_epoch_days + extra_days);
        DateTime { date, time }
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

    // Fixed is day count representation of calendars starting from Jan 1st of year 1.
    // The fixed calculations algorithms are from the Calendrical Calculations book.
    //
    // Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/1ee51ecfaae6f856b0d7de3e36e9042100b4f424/calendar.l#L1167-L1189
    pub(crate) fn fixed_from_iso(date: IsoDateInner) -> i32 {
        // Calculate days per year
        let mut fixed: i32 = EPOCH - 1 + 365 * (date.0.year - 1);
        // Adjust for leap year logic
        fixed += quotient(date.0.year - 1, 4) - quotient(date.0.year - 1, 100)
            + quotient(date.0.year - 1, 400);
        // Days of current year
        fixed += quotient(367 * (date.0.month as i32) - 362, 12);
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

    fn fixed_from_iso_integers(year: i32, month: u8, day: u8) -> Option<i32> {
        Date::try_new_iso_date(year, month, day)
            .ok()
            .map(|d| *d.inner())
            .map(Self::fixed_from_iso)
    }

    pub(crate) fn iso_from_year_day(year: i32, year_day: u32) -> Date<Iso> {
        let mut month = 1;
        let mut day = year_day as i32;
        while month <= 12 {
            let month_days = Self::days_in_month(year, month) as i32;
            if day <= month_days {
                break;
            } else {
                debug_assert!(month < 12); // don't try going to month 13
                day -= month_days;
                month += 1;
            }
        }
        let day = day as u8; // day <= month_days < u8::MAX

        #[allow(clippy::unwrap_used)] // month in 1..=12, day <= month_days
        Date::try_new_iso_date(year, month, day).unwrap()
    }

    // Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/1ee51ecfaae6f856b0d7de3e36e9042100b4f424/calendar.l#L1191-L1217
    fn iso_year_from_fixed(date: i32) -> i32 {
        let date = date - EPOCH;
        // 400 year cycles have 146097 days
        let (n_400, date) = div_rem_euclid(date, 146097);

        // 100 year cycles have 36524 days
        let (n_100, date) = div_rem_euclid(date, 36524);

        // 4 year cycles have 1461 days
        let (n_4, date) = div_rem_euclid(date, 1461);

        let n_1 = quotient(date, 365);

        let year = 400 * n_400 + 100 * n_100 + 4 * n_4 + n_1;

        if n_100 == 4 || n_1 == 4 {
            year
        } else {
            year + 1
        }
    }

    fn iso_new_year(year: i32) -> i32 {
        #[allow(clippy::unwrap_used)] // valid day and month
        Self::fixed_from_iso_integers(year, 1, 1).unwrap()
    }

    // Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/1ee51ecfaae6f856b0d7de3e36e9042100b4f424/calendar.l#L1237-L1258
    pub(crate) fn iso_from_fixed(date: i32) -> Date<Iso> {
        let year = Self::iso_year_from_fixed(date);
        let prior_days = date - Self::iso_new_year(year);
        #[allow(clippy::unwrap_used)] // valid day and month
        let correction = if date < Self::fixed_from_iso_integers(year, 3, 1).unwrap() {
            0
        } else if Self::is_leap_year(year) {
            1
        } else {
            2
        };
        let month = quotient(12 * (prior_days + correction) + 373, 367) as u8; // in 1..12 < u8::MAX
        #[allow(clippy::unwrap_used)] // valid day and month
        let day = (date - Self::fixed_from_iso_integers(year, month, 1).unwrap() + 1) as u8; // <= days_in_month < u8::MAX
        #[allow(clippy::unwrap_used)] // valid day and month
        Date::try_new_iso_date(year, month, day).unwrap()
    }

    pub(crate) fn day_of_year(date: IsoDateInner) -> u32 {
        // Cumulatively how much are dates in each month
        // offset from "30 days in each month" (in non leap years)
        let month_offset = [0, 1, -1, 0, 0, 1, 1, 2, 3, 3, 4, 4];
        #[allow(clippy::indexing_slicing)] // date.0.month in 1..=12
        let mut offset = month_offset[date.0.month as usize - 1];
        if Self::is_leap_year(date.0.year) && date.0.month > 2 {
            // Months after February in a leap year are offset by one less
            offset += 1;
        }
        let prev_month_days = (30 * (date.0.month as i32 - 1) + offset) as u32;

        prev_month_days + date.0.day as u32
    }

    /// Wrap the year in the appropriate era code
    fn year_as_iso(year: i32) -> types::FormattableYear {
        types::FormattableYear {
            era: types::Era(tinystr!(16, "default")),
            number: year,
            related_iso: None,
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
            Date::try_new_iso_date(2021, 6, 23).unwrap().day_of_week(),
            IsoWeekday::Wednesday,
        );
        // Feb 2, 1983 was a Wednesday
        assert_eq!(
            Date::try_new_iso_date(1983, 2, 2).unwrap().day_of_week(),
            IsoWeekday::Wednesday,
        );
        // Jan 21, 2021 was a Tuesday
        assert_eq!(
            Date::try_new_iso_date(2020, 1, 21).unwrap().day_of_week(),
            IsoWeekday::Tuesday,
        );
    }

    #[test]
    fn test_day_of_year() {
        // June 23, 2021 was day 174
        assert_eq!(
            Date::try_new_iso_date(2021, 6, 23)
                .unwrap()
                .day_of_year_info()
                .day_of_year,
            174,
        );
        // June 23, 2020 was day 175
        assert_eq!(
            Date::try_new_iso_date(2020, 6, 23)
                .unwrap()
                .day_of_year_info()
                .day_of_year,
            175,
        );
        // Feb 2, 1983 was a Wednesday
        assert_eq!(
            Date::try_new_iso_date(1983, 2, 2)
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
        let today = Date::try_new_iso_date(2021, 6, 23).unwrap();
        let today_plus_5000 = Date::try_new_iso_date(2035, 3, 2).unwrap();
        let offset = today.added(DateDuration::new(0, 0, 0, 5000));
        assert_eq!(offset, today_plus_5000);
        let offset = today.added(simple_subtract(&today_plus_5000, &today));
        assert_eq!(offset, today_plus_5000);

        let today = Date::try_new_iso_date(2021, 6, 23).unwrap();
        let today_minus_5000 = Date::try_new_iso_date(2007, 10, 15).unwrap();
        let offset = today.added(DateDuration::new(0, 0, 0, -5000));
        assert_eq!(offset, today_minus_5000);
        let offset = today.added(simple_subtract(&today_minus_5000, &today));
        assert_eq!(offset, today_minus_5000);
    }

    #[test]
    fn test_offset_at_month_boundary() {
        let today = Date::try_new_iso_date(2020, 2, 28).unwrap();
        let today_plus_2 = Date::try_new_iso_date(2020, 3, 1).unwrap();
        let offset = today.added(DateDuration::new(0, 0, 0, 2));
        assert_eq!(offset, today_plus_2);

        let today = Date::try_new_iso_date(2020, 2, 28).unwrap();
        let today_plus_3 = Date::try_new_iso_date(2020, 3, 2).unwrap();
        let offset = today.added(DateDuration::new(0, 0, 0, 3));
        assert_eq!(offset, today_plus_3);

        let today = Date::try_new_iso_date(2020, 2, 28).unwrap();
        let today_plus_1 = Date::try_new_iso_date(2020, 2, 29).unwrap();
        let offset = today.added(DateDuration::new(0, 0, 0, 1));
        assert_eq!(offset, today_plus_1);

        let today = Date::try_new_iso_date(2019, 2, 28).unwrap();
        let today_plus_2 = Date::try_new_iso_date(2019, 3, 2).unwrap();
        let offset = today.added(DateDuration::new(0, 0, 0, 2));
        assert_eq!(offset, today_plus_2);

        let today = Date::try_new_iso_date(2019, 2, 28).unwrap();
        let today_plus_1 = Date::try_new_iso_date(2019, 3, 1).unwrap();
        let offset = today.added(DateDuration::new(0, 0, 0, 1));
        assert_eq!(offset, today_plus_1);

        let today = Date::try_new_iso_date(2020, 3, 1).unwrap();
        let today_minus_1 = Date::try_new_iso_date(2020, 2, 29).unwrap();
        let offset = today.added(DateDuration::new(0, 0, 0, -1));
        assert_eq!(offset, today_minus_1);
    }

    #[test]
    fn test_offset_handles_negative_month_offset() {
        let today = Date::try_new_iso_date(2020, 3, 1).unwrap();
        let today_minus_2_months = Date::try_new_iso_date(2020, 1, 1).unwrap();
        let offset = today.added(DateDuration::new(0, -2, 0, 0));
        assert_eq!(offset, today_minus_2_months);

        let today = Date::try_new_iso_date(2020, 3, 1).unwrap();
        let today_minus_4_months = Date::try_new_iso_date(2019, 11, 1).unwrap();
        let offset = today.added(DateDuration::new(0, -4, 0, 0));
        assert_eq!(offset, today_minus_4_months);

        let today = Date::try_new_iso_date(2020, 3, 1).unwrap();
        let today_minus_24_months = Date::try_new_iso_date(2018, 3, 1).unwrap();
        let offset = today.added(DateDuration::new(0, -24, 0, 0));
        assert_eq!(offset, today_minus_24_months);

        let today = Date::try_new_iso_date(2020, 3, 1).unwrap();
        let today_minus_27_months = Date::try_new_iso_date(2017, 12, 1).unwrap();
        let offset = today.added(DateDuration::new(0, -27, 0, 0));
        assert_eq!(offset, today_minus_27_months);
    }

    #[test]
    fn test_offset_handles_out_of_bound_month_offset() {
        let today = Date::try_new_iso_date(2021, 1, 31).unwrap();
        // since 2021/02/31 isn't a valid date, `offset_date` auto-adjusts by adding 3 days to 2021/02/28
        let today_plus_1_month = Date::try_new_iso_date(2021, 3, 3).unwrap();
        let offset = today.added(DateDuration::new(0, 1, 0, 0));
        assert_eq!(offset, today_plus_1_month);

        let today = Date::try_new_iso_date(2021, 1, 31).unwrap();
        // since 2021/02/31 isn't a valid date, `offset_date` auto-adjusts by adding 3 days to 2021/02/28
        let today_plus_1_month_1_day = Date::try_new_iso_date(2021, 3, 4).unwrap();
        let offset = today.added(DateDuration::new(0, 1, 0, 1));
        assert_eq!(offset, today_plus_1_month_1_day);
    }

    #[test]
    fn test_iso_to_from_fixed() {
        // Reminder: ISO year 0 is Gregorian year 1 BCE.
        // Year 0 is a leap year due to the 400-year rule.
        fn check(fixed: i32, year: i32, month: u8, day: u8) {
            assert_eq!(Iso::iso_year_from_fixed(fixed), year, "fixed: {fixed}");
            assert_eq!(
                Iso::iso_from_fixed(fixed),
                Date::try_new_iso_date(year, month, day).unwrap(),
                "fixed: {fixed}"
            );
            assert_eq!(
                Iso::fixed_from_iso_integers(year, month, day),
                Some(fixed),
                "fixed: {fixed}"
            );
        }
        check(-1828, -5, 12, 30);
        check(-1827, -5, 12, 31); // leap year
        check(-1826, -4, 1, 1);
        check(-1462, -4, 12, 30);
        check(-1461, -4, 12, 31);
        check(-1460, -3, 1, 1);
        check(-1459, -3, 1, 2);
        check(-732, -2, 12, 30);
        check(-731, -2, 12, 31);
        check(-730, -1, 1, 1);
        check(-367, -1, 12, 30);
        check(-366, -1, 12, 31);
        check(-365, 0, 1, 1); // leap year
        check(-364, 0, 1, 2);
        check(-1, 0, 12, 30);
        check(0, 0, 12, 31);
        check(1, 1, 1, 1);
        check(2, 1, 1, 2);
        check(364, 1, 12, 30);
        check(365, 1, 12, 31);
        check(366, 2, 1, 1);
        check(1459, 4, 12, 29);
        check(1460, 4, 12, 30);
        check(1461, 4, 12, 31); // leap year
        check(1462, 5, 1, 1);
    }

    #[test]
    fn test_from_minutes_since_local_unix_epoch() {
        fn check(minutes: i32, year: i32, month: u8, day: u8, hour: u8, minute: u8) {
            let today = DateTime::try_new_iso_datetime(year, month, day, hour, minute, 0).unwrap();
            assert_eq!(today.minutes_since_local_unix_epoch(), minutes);
            assert_eq!(
                DateTime::from_minutes_since_local_unix_epoch(minutes),
                today
            );
        }

        check(-1441, 1969, 12, 30, 23, 59);
        check(-1440, 1969, 12, 31, 0, 0);
        check(-1439, 1969, 12, 31, 0, 1);
        check(-2879, 1969, 12, 30, 0, 1);
    }
}
