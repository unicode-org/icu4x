// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cal::iso::{Iso, IsoDateInner};
use crate::calendar_arithmetic::ArithmeticDate;
use crate::calendar_arithmetic::{ArithmeticDateBuilder, DateFieldsResolver};
use crate::error::DateError;
use crate::options::DateFromFieldsOptions;
use crate::options::{DateAddOptions, DateDifferenceOptions};
use crate::{types, Calendar, Date, RangeError};
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

impl DateFieldsResolver for Coptic {
    type YearInfo = i32;

    fn days_in_provided_month(year: i32, month: u8) -> u8 {
        if (1..=12).contains(&month) {
            30
        } else if month == 13 {
            if year.rem_euclid(4) == 3 {
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
    #[inline]
    fn year_info_from_era(&self, era: &str, era_year: i32) -> Result<Self::YearInfo, DateError> {
        match era {
            "am" => Ok(era_year),
            _ => Err(DateError::UnknownEra),
        }
    }

    #[inline]
    fn year_info_from_extended(&self, extended_year: i32) -> Self::YearInfo {
        extended_year
    }

    #[inline]
    fn reference_year_from_month_day(
        &self,
        month_code: types::MonthCode,
        day: u8,
    ) -> Result<Self::YearInfo, DateError> {
        Coptic::reference_year_from_month_day(month_code, day)
    }

    #[inline]
    fn ordinal_month_from_code(
        &self,
        _year: &Self::YearInfo,
        month_code: types::MonthCode,
        _options: DateFromFieldsOptions,
    ) -> Result<u8, DateError> {
        match month_code.parsed() {
            Some((month_number @ 1..=13, false)) => Ok(month_number),
            _ => Err(DateError::UnknownMonthCode(month_code)),
        }
    }
}

impl Coptic {
    pub(crate) fn reference_year_from_month_day(
        month_code: types::MonthCode,
        day: u8,
    ) -> Result<i32, DateError> {
        let (ordinal_month, _is_leap) = month_code
            .parsed()
            .ok_or(DateError::UnknownMonthCode(month_code))?;
        // December 31, 1972 occurs on 4th month, 22nd day, 1689 AM
        let anno_martyrum_year = if ordinal_month < 4 || (ordinal_month == 4 && day <= 22) {
            1689
        // Note: this must be >=6, not just == 6, since we have not yet
        // applied a potential Overflow::Constrain.
        } else if ordinal_month == 13 && day >= 6 {
            // 1687 AM is a leap year
            1687
        } else {
            1688
        };
        Ok(anno_martyrum_year)
    }
}

impl crate::cal::scaffold::UnstableSealed for Coptic {}
impl Calendar for Coptic {
    type DateInner = CopticDateInner;
    type Year = types::EraYear;
    type DifferenceError = core::convert::Infallible;

    fn from_fields(
        &self,
        fields: types::DateFields,
        options: DateFromFieldsOptions,
    ) -> Result<Self::DateInner, DateError> {
        let builder = ArithmeticDateBuilder::try_from_fields(fields, self, options)?;
        ArithmeticDate::try_from_builder(builder, options)
            .map(CopticDateInner)
            .map_err(|e| e.maybe_with_month_code(fields.month_code))
    }

    fn from_rata_die(&self, rd: RataDie) -> Self::DateInner {
        CopticDateInner(
            match calendrical_calculations::coptic::coptic_from_fixed(rd) {
                Err(I32CastError::BelowMin) => ArithmeticDate::new_unchecked(i32::MIN, 1, 1),
                Err(I32CastError::AboveMax) => ArithmeticDate::new_unchecked(i32::MAX, 13, 6),
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
        Self::months_in_provided_year(date.0.year)
    }

    fn days_in_year(&self, date: &Self::DateInner) -> u16 {
        if self.is_in_leap_year(date) {
            366
        } else {
            365
        }
    }

    fn days_in_month(&self, date: &Self::DateInner) -> u8 {
        Self::days_in_provided_month(date.0.year, date.0.month)
    }

    fn add(
        &self,
        date: &Self::DateInner,
        duration: types::DateDuration,
        options: DateAddOptions,
    ) -> Result<Self::DateInner, DateError> {
        date.0.added(duration, self, options).map(CopticDateInner)
    }

    fn until(
        &self,
        date1: &Self::DateInner,
        date2: &Self::DateInner,
        options: DateDifferenceOptions,
    ) -> Result<types::DateDuration, Self::DifferenceError> {
        Ok(date1.0.until(&date2.0, self, options))
    }

    fn year_info(&self, date: &Self::DateInner) -> Self::Year {
        let year = date.0.year;
        types::EraYear {
            era: tinystr!(16, "am"),
            era_index: Some(0),
            year,
            extended_year: year,
            ambiguity: types::YearAmbiguity::CenturyRequired,
        }
    }

    fn is_in_leap_year(&self, date: &Self::DateInner) -> bool {
        date.0.year.rem_euclid(4) == 3
    }

    fn month(&self, date: &Self::DateInner) -> types::MonthInfo {
        self.month_code_from_ordinal(&date.0.year, date.0.month)
    }

    fn day_of_month(&self, date: &Self::DateInner) -> types::DayOfMonth {
        types::DayOfMonth(date.0.day)
    }

    fn day_of_year(&self, date: &Self::DateInner) -> types::DayOfYear {
        types::DayOfYear(
            (1..date.0.month)
                .map(|m| Self::days_in_provided_month(date.0.year, m) as u16)
                .sum::<u16>()
                + date.0.day as u16,
        )
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
        ArithmeticDate::try_from_ymd(year, month, day)
            .map(CopticDateInner)
            .map(|inner| Date::from_raw(inner, Coptic))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::{DateFromFieldsOptions, MissingFieldsStrategy, Overflow};
    use crate::types::{DateFields, MonthCode};

    #[test]
    fn test_coptic_regression() {
        // https://github.com/unicode-org/icu4x/issues/2254
        let iso_date = Date::try_new_iso(-100, 3, 3).unwrap();
        let coptic = iso_date.to_calendar(Coptic);
        let recovered_iso = coptic.to_iso();
        assert_eq!(iso_date, recovered_iso);
    }

    #[test]
    fn test_from_fields_monthday_constrain() {
        // M13-7 is not a real day, however this should resolve to M12-6
        // with Overflow::Constrain
        let fields = DateFields {
            month_code: Some(MonthCode("M13".parse().unwrap())),
            day: Some(7),
            ..Default::default()
        };
        let options = DateFromFieldsOptions {
            overflow: Some(Overflow::Constrain),
            missing_fields_strategy: Some(MissingFieldsStrategy::Ecma),
            ..Default::default()
        };

        let date = Date::try_from_fields(fields, options, Coptic).unwrap();
        assert_eq!(date.day_of_month().0, 6, "Day was successfully constrained");
    }
}
