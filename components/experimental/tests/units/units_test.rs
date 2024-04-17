// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::str::FromStr;

use icu_experimental::units::converter_factory::ConverterFactory;
use icu_experimental::units::ratio::IcuRatio;
use num_bigint::BigInt;
use num_rational::Ratio;
use num_traits::Pow;

// TODO: add this function to IcuRatio.
/// Convert a decimal number to a BigRational.
fn convert_decimal_to_rational(decimal: &str) -> Option<IcuRatio> {
    let mut components = decimal.split('.');
    let integer_component = components.next().unwrap_or("0");
    let decimal_component = components.next().unwrap_or("0");
    let decimal_length = decimal_component.chars().count() as i32;

    if components.next().is_some() {
        return None;
    }

    let mut integer_component = IcuRatio::from_str(integer_component).unwrap();
    let decimal_component = IcuRatio::from_str(decimal_component).unwrap();

    let ten = IcuRatio::ten();
    let decimal_component = decimal_component / ten.pow(decimal_length);
    integer_component += decimal_component;
    Some(integer_component)
}

/// Convert a scientific notation string to a IcuRatio.
pub fn get_rational(rational: &str) -> Option<IcuRatio> {
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

    let ten = IcuRatio::ten();
    let exponent_part = ten.pow(exponent_part);
    rational_part *= exponent_part;
    Some(rational_part)
}

#[test]
fn test_cldr_unit_tests() {
    /// Represents a test case for units conversion.
    #[derive(Debug)]
    struct UnitsTest {
        category: String,
        input_unit: String,
        output_unit: String,
        result: IcuRatio,
    }

    let data = std::fs::read_to_string("tests/units/data/unitsTest.txt").unwrap();
    let tests: Vec<UnitsTest> = data
        .lines()
        .filter(|line| !line.starts_with('#') && !line.is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split(';').map(|s| s.trim()).collect();
            UnitsTest {
                category: parts[0].to_string(),
                input_unit: parts[1].to_string(),
                output_unit: parts[2].to_string(),
                result: get_rational(parts[4]).unwrap(),
            }
        })
        .collect();

    let converter_factory = ConverterFactory::new();
    let parser = converter_factory.parser();

    for test in tests {
        let input_unit = parser
            .try_from_identifier(test.input_unit.as_str())
            .unwrap();
        let output_unit = parser
            .try_from_identifier(test.output_unit.as_str())
            .unwrap();

        let converter = converter_factory
            .converter(&input_unit, &output_unit)
            .unwrap();
        let result = converter.convert(&IcuRatio::from(1000));
        let result_f64 = converter.convert_f64(1000.0);
        // TODO: remove this extra clones by implementing Sub<&IcuRatio> & Div<&IcuRatio> for IcuRatio.
        let diff_ratio = (result.clone() - test.result.clone()).abs() / test.result.clone();
        let diff_ratio_f64 = ((&test.result - result_f64).unwrap().abs() / &test.result).unwrap();

        if diff_ratio > IcuRatio::from(Ratio::new(BigInt::from(1), BigInt::from(1000000))) {
            panic!(
                "Failed test: Category: {:?}, Input Unit: {:?}, Output Unit: {:?}, Result: {:?}, Expected Result: {:?}",
                test.category, test.input_unit, test.output_unit, result, test.result
            );
        }

        if diff_ratio_f64 > 0.000001 {
            panic!(
                "Failed test: Category: {:?}, Input Unit: {:?}, Output Unit: {:?}, Result: {:?}, Expected Result: {:?}",
                test.category, test.input_unit, test.output_unit, result_f64, test.result
            );
        }
    }

    // TODO: add more test cases for the NonConvertible units.
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

    let converter_factory = ConverterFactory::new();
    let parser = converter_factory.parser();

    for unit in unparsable_units.iter() {
        let result = parser.try_from_identifier(unit);
        assert!(result.is_err());
    }
}
