// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains types and implementations for the Coptic calendar

#[derive(Copy, Clone, Debug, Default)]
// The Coptic Calendar
pub struct Coptic;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct CopticDateInner(ArithmeticDate<Coptic>);

impl CalendarArithmetic for Coptic {
    fn month_lengths(year: i32) -> [u8; 13] {
        let mut months = [30, 30, 30, 30, 30, 30, 30, 30, 30, 30];
        if Self::is_leap_year(year) {
            months[12] += 1;
        }
        months
    }

    fn months_for_every_year() -> u8 {
        13
    }

    fn is_leap_year(year: i32) -> bool {
        year % 4 == 0
    }
}

impl Calendar for Coptic {
    type DateInner = CopticDateInner;
    fn date_from_iso(&self, iso: Date<Iso>) -> CopticDateInner {
        let fixed_iso = Iso::fixed_from_iso(*iso.inner());
    }

    fn date_to_iso(&self, date: &Self::DateInner) -> Date<Iso> {}

    fn months_in_year(&self, date: &Self::DateInner) -> u8 {
        date.0.months_in_year()
    }

    fn days_in_year(&self, date: &Self::DateInner) -> u32 {
        date.0.days_in_year()
    }

    fn days_in_month(&self, date: &Self::DateInner) -> u8 {
        date.0.days_in_month()
    }

    fn day_of_week(&self, date: &Self::DateInner) -> types::IsoWeekday {}

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
        crate::georgian::year_as_gregorian(date.0.year)
    }

    fn month(&self, date: &Self::DateInner) -> types::Month {
        types::Month {
            number: date.0.month.into(),
            code: types::MonthCode(tinystr!(8, "TODO")),
        }
    }
}
