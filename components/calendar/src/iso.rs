// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains types and implementations for the ISO calendar.
//!
//! ```rust
//! use icu::calendar::{Date, DateTime};
//!
//! // `Date` type
//! let date_iso = Date::try_new_iso(1970, 1, 2)
//!     .expect("Failed to initialize ISO Date instance.");
//!
//! // `DateTime` type
//! let datetime_iso = DateTime::try_new_iso(1970, 1, 2, 13, 1, 0)
//!     .expect("Failed to initialize ISO DateTime instance.");
//!
//! // `Date` checks
//! assert_eq!(date_iso.year().era_year_or_extended(), 1970);
//! assert_eq!(date_iso.month().ordinal, 1);
//! assert_eq!(date_iso.day_of_month().0, 2);
//!
//! // `DateTime` type
//! assert_eq!(datetime_iso.date.year().era_year_or_extended(), 1970);
//! assert_eq!(datetime_iso.date.month().ordinal, 1);
//! assert_eq!(datetime_iso.date.day_of_month().0, 2);
//! assert_eq!(datetime_iso.time.hour.number(), 13);
//! assert_eq!(datetime_iso.time.minute.number(), 1);
//! assert_eq!(datetime_iso.time.second.number(), 0);
//! ```

use crate::calendar_arithmetic::{ArithmeticDate, CalendarArithmetic};
use crate::error::DateError;
use crate::{types, Calendar, Date, DateDuration, DateDurationUnit, DateTime, RangeError, Time};
use calendrical_calculations::helpers::I32CastError;
use calendrical_calculations::rata_die::RataDie;
use tinystr::tinystr;

/// The [ISO Calendar]
///
/// The [ISO Calendar] is a standardized solar calendar with twelve months.
/// It is identical to the Gregorian calendar, except it uses negative years for years before 1 CE,
/// and may have differing formatting data for a given locale.
///
/// This type can be used with [`Date`] or [`DateTime`] to represent dates in this calendar.
///
/// [ISO Calendar]: https://en.wikipedia.org/wiki/ISO_8601#Dates
///
/// # Era codes
///
/// This calendar supports one era, `"default"`

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(clippy::exhaustive_structs)] // this type is stable
pub struct Iso;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
/// The inner date type used for representing [`Date`]s of [`Iso`]. See [`Date`] and [`Iso`] for more details.
pub struct IsoDateInner(pub(crate) ArithmeticDate<Iso>);

impl CalendarArithmetic for Iso {
    type YearInfo = ();

    fn month_days(year: i32, month: u8, _: ()) -> u8 {
        // Binary representation of `30` is `0b__11110`
        // Month in 1..=12 represented as `0b__00001`..=`0b__01100`
        // So:
        // A. For any x in 0..31: `30 | x` = `30 + is_odd(x)`
        //  | so `30 | (month ^ (month >> 3))` = `30 + is_odd(month ^ (month >> 3))`
        // B. `month >> 3` is:
        //  |  * `0` for months in 1..=7,
        //  |  * `1` for months in 8..=12,
        // C. From [B] => `is_odd(month ^ (month >> 3))` is
        //  |  * `is_odd(month)`  for months in 1..=7,
        //  |  * `!is_odd(month)` for months in 8..=12,
        //
        // days:  | 31 | 28 | 31 | 30 | 31 | 30 | 31 | 31 | 30 | 31 | 30 | 31 |
        // month: |  1 |  2 |  3 |  4 |  5 |  6 |  7 |  8 |  9 | 10 | 11 | 12 |
        // B:     |  0 |  0 |  0 |  0 |  0 |  0 |  0 |  1 |  1 |  1 |  1 |  1 |
        // C:     |  1 |  0 |  1 |  0 |  1 |  0 |  1 |  1 |  0 |  1 |  0 |  1 |
        // A:     | 31 |=30=| 31 | 30 | 31 | 30 | 31 | 31 | 30 | 31 | 30 | 31 |
        //
        //
        // Avg. speed is ~the same as full matching because of
        // computation time for `30 | (month ^ (month >> 3))`,
        // but there will be less jump and therefore it can be
        // helpful for branch predictor.
        // Also it use less memory space (fewer generated code).
        debug_assert!((1..=12).contains(&month));
        match month {
            2 => 28 | (calendrical_calculations::iso::is_leap_year(year) as u8),
            _ => 30 | (month ^ (month >> 3)),
        }
    }

    fn months_for_every_year(_: i32, _data: ()) -> u8 {
        12
    }

