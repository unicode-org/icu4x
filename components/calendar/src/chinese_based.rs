// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains types and traits for use in the Chinese traditional lunar calendar,
//! as well as in related and derived calendars such as the Korean and Vietnamese lunar calendars.

use crate::{
    astronomy::{Astronomical, Location, MEAN_SYNODIC_MONTH, MEAN_TROPICAL_YEAR},
    calendar_arithmetic::{ArithmeticDate, CalendarArithmetic},
    helpers::{adjusted_rem_euclid, i64_to_i32, quotient, I32Result},
    rata_die::RataDie,
    types::Moment,
    Calendar, Date,
};

/// For an example of how to use this trait, see `impl ChineseBased<Chinese>` in [`Chinese`].
///
/// The trait ChineseBased is used by Chinese-based calendars to perform computations shared by such calendars.
/// To do so, calendars should implement `fn location` by providing a location at which observations of the
/// new moon are recorded (which may change over time), with the most important part being the time zone
/// offset (longitude, latitude, and elevation are not relevant for these particular calculations);
/// define `const EPOCH` as a `RataDie` marking the start date of the era of the Calendar for internal use,
/// which may not accurately reflect how years or eras are marked traditionally or how they will be seen
/// by end-users; and implement `fn new_chinese_based_date` by taking a year, month, and day, and
/// returning a Date of the relevant Calendar type.
pub(crate) trait ChineseBased<C: ChineseBased<C> + CalendarArithmetic> {
    /// Given a fixed date, the location used for observations of the new moon in order to
    /// calculate the beginning of months. For multiple Chinese-based lunar calendars, this has
    /// changed over the years, and can cause differences in calendar date.
    fn location(fixed: RataDie) -> Location;

    /// The RataDie of the beginning of the epoch used for internal computation; this may not
    /// reflect traditional methods of year-tracking or eras, since Chinese-based calendars
    /// may not track years ordinally in the same way many western calendars do.
    const EPOCH: RataDie;

    /// Given a year, month, and day, create a Date<C> where C is a Chinese-based calendar.
    ///
    /// This function should just call try_new_C_date where C is the name of the calendar.
    fn new_chinese_based_date(year: i32, month: u8, day: u8) -> Date<C>;
}

/// Chinese-based calendars define DateInner as a calendar-specific struct wrapping ChineseBasedDateInner.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub(crate) struct ChineseBasedDateInner<C: ChineseBased<C> + CalendarArithmetic>(
    pub(crate) ArithmeticDate<C>,
);

