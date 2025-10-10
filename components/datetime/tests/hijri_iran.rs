// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_calendar::cal::hijri::HijriYearData;
use icu_calendar::cal::hijri::Rules;
use icu_calendar::cal::hijri::TabularAlgorithm;
use icu_calendar::cal::hijri::TabularAlgorithmEpoch;
use icu_calendar::cal::hijri::TabularAlgorithmLeapYears;
use icu_calendar::cal::hijri::UmmAlQura;
use icu_calendar::cal::Hijri;
use icu_calendar::types::RataDie;
use icu_calendar::Date;
use icu_datetime::fieldsets;
use icu_datetime::scaffold::FormattableHijriRules;
use icu_datetime::FixedCalendarDateTimeFormatter;

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

impl icu_calendar::cal::scaffold::UnstableSealed for IranTestSighting {}
impl Rules for IranTestSighting {
    fn year_data(&self, extended_year: i32) -> HijriYearData {
        let s = false;
        let l = true;
        use calendrical_calculations::gregorian::fixed_from_gregorian as gregorian;
        match extended_year {
            1411 => HijriYearData::new(
                extended_year,
                gregorian(1990, 7, 24),
                [l, l, s, l, s, l, s, l, s, l, s, s],
            )
            .unwrap(),
            1412 => HijriYearData::new(
                extended_year,
                gregorian(1991, 7, 13),
                [l, l, s, l, s, l, s, l, s, l, l, s],
            )
            .unwrap(),
            1413 => HijriYearData::new(
                extended_year,
                gregorian(1992, 7, 2),
                [l, s, l, s, s, l, s, l, s, l, l, l],
            )
            .unwrap(),
            1414 => HijriYearData::new(
                extended_year,
                gregorian(1993, 6, 22),
                [s, l, s, s, l, s, l, s, s, l, l, l],
            )
            .unwrap(),
            1415 => HijriYearData::new(
                extended_year,
                gregorian(1994, 6, 11),
                [l, l, s, s, s, l, s, s, s, l, l, l],
            )
            .unwrap(),
            1416 => HijriYearData::new(
                extended_year,
                gregorian(1995, 5, 31),
                [l, l, s, l, s, s, l, s, s, l, l, s],
            )
            .unwrap(),
            1417 => HijriYearData::new(
                extended_year,
                gregorian(1996, 5, 19),
                [l, l, l, s, s, l, s, l, s, l, s, s],
            )
            .unwrap(),
            1418 => HijriYearData::new(
                extended_year,
                gregorian(1997, 5, 8),
                [l, l, s, l, l, s, l, s, s, l, l, s],
            )
            .unwrap(),
            _ => TabularAlgorithm::new(
                TabularAlgorithmLeapYears::TypeII,
                TabularAlgorithmEpoch::Friday,
            )
            .year_data(extended_year),
        }
    }
}

// Use the same display names as for UAQ
impl icu_datetime::scaffold::UnstableSealed for IranTestSighting {}
impl FormattableHijriRules for IranTestSighting {
    type MonthNamesV1 = <UmmAlQura as FormattableHijriRules>::MonthNamesV1;
    type YearNamesV1 = <UmmAlQura as FormattableHijriRules>::YearNamesV1;
    type SkeletaV1 = <UmmAlQura as FormattableHijriRules>::SkeletaV1;
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

#[test]
fn test_format() {
    let formatter =
        FixedCalendarDateTimeFormatter::try_new(Default::default(), fieldsets::YMD::long())
            .unwrap();

    assert_eq!(
        formatter
            .format(
                &Date::try_new_gregorian(2022, 10, 12)
                    .unwrap()
                    .to_calendar(Hijri(IranTestSighting))
            )
            .to_string(),
        "AH 1444 Rabiʻ I 16"
    );
}
