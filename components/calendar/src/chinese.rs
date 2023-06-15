// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::any_calendar::AnyCalendarKind;
use crate::astronomy::{Astronomical, Location};
use crate::calendar_arithmetic::{ArithmeticDate, CalendarArithmetic};
use crate::helpers::adjusted_rem_euclid;
use crate::iso::{Iso, IsoDateInner};
use crate::rata_die::RataDie;
use crate::{
    astronomy, types, Calendar, CalendarError, Date, DateDuration, DateDurationUnit, DateTime,
};
use tinystr::tinystr;

// The equivalent first day in the Chinese calendar (based on inception of the calendar)
const CHINESE_EPOCH: RataDie = RataDie::new(-963099); // Feb. 15, 2637 BCE (-2636)

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Chinese;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct ChineseDateInner {
    inner: ArithmeticDate<Chinese>, // Should this be an ArithmeticDate??
    leap_month: u8,
}

impl CalendarArithmetic for Chinese {
    fn month_days(year: i32, month: u8) -> u8 {
        todo!()
    }

    fn months_for_every_year(year: i32) -> u8 {
        todo!()
    }

    fn is_leap_year(year: i32) -> bool {
        todo!()
    }

    fn last_month_day_in_year(year: i32) -> (u8, u8) {
        todo!()
    } // Should this be implemented??
}

impl Calendar for Chinese {
    type DateInner = ChineseDateInner;

    // Construct a date from era/month codes and fields
    fn date_from_codes(
        &self,
        era: types::Era,
        year: i32,
        month_code: types::MonthCode,
        day: u8,
    ) -> Result<Self::DateInner, CalendarError> {
        // Figure out whether this should be implemented through ArithmeticDate or not
        todo!();
    }

    // Construct the date from an ISO date
    fn date_from_iso(&self, iso: Date<Iso>) -> Self::DateInner {
        todo!();
    }

    // Obtain an ISO date from a Chinese date
    fn date_to_iso(&self, date: &Self::DateInner) -> Date<Iso> {
        todo!();
    }

    //Count the number of months in a given year, specified by providing a date
    // from that year
    fn days_in_year(&self, date: &Self::DateInner) -> u32 {
        todo!();
    }

    fn days_in_month(&self, date: &Self::DateInner) -> u8 {
        todo!();
    }

    #[doc(hidden)] // unstable
    fn offset_date(&self, date: &mut Self::DateInner, offset: DateDuration<Self>) {
        todo!();
    }

    #[doc(hidden)] // unstable
    /// Calculate `date2 - date` as a duration
    ///
    /// `calendar2` is the calendar object associated with `date2`. In case the specific calendar objects
    /// differ on date, the date for the first calendar is used, and `date2` may be converted if necessary.
    fn until(
        &self,
        date1: &Self::DateInner,
        date2: &Self::DateInner,
        calendar2: &Self,
        largest_unit: DateDurationUnit,
        smallest_unit: DateDurationUnit,
    ) -> DateDuration<Self> {
        todo!();
    }

    /// Obtain a name for the calendar for debug printing
    fn debug_name(&self) -> &'static str {
        todo!();
    }

    /// The calendar-specific year represented by `date`
    fn year(&self, date: &Self::DateInner) -> types::FormattableYear {
        todo!();
    }

    /// The calendar-specific month represented by `date`
    fn month(&self, date: &Self::DateInner) -> types::FormattableMonth {
        todo!();
    }

    /// The calendar-specific day-of-month represented by `date`
    fn day_of_month(&self, date: &Self::DateInner) -> types::DayOfMonth {
        todo!();
    }

    /// Information of the day of the year
    fn day_of_year_info(&self, date: &Self::DateInner) -> types::DayOfYearInfo {
        todo!();
    }

    /// The [`AnyCalendarKind`] corresponding to this calendar
    fn any_calendar_kind(&self) -> Option<AnyCalendarKind> {
        todo!();
    }

    fn months_in_year(&self, date: &Self::DateInner) -> u8 {
        todo!()
    }
}

impl Date<Chinese> {
    /// Construct a new Chinese date;
    /// year represents the year counted infinitely from -2636 (2637 BCE);
    /// leap_month indicates whether the month of the given date is a leap month
    pub fn try_new_chinese_date(
        year: i32,
        month: u8,
        leap_month: bool,
        day: u8,
    ) -> Result<Date<Chinese>, CalendarError> {
        todo!();
    }
}

impl DateTime<Chinese> {
    /// Construct a new Chinese datetime from integers using the
    /// -2636-based year system
    pub fn try_new_chinese_datetime(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Result<DateTime<Chinese>, CalendarError> {
        todo!();
    }
}

impl Chinese {
    // pub(crate) fn chinese_from_fixed(fixed: RataDie) -> Date<Chinese> {
    //     let s1 = Self::chinese_winter_solstice_on_or_before(fixed);
    //     let s2 = Self::chinese_winter_solstice_on_or_before(s1 + 370);
    //     let m12 = Self::chinese_new_moon_on_or_after(s1 + 1);
    //     let next_m11 = Self::chinese_new_moon_before(s2 + 1);
    //     let m = Self::chinese_new_moon_before(fixed + 1);
    //     let leap_year = ((next_m11 - m12) / astronomy::MEAN_SYNODIC_MONTH).round() == 12;
    //     let month = adjusted_rem_euclid(((m - m12) / astronomy::MEAN_SYNODIC_MONTH).round() - (if leap_year && Self::chinese_prior_leap_month(m12, m) { 1 } else { 0 }), 12);
    //     let leap_month = leap_year && Self::chinese_no_major_solar_term(m) && !Self::chinese_prior_leap_month(m12, Self::chinese_new_moon_before(m));
    //     let year = (1.5 - (month / 12) + ((fixed - CHINESE_EPOCH) / astronomy::MEAN_TROPICAL_YEAR)).floor();
    //     let day = fixed - m + 1;
    //     Date::try_new_chinese_date(year, month, leap_month, day).unwrap()
    // }

    // pub(crate) fn fixed_from_chinese(chinese: Date<Chinese>) -> RataDie {
    //     let mid_year = CHINESE_EPOCH + astronomy::MEAN_TROPICAL_YEAR.quotient(2);
    //     let new_year = Self::chinese_new_year_on_or_before(mid_year);
    //     let p = Self::chinese_new_moon_on_or_after(new_year + (month - 1) * 29);
    //     let d = Self::chinese_from_fixed(p);

    // }
}
