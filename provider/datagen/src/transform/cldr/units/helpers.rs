// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::str::FromStr;

use icu_provider::DataError;
use icu_unitsconversion::provider::{ConstantExactness, Sign};
use num_bigint::BigInt;
use num_rational::BigRational;

/// Converts a decimal number represented as a string into a BigRational.
/// Examples:
/// - "1" is converted to 1/1
/// - "1.5" is converted to 15/10
/// - "1.05" is converted to 105/100
/// - "1.005" is converted to 1005/1000
/// - "1.5.5" is an invalid decimal number
/// NOTE:
/// - "1." is not a valid decimal number
/// - BigRational represents a rational number in the simplest form. For example, 15/10 is converted to 3/2.
pub fn convert_decimal_to_bigrational(decimal: &str) -> Result<BigRational, DataError> {
    let parts: Vec<&str> = decimal.split('.').collect();
    match parts.len() {
        1 => BigRational::from_str(parts[0])
            .map_err(|_| DataError::custom("the integer-part is not a valid number")),
        2 => {
            let numerator = BigInt::from_str(parts.join("").as_str()).map_err(|_| {
                DataError::custom("the integer-part and fractional-part are not a valid number")
            })?;
            let denominator = BigInt::from(10u32).pow(parts[1].len() as u32);
            Ok(BigRational::new(numerator, denominator))
        }
        _ => Err(DataError::custom("the base is not a valid number")),
    }
}

#[test]
fn test_convert_decimal_to_bigrational() {
    let input = "1";
    let expected = BigRational::new(BigInt::from(1u32), BigInt::from(1u32));
    let actual = convert_decimal_to_bigrational(input).unwrap();
    assert_eq!(expected, actual);

    let input = "1.5";
    let expected = BigRational::new(BigInt::from(15u32), BigInt::from(10u32));
    let actual = convert_decimal_to_bigrational(input).unwrap();
    assert_eq!(expected, actual);

    let input = "1.05";
    let expected = BigRational::new(BigInt::from(105u32), BigInt::from(100u32));
    let actual = convert_decimal_to_bigrational(input).unwrap();
    assert_eq!(expected, actual);

    let input = "1.005";
    let expected = BigRational::new(BigInt::from(1005u32), BigInt::from(1000u32));
    let actual = convert_decimal_to_bigrational(input).unwrap();
    assert_eq!(expected, actual);

    let input = "1.5.5";
    let actual = convert_decimal_to_bigrational(input);
    assert!(actual.is_err());
}

/// Converts a scientific notation number represented as a string into a GenericFraction.
/// Examples:
/// - "1E2" is converted to 100/1
/// - "1E-2" is converted to 1/100
/// - "1.5E2" is converted to 150/1
/// - "1.5E-2" is converted to 15/1000
/// - " 1.5 E -2 " is converted to 15/1000
/// - " 1.5 E - 2" is an invalid scientific notation number
/// - "1.5E-2.5" is an invalid scientific notation number
pub fn convert_scientific_notation_to_fraction(number: &str) -> Result<BigRational, DataError> {
    let parts: Vec<&str> = number.split('E').collect();
    if parts.len() > 2 {
        return Err(DataError::custom(
            "the number is not a scientific notation number",
        ));
    }
    let base = parts.first().unwrap_or(&"1").trim();
    let exponent = parts.get(1).unwrap_or(&"0").trim();

    let ten = BigRational::from(BigInt::from(10u32));
    let base = convert_decimal_to_bigrational(base)
        .map_err(|_| DataError::custom("the base is not a valid number"))?;
    let exponent = i32::from_str(exponent)
        .map_err(|_| DataError::custom("the exponent is not a valid number"))?;

    Ok(base * ten.pow(exponent))
}

// TODO: move this to the comment above.
#[test]
fn test_convert_scientific_notation_to_fraction() {
    let input = "1E2";
    let expected = BigRational::new(BigInt::from(100u32), BigInt::from(1u32));
    let actual = convert_scientific_notation_to_fraction(input).unwrap();
    assert_eq!(expected, actual);

    let input = "1E-2";
    let expected = BigRational::new(BigInt::from(1u32), BigInt::from(100u32));
    let actual = convert_scientific_notation_to_fraction(input).unwrap();
    assert_eq!(expected, actual);

    let input = "1.5E2";
    let expected = BigRational::new(BigInt::from(150u32), BigInt::from(1u32));
    let actual = convert_scientific_notation_to_fraction(input).unwrap();
    assert_eq!(expected, actual);

    let input = "1.5E-2";
    let expected = BigRational::new(BigInt::from(15u32), BigInt::from(1000u32));
    let actual = convert_scientific_notation_to_fraction(input).unwrap();
    assert_eq!(expected, actual);

    let input = " 1.5 E -2 ";
    let expected = BigRational::new(BigInt::from(15u32), BigInt::from(1000u32));
    let actual = convert_scientific_notation_to_fraction(input).unwrap();
    assert_eq!(expected, actual);

    let input = " 1.5 E - 2";
    let actual = convert_scientific_notation_to_fraction(input);
    assert!(actual.is_err());

    let input = "1.5E-2.5";
    let actual = convert_scientific_notation_to_fraction(input);
    assert!(actual.is_err());
}

