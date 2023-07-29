// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains types and implementations for the Biblical Hebrew calendar.
//!
//! ```rust
//! use icu::calendar::{Date, DateTime};
//!
//! // `Date` type
//! let hebrew_date = Date::try_new_hebrew_date(3425, 10, 11)
//!     .expect("Failed to initialize hebrew Date instance.");
//!
//! // `DateTime` type
//! let hebrew_datetime = DateTime::try_new_hebrew_datetime(3425, 10, 11, 13, 1, 0)
//!     .expect("Failed to initialize hebrew DateTime instance.");
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

use crate::calendar_arithmetic::{ArithmeticDate, CalendarArithmetic};
use crate::helpers::{self, div_rem_euclid, div_rem_euclid_f64, final_func, next_u8};
use crate::julian::Julian;
use crate::rata_die::RataDie;
use crate::types::Moment;
use crate::Iso;
use crate::{types, Calendar, CalendarError, Date, DateDuration, DateDurationUnit, DateTime};
use ::tinystr::tinystr;

/// Biblical Hebrew Calendar
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord)]
#[allow(clippy::exhaustive_structs)]
pub struct BookHebrew;
/// Civil Hebrew Calendar
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord)]
#[allow(clippy::exhaustive_structs)]
pub struct CivilHebrew;

// Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L2206
const FIXED_HEBREW_EPOCH: RataDie = Julian::fixed_from_julian_integers(-3761, 10, 8);

// The BookHebrew Months
const NISAN: u8 = 1;
const IYYAR: u8 = 2;
#[allow(dead_code)]
const SIVAN: u8 = 3;
const TAMMUZ: u8 = 4;
#[allow(dead_code)]
const AV: u8 = 5;
const ELUL: u8 = 6;
const TISHRI: u8 = 7;
const MARHESHVAN: u8 = 8;
const KISLEV: u8 = 9;
const TEVET: u8 = 10;
#[allow(dead_code)]
const SHEVAT: u8 = 11;
const ADAR: u8 = 12;
const ADARII: u8 = 13;

/// The inner date type used for representing [`Date`]s of [`BookHebrew`]. See [`Date`] and [`BookHebrew`] for more details.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct BookHebrewDateInner;
/// The inner date type used for representing [`Date`]s of [`CivilHebrew`]. See [`Date`] and [`CivilHebrew`] for more details.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct CivilHebrewDateInner(ArithmeticDate<CivilHebrew>);

// CIVIL HEBREW CALENDAR

impl CalendarArithmetic for CivilHebrew {
    fn month_days(year: i32, month: u8) -> u8 {
        todo!()
    }

    fn months_for_every_year(year: i32) -> u8 {
        todo!()
    }

    fn days_in_provided_year(year: i32) -> u16 {
        todo!()
    }

    fn is_leap_year(year: i32) -> bool {
        todo!()
    }

    fn last_month_day_in_year(year: i32) -> (u8, u8) {
        todo!()
    }
}

impl Calendar for CivilHebrew {
    type DateInner = CivilHebrewDateInner;

    fn date_from_codes(
        &self,
        era: types::Era,
        year: i32,
        month_code: types::MonthCode,
        day: u8,
    ) -> Result<Self::DateInner, CalendarError> {
        let year = if era.0 == tinystr!(16, "hebrew") {
            year
        } else {
            return Err(CalendarError::UnknownEra(era.0, self.debug_name()));
        };

        ArithmeticDate::new_from_codes(self, year, month_code, day).map(BookHebrewDateInner)
    }

    fn date_from_iso(&self, iso: Date<Iso>) -> Self::DateInner {
        let fixed_iso = Iso::fixed_from_iso(*iso.inner());
        Self::civil_hebrew_from_fixed(fixed_iso).inner
    }

    fn date_to_iso(&self, date: &Self::DateInner) -> Date<Iso> {
        let fixed_hebrew = Self::fixed_from_civil_hebrew(*date);
        Iso::iso_from_fixed(fixed_hebrew)
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
        "CivilHebrew"
    }

