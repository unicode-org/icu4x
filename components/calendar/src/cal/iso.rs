// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains types and implementations for the ISO calendar.
//!
//! ```rust
//! use icu::calendar::Date;
//!
//! let date_iso = Date::try_new_iso(1970, 1, 2)
//!     .expect("Failed to initialize ISO Date instance.");
//!
//! assert_eq!(date_iso.era_year().year, 1970);
//! assert_eq!(date_iso.month().ordinal, 1);
//! assert_eq!(date_iso.day_of_month().0, 2);
//! ```

use crate::calendar_arithmetic::{ArithmeticDate, CalendarArithmetic};
use crate::calendar_arithmetic::{ArithmeticDateBuilder, DateFieldsResolver};
use crate::error::DateError;
use crate::options::DateFromFieldsOptions;
use crate::types::DateFields;
use crate::{types, Calendar, Date, DateDuration, DateDurationUnit, RangeError};
use calendrical_calculations::helpers::I32CastError;
use calendrical_calculations::rata_die::RataDie;
use tinystr::tinystr;

/// The [ISO-8601 Calendar](https://en.wikipedia.org/wiki/ISO_8601#Dates)
///
/// The ISO-8601 Calendar is a standardized solar calendar with twelve months.
/// It is identical to the [`Gregorian`](super::Gregorian) calendar, except it uses
/// negative years for years before 1 CE, and may have differing formatting data for a given locale.
///
/// This type can be used with [`Date`] to represent dates in this calendar.
///
/// # Era codes
///
/// This calendar uses a single era: `default`

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(clippy::exhaustive_structs)] // this type is stable
pub struct Iso;

impl Iso {
    pub(crate) const REFERENCE_YEAR: i32 = 1972;
    pub(crate) const LAST_DAY_OF_REFERENCE_YEAR: ArithmeticDate<Iso> =
        ArithmeticDate::<Iso>::new_unchecked_ymd(1972, 12, 31);
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
/// The inner date type used for representing [`Date`]s of [`Iso`]. See [`Date`] and [`Iso`] for more details.
pub struct IsoDateInner(pub(crate) ArithmeticDate<Iso>);

impl IsoDateInner {
    pub(crate) fn iso_year(self) -> i32 {
        self.0.extended_year()
    }
}

impl CalendarArithmetic for Iso {
    type YearInfo = i32;

    fn days_in_provided_month(year: i32, month: u8) -> u8 {
        // https://www.youtube.com/watch?v=J9KijLyP-yg&t=1394s
        if month == 2 {
            28 + calendrical_calculations::iso::is_leap_year(year) as u8
        } else {
            30 | month ^ (month >> 3)
        }
    }

    fn months_in_provided_year(_: i32) -> u8 {
        12
    }

    fn provided_year_is_leap(year: i32) -> bool {
        calendrical_calculations::iso::is_leap_year(year)
    }

    fn last_month_day_in_provided_year(_year: i32) -> (u8, u8) {
        (12, 31)
    }

    fn days_in_provided_year(year: i32) -> u16 {
        365 + calendrical_calculations::iso::is_leap_year(year) as u16
    }

    fn day_of_provided_year(year: Self::YearInfo, month: u8, day: u8) -> u16 {
        calendrical_calculations::iso::days_before_month(year, month) + day as u16
    }

    fn date_from_provided_year_day(year: Self::YearInfo, year_day: u16) -> (u8, u8) {
        calendrical_calculations::iso::year_day(year, year_day)
    }
}

impl DateFieldsResolver for Iso {
    type YearInfo = i32;

    #[inline]
    fn era_year_to_extended(&self, era: &str, era_year: i32) -> Result<Self::YearInfo, DateError> {
        match era {
            "default" => Ok(era_year),
            _ => Err(DateError::UnknownEra),
        }
    }

    #[inline]
    fn year_info_from_extended(&self, extended_year: i32) -> Self::YearInfo {
        extended_year
    }

