// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains types and traits for use in the Chinese traditional lunar calendar,
//! as well as in related and derived calendars such as the Korean and Vietnamese lunar calendars.
//!
//! ```rust
//! use icu::calendar::{chinese::Chinese, Iso, Date};
//!
//! let iso_date = Date::try_new_iso_date(2023, 6, 23).unwrap();
//! let chinese_date = Date::new_from_iso(iso_date, Chinese);
//!
//! assert_eq!(chinese_date.year().number, 4660);
//! assert_eq!(chinese_date.year().related_iso, Some(2023));
//! assert_eq!(chinese_date.year().cyclic, Some(40));
//! assert_eq!(chinese_date.month().ordinal, 6);
//! assert_eq!(chinese_date.day_of_month().0, 6);
//! ```

use crate::{
    astronomy::{self, Astronomical, Location, MEAN_SYNODIC_MONTH, MEAN_TROPICAL_YEAR},
    calendar_arithmetic::{
        ArithmeticDate, CalendarArithmetic, MAX_ITERS_FOR_DAYS_OF_YEAR,
        MAX_ITERS_FOR_MONTHS_OF_YEAR,
    },
    helpers::{adjusted_rem_euclid, i64_to_i32, quotient, I32Result},
    rata_die::RataDie,
    types::{Moment, MonthCode},
    Calendar, CalendarError,
};
use core::num::NonZeroU8;

/// The trait ChineseBased is used by Chinese-based calendars to perform computations shared by such calendar.
/// To do so, calendars should:
///
/// - Implement `fn location` by providing a location at which observations of the moon are recorded, which
/// may change over time (the zone is important, long, lat, and elevation are not relevant for these calculations)
/// - Define `const EPOCH` as a `RataDie` marking the start date of the era of the Calendar for internal use,
/// which may not accurately reflect how years or eras are marked traditionally or seen by end-users
/// - Implement `fn new_chinese_based_date` by taking a year, month, and day in a Chinese-based calendar and
/// returning a Date of the relevant Calendar type.
///
/// For an example of how to use this trait, see `impl ChineseBased for Chinese` in [`Chinese`].
pub(crate) trait ChineseBased: CalendarArithmetic + Sized {
    /// Given a fixed date, return the location used for observations of the new moon in order to
    /// calculate the beginning of months. For multiple Chinese-based lunar calendars, this has
    /// changed over the years, and can cause differences in calendar date.
    fn location(fixed: RataDie) -> Location;

    /// The RataDie of the beginning of the epoch used for internal computation; this may not
    /// reflect traditional methods of year-tracking or eras, since Chinese-based calendars
    /// may not track years ordinally in the same way many western calendars do.
    const EPOCH: RataDie;
}

/// Chinese-based calendars define DateInner as a calendar-specific struct wrapping ChineseBasedDateInner.
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub(crate) struct ChineseBasedDateInner<C: ChineseBased>(
    pub(crate) ArithmeticDate<C>,
    pub(crate) ChineseBasedCache,
);

/// A caching struct used to store information for ChineseBasedDates
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub(crate) struct ChineseBasedCache {
    pub(crate) new_year: RataDie,
    pub(crate) next_new_year: RataDie,
    pub(crate) leap_month: Option<NonZeroU8>,
}

impl<C: ChineseBased + CalendarArithmetic> ChineseBasedDateInner<C> {
    /// Get the current major solar term of a fixed date, output as an integer from 1..=12.
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Lisp reference code: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L5273-L5281
    pub(crate) fn major_solar_term_from_fixed(date: RataDie) -> u32 {
        let moment: Moment = date.as_moment();
        let location = C::location(date);
        let universal: Moment = Location::universal_from_standard(moment, location);
        let solar_longitude = i64_to_i32(Astronomical::solar_longitude(
            Astronomical::julian_centuries(universal),
        ) as i64);
        debug_assert!(
            matches!(solar_longitude, I32Result::WithinRange(_)),
            "Solar longitude should be in range of i32"
        );
        let s = solar_longitude.saturate();
        let result_signed = adjusted_rem_euclid(2 + quotient(s, 30), 12);
        debug_assert!(result_signed >= 0);
        result_signed as u32
    }

    /// Returns true if the month of a given fixed date does not have a major solar term,
    /// false otherwise.
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Lisp reference code: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L5345-L5351
    pub(crate) fn no_major_solar_term(date: RataDie) -> bool {
        Self::major_solar_term_from_fixed(date)
            == Self::major_solar_term_from_fixed(Self::new_moon_on_or_after((date + 1).as_moment()))
    }

