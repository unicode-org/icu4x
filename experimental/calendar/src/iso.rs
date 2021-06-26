// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{Calendar, Date, DateDuration, DurationUnit, Error};
use std::convert::{TryFrom, TryInto};

#[derive(Copy, Clone, Debug, Default)]
/// The ISO Calendar
pub struct Iso;

/// A 1-indexed representation of an ISO day
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct IsoDay(u8);
/// A 1-indexed representation of an ISO month
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct IsoMonth(u8);
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
/// An ISO year. Year 0 == 1 BCE
pub struct IsoYear(pub i32);

impl TryFrom<u8> for IsoDay {
    type Error = Error;
    fn try_from(int: u8) -> Result<Self, Error> {
        if !(1..=31).contains(&int) {
            return Err(Error::OutOfRange);
        }
        Ok(Self(int))
    }
}

impl TryFrom<u8> for IsoMonth {
    type Error = Error;
    fn try_from(int: u8) -> Result<Self, Error> {
        if !(1..=12).contains(&int) {
            return Err(Error::OutOfRange);
        }
        Ok(Self(int))
    }
}

impl From<i32> for IsoYear {
    fn from(int: i32) -> Self {
        Self(int)
    }
}

impl From<IsoDay> for u8 {
    fn from(day: IsoDay) -> Self {
        day.0
    }
}

impl From<IsoMonth> for u8 {
    fn from(month: IsoMonth) -> Self {
        month.0
    }
}

impl From<IsoYear> for i32 {
    fn from(year: IsoYear) -> Self {
        year.0
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct IsoDateInner {
    day: IsoDay,
    month: IsoMonth,
    year: IsoYear,
}

impl IsoDateInner {
    fn add_months(&mut self, months: i32) {
        // Get a zero-indexed new month
        let new_month = (self.month.0 as i32 - 1) + months;
        if new_month >= 0 {
            self.year.0 += new_month / 12;
            self.month.0 = ((new_month % 12) + 1) as u8;
        } else {
            // subtract full years
            self.year.0 -= (-new_month) / 12;
            // subtract a partial year
            self.year.0 -= 1;
            // adding 13 since months are 1-indexed
            self.month.0 = (13 + (new_month % 12)) as u8
        }
    }
}

impl Calendar for Iso {
    type DateInner = IsoDateInner;
    fn date_from_iso(&self, iso: Date<Iso>) -> IsoDateInner {
        *iso.inner()
    }

    fn date_to_iso(&self, date: &Self::DateInner) -> Date<Iso> {
        Date::from_raw(*date, Iso)
    }

    fn months_in_year(&self, _date: &Self::DateInner) -> u8 {
        12
    }

    fn days_in_year(&self, date: &Self::DateInner) -> u32 {
        if Self::is_leap_year(date.year) {
            366
        } else {
            365
        }
    }

    fn days_in_month(&self, date: &Self::DateInner) -> u8 {
        Self::days_in_month(date.year, date.month)
    }

    fn day_of_week(&self, date: &Self::DateInner) -> u8 {
        // TODO (Manishearth) share code with icu_datetime

        // For the purposes of the calculation here, Monday is 0, Sunday is 6
        // ISO has Monday=1, Sunday=7, which we transform in the last step

        // The days of the week are the same every 400 years
        // so we normalize to the nearest multiple of 400
        let years_since_400 = date.year.0 % 400;
        let leap_years_since_400 = years_since_400 / 4 - years_since_400 / 100;
        // The number of days to the current year
        let days_to_current_year = 365 * years_since_400 + leap_years_since_400;
        // The weekday offset from January 1 this year and January 1 2000
        let year_offset = days_to_current_year % 7;

        // Corresponding months from
        // https://en.wikipedia.org/wiki/Determination_of_the_day_of_the_week#Corresponding_months
        let month_offset = if Self::is_leap_year(date.year) {
            match date.month.0 {
                1 | 4 | 7 => 0,
                10 => 1,
                5 => 2,
                2 | 8 => 3,
                3 | 11 => 4,
                6 => 5,
                9 | 12 => 6,
                _ => unreachable!(),
            }
        } else {
            match date.month.0 {
                1 | 10 => 0,
                5 => 1,
                8 => 2,
                2 | 3 | 11 => 3,
                6 => 4,
                9 | 12 => 5,
                4 | 7 => 6,
                _ => unreachable!(),
            }
        };

        let january_1_2000 = 5; // Saturday
        let day_offset = (january_1_2000 + year_offset + month_offset + date.day.0 as i32) % 7;

        // We calculated in a zero-indexed fashion, but ISO specifies one-indexed
        (day_offset + 1) as u8
    }

    fn offset_date(&self, date: &mut Self::DateInner, mut offset: DateDuration<Self>) {
        date.year.0 += offset.years;
        date.add_months(offset.months);
        offset.months = 0;

        offset.days += offset.weeks * 7;

        // Normalize date to beginning of month
        offset.days += date.day.0 as i32 - 1;
        date.day.0 = 1;

        while offset.days != 0 {
            if offset.days < 0 {
                date.add_months(-1);
                let month_days = self.days_in_month(date);
                if (-offset.days) > month_days as i32 {
                    offset.days += month_days as i32;
                } else {
                    // Add 1 since we are subtracting from the first day of the
                    // *next* month
                    date.day.0 = 1 + (month_days as i8 + offset.days as i8) as u8;
                    offset.days = 0;
                }
            } else {
                let month_days = self.days_in_month(date);
                if offset.days > month_days as i32 {
                    date.add_months(1);
                    offset.days -= month_days as i32;
                } else {
                    date.day.0 += offset.days as u8;
                    offset.days = 0;
                }
            }
        }
    }

    fn until(
        &self,
        date1: &Self::DateInner,
        date2: &Self::DateInner,
        _largest_unit: DurationUnit,
        _smallest_unit: DurationUnit,
    ) -> DateDuration<Self> {
        let mut difference = DateDuration::default();
        // TODO (Manishearth) handle the unit bounds and rounding behavior
        // (perhaps share code with icu_datetime)
        difference.years = date1.year.0 - date2.year.0;
        difference.months = date1.month.0 as i32 - date2.month.0 as i32;
        difference.days = date1.day.0 as i32 - date2.day.0 as i32;

        difference
    }

    fn debug_name() -> &'static str {
        "ISO"
    }
}

impl Date<Iso> {
    pub fn new_iso_date(day: IsoDay, month: IsoMonth, year: IsoYear) -> Result<Date<Iso>, Error> {
        if day.0 > 28 {
            let bound = Iso::days_in_month(year, month);
            if day.0 < bound {
                return Err(Error::OutOfRange);
            }
        }

        Ok(Date::from_raw(IsoDateInner { day, month, year }, Iso))
    }
    pub fn new_iso_date_from_integers(day: u8, month: u8, year: i32) -> Result<Date<Iso>, Error> {
        Self::new_iso_date(day.try_into()?, month.try_into()?, year.into())
    }
}

impl Iso {
    pub fn new() -> Self {
        Self
    }

