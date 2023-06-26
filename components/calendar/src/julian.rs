// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains types and implementations for the Julian calendar.
//!
//! ```rust
//! use icu::calendar::{julian::Julian, Date, DateTime};
//!
//! // `Date` type
//! let date_iso = Date::try_new_iso_date(1970, 1, 2)
//!     .expect("Failed to initialize ISO Date instance.");
//! let date_julian = Date::new_from_iso(date_iso, Julian);
//!
//! // `DateTime` type
//! let datetime_iso = DateTime::try_new_iso_datetime(1970, 1, 2, 13, 1, 0)
//!     .expect("Failed to initialize ISO DateTime instance.");
//! let datetime_julian = DateTime::new_from_iso(datetime_iso, Julian);
//!
//! // `Date` checks
//! assert_eq!(date_julian.year().number, 1969);
//! assert_eq!(date_julian.month().ordinal, 12);
//! assert_eq!(date_julian.day_of_month().0, 20);
//!
//! // `DateTime` type
//! assert_eq!(datetime_julian.date.year().number, 1969);
//! assert_eq!(datetime_julian.date.month().ordinal, 12);
//! assert_eq!(datetime_julian.date.day_of_month().0, 20);
//! assert_eq!(datetime_julian.time.hour.number(), 13);
//! assert_eq!(datetime_julian.time.minute.number(), 1);
//! assert_eq!(datetime_julian.time.second.number(), 0);
//! ```

use crate::any_calendar::AnyCalendarKind;
use crate::calendar_arithmetic::{ArithmeticDate, CalendarArithmetic};
use crate::helpers::{i64_to_i32, quotient64, I32Result};
use crate::iso::Iso;
use crate::rata_die::RataDie;
use crate::{types, Calendar, CalendarError, Date, DateDuration, DateDurationUnit, DateTime};
use core::marker::PhantomData;
use tinystr::tinystr;

// Julian epoch is equivalent to fixed_from_iso of December 30th of 0 year
// 1st Jan of 1st year Julian is equivalent to December 30th of 0th year of ISO year
const JULIAN_EPOCH: RataDie = RataDie::new(-1);

/// The [Julian Calendar]
///
/// The [Julian calendar] is a solar calendar that was used commonly historically, with twelve months.
///
/// This type can be used with [`Date`] or [`DateTime`] to represent dates in this calendar.
///
/// [Julian calendar]: https://en.wikipedia.org/wiki/Julian_calendar
///
/// # Era codes
///
/// This calendar supports two era codes: `"bc"`, and `"ad"`, corresponding to the BC and AD eras
#[derive(Copy, Clone, Debug, Hash, Default, Eq, PartialEq)]
#[allow(clippy::exhaustive_structs)] // this type is stable
pub struct Julian;

/// The inner date type used for representing [`Date`]s of [`Julian`]. See [`Date`] and [`Julian`] for more details.
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

    fn months_for_every_year(_: i32) -> u8 {
        12
    }

    fn is_leap_year(year: i32) -> bool {
        Self::is_leap_year_const(year)
    }

    fn last_month_day_in_year(_year: i32) -> (u8, u8) {
        (12, 31)
    }

    fn days_in_provided_year(year: i32) -> u32 {
        if Self::is_leap_year(year) {
            366
        } else {
            365
        }
    }
}

impl Calendar for Julian {
    type DateInner = JulianDateInner;
    fn date_from_codes(
        &self,
        era: types::Era,
        year: i32,
        month_code: types::MonthCode,
        day: u8,
    ) -> Result<Self::DateInner, CalendarError> {
        let year = if era.0 == tinystr!(16, "ad") {
            if year <= 0 {
                return Err(CalendarError::OutOfRange);
            }
            year
        } else if era.0 == tinystr!(16, "bc") {
            if year <= 0 {
                return Err(CalendarError::OutOfRange);
            }
            1 - year
        } else {
            return Err(CalendarError::UnknownEra(era.0, self.debug_name()));
        };

        ArithmeticDate::new_from_solar(self, year, month_code, day).map(JulianDateInner)
    }
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
    /// Julian has the same era scheme as Gregorian
    fn year(&self, date: &Self::DateInner) -> types::FormattableYear {
        crate::gregorian::year_as_gregorian(date.0.year)
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
            prev_year: crate::gregorian::year_as_gregorian(prev_year),
            days_in_prev_year: Julian::days_in_year_direct(prev_year),
            next_year: crate::gregorian::year_as_gregorian(next_year),
        }
    }

    fn debug_name(&self) -> &'static str {
        "Julian"
    }

    fn any_calendar_kind(&self) -> Option<AnyCalendarKind> {
        None
    }
}

