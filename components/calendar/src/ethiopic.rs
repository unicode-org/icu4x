// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains types and implementations for the Ethiopic calendar.
//!
//! ```rust
//! use icu::calendar::{
//!     ethiopic::Ethiopic, types::IsoHour, types::IsoMinute, types::IsoSecond, Date, DateTime,
//! };
//!
//! // `Date` type
//! let date_iso = Date::new_iso_date_from_integers(1970, 1, 2)
//!     .expect("Failed to initialize ISO Date instance.");
//! let date_ethiopic = Date::new_from_iso(date_iso, Ethiopic::new());
//!
//! // `DateTime` type
//! let datetime_iso = DateTime::new_iso_datetime_from_integers(1970, 1, 2, 13, 1, 0)
//!     .expect("Failed to initialize ISO DateTime instance.");
//! let datetime_ethiopic = DateTime::new_from_iso(datetime_iso, Ethiopic::new());
//!
//! // `Date` checks
//! assert_eq!(date_ethiopic.year().number, 1962);
//! assert_eq!(date_ethiopic.month().number, 4);
//! assert_eq!(date_ethiopic.day_of_month().0, 24);
//!
//! // `DateTime` type
//! assert_eq!(datetime_ethiopic.date.year().number, 1962);
//! assert_eq!(datetime_ethiopic.date.month().number, 4);
//! assert_eq!(datetime_ethiopic.date.day_of_month().0, 24);
//! assert_eq!(datetime_ethiopic.time.hour.number(), 13);
//! assert_eq!(datetime_ethiopic.time.minute.number(), 1);
//! assert_eq!(datetime_ethiopic.time.second.number(), 0);
//! ```

use crate::coptic::Coptic;
use crate::iso::{Iso, IsoYear};
use crate::julian::Julian;
use crate::{
    types, ArithmeticDate, Calendar, CalendarArithmetic, Date, DateDuration, DateDurationUnit,
    DateTime, DateTimeError,
};
use core::marker::PhantomData;
use tinystr::tinystr;

// The Ethiopic Calendar
#[derive(Copy, Clone, Debug, Hash, Default, Eq, PartialEq)]
pub struct Ethiopic(pub(crate) bool);

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct EthiopicDateInner(ArithmeticDate<Ethiopic>);

impl CalendarArithmetic for Ethiopic {
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

    fn months_for_every_year() -> u8 {
        13
    }

    fn is_leap_year(year: i32) -> bool {
        year % 4 == 3
    }
}

impl Calendar for Ethiopic {
    type DateInner = EthiopicDateInner;
    fn date_from_iso(&self, iso: Date<Iso>) -> EthiopicDateInner {
        let fixed_iso = Iso::fixed_from_iso(*iso.inner());
        Self::ethiopic_from_fixed(fixed_iso)
    }

    fn date_to_iso(&self, date: &Self::DateInner) -> Date<Iso> {
        let fixed_ethiopic = Ethiopic::fixed_from_ethiopic(date.0);
        Iso::iso_from_fixed(fixed_ethiopic)
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

    fn year(&self, date: &Self::DateInner) -> types::Year {
        if self.0 {
            types::Year {
                era: types::Era(tinystr!(16, "mundi")),
                number: date.0.year,
                related_iso: date.0.year + 5493 + 8,
            }
        } else {
            types::Year {
                era: if date.0.year > 0 {
                    types::Era(tinystr!(16, "incarnation"))
                } else {
                    types::Era(tinystr!(16, "before-incar"))
                },
                number: date.0.year,
                related_iso: date.0.year + 5493 + 8,
            }
        }
    }

    fn month(&self, date: &Self::DateInner) -> types::Month {
        types::Month {
            number: date.0.month.into(),
            code: types::MonthCode(tinystr!(8, "TODO")),
        }
    }

    fn day_of_month(&self, date: &Self::DateInner) -> types::DayOfMonth {
        date.0.day_of_month()
    }

    fn day_of_year_info(&self, date: &Self::DateInner) -> types::DayOfYearInfo {
        let prev_year = IsoYear(date.0.year - 1);
        let next_year = IsoYear(date.0.year + 1);
        types::DayOfYearInfo {
            day_of_year: date.0.day_of_year(),
            days_in_year: date.0.days_in_year(),
            prev_year: prev_year.into(),
            days_in_prev_year: Ethiopic::days_in_year_direct(prev_year.0),
            next_year: next_year.into(),
        }
    }

    fn debug_name(&self) -> &'static str {
        "Ethiopic"
    }
}

impl Ethiopic {
    /// Construct a new Ethiopic Calendar for the Amete Mihret era naming scheme
    pub fn new() -> Self {
        Self(false)
    }
    /// Construct a new Ethiopic Calendar with a value specifying whether or not it is Amete Alem
    pub fn new_with_amete_alem(amete_alem: bool) -> Self {
        Self(amete_alem)
    }
    /// Set whether or not this uses the Amete Alem era scheme
    pub fn set_amete_alem(&mut self, value: bool) {
        self.0 = value
    }

