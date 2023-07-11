// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::any_calendar::AnyCalendarKind;
use crate::calendar_arithmetic::{ArithmeticDate, CalendarArithmetic};
use crate::helpers::{div_rem_euclid, div_rem_euclid_f64, next};
use crate::julian::Julian;
use crate::rata_die::RataDie;
use crate::types::Moment;
use crate::{astronomy::*, helpers, Iso};
use crate::{types, Calendar, CalendarError, Date, DateDuration, DateDurationUnit, DateTime};
use ::tinystr::tinystr;
use libm::floor;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct IslamicObservational;
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct IslamicCivil;
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct UmmalQura;
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct IslamicTabular;

const FIXED_ISLAMIC_EPOCH_FRIDAY: RataDie = Julian::fixed_from_julian_integers(622, 7, 16);
const FIXED_ISLAMIC_EPOCH_THURSDAY: RataDie = Julian::fixed_from_julian_integers(622, 7, 15);

const CAIRO: Location = Location {
    latitude: 30.1,
    longitude: 31.3,
    elevation: 200.0,
    zone: (1_f64 / 12_f64),
};

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
/// The inner date type used for representing [`Date`]s of [`IslamicObservational`]. See [`Date`] and [`IslamicObservational`] for more details.
pub struct IslamicDateInner(ArithmeticDate<IslamicObservational>);

// OBSERVATIONAL CALENDAR

impl CalendarArithmetic for IslamicObservational {
    fn month_days(year: i32, month: u8) -> u8 {

        let midmonth = FIXED_ISLAMIC_EPOCH_FRIDAY.to_f64_date()
            + (((year - 1) as f64) * 12.0 + month as f64 - 0.5) * MEAN_SYNODIC_MONTH;

        let f_date = Astronomical::phasis_on_or_before(RataDie::new(midmonth as i64), CAIRO);

        Astronomical::month_length(f_date, CAIRO)
    }

    fn months_for_every_year(_: i32) -> u8 {
        12
    }

    fn days_in_provided_year(year: i32) -> u32 {
        355
    }

    // As an observational-lunar calendar, it does not have leap years.
    fn is_leap_year(year: i32) -> bool {
        false
    }

    fn last_month_day_in_year(year: i32) -> (u8, u8) {
        if Self::is_leap_year(year) {
            (12, 30)
        } else {
            (12, 29)
        }
    }
}

impl Calendar for IslamicObservational {
    type DateInner = IslamicDateInner;

    fn date_from_codes(
        &self,
        era: types::Era,
        year: i32,
        month_code: types::MonthCode,
        day: u8,
    ) -> Result<Self::DateInner, CalendarError> {
        let year = if era.0 == tinystr!(16, "ah") {
            year
        } else {
            return Err(CalendarError::UnknownEra(era.0, self.debug_name()));
        };

        ArithmeticDate::new_from_solar_codes(self, year, month_code, day).map(IslamicDateInner)
    }

    fn date_from_iso(&self, iso: Date<crate::Iso>) -> Self::DateInner {
        let fixed_iso = Iso::fixed_from_iso(*iso.inner());
        Self::islamic_from_fixed(fixed_iso).inner
    }

    fn date_to_iso(&self, date: &Self::DateInner) -> Date<crate::Iso> {
        let fixed_islamic = Self::fixed_from_islamic(*date);
        Iso::iso_from_fixed(fixed_islamic)
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

    fn debug_name(&self) -> &'static str {
        "IslamicObservational"
    }

    fn year(&self, date: &Self::DateInner) -> types::FormattableYear {
        Self::year_as_islamic(date.0.year)
    }
    // Change later: Islamic uses lunar months
    fn month(&self, date: &Self::DateInner) -> types::FormattableMonth {
        date.0.solar_month()
    }

    fn day_of_month(&self, date: &Self::DateInner) -> types::DayOfMonth {
        date.0.day_of_month()
    }

    fn day_of_year_info(&self, date: &Self::DateInner) -> types::DayOfYearInfo {
        let prev_year = date.0.year.saturating_sub(1);
        let next_year = date.0.year.saturating_add(1);
        types::DayOfYearInfo {
            day_of_year: date.0.day_of_year(),
            days_in_year: date.0.days_in_year(),
            prev_year: Self::year_as_islamic(prev_year),
            days_in_prev_year: Self::days_in_provided_year(prev_year),
            next_year: Self::year_as_islamic(next_year),
        }
    }
    // TODO: ADD TO ANYCALENDAR
    // fn any_calendar_kind(&self) -> Option<AnyCalendarKind> {
    //     Some(AnyCalendarKind::IslamicObservational)
    // }
}