impl Julian {
    /// Construct a new Julian Calendar
    pub fn new() -> Self {
        Self
    }

    #[inline(always)]
    const fn is_leap_year_const(year: i32) -> bool {
        year % 4 == 0
    }

    // "Fixed" is a day count representation of calendars staring from Jan 1st of year 1 of the Georgian Calendar.
    // The fixed date algorithms are from
    // Dershowitz, Nachum, and Edward M. Reingold. _Calendrical calculations_. Cambridge University Press, 2008.
    //
    // Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/1ee51ecfaae6f856b0d7de3e36e9042100b4f424/calendar.l#L1689-L1709
    pub(crate) const fn fixed_from_julian(date: ArithmeticDate<Julian>) -> RataDie {
        let year = if date.year < 0 {
            date.year + 1
        } else {
            date.year
        };
        let mut fixed: i64 = JULIAN_EPOCH.to_i64_date() - 1
            + 365 * (year as i64 - 1)
            + quotient64(year as i64 - 1, 4);
        fixed += quotient64(367 * (date.month as i64) - 362, 12);
        fixed += if date.month <= 2 {
            0
        } else if Self::is_leap_year_const(date.year) {
            -1
        } else {
            -2
        };

        RataDie::new(fixed + (date.day as i64))
    }

    pub(crate) const fn fixed_from_julian_integers(year: i32, month: u8, day: u8) -> RataDie {
        Self::fixed_from_julian(ArithmeticDate {
            year,
            month,
            day,
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
    fn julian_from_fixed(date: RataDie) -> JulianDateInner {
        let approx = quotient64((4 * date.to_i64_date()) + 1464, 1461);
        let year = if approx <= 0 { approx - 1 } else { approx };
        let year = match i64_to_i32(year) {
            I32Result::BelowMin(_) => return JulianDateInner(ArithmeticDate::min_date()),
            I32Result::AboveMax(_) => return JulianDateInner(ArithmeticDate::max_date()),
            I32Result::WithinRange(y) => y,
        };
        let prior_days = date - Self::fixed_from_julian_integers(year, 1, 1);
        let correction = if date < Self::fixed_from_julian_integers(year, 3, 1) {
            0
        } else if Julian::is_leap_year(year) {
            1
        } else {
            2
        };
        let month = quotient64(12 * (prior_days + correction) + 373, 367) as u8; // this expression is in 1..=12
        let day = (date - Self::fixed_from_julian_integers(year, month, 1) + 1) as u8; // as days_in_month is < u8::MAX

        #[allow(clippy::unwrap_used)] // day and month have the correct bounds
        *Date::try_new_julian_date(year, month, day).unwrap().inner()
    }
}

impl Date<Julian> {
    /// Construct new Julian Date.
    ///
    /// Zero and negative years are in BC, with year 0 = 1 BC
    ///
    /// ```rust
    /// use icu::calendar::Date;
    ///
    /// let date_julian = Date::try_new_julian_date(1969, 12, 20)
    ///     .expect("Failed to initialize Julian Date instance.");
    ///
    /// assert_eq!(date_julian.year().number, 1969);
    /// assert_eq!(date_julian.month().ordinal, 12);
    /// assert_eq!(date_julian.day_of_month().0, 20);
    /// ```
    pub fn try_new_julian_date(
        year: i32,
        month: u8,
        day: u8,
    ) -> Result<Date<Julian>, CalendarError> {
        let inner = ArithmeticDate {
            year,
            month,
            day,
            marker: PhantomData,
        };

        if day > 28 {
            let bound = inner.days_in_month();
            if day > bound {
                return Err(CalendarError::OutOfRange);
            }
        }

        Ok(Date::from_raw(JulianDateInner(inner), Julian))
    }
}

impl DateTime<Julian> {
    /// Construct a new Julian datetime from integers.
    ///
    /// Zero and negative years are in BC, with year 0 = 1 BC
    ///
    /// ```rust
    /// use icu::calendar::DateTime;
    ///
    /// let datetime_julian =
    ///     DateTime::try_new_julian_datetime(1969, 12, 20, 13, 1, 0)
    ///         .expect("Failed to initialize Julian DateTime instance.");
    ///
    /// assert_eq!(datetime_julian.date.year().number, 1969);
    /// assert_eq!(datetime_julian.date.month().ordinal, 12);
    /// assert_eq!(datetime_julian.date.day_of_month().0, 20);
    /// assert_eq!(datetime_julian.time.hour.number(), 13);
    /// assert_eq!(datetime_julian.time.minute.number(), 1);
    /// assert_eq!(datetime_julian.time.second.number(), 0);
    /// ```
    pub fn try_new_julian_datetime(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Result<DateTime<Julian>, CalendarError> {
        Ok(DateTime {
            date: Date::try_new_julian_date(year, month, day)?,
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
        let iso_date = Date::try_new_iso_date(200, 3, 1).unwrap();
        let julian_date = Julian.date_from_iso(iso_date);
        assert_eq!(julian_date.0.year, 200);
        assert_eq!(julian_date.0.month, 3);
        assert_eq!(julian_date.0.day, 1);

        // Feb 28th, 200 (iso) = Feb 29th, 200 (julian)
        let iso_date = Date::try_new_iso_date(200, 2, 28).unwrap();
        let julian_date = Julian.date_from_iso(iso_date);
        assert_eq!(julian_date.0.year, 200);
        assert_eq!(julian_date.0.month, 2);
        assert_eq!(julian_date.0.day, 29);

        // March 1st 400 (iso) = Feb 29th, 400 (julian)
        let iso_date = Date::try_new_iso_date(400, 3, 1).unwrap();
        let julian_date = Julian.date_from_iso(iso_date);
        assert_eq!(julian_date.0.year, 400);
        assert_eq!(julian_date.0.month, 2);
        assert_eq!(julian_date.0.day, 29);

        // Jan 1st, 2022 (iso) = Dec 19, 2021 (julian)
        let iso_date = Date::try_new_iso_date(2022, 1, 1).unwrap();
        let julian_date = Julian.date_from_iso(iso_date);
        assert_eq!(julian_date.0.year, 2021);
        assert_eq!(julian_date.0.month, 12);
        assert_eq!(julian_date.0.day, 19);
    }

    #[test]
    fn test_day_julian_to_iso() {
        // March 1st 200 is same on both calendars
        let julian_date = Date::try_new_julian_date(200, 3, 1).unwrap();
        let iso_date = Julian.date_to_iso(julian_date.inner());
        let iso_expected_date = Date::try_new_iso_date(200, 3, 1).unwrap();
        assert_eq!(iso_date, iso_expected_date);

        // Feb 28th, 200 (iso) = Feb 29th, 200 (julian)
        let julian_date = Date::try_new_julian_date(200, 2, 29).unwrap();
        let iso_date = Julian.date_to_iso(julian_date.inner());
        let iso_expected_date = Date::try_new_iso_date(200, 2, 28).unwrap();
        assert_eq!(iso_date, iso_expected_date);

        // March 1st 400 (iso) = Feb 29th, 400 (julian)
        let julian_date = Date::try_new_julian_date(400, 2, 29).unwrap();
        let iso_date = Julian.date_to_iso(julian_date.inner());
        let iso_expected_date = Date::try_new_iso_date(400, 3, 1).unwrap();
        assert_eq!(iso_date, iso_expected_date);

        // Jan 1st, 2022 (iso) = Dec 19, 2021 (julian)
        let julian_date = Date::try_new_julian_date(2021, 12, 19).unwrap();
        let iso_date = Julian.date_to_iso(julian_date.inner());
        let iso_expected_date = Date::try_new_iso_date(2022, 1, 1).unwrap();
        assert_eq!(iso_date, iso_expected_date);

        // March 1st, 2022 (iso) = Feb 16, 2022 (julian)
        let julian_date = Date::try_new_julian_date(2022, 2, 16).unwrap();
        let iso_date = Julian.date_to_iso(julian_date.inner());
        let iso_expected_date = Date::try_new_iso_date(2022, 3, 1).unwrap();
        assert_eq!(iso_date, iso_expected_date);
    }

    #[test]
    fn test_roundtrip_negative() {
        // https://github.com/unicode-org/icu4x/issues/2254
        let iso_date = Date::try_new_iso_date(-1000, 3, 3).unwrap();
        let julian = iso_date.to_calendar(Julian::new());
        let recovered_iso = julian.to_iso();
        assert_eq!(iso_date, recovered_iso);
    }
}
