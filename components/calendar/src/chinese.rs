// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains types and implementations for the Chinese calendar.
//!
//! ```rust
//! use icu::calendar::{Date, DateTime};
//!
//! // `Date` type
//! let chinese_date = Date::try_new_chinese_date(4660, 6, 6)
//!     .expect("Failed to initialize Chinese Date instance.");
//!
//! // `DateTime` type
//! let chinese_datetime = DateTime::try_new_chinese_datetime(4660, 6, 6, 13, 1, 0)
//!     .expect("Failed to initialize Chinese DateTime instance");
//!
//! // `Date` checks
//! assert_eq!(chinese_date.year().number, 4660);
//! assert_eq!(chinese_date.year().related_iso, Some(2023));
//! assert_eq!(chinese_date.year().cyclic, Some(40));
//! assert_eq!(chinese_date.month().ordinal, 6);
//! assert_eq!(chinese_date.day_of_month().0, 6);
//!
//! // `DateTime` checks
//! assert_eq!(chinese_datetime.date.year().number, 4660);
//! assert_eq!(chinese_datetime.date.year().related_iso, Some(2023));
//! assert_eq!(chinese_datetime.date.year().cyclic, Some(40));
//! assert_eq!(chinese_datetime.date.month().ordinal, 6);
//! assert_eq!(chinese_datetime.date.day_of_month().0, 6);
//! assert_eq!(chinese_datetime.time.hour.number(), 13);
//! assert_eq!(chinese_datetime.time.minute.number(), 1);
//! assert_eq!(chinese_datetime.time.second.number(), 0);
//! ```

use crate::any_calendar::AnyCalendarKind;
use crate::astronomy::Location;
use crate::calendar_arithmetic::CalendarArithmetic;
use crate::chinese_based::{
    chinese_based_ordinal_lunar_month_from_code, ChineseBased, ChineseBasedDateInner,
};
use crate::helpers::div_rem_euclid;
use crate::iso::{Iso, IsoDateInner};
use crate::rata_die::RataDie;
use crate::types::{Era, FormattableYear};
use crate::{types, Calendar, CalendarError, Date, DateDuration, DateDurationUnit, DateTime};
use tinystr::tinystr;

// The equivalent first day in the Chinese calendar (based on inception of the calendar)
const CHINESE_EPOCH: RataDie = RataDie::new(-963099); // Feb. 15, 2637 BCE (-2636)

/// The Chinese calendar relies on knowing the current day at the moment of a new moon;
/// however, this can vary depending on location. As such, new moon calculations are based
/// on the time in Beijing. Before 1929, local time was used, represented as UTC+(1397/180 h).
/// In 1929, China adopted a standard time zone based on 120 degrees of longitude, meaning
/// from 1929 onward, all new moon calculations are based on UTC+8h.
///
/// Offsets are not given in hours, but in partial days (1 hour = 1 / 24 day)
const UTC_OFFSET_PRE_1929: f64 = (1397.0 / 180.0) / 24.0;
const UTC_OFFSET_POST_1929: f64 = 8.0 / 24.0;

const CHINESE_LOCATION_PRE_1929: Location =
    Location::new_unchecked(39.0, 116.0, 43.5, UTC_OFFSET_PRE_1929);
const CHINESE_LOCATION_POST_1929: Location =
    Location::new_unchecked(39.0, 116.0, 43.5, UTC_OFFSET_POST_1929);

/// The Chinese Calendar
///
/// The [Chinese Calendar] is a lunisolar calendar used traditionally in China as well as in other
/// countries particularly in, but not limited to, East Asia. It is often used today to track important
/// cultural events and holidays like the Chinese Lunar New Year.
///
/// This type can be used with [`Date`] or [`DateTime`] to represent dates in the Chinese calendar.
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
/// conversions, this module counts Chinese years in an infinite system similar to ISO, with year 1 in the
/// calendar corresponding to the inception of the calendar, marked as 2637 BCE (ISO: -2636), and negative
/// years marking Chinese years before February 15, 2637 BCE.
///
/// Because the Chinese calendar does not traditionally count years, era codes are not used in this calendar;
/// this crate supports a single era code "chinese".
///
/// This Chinese calendar implementation also supports a related ISO year, which marks the ISO year in which a
/// Chinese year begins, and a cyclic year corresponding to the year in the 60 year cycle as described above.
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
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(clippy::exhaustive_structs)] // this type is stable
pub struct Chinese;

