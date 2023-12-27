// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::str::FromStr;
use icu_unitsconversion::{
    converter::{ConverterFactory, Convertibility},
};
use num::BigRational;
use std::collections::HashSet;

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
    // let provider = icu_unitsconversion::provider::UnitsInfoV1;

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

    let converter_factory = ConverterFactory::from_payload(
        icu_unitsconversion::provider::Baked::SINGLETON_UNITS_INFO_V1,
    );
    let parser = converter_factory.parser();

    // TODO(#4461): Those units must be parsable.
    let non_parsable_units: HashSet<&str> = [
        "g-force",
        "arc-second",
        "arc-minute",
        "bu-jp",
        "se-jp",
        "liter-per-100-kilometer",
        "mile-per-gallon-imperial",
        "day-person",
        "week-person",
        "pound-force-foot",
        "calorie-it",
        "british-thermal-unit",
        "british-thermal-unit-it",
        "therm-us",
        "pound-force",
        "kilogram-force",
        "kilowatt-hour-per-100-kilometer",
        "shaku-length",
        "shaku-cloth",
        "jo-jp",
        "ri-jp",
        "nautical-mile",
        "mile-scandinavian",
        "100-kilometer",
        "earth-radius",
        "solar-radius",
        "astronomical-unit",
        "light-year",
        "ounce-troy",
        "earth-mass",
        "solar-mass",
        "solar-luminosity",
        "pound-force-per-square-inch",
        "gasoline-energy-density",
        "dessert-spoon",
        "dessert-spoon-imperial",
        "fluid-ounce-imperial",
        "fluid-ounce",
        "cup-jp",
        "cup-metric",
        "pint-metric",
        "pint-imperial",
        "quart-imperial",
        "gallon-imperial",
        "to-jp",
        "month-person",
        "year-person",
        "decade",
    ]
    .iter()
    .cloned()
    .collect();
    for test in tests {
        // TODO(#4461): remove this line after fixing the parser.
        if non_parsable_units.contains(test.input_unit.as_str())
            || non_parsable_units.contains(test.output_unit.as_str())
        {
            continue;
        }
        let input_unit = parser
            .try_from_identifier(test.input_unit.as_str())
            .unwrap();
        let output_unit = parser
            .try_from_identifier(test.output_unit.as_str())
            .unwrap();

        println!("input_unit identifier: {:?}", test.input_unit);
        println!("input_unit: {:?}", input_unit);
        println!("output_unit identifier: {:?}", test.output_unit);
        println!("output_unit: {:?}", output_unit);

        let convertablity = converter_factory
            .extract_convertibility(&input_unit, &output_unit)
            .unwrap();

        match convertablity {
            Convertibility::Convertible | Convertibility::Reciprocal => (),
            _ => panic!("Units are not convertible or reciprocal"),
        }
    }
}
