// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains types and implementations for the Islamic calendars.
//!
//! ```rust
//! use icu::calendar::{Date, DateTime};
//!
//! // `Date` type
//! let islamic_date = Date::try_new_observational_islamic_date(1348, 10, 11)
//!     .expect("Failed to initialize islamic Date instance.");
//!
//! // `DateTime` type
//! let islamic_datetime = DateTime::try_new_observational_islamic_datetime(1348, 10, 11, 13, 1, 0)
//!     .expect("Failed to initialize islamic DateTime instance.");
//!
//! // `Date` checks
//! assert_eq!(islamic_date.year().number, 1348);
//! assert_eq!(islamic_date.month().ordinal, 10);
//! assert_eq!(islamic_date.day_of_month().0, 11);
//!
//! // `DateTime` checks
//! assert_eq!(islamic_datetime.date.year().number, 1348);
//! assert_eq!(islamic_datetime.date.month().ordinal, 10);
//! assert_eq!(islamic_datetime.date.day_of_month().0, 11);
//! assert_eq!(islamic_datetime.time.hour.number(), 13);
//! assert_eq!(islamic_datetime.time.minute.number(), 1);
//! assert_eq!(islamic_datetime.time.second.number(), 0);
//! ```

use crate::calendar_arithmetic::{ArithmeticDate, CalendarArithmetic};
use crate::helpers::{div_rem_euclid, div_rem_euclid_f64, next};
use crate::julian::Julian;
use crate::rata_die::RataDie;
use crate::types::Moment;
use crate::{astronomy::*, Iso};
use crate::{types, Calendar, CalendarError, Date, DateDuration, DateDurationUnit, DateTime};
use ::tinystr::tinystr;

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord)]
#[allow(clippy::exhaustive_structs)]
/// Islamic Observational Calendar (Default)
pub struct IslamicObservational;
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
#[allow(clippy::exhaustive_structs)]
/// Civil / Arithmetical Islamic Calendar (Used for administrative purposes)
pub struct IslamicCivil;
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
#[allow(clippy::exhaustive_structs)]
/// Umm-al-Qura Hijri Calendar (Used in Saudi Arabia)
pub struct UmmalQura;
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
#[allow(clippy::exhaustive_structs)]
/// A Tabular version of the Arithmetical Islamic Calendar
pub struct IslamicTabular;

// Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L2066
const FIXED_ISLAMIC_EPOCH_FRIDAY: RataDie = Julian::fixed_from_julian_integers(622, 7, 16);
// const FIXED_ISLAMIC_EPOCH_THURSDAY: RataDie = Julian::fixed_from_julian_integers(622, 7, 15);

// Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L6898
const CAIRO: Location = Location {
    latitude: 30.1,
    longitude: 31.3,
    elevation: 200.0,
    zone: (1_f64 / 12_f64),
};

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
/// The inner date type used for representing [`Date`]s of [`IslamicObservational`]. See [`Date`] and [`IslamicObservational`] for more details.
pub struct IslamicDateInner(ArithmeticDate<IslamicObservational>);

// OBSERVATIONAL CALENDAR

impl CalendarArithmetic for IslamicObservational {
    fn month_days(year: i32, month: u8) -> u8 {
        let midmonth = FIXED_ISLAMIC_EPOCH_FRIDAY.to_f64_date()
            + (((year - 1) as f64) * 12.0 + month as f64 - 0.5) * MEAN_SYNODIC_MONTH;

        let f_date = Astronomical::phasis_on_or_before(RataDie::new(midmonth as i64), CAIRO);

        Astronomical::month_length(f_date, CAIRO)
    }

    fn months_for_every_year(_year: i32) -> u8 {
        12
    }

    fn days_in_provided_year(_year: i32) -> u32 {
        355
    }

    // As an observational-lunar calendar, it does not have leap years.
    fn is_leap_year(_year: i32) -> bool {
        false
    }

    fn last_month_day_in_year(year: i32) -> (u8, u8) {
        let days = Self::month_days(year, 12);

        (12, days)
    }
}