    fn is_leap_year(year: i32, _data: ()) -> bool {
        calendrical_calculations::iso::is_leap_year(year)
    }

    fn last_month_day_in_year(_year: i32, _data: ()) -> (u8, u8) {
        (12, 31)
    }

    fn days_in_provided_year(year: i32, _: ()) -> u16 {
        Self::days_in_year_direct(year)
    }
}

impl Calendar for Iso {
    type DateInner = IsoDateInner;
    /// Construct a date from era/month codes and fields
    fn date_from_codes(
        &self,
        era: Option<types::Era>,
        year: i32,
        month_code: types::MonthCode,
        day: u8,
    ) -> Result<Self::DateInner, DateError> {
        if let Some(era) = era {
            if era.0 != tinystr!(16, "default") {
                return Err(DateError::UnknownEra(era));
            }
        }

        ArithmeticDate::new_from_codes(self, year, month_code, day).map(IsoDateInner)
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

    fn days_in_year(&self, date: &Self::DateInner) -> u16 {
        date.0.days_in_year()
    }

    fn days_in_month(&self, date: &Self::DateInner) -> u8 {
        date.0.days_in_month()
    }

    fn day_of_week(&self, date: &Self::DateInner) -> types::IsoWeekday {
        let day_of_week =
            calendrical_calculations::iso::day_of_week(date.0.year, date.0.month, date.0.day);
        types::IsoWeekday::from(day_of_week as usize)
    }

    fn offset_date(&self, date: &mut Self::DateInner, offset: DateDuration<Self>) {
        date.0.offset_date(offset, &());
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

    fn year(&self, date: &Self::DateInner) -> types::YearInfo {
        Self::year_as_iso(date.0.year)
    }

    fn is_in_leap_year(&self, date: &Self::DateInner) -> bool {
        calendrical_calculations::iso::is_leap_year(date.0.year)
    }

    /// The calendar-specific month represented by `date`
    fn month(&self, date: &Self::DateInner) -> types::MonthInfo {
        date.0.month()
    }

    /// The calendar-specific day-of-month represented by `date`
    fn day_of_month(&self, date: &Self::DateInner) -> types::DayOfMonth {
        date.0.day_of_month()
    }

    fn day_of_year_info(&self, date: &Self::DateInner) -> types::DayOfYearInfo {
        let prev_year = date.0.year.saturating_sub(1);
        let next_year = date.0.year.saturating_add(1);
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

    fn any_calendar_kind(&self) -> Option<crate::AnyCalendarKind> {
        Some(crate::any_calendar::IntoAnyCalendar::kind(self))
    }
}

impl Date<Iso> {
    /// Construct a new ISO date from integers.
    ///
    /// ```rust
    /// use icu::calendar::Date;
    ///
    /// let date_iso = Date::try_new_iso(1970, 1, 2)
    ///     .expect("Failed to initialize ISO Date instance.");
    ///
    /// assert_eq!(date_iso.year().era_year_or_extended(), 1970);
    /// assert_eq!(date_iso.month().ordinal, 1);
    /// assert_eq!(date_iso.day_of_month().0, 2);
    /// ```
    pub fn try_new_iso(year: i32, month: u8, day: u8) -> Result<Date<Iso>, RangeError> {
        ArithmeticDate::new_from_ordinals(year, month, day)
            .map(IsoDateInner)
            .map(|inner| Date::from_raw(inner, Iso))
    }
}

impl DateTime<Iso> {
    /// Construct a new ISO datetime from integers.
    ///
    /// ```rust
    /// use icu::calendar::DateTime;
    ///
    /// let datetime_iso = DateTime::try_new_iso(1970, 1, 2, 13, 1, 0)
    ///     .expect("Failed to initialize ISO DateTime instance.");
    ///
    /// assert_eq!(datetime_iso.date.year().era_year_or_extended(), 1970);
    /// assert_eq!(datetime_iso.date.month().ordinal, 1);
    /// assert_eq!(datetime_iso.date.day_of_month().0, 2);
    /// assert_eq!(datetime_iso.time.hour.number(), 13);
    /// assert_eq!(datetime_iso.time.minute.number(), 1);
    /// assert_eq!(datetime_iso.time.second.number(), 0);
    /// ```
    pub fn try_new_iso(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Result<DateTime<Iso>, DateError> {
        Ok(DateTime {
            date: Date::try_new_iso(year, month, day)?,
            time: Time::try_new(hour, minute, second, 0)?,
        })
    }
}

impl Iso {
    /// Construct a new ISO Calendar
    pub fn new() -> Self {
        Self
    }

    // /// Count the number of days in a given month/year combo
    // const fn days_in_month(year: i32, month: u8) -> u8 {
    //     // see comment to `<impl CalendarArithmetic for Iso>::month_days`
    //     match month {
    //         2 => 28 | (is_leap_year(year) as u8),
    //         _ => 30 | (month ^ (month >> 3)),
    //     }
    // }

    pub(crate) const fn days_in_year_direct(year: i32) -> u16 {
        if calendrical_calculations::iso::is_leap_year(year) {
            366
        } else {
            365
        }
    }

    // Fixed is day count representation of calendars starting from Jan 1st of year 1.
    // The fixed calculations algorithms are from the Calendrical Calculations book.
    pub(crate) fn fixed_from_iso(date: IsoDateInner) -> RataDie {
        calendrical_calculations::iso::fixed_from_iso(date.0.year, date.0.month, date.0.day)
    }

    pub(crate) fn iso_from_year_day(year: i32, year_day: u16) -> Date<Iso> {
        let (month, day) = calendrical_calculations::iso::iso_from_year_day(year, year_day);
        debug_assert!(month < 13);

        #[allow(clippy::unwrap_used)] // month in 1..=12, day <= month_days
        Date::try_new_iso(year, month, day).unwrap()
    }
    pub(crate) fn iso_from_fixed(date: RataDie) -> Date<Iso> {
        let (year, month, day) = match calendrical_calculations::iso::iso_from_fixed(date) {
            Err(I32CastError::BelowMin) => {
                return Date::from_raw(IsoDateInner(ArithmeticDate::min_date()), Iso)
            }
            Err(I32CastError::AboveMax) => {
                return Date::from_raw(IsoDateInner(ArithmeticDate::max_date()), Iso)
            }
            Ok(ymd) => ymd,
        };
        #[allow(clippy::unwrap_used)] // valid day and month
        Date::try_new_iso(year, month, day).unwrap()
    }

    pub(crate) fn day_of_year(date: IsoDateInner) -> u16 {
        calendrical_calculations::iso::day_of_year(date.0.year, date.0.month, date.0.day)
    }

    /// Wrap the year in the appropriate era code
    fn year_as_iso(year: i32) -> types::YearInfo {
        types::YearInfo::new(
            year,
            types::EraYear {
                formatting_era: types::FormattingEra::Index(0, tinystr!(16, "")),
                standard_era: tinystr!(16, "default").into(),
                era_year: year,
                ambiguity: types::YearAmbiguity::Unambiguous,
            },
        )
    }
}

impl IsoDateInner {
    pub(crate) fn jan_1(year: i32) -> Self {
        Self(ArithmeticDate::new_unchecked(year, 1, 1))
    }
    pub(crate) fn dec_31(year: i32) -> Self {
        Self(ArithmeticDate::new_unchecked(year, 12, 1))
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
    fn iso_overflow() {
        #[derive(Debug)]
        struct TestCase {
            year: i32,
            month: u8,
            day: u8,
            fixed: RataDie,
            saturating: bool,
        }
        // Calculates the max possible year representable using i32::MAX as the fixed date
        let max_year = Iso::iso_from_fixed(RataDie::new(i32::MAX as i64))
            .year()
            .era_year()
            .unwrap();

        // Calculates the minimum possible year representable using i32::MIN as the fixed date
        // *Cannot be tested yet due to hard coded date not being available yet (see line 436)
        let min_year = -5879610;

        let cases = [
            TestCase {
                // Earliest date that can be represented before causing a minimum overflow
                year: min_year,
                month: 6,
                day: 22,
                fixed: RataDie::new(i32::MIN as i64),
                saturating: false,
            },
            TestCase {
                year: min_year,
                month: 6,
                day: 23,
                fixed: RataDie::new(i32::MIN as i64 + 1),
                saturating: false,
            },
            TestCase {
                year: min_year,
                month: 6,
                day: 21,
                fixed: RataDie::new(i32::MIN as i64 - 1),
                saturating: false,
            },
            TestCase {
                year: min_year,
                month: 12,
                day: 31,
                fixed: RataDie::new(-2147483456),
                saturating: false,
            },
            TestCase {
                year: min_year + 1,
                month: 1,
                day: 1,
                fixed: RataDie::new(-2147483455),
                saturating: false,
            },
            TestCase {
                year: max_year,
                month: 6,
                day: 11,
                fixed: RataDie::new(i32::MAX as i64 - 30),
                saturating: false,
            },
            TestCase {
                year: max_year,
                month: 7,
                day: 9,
                fixed: RataDie::new(i32::MAX as i64 - 2),
                saturating: false,
            },
            TestCase {
                year: max_year,
                month: 7,
                day: 10,
                fixed: RataDie::new(i32::MAX as i64 - 1),
                saturating: false,
            },
            TestCase {
                // Latest date that can be represented before causing a maximum overflow
                year: max_year,
                month: 7,
                day: 11,
                fixed: RataDie::new(i32::MAX as i64),
                saturating: false,
            },
            TestCase {
                year: max_year,
                month: 7,
                day: 12,
                fixed: RataDie::new(i32::MAX as i64 + 1),
                saturating: false,
            },
            TestCase {
                year: i32::MIN,
                month: 1,
                day: 2,
                fixed: RataDie::new(-784352296669),
                saturating: false,
            },
            TestCase {
                year: i32::MIN,
                month: 1,
                day: 1,
                fixed: RataDie::new(-784352296670),
                saturating: false,
            },
            TestCase {
                year: i32::MIN,
                month: 1,
                day: 1,
                fixed: RataDie::new(-784352296671),
                saturating: true,
            },
            TestCase {
                year: i32::MAX,
                month: 12,
                day: 30,
                fixed: RataDie::new(784352295938),
                saturating: false,
            },
            TestCase {
                year: i32::MAX,
                month: 12,
                day: 31,
                fixed: RataDie::new(784352295939),
                saturating: false,
            },
            TestCase {
                year: i32::MAX,
                month: 12,
                day: 31,
                fixed: RataDie::new(784352295940),
                saturating: true,
            },
        ];

        for case in cases {
            let date = Date::try_new_iso(case.year, case.month, case.day).unwrap();
            if !case.saturating {
                assert_eq!(Iso::fixed_from_iso(date.inner), case.fixed, "{case:?}");
            }
            assert_eq!(Iso::iso_from_fixed(case.fixed), date, "{case:?}");
        }
    }

    // Calculates the minimum possible year representable using a large negative fixed date
    #[test]
    fn min_year() {
        assert_eq!(
            Iso::iso_from_fixed(RataDie::big_negative())
                .year()
                .era_year_or_extended(),
            i32::MIN
        );
    }

    #[test]
    fn test_day_of_week() {
        // June 23, 2021 is a Wednesday
        assert_eq!(
            Date::try_new_iso(2021, 6, 23).unwrap().day_of_week(),
            IsoWeekday::Wednesday,
        );
        // Feb 2, 1983 was a Wednesday
        assert_eq!(
            Date::try_new_iso(1983, 2, 2).unwrap().day_of_week(),
            IsoWeekday::Wednesday,
        );
        // Jan 21, 2021 was a Tuesday
        assert_eq!(
            Date::try_new_iso(2020, 1, 21).unwrap().day_of_week(),
            IsoWeekday::Tuesday,
        );
    }

    #[test]
    fn test_day_of_year() {
        // June 23, 2021 was day 174
        assert_eq!(
            Date::try_new_iso(2021, 6, 23)
                .unwrap()
                .day_of_year_info()
                .day_of_year,
            174,
        );
        // June 23, 2020 was day 175
        assert_eq!(
            Date::try_new_iso(2020, 6, 23)
                .unwrap()
                .day_of_year_info()
                .day_of_year,
            175,
        );
        // Feb 2, 1983 was a Wednesday
        assert_eq!(
            Date::try_new_iso(1983, 2, 2)
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
        let today = Date::try_new_iso(2021, 6, 23).unwrap();
        let today_plus_5000 = Date::try_new_iso(2035, 3, 2).unwrap();
        let offset = today.added(DateDuration::new(0, 0, 0, 5000));
        assert_eq!(offset, today_plus_5000);
        let offset = today.added(simple_subtract(&today_plus_5000, &today));
        assert_eq!(offset, today_plus_5000);

        let today = Date::try_new_iso(2021, 6, 23).unwrap();
        let today_minus_5000 = Date::try_new_iso(2007, 10, 15).unwrap();
        let offset = today.added(DateDuration::new(0, 0, 0, -5000));
        assert_eq!(offset, today_minus_5000);
        let offset = today.added(simple_subtract(&today_minus_5000, &today));
        assert_eq!(offset, today_minus_5000);
    }

    #[test]
    fn test_offset_at_month_boundary() {
        let today = Date::try_new_iso(2020, 2, 28).unwrap();
        let today_plus_2 = Date::try_new_iso(2020, 3, 1).unwrap();
        let offset = today.added(DateDuration::new(0, 0, 0, 2));
        assert_eq!(offset, today_plus_2);

        let today = Date::try_new_iso(2020, 2, 28).unwrap();
        let today_plus_3 = Date::try_new_iso(2020, 3, 2).unwrap();
        let offset = today.added(DateDuration::new(0, 0, 0, 3));
        assert_eq!(offset, today_plus_3);

        let today = Date::try_new_iso(2020, 2, 28).unwrap();
        let today_plus_1 = Date::try_new_iso(2020, 2, 29).unwrap();
        let offset = today.added(DateDuration::new(0, 0, 0, 1));
        assert_eq!(offset, today_plus_1);

        let today = Date::try_new_iso(2019, 2, 28).unwrap();
        let today_plus_2 = Date::try_new_iso(2019, 3, 2).unwrap();
        let offset = today.added(DateDuration::new(0, 0, 0, 2));
        assert_eq!(offset, today_plus_2);

        let today = Date::try_new_iso(2019, 2, 28).unwrap();
        let today_plus_1 = Date::try_new_iso(2019, 3, 1).unwrap();
        let offset = today.added(DateDuration::new(0, 0, 0, 1));
        assert_eq!(offset, today_plus_1);

        let today = Date::try_new_iso(2020, 3, 1).unwrap();
        let today_minus_1 = Date::try_new_iso(2020, 2, 29).unwrap();
        let offset = today.added(DateDuration::new(0, 0, 0, -1));
        assert_eq!(offset, today_minus_1);
    }

    #[test]
    fn test_offset_handles_negative_month_offset() {
        let today = Date::try_new_iso(2020, 3, 1).unwrap();
        let today_minus_2_months = Date::try_new_iso(2020, 1, 1).unwrap();
        let offset = today.added(DateDuration::new(0, -2, 0, 0));
        assert_eq!(offset, today_minus_2_months);

        let today = Date::try_new_iso(2020, 3, 1).unwrap();
        let today_minus_4_months = Date::try_new_iso(2019, 11, 1).unwrap();
        let offset = today.added(DateDuration::new(0, -4, 0, 0));
        assert_eq!(offset, today_minus_4_months);

        let today = Date::try_new_iso(2020, 3, 1).unwrap();
        let today_minus_24_months = Date::try_new_iso(2018, 3, 1).unwrap();
        let offset = today.added(DateDuration::new(0, -24, 0, 0));
        assert_eq!(offset, today_minus_24_months);

        let today = Date::try_new_iso(2020, 3, 1).unwrap();
        let today_minus_27_months = Date::try_new_iso(2017, 12, 1).unwrap();
        let offset = today.added(DateDuration::new(0, -27, 0, 0));
        assert_eq!(offset, today_minus_27_months);
    }

    #[test]
    fn test_offset_handles_out_of_bound_month_offset() {
        let today = Date::try_new_iso(2021, 1, 31).unwrap();
        // since 2021/02/31 isn't a valid date, `offset_date` auto-adjusts by adding 3 days to 2021/02/28
        let today_plus_1_month = Date::try_new_iso(2021, 3, 3).unwrap();
        let offset = today.added(DateDuration::new(0, 1, 0, 0));
        assert_eq!(offset, today_plus_1_month);

        let today = Date::try_new_iso(2021, 1, 31).unwrap();
        // since 2021/02/31 isn't a valid date, `offset_date` auto-adjusts by adding 3 days to 2021/02/28
        let today_plus_1_month_1_day = Date::try_new_iso(2021, 3, 4).unwrap();
        let offset = today.added(DateDuration::new(0, 1, 0, 1));
        assert_eq!(offset, today_plus_1_month_1_day);
    }

    #[test]
    fn test_iso_to_from_fixed() {
        // Reminder: ISO year 0 is Gregorian year 1 BCE.
        // Year 0 is a leap year due to the 400-year rule.
        fn check(fixed: i64, year: i32, month: u8, day: u8) {
            let fixed = RataDie::new(fixed);

            assert_eq!(
                Iso::iso_from_fixed(fixed),
                Date::try_new_iso(year, month, day).unwrap(),
                "fixed: {fixed:?}"
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
}
