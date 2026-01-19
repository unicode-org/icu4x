// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashMap;
use std::sync::LazyLock;

use icu::calendar::cal::{hijri, Hijri};
use icu::calendar::Date;

#[derive(Clone, Copy, Debug)]
struct IranSighting;

impl icu_calendar::cal::scaffold::UnstableSealed for IranSighting {}
impl hijri::Rules for IranSighting {
    fn year(&self, extended_year: i32) -> hijri::HijriYear {
        QAMARI.get(&extended_year).copied().unwrap_or_else(|| {
            hijri::TabularAlgorithm::new(
                hijri::TabularAlgorithmLeapYears::TypeII,
                hijri::TabularAlgorithmEpoch::Friday,
            )
            .year(extended_year)
        })
    }
}

static QAMARI: LazyLock<HashMap<i32, hijri::HijriYear>> = LazyLock::new(|| {
    let response = ureq::get(
            "https://raw.githubusercontent.com/roozbehp/qamari/575d275ef169c0a18012506d473ba1008a1c10d0/consolidated.txt",
        )
        .call()
        .unwrap();

    use std::io::{BufRead, BufReader};

    let mut parsed: HashMap<i32, HashMap<u8, _>> = HashMap::new();

    for line in BufReader::new(response.into_body().into_reader())
        .lines()
        .map_while(Result::ok)
    {
        let line = line
            .split('#')
            .next()
            .unwrap()
            .split('*')
            .next_back()
            .unwrap()
            .trim();

        let (ym, greg) = line.split_once(' ').unwrap();

        let mut greg = greg.split('-');
        let start = Date::try_new_gregorian(
            greg.next().unwrap().parse().unwrap(),
            greg.next().unwrap().parse().unwrap(),
            greg.next().unwrap().parse().unwrap(),
        )
        .unwrap()
        .to_rata_die();

        let (y, m) = ym.split_once('/').unwrap();

        parsed
            .entry(y.parse().unwrap())
            .or_default()
            .insert(m.parse().unwrap(), start);
    }

    let mut data = HashMap::new();

    for (&y, months) in &parsed {
        let Some(next) = parsed.get(&(y + 1)) else {
            continue;
        };

        data.insert(
            y,
            hijri::HijriYear::try_new(
                y,
                core::array::from_fn(|i| {
                    if i == 12 {
                        next[&1]
                    } else {
                        months[&(i as u8 + 1)]
                    }
                }),
            )
            .unwrap(),
        );
    }

    data
});

// Use the same display names as for TabularAlgorithm
impl icu_datetime::scaffold::UnstableSealed for IranSighting {}
impl icu::datetime::scaffold::FormattableHijriRules for IranSighting {
    type MonthNamesV1 =
        <hijri::TabularAlgorithm as icu::datetime::scaffold::FormattableHijriRules>::MonthNamesV1;
    type YearNamesV1 =
        <hijri::TabularAlgorithm as icu::datetime::scaffold::FormattableHijriRules>::YearNamesV1;
    type SkeletaV1 =
        <hijri::TabularAlgorithm as icu::datetime::scaffold::FormattableHijriRules>::SkeletaV1;
}

#[test]
fn test_conversion() {
    for ((y, m, d), rd) in [
        (
            (1412, 9, 12),
            Date::try_new_gregorian(1992, 3, 17).unwrap().to_rata_die(),
        ),
        (
            (1416, 10, 6),
            Date::try_new_gregorian(1996, 2, 25).unwrap().to_rata_die(),
        ),
    ] {
        let date = Date::try_new_hijri_with_calendar(y, m, d, Hijri(IranSighting)).unwrap();
        let date_rd = Date::from_rata_die(rd, Hijri(IranSighting));

        assert_eq!(date, date_rd, "{:?}", (y, m, d));
        assert_eq!(date.to_rata_die(), rd, "{:?}", (y, m, d));
    }
}

#[test]
fn test_format() {
    use icu::datetime::{fieldsets, FixedCalendarDateTimeFormatter};
    use icu::locale::locale;

    let formatter =
        FixedCalendarDateTimeFormatter::try_new(locale!("fa").into(), fieldsets::YMD::long())
            .unwrap();

    assert_eq!(
        formatter
            .format(
                &Date::try_new_gregorian(2022, 10, 12)
                    .unwrap()
                    .to_calendar(Hijri(IranSighting))
            )
            .to_string(),
        "۱۵ ربیع\u{200c}الاول ۱۴۴۴ ه\u{200d}.ق."
    );
}