impl Calendar for IslamicObservational {
    type DateInner = IslamicDateInner;
    fn date_from_codes(
        &self,
        era: types::Era,
        year: i32,
        month_code: types::MonthCode,
        day: u8,
    ) -> Result<Self::DateInner, CalendarError> {
        let year = if era.0 == tinystr!(16, "ah") {
            year
        } else {
            return Err(CalendarError::UnknownEra(era.0, self.debug_name()));
        };

        ArithmeticDate::new_from_solar_codes(self, year, month_code, day).map(IslamicDateInner)
    }

    fn date_from_iso(&self, iso: Date<crate::Iso>) -> Self::DateInner {
        let fixed_iso = Iso::fixed_from_iso(*iso.inner());
        Self::islamic_from_fixed(fixed_iso).inner
    }

    fn date_to_iso(&self, date: &Self::DateInner) -> Date<crate::Iso> {
        let fixed_islamic = Self::fixed_from_islamic(*date);
        Iso::iso_from_fixed(fixed_islamic)
    }

    fn months_in_year(&self, date: &Self::DateInner) -> u8 {
        date.0.months_in_year()
    }

    fn days_in_year(&self, date: &Self::DateInner) -> u32 {
        date.0.days_in_year()
    }

    fn day_of_week(&self, date: &Self::DateInner) -> types::IsoWeekday {
        Iso.day_of_week(self.date_to_iso(date).inner())
    }

    fn offset_date(&self, date: &mut Self::DateInner, offset: DateDuration<Self>) {
        date.0.offset_date(offset)
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

    fn debug_name(&self) -> &'static str {
        "IslamicObservational"
    }

    fn year(&self, date: &Self::DateInner) -> types::FormattableYear {
        Self::year_as_islamic(date.0.year)
    }

    fn month(&self, date: &Self::DateInner) -> types::FormattableMonth {
        date.0.solar_month()
    }

    fn day_of_month(&self, date: &Self::DateInner) -> types::DayOfMonth {
        date.0.day_of_month()
    }

    fn day_of_year_info(&self, date: &Self::DateInner) -> types::DayOfYearInfo {
        let prev_year = date.0.year.saturating_sub(1);
        let next_year = date.0.year.saturating_add(1);
        types::DayOfYearInfo {
            day_of_year: date.0.day_of_year(),
            days_in_year: date.0.days_in_year(),
            prev_year: Self::year_as_islamic(prev_year),
            days_in_prev_year: Self::days_in_provided_year(prev_year),
            next_year: Self::year_as_islamic(next_year),
        }
    }

    fn days_in_month(&self, date: &Self::DateInner) -> u8 {
        todo!()
    }
    // TODO: ADD TO ANYCALENDAR
    // fn any_calendar_kind(&self) -> Option<AnyCalendarKind> {
    //     Some(AnyCalendarKind::IslamicObservational)
    // }
}

impl IslamicObservational {
    /// Constructs a new Islamic Observational Calendar
    pub fn new() -> Self {
        Self
    }

    // "Fixed" is a day count representation of calendars staring from Jan 1st of year 1 of the Georgian Calendar.
    // The fixed date algorithms are from
    // Dershowitz, Nachum, and Edward M. Reingold. _Calendrical calculations_. Cambridge University Press, 2008.
    //
    // Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L6904
    fn fixed_from_islamic(i_date: IslamicDateInner) -> RataDie {
        let year = i64::from(i_date.0.year);
        let month = i64::from(i_date.0.month);
        let day = i64::from(i_date.0.day);

        let midmonth = FIXED_ISLAMIC_EPOCH_FRIDAY.to_f64_date()
            + (((year - 1) as f64) * 12.0 + month as f64 - 0.5) * MEAN_SYNODIC_MONTH;

        Astronomical::phasis_on_or_before(RataDie::new(midmonth as i64), CAIRO) + day - 1
    }

    // Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L6920
    #[allow(clippy::unwrap_used)]
    fn islamic_from_fixed(date: RataDie) -> Date<IslamicObservational> {
        let crescent = Astronomical::phasis_on_or_before(date, CAIRO);
        let elapsed_months =
            (libm::round((crescent - FIXED_ISLAMIC_EPOCH_FRIDAY) as f64 / MEAN_SYNODIC_MONTH))
                as i32;
        let year = div_rem_euclid(elapsed_months, 12).0 + 1;
        let month = div_rem_euclid(elapsed_months, 12).1 + 1;
        let day = (date - crescent + 1) as u8;

        Date::try_new_observational_islamic_date(year, month as u8, day).unwrap()
    }

    // pub(crate) fn fixed_from_islamic_integers(year: i32, month: u8, day: u8) -> Option<RataDie> {
    //     Date::try_new_observational_islamic_date(year, month, day)
    //     .ok()
    //     .map(|d| *d.inner())
    //     .map(Self::fixed_from_islamic)
    // }

    fn year_as_islamic(year: i32) -> types::FormattableYear {
        types::FormattableYear {
            era: types::Era(tinystr!(16, "ah")),
            number: year,
            cyclic: None,
            related_iso: None,
        }
    }
}

impl Date<IslamicObservational> {
    /// Construct new Islamic Observational Date.
    ///
    /// Has no negative years, only era is the AH.
    ///
    /// ```rust
    /// use icu::calendar::Date;
    ///
    /// let date_islamic = Date::try_new_observational_islamic_date(1392, 4, 25)
    ///     .expect("Failed to initialize Islamic Date instance.");
    ///
    /// assert_eq!(date_islamic.year().number, 1392);
    /// assert_eq!(date_islamic.month().ordinal, 4);
    /// assert_eq!(date_islamic.day_of_month().0, 25);
    /// ```
    pub fn try_new_observational_islamic_date(
        year: i32,
        month: u8,
        day: u8,
    ) -> Result<Date<IslamicObservational>, CalendarError> {
        ArithmeticDate::new_from_lunar_ordinals(year, month, day)
            .map(IslamicDateInner)
            .map(|inner| Date::from_raw(inner, IslamicObservational))
    }
}

