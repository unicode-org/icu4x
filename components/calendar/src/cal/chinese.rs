// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains types and implementations for the Chinese calendar.
//!
//! ```rust
//! use icu::calendar::{cal::Chinese, Date};
//!
//! let chinese = Chinese::new();
//! let chinese_date = Date::try_new_chinese_with_calendar(2023, 6, 6, chinese)
//!     .expect("Failed to initialize Chinese Date instance.");
//!
//! assert_eq!(chinese_date.cyclic_year().related_iso, 2023);
//! assert_eq!(chinese_date.cyclic_year().year, 40);
//! assert_eq!(chinese_date.month().ordinal, 6);
//! assert_eq!(chinese_date.day_of_month().0, 6);
//! ```

use crate::cal::chinese_based::ChineseBasedWithDataLoading;
use crate::cal::iso::{Iso, IsoDateInner};
use crate::calendar_arithmetic::{ArithmeticDate, ArithmeticDateBuilder, CalendarArithmetic};
use crate::calendar_arithmetic::{DateFieldsResolver, PrecomputedDataSource};
use crate::error::DateError;
use crate::options::DateFromFieldsOptions;
use crate::provider::chinese_based::CalendarChineseV1;
use crate::AsCalendar;
use crate::{types, Calendar, Date, DateDuration, DateDurationUnit};
use calendrical_calculations::rata_die::RataDie;
use core::cmp::Ordering;
use core::num::NonZeroU8;
use icu_provider::prelude::*;

mod data;

/// The [Chinese Calendar](https://en.wikipedia.org/wiki/Chinese_calendar)
///
/// The Chinese Calendar is a lunisolar calendar used traditionally in China as well as in other
/// countries particularly in, but not limited to, East Asia. It is often used today to track important
/// cultural events and holidays like the Chinese Lunar New Year.
///
/// This type can be used with [`Date`] to represent dates in the Chinese calendar.
///
/// # Months
///
/// The Chinese calendar is an astronomical calendar which uses the phases of the moon to track months.
/// Each month starts on the date of the new moon as observed from China, meaning that months last 29
/// or 30 days.
///
/// One year in the Chinese calendar is typically 12 lunar months; however, because 12 lunar months does
/// not line up to one solar year, the Chinese calendar will add an intercalary leap month approximately
/// every three years to keep Chinese calendar months in line with the solar year.
///
/// Leap months can happen after any month; the month in which a leap month occurs is based on the alignment
/// of months with 24 solar terms into which the solar year is divided.
///
/// # Year and Era codes
///
/// Unlike the Gregorian calendar, the Chinese calendar does not traditionally count years in an infinitely
/// increasing sequence. Instead, 10 "celestial stems" and 12 "terrestrial branches" are combined to form a
/// cycle of year names which repeats every 60 years. However, for the purposes of calendar calculations and
/// conversions, this calendar also counts years based on the ISO (Gregorian) calendar. This "related ISO year"
/// marks the ISO year in which a Chinese year begins.
///
/// Because the Chinese calendar does not traditionally count years, era codes are not used in this calendar.
///
/// For more information, suggested reading materials include:
/// * _Calendrical Calculations_ by Reingold & Dershowitz
/// * _The Mathematics of the Chinese Calendar_ by Helmer Aslaksen <https://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.139.9311&rep=rep1&type=pdf>
/// * Wikipedia: <https://en.wikipedia.org/wiki/Chinese_calendar>
///
/// # Month codes
///
/// This calendar is a lunisolar calendar. It supports regular month codes `"M01" - "M12"` as well
/// as leap month codes `"M01L" - "M12L"`.
///
/// This calendar is currently in a preview state: formatting for this calendar is not
/// going to be perfect.
#[derive(Clone, Debug, Default, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub struct Chinese;

/// The inner date type used for representing [`Date`]s of [`Chinese`]. See [`Date`] and [`Chinese`] for more details.
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Copy, Clone)]
pub struct ChineseDateInner(ArithmeticDate<Chinese>);