/// The inner date type used for representing [`Date`]s of [`Chinese`]. See [`Date`] and [`Chinese`] for more details.
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct ChineseDateInner(ChineseBasedDateInner<Chinese>);

type Inner = ChineseBasedDateInner<Chinese>;

impl Calendar for Chinese {
    type DateInner = ChineseDateInner;

    // Construct a date from era/month codes and fields
    fn date_from_codes(
        &self,
        era: types::Era,
        year: i32,
        month_code: types::MonthCode,
        day: u8,
    ) -> Result<Self::DateInner, CalendarError> {
        let cache = Inner::compute_cache(year);

        let month = if let Some(ordinal) =
            chinese_based_ordinal_lunar_month_from_code::<Chinese>(month_code, cache)
        {
            ordinal
        } else {
            return Err(CalendarError::UnknownMonthCode(
                month_code.0,
                self.debug_name(),
            ));
        };

        if era.0 != tinystr!(16, "chinese") {
            return Err(CalendarError::UnknownEra(era.0, self.debug_name()));
        }

        let arithmetic = Inner::new_from_ordinals(year, month, day, &cache);
        Ok(ChineseDateInner(ChineseBasedDateInner(arithmetic?, cache)))
    }

    // Construct the date from an ISO date
    fn date_from_iso(&self, iso: Date<Iso>) -> Self::DateInner {
        let fixed = Iso::fixed_from_iso(iso.inner);
        ChineseDateInner(Inner::chinese_based_date_from_fixed(fixed))
    }

    // Obtain an ISO date from a Chinese date
    fn date_to_iso(&self, date: &Self::DateInner) -> Date<Iso> {
        let fixed = Inner::fixed_from_chinese_based_date_inner(date.0);
        Iso::iso_from_fixed(fixed)
    }

    //Count the number of months in a given year, specified by providing a date
    // from that year
    fn days_in_year(&self, date: &Self::DateInner) -> u16 {
        date.0.days_in_year_inner()
    }

    fn days_in_month(&self, date: &Self::DateInner) -> u8 {
        date.0.days_in_month_inner()
    }

    #[doc(hidden)] // unstable
    fn offset_date(&self, date: &mut Self::DateInner, offset: DateDuration<Self>) {
        let year = date.0 .0.year;
        date.0 .0.offset_date(offset);
        if date.0 .0.year != year {
            date.0 .1 = Inner::compute_cache(date.0 .0.year);
        }
    }

