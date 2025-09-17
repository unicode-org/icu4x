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
//! assert_eq!(date_ethiopian.era_year().year, 1962);
//! assert_eq!(date_ethiopian.month().ordinal, 4);
//! assert_eq!(date_ethiopian.day_of_month().0, 24);
//! ```

use crate::cal::coptic::{Coptic, CopticDateInner};
use crate::cal::iso::IsoDateInner;
use crate::calendar_arithmetic::ArithmeticDate;
use crate::error::{range_check, DateError};
use crate::{types, Calendar, Date, DateDuration, DateDurationUnit, RangeError};
use calendrical_calculations::rata_die::RataDie;
use tinystr::tinystr;

/// The number of years the Amete Alem epoch precedes the Amete Mihret epoch
const INCARNATION_OFFSET: i32 = 5500;

/// The number of years the Amete Alem epoch precedes the Coptic epoch
const COPTIC_OFFSET: i32 = 5776;

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
/// This calendar always uses the `aa` era, where 1 Amete Alem is 5493 BCE. Dates before this era use negative years.
/// Dates before that use negative year numbers.
/// In the Amete Mihret scheme it uses the additional `am` era, 1 Amete Mihret is 9 CE.
///
/// # Month codes
///
/// This calendar supports 13 solar month codes (`"M01" - "M13"`), with `"M13"` being used for the short epagomenal month
/// at the end of the year.
// The bool specifies whether dates should be in the Amete Alem era scheme
#[derive(Copy, Clone, Debug, Hash, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct Ethiopian(pub(crate) bool);

impl crate::cal::scaffold::UnstableSealed for Ethiopian {}
impl Calendar for Ethiopian {
    type DateInner = CopticDateInner;
    type Year = types::EraYear;
    fn from_codes(
        &self,
        era: Option<&str>,
        year: i32,
        month_code: types::MonthCode,
        day: u8,
    ) -> Result<Self::DateInner, DateError> {
        let year = match (self.era_style(), era) {
            (EthiopianEraStyle::AmeteMihret, Some("am")) => {
                range_check(year, "year", 1..)? + INCARNATION_OFFSET
            }
            (EthiopianEraStyle::AmeteMihret, None) => year + INCARNATION_OFFSET,
            (EthiopianEraStyle::AmeteMihret, Some("aa")) => {
                range_check(year, "year", ..=INCARNATION_OFFSET)?
            }
            (EthiopianEraStyle::AmeteAlem, Some("aa") | None) => year,
            (_, Some(_)) => {
                return Err(DateError::UnknownEra);
            }
        };
        ArithmeticDate::new_from_codes(self, year - COPTIC_OFFSET, month_code, day)
            .map(CopticDateInner)
    }

    fn from_rata_die(&self, rd: RataDie) -> Self::DateInner {
        Coptic.from_rata_die(rd)
    }

    fn to_rata_die(&self, date: &Self::DateInner) -> RataDie {
        Coptic.to_rata_die(date)
    }

    fn from_iso(&self, iso: IsoDateInner) -> Self::DateInner {
        Coptic.from_iso(iso)
    }

    fn to_iso(&self, date: &Self::DateInner) -> IsoDateInner {
        Coptic.to_iso(date)
    }

    fn months_in_year(&self, date: &Self::DateInner) -> u8 {
        Coptic.months_in_year(date)
    }

    fn days_in_year(&self, date: &Self::DateInner) -> u16 {
        Coptic.days_in_year(date)
    }

    fn days_in_month(&self, date: &Self::DateInner) -> u8 {
        Coptic.days_in_month(date)
    }

    fn offset_date(&self, date: &mut Self::DateInner, offset: DateDuration<Self>) {
        Coptic.offset_date(date, offset.cast_unit());
    }

