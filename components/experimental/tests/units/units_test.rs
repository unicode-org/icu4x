// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::str::FromStr;

use icu_experimental::units::InvalidConversionError;
use icu_experimental::units::converter_factory::ConverterFactory;
use icu_experimental::units::ratio::IcuRatio;
use icu_experimental::{measure::measureunit::MeasureUnit, units::converter::UnitsConverter};
use num_bigint::BigInt;
use num_rational::Ratio;
use num_traits::{Signed, ToPrimitive};

#[test]
fn test_cldr_unit_tests() {
    #[derive(Debug)]
    struct UnitsTest {
        category: String,
        input_unit: String,
        output_unit: String,
        result: IcuRatio,
    }

    let data = std::fs::read_to_string("tests/units/data/unitsTest.txt")
        .expect("Failed to read test data file");
    let tests: Vec<UnitsTest> = data
        .lines()
        .filter(|line| !line.starts_with('#') && !line.is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split(';').map(str::trim).collect();
            UnitsTest {
                category: parts[0].to_string(),
                input_unit: parts[1].to_string(),
                output_unit: parts[2].to_string(),
                result: IcuRatio::from_str(parts[4]).expect("Failed to parse IcuRatio"),
            }
        })
        .collect();

    let converter_factory = ConverterFactory::new();

    for test in tests {
        let input_unit =
            MeasureUnit::try_from_str(&test.input_unit).expect("Failed to parse input unit");
        let output_unit =
            MeasureUnit::try_from_str(&test.output_unit).expect("Failed to parse output unit");

        let converter: UnitsConverter<&Ratio<BigInt>> = converter_factory
            .converter(&input_unit, &output_unit)
            .expect("Failed to create converter");
        let result =
            converter.convert(&Ratio::<BigInt>::from_str("1000").expect("Failed to parse ratio"));

        let converter_f64: UnitsConverter<f64> = converter_factory
            .converter(&input_unit, &output_unit)
            .expect("Failed to create converter for f64");
        let result_f64 = converter_f64.convert(1000.0);

        let diff_ratio = ((result.clone() - test.result.clone().get_ratio())
            / test.result.clone().get_ratio())
        .abs();
        assert!(
            diff_ratio <= Ratio::new(BigInt::from(1), BigInt::from(1000000)),
            "Failed test: Category: {:?}, Input Unit: {:?}, Output Unit: {:?}, Result: {:?}, Expected Result: {:?}",
            test.category,
            test.input_unit,
            test.output_unit,
            result,
            test.result
        );

        let test_result_f64 = test
            .result
            .get_ratio()
            .to_f64()
            .expect("Failed to convert ratio to f64");
        let diff_ratio_f64 = ((test_result_f64 - result_f64) / test_result_f64).abs();
        assert!(
            diff_ratio_f64 <= 0.000001,
            "Failed test: Category: {:?}, Input Unit: {:?}, Output Unit: {:?}, Result: {:?}, Expected Result: {:?}",
            test.category,
            test.input_unit,
            test.output_unit,
            result_f64,
            test_result_f64
        );
    }
}

