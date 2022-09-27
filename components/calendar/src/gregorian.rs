// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains types and implementations for the Gregorian calendar.
//!
//! ```rust
//! use icu::calendar::{gregorian::Gregorian, Date, DateTime};
//!
//! // `Date` type
//! let date_iso = Date::try_new_iso_date(1970, 1, 2)
//!     .expect("Failed to initialize ISO Date instance.");
//! let date_gregorian = Date::new_from_iso(date_iso, Gregorian);
//!
//! // `DateTime` type
//! let datetime_iso = DateTime::try_new_iso_datetime(1970, 1, 2, 13, 1, 0)
//!     .expect("Failed to initialize ISO DateTime instance.");
//! let datetime_gregorian = DateTime::new_from_iso(datetime_iso, Gregorian);
//!
//! // `Date` checks
//! assert_eq!(date_gregorian.year().number, 1970);
//! assert_eq!(date_gregorian.month().ordinal, 1);
//! assert_eq!(date_gregorian.day_of_month().0, 2);
//!
//! // `DateTime` type
//! assert_eq!(datetime_gregorian.date.year().number, 1970);
//! assert_eq!(datetime_gregorian.date.month().ordinal, 1);
//! assert_eq!(datetime_gregorian.date.day_of_month().0, 2);
//! assert_eq!(datetime_gregorian.time.hour.number(), 13);
//! assert_eq!(datetime_gregorian.time.minute.number(), 1);
//! assert_eq!(datetime_gregorian.time.second.number(), 0);
//! ```

use crate::any_calendar::AnyCalendarKind;
use crate::calendar_arithmetic::ArithmeticDate;
use crate::iso::{Iso, IsoDateInner};
use crate::{types, Calendar, CalendarError, Date, DateDuration, DateDurationUnit, DateTime};
use tinystr::tinystr;

/// The Gregorian Calendar
///
/// The [Gregorian calendar] is a solar calendar used by most of the world, with twelve months.
///
/// This type can be used with [`Date`] or [`DateTime`] to represent dates in this calendar.
///
/// [Gregorian calendar]: https://en.wikipedia.org/wiki/Gregorian_calendar
///
/// # Era codes
///
/// This calendar supports two era codes: `"bce"`, and `"ce"`, corresponding to the BCE and CE eras
#[derive(Copy, Clone, Debug, Default)]
#[allow(clippy::exhaustive_structs)] // this type is stable
pub struct Gregorian;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
/// The inner date type used for representing [`Date`]s of [`Gregorian`]. See [`Date`] and [`Gregorian`] for more details.
pub struct GregorianDateInner(IsoDateInner);

impl Calendar for Gregorian {
    type DateInner = GregorianDateInner;
    fn date_from_codes(
        &self,
        era: types::Era,
        year: i32,
        month_code: types::MonthCode,
        day: u8,
    ) -> Result<Self::DateInner, CalendarError> {
        let year = if era.0 == tinystr!(16, "ce") {
            if year <= 0 {
                return Err(CalendarError::OutOfRange);
            }
            year
        } else if era.0 == tinystr!(16, "bce") {
            if year <= 0 {
                return Err(CalendarError::OutOfRange);
            }
            1 - year
        } else {
            return Err(CalendarError::UnknownEra(era.0, self.debug_name()));
        };

        ArithmeticDate::new_from_solar(self, year, month_code, day)
            .map(IsoDateInner)
            .map(GregorianDateInner)
    }

    fn date_from_iso(&self, iso: Date<Iso>) -> GregorianDateInner {
        GregorianDateInner(*iso.inner())
    }

    fn date_to_iso(&self, date: &Self::DateInner) -> Date<Iso> {
        Date::from_raw(date.0, Iso)
    }

    fn months_in_year(&self, date: &Self::DateInner) -> u8 {
        Iso.months_in_year(&date.0)
    }

    fn days_in_year(&self, date: &Self::DateInner) -> u32 {
        Iso.days_in_year(&date.0)
    }

    fn days_in_month(&self, date: &Self::DateInner) -> u8 {
        Iso.days_in_month(&date.0)
    }

    fn offset_date(&self, date: &mut Self::DateInner, offset: DateDuration<Self>) {
        Iso.offset_date(&mut date.0, offset.cast_unit())
    }