    fn until(
        &self,
        date1: &Self::DateInner,
        date2: &Self::DateInner,
        _calendar2: &Self,
        largest_unit: DateDurationUnit,
        smallest_unit: DateDurationUnit,
    ) -> DateDuration<Self> {
        Coptic
            .until(date1, date2, &Coptic, largest_unit, smallest_unit)
            .cast_unit()
    }

    fn year_info(&self, date: &Self::DateInner) -> Self::Year {
        let year = date.0.extended_year() + COPTIC_OFFSET;
        let extended_year = if self.0 {
            year
        } else {
            year - INCARNATION_OFFSET
        };

        if self.0 || extended_year <= 0 {
            types::EraYear {
                era: tinystr!(16, "aa"),
                era_index: Some(0),
                year,
                extended_year,
                ambiguity: types::YearAmbiguity::CenturyRequired,
            }
        } else {
            types::EraYear {
                era: tinystr!(16, "am"),
                era_index: Some(1),
                year: year - INCARNATION_OFFSET,
                extended_year,
                ambiguity: types::YearAmbiguity::CenturyRequired,
            }
        }
    }

    fn is_in_leap_year(&self, date: &Self::DateInner) -> bool {
        Coptic.is_in_leap_year(date)
    }

    fn month(&self, date: &Self::DateInner) -> types::MonthInfo {
        Coptic.month(date)
    }

    fn day_of_month(&self, date: &Self::DateInner) -> types::DayOfMonth {
        Coptic.day_of_month(date)
    }

    fn day_of_year(&self, date: &Self::DateInner) -> types::DayOfYear {
        Coptic.day_of_year(date)
    }

    fn debug_name(&self) -> &'static str {
        "Ethiopian"
    }

    fn calendar_algorithm(&self) -> Option<crate::preferences::CalendarAlgorithm> {
        Some(crate::preferences::CalendarAlgorithm::Ethiopic)
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
    /// assert_eq!(date_ethiopian.era_year().year, 2014);
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
        ArithmeticDate::new_from_ordinals(year - COPTIC_OFFSET, month, day)
            .map(CopticDateInner)
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
        assert_eq!(date_ethiopian.extended_year(), 2015);
        assert_eq!(date_ethiopian.month().ordinal, 13);
        assert_eq!(date_ethiopian.day_of_month().0, 6);
    }

    #[test]
    fn test_iso_to_ethiopian_conversion_and_back() {
        let iso_date = Date::try_new_iso(1970, 1, 2).unwrap();
        let date_ethiopian = Date::new_from_iso(iso_date, Ethiopian::new());

        assert_eq!(date_ethiopian.extended_year(), 1962);
        assert_eq!(date_ethiopian.month().ordinal, 4);
        assert_eq!(date_ethiopian.day_of_month().0, 24);

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

        assert_eq!(date_ethiopian.extended_year(), 7462);
        assert_eq!(date_ethiopian.month().ordinal, 4);
        assert_eq!(date_ethiopian.day_of_month().0, 24);

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

    #[test]
    fn extended_year() {
        assert_eq!(
            Date::new_from_iso(
                Date::try_new_iso(-5500 + 9, 1, 1).unwrap(),
                Ethiopian::new_with_era_style(EthiopianEraStyle::AmeteAlem)
            )
            .extended_year(),
            1
        );
        assert_eq!(
            Date::new_from_iso(
                Date::try_new_iso(9, 1, 1).unwrap(),
                Ethiopian::new_with_era_style(EthiopianEraStyle::AmeteAlem)
            )
            .extended_year(),
            5501
        );

        assert_eq!(
            Date::new_from_iso(
                Date::try_new_iso(-5500 + 9, 1, 1).unwrap(),
                Ethiopian::new_with_era_style(EthiopianEraStyle::AmeteMihret)
            )
            .extended_year(),
            -5499
        );
        assert_eq!(
            Date::new_from_iso(
                Date::try_new_iso(9, 1, 1).unwrap(),
                Ethiopian::new_with_era_style(EthiopianEraStyle::AmeteMihret)
            )
            .extended_year(),
            1
        );
    }
}