impl Chinese {
    /// Creates a new [`Chinese`] with some precomputed calendrical calculations.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    pub const fn new() -> Self {
        Self
    }

    #[cfg(feature = "serde")]
    #[doc = icu_provider::gen_buffer_unstable_docs!(BUFFER,Self::new)]
    #[allow(deprecated)]
    #[deprecated]
    pub fn try_new_with_buffer_provider(
        provider: &(impl icu_provider::buf::BufferProvider + ?Sized),
    ) -> Result<Self, DataError> {
        use icu_provider::buf::AsDeserializingBufferProvider;
        Self::try_new_unstable(&provider.as_deserializing())
    }

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::new)]
    #[deprecated]
    pub fn try_new_unstable<D: ?Sized>(_provider: &D) -> Result<Self, DataError> {
        Ok(Self)
    }

    /// Construct a new [`Chinese`] without any precomputed calendrical calculations.
    #[deprecated]
    pub fn new_always_calculating() -> Self {
        Self
    }

    pub(crate) const DEBUG_NAME: &'static str = "Chinese";
}

impl DateFieldsResolver for Chinese {
    type YearInfo = super::chinese_based::ChineseBasedYearInfo;

    #[inline]
    fn era_year_to_monotonic(
        &self,
        _era: &str,
        _era_year: i32,
    ) -> Result<Self::YearInfo, DateError> {
        // This calendar has no era codes
        Err(DateError::UnknownEra)
    }

    #[inline]
    fn year_info_from_extended(&self, extended_year: i32) -> Self::YearInfo {
        self.get_precomputed_data()
            .load_or_compute_info(extended_year)
    }

    fn reference_year_from_month_day(
        &self,
        month_code: types::MonthCode,
        day: u8,
    ) -> Result<Self::YearInfo, DateError> {
        todo!()
    }

    fn ordinal_month_from_code(
        &self,
        year: &Self::YearInfo,
        month_code: types::MonthCode,
    ) -> Result<u8, DateError> {
        year.parse_month_code(month_code)
            .ok_or(DateError::UnknownMonthCode(month_code))
    }
}

impl crate::cal::scaffold::UnstableSealed for Chinese {}
impl Calendar for Chinese {
    type DateInner = ChineseDateInner;
    type Year = types::CyclicYear;

    fn from_fields(
        &self,
        fields: types::DateFields,
        options: DateFromFieldsOptions,
    ) -> Result<Self::DateInner, DateError> {
        let builder = ArithmeticDateBuilder::try_from_fields(fields, self, options)?;
        builder.year.validate_md(builder.month, builder.day)?;
        Ok(ChineseDateInner(ArithmeticDate::new_unchecked(builder)))
    }

    fn from_rata_die(&self, rd: RataDie) -> Self::DateInner {
        let iso = Iso.from_rata_die(rd);
        let y = Self::load_or_compute_info_for_rd(rd, iso.iso_year());
        let (m, d) = y.md_from_rd(rd);
        ChineseDateInner(ArithmeticDate::new_unchecked_ymd(y, m, d))
    }

    fn to_rata_die(&self, date: &Self::DateInner) -> RataDie {
        date.0.year.rd_from_md(date.0.month, date.0.day)
    }

    fn from_iso(&self, iso: IsoDateInner) -> Self::DateInner {
        let rd = Iso.to_rata_die(&iso);
        let y = Self::load_or_compute_info_for_rd(rd, iso.iso_year());
        let (m, d) = y.md_from_rd(rd);
        ChineseDateInner(ArithmeticDate::new_unchecked_ymd(y, m, d))
    }

    fn to_iso(&self, date: &Self::DateInner) -> IsoDateInner {
        Iso.from_rata_die(self.to_rata_die(date))
    }

    // Count the number of months in a given year, specified by providing a date
    // from that year
    fn days_in_year(&self, date: &Self::DateInner) -> u16 {
        date.0.days_in_year()
    }

    fn days_in_month(&self, date: &Self::DateInner) -> u8 {
        date.0.days_in_month()
    }

