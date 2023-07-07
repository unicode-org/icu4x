// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::ops::RangeInclusive;
use fixed_decimal::FixedDecimal;
use fixed_decimal::Sign;
use writeable::Writeable;

#[test]
pub fn test_ecma402_table() {
    // Source: <https://tc39.es/ecma402/#table-intl-rounding-modes>
    #[allow(clippy::type_complexity)] // best way to make it render like a table
    let cases: [(
        &'static str,
        fn(&mut FixedDecimal, i16),
        i32,
        i32,
        i32,
        i32,
        i32,
    ); 9] = [
        ("ceil", FixedDecimal::ceil, -1, 1, 1, 1, 2),
        ("floor", FixedDecimal::floor, -2, 0, 0, 0, 1),
        ("expand", FixedDecimal::expand, -2, 1, 1, 1, 2),
        ("trunc", FixedDecimal::trunc, -1, 0, 0, 0, 1),
        ("half_ceil", FixedDecimal::half_ceil, -1, 0, 1, 1, 2),
        ("half_floor", FixedDecimal::half_floor, -2, 0, 0, 1, 1),
        ("half_expand", FixedDecimal::half_expand, -2, 0, 1, 1, 2),
        ("half_trunc", FixedDecimal::half_trunc, -1, 0, 0, 1, 1),
        ("half_even", FixedDecimal::half_even, -2, 0, 0, 1, 2),
    ];
    for (rounding_mode, f, e1, e2, e3, e4, e5) in cases.into_iter() {
        let mut fd1: FixedDecimal = "-1.5".parse().unwrap();
        let mut fd2: FixedDecimal = "0.4".parse().unwrap();
        let mut fd3: FixedDecimal = "0.5".parse().unwrap();
        let mut fd4: FixedDecimal = "0.6".parse().unwrap();
        let mut fd5: FixedDecimal = "1.5".parse().unwrap();
        f(&mut fd1, 0);
        f(&mut fd2, 0);
        f(&mut fd3, 0);
        f(&mut fd4, 0);
        f(&mut fd5, 0);
        assert_eq!(
            fd1.write_to_string(),
            e1.write_to_string(),
            "-1.5 failed for {rounding_mode}"
        );
        assert_eq!(
            fd2.write_to_string(),
            e2.write_to_string(),
            "0.4 failed for {rounding_mode}"
        );
        assert_eq!(
            fd3.write_to_string(),
            e3.write_to_string(),
            "0.5 failed for {rounding_mode}"
        );
        assert_eq!(
            fd4.write_to_string(),
            e4.write_to_string(),
            "0.6 failed for {rounding_mode}"
        );
        assert_eq!(
            fd5.write_to_string(),
            e5.write_to_string(),
            "1.5 failed for {rounding_mode}"
        );
    }
}