impl DateTime<IslamicObservational> {
    /// Construct a new Islamic Observational datetime from integers.
    ///
    /// ```rust
    /// use icu::calendar::DateTime;
    ///
    /// let datetime_islamic = DateTime::try_new_observational_islamic_datetime(474, 10, 11, 13, 1, 0)
    ///     .expect("Failed to initialize Islamic DateTime instance.");
    ///
    /// assert_eq!(datetime_islamic.date.year().number, 474);
    /// assert_eq!(datetime_islamic.date.month().ordinal, 10);
    /// assert_eq!(datetime_islamic.date.day_of_month().0, 11);
    /// assert_eq!(datetime_islamic.time.hour.number(), 13);
    /// assert_eq!(datetime_islamic.time.minute.number(), 1);
    /// assert_eq!(datetime_islamic.time.second.number(), 0);
    /// ```
    pub fn try_new_observational_islamic_datetime(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Result<DateTime<IslamicObservational>, CalendarError> {
        Ok(DateTime {
            date: Date::try_new_observational_islamic_date(year, month, day)?,
            time: types::Time::try_new(hour, minute, second, 0)?,
        })
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
/// The inner date type used for representing [`Date`]s of [`UmmalQura`]. See [`Date`] and [`UmmalQura`] for more details.
pub struct UmmalQuraDateInner(ArithmeticDate<UmmalQura>);

impl CalendarArithmetic for UmmalQura {
    fn month_days(year: i32, month: u8) -> u8 {
        let midmonth = RataDie::new(libm::floor(
            FIXED_ISLAMIC_EPOCH_FRIDAY.to_f64_date()
                + (((year as f64 - 1.0) * 12.0 + month as f64 - 0.5) * MEAN_SYNODIC_MONTH),
        ) as i64);

        if Self::saudi_criterion(midmonth) {
            30
        } else {
            29
        }
    }

    fn months_for_every_year(_year: i32) -> u8 {
        12
    }

    fn days_in_provided_year(_year: i32) -> u32 {
        355
    }

    // As an observational-lunar calendar, it does not have leap years.
    fn is_leap_year(_year: i32) -> bool {
        false
    }

    fn last_month_day_in_year(year: i32) -> (u8, u8) {
        let days = Self::month_days(year, 12);

        (12, days)
    }
}

impl Calendar for UmmalQura {
    type DateInner = UmmalQuraDateInner;
    fn date_from_codes(
        &self,
        era: types::Era,
        year: i32,
        month_code: types::MonthCode,
        day: u8,
    ) -> Result<Self::DateInner, CalendarError> {
        let year = if era.0 == tinystr!(16, "ah") {
            year
        } else {
            return Err(CalendarError::UnknownEra(era.0, self.debug_name()));
        };

        ArithmeticDate::new_from_solar_codes(self, year, month_code, day).map(UmmalQuraDateInner)
    }

    fn date_from_iso(&self, iso: Date<Iso>) -> Self::DateInner {
        let fixed_iso = Iso::fixed_from_iso(*iso.inner());
        Self::saudi_islamic_from_fixed(fixed_iso).inner
    }

    fn date_to_iso(&self, date: &Self::DateInner) -> Date<Iso> {
        let fixed_islamic = Self::fixed_from_saudi_islamic(*date);
        Iso::iso_from_fixed(fixed_islamic)
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

    fn offset_date(&self, date: &mut Self::DateInner, offset: DateDuration<Self>) {
        date.0.offset_date(offset)
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

    fn debug_name(&self) -> &'static str {
        "Umm-al-Qura Islamic"
    }

    fn year(&self, date: &Self::DateInner) -> types::FormattableYear {
        Self::year_as_islamic(date.0.year)
    }

    fn month(&self, date: &Self::DateInner) -> types::FormattableMonth {
        date.0.solar_month()
    }

    fn day_of_month(&self, date: &Self::DateInner) -> types::DayOfMonth {
        date.0.day_of_month()
    }

    fn day_of_year_info(&self, date: &Self::DateInner) -> types::DayOfYearInfo {
        let prev_year = date.0.year.saturating_sub(1);
        let next_year = date.0.year.saturating_add(1);
        types::DayOfYearInfo {
            day_of_year: date.0.day_of_year(),
            days_in_year: date.0.days_in_year(),
            prev_year: Self::year_as_islamic(prev_year),
            days_in_prev_year: Self::days_in_provided_year(prev_year),
            next_year: Self::year_as_islamic(next_year),
        }
    }

    // TODO: ADD TO ANYCALENDAR
    // fn any_calendar_kind(&self) -> Option<AnyCalendarKind> {
    //     Some(AnyCalendarKind::UmmalQura)
    // }
}

impl Date<UmmalQura> {
    /// Construct new UmmalQura Islamic Date.
    ///
    /// Has no negative years, only era is the AH.
    ///
    /// ```rust
    /// use icu::calendar::Date;
    ///
    /// let date_islamic = Date::try_new_ummalqura_date(1392, 4, 25)
    ///     .expect("Failed to initialize Islamic Date instance.");
    ///
    /// assert_eq!(date_islamic.year().number, 1392);
    /// assert_eq!(date_islamic.month().ordinal, 4);
    /// assert_eq!(date_islamic.day_of_month().0, 25);
    /// ```
    pub fn try_new_ummalqura_date(
        year: i32,
        month: u8,
        day: u8,
    ) -> Result<Date<UmmalQura>, CalendarError> {
        ArithmeticDate::new_from_solar_ordinals(year, month, day)
            .map(UmmalQuraDateInner)
            .map(|inner| Date::from_raw(inner, UmmalQura))
    }
}

impl DateTime<UmmalQura> {
    /// Construct a new UmmalQura datetime from integers.
    ///
    /// ```rust
    /// use icu::calendar::DateTime;
    ///
    /// let datetime_islamic = DateTime::try_new_ummalqura_datetime(474, 10, 11, 13, 1, 0)
    ///     .expect("Failed to initialize Islamic DateTime instance.");
    ///
    /// assert_eq!(datetime_islamic.date.year().number, 474);
    /// assert_eq!(datetime_islamic.date.month().ordinal, 10);
    /// assert_eq!(datetime_islamic.date.day_of_month().0, 11);
    /// assert_eq!(datetime_islamic.time.hour.number(), 13);
    /// assert_eq!(datetime_islamic.time.minute.number(), 1);
    /// assert_eq!(datetime_islamic.time.second.number(), 0);
    /// ```
    pub fn try_new_ummalqura_datetime(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Result<DateTime<UmmalQura>, CalendarError> {
        Ok(DateTime {
            date: Date::try_new_ummalqura_date(year, month, day)?,
            time: types::Time::try_new(hour, minute, second, 0)?,
        })
    }
}

impl UmmalQura {
    /// Constructs a new UmmalQura Islamic Calendar
    pub fn new() -> Self {
        Self
    }
    // Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L6957
    fn saudi_criterion(date: RataDie) -> bool {
        let set = Astronomical::sunset((date - 1).as_moment(), MECCA).unwrap(); // Unwrap used temporarily
        let tee = Location::universal_from_standard(set, MECCA);
        let phase = Astronomical::lunar_phase(tee);

        phase > 0.0
            && phase < 90.0
            && Astronomical::moonlag((date - 1).as_moment(), MECCA).unwrap() > 0.0
    }

    fn saudi_new_month_on_or_before(date: RataDie) -> RataDie {
        let moon =
            libm::floor((Astronomical::lunar_phase_at_or_before(0.0, date.as_moment())).inner());
        let age = moon - date.to_f64_date();
        let tau = if age <= 3.0 && !Self::saudi_criterion(date) {
            moon - 30.0
        } else {
            moon
        };

        next(RataDie::new(tau as i64), Self::saudi_criterion)
    }

    // Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L6996
    fn saudi_islamic_from_fixed(date: RataDie) -> Date<UmmalQura> {
        let crescent = Self::saudi_new_month_on_or_before(date);
        let elapsed_months =
            libm::round((crescent - FIXED_ISLAMIC_EPOCH_FRIDAY) as f64 / MEAN_SYNODIC_MONTH);
        let year = (1.0 + div_rem_euclid_f64(elapsed_months, 12.0).0) as i32;
        let month = (1.0 + div_rem_euclid_f64(elapsed_months, 12.0).1) as u8;
        let day = (1 + (crescent - date)) as u8;

        Date::try_new_ummalqura_date(year, month, day).unwrap()
    }

    // "Fixed" is a day count representation of calendars staring from Jan 1st of year 1 of the Georgian Calendar.
    // The fixed date algorithms are from
    // Dershowitz, Nachum, and Edward M. Reingold. _Calendrical calculations_. Cambridge University Press, 2008.
    //
    // Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L6981
    fn fixed_from_saudi_islamic(date: UmmalQuraDateInner) -> RataDie {
        let year = date.0.year;
        let month = date.0.month;
        let day = date.0.day;

        let midmonth = RataDie::new(libm::floor(
            FIXED_ISLAMIC_EPOCH_FRIDAY.to_f64_date()
                + (((year as f64 - 1.0) * 12.0 + month as f64 - 0.5) * MEAN_SYNODIC_MONTH),
        ) as i64);
        let first_day_of_month = Self::saudi_new_month_on_or_before(midmonth).to_i64_date();

        RataDie::new(first_day_of_month + day as i64 - 1)
    }

    fn year_as_islamic(year: i32) -> types::FormattableYear {
        types::FormattableYear {
            era: types::Era(tinystr!(16, "ah")),
            number: year,
            cyclic: None,
            related_iso: None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug)]
    struct DateCase {
        year: i32,
        month: u8,
        day: u8,
    }

    static TEST_FIXED_DATE: [i64; 33] = [
        -214193, -61387, 25469, 49217, 171307, 210155, 253427, 369740, 400085, 434355, 452605,
        470160, 473837, 507850, 524156, 544676, 567118, 569477, 601716, 613424, 626596, 645554,
        664224, 671401, 694799, 704424, 708842, 709409, 709580, 727274, 728714, 744313, 764652,
    ];

    static ASTRONOMICAL_CASES: [DateCase; 33] = [
        DateCase {
            year: -1245,
            month: 12,
            day: 11,
        },
        DateCase {
            year: -813,
            month: 2,
            day: 25,
        },
        DateCase {
            year: -568,
            month: 4,
            day: 2,
        },
        DateCase {
            year: -501,
            month: 4,
            day: 7,
        },
        DateCase {
            year: -157,
            month: 10,
            day: 18,
        },
        DateCase {
            year: -47,
            month: 6,
            day: 3,
        },
        DateCase {
            year: 75,
            month: 7,
            day: 13,
        },
        DateCase {
            year: 403,
            month: 10,
            day: 5,
        },
        DateCase {
            year: 489,
            month: 5,
            day: 22,
        },
        DateCase {
            year: 586,
            month: 2,
            day: 7,
        },
        DateCase {
            year: 637,
            month: 8,
            day: 7,
        },
        DateCase {
            year: 687,
            month: 2,
            day: 21,
        },
        DateCase {
            year: 697,
            month: 7,
            day: 7,
        },
        DateCase {
            year: 793,
            month: 6,
            day: 30,
        },
        DateCase {
            year: 839,
            month: 7,
            day: 6,
        },
        DateCase {
            year: 897,
            month: 6,
            day: 2,
        },
        DateCase {
            year: 960,
            month: 9,
            day: 30,
        },
        DateCase {
            year: 967,
            month: 5,
            day: 27,
        },
        DateCase {
            year: 1058,
            month: 5,
            day: 18,
        },
        DateCase {
            year: 1091,
            month: 6,
            day: 3,
        },
        DateCase {
            year: 1128,
            month: 8,
            day: 4,
        },
        DateCase {
            year: 1182,
            month: 2,
            day: 4,
        },
        DateCase {
            year: 1234,
            month: 10,
            day: 10,
        },
        DateCase {
            year: 1255,
            month: 1,
            day: 11,
        },
        DateCase {
            year: 1321,
            month: 1,
            day: 20,
        },
        DateCase {
            year: 1348,
            month: 3,
            day: 19,
        },
        DateCase {
            year: 1360,
            month: 9,
            day: 7,
        },
        DateCase {
            year: 1362,
            month: 4,
            day: 14,
        },
        DateCase {
            year: 1362,
            month: 10,
            day: 7,
        },
        DateCase {
            year: 1412,
            month: 9,
            day: 12,
        },
        DateCase {
            year: 1416,
            month: 10,
            day: 5,
        },
        DateCase {
            year: 1460,
            month: 10,
            day: 12,
        },
        DateCase {
            year: 1518,
            month: 3,
            day: 5,
        },
    ];

    #[test]
    fn test_observational_islamic_from_fixed() {
        for (case, f_date) in ASTRONOMICAL_CASES.iter().zip(TEST_FIXED_DATE.iter()) {
            let date =
                Date::try_new_observational_islamic_date(case.year, case.month, case.day).unwrap();
            assert_eq!(
                IslamicObservational::islamic_from_fixed(RataDie::new(*f_date)),
                date,
                "{case:?}"
            );
        }
    }

    #[test]
    fn test_fixed_from_observational_islamic() {
        for (case, f_date) in ASTRONOMICAL_CASES.iter().zip(TEST_FIXED_DATE.iter()) {
            let date = IslamicDateInner(ArithmeticDate::new_unchecked(
                case.year, case.month, case.day,
            ));
            assert_eq!(
                IslamicObservational::fixed_from_islamic(date),
                RataDie::new(*f_date),
                "{case:?}"
            );
        }
    }

    #[test]
    fn test_islamic_epoch() {
        let epoch = FIXED_ISLAMIC_EPOCH_FRIDAY.to_i64_date();
        // Iso year of Islamic Epoch
        let epoch_year_from_fixed = Iso::iso_from_fixed(RataDie::new(epoch)).inner.0.year;
        // 622 is the correct ISO year for the Islamic Epoch
        assert_eq!(epoch_year_from_fixed, 622);
    }
}