    /// Get the current minor solar term of a fixed date, output as an integer from 1..=12.
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Lisp reference code: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L5303-L5316
    pub(crate) fn minor_solar_term_from_fixed(date: RataDie) -> u32 {
        let moment: Moment = date.as_moment();
        let location = C::location(date);
        let universal: Moment = Location::universal_from_standard(moment, location);
        let solar_longitude = i64_to_i32(Astronomical::solar_longitude(
            Astronomical::julian_centuries(universal),
        ) as i64);
        debug_assert!(
            matches!(solar_longitude, I32Result::WithinRange(_)),
            "Solar longitude should be in range of i32"
        );
        let s = solar_longitude.saturate();
        let result_signed = adjusted_rem_euclid(3 + quotient(s - 15, 30), 12);
        debug_assert!(result_signed >= 0);
        result_signed as u32
    }

    /// The fixed date in standard time at the observation location of the next new moon on or after a given Moment.
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Lisp reference code: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L5329-L5338
    pub(crate) fn new_moon_on_or_after(moment: Moment) -> RataDie {
        let new_moon_moment = Astronomical::new_moon_at_or_after(Self::midnight(moment));
        let location = C::location(new_moon_moment.as_rata_die());
        Location::standard_from_universal(new_moon_moment, location).as_rata_die()
    }

    /// The fixed date in standard time at the observation location of the previous new moon before a given Moment.
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Lisp reference code: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L5318-L5327
    pub(crate) fn new_moon_before(moment: Moment) -> RataDie {
        let new_moon_moment = Astronomical::new_moon_before(Self::midnight(moment));
        let location = C::location(new_moon_moment.as_rata_die());
        Location::standard_from_universal(new_moon_moment, location).as_rata_die()
    }

    /// Universal time of midnight at start of a Moment's day at the observation location
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Lisp reference code: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L5353-L5357
    pub(crate) fn midnight(moment: Moment) -> Moment {
        Location::universal_from_standard(moment, C::location(moment.as_rata_die()))
    }

    /// Determines the fixed date of the lunar new year in the sui4 (solar year based on the winter solstice)
    /// which contains the fixed date passed as an argument.
    /// This function also returns the local variable `following_solstice` for optimization (see #3743).
    /// An optional `prior_solstice` field can also be passed in for optimization.
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Lisp reference code: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L5370-L5394
    pub(crate) fn new_year_in_sui(
        date: RataDie,
        prior_solstice: Option<RataDie>,
    ) -> (RataDie, RataDie) {
        let prior_solstice = if let Some(prior) = prior_solstice {
            prior
        } else {
            Self::winter_solstice_on_or_before(date)
        }; // s1
        let following_solstice = Self::winter_solstice_on_or_before(prior_solstice + 400); // s2
        let month_after_eleventh = Self::new_moon_on_or_after((prior_solstice + 1).as_moment()); // m12
        let month_after_twelfth =
            Self::new_moon_on_or_after((month_after_eleventh + 1).as_moment()); // m13
        let next_eleventh_month = Self::new_moon_before((following_solstice + 1).as_moment()); // next-m11
        let lhs_argument =
            libm::round((next_eleventh_month - month_after_eleventh) as f64 / MEAN_SYNODIC_MONTH)
                as i64;
        if lhs_argument == 12
            && (Self::no_major_solar_term(month_after_eleventh)
                || Self::no_major_solar_term(month_after_twelfth))
        {
            (
                Self::new_moon_on_or_after((month_after_twelfth + 1).as_moment()),
                following_solstice,
            )
        } else {
            (month_after_twelfth, following_solstice)
        }
    }

    /// Get the moment of the nearest winter solstice on or before a given fixed date
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Lisp reference code: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L5359-L5368
    pub(crate) fn winter_solstice_on_or_before(date: RataDie) -> RataDie {
        let approx = Astronomical::estimate_prior_solar_longitude(
            astronomy::WINTER,
            Self::midnight((date + 1).as_moment()),
        );
        let mut iters = 0;
        let mut day = Moment::new(libm::floor(approx.inner() - 1.0));
        while iters < MAX_ITERS_FOR_DAYS_OF_YEAR
            && astronomy::WINTER
                >= Astronomical::solar_longitude(Astronomical::julian_centuries(Self::midnight(
                    day + 1.0,
                )))
        {
            iters += 1;
            day += 1.0;
        }
        debug_assert!(
            iters < MAX_ITERS_FOR_DAYS_OF_YEAR,
            "Number of iterations was higher than expected"
        );
        day.as_rata_die()
    }

