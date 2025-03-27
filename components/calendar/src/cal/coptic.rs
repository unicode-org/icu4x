// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains types and implementations for the Coptic calendar.
//!
//! ```rust
//! use icu::calendar::{cal::Coptic, Date};
//!
//! let date_iso = Date::try_new_iso(1970, 1, 2)
//!     .expect("Failed to initialize ISO Date instance.");
//! let date_coptic = Date::new_from_iso(date_iso, Coptic);
//!
//! assert_eq!(date_coptic.year().era_year_or_extended(), 1686);
//! assert_eq!(date_coptic.month().ordinal, 4);
//! assert_eq!(date_coptic.day_of_month().0, 24);
//! ```

use crate::cal::iso::Iso;
use crate::calendar_arithmetic::{ArithmeticDate, CalendarArithmetic};
use crate::error::{year_check, DateError};
use crate::{types, Calendar, Date, DateDuration, DateDurationUnit, RangeError};
use calendrical_calculations::helpers::I32CastError;
use calendrical_calculations::rata_die::RataDie;
use tinystr::tinystr;

/// The [Coptic Calendar]
///
/// The [Coptic calendar] is a solar calendar used by the Coptic Orthodox Church, with twelve normal months
/// and a thirteenth small epagomenal month.
///
/// This type can be used with [`Date`] to represent dates in this calendar.
///
/// [Coptic calendar]: https://en.wikipedia.org/wiki/Coptic_calendar
///
/// # Era codes
///
/// This calendar uses two era codes: `coptic-inverse`, and `coptic`, corresponding to the Before Diocletian and After Diocletian/Anno Martyrum
/// eras. 1 A.M. is equivalent to 284 C.E.
///
/// # Month codes
///
/// This calendar supports 13 solar month codes (`"M01" - "M13"`), with `"M13"` being used for the short epagomenal month
/// at the end of the year.
#[derive(Copy, Clone, Debug, Hash, Default, Eq, PartialEq, PartialOrd, Ord)]
#[allow(clippy::exhaustive_structs)] // this type is stable
pub struct Coptic;

/// The inner date type used for representing [`Date`]s of [`Coptic`]. See [`Date`] and [`Coptic`] for more details.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct CopticDateInner(pub(crate) ArithmeticDate<Coptic>);

impl CalendarArithmetic for Coptic {
    type YearInfo = ();

    fn days_in_provided_month(year: i32, month: u8, _data: ()) -> u8 {
        if (1..=12).contains(&month) {
            30
        } else if month == 13 {
            if Self::provided_year_is_leap(year, ()) {
                6
            } else {
                5
            }
        } else {
            0
        }
    }

    fn months_in_provided_year(_: i32, _data: ()) -> u8 {
        13
    }

    fn provided_year_is_leap(year: i32, _data: ()) -> bool {
        year.rem_euclid(4) == 3
    }

    fn last_month_day_in_provided_year(year: i32, _data: ()) -> (u8, u8) {
        if Self::provided_year_is_leap(year, ()) {
            (13, 6)
        } else {
            (13, 5)
        }
    }

    fn days_in_provided_year(year: i32, _data: ()) -> u16 {
        if Self::provided_year_is_leap(year, ()) {
            366
        } else {
            365
        }
    }
}

impl Calendar for Coptic {
    type DateInner = CopticDateInner;
    fn date_from_codes(
        &self,
        era: Option<&str>,
        year: i32,
        month_code: types::MonthCode,
        day: u8,
    ) -> Result<Self::DateInner, DateError> {
        let year = match era {
            Some("coptic") | None => year_check(year, 1..)?,
            Some("coptic-inverse") => 1 - year_check(year, 1..)?,
            Some(_) => return Err(DateError::UnknownEra),
        };

        ArithmeticDate::new_from_codes(self, year, month_code, day).map(CopticDateInner)
    }
    fn date_from_iso(&self, iso: Date<Iso>) -> CopticDateInner {
        let fixed_iso = Iso::to_fixed(iso);
        Self::coptic_from_fixed(fixed_iso)
    }

