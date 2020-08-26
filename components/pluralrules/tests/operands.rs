mod fixtures;
mod helpers;

use std::convert::TryInto;

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
}

#[test]
fn test_parsing_operand_errors() {
    let operands: Result<PluralOperands, _> = "".parse();
    assert!(operands.is_err());
}
