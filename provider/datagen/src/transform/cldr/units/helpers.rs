// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::BTreeMap;

use icu_provider::DataError;
use icu_unitsconversion::{
    helpers::gcd,
    provider::{ConstantType, ConstantValue},
    Error,
};

/// Removes all whitespace from a string.
pub fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

/// Split a string into a vector of strings, using specific characters as delimiters.
pub fn split_string(s: &str, delimiters: Vec<&str>) -> Vec<String> {
    let mut result = vec![];
    let mut current = String::new();
    for c in s.chars() {
        if delimiters.contains(&c.to_string().as_str()) {
            result.push(current.clone());
            current.clear();
        } else {
            current.push(c);
        }
    }
    result.push(current);
    result
}

/// Checks if a string represents a decimal number.
pub fn check_if_decimal_number(number: &str) -> bool {
    if number.chars().next().unwrap_or('0') == '-' || number.chars().next().unwrap_or('0') == '+' {
        return check_if_decimal_number(&number[1..]);
    }

    let mut split = number.split('.');
    if split.clone().count() > 2 {
        return false;
    }
    let whole = split.next().unwrap_or("0");
    let fractional = split.next().unwrap_or("0");
    if whole.chars().any(|c| !c.is_ascii_digit()) {
        return false;
    }
    if fractional.chars().any(|c| !c.is_ascii_digit()) {
        return false;
    }
    true
}

/// Converts a rational number represented as a string into a tuple of (numerator, denominator).
pub fn to_fractional(number: &str) -> Result<(u64, u64), DataError> {
    if !check_if_decimal_number(number) {
        return Err(DataError::custom("the number is not a decimal number"));
    }

    let mut split = number.split('.');
    let whole = split.next().unwrap_or("0");
    let fractional = split.next().unwrap_or("0");
    let mut denominator = 1;
    for _ in 0..fractional.len() {
        denominator *= 10;
    }
    let numerator = whole.to_string() + fractional;
    let numerator = numerator.parse::<u64>().unwrap_or(0);
    let gcd = gcd(numerator, denominator);
    Ok((numerator / gcd, denominator / gcd))
}

/// Checks if a string represents a scientific notation number.
pub fn check_if_scientific_notation_number(number: &str) -> bool {
    let mut split = number.split('E');
    if split.clone().count() > 2 {
        return false;
    }

    let base = split.next().unwrap_or("0");
    let exponent = split.next().unwrap_or("0");
    check_if_decimal_number(base) && check_if_decimal_number(exponent)
}

/// Converts a scientific notation number represented as a string into a tuple of (numerator, denominator).
pub fn convert_scientific_notation_number_to_fractional(
    number: &str,
) -> Result<(u64, u64), DataError> {
    if !check_if_scientific_notation_number(number) {
        return Err(DataError::custom(
            "the number is not a scientific notation number",
        ));
    }

    let mut split = number.split('E');
    let base = split.next().unwrap_or("0");
    let exponent = split.next().unwrap_or("0");

    let (mut numerator, mut denominator) = to_fractional(base).unwrap();
    let exponent = exponent.parse::<i32>().unwrap();
    if exponent > 0 {
        for _ in 0..exponent {
            numerator *= 10;
        }
    } else {
        for _ in 0..(-exponent) {
            denominator *= 10;
        }
    }
    let gcd = gcd(numerator, denominator);
    Ok((numerator / gcd, denominator / gcd))
}

