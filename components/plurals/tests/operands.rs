// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
mod fixtures;
mod helpers;

use std::convert::TryInto;

use fixed_decimal::FixedDecimal;
use icu_plurals::PluralOperands;

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
    let path = "./tests/fixtures/operands.json";
    let test_set: fixtures::OperandsTestSet =
        helpers::read_fixture(path).expect("Failed to read a fixture");
    for test in test_set.from_test {
        let input: FixedDecimal = FixedDecimal::from(&test.input);
        let actual: PluralOperands = PluralOperands::from(&input);
        let expected: PluralOperands = PluralOperands::from(test.expected.clone());
        assert_eq!(
            expected, actual,
            "\n\t(expected==left; actual==right)\n\t\t{:?}",
            &test
        );
    }
}