    fn year(&self, date: &Self::DateInner) -> types::FormattableYear {
        Self::year_as_hebrew(date.0.year)
    }

    fn month(&self, date: &Self::DateInner) -> types::FormattableMonth {
        date.0.month()
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
            days_in_prev_year: Self::days_in_provided_year(prev_year),
            next_year: Self::year_as_hebrew(next_year),
        }
    }
}

impl CivilHebrew {
    /// Constructs a new Civil Hebrew Calendar
    pub fn new() -> Self {
        Self
    }
    
    // Converts a Biblical Hebrew Date (which considers Tishri the start of the year) to a Civil Hebrew Year (year starts at Nisan).
    fn biblical_to_civil_date(biblical_date: BookHebrewDateInner) -> Date<CivilHebrew> {
        let month = biblical_date.0.month;
        let mut civil_year = biblical_date.0.year;
        let mut civil_month;

        if month >= TISHRI {
            civil_month = (month - 6) % 12;
            if civil_month == 0 {
                civil_month = 12;
            }
        } else {
            civil_month = month + 6;
            civil_year -= 1;
        }

        if Self::is_leap_year(civil_year) && civil_month > 6 {
            civil_month += 1;
        }

        Date::try_new_civil_hebrew_date(civil_year, civil_month, day).unwrap() // Safe unwrap
    }

    // ADAR = 12, ADARII = 13
    #[allow(dead_code)]
    fn last_month_of_civil_hebrew_year(h_year: i32) -> u8 {
        if Self::is_leap_year(h_year) {
            ADARII
        } else {
            ADAR
        }
    }

    // Sabbatical Year of the Civil Hebrew Calendar
    #[allow(dead_code)]
    fn is_civil_hebrew_sabbatical_year(h_year: i32) -> bool {
        div_rem_euclid(h_year, 7).1 == 0
    }

    // Number of days elapsed from the (Sunday) noon prior to the epoch of the Civil Hebrew Calendar to the molad of Tishri of Civil Hebrew year (h_year) or one day later
    #[allow(dead_code)]
    fn civil_hebrew_calendar_elapsed_days(h_year: i32) -> i32 {
        todo!()
    }

    // Delays to start of civilHebrew year to keep ordinary year in range 353-356 and leap year in range 383-386
    #[allow(dead_code)]
    fn civil_hebrew_year_length_correction(h_year: i32) -> u8 {
        todo!()
    }

    // Fixed date of Civil Hebrew new year
    #[allow(dead_code)]
    pub(crate) fn civil_hebrew_new_year(h_year: i32) -> RataDie {
        todo!()
    }

    #[allow(dead_code)]
    fn days_in_hebrew_year(h_year: i32) -> u16 {
        todo!()
    }

    // True if the month Marheshvan is going to be long in given CivilHebrew year
    #[allow(dead_code)]
    fn is_long_marheshvan(h_year: i32) -> bool {
        let long_marheshavan_year_lengths = [355, 385];
        long_marheshavan_year_lengths.contains(&Self::days_in_hebrew_year(h_year))
    }
    // True if the month Kislve is going to be short in given CivilHebrew year
    #[allow(dead_code)]
    fn is_short_kislev(h_year: i32) -> bool {
        let short_kislev_year_lengths = [353, 383];
        short_kislev_year_lengths.contains(&Self::days_in_hebrew_year(h_year))
    }

    // Last day of month (h_month) in CivilHebrew year (h_year)
    #[allow(dead_code)]
    fn last_day_of_civil_hebrew_month(h_year: i32, h_month: u8) -> u8 {
        match h_month {
            IYYAR | TAMMUZ | ELUL | TEVET | ADARII => 29,
            ADAR => {
                if !Self::is_leap_year(h_year) {
                    29
                } else {
                    30
                }
            }
            MARHESHVAN => {
                if !Self::is_long_marheshvan(h_year) {
                    29
                } else {
                    30
                }
            }
            KISLEV => {
                if Self::is_short_kislev(h_year) {
                    29
                } else {
                    30
                }
            }
            _ => 30,
        }
    }

