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
use crate::helpers::{self, div_rem_euclid, div_rem_euclid64, div_rem_euclid_f64, next};
use crate::julian::Julian;
use crate::rata_die::RataDie;
use crate::AnyCalendarKind;
use crate::{astronomy::*, Iso};
use crate::{types, Calendar, CalendarError, Date, DateDuration, DateDurationUnit, DateTime};
use ::tinystr::tinystr;

/// Islamic Observational Calendar (Default)
///
/// # Era codes
///
/// This calendar supports a single era code, Anno Mundi, with code `"ah"`
///
/// # Month codes
///
/// This calendar is a pure lunar calendar with no leap months. It uses month codes
/// `"M01" - "M12"`.
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord)]
#[allow(clippy::exhaustive_structs)]
pub struct IslamicObservational;

/// Civil / Arithmetical Islamic Calendar (Used for administrative purposes)
///
/// # Era codes
///
/// This calendar supports a single era code, Anno Mundi, with code `"ah"`
///
/// # Month codes
///
/// This calendar is a pure lunar calendar with no leap months. It uses month codes
/// `"M01" - "M12"`.
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord)]
#[allow(clippy::exhaustive_structs)]
pub struct IslamicCivil;

/// Umm al-Qura Hijri Calendar (Used in Saudi Arabia)
///
/// # Era codes
///
/// This calendar supports a single era code, Anno Mundi, with code `"ah"`
///
/// # Month codes
///
/// This calendar is a pure lunar calendar with no leap months. It uses month codes
/// `"M01" - "M12"`.
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord)]
#[allow(clippy::exhaustive_structs)]
pub struct IslamicUmmAlQura;

/// A Tabular version of the Arithmetical Islamic Calendar
///
/// # Era codes
///
/// This calendar supports a single era code, Anno Mundi, with code `"ah"`
///
/// # Month codes
///
/// This calendar is a pure lunar calendar with no leap months. It uses month codes
/// `"M01" - "M12"`.
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord)]
#[allow(clippy::exhaustive_structs)]
pub struct IslamicTabular;

// Different islamic calendars use different epochs (Thursday vs Friday) due to disagreement on the exact date of Mohammed's migration to Mecca.
// Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L2066
const FIXED_ISLAMIC_EPOCH_FRIDAY: RataDie = Julian::fixed_from_julian_integers(622, 7, 16);
const FIXED_ISLAMIC_EPOCH_THURSDAY: RataDie = Julian::fixed_from_julian_integers(622, 7, 15);

// Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L6898
const CAIRO: Location = Location {
    latitude: 30.1,
    longitude: 31.3,
    elevation: 200.0,
    zone: (1_f64 / 12_f64),
};

/// The inner date type used for representing [`Date`]s of [`IslamicObservational`]. See [`Date`] and [`IslamicObservational`] for more details.

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct IslamicDateInner(ArithmeticDate<IslamicObservational>);

impl CalendarArithmetic for IslamicObservational {
    fn month_days(year: i32, month: u8) -> u8 {
        let midmonth = FIXED_ISLAMIC_EPOCH_FRIDAY.to_f64_date()
            + (((year - 1) as f64) * 12.0 + month as f64 - 0.5) * MEAN_SYNODIC_MONTH;

        let lunar_phase: f64 =
            Astronomical::calculate_lunar_phase_at_or_before(RataDie::new(midmonth as i64));
        let f_date = Astronomical::phasis_on_or_before(
            RataDie::new(midmonth as i64),
            CAIRO,
            Some(lunar_phase),
        );

        Astronomical::month_length(f_date, CAIRO)
    }

    fn months_for_every_year(_year: i32) -> u8 {
        12
    }

