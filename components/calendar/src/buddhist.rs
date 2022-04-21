// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains types and implementations for the Buddhist calendar

use crate::iso::{Iso, IsoDateInner, IsoDay, IsoMonth, IsoYear};
use crate::{types, Calendar, Date, DateDuration, DateDurationUnit, DateTime, DateTimeError};
use core::convert::TryInto;
use tinystr::tinystr;

/// The number of years the Buddhist Era is ahead of C.E. by
///
/// (1 AD = 544 BE)
const BUDDHIST_ERA_OFFSET: i32 = 543;

#[derive(Copy, Clone, Debug, Default)]
/// The [Thai Solar Buddhist Calendar][cal]
///
/// This is basically the same as the Gregorian calendar,
/// however it has a different zero year: 1 AD = 544 BE
///
/// [cal]: https://en.wikipedia.org/wiki/Thai_solar_calendar
pub struct Buddhist;

impl Calendar for Buddhist {
    type DateInner = IsoDateInner;
    fn date_from_iso(&self, iso: Date<Iso>) -> IsoDateInner {
        *iso.inner()
    }

    fn date_to_iso(&self, date: &Self::DateInner) -> Date<Iso> {
        Date::from_raw(*date, Iso)
    }

    fn months_in_year(&self, date: &Self::DateInner) -> u8 {
        Iso.months_in_year(date)
    }

    fn days_in_year(&self, date: &Self::DateInner) -> u32 {
        Iso.days_in_year(date)
    }

    fn days_in_month(&self, date: &Self::DateInner) -> u8 {
        Iso.days_in_month(date)
    }

    fn offset_date(&self, date: &mut Self::DateInner, offset: DateDuration<Self>) {
        Iso.offset_date(date, offset.cast_unit())
    }

    #[allow(clippy::field_reassign_with_default)] // it's more clear this way
    fn until(
        &self,
        date1: &Self::DateInner,
        date2: &Self::DateInner,
        largest_unit: DateDurationUnit,
        smallest_unit: DateDurationUnit,
    ) -> DateDuration<Self> {
        Iso.until(date1, date2, largest_unit, smallest_unit)
            .cast_unit()
    }

    /// The calendar-specific year represented by `date`
    fn year(&self, date: &Self::DateInner) -> types::Year {
        iso_year_as_buddhist(date.year)
    }

    /// The calendar-specific month represented by `date`
    fn month(&self, date: &Self::DateInner) -> types::Month {
        Iso.month(date)
    }

    /// The calendar-specific day-of-month represented by `date`
    fn day_of_month(&self, date: &Self::DateInner) -> types::DayOfMonth {
        Iso.day_of_month(date)
    }

    /// Information of the day of the year
    fn day_of_year_info(&self, date: &Self::DateInner) -> types::DayOfYearInfo {
        let prev_year = IsoYear(date.year.0 - 1);
        let next_year = IsoYear(date.year.0 + 1);
        types::DayOfYearInfo {
            day_of_year: Iso::day_of_year(*date),
            days_in_year: Iso::days_in_year(date.year),
            prev_year: iso_year_as_buddhist(prev_year),
            days_in_prev_year: Iso::days_in_year(prev_year),
            next_year: iso_year_as_buddhist(next_year),
        }
    }

    fn debug_name() -> &'static str {
        "Buddhist"
    }
}

impl Date<Buddhist> {
    /// Construct a new Buddhist Date
    pub fn new_buddhist_date(
        year: IsoYear,
        month: IsoMonth,
        day: IsoDay,
    ) -> Result<Date<Buddhist>, DateTimeError> {
        Date::new_iso_date(year, month, day).map(|d| Date::new_from_iso(d, Buddhist))
    }
}

impl DateTime<Buddhist> {
    /// Construct a new Buddhist datetime from integers
    ///
    /// Years are specified as BE years
    pub fn new_buddhist_datetime_from_integers(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Result<DateTime<Buddhist>, DateTimeError> {
        let iso_year = year - BUDDHIST_ERA_OFFSET;
        Ok(DateTime {
            date: Date::new_buddhist_date(iso_year.into(), month.try_into()?, day.try_into()?)?,
            time: types::Time::try_new(hour, minute, second, 0)?,
        })
    }
}

fn iso_year_as_buddhist(year: IsoYear) -> types::Year {
    let buddhist_year = year.0 + BUDDHIST_ERA_OFFSET;
    types::Year {
        era: types::Era(tinystr!(16, "be")),
        number: buddhist_year,
        related_iso: year.0,
    }
}
