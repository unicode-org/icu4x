// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::{
    ops::{Div, Mul},
    str::FromStr,
};

use fraction::GenericFraction;
use icu_provider::DataError;
use icu_unitsconversion::provider::{ConstantType, Sign};
use num_bigint::BigUint;

/// Removes all whitespace from a string.
pub fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

// TODO: move this to the comment above.
#[test]
fn test_remove_whitespace() {
    let input = "He  llo Wo rld!";
    let expected = "HelloWorld!";
    let actual = remove_whitespace(input);
    assert_eq!(expected, actual);
}

/// Converts a scientific notation number represented as a string into a GenericFraction.
/// Examples:
/// - "1E2" is converted to 100
/// - "1E-2" is converted to 1/100
/// - "1.5E2" is converted to 150
/// - "1.5E-2" is converted to 15/1000
/// - "1.5E-2.5" is an invalid scientific notation number
pub fn convert_scientific_notation_to_fraction(
    number: &str,
) -> Result<GenericFraction<BigUint>, DataError> {
    let number = remove_whitespace(number);
    let parts: Vec<&str> = number.split('E').collect();
    if parts.len() > 2 {
        return Err(DataError::custom(
            "the number is not a scientific notation number",
        ));
    }
    let base = parts.get(0).unwrap_or(&"0");
    let exponent = parts.get(1).unwrap_or(&"0");
    let base: GenericFraction<BigUint> = GenericFraction::from_str(base)
        .map_err(|_| DataError::custom("the number is not a valid number"))?;
    let exponent = i64::from_str(exponent)
        .map_err(|_| DataError::custom("the exponent is not a valid number"))?;

    let mut result = base;
    let generic_ten: GenericFraction<BigUint> =
        GenericFraction::new(BigUint::from(10u32), BigUint::from(1u32));
    if exponent > 0 {
        for _ in 0..exponent as u32 {
            result = result.mul(generic_ten.clone());
        }
    } else {
        for _ in 0..(-exponent) as u32 {
            result = result.div(generic_ten.clone());
        }
    }

    Ok(result)
}

// TODO: move this to the comment above.
#[test]
fn test_convert_scientific_notation_to_fraction() {
    let input = "1E2";
    let expected = GenericFraction::new(BigUint::from(100u32), BigUint::from(1u32));
    let actual = convert_scientific_notation_to_fraction(input).unwrap();
    assert_eq!(expected, actual);

    let input = "1E-2";
    let expected = GenericFraction::new(BigUint::from(1u32), BigUint::from(100u32));
    let actual = convert_scientific_notation_to_fraction(input).unwrap();
    assert_eq!(expected, actual);

    let input = "1.5E2";
    let expected = GenericFraction::new(BigUint::from(150u32), BigUint::from(1u32));
    let actual = convert_scientific_notation_to_fraction(input).unwrap();
    assert_eq!(expected, actual);

    let input = "1.5E-2";
    let expected = GenericFraction::new(BigUint::from(15u32), BigUint::from(1000u32));
    let actual = convert_scientific_notation_to_fraction(input).unwrap();
    assert_eq!(expected, actual);

    let input = "1.5E-2.5";
    let actual = convert_scientific_notation_to_fraction(input);
    assert!(actual.is_err());
}


/// Determines if a string contains any alphabetic characters.
/// Returns true if the string contains at least one alphabetic character, false otherwise.
pub fn contains_alphabetic_chars(s: &str) -> bool {
    s.chars().any(char::is_alphabetic)
}

/// Checks if a string is a valid scientific notation number.
/// Returns true if the string is a valid scientific notation number, false otherwise.  
pub fn is_scientific_number(s: &str) -> bool {
    let parts: Vec<&str> = s.split('E').collect();
    if parts.len() > 2 {
        return false;
    }

    let base = parts.get(0).unwrap_or(&"0");
    let exponent = parts.get(1).unwrap_or(&"0");

    !contains_alphabetic_chars(base) && !contains_alphabetic_chars(exponent)
}