    fn days_in_provided_year(year: i32) -> u16 {
        (1..=12)
            .map(|month| IslamicObservational::month_days(year, month) as u16)
            .sum()
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
        let year = if era.0 == tinystr!(16, "islamic") || era.0 == tinystr!(16, "ah") {
            year
        } else {
            return Err(CalendarError::UnknownEra(era.0, self.debug_name()));
        };

        ArithmeticDate::new_from_codes(self, year, month_code, day).map(IslamicDateInner)
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

    fn days_in_year(&self, date: &Self::DateInner) -> u16 {
        date.0.days_in_year()
    }

    fn days_in_month(&self, date: &Self::DateInner) -> u8 {
        date.0.days_in_month()
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
        "Islamic (observational)"
    }

    fn year(&self, date: &Self::DateInner) -> types::FormattableYear {
        Self::year_as_islamic(date.0.year)
    }

    fn month(&self, date: &Self::DateInner) -> types::FormattableMonth {
        date.0.month()
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
    // Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L6904
    fn fixed_from_islamic(i_date: IslamicDateInner) -> RataDie {
        let year = i64::from(i_date.0.year);
        let month = i64::from(i_date.0.month);
        let day = i64::from(i_date.0.day);
        let midmonth = FIXED_ISLAMIC_EPOCH_FRIDAY.to_f64_date()
            + (((year - 1) as f64) * 12.0 + month as f64 - 0.5) * MEAN_SYNODIC_MONTH;
        let lunar_phase =
            Astronomical::calculate_lunar_phase_at_or_before(RataDie::new(midmonth as i64));
        Astronomical::phasis_on_or_before(RataDie::new(midmonth as i64), CAIRO, Some(lunar_phase))
            + day
            - 1
    }

    fn islamic_from_fixed(date: RataDie) -> Date<IslamicObservational> {
        let lunar_phase = Astronomical::calculate_lunar_phase_at_or_before(date);
        let crescent = Astronomical::phasis_on_or_before(date, CAIRO, Some(lunar_phase));
        let elapsed_months =
            (libm::round((crescent - FIXED_ISLAMIC_EPOCH_FRIDAY) as f64 / MEAN_SYNODIC_MONTH))
                as i32;
        let year = div_rem_euclid(elapsed_months, 12).0 + 1;
        let month = div_rem_euclid(elapsed_months, 12).1 + 1;
        let day = (date - crescent + 1) as u8;

        debug_assert!(Date::try_new_observational_islamic_date(year, month as u8, day).is_ok());
        Date::from_raw(
            IslamicDateInner(ArithmeticDate::new_unchecked(year, month as u8, day)),
            IslamicObservational,
        )
    }

    // pub(crate) fn fixed_from_islamic_integers(year: i32, month: u8, day: u8) -> Option<RataDie> {
    //     Date::try_new_observational_islamic_date(year, month, day)
    //     .ok()
    //     .map(|d| *d.inner())
    //     .map(Self::fixed_from_islamic)
    // }

    fn year_as_islamic(year: i32) -> types::FormattableYear {
        types::FormattableYear {
            era: types::Era(tinystr!(16, "islamic")),
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
/// The inner date type used for representing [`Date`]s of [`IslamicUmmAlQura`]. See [`Date`] and [`IslamicUmmAlQura`] for more details.
pub struct IslamicUmmAlQuraDateInner(ArithmeticDate<IslamicUmmAlQura>);

impl CalendarArithmetic for IslamicUmmAlQura {
    fn month_days(year: i32, month: u8) -> u8 {
        // We cannot use month_days from the book here, that is for the observational calendar
        //
        // Instead we subtract the two new months calculated using the saudi criterion
        let midmonth = types::Moment::new(
            FIXED_ISLAMIC_EPOCH_FRIDAY.to_f64_date()
                + (((year - 1) as f64) * 12.0 + month as f64 - 0.5) * MEAN_SYNODIC_MONTH,
        );
        let midmonth_next = midmonth + MEAN_SYNODIC_MONTH;

        let month_start = Self::saudi_new_month_on_or_before(midmonth.as_rata_die());
        let next_month_start = Self::saudi_new_month_on_or_before(midmonth_next.as_rata_die());

        let diff = next_month_start - month_start;
        debug_assert!(
            diff <= 30,
            "umm-al-qura months must not be more than 30 days"
        );
        u8::try_from(diff).unwrap_or(30)
    }

    fn months_for_every_year(_year: i32) -> u8 {
        12
    }

    fn days_in_provided_year(year: i32) -> u16 {
        (1..=12)
            .map(|month| IslamicUmmAlQura::month_days(year, month) as u16)
            .sum()
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

impl Calendar for IslamicUmmAlQura {
    type DateInner = IslamicUmmAlQuraDateInner;
    fn date_from_codes(
        &self,
        era: types::Era,
        year: i32,
        month_code: types::MonthCode,
        day: u8,
    ) -> Result<Self::DateInner, CalendarError> {
        let year = if era.0 == tinystr!(16, "islamic-umalqura")
            || era.0 == tinystr!(16, "islamic")
            || era.0 == tinystr!(16, "ah")
        {
            year
        } else {
            return Err(CalendarError::UnknownEra(era.0, self.debug_name()));
        };

        ArithmeticDate::new_from_codes(self, year, month_code, day).map(IslamicUmmAlQuraDateInner)
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

    fn days_in_year(&self, date: &Self::DateInner) -> u16 {
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
        "Islamic (Umm al-Qura)"
    }

    fn year(&self, date: &Self::DateInner) -> types::FormattableYear {
        Self::year_as_islamic(date.0.year)
    }

    fn month(&self, date: &Self::DateInner) -> types::FormattableMonth {
        date.0.month()
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

    fn any_calendar_kind(&self) -> Option<AnyCalendarKind> {
        Some(AnyCalendarKind::IslamicUmmAlQura)
    }
}

impl Date<IslamicUmmAlQura> {
    /// Construct new Islamic Umm al-Qura Date.
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
    ) -> Result<Date<IslamicUmmAlQura>, CalendarError> {
        ArithmeticDate::new_from_lunar_ordinals(year, month, day)
            .map(IslamicUmmAlQuraDateInner)
            .map(|inner| Date::from_raw(inner, IslamicUmmAlQura))
    }
}

impl DateTime<IslamicUmmAlQura> {
    /// Construct a new Islamic Umm al-Qura datetime from integers.
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
    ) -> Result<DateTime<IslamicUmmAlQura>, CalendarError> {
        Ok(DateTime {
            date: Date::try_new_ummalqura_date(year, month, day)?,
            time: types::Time::try_new(hour, minute, second, 0)?,
        })
    }
}

impl IslamicUmmAlQura {
    /// Constructs a new Islamic Umm al-Qura Calendar
    pub fn new() -> Self {
        Self
    }

    // Saudi visibility criterion on eve of fixed date in Mecca.
    // The start of the new month only happens if both of these criterias are met: The moon is a waxing crescent at sunset of the previous day
    // and the moon sets after the sun on that same evening.
    // Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L6957
    fn saudi_criterion(date: RataDie) -> Option<bool> {
        let sunset = Astronomical::sunset((date - 1).as_moment(), MECCA)?;
        let tee = Location::universal_from_standard(sunset, MECCA);
        let phase = Astronomical::lunar_phase(tee, Astronomical::julian_centuries(tee));
        let moonlag = Astronomical::moonlag((date - 1).as_moment(), MECCA)?;

        Some(phase > 0.0 && phase < 90.0 && moonlag > 0.0)
    }

    pub(crate) fn adjusted_saudi_criterion(date: RataDie) -> bool {
        if let Some(x) = Self::saudi_criterion(date) {
            x
        } else {
            false
        }
    }

    // Closest fixed date on or before date when Saudi visibility criterion is held.
    // Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L6966
    fn saudi_new_month_on_or_before(date: RataDie) -> RataDie {
        let last_new_moon =
            libm::floor((Astronomical::lunar_phase_at_or_before(0.0, date.as_moment())).inner()); // Gets the R.D Date of the prior new moon
        let age = date.to_f64_date() - last_new_moon;
        // Explanation of why the value 3.0 is chosen: https://github.com/unicode-org/icu4x/pull/3673/files#r1267460916
        let tau = if age <= 3.0 && !Self::adjusted_saudi_criterion(date) {
            // Checks if the criterion is not yet visibile on the evening of date
            last_new_moon - 30.0 // Goes back a month
        } else {
            last_new_moon
        };

        next(RataDie::new(tau as i64), Self::adjusted_saudi_criterion) // Loop that increments the day and checks if the criterion is now visibile
    }

    // Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L6996
    fn saudi_islamic_from_fixed(date: RataDie) -> Date<IslamicUmmAlQura> {
        let crescent = Self::saudi_new_month_on_or_before(date);
        let elapsed_months =
            libm::round((crescent - FIXED_ISLAMIC_EPOCH_FRIDAY) as f64 / MEAN_SYNODIC_MONTH) as i64;
        let year = helpers::i64_to_saturated_i32((div_rem_euclid64(elapsed_months, 12).0) + 1);
        let month = ((div_rem_euclid64(elapsed_months, 12).1) + 1) as u8;
        let day = ((date - crescent) + 1) as u8;

        debug_assert!(Date::try_new_ummalqura_date(year, month, day).is_ok());
        Date::from_raw(
            IslamicUmmAlQuraDateInner(ArithmeticDate::new_unchecked(year, month, day)),
            IslamicUmmAlQura,
        )
    }

    // "Fixed" is a day count representation of calendars staring from Jan 1st of year 1 of the Georgian Calendar.
    // The fixed date algorithms are from
    // Dershowitz, Nachum, and Edward M. Reingold. _Calendrical calculations_. Cambridge University Press, 2008.
    //
    // Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L6981
    fn fixed_from_saudi_islamic(date: IslamicUmmAlQuraDateInner) -> RataDie {
        let year = date.0.year;
        let month = date.0.month;
        let day = date.0.day;

        let midmonth = RataDie::new(
            FIXED_ISLAMIC_EPOCH_FRIDAY.to_i64_date()
                + libm::floor(
                    ((year as f64 - 1.0) * 12.0 + month as f64 - 0.5) * MEAN_SYNODIC_MONTH,
                ) as i64,
        );
        let first_day_of_month = Self::saudi_new_month_on_or_before(midmonth).to_i64_date();

        RataDie::new(first_day_of_month + day as i64 - 1)
    }

    fn year_as_islamic(year: i32) -> types::FormattableYear {
        types::FormattableYear {
            era: types::Era(tinystr!(16, "islamic")),
            number: year,
            cyclic: None,
            related_iso: None,
        }
    }
}

/// The inner date type used for representing [`Date`]s of [`IslamicCivil`]. See [`Date`] and [`IslamicCivil`] for more details.

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct IslamicCivilDateInner(ArithmeticDate<IslamicCivil>);

impl CalendarArithmetic for IslamicCivil {
    fn month_days(year: i32, month: u8) -> u8 {
        match month {
            1 | 3 | 5 | 7 | 9 | 11 => 30,
            2 | 4 | 6 | 8 | 10 => 29,
            12 if Self::is_leap_year(year) => 30,
            12 => 29,
            _ => 0,
        }
    }

    fn months_for_every_year(_year: i32) -> u8 {
        12
    }

    fn days_in_provided_year(year: i32) -> u16 {
        if Self::is_leap_year(year) {
            355
        } else {
            354
        }
    }

    fn is_leap_year(year: i32) -> bool {
        div_rem_euclid(14 + 11 * year, 30).1 < 11
    }

    fn last_month_day_in_year(year: i32) -> (u8, u8) {
        if Self::is_leap_year(year) {
            (12, 30)
        } else {
            (12, 29)
        }
    }
}

impl Calendar for IslamicCivil {
    type DateInner = IslamicCivilDateInner;

    fn date_from_codes(
        &self,
        era: types::Era,
        year: i32,
        month_code: types::MonthCode,
        day: u8,
    ) -> Result<Self::DateInner, CalendarError> {
        let year = if era.0 == tinystr!(16, "islamic-civil")
            || era.0 == tinystr!(16, "islamicc")
            || era.0 == tinystr!(16, "islamic")
            || era.0 == tinystr!(16, "ah")
        {
            // TODO: Check name and alias
            year
        } else {
            return Err(CalendarError::UnknownEra(era.0, self.debug_name()));
        };

        ArithmeticDate::new_from_codes(self, year, month_code, day).map(IslamicCivilDateInner)
    }

    fn date_from_iso(&self, iso: Date<Iso>) -> Self::DateInner {
        let fixed_iso = Iso::fixed_from_iso(*iso.inner());
        Self::islamic_from_fixed(fixed_iso).inner
    }

    fn date_to_iso(&self, date: &Self::DateInner) -> Date<Iso> {
        let fixed_islamic = Self::fixed_from_islamic(*date);
        Iso::iso_from_fixed(fixed_islamic)
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
        "Islamic (civil)"
    }

    fn year(&self, date: &Self::DateInner) -> types::FormattableYear {
        Self::year_as_islamic(date.0.year)
    }

    fn month(&self, date: &Self::DateInner) -> types::FormattableMonth {
        date.0.month()
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
}

impl IslamicCivil {
    /// Constructs a new Islamic Civil Calendar
    pub fn new() -> Self {
        Self
    }

    // "Fixed" is a day count representation of calendars staring from Jan 1st of year 1 of the Georgian Calendar.
    // The fixed date algorithms are from
    // Dershowitz, Nachum, and Edward M. Reingold. _Calendrical calculations_. Cambridge University Press, 2008.
    //
    // Lisp code reference:https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L2076
    fn fixed_from_islamic(i_date: IslamicCivilDateInner) -> RataDie {
        let year = i64::from(i_date.0.year);
        let month = i64::from(i_date.0.month);
        let day = i64::from(i_date.0.day);

        RataDie::new(
            (FIXED_ISLAMIC_EPOCH_FRIDAY.to_i64_date() - 1)
                + (year - 1) * 354
                + div_rem_euclid_f64(3.0 + year as f64 * 11.0, 30.0).0 as i64
                + 29 * (month - 1)
                + div_rem_euclid_f64(month as f64, 2.0).0 as i64
                + day,
        )
    }
    // Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L2090
    fn islamic_from_fixed(date: RataDie) -> Date<IslamicCivil> {
        let year = helpers::i64_to_saturated_i32(
            div_rem_euclid64(((date - FIXED_ISLAMIC_EPOCH_FRIDAY) * 30) + 10646, 10631).0,
        );
        let prior_days = date.to_f64_date()
            - ((Self::fixed_from_islamic(IslamicCivilDateInner(
                ArithmeticDate::new_unchecked(year, 1, 1), // Unchecked ok because of hardcoded values,
            )))
            .to_f64_date());
        debug_assert!(prior_days <= 354.);
        let month = div_rem_euclid_f64((prior_days * 11.0) + 330.0, 325.0).0 as u8; // Prior days is maximum 354 (when year length is 355), making the value always less than 12
        debug_assert!(month <= 12);
        let day = (date.to_f64_date()
            - (Self::fixed_from_islamic(IslamicCivilDateInner(
                ArithmeticDate::new_unchecked(year, month, 1), // Unchecked is ok because of hardcoded date and month being in range
            ))
            .to_f64_date())
            + 1.0) as u8; // The value will always be number between 1-30 because of the difference between the date and lunar ordinals function.

        debug_assert!(Date::try_new_islamic_civil_date(year, month, day).is_ok());
        Date::from_raw(
            IslamicCivilDateInner(ArithmeticDate::new_unchecked(year, month, day)),
            IslamicCivil,
        )
    }

    fn year_as_islamic(year: i32) -> types::FormattableYear {
        types::FormattableYear {
            era: types::Era(tinystr!(16, "islamic")),
            number: year,
            cyclic: None,
            related_iso: None,
        }
    }
}

impl Date<IslamicCivil> {
    /// Construct new Civil Islamic Date.
    ///
    /// Has no negative years, only era is the AH.
    ///
    /// ```rust
    /// use icu::calendar::Date;
    ///
    /// let date_islamic = Date::try_new_islamic_civil_date(1392, 4, 25)
    ///     .expect("Failed to initialize Islamic Date instance.");
    ///
    /// assert_eq!(date_islamic.year().number, 1392);
    /// assert_eq!(date_islamic.month().ordinal, 4);
    /// assert_eq!(date_islamic.day_of_month().0, 25);
    /// ```
    pub fn try_new_islamic_civil_date(
        year: i32,
        month: u8,
        day: u8,
    ) -> Result<Date<IslamicCivil>, CalendarError> {
        ArithmeticDate::new_from_lunar_ordinals(year, month, day)
            .map(IslamicCivilDateInner)
            .map(|inner| Date::from_raw(inner, IslamicCivil))
    }
}

impl DateTime<IslamicCivil> {
    /// Construct a new Civil Islamic datetime from integers.
    ///
    /// ```rust
    /// use icu::calendar::DateTime;
    ///
    /// let datetime_islamic = DateTime::try_new_islamic_civil_datetime(474, 10, 11, 13, 1, 0)
    ///     .expect("Failed to initialize Islamic DateTime instance.");
    ///
    /// assert_eq!(datetime_islamic.date.year().number, 474);
    /// assert_eq!(datetime_islamic.date.month().ordinal, 10);
    /// assert_eq!(datetime_islamic.date.day_of_month().0, 11);
    /// assert_eq!(datetime_islamic.time.hour.number(), 13);
    /// assert_eq!(datetime_islamic.time.minute.number(), 1);
    /// assert_eq!(datetime_islamic.time.second.number(), 0);
    /// ```
    pub fn try_new_islamic_civil_datetime(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Result<DateTime<IslamicCivil>, CalendarError> {
        Ok(DateTime {
            date: Date::try_new_islamic_civil_date(year, month, day)?,
            time: types::Time::try_new(hour, minute, second, 0)?,
        })
    }
}

/// The inner date type used for representing [`Date`]s of [`IslamicTabular`]. See [`Date`] and [`IslamicTabular`] for more details.

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct IslamicTabularDateInner(ArithmeticDate<IslamicTabular>);

impl CalendarArithmetic for IslamicTabular {
    fn month_days(year: i32, month: u8) -> u8 {
        match month {
            1 | 3 | 5 | 7 | 9 | 11 => 30,
            2 | 4 | 6 | 8 | 10 => 29,
            12 if Self::is_leap_year(year) => 30,
            12 => 29,
            _ => 0,
        }
    }

    fn months_for_every_year(_year: i32) -> u8 {
        12
    }

    fn days_in_provided_year(year: i32) -> u16 {
        if Self::is_leap_year(year) {
            355
        } else {
            354
        }
    }

    fn is_leap_year(year: i32) -> bool {
        div_rem_euclid(14 + 11 * year, 30).1 < 11
    }

    fn last_month_day_in_year(year: i32) -> (u8, u8) {
        if Self::is_leap_year(year) {
            (12, 30)
        } else {
            (12, 29)
        }
    }
}

impl Calendar for IslamicTabular {
    type DateInner = IslamicTabularDateInner;

    fn date_from_codes(
        &self,
        era: types::Era,
        year: i32,
        month_code: types::MonthCode,
        day: u8,
    ) -> Result<Self::DateInner, CalendarError> {
        let year = if era.0 == tinystr!(16, "islamic-tbla")
            || era.0 == tinystr!(16, "islamic")
            || era.0 == tinystr!(16, "ah")
        {
            year
        } else {
            return Err(CalendarError::UnknownEra(era.0, self.debug_name()));
        };

        ArithmeticDate::new_from_codes(self, year, month_code, day).map(IslamicTabularDateInner)
    }

    fn date_from_iso(&self, iso: Date<Iso>) -> Self::DateInner {
        let fixed_iso = Iso::fixed_from_iso(*iso.inner());
        Self::islamic_from_fixed(fixed_iso).inner
    }

    fn date_to_iso(&self, date: &Self::DateInner) -> Date<Iso> {
        let fixed_islamic = Self::fixed_from_islamic(*date);
        Iso::iso_from_fixed(fixed_islamic)
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
        "Islamic (tabular)"
    }

    fn year(&self, date: &Self::DateInner) -> types::FormattableYear {
        Self::year_as_islamic(date.0.year)
    }

    fn month(&self, date: &Self::DateInner) -> types::FormattableMonth {
        date.0.month()
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
}

impl IslamicTabular {
    /// Constructs a new Islamic Tabular Calendar
    pub fn new() -> Self {
        Self
    }

    // "Fixed" is a day count representation of calendars staring from Jan 1st of year 1 of the Georgian Calendar.
    // The fixed date algorithms are from
    // Dershowitz, Nachum, and Edward M. Reingold. _Calendrical calculations_. Cambridge University Press, 2008.
    //
    // Lisp code reference:https: //github.com/EdReingold/calendar-code2/blob/main/calendar.l#L2076
    fn fixed_from_islamic(i_date: IslamicTabularDateInner) -> RataDie {
        let year = i64::from(i_date.0.year);
        let month = i64::from(i_date.0.month);
        let day = i64::from(i_date.0.day);

        RataDie::new(
            (FIXED_ISLAMIC_EPOCH_THURSDAY.to_i64_date() - 1)
                + (year - 1) * 354
                + div_rem_euclid_f64(3.0 + year as f64 * 11.0, 30.0).0 as i64
                + 29 * (month - 1)
                + div_rem_euclid_f64(month as f64, 2.0).0 as i64
                + day,
        )
    }
    // Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L2090
    fn islamic_from_fixed(date: RataDie) -> Date<IslamicTabular> {
        let year = helpers::i64_to_saturated_i32(
            div_rem_euclid64(((date - FIXED_ISLAMIC_EPOCH_THURSDAY) * 30) + 10646, 10631).0,
        );
        let prior_days = date.to_f64_date()
            - ((Self::fixed_from_islamic(IslamicTabularDateInner(
                ArithmeticDate::new_unchecked(year, 1, 1), // Unchecked ok to hardcoded values,
            )))
            .to_f64_date());
        debug_assert!(prior_days <= 354.);
        let month = div_rem_euclid_f64((prior_days * 11.0) + 330.0, 325.0).0 as u8; // Prior days is at most 354 (with year length 355), making the value 12 or less and within bounds of u8
        debug_assert!(month <= 12);
        let day = (date.to_f64_date()
            - (Self::fixed_from_islamic(IslamicTabularDateInner(
                ArithmeticDate::new_unchecked(year, month, 1), // Unchecked ok due to hardcoded day value and month being in range,
            ))
            .to_f64_date())
            + 1.0) as u8; // The value will always be number between 1-30 because of the difference between the date and lunar ordinals function.

        debug_assert!(Date::try_new_islamic_tabular_date(year, month, day).is_ok());
        Date::from_raw(
            IslamicTabularDateInner(ArithmeticDate::new_unchecked(year, month, day)),
            IslamicTabular,
        )
    }

    fn year_as_islamic(year: i32) -> types::FormattableYear {
        types::FormattableYear {
            era: types::Era(tinystr!(16, "islamic")),
            number: year,
            cyclic: None,
            related_iso: None,
        }
    }
}

impl Date<IslamicTabular> {
    /// Construct new Tabular Islamic Date.
    ///
    /// Has no negative years, only era is the AH.
    ///
    /// ```rust
    /// use icu::calendar::Date;
    ///
    /// let date_islamic = Date::try_new_islamic_tabular_date(1392, 4, 25)
    ///     .expect("Failed to initialize Islamic Date instance.");
    ///
    /// assert_eq!(date_islamic.year().number, 1392);
    /// assert_eq!(date_islamic.month().ordinal, 4);
    /// assert_eq!(date_islamic.day_of_month().0, 25);
    /// ```
    pub fn try_new_islamic_tabular_date(
        year: i32,
        month: u8,
        day: u8,
    ) -> Result<Date<IslamicTabular>, CalendarError> {
        ArithmeticDate::new_from_lunar_ordinals(year, month, day)
            .map(IslamicTabularDateInner)
            .map(|inner| Date::from_raw(inner, IslamicTabular))
    }
}

impl DateTime<IslamicTabular> {
    /// Construct a new Tabular Islamic datetime from integers.
    ///
    /// ```rust
    /// use icu::calendar::DateTime;
    ///
    /// let datetime_islamic = DateTime::try_new_islamic_tabular_datetime(474, 10, 11, 13, 1, 0)
    ///     .expect("Failed to initialize Islamic DateTime instance.");
    ///
    /// assert_eq!(datetime_islamic.date.year().number, 474);
    /// assert_eq!(datetime_islamic.date.month().ordinal, 10);
    /// assert_eq!(datetime_islamic.date.day_of_month().0, 11);
    /// assert_eq!(datetime_islamic.time.hour.number(), 13);
    /// assert_eq!(datetime_islamic.time.minute.number(), 1);
    /// assert_eq!(datetime_islamic.time.second.number(), 0);
    /// ```
    pub fn try_new_islamic_tabular_datetime(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Result<DateTime<IslamicTabular>, CalendarError> {
        Ok(DateTime {
            date: Date::try_new_islamic_tabular_date(year, month, day)?,
            time: types::Time::try_new(hour, minute, second, 0)?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const START_YEAR: i32 = -1245;
    const END_YEAR: i32 = 1518;

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
    // Removed: 601716 and 727274 fixed dates
    static TEST_FIXED_DATE_UMMALQURA: [i64; 31] = [
        -214193, -61387, 25469, 49217, 171307, 210155, 253427, 369740, 400085, 434355, 452605,
        470160, 473837, 507850, 524156, 544676, 567118, 569477, 613424, 626596, 645554, 664224,
        671401, 694799, 704424, 708842, 709409, 709580, 728714, 744313, 764652,
    ];
    // Values from lisp code
    static SAUDI_CRITERION_EXPECTED: [bool; 33] = [
        false, false, true, false, false, true, false, true, false, false, true, false, false,
        true, true, true, true, false, false, true, true, true, false, false, false, false, false,
        false, true, false, true, false, true,
    ];
    // Values from lisp code, removed two expected months.
    static SAUDI_NEW_MONTH_OR_BEFORE_EXPECTED: [f64; 31] = [
        -214203.0, -61412.0, 25467.0, 49210.0, 171290.0, 210152.0, 253414.0, 369735.0, 400063.0,
        434348.0, 452598.0, 470139.0, 473830.0, 507850.0, 524150.0, 544674.0, 567118.0, 569450.0,
        613421.0, 626592.0, 645551.0, 664214.0, 671391.0, 694779.0, 704405.0, 708835.0, 709396.0,
        709573.0, 728709.0, 744301.0, 764647.0,
    ];

    static UMMALQURA_DATE_EXPECTED: [DateCase; 31] = [
        DateCase {
            year: -1245,
            month: 12,
            day: 11,
        },
        DateCase {
            year: -813,
            month: 2,
            day: 26,
        },
        DateCase {
            year: -568,
            month: 4,
            day: 3,
        },
        DateCase {
            year: -501,
            month: 4,
            day: 8,
        },
        DateCase {
            year: -157,
            month: 10,
            day: 18,
        },
        DateCase {
            year: -47,
            month: 6,
            day: 4,
        },
        DateCase {
            year: 75,
            month: 7,
            day: 14,
        },
        DateCase {
            year: 403,
            month: 10,
            day: 6,
        },
        DateCase {
            year: 489,
            month: 5,
            day: 23,
        },
        DateCase {
            year: 586,
            month: 2,
            day: 8,
        },
        DateCase {
            year: 637,
            month: 8,
            day: 8,
        },
        DateCase {
            year: 687,
            month: 2,
            day: 22,
        },
        DateCase {
            year: 697,
            month: 7,
            day: 8,
        },
        DateCase {
            year: 793,
            month: 7,
            day: 1,
        },
        DateCase {
            year: 839,
            month: 7,
            day: 7,
        },
        DateCase {
            year: 897,
            month: 6,
            day: 3,
        },
        DateCase {
            year: 960,
            month: 10,
            day: 1,
        },
        DateCase {
            year: 967,
            month: 5,
            day: 28,
        },
        DateCase {
            year: 1091,
            month: 6,
            day: 4,
        },
        DateCase {
            year: 1128,
            month: 8,
            day: 5,
        },
        DateCase {
            year: 1182,
            month: 2,
            day: 4,
        },
        DateCase {
            year: 1234,
            month: 10,
            day: 11,
        },
        DateCase {
            year: 1255,
            month: 1,
            day: 11,
        },
        DateCase {
            year: 1321,
            month: 1,
            day: 21,
        },
        DateCase {
            year: 1348,
            month: 3,
            day: 20,
        },
        DateCase {
            year: 1360,
            month: 9,
            day: 8,
        },
        DateCase {
            year: 1362,
            month: 4,
            day: 14,
        },
        DateCase {
            year: 1362,
            month: 10,
            day: 8,
        },
        DateCase {
            year: 1416,
            month: 10,
            day: 6,
        },
        DateCase {
            year: 1460,
            month: 10,
            day: 13,
        },
        DateCase {
            year: 1518,
            month: 3,
            day: 6,
        },
    ];

    static OBSERVATIONAL_CASES: [DateCase; 33] = [
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

    static ARITHMETIC_CASES: [DateCase; 33] = [
        DateCase {
            year: -1245,
            month: 12,
            day: 9,
        },
        DateCase {
            year: -813,
            month: 2,
            day: 23,
        },
        DateCase {
            year: -568,
            month: 4,
            day: 1,
        },
        DateCase {
            year: -501,
            month: 4,
            day: 6,
        },
        DateCase {
            year: -157,
            month: 10,
            day: 17,
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
            day: 20,
        },
        DateCase {
            year: 697,
            month: 7,
            day: 7,
        },
        DateCase {
            year: 793,
            month: 7,
            day: 1,
        },
        DateCase {
            year: 839,
            month: 7,
            day: 6,
        },
        DateCase {
            year: 897,
            month: 6,
            day: 1,
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
            day: 2,
        },
        DateCase {
            year: 1128,
            month: 8,
            day: 4,
        },
        DateCase {
            year: 1182,
            month: 2,
            day: 3,
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
            day: 21,
        },
        DateCase {
            year: 1348,
            month: 3,
            day: 19,
        },
        DateCase {
            year: 1360,
            month: 9,
            day: 8,
        },
        DateCase {
            year: 1362,
            month: 4,
            day: 13,
        },
        DateCase {
            year: 1362,
            month: 10,
            day: 7,
        },
        DateCase {
            year: 1412,
            month: 9,
            day: 13,
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

    static TABULAR_CASES: [DateCase; 33] = [
        DateCase {
            year: -1245,
            month: 12,
            day: 10,
        },
        DateCase {
            year: -813,
            month: 2,
            day: 24,
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
            day: 4,
        },
        DateCase {
            year: 75,
            month: 7,
            day: 14,
        },
        DateCase {
            year: 403,
            month: 10,
            day: 6,
        },
        DateCase {
            year: 489,
            month: 5,
            day: 23,
        },
        DateCase {
            year: 586,
            month: 2,
            day: 8,
        },
        DateCase {
            year: 637,
            month: 8,
            day: 8,
        },
        DateCase {
            year: 687,
            month: 2,
            day: 21,
        },
        DateCase {
            year: 697,
            month: 7,
            day: 8,
        },
        DateCase {
            year: 793,
            month: 7,
            day: 2,
        },
        DateCase {
            year: 839,
            month: 7,
            day: 7,
        },
        DateCase {
            year: 897,
            month: 6,
            day: 2,
        },
        DateCase {
            year: 960,
            month: 10,
            day: 1,
        },
        DateCase {
            year: 967,
            month: 5,
            day: 28,
        },
        DateCase {
            year: 1058,
            month: 5,
            day: 19,
        },
        DateCase {
            year: 1091,
            month: 6,
            day: 3,
        },
        DateCase {
            year: 1128,
            month: 8,
            day: 5,
        },
        DateCase {
            year: 1182,
            month: 2,
            day: 4,
        },
        DateCase {
            year: 1234,
            month: 10,
            day: 11,
        },
        DateCase {
            year: 1255,
            month: 1,
            day: 12,
        },
        DateCase {
            year: 1321,
            month: 1,
            day: 22,
        },
        DateCase {
            year: 1348,
            month: 3,
            day: 20,
        },
        DateCase {
            year: 1360,
            month: 9,
            day: 9,
        },
        DateCase {
            year: 1362,
            month: 4,
            day: 14,
        },
        DateCase {
            year: 1362,
            month: 10,
            day: 8,
        },
        DateCase {
            year: 1412,
            month: 9,
            day: 14,
        },
        DateCase {
            year: 1416,
            month: 10,
            day: 6,
        },
        DateCase {
            year: 1460,
            month: 10,
            day: 13,
        },
        DateCase {
            year: 1518,
            month: 3,
            day: 6,
        },
    ];

    #[test]
    fn test_observational_islamic_from_fixed() {
        for (case, f_date) in OBSERVATIONAL_CASES.iter().zip(TEST_FIXED_DATE.iter()) {
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
        for (case, f_date) in OBSERVATIONAL_CASES.iter().zip(TEST_FIXED_DATE.iter()) {
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
    fn test_islamic_epoch_friday() {
        let epoch = FIXED_ISLAMIC_EPOCH_FRIDAY.to_i64_date();
        // Iso year of Islamic Epoch
        let epoch_year_from_fixed = Iso::iso_from_fixed(RataDie::new(epoch)).inner.0.year;
        // 622 is the correct ISO year for the Islamic Epoch
        assert_eq!(epoch_year_from_fixed, 622);
    }

    #[test]
    fn test_islamic_epoch_thursday() {
        let epoch = FIXED_ISLAMIC_EPOCH_THURSDAY.to_i64_date();
        // Iso year of Islamic Epoch
        let epoch_year_from_fixed = Iso::iso_from_fixed(RataDie::new(epoch)).inner.0.year;
        // 622 is the correct ISO year for the Islamic Epoch
        assert_eq!(epoch_year_from_fixed, 622);
    }

    #[test]
    fn test_fixed_from_islamic() {
        for (case, f_date) in ARITHMETIC_CASES.iter().zip(TEST_FIXED_DATE.iter()) {
            let date = IslamicCivilDateInner(ArithmeticDate::new_unchecked(
                case.year, case.month, case.day,
            ));
            assert_eq!(
                IslamicCivil::fixed_from_islamic(date),
                RataDie::new(*f_date),
                "{case:?}"
            );
        }
    }

    #[test]
    fn test_islamic_from_fixed() {
        for (case, f_date) in ARITHMETIC_CASES.iter().zip(TEST_FIXED_DATE.iter()) {
            let date = Date::try_new_islamic_civil_date(case.year, case.month, case.day).unwrap();
            assert_eq!(
                IslamicCivil::islamic_from_fixed(RataDie::new(*f_date)),
                date,
                "{case:?}"
            );
        }
    }

    #[test]
    fn test_fixed_from_islamic_tbla() {
        for (case, f_date) in TABULAR_CASES.iter().zip(TEST_FIXED_DATE.iter()) {
            let date = IslamicTabularDateInner(ArithmeticDate::new_unchecked(
                case.year, case.month, case.day,
            ));
            assert_eq!(
                IslamicTabular::fixed_from_islamic(date),
                RataDie::new(*f_date),
                "{case:?}"
            );
        }
    }

    #[test]
    fn test_islamic_tbla_from_fixed() {
        for (case, f_date) in TABULAR_CASES.iter().zip(TEST_FIXED_DATE.iter()) {
            let date = Date::try_new_islamic_tabular_date(case.year, case.month, case.day).unwrap();
            assert_eq!(
                IslamicTabular::islamic_from_fixed(RataDie::new(*f_date)),
                date,
                "{case:?}"
            );
        }
    }

    #[test]
    fn test_saudi_criterion() {
        for (boolean, f_date) in SAUDI_CRITERION_EXPECTED.iter().zip(TEST_FIXED_DATE.iter()) {
            let bool_result = IslamicUmmAlQura::saudi_criterion(RataDie::new(*f_date)).unwrap();
            assert_eq!(*boolean, bool_result, "{f_date:?}");
        }
    }

    #[test]
    fn test_saudi_new_month_or_before() {
        for (date, f_date) in SAUDI_NEW_MONTH_OR_BEFORE_EXPECTED
            .iter()
            .zip(TEST_FIXED_DATE_UMMALQURA.iter())
        {
            let date_result =
                IslamicUmmAlQura::saudi_new_month_on_or_before(RataDie::new(*f_date)).to_f64_date();
            assert_eq!(*date, date_result, "{f_date:?}");
        }
    }

    #[test]
    fn test_saudi_islamic_from_fixed() {
        for (case, f_date) in UMMALQURA_DATE_EXPECTED
            .iter()
            .zip(TEST_FIXED_DATE_UMMALQURA.iter())
        {
            let date = Date::try_new_ummalqura_date(case.year, case.month, case.day).unwrap();
            assert_eq!(
                IslamicUmmAlQura::saudi_islamic_from_fixed(RataDie::new(*f_date)),
                date,
                "{case:?}"
            );
        }
    }

    #[test]
    fn test_fixed_from_saudi_islamic() {
        for (case, f_date) in UMMALQURA_DATE_EXPECTED
            .iter()
            .zip(TEST_FIXED_DATE_UMMALQURA.iter())
        {
            let date = IslamicUmmAlQuraDateInner(ArithmeticDate::new_unchecked(
                case.year, case.month, case.day,
            ));
            assert_eq!(
                IslamicUmmAlQura::fixed_from_saudi_islamic(date),
                RataDie::new(*f_date),
                "{case:?}"
            );
        }
    }

    #[ignore]
    #[test]
    fn test_days_in_provided_year_observational() {
        // -1245 1 1 = -214526 (R.D Date)
        // 1518 1 1 = 764589 (R.D Date)
        let sum_days_in_year: i64 = (START_YEAR..END_YEAR)
            .map(|year| IslamicObservational::days_in_provided_year(year) as i64)
            .sum();
        let expected_number_of_days = IslamicObservational::fixed_from_islamic(IslamicDateInner(
            ArithmeticDate::new_from_lunar_ordinals(END_YEAR, 1, 1).unwrap(),
        )) - IslamicObservational::fixed_from_islamic(
            IslamicDateInner(ArithmeticDate::new_from_lunar_ordinals(START_YEAR, 1, 1).unwrap()),
        ); // The number of days between Islamic years -1245 and 1518
        let tolerance = 1; // One day tolerance (See Astronomical::month_length for more context)

        assert!(
            (sum_days_in_year - expected_number_of_days).abs() <= tolerance,
            "Difference between sum_days_in_year and expected_number_of_days is more than the tolerance"
        );
    }

    #[ignore]
    #[test]
    fn test_days_in_provided_year_ummalqura() {
        // -1245 1 1 = -214528 (R.D Date)
        // 1518 1 1 = 764588 (R.D Date)
        let sum_days_in_year: i64 = (START_YEAR..END_YEAR)
            .map(|year| IslamicUmmAlQura::days_in_provided_year(year) as i64)
            .sum();

        let expected_number_of_days =
            IslamicUmmAlQura::fixed_from_saudi_islamic(IslamicUmmAlQuraDateInner(
                ArithmeticDate::new_from_lunar_ordinals(END_YEAR, 1, 1).unwrap(),
            )) - IslamicUmmAlQura::fixed_from_saudi_islamic(IslamicUmmAlQuraDateInner(
                ArithmeticDate::new_from_lunar_ordinals(START_YEAR, 1, 1).unwrap(),
            )); // The number of days between Umm al-Qura Islamic years -1245 and 1518

        assert_eq!(sum_days_in_year, expected_number_of_days);
    }

    #[test]
    fn test_regression_3868() {
        // This date used to panic on creation
        let iso = Date::try_new_iso_date(2011, 4, 4).unwrap();
        let islamic = iso.to_calendar(IslamicUmmAlQura);
        // Data from https://www.ummulqura.org.sa/Index.aspx
        assert_eq!(islamic.day_of_month().0, 30);
        assert_eq!(islamic.month().ordinal, 4);
        assert_eq!(islamic.year().number, 1432);
    }
}
