mod fixtures;
mod helpers;

use std::convert::{Into, TryInto};

use icu_num_util::FixedDecimal;
use icu_pluralrules::PluralOperands;

#[test]
fn test_parsing_operands() {
    let path = "./tests/fixtures/operands.json";
    let test_set: fixtures::OperandsTestSet =
        helpers::read_fixture(path).expect("Failed to read a fixture");

    for test in test_set.string {
        let operands: PluralOperands = test.input.parse().expect("Failed to parse to operands.");
        assert_eq!(operands, test.output.into());
    }

    for test in test_set.int {
        let operands: PluralOperands = test.input.try_into().expect("Failed to parse to operands.");
        assert_eq!(operands, test.output.clone().into());

        if test.input.is_positive() {
            let x = test.input as usize;
            let operands: PluralOperands = x.into();
            assert_eq!(operands, test.output.into());
        }
    }

    for test in test_set.floats {
        let t = test.clone();
        let operands: PluralOperands = t.output.try_into().expect("Failed to parse to operands.");
        let expected: f64 = t.input.abs();
        let actual = (operands.n() - expected).abs();
        assert!(
            actual < f64::EPSILON,
            "actual: {}, for test: {:?}",
            actual,
            &test
        );
    }
}

#[test]
fn test_parsing_operand_errors() {
    let operands: Result<PluralOperands, _> = "".parse();
    assert!(operands.is_err());
}

#[test]
fn test_from_fixed_decimals() {
    #[derive(Debug)]
    struct TestCase {
        input: FixedDecimal,
        expected: PluralOperands,
    };
    let tests = vec![
        TestCase {
            input: FixedDecimal::from(2500).multiplied_pow10(-2).unwrap(),
            expected: PluralOperands {
                n: 25.0,
                i: 25,
                v: 2,
                w: 0,
                f: 0,
                t: 0,
            },
        },
        TestCase {
            input: FixedDecimal::from(2500),
            expected: PluralOperands {
                n: 2500.0,
                i: 2500,
                v: 0,
                w: 0,
                f: 0,
                t: 0,
            },
        },
        TestCase {
            input: FixedDecimal::from(-123450).multiplied_pow10(-4).unwrap(),
            expected: PluralOperands {
                n: 12.345,
                i: 12,
                v: 4,
                w: 3,
                f: 3450,
                t: 345,
            },
        },
        TestCase {
            input: FixedDecimal::from(123450).multiplied_pow10(-4).unwrap(),
            expected: PluralOperands {
                n: 12.345,
                i: 12,
                v: 4,
                w: 3,
                f: 3450,
                t: 345,
            },
        },
        TestCase {
            input: FixedDecimal::from(1).multiplied_pow10(-2).unwrap(),
            expected: PluralOperands {
                n: 0.01,
                i: 0,
                v: 2,
                w: 2,
                f: 1,
                t: 1,
            },
        },
        TestCase {
            input: FixedDecimal::from(123450).multiplied_pow10(-7).unwrap(),
            expected: PluralOperands {
                n: 0.012345,
                i: 0,
                v: 7,
                w: 6,
                f: 123450,
                t: 12345,
            },
        },
    ];
    for test in tests {
        let actual: PluralOperands = (&test.input).into();
        assert_eq!(
            test.expected, actual,
            "\n\t(expected==left; actual==right)\n\t\t{:?}",
            &test
        );
    }

    assert!(true);
}