impl<C: ChineseBased<C> + CalendarArithmetic> ChineseBasedDateInner<C> {
    /// Get the current major solar term of a fixed date, output as an integer from 1..=12.
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Lisp reference code: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L5273-L5281
    pub(crate) fn major_solar_term_from_fixed(date: RataDie) -> i32 {
        let moment: Moment = date.as_moment();
        let location = C::location(date);
        let universal: Moment = Location::universal_from_standard(moment, location);
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
    pub(crate) fn no_major_solar_term(date: RataDie) -> bool {
        Self::major_solar_term_from_fixed(date)
            == Self::major_solar_term_from_fixed(Self::new_moon_on_or_after((date + 1).as_moment()))
    }

    /// Get the current minor solar term of a fixed date, output as an integer from 1..=12.
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Lisp reference code: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L5303-L5316
    pub(crate) fn minor_solar_term_from_fixed(date: RataDie) -> i32 {
        let moment: Moment = date.as_moment();
        let location = C::location(date);
        let universal: Moment = Location::universal_from_standard(moment, location);
        let solar_longitude = i64_to_i32(Astronomical::solar_longitude(universal) as i64);
        debug_assert!(
            matches!(solar_longitude, I32Result::WithinRange(_)),
            "Solar longitude should be in range of i32"
        );
        let s = solar_longitude.saturate();
        adjusted_rem_euclid(3 + quotient(s - 15, 30), 12)
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
    pub(crate) fn midnight(date: Moment) -> Moment {
        Location::universal_from_standard(date, C::location(date.as_rata_die()))
    }

    /// Determines the fixed date of the lunar new year in the sui4 (solar year based on the winter solstice)
    /// which contains the fixed date passed as an argument.
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Lisp reference code: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L5370-L5394
    pub(crate) fn new_year_in_sui(date: RataDie) -> RataDie {
        let prior_solstice = Self::winter_solstice_on_or_before(date); // s1
        let following_solstice = Self::winter_solstice_on_or_before(prior_solstice + 370); // s2
        let month_after_eleventh = Self::new_moon_on_or_after((prior_solstice + 1).as_moment()); // m12
        let month_after_twelfth =
            Self::new_moon_on_or_after((month_after_eleventh + 1).as_moment()); // m13
        let next_eleventh_month = Self::new_moon_before((following_solstice + 1).as_moment()); // next-m11
        let m12_float = month_after_eleventh.as_moment().inner();
        let next_m11_float = next_eleventh_month.as_moment().inner();
        let lhs_argument = libm::round((next_m11_float - m12_float) / MEAN_SYNODIC_MONTH) as i64;
        if lhs_argument == 12
            && (Self::no_major_solar_term(month_after_eleventh)
                || Self::no_major_solar_term(month_after_twelfth))
        {
            Self::new_moon_on_or_after((month_after_twelfth + 1).as_moment())
        } else {
            month_after_twelfth
        }
    }

    /// Get the moment of the nearest winter solstice on or before a given fixed date
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Lisp reference code: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L5359-L5368
    pub(crate) fn winter_solstice_on_or_before(date: RataDie) -> RataDie {
        let approx = Astronomical::estimate_prior_solar_longitude(
            270.0,
            Self::midnight((date + 1).as_moment()),
        );
        let mut iters = 0;
        let max_iters = 367;
        let mut day = Moment::new(libm::floor(approx.inner() - 1.0));
        while iters < max_iters && 270.0 >= Astronomical::solar_longitude(Self::midnight(day + 1.0))
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

    /// Get the fixed date of the nearest Lunar New Year on or before a given fixed date.
    ///
    /// Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Lisp reference code: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L5396-L5405
    pub(crate) fn new_year_on_or_before_fixed_date(date: RataDie) -> RataDie {
        let new_year = Self::new_year_in_sui(date);
        if date >= new_year {
            new_year
        } else {
            Self::new_year_in_sui(date - 180)
        }
    }

    /// Get a Date<C> from a fixed date
    ///
    /// Months are calculated by iterating through the dates of new moons until finding the last month which
    /// does not exceed the given fixed date. The day of month is calculated by subtracting the fixed date
    /// from the fixed date of the beginning of the month.
    ///
    /// The calculation for `elapsed_years` in this function is based on code from _Calendrical Calculations_ by Reingold & Dershowitz.
    /// Lisp reference code: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L5414-L5459
    pub(crate) fn chinese_based_date_from_fixed(date: RataDie) -> Date<C> {
        let new_year = Self::new_year_on_or_before_fixed_date(date);
        let elapsed_years =
            libm::floor(1.5 - 1.0 / 12.0 + ((new_year - C::EPOCH) as f64) / MEAN_TROPICAL_YEAR)
                as i64;
        let year = match i64_to_i32(elapsed_years) {
            I32Result::BelowMin(_) => {
                return Self::min_chinese_based_date();
            }
            I32Result::AboveMax(_) => {
                return Self::max_chinese_based_date();
            }
            I32Result::WithinRange(y) => y,
        };
        let mut month = 1;
        let max_months = 14;
        let mut cur_month = new_year;
        let mut next_month = Self::new_moon_on_or_after((cur_month + 1).as_moment());
        while next_month <= date && month < max_months {
            month += 1;
            cur_month = next_month;
            next_month = Self::new_moon_on_or_after((cur_month + 1).as_moment());
        }
        debug_assert!(month < max_months, "Unexpectedly large number of months");
        let day = (date - cur_month + 1) as u8;
        C::new_chinese_based_date(year, month, day)
    }

    /// The minimum possible ChineseBasedDate given the minimum values of
    /// year, month, and day fields in ArithmeticDate
    fn min_chinese_based_date() -> Date<C> {
        let min_arithmetic: ArithmeticDate<C> = ArithmeticDate::min_date();
        let year = min_arithmetic.year;
        let month = min_arithmetic.month;
        let day = min_arithmetic.day;
        C::new_chinese_based_date(year, month, day)
    }

    /// The maximum possible ChineseBasedDate given the maximum values of
    /// year, month, and day fields in ArithmeticDate
    fn max_chinese_based_date() -> Date<C> {
        let max_arithmetic: ArithmeticDate<C> = ArithmeticDate::max_date();
        let year = max_arithmetic.year;
        let month = max_arithmetic.month;
        let day = max_arithmetic.day;
        C::new_chinese_based_date(year, month, day)
    }

    /// Get a RataDie from a ChineseBasedDateInner
    ///
    /// This finds the RataDie of the new year of the year given, then finds the RataDie of the new moon
    /// (beginning of the month) of the month given, then adds the necessary number of days.
    pub(crate) fn fixed_from_chinese_based_date_inner(date: ChineseBasedDateInner<C>) -> RataDie {
        let year = date.0.year;
        let month = date.0.month as i64;
        let day = date.0.day as i64;
        let mid_year = Self::fixed_mid_year_from_year(year);
        let new_year = Self::new_year_on_or_before_fixed_date(mid_year);
        let month_approx = new_year + (month - 1) * 29;
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
        let prev_new_year = Self::new_year_on_or_before_fixed_date(date);
        let next_new_year = Self::new_year_on_or_before_fixed_date(prev_new_year + 400);
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
    pub(crate) fn get_leap_month_in_year(date: RataDie) -> u8 {
        let mut cur = Self::new_year_on_or_before_fixed_date(date);
        let mut result = 1;
        let max_iters = 13;
        while result < max_iters && !Self::no_major_solar_term(cur) {
            cur = Self::new_moon_on_or_after((cur + 1).as_moment());
            result += 1;
        }
        debug_assert!(result < max_iters, "The given year was not a leap year and an unexpected number of iterations occurred searching for a leap month.");
        result
    }
}

impl<C: ChineseBased<C> + Calendar> CalendarArithmetic for C {
    /// Returns the number of days in the given (year, month). In the Chinese calendar, months start at each
    /// new moon, so this function finds the number of days between the new moon at the beginning of the given
    /// month and the new moon at the beginning of the next month.
    fn month_days(year: i32, month: u8) -> u8 {
        let mid_year = ChineseBasedDateInner::<C>::fixed_mid_year_from_year(year);
        let new_year = ChineseBasedDateInner::<C>::new_year_on_or_before_fixed_date(mid_year);
        let approx = new_year + ((month - 1) as i64 * 29);
        let prev_new_moon = ChineseBasedDateInner::<C>::new_moon_before((approx + 15).as_moment());
        let next_new_moon =
            ChineseBasedDateInner::<C>::new_moon_on_or_after((approx + 15).as_moment());
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
            ChineseBasedDateInner::<C>::new_year_on_or_before_fixed_date(mid_year + 370);
        let last_day = next_new_year - 1;
        let month = if ChineseBasedDateInner::<C>::fixed_date_is_in_leap_year(last_day) {
            13
        } else {
            12
        };
        let day = last_day - ChineseBasedDateInner::<C>::new_moon_before(last_day.as_moment()) + 1;
        (month, day as u8)
    }

    fn days_in_provided_year(year: i32) -> u32 {
        let months_in_year = Self::months_for_every_year(year);
        let mut days: u32 = 0;
        for month in 1..=months_in_year {
            days += Self::month_days(year, month) as u32;
        }
        days
    }
}