#[test]
fn test_units_non_convertible() {
    let non_convertible_units = [
        ("meter", "second"),
        ("kilogram", "liter"),
        ("celsius", "meter"),
        ("ampere", "candela"),
        ("radian", "steradian"),
        ("byte", "meter"),
        ("kilogram", "radian"),
        ("kelvin", "candela"),
        ("second", "radian"),
        ("second", "steradian"),
        ("second", "byte"),
        ("second", "kilogram"),
        ("second", "kelvin"),
        ("second", "ampere"),
        ("second", "candela"),
        ("meter", "radian"),
        ("meter", "steradian"),
        ("meter", "byte"),
        ("meter", "kilogram"),
        ("meter", "kelvin"),
        ("meter", "ampere"),
        ("meter", "candela"),
        ("kilogram", "radian"),
        ("kilogram", "steradian"),
        ("kilogram", "byte"),
        ("kilogram", "kelvin"),
        ("kilogram", "ampere"),
        ("kilogram", "candela"),
        ("kelvin", "radian"),
        ("kelvin", "steradian"),
        ("kelvin", "byte"),
        ("kelvin", "ampere"),
        ("kelvin", "candela"),
        ("ampere", "radian"),
        ("ampere", "steradian"),
        ("ampere", "byte"),
        ("ampere", "kelvin"),
        ("ampere", "candela"),
        ("candela", "radian"),
        ("candela", "steradian"),
        ("candela", "byte"),
        ("candela", "kelvin"),
        ("candela", "ampere"),
        ("radian", "second"),
        ("radian", "meter"),
        ("radian", "kilogram"),
        ("radian", "kelvin"),
        ("radian", "ampere"),
        ("radian", "candela"),
        ("steradian", "second"),
        ("steradian", "meter"),
        ("steradian", "kilogram"),
        ("steradian", "kelvin"),
        ("steradian", "ampere"),
        ("steradian", "candela"),
        ("byte", "second"),
        ("byte", "meter"),
        ("byte", "kilogram"),
        ("byte", "kelvin"),
        ("byte", "ampere"),
        ("byte", "candela"),
        ("square-meter-per-second", "second"),
        ("square-meter-per-second", "kilogram"),
        ("square-meter-per-second", "kelvin"),
        ("square-meter-per-second", "ampere"),
        ("square-meter-per-second", "candela"),
        ("square-meter-per-second", "radian"),
        ("square-meter-per-second", "steradian"),
        ("square-meter-per-second", "byte"),
        ("second", "square-meter-per-second"),
        ("kilogram", "square-meter-per-second"),
        ("kelvin", "square-meter-per-second"),
        ("ampere", "square-meter-per-second"),
        ("candela", "square-meter-per-second"),
        ("radian", "square-meter-per-second"),
        ("steradian", "square-meter-per-second"),
        ("byte", "square-meter-per-second"),
        ("cubic-meter-per-second", "second"),
        ("cubic-meter-per-second", "kilogram"),
        ("cubic-meter-per-second", "kelvin"),
        ("cubic-meter-per-second", "ampere"),
        ("cubic-meter-per-second", "candela"),
        ("cubic-meter-per-second", "radian"),
        ("cubic-meter-per-second", "steradian"),
        ("cubic-meter-per-second", "byte"),
        ("second", "cubic-meter-per-second"),
        ("kilogram", "cubic-meter-per-second"),
        ("kelvin", "cubic-meter-per-second"),
        ("ampere", "cubic-meter-per-second"),
        ("candela", "cubic-meter-per-second"),
        ("radian", "cubic-meter-per-second"),
        ("steradian", "cubic-meter-per-second"),
        ("byte", "cubic-meter-per-second"),
        ("joule-per-second", "second"),
        ("joule-per-second", "kilogram"),
        ("joule-per-second", "kelvin"),
        ("joule-per-second", "ampere"),
        ("joule-per-second", "candela"),
        ("joule-per-second", "radian"),
        ("joule-per-second", "steradian"),
        ("joule-per-second", "byte"),
        ("second", "joule-per-second"),
        ("kilogram", "joule-per-second"),
        ("kelvin", "joule-per-second"),
        ("ampere", "joule-per-second"),
        ("candela", "joule-per-second"),
        ("radian", "joule-per-second"),
        ("steradian", "joule-per-second"),
        ("byte", "joule-per-second"),
        ("cubic-meter-per-second", "joule-per-second"),
        ("joule-per-second", "cubic-meter-per-second"),
        ("meter-per-second", "newton-meter"),
        ("newton-meter", "meter-per-square-second"),
        ("watt-per-square-meter", "pascal-second"),
        ("pascal-second", "watt-per-square-meter"),
        ("volt-per-meter", "ohm-meter"),
        ("ohm-meter", "volt-per-meter"),
        ("tesla-second", "weber-meter"),
        ("weber-meter", "tesla-second"),
    ];

    let converter_factory = ConverterFactory::new();

    for (input, output) in non_convertible_units.iter() {
        let input_unit = MeasureUnit::try_from_str(input).expect("Failed to parse input unit");
        let output_unit = MeasureUnit::try_from_str(output).expect("Failed to parse output unit");

        let result: Result<UnitsConverter<f64>, InvalidConversionError> =
            converter_factory.converter::<f64>(&input_unit, &output_unit);
        assert!(
            result.is_err(),
            "Conversion should not be possible between {input:?} and {output:?}"
        );
    }
}