    #[inline]
    fn reference_year_from_month_day(
        &self,
        _month_code: types::MonthCode,
        _day: u8,
    ) -> Result<Self::YearInfo, DateError> {
        Ok(Self::REFERENCE_YEAR)
    }
}

impl crate::cal::scaffold::UnstableSealed for Iso {}
impl Calendar for Iso {
    type DateInner = IsoDateInner;
    type Year = types::EraYear;
    fn from_fields(
        &self,
        fields: DateFields,
        options: DateFromFieldsOptions,
    ) -> Result<Self::DateInner, DateError> {
        let builder = ArithmeticDateBuilder::try_from_fields(fields, self, options)?;
        ArithmeticDate::try_from_builder(builder, options)
            .map(IsoDateInner)
            .map_err(|e| e.maybe_with_month_code(fields.month_code))
    }

    fn from_rata_die(&self, date: RataDie) -> IsoDateInner {
        IsoDateInner(match calendrical_calculations::iso::iso_from_fixed(date) {
            Err(I32CastError::BelowMin) => ArithmeticDate::min_date(),
            Err(I32CastError::AboveMax) => ArithmeticDate::max_date(),
            Ok((year, month, day)) => ArithmeticDate::new_unchecked_ymd(year, month, day),
        })
    }

    fn to_rata_die(&self, date: &IsoDateInner) -> RataDie {
        calendrical_calculations::iso::fixed_from_iso(date.0.year, date.0.month, date.0.day)
    }

    fn from_iso(&self, iso: IsoDateInner) -> IsoDateInner {
        iso
    }

    fn to_iso(&self, date: &Self::DateInner) -> IsoDateInner {
        *date
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

    fn offset_date(&self, date: &mut Self::DateInner, offset: DateDuration<Self>) {
        date.0.offset_date(offset, &());
    }

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

    fn year_info(&self, date: &Self::DateInner) -> Self::Year {
        let extended_year = date.iso_year();
        types::EraYear {
            era_index: Some(0),
            era: tinystr!(16, "default"),
            year: extended_year,
            extended_year,
            ambiguity: types::YearAmbiguity::Unambiguous,
        }
    }

    fn is_in_leap_year(&self, date: &Self::DateInner) -> bool {
        Self::provided_year_is_leap(date.0.year)
    }

    /// The calendar-specific month represented by `date`
    fn month(&self, date: &Self::DateInner) -> types::MonthInfo {
        date.0.month()
    }

    /// The calendar-specific day-of-month represented by `date`
    fn day_of_month(&self, date: &Self::DateInner) -> types::DayOfMonth {
        date.0.day_of_month()
    }

    fn day_of_year(&self, date: &Self::DateInner) -> types::DayOfYear {
        date.0.day_of_year()
    }

    fn debug_name(&self) -> &'static str {
        "ISO"
    }

