// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::marker::PhantomData;

use crate::any_calendar::AnyCalendarKind;
use crate::calendar_arithmetic::{ArithmeticDate, CalendarArithmetic};
use crate::gregorian::year_as_gregorian;
use crate::iso::Iso;
use crate::julian::Julian;
use crate::rata_die::RataDie;
use crate::{types, Calendar, CalendarError, Date, DateDuration, DateDurationUnit, DateTime};
use ::tinystr::tinystr;

const PERSIAN_EPOCH: i64 = Julian::fixed_from_julian(ArithmeticDate {
    year: (622),
    month: (3),
    day: (19),
    marker: core::marker::PhantomData,
})
.to_i64_date();
/// The Persian Calendar
///
/// The [Persian Calendar] is a solar calendar used officially by the countries of Iran and Afghanistan and many Persian-speaking regions.
/// It has 12 months and other similarities to the Gregorian Calendar
///
/// This type can be used with [`Date`] or [`DateTime`] to represent dates in this calendar.
///
/// [Persian Calendar]: https://en.wikipedia.org/wiki/Solar_Hijri_calendar
///
/// # Era codes
/// This calendar supports two era codes: '"BH"', and '"AH"', which correspond to the dates before the Hijrah and after the Hijrah

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct Persian;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct PersianDateInner(ArithmeticDate<Persian>);

impl CalendarArithmetic for Persian {
    fn month_days(year: i32, month: u8) -> u8 {
        match month {
            1 | 2 | 3 | 4 | 5 | 6 => 31,
            7 | 8 | 9 | 10 | 11 => 30,
            12 if Self::is_leap_year(year) => 30,
            12 => 29,
            _ => 0,
        }
    }

    fn months_for_every_year(_: i32) -> u8 {
        12
    }

    fn is_leap_year(p_year: i32) -> bool {
        if 0 < p_year {
            p_year = p_year - 474;
        } else {
            p_year = p_year - 473;
        };

        year = (p_year % 2820) + 474;

        (((year + 38) * 31) % 128) < 31
    }

    fn days_in_provided_year(year: i32) -> u32 {
        if Self::is_leayear(year) {
            366
        } else {
            365
        }
    }

    fn last_month_day_in_year(year: i32) -> (u8, u8) {
        if Self::is_leap_year(year) {
            (12, 30)
        } else {
            (12, 29)
        }
    }
}

impl Calendar for Persian {
    type DateInner = PersianDateInner;
    fn date_from_codes(
        &self,
        era: types::Era,
        year: i32,
        month_code: types::MonthCode,
        day: u8,
    ) -> Result<Self::DateInner, CalendarError> {
        let year = if era.0 == tinystr!(16, "ah") {
            if year <= 0 {
                return Err(CalendarError::OutOfRange);
            }
            year
        } else if era.0 == tinystr!(16, "bh") {
            if year <= 0 {
                return Err(CalendarError::OutOfRange);
            }
            1 - year
        } else {
            return Err(CalendarError::UnknownEra(era.0, self.debug_name()));
        };

        ArithmeticDate::new_from_solar(self, year, month_code, day).map(PersianDateInner)
    }

    fn date_from_iso(&self, iso: Date<Iso>) -> PersianDateInner {
        let fixed_iso = Iso::fixed_from_iso(*iso.inner());
        Self::arithmetic_persian_from_fixed(fixed_iso).inner
    }

    fn date_to_iso(&self, date: &Self::DateInner) -> Date<Iso> {
        let fixed_persian = Persian::fixed_from_arithmetic_persian(*date);
        Iso::iso_from_fixed(fixed_persian)
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
        date.0.offset_date(offset)
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
        Self::year_as_persian(date.0.year)
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
            prev_year: Persian::year_as_persian(prev_year),
            days_in_prev_year: Persian::days_in_year_direct(prev_year),
            next_year: Persian::year_as_persian(next_year),
        }
    }

    fn debug_name(&self) -> &'static str {
        "Persian"
    }
    // Missing any_calendar persian tests, the rest is completed
    fn any_calendar_kind(&self) -> Option<AnyCalendarKind> {
        Some(AnyCalendarKind::Persian)
    }
}