#[test]
pub fn test_within_ranges() {
    struct TestCase {
        rounding_mode: &'static str,
        f: fn(&mut FixedDecimal, i16),
        range_n2000: RangeInclusive<i32>,
        range_n1000: RangeInclusive<i32>,
        range_0: RangeInclusive<i32>,
        range_1000: RangeInclusive<i32>,
        range_2000: RangeInclusive<i32>,
    }
    let cases: [TestCase; 9] = [
        TestCase {
            rounding_mode: "ceil",
            f: FixedDecimal::ceil,
            range_n2000: -2999..=-2000,
            range_n1000: -1999..=-1000,
            range_0: -999..=0,
            range_1000: 1..=1000,
            range_2000: 1001..=2000,
        },
        TestCase {
            rounding_mode: "floor",
            f: FixedDecimal::floor,
            range_n2000: -2000..=-1001,
            range_n1000: -1000..=-1,
            range_0: 0..=999,
            range_1000: 1000..=1999,
            range_2000: 2000..=2999,
        },
        TestCase {
            rounding_mode: "expand",
            f: FixedDecimal::expand,
            range_n2000: -2000..=-1001,
            range_n1000: -1000..=-1,
            range_0: 0..=0,
            range_1000: 1..=1000,
            range_2000: 1001..=2000,
        },
        TestCase {
            rounding_mode: "trunc",
            f: FixedDecimal::trunc,
            range_n2000: -2999..=-2000,
            range_n1000: -1999..=-1000,
            range_0: -999..=999,
            range_1000: 1000..=1999,
            range_2000: 2000..=2999,
        },
        TestCase {
            rounding_mode: "half_ceil",
            f: FixedDecimal::half_ceil,
            range_n2000: -2500..=-1501,
            range_n1000: -1500..=-501,
            range_0: -500..=449,
            range_1000: 500..=1449,
            range_2000: 1500..=2449,
        },
        TestCase {
            rounding_mode: "half_floor",
            f: FixedDecimal::half_floor,
            range_n2000: -2449..=-1500,
            range_n1000: -1449..=-500,
            range_0: -449..=500,
            range_1000: 501..=1500,
            range_2000: 1501..=2500,
        },
        TestCase {
            rounding_mode: "half_expand",
            f: FixedDecimal::half_expand,
            range_n2000: -2449..=-1500,
            range_n1000: -1449..=-500,
            range_0: -449..=449,
            range_1000: 500..=1449,
            range_2000: 1500..=2449,
        },
        TestCase {
            rounding_mode: "half_trunc",
            f: FixedDecimal::half_trunc,
            range_n2000: -2500..=-1501,
            range_n1000: -1500..=-501,
            range_0: -500..=500,
            range_1000: 501..=1500,
            range_2000: 1501..=2500,
        },
        TestCase {
            rounding_mode: "half_even",
            f: FixedDecimal::half_even,
            range_n2000: -2500..=-1500,
            range_n1000: -1449..=-501,
            range_0: -500..=500,
            range_1000: 501..=1449,
            range_2000: 1500..=2500,
        },
    ];
    for TestCase {
        rounding_mode,
        f,
        range_n2000,
        range_n1000,
        range_0,
        range_1000,
        range_2000,
    } in cases
    {
        for n in range_n2000 {
            let mut fd = FixedDecimal::from(n);
            f(&mut fd, 3);
            assert_eq!(fd.write_to_string(), "-2000", "{rounding_mode}: {n}");
            let mut fd = FixedDecimal::from(n - 1000000).multiplied_pow10(-5);
            f(&mut fd, -2);
            assert_eq!(
                fd.write_to_string(),
                "-10.02",
                "{rounding_mode}: {n} ÷ 10^5 ± 10"
            );
        }
        for n in range_n1000 {
            let mut fd = FixedDecimal::from(n);
            f(&mut fd, 3);
            assert_eq!(fd.write_to_string(), "-1000", "{rounding_mode}: {n}");
            let mut fd = FixedDecimal::from(n - 1000000).multiplied_pow10(-5);
            f(&mut fd, -2);
            assert_eq!(
                fd.write_to_string(),
                "-10.01",
                "{rounding_mode}: {n} ÷ 10^5 ± 10"
            );
        }
        for n in range_0 {
            let mut fd = FixedDecimal::from(n);
            f(&mut fd, 3);
            fd.set_sign(Sign::None); // get rid of -0
            assert_eq!(fd.write_to_string(), "000", "{rounding_mode}: {n}");
            let (mut fd, expected) = if n < 0 {
                (
                    FixedDecimal::from(n - 1000000).multiplied_pow10(-5),
                    "-10.00",
                )
            } else {
                (
                    FixedDecimal::from(n + 1000000).multiplied_pow10(-5),
                    "10.00",
                )
            };
            f(&mut fd, -2);
            assert_eq!(
                fd.write_to_string(),
                expected,
                "{rounding_mode}: {n} ÷ 10^5 ± 10"
            );
        }
        for n in range_1000 {
            let mut fd = FixedDecimal::from(n);
            f(&mut fd, 3);
            assert_eq!(fd.write_to_string(), "1000", "{rounding_mode}: {n}");
            let mut fd = FixedDecimal::from(n + 1000000).multiplied_pow10(-5);
            f(&mut fd, -2);
            assert_eq!(
                fd.write_to_string(),
                "10.01",
                "{rounding_mode}: {n} ÷ 10^5 ± 10"
            );
        }
        for n in range_2000 {
            let mut fd = FixedDecimal::from(n);
            f(&mut fd, 3);
            assert_eq!(fd.write_to_string(), "2000", "{rounding_mode}: {n}");
            let mut fd = FixedDecimal::from(n + 1000000).multiplied_pow10(-5);
            f(&mut fd, -2);
            assert_eq!(
                fd.write_to_string(),
                "10.02",
                "{rounding_mode}: {n} ÷ 10^5 ± 10"
            );
        }
    }
}