    #[doc(hidden)] // unstable
    fn offset_date(&self, date: &mut Self::DateInner, offset: DateDuration<Self>) {
        date.0.offset_date(offset, &Self);
    }

    #[doc(hidden)] // unstable
    /// Calculate `date2 - date` as a duration
    ///
    /// `calendar2` is the calendar object associated with `date2`. In case the specific calendar objects
    /// differ on date, the date for the first calendar is used, and `date2` may be converted if necessary.
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

    /// Obtain a name for the calendar for debug printing
    fn debug_name(&self) -> &'static str {
        Self::DEBUG_NAME
    }

    fn year_info(&self, date: &Self::DateInner) -> Self::Year {
        let year = date.0.year;
        types::CyclicYear {
            year: (year.related_iso - 4).rem_euclid(60) as u8 + 1,
            related_iso: year.related_iso,
        }
    }

    fn is_in_leap_year(&self, date: &Self::DateInner) -> bool {
        Self::provided_year_is_leap(date.0.year)
    }

    /// The calendar-specific month code represented by `date`;
    /// since the Chinese calendar has leap months, an "L" is appended to the month code for
    /// leap months. For example, in a year where an intercalary month is added after the second
    /// month, the month codes for ordinal months 1, 2, 3, 4, 5 would be "M01", "M02", "M02L", "M03", "M04".
    fn month(&self, date: &Self::DateInner) -> types::MonthInfo {
        date.0.year.month(date.0.month)
    }

    /// The calendar-specific day-of-month represented by `date`
    fn day_of_month(&self, date: &Self::DateInner) -> types::DayOfMonth {
        date.0.day_of_month()
    }

    /// Information of the day of the year
    fn day_of_year(&self, date: &Self::DateInner) -> types::DayOfYear {
        types::DayOfYear(date.0.year.day_of_year(date.0.month, date.0.day))
    }

    fn calendar_algorithm(&self) -> Option<crate::preferences::CalendarAlgorithm> {
        Some(crate::preferences::CalendarAlgorithm::Chinese)
    }

    fn months_in_year(&self, date: &Self::DateInner) -> u8 {
        date.0.months_in_year()
    }
}

impl<A: AsCalendar<Calendar = Chinese>> Date<A> {
    /// Construct a new Chinese date from a `year`, `month`, and `day`.
    /// `year` represents the [ISO](crate::Iso) year that roughly matches the Chinese year;
    /// `month` represents the month of the year ordinally (ex. if it is a leap year, the last month will be 13, not 12);
    /// `day` indicates the day of month
    ///
    /// This date will not use any precomputed calendrical calculations,
    /// one that loads such data from a provider will be added in the future (#3933)
    ///
    /// ```rust
    /// use icu::calendar::{cal::Chinese, Date};
    ///
    /// let chinese = Chinese::new();
    ///
    /// let date_chinese =
    ///     Date::try_new_chinese_with_calendar(2023, 6, 11, chinese)
    ///         .expect("Failed to initialize Chinese Date instance.");
    ///
    /// assert_eq!(date_chinese.cyclic_year().related_iso, 2023);
    /// assert_eq!(date_chinese.cyclic_year().year, 40);
    /// assert_eq!(date_chinese.month().ordinal, 6);
    /// assert_eq!(date_chinese.day_of_month().0, 11);
    /// ```
    pub fn try_new_chinese_with_calendar(
        related_iso_year: i32,
        month: u8,
        day: u8,
        calendar: A,
    ) -> Result<Date<A>, DateError> {
        let year = Chinese.load_or_compute_info(related_iso_year);
        year.validate_md(month, day)?;
        Ok(Date::from_raw(
            ChineseDateInner(ArithmeticDate::new_unchecked_ymd(year, month, day)),
            calendar,
        ))
    }
}

