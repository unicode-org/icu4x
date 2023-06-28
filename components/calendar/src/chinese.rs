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
use crate::astronomy::{Astronomical, Location, MEAN_SYNODIC_MONTH, MEAN_TROPICAL_YEAR};
use crate::calendar_arithmetic::{ArithmeticDate, CalendarArithmetic};
use crate::helpers::{
    adjusted_rem_euclid, adjusted_rem_euclid_f64, div_rem_euclid, i64_to_i32, quotient, I32Result,
};
use crate::iso::{Iso, IsoDateInner};
use crate::rata_die::RataDie;
use crate::types::{Era, FormattableYear, Moment};
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
/// internally, a single era code "era" is used, but this is never checked, so era code input can be anything.
/// This Chinese calendar implementation does support a related ISO year, which marks the ISO year in which a
/// Chinese year begins, and a cyclic year corresponding to the year in the 60 year cycle as described above.
///
/// For more information, suggested reading materials include:
/// * _Calendrical Calculations_ by Reingold & Dershowitz
/// * _The Mathematics of the Chinese Calendar_ by Helmer Aslaksen (https://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.139.9311&rep=rep1&type=pdf)
/// * Wikipedia: https://en.wikipedia.org/wiki/Chinese_calendar
///
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Chinese;

/// The inner date type used for representing [`Date`]s of [`Chinese`]. See [`Date`] and [`Chinese`] for more details.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct ChineseDateInner(ArithmeticDate<Chinese>);

impl CalendarArithmetic for Chinese {
    fn month_days(year: i32, month: u8) -> u8 {
        let mid_year = Self::fixed_mid_year_from_year(year);
        let new_year = Chinese::chinese_new_year_on_or_before_fixed_date(mid_year);
        let mut cur_month: u8 = 1;
        let mut cur_rata_die = new_year;
        let mut iters: u8 = 0;
        let max_iters: u8 = 13;
        while cur_month < month && iters < max_iters {
            cur_rata_die = Chinese::chinese_new_moon_on_or_after((cur_rata_die + 1).as_moment());
            cur_month += 1;
            iters += 1;
        }
        debug_assert!(iters < max_iters, "Unexpectedly large number of iterations");
        (Chinese::chinese_new_moon_on_or_after((cur_rata_die + 1).as_moment()) - cur_rata_die) as u8
        // TODO: Add saturating functions to prevent overflow to u8
    }

    fn months_for_every_year(year: i32) -> u8 {
        if Self::is_leap_year(year) {
            13
        } else {
            12
        }
    }

    fn is_leap_year(year: i32) -> bool {
        let mid_year = Self::fixed_mid_year_from_year(year);
        Self::fixed_date_is_in_leap_year(mid_year)
    }

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
        (month, day as u8) // TODO: Add saturating functions to avoid overflow
    }
}

impl Calendar for Chinese {
    type DateInner = ChineseDateInner;

