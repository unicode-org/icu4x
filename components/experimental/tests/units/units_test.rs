// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::str::FromStr;

use icu_experimental::units::converter::UnitsConverter;
use icu_experimental::units::converter_factory::ConverterFactory;
use icu_experimental::units::ratio::IcuRatio;
use num_bigint::BigInt;
use num_rational::Ratio;
use num_traits::{Signed, ToPrimitive};

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
                result: IcuRatio::from_str(parts[4]).unwrap(),
            }
        })
        .collect();

    let converter_factory = ConverterFactory::new();
    let parser = converter_factory.parser();

    for test in tests {
        let input_unit = parser.try_from_bytes(test.input_unit.as_bytes()).unwrap();
        let output_unit = parser.try_from_bytes(test.output_unit.as_bytes()).unwrap();

        let converter: UnitsConverter<Ratio<BigInt>> = converter_factory
            .converter(&input_unit, &output_unit)
            .unwrap();
        let result = converter.convert(&Ratio::<BigInt>::from_str("1000").unwrap());

        let converter_f64: UnitsConverter<f64> = converter_factory
            .converter(&input_unit, &output_unit)
            .unwrap();
        let result_f64 = converter_f64.convert(&1000.0);

        // TODO: remove this extra clones by implementing Sub<&IcuRatio> & Div<&IcuRatio> for IcuRatio.
        let diff_ratio = ((result.clone() - test.result.clone().get_ratio())
            / test.result.clone().get_ratio())
        .abs();
        if diff_ratio > Ratio::new(BigInt::from(1), BigInt::from(1000000)) {
            panic!(
                "Failed test: Category: {:?}, Input Unit: {:?}, Output Unit: {:?}, Result: {:?}, Expected Result: {:?}",
                test.category, test.input_unit, test.output_unit, result, test.result
            );
        }

        let test_result_f64 = test.result.get_ratio().to_f64().unwrap();
        let diff_ratio_f64 = ((test_result_f64 - result_f64) / test_result_f64).abs();

        if diff_ratio_f64 > 0.000001 {
            panic!(
                "Failed test: Category: {:?}, Input Unit: {:?}, Output Unit: {:?}, Result: {:?}, Expected Result: {:?}",
                test.category, test.input_unit, test.output_unit, result_f64, test_result_f64
            );
        }
    }

    // TODO: add more test cases for the NonConvertible units.
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

    let converter_factory = ConverterFactory::new();
    let parser = converter_factory.parser();

    unparsable_units.iter().for_each(|unit| {
        assert!(
            parser.try_from_bytes(unit.as_bytes()).is_err(),
            "Unit '{}' should be unparsable but was parsed successfully.",
            unit
        );
    });
}
