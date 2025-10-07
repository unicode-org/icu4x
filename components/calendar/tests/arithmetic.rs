// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::convert::Infallible;

use icu_calendar::{
    cal::Hebrew,
    options::{DateAddOptions, DateUntilOptions, Overflow},
    types::{DateDuration, DateDurationUnit, MonthCode},
    AsCalendar, Calendar, Date, Iso,
};

#[rustfmt::skip]
#[allow(clippy::type_complexity)]
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
    ("2020-06-30", "2020-01-31", 151,  (21, 4),  (4, 29),  (0, 4, 29)), // DIFF +/-
    ("2020-07-30", "2020-03-31", 121,  (17, 2),  (3, 30),  (0, 3, 30)),
    ("2020-07-31", "2020-03-31", 122,  (17, 3),  (4, 0),   (0, 4, 0)),
    ("2020-03-05", "2016-03-20", 1446, (206, 4), (47, 16), (3, 11, 16)), // DIFF +/-
    ("2022-03-01", "2020-02-29", 731,  (104, 3), (24, 1),  (2, 0, 1)),
];

fn check<A>(
    d0: &Date<A>,
    d1: &Date<A>,
    exp0: &u64,
    exp1: &(u32, u64),
    exp2: &(u32, u64),
    exp3: &(u32, u32, u64),
) where
    A: AsCalendar + Copy,
    <A as AsCalendar>::Calendar: Calendar<UntilError = Infallible>,
    <<A as AsCalendar>::Calendar as Calendar>::DateInner: PartialOrd,
{
    let is_negative = d0 > d1;
    let add_options = DateAddOptions::default();
    let mut until_options0 = DateUntilOptions::default();
    until_options0.largest_unit = Some(DateDurationUnit::Days);
    let mut until_options1 = DateUntilOptions::default();
    until_options1.largest_unit = Some(DateDurationUnit::Weeks);
    let mut until_options2 = DateUntilOptions::default();
    until_options2.largest_unit = Some(DateDurationUnit::Months);
    let mut until_options3 = DateUntilOptions::default();
    until_options3.largest_unit = Some(DateDurationUnit::Years);

    let Ok(p0) = d0.until_with_options(d1, until_options0);
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
        *d1,
        "{d0:?}/{d1:?}"
    );

    let Ok(p1) = d0.until_with_options(d1, until_options1);
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
        *d1,
        "{d0:?}/{d1:?}"
    );

    let Ok(p2) = d0.until_with_options(d1, until_options2);
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
        *d1,
        "{d0:?}/{d1:?}"
    );

    let Ok(p3) = d0.until_with_options(d1, until_options3);
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
        *d1,
        "{d0:?}/{d1:?}"
    );
}

#[test]
fn test_arithmetic_cases() {
    for (d0, d1, exp0, exp1, exp2, exp3) in ISO_DATE_PAIRS {
        let d0 = Date::try_from_str(d0, Iso).unwrap();
        let d1 = Date::try_from_str(d1, Iso).unwrap();
        check(&d0, &d1, exp0, exp1, exp2, exp3);
    }
}

