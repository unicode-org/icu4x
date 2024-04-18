// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::{
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
    str::FromStr,
};

use num_bigint::BigInt;
use num_rational::{BigRational, Ratio};
use num_traits::Signed;
use num_traits::ToPrimitive;
use num_traits::{One, Pow, Zero};

use super::provider::{Base, SiPrefix};

// TODO: add test cases for IcuRatio.
// TODO: Make a decicion on whether to keep the `IcuRatio` public or not.
/// A ratio type that uses `BigInt` as the underlying type.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IcuRatio(Ratio<BigInt>);

#[derive(Debug, PartialEq)]
pub enum IcuRatioError {
    /// Represents an error when a division by zero is attempted or when the denominator is zero in a ratio.
    DivisionByZero,

    /// Represents an error when a ratio string contains multiple slashes.
    /// For example, the string "1/2/3" is invalid because it contains more than one '/' character.
    MultipleSlashes,

    /// Represents an error when a ratio string contains multiple scientific notations.
    /// For example, the string "1.5E6E6" is invalid because it contains more than one 'E' character.
    MultipleScientificNotations,

    /// Represents an error when a ratio string contains multiple decimal points.
    /// For example, the string "1.5.6" is invalid because it contains more than one '.' character.
    MultipleDecimalPoints,

    /// Represents an error when the exponent part of a ratio string is not an integer.
    /// For example, the string "1.5E6.5" is invalid because the exponent part "6.5" is not an integer.
    ExponentPartIsNotInteger,

    /// Represents an error when a ratio string is invalid.
    InvalidRatioString,
}

impl IcuRatio {
    /// Creates a new `IcuRatio` from the given numerator and denominator.
    pub(crate) fn from_big_ints(numerator: BigInt, denominator: BigInt) -> Self {
        Self(Ratio::new(numerator, denominator))
    }

    /// Returns the reciprocal of the ratio.
    /// For example, the reciprocal of 2/3 is 3/2.
    pub(crate) fn recip(&self) -> Self {
        Self(self.0.recip())
    }

    // TODO: Make the function private after fixing the need for it in the tests.
    /// Returns the value of the ratio as a `f64`.
    pub fn to_f64(&self) -> Option<f64> {
        Some(self.0.numer().to_f64()? / self.0.denom().to_f64()?)
    }

    /// Returns the absolute value of the ratio.
    pub fn abs(&self) -> Self {
        Self(self.0.abs())
    }

    /// Returns a Ratio with value of 2.
    pub fn two() -> Self {
        Self(Ratio::from_integer(2.into()))
    }

    /// Returns a Ratio with value of 10.
    pub fn ten() -> Self {
        Self(Ratio::from_integer(10.into()))
    }
}

impl Mul for IcuRatio {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0)
    }
}

impl Mul<&IcuRatio> for &IcuRatio {
    type Output = IcuRatio;

    fn mul(self, rhs: &IcuRatio) -> IcuRatio {
        IcuRatio(&self.0 * &rhs.0)
    }
}

impl MulAssign<IcuRatio> for IcuRatio {
    fn mul_assign(&mut self, rhs: IcuRatio) {
        self.0 *= rhs.0;
    }
}

impl MulAssign<&SiPrefix> for IcuRatio {
    fn mul_assign(&mut self, rhs: &SiPrefix) {
        match rhs.base {
            Base::Decimal => {
                *self *= IcuRatio::ten().pow(rhs.power as i32);
            }
            Base::Binary => {
                *self *= IcuRatio::two().pow(rhs.power as i32);
            }
        }
    }
}

impl Div for IcuRatio {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self(self.0 / rhs.0)
    }
}

impl DivAssign for IcuRatio {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
    }
}

impl Add for IcuRatio {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl Add<&IcuRatio> for &IcuRatio {
    type Output = IcuRatio;

    fn add(self, rhs: &IcuRatio) -> IcuRatio {
        IcuRatio(&self.0 + &rhs.0)
    }
}

impl AddAssign for IcuRatio {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub for IcuRatio {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for IcuRatio {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl Pow<i32> for IcuRatio {
    type Output = Self;