    #[doc(hidden)] // unstable
    #[allow(clippy::field_reassign_with_default)]
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
        date1.0 .0.until(date2.0 .0, _largest_unit, _smallest_unit)
    }

    /// Obtain a name for the calendar for debug printing
    fn debug_name(&self) -> &'static str {
        "Chinese"
    }

    /// The calendar-specific year represented by `date`
    fn year(&self, date: &Self::DateInner) -> types::FormattableYear {
        Self::format_chinese_year(date.0 .0.year)
    }

    /// The calendar-specific month code represented by `date`;
    /// since the Chinese calendar has leap months, an "L" is appended to the month code for
    /// leap months. For example, in a year where an intercalary month is added after the second
    /// month, the month codes for ordinal monts 1, 2, 3, 4, 5 would be "M01", "M02", "M02L", "M03", "M04".
    fn month(&self, date: &Self::DateInner) -> types::FormattableMonth {
        let ordinal = date.0 .0.month;
        let leap_month_option = date.0 .1.leap_month;
        let leap_month = if let Some(leap) = leap_month_option {
            leap.get()
        } else {
            14
        };
        let code_inner = if leap_month == ordinal {
            // Month cannot be 1 because a year cannot have a leap month before the first actual month,
            // and the maximum num of months ina leap year is 13.
            debug_assert!((2..=13).contains(&ordinal));
            match ordinal {
                2 => tinystr!(4, "M01L"),
                3 => tinystr!(4, "M02L"),
                4 => tinystr!(4, "M03L"),
                5 => tinystr!(4, "M04L"),
                6 => tinystr!(4, "M05L"),
                7 => tinystr!(4, "M06L"),
                8 => tinystr!(4, "M07L"),
                9 => tinystr!(4, "M08L"),
                10 => tinystr!(4, "M09L"),
                11 => tinystr!(4, "M10L"),
                12 => tinystr!(4, "M11L"),
                13 => tinystr!(4, "M12L"),
                _ => tinystr!(4, "und"),
            }
        } else {
            let mut adjusted_ordinal = ordinal;
            if ordinal > leap_month {
                // Before adjusting for leap month, if ordinal > leap_month,
                // the month cannot be 1 because this implies the leap month is < 1, which is impossible;
                // cannot be 2 because that implies the leap month is = 1, which is impossible,
                // and cannot be more than 13 because max number of months in a year is 13.
                debug_assert!((2..=13).contains(&ordinal));
                adjusted_ordinal -= 1;
            }
            debug_assert!((1..=12).contains(&adjusted_ordinal));
            match adjusted_ordinal {
                1 => tinystr!(4, "M01"),
                2 => tinystr!(4, "M02"),
                3 => tinystr!(4, "M03"),
                4 => tinystr!(4, "M04"),
                5 => tinystr!(4, "M05"),
                6 => tinystr!(4, "M06"),
                7 => tinystr!(4, "M07"),
                8 => tinystr!(4, "M08"),
                9 => tinystr!(4, "M09"),
                10 => tinystr!(4, "M10"),
                11 => tinystr!(4, "M11"),
                12 => tinystr!(4, "M12"),
                _ => tinystr!(4, "und"),
            }
        };
        let code = types::MonthCode(code_inner);
        types::FormattableMonth {
            ordinal: ordinal as u32,
            code,
        }
    }

    /// The calendar-specific day-of-month represented by `date`
    fn day_of_month(&self, date: &Self::DateInner) -> types::DayOfMonth {
        types::DayOfMonth(date.0 .0.day as u32)
    }

    /// Information of the day of the year
    fn day_of_year_info(&self, date: &Self::DateInner) -> types::DayOfYearInfo {
        let prev_year = date.0 .0.year.saturating_sub(1);
        let next_year = date.0 .0.year.saturating_add(1);
        types::DayOfYearInfo {
            day_of_year: date.0 .0.day_of_year(),
            days_in_year: date.0.days_in_year_inner(),
            prev_year: Self::format_chinese_year(prev_year),
            days_in_prev_year: Self::days_in_provided_year(prev_year),
            next_year: Self::format_chinese_year(next_year),
        }
    }

    /// The [`AnyCalendarKind`] corresponding to this calendar
    fn any_calendar_kind(&self) -> Option<AnyCalendarKind> {
        Some(AnyCalendarKind::Chinese)
    }

    fn months_in_year(&self, date: &Self::DateInner) -> u8 {
        date.0.months_in_year_inner()
    }
}

impl Date<Chinese> {
    /// Construct a new Chinese date from a `year`, `month`, and `day`.
    /// `year` represents the Chinese year counted infinitely with -2636 (2637 BCE) as Chinese year 1;
    /// `month` represents the month of the year ordinally (ex. if it is a leap year, the last month will be 13, not 12);
    /// `day` indicates the day of month
    ///
    /// ```rust
    /// use icu::calendar::Date;
    ///
    /// let date_chinese = Date::try_new_chinese_date(4660, 6, 11)
    ///     .expect("Failed to initialize Chinese Date instance.");
    ///
    /// assert_eq!(date_chinese.year().number, 4660);
    /// assert_eq!(date_chinese.year().cyclic, Some(40));
    /// assert_eq!(date_chinese.year().related_iso, Some(2023));
    /// assert_eq!(date_chinese.month().ordinal, 6);
    /// assert_eq!(date_chinese.day_of_month().0, 11);
    /// ```
    pub fn try_new_chinese_date(
        year: i32,
        month: u8,
        day: u8,
    ) -> Result<Date<Chinese>, CalendarError> {
        let cache = Inner::compute_cache(year);
        let arithmetic = Inner::new_from_ordinals(year, month, day, &cache);
        Ok(Date::from_raw(
            ChineseDateInner(ChineseBasedDateInner(arithmetic?, cache)),
            Chinese,
        ))
    }
}

