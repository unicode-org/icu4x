// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_datetime::input;
use icu_provider::DataError;

/// Removes all whitespace from a string.
fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

/// Split a string into a vector of strings, using specific characters as delimiters.
fn split_string(s: &str, delimiters: Vec<&str>) -> Vec<String> {
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

/// Returns the greatest common divisor of two numbers.
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

/// Checks if a string represents a decimal number.
fn check_if_decimal_number(number: &str) -> bool {
    if number.chars().next().unwrap_or('0') == '-' || number.chars().next().unwrap_or('0') == '+' {
        return check_if_decimal_number(&number[1..]);
    }

    let mut split = number.split('.');
    if split.clone().count() > 2 {
        return false;
    }
    let whole = split.next().unwrap_or("0");
    let fractional = split.next().unwrap_or("0");
    if whole.chars().any(|c| !c.is_digit(10)) {
        return false;
    }
    if fractional.chars().any(|c| !c.is_digit(10)) {
        return false;
    }
    true
}

/// Converts a rational number represented as a string into a tuple of (numerator, denominator).
fn to_fractional(number: &str) -> Result<(u64, u64), DataError> {
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
fn check_if_scientific_notation_number(number: &str) -> bool {
    let mut split = number.split('E');
    if split.clone().count() > 2 {
        return false;
    }

    let base = split.next().unwrap_or("0");
    let exponent = split.next().unwrap_or("0");
    check_if_decimal_number(base) && check_if_decimal_number(exponent)
}

/// Converts a scientific notation number represented as a string into a tuple of (numerator, denominator).
fn convert_scientific_notation_number_to_fractional(number: &str) -> Result<(u64, u64), DataError> {
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
}