    // "Fixed" is a day count representation of calendars staring from Jan 1st of year 1 of the Georgian Calendar.
    // The fixed date algorithms are from
    // Dershowitz, Nachum, and Edward M. Reingold. _Calendrical calculations_. Cambridge University Press, 2008.
    #[allow(dead_code)]
    fn fixed_from_civil_hebrew(date: CivilHebrewDateInner) -> RataDie {
        todo!()
    }

    #[allow(dead_code, clippy::unwrap_used)]
    fn civil_hebrew_from_fixed(date: RataDie) -> Date<CivilHebrew> {
        todo!()
    }

    fn year_as_hebrew(year: i32) -> types::FormattableYear {
        types::FormattableYear {
            era: types::Era(tinystr!(16, "hebrew")),
            number: year,
            cyclic: None,
            related_iso: None,
        }
    }
}

impl Date<CivilHebrew> {
    /// Construct new CivilHebrew Date.
    ///
    ///
    /// ```rust
    /// use icu::calendar::Date;
    ///
    /// let date_hebrew = Date::try_new_civil_hebrew_date(3425, 4, 25)
    ///     .expect("Failed to initialize CivilHebrew Date instance.");
    ///
    /// assert_eq!(date_hebrew.year().number, 3425);
    /// assert_eq!(date_hebrew.month().ordinal, 4);
    /// assert_eq!(date_hebrew.day_of_month().0, 25);
    /// ```
    pub fn try_new_civil_hebrew_date(
        year: i32,
        month: u8,
        day: u8,
    ) -> Result<Date<CiivlHebrew>, CalendarError> {
        ArithmeticDate::new_from_lunar_ordinals(year, month, day)
            .map(CiivlHebrewDateInner)
            .map(|inner| Date::from_raw(inner, CiivlHebrew))
    }
}

impl DateTime<CivilHebrew> {
    /// Construct a new CivilHebrew datetime from integers.
    ///
    /// ```rust
    /// use icu::calendar::DateTime;
    ///
    /// let datetime_hebrew = DateTime::try_new_civil_hebrew_datetime(4201, 10, 11, 13, 1, 0)
    ///     .expect("Failed to initialize CivilHebrew DateTime instance");
    ///
    /// assert_eq!(datetime_hebrew.date.year().number, 4201);
    /// assert_eq!(datetime_hebrew.date.month().ordinal, 10);
    /// assert_eq!(datetime_hebrew.date.day_of_month().0, 11);
    /// assert_eq!(datetime_hebrew.time.hour.number(), 13);
    /// assert_eq!(datetime_hebrew.time.minute.number(), 1);
    /// assert_eq!(datetime_hebrew.time.second.number(), 0);
    /// ```
    pub fn try_new_civil_hebrew_datetime(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Result<DateTime<CivilHebrew>, CalendarError> {
        Ok(DateTime {
            date: Date::try_new_civil_hebrew_date(year, month, day)?,
            time: types::Time::try_new(hour, minute, second, 0)?,
        })
    }
}

// BIBLICAL HEBREW CALENDAR FUNCTIONS

impl BookHebrew {

    /// Constructs a new BookHebrew Object
    pub fn new() -> Self {
        Self
    }

    // Moment of mean conjunction (New Moon) of h_month in BookHebrew
    // Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L2244
    #[allow(dead_code)]
    pub(crate) fn molad(h_year: i32, h_month: u8) -> Moment {
        let y = if h_month < TISHRI { h_year + 1 } else { h_year }; // Treat Nisan as start of year

        let months_elapsed = (h_month as f64 - TISHRI as f64) // Months this year
            + (libm::floor((235.0 * y as f64 - 234.0) / 19.0)); // Months until New Year.

        Moment::new(
            FIXED_HEBREW_EPOCH.to_f64_date() - (876.0 / 25920.0)
                + months_elapsed * (29.0 + (1.0 / 2.0) + (793.0 / 25920.0)),
        )
    }