/// Transforms a fractional number into a constant value.
pub fn transform_fraction_to_constant_value(
    fraction: GenericFraction<BigUint>,
    constant_type: ConstantType,
) -> Result<(Vec<u8>, Vec<u8>, Sign, ConstantType), DataError> {
    let numerator = match fraction.numer() {
        Some(numerator) => numerator.to_bytes_le(),
        None => return Err(DataError::custom("the numerator is too large")),
    };

    let denominator = match fraction.denom() {
        Some(denominator) => denominator.to_bytes_le(),
        None => return Err(DataError::custom("the denominator is too large")),
    };

    let sign = match fraction.sign() {
        Some(sign) => match sign {
            fraction::Sign::Plus => Sign::Positive,
            fraction::Sign::Minus => Sign::Negative,
        },
        None => return Err(DataError::custom("the sign is not valid")),
    };

    Ok((numerator, denominator, sign, constant_type))
}

/// Converts an array of strings of numerator or denominator to fraction.
pub fn convert_array_of_strings_to_fraction(
    num: &Vec<String>,
    den: &Vec<String>,
) -> Result<GenericFraction<BigUint>, DataError> {
    let mut result = GenericFraction::new(BigUint::from(1u32), BigUint::from(1u32));

    for vnum in num.iter() {
        let num = match convert_scientific_notation_to_fraction(vnum) {
            Ok(num) => num,
            Err(e) => return Err(e),
        };
        result = result.mul(num);
    }

    for vden in den.iter() {
        let den = match convert_scientific_notation_to_fraction(vden) {
            Ok(den) => den,
            Err(e) => return Err(e),
        };
        result = result.div(den);
    }

    Ok(result)
}

/// Splits a constant string into a tuple of (numerator, denominator).
/// The numerator and denominator are represented as arrays of strings.
/// Examples:
/// - "1/2" is split into (["1"], ["2"])
/// - "1 * 2 / 3 * ft_to_m" is split into (["1", "2"], ["3" , "ft_to_m"])
/// - "/2" is split into (["1"], ["2"])
/// - "2" is split into (["2"], ["1"])
pub fn split_constant_string(
    constant_string: &str,
) -> Result<(Vec<String>, Vec<String>), DataError> {
    let cleaned_string = remove_whitespace(constant_string);
    let mut numerator = Vec::<String>::new();
    let mut denominator = Vec::<String>::new();

    let mut split = cleaned_string.split('/');
    if split.clone().count() > 2 {
        return Err(DataError::custom("Invalid constant string"));
    }

    let numerator_string = split.next().unwrap_or("1");
    let denominator_string = split.next().unwrap_or("1");

    let numerator_values = if numerator_string.is_empty() {
        vec!["1".to_string()]
    } else {
        numerator_string.split('*').map(|s| s.to_string()).collect()
    };

    let denominator_values = if denominator_string.is_empty() {
        vec!["1".to_string()]
    } else {
        denominator_string.split('*').map(|s| s.to_string()).collect()
    };

    numerator.extend(numerator_values);
    denominator.extend(denominator_values);

    Ok((numerator, denominator))
}
// TODO: move this to the comment above.
#[test]
fn test_split_constant_string() {
    let input = "1/2";
    let expected = (vec!["1".to_string()], vec!["2".to_string()]);
    let actual = split_constant_string(input).unwrap();
    assert_eq!(expected, actual);

    let input = "1 * 2 / 3 * ft_to_m";
    let expected = (
        vec!["1".to_string(), "2".to_string()],
        vec!["3".to_string(), "ft_to_m".to_string()],
    );
    let actual = split_constant_string(input).unwrap();
    assert_eq!(expected, actual);

    let input = "/2";
    let expected = (vec!["1".to_string()], vec!["2".to_string()]);
    let actual = split_constant_string(input).unwrap();
    assert_eq!(expected, actual);

    let input = "2";
    let expected = (vec!["2".to_string()], vec!["1".to_string()]);
    let actual = split_constant_string(input).unwrap();
    assert_eq!(expected, actual);
}