    /// Returns whether this has the Amete Alem era
    pub fn is_amete_alem(&self) -> bool {
        self.0
    }

    // "Fixed" is a day count representation of calendars staring from Jan 1st of year 1 of the Georgian Calendar.
    // The fixed date algorithms are from
    // Dershowitz, Nachum, and Edward M. Reingold. _Calendrical calculations_. Cambridge University Press, 2008.
    //
    // Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/1ee51ecfaae6f856b0d7de3e36e9042100b4f424/calendar.l#L2017
    fn fixed_from_ethiopic(date: ArithmeticDate<Ethiopic>) -> i32 {
        let coptic_epoch = Julian::fixed_from_julian_integers(284, 8, 29);
        let ethiopic_epoch = Julian::fixed_from_julian_integers(8, 8, 29);
        ethiopic_epoch - coptic_epoch
            + Coptic::fixed_from_coptic_integers(date.year, date.month as i32, date.day as i32)
    }

    // Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/1ee51ecfaae6f856b0d7de3e36e9042100b4f424/calendar.l#L2028
    fn ethiopic_from_fixed(date: i32) -> EthiopicDateInner {
        let coptic_epoch = Julian::fixed_from_julian_integers(284, 8, 29);
        let ethiopic_epoch = Julian::fixed_from_julian_integers(8, 8, 29);
        let coptic_date = Coptic::coptic_from_fixed(date + coptic_epoch - ethiopic_epoch);

        #[allow(clippy::unwrap_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
        *Date::new_ethiopic_date(coptic_date.0.year, coptic_date.0.month, coptic_date.0.day)
            .unwrap()
            .inner()
    }

    fn days_in_year_direct(year: i32) -> u32 {
        if Ethiopic::is_leap_year(year) {
            366
        } else {
            365
        }
    }
}

impl Date<Ethiopic> {
    /// Construct new Ethiopic Date.
    ///
    /// ```rust
    /// use icu::calendar::Date;
    ///
    /// let date_ethiopic =
    ///     Date::new_ethiopic_date(2014, 8, 25).expect("Failed to initialize Ethopic Date instance.");
    ///
    /// assert_eq!(date_ethiopic.year().number, 2014);
    /// assert_eq!(date_ethiopic.month().number, 8);
    /// assert_eq!(date_ethiopic.day_of_month().0, 25);
    /// ```
    pub fn new_ethiopic_date(
        year: i32,
        month: u8,
        day: u8,
    ) -> Result<Date<Ethiopic>, DateTimeError> {
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

        Ok(Date::from_raw(EthiopicDateInner(inner), Ethiopic::new()))
    }
}

impl DateTime<Ethiopic> {
    /// Construct a new Ethiopic datetime from integers.
    ///
    /// ```rust
    /// use icu::calendar::{types::IsoHour, types::IsoMinute, types::IsoSecond, DateTime};
    ///
    /// let datetime_ethiopic = DateTime::new_ethiopic_datetime(2014, 8, 25, 13, 1, 0, 0)
    ///     .expect("Failed to initialize Ethiopic DateTime instance.");
    ///
    /// assert_eq!(datetime_ethiopic.date.year().number, 2014);
    /// assert_eq!(datetime_ethiopic.date.month().number, 8);
    /// assert_eq!(datetime_ethiopic.date.day_of_month().0, 25);
    /// assert_eq!(datetime_ethiopic.time.hour.number(), 13);
    /// assert_eq!(datetime_ethiopic.time.minute.number(), 1);
    /// assert_eq!(datetime_ethiopic.time.second.number(), 0);
    /// ```
    pub fn new_ethiopic_datetime(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        fraction: u32,
    ) -> Result<DateTime<Ethiopic>, DateTimeError> {
        Ok(DateTime {
            date: Date::new_ethiopic_date(year, month, day)?,
            time: types::Time::try_new(hour, minute, second, fraction)?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_leap_year() {
        // 11th September 2023 in gregorian is 6/13/2015 in ethiopic
        let iso_date = Date::new_iso_date_from_integers(2023, 9, 11).unwrap();
        let ethiopic_date = Ethiopic::new().date_from_iso(iso_date);
        assert_eq!(ethiopic_date.0.year, 2015);
        assert_eq!(ethiopic_date.0.month, 13);
        assert_eq!(ethiopic_date.0.day, 6);
    }

    #[test]
    fn test_iso_to_ethiopic_conversion_and_back() {
        let iso_date = Date::new_iso_date_from_integers(1970, 1, 2).unwrap();
        let date_ethiopic = Date::new_from_iso(iso_date, Ethiopic::new());

        assert_eq!(date_ethiopic.inner.0.year, 1962);
        assert_eq!(date_ethiopic.inner.0.month, 4);
        assert_eq!(date_ethiopic.inner.0.day, 24);

        assert_eq!(
            date_ethiopic.to_iso(),
            Date::new_iso_date_from_integers(1970, 1, 2).unwrap()
        );
    }
}