#[test]
pub fn extra_rounding_mode_cases() {
    struct TestCase {
        input: &'static str,
        position: i16,
        // ceil, floor, expand, trunc, half_ceil, half_floor, half_expand, half_trunc, half_even
        all_expected: [&'static str; 9],
    }
    let cases: [TestCase; 8] = [
        TestCase {
            input: "505.050",
            position: -3,
            all_expected: [
                "505.050", "505.050", "505.050", "505.050", "505.050", "505.050", "505.050",
                "505.050", "505.050",
            ],
        },
        TestCase {
            input: "505.050",
            position: -2,
            all_expected: [
                "505.05", "505.05", "505.05", "505.05", "505.05", "505.05", "505.05", "505.05",
                "505.05",
            ],
        },
        TestCase {
            input: "505.050",
            position: -1,
            all_expected: [
                "505.1", "505.0", "505.1", "505.0", "505.1", "505.0", "505.1", "505.0", "505.0",
            ],
        },
        TestCase {
            input: "505.050",
            position: 0,
            all_expected: [
                "506", "505", "506", "505", "505", "505", "505", "505", "505",
            ],
        },
        TestCase {
            input: "505.050",
            position: 1,
            all_expected: [
                "510", "500", "510", "500", "510", "510", "510", "510", "510",
            ],
        },
        TestCase {
            input: "505.050",
            position: 2,
            all_expected: [
                "600", "500", "600", "500", "500", "500", "500", "500", "500",
            ],
        },
        TestCase {
            input: "505.050",
            position: 3,
            all_expected: [
                "1000", "000", "1000", "000", "1000", "1000", "1000", "1000", "1000",
            ],
        },
        TestCase {
            input: "505.050",
            position: 4,
            all_expected: [
                "10000", "0000", "10000", "0000", "0000", "0000", "0000", "0000", "0000",
            ],
        },
    ];
    #[allow(clippy::type_complexity)] // most compact representation in code
    let rounding_modes: [(&'static str, fn(&mut FixedDecimal, i16)); 9] = [
        ("ceil", FixedDecimal::ceil),
        ("floor", FixedDecimal::floor),
        ("expand", FixedDecimal::expand),
        ("trunc", FixedDecimal::trunc),
        ("half_ceil", FixedDecimal::half_ceil),
        ("half_floor", FixedDecimal::half_floor),
        ("half_expand", FixedDecimal::half_expand),
        ("half_trunc", FixedDecimal::half_trunc),
        ("half_even", FixedDecimal::half_even),
    ];
    for TestCase {
        input,
        position,
        all_expected,
    } in cases
    {
        for ((rounding_mode, f), expected) in rounding_modes.iter().zip(all_expected.iter()) {
            let mut fd: FixedDecimal = input.parse().unwrap();
            f(&mut fd, position);
            assert_eq!(
                &*fd.write_to_string(),
                *expected,
                "{input}: {rounding_mode} @ {position}"
            )
        }
    }
}
