// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use libm::floor;

use crate::any_calendar::AnyCalendarKind;
use crate::astronomy::*;
use crate::calendar_arithmetic::{ArithmeticDate, CalendarArithmetic};
use crate::helpers::{div_rem_euclid, div_rem_euclid_f64, next};
use crate::julian::Julian;
use crate::rata_die::RataDie;
use crate::types::Moment;
use crate::{types, Calendar, CalendarError, Date, DateDuration, DateDurationUnit, DateTime};

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
    zone: 2.0,
};

#[derive(Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
/// The inner date type used for representing [`Date`]s of [`Islamic`]. See [`Date`] and [`Islamic`] for more details.
pub struct IslamicDateInner(ArithmeticDate<IslamicObservational>);

impl CalendarArithmetic for IslamicObservational {
    fn month_days(year: i32, month: u8) -> u8 {
        let fixed_date = Self::fixed_from_islamic(year, month, 1);
        Astronomical::month_length(fixed_date, CAIRO)
    }

    fn months_for_every_year(year: i32) -> u8 {
        12
    }

    fn days_in_provided_year(year: i32) -> u32 {
        355
    }

    fn is_leap_year(year: i32) -> bool {
        (14 + 11 * year) % 30 < 11
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
        todo!()
    }

    fn date_from_iso(&self, iso: Date<crate::Iso>) -> Self::DateInner {
        todo!()
    }

    fn date_to_iso(&self, date: &Self::DateInner) -> Date<crate::Iso> {
        todo!()
    }

    fn months_in_year(&self, date: &Self::DateInner) -> u8 {
        todo!()
    }

    fn days_in_year(&self, date: &Self::DateInner) -> u32 {
        todo!()
    }

    fn days_in_month(&self, date: &Self::DateInner) -> u8 {
        todo!()
    }

    fn offset_date(&self, date: &mut Self::DateInner, offset: DateDuration<Self>) {
        todo!()
    }

    fn until(
        &self,
        date1: &Self::DateInner,
        date2: &Self::DateInner,
        calendar2: &Self,
        largest_unit: DateDurationUnit,
        smallest_unit: DateDurationUnit,
    ) -> DateDuration<Self> {
        todo!()
    }

    fn debug_name(&self) -> &'static str {
        todo!()
    }

    fn year(&self, date: &Self::DateInner) -> types::FormattableYear {
        todo!()
    }

    fn month(&self, date: &Self::DateInner) -> types::FormattableMonth {
        todo!()
    }

    fn day_of_month(&self, date: &Self::DateInner) -> types::DayOfMonth {
        todo!()
    }

    fn day_of_year_info(&self, date: &Self::DateInner) -> types::DayOfYearInfo {
        todo!()
    }
}

impl IslamicObservational {
    fn fixed_from_islamic(year: i32, month: u8, day: u8) -> RataDie {
        let midmonth = FIXED_ISLAMIC_EPOCH_FRIDAY.to_f64_date()
            + (((year - 1) as f64) * 12.0 + month as f64 - 0.5) * MEAN_SYNODIC_MONTH;
        let date = Astronomical::phasis_on_or_before(RataDie::new(midmonth as i64), CAIRO)
            + day.into()
            - 1;

        date
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
/// The inner date type used for representing [`Date`]s of [`Islamic`]. See [`Date`] and [`Islamic`] for more details.
pub struct UmmalQuraDateInner(ArithmeticDate<UmmalQura>);

impl CalendarArithmetic for UmmalQura {
    fn month_days(year: i32, month: u8) -> u8 {
        let fixed_date = Self::fixed_from_saudi_islamic(year, month, 1);
        u8::default()
    }

    fn months_for_every_year(year: i32) -> u8 {
        todo!()
    }

    fn is_leap_year(year: i32) -> bool {
        todo!()
    }

    fn last_month_day_in_year(year: i32) -> (u8, u8) {
        todo!()
    }

    fn days_in_provided_year(year: i32) -> u32 {
        let months_in_year = Self::months_for_every_year(year);
        let mut days: u32 = 0;
        for month in 1..=months_in_year {
            days += Self::month_days(year, month) as u32;
        }
        days
    }
}

impl Calendar for UmmalQura {
    type DateInner = UmmalQuraDateInner;

    fn date_from_codes(
        &self,
        era: types::Era,
        year: i32,
        month_code: types::MonthCode,
        day: u8,
    ) -> Result<Self::DateInner, CalendarError> {
        todo!()
    }

    fn date_from_iso(&self, iso: Date<crate::Iso>) -> Self::DateInner {
        todo!()
    }

    fn date_to_iso(&self, date: &Self::DateInner) -> Date<crate::Iso> {
        todo!()
    }

    fn months_in_year(&self, date: &Self::DateInner) -> u8 {
        todo!()
    }

    fn days_in_year(&self, date: &Self::DateInner) -> u32 {
        todo!()
    }

    fn days_in_month(&self, date: &Self::DateInner) -> u8 {
        todo!()
    }

    fn offset_date(&self, date: &mut Self::DateInner, offset: DateDuration<Self>) {
        todo!()
    }

    fn until(
        &self,
        date1: &Self::DateInner,
        date2: &Self::DateInner,
        calendar2: &Self,
        largest_unit: DateDurationUnit,
        smallest_unit: DateDurationUnit,
    ) -> DateDuration<Self> {
        todo!()
    }

    fn debug_name(&self) -> &'static str {
        todo!()
    }

    fn year(&self, date: &Self::DateInner) -> types::FormattableYear {
        todo!()
    }

    fn month(&self, date: &Self::DateInner) -> types::FormattableMonth {
        todo!()
    }

    fn day_of_month(&self, date: &Self::DateInner) -> types::DayOfMonth {
        todo!()
    }

    fn day_of_year_info(&self, date: &Self::DateInner) -> types::DayOfYearInfo {
        todo!()
    }
}

impl Date<UmmalQura> {}

impl DateTime<UmmalQura> {}

impl UmmalQura {
    fn saudi_criterion(date: RataDie) -> bool {
        let prev_day = date - 1;
        let set = Astronomical::sunset(Moment::new(prev_day.to_f64_date()), MECCA).unwrap();
        let tee = Location::universal_from_standard(set, MECCA);
        let phase = Astronomical::lunar_phase(tee);

        0.0 < phase
            && 90.0 > Astronomical::moonlag(Moment::new(prev_day.to_f64_date()), MECCA).unwrap()
    }

    fn saudi_new_month_on_or_before(date: RataDie) -> RataDie {
        let moon = Astronomical::lunar_phase_at_or_before(0.0, Moment::new(date.to_f64_date()));
        let age = date - moon.as_rata_die();

        let tau = if age <= 3 && !Self::saudi_criterion(date) {
            (moon - 30.0).as_rata_die()
        } else {
            moon.as_rata_die()
        };
        let d = tau;

        next(d, tau, Self::saudi_criterion)
    }

    fn fixed_from_saudi_islamic(year: i32, month: u8, day: u8) -> RataDie {
        let midmonth = FIXED_ISLAMIC_EPOCH_FRIDAY
            + floor(((year - 1) as f64 * 12.0 + (month as f64) - 0.5) * MEAN_SYNODIC_MONTH) as i64;
        let date = Self::saudi_new_month_on_or_before(midmonth) + day.into() - 1;

        date
    }
    // TODO: Stopped working with Umm-Al-Qura to implement the Observational which will be used as the default Date calendar type
    //fn saudi_islamic_from_fixed(date: RataDie) -> Date<Islamic> {}
}