impl Persian {
    pub fn new() -> Self {
        Self
    }

    fn fixed_from_arithmetic_persian(date: PersianDateInner) -> RataDie {
        let mut month = date.0.month;

        let mut day = date.0.day;

        let mut year: i32 = if 0 < date.0.year {
            date.0.year - 474
        } else {
            date.0.year - 473
        };
        year = (year % 2820) + 474;

        let result: i32 = 1_948_320 - 1
            + (1029983 * (year / 2820))
            + (365 * (year - 1))
            + ((31 * year - 5) / 128)
            + if month <= 7 {
                31 * (month as i32 - 1)
            } else {
                30 * (month as i32 - 1) + 6
            }
            + day as i32;

        RataDie::new(result as i64)
    }

    fn arithmetic_persian_from_fixed(date: RataDie) -> Date<Persian> {
        let year = Self::arithmetic_persian_year_from_fixed(date);
        let day_of_year = (date
            - Self::fixed_from_arithmetic_persian(PersianDateInner(ArithmeticDate {
                year: (year),
                month: (1),
                day: 1,
                marker: (core::marker::PhantomData),
            })))
            + 1;
        let month = if day_of_year <= 186 {
            ((day_of_year as f64 / 31.0).ceil()) as u8
        } else {
            (((day_of_year - 6) as f64 / 30.0).ceil()) as u8
        };
        let day = (date.to_i64_date()
            - (Self::fixed_from_arithmetic_persian(PersianDateInner(ArithmeticDate {
                year: year,
                month: month,
                day: 1,
                marker: core::marker::PhantomData,
            })) - 1)
                .to_i64_date()) as u8;

        Date::try_new_persian_date(year, month, day).unwrap()
    }

    fn arithmetic_persian_year_from_fixed(date: RataDie) -> i32 {
        let d0 = date
            - Self::fixed_from_arithmetic_persian(PersianDateInner(ArithmeticDate {
                year: 475,
                month: 1,
                day: 1,
                marker: PhantomData,
            }));
        let last_day_of_cycle = 1029983;
        let n2820 = d0 / last_day_of_cycle;
        let d1 = d0 % last_day_of_cycle;
        let y2820 = if d1 == last_day_of_cycle - 1 {
            2820
        } else {
            (d1 * 128 + 46878) / 46751
        };
        let year: i32 = (474 + (2820 * n2820) + y2820).try_into().unwrap();

        if year > 0 {
            year
        } else {
            year - 1
        }
    }

    // Persian New Year to fixed year
    fn nowruz(g_year: i32) -> RataDie {
        let persian_year = g_year - year_as_gregorian(PERSIAN_EPOCH as i32).number + 1;
        let year = if persian_year <= 0 {
            persian_year - 1
        } else {
            persian_year
        };
        Self::fixed_from_arithmetic_persian(PersianDateInner(ArithmeticDate {
            year: year,
            month: 1,
            day: 1,
            marker: core::marker::PhantomData,
        }))
    }

    fn days_in_year_direct(year: i32) -> u32 {
        if Persian::is_leap_year(year) {
            366
        } else {
            365
        }
    }

    fn year_as_persian(year: i32) -> types::FormattableYear {
        if year > 0 {
            types::FormattableYear {
                era: types::Era(tinystr!(16, "ah")),
                number: year,
                related_iso: None,
            }
        } else {
            types::FormattableYear {
                era: types::Era(tinystr!(16, "bh")),
                number: year,
                related_iso: None,
            }
        }
    }
}

impl Date<Persian> {
    pub fn try_new_persian_date(
        year: i32,
        month: u8,
        day: u8,
    ) -> Result<Date<Persian>, CalendarError> {
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
        Ok(Date::from_raw(PersianDateInner(inner), Persian))
    }
}

impl DateTime<Persian> {
    pub fn try_new_persian_datetime(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Result<DateTime<Persian>, CalendarError> {
        Ok(DateTime {
            date: Date::try_new_persian_date(year, month, day)?,
            time: types::Time::try_new(hour, minute, second, 0)?,
        })
    }
}