// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains types and implementations for the Hebrew calendar.
//!
//! ```rust
//! use icu::calendar::{hebrew::Hebrew, Date, DateTime, Ref};
//!
//! let hebrew = Hebrew::new_always_calculating();
//! let hebrew = Ref(&hebrew); // to avoid cloning
//!
//! // `Date` type
//! let hebrew_date =
//!     Date::try_new_hebrew_date_with_calendar(3425, 10, 11, hebrew)
//!         .expect("Failed to initialize hebrew Date instance.");
//!
//! // `DateTime` type
//! let hebrew_datetime = DateTime::try_new_hebrew_datetime_with_calendar(
//!     3425, 10, 11, 13, 1, 0, hebrew,
//! )
//! .expect("Failed to initialize hebrew DateTime instance.");
//!
//! // `Date` checks
//! assert_eq!(hebrew_date.year().number, 3425);
//! assert_eq!(hebrew_date.month().ordinal, 10);
//! assert_eq!(hebrew_date.day_of_month().0, 11);
//!
//! // `DateTime` checks
//! assert_eq!(hebrew_datetime.date.year().number, 3425);
//! assert_eq!(hebrew_datetime.date.month().ordinal, 10);
//! assert_eq!(hebrew_datetime.date.day_of_month().0, 11);
//! assert_eq!(hebrew_datetime.time.hour.number(), 13);
//! assert_eq!(hebrew_datetime.time.minute.number(), 1);
//! assert_eq!(hebrew_datetime.time.second.number(), 0);
//! ```

use crate::calendar_arithmetic::PrecomputedDataSource;
use crate::calendar_arithmetic::{ArithmeticDate, CalendarArithmetic};
use crate::types::FormattableMonth;
use crate::AnyCalendarKind;
use crate::AsCalendar;
use crate::Iso;
use crate::{types, Calendar, CalendarError, Date, DateDuration, DateDurationUnit, DateTime};
use ::tinystr::tinystr;
use calendrical_calculations::hebrew_keviyah::{Keviyah, YearInfo};

/// The Civil Hebrew Calendar
///
/// The [Hebrew calendar] is a lunisolar calendar used as the Jewish liturgical calendar
/// as well as an official calendar in Israel.
///
/// This calendar is the _civil_ Hebrew calendar, with the year starting at in the month of Tishrei.
///
/// # Era codes
///
/// This calendar supports a single era code, Anno Mundi, with code `"am"`
///
/// # Month codes
///
/// This calendar is a lunisolar calendar and thus has a leap month. It supports codes `"M01"-"M12"`
/// for regular months, and the leap month Adar I being coded as `"M05L"`.
///
/// [`FormattableMonth`] has slightly divergent behavior: because the regular month Adar is formatted
/// as "Adar II" in a leap year, this calendar will produce the special code `"M06L"` in any [`FormattableMonth`]
/// objects it creates.
///
/// [Hebrew calendar]: https://en.wikipedia.org/wiki/Hebrew_calendar
#[derive(Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
#[non_exhaustive] // we'll be adding precompiled data to this
pub struct Hebrew;

/// The inner date type used for representing [`Date`]s of [`BookHebrew`]. See [`Date`] and [`BookHebrew`] for more details.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct BookHebrewDateInner;
/// The inner date type used for representing [`Date`]s of [`Hebrew`]. See [`Date`] and [`Hebrew`] for more details.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct HebrewDateInner(ArithmeticDate<Hebrew>);