impl DateTime<Chinese> {
    /// Construct a new Chinese datetime from integers using the
    /// -2636-based year system
    ///
    /// ```rust
    /// use icu::calendar::DateTime;
    ///
    /// let chinese_datetime = DateTime::try_new_chinese_datetime(4660, 6, 11, 13, 1, 0)
    ///     .expect("Failed to initialize Chinese DateTime instance.");
    ///
    /// assert_eq!(chinese_datetime.date.year().number, 4660);
    /// assert_eq!(chinese_datetime.date.year().related_iso, Some(2023));
    /// assert_eq!(chinese_datetime.date.year().cyclic, Some(40));
    /// assert_eq!(chinese_datetime.date.month().ordinal, 6);
    /// assert_eq!(chinese_datetime.date.day_of_month().0, 11);
    /// assert_eq!(chinese_datetime.time.hour.number(), 13);
    /// assert_eq!(chinese_datetime.time.minute.number(), 1);
    /// assert_eq!(chinese_datetime.time.second.number(), 0);
    /// ```
    pub fn try_new_chinese_datetime(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Result<DateTime<Chinese>, CalendarError> {
        Ok(DateTime {
            date: Date::try_new_chinese_date(year, month, day)?,
            time: types::Time::try_new(hour, minute, second, 0)?,
        })
    }
}

impl ChineseBased for Chinese {
    fn location(fixed: RataDie) -> Location {
        let year = Iso::iso_from_fixed(fixed).year().number;
        if year < 1929 {
            CHINESE_LOCATION_PRE_1929
        } else {
            CHINESE_LOCATION_POST_1929
        }
    }

    const EPOCH: RataDie = CHINESE_EPOCH;
}

impl Chinese {
    /// Get the current major solar term of an ISO date
    ///
    /// ```rust
    /// use icu::calendar::Date;
    /// use icu::calendar::chinese::Chinese;
    ///
    /// let iso_date = Date::try_new_iso_date(2023, 6, 28)
    ///     .expect("Failed to initialize ISO Date instance.");
    /// let major_solar_term = Chinese::major_solar_term_from_iso(*iso_date.inner());
    /// assert_eq!(major_solar_term, 5);
    /// ```
    pub fn major_solar_term_from_iso(iso: IsoDateInner) -> u32 {
        let fixed: RataDie = Iso::fixed_from_iso(iso);
        Inner::major_solar_term_from_fixed(fixed)
    }

    /// Get the current major solar term of an ISO date
    ///
    /// ```rust
    /// use icu::calendar::Date;
    /// use icu::calendar::chinese::Chinese;
    ///
    /// let iso_date = Date::try_new_iso_date(2023, 6, 28)
    ///     .expect("Failed to initialize ISO Date instance.");
    /// let minor_solar_term = Chinese::minor_solar_term_from_iso(*iso_date.inner());
    /// assert_eq!(minor_solar_term, 5);
    /// ```
    pub fn minor_solar_term_from_iso(iso: IsoDateInner) -> u32 {
        let fixed: RataDie = Iso::fixed_from_iso(iso);
        Inner::minor_solar_term_from_fixed(fixed)
    }

    /// Get the ISO date of the nearest Chinese New Year on or before a given ISO date
    /// ```rust
    /// use icu::calendar::Date;
    /// use icu::calendar::chinese::Chinese;
    ///
    /// let date = Date::try_new_iso_date(2023, 6, 22).expect("Failed to initialize ISO Date");
    /// let chinese_new_year = Chinese::chinese_new_year_on_or_before_iso(date);
    /// assert_eq!(chinese_new_year.year().number, 2023);
    /// assert_eq!(chinese_new_year.month().ordinal, 1);
    /// assert_eq!(chinese_new_year.day_of_month().0, 22);
    /// ```
    pub fn chinese_new_year_on_or_before_iso(iso: Date<Iso>) -> Date<Iso> {
        let iso_inner = iso.inner;
        let fixed = Iso::fixed_from_iso(iso_inner);
        let result_fixed = Inner::new_year_on_or_before_fixed_date(fixed, None).0;
        Iso::iso_from_fixed(result_fixed)
    }