    /// Get the fixed date of the nearest Lunar New Year on or before a given fixed date.
    /// This function also returns the solstice following a given date for optimization (see #3743).
    /// In some situations, the RataDie for the prior winter solstice can be passed in
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Lisp reference code: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L5396-L5405
    pub(crate) fn new_year_on_or_before_fixed_date(
        date: RataDie,
        prior_solstice: Option<RataDie>,
    ) -> (RataDie, RataDie) {
        let new_year = Self::new_year_in_sui(date, prior_solstice);
        if date >= new_year.0 {
            new_year
        } else {
            Self::new_year_in_sui(date - 180, prior_solstice)
        }
    }

    /// Get a ChineseBasedDateInner from a fixed date
    ///
    /// Months are calculated by iterating through the dates of new moons until finding the last month which
    /// does not exceed the given fixed date. The day of month is calculated by subtracting the fixed date
    /// from the fixed date of the beginning of the month.
    ///
    /// The calculation for `elapsed_years` and `month` in this function are based on code from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Lisp reference code: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L5414-L5459
    pub(crate) fn chinese_based_date_from_fixed(date: RataDie) -> ChineseBasedDateInner<C> {
        let (first_day_of_year, next_solstice) = Self::new_year_on_or_before_fixed_date(date, None);
        let next_new_year =
            Self::new_year_on_or_before_fixed_date(first_day_of_year + 400, Some(next_solstice)).0;
        let year_float = libm::floor(
            1.5 - 1.0 / 12.0 + ((first_day_of_year - C::EPOCH) as f64) / MEAN_TROPICAL_YEAR,
        );
        let year_int = i64_to_i32(year_float as i64);
        debug_assert!(
            matches!(year_int, I32Result::WithinRange(_)),
            "Year should be in range of i32"
        );
        let year = year_int.saturate();
        let new_moon = Self::new_moon_before((date + 1).as_moment());
        let month_i64 =
            libm::round((new_moon - first_day_of_year) as f64 / MEAN_SYNODIC_MONTH) as i64 + 1;
        debug_assert!(
            ((u8::MIN as i64)..=(u8::MAX as i64)).contains(&month_i64),
            "Month should be in range of u8! Value {month_i64} failed for RD {date:?}"
        );
        let month = month_i64 as u8;
        let day_i64 = date - new_moon + 1;
        debug_assert!(
            ((u8::MIN as i64)..=(u8::MAX as i64)).contains(&month_i64),
            "Day should be in range of u8! Value {month_i64} failed for RD {date:?}"
        );
        let day = day_i64 as u8;
        let is_leap_year = Self::new_year_is_leap_year(
            first_day_of_year,
            Some(next_solstice),
            Some(next_new_year),
        );
        let leap_month = if is_leap_year {
            // This doesn't need to be checked for `None`, since `get_leap_month_from_new_year`
            // will always return a number greater than or equal to 1, and less than 14.
            NonZeroU8::new(Self::get_leap_month_from_new_year(first_day_of_year))
        } else {
            None
        };
        let cache = ChineseBasedCache {
            new_year: first_day_of_year,
            next_new_year,
            leap_month,
        };
        // This can use `new_unchecked` because this function is only ever called from functions which
        // generate the year, month, and day; therefore, there should never be a situation where
        // creating this ArithmeticDate would fail, since the same algorithms used to generate the ymd
        // are also used to check for valid ymd.
        ChineseBasedDateInner(ArithmeticDate::new_unchecked(year, month, day), cache)
    }

    /// Get a RataDie from a ChineseBasedDateInner
    ///
    /// This finds the RataDie of the new year of the year given, then finds the RataDie of the new moon
    /// (beginning of the month) of the month given, then adds the necessary number of days.
    pub(crate) fn fixed_from_chinese_based_date_inner(date: ChineseBasedDateInner<C>) -> RataDie {
        let month = date.0.month as i64;
        let day = date.0.day as i64;
        let first_day_of_year = date.1.new_year;
        let month_approx = first_day_of_year + (month - 1) * 29;
        let prior_new_moon = Self::new_moon_on_or_after(month_approx.as_moment());
        prior_new_moon + day - 1
    }

    /// Get a RataDie in the middle of a year; this is not necessarily meant for direct use in
    /// calculations; rather, it is useful for getting a RataDie guaranteed to be in a given year
    /// as input for other calculations like [`get_leap_month_in_year`].
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz
    /// Lisp reference code: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L5469-L5475
    pub(crate) fn fixed_mid_year_from_year(elapsed_years: i32) -> RataDie {
        let cycle = quotient(elapsed_years - 1, 60) + 1;
        let year = adjusted_rem_euclid(elapsed_years, 60);
        C::EPOCH + ((((cycle - 1) * 60 + year - 1) as f64 + 0.5) * MEAN_TROPICAL_YEAR) as i64
    }

