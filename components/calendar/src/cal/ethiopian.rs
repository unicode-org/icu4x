// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains types and implementations for the Ethiopian calendar.
//!
//! ```rust
//! use icu::calendar::{cal::Ethiopian, Date};
//!
//! let date_iso = Date::try_new_iso(1970, 1, 2)
//!     .expect("Failed to initialize ISO Date instance.");
//! let date_ethiopian = Date::new_from_iso(date_iso, Ethiopian::new());
//!
//! assert_eq!(date_ethiopian.year().era_year_or_extended(), 1962);
//! assert_eq!(date_ethiopian.month().ordinal, 4);
//! assert_eq!(date_ethiopian.day_of_month().0, 22);
//! ```

use crate::cal::iso::Iso;
use crate::calendar_arithmetic::{ArithmeticDate, CalendarArithmetic};
use crate::error::{year_check, DateError};
use crate::{types, Calendar, Date, DateDuration, DateDurationUnit, RangeError};
use calendrical_calculations::helpers::I32CastError;
use calendrical_calculations::rata_die::RataDie;
use tinystr::tinystr;

/// The number of years the Amete Alem epoch precedes the Amete Mihret epoch
const INCARNATION_OFFSET: i32 = 5500;

/// Which era style the ethiopian calendar uses
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[non_exhaustive]
pub enum EthiopianEraStyle {
    /// Use the Anno Mundi era, anchored at the date of Creation, followed by the
    /// Incarnation era, anchored at the date of the Incarnation of Jesus
    AmeteMihret,
    /// Use the single Anno Mundi era, anchored at the date of Creation
    AmeteAlem,
}

/// The [Ethiopian Calendar]
///
/// The [Ethiopian calendar] is a solar calendar used by the Coptic Orthodox Church, with twelve normal months
/// and a thirteenth small epagomenal month.
///
/// This type can be used with [`Date`] to represent dates in this calendar.
///
/// It can be constructed in two modes: using the Amete Alem era scheme, or the Amete Mihret era scheme (the default),
/// see [`EthiopianEraStyle`] for more info.
///
/// [Ethiopian calendar]: https://en.wikipedia.org/wiki/Ethiopian_calendar
///
/// # Era codes
///
/// This calendar always uses the `ethioaa` era (aliases `mundi`, `ethiopic-amete-alem`), where 1 Anno Mundi is 5493 BCE.
/// Dates before that use negative year numbers.
/// In the Amete Mihret scheme it uses the additional `ethiopic` era (alias `incar`), 1 Incarnation is 9 CE.
///
/// # Month codes
///
/// This calendar supports 13 solar month codes (`"M01" - "M13"`), with `"M13"` being used for the short epagomenal month
/// at the end of the year.
// The bool specifies whether dates should be in the Amete Alem era scheme
#[derive(Copy, Clone, Debug, Hash, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct Ethiopian(pub(crate) bool);

/// The inner date type used for representing [`Date`]s of [`Ethiopian`]. See [`Date`] and [`Ethiopian`] for more details.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct EthiopianDateInner(ArithmeticDate<Ethiopian>);

impl CalendarArithmetic for Ethiopian {
    type YearInfo = ();

    fn month_days(year: i32, month: u8, _data: ()) -> u8 {
        if (1..=12).contains(&month) {
            30
        } else if month == 13 {
            if Self::is_leap_year(year, ()) {
                6
            } else {
                5
            }
        } else {
            0
        }
    }

    fn months_for_every_year(_: i32, _data: ()) -> u8 {
        13
    }

    fn is_leap_year(year: i32, _data: ()) -> bool {
        year.rem_euclid(4) == 3
    }

    fn last_month_day_in_year(year: i32, _data: ()) -> (u8, u8) {
        if Self::is_leap_year(year, ()) {
            (13, 6)
        } else {
            (13, 5)
        }
    }

    fn days_in_provided_year(year: i32, _data: ()) -> u16 {
        if Self::is_leap_year(year, ()) {
            366
        } else {
            365
        }
    }
}