#[test]
fn test_remove_whitespace() {
    let input = "He  llo Wo rld!";
    let expected = "HelloWorld!";
    let actual = remove_whitespace(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_split_string() {
    let input = "Hello,World!/in/ICU4X";
    let expected = vec![
        "Hello".to_string(),
        "World!".to_string(),
        "in".to_string(),
        "ICU4X".to_string(),
    ];
    let actual = split_string(input, vec![",", "/"]);
    assert_eq!(expected, actual);

    let input = "ft3_to_m3/12*12*12";
    let expected = vec![
        "ft3_to_m3".to_string(),
        "12".to_string(),
        "12".to_string(),
        "12".to_string(),
    ];
    let actual = split_string(input, vec!["/", "*"]);
    assert_eq!(expected, actual);
}

#[test]
fn test_gcd() {
    let input = (12, 8);
    let expected = 4;
    let actual = gcd(input.0, input.1);
    assert_eq!(expected, actual);

    let input = (8, 12);
    let expected = 4;
    let actual = gcd(input.0, input.1);
    assert_eq!(expected, actual);

    let input = (12, 0);
    let expected = 12;
    let actual = gcd(input.0, input.1);
    assert_eq!(expected, actual);

    let input = (0, 12);
    let expected = 12;
    let actual = gcd(input.0, input.1);
    assert_eq!(expected, actual);
}

/// Converts a string representing a constant value into a tuple of (numerator, denominator).
/// For example: input = "6.67408E-11", output will be = (41713, 625000000000000)
///              input = "ft_to_m", output will be error.
pub fn convert_constant_value_in_scientific_to_fractional(
    input: &str,
    constant_type: ConstantType,
) -> Result<ConstantValue, DataError> {
    if !check_if_scientific_notation_number(input) {
        return Err(DataError::custom(
            "the number is not a scientific notation number",
        ));
    }

    let (numerator, denominator) = match convert_scientific_notation_number_to_fractional(input) {
        Ok((numerator, denominator)) => (numerator, denominator),
        Err(e) => return Err(e),
    };

    let gcd = gcd(numerator, denominator);

    let numerator = match u32::try_from(numerator / gcd) {
        Ok(numerator) => numerator,
        Err(_) => return Err(DataError::custom("the numerator is too large")),
    };

    let denominator = match u32::try_from(denominator / gcd) {
        Ok(denominator) => denominator,
        Err(_) => return Err(DataError::custom("the denominator is too large")),
    };

    Ok(ConstantValue {
        numerator,
        denominator,
        constant_type,
    })
}

pub fn convert_any_constant_value_to_fractional(
    constant_str: &str,
    constants_map: &BTreeMap<&str, ConstantValue>,
    constant_type: ConstantType,
) -> Result<ConstantValue, DataError> {
    let constant_string_cleaned = remove_whitespace(constant_str);
    let fraction_str = split_string(constant_string_cleaned.as_str(), vec!["/"]);
    let numerator_strs = split_string(&fraction_str[0].to_string(), vec!["*"]);
    let denominator_strs = split_string(&fraction_str[1].to_string(), vec!["*"]);

    let mut result = ConstantValue {
        numerator: 1,
        denominator: 1,
        constant_type,
    };

    for numerator_str in numerator_strs {
        let numerator = match constants_map.get(numerator_str.as_str()) {
            Some(numerator) => *numerator,
            None => match convert_constant_value_in_scientific_to_fractional(
                numerator_str.as_str(),
                constant_type,
            ) {
                Ok(numerator) => numerator,
                Err(e) => return Err(e),
            },
        };
        result = match result.multiply(&numerator) {
            Ok(result) => result,
            Err(e) => {
                return match e {
                    Error::Limit => Err(DataError::custom("calculations exceeded the limit")),
                    _ => Err(DataError::custom("the numerator is too large")),
                }
            }
        };
    }

    for denominator_str in denominator_strs {
        let denominator = match constants_map.get(denominator_str.as_str()) {
            Some(denominator) => *denominator,
            None => match convert_constant_value_in_scientific_to_fractional(
                denominator_str.as_str(),
                constant_type,
            ) {
                Ok(denominator) => denominator,
                Err(e) => return Err(e),
            },
        };
        result = match result.divide(&denominator) {
            Ok(result) => result,
            Err(e) => {
                return match e {
                    Error::Limit => Err(DataError::custom("calculations exceeded the limit")),
                    _ => Err(DataError::custom("the denominator is too large")),
                }
            }
        };
    }

    Ok(result)
}

#[test]
fn test_to_fractional() {
    let input = "1.5";
    let expected = (3, 2);
    let actual = to_fractional(input);
    assert_eq!(expected, actual.unwrap());

    let input = "1.25";
    let expected = (5, 4);
    let actual = to_fractional(input);
    assert_eq!(expected, actual.unwrap());

    let input = "1.125";
    let expected = (9, 8);
    let actual = to_fractional(input);
    assert_eq!(expected, actual.unwrap());

    let input = "1.0625";
    let expected = (17, 16);
    let actual = to_fractional(input);
    assert_eq!(expected, actual.unwrap());

    let input = "0.000003333";
    let expected = (3333, 1000000000);
    let actual = to_fractional(input);
    assert_eq!(expected, actual.unwrap());

    let input = "11111";
    let expected = (11111, 1);
    let actual = to_fractional(input);
    assert_eq!(expected, actual.unwrap());

    let input = "1,000,000.5";
    let actual = to_fractional(input);
    assert!(actual.is_err());

    let input = "1.5.5";
    let actual = to_fractional(input);
    assert!(actual.is_err());
}

#[test]
fn test_convert_scientific_notation_number_to_fractional() {
    let input = "1.5E1";
    let expected = (15, 1);
    let actual = convert_scientific_notation_number_to_fractional(input);
    assert_eq!(expected, actual.unwrap());

    let input = "1.5E-1";
    let expected = (3, 20);
    let actual = convert_scientific_notation_number_to_fractional(input);
    assert_eq!(expected, actual.unwrap());

    let input = "1.5E-2";
    let expected = (3, 200);
    let actual = convert_scientific_notation_number_to_fractional(input);
    assert_eq!(expected, actual.unwrap());

    let input = "6.67408E-11";
    let expected = (41713, 625000000000000);
    let actual = convert_scientific_notation_number_to_fractional(input);
    assert_eq!(expected, actual.unwrap());
}
