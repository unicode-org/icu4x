// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use tinystr::tinystr;

use crate::{iso::IsoDateInner, Calendar, CalendarError, calendar_arithmetic::ArithmeticDate, Date, Iso, types, DateTime};

/// Year of the beginning of the Taiwanese (ROC/Minguo) calendar.
/// 1912 ISO = ROC 1
const ROC_ERA_OFFSET: i32 = 1911;

/// TODO: Documentation
#[derive(Copy, Clone, Debug, Default)]
#[allow(clippy::exhaustive_structs)] // this type is stable
pub struct Roc;

/// TODO: Documentation
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct RocDateInner(IsoDateInner);

impl Calendar for Roc {
    type DateInner = RocDateInner;

    fn date_from_codes(
        &self,
        era: crate::types::Era,
        year: i32,
        month_code: crate::types::MonthCode,
        day: u8,
    ) -> Result<Self::DateInner, crate::Error> {
        let year = if era.0 == tinystr!(16, "minguo") {
            if year <= 0 {
                return Err(CalendarError::OutOfRange);
            }
            year + ROC_ERA_OFFSET
        } else if era.0 == tinystr!(16, "before minguo") {
            if year <= 0 {
                return Err(CalendarError::OutOfRange);
            }
            1 - (year + ROC_ERA_OFFSET)
        } else {
            return Err(CalendarError::UnknownEra(era.0, self.debug_name()));
        };

        ArithmeticDate::new_from_solar_codes(self, year, month_code, day)
            .map(IsoDateInner)
            .map(RocDateInner)
    }

    fn date_from_iso(&self, iso: crate::Date<crate::Iso>) -> Self::DateInner {
        RocDateInner(*iso.inner())
    }

    fn date_to_iso(&self, date: &Self::DateInner) -> crate::Date<crate::Iso> {
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

    fn offset_date(&self, date: &mut Self::DateInner, offset: crate::DateDuration<Self>) {
        Iso.offset_date(&mut date.0, offset.cast_unit())
    }

    fn until(
        &self,
        date1: &Self::DateInner,
        date2: &Self::DateInner,
        calendar2: &Self,
        largest_unit: crate::DateDurationUnit,
        smallest_unit: crate::DateDurationUnit,
    ) -> crate::DateDuration<Self> {
        Iso.until(&date1.0, &date2.0, &Iso, largest_unit, smallest_unit)
            .cast_unit()
    }

    fn debug_name(&self) -> &'static str {
        "ROC"
    }

    fn year(&self, date: &Self::DateInner) -> crate::types::FormattableYear {
        year_as_roc(date.0.0.year)
    }

    fn month(&self, date: &Self::DateInner) -> crate::types::FormattableMonth {
        Iso.month(&date.0)
    }

    fn day_of_month(&self, date: &Self::DateInner) -> crate::types::DayOfMonth {
        Iso.day_of_month(&date.0)
    }

    fn day_of_year_info(&self, date: &Self::DateInner) -> crate::types::DayOfYearInfo {
        let prev_year = date.0.0.year.saturating_sub(1);
        let next_year = date.0.0.year.saturating_add(1);
        types::DayOfYearInfo {
            day_of_year: Iso::day_of_year(date.0),
            days_in_year: Iso::days_in_year_direct(date.0.0.year),
            prev_year: year_as_roc(prev_year),
            days_in_prev_year: Iso::days_in_year_direct(prev_year),
            next_year: year_as_roc(next_year),
        }
    }
}

impl Date<Roc> {
    /// TODO: Documentation
    pub fn try_new_roc_date(
        year: i32,
        month: u8,
        day: u8,
    ) -> Result<Date<Roc>, CalendarError> {
        Date::try_new_iso_date(year, month, day).map(|d| Date::new_from_iso(d, Roc))
    }
}

impl DateTime<Roc> {
    /// TODO: Documentation
    pub fn try_new_roc_datetime(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Result<DateTime<Roc>, CalendarError> {
        Ok(DateTime {
            date: Date::try_new_roc_date(year, month, day)?,
            time: types::Time::try_new(hour, minute, second, 0)?,
        })
    }
}

pub(crate) fn year_as_roc(year: i32) -> types::FormattableYear {
    if year > ROC_ERA_OFFSET {
        types::FormattableYear {
            era: types::Era(tinystr!(16, "minguo")),
            number: year.saturating_sub(ROC_ERA_OFFSET),
            cyclic: None,
            related_iso: Some(year),
        }
    } else {
        types::FormattableYear {
            era: types::Era(tinystr!(16, "before minguo")),
            number: (ROC_ERA_OFFSET + 1).saturating_sub(year),
            cyclic: None,
            related_iso: Some(year),
        }
    }
}