    // ADAR = 12, ADARII = 13
    // Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L2217
    #[allow(dead_code)]
    fn last_month_of_book_hebrew_year(h_year: i32) -> u8 {
        if Self::is_hebrew_leap_year(h_year) {
            ADARII
        } else {
            ADAR
        }
    }

    // Sabbatical Year of the BookHebrew Calendar
    // Lisp code refernence: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L2224
    #[allow(dead_code)]
    fn is_book_hebrew_sabbatical_year(h_year: i32) -> bool {
        div_rem_euclid(h_year, 7).1 == 0
    }

    // Number of days elapsed from the (Sunday) noon prior to the epoch of the BookHebrew Calendar to the molad of Tishri of BookHebrew year (h_year) or one day later
    // Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L2261
    #[allow(dead_code)]
    fn book_hebrew_calendar_elapsed_days(h_year: i32) -> i32 {
        let months_elapsed = libm::floor((235.0 * h_year as f64 - 234.0) / 19.0);
        let parts_elapsed = 12084.0 + 13753.0 * months_elapsed;
        let days = 29.0 * months_elapsed + libm::floor(parts_elapsed / 25920.0);

        if div_rem_euclid_f64(3.0 * (days + 1.0), 7.0).1 < 3.0 {
            days as i32 + 1
        } else {
            days as i32
        }
    }

    // Delays to start of BookHebrew year to keep ordinary year in range 353-356 and leap year in range 383-386
    // Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L2301
    #[allow(dead_code)]
    fn book_hebrew_year_length_correction(h_year: i32) -> u8 {
        let ny0 = Self::book_hebrew_calendar_elapsed_days(h_year - 1);
        let ny1 = Self::book_hebrew_calendar_elapsed_days(h_year);
        let ny2 = Self::book_hebrew_calendar_elapsed_days(h_year + 1);

        if (ny2 - ny1) == 356 {
            2
        } else if (ny1 - ny0) == 382 {
            1
        } else {
            0
        }
    }

    // Fixed date of BookHebrew new year
    // Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L2294
    #[allow(dead_code)]
    pub(crate) fn book_hebrew_new_year(h_year: i32) -> RataDie {
        RataDie::new(
            FIXED_HEBREW_EPOCH.to_i64_date()
                + Self::book_hebrew_calendar_elapsed_days(h_year) as i64
                + Self::book_hebrew_year_length_correction(h_year) as i64,
        )
    }

    // Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L2315
    #[allow(dead_code)]
    fn days_in_hebrew_year(h_year: i32) -> u16 {
        (Self::book_hebrew_new_year(1 + h_year) - Self::book_hebrew_new_year(h_year)) as u16
    }

    #[allow(dead_code)]
    fn is_hebrew_leap_year(h_year: i32) -> bool {
        div_rem_euclid(7 * year + 1, 19).1 < 7
    }
    // True if the month Marheshvan is going to be long in given BookHebrew year
    // Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L2321
    #[allow(dead_code)]
    fn is_long_marheshvan(h_year: i32) -> bool {
        let long_marheshavan_year_lengths = [355, 385];
        long_marheshavan_year_lengths.contains(&Self::days_in_hebrew_year(h_year))
    }
    // True if the month Kislve is going to be short in given BookHebrew year
    // Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L2326
    #[allow(dead_code)]
    fn is_short_kislev(h_year: i32) -> bool {
        let short_kislev_year_lengths = [353, 383];
        short_kislev_year_lengths.contains(&Self::days_in_hebrew_year(h_year))
    }

