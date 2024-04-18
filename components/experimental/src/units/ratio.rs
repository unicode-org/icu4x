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
    /// Represents an error when parsing a ratio from a string.
    /// For example, the string "1/0/2" is invalid because it contains more than one '/' character.
    InvalidRatioString,

    /// Represents an error when a division by zero is attempted or when the denominator is zero in a ratio.
    DivisionByZero,

    /// Represents an error when a ratio string contains multiple slashes.
    /// For example, the string "1/2/3" is invalid because it contains more than one '/' character.
    MultipleSlashes,

    /// Represents an error when a ratio string contains multiple scientific notations.
    /// For example, the string "1.5E6E6" is invalid because it contains more than one 'E' character.
    MultipleScientificNotation,
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
    /// Parsing errors are returned for:
    /// - Division by zero: "1/0".
    /// - Multiple slashes: "1/2/3".
    /// - Non-numeric characters in fractions: "1/2A".
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        /// Converts a decimal string to an `IcuRatio`.
        ///
        /// # Examples
        /// - "1.5" is converted to an `IcuRatio` representing "3/2".
        fn decimal_str_to_icu_ratio(decimal: &str) -> Option<IcuRatio> {
            let mut components = decimal.split('.');
            let integer_component = components.next().unwrap_or("0");
            let decimal_component = components.next().unwrap_or("0");
            let decimal_length = decimal_component.chars().count() as i32;

            if components.next().is_some() {
                return None;
            }

            let integer_component = match BigRational::from_str(integer_component) {
                Ok(ratio) => IcuRatio(ratio),
                Err(_) => return None,
            };
            let decimal_component = match BigRational::from_str(decimal_component) {
                Ok(ratio) => IcuRatio(ratio),
                Err(_) => return None,
            };

            let ten = IcuRatio::ten();
            let decimal_component = decimal_component / ten.pow(decimal_length);
            Some(integer_component + decimal_component)
        }

        /// Converts a string in scientific notation to an `IcuRatio`.
        ///
        /// # Examples
        /// - "1.5E6" is converted to an `IcuRatio` representing "1500000".
        /// - "1.5E-6" is converted to an `IcuRatio` representing "0.0000015".
        /// - "1,500E6" is converted to an `IcuRatio` representing "1500000". (Commas are ignored)
        /// - "1.5E6A" returns `None` because of the non-numeric character 'A'.
        pub fn scientific_notation_to_icu_ratio(scientific_notation: &str) -> Result<IcuRatio, IcuRatioError> {
            //remove all the commas
            let rational = scientific_notation.replace(',', "");

            let mut parts = rational.split('E');
            let rational_part = parts.next().unwrap_or("1");
            let exponent_part = parts.next().unwrap_or("0");
            if parts.next().is_some() {
                return Err(IcuRatioError::MultipleScientificNotation);
            }

            let rational_part = decimal_str_to_icu_ratio(rational_part)?;
            let exponent_part = i32::from_str(exponent_part).unwrap();

            let ten = IcuRatio::ten();
            let exponent_part = ten.pow(exponent_part);
            Ok(rational_part * exponent_part)
        }

        let mut parts = s.split('/');
        let numerator = parts.next();
        let denominator = parts.next();
        if parts.next().is_some() {
            return Err(IcuRatioError::MultipleSlashes);
        }

        let numerator = match numerator {
            Some(num_str) => scientific_notation_to_icu_ratio(num_str)
                .ok_or(IcuRatioError::InvalidRatioString)?,
            None => IcuRatio::zero(),
        };

        let denominator = match denominator {
            Some(den_str) => scientific_notation_to_icu_ratio(den_str)
                .ok_or(IcuRatioError::InvalidRatioString)?,
            None => IcuRatio::one(),
        };

        if denominator.is_zero() {
            return Err(IcuRatioError::DivisionByZero);
        }

        Ok(numerator / denominator)
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
            ("1", Ok(IcuRatio::from_big_ints(1.into(), 1.into()))),
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
