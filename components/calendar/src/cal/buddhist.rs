// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::preferences::CalendarAlgorithm;
use crate::{
    cal::abstract_gregorian::{impl_with_abstract_gregorian, GregorianYears},
    calendar_arithmetic::ArithmeticDate,
    types, Date, DateError, RangeError,
};
use tinystr::tinystr;

#[derive(Copy, Clone, Debug, Default)]
/// The [Thai Solar Buddhist Calendar][cal]
///
/// The [Thai Solar Buddhist Calendar][cal] is a solar calendar used in Thailand, with twelve months.
/// The months and days are identical to that of the Gregorian calendar, however the years are counted
/// differently using the Buddhist Era.
///
/// This type can be used with [`Date`] to represent dates in this calendar.
///
/// [cal]: https://en.wikipedia.org/wiki/Thai_solar_calendar
///
/// # Era codes
///
/// This calendar uses a single era code `be`, with 1 Buddhist Era being 543 BCE. Dates before this era use negative years.
///
/// # Month codes
///
/// This calendar supports 12 solar month codes (`"M01" - "M12"`)
#[allow(clippy::exhaustive_structs)] // this type is stable
pub struct Buddhist;

impl_with_abstract_gregorian!(
    crate::cal::Buddhist,
    BuddhistDateInner,
    BuddhistEra,
    _x,
    BuddhistEra
);

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct BuddhistEra;

impl GregorianYears for BuddhistEra {
    const EXTENDED_YEAR_OFFSET: i32 = -543;

    fn extended_from_era_year(&self, era: Option<&str>, year: i32) -> Result<i32, DateError> {
        match era {
            Some("be") | None => Ok(year),
            _ => Err(DateError::UnknownEra),
        }
    }

    fn era_year_from_extended(&self, extended_year: i32, _month: u8, _day: u8) -> types::EraYear {
        types::EraYear {
            era: tinystr!(16, "be"),
            era_index: Some(0),
            year: extended_year,
            extended_year,
            ambiguity: types::YearAmbiguity::CenturyRequired,
        }
    }

    fn debug_name(&self) -> &'static str {
        "Buddhist"
    }

    fn calendar_algorithm(&self) -> Option<CalendarAlgorithm> {
        Some(CalendarAlgorithm::Buddhist)
    }
}

impl Date<Buddhist> {
    /// Construct a new Buddhist Date.
    ///
    /// Years are specified as BE years.
    ///
    /// ```rust
    /// use icu::calendar::Date;
    ///
    /// let date_buddhist = Date::try_new_buddhist(1970, 1, 2)
    ///     .expect("Failed to initialize Buddhist Date instance.");
    ///
    /// assert_eq!(date_buddhist.era_year().year, 1970);
    /// assert_eq!(date_buddhist.month().ordinal, 1);
    /// assert_eq!(date_buddhist.day_of_month().0, 2);
    /// ```
    pub fn try_new_buddhist(year: i32, month: u8, day: u8) -> Result<Date<Buddhist>, RangeError> {
        ArithmeticDate::new_gregorian::<BuddhistEra>(year, month, day)
            .map(BuddhistDateInner)
            .map(|i| Date::from_raw(i, Buddhist))
    }
}

#[cfg(test)]
mod test {
    use crate::cal::Iso;
    use calendrical_calculations::rata_die::RataDie;

    use super::*;

    #[test]
    fn test_buddhist_roundtrip_near_rd_zero() {
        for i in -10000..=10000 {
            let rd = RataDie::new(i);
            let iso1 = Date::from_rata_die(rd, Iso);
            let buddhist = iso1.to_calendar(Buddhist);
            let iso2 = buddhist.to_calendar(Iso);
            let result = iso2.to_rata_die();
            assert_eq!(rd, result);
        }
    }

    #[test]
    fn test_buddhist_roundtrip_near_epoch() {
        // Buddhist epoch start RD: -198326
        for i in -208326..=-188326 {
            let rd = RataDie::new(i);
            let iso1 = Date::from_rata_die(rd, Iso);
            let buddhist = iso1.to_calendar(Buddhist);
            let iso2 = buddhist.to_calendar(Iso);
            let result = iso2.to_rata_die();
            assert_eq!(rd, result);
        }
    }

    #[test]
    fn test_buddhist_directionality_near_rd_zero() {
        for i in -100..=100 {
            for j in -100..=100 {
                let iso_i = Date::from_rata_die(RataDie::new(i), Iso);
                let iso_j = Date::from_rata_die(RataDie::new(j), Iso);

                let buddhist_i = Date::new_from_iso(iso_i, Buddhist);
                let buddhist_j = Date::new_from_iso(iso_j, Buddhist);

                assert_eq!(
                    i.cmp(&j),
                    iso_i.cmp(&iso_j),
                    "ISO directionality inconsistent with directionality for i: {i}, j: {j}"
                );

                assert_eq!(
                    i.cmp(&j),
                    buddhist_i.cmp(&buddhist_j),
                    "Buddhist directionality inconsistent with directionality for i: {i}, j: {j}"
                );
            }
        }
    }