    fn pow(self, exp: i32) -> Self {
        Self(self.0.pow(exp))
    }
}

impl Zero for IcuRatio {
    fn zero() -> Self {
        Self(Ratio::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl One for IcuRatio {
    fn one() -> Self {
        Self(Ratio::one())
    }

    fn is_one(&self) -> bool {
        self.0.is_one()
    }
}

impl From<Ratio<BigInt>> for IcuRatio {
    fn from(ratio: Ratio<BigInt>) -> Self {
        Self(ratio)
    }
}

impl From<u32> for IcuRatio {
    fn from(value: u32) -> Self {
        Self(Ratio::from_integer(value.into()))
    }
}

impl FromStr for IcuRatio {
    type Err = IcuRatioError;

    /// Converts a string representation of a ratio into an `IcuRatio`.
    /// Supported string formats include:
    /// - Fractional notation: "1/2" becomes "1/2".
    /// - Decimal notation: "1.5" becomes "3/2".
    /// - Scientific notation: "1.5E6" becomes "1500000", "1.5E-6" becomes "0.0000015".
    /// - Scientific notation with commas: "1,500E6" becomes "1500000000". Commas are disregarded.
    /// - Integer notation: "1" becomes "1".
    /// - Empty string: "" becomes "0".
    /// - Negative numbers: "-1/2" becomes "-1/2".
    /// - Negative numbers with decimal notation: "-1.5" becomes "-3/2".
    /// - Negative numbers with scientific notation: "-1.5E6" becomes "-1500000".
    /// - Negative numbers with scientific notation with commas: "-1,500E6" becomes "-1500000000".
    /// Parsing errors are returned for:
    /// - Division by zero: "1/0".
    /// - Multiple slashes: "1/2/3".
    /// - Non-numeric characters in fractions: "1/2A".
    /// - Multiple scientific notations: "1.5E6E6".
    /// - Multiple decimal points: "1.5.6".
    /// - Exponent part is not an integer: "1.5E6.5".
    /// NOTE:
    ///    You can add as many commas as you want in the string, they will be disregarded.
    fn from_str(number_str: &str) -> Result<Self, IcuRatioError> {
        /// Parses a fraction from a string.
        /// The input string is expected to be in any fractional format.
        /// For example, "1", "1/2", "1.5", "1.5/6", "1.4/5.6".
        /// No scientific notation is allowed.
        fn parce_fraction(decimal: &str) -> Result<IcuRatio, IcuRatioError> {
            let mut components = decimal.split('/');
            let numerator = components.next();
            let denominator = components.next();
            if components.next().is_some() {
                return Err(IcuRatioError::MultipleSlashes);
            }

            let numerator = match numerator {
                Some(numerator) => parse_decimal(numerator)?,
                None => IcuRatio::zero(),
            };

            let denominator = match denominator {
                Some(denominator) => parse_decimal(denominator)?,
                None => IcuRatio::one(),
            };

            if denominator.is_zero() {
                return Err(IcuRatioError::DivisionByZero);
            }

            Ok(numerator / denominator)
        }

        /// Parses a decimal from a string.
        /// The input string is expected to be in any decimal format.
        /// For example, "1", "1.5".
        /// Empty strings are treated as "0".
        /// No fractions are allowed.
        fn parse_decimal(decimal: &str) -> Result<IcuRatio, IcuRatioError> {
            // count the "." in the string
            let dot_count = decimal.chars().filter(|&c| c == '.').count();
            if dot_count > 1 {
                return Err(IcuRatioError::MultipleDecimalPoints);
            }

            if dot_count == 0 {
                return Ok(IcuRatio(
                    BigRational::from_str(decimal)
                        .map_err(|_| IcuRatioError::InvalidRatioString)?,
                ));
            }

            // calculate the length between the "." and the end of the string
            let decimal_length = decimal.len() - decimal.find('.').unwrap() - 1;

            // remove the "." from the string
            let decimal = decimal.replace(".", "");

            let numerator = IcuRatio(
                BigRational::from_str(&decimal).map_err(|_| IcuRatioError::InvalidRatioString)?,
            );

            let ten = IcuRatio::ten();
            let denomerator = ten.pow(decimal_length as i32);

            Ok(numerator / denomerator)
        }

        // remove commas from the string
        let number_str = number_str.replace(",", "");

        // check if the number is empty after removing commas
        if number_str.is_empty() {
            return Ok(IcuRatio::zero());
        }

        let mut parts = number_str.split(|c| c == 'e' || c == 'E');
        let significand = parts.next();
        let exponent = parts.next();
        if parts.next().is_some() {
            return Err(IcuRatioError::MultipleScientificNotations);
        }

        let significand = match significand {
            Some(significand) => parce_fraction(significand)?,
            None => IcuRatio::one(),
        };

        let exponent = match exponent {
            Some(exponent) => {
                let exponent = match exponent.parse::<i32>() {
                    Ok(exponent) => exponent,
                    Err(_) => return Err(IcuRatioError::ExponentPartIsNotInteger),
                };
                let ten = IcuRatio::ten();
                ten.pow(exponent)
            }
            None => IcuRatio::one(),
        };

        Ok(significand * exponent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icu_ratio_from_str() {
        let test_cases: &[(&str, Result<IcuRatio, IcuRatioError>)] = &[
            ("1/2", Ok(IcuRatio::from_big_ints(1.into(), 2.into()))),
            ("1.5", Ok(IcuRatio::from_big_ints(3.into(), 2.into()))),
            (
                "1.5E6",
                Ok(IcuRatio::from_big_ints(1500000.into(), 1.into())),
            ),
            (
                "1.5E-6",
                Ok(IcuRatio::from_big_ints(15.into(), 10000000.into())),
            ),
            (
                "1,500E6",
                Ok(IcuRatio::from_big_ints(1500000000.into(), 1.into())),
            ),
            (
                "1.0005",
                Ok(IcuRatio::from_big_ints(10005.into(), 10000.into())),
            ),
            (".0005", Ok(IcuRatio::from_big_ints(5.into(), 10000.into()))),
            ("1", Ok(IcuRatio::from_big_ints(1.into(), 1.into()))),
            ("", Ok(IcuRatio::from_big_ints(0.into(), 1.into()))),
            ("1/0", Err(IcuRatioError::DivisionByZero)),
            ("1/2/3", Err(IcuRatioError::MultipleSlashes)),
            ("1/2A", Err(IcuRatioError::InvalidRatioString)),
        ];

        for (input, expected) in test_cases.iter() {
            let actual = IcuRatio::from_str(input);
            match (actual, expected) {
                (Ok(ref actual_val), Ok(ref expected_val)) => assert_eq!(
                    actual_val, expected_val,
                    "Values do not match for input: {}",
                    input
                ),
                (Err(ref actual_err), Err(ref expected_err)) => {
                    assert_eq!(
                        actual_err, expected_err,
                        "Error types do not match for input: {}",
                        input
                    )
                }
                _ => assert!(
                    false,
                    "Result types (Ok/Err) do not match for input: {}",
                    input
                ),
            }
        }

        let actual = IcuRatio::from_str("1.5").unwrap();
        let expected = IcuRatio::from_big_ints(3.into(), 2.into());
        assert_eq!(actual, expected);
    }
}