    /// Returns true if the fixed date given is in a leap year, false otherwise
    pub(crate) fn fixed_date_is_in_leap_year(date: RataDie) -> bool {
        let (prev_new_year, next_solstice) = Self::new_year_on_or_before_fixed_date(date, None);
        Self::new_year_is_leap_year(prev_new_year, Some(next_solstice), None)
    }

    /// Returns true if the fixed date given is in a leap year, assuming the fixed date
    /// given is the RataDie of a new year. Optionally, a RataDie representing the prior winter
    /// solstice before the `new_year` can be passed in as an Option argument, as well as an
    /// Option<RataDie> representing the next new year.
    pub(crate) fn new_year_is_leap_year(
        new_year: RataDie,
        next_solstice: Option<RataDie>,
        next_new_year: Option<RataDie>,
    ) -> bool {
        let new_year_after = if let Some(next) = next_new_year {
            next
        } else {
            Self::new_year_on_or_before_fixed_date(new_year + 400, next_solstice).0
        };
        let difference = new_year_after - new_year;
        difference > 365
    }

    /// Given that `new_year` is the first day of a leap year, find which month in the year is a leap month.
    /// Since the first month in which there are no major solar terms is a leap month, this function
    /// cycles through months until it finds the leap month, then returns the number of that month. This
    /// function assumes the date passed in is in a leap year and tests to ensure this is the case in debug
    /// mode by asserting that no more than thirteen months are analyzed.
    ///
    /// Conceptually similar to code from _Calendrical Calculations_ by Reingold & Dershowitz
    /// Lisp reference code: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L5443-L5450
    pub(crate) fn get_leap_month_from_new_year(new_year: RataDie) -> u8 {
        let mut cur = new_year;
        let mut result = 1;
        while result < MAX_ITERS_FOR_MONTHS_OF_YEAR && !Self::no_major_solar_term(cur) {
            cur = Self::new_moon_on_or_after((cur + 1).as_moment());
            result += 1;
        }
        debug_assert!(result < MAX_ITERS_FOR_MONTHS_OF_YEAR, "The given year was not a leap year and an unexpected number of iterations occurred searching for a leap month.");
        result
    }

    /// Create a new arithmetic date from a year, month ordinal, and day with bounds checking; returns the
    /// result of creating this arithmetic date, as well as a ChineseBasedCache - either the one passed in
    /// optionally as an argument, or a new ChineseBasedCache for the given year, month, and day args.
    pub(crate) fn new_from_ordinals(
        year: i32,
        month: u8,
        day: u8,
        cache: &ChineseBasedCache,
    ) -> Result<ArithmeticDate<C>, CalendarError> {
        let max_month = Self::months_in_year_cached(cache);
        if month > max_month {
            return Err(CalendarError::Overflow {
                field: "month",
                max: max_month as usize,
            });
        }

        let max_day = Self::days_in_month(month, cache.new_year);
        if day > max_day {
            return Err(CalendarError::Overflow {
                field: "day",
                max: max_day as usize,
            });
        }

        // Unchecked can be used because month and day are already checked in this fn

        Ok(ArithmeticDate::<C>::new_unchecked(year, month, day))
    }

    /// Call `months_in_year_cached` on a `ChineseBasedDateInner`
    pub(crate) fn months_in_year_inner(&self) -> u8 {
        Self::months_in_year_cached(&self.1)
    }

    /// Return the number of months in a given year, which is 13 in a leap year, and 12 in a common year.
    /// Also takes a `ChineseBasedCache` argument.
    fn months_in_year_cached(cache: &ChineseBasedCache) -> u8 {
        if cache.leap_month.is_some() {
            13
        } else {
            12
        }
    }

    /// Calls `days_in_month` on an instance of ChineseBasedDateInner
    pub(crate) fn days_in_month_inner(&self) -> u8 {
        Self::days_in_month(self.0.month, self.1.new_year)
    }

    /// Returns the number of days in the given `month` after the given `new_year`.
    fn days_in_month(month: u8, new_year: RataDie) -> u8 {
        let approx = new_year + ((month - 1) as i64 * 29);
        let prev_new_moon = Self::new_moon_before((approx + 15).as_moment());
        let next_new_moon = Self::new_moon_on_or_after((approx + 15).as_moment());
        let result = (next_new_moon - prev_new_moon) as u8;
        debug_assert!(result == 29 || result == 30);
        result
    }