type ChineseCB = calendrical_calculations::chinese_based::Chinese;
impl ChineseBasedWithDataLoading for Chinese {
    type CB = ChineseCB;
    const DATA: &ChineseBasedCache<'static> = &ChineseBasedCache {
        first_related_iso_year: data::STARTING_YEAR,
        data: data::DATA,
    };
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::MonthCode;
    use calendrical_calculations::{iso::fixed_from_iso, rata_die::RataDie};
    use tinystr::tinystr;

    #[test]
    fn test_chinese_from_rd() {
        #[derive(Debug)]
        struct TestCase {
            rd: i64,
            expected_year: i32,
            expected_month: u8,
            expected_day: u8,
        }

        let cases = [
            TestCase {
                rd: -964192,
                expected_year: -2639,
                expected_month: 1,
                expected_day: 1,
            },
            TestCase {
                rd: -963838,
                expected_year: -2638,
                expected_month: 1,
                expected_day: 1,
            },
            TestCase {
                rd: -963129,
                expected_year: -2637,
                expected_month: 13,
                expected_day: 1,
            },
            TestCase {
                rd: -963100,
                expected_year: -2637,
                expected_month: 13,
                expected_day: 30,
            },
            TestCase {
                rd: -963099,
                expected_year: -2636,
                expected_month: 1,
                expected_day: 1,
            },
            TestCase {
                rd: 738700,
                expected_year: 2023,
                expected_month: 6,
                expected_day: 12,
            },
            TestCase {
                rd: fixed_from_iso(2319, 2, 20).to_i64_date(),
                expected_year: 2318,
                expected_month: 13,
                expected_day: 30,
            },
            TestCase {
                rd: fixed_from_iso(2319, 2, 21).to_i64_date(),
                expected_year: 2319,
                expected_month: 1,
                expected_day: 1,
            },
            TestCase {
                rd: 738718,
                expected_year: 2023,
                expected_month: 6,
                expected_day: 30,
            },
            TestCase {
                rd: 738747,
                expected_year: 2023,
                expected_month: 7,
                expected_day: 29,
            },
            TestCase {
                rd: 738748,
                expected_year: 2023,
                expected_month: 8,
                expected_day: 1,
            },
            TestCase {
                rd: 738865,
                expected_year: 2023,
                expected_month: 11,
                expected_day: 29,
            },
            TestCase {
                rd: 738895,
                expected_year: 2023,
                expected_month: 12,
                expected_day: 29,
            },
            TestCase {
                rd: 738925,
                expected_year: 2023,
                expected_month: 13,
                expected_day: 30,
            },
            TestCase {
                rd: 0,
                expected_year: 0,
                expected_month: 12,
                expected_day: 20,
            },
            TestCase {
                rd: -1,
                expected_year: 0,
                expected_month: 12,
                expected_day: 19,
            },
            TestCase {
                rd: -365,
                expected_year: -1,
                expected_month: 12,
                expected_day: 9,
            },
            TestCase {
                rd: 100,
                expected_year: 1,
                expected_month: 3,
                expected_day: 1,
            },
        ];

        for case in cases {
            let rata_die = RataDie::new(case.rd);

            let chinese = Date::from_rata_die(rata_die, Chinese);
            assert_eq!(
                case.expected_year,
                chinese.extended_year(),
                "Chinese from RD failed, case: {case:?}"
            );
            assert_eq!(
                case.expected_month,
                chinese.month().ordinal,
                "Chinese from RD failed, case: {case:?}"
            );
            assert_eq!(
                case.expected_day,
                chinese.day_of_month().0,
                "Chinese from RD failed, case: {case:?}"
            );
        }
    }

    #[test]
    fn test_rd_from_chinese() {
        #[derive(Debug)]
        struct TestCase {
            year: i32,
            month: u8,
            day: u8,
            expected: i64,
        }

        let cases = [
            TestCase {
                year: 2023,
                month: 6,
                day: 6,
                // June 23 2023
                expected: 738694,
            },
            TestCase {
                year: -2636,
                month: 1,
                day: 1,
                expected: -963099,
            },
        ];

        for case in cases {
            let date =
                Date::try_new_chinese_with_calendar(case.year, case.month, case.day, Chinese)
                    .unwrap();
            let rd = date.to_rata_die().to_i64_date();
            let expected = case.expected;
            assert_eq!(rd, expected, "RD from Chinese failed, with expected: {expected} and calculated: {rd}, for test case: {case:?}");
        }
    }

    #[test]
    fn test_rd_chinese_roundtrip() {
        let mut rd = -1963020;
        let max_rd = 1963020;
        let mut iters = 0;
        let max_iters = 560;
        while rd < max_rd && iters < max_iters {
            let rata_die = RataDie::new(rd);

            let chinese = Date::from_rata_die(rata_die, Chinese);
            let result = chinese.to_rata_die();
            assert_eq!(result, rata_die, "Failed roundtrip RD -> Chinese -> RD for RD: {rata_die:?}, with calculated: {result:?} from Chinese date:\n{chinese:?}");

            rd += 7043;
            iters += 1;
        }
    }

    #[test]
    fn test_chinese_epoch() {
        let iso = Date::try_new_iso(-2636, 2, 15).unwrap();

        let chinese = iso.to_calendar(Chinese);

        assert_eq!(chinese.cyclic_year().related_iso, -2636);
        assert_eq!(chinese.month().ordinal, 1);
        assert_eq!(chinese.month().standard_code.0, "M01");
        assert_eq!(chinese.day_of_month().0, 1);
        assert_eq!(chinese.cyclic_year().year, 1);
        assert_eq!(chinese.cyclic_year().related_iso, -2636);
    }

    #[test]
    fn test_iso_to_chinese_negative_years() {
        #[derive(Debug)]
        struct TestCase {
            iso_year: i32,
            iso_month: u8,
            iso_day: u8,
            expected_year: i32,
            expected_month: u8,
            expected_day: u8,
        }

        let cases = [
            TestCase {
                iso_year: -2636,
                iso_month: 2,
                iso_day: 14,
                expected_year: -2637,
                expected_month: 13,
                expected_day: 30,
            },
            TestCase {
                iso_year: -2636,
                iso_month: 1,
                iso_day: 15,
                expected_year: -2637,
                expected_month: 12,
                expected_day: 30,
            },
        ];

        for case in cases {
            let iso = Date::try_new_iso(case.iso_year, case.iso_month, case.iso_day).unwrap();

            let chinese = iso.to_calendar(Chinese);
            assert_eq!(
                case.expected_year,
                chinese.cyclic_year().related_iso,
                "ISO to Chinese failed for case: {case:?}"
            );
            assert_eq!(
                case.expected_month,
                chinese.month().ordinal,
                "ISO to Chinese failed for case: {case:?}"
            );
            assert_eq!(
                case.expected_day,
                chinese.day_of_month().0,
                "ISO to Chinese failed for case: {case:?}"
            );
        }
    }

    #[test]
    fn test_chinese_leap_months() {
        let expected = [
            (1933, 6),
            (1938, 8),
            (1984, 11),
            (2009, 6),
            (2017, 7),
            (2028, 6),
        ];

        for case in expected {
            let year = case.0;
            let expected_month = case.1;
            let iso = Date::try_new_iso(year, 6, 1).unwrap();

            let chinese_date = iso.to_calendar(Chinese);
            assert!(
                chinese_date.is_in_leap_year(),
                "{year} should be a leap year"
            );
            let new_year = chinese_date.inner.0.year.new_year();
            assert_eq!(
                expected_month,
                calendrical_calculations::chinese_based::get_leap_month_from_new_year::<
                    calendrical_calculations::chinese_based::Chinese,
                >(new_year),
                "{year} have leap month {expected_month}"
            );
        }
    }

    #[test]
    fn test_month_days() {
        let year = Chinese.load_or_compute_info(2023);
        let cases = [
            (1, 29),
            (2, 30),
            (3, 29),
            (4, 29),
            (5, 30),
            (6, 30),
            (7, 29),
            (8, 30),
            (9, 30),
            (10, 29),
            (11, 30),
            (12, 29),
            (13, 30),
        ];
        for case in cases {
            let days_in_month = Chinese::days_in_provided_month(year, case.0);
            assert_eq!(
                case.1, days_in_month,
                "month_days test failed for case: {case:?}"
            );
        }
    }

    #[test]
    fn test_ordinal_to_month_code() {
        #[derive(Debug)]
        struct TestCase {
            year: i32,
            month: u8,
            day: u8,
            expected_code: &'static str,
        }

        let cases = [
            TestCase {
                year: 2023,
                month: 1,
                day: 9,
                expected_code: "M12",
            },
            TestCase {
                year: 2023,
                month: 2,
                day: 9,
                expected_code: "M01",
            },
            TestCase {
                year: 2023,
                month: 3,
                day: 9,
                expected_code: "M02",
            },
            TestCase {
                year: 2023,
                month: 4,
                day: 9,
                expected_code: "M02L",
            },
            TestCase {
                year: 2023,
                month: 5,
                day: 9,
                expected_code: "M03",
            },
            TestCase {
                year: 2023,
                month: 6,
                day: 9,
                expected_code: "M04",
            },
            TestCase {
                year: 2023,
                month: 7,
                day: 9,
                expected_code: "M05",
            },
            TestCase {
                year: 2023,
                month: 8,
                day: 9,
                expected_code: "M06",
            },
            TestCase {
                year: 2023,
                month: 9,
                day: 9,
                expected_code: "M07",
            },
            TestCase {
                year: 2023,
                month: 10,
                day: 9,
                expected_code: "M08",
            },
            TestCase {
                year: 2023,
                month: 11,
                day: 9,
                expected_code: "M09",
            },
            TestCase {
                year: 2023,
                month: 12,
                day: 9,
                expected_code: "M10",
            },
            TestCase {
                year: 2024,
                month: 1,
                day: 9,
                expected_code: "M11",
            },
            TestCase {
                year: 2024,
                month: 2,
                day: 9,
                expected_code: "M12",
            },
            TestCase {
                year: 2024,
                month: 2,
                day: 10,
                expected_code: "M01",
            },
        ];

        for case in cases {
            let iso = Date::try_new_iso(case.year, case.month, case.day).unwrap();
            let chinese = iso.to_calendar(Chinese);
            let result_code = chinese.month().standard_code.0;
            let expected_code = case.expected_code.to_string();
            assert_eq!(
                expected_code, result_code,
                "Month codes did not match for test case: {case:?}"
            );
        }
    }

    #[test]
    fn test_month_code_to_ordinal() {
        // construct using ::default() to force recomputation
        let year = Chinese.load_or_compute_info(2023);
        let codes = [
            (1, tinystr!(4, "M01")),
            (2, tinystr!(4, "M02")),
            (3, tinystr!(4, "M02L")),
            (4, tinystr!(4, "M03")),
            (5, tinystr!(4, "M04")),
            (6, tinystr!(4, "M05")),
            (7, tinystr!(4, "M06")),
            (8, tinystr!(4, "M07")),
            (9, tinystr!(4, "M08")),
            (10, tinystr!(4, "M09")),
            (11, tinystr!(4, "M10")),
            (12, tinystr!(4, "M11")),
            (13, tinystr!(4, "M12")),
        ];
        for ordinal_code_pair in codes {
            let code = MonthCode(ordinal_code_pair.1);
            let ordinal = year.parse_month_code(code);
            assert_eq!(
                ordinal,
                Some(ordinal_code_pair.0),
                "Code to ordinal failed for year: {}, code: {code}",
                year.related_iso
            );
        }
    }

    #[test]
    fn check_invalid_month_code_to_ordinal() {
        let non_leap_year = 4659;
        let leap_year = 4660;
        let invalid_codes = [
            (non_leap_year, tinystr!(4, "M2")),
            (leap_year, tinystr!(4, "M0")),
            (non_leap_year, tinystr!(4, "J01")),
            (leap_year, tinystr!(4, "3M")),
            (non_leap_year, tinystr!(4, "M04L")),
            (leap_year, tinystr!(4, "M04L")),
            (non_leap_year, tinystr!(4, "M13")),
            (leap_year, tinystr!(4, "M13")),
        ];
        for (year, code) in invalid_codes {
            // construct using ::default() to force recomputation
            let year = Chinese.load_or_compute_info(year);
            let code = MonthCode(code);
            let ordinal = year.parse_month_code(code);
            assert_eq!(
                ordinal, None,
                "Invalid month code failed for year: {}, code: {code}",
                year.related_iso
            );
        }
    }

    #[test]
    fn test_iso_chinese_roundtrip() {
        for i in -1000..=1000 {
            let year = i;
            let month = i as u8 % 12 + 1;
            let day = i as u8 % 28 + 1;
            let iso = Date::try_new_iso(year, month, day).unwrap();
            let chinese = iso.to_calendar(Chinese);
            let result = chinese.to_calendar(Iso);
            assert_eq!(iso, result, "ISO to Chinese roundtrip failed!\nIso: {iso:?}\nChinese: {chinese:?}\nResult: {result:?}");
        }
    }

    #[test]
    fn test_consistent_with_icu() {
        #[derive(Debug)]
        struct TestCase {
            iso_year: i32,
            iso_month: u8,
            iso_day: u8,
            expected_rel_iso: i32,
            expected_cyclic: u8,
            expected_month: u8,
            expected_day: u8,
        }

        let cases = [
            TestCase {
                iso_year: -2332,
                iso_month: 3,
                iso_day: 1,
                expected_rel_iso: -2332,
                expected_cyclic: 5,
                expected_month: 1,
                expected_day: 16,
            },
            TestCase {
                iso_year: -2332,
                iso_month: 2,
                iso_day: 15,
                expected_rel_iso: -2332,
                expected_cyclic: 5,
                expected_month: 1,
                expected_day: 1,
            },
            TestCase {
                // This test case fails to match ICU
                iso_year: -2332,
                iso_month: 2,
                iso_day: 14,
                expected_rel_iso: -2333,
                expected_cyclic: 4,
                expected_month: 13,
                expected_day: 30,
            },
            TestCase {
                // This test case fails to match ICU
                iso_year: -2332,
                iso_month: 1,
                iso_day: 17,
                expected_rel_iso: -2333,
                expected_cyclic: 4,
                expected_month: 13,
                expected_day: 2,
            },
            TestCase {
                // This test case fails to match ICU
                iso_year: -2332,
                iso_month: 1,
                iso_day: 16,
                expected_rel_iso: -2333,
                expected_cyclic: 4,
                expected_month: 13,
                expected_day: 1,
            },
            TestCase {
                iso_year: -2332,
                iso_month: 1,
                iso_day: 15,
                expected_rel_iso: -2333,
                expected_cyclic: 4,
                expected_month: 12,
                expected_day: 29,
            },
            TestCase {
                iso_year: -2332,
                iso_month: 1,
                iso_day: 1,
                expected_rel_iso: -2333,
                expected_cyclic: 4,
                expected_month: 12,
                expected_day: 15,
            },
            TestCase {
                iso_year: -2333,
                iso_month: 1,
                iso_day: 16,
                expected_rel_iso: -2334,
                expected_cyclic: 3,
                expected_month: 12,
                expected_day: 19,
            },
        ];

        for case in cases {
            let iso = Date::try_new_iso(case.iso_year, case.iso_month, case.iso_day).unwrap();
            let chinese = iso.to_calendar(Chinese);
            let chinese_rel_iso = chinese.cyclic_year().related_iso;
            let chinese_cyclic = chinese.cyclic_year().year;
            let chinese_month = chinese.month().ordinal;
            let chinese_day = chinese.day_of_month().0;

            assert_eq!(
                chinese_rel_iso, case.expected_rel_iso,
                "Related ISO failed for test case: {case:?}"
            );
            assert_eq!(
                chinese_cyclic, case.expected_cyclic,
                "Cyclic year failed for test case: {case:?}"
            );
            assert_eq!(
                chinese_month, case.expected_month,
                "Month failed for test case: {case:?}"
            );
            assert_eq!(
                chinese_day, case.expected_day,
                "Day failed for test case: {case:?}"
            );
        }
    }
}