impl Calendar for Ethiopian {
    type DateInner = EthiopianDateInner;
    fn date_from_codes(
        &self,
        era: Option<&str>,
        year: i32,
        month_code: types::MonthCode,
        day: u8,
    ) -> Result<Self::DateInner, DateError> {
        let year = match (self.era_style(), era) {
            (EthiopianEraStyle::AmeteMihret, Some("incar" | "ethiopic") | None) => {
                year_check(year, 1..)? + INCARNATION_OFFSET
            }
            (EthiopianEraStyle::AmeteMihret, Some("mundi" | "ethioaa" | "ethiopic-amete-alem")) => {
                year_check(year, ..=INCARNATION_OFFSET)?
            }
            (
                EthiopianEraStyle::AmeteAlem,
                Some("mundi" | "ethioaa" | "ethiopic-amete-alem") | None,
            ) => year,
            (_, Some(_)) => {
                return Err(DateError::UnknownEra);
            }
        };
        ArithmeticDate::new_from_codes(self, year, month_code, day).map(EthiopianDateInner)
    }
    fn date_from_iso(&self, iso: Date<Iso>) -> EthiopianDateInner {
        let fixed_iso = Iso::to_fixed(iso);
        Self::ethiopian_from_fixed(fixed_iso)
    }

    fn date_to_iso(&self, date: &Self::DateInner) -> Date<Iso> {
        let fixed_ethiopian = Ethiopian::fixed_from_ethiopian(date.0);
        Iso::from_fixed(fixed_ethiopian)
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

    fn day_of_week(&self, date: &Self::DateInner) -> types::Weekday {
        Iso.day_of_week(self.date_to_iso(date).inner())
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
        Self::year_as_ethiopian(date.0.year, self.0)
    }

    fn is_in_leap_year(&self, date: &Self::DateInner) -> bool {
        Self::is_leap_year(date.0.year, ())
    }

    fn month(&self, date: &Self::DateInner) -> types::MonthInfo {
        date.0.month()
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
            prev_year: Self::year_as_ethiopian(prev_year, self.0),
            days_in_prev_year: Ethiopian::days_in_year_direct(prev_year),
            next_year: Self::year_as_ethiopian(next_year, self.0),
        }
    }

    fn debug_name(&self) -> &'static str {
        "Ethiopian"
    }

    fn any_calendar_kind(&self) -> Option<crate::AnyCalendarKind> {
        Some(crate::any_calendar::IntoAnyCalendar::kind(self))
    }
}

impl Ethiopian {
    /// Construct a new Ethiopian Calendar for the Amete Mihret era naming scheme
    pub const fn new() -> Self {
        Self(false)
    }
    /// Construct a new Ethiopian Calendar with a value specifying whether or not it is Amete Alem
    pub const fn new_with_era_style(era_style: EthiopianEraStyle) -> Self {
        Self(matches!(era_style, EthiopianEraStyle::AmeteAlem))
    }

    /// Returns whether this has the Amete Alem era
    pub fn era_style(&self) -> EthiopianEraStyle {
        if self.0 {
            EthiopianEraStyle::AmeteAlem
        } else {
            EthiopianEraStyle::AmeteMihret
        }
    }

    fn fixed_from_ethiopian(date: ArithmeticDate<Ethiopian>) -> RataDie {
        // calendrical calculations expects years in the Incarnation era
        calendrical_calculations::ethiopian::fixed_from_ethiopian(
            date.year - INCARNATION_OFFSET,
            date.month,
            date.day,
        )
    }

    fn ethiopian_from_fixed(date: RataDie) -> EthiopianDateInner {
        let (year, month, day) =
            match calendrical_calculations::ethiopian::ethiopian_from_fixed(date) {
                Err(I32CastError::BelowMin) => {
                    return EthiopianDateInner(ArithmeticDate::min_date())
                }
                Err(I32CastError::AboveMax) => {
                    return EthiopianDateInner(ArithmeticDate::max_date())
                }
                Ok(ymd) => ymd,
            };
        // calendrical calculations returns years in the Incarnation era
        EthiopianDateInner(ArithmeticDate::new_unchecked(
            year + INCARNATION_OFFSET,
            month,
            day,
        ))
    }

    fn days_in_year_direct(year: i32) -> u16 {
        if Ethiopian::is_leap_year(year, ()) {
            366
        } else {
            365
        }
    }
    fn year_as_ethiopian(year: i32, amete_alem: bool) -> types::YearInfo {
        if amete_alem || year <= INCARNATION_OFFSET {
            types::YearInfo::new(
                year,
                types::EraYear {
                    standard_era: tinystr!(16, "ethioaa").into(),
                    formatting_era: types::FormattingEra::Index(0, tinystr!(16, "Anno Mundi")),
                    era_year: year,
                    ambiguity: types::YearAmbiguity::CenturyRequired,
                },
            )
        } else {
            types::YearInfo::new(
                year - INCARNATION_OFFSET,
                types::EraYear {
                    standard_era: tinystr!(16, "ethiopic").into(),
                    formatting_era: types::FormattingEra::Index(1, tinystr!(16, "Incarnation")),
                    era_year: year - INCARNATION_OFFSET,
                    ambiguity: types::YearAmbiguity::CenturyRequired,
                },
            )
        }
    }
}