impl IslamicObservational {
    pub fn new() -> Self {
        Self
    }

    fn fixed_from_islamic(i_date: IslamicDateInner) -> RataDie {
        let year = i64::from(i_date.0.year);
        let month = i64::from(i_date.0.month);
        let day = i64::from(i_date.0.day);

        let midmonth = FIXED_ISLAMIC_EPOCH_FRIDAY.to_f64_date()
            + (((year - 1) as f64) * 12.0 + month as f64 - 0.5) * MEAN_SYNODIC_MONTH;
        let date = Astronomical::phasis_on_or_before(RataDie::new(midmonth as i64), CAIRO)
            + day.into()
            - 1;

        date
    }

    fn islamic_from_fixed(date: RataDie) -> Date<IslamicObservational> {
        let crescent = Astronomical::phasis_on_or_before(date, CAIRO);
        let elapsed_months = (crescent - FIXED_ISLAMIC_EPOCH_FRIDAY) as f64 / MEAN_SYNODIC_MONTH;

        let year = libm::floor((elapsed_months / 12.0) + 1.0) as i32;
        let month = ((elapsed_months % 12.0) + 1.0) as u8;
        let day = (date - crescent + 1) as u8;

        Date::try_new_islamic_date(year, month, day).unwrap()
    }

    pub(crate) fn fixed_from_islamic_integers(year: i32, month: u8, day: u8) -> Option<RataDie> {
        Date::try_new_islamic_date(year, month, day)
        .ok()
        .map(|d| *d.inner())
        .map(Self::fixed_from_islamic)
    }

    fn year_as_islamic(year: i32) -> types::FormattableYear {
        types::FormattableYear {
            era: types::Era(tinystr!(16, "ah")),
            number: year,
            cyclic: None,
            related_iso: None,
        }
    }
}

impl Date<IslamicObservational> {
    /// Construct new Islamic Observational Date.
    ///
    /// Has no negative years, only era is the AH.
    ///
    /// ```rust
    /// use icu::calendar::Date;
    ///
    /// let date_islamic = Date::try_new_islamic_date(1392, 4, 25)
    ///     .expect("Failed to initialize Islamic Date instance.");
    ///
    /// assert_eq!(date_islamic.year().number, 1392);
    /// assert_eq!(date_islamic.month().ordinal, 4);
    /// assert_eq!(date_islamic.day_of_month().0, 25);
    /// ```
    pub fn try_new_islamic_date(
        year: i32,
        month: u8,
        day: u8,
    ) -> Result<Date<IslamicObservational>, CalendarError> {
        ArithmeticDate::new_from_solar_ordinals(year, month, day)
            .map(IslamicDateInner)
            .map(|inner| Date::from_raw(inner, IslamicObservational))
    }
}

