// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains types and implementations for the Ethiopic calendar

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
#[allow(clippy::exhaustive_structs)] // this type is stable
pub struct Ethiopic;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct EthiopicDateInner(ArithmeticDate<Ethiopic>, bool);

impl EthiopicDateInner {
    pub fn set_amete_alem(&self, date: &mut Self, amete_alem: bool) {
        date.1 = amete_alem;
    }
}

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
        Iso.day_of_week(Ethiopic.date_to_iso(date).inner())
    }

    fn offset_date(&self, date: &mut Self::DateInner, offset: DateDuration<Self>) {
        date.0.offset_date(offset);
    }

    #[allow(clippy::field_reassign_with_default)]
    fn until(
        &self,
        date1: &Self::DateInner,
        date2: &Self::DateInner,
        _largest_unit: DateDurationUnit,
        _smallest_unit: DateDurationUnit,
    ) -> DateDuration<Self> {
        date1.0.until(date2.0, _largest_unit, _smallest_unit)
    }

    fn year(&self, date: &Self::DateInner) -> types::Year {
        if date.1 {
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

    fn debug_name() -> &'static str {
        "Ethiopic"
    }
}

impl Ethiopic {
    /// Construct a new Ethiopic Calendar
    pub fn new() -> Self {
        Self
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
        *Date::new_ethiopic_date_from_integers(
            coptic_date.0.year,
            coptic_date.0.month,
            coptic_date.0.day,
            false,
        )
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
    /// Construct new Ethiopic Date
    pub fn new_ethiopic_date_from_integers(
        year: i32,
        month: u8,
        day: u8,
        amete_alem: bool,
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

        Ok(Date::from_raw(
            EthiopicDateInner(inner, amete_alem),
            Ethiopic,
        ))
    }
}

impl DateTime<Ethiopic> {
    /// Construct a new Ethiopic datetime from integers
    #[allow(clippy::too_many_arguments)]
    pub fn new_ethiopic_datetime_from_integers(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        fraction: u32,
        amete_alem: bool,
    ) -> Result<DateTime<Ethiopic>, DateTimeError> {
        Ok(DateTime {
            date: Date::new_ethiopic_date_from_integers(year, month, day, amete_alem)?,
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
        let ethiopic_date = Ethiopic.date_from_iso(iso_date);
        assert_eq!(ethiopic_date.0.year, 2015);
        assert_eq!(ethiopic_date.0.month, 13);
        assert_eq!(ethiopic_date.0.day, 6);
    }
}
