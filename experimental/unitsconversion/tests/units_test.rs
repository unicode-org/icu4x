// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::str::FromStr;
use icu_unitsconversion::measureunit::MeasureUnitParser;
use num::BigRational;

use zerotrie::ZeroTrieSimpleAscii;

/// Convert a decimal number to a BigRational.
fn convert_decimal_to_rational(decimal: &str) -> Option<BigRational> {
    let mut components = decimal.split('.');
    let integer_component = components.next().unwrap_or("0");
    let decimal_component = components.next().unwrap_or("0");
    let decimal_length = decimal_component.chars().count() as i32;

    if components.next().is_some() {
        return None;
    }

    let mut integer_component = BigRational::from_str(integer_component).unwrap();
    let decimal_component = BigRational::from_str(decimal_component).unwrap();

    let ten = BigRational::from_integer(10.into());
    let decimal_component = decimal_component / ten.pow(decimal_length);
    integer_component += decimal_component;
    Some(integer_component)
}

/// Convert a scientific notation string to a BigRational.
pub fn get_rational(rational: &str) -> Option<BigRational> {
    // remove all the commas
    let rational = rational.replace(',', "");

    let mut parts = rational.split('E');
    let rational_part = parts.next().unwrap_or("1");
    let exponent_part = parts.next().unwrap_or("0");
    if parts.next().is_some() {
        return None;
    }

    let mut rational_part = convert_decimal_to_rational(rational_part)?;
    let exponent_part = i32::from_str(exponent_part).unwrap();

    let ten = BigRational::from_integer(10.into());
    let exponent_part = ten.pow(exponent_part);
    rational_part *= exponent_part;
    Some(rational_part)
}

#[test]
fn test_conversion() {
    /// Represents a test case for units conversion.
    #[derive(Debug)]
    struct UnitsTest {
        _category: String,
        input_unit: String,
        output_unit: String,
        _result: BigRational,
    }

    let data = std::fs::read_to_string("tests/data/unitsTest.txt").unwrap();
    let tests: Vec<UnitsTest> = data
        .lines()
        .filter(|line| !line.starts_with('#') && !line.is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split(';').map(|s| s.trim()).collect();
            UnitsTest {
                _category: parts[0].to_string(),
                input_unit: parts[1].to_string(),
                output_unit: parts[2].to_string(),
                _result: get_rational(parts[4]).unwrap(),
            }
        })
        .collect();

    // TODO: how to convert from `&ZeroTrie<ZeroVec<'_, u8>>` to &ZeroTrieSimpleAscii<Vec<u8>>?
    let store = icu_unitsconversion::provider::Baked::SINGLETON_UNITS_INFO_V1
        .units_conversion_trie
        .clone() // cheap since store is a borrowed ZeroVec
        .take_store();

    let payload = ZeroTrieSimpleAscii::from_store(store);
    let parser = MeasureUnitParser::from_payload(&payload);

    for test in tests {
        let input_unit = parser.try_from_identifier(test.input_unit.as_str());
        let output_unit = parser.try_from_identifier(test.output_unit.as_str());

        assert!(input_unit.is_ok());
        assert!(output_unit.is_ok());
    }
}

#[test]
fn test_units_not_parsable() {
    let unparsable_units = [
        "garbage-unit-dafdsafdsafdsaf",
        "meter-per-second-",
        "meter-per-second-per-second",
        "-meter-per-second-per-second",
        "kilo-squared-meter",
    ];

    for unit in unparsable_units.iter() {
        let store = icu_unitsconversion::provider::Baked::SINGLETON_UNITS_INFO_V1
            .units_conversion_trie
            .clone() // cheap since store is a borrowed ZeroVec
            .take_store();

        let payload = ZeroTrieSimpleAscii::from_store(store);
        let parser = MeasureUnitParser::from_payload(&payload);

        let result = parser.try_from_identifier(unit);
        assert!(result.is_err());
    }
}
