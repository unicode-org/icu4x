// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains types and implementations for the Chinese calendar.
//!
//! ```rust
//! use icu::calendar::{Date, DateTime};
//! use tinystr::tinystr;
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
use crate::astronomy::{Astronomical, Location, MEAN_TROPICAL_YEAR};
use crate::calendar_arithmetic::{ArithmeticDate, CalendarArithmetic};
use crate::helpers::{adjusted_rem_euclid, div_rem_euclid, i64_to_i32, quotient, I32Result};
use crate::iso::{Iso, IsoDateInner};
use crate::rata_die::RataDie;
use crate::types::{Era, FormattableYear, Moment, MonthCode};
use crate::{
    astronomy, types, Calendar, CalendarError, Date, DateDuration, DateDurationUnit, DateTime,
};
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
/// * _The Mathematics of the Chinese Calendar_ by Helmer Aslaksen (https://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.139.9311&rep=rep1&type=pdf)
/// * Wikipedia: https://en.wikipedia.org/wiki/Chinese_calendar
///
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub struct Chinese;

/// The inner date type used for representing [`Date`]s of [`Chinese`]. See [`Date`] and [`Chinese`] for more details.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct ChineseDateInner(ArithmeticDate<Chinese>);

impl CalendarArithmetic for Chinese {
    /// Returns the number of days in the given (year, month). In the Chinese calendar, months start at each
    /// new moon, so this function finds the number of days between the new moon at the beginning of the given
    /// month and the new moon at the beginning of the next month.
    fn month_days(year: i32, month: u8) -> u8 {
        let mid_year = Self::fixed_mid_year_from_year(year);
        let new_year = Chinese::chinese_new_year_on_or_before_fixed_date(mid_year);
        let approx = new_year + ((month - 1) as i64 * 29);
        let prev_new_moon = Chinese::chinese_new_moon_before((approx + 15).as_moment());
        let next_new_moon = Chinese::chinese_new_moon_on_or_after((approx + 15).as_moment());
        let result = (next_new_moon - prev_new_moon) as u8;
        debug_assert!(result == 29 || result == 30);
        result
    }

    /// Returns the number of months in a given year, which is 13 in a leap year, and 12 in a common year.
    fn months_for_every_year(year: i32) -> u8 {
        if Self::is_leap_year(year) {
            13
        } else {
            12
        }
    }

    /// Returns true if the given year is a leap year, and false if not.
    fn is_leap_year(year: i32) -> bool {
        let mid_year = Self::fixed_mid_year_from_year(year);
        Self::fixed_date_is_in_leap_year(mid_year)
    }