impl DateTime<IslamicObservational> {
    pub fn try_new_islamic_datetime(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Result<DateTime<IslamicObservational>, CalendarError> {
        Ok(DateTime {
            date: Date::try_new_islamic_date(year, month, day)?,
            time: types::Time::try_new(hour, minute, second, 0)?,
        })
    }
}

// // UMM AL QURA CALENDAR

// #[derive(Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
// /// The inner date type used for representing [`Date`]s of [`Islamic`]. See [`Date`] and [`Islamic`] for more details.
// pub struct UmmalQuraDateInner(ArithmeticDate<UmmalQura>);

// impl CalendarArithmetic for UmmalQura {
//     fn month_days(year: i32, month: u8) -> u8 {
//         let fixed_date = Self::fixed_from_saudi_islamic(UmmalQuraDateInner(
//             (ArithmeticDate::new_from_solar_ordinals(year, month, 1)).unwrap(),
//         ));
//         u8::default()
//     }

//     fn months_for_every_year(year: i32) -> u8 {
//         todo!()
//     }

//     fn is_leap_year(year: i32) -> bool {
//         todo!()
//     }

//     fn last_month_day_in_year(year: i32) -> (u8, u8) {
//         todo!()
//     }

//     fn days_in_provided_year(year: i32) -> u32 {
//         let months_in_year = Self::months_for_every_year(year);
//         let mut days: u32 = 0;
//         for month in 1..=months_in_year {
//             days += Self::month_days(year, month) as u32;
//         }
//         days
//     }
// }

// impl Calendar for UmmalQura {
//     type DateInner = UmmalQuraDateInner;

//     fn date_from_codes(
//         &self,
//         era: types::Era,
//         year: i32,
//         month_code: types::MonthCode,
//         day: u8,
//     ) -> Result<Self::DateInner, CalendarError> {
//         todo!()
//     }

//     fn date_from_iso(&self, iso: Date<crate::Iso>) -> Self::DateInner {
//         todo!()
//     }

//     fn date_to_iso(&self, date: &Self::DateInner) -> Date<crate::Iso> {
//         todo!()
//     }

//     fn months_in_year(&self, date: &Self::DateInner) -> u8 {
//         todo!()
//     }

//     fn days_in_year(&self, date: &Self::DateInner) -> u32 {
//         todo!()
//     }

//     fn days_in_month(&self, date: &Self::DateInner) -> u8 {
//         todo!()
//     }

//     fn offset_date(&self, date: &mut Self::DateInner, offset: DateDuration<Self>) {
//         todo!()
//     }

//     fn until(
//         &self,
//         date1: &Self::DateInner,
//         date2: &Self::DateInner,
//         calendar2: &Self,
//         largest_unit: DateDurationUnit,
//         smallest_unit: DateDurationUnit,
//     ) -> DateDuration<Self> {
//         todo!()
//     }

//     fn debug_name(&self) -> &'static str {
//         todo!()
//     }

//     fn year(&self, date: &Self::DateInner) -> types::FormattableYear {
//         todo!()
//     }

//     fn month(&self, date: &Self::DateInner) -> types::FormattableMonth {
//         todo!()
//     }

//     fn day_of_month(&self, date: &Self::DateInner) -> types::DayOfMonth {
//         todo!()
//     }

//     fn day_of_year_info(&self, date: &Self::DateInner) -> types::DayOfYearInfo {
//         todo!()
//     }
// }

// impl UmmalQura {
//     fn saudi_criterion(date: RataDie) -> bool {
//         let prev_day = date - 1;
//         let set = Astronomical::sunset(Moment::new(prev_day.to_f64_date()), MECCA).unwrap();
//         let tee = Location::universal_from_standard(set, MECCA);
//         let phase = Astronomical::lunar_phase(tee);

//         0.0 < phase
//             && 90.0 > Astronomical::moonlag(Moment::new(prev_day.to_f64_date()), MECCA).unwrap()
//     }

//     fn saudi_new_month_on_or_before(date: RataDie) -> RataDie {
//         let moon = Astronomical::lunar_phase_at_or_before(0.0, Moment::new(date.to_f64_date()));
//         let age = date - moon.as_rata_die();

//         let tau = if age <= 3 && !Self::saudi_criterion(date) {
//             moon - 30.0
//         } else {
//             moon
//         };

//         next(tau.as_rata_die(), MECCA, Self::saudi_criterion)
//     }

//     fn fixed_from_saudi_islamic(date: UmmalQuraDateInner) -> RataDie {
//         let year = date.0.year;
//         let month = date.0.month;
//         let day = date.0.day;
//         let midmonth = FIXED_ISLAMIC_EPOCH_FRIDAY
//             + floor(((year - 1) as f64 * 12.0 + (month as f64) - 0.5) * MEAN_SYNODIC_MONTH) as i64;
//         let date = Self::saudi_new_month_on_or_before(midmonth) + day.into() - 1;

//         date
//     }

//     fn saudi_islamic_from_fixed(date: RataDie) -> Date<UmmalQura> {
//         let crescent = Self::saudi_new_month_on_or_before(date);
//         let elapsed_months_f64 = libm::round((crescent - FIXED_ISLAMIC_EPOCH_FRIDAY) as f64 / MEAN_SYNODIC_MONTH);
//         let elapsed_months = libm::round(elapsed_months_f64) as u32;
//         let year = ((elapsed_months / 12) + 1) as i32;
//         let month = ((elapsed_months % 12)+ 1) as u8;
//         let day = ((date - crescent) + 1) as u8;

//         Date::try_new_ummalqura_date(year, month, day).unwrap()
//     }
// }

// impl Date<UmmalQura> {
//     pub fn try_new_ummalqura_date(
//         year: i32,
//         month: u8,
//         day: u8,
//     ) -> Result<Date<UmmalQura>, CalendarError> {
//         ArithmeticDate::new_from_solar_ordinals(year, month, day)
//             .map(UmmalQuraDateInner)
//             .map(|inner| Date::from_raw(inner, UmmalQura))
//     }
// }

// impl DateTime<UmmalQura> {
//     pub fn try_new_ummalqura_datetime(
//         year: i32,
//         month: u8,
//         day: u8,
//         hour: u8,
//         minute: u8,
//         second: u8,
//     ) -> Result<DateTime<UmmalQura>, CalendarError> {
//         Ok(DateTime {
//             date: Date::try_new_ummalqura_date(year, month, day)?,
//             time: types::Time::try_new(hour, minute, second, 0)?,
//         })
//     }
// }

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug)]
    struct DateCase {
        year: i32,
        month: u8,
        day: u8,
    }
    
    static TEST_FIXED_DATE: [i64; 33] = [
        -214193, -61387, 25469, 49217, 171307, 210155, 253427, 369740, 400085, 434355, 452605,
        470160, 473837, 507850, 524156, 544676, 567118, 569477, 601716, 613424, 626596, 645554,
        664224, 671401, 694799, 704424, 708842, 709409, 709580, 727274, 728714, 744313, 764652,
    ];

    static ASTRONOMICAL_CASES: [DateCase; 33] = [
        DateCase {
            year: -1245,
            month: 12,
            day: 11,
        },
        DateCase {
            year: -813,
            month: 2,
            day: 25,
        },
        DateCase {
            year: -568,
            month: 4,
            day: 2,
        },
        DateCase {
            year: -501,
            month: 4,
            day: 7,
        },
        DateCase {
            year: -157,
            month: 10,
            day: 18,
        },
        DateCase {
            year: -47,
            month: 6,
            day: 3,
        },
        DateCase {
            year: 75,
            month: 7,
            day: 13,
        },
        DateCase {
            year: 403,
            month: 10,
            day: 5,
        },
        DateCase {
            year: 489,
            month: 5,
            day: 22,
        },
        DateCase {
            year: 586,
            month: 2,
            day: 7,
        },
        DateCase {
            year: 637,
            month: 8,
            day: 7,
        },
        DateCase {
            year: 687,
            month: 2,
            day: 21,
        },
        DateCase {
            year: 697,
            month: 7,
            day: 7,
        },
        DateCase {
            year: 793,
            month: 6,
            day: 30,
        },
        DateCase {
            year: 839,
            month: 7,
            day: 6,
        },
        DateCase {
            year: 897,
            month: 6,
            day: 2,
        },
        DateCase {
            year: 960,
            month: 9,
            day: 30,
        },
        DateCase {
            year: 967,
            month: 5,
            day: 27,
        },
        DateCase {
            year: 1058,
            month: 5,
            day: 18,
        },
        DateCase {
            year: 1091,
            month: 6,
            day: 3,
        },
        DateCase {
            year: 1128,
            month: 8,
            day: 4,
        },
        DateCase {
            year: 1182,
            month: 2,
            day: 4,
        },
        DateCase {
            year: 1234,
            month: 10,
            day: 10,
        },
        DateCase {
            year: 1255,
            month: 1,
            day: 11,
        },
        DateCase {
            year: 1321,
            month: 1,
            day: 20,
        },
        DateCase {
            year: 1348,
            month: 3,
            day: 19,
        },
        DateCase {
            year: 1360,
            month: 9,
            day: 7,
        },
        DateCase {
            year: 1362,
            month: 4,
            day: 14,
        },
        DateCase {
            year: 1362,
            month: 10,
            day: 7,
        },
        DateCase {
            year: 1412,
            month: 9,
            day: 12,
        },
        DateCase {
            year: 1416,
            month: 10,
            day: 5,
        },
        DateCase {
            year: 1460,
            month: 10,
            day: 12,
        },
        DateCase {
            year: 1518,
            month: 3,
            day: 5,
        },
    ];

    #[test]
    fn test_islam_from_fixed() {
        for (case, f_date) in ASTRONOMICAL_CASES.iter().zip(TEST_FIXED_DATE.iter()) {
            let date = Date::try_new_islamic_date(case.year, case.month, case.day).unwrap();
            assert_eq!(
                IslamicObservational::islamic_from_fixed(RataDie::new(*f_date)),
                date,
                "{case:?}"
            );
        }
    }


}