    /// Get a FormattableYear from an integer Chinese year
    ///
    /// `era` is always `Era(tinystr!(16, "chinese"))`
    /// `number` is the year since the inception of the Chinese calendar (see [`Chinese`])
    /// `cyclic` is an option with the current year in the sexigesimal cycle (see [`Chinese`])
    /// `related_iso` is the ISO year in which the given Chinese year begins (see [`Chinese`])
    fn format_chinese_year(year: i32) -> FormattableYear {
        let era = Era(tinystr!(16, "chinese"));
        let number = year;
        let cyclic = Some(div_rem_euclid(number - 1, 60).1 + 1);
        let mid_year = Inner::fixed_mid_year_from_year(number);
        let iso_formattable_year = Iso::iso_from_fixed(mid_year).year();
        let related_iso = Some(iso_formattable_year.number);
        types::FormattableYear {
            era,
            number,
            cyclic,
            related_iso,
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::types::Moment;
    use crate::types::MonthCode;

    #[test]
    fn test_chinese_new_moon_directionality() {
        for i in (-1000..1000).step_by(31) {
            let moment = Moment::new(i as f64);
            let before = Inner::new_moon_before(moment);
            let after = Inner::new_moon_on_or_after(moment);
            assert!(before < after, "Chinese new moon directionality failed for Moment: {moment:?}, with:\n\tBefore: {before:?}\n\tAfter: {after:?}");
        }
    }

    #[test]
    fn test_chinese_new_year_on_or_before() {
        let date = Date::try_new_iso_date(2023, 6, 22).expect("Failed to initialize ISO Date");
        let chinese_new_year = Chinese::chinese_new_year_on_or_before_iso(date);
        assert_eq!(chinese_new_year.year().number, 2023);
        assert_eq!(chinese_new_year.month().ordinal, 1);
        assert_eq!(chinese_new_year.day_of_month().0, 22);
    }

    #[test]
    fn test_chinese_from_fixed() {
        #[derive(Debug)]
        struct TestCase {
            fixed: i64,
            expected_year: i32,
            expected_month: u8,
            expected_day: u8,
        }

        let cases = [
            TestCase {
                fixed: -964192,
                expected_year: -2,
                expected_month: 1,
                expected_day: 1,
            },
            TestCase {
                fixed: -963838,
                expected_year: -1,
                expected_month: 1,
                expected_day: 1,
            },
            TestCase {
                fixed: -963129,
                expected_year: 0,
                expected_month: 13,
                expected_day: 1,
            },
            TestCase {
                fixed: -963100,
                expected_year: 0,
                expected_month: 13,
                expected_day: 30,
            },
            TestCase {
                fixed: -963099,
                expected_year: 1,
                expected_month: 1,
                expected_day: 1,
            },
            TestCase {
                fixed: 738700,
                expected_year: 4660,
                expected_month: 6,
                expected_day: 12,
            },
            TestCase {
                fixed: 738718,
                expected_year: 4660,
                expected_month: 6,
                expected_day: 30,
            },
            TestCase {
                fixed: 738747,
                expected_year: 4660,
                expected_month: 7,
                expected_day: 29,
            },
            TestCase {
                fixed: 738748,
                expected_year: 4660,
                expected_month: 8,
                expected_day: 1,
            },
            TestCase {
                fixed: 738865,
                expected_year: 4660,
                expected_month: 11,
                expected_day: 29,
            },
            TestCase {
                fixed: 738895,
                expected_year: 4660,
                expected_month: 12,
                expected_day: 29,
            },
            TestCase {
                fixed: 738925,
                expected_year: 4660,
                expected_month: 13,
                expected_day: 30,
            },
        ];

        for case in cases {
            let chinese = Inner::chinese_based_date_from_fixed(RataDie::new(case.fixed));
            assert_eq!(
                case.expected_year, chinese.0.year,
                "Chinese from fixed failed for case: {case:?}"
            );
            assert_eq!(
                case.expected_month, chinese.0.month,
                "Chinese from fixed failed for case: {case:?}"
            );
            assert_eq!(
                case.expected_day, chinese.0.day,
                "Chinese from fixed failed for case: {case:?}"
            );
        }
    }

    #[test]
    fn test_fixed_from_chinese() {
        #[derive(Debug)]
        struct TestCase {
            year: i32,
            month: u8,
            day: u8,
            expected: i64,
        }

        let cases = [
            TestCase {
                year: 4660,
                month: 6,
                day: 6,
                expected: 738694,
            },
            TestCase {
                year: 1,
                month: 1,
                day: 1,
                expected: -963099,
            },
        ];

        for case in cases {
            let date = Date::try_new_chinese_date(case.year, case.month, case.day).unwrap();
            let fixed = Inner::fixed_from_chinese_based_date_inner(date.inner.0).to_i64_date();
            let expected = case.expected;
            assert_eq!(fixed, expected, "Fixed from Chinese failed with expected: {fixed} and calculated: {expected}, for test case: {case:?}");
        }
    }

    #[test]
    fn test_fixed_chinese_roundtrip() {
        let mut fixed = -1963020;
        let max_fixed = 1963020;
        let mut iters = 0;
        let max_iters = 560;
        while fixed < max_fixed && iters < max_iters {
            let rata_die = RataDie::new(fixed);
            let chinese = Inner::chinese_based_date_from_fixed(rata_die);
            let result = Inner::fixed_from_chinese_based_date_inner(chinese);
            let result_debug = result.to_i64_date();
            assert_eq!(result, rata_die, "Failed roundtrip fixed -> Chinese -> fixed for fixed: {fixed}, with calculated: {result_debug} from Chinese date:\n{chinese:?}");
            fixed += 7043;
            iters += 1;
        }
    }

    #[test]
    fn test_chinese_epoch() {
        let iso = Date::try_new_iso_date(-2636, 2, 15).unwrap();
        let chinese = iso.to_calendar(Chinese);
        assert_eq!(chinese.year().number, 1);
        assert_eq!(chinese.month().ordinal, 1);
        assert_eq!(chinese.month().code.0, "M01");
        assert_eq!(chinese.day_of_month().0, 1);
        assert_eq!(chinese.year().cyclic, Some(1));
        assert_eq!(chinese.year().related_iso, Some(-2636));
    }

    #[test]
    fn test_iso_to_chinese_negative_years() {
        #[derive(Debug)]
        struct TestCase {
            iso_year: i32,
            iso_month: u8,
            iso_day: u8,
            expected_year: i32,
            expected_month: u32,
            expected_day: u32,
        }

        let cases = [
            TestCase {
                iso_year: -2636,
                iso_month: 2,
                iso_day: 14,
                expected_year: 0,
                expected_month: 13,
                expected_day: 30,
            },
            TestCase {
                iso_year: -2636,
                iso_month: 1,
                iso_day: 15,
                expected_year: 0,
                expected_month: 12,
                expected_day: 30,
            },
        ];

        for case in cases {
            let iso = Date::try_new_iso_date(case.iso_year, case.iso_month, case.iso_day).unwrap();
            let chinese = iso.to_calendar(Chinese);
            assert_eq!(
                case.expected_year,
                chinese.year().number,
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
            let iso = Date::try_new_iso_date(year, 6, 1).unwrap();
            let chinese_year = iso.to_calendar(Chinese).year().number;
            let cache = Inner::compute_cache(chinese_year);
            assert!(Chinese::is_leap_year(chinese_year));
            assert_eq!(
                expected_month,
                Inner::get_leap_month_from_new_year(cache.new_year)
            );
        }
    }

    #[test]
    fn test_month_days() {
        let year = 4660;
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
            let days_in_month = Chinese::month_days(year, case.0);
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
            let iso = Date::try_new_iso_date(case.year, case.month, case.day).unwrap();
            let chinese = iso.to_calendar(Chinese);
            let result_code = chinese.month().code.0;
            let expected_code = case.expected_code.to_string();
            assert_eq!(
                expected_code, result_code,
                "Month codes did not match for test case: {case:?}"
            );
        }
    }

    #[test]
    fn test_month_code_to_ordinal() {
        let year = 4660;
        let cache = Inner::compute_cache(year);
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
            let ordinal = chinese_based_ordinal_lunar_month_from_code::<Chinese>(code, cache);
            assert_eq!(
                ordinal,
                Some(ordinal_code_pair.0),
                "Code to ordinal failed for year: {year}, code: {code}"
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
        for year_code_pair in invalid_codes {
            let year = year_code_pair.0;
            let cache = Inner::compute_cache(year);
            let code = MonthCode(year_code_pair.1);
            let ordinal = chinese_based_ordinal_lunar_month_from_code::<Chinese>(code, cache);
            assert_eq!(
                ordinal, None,
                "Invalid month code failed for year: {year}, code: {code}"
            );
        }
    }

    #[test]
    fn test_iso_chinese_roundtrip() {
        for i in -1000..=1000 {
            let year = i;
            let month = i as u8 % 12 + 1;
            let day = i as u8 % 28 + 1;
            let iso = Date::try_new_iso_date(year, month, day).unwrap();
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
            expected_cyclic: i32,
            expected_month: u32,
            expected_day: u32,
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
            let iso = Date::try_new_iso_date(case.iso_year, case.iso_month, case.iso_day).unwrap();
            let chinese = iso.to_calendar(Chinese);
            let chinese_rel_iso = chinese.year().related_iso;
            let chinese_cyclic = chinese.year().cyclic;
            let chinese_month = chinese.month().ordinal;
            let chinese_day = chinese.day_of_month().0;

            assert_eq!(
                chinese_rel_iso,
                Some(case.expected_rel_iso),
                "Related ISO failed for test case: {case:?}"
            );
            assert_eq!(
                chinese_cyclic,
                Some(case.expected_cyclic),
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
