// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::any_calendar::AnyCalendarKind;
use crate::astronomy::{Astronomical, Location};
use crate::calendar_arithmetic::{ArithmeticDate, CalendarArithmetic};
use crate::helpers::{adjusted_rem_euclid, i64_to_i32, quotient, I32Result};
use crate::iso::{self, Iso, IsoDateInner};
use crate::rata_die::RataDie;
use crate::types::Moment;
use crate::{
    astronomy, types, Calendar, CalendarError, Date, DateDuration, DateDurationUnit, DateTime,
};
use core::marker::PhantomData;
use tinystr::tinystr;

// The equivalent first day in the Chinese calendar (based on inception of the calendar)
const CHINESE_EPOCH: RataDie = RataDie::new(-963099); // Feb. 15, 2637 BCE (-2636)

/// The Chinese calendar relies on knowing the current day at the moment of a new moon;
/// however, this can vary depending on location. As such, new moon calculations are based
/// on the time in Beijing. Before 1929, local time was used, represented as UTC+(1397/180 h).
/// In 1929, China adopted a standard time zone based on 120 degrees of longitude, meaning
/// from 1929 onward, all new moon calculations are based on UTC+8h.
///
/// Offsets are not given in hours, but in partial days (1 hour = 1 / 24 day)
const UTC_OFFSET_PRE_1929: f64 = (1397.0 / 180.0) / 24.0;
const UTC_OFFSET_POST_1929: f64 = 8.0 / 24.0;

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
    /// Get the current major solar term of an ISO date
    pub fn major_solar_term_from_iso(iso: IsoDateInner) -> i32 {
        let fixed: RataDie = Iso::fixed_from_iso(iso);
        Self::major_solar_term_from_fixed(fixed)
    }

    /// Get the current major solar term of a fixed date, output as an integer from 1..=12.
    pub(crate) fn major_solar_term_from_fixed(date: RataDie) -> i32 {
        let moment: Moment = date.as_moment();
        let offset = Self::chinese_offset(date);
        let universal: Moment = Location::universal_from_standard(moment, offset);
        let solar_longitude = i64_to_i32(Astronomical::solar_longitude(universal) as i64);
        debug_assert!(
            matches!(solar_longitude, I32Result::WithinRange(_)),
            "Solar longitude should be in range of i32"
        );
        let s = solar_longitude.saturate();
        adjusted_rem_euclid(2 + quotient(s, 30), 12)
    }

    /// Get the current major solar term of an ISO date
    pub fn minor_solar_term_from_iso(iso: IsoDateInner) -> i32 {
        let fixed: RataDie = Iso::fixed_from_iso(iso);
        Self::minor_solar_term_from_fixed(fixed)
    }

    /// Get the current minor solar term of a fixed date, output as an integer from 1..=12.
    pub(crate) fn minor_solar_term_from_fixed(date: RataDie) -> i32 {
        let moment: Moment = date.as_moment();
        let offset = Self::chinese_offset(date);
        let universal: Moment = Location::universal_from_standard(moment, offset);
        let solar_longitude = i64_to_i32(Astronomical::solar_longitude(universal) as i64);
        debug_assert!(
            matches!(solar_longitude, I32Result::WithinRange(_)),
            "Solar longitude should be in range of i32"
        );
        let s = solar_longitude.saturate();
        adjusted_rem_euclid(3 + quotient(s - 15, 30), 12)
    }

    // Returns UTC_OFFSET_PRE_1929 if the year is before 1929,
    // returns UTC_OFFSET_POST_1929 if the year is 1929 or after.
    fn chinese_offset(date: RataDie) -> f64 {
        let year = Iso::iso_from_fixed(date).year().number;
        if year < 1929 {
            UTC_OFFSET_PRE_1929
        } else {
            UTC_OFFSET_POST_1929
        }
    }

    // The fixed date in Chinese standard time of the next new moon
    // on or after the given moment.
    fn chinese_new_moon_on_or_after(moment: Moment) -> RataDie {
        let new_moon_moment = Astronomical::new_moon_at_or_after(Self::midnight_in_china(moment));
        let chinese_offset = Self::chinese_offset(new_moon_moment.as_rata_die());
        Location::standard_from_universal(new_moon_moment, chinese_offset).as_rata_die()
    }

    // The fixed date in Chinese standard time of the previous new moon
    // before the given moment.
    fn chinese_new_moon_before(moment: Moment) -> RataDie {
        let new_moon_moment = Astronomical::new_moon_before(Self::midnight_in_china(moment));
        let chinese_offset = Self::chinese_offset(new_moon_moment.as_rata_die());
        Location::standard_from_universal(new_moon_moment, chinese_offset).as_rata_die()
    }

    // Universal time of midnight at start of a Moment in China
    fn midnight_in_china(date: Moment) -> Moment {
        Location::universal_from_standard(date, Self::chinese_offset(date.as_rata_die()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_chinese_new_moon_directionality() {
        for i in (-255..1000).step_by(31) {
            let moment = Moment::new(i as f64);
            let before = Chinese::chinese_new_moon_before(moment);
            let after = Chinese::chinese_new_moon_on_or_after(moment);
            assert!(before < after, "Chinese new moon directionality failed for Moment: {moment:?}, with:\n\tBefore: {before:?}\n\tAfter: {after:?}");
        }
    }
}
