// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains types and implementations for the Indian national calendar.
//!
//! ```rust
//! use icu::calendar::{indian::Indian, Date, DateTime};
//!
//! // `Date` type
//! let date_iso = Date::new_iso_date(1970, 1, 2)
//!     .expect("Failed to initialize ISO Date instance.");
//! let date_indian = Date::new_from_iso(date_iso, Indian);
//!
//! // `DateTime` type
//! let datetime_iso = DateTime::new_iso_datetime(1970, 1, 2, 13, 1, 0)
//!     .expect("Failed to initialize ISO DateTime instance.");
//! let datetime_indian = DateTime::new_from_iso(datetime_iso, Indian);
//!
//! // `Date` checks
//! assert_eq!(date_indian.year().number, 1892);
//! assert_eq!(date_indian.month().ordinal, 1);
//! assert_eq!(date_indian.day_of_month().0, 2);
//!
//! // `DateTime` type
//! assert_eq!(datetime_indian.date.year().number, 1892);
//! assert_eq!(datetime_indian.date.month().ordinal, 1);
//! assert_eq!(datetime_indian.date.day_of_month().0, 2);
//! assert_eq!(datetime_indian.time.hour.number(), 13);
//! assert_eq!(datetime_indian.time.minute.number(), 1);
//! assert_eq!(datetime_indian.time.second.number(), 0);
//! ```

use crate::any_calendar::AnyCalendarKind;
use crate::calendar_arithmetic::{ArithmeticDate, CalendarArithmetic};
use crate::iso::Iso;
use crate::{types, Calendar, Date, DateDuration, DateDurationUnit, DateTime, DateTimeError};
use core::marker::PhantomData;
use tinystr::tinystr;

/// The Indian National Calendar (aka the Saka calendar)
///
/// The [Indian National calendar] is a solar calendar used by the Indian government, with twelve months.
///
/// This type can be used with [`Date`] or [`DateTime`] to represent dates in this calendar.
///
/// [Indian National calendar]: https://en.wikipedia.org/wiki/Indian_national_calendar
///
/// # Era codes
///
/// This calendar has a single era: `"saka"`, with Saka 0 being 78 CE. Dates before this era use negative years.
#[derive(Copy, Clone, Debug, Hash, Default, Eq, PartialEq)]
#[allow(clippy::exhaustive_structs)] // this type is stable
pub struct Indian;

/// The inner date type used for representing [`Date`]s of [`Indian`]. See [`Date`] and [`Indian`] for more details.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct IndianDateInner(ArithmeticDate<Indian>);

impl CalendarArithmetic for Indian {
    fn month_days(year: i32, month: u8) -> u8 {
        if month == 1 {
            if Self::is_leap_year(year) {
                31
            } else {
                30
            }
        } else if (2..=6).contains(&month) {
            31
        } else if (7..=12).contains(&month) {
            30
        } else {
            0
        }
    }

    fn months_for_every_year() -> u8 {
        12
    }

    fn is_leap_year(year: i32) -> bool {
        Iso::is_leap_year(year + 78)
    }
}

impl Calendar for Indian {
    type DateInner = IndianDateInner;
    fn date_from_codes(
        &self,
        era: types::Era,
        year: i32,
        month_code: types::MonthCode,
        day: u8,
    ) -> Result<Self::DateInner, DateTimeError> {
        if era.0 != tinystr!(16, "saka") {
            return Err(DateTimeError::UnknownEra(era.0, self.debug_name()));
        }

        ArithmeticDate::new_from_solar(self, year, month_code, day).map(IndianDateInner)
    }
    fn date_from_iso(&self, iso: Date<Iso>) -> IndianDateInner {
        let day_of_year = Iso::day_of_year(*iso.inner());
        IndianDateInner(ArithmeticDate::date_from_year_day(
            iso.inner().0.year - 78,
            day_of_year,
        ))
    }