    fn date_to_iso(&self, date: &Self::DateInner) -> Date<Iso> {
        let fixed_coptic = Coptic::fixed_from_coptic(date.0);
        Iso::from_fixed(fixed_coptic)
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
        date.0.offset_date(offset, &());
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

    fn year(&self, date: &Self::DateInner) -> types::YearInfo {
        let year = date.0.year;
        if year > 0 {
            types::YearInfo::new(
                year,
                types::EraYear {
                    standard_era: tinystr!(16, "coptic").into(),
                    formatting_era: types::FormattingEra::Index(1, tinystr!(16, "AD")),
                    era_year: year,
                    ambiguity: types::YearAmbiguity::CenturyRequired,
                },
            )
        } else {
            types::YearInfo::new(
                year,
                types::EraYear {
                    standard_era: tinystr!(16, "coptic-inverse").into(),
                    formatting_era: types::FormattingEra::Index(0, tinystr!(16, "BD")),
                    era_year: 1 - year,
                    ambiguity: types::YearAmbiguity::EraAndCenturyRequired,
                },
            )
        }
    }

    fn is_in_leap_year(&self, date: &Self::DateInner) -> bool {
        Self::provided_year_is_leap(date.0.year, ())
    }

    fn month(&self, date: &Self::DateInner) -> types::MonthInfo {
        date.0.month()
    }

    fn day_of_month(&self, date: &Self::DateInner) -> types::DayOfMonth {
        date.0.day_of_month()
    }

    fn day_of_year(&self, date: &Self::DateInner) -> types::DayOfYear {
        date.0.day_of_year()
    }

    fn debug_name(&self) -> &'static str {
        "Coptic"
    }

    fn any_calendar_kind(&self) -> Option<crate::AnyCalendarKind> {
        Some(crate::any_calendar::IntoAnyCalendar::kind(self))
    }
}

impl Coptic {
    fn fixed_from_coptic(date: ArithmeticDate<Coptic>) -> RataDie {
        calendrical_calculations::coptic::fixed_from_coptic(date.year, date.month, date.day)
    }

    pub(crate) fn coptic_from_fixed(date: RataDie) -> CopticDateInner {
        let (year, month, day) = match calendrical_calculations::coptic::coptic_from_fixed(date) {
            Err(I32CastError::BelowMin) => return CopticDateInner(ArithmeticDate::min_date()),
            Err(I32CastError::AboveMax) => return CopticDateInner(ArithmeticDate::max_date()),
            Ok(ymd) => ymd,
        };

        CopticDateInner(ArithmeticDate::new_unchecked(year, month, day))
    }
}

impl Date<Coptic> {
    /// Construct new Coptic Date.
    ///
    /// Negative years are in the B.D. era, starting with 0 = 1 B.D.
    ///
    /// ```rust
    /// use icu::calendar::Date;
    ///
    /// let date_coptic = Date::try_new_coptic(1686, 5, 6)
    ///     .expect("Failed to initialize Coptic Date instance.");
    ///
    /// assert_eq!(date_coptic.year().era_year_or_extended(), 1686);
    /// assert_eq!(date_coptic.month().ordinal, 5);
    /// assert_eq!(date_coptic.day_of_month().0, 6);
    /// ```
    pub fn try_new_coptic(year: i32, month: u8, day: u8) -> Result<Date<Coptic>, RangeError> {
        ArithmeticDate::new_from_ordinals(year, month, day)
            .map(CopticDateInner)
            .map(|inner| Date::from_raw(inner, Coptic))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_coptic_regression() {
        // https://github.com/unicode-org/icu4x/issues/2254
        let iso_date = Date::try_new_iso(-100, 3, 3).unwrap();
        let coptic = iso_date.to_calendar(Coptic);
        let recovered_iso = coptic.to_iso();
        assert_eq!(iso_date, recovered_iso);
    }
}