impl Date<Ethiopian> {
    /// Construct new Ethiopian Date.
    ///
    /// ```rust
    /// use icu::calendar::cal::EthiopianEraStyle;
    /// use icu::calendar::Date;
    ///
    /// let date_ethiopian =
    ///     Date::try_new_ethiopian(EthiopianEraStyle::AmeteMihret, 2014, 8, 25)
    ///         .expect("Failed to initialize Ethopic Date instance.");
    ///
    /// assert_eq!(date_ethiopian.year().era_year_or_extended(), 2014);
    /// assert_eq!(date_ethiopian.month().ordinal, 8);
    /// assert_eq!(date_ethiopian.day_of_month().0, 25);
    /// ```
    pub fn try_new_ethiopian(
        era_style: EthiopianEraStyle,
        mut year: i32,
        month: u8,
        day: u8,
    ) -> Result<Date<Ethiopian>, RangeError> {
        if era_style == EthiopianEraStyle::AmeteAlem {
            year -= INCARNATION_OFFSET;
        }
        ArithmeticDate::new_from_ordinals(year, month, day)
            .map(EthiopianDateInner)
            .map(|inner| Date::from_raw(inner, Ethiopian::new_with_era_style(era_style)))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_leap_year() {
        // 11th September 2023 in gregorian is 6/13/2015 in ethiopian
        let iso_date = Date::try_new_iso(2023, 9, 11).unwrap();
        let date_ethiopian = Date::new_from_iso(iso_date, Ethiopian::new());
        assert_eq!(date_ethiopian.year().extended_year, 2015);
        assert_eq!(date_ethiopian.month().ordinal, 13);
        assert_eq!(date_ethiopian.day_of_month().0, 4);
    }

    #[test]
    fn test_epoch() {
        let iso_date = Date::try_new_iso(8, 8, 29).unwrap();
        let date_ethiopian = Date::new_from_iso(iso_date, Ethiopian::new());

        assert_eq!(date_ethiopian.year().standard_era().unwrap().0, "ethiopic");
        assert_eq!(date_ethiopian.year().extended_year, 1);
        assert_eq!(date_ethiopian.month().ordinal, 1);
        assert_eq!(date_ethiopian.day_of_month().0, 1);

        let iso_date = Date::try_new_iso(8, 8, 28).unwrap();
        let date_ethiopian = Date::new_from_iso(iso_date, Ethiopian::new());

        assert_eq!(date_ethiopian.year().standard_era().unwrap().0, "ethioaa");
    }

    #[test]
    fn test_iso_to_ethiopian_conversion_and_back() {
        let iso_date = Date::try_new_iso(1970, 1, 2).unwrap();
        let date_ethiopian = Date::new_from_iso(iso_date, Ethiopian::new());

        assert_eq!(date_ethiopian.year().extended_year, 1962);
        assert_eq!(date_ethiopian.month().ordinal, 4);
        assert_eq!(date_ethiopian.day_of_month().0, 22);

        assert_eq!(
            date_ethiopian.to_iso(),
            Date::try_new_iso(1970, 1, 2).unwrap()
        );
    }

    #[test]
    fn test_iso_to_ethiopian_aa_conversion_and_back() {
        let iso_date = Date::try_new_iso(1970, 1, 2).unwrap();
        let date_ethiopian = Date::new_from_iso(
            iso_date,
            Ethiopian::new_with_era_style(EthiopianEraStyle::AmeteAlem),
        );

        assert_eq!(date_ethiopian.year().extended_year, 7462);
        assert_eq!(date_ethiopian.month().ordinal, 4);
        assert_eq!(date_ethiopian.day_of_month().0, 22);

        assert_eq!(
            date_ethiopian.to_iso(),
            Date::try_new_iso(1970, 1, 2).unwrap()
        );
    }

    #[test]
    fn test_roundtrip_negative() {
        // https://github.com/unicode-org/icu4x/issues/2254
        let iso_date = Date::try_new_iso(-1000, 3, 3).unwrap();
        let ethiopian = iso_date.to_calendar(Ethiopian::new());
        let recovered_iso = ethiopian.to_iso();
        assert_eq!(iso_date, recovered_iso);
    }
}