    // Last day of month (h_month) in BookHebrew year (h_year)
    // Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L2230
    #[allow(dead_code)]
    fn last_day_of_book_hebrew_month(h_year: i32, h_month: u8) -> u8 {
        match h_month {
            IYYAR | TAMMUZ | ELUL | TEVET | ADARII => 29,
            ADAR => {
                if !Self::is_hebrew_leap_year(h_year) {
                    29
                } else {
                    30
                }
            }
            MARHESHVAN => {
                if !Self::is_long_marheshvan(h_year) {
                    29
                } else {
                    30
                }
            }
            KISLEV => {
                if Self::is_short_kislev(h_year) {
                    29
                } else {
                    30
                }
            }
            _ => 30,
        }
    }

    // "Fixed" is a day count representation of calendars staring from Jan 1st of year 1 of the Georgian Calendar.
    // The fixed date algorithms are from
    // Dershowitz, Nachum, and Edward M. Reingold. _Calendrical calculations_. Cambridge University Press, 2008.
    //
    // Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L2331
    #[allow(dead_code)]
    fn fixed_from_book_hebrew(date: BookHebrewDateInner) -> RataDie {
        let year = date.0.year;
        let month = date.0.month;
        let day = date.0.day;

        let mut total_days = Self::book_hebrew_new_year(year) + day.into() - 1; // (day - 1) Days so far this month.

        if month < TISHRI {
            // Then add days in prior months this year before
            for m in (TISHRI..=Self::last_month_of_book_hebrew_year(year)).chain(NISAN..month) {
                total_days += Self::last_day_of_book_hebrew_month(year, m).into();
            }
        } else {
            // Else add days in prior months this year
            for m in TISHRI..month {
                total_days += Self::last_day_of_book_hebrew_month(year, m).into();
            }
        }

        total_days
    }

    // Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L2352
    #[allow(dead_code, clippy::unwrap_used)]
    fn book_hebrew_from_fixed(date: RataDie) -> Date<BookHebrew> {
        let approx = helpers::i64_to_i32(
            (div_rem_euclid_f64((date - FIXED_HEBREW_EPOCH) as f64, 35975351.0 / 98496.0).0) as i64, //  The value 35975351/98496, the average length of a BookHebrew year, can be approximated by 365.25
        )
        .saturate()
            + 1;

        // Search forward for the year
        let year_condition = |year: i32| Self::book_hebrew_new_year(year) <= date;
        let year = final_func(approx - 1, year_condition);

        // Starting month for search for month.
        let start = if date
            < Self::fixed_from_book_hebrew(BookHebrewDateInner(ArithmeticDate::new_unchecked(
                year, NISAN, 1,
            ))) {
            TISHRI
        } else {
            NISAN
        };

        let month_condition = |m: u8| {
            date <= Self::fixed_from_book_hebrew(BookHebrewDateInner(
                ArithmeticDate::new_unchecked(
                    year,
                    m,
                    Self::last_day_of_book_hebrew_month(year, m),
                ),
            ))
        };
        // Search forward from either Tishri or Nisan.
        let month = next_u8(start, month_condition);

        // Calculate the day by subtraction.
        let day = (date
            - Self::fixed_from_book_hebrew(BookHebrewDateInner(ArithmeticDate::new_unchecked(
                year, month, 1,
            ))))
            + 1;

        Date::try_new_civil_hebrew_date(year, month, day as u8).unwrap() // Safe unwrap
    }

    fn year_as_hebrew(year: i32) -> types::FormattableYear {
        types::FormattableYear {
            era: types::Era(tinystr!(16, "hebrew")),
            number: year,
            cyclic: None,
            related_iso: None,
        }
    }
}

#[cfg(test)]
mod tests {

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