    fn calendar_algorithm(&self) -> Option<crate::preferences::CalendarAlgorithm> {
        None
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
    /// assert_eq!(date_iso.era_year().year, 1970);
    /// assert_eq!(date_iso.month().ordinal, 1);
    /// assert_eq!(date_iso.day_of_month().0, 2);
    /// ```
    pub fn try_new_iso(year: i32, month: u8, day: u8) -> Result<Date<Iso>, RangeError> {
        ArithmeticDate::try_from_ymd(year, month, day)
            .map(IsoDateInner)
            .map(|inner| Date::from_raw(inner, Iso))
    }
}

impl Iso {
    /// Construct a new ISO Calendar
    pub fn new() -> Self {
        Self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::Weekday;

    #[test]
    fn iso_overflow() {
        #[derive(Debug)]
        struct TestCase {
            year: i32,
            month: u8,
            day: u8,
            rd: RataDie,
            saturating: bool,
        }
        // Calculates the max possible year representable using i32::MAX as the RD
        let max_year = Iso.from_rata_die(RataDie::new(i32::MAX as i64)).0.year;

        // Calculates the minimum possible year representable using i32::MIN as the RD
        // *Cannot be tested yet due to hard coded date not being available yet (see line 436)
        let min_year = -5879610;

        let cases = [
            TestCase {
                // Earliest date that can be represented before causing a minimum overflow
                year: min_year,
                month: 6,
                day: 22,
                rd: RataDie::new(i32::MIN as i64),
                saturating: false,
            },
            TestCase {
                year: min_year,
                month: 6,
                day: 23,
                rd: RataDie::new(i32::MIN as i64 + 1),
                saturating: false,
            },
            TestCase {
                year: min_year,
                month: 6,
                day: 21,
                rd: RataDie::new(i32::MIN as i64 - 1),
                saturating: false,
            },
            TestCase {
                year: min_year,
                month: 12,
                day: 31,
                rd: RataDie::new(-2147483456),
                saturating: false,
            },
            TestCase {
                year: min_year + 1,
                month: 1,
                day: 1,
                rd: RataDie::new(-2147483455),
                saturating: false,
            },
            TestCase {
                year: max_year,
                month: 6,
                day: 11,
                rd: RataDie::new(i32::MAX as i64 - 30),
                saturating: false,
            },
            TestCase {
                year: max_year,
                month: 7,
                day: 9,
                rd: RataDie::new(i32::MAX as i64 - 2),
                saturating: false,
            },
            TestCase {
                year: max_year,
                month: 7,
                day: 10,
                rd: RataDie::new(i32::MAX as i64 - 1),
                saturating: false,
            },
            TestCase {
                // Latest date that can be represented before causing a maximum overflow
                year: max_year,
                month: 7,
                day: 11,
                rd: RataDie::new(i32::MAX as i64),
                saturating: false,
            },
            TestCase {
                year: max_year,
                month: 7,
                day: 12,
                rd: RataDie::new(i32::MAX as i64 + 1),
                saturating: false,
            },
            TestCase {
                year: i32::MIN,
                month: 1,
                day: 2,
                rd: RataDie::new(-784352296669),
                saturating: false,
            },
            TestCase {
                year: i32::MIN,
                month: 1,
                day: 1,
                rd: RataDie::new(-784352296670),
                saturating: false,
            },
            TestCase {
                year: i32::MIN,
                month: 1,
                day: 1,
                rd: RataDie::new(-784352296671),
                saturating: true,
            },
            TestCase {
                year: i32::MAX,
                month: 12,
                day: 30,
                rd: RataDie::new(784352295938),
                saturating: false,
            },
            TestCase {
                year: i32::MAX,
                month: 12,
                day: 31,
                rd: RataDie::new(784352295939),
                saturating: false,
            },
            TestCase {
                year: i32::MAX,
                month: 12,
                day: 31,
                rd: RataDie::new(784352295940),
                saturating: true,
            },
        ];

        for case in cases {
            let date = Date::try_new_iso(case.year, case.month, case.day).unwrap();
            if !case.saturating {
                assert_eq!(date.to_rata_die(), case.rd, "{case:?}");
            }
            assert_eq!(Date::from_rata_die(case.rd, Iso), date, "{case:?}");
        }
    }

    // Calculates the minimum possible year representable using a large negative fixed date
    #[test]
    fn min_year() {
        assert_eq!(
            Date::from_rata_die(RataDie::big_negative(), Iso)
                .year()
                .era()
                .unwrap()
                .year,
            i32::MIN
        );
    }

    #[test]
    fn test_day_of_week() {
        // June 23, 2021 is a Wednesday
        assert_eq!(
            Date::try_new_iso(2021, 6, 23).unwrap().day_of_week(),
            Weekday::Wednesday,
        );
        // Feb 2, 1983 was a Wednesday
        assert_eq!(
            Date::try_new_iso(1983, 2, 2).unwrap().day_of_week(),
            Weekday::Wednesday,
        );
        // Jan 21, 2021 was a Tuesday
        assert_eq!(
            Date::try_new_iso(2020, 1, 21).unwrap().day_of_week(),
            Weekday::Tuesday,
        );
    }

    #[test]
    fn test_day_of_year() {
        // June 23, 2021 was day 174
        assert_eq!(Date::try_new_iso(2021, 6, 23).unwrap().day_of_year().0, 174,);
        // June 23, 2020 was day 175
        assert_eq!(Date::try_new_iso(2020, 6, 23).unwrap().day_of_year().0, 175,);
        // Feb 2, 1983 was a Wednesday
        assert_eq!(Date::try_new_iso(1983, 2, 2).unwrap().day_of_year().0, 33,);
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
    fn test_iso_to_from_rd() {
        // Reminder: ISO year 0 is Gregorian year 1 BCE.
        // Year 0 is a leap year due to the 400-year rule.
        fn check(rd: i64, year: i32, month: u8, day: u8) {
            let rd = RataDie::new(rd);

            assert_eq!(
                Date::from_rata_die(rd, Iso),
                Date::try_new_iso(year, month, day).unwrap(),
                "RD: {rd:?}"
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
