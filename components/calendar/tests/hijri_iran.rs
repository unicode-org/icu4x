// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_calendar::cal::hijri::HijriSighting;
use icu_calendar::cal::hijri::HijriTabularEpoch;
use icu_calendar::cal::hijri::HijriTabularLeapYears;
use icu_calendar::cal::hijri::HijriYearInfo;
use icu_calendar::cal::hijri::TabularAlgorithm;
use icu_calendar::cal::Hijri;
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
    fn year_info(&self, monotonic_year: i32) -> HijriYearInfo {
        let s = false;
        let l = true;
        use calendrical_calculations::iso::fixed_from_iso as iso;
        match monotonic_year {
            1411 => HijriYearInfo {
                monotonic_year,
                month_lengths: [l, l, s, l, s, l, s, l, s, l, s, s],
                start_day: iso(1990, 7, 24),
            },
            1412 => HijriYearInfo {
                monotonic_year,
                month_lengths: [l, l, s, l, s, l, s, l, s, l, l, s],
                start_day: iso(1991, 7, 13),
            },
            1413 => HijriYearInfo {
                monotonic_year,
                month_lengths: [l, s, l, s, s, l, s, l, s, l, l, l],
                start_day: iso(1992, 7, 2),
            },
            1414 => HijriYearInfo {
                monotonic_year,
                month_lengths: [s, l, s, s, l, s, l, s, s, l, l, l],
                start_day: iso(1993, 6, 22),
            },
            1415 => HijriYearInfo {
                monotonic_year,
                month_lengths: [l, l, s, s, s, l, s, s, s, l, l, l],
                start_day: iso(1994, 6, 11),
            },
            1416 => HijriYearInfo {
                monotonic_year,
                month_lengths: [l, l, s, l, s, s, l, s, s, l, l, s],
                start_day: iso(1995, 5, 31),
            },
            1417 => HijriYearInfo {
                monotonic_year,
                month_lengths: [l, l, l, s, s, l, s, l, s, l, s, s],
                start_day: iso(1996, 5, 19),
            },
            1418 => HijriYearInfo {
                monotonic_year,
                month_lengths: [l, l, s, l, l, s, l, s, s, l, l, s],
                start_day: iso(1997, 5, 8),
            },
            _ => TabularAlgorithm::new(HijriTabularLeapYears::TypeII, HijriTabularEpoch::Friday)
                .year_info(monotonic_year),
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