    // Construct a date from era/month codes and fields
    fn date_from_codes(
        &self,
        _era: types::Era,
        year: i32,
        month_code: types::MonthCode,
        day: u8,
    ) -> Result<Self::DateInner, CalendarError> {
        ArithmeticDate::new_from_lunar_codes(self, year, month_code, day).map(ChineseDateInner)
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

    /// The calendar-specific month code represented by `date`
    /// since the Chinese calendar has leap months, it is necessary to track
    /// whether a month is a leap month, as well as whether it comes after a
    /// leap month in the current calendar year. To accomplish this, the Chinese
    /// calendar month codes appends "L" (for "Leap") to a month code if it is a leap month,
    /// and appends "I" (for "Increment") to a month code if it occurs after a leap month.
    fn month(&self, date: &Self::DateInner) -> types::FormattableMonth {
        let ordinal = date.0.month;
        let leap_month = if Self::is_leap_year(date.0.year) {
            Self::get_leap_month_in_year(Self::fixed_mid_year_from_year(date.0.year))
        } else {
            14
        };
        let code_inner = if ordinal < leap_month {
            match ordinal {
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
                _ => tinystr!(4, "und"), // maximum num of months in a non-leap year is 12
            }
        } else {
            if ordinal == leap_month {
                match ordinal {
                    1 => tinystr!(4, "und"), // cannot have a leap month before the actual month
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
                    _ => tinystr!(4, "und"), // maximum num of months in a leap year is 13
                }
            } else {
                // if ordinal > leap_month; this means the date is in a leap year
                match ordinal {
                    1 => tinystr!(4, "und"), // this implies the leap month is < 1, which is impossible
                    2 => tinystr!(4, "und"), // this implies the leap month is = 1, which is impossible
                    3 => tinystr!(4, "M02I"),
                    4 => tinystr!(4, "M03I"),
                    5 => tinystr!(4, "M04I"),
                    6 => tinystr!(4, "M05I"),
                    7 => tinystr!(4, "M06I"),
                    8 => tinystr!(4, "M07I"),
                    9 => tinystr!(4, "M08I"),
                    10 => tinystr!(4, "M09I"),
                    11 => tinystr!(4, "M10I"),
                    12 => tinystr!(4, "M11I"),
                    13 => tinystr!(4, "M12I"),
                    _ => tinystr!(4, "und"), // maximum number of months in a leap year is 13
                }
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
    pub fn major_solar_term_from_iso(iso: IsoDateInner) -> i32 {
        let fixed: RataDie = Iso::fixed_from_iso(iso);
        Self::major_solar_term_from_fixed(fixed)
    }

    /// Get the current major solar term of a fixed date, output as an integer from 1..=12.
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

    // Returns true if the month of a given fixed date does not have a major solar term,
    // false otherwise.
    fn chinese_no_major_solar_term(date: RataDie) -> bool {
        Self::major_solar_term_from_fixed(date)
            == Self::major_solar_term_from_fixed(Self::chinese_new_moon_on_or_after(
                (date + 1).as_moment(),
            ))
    }

    /// Get the current major solar term of an ISO date
    pub fn minor_solar_term_from_iso(iso: IsoDateInner) -> i32 {
        let fixed: RataDie = Iso::fixed_from_iso(iso);
        Self::minor_solar_term_from_fixed(fixed)
    }

    /// Get the current minor solar term of a fixed date, output as an integer from 1..=12.
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

    // Returns UTC_OFFSET_PRE_1929 if the year is before 1929,
    // returns UTC_OFFSET_POST_1929 if the year is 1929 or after.
    fn chinese_offset(date: RataDie) -> f64 {
        let year = Iso::iso_from_fixed(date).year().number;
        if year < 1929 {
            UTC_OFFSET_PRE_1929
        } else {
            UTC_OFFSET_POST_1929
        }
    }

    // The fixed date in Chinese standard time of the next new moon
    // on or after the given moment.
    fn chinese_new_moon_on_or_after(moment: Moment) -> RataDie {
        let new_moon_moment = Astronomical::new_moon_at_or_after(Self::midnight_in_china(moment));
        let chinese_offset = Self::chinese_offset(new_moon_moment.as_rata_die());
        Location::standard_from_universal(new_moon_moment, chinese_offset).as_rata_die()
    }

    // The fixed date in Chinese standard time of the previous new moon
    // before the given moment.
    fn chinese_new_moon_before(moment: Moment) -> RataDie {
        let new_moon_moment = Astronomical::new_moon_before(Self::midnight_in_china(moment));
        let chinese_offset = Self::chinese_offset(new_moon_moment.as_rata_die());
        Location::standard_from_universal(new_moon_moment, chinese_offset).as_rata_die()
    }

    // Universal time of midnight at start of a Moment in China
    fn midnight_in_china(date: Moment) -> Moment {
        Location::universal_from_standard(date, Self::chinese_offset(date.as_rata_die()))
    }

    // Determines the fixed date of the Chinese new year in the sui4 (solar year based on the winter solstice)
    // which contains the fixed date passed as an argument.
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

    // Get the moment of the nearest winter solstice on or before a given fixed date
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
    /// Lisp reference code: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L5414-L5459
    pub(crate) fn chinese_date_from_fixed(date: RataDie) -> Date<Chinese> {
        let solstices = Self::get_chinese_winter_solstices(date);
        let prior_solstice = solstices.0;
        let month_after_eleventh =
            Self::chinese_new_moon_on_or_after((prior_solstice + 1).as_moment());
        let start_of_month = Self::chinese_new_moon_before((date + 1).as_moment());
        let m_float = start_of_month.as_moment().inner();
        let m12_float = month_after_eleventh.as_moment().inner();
        let month = adjusted_rem_euclid_f64(
            libm::round((m_float - m12_float) / MEAN_SYNODIC_MONTH),
            12.0,
        );
        let month_int = month as u8; // TODO: Add saturating functions to avoid overflow
        let elapsed_years =
            libm::floor(1.5 - month / 12.0 + ((date - CHINESE_EPOCH) as f64) / MEAN_TROPICAL_YEAR);
        let elapsed_years_int = i64_to_i32(elapsed_years as i64);
        debug_assert!(
            matches!(elapsed_years_int, I32Result::WithinRange(_)),
            "Chinese year should be in range of i32"
        );
        let year = elapsed_years_int.saturate();
        let day = (date - start_of_month + 1) as u8;
        Date::try_new_chinese_date(year, month_int, day).unwrap()
    }

    /// Get a RataDie from a ChineseDateInner
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

    /// Get the mid-year RataDie of a given year
    fn fixed_mid_year_from_year(elapsed_years: i32) -> RataDie {
        let cycle = quotient(elapsed_years - 1, 60) + 1;
        let year = adjusted_rem_euclid(elapsed_years, 60);
        CHINESE_EPOCH + ((((cycle - 1) * 60 + year - 1) as f64 + 0.5) * MEAN_TROPICAL_YEAR) as i64
    }

    /// Get the fixed date of the winter solstices immediately prior to and following a fixed date
    /// Returned as a tuple (prior, following)
    fn get_chinese_winter_solstices(date: RataDie) -> (RataDie, RataDie) {
        let prior_solstice = Self::chinese_winter_solstice_on_or_before(date);
        let following_solstice = Self::chinese_winter_solstice_on_or_before(prior_solstice + 370);
        (prior_solstice, following_solstice)
    }

    /// Returns true if the fixed date given is in a leap year, false otherwise
    fn fixed_date_is_in_leap_year(date: RataDie) -> bool {
        let solstices = Self::get_chinese_winter_solstices(date);
        let prior = solstices.0;
        let following = solstices.1;
        let month_after_eleventh =
            Self::chinese_new_moon_on_or_after((prior + 1).as_moment()).as_moment();
        let next_eleventh_month =
            Self::chinese_new_moon_before((following + 1).as_moment()).as_moment();
        libm::round((next_eleventh_month - month_after_eleventh) / MEAN_SYNODIC_MONTH) == 12.0
    }

    /// Given that a date is in a leap year, find which month in the year is a leap month.
    /// Since the first month in which there are no major solar terms is a leap month,
    /// this function cycles through months until it finds the leap month, then returns
    /// the number of that month. This function assumes the date passed in is in a leap year
    /// and tests to ensure this is the case by asserting that no more than twelve months are
    /// analyzed.
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
    fn format_chinese_year(year: i32) -> FormattableYear {
        let era = Era(tinystr!(16, "era"));
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
    fn test_fixed_from_chinese() {
        let date = Date::try_new_chinese_date(4660, 6, 6).unwrap();
        let fixed = Chinese::fixed_from_chinese_date_inner(date.inner);
        assert_eq!(fixed.to_i64_date(), 738694);
    }
}
