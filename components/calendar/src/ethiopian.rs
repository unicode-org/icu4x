// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains types and implementations for the Ethiopian calendar.
//!
//! ```rust
//! use icu::calendar::{ethiopian::Ethiopian, Date, DateTime};
//!
//! // `Date` type
//! let date_iso = Date::try_new_iso_date(1970, 1, 2)
//!     .expect("Failed to initialize ISO Date instance.");
//! let date_ethiopian = Date::new_from_iso(date_iso, Ethiopian::new());
//!
//! // `DateTime` type
//! let datetime_iso = DateTime::try_new_iso_datetime(1970, 1, 2, 13, 1, 0)
//!     .expect("Failed to initialize ISO DateTime instance.");
//! let datetime_ethiopian =
//!     DateTime::new_from_iso(datetime_iso, Ethiopian::new());
//!
//! // `Date` checks
//! assert_eq!(date_ethiopian.year().number, 1962);
//! assert_eq!(date_ethiopian.month().ordinal, 4);
//! assert_eq!(date_ethiopian.day_of_month().0, 24);
//!
//! // `DateTime` type
//! assert_eq!(datetime_ethiopian.date.year().number, 1962);
//! assert_eq!(datetime_ethiopian.date.month().ordinal, 4);
//! assert_eq!(datetime_ethiopian.date.day_of_month().0, 24);
//! assert_eq!(datetime_ethiopian.time.hour.number(), 13);
//! assert_eq!(datetime_ethiopian.time.minute.number(), 1);
//! assert_eq!(datetime_ethiopian.time.second.number(), 0);
//! ```

use crate::any_calendar::AnyCalendarKind;
use crate::calendar_arithmetic::{ArithmeticDate, CalendarArithmetic};
use crate::coptic::Coptic;
use crate::iso::Iso;
use crate::julian::Julian;
use crate::rata_die::RataDie;
use crate::{types, Calendar, CalendarError, Date, DateDuration, DateDurationUnit, DateTime};
use core::marker::PhantomData;
use tinystr::tinystr;

/// The number of years the Amete Alem epoch precedes the Amete Mihret epoch
const AMETE_ALEM_OFFSET: i32 = 5500;

/// Which era style the ethiopian calendar uses
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[non_exhaustive]
pub enum EthiopianEraStyle {
    /// Use an era scheme of pre- and post- Incarnation eras,
    /// anchored at the date of the Incarnation of Jesus in this calendar
    AmeteMihret,
    /// Use an era scheme of the Anno Mundi era, anchored at the date of Creation
    /// in this calendar
    AmeteAlem,
}

/// The [Ethiopian Calendar]
///
/// The [Ethiopian calendar] is a solar calendar used by the Coptic Orthodox Church, with twelve normal months
/// and a thirteenth small epagomenal month.
///
/// This type can be used with [`Date`] or [`DateTime`] to represent dates in this calendar.
///
/// It can be constructed in two modes: using the Amete Alem era scheme, or the Amete Mihret era scheme (the default),
/// see [`EthiopianEraStyle`] for more info.
///
/// [Ethiopian calendar]: https://en.wikipedia.org/wiki/Ethiopian_calendar
///
/// # Era codes
///
/// This calendar supports three era codes, based on what mode it is in. In the Amete Mihret scheme it has
/// the `"incar"` and `"pre-incar"` eras, 1 Incarnation is 9 CE. In the Amete Alem scheme, it instead has a single era,
/// `"mundi`, where 1 Anno Mundi is 5493 BCE. Dates before that use negative year numbers.
// The bool specifies whether dates should be in the Amete Alem era scheme
#[derive(Copy, Clone, Debug, Hash, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct Ethiopian(pub(crate) bool);

/// The inner date type used for representing [`Date`]s of [`Ethiopian`]. See [`Date`] and [`Ethiopian`] for more details.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct EthiopianDateInner(ArithmeticDate<Ethiopian>);

impl CalendarArithmetic for Ethiopian {
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

impl Calendar for Ethiopian {
    type DateInner = EthiopianDateInner;
    fn date_from_codes(
        &self,
        era: types::Era,
        year: i32,
        month_code: types::MonthCode,
        day: u8,
    ) -> Result<Self::DateInner, CalendarError> {
        let year = if era.0 == tinystr!(16, "incar") {
            if year <= 0 {
                return Err(CalendarError::OutOfRange);
            }
            year
        } else if era.0 == tinystr!(16, "pre-incar") {
            if year <= 0 {
                return Err(CalendarError::OutOfRange);
            }
            1 - year
        } else if era.0 == tinystr!(16, "mundi") {
            year - AMETE_ALEM_OFFSET
        } else {
            return Err(CalendarError::UnknownEra(era.0, self.debug_name()));
        };

        ArithmeticDate::new_from_solar(self, year, month_code, day).map(EthiopianDateInner)
    }
    fn date_from_iso(&self, iso: Date<Iso>) -> EthiopianDateInner {
        let fixed_iso = Iso::fixed_from_iso(*iso.inner());
        Self::ethiopian_from_fixed(fixed_iso)
    }