#[test]
fn test_unparsable_units() {
    let unparsable_units = vec![
        "garbage-unit-dafdsafdsafdsaf",
        "meter-per-second-",
        "meter-per-second-per-second",
        "-meter-per-second-per-second",
        "kilo-squared-meter",
        "metersecond",
        "metersecond-per-second",
        "metersecond-per-second-per-second",
        "-metersecond-per-second-per-second",
        "kilo-squared-meter-second",
        "squarefoot",
        "p4meter",
        "nonexistent-unit",
        "mega--kilogram",
        "giga--meter",
        "invalid-unit-format",
        "unit-without-measurement",
        "meter123",
        "123meter",
        "meter^second",
        "meter/second",
        "meter*second",
        "meter+second",
        "meter@second",
        "meter!second",
        "meter?second",
        "meter.second",
        "meter:second",
        "meter;second",
        "meter[second",
        "meter]second",
        "meter{second",
        "meter}second",
        "meter<second",
        "meter>second",
        "meter,second",
        "meter%second",
        "meter$second",
        "meter#second",
        "meter&second",
        "meter=second",
        "meter_second",
        "meter|second",
        "meter~second",
        "meter`second",
        "meter'second",
        "meter\"second",
        "meter\\second",
        "meter\nsecond",
        "meter\tsecond",
        "meter\rsecond",
        "meter second",
    ];

    unparsable_units.iter().for_each(|unit| {
        assert!(
            MeasureUnit::try_from_str(unit).is_err(),
            "Unit '{unit}' should be unparsable but was parsed successfully."
        );
    });
}

#[test]
fn test_f64_vs_bigint_precision() {
    let factory = ConverterFactory::new();

    // Define specific test cases with their expected exactness: (input, output, value, expect_exact)
    // - Proportional conversions are always exact matches because FMA achieves single rounding.
    // - Offset conversions can be 1 unit of precision off due to double rounding (offset addition + FMA).
    //   However, if they land on exact integers, they remain exact matches.
    let test_cases = [
        // Proportional conversions (always exact)
        ("gram", "tonne", 5.0, true),
        ("gram", "tonne", 0.123456789, true),
        ("gram", "kilogram", 825.0, true),
        ("inch", "centimeter", 5.0, true),
        ("inch", "foot", 84.0, true),
        ("mile-per-hour", "kilometer-per-hour", 120.0, true),
        // Offset conversions
        ("fahrenheit", "celsius", 32.0, true), // 32 F = 0 C (exact)
        ("fahrenheit", "celsius", 212.0, true), // 212 F = 100 C (exact)
        ("fahrenheit", "celsius", -40.0, true), // -40 F = -40 C (exact)
        ("fahrenheit", "celsius", 0.123456789, false), // 1 unit off (verified)
    ];

    for &(input_str, output_str, val, expect_exact) in &test_cases {
        let input_unit = MeasureUnit::try_from_str(input_str).unwrap();
        let output_unit = MeasureUnit::try_from_str(output_str).unwrap();

        let converter_f64 = factory.converter::<f64>(&input_unit, &output_unit).unwrap();
        let converter_bigint = factory
            .converter::<&Ratio<BigInt>>(&input_unit, &output_unit)
            .unwrap();

        // 1. Compute using f64 (FMA)
        let actual = converter_f64.convert(val);

        // 2. Compute using Ratio<BigInt> (exact + single rounding)
        let val_ratio = Ratio::from_float(val).expect("Failed to convert float to Ratio");
        let expected_ratio = converter_bigint.convert(&val_ratio);
        let expected = expected_ratio
            .to_f64()
            .expect("Failed to convert Ratio to float");

        // 3. Assert exact match or minimal difference (closest representable float)
        if expect_exact {
            assert_eq!(
                actual, expected,
                "{input_str} -> {output_str} with value {val}"
            );
        } else {
            let next_down = expected.next_down();
            let next_up = expected.next_up();
            let is_minimal = actual == next_up || actual == next_down;
            assert!(
                is_minimal,
                "Precision mismatch for {input_str} -> {output_str} with value {val}: actual ({actual}) is not the closest representable float to expected ({expected}) [next_down={next_down}, next_up={next_up}]"
            );
        }
    }
}
