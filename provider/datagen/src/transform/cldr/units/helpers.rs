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

/// Converts a scientific notation number represented as a string into a tuple of (numerator, denominator).
pub fn convert_scientific_notation_number_to_fractional(
    number: &str,
) -> Result<GenericFraction<BigUint>, DataError> {
    let number = remove_whitespace(number); // TODO: check this.
    let mut split = number.split('E');
    if split.clone().count() > 2 {
        return Err(DataError::custom(
            "the number is not a scientific notation number",
        ));
    }
    let base = split.next().unwrap_or("0");
    let exponent = split.next().unwrap_or("0");
    let base: GenericFraction<BigUint> = match GenericFraction::from_str(base) {
        Ok(base) => base,
        Err(_) => return Err(DataError::custom("the number is not a valid number")),
    };
    let exponent = match f64::from_str(exponent) {
        Ok(exponent) => exponent,
        Err(_) => return Err(DataError::custom("the exponent is not a valid number")),
    };

    let mut result = base;
    let generic_ten: GenericFraction<BigUint> =
        GenericFraction::new(BigUint::from(10u32), BigUint::from(1u32)); // TODO: fix this
    if exponent > 0.0 {
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

/// Checks if a string has letters.
/// Returns true if the string has letters, false otherwise.
pub fn has_letters(s: &str) -> bool {
    s.chars().any(|c| c.is_alphabetic())
}

/// Converts a fractional number to a constant value.
pub fn convert_fractional_to_constant_value(
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
        let num = match convert_scientific_notation_number_to_fractional(vnum) {
            Ok(num) => num,
            Err(e) => return Err(e),
        };
        result = result.mul(num);
    }

    for vden in den.iter() {
        let den = match convert_scientific_notation_number_to_fractional(vden) {
            Ok(den) => den,
            Err(e) => return Err(e),
        };
        result = result.div(den);
    }

    Ok(result)
}

/// Splits the constant string into a tuple of (numerator, denominator).
/// The numerator and denominator are represented as array of strings.
///    For example: "1/2" -> (["1"], ["2"])
///                 "1 * 2 / 3 * ft_to_m" -> (["1", "2"], ["3" , "ft_to_m"])
pub fn split_constant_string(
    constant_string: &str,
) -> Result<(Vec<String>, Vec<String>), DataError> {
    let constant_string = remove_whitespace(constant_string);
    let mut numerator = Vec::<String>::new();
    let mut denominator = Vec::<String>::new();

    let mut split = constant_string.split('/');
    if split.clone().count() > 2 {
        return Err(DataError::custom("the constant string is not valid"));
    }
    let numerator_string = split.next().unwrap_or("1");
    let denominator_string = split.next().unwrap_or("1");

    let mut split = numerator_string.split('*');
    for num in split {
        numerator.push(num.to_string());
    }

    let mut split = denominator_string.split('*');
    for num in split {
        denominator.push(num.to_string());
    }

    Ok((numerator, denominator))
}

#[test]
fn test_remove_whitespace() {
    let input = "He  llo Wo rld!";
    let expected = "HelloWorld!";
    let actual = remove_whitespace(input);
    assert_eq!(expected, actual);
}