    static HEBREW_DATES: [DateCase; 33] = [
        DateCase {
            year: 3174,
            month: 5,
            day: 10,
        },
        DateCase {
            year: 3593,
            month: 9,
            day: 25,
        },
        DateCase {
            year: 3831,
            month: 7,
            day: 3,
        },
        DateCase {
            year: 3896,
            month: 7,
            day: 9,
        },
        DateCase {
            year: 4230,
            month: 10,
            day: 18,
        },
        DateCase {
            year: 4336,
            month: 3,
            day: 4,
        },
        DateCase {
            year: 4455,
            month: 8,
            day: 13,
        },
        DateCase {
            year: 4773,
            month: 2,
            day: 6,
        },
        DateCase {
            year: 4856,
            month: 2,
            day: 23,
        },
        DateCase {
            year: 4950,
            month: 1,
            day: 7,
        },
        DateCase {
            year: 5000,
            month: 13,
            day: 8,
        },
        DateCase {
            year: 5048,
            month: 1,
            day: 21,
        },
        DateCase {
            year: 5058,
            month: 2,
            day: 7,
        },
        DateCase {
            year: 5151,
            month: 4,
            day: 1,
        },
        DateCase {
            year: 5196,
            month: 11,
            day: 7,
        },
        DateCase {
            year: 5252,
            month: 1,
            day: 3,
        },
        DateCase {
            year: 5314,
            month: 7,
            day: 1,
        },
        DateCase {
            year: 5320,
            month: 12,
            day: 27,
        },
        DateCase {
            year: 5408,
            month: 3,
            day: 20,
        },
        DateCase {
            year: 5440,
            month: 4,
            day: 3,
        },
        DateCase {
            year: 5476,
            month: 5,
            day: 5,
        },
        DateCase {
            year: 5528,
            month: 4,
            day: 4,
        },
        DateCase {
            year: 5579,
            month: 5,
            day: 11,
        },
        DateCase {
            year: 5599,
            month: 1,
            day: 12,
        },
        DateCase {
            year: 5663,
            month: 1,
            day: 22,
        },
        DateCase {
            year: 5689,
            month: 5,
            day: 19,
        },
        DateCase {
            year: 5702,
            month: 7,
            day: 8,
        },
        DateCase {
            year: 5703,
            month: 1,
            day: 14,
        },
        DateCase {
            year: 5704,
            month: 7,
            day: 8,
        },
        DateCase {
            year: 5752,
            month: 13,
            day: 12,
        },
        DateCase {
            year: 5756,
            month: 12,
            day: 5,
        },
        DateCase {
            year: 5799,
            month: 8,
            day: 12,
        },
        DateCase {
            year: 5854,
            month: 5,
            day: 5,
        },
    ];

    static EXPECTED_MOLAD_DATES: [f64; 33] = [
        -1850718767f64 / 8640f64,
        -1591805959f64 / 25920f64,
        660097927f64 / 25920f64,
        1275506059f64 / 25920f64,
        4439806081f64 / 25920f64,
        605235101f64 / 2880f64,
        3284237627f64 / 12960f64,
        9583515841f64 / 25920f64,
        2592403883f64 / 6480f64,
        2251656649f64 / 5184f64,
        11731320839f64 / 25920f64,
        12185988041f64 / 25920f64,
        6140833583f64 / 12960f64,
        6581722991f64 / 12960f64,
        6792982499f64 / 12960f64,
        4705980311f64 / 8640f64,
        14699670013f64 / 25920f64,
        738006961f64 / 1296f64,
        1949499007f64 / 3240f64,
        5299956319f64 / 8640f64,
        3248250415f64 / 5184f64,
        16732660061f64 / 25920f64,
        17216413717f64 / 25920f64,
        1087650871f64 / 1620f64,
        2251079609f64 / 3240f64,
        608605601f64 / 864f64,
        306216383f64 / 432f64,
        18387526207f64 / 25920f64,
        3678423761f64 / 5184f64,
        1570884431f64 / 2160f64,
        18888119389f64 / 25920f64,
        19292268013f64 / 25920f64,
        660655045f64 / 864f64,
    ];