    #[allow(clippy::field_reassign_with_default)] // it's more clear this way
    fn until(
        &self,
        date1: &Self::DateInner,
        date2: &Self::DateInner,
        _calendar2: &Self,
        largest_unit: DateDurationUnit,
        smallest_unit: DateDurationUnit,
    ) -> DateDuration<Self> {
        Iso.until(&date1.0, &date2.0, &Iso, largest_unit, smallest_unit)
            .cast_unit()
    }

    /// The calendar-specific year represented by `date`
    fn year(&self, date: &Self::DateInner) -> types::FormattableYear {
        year_as_gregorian(date.0 .0.year)
    }

    /// The calendar-specific month represented by `date`
    fn month(&self, date: &Self::DateInner) -> types::FormattableMonth {
        Iso.month(&date.0)
    }

    /// The calendar-specific day-of-month represented by `date`
    fn day_of_month(&self, date: &Self::DateInner) -> types::DayOfMonth {
        Iso.day_of_month(&date.0)
    }

    /// Information of the day of the year
    fn day_of_year_info(&self, date: &Self::DateInner) -> types::DayOfYearInfo {
        let prev_year = date.0 .0.year - 1;
        let next_year = date.0 .0.year + 1;
        types::DayOfYearInfo {
            day_of_year: Iso::day_of_year(date.0),
            days_in_year: Iso::days_in_year_direct(date.0 .0.year),
            prev_year: year_as_gregorian(prev_year),
            days_in_prev_year: Iso::days_in_year_direct(prev_year),
            next_year: year_as_gregorian(next_year),
        }
    }

    fn debug_name(&self) -> &'static str {
        "Gregorian"
    }

    fn any_calendar_kind(&self) -> Option<AnyCalendarKind> {
        Some(AnyCalendarKind::Gregorian)
    }
}

impl Date<Gregorian> {
    /// Construct a new Gregorian Date.
    ///
    /// Years are specified as ISO years.
    ///
    /// ```rust
    /// use icu::calendar::Date;
    /// use std::convert::TryFrom;
    ///
    /// // Conversion from ISO to Gregorian
    /// let date_gregorian = Date::try_new_gregorian_date(1970, 1, 2)
    ///     .expect("Failed to initialize Gregorian Date instance.");
    ///
    /// assert_eq!(date_gregorian.year().number, 1970);
    /// assert_eq!(date_gregorian.month().ordinal, 1);
    /// assert_eq!(date_gregorian.day_of_month().0, 2);
    /// ```
    pub fn try_new_gregorian_date(
        year: i32,
        month: u8,
        day: u8,
    ) -> Result<Date<Gregorian>, CalendarError> {
        Date::try_new_iso_date(year, month, day).map(|d| Date::new_from_iso(d, Gregorian))
    }
}

impl DateTime<Gregorian> {
    /// Construct a new Gregorian datetime from integers.
    ///
    /// Years are specified as ISO years.
    ///
    /// ```rust
    /// use icu::calendar::DateTime;
    ///
    /// let datetime_gregorian =
    ///     DateTime::try_new_gregorian_datetime(1970, 1, 2, 13, 1, 0)
    ///         .expect("Failed to initialize Gregorian DateTime instance.");
    ///
    /// assert_eq!(datetime_gregorian.date.year().number, 1970);
    /// assert_eq!(datetime_gregorian.date.month().ordinal, 1);
    /// assert_eq!(datetime_gregorian.date.day_of_month().0, 2);
    /// assert_eq!(datetime_gregorian.time.hour.number(), 13);
    /// assert_eq!(datetime_gregorian.time.minute.number(), 1);
    /// assert_eq!(datetime_gregorian.time.second.number(), 0);
    /// ```
    pub fn try_new_gregorian_datetime(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Result<DateTime<Gregorian>, CalendarError> {
        Ok(DateTime {
            date: Date::try_new_gregorian_date(year, month, day)?,
            time: types::Time::try_new(hour, minute, second, 0)?,
        })
    }
}

pub(crate) fn year_as_gregorian(year: i32) -> types::FormattableYear {
    if year > 0 {
        types::FormattableYear {
            era: types::Era(tinystr!(16, "ce")),
            number: year,
            related_iso: None,
        }
    } else {
        types::FormattableYear {
            era: types::Era(tinystr!(16, "bce")),
            number: 1 - year,
            related_iso: None,
        }
    }
}