#[test]
fn test_hebrew() {
    let m06z_20 = Date::try_new_hebrew(5783, 6, 20).unwrap();
    let m05l_15 = Date::try_new_hebrew(5784, 6, 15).unwrap();
    let m05l_30 = Date::try_new_hebrew(5784, 6, 30).unwrap();
    let m06a_29 = Date::try_new_hebrew(5784, 7, 29).unwrap();
    let m07a_10 = Date::try_new_hebrew(5784, 8, 10).unwrap();
    let m06b_15 = Date::try_new_hebrew(5785, 6, 15).unwrap();
    let m07b_20 = Date::try_new_hebrew(5785, 7, 20).unwrap();

    #[rustfmt::skip]
    #[allow(clippy::type_complexity)]
    let cases: &[(&Date<Hebrew>, &Date<Hebrew>, u64, (u32, u64), (u32, u64), (u32, u32, u64))] = &[
        (&m06z_20, &m05l_15, 348, (49, 5), (11, 25), (0, 11, 25)),
        // (&m06z_20, &m05l_30, 363, (51, 6), (12, 10), (0, 12, 10)), // TODO
        // (&m06z_20, &m06a_29, 392, (56, 0), (13, 9),  (1, 0, 9)), // TODO
        // (&m06z_20, &m07a_10, 402, (57, 3), (13, 19), (1, 0, 19)), // TODO
        // (&m06z_20, &m06b_15, 733, (104,5), (24, 25), (1, 11, 25)), // TODO
        (&m06z_20, &m07b_20, 767, (109,4), (26, 0),  (2, 1, 0)),

        (&m05l_15, &m05l_30, 15,  (2, 1),  (0, 15),  (0, 0, 15)),
        (&m05l_15, &m06a_29, 44,  (6, 2),  (1, 14),  (0, 1, 14)),
        (&m05l_15, &m07a_10, 54,  (7, 5),  (1, 24),  (0, 1, 24)),
        (&m05l_15, &m06b_15, 385, (55, 0), (13, 0),  (1, 0, 0)), // M05L to M06 common year
        (&m05l_15, &m07b_20, 419, (59, 6), (14, 5),  (1, 1, 5)), // M05L to M07 common year

        (&m05l_30, &m06a_29, 29,  (4, 1),  (0, 29),  (0, 0, 29)),
        (&m05l_30, &m07a_10, 39,  (5, 4),  (1, 10),  (0, 1, 10)),
        (&m05l_30, &m06b_15, 370, (52, 6), (12, 15), (0, 12, 15)), // M05L to M06 common year
        (&m05l_30, &m07b_20, 404, (57, 5), (13, 20), (1, 0, 20)), // M05L to M07 common year

        (&m06a_29, &m07a_10, 10,  (1, 3),  (0, 10),  (0, 0, 10)),
        (&m06a_29, &m06b_15, 341, (48, 5), (11, 16), (0, 11, 16)), // M06 leap year to M06 common year
        // (&m06a_29, &m07b_20, 375, (53, 4), (12, 20), (1, 0, 20)), // M06 leap year to M07 common year // TODO

        (&m07a_10, &m06b_15, 331, (47, 2), (11, 5),  (0, 11, 5)), // M07 leap year to M06 common year
        // (&m07a_10, &m07b_20, 365, (52, 1), (12, 10), (1, 0, 10)), // M07 leap year to M06 common year // TODO

        (&m06b_15, &m07b_20, 34,  (4, 6), (1, 5),   (0, 1, 5)),
    ];

    for (d0, d1, exp0, exp1, exp2, exp3) in cases {
        check(d0, d1, exp0, exp1, exp2, exp3);
    }
}

#[test]
fn test_tricky_leap_months() {
    let mut add_options = DateAddOptions::default();
    add_options.overflow = Some(Overflow::Constrain);
    let mut until_options = DateUntilOptions::default();
    until_options.largest_unit = Some(DateDurationUnit::Years);

    fn hebrew_date(year: i32, month: &str, day: u8) -> Date<Hebrew> {
        Date::try_new_from_codes(None, year, MonthCode(month.parse().unwrap()), day, Hebrew).unwrap()
    }

    // M06 + 1yr = M06 (common to leap)
    let date0 = hebrew_date(5783, "M06", 20);
    let duration0 = DateDuration::for_years(1);
    let date1 = date0.added_with_options(duration0, add_options).unwrap();
    assert_eq!(date1, hebrew_date(5784, "M06", 20));
    let duration0_actual = date0.until_with_options(&date1, until_options).unwrap();
    assert_eq!(duration0_actual, duration0);

    // M06 - 1mo = M05L (leap to leap)
    let duration1 = DateDuration::for_months(-1);
    let date2 = date1.added_with_options(duration1, add_options).unwrap();
    assert_eq!(date2, hebrew_date(5784, "M05L", 20));
    let duration1_actual = date1.until_with_options(&date2, until_options).unwrap();
    assert_eq!(duration1_actual, duration1);

    // M05L + 1yr1mo = M07 (leap to common)
    let mut duration2 = DateDuration::default();
    duration2.years = 1;
    duration2.months = 1;
    let date3 = date2.added_with_options(duration2, add_options).unwrap();
    assert_eq!(date3, hebrew_date(5785, "M07", 20));
    let duration2_actual = date2.until_with_options(&date3, until_options).unwrap();
    assert_eq!(duration2_actual, duration2);

    // M06 + 1yr1mo = M07 (leap to common)
    let date4 = date1.added_with_options(duration2, add_options).unwrap();
    assert_eq!(date4, hebrew_date(5785, "M07", 20));
    let duration2_actual = date1.until_with_options(&date4, until_options).unwrap();
    assert_eq!(duration2_actual, duration2);
}