    /// Returns the (month, day) of the last day in a Chinese year (the day before Chinese New Year).
    /// The last month in a year will always be 12 in a common year or 13 in a leap year. The day is
    /// determined by finding the day immediately before the next new year and calculating the number
    /// of days since the last new moon (beginning of the last month in the year).
    fn last_month_day_in_year(year: i32) -> (u8, u8) {
        let mid_year = Chinese::fixed_mid_year_from_year(year);
        let next_new_year = Chinese::chinese_new_year_on_or_before_fixed_date(mid_year + 370);
        let last_day = next_new_year - 1;
        let month = if Chinese::fixed_date_is_in_leap_year(last_day) {
            13
        } else {
            12
        };
        let day = last_day - Chinese::chinese_new_moon_before(last_day.as_moment()) + 1;
        (month, day as u8)
    }
}

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
        let month = if let Some(ordinal) = Self::ordinal_lunar_month_from_code(year, month_code) {
            ordinal
        } else {
            return Err(CalendarError::UnknownMonthCode(
                month_code.0,
                self.debug_name(),
            ));
        };

        if month > Self::months_for_every_year(year) {
            return Err(CalendarError::UnknownMonthCode(
                month_code.0,
                self.debug_name(),
            ));
        }

        let max_day = Self::month_days(year, month);
        if day > max_day {
            return Err(CalendarError::Overflow {
                field: "day",
                max: max_day as usize,
            });
        }

        if era.0 != tinystr!(16, "chinese") {
            return Err(CalendarError::UnknownEra(era.0, self.debug_name()));
        }

        Ok(ArithmeticDate::new_unchecked(year, month, day)).map(ChineseDateInner)
    }

    // Construct the date from an ISO date
    fn date_from_iso(&self, iso: Date<Iso>) -> Self::DateInner {
        let fixed = Iso::fixed_from_iso(iso.inner);
        Chinese::chinese_date_from_fixed(fixed).inner
    }

    // Obtain an ISO date from a Chinese date
    fn date_to_iso(&self, date: &Self::DateInner) -> Date<Iso> {
        let fixed = Chinese::fixed_from_chinese_date_inner(*date);
        Iso::iso_from_fixed(fixed)
    }

    //Count the number of months in a given year, specified by providing a date
    // from that year
    fn days_in_year(&self, date: &Self::DateInner) -> u32 {
        Self::days_in_provided_year(date.0.year)
    }

    fn days_in_month(&self, date: &Self::DateInner) -> u8 {
        Self::month_days(date.0.year, date.0.month)
    }

    #[doc(hidden)] // unstable
    fn offset_date(&self, date: &mut Self::DateInner, offset: DateDuration<Self>) {
        date.0.offset_date(offset)
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
        date1.0.until(date2.0, _largest_unit, _smallest_unit)
    }

    /// Obtain a name for the calendar for debug printing
    fn debug_name(&self) -> &'static str {
        "Chinese"
    }

    /// The calendar-specific year represented by `date`
    fn year(&self, date: &Self::DateInner) -> types::FormattableYear {
        Self::format_chinese_year(date.0.year)
    }

    /// The calendar-specific month code represented by `date`;
    /// since the Chinese calendar has leap months, an "L" is appended to the month code for
    /// leap months. For example, in a year where an intercalary month is added after the second
    /// month, the month codes for ordinal monts 1, 2, 3, 4, 5 would be "M01", "M02", "M02L", "M03", "M04".
    fn month(&self, date: &Self::DateInner) -> types::FormattableMonth {
        let ordinal = date.0.month;
        let leap_month = if Self::is_leap_year(date.0.year) {
            Self::get_leap_month_in_year(Self::fixed_mid_year_from_year(date.0.year))
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
        types::DayOfMonth(date.0.day as u32)
    }

    /// Information of the day of the year
    fn day_of_year_info(&self, date: &Self::DateInner) -> types::DayOfYearInfo {
        let prev_year = date.0.year.saturating_sub(1);
        let next_year = date.0.year.saturating_add(1);
        types::DayOfYearInfo {
            day_of_year: date.0.day_of_year(),
            days_in_year: date.0.days_in_year(),
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
        Self::months_for_every_year(date.0.year)
    }
}

impl Date<Chinese> {
    /// Construct a new Chinese date from a `year`, `month`, `leap_month`, and `day`.
    /// `year` represents the Chinese year counted infinitely with -2636 (2637 BCE) as year Chinese year 1;
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
        ArithmeticDate::new_from_lunar_ordinals(year, month, day)
            .map(ChineseDateInner)
            .map(|inner| Date::from_raw(inner, Chinese))
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

impl Chinese {
    // TODO: A lot of the functions used here require converting between Moment and RataDie frequently.
    // This can quickly become tedious and annoying, so once the code works, we should consider making
    // some of these functions take generic types and then implementing them for Moment and RataDie,
    // or otherwise figure out the optimal way to rewrite these functions to improve consistency and
    // decrease the amount of times as_rata_die() and as_moment() need to be called.

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
    pub fn major_solar_term_from_iso(iso: IsoDateInner) -> i32 {
        let fixed: RataDie = Iso::fixed_from_iso(iso);
        Self::major_solar_term_from_fixed(fixed)
    }

    /// Get the current major solar term of a fixed date, output as an integer from 1..=12.
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Lisp reference code: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L5273-L5281
    pub(crate) fn major_solar_term_from_fixed(date: RataDie) -> i32 {
        let moment: Moment = date.as_moment();
        let offset = Self::chinese_offset(date);
        let universal: Moment = Location::universal_from_standard(moment, offset);
        let solar_longitude = i64_to_i32(Astronomical::solar_longitude(universal) as i64);
        debug_assert!(
            matches!(solar_longitude, I32Result::WithinRange(_)),
            "Solar longitude should be in range of i32"
        );
        let s = solar_longitude.saturate();
        adjusted_rem_euclid(2 + quotient(s, 30), 12)
    }

    /// Returns true if the month of a given fixed date does not have a major solar term,
    /// false otherwise.
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Lisp reference code: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L5345-L5351
    fn chinese_no_major_solar_term(date: RataDie) -> bool {
        Self::major_solar_term_from_fixed(date)
            == Self::major_solar_term_from_fixed(Self::chinese_new_moon_on_or_after(
                (date + 1).as_moment(),
            ))
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
    pub fn minor_solar_term_from_iso(iso: IsoDateInner) -> i32 {
        let fixed: RataDie = Iso::fixed_from_iso(iso);
        Self::minor_solar_term_from_fixed(fixed)
    }

    /// Get the current minor solar term of a fixed date, output as an integer from 1..=12.
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Lisp reference code: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L5273-L5281
    pub(crate) fn minor_solar_term_from_fixed(date: RataDie) -> i32 {
        let moment: Moment = date.as_moment();
        let offset = Self::chinese_offset(date);
        let universal: Moment = Location::universal_from_standard(moment, offset);
        let solar_longitude = i64_to_i32(Astronomical::solar_longitude(universal) as i64);
        debug_assert!(
            matches!(solar_longitude, I32Result::WithinRange(_)),
            "Solar longitude should be in range of i32"
        );
        let s = solar_longitude.saturate();
        adjusted_rem_euclid(3 + quotient(s - 15, 30), 12)
    }

    /// Returns the UTC time offset in China based on the date. The Chinese calendar determines the month based on
    /// observations from China, so this function is necessary to discern the time offset in China, which changed in
    /// 1929 when China adopted a standard time zone set at 120 degrees longitude.
    fn chinese_offset(date: RataDie) -> Location {
        let year = Iso::iso_from_fixed(date).year().number;
        if year < 1929 {
            CHINESE_LOCATION_PRE_1929
        } else {
            CHINESE_LOCATION_POST_1929
        }
    }

    /// The fixed date in Chinese standard time of the next new moon on or after a given Moment.
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Lisp reference code: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L5329-L5338
    fn chinese_new_moon_on_or_after(moment: Moment) -> RataDie {
        let new_moon_moment = Astronomical::new_moon_at_or_after(Self::midnight_in_china(moment));
        let chinese_offset = Self::chinese_offset(new_moon_moment.as_rata_die());
        Location::standard_from_universal(new_moon_moment, chinese_offset).as_rata_die()
    }

    /// The fixed date in Chinese standard time of the previous new moon before a given Moment.
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Lisp reference code: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L5318-L5327
    fn chinese_new_moon_before(moment: Moment) -> RataDie {
        let new_moon_moment = Astronomical::new_moon_before(Self::midnight_in_china(moment));
        let chinese_offset = Self::chinese_offset(new_moon_moment.as_rata_die());
        Location::standard_from_universal(new_moon_moment, chinese_offset).as_rata_die()
    }

    /// Universal time of midnight at start of a Moment in China
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Lisp reference code: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L5353-L5357
    fn midnight_in_china(date: Moment) -> Moment {
        Location::universal_from_standard(date, Self::chinese_offset(date.as_rata_die()))
    }

    /// Determines the fixed date of the Chinese new year in the sui4 (solar year based on the winter solstice)
    /// which contains the fixed date passed as an argument.
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Lisp reference code: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L5370-L5394
    fn chinese_new_year_in_sui(date: RataDie) -> RataDie {
        let prior_solstice = Self::chinese_winter_solstice_on_or_before(date); // s1
        let following_solstice = Self::chinese_winter_solstice_on_or_before(prior_solstice + 370); // s2
        let month_after_eleventh =
            Self::chinese_new_moon_on_or_after((prior_solstice + 1).as_moment()); // m12
        let month_after_twelfth =
            Self::chinese_new_moon_on_or_after((month_after_eleventh + 1).as_moment()); // m13
        let next_eleventh_month =
            Self::chinese_new_moon_before((following_solstice + 1).as_moment()); // next-m11
        let m12_float = month_after_eleventh.as_moment().inner();
        let next_m11_float = next_eleventh_month.as_moment().inner();
        let lhs_argument =
            libm::round((next_m11_float - m12_float) / astronomy::MEAN_SYNODIC_MONTH) as i64;
        if lhs_argument == 12
            && (Self::chinese_no_major_solar_term(month_after_eleventh)
                || Self::chinese_no_major_solar_term(month_after_twelfth))
        {
            Self::chinese_new_moon_on_or_after((month_after_twelfth + 1).as_moment())
        } else {
            month_after_twelfth
        }
    }

    /// Get the moment of the nearest winter solstice on or before a given fixed date
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Lisp reference code: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L5359-L5368
    fn chinese_winter_solstice_on_or_before(date: RataDie) -> RataDie {
        let approx = Astronomical::estimate_prior_solar_longitude(
            270.0,
            Self::midnight_in_china((date + 1).as_moment()),
        );
        let mut iters = 0;
        let max_iters = 367;
        let mut day = Moment::new(libm::floor(approx.inner() - 1.0));
        while iters < max_iters
            && 270.0 >= Astronomical::solar_longitude(Self::midnight_in_china(day + 1.0))
        {
            iters += 1;
            day += 1.0;
        }
        debug_assert!(
            iters < max_iters,
            "Number of iterations was higher than expected"
        );
        day.as_rata_die()
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
        let result_fixed = Self::chinese_new_year_on_or_before_fixed_date(fixed);
        Iso::iso_from_fixed(result_fixed)
    }

    /// Get the fixed date of the nearest Chinese New Year on or before a given fixed date.
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Lisp reference code: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L5396-L5405
    pub(crate) fn chinese_new_year_on_or_before_fixed_date(date: RataDie) -> RataDie {
        let new_year = Self::chinese_new_year_in_sui(date);
        if date >= new_year {
            new_year
        } else {
            Self::chinese_new_year_in_sui(date - 180)
        }
    }

    /// Get a Date<Chinese> from a fixed date
    ///
    /// Months are calculated by iterating through the dates of new moons until finding the last month which
    /// does not exceed the given fixed date. The day of month is calculated by subtracting the fixed date
    /// from the fixed date of the beginning of the month.
    ///
    /// The calculation for `elapsed_years` in this function is based on code from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Lisp reference code: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L5414-L5459
    #[allow(clippy::unwrap_used)]
    pub(crate) fn chinese_date_from_fixed(date: RataDie) -> Date<Chinese> {
        let new_year = Self::chinese_new_year_on_or_before_fixed_date(date);
        let elapsed_years = libm::floor(
            1.5 - 1.0 / 12.0 + ((new_year - CHINESE_EPOCH) as f64) / MEAN_TROPICAL_YEAR,
        );
        let elapsed_years_int = i64_to_i32(elapsed_years as i64);
        debug_assert!(
            matches!(elapsed_years_int, I32Result::WithinRange(_)),
            "Chinese year should be in range of i32"
        );
        let year = elapsed_years_int.saturate();
        let mut month = 1;
        let max_months = 14;
        let mut cur_month = new_year;
        let mut next_month = Self::chinese_new_moon_on_or_after((cur_month + 1).as_moment());
        while next_month <= date && month < max_months {
            month += 1;
            cur_month = next_month;
            next_month = Self::chinese_new_moon_on_or_after((cur_month + 1).as_moment());
        }
        debug_assert!(month < max_months, "Unexpectedly large number of months");
        let day = (date - cur_month + 1) as u8;
        Date::try_new_chinese_date(year, month, day).unwrap()
    }

    /// Get a RataDie from a ChineseDateInner
    ///
    /// This finds the RataDie of the new year of the year given, then finds the RataDie of the new moon
    /// (beginning of month) of the month given, then adds the necessary number of days.
    pub(crate) fn fixed_from_chinese_date_inner(date: ChineseDateInner) -> RataDie {
        let year = date.0.year;
        let month = date.0.month as i64;
        let day = date.0.day as i64;
        let mid_year = Self::fixed_mid_year_from_year(year);
        let new_year = Self::chinese_new_year_on_or_before_fixed_date(mid_year);
        let month_approx = new_year + (month - 1) * 29;
        let prior_new_moon = Self::chinese_new_moon_on_or_after(month_approx.as_moment());
        prior_new_moon + day - 1
    }

    /// Get the a RataDie in the middle of a Chinese year; this is not necessarily meant for direct use
    /// in calculations, rather it is useful for getting a RataDie guaranteed to be in a given Chinese
    /// year as input for other functions like [`get_leap_month_in_year`].
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz
    /// Lisp reference code: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L5469-L5475
    fn fixed_mid_year_from_year(elapsed_years: i32) -> RataDie {
        let cycle = quotient(elapsed_years - 1, 60) + 1;
        let year = adjusted_rem_euclid(elapsed_years, 60);
        CHINESE_EPOCH + ((((cycle - 1) * 60 + year - 1) as f64 + 0.5) * MEAN_TROPICAL_YEAR) as i64
    }

    /// Returns true if the fixed date given is in a leap year, false otherwise
    fn fixed_date_is_in_leap_year(date: RataDie) -> bool {
        let prev_new_year = Self::chinese_new_year_on_or_before_fixed_date(date);
        let next_new_year = Self::chinese_new_year_on_or_before_fixed_date(prev_new_year + 400);
        let difference = next_new_year - prev_new_year;
        difference > 365
    }

    /// Given that a date is in a leap year, find which month in the year is a leap month.
    /// Since the first month in which there are no major solar terms is a leap month,
    /// this function cycles through months until it finds the leap month, then returns
    /// the number of that month. This function assumes the date passed in is in a leap year
    /// and tests to ensure this is the case by asserting that no more than twelve months are
    /// analyzed.
    ///
    /// Conceptually similar to code from _Calendrical Calculations_ by Reingold & Dershowitz
    /// Lisp reference code: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L5443-L5450
    fn get_leap_month_in_year(date: RataDie) -> u8 {
        let mut cur = Chinese::chinese_new_year_on_or_before_fixed_date(date);
        let mut result = 1;
        let max_iters = 13;
        while result < max_iters && !Self::chinese_no_major_solar_term(cur) {
            cur = Chinese::chinese_new_moon_on_or_after((cur + 1).as_moment());
            result += 1;
        }
        debug_assert!(result < max_iters, "The given year was not a leap year and an unexpected number of iterations occurred searching for a leap month.");
        result
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
        let mid_year = Self::fixed_mid_year_from_year(number);
        let iso_formattable_year = Iso::iso_from_fixed(mid_year).year();
        let related_iso = Some(iso_formattable_year.number);
        types::FormattableYear {
            era,
            number,
            cyclic,
            related_iso,
        }
    }

    /// Get the ordinal lunar month from a code
    ///
    /// TODO: The behavior of this fn is calendar specific, but this needs to be implemented in every lunar
    /// calendar; consider abstracting this function to a Lunar trait
    fn ordinal_lunar_month_from_code(year: i32, code: MonthCode) -> Option<u8> {
        if code.0.len() < 3 {
            return None;
        }
        let mid_year = Self::fixed_mid_year_from_year(year);
        let leap_month = if Self::is_leap_year(year) {
            Self::get_leap_month_in_year(mid_year)
        } else {
            // 14 is a sentinel value, greater than all other months, for the purpose of computation only;
            // it is impossible to actually have 14 months in a year.
            14
        };
        let bytes = code.0.all_bytes();
        if bytes[0] != b'M' {
            return None;
        }
        if code.0.len() == 4 && bytes[3] != b'L' {
            return None;
        }
        let mut unadjusted = 0;
        if bytes[1] == b'0' {
            if bytes[2] >= b'1' && bytes[2] <= b'9' {
                unadjusted = bytes[2] - b'0';
            }
        } else if bytes[1] == b'1' && bytes[2] >= b'0' && bytes[2] <= b'2' {
            unadjusted = 10 + bytes[2] - b'0';
        }
        if bytes[3] == b'L' {
            if unadjusted + 1 != leap_month {
                return None;
            } else {
                return Some(unadjusted + 1);
            }
        }
        if unadjusted != 0 {
            if unadjusted + 1 > leap_month {
                return Some(unadjusted + 1);
            } else {
                return Some(unadjusted);
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_chinese_new_moon_directionality() {
        for i in (-1000..1000).step_by(31) {
            let moment = Moment::new(i as f64);
            let before = Chinese::chinese_new_moon_before(moment);
            let after = Chinese::chinese_new_moon_on_or_after(moment);
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
            expected_month: u32,
            expected_day: u32,
        }

        let cases = [
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
            let chinese = Chinese::chinese_date_from_fixed(RataDie::new(case.fixed));
            assert_eq!(
                case.expected_year,
                chinese.year().number,
                "Chinese from fixed failed for case: {case:?}"
            );
            assert_eq!(
                case.expected_month,
                chinese.month().ordinal,
                "Chinese from fixed failed for case: {case:?}"
            );
            assert_eq!(
                case.expected_day,
                chinese.day_of_month().0,
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
            let fixed = Chinese::fixed_from_chinese_date_inner(date.inner).to_i64_date();
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
            let chinese = Chinese::chinese_date_from_fixed(rata_die);
            let result = Chinese::fixed_from_chinese_date_inner(chinese.inner);
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
            let rata_die = Iso::fixed_from_iso(*iso.inner());
            assert!(Chinese::is_leap_year(chinese_year));
            assert_eq!(expected_month, Chinese::get_leap_month_in_year(rata_die));
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
            let ordinal = Chinese::ordinal_lunar_month_from_code(year, code);
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
            let code = MonthCode(year_code_pair.1);
            let ordinal = Chinese::ordinal_lunar_month_from_code(year, code);
            assert_eq!(
                ordinal, None,
                "Invalid month code failed for year: {year}, code: {code}"
            );
        }
    }
}