    pub fn is_leap_year(year: IsoYear) -> bool {
        year.0 % 4 == 0 && (year.0 % 400 == 0 || year.0 % 100 != 0)
    }

    fn days_in_month(year: IsoYear, month: IsoMonth) -> u8 {
        match month.0 {
            4 | 6 | 9 | 11 => 30,
            2 if Self::is_leap_year(year) => 29,
            2 => 28,
            _ => 31,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day_of_week() {
        // June 23, 2021 is a Wednesday
        assert_eq!(
            Date::new_iso_date_from_integers(23, 6, 2021)
                .unwrap()
                .day_of_week(),
            3
        );
        // Feb 2, 1983 was a Wednesday
        assert_eq!(
            Date::new_iso_date_from_integers(2, 2, 1983)
                .unwrap()
                .day_of_week(),
            3
        );
    }

    fn simple_subtract(a: &Date<Iso>, b: &Date<Iso>) -> DateDuration<Iso> {
        let a = a.inner();
        let b = b.inner();
        DateDuration::new(
            a.year.0 - b.year.0,
            a.month.0 as i32 - b.month.0 as i32,
            0,
            a.day.0 as i32 - b.day.0 as i32,
        )
    }

    #[test]
    fn test_offset() {
        let today = Date::new_iso_date_from_integers(23, 6, 2021).unwrap();
        let today_plus_5000 = Date::new_iso_date_from_integers(2, 3, 2035).unwrap();
        let offset = today.clone().added(DateDuration::new(0, 0, 0, 5000));
        assert_eq!(offset, today_plus_5000);
        let offset = today
            .clone()
            .added(simple_subtract(&today_plus_5000, &today));
        assert_eq!(offset, today_plus_5000);

        let today = Date::new_iso_date_from_integers(23, 6, 2021).unwrap();
        let today_minus_5000 = Date::new_iso_date_from_integers(15, 10, 2007).unwrap();
        let offset = today.clone().added(DateDuration::new(0, 0, 0, -5000));
        assert_eq!(offset, today_minus_5000);
        let offset = today
            .clone()
            .added(simple_subtract(&today_minus_5000, &today));
        assert_eq!(offset, today_minus_5000);
    }
}
