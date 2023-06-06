// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains types and implementations for the Coptic calendar.
//!
//! ```rust
//! use icu::calendar::{coptic::Coptic, Date, DateTime};
//!
//! // `Date` type
//! let date_iso = Date::try_new_iso_date(1970, 1, 2)
//!     .expect("Failed to initialize ISO Date instance.");
//! let date_coptic = Date::new_from_iso(date_iso, Coptic);
//!
//! // `DateTime` type
//! let datetime_iso = DateTime::try_new_iso_datetime(1970, 1, 2, 13, 1, 0)
//!     .expect("Failed to initialize ISO DateTime instance.");
//! let datetime_coptic = DateTime::new_from_iso(datetime_iso, Coptic);
//!
//! // `Date` checks
//! assert_eq!(date_coptic.year().number, 1686);
//! assert_eq!(date_coptic.month().ordinal, 4);
//! assert_eq!(date_coptic.day_of_month().0, 24);
//!
//! // `DateTime` type
//! assert_eq!(datetime_coptic.date.year().number, 1686);
//! assert_eq!(datetime_coptic.date.month().ordinal, 4);
//! assert_eq!(datetime_coptic.date.day_of_month().0, 24);
//! assert_eq!(datetime_coptic.time.hour.number(), 13);
//! assert_eq!(datetime_coptic.time.minute.number(), 1);
//! assert_eq!(datetime_coptic.time.second.number(), 0);
//! ```

use crate::any_calendar::AnyCalendarKind;
use crate::calendar_arithmetic::{ArithmeticDate, CalendarArithmetic};
use crate::helpers::{i64_to_i32, quotient, quotient64, I32Result};
use crate::iso::Iso;
use crate::julian::Julian;
use crate::rata_die::RataDie;
use crate::{types, Calendar, CalendarError, Date, DateDuration, DateDurationUnit, DateTime};
use core::marker::PhantomData;
use tinystr::tinystr;

/// The [Coptic Calendar]
///
/// The [Coptic calendar] is a solar calendar used by the Coptic Orthodox Church, with twelve normal months
/// and a thirteenth small epagomenal month.
///
/// This type can be used with [`Date`] or [`DateTime`] to represent dates in this calendar.
///
/// [Coptic calendar]: https://en.wikipedia.org/wiki/Coptic_calendar
///
/// # Era codes
///
/// This calendar supports two era codes: `"bd"`, and `"ad"`, corresponding to the Before Diocletian and After Diocletian/Anno Martyrum
/// eras. 1 A.M. is equivalent to 284 C.E.
#[derive(Copy, Clone, Debug, Hash, Default, Eq, PartialEq, PartialOrd, Ord)]
#[allow(clippy::exhaustive_structs)] // this type is stable
pub struct Coptic;

/// The inner date type used for representing [`Date`]s of [`Coptic`]. See [`Date`] and [`Coptic`] for more details.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct CopticDateInner(pub(crate) ArithmeticDate<Coptic>);

impl CalendarArithmetic for Coptic {
    fn month_days(year: i32, month: u8) -> u8 {
        if (1..=12).contains(&month) {
            30
        } else if month == 13 {
            if Self::is_leap_year(year) {
                6
            } else {
                5
            }
        } else {
            0
        }
    }

    fn months_for_every_year(_: i32) -> u8 {
        13
    }

    fn is_leap_year(year: i32) -> bool {
        year % 4 == 3
    }

    fn last_month_day_in_year(year: i32) -> (u8, u8) {
        if Self::is_leap_year(year) {
            (13, 6)
        } else {
            (13, 5)
        }
    }

    fn days_in_provided_year(year: i32) -> u32 {
        if Self::is_leap_year(year) {
            366
        } else {
            365
        }
    }
}