    static EXPECTED_LAST_HEBREW_MONTH: [u8; 33] = [
        12, 12, 12, 12, 12, 12, 12, 12, 13, 12, 13, 12, 12, 12, 12, 13, 12, 13, 12, 13, 12, 12, 12,
        12, 12, 13, 12, 13, 12, 13, 12, 12, 12,
    ];

    static EXPECTED_HEBREW_SABBATICAL_YEAR: [bool; 33] = [
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, true, false, false, false, false, true, false, true, false,
        false, false, false, false, false, false, false,
    ];

    static EXPECTED_HEBREW_ELASPED_CALENDAR_DAYS: [i32; 33] = [
        1158928, 1311957, 1398894, 1422636, 1544627, 1583342, 1626812, 1742956, 1773254, 1807597,
        1825848, 1843388, 1847051, 1881010, 1897460, 1917895, 1940545, 1942729, 1974889, 1986554,
        1999723, 2018712, 2037346, 2044640, 2068027, 2077507, 2082262, 2082617, 2083000, 2100511,
        2101988, 2117699, 2137779,
    ];

    static EXPECTED_FIXED_HEBREW_NEW_YEAR: [i64; 33] = [
        -214497, -61470, 25467, 49209, 171200, 209915, 253385, 369529, 399827, 434172, 452421,
        469963, 473624, 507583, 524033, 544468, 567118, 569302, 601462, 613127, 626296, 645285,
        663919, 671213, 694600, 704080, 708835, 709190, 709573, 727084, 728561, 744272, 764352,
    ];

    static EXPECTED_DAYS_IN_HEBREW_YEAR: [u16; 33] = [
        354, 354, 355, 355, 355, 355, 355, 353, 383, 354, 383, 354, 354, 355, 353, 383, 353, 385,
        353, 383, 355, 354, 354, 354, 355, 385, 355, 383, 354, 385, 355, 354, 355,
    ];

    static EXPECTED_MARHESHVAN_VALUES: [bool; 33] = [
        false, false, true, true, true, true, true, false, false, false, false, false, false, true,
        false, false, false, true, false, false, true, false, false, false, true, true, true,
        false, false, true, true, false, true,
    ];

    static EXPECTED_KISLEV_VALUES: [bool; 33] = [
        false, false, false, false, false, false, false, true, true, false, true, false, false,
        false, true, true, true, false, true, true, false, false, false, false, false, false,
        false, true, false, false, false, false, false,
    ];

    static EXPECTED_DAY_IN_MONTH: [u8; 33] = [
        30, 30, 30, 30, 29, 30, 30, 29, 29, 30, 29, 30, 29, 29, 30, 30, 30, 30, 30, 29, 30, 29, 30,
        30, 30, 30, 30, 30, 30, 29, 29, 29, 30,
    ];

    #[test]
    fn test_hebrew_epoch() {
        // page 119 of the Calendrical Calculations book
        let fixed_hebrew_date = -1373427.0;
        assert_eq!(FIXED_HEBREW_EPOCH.to_f64_date(), fixed_hebrew_date);
    }

    #[test]
    fn test_hebrew_molad() {
        let precision = 1_00000f64;
        for (case, expected) in HEBREW_DATES.iter().zip(EXPECTED_MOLAD_DATES.iter()) {
            let molad =
                (BookHebrew::molad(case.year, case.month).inner() * precision).round() / precision;
            let final_expected = (expected * precision).round() / precision;
            assert_eq!(molad, final_expected, "{case:?}");
        }
    }

    #[test]
    fn test_last_hebrew_month() {
        for (case, expected) in HEBREW_DATES.iter().zip(EXPECTED_LAST_HEBREW_MONTH.iter()) {
            let last_month = BookHebrew::last_month_of_book_hebrew_year(case.year);
            assert_eq!(last_month, *expected);
        }
    }

    #[test]
    fn test_hebrew_sabbatical_year() {
        for (case, expected) in HEBREW_DATES
            .iter()
            .zip(EXPECTED_HEBREW_SABBATICAL_YEAR.iter())
        {
            let boolean = BookHebrew::is_book_hebrew_sabbatical_year(case.year);
            assert_eq!(boolean, *expected);
        }
    }