    /// Calls day_in_year_cached on an instance of ChineseBasedDateInner
    pub(crate) fn days_in_year_inner(&self) -> u16 {
        Self::days_in_year(self.1.new_year, self.1.next_new_year)
    }

    /// Returns the number of day in the given year bounds
    fn days_in_year(prev_new_year: RataDie, next_new_year: RataDie) -> u16 {
        let result = next_new_year - prev_new_year;
        debug_assert!(
            ((u16::MIN as i64)..=(u16::MAX as i64)).contains(&result),
            "Days in year should be in range of u16."
        );
        result as u16
    }

    /// Compute a `ChineseBasedCache` from a ChineseBased year
    pub(crate) fn compute_cache(year: i32) -> ChineseBasedCache {
        let mid_year = Self::fixed_mid_year_from_year(year);
        let prior_solstice = Self::winter_solstice_on_or_before(mid_year);
        let (new_year, following_solstice) =
            Self::new_year_on_or_before_fixed_date(mid_year, Some(prior_solstice));
        let next_new_year =
            Self::new_year_on_or_before_fixed_date(new_year + 400, Some(following_solstice)).0;
        let is_leap_year =
            Self::new_year_is_leap_year(new_year, Some(following_solstice), Some(next_new_year));
        let leap_month = if is_leap_year {
            // This doesn't need to be checked for None because `get_leap_month_from_new_year`
            // will always return a value between 1..=13
            NonZeroU8::new(Self::get_leap_month_from_new_year(new_year))
        } else {
            None
        };
        ChineseBasedCache {
            new_year,
            next_new_year,
            leap_month,
        }
    }
}

impl<C: ChineseBased + Calendar> CalendarArithmetic for C {
    /// Returns the number of days in the given (year, month). In the Chinese calendar, months start at each
    /// new moon, so this function finds the number of days between the new moon at the beginning of the given
    /// month and the new moon at the beginning of the next month.
    fn month_days(year: i32, month: u8) -> u8 {
        let mid_year = ChineseBasedDateInner::<C>::fixed_mid_year_from_year(year);
        let new_year =
            ChineseBasedDateInner::<C>::new_year_on_or_before_fixed_date(mid_year, None).0;
        ChineseBasedDateInner::<C>::days_in_month(month, new_year)
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
        let mid_year = ChineseBasedDateInner::<C>::fixed_mid_year_from_year(year);
        ChineseBasedDateInner::<C>::fixed_date_is_in_leap_year(mid_year)
    }

    /// Returns the (month, day) of the last day in a Chinese year (the day before Chinese New Year).
    /// The last month in a year will always be 12 in a common year or 13 in a leap year. The day is
    /// determined by finding the day immediately before the next new year and calculating the number
    /// of days since the last new moon (beginning of the last month in the year).
    fn last_month_day_in_year(year: i32) -> (u8, u8) {
        let mid_year = ChineseBasedDateInner::<C>::fixed_mid_year_from_year(year);
        let next_new_year =
            ChineseBasedDateInner::<C>::new_year_on_or_before_fixed_date(mid_year + 370, None).0;
        let last_day = next_new_year - 1;
        let month = if ChineseBasedDateInner::<C>::fixed_date_is_in_leap_year(last_day) {
            13
        } else {
            12
        };
        let day = last_day - ChineseBasedDateInner::<C>::new_moon_before(last_day.as_moment()) + 1;
        (month, day as u8)
    }

    fn days_in_provided_year(year: i32) -> u16 {
        let mid_year = ChineseBasedDateInner::<C>::fixed_mid_year_from_year(year);
        let (prev_new_year, solstice) =
            ChineseBasedDateInner::<C>::new_year_on_or_before_fixed_date(mid_year, None);
        let next_new_year = ChineseBasedDateInner::<C>::new_year_on_or_before_fixed_date(
            prev_new_year + 370,
            Some(solstice),
        )
        .0;
        ChineseBasedDateInner::<C>::days_in_year(prev_new_year, next_new_year)
    }
}

/// Get the ordinal lunar month from a code for chinese-based calendars.
pub(crate) fn chinese_based_ordinal_lunar_month_from_code<C: ChineseBased>(
    code: MonthCode,
    cache: ChineseBasedCache,
) -> Option<u8> {
    let leap_month = if let Some(leap) = cache.leap_month {
        leap.get()
    } else {
        // 14 is a sentinel value, greater than all other months, for the purpose of computation only;
        // it is impossible to actually have 14 months in a year.
        14
    };

    if code.0.len() < 3 {
        return None;
    }
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