impl Calendar for Coptic {
    type DateInner = CopticDateInner;
    fn date_from_codes(
        &self,
        era: types::Era,
        year: i32,
        month_code: types::MonthCode,
        day: u8,
    ) -> Result<Self::DateInner, CalendarError> {
        let year = if era.0 == tinystr!(16, "ad") {
            if year <= 0 {
                return Err(CalendarError::OutOfRange);
            }
            year
        } else if era.0 == tinystr!(16, "bd") {
            if year <= 0 {
                return Err(CalendarError::OutOfRange);
            }
            1 - year
        } else {
            return Err(CalendarError::UnknownEra(era.0, self.debug_name()));
        };

        ArithmeticDate::new_from_solar(self, year, month_code, day).map(CopticDateInner)
    }
    fn date_from_iso(&self, iso: Date<Iso>) -> CopticDateInner {
        let fixed_iso = Iso::fixed_from_iso(*iso.inner());
        Self::coptic_from_fixed(fixed_iso)
    }

    fn date_to_iso(&self, date: &Self::DateInner) -> Date<Iso> {
        let fixed_coptic = Coptic::fixed_from_coptic(date.0);
        Iso::iso_from_fixed(fixed_coptic)
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

    fn day_of_week(&self, date: &Self::DateInner) -> types::IsoWeekday {
        Iso.day_of_week(Coptic.date_to_iso(date).inner())
    }

    fn offset_date(&self, date: &mut Self::DateInner, offset: DateDuration<Self>) {
        date.0.offset_date(offset);
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

    fn year(&self, date: &Self::DateInner) -> types::FormattableYear {
        year_as_coptic(date.0.year)
    }

    fn month(&self, date: &Self::DateInner) -> types::FormattableMonth {
        date.0.solar_month()
    }

    fn day_of_month(&self, date: &Self::DateInner) -> types::DayOfMonth {
        date.0.day_of_month()
    }

    fn day_of_year_info(&self, date: &Self::DateInner) -> types::DayOfYearInfo {
        let prev_year = date.0.year - 1;
        let next_year = date.0.year + 1;
        types::DayOfYearInfo {
            day_of_year: date.0.day_of_year(),
            days_in_year: date.0.days_in_year(),
            prev_year: year_as_coptic(prev_year),
            days_in_prev_year: Coptic::days_in_year_direct(prev_year),
            next_year: year_as_coptic(next_year),
        }
    }

    fn debug_name(&self) -> &'static str {
        "Coptic"
    }

    fn any_calendar_kind(&self) -> Option<AnyCalendarKind> {
        Some(AnyCalendarKind::Coptic)
    }
}

pub(crate) const COPTIC_EPOCH: RataDie = Julian::fixed_from_julian_integers(284, 8, 29);

impl Coptic {
    // "Fixed" is a day count representation of calendars staring from Jan 1st of year 1 of the Georgian Calendar.
    // The fixed date algorithms are from
    // Dershowitz, Nachum, and Edward M. Reingold. _Calendrical calculations_. Cambridge University Press, 2008.
    //
    // Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/1ee51ecfaae6f856b0d7de3e36e9042100b4f424/calendar.l#L1978
    fn fixed_from_coptic(date: ArithmeticDate<Coptic>) -> RataDie {
        COPTIC_EPOCH - 1
            + 365 * (date.year as i64 - 1)
            + quotient(date.year, 4) as i64
            + 30 * (date.month as i64 - 1)
            + date.day as i64
    }

    pub(crate) fn fixed_from_coptic_integers(year: i32, month: u8, day: u8) -> RataDie {
        Self::fixed_from_coptic(ArithmeticDate {
            year,
            month,
            day,
            marker: PhantomData,
        })
    }

    // Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/1ee51ecfaae6f856b0d7de3e36e9042100b4f424/calendar.l#L1990
    pub(crate) fn coptic_from_fixed(date: RataDie) -> CopticDateInner {
        let year = quotient64(4 * (date - COPTIC_EPOCH) + 1463, 1461);
        let year = match i64_to_i32(year) {
            I32Result::BelowMin(_) => return CopticDateInner(ArithmeticDate::min_date()),
            I32Result::AboveMax(_) => return CopticDateInner(ArithmeticDate::max_date()),
            I32Result::WithinRange(y) => y,
        };
        let month = (quotient64(date - Self::fixed_from_coptic_integers(year, 1, 1), 30) + 1) as u8; // <= 12 < u8::MAX
        let day = (date + 1 - Self::fixed_from_coptic_integers(year, month, 1)) as u8; // <= days_in_month < u8::MAX

        #[allow(clippy::unwrap_used)] // day and month have the correct bounds
        *Date::try_new_coptic_date(year, month, day).unwrap().inner()
    }

    fn days_in_year_direct(year: i32) -> u32 {
        if Coptic::is_leap_year(year) {
            366
        } else {
            365
        }
    }
}

impl Date<Coptic> {
    /// Construct new Coptic Date.
    ///
    /// Negative years are in the B.D. era, starting with 0 = 1 B.D.
    ///
    /// ```rust
    /// use icu::calendar::Date;
    ///
    /// let date_coptic = Date::try_new_coptic_date(1686, 5, 6)
    ///     .expect("Failed to initialize Coptic Date instance.");
    ///
    /// assert_eq!(date_coptic.year().number, 1686);
    /// assert_eq!(date_coptic.month().ordinal, 5);
    /// assert_eq!(date_coptic.day_of_month().0, 6);
    /// ```
    pub fn try_new_coptic_date(
        year: i32,
        month: u8,
        day: u8,
    ) -> Result<Date<Coptic>, CalendarError> {
        let inner = ArithmeticDate {
            year,
            month,
            day,
            marker: PhantomData,
        };

        let bound = inner.days_in_month();
        if day > bound {
            return Err(CalendarError::OutOfRange);
        }

        Ok(Date::from_raw(CopticDateInner(inner), Coptic))
    }
}

impl DateTime<Coptic> {
    /// Construct a new Coptic datetime from integers.
    ///
    /// Negative years are in the B.D. era, starting with 0 = 1 B.D.
    ///
    /// ```rust
    /// use icu::calendar::DateTime;
    ///
    /// let datetime_coptic =
    ///     DateTime::try_new_coptic_datetime(1686, 5, 6, 13, 1, 0)
    ///         .expect("Failed to initialize Coptic DateTime instance.");
    ///
    /// assert_eq!(datetime_coptic.date.year().number, 1686);
    /// assert_eq!(datetime_coptic.date.month().ordinal, 5);
    /// assert_eq!(datetime_coptic.date.day_of_month().0, 6);
    /// assert_eq!(datetime_coptic.time.hour.number(), 13);
    /// assert_eq!(datetime_coptic.time.minute.number(), 1);
    /// assert_eq!(datetime_coptic.time.second.number(), 0);
    /// ```
    pub fn try_new_coptic_datetime(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Result<DateTime<Coptic>, CalendarError> {
        Ok(DateTime {
            date: Date::try_new_coptic_date(year, month, day)?,
            time: types::Time::try_new(hour, minute, second, 0)?,
        })
    }
}

fn year_as_coptic(year: i32) -> types::FormattableYear {
    if year > 0 {
        types::FormattableYear {
            era: types::Era(tinystr!(16, "ad")),
            number: year,
            related_iso: None,
        }
    } else {
        types::FormattableYear {
            era: types::Era(tinystr!(16, "bd")),
            number: 1 - year,
            related_iso: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_coptic_regression() {
        // https://github.com/unicode-org/icu4x/issues/2254
        let iso_date = Date::try_new_iso_date(-100, 3, 3).unwrap();
        let coptic = iso_date.to_calendar(Coptic);
        let recovered_iso = coptic.to_iso();
        assert_eq!(iso_date, recovered_iso);
    }
}