    fn date_to_iso(&self, date: &Self::DateInner) -> Date<Iso> {
        let day_of_year = date.0.day_of_year();
        Iso::iso_from_year_day(date.0.year + 78, day_of_year)
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
        Iso.day_of_week(Indian.date_to_iso(date).inner())
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
        types::FormattableYear {
            era: types::Era(tinystr!(16, "saka")),
            number: date.0.year,
            related_iso: None,
        }
    }

    fn month(&self, date: &Self::DateInner) -> types::FormattableMonth {
        date.0.solar_month()
    }

    fn day_of_month(&self, date: &Self::DateInner) -> types::DayOfMonth {
        date.0.day_of_month()
    }

    fn day_of_year_info(&self, date: &Self::DateInner) -> types::DayOfYearInfo {
        let prev_year = types::FormattableYear {
            era: types::Era(tinystr!(16, "saka")),
            number: date.0.year - 1,
            related_iso: None,
        };
        let next_year = types::FormattableYear {
            era: types::Era(tinystr!(16, "saka")),
            number: date.0.year + 1,
            related_iso: None,
        };
        types::DayOfYearInfo {
            day_of_year: date.0.day_of_year(),
            days_in_year: date.0.days_in_year(),
            prev_year,
            days_in_prev_year: Indian::days_in_year_direct(date.0.year - 1),
            next_year,
        }
    }

    fn debug_name(&self) -> &'static str {
        "Indian"
    }

    fn any_calendar_kind(&self) -> Option<AnyCalendarKind> {
        Some(AnyCalendarKind::Indian)
    }
}

impl Indian {
    /// Construct a new Indian Calendar
    pub fn new() -> Self {
        Self
    }

    fn days_in_year_direct(year: i32) -> u32 {
        if Indian::is_leap_year(year) {
            366
        } else {
            365
        }
    }
}

impl Date<Indian> {
    /// Construct new Indian Date, with year provided in the Śaka era.
    ///
    /// ```rust
    /// use icu::calendar::Date;
    ///
    /// let date_indian =
    ///     Date::new_indian_date(1891, 10, 12).expect("Failed to initialize Indian Date instance.");
    ///
    /// assert_eq!(date_indian.year().number, 1891);
    /// assert_eq!(date_indian.month().ordinal, 10);
    /// assert_eq!(date_indian.day_of_month().0, 12);
    /// ```
    pub fn new_indian_date(year: i32, month: u8, day: u8) -> Result<Date<Indian>, DateTimeError> {
        let inner = ArithmeticDate {
            year,
            month,
            day,
            marker: PhantomData,
        };

        let bound = inner.days_in_month();
        if day > bound {
            return Err(DateTimeError::OutOfRange);
        }

        Ok(Date::from_raw(IndianDateInner(inner), Indian))
    }
}

impl DateTime<Indian> {
    /// Construct a new Indian datetime from integers, with year provided in the Śaka era.
    ///
    /// ```rust
    /// use icu::calendar::DateTime;
    ///
    /// let datetime_indian = DateTime::new_indian_datetime(1891, 10, 12, 13, 1, 0)
    ///     .expect("Failed to initialize Indian DateTime instance.");
    ///
    /// assert_eq!(datetime_indian.date.year().number, 1891);
    /// assert_eq!(datetime_indian.date.month().ordinal, 10);
    /// assert_eq!(datetime_indian.date.day_of_month().0, 12);
    /// assert_eq!(datetime_indian.time.hour.number(), 13);
    /// assert_eq!(datetime_indian.time.minute.number(), 1);
    /// assert_eq!(datetime_indian.time.second.number(), 0);
    /// ```
    pub fn new_indian_datetime(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Result<DateTime<Indian>, DateTimeError> {
        Ok(DateTime {
            date: Date::new_indian_date(year, month, day)?,
            time: types::Time::try_new(hour, minute, second, 0)?,
        })
    }
}
