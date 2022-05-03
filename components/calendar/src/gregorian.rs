// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains types and implementations for the Gregorian calendar

use crate::iso::{Iso, IsoDateInner, IsoDay, IsoMonth, IsoYear};
use crate::{types, Calendar, Date, DateDuration, DateDurationUnit, DateTime, DateTimeError};
use core::convert::TryInto;
use tinystr::tinystr;

/// The Gregorian Calendar
#[derive(Copy, Clone, Debug, Default)]
#[allow(clippy::exhaustive_structs)] // this type is stable
pub struct Gregorian;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
/// The inner date type used for representing Date<Gregorian>
pub struct GregorianDateInner(IsoDateInner);

impl Calendar for Gregorian {
    type DateInner = GregorianDateInner;
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
    fn year(&self, date: &Self::DateInner) -> types::Year {
        year_as_gregorian(date.0.year.0)
    }

    /// The calendar-specific month represented by `date`
    fn month(&self, date: &Self::DateInner) -> types::Month {
        Iso.month(&date.0)
    }

    /// The calendar-specific day-of-month represented by `date`
    fn day_of_month(&self, date: &Self::DateInner) -> types::DayOfMonth {
        Iso.day_of_month(&date.0)
    }

    /// Information of the day of the year
    fn day_of_year_info(&self, date: &Self::DateInner) -> types::DayOfYearInfo {
        let prev_year = IsoYear(date.0.year.0 - 1);
        let next_year = IsoYear(date.0.year.0 + 1);
        types::DayOfYearInfo {
            day_of_year: Iso::day_of_year(date.0),
            days_in_year: Iso::days_in_year(date.0.year),
            prev_year: year_as_gregorian(prev_year.0),
            days_in_prev_year: Iso::days_in_year(prev_year),
            next_year: year_as_gregorian(next_year.0),
        }
    }

    fn debug_name(&self) -> &'static str {
        "Gregorian"
    }
}

impl Date<Gregorian> {
    /// Construct a new Gregorian Date.
    ///
    /// Years are specified as ISO years.
    ///
    /// ```rust
    /// use icu::calendar::{Date,
    ///                     iso::IsoYear,
    ///                     iso::IsoMonth,
    ///                     iso::IsoDay};
    /// use std::convert::TryFrom;
    ///
    /// let iso_year = IsoYear(1970);
    /// let iso_month = IsoMonth::try_from(1).unwrap();
    /// let iso_day = IsoDay::try_from(2).unwrap();
    ///
    /// // Conversion from ISO to Gregorian
    /// let date_gregorian = Date::new_gregorian_date(iso_year, iso_month, iso_day).unwrap();
    ///
    /// assert_eq!(date_gregorian.year().number, 1970);
    /// assert_eq!(date_gregorian.month().number, 1);
    /// assert_eq!(date_gregorian.day_of_month().0, 2);
    /// ```
    pub fn new_gregorian_date(
        year: IsoYear,
        month: IsoMonth,
        day: IsoDay,
    ) -> Result<Date<Gregorian>, DateTimeError> {
        Date::new_iso_date(year, month, day).map(|d| Date::new_from_iso(d, Gregorian))
    }
}

impl DateTime<Gregorian> {
    /// Construct a new Gregorian datetime from integers.
    ///
    /// Years are specified as ISO years.
    ///
    /// ```rust
    /// use icu::calendar::{DateTime,
    ///                     types::IsoHour,
    ///                     types::IsoMinute,
    ///                     types::IsoSecond};
    ///
    /// let datetime_gregorian = DateTime::new_gregorian_datetime_from_integers(1970, 1, 2, 13, 1, 0, 0).unwrap();
    ///
    /// assert_eq!(datetime_gregorian.date.year().number, 1970);
    /// assert_eq!(datetime_gregorian.date.month().number, 1);
    /// assert_eq!(datetime_gregorian.date.day_of_month().0, 2);
    /// assert_eq!(datetime_gregorian.time.hour, IsoHour::new_unchecked(13));
    /// assert_eq!(datetime_gregorian.time.minute, IsoMinute::new_unchecked(1));
    /// assert_eq!(datetime_gregorian.time.second, IsoSecond::new_unchecked(0));
    /// ```
    pub fn new_gregorian_datetime_from_integers(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        fraction: u32,
    ) -> Result<DateTime<Gregorian>, DateTimeError> {
        Ok(DateTime {
            date: Date::new_gregorian_date(year.into(), month.try_into()?, day.try_into()?)?,
            time: types::Time::try_new(hour, minute, second, fraction)?,
        })
    }
}

pub fn year_as_gregorian(year: i32) -> types::Year {
    if year > 0 {
        types::Year {
            era: types::Era(tinystr!(16, "ad")),
            number: year,
            related_iso: year,
        }
    } else {
        types::Year {
            era: types::Era(tinystr!(16, "bc")),
            number: 1 - year,
            related_iso: year,
        }
    }
}