    #[test]
    fn test_buddhist_directionality_near_epoch() {
        // Buddhist epoch start RD: -198326
        for i in -198426..=-198226 {
            for j in -198426..=-198226 {
                let iso_i = Date::from_rata_die(RataDie::new(i), Iso);
                let iso_j = Date::from_rata_die(RataDie::new(j), Iso);

                let buddhist_i = Date::new_from_iso(iso_i, Buddhist);
                let buddhist_j = Date::new_from_iso(iso_j, Buddhist);

                assert_eq!(
                    i.cmp(&j),
                    iso_i.cmp(&iso_j),
                    "ISO directionality inconsistent with directionality for i: {i}, j: {j}"
                );

                assert_eq!(
                    i.cmp(&j),
                    buddhist_i.cmp(&buddhist_j),
                    "Buddhist directionality inconsistent with directionality for i: {i}, j: {j}"
                );
            }
        }
    }

    #[derive(Debug)]
    struct TestCase {
        iso_year: i32,
        iso_month: u8,
        iso_day: u8,
        buddhist_year: i32,
        buddhist_month: u8,
        buddhist_day: u8,
    }

    fn check_test_case(case: TestCase) {
        let iso_year = case.iso_year;
        let iso_month = case.iso_month;
        let iso_day = case.iso_day;
        let buddhist_year = case.buddhist_year;
        let buddhist_month = case.buddhist_month;
        let buddhist_day = case.buddhist_day;

        let iso1 = Date::try_new_iso(iso_year, iso_month, iso_day).unwrap();
        let buddhist1 = iso1.to_calendar(Buddhist);
        assert_eq!(
            buddhist1.era_year().year,
            buddhist_year,
            "Iso -> Buddhist year check failed for case: {case:?}"
        );
        assert_eq!(
            buddhist1.month().ordinal,
            buddhist_month,
            "Iso -> Buddhist month check failed for case: {case:?}"
        );
        assert_eq!(
            buddhist1.day_of_month().0,
            buddhist_day,
            "Iso -> Buddhist day check failed for case: {case:?}"
        );

        let buddhist2 =
            Date::try_new_buddhist(buddhist_year, buddhist_month, buddhist_day).unwrap();
        let iso2 = buddhist2.to_calendar(Iso);
        assert_eq!(
            iso2.era_year().year,
            iso_year,
            "Buddhist -> Iso year check failed for case: {case:?}"
        );
        assert_eq!(
            iso2.month().ordinal,
            iso_month,
            "Buddhist -> Iso month check failed for case: {case:?}"
        );
        assert_eq!(
            iso2.day_of_month().0,
            iso_day,
            "Buddhist -> Iso day check failed for case: {case:?}"
        );
    }

    #[test]
    fn test_buddhist_cases_near_rd_zero() {
        let cases = [
            TestCase {
                iso_year: -100,
                iso_month: 2,
                iso_day: 15,
                buddhist_year: 443,
                buddhist_month: 2,
                buddhist_day: 15,
            },
            TestCase {
                iso_year: -3,
                iso_month: 10,
                iso_day: 29,
                buddhist_year: 540,
                buddhist_month: 10,
                buddhist_day: 29,
            },
            TestCase {
                iso_year: 0,
                iso_month: 12,
                iso_day: 31,
                buddhist_year: 543,
                buddhist_month: 12,
                buddhist_day: 31,
            },
            TestCase {
                iso_year: 1,
                iso_month: 1,
                iso_day: 1,
                buddhist_year: 544,
                buddhist_month: 1,
                buddhist_day: 1,
            },
            TestCase {
                iso_year: 4,
                iso_month: 2,
                iso_day: 29,
                buddhist_year: 547,
                buddhist_month: 2,
                buddhist_day: 29,
            },
        ];

        for case in cases {
            check_test_case(case);
        }
    }

    #[test]
    fn test_buddhist_cases_near_epoch() {
        // 1 BE = 543 BCE = -542 ISO
        let cases = [
            TestCase {
                iso_year: -554,
                iso_month: 12,
                iso_day: 31,
                buddhist_year: -11,
                buddhist_month: 12,
                buddhist_day: 31,
            },
            TestCase {
                iso_year: -553,
                iso_month: 1,
                iso_day: 1,
                buddhist_year: -10,
                buddhist_month: 1,
                buddhist_day: 1,
            },
            TestCase {
                iso_year: -544,
                iso_month: 8,
                iso_day: 31,
                buddhist_year: -1,
                buddhist_month: 8,
                buddhist_day: 31,
            },
            TestCase {
                iso_year: -543,
                iso_month: 5,
                iso_day: 12,
                buddhist_year: 0,
                buddhist_month: 5,
                buddhist_day: 12,
            },
            TestCase {
                iso_year: -543,
                iso_month: 12,
                iso_day: 31,
                buddhist_year: 0,
                buddhist_month: 12,
                buddhist_day: 31,
            },
            TestCase {
                iso_year: -542,
                iso_month: 1,
                iso_day: 1,
                buddhist_year: 1,
                buddhist_month: 1,
                buddhist_day: 1,
            },
            TestCase {
                iso_year: -541,
                iso_month: 7,
                iso_day: 9,
                buddhist_year: 2,
                buddhist_month: 7,
                buddhist_day: 9,
            },
        ];

        for case in cases {
            check_test_case(case);
        }
    }
}
