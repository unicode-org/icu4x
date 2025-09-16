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
//! assert_eq!(date_coptic.era_year().year, 1686);
//! assert_eq!(date_coptic.month().ordinal, 4);
//! assert_eq!(date_coptic.day_of_month().0, 24);
//! ```

use crate::cal::iso::{Iso, IsoDateInner};
use crate::calendar_arithmetic::{ArithmeticDate, CalendarArithmetic};
use crate::calendar_arithmetic::{CalendarNonLunisolar, CalendarWithEras};
use crate::error::DateError;
use crate::options::{DateFromFieldsOptions, Overflow};
use crate::types::DateFields;
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
/// This calendar uses a single code: `am`, corresponding to the After Diocletian/Anno Martyrum
/// era. 1 A.M. is equivalent to 284 C.E.
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
    type YearInfo = i32;

    fn days_in_provided_month(year: i32, month: u8) -> u8 {
        if (1..=12).contains(&month) {
            30
        } else if month == 13 {
            if Self::provided_year_is_leap(year) {
                6
            } else {
                5
            }
        } else {
            0
        }
    }

    fn months_in_provided_year(_: i32) -> u8 {
        13
    }

    fn provided_year_is_leap(year: i32) -> bool {
        year.rem_euclid(4) == 3
    }

    fn last_month_day_in_provided_year(year: i32) -> (u8, u8) {
        if Self::provided_year_is_leap(year) {
            (13, 6)
        } else {
            (13, 5)
        }
    }

    fn days_in_provided_year(year: i32) -> u16 {
        if Self::provided_year_is_leap(year) {
            366
        } else {
            365
        }
    }
}

impl CalendarWithEras for Coptic {
    #[inline]
    fn era_year_to_monotonic(&self, era: &str, era_year: i32) -> Result<i32, DateError> {
        match era {
            "am" => Ok(era_year),
            _ => Err(DateError::UnknownEra),
        }
    }
}

impl CalendarNonLunisolar for Coptic {
    #[inline]
    fn fixed_monotonic_reference_year(&self) -> i32 {
        todo!()
    }
}

impl crate::cal::scaffold::UnstableSealed for Coptic {}
impl Calendar for Coptic {
    type DateInner = CopticDateInner;
    type Year = types::EraYear;
    fn from_fields(
        &self,
        fields: DateFields,
        options: DateFromFieldsOptions,
    ) -> Result<Self::DateInner, DateError> {
        let (year, month, day) = fields.get_non_lunisolar_ordinals(self, options)?;
        ArithmeticDate::new_from_ordinals(year, month, day, options)
            .map(CopticDateInner)
            .map_err(|e| e.maybe_with_month_code(fields.month_code))
    }

    fn from_rata_die(&self, rd: RataDie) -> Self::DateInner {
        CopticDateInner(
            match calendrical_calculations::coptic::coptic_from_fixed(rd) {
                Err(I32CastError::BelowMin) => ArithmeticDate::min_date(),
                Err(I32CastError::AboveMax) => ArithmeticDate::max_date(),
                Ok((year, month, day)) => ArithmeticDate::new_unchecked(year, month, day),
            },
        )
    }

    fn to_rata_die(&self, date: &Self::DateInner) -> RataDie {
        calendrical_calculations::coptic::fixed_from_coptic(date.0.year, date.0.month, date.0.day)
    }

    fn from_iso(&self, iso: IsoDateInner) -> CopticDateInner {
        self.from_rata_die(Iso.to_rata_die(&iso))
    }

    fn to_iso(&self, date: &Self::DateInner) -> IsoDateInner {
        Iso.from_rata_die(self.to_rata_die(date))
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

    fn year_info(&self, date: &Self::DateInner) -> Self::Year {
        let year = date.0.monotonic_year();
        types::EraYear {
            era: tinystr!(16, "am"),
            era_index: Some(0),
            year,
            monotonic_year: year,
            ambiguity: types::YearAmbiguity::CenturyRequired,
        }
    }

    fn is_in_leap_year(&self, date: &Self::DateInner) -> bool {
        Self::provided_year_is_leap(date.0.year)
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

    fn calendar_algorithm(&self) -> Option<crate::preferences::CalendarAlgorithm> {
        Some(crate::preferences::CalendarAlgorithm::Coptic)
    }
}

impl Date<Coptic> {
    /// Construct new Coptic Date.
    ///
    /// ```rust
    /// use icu::calendar::Date;
    ///
    /// let date_coptic = Date::try_new_coptic(1686, 5, 6)
    ///     .expect("Failed to initialize Coptic Date instance.");
    ///
    /// assert_eq!(date_coptic.era_year().year, 1686);
    /// assert_eq!(date_coptic.month().ordinal, 5);
    /// assert_eq!(date_coptic.day_of_month().0, 6);
    /// ```
    pub fn try_new_coptic(year: i32, month: u8, day: u8) -> Result<Date<Coptic>, RangeError> {
        ArithmeticDate::new_from_ordinals(year, month, day, Default::default())
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