    #[test]
    fn test_hebrew_calendar_elapsed_days() {
        for (case, expected) in HEBREW_DATES
            .iter()
            .zip(EXPECTED_HEBREW_ELASPED_CALENDAR_DAYS.iter())
        {
            let elapsed_days = BookHebrew::book_hebrew_calendar_elapsed_days(case.year);
            assert_eq!(elapsed_days, *expected);
        }
    }

    #[test]
    fn test_hebrew_year_length_correction() {
        let year_length_correction: [u8; 33] = [
            2, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0,
        ];
        for (case, expected) in HEBREW_DATES.iter().zip(year_length_correction.iter()) {
            let correction = BookHebrew::book_hebrew_year_length_correction(case.year);
            assert_eq!(correction, *expected);
        }
    }

    #[test]
    fn test_hebrew_new_year() {
        for (case, expected) in HEBREW_DATES
            .iter()
            .zip(EXPECTED_FIXED_HEBREW_NEW_YEAR.iter())
        {
            let f_date = BookHebrew::book_hebrew_new_year(case.year);
            assert_eq!(f_date.to_i64_date(), *expected);
        }
    }

    #[test]
    fn test_days_in_hebrew_year() {
        for (case, expected) in HEBREW_DATES.iter().zip(EXPECTED_DAYS_IN_HEBREW_YEAR.iter()) {
            let days_in_year = BookHebrew::days_in_hebrew_year(case.year);
            assert_eq!(days_in_year, *expected);
        }
    }

    #[test]
    fn test_long_marheshvan() {
        for (case, expected) in HEBREW_DATES.iter().zip(EXPECTED_MARHESHVAN_VALUES.iter()) {
            let marsheshvan = BookHebrew::is_long_marheshvan(case.year);
            assert_eq!(marsheshvan, *expected);
        }
    }

    #[test]
    fn test_short_kislev() {
        for (case, expected) in HEBREW_DATES.iter().zip(EXPECTED_KISLEV_VALUES.iter()) {
            let kislev = BookHebrew::is_short_kislev(case.year);
            assert_eq!(kislev, *expected);
        }
    }

    #[test]
    fn test_last_day_in_hebrew_month() {
        for (case, expected) in HEBREW_DATES.iter().zip(EXPECTED_DAY_IN_MONTH.iter()) {
            let days_in_month = BookHebrew::last_day_of_book_hebrew_month(case.year, case.month);
            assert_eq!(days_in_month, *expected);
        }
    }

    #[test]
    fn test_fixed_from_hebrew() {
        for (case, f_date) in HEBREW_DATES.iter().zip(TEST_FIXED_DATE.iter()) {
            let date = BookHebrewDateInner(ArithmeticDate::new_unchecked(
                case.year, case.month, case.day,
            ));
            assert_eq!(
                BookHebrew::fixed_from_book_hebrew(date),
                RataDie::new(*f_date),
                "{case:?}"
            );
        }
    }

    #[test]
    fn test_hebrew_from_fixed() {
        for (case, f_date) in HEBREW_DATES.iter().zip(TEST_FIXED_DATE.iter()) {
            let date = Date::try_new_hebrew_date(case.year, case.month, case.day).unwrap();
            assert_eq!(
                BookHebrew::book_hebrew_from_fixed(RataDie::new(*f_date)),
                date,
                "{case:?}"
            );
        }
    }

    #[test]
    fn test_book_month() {
        let iso_dates: [Date<Iso>; 48] = [
            *Date::try_new_iso_date(2021, 1, 10).unwrap(),
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

        for date in iso_dates.iter() {
            println!("{:?}", Date::to_calendar(date, BookHebrew))
        }
    }

    #[test]
    fn test_icu_bug_22441() {
        assert_wq!(BookHebrew::days_in_hebrew_year(88369), 383);
    }
}
