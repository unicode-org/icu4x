// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_calendar::{
    options::{DateAddOptions, DateUntilOptions},
    types::{DateDuration, DateDurationUnit},
    Date, Iso,
};

#[rustfmt::skip]
const ISO_DATE_PAIRS: &[(&str, &str, u64, (u32, u64), (u32, u64), (u32, u32, u64))] = &[
    //         d0,           d1, D,    (W, D),   (M, D),   (Y, M, D)
    ("2020-01-03", "2020-02-15", 43,   (6, 1),   (1, 12),  (0, 1, 12)),
    ("2020-01-31", "2020-06-30", 151,  (21, 4),  (4, 30),  (0, 4, 30)),
    ("2020-03-31", "2020-07-30", 121,  (17, 2),  (3, 30),  (0, 3, 30)),
    ("2020-03-31", "2020-07-31", 122,  (17, 3),  (4, 0),   (0, 4, 0)),
    ("2016-03-20", "2020-03-05", 1446, (206, 4), (47, 14), (3, 11, 14)),
    ("2020-02-29", "2022-03-01", 731,  (104, 3), (24, 1),  (2, 0, 1)),

    // Negative direction:
    ("2020-02-15", "2020-01-03", 43,   (6, 1),   (1, 12),  (0, 1, 12)),
    ("2020-06-30", "2020-01-31", 151,  (21, 4),  (4, 29),  (0, 4, 29)),
    ("2020-07-30", "2020-03-31", 121,  (17, 2),  (3, 30),  (0, 3, 30)),
    ("2020-07-31", "2020-03-31", 122,  (17, 3),  (4, 0),   (0, 4, 0)),
    ("2020-03-05", "2016-03-20", 1446, (206, 4), (47, 16), (3, 11, 16)),
    ("2022-03-01", "2020-02-29", 731,  (104, 3), (24, 1),  (2, 0, 1)),
];

#[test]
fn test_arithmetic_cases() {
    let add_options = DateAddOptions::default();

    let mut until_options0 = DateUntilOptions::default();
    until_options0.largest_unit = Some(DateDurationUnit::Days);

    let mut until_options1 = DateUntilOptions::default();
    until_options1.largest_unit = Some(DateDurationUnit::Weeks);

    let mut until_options2 = DateUntilOptions::default();
    until_options2.largest_unit = Some(DateDurationUnit::Months);

    let mut until_options3 = DateUntilOptions::default();
    until_options3.largest_unit = Some(DateDurationUnit::Years);

    for (d0, d1, exp0, exp1, exp2, exp3) in ISO_DATE_PAIRS {
        let d0 = Date::try_from_str(d0, Iso).unwrap();
        let d1 = Date::try_from_str(d1, Iso).unwrap();
        let is_negative = d0 > d1;

        let Ok(p0) = d0.until_with_options(&d1, until_options0);
        assert_eq!(
            p0,
            DateDuration {
                is_negative,
                days: *exp0,
                ..Default::default()
            },
            "{d0:?}/{d1:?}"
        );
        assert_eq!(
            d0.added_with_options(p0, add_options).unwrap(),
            d1,
            "{d0:?}/{d1:?}"
        );

        let Ok(p1) = d0.until_with_options(&d1, until_options1);
        assert_eq!(
            p1,
            DateDuration {
                is_negative,
                weeks: exp1.0,
                days: exp1.1,
                ..Default::default()
            },
            "{d0:?}/{d1:?}"
        );
        assert_eq!(
            d0.added_with_options(p1, add_options).unwrap(),
            d1,
            "{d0:?}/{d1:?}"
        );

        let Ok(p2) = d0.until_with_options(&d1, until_options2);
        assert_eq!(
            p2,
            DateDuration {
                is_negative,
                months: exp2.0,
                days: exp2.1,
                ..Default::default()
            },
            "{d0:?}/{d1:?}"
        );
        assert_eq!(
            d0.added_with_options(p2, add_options).unwrap(),
            d1,
            "{d0:?}/{d1:?}"
        );

        let Ok(p3) = d0.until_with_options(&d1, until_options3);
        assert_eq!(
            p3,
            DateDuration {
                is_negative,
                years: exp3.0,
                months: exp3.1,
                days: exp3.2,
                ..Default::default()
            },
            "{d0:?}/{d1:?}"
        );
        assert_eq!(
            d0.added_with_options(p3, add_options).unwrap(),
            d1,
            "{d0:?}/{d1:?}"
        );
    }
}