/// Determines if a string contains any alphabetic characters.
/// Returns true if the string contains at least one alphabetic character, false otherwise.
/// Examples:
/// - "1" returns false
/// - "ft_to_m" returns true
/// - "1E2" returns true
/// - "1.5E-2" returns true
pub fn contains_alphabetic_chars(s: &str) -> bool {
    s.chars().any(char::is_alphabetic)
}

#[test]
fn test_contains_alphabetic_chars() {
    let input = "1";
    let expected = false;
    let actual = contains_alphabetic_chars(input);
    assert_eq!(expected, actual);

    let input = "ft_to_m";
    let expected = true;
    let actual = contains_alphabetic_chars(input);
    assert_eq!(expected, actual);

    let input = "1E2";
    let expected = true;
    let actual = contains_alphabetic_chars(input);
    assert_eq!(expected, actual);

    let input = "1.5E-2";
    let expected = true;
    let actual = contains_alphabetic_chars(input);
    assert_eq!(expected, actual);
}

/// Checks if a string is a valid scientific notation number.
/// Returns true if the string is a valid scientific notation number, false otherwise.  
pub fn is_scientific_number(s: &str) -> bool {
    let parts: Vec<&str> = s.split('E').collect();
    if parts.len() > 2 {
        return false;
    }

    let base = parts.first().unwrap_or(&"0");
    let exponent = parts.get(1).unwrap_or(&"0");

    !contains_alphabetic_chars(base) && !contains_alphabetic_chars(exponent)
}

/// Transforms a fractional number into a constant value.
pub fn transform_fraction_to_constant_value(
    fraction: BigRational,
    constant_exactness: ConstantExactness,
) -> Result<(Vec<u8>, Vec<u8>, Sign, ConstantExactness), DataError> {
    let numerator = match fraction.numer().to_biguint() {
        Some(numerator) => numerator.to_bytes_le(),
        None => return Err(DataError::custom("the numerator is too large")),
    };

    let denominator = match fraction.denom().to_biguint() {
        Some(denominator) => denominator.to_bytes_le(),
        None => return Err(DataError::custom("the denominator is too large")),
    };

    let sign = match fraction.numer().sign() {
        num_bigint::Sign::Plus => Sign::Positive,
        num_bigint::Sign::Minus => Sign::Negative,
        num_bigint::Sign::NoSign => {
            return Err(DataError::custom("the numerator is zero"));
        }
    };

    Ok((numerator, denominator, sign, constant_exactness))
}

/// Converts slices of numerator and denominator strings to a fraction.
/// Examples:
/// - ["1"], ["2"] is converted to 1/2
/// - ["1", "2"], ["3", "1E2"] is converted to 1*2/(3*1E2) --> 2/300
/// - ["1", "2"], ["3", "1E-2"] is converted to 1*2/(3*1E-2) --> 200/3
/// - ["1", "2"], ["3", "1E-2.5"] is an invalid scientific notation number
/// - ["1E2"], ["2"] is converted to 1E2/2 --> 100/2 --> 50/1
/// - ["1E2", "2"], ["3", "1E2"] is converted to 1E2*2/(3*1E2) --> 2/3
pub fn convert_slices_to_fraction(
    numerator_strings: &[String],
    denominator_strings: &[String],
) -> Result<BigRational, DataError> {
    let mut fraction = BigRational::new(BigInt::from(1u32), BigInt::from(1u32));

    for numerator in numerator_strings {
        let num_fraction = convert_scientific_notation_to_fraction(numerator)?;
        fraction *= num_fraction;
    }

    for denominator in denominator_strings {
        let den_fraction = convert_scientific_notation_to_fraction(denominator)?;
        fraction /= den_fraction;
    }

    Ok(fraction)
}

