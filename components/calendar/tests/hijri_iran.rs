// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_calendar::cal::{hijri::*, Hijri};
use icu_calendar::types::RataDie;
use icu_calendar::Date;

static TEST_RD: [i64; 4] = [727274, 728714, 744313, 764652];

#[derive(Debug)]
struct DateCase {
    year: i32,
    month: u8,
    day: u8,
}

static HIJRI_IRAN_CASES: [DateCase; 4] = [
    DateCase {
        year: 1412,
        month: 9,
        day: 12,
    },
    DateCase {
        year: 1416,
        month: 10,
        day: 6,
    },
    DateCase {
        year: 1460,
        month: 10,
        day: 12,
    },
    DateCase {
        year: 1518,
        month: 3,
        day: 5,
    },
];

#[derive(Clone, Copy, Debug)]
struct IranTestSighting;

impl HijriSighting for IranTestSighting {
    fn is_month_long(&self, monotonic_year: i32, month: u8) -> bool {
        match monotonic_year {
            1411 => matches!(month, 1 | 2 | 4 | 6 | 8 | 10),
            1412 => matches!(month, 1 | 2 | 4 | 6 | 8 | 10 | 11),
            1413 => matches!(month, 1 | 3 | 6 | 8 | 10 | 11 | 12),
            1414 => matches!(month, 2 | 5 | 7 | 10 | 11 | 12),
            1415 => matches!(month, 1 | 2 | 6 | 10 | 11 | 12),
            1416 => matches!(month, 1 | 2 | 4 | 7 | 10 | 11),
            1417 => matches!(month, 1 | 2 | 3 | 6 | 8 | 10),
            1418 => matches!(month, 1 | 2 | 4 | 5 | 7 | 10 | 11),
            _ => TabularAlgorithm::new(
                TabularAlgorithmLeapYears::TypeII,
                TabularAlgorithmEpoch::Friday,
            )
            .is_month_long(monotonic_year, month),
        }
    }

    fn start_day(&self, monotonic_year: i32) -> RataDie {
        use calendrical_calculations::iso::fixed_from_iso as iso;
        match monotonic_year {
            1411 => iso(1990, 7, 24),
            1412 => iso(1991, 7, 13),
            1413 => iso(1992, 7, 2),
            1414 => iso(1993, 6, 22),
            1415 => iso(1994, 6, 11),
            1416 => iso(1995, 5, 31),
            1417 => iso(1996, 5, 19),
            1418 => iso(1997, 5, 8),
            _ => TabularAlgorithm::new(
                TabularAlgorithmLeapYears::TypeII,
                TabularAlgorithmEpoch::Friday,
            )
            .start_day(monotonic_year),
        }
    }
}

#[test]
fn test_hijri_iran_from_rd() {
    let calendar = Hijri(IranTestSighting);
    for (case, f_date) in HIJRI_IRAN_CASES.iter().zip(TEST_RD.iter()) {
        let date =
            Date::try_new_hijri_with_calendar(case.year, case.month, case.day, calendar).unwrap();
        let date_rd = Date::from_rata_die(RataDie::new(*f_date), calendar);

        assert_eq!(date, date_rd, "{case:?}");
    }
}

#[test]
fn test_rd_from_hijri_iran() {
    let calendar = Hijri(IranTestSighting);
    for (case, f_date) in HIJRI_IRAN_CASES.iter().zip(TEST_RD.iter()) {
        let date =
            Date::try_new_hijri_with_calendar(case.year, case.month, case.day, calendar).unwrap();
        assert_eq!(date.to_rata_die(), RataDie::new(*f_date), "{case:?}");
    }
}
