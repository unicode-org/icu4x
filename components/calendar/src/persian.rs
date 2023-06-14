// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::any_calendar::AnyCalendarKind;
use crate::calendar_arithmetic::{ArithmeticDate, CalendarArithmetic};
use crate::gregorian::year_as_gregorian;
use crate::helpers::{div_rem_euclid64, i64_to_i32, quotient, quotient64, I32Result};
use crate::iso::Iso;
use crate::julian::Julian;
use crate::rata_die::RataDie;
use crate::{types, Calendar, CalendarError, Date, DateDuration, DateDurationUnit, DateTime};
use ::tinystr::tinystr;
use core::marker::PhantomData;

const PERSIAN_EPOCH: RataDie = Julian::fixed_from_julian(ArithmeticDate {
    year: (622),
    month: (3),
    day: (19),
    marker: core::marker::PhantomData,
});
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

    fn is_leap_year(mut p_year: i32) -> bool {
        if 0 < p_year {
            p_year -= 474;
        } else {
            p_year -= 473;
        };

        let year = (p_year % 2820) + 474;

        (((year + 38) * 31) % 128) < 31
    }

    fn days_in_provided_year(year: i32) -> u32 {
        if Self::is_leap_year(year) {
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

    fn fixed_from_arithmetic_persian(p_date: PersianDateInner) -> RataDie {
        let p_year = i64::from(p_date.0.year);
        let month = i64::from(p_date.0.month);
        let day = i64::from(p_date.0.day);
        let y = if p_year > 0 {
            p_year - 474
        } else {
            p_year - 473
        };
        let x = div_rem_euclid64(y, 2820);
        let year = x.1 + 474;
        let z = div_rem_euclid64(31 * year - 5, 128);

        RataDie::new(
            PERSIAN_EPOCH.to_i64_date() - 1
                + 1029983 * x.0
                + 365 * (year - 1)
                + z.0
                + if month <= 7 {
                    (31 * (month - 1))
                } else {
                    (30 * (month - 1) + 6)
                }
                + day,
        )
    }

    fn arithmetic_persian_from_fixed(date: RataDie) -> Date<Persian> {
        let year = Self::arithmetic_persian_year_from_fixed(date);
        let year = match i64_to_i32(year) {
            I32Result::BelowMin(_) => {
                return Date::from_raw(PersianDateInner(ArithmeticDate::min_date()), Persian)
            }
            I32Result::AboveMax(_) => {
                return Date::from_raw(PersianDateInner(ArithmeticDate::max_date()), Persian)
            }
            I32Result::WithinRange(y) => y,
        };
        #[allow(clippy::unwrap_used)] // valid month,day
        let day_of_year = (1_i64 + date.to_i64_date()
            - Self::fixed_from_persian_integers(year, 1, 1)
                .unwrap()
                .to_i64_date()) as i32;
        let month = if day_of_year <= 186 {
            libm::ceilf(day_of_year as f32 / 31.0) as u8
        } else {
            libm::ceilf((day_of_year as f32 - 6.0) / 30.0) as u8
        };
        #[allow(clippy::unwrap_used)] // month and day
        let day = (date - Self::fixed_from_persian_integers(year, month, 1).unwrap() + 1) as u8;
        #[allow(clippy::unwrap_used)] // valid month and day
        Date::try_new_persian_date(year, month, day).unwrap()
    }

    fn arithmetic_persian_year_from_fixed(date: RataDie) -> i64 {
        #[allow(clippy::unwrap_used)] // valid year,month,day
        let d0 = date - Self::fixed_from_persian_integers(475, 1, 1).unwrap();
        let d = div_rem_euclid64(d0, 1029983);
        let n2820 = d.0;
        let d1 = d.1;
        let y2820 = if d1 == 1029982 {
            2820
        } else {
            div_rem_euclid64(128 * d1 + 46878, 46751).0
        };
        let year = 474 + n2820 * 2820 + y2820;
        if year > 0 {
            year
        } else {
            year - 1
        }
    }

    pub(crate) fn fixed_from_persian_integers(year: i32, month: u8, day: u8) -> Option<RataDie> {
        Date::try_new_persian_date(year, month, day)
            .ok()
            .map(|d| *d.inner())
            .map(Self::fixed_from_arithmetic_persian)
    }

    // Persian New Year to fixed year
    fn nowruz(g_year: i32) -> RataDie {
        let persian_year =
            g_year - year_as_gregorian(PERSIAN_EPOCH.to_i64_date() as i32).number + 1;
        let _year = if persian_year <= 0 {
            persian_year - 1
        } else {
            persian_year
        };
        #[allow(clippy::unwrap_used)] // valid day and month
        Self::fixed_from_persian_integers(_year, 1, 1).unwrap()
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
mod tests {

    #[cfg(test)]
    use super::*;
    #[derive(Debug)]
    struct DateCase {
        year: i32,
        month: u8,
        day: u8,
    }

    static FIXED_DATE: [i64; 33] = [
        -214193, -61387, 25469, 49217, 171307, 210155, 253427, 369740, 400085, 434355, 452605,
        470160, 473837, 507850, 524156, 544676, 567118, 569477, 601716, 613424, 626596, 645554,
        664224, 671401, 694799, 704424, 708842, 709409, 709580, 727274, 728714, 744313, 764652,
    ];

    static CASES: [DateCase; 33] = [
        DateCase {
            year: -1208,
            month: 5,
            day: 1,
        },
        DateCase {
            year: -790,
            month: 9,
            day: 14,
        },
        DateCase {
            year: -552,
            month: 7,
            day: 2,
        },
        DateCase {
            year: -487,
            month: 7,
            day: 9,
        },
        DateCase {
            year: -153,
            month: 10,
            day: 18,
        },
        DateCase {
            year: -46,
            month: 2,
            day: 30,
        },
        DateCase {
            year: 73,
            month: 8,
            day: 19,
        },
        DateCase {
            year: 392,
            month: 2,
            day: 5,
        },
        DateCase {
            year: 475,
            month: 3,
            day: 3,
        },
        DateCase {
            year: 569,
            month: 1,
            day: 3,
        },
        DateCase {
            year: 618,
            month: 12,
            day: 20,
        },
        DateCase {
            year: 667,
            month: 1,
            day: 14,
        },
        DateCase {
            year: 677,
            month: 2,
            day: 8,
        },
        DateCase {
            year: 770,
            month: 3,
            day: 22,
        },
        DateCase {
            year: 814,
            month: 11,
            day: 13,
        },
        DateCase {
            year: 871,
            month: 1,
            day: 21,
        },
        DateCase {
            year: 932,
            month: 6,
            day: 28,
        },
        DateCase {
            year: 938,
            month: 12,
            day: 14,
        },
        DateCase {
            year: 1027,
            month: 3,
            day: 21,
        },
        DateCase {
            year: 1059,
            month: 4,
            day: 10,
        },
        DateCase {
            year: 1095,
            month: 5,
            day: 2,
        },
        DateCase {
            year: 1147,
            month: 3,
            day: 30,
        },
        DateCase {
            year: 1198,
            month: 5,
            day: 10,
        },
        DateCase {
            year: 1218,
            month: 1,
            day: 7,
        },
        DateCase {
            year: 1282,
            month: 1,
            day: 29,
        },
        DateCase {
            year: 1308,
            month: 6,
            day: 3,
        },
        DateCase {
            year: 1320,
            month: 7,
            day: 7,
        },
        DateCase {
            year: 1322,
            month: 1,
            day: 29,
        },
        DateCase {
            year: 1322,
            month: 7,
            day: 14,
        },
        DateCase {
            year: 1370,
            month: 12,
            day: 27,
        },
        DateCase {
            year: 1374,
            month: 12,
            day: 6,
        },
        DateCase {
            year: 1417,
            month: 8,
            day: 19,
        },
        DateCase {
            year: 1473,
            month: 4,
            day: 28,
        },
    ];
    #[test]
    fn test_persian_year_from_fixed() {
        for (case, f_date) in CASES.iter().zip(FIXED_DATE.iter()) {
            let date = PersianDateInner(ArithmeticDate {
                year: (case.year),
                month: (case.month),
                day: (case.day),
                marker: (PhantomData),
            });
            assert_eq!(
                date.0.year as i64,
                Persian::arithmetic_persian_year_from_fixed(RataDie::new(*f_date)),
                "{case:?}"
            );
        }
    }

    #[test]
    fn test_fixed_from_persian() {
        for (case, f_date) in CASES.iter().zip(FIXED_DATE.iter()) {
            let date = PersianDateInner(ArithmeticDate {
                year: (case.year),
                month: (case.month),
                day: (case.day),
                marker: (PhantomData),
            });

            assert_eq!(
                Persian::fixed_from_arithmetic_persian(date).to_i64_date(),
                *f_date,
                "{case:?}"
            );
        }
    }

    #[test]
    fn test_persian_from_fixed() {
        for (case, f_date) in CASES.iter().zip(FIXED_DATE.iter()) {
            let date = Date::try_new_persian_date(case.year, case.month, case.day).unwrap();
            assert_eq!(
                Persian::arithmetic_persian_from_fixed(RataDie::new(*f_date)),
                date,
                "{case:?}"
            );
        }
    }

    //#[test]
    // fn test_persian_epoch() {

    //     assert_eq!(PERSIAN_EPOCH.to_i64_date(), -475);
    // }
}