    fn date_to_iso(&self, date: &Self::DateInner) -> Date<Iso> {
        let fixed_ethiopian = Ethiopian::fixed_from_ethiopian(date.0);
        Iso::iso_from_fixed(fixed_ethiopian)
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
        Iso.day_of_week(self.date_to_iso(date).inner())
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
        Self::year_as_ethiopian(date.0.year, self.0)
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
            prev_year: Self::year_as_ethiopian(prev_year, self.0),
            days_in_prev_year: Ethiopian::days_in_year_direct(prev_year),
            next_year: Self::year_as_ethiopian(next_year, self.0),
        }
    }

    fn debug_name(&self) -> &'static str {
        "Ethiopian"
    }

    fn any_calendar_kind(&self) -> Option<AnyCalendarKind> {
        if self.0 {
            Some(AnyCalendarKind::EthiopianAmeteAlem)
        } else {
            Some(AnyCalendarKind::Ethiopian)
        }
    }
}

const ETHIOPIC_TO_COPTIC_OFFSET: i64 =
    super::coptic::COPTIC_EPOCH.const_diff(Julian::fixed_from_julian_integers(8, 8, 29));

impl Ethiopian {
    /// Construct a new Ethiopian Calendar for the Amete Mihret era naming scheme
    pub const fn new() -> Self {
        Self(false)
    }
    /// Construct a new Ethiopian Calendar with a value specifying whether or not it is Amete Alem
    pub const fn new_with_era_style(era_style: EthiopianEraStyle) -> Self {
        Self(matches!(era_style, EthiopianEraStyle::AmeteAlem))
    }
    /// Set whether or not this uses the Amete Alem era scheme
    pub fn set_era_style(&mut self, era_style: EthiopianEraStyle) {
        self.0 = era_style == EthiopianEraStyle::AmeteAlem
    }

    /// Returns whether this has the Amete Alem era
    pub fn era_style(&self) -> EthiopianEraStyle {
        if self.0 {
            EthiopianEraStyle::AmeteAlem
        } else {
            EthiopianEraStyle::AmeteMihret
        }
    }

    // "Fixed" is a day count representation of calendars staring from Jan 1st of year 1 of the Georgian Calendar.
    // The fixed date algorithms are from
    // Dershowitz, Nachum, and Edward M. Reingold. _Calendrical calculations_. Cambridge University Press, 2008.
    //
    // Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/1ee51ecfaae6f856b0d7de3e36e9042100b4f424/calendar.l#L2017
    fn fixed_from_ethiopian(date: ArithmeticDate<Ethiopian>) -> RataDie {
        Coptic::fixed_from_coptic_integers(date.year, date.month, date.day)
            - ETHIOPIC_TO_COPTIC_OFFSET
    }

    // Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/1ee51ecfaae6f856b0d7de3e36e9042100b4f424/calendar.l#L2028
    fn ethiopian_from_fixed(date: RataDie) -> EthiopianDateInner {
        let coptic_date = Coptic::coptic_from_fixed(date + ETHIOPIC_TO_COPTIC_OFFSET);

        #[allow(clippy::unwrap_used)] // Coptic and Ethiopic have the same allowed ranges for dates
        *Date::try_new_ethiopian_date(
            EthiopianEraStyle::AmeteMihret,
            coptic_date.0.year,
            coptic_date.0.month,
            coptic_date.0.day,
        )
        .unwrap()
        .inner()
    }

    fn days_in_year_direct(year: i32) -> u32 {
        if Ethiopian::is_leap_year(year) {
            366
        } else {
            365
        }
    }

    fn year_as_ethiopian(year: i32, amete_alem: bool) -> types::FormattableYear {
        if amete_alem {
            types::FormattableYear {
                era: types::Era(tinystr!(16, "mundi")),
                number: year + AMETE_ALEM_OFFSET,
                related_iso: None,
            }
        } else if year > 0 {
            types::FormattableYear {
                era: types::Era(tinystr!(16, "incar")),
                number: year,
                related_iso: None,
            }
        } else {
            types::FormattableYear {
                era: types::Era(tinystr!(16, "pre-incar")),
                number: 1 - year,
                related_iso: None,
            }
        }
    }
}