impl Hebrew {
    /// Construct a new [`Hebrew`] without any precomputed calendrical calculations.
    ///
    /// This is the only mode currently possible, but once precomputing is available (#3933)
    /// there will be additional constructors that load from data providers.
    pub fn new_always_calculating() -> Self {
        Hebrew
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub(crate) struct HebrewYearInfo {
    keviyah: Keviyah,
    prev_keviyah: Keviyah,
}

impl HebrewYearInfo {
    /// Convenience method to compute for a given year. Don't use this if you actually need
    /// a YearInfo that you want to call .new_year() on.
    ///
    /// This can potentially be optimized with adjacent-year knowledge, but it's complex
    #[inline]
    fn compute(h_year: i32) -> Self {
        let keviyah = YearInfo::compute_for(h_year).keviyah;
        Self::compute_with_keviyah(keviyah, h_year)
    }
    /// Compute for a given year when the keviyah is already known
    #[inline]
    fn compute_with_keviyah(keviyah: Keviyah, h_year: i32) -> Self {
        let prev_keviyah = YearInfo::compute_for(h_year - 1).keviyah;
        Self {
            keviyah,
            prev_keviyah,
        }
    }
}
//  HEBREW CALENDAR

impl CalendarArithmetic for Hebrew {
    type YearInfo = HebrewYearInfo;

    fn month_days(_h_year: i32, ordinal_month: u8, info: HebrewYearInfo) -> u8 {
        info.keviyah.month_len(ordinal_month)
    }

    fn months_for_every_year(_h_year: i32, info: HebrewYearInfo) -> u8 {
        if info.keviyah.is_leap() {
            13
        } else {
            12
        }
    }

    fn days_in_provided_year(_h_year: i32, info: HebrewYearInfo) -> u16 {
        info.keviyah.year_length()
    }

    fn is_leap_year(_h_year: i32, info: HebrewYearInfo) -> bool {
        info.keviyah.is_leap()
    }

    fn last_month_day_in_year(h_year: i32, info: HebrewYearInfo) -> (u8, u8) {
        // Calculate the index of the last month (Elul)
        let last_month = Self::months_for_every_year(h_year, info);
        // Elul always has 29 days
        (last_month, 29)
    }
}

impl PrecomputedDataSource<HebrewYearInfo> for () {
    fn load_or_compute_info(&self, h_year: i32) -> HebrewYearInfo {
        HebrewYearInfo::compute(h_year)
    }
}

impl Calendar for Hebrew {
    type DateInner = HebrewDateInner;

    fn date_from_codes(
        &self,
        era: types::Era,
        year: i32,
        month_code: types::MonthCode,
        day: u8,
    ) -> Result<Self::DateInner, CalendarError> {
        let year = if era.0 == tinystr!(16, "hebrew") || era.0 == tinystr!(16, "am") {
            year
        } else {
            return Err(CalendarError::UnknownEra(era.0, self.debug_name()));
        };

        let year_info = HebrewYearInfo::compute(year);

        let is_leap_year = year_info.keviyah.is_leap();

        let month_code_str = month_code.0.as_str();

        let month_ordinal = if is_leap_year {
            match month_code_str {
                "M01" => 1,
                "M02" => 2,
                "M03" => 3,
                "M04" => 4,
                "M05" => 5,
                "M05L" => 6,
                // M06L is the formatting era code used for Adar II
                "M06" | "M06L" => 7,
                "M07" => 8,
                "M08" => 9,
                "M09" => 10,
                "M10" => 11,
                "M11" => 12,
                "M12" => 13,
                _ => {
                    return Err(CalendarError::UnknownMonthCode(
                        month_code.0,
                        self.debug_name(),
                    ))
                }
            }
        } else {
            match month_code_str {
                "M01" => 1,
                "M02" => 2,
                "M03" => 3,
                "M04" => 4,
                "M05" => 5,
                "M06" => 6,
                "M07" => 7,
                "M08" => 8,
                "M09" => 9,
                "M10" => 10,
                "M11" => 11,
                "M12" => 12,
                _ => {
                    return Err(CalendarError::UnknownMonthCode(
                        month_code.0,
                        self.debug_name(),
                    ))
                }
            }
        };

        ArithmeticDate::new_from_ordinals_with_info(year, month_ordinal, day, year_info)
            .map(HebrewDateInner)
    }

    fn date_from_iso(&self, iso: Date<Iso>) -> Self::DateInner {
        let fixed_iso = Iso::fixed_from_iso(*iso.inner());
        let (year_info, h_year) = YearInfo::year_containing_rd(fixed_iso);
        // Obtaining a 1-indexed day-in-year value
        let mut day = fixed_iso - year_info.new_year() + 1;

        let year_info = HebrewYearInfo::compute_with_keviyah(year_info.keviyah, h_year);
        for month in 1..14 {
            let month_len = year_info.keviyah.month_len(month);
            if let Ok(day) = u8::try_from(day) {
                if day <= month_len {
                    return HebrewDateInner(ArithmeticDate::new_unchecked_with_info(
                        h_year, month, day, year_info,
                    ));
                }
            }
            day -= i64::from(month_len);
        }
        debug_assert!(false, "Attempted to get Hebrew date for {fixed_iso:?}, in year {h_year}, didn't have enough days in the year");
        return HebrewDateInner(ArithmeticDate::new_unchecked_with_info(
            h_year, 13, 29, year_info,
        ));
    }

    fn date_to_iso(&self, date: &Self::DateInner) -> Date<Iso> {
        let year_info = date.0.year_info.keviyah.year_info(date.0.year);

        let ny = year_info.new_year();
        let days_preceding = year_info.keviyah.days_preceding(date.0.month);

        // Need to subtract 1 since the new year is itself in this year
        Iso::iso_from_fixed(ny + i64::from(days_preceding) + i64::from(date.0.day) - 1)
    }

    fn months_in_year(&self, date: &Self::DateInner) -> u8 {
        date.0.months_in_year()
    }

    fn days_in_year(&self, date: &Self::DateInner) -> u16 {
        date.0.days_in_year()
    }

    fn days_in_month(&self, date: &Self::DateInner) -> u8 {
        date.0.days_in_month()
    }

    fn offset_date(&self, date: &mut Self::DateInner, offset: DateDuration<Self>) {
        date.0.offset_date(offset, &())
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
        "Hebrew"
    }

    fn year(&self, date: &Self::DateInner) -> types::FormattableYear {
        Self::year_as_hebrew(date.0.year)
    }

    fn is_in_leap_year(&self, date: &Self::DateInner) -> bool {
        Self::is_leap_year(date.0.year, date.0.year_info)
    }

    fn month(&self, date: &Self::DateInner) -> FormattableMonth {
        let mut ordinal = date.0.month;
        let is_leap_year = Self::is_leap_year(date.0.year, date.0.year_info);

        if is_leap_year {
            if ordinal == 6 {
                return types::FormattableMonth {
                    ordinal: ordinal as u32,
                    code: types::MonthCode(tinystr!(4, "M05L")),
                };
            } else if ordinal == 7 {
                return types::FormattableMonth {
                    ordinal: ordinal as u32,
                    code: types::MonthCode(tinystr!(4, "M06L")),
                };
            }
        }

        if is_leap_year && ordinal > 6 {
            ordinal -= 1;
        }

        let code = match ordinal {
            1 => tinystr!(4, "M01"),
            2 => tinystr!(4, "M02"),
            3 => tinystr!(4, "M03"),
            4 => tinystr!(4, "M04"),
            5 => tinystr!(4, "M05"),
            6 => tinystr!(4, "M06"),
            7 => tinystr!(4, "M07"),
            8 => tinystr!(4, "M08"),
            9 => tinystr!(4, "M09"),
            10 => tinystr!(4, "M10"),
            11 => tinystr!(4, "M11"),
            12 => tinystr!(4, "M12"),
            _ => tinystr!(4, "und"),
        };

        types::FormattableMonth {
            ordinal: date.0.month as u32,
            code: types::MonthCode(code),
        }
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
            prev_year: Self::year_as_hebrew(prev_year),
            days_in_prev_year: date.0.year_info.prev_keviyah.year_length(),
            next_year: Self::year_as_hebrew(next_year),
        }
    }
    fn any_calendar_kind(&self) -> Option<AnyCalendarKind> {
        Some(AnyCalendarKind::Hebrew)
    }
}

impl Hebrew {
    fn year_as_hebrew(civil_year: i32) -> types::FormattableYear {
        types::FormattableYear {
            era: types::Era(tinystr!(16, "hebrew")),
            number: civil_year,
            cyclic: None,
            related_iso: None,
        }
    }
}

impl<A: AsCalendar<Calendar = Hebrew>> Date<A> {
    /// Construct new Hebrew Date.
    ///
    /// This datetime will not use any precomputed calendrical calculations,
    /// one that loads such data from a provider will be added in the future (#3933)
    ///
    ///
    /// ```rust
    /// use icu::calendar::hebrew::Hebrew;
    /// use icu::calendar::Date;
    ///
    /// let hebrew = Hebrew::new_always_calculating();
    ///
    /// let date_hebrew =
    ///     Date::try_new_hebrew_date_with_calendar(3425, 4, 25, hebrew)
    ///         .expect("Failed to initialize Hebrew Date instance.");
    ///
    /// assert_eq!(date_hebrew.year().number, 3425);
    /// assert_eq!(date_hebrew.month().ordinal, 4);
    /// assert_eq!(date_hebrew.day_of_month().0, 25);
    /// ```
    pub fn try_new_hebrew_date_with_calendar(
        year: i32,
        month: u8,
        day: u8,
        calendar: A,
    ) -> Result<Date<A>, CalendarError> {
        let year_info = HebrewYearInfo::compute(year);

        ArithmeticDate::new_from_ordinals_with_info(year, month, day, year_info)
            .map(HebrewDateInner)
            .map(|inner| Date::from_raw(inner, calendar))
    }
}

impl<A: AsCalendar<Calendar = Hebrew>> DateTime<A> {
    /// Construct a new Hebrew datetime from integers.
    ///
    /// This datetime will not use any precomputed calendrical calculations,
    /// one that loads such data from a provider will be added in the future (#3933)
    ///
    /// ```rust
    /// use icu::calendar::hebrew::Hebrew;
    /// use icu::calendar::DateTime;
    ///
    /// let hebrew = Hebrew::new_always_calculating();
    ///
    /// let datetime_hebrew = DateTime::try_new_hebrew_datetime_with_calendar(
    ///     4201, 10, 11, 13, 1, 0, hebrew,
    /// )
    /// .expect("Failed to initialize Hebrew DateTime instance");
    ///
    /// assert_eq!(datetime_hebrew.date.year().number, 4201);
    /// assert_eq!(datetime_hebrew.date.month().ordinal, 10);
    /// assert_eq!(datetime_hebrew.date.day_of_month().0, 11);
    /// assert_eq!(datetime_hebrew.time.hour.number(), 13);
    /// assert_eq!(datetime_hebrew.time.minute.number(), 1);
    /// assert_eq!(datetime_hebrew.time.second.number(), 0);
    /// ```
    pub fn try_new_hebrew_datetime_with_calendar(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        calendar: A,
    ) -> Result<DateTime<A>, CalendarError> {
        Ok(DateTime {
            date: Date::try_new_hebrew_date_with_calendar(year, month, day, calendar)?,
            time: types::Time::try_new(hour, minute, second, 0)?,
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use calendrical_calculations::hebrew::*;

    #[test]
    fn test_conversions() {
        let iso_dates: [Date<Iso>; 48] = [
            Date::try_new_iso_date(2021, 1, 10).unwrap(),
            Date::try_new_iso_date(2021, 1, 25).unwrap(),
            Date::try_new_iso_date(2021, 2, 10).unwrap(),
            Date::try_new_iso_date(2021, 2, 25).unwrap(),
            Date::try_new_iso_date(2021, 3, 10).unwrap(),
            Date::try_new_iso_date(2021, 3, 25).unwrap(),
            Date::try_new_iso_date(2021, 4, 10).unwrap(),
            Date::try_new_iso_date(2021, 4, 25).unwrap(),
            Date::try_new_iso_date(2021, 5, 10).unwrap(),
            Date::try_new_iso_date(2021, 5, 25).unwrap(),
            Date::try_new_iso_date(2021, 6, 10).unwrap(),
            Date::try_new_iso_date(2021, 6, 25).unwrap(),
            Date::try_new_iso_date(2021, 7, 10).unwrap(),
            Date::try_new_iso_date(2021, 7, 25).unwrap(),
            Date::try_new_iso_date(2021, 8, 10).unwrap(),
            Date::try_new_iso_date(2021, 8, 25).unwrap(),
            Date::try_new_iso_date(2021, 9, 10).unwrap(),
            Date::try_new_iso_date(2021, 9, 25).unwrap(),
            Date::try_new_iso_date(2021, 10, 10).unwrap(),
            Date::try_new_iso_date(2021, 10, 25).unwrap(),
            Date::try_new_iso_date(2021, 11, 10).unwrap(),
            Date::try_new_iso_date(2021, 11, 25).unwrap(),
            Date::try_new_iso_date(2021, 12, 10).unwrap(),
            Date::try_new_iso_date(2021, 12, 25).unwrap(),
            Date::try_new_iso_date(2022, 1, 10).unwrap(),
            Date::try_new_iso_date(2022, 1, 25).unwrap(),
            Date::try_new_iso_date(2022, 2, 10).unwrap(),
            Date::try_new_iso_date(2022, 2, 25).unwrap(),
            Date::try_new_iso_date(2022, 3, 10).unwrap(),
            Date::try_new_iso_date(2022, 3, 25).unwrap(),
            Date::try_new_iso_date(2022, 4, 10).unwrap(),
            Date::try_new_iso_date(2022, 4, 25).unwrap(),
            Date::try_new_iso_date(2022, 5, 10).unwrap(),
            Date::try_new_iso_date(2022, 5, 25).unwrap(),
            Date::try_new_iso_date(2022, 6, 10).unwrap(),
            Date::try_new_iso_date(2022, 6, 25).unwrap(),
            Date::try_new_iso_date(2022, 7, 10).unwrap(),
            Date::try_new_iso_date(2022, 7, 25).unwrap(),
            Date::try_new_iso_date(2022, 8, 10).unwrap(),
            Date::try_new_iso_date(2022, 8, 25).unwrap(),
            Date::try_new_iso_date(2022, 9, 10).unwrap(),
            Date::try_new_iso_date(2022, 9, 25).unwrap(),
            Date::try_new_iso_date(2022, 10, 10).unwrap(),
            Date::try_new_iso_date(2022, 10, 25).unwrap(),
            Date::try_new_iso_date(2022, 11, 10).unwrap(),
            Date::try_new_iso_date(2022, 11, 25).unwrap(),
            Date::try_new_iso_date(2022, 12, 10).unwrap(),
            Date::try_new_iso_date(2022, 12, 25).unwrap(),
        ];

        let book_hebrew_dates: [(u8, u8, i32); 48] = [
            (26, TEVET, 5781),
            (12, SHEVAT, 5781),
            (28, SHEVAT, 5781),
            (13, ADAR, 5781),
            (26, ADAR, 5781),
            (12, NISAN, 5781),
            (28, NISAN, 5781),
            (13, IYYAR, 5781),
            (28, IYYAR, 5781),
            (14, SIVAN, 5781),
            (30, SIVAN, 5781),
            (15, TAMMUZ, 5781),
            (1, AV, 5781),
            (16, AV, 5781),
            (2, ELUL, 5781),
            (17, ELUL, 5781),
            (4, TISHRI, 5782),
            (19, TISHRI, 5782),
            (4, MARHESHVAN, 5782),
            (19, MARHESHVAN, 5782),
            (6, KISLEV, 5782),
            (21, KISLEV, 5782),
            (6, TEVET, 5782),
            (21, TEVET, 5782),
            (8, SHEVAT, 5782),
            (23, SHEVAT, 5782),
            (9, ADAR, 5782),
            (24, ADAR, 5782),
            (7, ADARII, 5782),
            (22, ADARII, 5782),
            (9, NISAN, 5782),
            (24, NISAN, 5782),
            (9, IYYAR, 5782),
            (24, IYYAR, 5782),
            (11, SIVAN, 5782),
            (26, SIVAN, 5782),
            (11, TAMMUZ, 5782),
            (26, TAMMUZ, 5782),
            (13, AV, 5782),
            (28, AV, 5782),
            (14, ELUL, 5782),
            (29, ELUL, 5782),
            (15, TISHRI, 5783),
            (30, TISHRI, 5783),
            (16, MARHESHVAN, 5783),
            (1, KISLEV, 5783),
            (16, KISLEV, 5783),
            (1, TEVET, 5783),
        ];

        for (iso_date, book_date_nums) in iso_dates.iter().zip(book_hebrew_dates.iter()) {
            // This just checks the integrity of the test data
            let book_date = BookHebrew {
                year: book_date_nums.2,
                month: book_date_nums.1,
                day: book_date_nums.0,
            };

            let (y, m, d) = book_date.to_civil_date();
            // let book_from_civil = BookHebrew::from_civil_date(civil_date_nums.2, civil_date_nums.1, civil_date_nums.1);
            // assert_eq!(book_date, book_from_civil);

            let hy = HebrewYearInfo::compute(y);
            let hebrew_date: HebrewDateInner =
                HebrewDateInner(ArithmeticDate::new_unchecked_with_info(y, m, d, hy));
            let hebrew_date = Date::from_raw(hebrew_date, Hebrew);

            let iso_to_hebrew = iso_date.to_calendar(Hebrew);

            let hebrew_to_iso = hebrew_date.to_calendar(Iso);

            assert_eq!(
                hebrew_to_iso, *iso_date,
                "Failed comparing to-ISO value for {hebrew_date:?} => {iso_date:?}"
            );
            assert_eq!(
                iso_to_hebrew, hebrew_date,
                "Failed comparing to-hebrew value for {iso_date:?} => {hebrew_date:?}"
            );
        }
    }

    #[test]
    fn test_icu_bug_22441() {
        assert_eq!(BookHebrew::days_in_book_hebrew_year(88369), 383);
    }
}