// TODO: move some of these tests to the comment above.
#[test]
fn test_convert_array_of_strings_to_fraction() {
    let numerator = vec!["1".to_string()];
    let denominator = vec!["2".to_string()];
    let expected = BigRational::new(BigInt::from(1u32), BigInt::from(2u32));
    let actual = convert_slices_to_fraction(&numerator, &denominator).unwrap();
    assert_eq!(expected, actual);

    let numerator = vec!["1".to_string(), "2".to_string()];
    let denominator = vec!["3".to_string(), "1E2".to_string()];
    let expected = BigRational::new(BigInt::from(2u32), BigInt::from(300u32));
    let actual = convert_slices_to_fraction(&numerator, &denominator).unwrap();
    assert_eq!(expected, actual);

    let numerator = vec!["1".to_string(), "2".to_string()];
    let denominator = vec!["3".to_string(), "1E-2".to_string()];
    let expected = BigRational::new(BigInt::from(200u32), BigInt::from(3u32));
    let actual = convert_slices_to_fraction(&numerator, &denominator).unwrap();
    assert_eq!(expected, actual);

    let numerator = vec!["1".to_string(), "2".to_string()];
    let denominator = vec!["3".to_string(), "1E-2.5".to_string()];
    let actual = convert_slices_to_fraction(&numerator, &denominator);
    assert!(actual.is_err());

    let numerator = vec!["1E2".to_string()];
    let denominator = vec!["2".to_string()];
    let expected = BigRational::new(BigInt::from(50u32), BigInt::from(1u32));
    let actual = convert_slices_to_fraction(&numerator, &denominator).unwrap();
    assert_eq!(expected, actual);

    let numerator = vec!["1E2".to_string(), "2".to_string()];
    let denominator = vec!["3".to_string(), "1E2".to_string()];
    let expected = BigRational::new(BigInt::from(2u32), BigInt::from(3u32));
    let actual = convert_slices_to_fraction(&numerator, &denominator).unwrap();
    assert_eq!(expected, actual);
}

/// Splits a constant string into a tuple of (numerator, denominator).
/// The numerator and denominator are represented as arrays of strings.
/// Examples:
/// - "1/2" is split into (["1"], ["2"])
/// - "1 * 2 / 3 * ft_to_m" is split into (["1", "2"], ["3" , "ft_to_m"])
/// - "/2" is split into (["1"], ["2"])
/// - "2" is split into (["2"], ["1"])
/// - "1E2" is split into (["1E2"], ["1"])
/// - "1 2 * 3" is an invalid constant string
pub fn convert_constant_to_num_denom_strings(
    constant_string: &str,
) -> Result<(Vec<String>, Vec<String>), DataError> {
    let mut split = constant_string.split('/');
    if split.clone().count() > 2 {
        return Err(DataError::custom("Invalid constant string"));
    }

    let numerator_string = split.next().unwrap_or("1");
    let denominator_string = split.next().unwrap_or("1");

    let mut has_whitespace_within = false;
    let numerator_values = if numerator_string.is_empty() {
        vec!["1".to_string()]
    } else {
        numerator_string
            .split('*')
            .map(|s| {
                let s = s.trim();
                if s.chars().any(char::is_whitespace) {
                    has_whitespace_within = true;
                }
                s.to_string()
            })
            .collect()
    };

    let denominator_values = if denominator_string.is_empty() {
        vec!["1".to_string()]
    } else {
        denominator_string
            .split('*')
            .map(|s| {
                let s = s.trim();
                if s.chars().any(char::is_whitespace) {
                    has_whitespace_within = true;
                }
                s.to_string()
            })
            .collect::<Vec<String>>()
    };

    if has_whitespace_within {
        return Err(DataError::custom(
            "The constant string contains internal white spaces",
        ));
    }

    Ok((numerator_values, denominator_values))
}
// TODO: move this to the comment above.
#[test]
fn test_convert_constant_to_num_denom_strings() {
    let input = "1/2";
    let expected = (vec!["1".to_string()], vec!["2".to_string()]);
    let actual = convert_constant_to_num_denom_strings(input).unwrap();
    assert_eq!(expected, actual);

    let input = "1 * 2 / 3 * ft_to_m";
    let expected = (
        vec!["1".to_string(), "2".to_string()],
        vec!["3".to_string(), "ft_to_m".to_string()],
    );
    let actual = convert_constant_to_num_denom_strings(input).unwrap();
    assert_eq!(expected, actual);

    let input = "/2";
    let expected = (vec!["1".to_string()], vec!["2".to_string()]);
    let actual = convert_constant_to_num_denom_strings(input).unwrap();
    assert_eq!(expected, actual);

    let input = "2";
    let expected = (vec!["2".to_string()], vec!["1".to_string()]);
    let actual = convert_constant_to_num_denom_strings(input).unwrap();
    assert_eq!(expected, actual);

    let input = "1E2";
    let expected = (vec!["1E2".to_string()], vec!["1".to_string()]);
    let actual = convert_constant_to_num_denom_strings(input).unwrap();
    assert_eq!(expected, actual);

    let input = "1 2 * 3";
    let actual = convert_constant_to_num_denom_strings(input);
    assert!(actual.is_err());
}