impl Date<Ethiopian> {
    /// Construct new Ethiopian Date.
    ///
    /// For the Amete Mihret era style, negative years work with
    /// year 0 as 1 pre-Incarnation, year -1 as 2 pre-Incarnation,
    /// and so on.
    ///
    /// ```rust
    /// use icu::calendar::ethiopian::EthiopianEraStyle;
    /// use icu::calendar::Date;
    ///
    /// let date_ethiopian = Date::try_new_ethiopian_date(
    ///     EthiopianEraStyle::AmeteMihret,
    ///     2014,
    ///     8,
    ///     25,
    /// )
    /// .expect("Failed to initialize Ethopic Date instance.");
    ///
    /// assert_eq!(date_ethiopian.year().number, 2014);
    /// assert_eq!(date_ethiopian.month().ordinal, 8);
    /// assert_eq!(date_ethiopian.day_of_month().0, 25);
    /// ```
    pub fn try_new_ethiopian_date(
        era_style: EthiopianEraStyle,
        mut year: i32,
        month: u8,
        day: u8,
    ) -> Result<Date<Ethiopian>, CalendarError> {
        if era_style == EthiopianEraStyle::AmeteAlem {
            year -= AMETE_ALEM_OFFSET;
        }
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

        Ok(Date::from_raw(
            EthiopianDateInner(inner),
            Ethiopian::new_with_era_style(era_style),
        ))
    }
}

impl DateTime<Ethiopian> {
    /// Construct a new Ethiopian datetime from integers.
    ///
    /// For the Amete Mihret era style, negative years work with
    /// year 0 as 1 pre-Incarnation, year -1 as 2 pre-Incarnation,
    /// and so on.
    ///
    /// ```rust
    /// use icu::calendar::ethiopian::EthiopianEraStyle;
    /// use icu::calendar::DateTime;
    ///
    /// let datetime_ethiopian = DateTime::try_new_ethiopian_datetime(
    ///     EthiopianEraStyle::AmeteMihret,
    ///     2014,
    ///     8,
    ///     25,
    ///     13,
    ///     1,
    ///     0,
    /// )
    /// .expect("Failed to initialize Ethiopian DateTime instance.");
    ///
    /// assert_eq!(datetime_ethiopian.date.year().number, 2014);
    /// assert_eq!(datetime_ethiopian.date.month().ordinal, 8);
    /// assert_eq!(datetime_ethiopian.date.day_of_month().0, 25);
    /// assert_eq!(datetime_ethiopian.time.hour.number(), 13);
    /// assert_eq!(datetime_ethiopian.time.minute.number(), 1);
    /// assert_eq!(datetime_ethiopian.time.second.number(), 0);
    /// ```
    pub fn try_new_ethiopian_datetime(
        era_style: EthiopianEraStyle,
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Result<DateTime<Ethiopian>, CalendarError> {
        Ok(DateTime {
            date: Date::try_new_ethiopian_date(era_style, year, month, day)?,
            time: types::Time::try_new(hour, minute, second, 0)?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_leap_year() {
        // 11th September 2023 in gregorian is 6/13/2015 in ethiopian
        let iso_date = Date::try_new_iso_date(2023, 9, 11).unwrap();
        let ethiopian_date = Ethiopian::new().date_from_iso(iso_date);
        assert_eq!(ethiopian_date.0.year, 2015);
        assert_eq!(ethiopian_date.0.month, 13);
        assert_eq!(ethiopian_date.0.day, 6);
    }

    #[test]
    fn test_iso_to_ethiopian_conversion_and_back() {
        let iso_date = Date::try_new_iso_date(1970, 1, 2).unwrap();
        let date_ethiopian = Date::new_from_iso(iso_date, Ethiopian::new());

        assert_eq!(date_ethiopian.inner.0.year, 1962);
        assert_eq!(date_ethiopian.inner.0.month, 4);
        assert_eq!(date_ethiopian.inner.0.day, 24);

        assert_eq!(
            date_ethiopian.to_iso(),
            Date::try_new_iso_date(1970, 1, 2).unwrap()
        );
    }

    #[test]
    fn test_roundtrip_negative() {
        // https://github.com/unicode-org/icu4x/issues/2254
        let iso_date = Date::try_new_iso_date(-1000, 3, 3).unwrap();
        let ethiopian = iso_date.to_calendar(Ethiopian::new());
        let recovered_iso = ethiopian.to_iso();
        assert_eq!(iso_date, recovered_iso);
    }
}
